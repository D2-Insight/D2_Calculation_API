
use crate::{types::rs_types::{RangeFormula,
    ReloadResponse,
    ReloadFormula,
    HandlingFormula,
    HandlingResponse,
    RangeResponse,
    AmmoFormula,
    AmmoResponse
}, perks::lib::RangeModifierResponse};


// #[derive(Debug, Clone, Copy)]
// pub struct ReloadData {
//     a: f64,
//     b: f64,
//     c: f64,
// }
// impl ReloadData {
//     fn calc_reload_time(&self, _reload_stat: i32, _duration_scalar: f64) -> f64 {
//         let reload_stat = _reload_stat as f64;
//         (self.a * (reload_stat.powi(2))) + (self.b * reload_stat) + self.c
//     }
// }

// #[derive(Debug, Clone, Copy)]
// struct RangeResponse {
//     hip_falloff_start: f64,
//     hip_falloff_end: f64,
//     falloff_start: f64,
//     falloff_end: f64,
// }
// #[derive(Debug, Clone, Copy)]
// pub struct RangeData {
//     zoom: f64,
//     zoom_tier: f64,
//     vpp: f64,
//     base_min: f64,
//     base_max: f64,
//     scale_max: bool,
// }
// impl RangeData {
//     fn calc_range_falloff(
//         &self,
//         _range_stat: i32,
//         _zoom_stat: i32,
//         _hipfire_mult: f64,
//         _global_mult: f64,
//     ) -> RangeResponse {
//         let range_stat = _range_stat as f64;
//         let zoom_stat = _zoom_stat as f64;

//         let new_zoom = (zoom_stat - self.zoom_tier) / 10.0 + self.zoom;

//         let hip_falloff_start =
//             (range_stat * self.vpp + self.base_min) * _hipfire_mult * _global_mult;
//         let hip_falloff_end =
//             (range_stat * self.vpp + self.base_max) * _hipfire_mult * _global_mult;

//         let falloff_start = (range_stat * self.vpp + self.base_min) * new_zoom * _global_mult;
//         let falloff_end = (range_stat * self.vpp + self.base_max) * new_zoom * _global_mult;

//         RangeResponse {
//             hip_falloff_start,
//             hip_falloff_end,
//             falloff_start,
//             falloff_end,
//         }
//     }
// }

// #[derive(Debug, Clone)]
// struct HandlingResponse {
//     ready_time: f64,
//     stow_time: f64,
// }
// #[derive(Debug, Clone)]
// pub struct HandlingData {
//     ready_vpp: f64,
//     ready_base: f64,
//     stow_vpp: f64,
//     stow_base: f64,
// }
// impl HandlingData {
//     fn calc_handling_times(&self, _handling_stat: i32, _duraction_scalar: f64) -> HandlingResponse {
//         let handling_stat = _handling_stat as f64;

//         let ready_time = (handling_stat * self.ready_vpp + self.ready_base) * _duraction_scalar;
//         let stow_time = (handling_stat * self.stow_vpp + self.stow_base) * _duraction_scalar;

//         HandlingResponse {
//             ready_time,
//             stow_time,
//         }
//     }
// }

// pub fn calc_reload(
//     _reload_stat: i32,
//     _duration_scalar: f64,
//     _ammo_percent: f64,
//     _formula_data: ReloadFormula,
// ) -> f64 {
//     let reload_stat = _reload_stat as f64;
//     let base_val = (_formula_data.evpp * (reload_stat.powi(2)))
//         + (_formula_data.vpp * reload_stat)
//         + _formula_data.offset;
//     base_val * _duration_scalar
// }
impl ReloadFormula {
    pub fn calc_reload_time(&self, _reload_stat: i32, _duration_scalar: f64) -> ReloadResponse {
        let reload_stat = _reload_stat as f64;
        let reload_time = self.reload_data.solve_at(reload_stat) * _duration_scalar;
        ReloadResponse { reload_time, ammo_time: reload_time*self.ammo_percent }
    }
}


impl RangeFormula {
    pub fn calc_range_falloff(
        &self,
        _range_stat: i32,
        _zoom_stat: i32,
        _modifiers: RangeModifierResponse,
    ) -> RangeResponse {
        let range_stat = if (_range_stat + _modifiers.range_stat_add) > 100 {100} else {_range_stat + _modifiers.range_stat_add} as f64;
        let zoom_stat = _zoom_stat as f64 * _modifiers.range_zoom_scale;

        let zoom_mult = if self.is_fusion { 1.0 + 0.02*zoom_stat } else { 0.1*zoom_stat };

        let hip_falloff_start =
            self.start.solve_at(range_stat) * _modifiers.range_hip_scale * _modifiers.range_all_scale;
        let hip_falloff_end =
            self.end.solve_at(range_stat) * _modifiers.range_hip_scale * _modifiers.range_all_scale;

        let ads_falloff_start = hip_falloff_start * zoom_mult;
        let ads_falloff_end = hip_falloff_end * zoom_mult;

        RangeResponse {
            hip_falloff_start,
            hip_falloff_end,
            ads_falloff_start,
            ads_falloff_end,
        }
    }
}


pub fn calc_handling(
    _handling_stat: i32,
    _ads_duraction_scalar: f64,
    _draw_duraction_scalar: f64,
    _formula_data: HandlingFormula,
) -> HandlingResponse {
    let handling_stat = _handling_stat as f64;

    let ready_time = (handling_stat * _formula_data.ready.vpp + _formula_data.ready.offset)
        * _draw_duraction_scalar;
    let mut stow_time = (handling_stat * _formula_data.stow.vpp + _formula_data.stow.offset)
        * _draw_duraction_scalar;
    let ads_time = (handling_stat * _formula_data.ads.vpp + _formula_data.ads.offset)
        * _ads_duraction_scalar;

    let hundred_stow_time = 100.0 * _formula_data.ads.vpp + _formula_data.ads.offset;
    if stow_time > hundred_stow_time {
        stow_time = hundred_stow_time;
    }

    HandlingResponse {
        ready_time,
        stow_time,
        ads_time,
    }
}

pub fn calc_ammo(
    _mag_stat: i32,
    _reserve_stat: i32,
    _formula_data: AmmoFormula,
) -> AmmoResponse {
    let mag_stat = _mag_stat as f64;
    let reserve_stat = _reserve_stat as f64;

    let mut mag_size = ((_formula_data.mag.evpp * (mag_stat.powi(2)))
    + (_formula_data.mag.vpp * mag_stat)
    + _formula_data.mag.offset).ceil() as i32;
    if mag_size < 1 {
        mag_size = 1;
    }

    let reserve_known_values = _formula_data.reserves.keys().collect::<Vec<&i32>>();
    //find value in reserve_known_values that is closest to reserve_stat
    let closest_reserve_value = *reserve_known_values
        .iter()
        .min_by(|a, b| {
            let a_diff = (***a - _reserve_stat).abs();
            let b_diff = (***b - _reserve_stat).abs();
            a_diff.cmp(&b_diff)
        })
        .unwrap().clone();
    let reserve_formula_data = &*_formula_data.reserves.get(&closest_reserve_value).unwrap();
    let reserve_size = (reserve_formula_data.vpp * reserve_stat + reserve_formula_data.offset).ceil() as i32;

    AmmoResponse {
        mag: mag_size,
        mag_perk: 0,
        reserves: reserve_size,
    }
}