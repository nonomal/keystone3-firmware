use crate::errors::{Result, SolanaError};
use crate::read::Read;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

pub struct Compact<T> {
    compact_length: u32,
    pub(crate) data: Vec<T>,
}

impl<T: Read<T>> Compact<T> {
    fn new(raw: &mut Vec<u8>) -> Result<Compact<T>> {
        let length: u32 = Compact::<T>::read_length(raw)?;
        if length as usize > raw.len() {
            return Err(SolanaError::InvalidData(
                "compact length exceeds remaining data".to_string(),
            ));
        }
        let mut compact = Compact {
            compact_length: length,
            data: vec![],
        };
        for _ in 0..compact.compact_length {
            compact.data.push(T::read(raw)?);
        }
        Ok(compact)
    }

    fn read_length(raw: &mut Vec<u8>) -> Result<u32> {
        let mut len: u32 = 0;
        for byte_index in 0..3u32 {
            if raw.is_empty() {
                return Err(SolanaError::InvalidData("compact length".to_string()));
            }
            let element: u32 = raw.remove(0) as u32;
            if byte_index == 2 && (element & 0x7c) != 0 {
                return Err(SolanaError::InvalidData(
                    "compact length overflow".to_string(),
                ));
            }
            let value = element & 0x7f;
            if byte_index > 0 && value == 0 && (element & 0x80) == 0 {
                return Err(SolanaError::InvalidData(
                    "non-canonical compact length".to_string(),
                ));
            }
            len |= value << (byte_index * 7);
            if (element & 0x80) == 0 {
                return Ok(len);
            }
        }
        Err(SolanaError::InvalidData(
            "compact length is too long".to_string(),
        ))
    }
}

impl<T: Read<T>> Read<Compact<T>> for Compact<T> {
    fn read(raw: &mut Vec<u8>) -> Result<Compact<T>> {
        Compact::new(raw)
    }
}

#[cfg(test)]
mod tests {
    use super::Compact;

    #[test]
    fn rejects_overlong_overflowing_and_non_canonical_lengths() {
        assert!(Compact::<u8>::read_length(&mut vec![0x83, 0x80, 0x80, 0x80, 0x00]).is_err());
        assert!(Compact::<u8>::read_length(&mut vec![0x80, 0x00]).is_err());
        assert!(Compact::<u8>::read_length(&mut vec![0xff, 0xff, 0x04]).is_err());
    }

    #[test]
    fn accepts_canonical_shortvec_lengths() {
        assert_eq!(Compact::<u8>::read_length(&mut vec![0x00]).unwrap(), 0);
        assert_eq!(Compact::<u8>::read_length(&mut vec![0x7f]).unwrap(), 127);
        assert_eq!(
            Compact::<u8>::read_length(&mut vec![0x80, 0x01]).unwrap(),
            128
        );
        assert_eq!(
            Compact::<u8>::read_length(&mut vec![0xff, 0xff, 0x03]).unwrap(),
            u16::MAX as u32
        );
    }
}
