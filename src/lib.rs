#![feature(box_syntax, unboxed_closures, ip_addr)]
use std::fmt;
use std::str::FromStr;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Clone, Debug)]
pub struct Origin {
    pub username: String,
    pub session_id: String,
    pub session_version: i64,
    pub ip_address: IpAddr,
}

#[derive(Clone, Debug)]
pub struct ConnectionData {
    pub ip_address: IpAddr,
    pub ttl: Option<u8>,
    pub num_addresses: Option<u8>
}

#[derive(Clone, Debug)]
pub struct SessionDescription {
    pub protocol_version: Option<i32>,
    pub origin: Option<Origin>,
    pub session_name: Option<String>,
    pub session_information: Vec<String>,
    pub uri: Option<String>,
    // TODO: email, phone
    pub connection_data: Option<ConnectionData>,
    pub media: Vec<MediaDescription>,
}

#[derive(Clone, Debug)]
pub struct MediaDescription {
    pub todo: u8,
}

impl SessionDescription {
    pub fn new() -> SessionDescription {
        SessionDescription {
            protocol_version: None,
            origin: None,
            session_name: None,
            session_information: vec![],
            uri: None,
            connection_data: None,
            media: vec![],
        }
    }

    pub fn from_sdp(sdp: &str) -> ParseResult {
        let mut res = ParseResult::new();
        let mut sdm: Option<MediaDescription> = None;

        for line in sdp.lines() {
            println!("line: {}", line);
            if let Some(parsed) = parse_line(line) {
                match sdm {
                    None => {
                        match parsed {
                            SdpLine::ProtocolVersion(v) => { res.desc.protocol_version = Some(v); },
                            SdpLine::Origin(o) => { res.desc.origin = Some(o); },
                        }
                    }, Some(ref media) => {
                        match parsed {
                            SdpLine::ProtocolVersion(_) => { res.ignored_lines.push(parsed.clone()); },
                            SdpLine::Origin(_) => { res.ignored_lines.push(parsed.clone()); },
                        }
                    }
                }
            } else {
                println!("invalid: {}", line);
                res.unparsed_lines.push(line.to_string());
            }
        }

        res
    }
}

#[derive(Debug)]
pub struct ParseResult {
    pub desc: SessionDescription,
    pub ignored_lines: Vec<SdpLine>,
    pub unparsed_lines: Vec<String>,
}


impl ParseResult {
    pub fn new() -> ParseResult {
        ParseResult {
            desc: SessionDescription::new(),
            ignored_lines: vec![],
            unparsed_lines: vec![],
        }
    }
}

impl fmt::Display for SessionDescription {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.protocol_version {
            Some(ref v) => try!(write!(f, "v={}\n", v)),
            None => {}
        };

        match self.origin {
            Some(ref o) => {
                try!(write!(f, "o={} {} {} ", o.username, o.session_id, o.session_version));
                match o.ip_address {
                    IpAddr::V4(_) =>
                        try!(write!(f, "IN IP4 {}", o.ip_address)),
                    IpAddr::V6(_) =>
                        try!(write!(f, "IN IP6 {}", o.ip_address))
                }
                try!(write!(f, "\n"));
            },
            None => {}
        };

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum SdpLine {
    ProtocolVersion(i32),
    Origin(Origin),
}

fn parse_line(line: &str) -> Option<SdpLine> {
    let parts = line.splitn(2, '=').collect::<Vec<&str>>();
    if parts.len() != 2 {
        return None;
    }

    let line_type = parts[0];
    let line_val = parts[1];

    match line_type {
        "v" => {
            if let Ok(v) = FromStr::from_str(line_val) {
                println!("v => {}", v);
                Some(SdpLine::ProtocolVersion(v))
            } else {
                None
            }
        },
        "o" => {
            if let Some(o) = parse_origin(line_val) {
                //println!("o => {}", o);
                Some(SdpLine::Origin(o))
            } else {
                None
            }
        },
        _ => None
    }
    //Some(SdpLine::ProtocolVersion(3))
}

fn parse_origin(text: &str) -> Option<Origin> {
    let parts = text.split(' ').collect::<Vec<&str>>();
    if parts.len() != 6 {
        return None;
    }

    let session_version = FromStr::from_str(parts[2]);
    let ip_addr = FromStr::from_str(parts[5]);
    // TODO: Care about 'IN IP[46]'

    if session_version.is_err() || ip_addr.is_err() {
        return None
    }

    Some(Origin {
        username: parts[0].to_string(),
        session_id: parts[1].to_string(),
        session_version: session_version.unwrap(),
        ip_address: ip_addr.unwrap(),
    })
}
