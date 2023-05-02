use std::collections::HashMap;

use crate::d2_enums::{AmmoType, BungieHash, DamageType, StatBump, StatHashes, WeaponType};

use super::{
    add_dmr, add_epr, add_fmr, add_hmr, add_mmr, add_rmr, add_rsmr, add_sbr, add_vmr, clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExplosivePercentResponse, ExtraDamageResponse,
        FiringModifierResponse, HandlingModifierResponse, InventoryModifierResponse,
        MagazineModifierResponse, RangeModifierResponse, RefundResponse, ReloadModifierResponse,
        VelocityModifierResponse,
    },
    ModifierResponseInput, Perks,
};

pub fn year_6_perks() {
    add_sbr(
        Perks::KeepAway,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            let mut range_bonus = 0;
            let mut reload_bonus = 0;
            if _input.value > 0 {
                range_bonus = 10;
                reload_bonus = 30;
            };
            map.insert(StatHashes::Range.into(), range_bonus);
            map.insert(StatHashes::Reload.into(), reload_bonus);
            map
        }),
    );

    add_rmr(
        Perks::KeepAway,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            let range_bonus = if _input.value > 0 { 10 } else { 0 };
            RangeModifierResponse {
                range_stat_add: range_bonus,
                ..Default::default()
            }
        }),
    );

    add_rsmr(
        Perks::KeepAway,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let reload_bonus = if _input.value > 0 { 30 } else { 0 };
            ReloadModifierResponse {
                reload_stat_add: reload_bonus,
                ..Default::default()
            }
        }),
    );

    add_sbr(
        Perks::FieldTested,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            let val = clamp(_input.value, 0, 5) as i32;
            map.insert(StatHashes::Range.into(), val * 5);
            map.insert(StatHashes::Handling.into(), val * 5);
            map.insert(StatHashes::Reload.into(), val * 5);
            map.insert(StatHashes::Stability.into(), val * 5);
            map
        }),
    );

    add_hmr(
        Perks::FieldTested,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                let val = clamp(_input.value, 0, 5) as i32;
                HandlingModifierResponse {
                    stat_add: val * 5,
                    ..Default::default()
                }
            },
        ),
    );

    add_rsmr(
        Perks::FieldTested,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let val = clamp(_input.value, 0, 5) as i32;
            ReloadModifierResponse {
                reload_stat_add: val * 5,
                ..Default::default()
            }
        }),
    );

    add_rmr(
        Perks::FieldTested,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            let val = clamp(_input.value, 0, 5) as i32;
            RangeModifierResponse {
                range_stat_add: val * 5,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::ParacausalAffinity,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value > 0 {
                DamageModifierResponse {
                    explosive_dmg_scale: 1.2,
                    impact_dmg_scale: 1.2,
                    ..Default::default()
                }
            } else {
                DamageModifierResponse::default()
            }
        }),
    );

    add_mmr(
        Perks::EnviousAssasin,
        Box::new(
            |_input: ModifierResponseInput| -> MagazineModifierResponse {
                let val = clamp(_input.value, 0, 15) as f64;
                if _input.calc_data.total_shots_fired == 0.0 {
                    let mut mag_mult = 1.0;
                    if *_input.calc_data.ammo_type == AmmoType::Primary {
                        mag_mult += 0.1 * val;
                    } else {
                        mag_mult += 0.2 * val;
                    };
                    return MagazineModifierResponse {
                        magazine_stat_add: 0,
                        magazine_scale: clamp(mag_mult, 1.0, 2.5),
                        magazine_add: 0.0,
                    };
                };
                MagazineModifierResponse {
                    magazine_stat_add: 0,
                    magazine_scale: 1.0,
                    magazine_add: 0.0,
                }
            },
        ),
    );
}
