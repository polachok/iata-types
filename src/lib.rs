extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::fmt;
use std::str;
use std::str::FromStr;

macro_rules! gen_as {
    () => {
        pub fn as_str(&self) -> &str {
            str::from_utf8(&self.0).unwrap()
        }

        pub fn as_bytes(&self) -> &[u8] {
            &self.0
        }
    }
}

macro_rules! gen_display {
    ($t: ty) => {
        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }
    }
}

/// IATA aircraft type code
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Hash, Clone)]
pub struct AircraftCode([u8; 3]);

gen_display!(AircraftCode);

impl AircraftCode {
    gen_as!();

    /// Reconstruct AircraftCode from AircraftCode.as_bytes()
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> Self {
        let mut mine = [0; 3];

        mine.copy_from_slice(bytes);
        AircraftCode(mine)
    }
}

#[derive(Debug)]
pub enum AircraftCodeParseError {
    InvalidLength(usize),
    InvalidCharacter(char)
}

impl std::fmt::Display for AircraftCodeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AircraftCodeParseError::InvalidLength(len) => {
                write!(f, "invalid length: {}, expected 3", len)
            },
            AircraftCodeParseError::InvalidCharacter(c) => {
                write!(f, "invalid character: {}, expected A-Z0-9", c)
            }
        }
    }
}

impl std::error::Error for AircraftCodeParseError {
    fn description(&self) -> &str {
        "aircraft code parsing error"
    }
}

impl FromStr for AircraftCode {
    type Err = AircraftCodeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != 3 {
            return Err(AircraftCodeParseError::InvalidLength(value.len()));
        }

        for c in value.chars() {
            if c.is_ascii_uppercase() || c.is_ascii_digit() {
                continue;
            } else {
                return Err(AircraftCodeParseError::InvalidCharacter(c));
            }
        }
        let mut bytes = [0; 3];
        bytes.copy_from_slice(value.as_bytes());
        Ok(AircraftCode(bytes))
    }
}

// Airline designator codes follow the format xx(a), i.e.,
// two alphanumeric characters (letters or digits) followed by an optional letter.
// Although the IATA standard provides for three-character airline designators,
// IATA has not used the optional third character in any assigned code.
// This is because some legacy computer systems, especially the "central reservations systems",
// have failed to comply with the standard, notwithstanding the fact that it has been in place for 20 years.
// The codes issued to date comply with IATA Resolution 762, which provides for only two characters.
/// IATA airline designators are used to identify an airline for commercial purposes in reservations,
/// timetables, tickets, tariffs, air waybills and in telecommunications.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Hash, Clone)]
pub struct AirlineCode([u8; 2]);

gen_display!(AirlineCode);

impl AirlineCode {
    gen_as!();


    /// Reconstruct AirlineCode from AirlineCode.as_bytes()
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> Self {
        let mut mine = [0; 2];

        mine.copy_from_slice(bytes);
        AirlineCode(mine)
    }
}

#[derive(Debug)]
pub enum AirlineCodeParseError {
    InvalidLength(usize),
    InvalidCharacter(char)
}

impl std::fmt::Display for AirlineCodeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AirlineCodeParseError::InvalidLength(len) => {
                write!(f, "invalid length: {}, expected 2", len)
            },
            AirlineCodeParseError::InvalidCharacter(c) => {
                write!(f, "invalid character: {}, expected A-Z0-9", c)
            }
        }
    }
}

impl std::error::Error for AirlineCodeParseError {
    fn description(&self) -> &str {
        "airline code parsing error"
    }
}

impl FromStr for AirlineCode {
    type Err = AirlineCodeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != 2 {
            return Err(AirlineCodeParseError::InvalidLength(value.len()));
        }

        for c in value.chars() {
            if c.is_ascii_uppercase() || c.is_ascii_digit() {
                continue;
            } else {
                return Err(AirlineCodeParseError::InvalidCharacter(c))
            }
        }
        let mut bytes = [0; 2];
        bytes.copy_from_slice(value.as_bytes());
        Ok(AirlineCode(bytes))
    }
}

