use serde::Serialize;
use serde_json::{Map, Number, Value};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;

// use json_value_remove::Remove;
// extern crate json_value_remove;

// fn vec_to_string<T: Debug>(vec: Vec<T>) -> String {
//     return format!("vec!{:?}", vec);
// }

fn json_0_float() -> Value {
    Value::Number(Number::from_f64(0.0).unwrap())
}

fn json_1_float() -> Value {
    Value::Number(Number::from_f64(1.0).unwrap())
}

//these types reflect whats in src/types/rs_types.rs
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct StatQuadraticFormula {
    pub evpp: f64,
    pub vpp: f64,
    pub offset: f64,
}
impl From<&Map<String, Value>> for StatQuadraticFormula {
    fn from(_val: &Map<String, Value>) -> Self {
        StatQuadraticFormula {
            evpp: _val
                .get("evpp")
                .unwrap_or(&json_0_float())
                .as_f64()
                .unwrap_or_default(),
            vpp: _val
                .get("vpp")
                .unwrap_or(&json_0_float())
                .as_f64()
                .unwrap_or_default(),
            offset: _val
                .get("offset")
                .unwrap_or(&json_0_float())
                .as_f64()
                .unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DamageMods {
    pub pve: f64,
    pub minor: f64,
    pub elite: f64,
    pub miniboss: f64,
    pub champion: f64,
    pub boss: f64,
    pub vehicle: f64,
}
impl From<&Map<String, Value>> for DamageMods {
    fn from(_val: &Map<String, Value>) -> Self {
        DamageMods {
            pve: 1.0,
            minor: _val
                .get("minor")
                .unwrap_or(&json_1_float())
                .as_f64()
                .unwrap_or(1_f64),
            elite: _val
                .get("elite")
                .unwrap_or(&json_1_float())
                .as_f64()
                .unwrap_or(1_f64),
            miniboss: _val
                .get("miniboss")
                .unwrap_or(&json_1_float())
                .as_f64()
                .unwrap_or(1_f64),
            champion: _val
                .get("champion")
                .unwrap_or(&json_1_float())
                .as_f64()
                .unwrap_or(1_f64),
            boss: _val
                .get("boss")
                .unwrap_or(&json_1_float())
                .as_f64()
                .unwrap_or(1_f64),
            vehicle: _val
                .get("vehicle")
                .unwrap_or(&json_1_float())
                .as_f64()
                .unwrap_or(1_f64),
        }
    }
}
impl DamageMods {
    pub fn add_pve_mult(self, mult: f64) -> Self {
        // not super memory efficient but it works
        DamageMods {
            pve: mult,
            minor: self.minor,
            elite: self.elite,
            miniboss: self.miniboss,
            champion: self.champion,
            boss: self.boss,
            vehicle: self.vehicle,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RangeFormula {
    pub start: StatQuadraticFormula,
    pub end: StatQuadraticFormula,
    pub floor_percent: f64,
    pub fusion: bool,
}
impl From<&Map<String, Value>> for RangeFormula {
    fn from(_val: &Map<String, Value>) -> Self {
        RangeFormula {
            start: StatQuadraticFormula {
                evpp: 0.0,
                vpp: _val["vpp_start"].as_f64().unwrap_or_default(),
                offset: _val["offset_start"].as_f64().unwrap_or_default(),
            },
            end: StatQuadraticFormula {
                evpp: 0.0,
                vpp: _val["vpp_end"].as_f64().unwrap_or_default(),
                offset: _val["offset_end"].as_f64().unwrap_or_default(),
            },
            floor_percent: _val["floor_percent"].as_f64().unwrap_or_default(),
            fusion: _val
                .get("fusion")
                .unwrap_or(&Value::Bool(false))
                .as_bool()
                .unwrap_or(false),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ReloadFormula {
    pub reload_data: StatQuadraticFormula,
    pub ammo_percent: f64,
}
impl From<&Map<String, Value>> for ReloadFormula {
    fn from(_val: &Map<String, Value>) -> Self {
        ReloadFormula {
            reload_data: StatQuadraticFormula::from(&_val.clone()),
            ammo_percent: _val
                .get("ammo_percent")
                .unwrap_or(&json_0_float())
                .as_f64()
                .unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct HandlingFormula {
    pub ready: StatQuadraticFormula,
    pub stow: StatQuadraticFormula,
    pub ads: StatQuadraticFormula,
}
impl From<&Map<String, Value>> for HandlingFormula {
    fn from(_val: &Map<String, Value>) -> Self {
        HandlingFormula {
            ready: StatQuadraticFormula::from(_val["ready"].as_object().unwrap_or(&Map::new())),
            stow: StatQuadraticFormula::from(_val["stow"].as_object().unwrap_or(&Map::new())),
            ads: StatQuadraticFormula::from(_val["ads"].as_object().unwrap_or(&Map::new())),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct AmmoFormula {
    pub mag: StatQuadraticFormula,
    pub round_to: i32,
    pub reserve_id: u32,
}
impl From<&Map<String, Value>> for AmmoFormula {
    fn from(_val: &Map<String, Value>) -> Self {
        AmmoFormula {
            mag: StatQuadraticFormula::from(_val["mag"].as_object().unwrap_or(&Map::new())),
            round_to: _val
                .get("round_to")
                .unwrap_or(&Value::Null)
                .as_i64()
                .unwrap_or_default() as i32,
            reserve_id: _val
                .get("reserve_id")
                .unwrap_or(&Value::Null)
                .as_u64()
                .unwrap_or_default() as u32,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct FiringData {
    pub damage: f64,
    pub crit_mult: f64,
    pub burst_delay: f64,
    pub inner_burst_delay: f64,
    pub burst_size: i32,
    pub one_ammo: bool,
    pub charge: bool,
}
impl From<&Map<String, Value>> for FiringData {
    fn from(_val: &Map<String, Value>) -> Self {
        FiringData {
            damage: _val["damage"].as_f64().unwrap_or_default(),
            crit_mult: _val["crit_mult"].as_f64().unwrap_or_default(),
            burst_delay: _val["burst_delay"].as_f64().unwrap_or_default(),
            inner_burst_delay: _val["inner_burst_delay"].as_f64().unwrap_or_default(),
            burst_size: _val["burst_size"].as_i64().unwrap_or_default() as i32,
            one_ammo: _val
                .get("one_ammo")
                .unwrap_or(&Value::Bool(false))
                .as_bool()
                .unwrap_or(false),
            charge: _val
                .get("charge")
                .unwrap_or(&Value::Bool(false))
                .as_bool()
                .unwrap_or(false),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct DataPointers {
    h: usize,
    r: usize,
    rl: usize,
    s: usize,
    f: usize,
    a: usize,
}

fn write_variable(
    writer: &mut std::fs::File,
    name: &str,
    datatype: &str,
    value: String,
    doc: &str,
) {
    let res = writeln!(
        writer,
        "#[doc=r#\"{}\"#]\n#[allow(dead_code)]\npub const {}: {} = {};",
        doc, name, datatype, value
    );
    if res.is_err() {
        println!("cargo:warning=error writing variable");
    }
}

fn main() {
    let mut opts = built::Options::default();
    opts.set_dependencies(true);

    let src = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let built_dst = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("built.rs");
    let formula_dst = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("formulas.rs");
    println!(
        "cargo:warning=OUT_DIR: {}",
        &std::env::var("OUT_DIR").unwrap()
    );
    built::write_built_file_with_opts(&opts, src.as_ref(), &built_dst)
        .expect("Failed to acquire build-time information");
    let mut formula_file = std::fs::File::create(formula_dst).unwrap();
    // checking to see if we have internet connection
    let ping = reqwest::blocking::get("https://www.bungie.net");
    if ping.is_ok() {
        let status = ping.unwrap().status();
        if status == reqwest::StatusCode::OK {
            construct_enhance_perk_mapping(&mut formula_file);
        }
    } else {
        panic!("no internet connection :(");
    }
    fn construct_enhance_perk_mapping(formula_file: &mut File) {
        let mut perk_mappings: Vec<(u32, u32)> = Vec::new();
        let json_file = reqwest::blocking::get("https://raw.githubusercontent.com/DestinyItemManager/d2-additional-info/master/output/trait-to-enhanced-trait.json");
        if json_file.is_ok() {
            let json_file = json_file.unwrap();
            let dct = json_file.json::<HashMap<String, u32>>();
            if dct.is_ok() {
                let dct = dct.unwrap();
                for i in dct {
                    perk_mappings.push((i.1, i.0.parse::<u32>().unwrap()));
                }
            } else {
                println!("cargo:warning=dim enhanced mapping could not be parsed");
                return;
            }
        } else {
            println!("cargo:warning=dim enhanced mapping not found");
            return;
        }
        let intrinsic_map: HashMap<u32, Vec<&str>> = HashMap::from([
            (901, vec!["High-Impact Frame"]),
            (902, vec!["VEIST Rapid-Fire", "Rapid-Fire Frame"]),
            (903, vec!["Adaptive Frame", "Adaptive Glaive"]),
            (904, vec!["Aggressive Frame", "Aggressive Glaive"]),
            (905, vec!["Lightweight Frame", "MIDA Synergy"]),
            (906, vec!["Precision Frame", "Häkke Precision Frame"]),
            (907, vec!["Double Fire"]),
            (908, vec!["Wave Frame"]),
            (911, vec!["Legacy PR-55 Frame"]),
        ]);
        let manifest_raw =
            reqwest::blocking::get("https://www.bungie.net/Platform/Destiny2/Manifest/");
        let manifest_text: String;
        if manifest_raw.is_ok() {
            manifest_text = manifest_raw.unwrap().text().unwrap();
        } else {
            println!("cargo:warning=manifest not found");
            return;
        }
        let manifest_json: Value = serde_json::from_str(&manifest_text).unwrap();
        let content_paths = manifest_json["Response"]["jsonWorldComponentContentPaths"]["en"]
            .as_object()
            .unwrap();
        let item_data_raw = reqwest::blocking::get(format!(
            "https://www.bungie.net{}",
            content_paths["DestinyInventoryItemDefinition"]
                .as_str()
                .unwrap()
        ));
        let item_data_json: Value =
            serde_json::from_str(&item_data_raw.unwrap().text().unwrap()).unwrap();
        for (key, value) in item_data_json.as_object().unwrap() {
            let hash = key.parse::<u32>().unwrap();
            //does value have a key called itemTypeDisplayName?
            if !value
                .as_object()
                .unwrap()
                .contains_key("itemTypeDisplayName")
            {
                continue;
            }
            if !value["itemTypeDisplayName"]
                .as_str()
                .unwrap()
                .contains("Intrinsic")
            {
                continue;
            }
            let name = value["displayProperties"]["name"].as_str().unwrap();
            for (id, names) in intrinsic_map.iter() {
                if names.contains(&name) {
                    perk_mappings.push((hash, *id));
                }
            }
        }
        write_variable(
            formula_file,
            "ENHANCE_PERK_MAPPING",
            &format!("[(u32, u32); {}]", perk_mappings.len()),
            format!("{:?}", perk_mappings),
            "Mapping of enhanced perks and intrinsics to their base perk/intrinsic",
        );
    }
    construct_weapon_formulas(&mut formula_file);
    fn construct_weapon_formulas(formula_file: &mut File) {
        let id_to_name = HashMap::from([
            (6, "Auto Rifle"),
            (31, "Combat Bow"),
            (11, "Fusion Rifle"),
            (23, "Grenade Launcher"),
            (9, "Hand Cannon"),
            (22, "Linear Fusion Rifle"),
            (8, "Machine Gun"),
            (13, "Pulse Rifle"),
            (10, "Rocket Launcher"),
            (14, "Scout Rifle"),
            (7, "Shotgun"),
            (12, "Sniper Rifle"),
            (24, "Submachine Gun"),
            (33, "Glaive"),
            (25, "Trace Rifle"),
            (17, "Sidearm"),
        ]);

        //get current directory
        let jdata_path = std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("database/weapon_formulas_editable.json");
        let mut jdata: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(jdata_path).unwrap()).unwrap();
        // remove "COMMENTS" from jdata
        let res = jdata.get("COMMENTS");
        if res.is_none() {
            println!("cargo:warning=comments not found");
            return;
        } else {
            jdata.as_object_mut().unwrap().remove("COMMENTS");
        }

        let mut new_jdata: Value = Value::Object(Map::new());
        let index_data: Value = jdata["INDEX"].take();
        for (key, value) in index_data.as_object().unwrap() {
            let tmp_data_get = jdata[value.as_str().unwrap()].take();
            let mut tmp_data_set = tmp_data_get.as_object().unwrap().clone();
            for (key, value) in tmp_data_get.as_object().unwrap() {
                if key.parse::<u32>().is_ok() {
                    let mut tmp_value = value.as_object().unwrap().clone();
                    tmp_value.remove("name");
                    tmp_data_set[key] = Value::Object(tmp_value);
                }
            }
            new_jdata[key] = Value::Object(tmp_data_set);
        }

        let mut handling_data: Vec<HandlingFormula> = Vec::new();
        let mut range_data: Vec<RangeFormula> = Vec::new();
        let mut reload_data: Vec<ReloadFormula> = Vec::new();
        let mut ammo_data: Vec<AmmoFormula> = Vec::new();
        let mut firing_data: Vec<FiringData> = Vec::new();
        let mut scalar_data: Vec<DamageMods> = Vec::new();

        let mut updated_weapon_defs: Vec<(u32, DataPointers)> = Vec::new();
        for (weapon_id, inner_values) in new_jdata.as_object().unwrap() {
            for (weapon_hash, weapon_def) in inner_values.as_object().unwrap() {
                let mut data = DataPointers::default();
                if weapon_hash.parse::<u32>().is_err() {
                    continue;
                }
                let mut set_data = |val: Value| -> Result<(), Vec<String>> {
                    let mut err_list: Vec<String> = Vec::new();
                    let cat: Map<String, Value>;
                    if val.get("cat").is_some() && weapon_def.get("cat").is_some() {
                        if weapon_def.get("cat") == Some(&Value::Null) {
                            err_list.push(format!(
                                "category not found for {} :> {}",
                                id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                                weapon_hash
                            ));
                            cat = Map::new();
                        } else {
                            cat = val["cat"][weapon_def["cat"].as_str().unwrap()]
                                .as_object()
                                .unwrap()
                                .clone();
                        }
                    } else {
                        err_list.push(format!(
                            "category not found for {} :> {}",
                            id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                            weapon_hash
                        ));
                        cat = Map::new();
                    }
                    let mag: Map<String, Value>;
                    if val.get("magProf").is_some() && weapon_def.get("magProf").is_some() {
                        if weapon_def.get("magProf") == Some(&Value::Null) {
                            err_list.push(format!(
                                "magazine profile not found for {} :> {}",
                                id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                                weapon_hash
                            ));
                            mag = Map::new();
                        } else {
                            mag = val["magProf"][weapon_def["magProf"].as_str().unwrap()]
                                .as_object()
                                .unwrap()
                                .clone();
                        }
                    } else {
                        err_list.push(format!(
                            "magazine profile not found for {} :> {}",
                            id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                            weapon_hash
                        ));
                        mag = Map::new();
                    }
                    let fam: Map<String, Value>;
                    if val.get("subFam").is_some() && weapon_def.get("subFam").is_some() {
                        if weapon_def.get("subFam") == Some(&Value::Null) {
                            err_list.push(format!(
                                "subfamily not found for {} :> {}",
                                id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                                weapon_hash
                            ));
                            fam = Map::new();
                        } else {
                            fam = val["subFam"][weapon_def["subFam"].as_str().unwrap()]
                                .as_object()
                                .unwrap()
                                .clone();
                        }
                    } else {
                        err_list.push(format!(
                            "subfamily not found for {} :> {}",
                            id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                            weapon_hash
                        ));
                        fam = Map::new();
                    }
                    let empty_object = Value::Object(Map::new());

                    let range: RangeFormula = cat
                        .get("range")
                        .unwrap_or_else(|| {
                            err_list.push(format!(
                                "range not found for {} :> {}",
                                id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                                weapon_hash
                            ));
                            &empty_object
                        })
                        .as_object()
                        .unwrap()
                        .into();
                    if range_data.contains(&range) {
                        data.r = range_data.iter().position(|x| x == &range).unwrap();
                    } else {
                        data.r = range_data.len();
                        range_data.push(range);
                    }

                    let handling: HandlingFormula = cat
                        .get("handling")
                        .unwrap_or_else(|| {
                            err_list.push(format!(
                                "handling not found for {} :> {}",
                                id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                                weapon_hash
                            ));
                            &empty_object
                        })
                        .as_object()
                        .unwrap()
                        .into();
                    if handling_data.contains(&handling) {
                        data.h = handling_data.iter().position(|x| x == &handling).unwrap();
                    } else {
                        data.h = handling_data.len();
                        handling_data.push(handling);
                    }

                    let reload: ReloadFormula = cat
                        .get("reload")
                        .unwrap_or_else(|| {
                            err_list.push(format!(
                                "reload not found for {} :> {}",
                                id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                                weapon_hash
                            ));
                            &empty_object
                        })
                        .as_object()
                        .unwrap()
                        .into();
                    if reload_data.contains(&reload) {
                        data.rl = reload_data.iter().position(|x| x == &reload).unwrap();
                    } else {
                        data.rl = reload_data.len();
                        reload_data.push(reload);
                    }

                    let b_scalar: DamageMods = cat
                        .get("combatant_scalars")
                        .unwrap_or_else(|| {
                            err_list.push(format!(
                                "combatant_scalars not found for {} :> {}",
                                id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                                weapon_hash
                            ));
                            &empty_object
                        })
                        .as_object()
                        .unwrap()
                        .into();
                    let scalar = b_scalar.add_pve_mult(
                        weapon_def
                            .get("pve_mult")
                            .unwrap_or(&json_1_float())
                            .as_f64()
                            .unwrap_or(1.0),
                    );
                    if scalar_data.contains(&scalar) {
                        data.s = scalar_data.iter().position(|x| x == &scalar).unwrap();
                    } else {
                        data.s = scalar_data.len();
                        scalar_data.push(scalar);
                    }

                    let ammo: AmmoFormula = (&mag).into();
                    if ammo_data.contains(&ammo) {
                        data.a = ammo_data.iter().position(|x| x == &ammo).unwrap();
                    } else {
                        data.a = ammo_data.len();
                        ammo_data.push(ammo);
                    }

                    let firing: FiringData = (&fam).into();
                    if firing_data.contains(&firing) {
                        data.f = firing_data.iter().position(|x| x == &firing).unwrap();
                    } else {
                        data.f = firing_data.len();
                        firing_data.push(firing);
                    }

                    if err_list.len() > 0 {
                        return Err(err_list);
                    }
                    return Ok(());
                };
                let set_data_res = set_data(inner_values.clone());
                if set_data_res.is_err() {
                    println!("cargo:warning={:?}", set_data_res.unwrap_err());
                }
                updated_weapon_defs.push((weapon_hash.parse::<u32>().unwrap(), data));
            }
        }
        // println!("cargo:warning=Finished parsing weapon definitions {:?}", updated_weapon_defs);
        write_variable(
            formula_file,
            "DATA_POINTERS",
            &format!("[(u32, DataPointers); {}]", updated_weapon_defs.len()),
            format!("{:?}", updated_weapon_defs),
            "Hashmapping for weapon intrinsic hash to data pointers",
        );
        write_variable(
            formula_file,
            "RANGE_DATA",
            &format!("[RangeFormula; {}]", range_data.len()),
            format!("{:?}", range_data),
            "Array of range formulas",
        );
        write_variable(
            formula_file,
            "HANDLING_DATA",
            &format!("[HandlingFormula; {}]", handling_data.len()),
            format!("{:?}", handling_data),
            "Array of handling formulas",
        );
        write_variable(
            formula_file,
            "RELOAD_DATA",
            &format!("[ReloadFormula; {}]", reload_data.len()),
            format!("{:?}", reload_data),
            "Array of reload formulas",
        );
        write_variable(
            formula_file,
            "SCALAR_DATA",
            &format!("[DamageMods; {}]", scalar_data.len()),
            format!("{:?}", scalar_data),
            "Array of combatant scalar formulas",
        );
        write_variable(
            formula_file,
            "FIRING_DATA",
            &format!("[FiringData; {}]", firing_data.len()),
            format!("{:?}", firing_data),
            "Array of firing data formulas",
        );
        write_variable(
            formula_file,
            "AMMO_DATA",
            &format!("[AmmoFormula; {}]", ammo_data.len()),
            format!("{:?}", ammo_data),
            "Array of ammo formulas",
        );
    }
}
