pub use crate::offsets::OffsetMarkerTrait;
use crate::{
    DeserializationError, Deserialize, Deserializer, ReaderContext, SerializationError, Serialize,
};

use std::convert::TryInto;

#[allow(non_camel_case_types)]
pub type uint16 = u16;
#[allow(non_camel_case_types)]
pub type uint8 = u8;
#[allow(non_camel_case_types)]
pub type uint32 = u32;
#[allow(non_camel_case_types)]
pub type int16 = i16;
#[allow(clippy::upper_case_acronyms)]
pub type FWORD = i16;
#[allow(clippy::upper_case_acronyms)]
pub type UFWORD = u16;
#[allow(non_camel_case_types)]
pub type GlyphID = u16;

#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct uint24(u32);

pub use super::tag::{InvalidTag, Tag};

impl Serialize for uint24 {
    fn to_bytes(&self, data: &mut Vec<u8>) -> Result<(), SerializationError> {
        if self.0 > (1 << 24) - 1 {
            return Err(SerializationError(format!(
                "Could not fit {:} into uint24",
                self.0
            )));
        }
        data.extend(&self.0.to_be_bytes()[1..]);
        Ok(())
    }
}

impl From<u32> for uint24 {
    fn from(val: u32) -> Self {
        uint24(val)
    }
}

impl From<uint24> for u32 {
    fn from(val: uint24) -> Self {
        val.0
    }
}

impl Deserialize for uint24 {
    fn from_bytes(c: &mut ReaderContext) -> Result<Self, DeserializationError> {
        let bytes: Vec<u8> = c.de_counted(3)?;
        Ok(uint24(
            ((bytes[0] as u32) << 16) + ((bytes[1] as u32) << 8) + bytes[2] as u32,
        ))
    }
}

pub use fixed::types::U16F16;

#[derive(Shrinkwrap, Debug, PartialEq, Copy, Clone)]
pub struct Fixed(pub f32);

pub type Tuple = Vec<f32>;

fn ot_round(value: f32) -> i32 {
    (value + 0.5).floor() as i32
}

impl Fixed {
    pub fn as_packed(&self) -> i32 {
        ot_round(self.0 * 65536.0)
    }
    pub fn from_packed(packed: i32) -> Self {
        Fixed(packed as f32 / 65536.0)
    }

    pub fn round(f: f32) -> f32 {
        Fixed::from_packed(Fixed(f).as_packed()).0
    }
}
impl Serialize for Fixed {
    fn to_bytes(&self, data: &mut Vec<u8>) -> Result<(), SerializationError> {
        let packed: i32 = self.as_packed();
        packed.to_bytes(data)
    }
    fn ot_binary_size(&self) -> usize {
        4
    }
}
impl Deserialize for Fixed {
    fn from_bytes(c: &mut ReaderContext) -> Result<Self, DeserializationError> {
        let packed: i32 = c.de()?;
        Ok(Fixed::from_packed(packed))
    }
}

impl From<f32> for Fixed {
    fn from(num: f32) -> Self {
        Self(num)
    }
}
impl From<Fixed> for f32 {
    fn from(num: Fixed) -> Self {
        num.0
    }
}

#[derive(Shrinkwrap, Debug, Copy, Clone)]
pub struct F2DOT14(pub f32);

impl F2DOT14 {
    pub fn as_packed(&self) -> Result<i16, std::num::TryFromIntError> {
        ot_round(self.0 * 16384.0).try_into()
    }
    pub fn from_packed(packed: i16) -> Self {
        F2DOT14(packed as f32 / 16384.0)
    }

    pub fn round(f: f32) -> f32 {
        F2DOT14::from_packed(F2DOT14(f).as_packed().unwrap()).0
    }
}
impl PartialEq for F2DOT14 {
    fn eq(&self, other: &Self) -> bool {
        self.as_packed() == other.as_packed()
    }
}
impl Eq for F2DOT14 {}
impl PartialOrd for F2DOT14 {
    fn partial_cmp(&self, other: &Self) -> std::option::Option<std::cmp::Ordering> {
        self.as_packed()
            .unwrap()
            .partial_cmp(&other.as_packed().unwrap())
    }
}
impl Ord for F2DOT14 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_packed().unwrap().cmp(&other.as_packed().unwrap())
    }
}

impl std::hash::Hash for F2DOT14 {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.as_packed().unwrap().hash(state)
    }
}

