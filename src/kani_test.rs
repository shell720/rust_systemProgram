use std::mem::transmute;

pub fn bit_invest(){
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);
}

pub fn check_endian(){
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    let a: i32 = unsafe{ transmute(big_endian)};
    let b: i32 = unsafe{ transmute(little_endian)};

    println!("{} vs {}", a, b);
}