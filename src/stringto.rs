use marine_rs_sdk::marine;

#[marine]
pub fn str_to_i32(n: String) -> i32 {
    n.parse::<i32>().unwrap_or(0)
}

#[marine]
pub fn str_to_u32(n: String) -> u32 {
    n.parse::<u32>().unwrap_or(0)
}

#[marine]
pub fn str_to_u64(n: String) -> u64 {
    n.parse::<u64>().unwrap_or(0)
}

/**
 * Check if the string is an integer or not
 */
#[marine]
pub fn is_it_integer(n: String) -> bool {
    match n.parse::<u64>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

/**
 * Check if the string is an integer or not
 */
#[marine]
pub fn is_it_boolean(n: &str) -> bool {
    if n == "1" || n == "0" {
        return true;
    }

    match n.parse::<bool>() {
        Ok(_) => true,
        Err(_) => false,
    }
}
