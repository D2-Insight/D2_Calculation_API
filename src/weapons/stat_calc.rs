use super::{Stat, Weapon, reserve_calc::calc_reserves};
use crate::{
    d2_enums::StatHashes,
    perks::{
        get_handling_modifier, get_magazine_modifier, get_range_modifier, get_reload_modifier,
        get_reserve_modifier,
        lib::{
            CalculationInput, HandlingModifierResponse, InventoryModifierResponse,
            MagazineModifierResponse, RangeModifierResponse, ReloadModifierResponse,
        },
    },
    types::rs_types::{
        AmmoFormula, HandlingFormula, HandlingResponse, RangeFormula,
        RangeResponse, ReloadFormula, ReloadResponse, AmmoResponse
    },
};

impl ReloadFormula {
    pub fn calc_reload_time_formula(
        &self,
        _reload_stat: i32,
        _modifiers: ReloadModifierResponse,
    ) -> ReloadResponse {
        let reload_stat = if (_reload_stat + _modifiers.reload_stat_add) > 100 {
            100
        } else {
            _reload_stat + _modifiers.reload_stat_add
        } as f64;
        let reload_time = self.reload_data.solve_at(reload_stat) * _modifiers.reload_time_scale;
        ReloadResponse {
            reload_time,
            ammo_time: reload_time * self.ammo_percent,
        }
    }
}
impl Weapon {
    pub fn calc_reload_time(&self, _calc_input: Option<CalculationInput>) -> ReloadResponse {
        let reload_stat = self
            .stats
            .get(&StatHashes::RELOAD.to_u32())
            .unwrap_or(&Stat::new())
            .val();
        if _calc_input.is_some() {
            let modifiers =
                get_reload_modifier(self.list_perks(), &_calc_input.unwrap(), self.is_pvp);
            self.reload_formula
                .calc_reload_time_formula(reload_stat, modifiers)
        } else {
            self.reload_formula
                .calc_reload_time_formula(reload_stat, ReloadModifierResponse::default())
        }
    }
}

impl RangeFormula {
    pub fn calc_range_falloff_formula(
        &self,
        _range_stat: i32,
        _zoom_stat: i32,
        _modifiers: RangeModifierResponse,
        _floor: f64,
    ) -> RangeResponse {
        let range_stat = if (_range_stat + _modifiers.range_stat_add) > 100 {
            100
        } else {
            _range_stat + _modifiers.range_stat_add
        } as f64;
        let zoom_stat = _zoom_stat as f64 * _modifiers.range_zoom_scale;

        let zoom_mult = if self.is_fusion {
            1.0 + 0.02 * zoom_stat
        } else {
            0.1 * zoom_stat - 0.025
        };

        let hip_falloff_start = self.start.solve_at(range_stat)
            * _modifiers.range_hip_scale
            * _modifiers.range_all_scale;
        let hip_falloff_end =
            self.end.solve_at(range_stat) * _modifiers.range_hip_scale * _modifiers.range_all_scale;

        let ads_falloff_start = hip_falloff_start * zoom_mult;
        let ads_falloff_end = hip_falloff_end * zoom_mult;

        RangeResponse {
            hip_falloff_start,
            hip_falloff_end,
            ads_falloff_start,
            ads_falloff_end,
            floor_percent: _floor,
        }
    }
}
impl Weapon {
    pub fn calc_range_falloff(&self, _calc_input: Option<CalculationInput>) -> RangeResponse {
        let range_stat = self
            .stats
            .get(&StatHashes::RANGE.to_u32())
            .unwrap_or(&Stat::new())
            .val();
        let zoom_stat = self
            .stats
            .get(&StatHashes::ZOOM.to_u32())
            .unwrap_or(&Stat::new())
            .val();
        if _calc_input.is_some() {
            let modifiers =
                get_range_modifier(self.list_perks(), &_calc_input.unwrap(), self.is_pvp);
            self.range_formula
                .calc_range_falloff_formula(range_stat, zoom_stat, modifiers, self.range_formula.floor_percent)
        } else {
            self.range_formula.calc_range_falloff_formula(
                range_stat,
                zoom_stat,
                RangeModifierResponse::default(),
                self.range_formula.floor_percent
            )
        }
    }
}

