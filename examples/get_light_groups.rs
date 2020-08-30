use std::net::{IpAddr, Ipv4Addr};
use lighthouse::bridge::Bridge;

fn main() {

    let bridge = Bridge::new("192.168.1.7".parse().unwrap(), "iNfKbUViaMmSITeJ5MPQlKA0-jsIJpsFK0nhjCRU".to_string()).unwrap();
    println!("{:#?}", bridge.get_groups().unwrap());

}