impl Serialize for F2DOT14 {
    fn to_bytes(&self, data: &mut Vec<u8>) -> Result<(), SerializationError> {
        let packed: i16 = self
            .as_packed()
            .map_err(|_| SerializationError("Value didn't fit into a F2DOT14".to_string()))?;
        packed.to_bytes(data)
    }
    fn ot_binary_size(&self) -> usize {
        2
    }
}
impl Deserialize for F2DOT14 {
    fn from_bytes(c: &mut ReaderContext) -> Result<Self, DeserializationError> {
        let packed: i16 = c.de()?;
        Ok(F2DOT14::from_packed(packed))
    }
}

impl From<f32> for F2DOT14 {
    fn from(num: f32) -> Self {
        Self(num)
    }
}
impl From<F2DOT14> for f32 {
    fn from(num: F2DOT14) -> Self {
        num.0
    }
}

/// A 16-bit major version number and a minor version number in the range `0..=9`.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Version16Dot16(u32);

impl Version16Dot16 {
    /// Construct a new version from a float.
    ///
    /// The input should be a positive value, where the fractional part is in the
    /// sequence, `{.0, .1, .. .9}`.
    pub fn from_num(float: f32) -> Self {
        let major = float.trunc() as u16;
        let minor = (float.fract() * 10.) as u8;
        assert!(minor <= 9);
        Self::from_major_minor(major, minor)
    }

    /// Construct a new version from a major and minor pair.
    ///
    /// The minor version is expected to be in the range (0..=9), although
    /// this is not enforced (because we cannot assert in a const fn)
    pub const fn from_major_minor(major: u16, minor: u8) -> Self {
        let major = (major as u32) << 16;
        // we only take the lower nibble
        let minor = (minor & 0x0F) as u32;
        Self(major | minor << 12)
    }

    /// The major version number.
    pub fn major(self) -> u16 {
        ((self.0 >> 16) & 0xFFFF) as u16
    }

    /// The minor version number.
    pub fn minor(&self) -> u8 {
        ((self.0 >> 12) & 0x0f) as u8
    }
}

impl Serialize for Version16Dot16 {
    fn to_bytes(&self, data: &mut Vec<u8>) -> Result<(), SerializationError> {
        self.0.to_bytes(data)
    }

    fn ot_binary_size(&self) -> usize {
        2
    }
}

impl Deserialize for Version16Dot16 {
    fn from_bytes(c: &mut ReaderContext) -> Result<Self, DeserializationError> {
        let packed: u32 = c.de()?;
        Ok(Self(packed))
    }
}

#[derive(Shrinkwrap, Debug, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub struct LONGDATETIME(pub chrono::NaiveDateTime);

use chrono::{Duration, NaiveDate};

impl Serialize for LONGDATETIME {
    fn to_bytes(&self, data: &mut Vec<u8>) -> Result<(), SerializationError> {
        let now = self.timestamp();
        let epoch = NaiveDate::from_ymd(1904, 1, 1).and_hms(0, 0, 0).timestamp();
        (now - epoch).to_bytes(data)
    }
    fn ot_binary_size(&self) -> usize {
        8
    }
}
impl Deserialize for LONGDATETIME {
    fn from_bytes(c: &mut ReaderContext) -> Result<Self, DeserializationError> {
        let diff: i64 = c.de()?;
        let epoch = NaiveDate::from_ymd(1904, 1, 1).and_hms(0, 0, 0);
        let res = epoch + Duration::seconds(diff);
        Ok(LONGDATETIME(res))
    }
}

impl From<chrono::NaiveDateTime> for LONGDATETIME {
    fn from(num: chrono::NaiveDateTime) -> Self {
        Self(num)
    }
}
impl From<LONGDATETIME> for chrono::NaiveDateTime {
    fn from(num: LONGDATETIME) -> Self {
        num.0
    }
}

pub use crate::offsets::{Offset16, Offset32, VecOffset, VecOffset16, VecOffset32};
// OK, the offset type is going to be terrifying.+

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_16_repr() {
        let version = Version16Dot16::from_num(1.9);
        assert_eq!(version.major(), 1);
        assert_eq!(version.minor(), 9);
        let exp = 0x00019000;
        assert_eq!(version.0, exp, "found: 0x{:08x}", version.0);

        let version = Version16Dot16::from_major_minor(505, 2);
        assert_eq!(version.major(), 505);
        assert_eq!(version.minor(), 2);
    }
}
