use std::collections::HashMap;

use crate::{
    d2_enums::{AmmoType, DamageType, StatHashes, WeaponType},
    weapons::Stat,
};

use super::{
    add_dmr, add_epr, add_flmr, add_fmr, add_hmr, add_imr, add_mmr, add_rmr, add_rsmr, add_sbr,
    add_vmr, clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExplosivePercentResponse, ExtraDamageResponse,
        FiringModifierResponse, FlinchModifierResponse, HandlingModifierResponse,
        InventoryModifierResponse, MagazineModifierResponse, RangeModifierResponse, RefundResponse,
        ReloadModifierResponse,
    },
    ModifierResponseInput, Perks,
};

pub fn meta_perks() {
    add_dmr(
        Perks::BuiltIn,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let mut crit_scale = 1.0;
            let mut dmg_scale = 1.0;
            if *_input.calc_data.weapon_type == WeaponType::LINEARFUSIONRIFLE && !_input.pvp {
                crit_scale *= 1.15;
            };
            if *_input.calc_data.damage_type == DamageType::KINETIC && !_input.pvp {
                if _input.calc_data.ammo_type == &AmmoType::PRIMARY {
                    dmg_scale *= 1.1;
                } else if _input.calc_data.ammo_type == &AmmoType::SPECIAL {
                    dmg_scale *= 1.15;
                };
            };
            if *_input
                .calc_data
                .perk_value_map
                .get(&_input.calc_data.intrinsic_hash)
                .unwrap_or(&0)
                > 1
                && _input.calc_data.intrinsic_hash < 1000
            {
                let stat_bump_id: StatHashes = _input
                    .calc_data
                    .perk_value_map
                    .get(&_input.calc_data.intrinsic_hash)
                    .unwrap()
                    .to_owned()
                    .into();
                if stat_bump_id == StatHashes::CHARGE_TIME
                    && _input.calc_data.weapon_type == &WeaponType::FUSIONRIFLE
                {
                    // dmg_scale *=
                    //     dmr_chargetime_mw(_input, _input.value, is_enhanced, _pvp, _cached_data).impact_dmg_scale;
                }
            }
            DamageModifierResponse {
                crit_scale,
                impact_dmg_scale: dmg_scale,
                explosive_dmg_scale: dmg_scale,
            }
        }),
    );

    add_fmr(
        Perks::BuiltIn,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            #[allow(unused_mut)]
            let mut delay_add = 0.0;
            if *_input
                .calc_data
                .perk_value_map
                .get(&_input.calc_data.intrinsic_hash)
                .unwrap_or(&0)
                > 1
                && _input.calc_data.intrinsic_hash < 1000
            {
                let stat_bump_id: StatHashes = _input
                    .calc_data
                    .perk_value_map
                    .get(&_input.calc_data.intrinsic_hash)
                    .unwrap()
                    .to_owned()
                    .into();
                if stat_bump_id == StatHashes::CHARGE_TIME {
                    // delay_add += fmr_accelerated_coils(_input, _input.value, is_enhanced, _pvp, _cached_data)
                    //     .burst_delay_add;
                }
                if stat_bump_id == StatHashes::DRAW_TIME
                    && _input.calc_data.weapon_type == &WeaponType::BOW
                {
                    // delay_add += fmr_faster_string_t1(_input, _input.value, is_enhanced, _pvp, _cached_data)
                    //     .burst_delay_add;
                }
            }
            FiringModifierResponse {
                burst_delay_add: delay_add,
                ..Default::default()
            }
        }),
    );

    add_epr(
        Perks::BuiltIn,
        Box::new(
            |_input: ModifierResponseInput| -> ExplosivePercentResponse {
                if *_input.calc_data.weapon_type == WeaponType::GRENADELAUNCHER {
                    let blast_radius_struct =
                        _input.calc_data.stats.get(&StatHashes::BLAST_RADIUS.into());
                    let blast_radius;
                    if blast_radius_struct.is_none() {
                        blast_radius = 0;
                    } else {
                        blast_radius = blast_radius_struct.unwrap().val();
                    };
                    if _input.calc_data.ammo_type == &AmmoType::SPECIAL {
                        return ExplosivePercentResponse {
                            percent: 0.5 + 0.0025 * blast_radius as f64,
                            delyed: 0.0,
                            retain_base_total: true,
                        };
                    } else if _input.calc_data.ammo_type == &AmmoType::HEAVY {
                        return ExplosivePercentResponse {
                            percent: 0.7 + 0.00175 * blast_radius as f64,
                            delyed: 0.0,
                            retain_base_total: true,
                        };
                    };
                }
                if *_input.calc_data.weapon_type == WeaponType::ROCKET
                    && _input.calc_data.intrinsic_hash < 1000
                //ensures not exotic
                {
                    return ExplosivePercentResponse {
                        percent: 0.28,
                        delyed: 0.0,
                        retain_base_total: true,
                    };
                }
                ExplosivePercentResponse {
                    percent: 0.0,
                    delyed: 0.0,
                    retain_base_total: true,
                }
            },
        ),
    );

    add_hmr(
        Perks::DexterityMod,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                let swap_scale = if _input.value > 0 {
                    0.85 - clamp(_input.value, 1, 3) as f64 * 0.05
                } else {
                    1.0
                };
                HandlingModifierResponse {
                    stow_scale: swap_scale,
                    draw_scale: swap_scale,
                    ..Default::default()
                }
            },
        ),
    );

    add_hmr(
        Perks::TargetingMod,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                HandlingModifierResponse {
                    ads_scale: if _input.value > 0 { 0.75 } else { 1.0 },
                    ..Default::default()
                }
            },
        ),
    );

    add_sbr(
        Perks::TargetingMod,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value == 1 {
                stats.insert(StatHashes::AIM_ASSIST.into(), 10);
            } else if _input.value == 2 {
                stats.insert(StatHashes::AIM_ASSIST.into(), 15);
            } else if _input.value > 2 {
                stats.insert(StatHashes::AIM_ASSIST.into(), 20);
            }
            stats
        }),
    );

    add_imr(
        Perks::ReserveMod,
        Box::new(
            |_input: ModifierResponseInput| -> InventoryModifierResponse {
                let mut inv_buff = if _input.value > 0 { 20 } else { 0 };
                if _input.value == 2 {
                    inv_buff += 15;
                }
                if _input.value > 2 {
                    inv_buff += 20;
                }
                InventoryModifierResponse {
                    inv_stat_add: inv_buff,
                    inv_scale: 1.0,
                    inv_add: 0.0,
                }
            },
        ),
    );

    add_sbr(
        Perks::ReserveMod,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut inv_buff = if _input.value > 0 { 20 } else { 0 };
            if _input.value == 2 {
                inv_buff += 15;
            }
            if _input.value > 2 {
                inv_buff += 20;
            }
            let mut stats = HashMap::new();
            stats.insert(StatHashes::INVENTORY_SIZE.into(), inv_buff);
            stats
        }),
    );

    add_rsmr(
        Perks::LoaderMod,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value > 0 {
                let mut reload_stat_buff = 10;
                if _input.value > 1 {
                    reload_stat_buff += 5;
                };
                if _input.value > 2 {
                    reload_stat_buff += 5;
                };
                return ReloadModifierResponse {
                    reload_stat_add: reload_stat_buff,
                    reload_time_scale: 0.85,
                };
            } else {
                return ReloadModifierResponse::default();
            };
        }),
    );

    add_sbr(
        Perks::LoaderMod,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value > 0 {
                let mut reload_stat_buff = 10;
                if _input.value > 1 {
                    reload_stat_buff += 5;
                };
                if _input.value > 2 {
                    reload_stat_buff += 5;
                };
                stats.insert(StatHashes::RELOAD.into(), reload_stat_buff);
            };
            stats
        }),
    );

    add_flmr(
        Perks::UnflinchingMod,
        Box::new(|_input: ModifierResponseInput| -> FlinchModifierResponse {
            if _input.value > 2 {
                FlinchModifierResponse { flinch_scale: 0.6 }
            } else if _input.value == 2 {
                FlinchModifierResponse { flinch_scale: 0.7 }
            } else if _input.value == 1 {
                FlinchModifierResponse { flinch_scale: 0.75 }
            } else {
                FlinchModifierResponse::default()
            }
        }),
    );

    add_sbr(
        Perks::RallyBarricade,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            stats.insert(StatHashes::STABILITY.into(), 30);
            stats.insert(StatHashes::RELOAD.into(), 100);
            stats
        }),
    );

    add_flmr(
        Perks::RallyBarricade,
        Box::new(|_input: ModifierResponseInput| -> FlinchModifierResponse {
            FlinchModifierResponse { flinch_scale: 0.5 }
        }),
    );

    add_rsmr(
        Perks::RallyBarricade,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            ReloadModifierResponse {
                reload_stat_add: 100,
                reload_time_scale: 0.9,
            }
        }),
    );

    add_rmr(
        Perks::RallyBarricade,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            RangeModifierResponse {
                range_all_scale: 1.1,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::ChargetimeMW,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            fn down5(x: i32) -> f64 {
                (x as f64 - 5.0) / x as f64
            }
            let damage_mod = match _input.calc_data.intrinsic_hash {
                901 => down5(330), //high impact
                906 => down5(280),
                903 => down5(270),
                902 => down5(245), //rapid fire
                _ => 1.0,
            };
            DamageModifierResponse {
                explosive_dmg_scale: damage_mod,
                impact_dmg_scale: damage_mod,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::SurgeMod,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let damage_mod;
            if _input.pvp {
                if _input.value == 1 {
                    damage_mod = 1.03;
                } else if _input.value == 2 {
                    damage_mod = 1.044;
                } else if _input.value > 2 {
                    damage_mod = 1.055;
                } else {
                    damage_mod = 1.0;
                }
            } else {
                if _input.value == 1 {
                    damage_mod = 1.10;
                } else if _input.value == 2 {
                    damage_mod = 1.17;
                } else if _input.value > 2 {
                    damage_mod = 1.22;
                } else {
                    damage_mod = 1.0;
                }
            }
            DamageModifierResponse {
                explosive_dmg_scale: damage_mod,
                impact_dmg_scale: damage_mod,
                ..Default::default()
            }
        }),
    );
}
