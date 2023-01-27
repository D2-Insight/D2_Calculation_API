import json


def verify_handling_data(_handling_data: dict) -> dict:
    default_formula = {"evpp": 0.0, "vpp": 0.0, "offset": 0.0}
    if "ready" not in _handling_data:
        _handling_data["ready"] = default_formula
    if "stow" not in _handling_data:
        _handling_data["stow"] = default_formula
    if "ads" not in _handling_data:
        _handling_data["ads"] = default_formula
    if len(_handling_data) > 3:
        raise Exception("Bad entries in handling data")
    return _handling_data

def verify_range_data(_range_data: dict) -> dict:
    if "vpp_start" not in _range_data:
        _range_data["vpp_start"] = 0.0
    if "vpp_end" not in _range_data:
        _range_data["vpp_end"] = 0.0
    if "offset_start" not in _range_data:
        _range_data["offset_start"] = 0.0
    if "offset_end" not in _range_data:
        _range_data["offset_end"] = 0.0
    if "floor_percent" not in _range_data:
        _range_data["floor_percent"] = 0.0
    if "fusion" not in _range_data:
        _range_data["fusion"] = False
    if len(_range_data) > 6:
        raise Exception("Bad entries in range data")
    return _range_data

def verify_reload_data(_reload_data: dict) -> dict:
    if "evpp" not in _reload_data:
        _reload_data["evpp"] = 0.0
    if "vpp" not in _reload_data:
        _reload_data["vpp"] = 0.0
    if "offset" not in _reload_data:
        _reload_data["offset"] = 0.0
    if "ammo_percent" not in _reload_data:
        _reload_data["ammo_percent"] = 0.0
    if len(_reload_data) > 4:
        raise Exception("Bad entries in reload data")
    return _reload_data

def verify_scalar_data(_scalar_data: dict) -> dict:
    if "pve" not in _scalar_data:
        _scalar_data["pve"] = 1.0
    if "vehicle" not in _scalar_data:
        _scalar_data["vehicle"] = 1.0
    if "miniboss" not in _scalar_data:
        _scalar_data["miniboss"] = 1.0
    if "boss" not in _scalar_data:
        _scalar_data["boss"] = 1.0
    if "elite" not in _scalar_data:
        _scalar_data["elite"] = 1.0
    if "minor" not in _scalar_data:
        _scalar_data["minor"] = 1.0
    if "champion" not in _scalar_data:
        _scalar_data["champion"] = _scalar_data["miniboss"]
    if len(_scalar_data) > 7:
        raise Exception("Bad entries in scalar data")
    return _scalar_data

def verify_firing_data(_firing_data: dict) -> dict:
    if "damage" not in _firing_data:
        _firing_data["damage"] = 1.0
    if "crit_mult" not in _firing_data:
        _firing_data["crit_mult"] = 1.0
    if "burst_delay" not in _firing_data:
        _firing_data["burst_delay"] = 0.0
    if "burst_size" not in _firing_data:
        _firing_data["burst_size"] = 0.0
    if "burst_duration" not in _firing_data:
        _firing_data["burst_duration"] = 0.0
    if "is_explosive" not in _firing_data:
        _firing_data["explosive"] = False
    if "is_charge" not in _firing_data:
        _firing_data["charge"] = False
    if "one_ammo_burst" not in _firing_data:
        _firing_data["one_ammo"] = False
    if len(_firing_data) > 8:
        raise Exception("Bad entries in firing data")
    return _firing_data

def verify_ammo_data(_ammo_data: dict) -> dict:
    if "mag" not in _ammo_data:
        _ammo_data["mag"] = {"evpp": 0.0, "vpp": 0.0, "offset": 1.0}
    if "reserve_id" not in _ammo_data:
        _ammo_data["reserve_id"] = 0
    if "round_to" not in _ammo_data:
        _ammo_data["round_to"] = 0
    if len(_ammo_data) > 3:
        raise Exception("Bad entries in mag data")
    return _ammo_data





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
    if weapon_id == "18" or weapon_id == "31":
        continue
    inner_values = new_jdata[weapon_id]
    print()
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
                data["R"] = range_data.index(range)

                handling:dict = verify_handling_data(basic_defs["handling"])
                if handling not in handling_data:
                    handling_data.append(handling)
                data["H"] = handling_data.index(handling)

                reload:dict = verify_reload_data(basic_defs["reload"])
                if reload not in reload_data:
                    reload_data.append(reload)
                data["RL"] = reload_data.index(reload)

                scalar:dict = verify_scalar_data(basic_defs["combatant_scalars"])
                scalar["pve"] = weapon_def.get("pve", 1.0)
                if scalar not in scalar_data:
                    scalar_data.append(scalar)
                data["S"] = scalar_data.index(scalar)

                firing:dict = verify_firing_data(inner_values["subFam"][weapon_def["subFam"]])
                if firing not in firing_data:
                    firing_data.append(firing)
                data["F"] = firing_data.index(firing)

                ammo:dict = verify_ammo_data(inner_values["magProf"][weapon_def["magProf"]])
                if ammo not in ammo_data:
                    ammo_data.append(ammo)
                data["A"] = ammo_data.index(ammo)
            except Exception as e:
                print(e, weapon_id, weapon_hash)

            updated_weapon_defs[weapon_hash] = data

    weapon_paths[weapon_id] = updated_weapon_defs

def list_str(_list: list) -> str:
    return str([json.JSONEncoder().encode(i).replace(" ", "") for i in _list])

with open("database\\template.rs", "r") as f:
    template = f.read()



template = template.replace("{HANDLING_REPLACE_POINT}", list_str(handling_data))
template = template.replace("{RANGE_REPLACE_POINT}", list_str(range_data))
template = template.replace("{RELOAD_REPLACE_POINT}", list_str(reload_data))
template = template.replace("{SCALAR_REPLACE_POINT}", list_str(scalar_data))
template = template.replace("{FIRING_REPLACE_POINT}", list_str(firing_data))
template = template.replace("{AMMO_REPLACE_POINT}", list_str(ammo_data))

template = template.replace("{HANDLING_REPLACE_POINT_len}", str(len(handling_data)))
template = template.replace("{RANGE_REPLACE_POINT_len}", str(len(range_data)))
template = template.replace("{RELOAD_REPLACE_POINT_len}", str(len(reload_data)))
template = template.replace("{SCALAR_REPLACE_POINT_len}", str(len(scalar_data)))
template = template.replace("{FIRING_REPLACE_POINT_len}", str(len(firing_data)))
template = template.replace("{AMMO_REPLACE_POINT_len}", str(len(ammo_data)))

template = template.replace("{PATH_REPLACE_POINT}", json.JSONEncoder().encode(weapon_paths).replace(" ", ""))

template = template.replace("\'{", "r#\"{").replace("}\'", "}\"#")
with open("database\\weapon_formulas.rs", "w") as f:
    f.write(template)

# debug_out = {"handling_data": handling_data, "range_data": range_data, "reload_data": reload_data, "scalar_data": scalar_data, "firing_data": firing_data, "ammo_data": ammo_data, "weapon_paths": weapon_paths}

# jstring = json.JSONEncoder().encode(debug_out)
# jstring = jstring.replace(" ", "")
# with open("database\\weapon_formulas_special_pathing.json", "w") as f:
#     f.write(jstring)

