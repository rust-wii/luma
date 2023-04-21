//! ``sysconf`` module of ``luma_core``.
//!
//! Provides access to the Wii system settings, as set by the system menu.

use crate::ios;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

#[derive(Debug, Clone)]
enum Item {
    BigArray(Vec<u8>),
    SmallArray(Vec<u8>),
    Byte(u8),
    Short(u16),
    Long(u32),
    LongLong(u64),
    Bool(bool),
}

#[derive(Clone, Copy)]
pub enum AspectRatio {
    A4_3,
    A16_9,
    Unknown,
}

impl fmt::Debug for AspectRatio {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AspectRatio::A4_3 => write!(fmt, "4:3"),
            AspectRatio::A16_9 => write!(fmt, "16:9"),
            AspectRatio::Unknown => write!(fmt, "unknown"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Scan {
    Interlaced,
    Progressive,
    Unknown,
}

impl fmt::Debug for Scan {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Scan::Interlaced => write!(fmt, "interlaced"),
            Scan::Progressive => write!(fmt, "progressive"),
            Scan::Unknown => write!(fmt, "unknown"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum RefreshRate {
    R50,
    R60,
}

impl fmt::Debug for RefreshRate {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RefreshRate::R50 => write!(fmt, "50 Hz"),
            RefreshRate::R60 => write!(fmt, "60 Hz"),
        }
    }
}

/// Parsed structure containing the system settings.
pub struct Sysconf(BTreeMap<String, Item>);

impl Sysconf {
    fn parse(data: [u8; 0x4000]) -> Result<Sysconf, core::str::Utf8Error> {
        assert_eq!(&data[..4], b"SCv0");

        let num_items = u16::from_be_bytes([data[4], data[5]]);
        let mut offsets = Vec::with_capacity(num_items as usize);
        for i in 0..num_items {
            let offset = u16::from_be_bytes([data[6 + 2 * i as usize], data[7 + 2 * i as usize]]);
            offsets.push(offset);
        }

        let mut tree = BTreeMap::new();
        for offset in offsets {
            let type_and_name_length = data[offset as usize];
            let type_ = type_and_name_length >> 5;
            let name_length = 1 + type_and_name_length & 0x1f;
            let name = alloc::str::from_utf8(
                &data[offset as usize + 1..offset as usize + 1 + name_length as usize],
            )?
            .to_string();

            let offset = offset as usize + 1 + name_length as usize;
            match type_ {
                1 => {
                    let length = u16::from_be_bytes([data[offset], data[offset + 1]]);
                    let value = &data[offset + 2..offset + 2 + length as usize + 1];
                    tree.insert(name, Item::BigArray(value.to_vec()));
                }
                2 => {
                    let length = data[offset];
                    let value = &data[offset + 1..offset + 1 + length as usize + 1];
                    tree.insert(name, Item::SmallArray(value.to_vec()));
                }
                3 => {
                    let value = data[offset];
                    tree.insert(name, Item::Byte(value));
                }
                4 => {
                    let value = u16::from_be_bytes([data[offset], data[offset + 1]]);
                    tree.insert(name, Item::Short(value));
                }
                5 => {
                    let value = u32::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3],
                    ]);
                    tree.insert(name, Item::Long(value));
                }
                6 => {
                    let value = u64::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3],
                        data[offset + 4],
                        data[offset + 5],
                        data[offset + 6],
                        data[offset + 7],
                    ]);
                    tree.insert(name, Item::LongLong(value));
                }
                7 => {
                    let value = data[offset] != 0;
                    tree.insert(name, Item::Bool(value));
                }
                _ => unreachable!("Unknown type {}!", type_),
            }
        }
        Ok(Sysconf(tree))
    }

    /// Return the preferred aspect ratio.
    pub fn aspect_ratio(&self) -> AspectRatio {
        match self.0.get("IPL.AR") {
            Some(Item::Byte(value)) if *value == 0 => AspectRatio::A4_3,
            Some(Item::Byte(_)) => AspectRatio::A16_9,
            _ => AspectRatio::Unknown,
        }
    }

    /// Return whether interlaced or progressive scan is preferred.
    pub fn progressive(&self) -> Scan {
        match self.0.get("IPL.PGS") {
            Some(Item::Byte(value)) if *value == 0 => Scan::Interlaced,
            Some(Item::Byte(_)) => Scan::Progressive,
            _ => Scan::Unknown,
        }
    }

    /// Return the preferred refresh rate.
    pub fn refresh_rate(&self) -> RefreshRate {
        match self.0.get("IPL.E60") {
            Some(Item::Byte(value)) if *value == 0 => RefreshRate::R50,
            _ => RefreshRate::R60,
        }
    }
}

/// Read and parse the SYSCONF file from /shared2/sys on the NAND.
pub fn read_and_parse() -> Result<Sysconf, i32> {
    let fd = ios::open("/shared2/sys/SYSCONF\0", ios::Mode::Read)?;

    let mut buf = [0u8; 0x4000];
    ios::read(fd, &mut buf)?;
    ios::close(fd)?;

    // TODO: Use a better way to bubble up errors.
    Sysconf::parse(buf).map_err(|_| -103)
}