impl HandlingFormula {
    pub fn calc_handling_times_formula(
        &self,
        _handling_stat: i32,
        _modifiers: HandlingModifierResponse,
    ) -> HandlingResponse {
        let handling_stat = if (_handling_stat + _modifiers.handling_stat_add) > 100 {
            100
        } else {
            _handling_stat + _modifiers.handling_stat_add
        } as f64;
        let ready_time = self.ready.solve_at(handling_stat) * _modifiers.handling_swap_scale;
        let mut stow_time = self.stow.solve_at(handling_stat) * _modifiers.handling_swap_scale;
        let ads_time = self.ads.solve_at(handling_stat) * _modifiers.handling_ads_scale;
        if stow_time < self.stow.solve_at(handling_stat) {
            stow_time = self.stow.solve_at(handling_stat);
        }
        HandlingResponse {
            ready_time,
            stow_time,
            ads_time,
        }
    }
}
impl Weapon {
    pub fn calc_handling_times(&self, _calc_input: Option<CalculationInput>) -> HandlingResponse {
        let handling_stat = self
            .stats
            .get(&StatHashes::HANDLING.to_u32())
            .unwrap_or(&Stat::new())
            .val();
        if _calc_input.is_some() {
            let modifiers =
                get_handling_modifier(self.list_perks(), &_calc_input.unwrap(), self.is_pvp);
            self.handling_formula
                .calc_handling_times_formula(handling_stat, modifiers)
        } else {
            self.handling_formula
                .calc_handling_times_formula(handling_stat, HandlingModifierResponse::default())
        }
    }
}

impl AmmoFormula {
    pub fn calc_ammo_size_formula(
        &self,
        _mag_stat: i32,
        _mag_modifiers: MagazineModifierResponse,
        _reserve_stat: i32,
        _inv_modifiers: InventoryModifierResponse,
        _calc_inv: bool,
        _inv_id: i32,
    ) -> AmmoResponse {
        let mag_stat = if (_mag_stat + _mag_modifiers.magazine_stat_add) > 100 {
            100
        } else {
            _mag_stat + _mag_modifiers.magazine_stat_add
        } as f64;

        let inv_stat = if (_reserve_stat + _inv_modifiers.inv_stat_add) > 100 {
            100
        } else {
            _reserve_stat + _inv_modifiers.inv_stat_add
        } as f64;

        let raw_mag_size = (self.mag.evpp * (mag_stat.powi(2))) + (self.mag.vpp * mag_stat) + self.mag.offset;

        let mut mag_size =
            (((self.mag.evpp * (mag_stat.powi(2))) + (self.mag.vpp * mag_stat) + self.mag.offset)
                * _mag_modifiers.magazine_scale
                + _mag_modifiers.magazine_add)
                .ceil() as i32;
        if mag_size < 1 {
            mag_size = 1;
        }

        let mut reserve_size = 0;
        if _calc_inv {
            reserve_size = calc_reserves(raw_mag_size, inv_stat as i32, _inv_id);
        }
        AmmoResponse { mag_size, reserve_size }
    }
}
impl Weapon {
    pub fn calc_ammo_sizes(&self, _calc_input: Option<CalculationInput>) -> AmmoResponse {
        let mag_stat = self
            .stats
            .get(&StatHashes::MAGAZINE.to_u32())
            .unwrap_or(&Stat::new())
            .val();
        let inv_stat = self
            .stats
            .get(&StatHashes::INVENTORY_SIZE.to_u32())
            .unwrap_or(&Stat::new())
            .val();
        if _calc_input.is_some() {
            let mag_modifiers =
                get_magazine_modifier(self.list_perks(), &_calc_input.clone().unwrap(), self.is_pvp);
            let inv_modifiers =
                get_reserve_modifier(self.list_perks(), &_calc_input.clone().unwrap(), self.is_pvp);
            self.ammo_formula.calc_ammo_size_formula(mag_stat, mag_modifiers, inv_stat, inv_modifiers, true, self.ammo_formula.reserve_id)
        } else {
            self.ammo_formula
                .calc_ammo_size_formula(mag_stat, MagazineModifierResponse::default(), inv_stat, InventoryModifierResponse::default(),true, self.ammo_formula.reserve_id)
        }
    }
}
