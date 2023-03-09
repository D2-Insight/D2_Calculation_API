import json
import requests

API_ROOT = "https://www.bungie.net/Platform/Destiny2/"
CONTENT_ROOT = "https://www.bungie.net"
API_KEY = "89c9db2c0a8b46449bb5e654b6e594d0"  # no yoinky😡
API_KEY_HEADER = {"X-API-Key": ""}

json_file = requests.get("https://raw.githubusercontent.com/DestinyItemManager/d2-additional-info/master/output/trait-to-enhanced-trait.json")
dct = json.loads(json_file.text)
out_lst = []
for i in dct:
    out_lst.append((int(dct[i]), int(i)))


intrinsic_map = {
    901: ["High-Impact Frame"],
    902: ["VEIST Rapid-Fire", "Rapid-Fire Frame"],
    903: ["Adaptive Frame", "Adaptive Glaive"],
    904: ["Aggressive Frame", "Aggressive Glaive"],
    905: ["Lightweight Frame", "MIDA Synergy"],
    906: ["Precision Frame", "Häkke Precision Frame"],
    907: ["Double Fire"],

    911: ["Legacy PR-55 Frame"],
}

manifest = requests.get(API_ROOT + "Manifest/", headers=API_KEY_HEADER).json()
contentPaths = manifest["Response"]["jsonWorldComponentContentPaths"]["en"]
itemData:dict[str, dict] = requests.get(CONTENT_ROOT + contentPaths["DestinyInventoryItemDefinition"], headers=API_KEY_HEADER).json()

#enumerate through all item data
for key in itemData:
    value = itemData[key]
    hash = int(key)
    try:
        if "Intrinsic" not in value["itemTypeDisplayName"]:
            continue
    except KeyError:
        continue
    name = value["displayProperties"]["name"]
    for id in intrinsic_map:
        if name in intrinsic_map[id]:
            out_lst.append((hash, id))


out_str = str(out_lst)
out_len = len(out_lst)
with open ("./database/enhanced_handler_template.rs", "rw") as f:
    template = f.read()
    template = template.replace("{REROUTE_DATA_POINT}", out_str)
    template = template.replace("{REROUTE_DATA_len}", str(out_len))
    f.write(template)