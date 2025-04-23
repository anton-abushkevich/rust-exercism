pub fn egg_count(display_value: u32) -> usize {
    let mut display_value = display_value;
    let mut count = 0;
    while display_value > 0 {
        count += display_value & 1;
        display_value >>= 1;
    }
    count as usize
}

#[test]
fn check() {
    assert_eq!(egg_count(0), 0);
    assert_eq!(egg_count(16), 1);
    assert_eq!(egg_count(89), 4);
    assert_eq!(egg_count(2_000_000_000), 13);
}