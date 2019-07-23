use std::fmt;
use std::net::Ipv4Addr;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Deserialize, Serialize)]
pub enum MPEGType {
    I,
    P,
    B,
    X, // The undefined state.
}

impl Default for MPEGType {
    fn default() -> Self {
        MPEGType::X
    }
}

impl fmt::Display for MPEGType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MPEGType::I => write!(f, "{}", "I"),
            MPEGType::P => write!(f, "{}", "P"),
            MPEGType::B => write!(f, "{}", "B"),
            MPEGType::X => write!(f, "{}", "undefined"),
        }
    }
}

impl From<&str> for MPEGType {
    fn from(mpeg_type: &str) -> Self {
        match mpeg_type.to_uppercase().as_str() {
            "I" => MPEGType::I,
            "P" => MPEGType::P,
            "B" => MPEGType::B,
            _ => panic!("Unknown MPEG type '{}'", mpeg_type),
        }
    }
}

impl From<&u16> for MPEGType {
    fn from(mpeg_type: &u16) -> Self {
        match mpeg_type {
            8 => MPEGType::I,
            6 => MPEGType::P,
            7 => MPEGType::B,
            _ => panic!("Unknown MPEG type '{}'", mpeg_type),
        }
    }
}

impl From<&MPEGType> for u16 {
    fn from(mpeg_type: &MPEGType) -> Self {
        match mpeg_type {
            MPEGType::I => 8,
            MPEGType::P => 6,
            MPEGType::B => 7,
            _ => panic!("Unknown MPEG type '{}'", mpeg_type),
        }
    }
}

impl From<&u8> for MPEGType {
    fn from(mpeg_type: &u8) -> Self {
        match mpeg_type {
            8 => MPEGType::I,
            6 => MPEGType::P,
            7 => MPEGType::B,
            _ => panic!("Unknown MPEG type '{}'", mpeg_type),
        }
    }
}

impl From<&MPEGType> for u8 {
    fn from(mpeg_type: &MPEGType) -> Self {
        match mpeg_type {
            MPEGType::I => 8,
            MPEGType::P => 6,
            MPEGType::B => 7,
            _ => panic!("Unknown MPEG type '{}'", mpeg_type),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Frame {
    pub ssrc: u32,
    pub dest_address: Ipv4Addr,
    pub stream_port: u16,
    pub timestamp: u32, // TODO : use a proper structure for timestamp
    pub mpeg_type: MPEGType,
}

// TODO Complete this structure
impl Frame {
    pub fn new(
        ssrc: u32,
        dest_address: Ipv4Addr,
        stream_port: u16,
        timestamp: u32,
        mpeg_type: MPEGType,
    ) -> Self {
        Frame {
            ssrc,
            dest_address,
            stream_port,
            timestamp,
            mpeg_type,
        }
    }

    pub fn get_timestamp(&self) -> u32 {
        self.timestamp
    }

    pub fn get_mpeg_type(&self) -> MPEGType {
        self.mpeg_type
    }
}
