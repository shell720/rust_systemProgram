static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

pub fn main(){
    let a = 42;
    let b = &B;
    let c = &C;

    println!("a: {}, b: {:p}, c:{:p}", a, b, c);
}

pub fn rawPointer_cast(){
    let a: i64 = 52;
    let a_ptr = &a as *const i64;
    let a_addr: usize = unsafe{
        std::mem::transmute(a_ptr)
    };
    println!("a: {} ({:p}..0x{:x})", a, a_ptr, a_addr+7);
}

use std::mem::drop;
pub fn release_heapMemory(){
    let a = Box::new(1);
    let b = Box::new(2);
    let c = Box::new(3);
    let result1 = *a + *b + *c;

    drop(a);
    let d = Box::new(4);
    let result2 = *b + *c + *d;

    println!("{} {}", result1, result2);
}