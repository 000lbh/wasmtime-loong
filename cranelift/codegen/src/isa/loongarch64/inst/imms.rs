//! Loongarch64 ISA definitions: immediate constants.

// All definitions can be found here: 
// https://blog.xen0n.name/posts/tinkering/loongarch-faq/

// Notations in this file:
// S for signed
// U for unsigned
// d for index 0
// j for index 5
// k for index 10
// a for index 15
// m for index 16
// For example, Sd5k16 means the immediate constant consists of 5 bits
// from index 0, 16 bits from index 10, and signed extended.

use super::Inst;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Copy, Clone, Debug, Default)]
pub struct ImmSd5k16 {
    bits: u32,
}

impl ImmSd5k16 {
    pub fn new(v: i64) -> Self {
        assert!(v < 0x100000 && v >= -0x100000);
        Self { bits: v as u32 }
    }
    
    // Convert to layout used in instructions
    #[inline]
    pub fn to_inst_layout(&self) -> u32 {
        self.bits >> 16 | (self.bits << 16) >> 6
    }

    #[inline]
    pub fn as_i32(&self) -> i32 {
        (self.bits << 11) as i32 >> 11
    }
}

impl Display for ImmSd5k16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:+}", self.bits.as_i32())
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ImmSd10k16 {
    bits: u32,
}

impl ImmSd10k16 {
    pub fn new(v: i64) -> Self {
        assert!(v < 0x2000000 && v >= -0x2000000);
        Self { bits: v as u32 }
    }

    #[inline]
    pub fn to_inst_layout(&self) -> u32 {
        self.bits >> 16 | (self.bits << 16) >> 6
    }

    #[inline]
    pub fn as_i32(&self) -> i32 {
        (self.bits << 6) as i32 >> 6
    }
}

impl Display for ImmSd10k16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:+}", self.bits.as_i32())
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ImmUa2 {
    bits: u8,
}

impl ImmUa2 {
    pub fn new(v: i64) -> Self {
        assert!(v >= 0 && v < 4);
        Self { bits: v as u8 }
    }

    #[inline]
    pub fn to_inst_layout(&self) -> u32 {
        self.bits << 15
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        self.bits as u32
    }
}

impl Display for ImmUa2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.bits)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ImmUa3 {
    bits: u8,
}

impl ImmUa3 {
    pub fn new(v: i64) -> Self {
        assert!(v >= 0 && v < 8);
        Self { bits: v as u8 }
    }

    #[inline]
    pub fn to_inst_layout(&self) -> u32 {
        self.bits << 15
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        self.bits as u32
    }
}

impl Display for ImmUa3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.bits)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ImmSk12 {
    bits: u16,
}

impl ImmSk12 {
    pub fn new(v: i64) -> Self {
        assert!(v < 0x800 && v >= -0x800);
        Self { bits: v as u16 }
    }

    #[inline]
    pub fn to_inst_layout(&self) -> u32 {
        self.bits << 10
    }

    #[inline]
    pub fn as_i32(&self) -> i32 {
        ((self.bits as i32) << 20) >> 20
    }
}

impl Display for ImmSk12 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:+}", self.bits.as_i32())
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ImmSk14 {
    bits: u16,
}

impl ImmSk14 {
    pub fn new(v: i64) -> Self {
        assert!(v < 0x2000 && v >= -0x2000);
        Self { bits: v as u16 }
    }

    #[inline]
    pub fn to_inst_layout(&self) -> u32 {
        self.bits << 10
    }

    #[inline]
    pub fn as_i32(&self) -> i32 {
        ((self.bits as i32) << 18) >> 18
    }
}

impl Display for ImmSk14 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:+}", self.bits.as_i32())
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ImmSk16 {
    bits: u16,
}

impl ImmSk16 {
    pub fn new(v: i64) -> Self {
        assert!(v < 0x8000 && v >= -0x8000);
        Self { bits: v as u16 }
    }

