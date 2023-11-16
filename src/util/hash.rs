// https://en.wikipedia.org/wiki/Jenkins_hash_function
pub fn jenkins(str: &String) -> u64 {
    let mut value: u64 = 0;

    for i in str.chars() {
        value += i as u64;
        value += value << 10;
        value ^= value >> 16;
    }

    value += value << 3;
    value ^= value >> 11;
    value += value << 15;

    return value;
}