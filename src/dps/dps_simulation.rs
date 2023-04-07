use serde::{Serialize, Deserialize};

use crate::{weapons::Weapon, activity::{self, Activity}, enemies::Enemy, perks::{get_dmg_modifier, lib::CalculationInput}};

use super::{
    instruction_set::{InstructionSet, WeaponSlot},
    Loadout, LoadoutDpsData, CachedWeaponDpsData,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum EventType {
    //firing
    FireKinetic,
    FireEnergy,
    FirePower,
    ExtraPerkDamage,
    DamageOverTime,
    //swapping
    SwapToKinetic,
    SwapToEnergy,
    SwapToPower,
    //abilities
    Grenade,
    Super,
    Melee,
    UnchargedMelee,
    ClassAbility,
    //reload
    Reload,
    ReloadCancel,
    ReloadOverride,
    //verbs
    Shatter,
    Ignition,
    Volatile,
    Unravel,
    Jolt,
    //misc
    Wait,
}

#[derive(Debug, Clone, Serialize)]
pub struct Event {
    pub pre_time: f64,
    pub damage: f64,
    pub event_type: EventType,
    pub post_time: f64,
}

#[derive(Debug, Clone)]
pub struct DpsSimulationInstance {
    pub loadout: Loadout,
    pub dps_data: LoadoutDpsData,
    pub activity: Activity,
    pub enemy: Enemy,
    pub active_weapon: WeaponSlot,
    pub total_time: f64,
    pub event_timeline: Vec<Event>,
    pub ads_time_in_swap: f64, //% of ads time to add to swap time
}
impl DpsSimulationInstance {
    pub fn new(loadout: Loadout, activity: Activity, enemy: Enemy) -> DpsSimulationInstance {
        DpsSimulationInstance {
            loadout,
            dps_data: LoadoutDpsData::new(),
            activity,
            enemy,
            active_weapon: WeaponSlot::Kinetic,
            total_time: 0.0,
            event_timeline: Vec::new(),
            ads_time_in_swap: 0.0,
        }
    }
    fn add_event(&mut self, event: Event) {
        self.total_time += event.pre_time+event.post_time;
        self.event_timeline.push(event);
    }
    fn calc_input_for(&self, slot: WeaponSlot) -> CalculationInput {
        //WARNING: can be unsafe if called for a weapon that is not in the loadout
        let mut weapon = match slot {
            WeaponSlot::Kinetic => self.loadout.kinetic_weapon.clone().unwrap(),
            WeaponSlot::Energy => self.loadout.energy_weapon.clone().unwrap(),
            WeaponSlot::Power => self.loadout.power_weapon.clone().unwrap(),
        };
        let cache = match slot {
            WeaponSlot::Kinetic => self.dps_data.kinetic.as_mut().unwrap(),
            WeaponSlot::Energy => self.dps_data.energy.as_mut().unwrap(),
            WeaponSlot::Power => self.dps_data.power.as_mut().unwrap(),
        };
        CalculationInput {
            intrinsic_hash: weapon.intrinsic_hash,
            ammo_type: &weapon.ammo_type,
            curr_mag: cache.magazine,
            base_mag: cache.base_magazine,
            base_crit_mult: weapon.firing_data.crit_mult,
            damage_type: &weapon.damage_type,
            enemy_type: &self.enemy.type_,
            stats: &weapon.get_stats(),
            has_overshield: false,
            num_reloads: cache.num_reloads as f64,
            perk_value_map: &weapon.perk_value_map,
            ammo_fired_this_mag: cache.ammo_fired_this_mag as f64,
            reserves_left: cache.reserves_left as f64,
            time_total: self.total_time,
            total_ammo_fired: cache.ammo_fired as f64,
            total_shots_hit: cache.bullets_fired as f64,
            weapon_type: &weapon.weapon_type,
        }
    }
    fn current_weapon(&self) -> Weapon {
        match self.active_weapon {
            WeaponSlot::Kinetic => self.loadout.kinetic_weapon.clone().unwrap(),
            WeaponSlot::Energy => self.loadout.energy_weapon.clone().unwrap(),
            WeaponSlot::Power => self.loadout.power_weapon.clone().unwrap(),
        }
    }
    fn current_weapon_cache(&mut self) -> &mut CachedWeaponDpsData {
        match self.active_weapon {
            WeaponSlot::Kinetic => self.dps_data.kinetic.as_mut().unwrap(),
            WeaponSlot::Energy => self.dps_data.energy.as_mut().unwrap(),
            WeaponSlot::Power => self.dps_data.power.as_mut().unwrap(),
        }
    }
    pub fn fire(&mut self) {
        let weapon = self.current_weapon();
        let mut cache = self.current_weapon_cache();
        if cache.magazine <= 0.0 { return; }
        let shoot_delay = (
            (cache.last_shot_time + cache.required_shot_delay)
            - self.total_time).min(0.0);
        let mods = get_dmg_modifier(
            weapon.list_perks(),
            &self.calc_input_for(self.active_weapon),
            false,
            &mut cache.untyped
        );
        let damage = weapon.firing_data.damage
            * self.activity.get_rpl_mult()
            * self.activity.get_pl_delta()
            * weapon.damage_mods.get_mod(&self.enemy.type_);
    }
    pub fn swap_weapon(&mut self, slot: WeaponSlot) {
        self.active_weapon = slot;
    }
    pub fn use_grenade(&mut self) {
        if !self.loadout.has_grenade_ability() {
        }
    }
    pub fn use_super(&mut self) {
        if !self.loadout.has_super_ability() {
        }
    }
    pub fn use_melee(&mut self) {
        if !self.loadout.has_melee_ability() {
        }
    }
    pub fn use_uncharged_melee(&mut self) {
    }
    pub fn use_class_ability(&mut self) {
        if !self.loadout.has_class_ability() {
        }
    }
    pub fn reload(&mut self) {
    }
    pub fn reload_cancel(&mut self) {
    }
    pub fn wait(&mut self, time: f64) {
        self.add_event(Event {
            pre_time: 0.0,
            damage: 0.0,
            event_type: EventType::Wait,
            post_time: time,
        });
    }
}
