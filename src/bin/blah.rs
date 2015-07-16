#![feature(ip_addr)]
extern crate sdp;
use sdp::{SessionDescription, Origin};
use std::net::{IpAddr, Ipv6Addr};

fn main() {
    let mut s1 = SessionDescription::new();
    s1.protocol_version = Some(1);
    s1.origin = Some(Origin {
        username: "me".to_string(),
        session_id: "sessA".to_string(),
        session_version: 11,
        ip_address: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 5, 2))
    });
    let s1_exp = format!("{:?}", s1);

    let s2_res = SessionDescription::from_sdp(&s1_exp);

    println!("\ns1\n---\n{:?}\ns2\n---\n{:?}\n", s1, s2_res.desc);
    println!("---\ns2_res:\n{:?}\n---", s2_res);

    let s3_res = SessionDescription::from_sdp("v=17
o");

    println!("---\nv3_res:\n{:?}", s3_res);
}
