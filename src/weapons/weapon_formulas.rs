use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

use crate::{types::rs_types::{StatQuadraticFormula, RangeFormula, HandlingFormula, ReloadFormula, DamageMods, AmmoFormula}, d2_enums::{DamageType, AmmoType, WeaponType}, perks::Perk};

use super::{Weapon, FiringConfig};

const HANDLING_DATA: [&str; 9] =  [r#"{"ready":{"evpp":0.0,"vpp":0.0,"offset":0.0},"stow":{"evpp":0.0,"vpp":0.0,"offset":0.0},"ads":{"evpp":0.0,"vpp":0.0,"offset":0.0}}"#, r#"{"ready":{"vpp":-0.199386,"offset":33.3015},"stow":{"vpp":-0.153306,"offset":28.8991},"ads":{"evpp":0.0,"vpp":0.0,"offset":0.0}}"#, r#"{"ready":{"vpp":-0.002942857143,"offset":0.4782571429},"stow":{"vpp":-0.002952380952,"offset":0.5133809524},"ads":{"vpp":-0.001666666667,"offset":0.3316666667}}"#, r#"{"ready":{"evpp":0.0,"vpp":-0.001448069241,"offset":0.4990612517},"stow":{"evpp":0.0,"vpp":-0.002863515313,"offset":0.4445712383},"ads":{"evpp":0.0,"vpp":-0.001693741678,"offset":0.4112330226}}"#, r#"{"ready":{"vpp":-0.003998740554,"offset":0.6635944584},"stow":{"vpp":-0.003296509536,"offset":0.5463332134},"ads":{"vpp":-0.002139258726,"offset":0.528984167}}"#, r#"{"ready":{"vpp":-0.00285336856,"offset":0.540561867},"stow":{"vpp":-0.002941215324,"offset":0.527217745},"ads":{"vpp":-0.001693527081,"offset":0.4114236019}}"#, r#"{"ready":{"vpp":-0.003271255061,"offset":0.5388744939},"stow":{"vpp":-0.003388663968,"offset":0.5711336032},"ads":{"vpp":-0.00233805668,"offset":0.451194332}}"#, r#"{"ready":{"vpp":-0.002623944983,"offset":0.5079465458},"stow":{"vpp":-0.002083932479,"offset":0.4392525789},"ads":{"vpp":-0.00194998437,"offset":0.5021325414}}"#, r#"{"ready":{"vpp":-0.002376970528,"offset":0.4710178204},"stow":{"vpp":-0.002547978067,"offset":0.4481295408},"ads":{"vpp":-0.001873200822,"offset":0.3581576422}}"#];
const RANGE_DATA:    [&str; 20] = [r#"{"vpp_start":0.0,"vpp_end":0.0,"offset_start":0.0,"offset_end":0.0,"floor_percent":0.0,"fusion":false}"#, r#"{"vpp":0.0963,"base_min":11.87,"base_max":40.8,"scale":false,"floor_percent":0.5,"vpp_start":0.0,"vpp_end":0.0,"offset_start":0.0,"offset_end":0.0,"fusion":false}"#, r#"{"zrm":1.3,"zrm_tier":{},"vpp":0.0324,"base_min":10.56,"base_max":15.1,"scale":true,"vpp_start":0.0,"vpp_end":0.0,"offset_start":0.0,"offset_end":0.0,"floor_percent":0.0,"fusion":false}"#, r#"{"vpp_start":0.0877,"offset_start":16.83,"vpp_end":0.0352,"offset_end":29.67,"floor_percent":0.33,"fusion":false}"#, r#"{"vpp_start":0.102,"offset_start":18.65,"vpp_end":0.0205,"offset_end":32.8,"floor_percent":0.33,"fusion":false}"#, r#"{"vpp_start":0.0,"offset_start":19.3,"vpp_end":0.0,"offset_end":29.67,"floor_percent":0.33,"fusion":false}"#, r#"{"vpp_start":0.0,"offset_start":38.0,"vpp_end":0.0,"offset_end":70.0,"floor_percent":0.33,"fusion":false}"#, r#"{"vpp":0.072,"base_min":17.3,"base_max":40.4,"scale":false,"floor_percent":0.5,"vpp_start":0.0,"vpp_end":0.0,"offset_start":0.0,"offset_end":0.0,"fusion":false}"#, r#"{"vpp":0.1521,"base_min":30.89,"base_max":60.8,"scale":false,"floor_percent":0.5,"vpp_start":0.0,"vpp_end":0.0,"offset_start":0.0,"offset_end":0.0,"fusion":false}"#, r#"{"vpp":0.169,"base_min":29.2,"base_max":60.8,"scale":false,"floor_percent":0.5,"vpp_start":0.0,"vpp_end":0.0,"offset_start":0.0,"offset_end":0.0,"fusion":false}"#, r#"{"vpp_start":0.0294,"offset_start":3.77,"vpp_end":0.0,"offset_end":14.5,"floor_percent":0.001,"fusion":false}"#, r#"{"vpp":0,"base_min":9,"base_max":9,"scale":false,"vpp_start":0.0,"vpp_end":0.0,"offset_start":0.0,"offset_end":0.0,"floor_percent":0.0,"fusion":false}"#, r#"{"vpp_start":0.0295,"offset_start":5.77,"vpp_end":0.0,"offset_end":12.75,"floor_percent":0.001,"fusion":false}"#, r#"{"vpp_start":0.0,"offset_start":999.0,"vpp_end":0.0,"offset_end":999.9,"floor_percent":0.999,"fusion":false}"#, r#"{"vpp_start":0.093,"offset_start":7.8,"vpp_end":0.0,"offset_end":23.71,"floor_percent":0.5,"fusion":false}"#, r#"{"vpp":0.1247,"base_min":9.0835,"base_max":9.0835,"scale":false,"floor_percent":0.5,"vpp_start":0.0,"vpp_end":0.0,"offset_start":0.0,"offset_end":0.0,"fusion":false}"#, r#"{"vpp_start":0.0546,"offset_start":15.0,"vpp_end":0.198,"offset_end":30.33,"floor_percent":0.33,"fusion":false}"#, r#"{"vpp_start":0.1017,"offset_start":14.756,"vpp_end":0.0,"offset_end":35.9,"floor_percent":0.4,"fusion":false}"#, r#"{"vpp_start":0.0295,"offset_start":11.85,"vpp_end":0.0287,"offset_end":22.85,"floor_percent":0.5,"fusion":false}"#, r#"{"vpp_start":0.0,"offset_start":28.3,"vpp_end":0.0,"offset_end":43.2,"floor_percent":0.5,"fusion":false}"#];
const RELOAD_DATA:   [&str; 16] = [r#"{"evpp":0.0,"vpp":0.0,"offset":0.0,"ammo_percent":0.0}"#, r#"{"evpp":8.55689e-05,"vpp":-0.0242021,"offset":2.80673006666667,"ammo_percent":0.0}"#, r#"{"evpp":6.15281e-05,"vpp":-0.0198054,"offset":2.8285704,"ammo_percent":0.0}"#, r#"{"evpp":7.55233e-05,"vpp":-0.0248947,"offset":4.12880153333333,"ammo_percent":0.0}"#, r#"{"evpp":7.24199e-05,"vpp":-0.0216432,"offset":3.24104606666667,"ammo_percent":0.0}"#, r#"{"evpp":0.000129019,"vpp":-0.0363945,"offset":4.19575,"ammo_percent":0.71}"#, r#"{"evpp":5.88462e-05,"vpp":-0.0199884,"offset":2.87206463333,"ammo_percent":0.0}"#, r#"{"evpp":9.05351e-05,"vpp":-0.0305819,"offset":6.1219905,"ammo_percent":0.0}"#, r#"{"evpp":9.26208e-05,"vpp":-0.0256877,"offset":2.92627266666667,"ammo_percent":0.0}"#, r#"{"evpp":0.000103959,"vpp":-0.0252069,"offset":4.09182213333333,"ammo_percent":0.0}"#, r#"{"evpp":0.000102915,"vpp":-0.0276889,"offset":3.11797356666666,"ammo_percent":0.0}"#, r#"{"evpp":6.40462e-05,"vpp":-0.0141721,"offset":1.25061,"ammo_percent":0.0}"#, r#"{"evpp":6.74498e-05,"vpp":-0.0231542,"offset":3.8384,"ammo_percent":0.0}"#, r#"{"evpp":6.08642e-05,"vpp":-0.0191345,"offset":2.62769,"ammo_percent":0.0}"#, r#"{"evpp":2.38311e-05,"vpp":-0.0124553,"offset":2.14667245,"ammo_percent":0.0}"#, r#"{"evpp":2.38311e-05,"vpp":-0.0124553,"offset":2.64667245,"ammo_percent":0.0}"#];
const SCALAR_DATA:   [&str; 30] = [r#"{"pve":1.0,"vehicle":1.0,"miniboss":1.0,"boss":1.0,"elite":1.0,"minor":1.0,"champion":1.0}"#, r#"{"vehicle":1.15,"boss":1.15,"champion":1.2,"miniboss":1.2,"elite":1.3,"minor":1.3,"pve":1.0}"#, r#"{"vehicle":2.18,"boss":2.55,"champion":2.55,"miniboss":2.55,"elite":2.55,"minor":2.73,"pve":1.0}"#, r#"{"vehicle":2.63,"boss":2.55,"champion":2.55,"miniboss":2.55,"elite":2.55,"minor":2.73,"pve":1.0}"#, r#"{"vehicle":2.0,"boss":2.0,"champion":2.0,"miniboss":2.0,"elite":2.0,"minor":2.0,"pve":1.0}"#, r#"{"vehicle":1.13,"boss":1.58,"champion":2.36,"miniboss":2.36,"elite":2.36,"minor":3.13,"pve":1.0}"#, r#"{"vehicle":2.5,"boss":2.5,"champion":2.63,"miniboss":2.63,"elite":2.63,"minor":3.13,"pve":1.0}"#, r#"{"vehicle":2.25,"boss":2.25,"champion":2.36,"miniboss":2.36,"elite":2.36,"minor":3.13,"pve":1.0}"#, r#"{"vehicle":2.5,"boss":2.5,"champion":2.63,"miniboss":2.63,"elite":2.63,"minor":3.13,"pve":1.5}"#, r#"{"vehicle":2.27,"boss":2.27,"champion":2.38,"miniboss":2.38,"elite":2.36,"minor":3.13,"pve":1.0}"#, r#"{"vehicle":1.2,"boss":1.3,"champion":1.4,"miniboss":1.4,"elite":1.4,"minor":2.13,"pve":1.0}"#, r#"{"vehicle":1.81,"boss":1.81,"champion":1.92,"miniboss":1.92,"elite":1.92,"minor":2.03,"pve":1.0}"#, r#"{"vehicle":1.81,"boss":1.54,"champion":1.23,"miniboss":1.64,"elite":1.92,"minor":2.03,"pve":1.1}"#, r#"{"vehicle":1.81,"boss":1.54,"champion":1.64,"miniboss":1.64,"elite":1.92,"minor":2.03,"pve":1.1}"#, r#"{"vehicle":2.38,"boss":2.16,"champion":2.25,"miniboss":2.25,"elite":3.36,"minor":4.2,"pve":1.0}"#, r#"{"vehicle":0.9,"boss":1.0,"champion":1.05,"miniboss":1.05,"elite":1.05,"minor":1.48,"pve":1.0}"#, r#"{"vehicle":0.9,"boss":0.9,"champion":1.05,"miniboss":1.05,"elite":1.05,"minor":1.48,"pve":1.0}"#, r#"{"vehicle":4.7,"boss":4.7,"champion":4.7,"miniboss":5.0,"elite":1.05,"minor":6.0,"pve":0.75}"#, r#"{"vehicle":2.12,"boss":2.12,"champion":2.72,"miniboss":2.72,"elite":1.05,"minor":6.0,"pve":1.0}"#, r#"{"vehicle":0.94,"boss":0.94,"champion":0.94,"miniboss":0.94,"elite":1.05,"minor":6.0,"pve":1.0}"#, r#"{"vehicle":1.3,"boss":1.3,"champion":1.3,"miniboss":1.3,"elite":1.3,"minor":1.9,"pve":1.0}"#, r#"{"vehicle":1.3,"boss":1.3,"champion":1.3,"miniboss":1.3,"elite":1.3,"minor":1.9,"pve":1.0}"#, r#"{"vehicle":2.8,"boss":2.3,"champion":2.5,"miniboss":2.5,"elite":2.88,"minor":3.2,"pve":1.1}"#, r#"{"vehicle":2.8,"boss":2.3,"champion":2.5,"miniboss":2.5,"elite":2.88,"minor":3.2,"pve":1.23}"#, r#"{"vehicle":2.8,"boss":2.3,"champion":2.5,"miniboss":2.5,"elite":2.88,"minor":3.2,"pve":1.1}"#, r#"{"vehicle":1.12,"boss":1.2,"champion":1.3,"miniboss":1.3,"elite":1.55,"minor":2.55,"pve":1.0}"#, r#"{"vehicle":1.4,"boss":1.53,"champion":1.53,"miniboss":1.53,"elite":1.9,"minor":1.9,"pve":1.0}"#, r#"{"vehicle":3.0,"boss":3.0,"champion":3.0,"miniboss":3.0,"elite":3.0,"minor":4.5,"pve":1.0}"#, r#"{"vehicle":1.4,"boss":1.5,"champion":1.7,"miniboss":1.7,"elite":1.7,"minor":1.9,"pve":1.0}"#, r#"{"vehicle":1.4,"boss":1.55,"champion":1.55,"miniboss":1.55,"elite":1.8,"minor":1.8,"pve":1.0}"#];
const FIRING_DATA:   [&str; 66] = [r#"{"damage":1.0,"crit_mult":1.0,"burst_delay":0.0,"burst_size":0.0,"burst_duration":0.0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":13.4,"crit_mult":1.5,"burst_delay":0.083333,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":14.3,"crit_mult":1.6,"burst_delay":0.1,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":22.0,"crit_mult":1.6,"burst_delay":0.166667,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":20.0,"crit_mult":1.6,"burst_delay":0.133333,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":18.0,"crit_mult":1.7,"burst_delay":0.166667,"burst_size":4,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":15.0,"crit_mult":1.4,"burst_delay":0.133333,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":46.5,"crit_mult":1.5,"burst_delay":0.43321,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":50,"crit_mult":1.6,"burst_delay":0.5,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":19,"crit_mult":1.6,"burst_delay":0.14458,"burst_size":3,"burst_duration":0.2333,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":40,"crit_mult":1.5,"burst_delay":0.3333,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":38,"crit_mult":1.4,"burst_delay":0.26667,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":78,"crit_mult":1.6,"burst_delay":0.66667,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":42,"crit_mult":1.6,"burst_delay":0.4,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":131,"crit_mult":2.5,"burst_delay":0.3,"burst_size":1,"burst_duration":0.0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":165.03,"crit_mult":3.009,"burst_delay":0.533,"burst_size":1,"burst_duration":0.0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":726,"crit_mult":1.2,"burst_delay":1.033,"burst_size":1,"burst_duration":0.0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":18,"crit_mult":1.3,"burst_delay":0.06667,"burst_size":1,"burst_duration":0.0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":34,"crit_mult":1.4,"burst_delay":0.16667,"burst_size":1,"burst_duration":0.0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":30,"crit_mult":1.4,"burst_delay":0.13333,"burst_size":1,"burst_duration":0.0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":150,"crit_mult":1.0,"burst_delay":0.5,"burst_size":1,"burst_duration":0.0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":14.0,"crit_mult":1.7,"burst_delay":0.11111,"burst_size":3,"burst_duration":0.1667,"oneAmmoBurst":false,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":22.0,"crit_mult":1.65,"burst_delay":0.17647,"burst_size":3,"burst_duration":0.1667,"oneAmmoBurst":false,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":19.0,"crit_mult":1.7,"burst_delay":0.15385,"burst_size":3,"burst_duration":0.1667,"oneAmmoBurst":false,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":16.0,"crit_mult":1.65,"burst_delay":0.11111,"burst_size":3,"burst_duration":0.1667,"oneAmmoBurst":false,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":15.7,"crit_mult":1.46,"burst_delay":0.11321,"burst_size":5,"burst_duration":0.3,"oneAmmoBurst":false,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":15.5,"crit_mult":1.7,"burst_delay":0.13333,"burst_size":4,"burst_duration":0.2333,"oneAmmoBurst":false,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":23.0,"crit_mult":1.65,"burst_delay":0.23346,"burst_size":5,"burst_duration":0.1,"oneAmmoBurst":false,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":405,"crit_mult":1.0,"burst_delay":4.0,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":150,"crit_mult":1.0,"burst_delay":1.0,"burst_size":8,"burst_duration":0.0,"oneAmmoBurst":true,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":450,"crit_mult":1.0,"burst_delay":4.0,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":495,"crit_mult":1.0,"burst_delay":3.0,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":300,"crit_mult":1.0,"burst_delay":1.0,"burst_size":8,"burst_duration":0.7,"oneAmmoBurst":true,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":495,"crit_mult":1.0,"burst_delay":2.4,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":193,"crit_mult":1.0,"burst_delay":1.0,"burst_size":2,"burst_duration":0.3,"oneAmmoBurst":true,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":31.0,"crit_mult":1.7,"burst_delay":0.3,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":40.0,"crit_mult":1.8,"burst_delay":0.4,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":47.0,"crit_mult":1.7,"burst_delay":0.5,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":30.5,"crit_mult":1.7,"burst_delay":0.3333,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":27.5,"crit_mult":1.7,"burst_delay":0.23346,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":22.3,"crit_mult":1.1,"burst_delay":1.099908,"burst_size":12,"burst_duration":0,"oneAmmoBurst":true,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":35.0,"crit_mult":1.44,"burst_delay":0.1,"burst_size":5,"burst_duration":0.29,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":19.1,"crit_mult":1.1,"burst_delay":0.933126,"burst_size":12,"burst_duration":0,"oneAmmoBurst":true,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":151.0,"crit_mult":1.75,"burst_delay":0.933126,"burst_size":1,"burst_duration":0,"oneAmmoBurst":true,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":16.7,"crit_mult":1.1,"burst_delay":0.433213,"burst_size":12,"burst_duration":0,"oneAmmoBurst":true,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":16.7,"crit_mult":1.1,"burst_delay":0.166667,"burst_size":12,"burst_duration":0,"oneAmmoBurst":true,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":170,"crit_mult":4.5,"burst_delay":0.83333,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":90,"crit_mult":3.25,"burst_delay":0.43321,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":131,"crit_mult":2.9,"burst_delay":0.66667,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":158,"crit_mult":3.0,"burst_delay":0.83333,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":15.0,"crit_mult":1.44,"burst_delay":0.08333,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":11.2,"crit_mult":1.44,"burst_delay":0.06667,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":10.9,"crit_mult":1.65,"burst_delay":0.06667,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":17.0,"crit_mult":1.4,"burst_delay":0.1,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":18.7,"crit_mult":1.4,"burst_delay":0.1,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":130.97,"crit_mult":1.0,"burst_delay":1.09,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":105.77,"crit_mult":1.0,"burst_delay":0.75,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":146.05,"crit_mult":1.0,"burst_delay":1.33,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":12,"crit_mult":1.4,"burst_delay":0.0667,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":20,"crit_mult":1.6,"burst_delay":0.2,"burst_size":3,"burst_duration":0.1667,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":36,"crit_mult":1.4,"burst_delay":0.2,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":25,"crit_mult":1.4,"burst_delay":0.133333,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":40,"crit_mult":1.4,"burst_delay":0.233463,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":31,"crit_mult":1.4,"burst_delay":0.2,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":32,"crit_mult":1.4,"burst_delay":0.23331,"burst_size":2,"burst_duration":0.1333,"explosive":false,"charge":false,"one_ammo":false}"#, r#"{"damage":34,"crit_mult":2.0,"burst_delay":0.3,"burst_size":1,"burst_duration":0,"explosive":false,"charge":false,"one_ammo":false}"#];
const AMMO_DATA:     [&str; 18] = [r#"{"mag":{"evpp":0.0,"vpp":0.0,"offset":1.0},"reserve_id":0,"round_to":0}"#, r#"{"mag":{"evpp":0.0,"vpp":0.2,"offset":25.0},"reserve_id":0,"round_to":0}"#, r#"{"mag":{"evpp":0.0,"vpp":0.1,"offset":3.5},"reserve_id":0,"round_to":0}"#, r#"{"mag":{"evpp":0.0,"vpp":0.0,"offset":24},"reserve_id":0,"round_to":0}"#, r#"{"mag":{"evpp":0.0,"vpp":0.14166,"offset":4.25},"reserve_id":0,"round_to":0}"#, r#"{"mag":{"evpp":0.0,"vpp":0.1,"offset":3.5},"reserve_id":3174300811,"round_to":0}"#, r#"{"mag":{"evpp":0.0,"vpp":0.0293,"offset":3.2},"reserve_id":221,"round_to":0}"#, r#"{"mag":{"evpp":0.0,"vpp":0.7,"offset":45.0},"reserve_id":81,"round_to":0}"#, r#"{"mag":{"evpp":0.0,"vpp":0.45,"offset":29.5},"reserve_id":82,"round_to":0}"#, r#"{"mag":{"evpp":0.0,"vpp":0.0,"offset":13},"reserve_id":2261491232,"round_to":0}"#, r#"{"mag":{"evpp":0.0,"vpp":0.0,"offset":20},"reserve_id":2940035732,"round_to":0}"#, r#"{"mag":{"evpp":0.0,"vpp":0.0,"offset":1},"reserve_id":101,"round_to":0}"#, r#"{"mag":{"evpp":0.0,"vpp":0.1,"offset":23.9},"reserve_id":0,"round_to":0}"#, r#"{"mag":{"evpp":0.0,"vpp":0.1,"offset":28.9},"reserve_id":0,"round_to":0}"#, r#"{"mag":{"evpp":0.0,"vpp":0.035,"offset":1.75},"reserve_id":331,"round_to":0}"#, r#"{"mag":{"evpp":0.0,"vpp":0.3,"offset":74.0},"reserve_id":251,"round_to":0}"#, r#"{"mag":{"evpp":0.0,"vpp":0.11942675159235666,"offset":5.289808917197453},"round_to":3,"reserve_id":17}"#, r#"{"mag":{"evpp":0.0,"vpp":0.0,"offset":12},"reserve_id":2984682260,"round_to":0}"#];

const WEAPON_PATHS: &str = r#"{"6":{"878286503":{"R":1,"H":1,"RL":1,"S":1,"F":1,"A":1},"944506345":{"R":1,"H":1,"RL":1,"S":1,"F":2,"A":2},"961505134":{"R":1,"H":1,"RL":1,"S":1,"F":1,"A":1},"1019291327":{"R":1,"H":1,"RL":1,"S":1,"F":3,"A":3},"1294026524":{"R":1,"H":1,"RL":1,"S":1,"F":2,"A":2},"1458010786":{"R":1,"H":1,"RL":1,"S":1,"F":2,"A":2},"1484442054":{"R":1,"H":1,"RL":1,"S":1,"F":2,"A":2},"1636108362":{"R":1,"H":1,"RL":1,"S":1,"F":4,"A":3},"3208839961":{"R":0,"H":1,"RL":1,"S":1,"F":5,"A":3},"3610814281":{"R":1,"H":1,"RL":1,"S":1,"F":2,"A":2},"3755070117":{"R":1,"H":1,"RL":1,"S":1,"F":6,"A":4},"3602718766":{"R":1,"H":1,"RL":1,"S":1,"F":1,"A":1}},"11":{"656200654":{"R":2,"H":1,"RL":2,"S":2},"878286503":{"R":2,"H":1,"RL":2,"S":2},"1019291327":{"R":2,"H":1,"RL":2,"S":2},"1186480754":{"R":2,"H":1,"RL":2,"S":2},"1294026524":{"R":2,"H":1,"RL":2,"S":2},"1636108362":{"R":2,"H":1,"RL":2,"S":2},"1656957541":{"R":0,"H":1,"RL":2,"S":2},"1657056865":{"R":0,"H":1,"RL":0,"S":3},"1927916065":{"R":0,"H":1,"RL":2,"S":2},"2518716062":{"R":2,"H":1,"RL":2,"S":2},"3610750208":{"R":1,"H":1,"RL":1,"S":4},"2585427437":{"R":2,"H":1,"RL":2,"S":2}},"23":{"389268985":{"R":0,"H":1,"RL":3,"S":5},"425960662":{"R":0,"H":1,"RL":4,"S":6},"474269988":{"R":0,"H":1,"RL":4,"S":6},"1294026524":{"R":0,"H":1,"RL":3,"S":7},"1315870387":{"R":0,"H":1,"RL":3,"S":7},"1395789926":{"R":0,"H":1,"RL":4,"S":8},"1458010786":{"R":0,"H":1,"RL":4,"S":6},"2353477480":{"R":0,"H":1,"RL":3,"S":7},"2977709078":{"R":0,"H":1,"RL":3,"S":7},"3063320916":{"R":0,"H":1,"RL":4,"S":6},"3913463509":{"R":0,"H":1,"RL":3,"S":7},"4130495068":{"R":0,"H":1,"RL":3,"S":7},"1174163613":{"R":0,"H":1,"RL":3,"S":9},"1759472859":{"R":0,"H":1,"RL":4,"S":6}},"9":{"213689231":{"R":3,"H":2,"RL":5,"S":10,"F":7,"A":5},"507151084":{"R":4,"H":2,"RL":5,"S":10,"F":8,"A":5},"647617635":{"R":3,"H":2,"RL":5,"S":10,"F":7,"A":5},"1030990989":{"R":3,"H":2,"RL":5,"S":10,"F":9,"A":6},"1294026524":{"R":3,"H":2,"RL":5,"S":10,"F":7,"A":5},"1322370662":{"R":3,"H":2,"RL":5,"S":10,"F":10,"A":7},"1791592647":{"R":3,"H":2,"RL":5,"S":10,"F":10,"A":7},"1863355414":{"R":3,"H":2,"RL":5,"S":10,"F":7,"A":5},"2144092201":{"R":3,"H":2,"RL":5,"S":10,"F":7,"A":5},"2189829540":{"R":3,"H":2,"RL":5,"S":10,"F":7,"A":5},"2757685314":{"R":4,"H":2,"RL":5,"S":10,"F":8,"A":5},"2770223582":{"R":5,"H":2,"RL":5,"S":10,"F":11,"A":5},"3174300811":{"R":6,"H":2,"RL":5,"S":10,"F":12,"A":8},"3468089894":{"R":4,"H":2,"RL":5,"S":10,"F":8,"A":5},"3923638944":{"R":3,"H":2,"RL":5,"S":10,"F":8,"A":5},"4045839491":{"R":4,"H":2,"RL":5,"S":10,"F":13,"A":5}},"22":{"1531126198":{"R":0,"H":3,"RL":6,"S":11,"F":14,"A":9},"2057203855":{"R":0,"H":3,"RL":6,"S":11,"F":15,"A":9},"2564164194":{"R":0,"H":3,"RL":6,"S":12,"F":15,"A":9},"3884127242":{"R":0,"H":3,"RL":6,"S":11,"F":16,"A":9},"2881100038":{"R":0,"H":3,"RL":6,"S":13,"F":15,"A":9}},"8":{"878286503":{"R":0,"H":1,"RL":7,"S":14,"F":17,"A":10},"1019291327":{"R":0,"H":1,"RL":7,"S":14,"F":18,"A":11},"1294026524":{"R":0,"H":1,"RL":7,"S":14,"F":19,"A":11},"2261491232":{"R":0,"H":1,"RL":7,"S":14,"F":20,"A":12},"2608508147":{"R":0,"H":1,"RL":7,"S":14,"F":19,"A":10},"4148158229":{"R":0,"H":1,"RL":7,"S":14,"F":19,"A":11},"2940035732":{"R":0,"H":1,"RL":7,"S":14,"F":20,"A":13}},"13":{"878286503":{"R":7,"H":1,"RL":8,"S":15,"F":21,"A":14},"1019291327":{"R":7,"H":1,"RL":8,"S":15,"F":22,"A":14},"1294026524":{"R":7,"H":1,"RL":8,"S":15,"F":23,"A":14},"1458010786":{"R":7,"H":1,"RL":8,"S":15,"F":24,"A":14},"2307143135":{"R":7,"H":1,"RL":8,"S":15,"F":25,"A":14},"2874284214":{"R":7,"H":1,"RL":8,"S":16,"F":26,"A":14},"3837077246":{"R":7,"H":1,"RL":8,"S":16,"F":26,"A":14},"3905543891":{"R":7,"H":1,"RL":8,"S":15,"F":27,"A":14},"4004944400":{"R":7,"H":1,"RL":8,"S":15,"F":24,"A":14},"4208418110":{"R":7,"H":1,"RL":8,"S":15,"F":24,"A":14},"4172222323":{"R":7,"H":1,"RL":8,"S":15,"F":24,"A":14},"3441203855":{"R":7,"H":1,"RL":8,"S":15,"F":23,"A":14}},"10":{"216781713":{"R":0,"H":4,"RL":9,"S":17,"F":28,"A":15},"411799453":{"R":0,"H":4,"RL":9,"S":17,"F":29,"A":15},"1019291327":{"R":0,"H":4,"RL":9,"S":17,"F":30,"A":15},"1294026524":{"R":0,"H":4,"RL":9,"S":17,"F":31,"A":15},"2200569208":{"R":0,"H":4,"RL":9,"S":18,"F":32,"A":15},"2473404935":{"R":0,"H":4,"RL":9,"S":19,"F":32,"A":15},"3419274965":{"R":0,"H":4,"RL":9,"S":17,"F":28,"A":15},"3468089894":{"R":0,"H":4,"RL":9,"S":17,"F":33,"A":15},"3649430342":{"R":0,"H":4,"RL":9,"S":17,"F":34,"A":15},"2962361451":{"R":0,"H":4,"RL":9,"S":17,"F":31,"A":15}},"14":{"377257911":{"R":8,"H":5,"RL":10,"S":20,"F":35,"A":16},"1000724343":{"R":8,"H":5,"RL":10,"S":20,"F":36,"A":16},"1019291327":{"R":8,"H":5,"RL":10,"S":20,"F":36,"A":16},"1319823571":{"R":9,"H":5,"RL":0,"S":21,"F":37,"A":16},"1458010786":{"R":8,"H":5,"RL":10,"S":20,"F":35,"A":16},"1636108362":{"R":8,"H":5,"RL":10,"S":20,"F":38,"A":16},"2741975068":{"R":8,"H":5,"RL":10,"S":20,"F":36,"A":16},"3364911712":{"R":8,"H":5,"RL":10,"S":20,"F":39,"A":16},"3668782036":{"R":8,"H":5,"RL":10,"S":20,"F":36,"A":16},"3920852688":{"R":8,"H":5,"RL":10,"S":20,"F":39,"A":16},"4185339856":{"R":8,"H":5,"RL":10,"S":20,"F":39,"A":16},"3468089894":{"R":9,"H":5,"RL":0,"S":20,"F":37,"A":16},"2724693746":{"R":8,"H":5,"RL":10,"S":20,"F":39,"A":16}},"7":{"372430833":{"R":10,"H":6,"RL":11,"S":22,"F":40,"A":17},"481338655":{"R":11,"H":6,"RL":11,"S":22,"F":41,"A":17},"895140517":{"R":10,"H":6,"RL":11,"S":22,"F":42,"A":17},"536517534":{"R":0,"H":6,"RL":11,"S":23,"F":43,"A":17},"918679156":{"R":12,"H":6,"RL":11,"S":23,"F":43,"A":17},"996573084":{"R":10,"H":6,"RL":11,"S":22,"F":44,"A":17},"1210807262":{"R":0,"H":6,"RL":11,"S":24},"1394384862":{"R":12,"H":6,"RL":11,"S":23,"F":43,"A":17},"1458010786":{"R":10,"H":6,"RL":11,"S":22},"1636108362":{"R":10,"H":6,"RL":11,"S":22,"F":42,"A":17},"2223914385":{"R":10,"H":6,"RL":11,"S":22,"F":45,"A":17},"3054949324":{"R":10,"H":6,"RL":11,"S":22,"F":40,"A":17},"3468089894":{"R":10,"H":6,"RL":11,"S":22,"F":40,"A":17},"3983457027":{"R":10,"H":6,"RL":11,"S":22,"F":40,"A":17}},"12":{"281315705":{"R":13,"H":7,"RL":12,"S":25,"F":46},"878286503":{"R":13,"H":7,"RL":12,"S":25,"F":47},"938999636":{"R":13,"H":7,"RL":12,"S":25,"F":47},"1070100196":{"R":13,"H":7,"RL":12,"S":25,"F":48},"1294026524":{"R":13,"H":7,"RL":12,"S":25,"F":48},"2909403175":{"R":13,"H":7,"RL":12,"S":25,"F":47},"3081173348":{"R":13,"H":7,"RL":12,"S":25,"F":49},"3468089894":{"R":13,"H":7,"RL":12,"S":25,"F":49}},"24":{"630329983":{"R":14,"H":8,"RL":13,"S":26,"F":50,"A":18},"1294026524":{"R":14,"H":8,"RL":13,"S":26,"F":51,"A":19},"1458010786":{"R":14,"H":8,"RL":13,"S":26,"F":52,"A":19},"1525239159":{"R":14,"H":8,"RL":13,"S":26,"F":50,"A":18},"1636108362":{"R":14,"H":8,"RL":13,"S":26,"F":53,"A":18},"2213377102":{"R":14,"H":8,"RL":13,"S":26,"F":52,"A":19},"2516532331":{"R":14,"H":8,"RL":13,"S":26,"F":51,"A":19},"2540536653":{"R":14,"H":8,"RL":13,"S":26,"F":50,"A":18},"3468089894":{"R":14,"H":8,"RL":13,"S":26,"F":50,"A":18},"2965975126":{"R":15,"H":1,"RL":13,"S":26,"F":54,"A":19},"228577175":{"R":14,"H":8,"RL":13,"S":26,"F":52,"A":19}},"33":{"1900919151":{"R":16,"H":1,"RL":0,"S":27,"F":55,"A":20},"3551884421":{"R":16,"H":1,"RL":0,"S":27,"F":56,"A":20},"3024740338":{"R":16,"H":1,"RL":0,"S":27,"F":57,"A":20},"131675355":{"R":16,"H":1,"RL":0,"S":27,"F":55,"A":20},"1986105578":{"R":16,"H":1,"RL":0,"S":27,"F":57,"A":20}},"25":{"459441288":{"R":17,"H":1,"RL":0,"S":28,"F":58,"A":21},"571267712":{"R":17,"H":1,"RL":0,"S":28,"F":58,"A":21},"1036269296":{"R":17,"H":1,"RL":0,"S":28,"F":58,"A":21},"1657401727":{"R":17,"H":1,"RL":0,"S":28,"F":58,"A":21},"1797707170":{"R":17,"H":1,"RL":0,"S":28,"F":58,"A":21},"1294026524":{"R":17,"H":1,"RL":0,"S":28,"F":58,"A":21},"3164944314":{"R":17,"H":1,"RL":0,"S":28,"F":58,"A":21}},"17":{"31057037":{"R":18,"H":1,"RL":14,"S":29,"F":59,"A":22},"334466122":{"R":18,"H":1,"RL":14,"S":29,"F":60,"A":22},"806997698":{"R":18,"H":1,"RL":14,"S":29,"F":61,"A":22},"975429949":{"R":18,"H":1,"RL":14,"S":29,"F":62,"A":22},"986191425":{"R":18,"H":1,"RL":14,"S":29,"F":62,"A":22},"1282254042":{"R":18,"H":1,"RL":14,"S":29,"F":62,"A":22},"1294026524":{"R":18,"H":1,"RL":14,"S":29,"F":60,"A":22},"1458010786":{"R":18,"H":1,"RL":14,"S":29,"F":63,"A":22},"1636108362":{"R":18,"H":1,"RL":14,"S":29,"F":62,"A":22},"2121086290":{"R":18,"H":1,"RL":14,"S":29,"F":60,"A":22},"3330548924":{"R":18,"H":1,"RL":14,"S":29,"F":64,"A":22},"3449390870":{"R":18,"H":1,"RL":14,"S":29,"F":60,"A":22},"2641107734":{"R":18,"H":1,"RL":14,"S":29,"F":59,"A":22},"2984682260":{"R":19,"H":1,"RL":15,"S":29,"F":65,"A":23}}}"#;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct InterRangeFormula {
    pub vpp_start: f64,
    pub vpp_end: f64,
    pub offset_start: f64,
    pub offset_end: f64,
    pub floor_percent: f64,
    pub fusion: bool,
}
impl Into<RangeFormula> for InterRangeFormula {
    fn into(self) -> RangeFormula {
        RangeFormula {
            start: StatQuadraticFormula{evpp:0.0, vpp: self.vpp_start, offset: self.offset_start},
            end: StatQuadraticFormula{evpp: 0.0, vpp: self.vpp_end, offset: self.offset_end},
            floor_percent: self.floor_percent,
            is_fusion: self.fusion
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct InterReloadFormula {
    pub evpp: f64,
    pub vpp: f64,
    pub offset: f64,
    pub ammo_percent: f64,
}
impl Into<ReloadFormula> for InterReloadFormula {
    fn into(self) -> ReloadFormula {
        ReloadFormula {
            reload_data: StatQuadraticFormula{ evpp: self.evpp, vpp: self.vpp, offset: self.offset},
            ammo_percent: self.ammo_percent,
        }
    }
}

impl Weapon {
    pub fn generate_weapon(_hash: u32, _weapon_type_id: u32, _intrinsic_hash: u32, _ammo_type_id: u32, _damage_type_id: u32) -> Result<Weapon, ()> {
        let jdata: Value = serde_json::from_str(WEAPON_PATHS).unwrap();
        
        let formula_map = jdata[_weapon_type_id.to_string()][_intrinsic_hash.to_string()].clone();
        if formula_map == Value::Null {
            return Err(())
        };

        let range_formula_id = formula_map["R"].as_i64().unwrap_or(0) as usize;
        let range_formula_str = RANGE_DATA.get(range_formula_id).unwrap();
        let range_formula: InterRangeFormula = serde_json::from_str(range_formula_str).unwrap();

        let handling_formula_id = formula_map["H"].as_i64().unwrap_or(0) as usize;
        let handling_formula_str = HANDLING_DATA.get(handling_formula_id).unwrap();
        let handling_formula: HandlingFormula = serde_json::from_str(handling_formula_str).unwrap();

        let reload_formula_id = formula_map["RL"].as_i64().unwrap_or(0) as usize;
        let reload_formula_str = RELOAD_DATA.get(reload_formula_id).unwrap();
        let reload_formula: InterReloadFormula = serde_json::from_str(reload_formula_str).unwrap();

        let damage_mods_id = formula_map["S"].as_i64().unwrap_or(0) as usize;
        let damage_mods_str = SCALAR_DATA.get(damage_mods_id).unwrap();
        let damage_mods: DamageMods = serde_json::from_str(damage_mods_str).unwrap();

        let firing_data_id = formula_map["S"].as_i64().unwrap_or(0) as usize;
        let firing_data_str = FIRING_DATA.get(firing_data_id).unwrap();
        let firing_data: FiringConfig = serde_json::from_str(firing_data_str).unwrap();

        let ammo_formula_id = formula_map["S"].as_i64().unwrap_or(0) as usize;
        let ammo_formula_str = AMMO_DATA.get(ammo_formula_id).unwrap();
        let ammo_formula: AmmoFormula = serde_json::from_str(ammo_formula_str).unwrap();

        let weapon_type = WeaponType::from_u32(_weapon_type_id);
        let ammo_type = AmmoType::from_u32(_ammo_type_id);
        let damage_type = DamageType::from_u32(_damage_type_id);

        Ok(Weapon {
            is_pvp: false,
            hash: _hash,
            perks: HashMap::from([(_intrinsic_hash, Perk{stat_buffs:HashMap::new(), enhanced: false, value: 0, hash:_intrinsic_hash})]),
            stats: HashMap::new(),
            damage_mods,
            ammo_formula,
            firing_data,
            handling_formula,
            reload_formula: reload_formula.into(),
            range_formula: range_formula.into(),
            ammo_type,
            damage_type,
            weapon_type,
        })
    }
}