    #[inline]
    pub fn to_inst_layout(&self) -> u32 {
        self.bits << 10
    }

    #[inline]
    pub fn as_i32(&self) -> i32 {
        self.bits as i16 as i32
    }
}

impl Display for ImmSk16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:+}", self.bits.as_i32())
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ImmUk5 {
    bits: u32,
}

impl ImmUk5 {
    pub fn new(v: i64) -> Self {
        assert!(v >= 0 && v < 32);
        Self { bits: v as u32 }
    }

    #[inline]
    pub fn to_inst_layout(&self) -> u32 {
        self.bits << 10
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        self.bits
    }
}

impl Display for ImmUk5 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.bits)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ImmUk6 {
    bits: u32,
}

impl ImmUk6 {
    pub fn new(v: i64) -> Self {
        assert!(v >= 0 && v < 64);
        Self { bits: v as u32 }
    }

    #[inline]
    pub fn to_inst_layout(&self) -> u32 {
        self.bits << 10
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        self.bits
    }
}

impl Display for ImmUk6 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.bits)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ImmUk8 {
    bits: u32,
}

impl ImmUk8 {
    pub fn new(v: i64) -> Self {
        assert!(v >= 0 && v < 256);
        Self { bits: v as u32 }
    }

    #[inline]
    pub fn to_inst_layout(&self) -> u32 {
        self.bits << 10
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        self.bits
    }
}

impl Display for ImmUk8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.bits)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ImmUk12 {
    bits: u32,
}

impl ImmUk12 {
    pub fn new(v: i64) -> Self {
        assert!(v >= 0 && v < 4096);
        Self { bits: v as u32 }
    }

    #[inline]
    pub fn to_inst_layout(&self) -> u32 {
        self.bits << 10
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        self.bits
    }
}

impl Display for ImmUk12 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.bits)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ImmUk14 {
    bits: u32,
}

impl ImmUk14 {
    pub fn new(v: i64) -> Self {
        assert!(v >= 0 && v < 16384);
        Self { bits: v as u32 }
    }

    #[inline]
    pub fn to_inst_layout(&self) -> u32 {
        self.bits << 10
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        self.bits
    }
}

impl Display for ImmUk14 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.bits)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ImmUj5 {
    bits: u8,
}

impl ImmUj5 {
    pub fn new(v: i64) -> Self {
        assert!(v >= 0 && v < 32);
        Self { bits: v as u8 }
    }

    #[inline]
    pub fn to_inst_layout(&self) -> u32 {
        (self.bits as u32) << 5
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        self.bits as u32
    }
}

impl Display for ImmUj5 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.bits)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ImmUd5 {
    bits: u8,
}

impl ImmUd5 {
    pub fn new(v: i64) -> Self {
        assert!(v >= 0 && v < 32);
        Self { bits: v as u8 }
    }

    #[inline]
    pub fn to_inst_layout(&self) -> u32 {
        self.bits as u32
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        self.bits as u32
    }
}

impl Display for ImmUd5 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.bits)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ImmUd15 {
    bits: u16,
}

impl ImmUd15 {
    pub fn new(v: i64) -> Self {
        assert!(v >= 0 && v < 32768);
        Self { bits: v as u16 }
    }

    #[inline]
    pub fn to_inst_layout(&self) -> u32 {
        self.bits as u32
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        self.bits as u32
    }
}

impl Display for ImmUd15 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.bits)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ImmSj20 {
    bits: i32,
}

impl ImmSj20 {
    pub fn new(v: i64) -> Self {
        assert!(v < 0x80000 && v >= -0x80000);
        Self { bits: v as i32 }
    }

    #[inline]
    pub fn to_inst_layout(&self) -> u32 {
        (self.bits as u32) << 5
    }

    #[inline]
    pub fn as_i32(&self) -> i32 {
        (self.bits << 12) >> 12
    }
}

impl Display for ImmSj20 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:+}", self.bits.as_i32())
    }
}
