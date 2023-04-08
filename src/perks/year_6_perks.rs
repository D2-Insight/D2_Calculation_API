use std::collections::HashMap;

use crate::d2_enums::{AmmoType, DamageType, StatHashes, WeaponType, StatBump, BungieHash};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExplosivePercentResponse, ExtraDamageResponse,
        FiringModifierResponse, HandlingModifierResponse, InventoryModifierResponse,
        MagazineModifierResponse, RangeModifierResponse, RefundResponse, ReloadModifierResponse,
        VelocityModifierResponse,
    }, ModifierResponsInput, Perks, add_dmr, add_hmr, add_rsmr, add_vmr, add_fmr, add_rmr, add_mmr, add_epr, add_sbr
};

pub fn year_6_perks() {
    add_sbr(
        Perks::KeepAway,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
                let mut map = HashMap::new();
                let mut range_bonus = 0;
                let mut reload_bonus = 0;
                if _input.value > 0 {
                    range_bonus = 10;
                    reload_bonus = 30;
                };
                map.insert(StatHashes::RANGE.into(), range_bonus);
                map.insert(StatHashes::RELOAD.into(), reload_bonus);
                map
            }
        )
    );

    add_rmr(
        Perks::KeepAway,
        Box::new(
            |_input: ModifierResponsInput| -> RangeModifierResponse {
                let range_bonus = if _input.value > 0 { 10 } else { 0 };
                RangeModifierResponse {
                    range_stat_add: range_bonus,
                    ..Default::default()
                }
            }
        )
    );

    add_rsmr(
        Perks::KeepAway,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
                let reload_bonus = if _input.value > 0 { 30 } else { 0 };
                ReloadModifierResponse {
                    reload_stat_add: reload_bonus,
                    ..Default::default()
                }
            }
        )
    );

    add_sbr_field_tested(
        Perks::FieldTested,
        Box::new(
            |_input: ModifierResponsInput| -> HashMap<u32, i32> {
                let mut map = HashMap::new();
                let val = clamp(_input.value, 0, 5) as i32;
                map.insert(StatHashes::RANGE.into(), val * 5);
                map.insert(StatHashes::HANDLING.into(), val * 5);
                map.insert(StatHashes::RELOAD.into(), val * 5);
                map.insert(StatHashes::STABILITY.into(), val * 5);
                map
            }
        )
    );

    add_hmr(
        Perks::FieldTested,
        Box::new(
            |_input: ModifierResponsInput| -> HandlingModifierResponse {
                let val = clamp(_input.value, 0, 5) as i32;
                HandlingModifierResponse {
                    stat_add: val * 5,
                    ..Default::default()
                }
            }
        )
    );

    add_rsmr_field_tested(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> ReloadModifierResponse {
        let val = clamp(_input.value, 0, 5) as i32;
        ReloadModifierResponse {
            reload_stat_add: val * 5,
            ..Default::default()
        }
    }

    add_rmr_field_tested(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> RangeModifierResponse {
        let val = clamp(_input.value, 0, 5) as i32;
        RangeModifierResponse {
            range_stat_add: val * 5,
            ..Default::default()
        }
    }

    add_dmr_paracausal_affinity(
        Perks::,
        Box::new(
            |_input: ModifierResponsInput| -> DamageModifierResponse {
        if _input.value > 0 {
            DamageModifierResponse {
                explosive_dmg_scale: 1.2,
                impact_dmg_scale: 1.2,
                ..Default::default()
            }
        } else {
            DamageModifierResponse::default()
        }
    }
}
