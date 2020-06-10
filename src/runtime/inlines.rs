pub fn get_hash_value_case(name: &str) -> u32 {
    if name.is_empty() {
        assert!(false);
    }

    let mut h: u32 = 0;
    for c in name.chars() {
        h = h * 131 + c as u32;
    }

    h
}
