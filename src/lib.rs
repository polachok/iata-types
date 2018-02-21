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
}

#[derive(Debug)]
pub enum AircraftCodeParseError {
    InvalidLength,
    InvalidCharacter
}

impl FromStr for AircraftCode {
    type Err = AircraftCodeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != 3 {
            return Err(AircraftCodeParseError::InvalidLength);
        }

        if value.bytes().all(|item| item.is_ascii_uppercase() || item.is_ascii_digit()) {
            let mut bytes = [0; 3];
            bytes.copy_from_slice(value.as_bytes());
            return Ok(AircraftCode(bytes))
        }
        Err(AircraftCodeParseError::InvalidCharacter)
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
}

#[derive(Debug)]
pub enum AirlineCodeParseError {
    InvalidLength,
    InvalidCharacter
}

impl FromStr for AirlineCode {
    type Err = AirlineCodeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != 2 {
            return Err(AirlineCodeParseError::InvalidLength);
        }

        if value.bytes().all(|item| item.is_ascii_uppercase() || item.is_ascii_digit()) {
            let mut bytes = [0; 2];
            bytes.copy_from_slice(value.as_bytes());
            Ok(AirlineCode(bytes))
        } else {
            return Err(AirlineCodeParseError::InvalidCharacter)
        }
    }
}

/// 3 letter airport code
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Hash, Clone)]
pub struct AirportCode([u8; 3]);

gen_display!(AirportCode);

impl AirportCode {
    gen_as!();
}

#[derive(Debug)]
pub enum AirportCodeParseError {
    InvalidLength,
    InvalidCharacter
}

impl FromStr for AirportCode {
    type Err = AirportCodeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != 3 {
            return Err(AirportCodeParseError::InvalidLength);
        }
        if value.bytes().all(|c| c.is_ascii_uppercase()) {
            let mut bytes = [0; 3];
            bytes.copy_from_slice(value.as_bytes());
            return Ok(AirportCode(bytes))
        }
        Err(AirportCodeParseError::InvalidCharacter)
    }
}

/// 3 letter location code
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Hash, Clone)]
pub struct CityCode([u8; 3]);

gen_display!(CityCode);

impl CityCode {
    gen_as!();
}

#[derive(Debug)]
pub enum CityCodeParseError {
    InvalidLength,
    InvalidCharacter
}

impl FromStr for CityCode {
    type Err = CityCodeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != 3 {
            return Err(CityCodeParseError::InvalidLength);
        }
        if value.bytes().all(|c| c.is_ascii_uppercase()) {
            let mut bytes = [0; 3];
            bytes.copy_from_slice(value.as_bytes());
            return Ok(CityCode(bytes))
        }
        Err(CityCodeParseError::InvalidCharacter)
    }
}

/// Numeric part of flight designator
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Hash, Clone)]
pub struct FlightNumber(u16);

impl fmt::Display for FlightNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}", self.0)
    }
}

pub enum FlightNumberParseError {
    NotANumber,
    InvalidNumber,
}

impl FromStr for FlightNumber {
    type Err = FlightNumberParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let num = value.parse()
                .map_err(|_| FlightNumberParseError::NotANumber)?;
        if num >= 1 && num <= 9999 {
            return Ok(FlightNumber(num))
        }
        Err(FlightNumberParseError::InvalidNumber)
    }
}