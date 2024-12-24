mod kani_test;
mod visualize_f32;
//mod q7_format;
mod cpu;
mod pointer_intro;
//mod check_heapCost;
mod files;
mod nw;

fn main() {
    // section5();
    // cpu::main();
    
    // section6();
    // section7();
    section8();
}

fn section8(){
    // nw::basic_httpreq();
    nw::http_bytcpconn();
}

fn section7(){
    //files::serialize();
    //files::hexdump();
    files::writebyte_123();
}

fn section6(){
    pointer_intro::main();
    pointer_intro::rawPointer_cast();
    pointer_intro::release_heapMemory();
    //check_heapCost::main();
}

fn section5(){
    // kani_test::bit_invest();
    // kani_test::check_endian();
    // visualize_f32::main();
}