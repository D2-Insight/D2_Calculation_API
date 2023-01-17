use crate::js_types::{JsRangeFormula,
    JsReloadResponse,
    JsReloadFormula,
    JsHandlingFormula,
    JsHandlingResponse,
    JsRangeResponse,
    JsAmmoData,
    JsAmmoResponse
};

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

pub fn calc_reload(
    _reload_stat: i32,
    _duration_scalar: f64,
    _formula_data: JsReloadFormula,
) -> f64 {
    let reload_stat = _reload_stat as f64;
    (_formula_data.evpp * (reload_stat.powi(2)))
        + (_formula_data.vpp * reload_stat)
        + _formula_data.offset
}

pub fn calc_range(
    _range_stat: i32,
    _zoom_val: f64,
    _hipfire_mult: f64,
    _global_mult: f64,
    _formula_data: JsRangeFormula,
) -> JsRangeResponse {
    let range_stat = _range_stat as f64;
    let zoom_stat = _zoom_val;

    let new_zoom = (zoom_stat - _formula_data.zrm_tier as f64) / 10.0 + _formula_data.zrm;

    let hip_falloff_start =
        (range_stat * _formula_data.vpp + _formula_data.base_min) * _hipfire_mult * _global_mult;
    let hip_falloff_end =
        (range_stat * _formula_data.vpp + _formula_data.base_max) * _hipfire_mult * _global_mult;

    let ads_falloff_start =
        (range_stat * _formula_data.vpp + _formula_data.base_min) * new_zoom * _global_mult;
    let ads_falloff_end =
        (range_stat * _formula_data.vpp + _formula_data.base_max) * new_zoom * _global_mult;

    JsRangeResponse {
        hip_falloff_start,
        hip_falloff_end,
        ads_falloff_start,
        ads_falloff_end,
    }
}

pub fn calc_handling(
    _handling_stat: i32,
    _ads_duraction_scalar: f64,
    _draw_duraction_scalar: f64,
    _formula_data: JsHandlingFormula,
) -> JsHandlingResponse {
    let handling_stat = _handling_stat as f64;

    let ready_time = (handling_stat * _formula_data.ready_vpp + _formula_data.ready_offset)
        * _draw_duraction_scalar;
    let mut stow_time = (handling_stat * _formula_data.stow_vpp + _formula_data.stow_offset)
        * _draw_duraction_scalar;
    let ads_time = (handling_stat * _formula_data.ads_vpp + _formula_data.ads_offset)
        * _ads_duraction_scalar;

    let hundred_stow_time = 100.0 * _formula_data.ads_vpp + _formula_data.ads_offset;
    if stow_time > hundred_stow_time {
        stow_time = hundred_stow_time;
    }

    JsHandlingResponse {
        ready_time,
        stow_time,
        ads_time,
    }
}

pub fn calc_ammo(
    _mag_stat: i32,
    _reserve_stat: i32,
    _formula_data: JsAmmoData,
) -> JsAmmoResponse {
    let mag_stat = _mag_stat as f64;
    let reserve_stat = _reserve_stat as f64;

    let mut mag_size = ((_formula_data.mag_evpp * (mag_stat.powi(2)))
    + (_formula_data.mag_vpp * mag_stat)
    + _formula_data.mag_offset).ceil() as i32;
    if mag_size < 1 {
        mag_size = 1;
    }

    let reserve_known_values = _formula_data.reserve_formulas.keys().collect::<Vec<&i32>>();
    //find value in reserve_known_values that is closest to reserve_stat
    let closest_reserve_value = *reserve_known_values
        .iter()
        .min_by(|a, b| {
            let a_diff = (***a - _reserve_stat).abs();
            let b_diff = (***b - _reserve_stat).abs();
            a_diff.cmp(&b_diff)
        })
        .unwrap().clone();
    let reserve_formula_data = *_formula_data.reserve_formulas.get(&closest_reserve_value).unwrap();
    let reserve_size = (reserve_formula_data.0 * reserve_stat + reserve_formula_data.1).ceil() as i32;

    JsAmmoResponse {
        mag_size,
        mag_size_perk: 0,
        reserve_size,
    }
}