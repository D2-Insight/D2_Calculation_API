const ENHANCED_TO_NORMAL_TRAIT: [(u32, u32); {REROUTE_DATA_len}] = {REROUTE_DATA_POINT};

pub fn enhanced_check(_hash: u32) -> (u32, bool) {
    let mut result = _hash;
    let mut found = false;
    for (_, (h, r)) in ENHANCED_TO_NORMAL_TRAIT.iter().enumerate() {
        if _hash == *h {
            result = *r;
            found = true;
            break;
        }
    }
    (result, found)
}


