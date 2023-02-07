import json
import traceback

id_to_name = {
        "6":  "Auto Rifle",
        "31": "Combat Bow",
        "11": "Fusion Rifle",
        "23": "Grenade Launcher",
        "9":  "Hand Cannon",
        "22": "Linear Fusion Rifle",
        "8":  "Machine Gun",
        "13": "Pulse Rifle",
        "10": "Rocket Launcher",
        "14": "Scout Rifle",
        "7":  "Shotgun",
        "12": "Sniper Rifle",
        "24": "Submachine Gun",
        "18": "Sword",
        "33": "Glaive",
        "25": "Trace Rifle",
        "17": "Sidearm"
    }

def verify_stat_formula(_stat_formula: dict) -> dict:
    if "evpp" not in _stat_formula:
        _stat_formula["evpp"] = 0.0
    if "vpp" not in _stat_formula:
        _stat_formula["vpp"] = 0.0
    if "offset" not in _stat_formula:
        _stat_formula["offset"] = 0.0
    vpp_val = _stat_formula["vpp"]
    del _stat_formula["vpp"]
    _stat_formula["vpp"] = vpp_val
    offset_val = _stat_formula["offset"]
    del _stat_formula["offset"]
    _stat_formula["offset"] = offset_val
    if len(_stat_formula) > 3:
        raise Exception("Bad entries in stat formula")
    _stat_formula["evpp"] = round(float(_stat_formula["evpp"]), 8)
    _stat_formula["vpp"] = round(float(_stat_formula["vpp"]), 8)
    _stat_formula["offset"] = round(float(_stat_formula["offset"]), 8)
    return _stat_formula

def verify_handling_data(_handling_data: dict) -> dict:
    default_formula = {"evpp": 0.0, "vpp": 0.0, "offset": 0.0}
    if "ready" not in _handling_data:
        _handling_data["ready"] = default_formula
    if "stow" not in _handling_data:
        _handling_data["stow"] = default_formula
    if "ads" not in _handling_data:
        _handling_data["ads"] = default_formula
    _handling_data["ready"] = verify_stat_formula(_handling_data["ready"])
    _handling_data["stow"] = verify_stat_formula(_handling_data["stow"])
    _handling_data["ads"] = verify_stat_formula(_handling_data["ads"])
    if len(_handling_data) > 3:
        raise Exception("Bad entries in handling data")
    return _handling_data

def verify_range_data(_range_data: dict) -> dict:
    range_data_cpy = _range_data.copy()
    if "vpp_start" not in range_data_cpy and "start" not in range_data_cpy:
        range_data_cpy["vpp_start"] = 0.0
    if "vpp_end" not in range_data_cpy and "end" not in range_data_cpy:
        range_data_cpy["vpp_end"] = 0.0
    if "offset_start" not in range_data_cpy and "start" not in range_data_cpy:
        range_data_cpy["offset_start"] = 0.0
    if "offset_end" not in range_data_cpy and "end" not in range_data_cpy:
        range_data_cpy["offset_end"] = 0.0
    if "floor_percent" not in range_data_cpy:
        range_data_cpy["floor_percent"] = 0.0
    if "fusion" not in range_data_cpy:
        range_data_cpy["fusion"] = False
    if len(range_data_cpy) > 6:
        raise Exception("Bad entries in range data")
    range_data_cpy["start"] = verify_stat_formula({"evpp": 0.0, "vpp": range_data_cpy["vpp_start"], "offset": range_data_cpy["offset_start"]})
    range_data_cpy["end"] = verify_stat_formula({"evpp": 0.0, "vpp": range_data_cpy["vpp_end"], "offset": range_data_cpy["offset_end"]})
    if "vpp_start" in range_data_cpy:
        del range_data_cpy["vpp_start"]
    if "vpp_end" in range_data_cpy:
        del range_data_cpy["vpp_end"]
    if "offset_start" in range_data_cpy:
        del range_data_cpy["offset_start"]
    if "offset_end" in range_data_cpy:
        del range_data_cpy["offset_end"]
    range_data_cpy["floor_percent"] = float(range_data_cpy["floor_percent"])
    return range_data_cpy

