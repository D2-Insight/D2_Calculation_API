use std::collections::HashMap;



fn main() {
    let mut opts = built::Options::default();
    opts.set_dependencies(true);

    let src = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dst = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("built.rs");
    println!("cargo:warning=OUT_DIR: {}", &std::env::var("OUT_DIR").unwrap());
    built::write_built_file_with_opts(&opts, src.as_ref(), &dst)
        .expect("Failed to acquire build-time information");

    // checking to see if we have internet connection
    let ping = reqwest::blocking::get("https://github.com");
    if ping.is_ok() {
        let status = ping.unwrap().status();
        if status == reqwest::StatusCode::OK {
            construct_enhance_perk_mapping();
        }
    }
    fn construct_enhance_perk_mapping() {
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
    }
}