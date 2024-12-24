use bincode::serialize as to_bincode;
use serde_cbor::to_vec as to_cbor;
use serde_json::to_string as to_json;
use serde_derive::{Serialize};

#[derive(Serialize)]
struct City{
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}

pub fn serialize(){
    let calabar = City{
        name: String::from("Calabar"),
        population: 470_000,
        latitude: 4.95,
        longitude: 8.33,
    };

    let as_json = to_json(&calabar).unwrap();
    let as_cbor = to_cbor(&calabar).unwrap();
    let as_bincode = to_bincode(&calabar).unwrap();

    println!("json:\n{}\n", &as_json);
    println!("cbor:\n{:?}\n", &as_cbor);
    println!("bincode:\n{:?}\n", &as_bincode);
    println!("json (as UTF-8):\n{}\n", 
        &String::from_utf8_lossy(as_json.as_bytes())
    );
    println!("cbor (as UTF-8):\n{:?}\n", 
        &String::from_utf8_lossy(&as_cbor)
    );
    println!("bincode (as UTF-8):\n{:?}\n", 
        &String::from_utf8_lossy(&as_bincode)
    );
}

use std::io::{prelude::*, Bytes};
use std::fs::File;
use std::env;

const BYTES_PER_LINE: usize = 16;
/* const INPUT: &'static [u8] = br#"
fn main(){
    println!("Hello. world");
}"#; */

pub fn hexdump() {
    let arg1 = env::args().nth(1);
    let fname = arg1.expect("usage: fview FILENAME");

    let mut f = File::open(&fname).expect("Unable to open file.");
    let mut pos = 0;
    let mut buffer= [0; BYTES_PER_LINE];
    //INPUT.read_to_end(&mut buffer)?;

    while let Ok(_) = f.read_exact(&mut buffer){
        print!("[0x{:08x}]", pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!("."),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte),
            }
        }
        println!("");
        pos += BYTES_PER_LINE;
    }
}

use std::io::Cursor;
use byteorder::{LittleEndian, BigEndian};
use byteorder::{ReadBytesExt, WriteBytesExt};

fn write_numbers_to_file() -> (u32, i8, f64) {
    let mut w = vec![]; // slices implement Read and Write, and can thus act as mock files

    let one: u32   = 1;
    let two: i8    = 2;
    let three: f64 = 4.0;

    w.write_u32::<LittleEndian>(one).unwrap();
    println!("{:?}", &w);

    w.write_i8(two).unwrap(); // single byte methods don't take an endianness parameter
    println!("{:?}", &w);

    w.write_f64::<BigEndian>(three).unwrap();
    println!("{:?}", &w);

    (one, two, three)
}

fn read_numbers_from_file() -> (u32, i8, f64) {
    let mut r = Cursor::new(vec![1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 8, 64]);
    let one_ = r.read_u32::<LittleEndian>().unwrap();
    let two_ = r.read_i8().unwrap();
    let three_ = r.read_f64::<LittleEndian>().unwrap();

    (one_, two_, three_)
}

pub fn writebyte_123() {
    write_numbers_to_file();
    read_numbers_from_file();
}