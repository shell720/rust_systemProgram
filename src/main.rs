#![feature(link_llvm_intrinsics)]
#[macro_use]
extern crate crossbeam;

//mod kani_test;
//mod visualize_f32;
//mod q7_format;
//mod cpu;
//mod pointer_intro;
//mod check_heapCost;
//mod files;
//mod nw;
//mod clock;
//mod parallel;
mod warikomi;

fn main() {
    // section5();
    // cpu::main();
    
    // section6();
    // section7();
    // section8();
    //section9();
    //section10();
    section12();
}

fn section12(){
    // warikomi::sixty();
    //warikomi::main();
    //warikomi::change_sigfun();
    warikomi::sisj();
}

fn section10(){
    //parallel::two_threads();
    //parallel::many_threads_sleep();
    //parallel::many_threads_busyweight();
    //parallel::channel_intro();
    //parallel::channel_duplex();
}

fn section9(){
    //clock::main();
}

fn section8(){
    // nw::basic_httpreq();
    // nw::http_bytcpconn();
    //nw::macgen();
}

fn section7(){
    //files::serialize();
    //files::hexdump();
    // files::writebyte_123();
}

fn section6(){
    // pointer_intro::main();
    // pointer_intro::rawPointer_cast();
    // pointer_intro::release_heapMemory();
    // check_heapCost::main();
}

fn section5(){
    // kani_test::bit_invest();
    // kani_test::check_endian();
    // visualize_f32::main();
}