/// 3 letter airport code
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Hash, Clone)]
pub struct AirportCode([u8; 3]);

gen_display!(AirportCode);

impl AirportCode {
    gen_as!();


    /// Reconstruct AirportCode from AirportCode.as_bytes()
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> Self {
        let mut mine = [0; 3];

        mine.copy_from_slice(bytes);
        AirportCode(mine)
    }
}

#[derive(Debug)]
pub enum AirportCodeParseError {
    InvalidLength(usize),
    InvalidCharacter(char)
}

impl std::fmt::Display for AirportCodeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AirportCodeParseError::InvalidLength(len) => {
                write!(f, "invalid length: {}, expected 3", len)
            },
            AirportCodeParseError::InvalidCharacter(c) => {
                write!(f, "invalid character: {}, expected A-Z0-9", c)
            }
        }
    }
}

impl std::error::Error for AirportCodeParseError {
    fn description(&self) -> &str {
        "airport code parsing error"
    }
}

impl FromStr for AirportCode {
    type Err = AirportCodeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != 3 {
            return Err(AirportCodeParseError::InvalidLength(value.len()));
        }
        for c in value.chars() {
            if c.is_ascii_uppercase() {
                continue;
            } else {
                return Err(AirportCodeParseError::InvalidCharacter(c));
            }
        }
        let mut bytes = [0; 3];
        bytes.copy_from_slice(value.as_bytes());
        Ok(AirportCode(bytes))
    }
}

/// 3 letter location code
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Hash, Clone)]
pub struct CityCode([u8; 3]);

gen_display!(CityCode);

impl CityCode {
    gen_as!();

    /// Reconstruct CityCode from CityCode.as_bytes()
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> Self {
        let mut mine = [0; 3];

        mine.copy_from_slice(bytes);
        CityCode(mine)
    }
}

#[derive(Debug)]
pub enum CityCodeParseError {
    InvalidLength(usize),
    InvalidCharacter(char)
}

impl std::fmt::Display for CityCodeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CityCodeParseError::InvalidLength(len) => {
                write!(f, "invalid length: {}, expected 3", len)
            },
            CityCodeParseError::InvalidCharacter(c) => {
                write!(f, "invalid character: {}, expected A-Z0-9", c)
            }
        }
    }
}

impl std::error::Error for CityCodeParseError {
    fn description(&self) -> &str {
        "airport code parsing error"
    }
}

impl FromStr for CityCode {
    type Err = CityCodeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != 3 {
            return Err(CityCodeParseError::InvalidLength(value.len()));
        }
        for c in value.chars() {
            if c.is_ascii_uppercase() {
                continue;
            } else {
                return Err(CityCodeParseError::InvalidCharacter(c));
            }
        }
        let mut bytes = [0; 3];
        bytes.copy_from_slice(value.as_bytes());
        Ok(CityCode(bytes))
    }
}

/// Numeric part of flight designator
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Hash, Clone)]
pub struct FlightNumber(u16);

impl FlightNumber {
    pub fn to_u16(&self) -> u16 {
        self.0
    }

    pub fn from_u16(num: u16) -> Result<Self, FlightNumberParseError> {
        if num >= 1 && num <= 9999 {
            return Ok(FlightNumber(num))
        }
        Err(FlightNumberParseError::InvalidNumber)
    }
}

impl fmt::Display for FlightNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}", self.0)
    }
}

#[derive(Debug)]
pub enum FlightNumberParseError {
    NotANumber,
    InvalidNumber,
}

impl std::fmt::Display for FlightNumberParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FlightNumberParseError::NotANumber => {
                write!(f, "not a number")
            },
            FlightNumberParseError::InvalidNumber => {
                write!(f, "invalid number, expect 0001 to 9999")
            }
        }
    }
}

impl std::error::Error for FlightNumberParseError {
    fn description(&self) -> &str {
        "airport code parsing error"
    }
}

impl FromStr for FlightNumber {
    type Err = FlightNumberParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let num = value.parse()
                .map_err(|_| FlightNumberParseError::NotANumber)?;
        Self::from_u16(num)
    }
}