def verify_reload_data(_reload_data: dict) -> dict:
    if "evpp" not in _reload_data and "reload_data" not in _reload_data:
        _reload_data["evpp"] = 0.0
    if "vpp" not in _reload_data and "reload_data" not in _reload_data:
        _reload_data["vpp"] = 0.0
    if "offset" not in _reload_data and "reload_data" not in _reload_data:
        _reload_data["offset"] = 0.0
    if "ammo_percent" not in _reload_data:
        _reload_data["ammo_percent"] = 0.0
    if len(_reload_data) > 4:
        raise Exception("Bad entries in reload data")
    if "reload_data" not in _reload_data:
        _reload_data["reload_data"] = verify_stat_formula({"evpp": _reload_data["evpp"], "vpp": _reload_data["vpp"], "offset": _reload_data["offset"]})
    if "evpp" in _reload_data:
        del _reload_data["evpp"]
    if "vpp" in _reload_data:
        del _reload_data["vpp"]
    if "offset" in _reload_data:
        del _reload_data["offset"]
    _reload_data["ammo_percent"] = float(_reload_data["ammo_percent"])
    return _reload_data

def verify_scalar_data(_scalar_data: dict) -> dict:
    if "pve" not in _scalar_data:
        _scalar_data["pve"] = 1.0
    if "vehicle" not in _scalar_data:
        _scalar_data["vehicle"] = 1.0
    if "miniboss" not in _scalar_data:
        _scalar_data["miniboss"] = 1.0
    if "champion" not in _scalar_data:
        _scalar_data["champion"] = _scalar_data["miniboss"]
    if "boss" not in _scalar_data:
        _scalar_data["boss"] = 1.0
    if "elite" not in _scalar_data:
        _scalar_data["elite"] = 1.0
    if "minor" not in _scalar_data:
        _scalar_data["minor"] = 1.0
    if len(_scalar_data) > 7:
        raise Exception("Bad entries in scalar data")
    _scalar_data["pve"] = float(_scalar_data["pve"])
    _scalar_data["vehicle"] = float(_scalar_data["vehicle"])
    _scalar_data["miniboss"] = float(_scalar_data["miniboss"])
    _scalar_data["champion"] = float(_scalar_data["champion"])
    _scalar_data["boss"] = float(_scalar_data["boss"])
    _scalar_data["elite"] = float(_scalar_data["elite"])
    _scalar_data["minor"] = float(_scalar_data["minor"])
    return _scalar_data

def verify_firing_data(_firing_data: dict) -> dict:
    if "damage" not in _firing_data:
        _firing_data["damage"] = 1.0
    if "crit_mult" not in _firing_data:
        _firing_data["crit_mult"] = 0.0
    if "burst_delay" not in _firing_data:
        _firing_data["burst_delay"] = 0.0
    if "burst_size" not in _firing_data:
        _firing_data["burst_size"] = 0
    if "inner_burst_delay" not in _firing_data:
        _firing_data["inner_burst_delay"] = 0.0
    if "is_charge" not in _firing_data:
        _firing_data["charge"] = False
    if "one_ammo" not in _firing_data:
        _firing_data["one_ammo"] = False
    if len(_firing_data) > 8:
        raise Exception("Bad entries in firing data")
    if (_firing_data["crit_mult"] % 1 == 0 and type(_firing_data["crit_mult"]) == int) or _firing_data["crit_mult"] == -25.5:
        _firing_data["damage"] = float(_firing_data["damage"])
        _firing_data["crit_mult"] = float((1.5 + _firing_data["crit_mult"]/51))
        _firing_data["burst_delay"] = float(_firing_data["burst_delay"]/30)
        _firing_data["burst_size"] = int(_firing_data["burst_size"])
        _firing_data["inner_burst_delay"] = float(_firing_data["inner_burst_delay"]/30)
        _firing_data["charge"] = bool(_firing_data["charge"])
        _firing_data["one_ammo"] = bool(_firing_data["one_ammo"])
    return _firing_data

