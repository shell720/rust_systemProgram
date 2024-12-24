use std::{error::Error, net::TcpStream};
use reqwest::{self, Response};

pub fn basic_httpreq() -> Result<(), Box<dyn Error>>{
    let url = "http://www.rustinaction.com/";
    let mut response = reqwest::get(url)?;

    let content = response.text()?;
    print!("{}", content);

    Ok(())
}

use std::io::prelude::*;

pub fn http_bytcpconn() -> std::io::Result<()>{
    let host = "www.rustinaction.com:80";
    let mut conn = TcpStream::connect(host)?;

    conn.write_all(b"GET / HTTP/1.0");
    conn.write_all(b"\r\n");

    conn.write_all(b"Host: www.rustinaction.com")?;
    conn.write_all(b"\r\n\r\n")?;

    std::io::copy(
        &mut conn,
        &mut std::io::stdout()
    )?;

    Ok(())
}

extern crate rand;
use rand::RngCore;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
struct MacAddress([u8; 6]);

impl Display for MacAddress{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result{
        let octet = &self.0;
        write!(f, "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", 
            octet[0], octet[1], octet[2], octet[3], octet[4], octet[5])
    }
}

impl MacAddress{
    fn new()->MacAddress{
        let mut octets:[u8; 6] = [0;6];
        rand::thread_rng().fill_bytes(&mut octets);
        octets[0] |= 0b_0000_0011;
        MacAddress{0: octets}
    }

    fn is_local(&self) -> bool{
        (self.0[0] & 0b_0000_0010) == 0b_0000_0010
    }

    fn is_unicast(&self) -> bool{
        (self.0[0] & 0b_0000_0001) == 0b_0000_0001
    }
}

pub fn macgen() {
    let mac = MacAddress::new();
    assert!(mac.is_local());
    assert!(mac.is_unicast());
    println!("mac:{}", mac);
}