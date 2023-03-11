use crate::HashId;
use crate::weapons::Weapon;
use crate::abilities::Ability;




#[derive(Debug, Clone)]
pub struct Loadout {
    pub dark_weapon: Option<Weapon>,
    pub light_weapon: Option<Weapon>,
    pub power_weapon: Option<Weapon>,
    pub melee_ability: Option<Ability>,
    pub grenade_ability: Option<Ability>,
    pub super_ability: Option<Ability>,
    pub class_ability: Option<Ability>,
    pub armor_effects: Vec<(HashId, u32)>
}