def verify_ammo_data(_ammo_data: dict) -> dict:
    if "mag" not in _ammo_data:
        _ammo_data["mag"] = {"evpp": 0.0, "vpp": 0.0, "offset": 1.0}
    if "reserve_id" not in _ammo_data:
        _ammo_data["reserve_id"] = 0
    if "round_to" not in _ammo_data:
        _ammo_data["round_to"] = 0
    _ammo_data["mag"] = verify_stat_formula(_ammo_data["mag"])
    if len(_ammo_data) > 3:
        raise Exception("Bad entries in mag data")
    _ammo_data["reserve_id"] = int(_ammo_data["reserve_id"])
    _ammo_data["round_to"] = int(_ammo_data["round_to"])
    return _ammo_data

def verify_data_pointers(_in: dict) -> dict:
    data_pointers = _in.copy()
    if "r" not in data_pointers:
        data_pointers["r"] = 0
    if "h" not in data_pointers:
        data_pointers["h"] = 0
    if "rl" not in data_pointers:
        data_pointers["rl"] = 0
    if "s" not in data_pointers:
        data_pointers["s"] = 0
    if "f" not in data_pointers:
        data_pointers["f"] = 0
    if "a" not in data_pointers:
        data_pointers["a"] = 0
    return data_pointers

def rustify_handling_string(_in: str) -> str:
    out = _in.replace("\'{", "HandlingFormula{").replace("}\'", "}").replace("\"", "")
    out = out.replace(":{evpp", ": StatQuadraticFormula{evpp")
    return out

def rustify_range_string(_in: str) -> str:
    out = _in.replace("\'{", "RangeFormula{").replace("}\'", "}").replace("\"", "")
    out = out.replace(":{evpp", ": StatQuadraticFormula{evpp")
    return out

def rustify_reload_string(_in: str) -> str:
    out = _in.replace("\'{", "ReloadFormula{").replace("}\'", "}").replace("\"", "")
    out = out.replace(":{evpp", ": StatQuadraticFormula{evpp")
    return out

def rustify_scalar_string(_in: str) -> str:
    out = _in.replace("\'{", "DamageMods{").replace("}\'", "}").replace("\"", "")
    return out

def rustify_firing_string(_in: str) -> str:
    out = _in.replace("\'{", "FiringData{").replace("}\'", "}").replace("\"", "")
    return out

def rustify_ammo_string(_in: str) -> str:
    out = _in.replace("\'{", "AmmoFormula{").replace("}\'", "}").replace("\"", "")
    out = out.replace(":{evpp", ": StatQuadraticFormula{evpp")
    return out

def rustify_weapon_path(_in: dict) -> tuple[tuple[str, str],tuple[str, str]]:
    meta_pointers: list[tuple[int, int]] = []
    out_lst = []
    counter = 0
    for i in _in:
        if i == "18":
            continue
        meta_pointers.append((int(i), counter))
        family_defs = _in[i]
        def_lst = []
        for j in family_defs:
            hash = int(j)
            def_lst.append((hash, family_defs[j]))
        out_lst.append(def_lst)
        counter += 1
    str_data = str(out_lst)
    str_data = str_data.replace("\'", "").replace("[(", "vec![(").replace("{r:", "DataPointers{r:").replace(" ", "")
    return ((str(meta_pointers).replace("\'", ""), str(len(meta_pointers))),(str_data, str(len(out_lst))))

with open("database\\weapon_formulas_editable.json", "r") as f:
    jdata = json.load(f)

del jdata["COMMENTS"]

new_jdata:dict = {}
for i in jdata["INDEX"]:
    tmp_data = jdata[jdata["INDEX"][i]]
    for j in tmp_data:
        if j.isnumeric():
            del tmp_data[j]["name"]
    new_jdata[i] = tmp_data

handling_data:list[dict[str,dict[str,float]]] = [verify_handling_data({})]
range_data:list[dict[str,dict[str,float]]] =    [verify_range_data({})]
reload_data:list[dict[str,dict[str,float]]] =   [verify_reload_data({})]
scalar_data:list[dict[str,dict[str,float]]] =   [verify_scalar_data({})]
firing_data:list[dict[str,dict[str,float]]] =   [verify_firing_data({})]
ammo_data:list[dict[str,dict[str,float]]] =     [verify_ammo_data({})]

weapon_paths:dict[int, dict[str, int]] = {}

