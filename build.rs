use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

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
    println!("cargo:warning=OUT_DIR: {}", &std::env::var("OUT_DIR").unwrap());
    built::write_built_file_with_opts(&opts, src.as_ref(), &built_dst)
        .expect("Failed to acquire build-time information");
    let mut formula_file = std::fs::File::create(formula_dst).unwrap();
    // checking to see if we have internet connection
    let ping = reqwest::blocking::get("https://github.com");
    if ping.is_ok() {
        let status = ping.unwrap().status();
        if status == reqwest::StatusCode::OK {
            construct_enhance_perk_mapping(&mut formula_file);
        }
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
        let intrinsic_map: HashMap<u32, Vec<&str>> = HashMap::from(
            [
                (901, vec!["High-Impact Frame"]),
                (902, vec!["VEIST Rapid-Fire", "Rapid-Fire Frame"]),
                (903, vec!["Adaptive Frame", "Adaptive Glaive"]),
                (904, vec!["Aggressive Frame", "Aggressive Glaive"]),
                (905, vec!["Lightweight Frame", "MIDA Synergy"]),
                (906, vec!["Precision Frame", "Häkke Precision Frame"]),
                (907, vec!["Double Fire"]),
                (911, vec!["Legacy PR-55 Frame"]),
            ]
        );
        let manifest_raw = reqwest::blocking::get("https://www.bungie.net/Platform/Destiny2/Manifest/");
        let manifest_text: String;
        if manifest_raw.is_ok() {
            manifest_text = manifest_raw.unwrap().text().unwrap();
        } else {
            println!("cargo:warning=manifest not found");
            return;
        }
        let manifest_json: serde_json::Value = serde_json::from_str(&manifest_text).unwrap();
        let content_paths = manifest_json["Response"]["jsonWorldComponentContentPaths"]["en"].as_object().unwrap();
        let item_data_raw = reqwest::blocking::get(format!("https://www.bungie.net{}", content_paths["DestinyInventoryItemDefinition"].as_str().unwrap()));
        let item_data_json: serde_json::Value = serde_json::from_str(&item_data_raw.unwrap().text().unwrap()).unwrap();
        for (key, value) in item_data_json.as_object().unwrap() {
            let hash = key.parse::<u32>().unwrap();
            //does value have a key called itemTypeDisplayName?
            if !value.as_object().unwrap().contains_key("itemTypeDisplayName") {
                continue;
            }
            if !value["itemTypeDisplayName"].as_str().unwrap().contains("Intrinsic")  {
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
            "Mapping of enhanced perks and intrinsics to their base perk/intrinsic"
        );
    }


    fn construct_weapon_formulas() {
        // id_to_name = {
        //     "6":  "Auto Rifle",
        //     "31": "Combat Bow",
        //     "11": "Fusion Rifle",
        //     "23": "Grenade Launcher",
        //     "9":  "Hand Cannon",
        //     "22": "Linear Fusion Rifle",
        //     "8":  "Machine Gun",
        //     "13": "Pulse Rifle",
        //     "10": "Rocket Launcher",
        //     "14": "Scout Rifle",
        //     "7":  "Shotgun",
        //     "12": "Sniper Rifle",
        //     "24": "Submachine Gun",
        //     "18": "Sword",
        //     "33": "Glaive",
        //     "25": "Trace Rifle",
        //     "17": "Sidearm"
        // }
        let id_to_name = HashMap::from(
            [
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
                (18, "Sword"),
                (33, "Glaive"),
                (25, "Trace Rifle"),
                (17, "Sidearm"),
            ]
        );

        
    }
}