for weapon_id in new_jdata:
    if weapon_id == "18":
        continue
    inner_values = new_jdata[weapon_id]
    updated_weapon_defs = {}
    for weapon_hash in inner_values:
        data = {}
        if weapon_hash.isnumeric():
            weapon_def = inner_values[weapon_hash]
            try:
                basic_defs = inner_values["cat"][weapon_def["cat"]]

                range:dict = verify_range_data(basic_defs["range"])
                if range not in range_data:
                    range_data.append(range)
                data["r"] = range_data.index(range)

                handling:dict = verify_handling_data(basic_defs["handling"])
                if handling not in handling_data:
                    handling_data.append(handling)
                data["h"] = handling_data.index(handling)

                reload:dict = verify_reload_data(basic_defs["reload"])
                if reload not in reload_data:
                    reload_data.append(reload)
                data["rl"] = reload_data.index(reload)

                scalar:dict = verify_scalar_data(basic_defs["combatant_scalars"])
                scalar["pve"] = weapon_def.get("pve", 1.0)
                if scalar not in scalar_data:
                    scalar_data.append(scalar)
                data["s"] = scalar_data.index(scalar)

                firing:dict = verify_firing_data(inner_values["subFam"][weapon_def["subFam"]])
                if firing not in firing_data:
                    firing_data.append(firing)
                data["f"] = firing_data.index(firing)

                ammo:dict = verify_ammo_data(inner_values["magProf"][weapon_def["magProf"]])
                if ammo not in ammo_data:
                    ammo_data.append(ammo)
                data["a"] = ammo_data.index(ammo)
            except Exception as e:
                print(e, id_to_name[weapon_id], weapon_hash)

            updated_weapon_defs[weapon_hash] = verify_data_pointers(data)

    weapon_paths[weapon_id] = updated_weapon_defs

def list_str(_list: list) -> str:
    return str([json.JSONEncoder().encode(i).replace(" ", "") for i in _list])

with open("database/weapon_formulas_template.rs", "r") as f:
    template = f.read()



template = template.replace("{HANDLING_REPLACE_POINT}", rustify_handling_string(list_str(handling_data)))
template = template.replace("{RANGE_REPLACE_POINT}", rustify_range_string(list_str(range_data)))
template = template.replace("{RELOAD_REPLACE_POINT}", rustify_reload_string(list_str(reload_data)))
template = template.replace("{SCALAR_REPLACE_POINT}", rustify_scalar_string(list_str(scalar_data)))
template = template.replace("{FIRING_REPLACE_POINT}", rustify_firing_string(list_str(firing_data)))
template = template.replace("{AMMO_REPLACE_POINT}", rustify_ammo_string(list_str(ammo_data)))

template = template.replace("{HANDLING_REPLACE_POINT_len}", str(len(handling_data)))
template = template.replace("{RANGE_REPLACE_POINT_len}", str(len(range_data)))
template = template.replace("{RELOAD_REPLACE_POINT_len}", str(len(reload_data)))
template = template.replace("{SCALAR_REPLACE_POINT_len}", str(len(scalar_data)))
template = template.replace("{FIRING_REPLACE_POINT_len}", str(len(firing_data)))
template = template.replace("{AMMO_REPLACE_POINT_len}", str(len(ammo_data)))

weapon_path_data = rustify_weapon_path(weapon_paths)

template = template.replace("{PATH_REPLACE_POINT}", weapon_path_data[-1][0])
template = template.replace("{PATH_REPLACE_POINT_len}", weapon_path_data[-1][1])

template = template.replace("{META_REPLACE_POINT}", weapon_path_data[0][0])
template = template.replace("{META_REPLACE_POINT_len}", weapon_path_data[0][1])

with open("database\\weapon_formulas.rs", "w") as f:
    f.write(template)

debug_out = {"handling_data": handling_data, "range_data": range_data, "reload_data": reload_data, "scalar_data": scalar_data, "firing_data": firing_data, "ammo_data": ammo_data, "weapon_paths": weapon_paths}

jstring = json.JSONEncoder().encode(debug_out)
jstring = jstring.replace(" ", "")
with open("database\\weapon_formulas.json", "w") as f:
    f.write(jstring)

