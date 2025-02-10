// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::byte::Error::{OutOfBounds, UnexpectedEndOfFile};
use crate::leb128::{Leb128, Leb128Error};
use core::cell::RefCell;

#[derive(Debug, PartialEq)]
pub enum Error {
    OutOfBounds,
    UnexpectedEndOfFile,
    InvalidLEB128Encoding,
}

impl From<Leb128Error> for Error {
    fn from(value: Leb128Error) -> Self {
        match value {
            Leb128Error::InvalidEncoding => Error::InvalidLEB128Encoding,
            Leb128Error::IncompleteEncoding => Error::UnexpectedEndOfFile,
        }
    }
}

type Result<T, E = Error> = core::result::Result<T, E>;

pub struct ByteReader<'a> {
    data: &'a [u8],
    pos: RefCell<usize>,
}

impl<'a> ByteReader<'a> {
    fn length(&self) -> usize {
        self.data.len()
    }

    pub fn pos(&self) -> usize {
        *self.pos.borrow()
    }

    pub fn new(data: &'a [u8]) -> Self {
        ByteReader { data, pos: RefCell::new(0) }
    }

    pub fn read_u8(&self) -> Result<u8> {
        let mut pos = self.pos.borrow_mut();

        if *pos + 1 > self.length() {
            return Err(UnexpectedEndOfFile);
        }

        let res = self.data[*pos];
        *pos += 1;
        Ok(res)
    }

    pub fn peek_u8(&self) -> Result<u8> {
        let pos = self.pos.borrow();

        if *pos + 1 > self.length() {
            return Err(UnexpectedEndOfFile);
        }

        Ok(self.data[*pos])
    }

    pub fn read_u16(&self) -> Result<u16> {
        let mut pos = self.pos.borrow_mut();

        if *pos + 2 > self.length() {
            return Err(UnexpectedEndOfFile);
        }
        let _1 = self.data[*pos] as u16;
        let _2 = self.data[*pos + 1] as u16;
        let res = (_2 << 8) | _1;
        *pos += 2;
        Ok(res)
    }

    pub fn read_u32(&self) -> Result<u32> {
        let mut pos = self.pos.borrow_mut();

        if *pos + 4 > self.length() {
            return Err(UnexpectedEndOfFile);
        }

        let _1 = self.data[*pos] as u32;
        let _2 = self.data[*pos + 1] as u32;
        let _3 = self.data[*pos + 2] as u32;
        let _4 = self.data[*pos + 3] as u32;

        let res = _4 << 24 | _3 << 16 | _2 << 8 | _1;
        *pos += 4;
        Ok(res)
    }

    pub fn read_f32(&self) -> Result<f32> {
        let result = self.read_u32()?;
        Ok(f32::from_bits(result))
    }

    pub fn read_leb128_u32(&self) -> Result<u32> {
        let (result, consumed) = u32::read_leb128(self.peek_range(5)?)?;
        let mut pos = self.pos.borrow_mut();
        *pos += consumed;
        Ok(result)
    }

    pub fn read_leb128_u64(&self) -> Result<u64> {
        let (result, consumed) = u64::read_leb128(self.peek_range(10)?)?;
        let mut pos = self.pos.borrow_mut();
        *pos += consumed;
        Ok(result)
    }

    pub fn read_leb128_i32(&self) -> Result<i32> {
        let (result, consumed) = i32::read_leb128(self.peek_range(5)?)?;
        let mut pos = self.pos.borrow_mut();
        *pos += consumed;
        Ok(result)
    }

    pub fn read_leb128_i64(&self) -> Result<i64> {
        let (result, consumed) = i64::read_leb128(self.peek_range(10)?)?;
        let mut pos = self.pos.borrow_mut();
        *pos += consumed;
        Ok(result)
    }

    pub fn read_u64(&self) -> Result<u64> {
        let mut pos = self.pos.borrow_mut();

        if *pos + 8 > self.length() {
            return Err(UnexpectedEndOfFile);
        }

        let _1 = self.data[*pos] as u64;
        let _2 = self.data[*pos + 1] as u64;
        let _3 = self.data[*pos + 2] as u64;
        let _4 = self.data[*pos + 3] as u64;
        let _5 = self.data[*pos + 4] as u64;
        let _6 = self.data[*pos + 5] as u64;
        let _7 = self.data[*pos + 6] as u64;
        let _8 = self.data[*pos + 7] as u64;

        let res = _8 << 56 | _7 << 48 | _6 << 40 | _5 << 32 | _4 << 24 | _3 << 16 | _2 << 8 | _1;
        *pos += 8;
        Ok(res)
    }

    pub fn read_f64(&self) -> Result<f64> {
        let result = self.read_u64()?;
        Ok(f64::from_bits(result))
    }

    pub fn read_range<T: TryInto<usize>>(&self, len: T) -> Result<&[u8]>
    where
        <T as TryInto<usize>>::Error: std::fmt::Debug,
    {
        let len = len.try_into().unwrap();
        let mut pos = self.pos.borrow_mut();

        let data = self.data;
        if *pos + len > data.len() {
            return Err(UnexpectedEndOfFile);
        }

        let result = &data[*pos..*pos + len];
        *pos += len;

        Ok(result)
    }

    pub fn read_variable_length<N: TryInto<usize> + Copy>(&self) -> Result<&[u8]> {
        let mut pos = self.pos.borrow_mut();
        let data = self.data;

        // Read N bytes from data to get the length
        let len_bytes = &data[*pos..*pos + std::mem::size_of::<N>()];
        *pos += std::mem::size_of::<N>();

        // Convert the read bytes into an integer of type N
        let len = match std::mem::size_of::<N>() {
            1 => len_bytes[0] as usize,
            2 => u16::from_le_bytes(len_bytes.try_into().unwrap()) as usize,
            4 => u32::from_le_bytes(len_bytes.try_into().unwrap()) as usize,
            8 => u64::from_le_bytes(len_bytes.try_into().unwrap()) as usize,
            _ => return Err(UnexpectedEndOfFile),
        };

        // Read the actual payload of size `len`
        if *pos + len > data.len() {
            return Err(UnexpectedEndOfFile);
        }

        let result = &data[*pos..*pos + len];
        *pos += len;

        Ok(result)
    }

    pub fn peek_range(&self, len: usize) -> Result<&[u8]> {
        let pos = self.pos.borrow();
        let data = self.data;
        let end_pos = (*pos + len).min(data.len());

        let result = &data[*pos..end_pos];
        Ok(result)
    }

    pub fn seek(&self, offset: isize) -> Result<usize> {
        let mut pos = self.pos.borrow_mut();
        let new_pos = if offset.is_negative() {
            // Ensure we do not go below 0
            pos.saturating_sub(offset.abs() as usize)
        } else {
            // Ensure we do not go beyond the end of the data
            pos.saturating_add(offset as usize)
        };

        let data_len = self.length();

        if new_pos > data_len {
            Err(OutOfBounds)
        } else {
            *pos = new_pos;
            Ok(*pos)
        }
    }

    pub fn eof(&self) -> bool {
        *self.pos.borrow() >= self.length()
    }
}

#[cfg(test)]
mod tests {
	use crate::byte::Error::OutOfBounds;
	use crate::byte::{ByteReader, Error};

	#[test]
    fn read_empty() {
        let data: &[u8] = &[];
        let test_instance = ByteReader::new(data);

        assert!(test_instance.read_u8().is_err());
        assert!(test_instance.read_u16().is_err());
        assert!(test_instance.read_u32().is_err());
        assert!(test_instance.read_u64().is_err());
    }

    #[test]
    fn read_u8() {
        let data: &[u8] = &[0x05, 0x06, 0x07, 0x08];
        let test_instance = ByteReader::new(data);

        assert_eq!(test_instance.read_u8().unwrap(), 0x05);
        assert_eq!(test_instance.read_u8().unwrap(), 0x06);
        assert_eq!(test_instance.read_u8().unwrap(), 0x07);
        assert_eq!(test_instance.read_u8().unwrap(), 0x08);
    }

    #[test]
    fn read_u16() {
        let data: &[u8] = &[0x05, 0x06, 0x07, 0x08];
        let test_instance = ByteReader::new(data);

        assert_eq!(test_instance.read_u16().unwrap(), 0x0605); // Little-endian: 0x0506
        assert_eq!(test_instance.read_u16().unwrap(), 0x0807); // Little-endian: 0x0708
    }

    #[test]
    fn read_u32() {
        let data: &[u8] = &[0x05, 0x06, 0x07, 0x08];
        let test_instance = ByteReader::new(data);

        assert_eq!(test_instance.read_u32().unwrap(), 0x08070605); // Little-endian: 0x05060708
    }

    #[test]
    fn read_f32() {
        let cases = [
            // Little-endian bytes for 1.0f32
            (vec![0x00, 0x00, 0x80, 0x3F], 1.0f32),
            // Little-endian bytes for -1.0f32
            (vec![0x00, 0x00, 0x80, 0xBF], -1.0f32),
            // Little-endian bytes for 0.0f32
            (vec![0x00, 0x00, 0x00, 0x00], 0.0f32),
            // Little-endian bytes for -0.0f32
            (vec![0x00, 0x00, 0x00, 0x80], -0.0f32),
            // Little-endian bytes for 2.5f32
            (vec![0x00, 0x00, 0x20, 0x40], 2.5f32),
            // Little-endian bytes for -2.5f32
            (vec![0x00, 0x00, 0x20, 0xC0], -2.5f32),
        ];

        for (data, expected) in cases.iter() {
            let test_instance = ByteReader::new(data);
            assert_eq!(test_instance.read_f32().unwrap(), *expected);
        }
    }

    #[test]
    fn read_u64() {
        let data: &[u8] = &[0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];
        let test_instance = ByteReader::new(data);

        assert_eq!(test_instance.read_u64().unwrap(), 0x100F0E0D0C0B0A09); // Little-endian: 0x090A0B0C0D0E0F10
    }

    #[test]
    fn read_f64() {
        let cases = [
            // Little-endian bytes for 1.0f64
            (vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x3F], 1.0f64),
            // Little-endian bytes for -1.0f64
            (vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0xBF], -1.0f64),
            // Little-endian bytes for 0.0f64
            (vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 0.0f64),
            // Little-endian bytes for -0.0f64
            (vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80], -0.0f64),
            // Little-endian bytes for 2.5f64
            (vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x40], 2.5f64),
            // Little-endian bytes for -2.5f64
            (vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0xC0], -2.5f64),
        ];

        for (data, expected) in cases.iter() {
            let test_instance = ByteReader::new(data);

            assert_eq!(test_instance.read_f64().unwrap(), *expected);
        }
    }


    #[test]
    fn read_range() {
        let data: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let test_instance = ByteReader::new(data);

        assert_eq!(test_instance.read_range(4).unwrap().as_ref(), [0x01, 0x02, 0x03, 0x04]);
        assert_eq!(test_instance.read_u8().unwrap(), 0x05);
        assert_eq!(test_instance.read_range(2).unwrap().as_ref(), [0x06, 0x07]);
        assert_eq!(test_instance.read_u8().unwrap(), 0x08);
    }

    #[test]
    fn read_range_out_of_bounds() {
        let data: &[u8] = &[0x01, 0x02, 0x03, 0x04];
        let test_instance = ByteReader::new(data);

        test_instance.seek(3).unwrap();
        assert!(test_instance.read_range(2).is_err());
    }

    #[test]
    fn seek() {
        let data = b"Hello, world!";
        let reader = ByteReader::new(&data[..]);

        // Test seeking forward within bounds
        assert_eq!(reader.seek(7).unwrap(), 7);
        assert_eq!(reader.seek(3).unwrap(), 10);

        // Test seeking backward within bounds
        assert_eq!(reader.seek(-5).unwrap(), 5);
        assert_eq!(reader.seek(-10).unwrap(), 0); // Should clamp to 0

        // Test seeking beyond the data length
        assert_eq!(reader.seek(50).err().unwrap(), OutOfBounds);
    }

    #[test]
    fn read_leb128_u32_single_byte() {
        let data = [0x7F]; // 127 in LEB128
        let test_instance = ByteReader::new(&data);
        let result = test_instance.read_leb128_u32().unwrap();
        assert_eq!(result, 127);
    }

    #[test]
    fn read_leb128_u32_multiple_bytes() {
        let data = [0xE5, 0x8E, 0x26]; // 624485 in LEB128
        let test_instance = ByteReader::new(&data);
        let result = test_instance.read_leb128_u32().unwrap();
        assert_eq!(result, 624485);
    }

    #[test]
    fn read_leb128_u32_max_u32() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF, 0x0F]; // Maximum u32 in LEB128
        let test_instance = ByteReader::new(&data);
        let result = test_instance.read_leb128_u32().unwrap();
        assert_eq!(result, 4294967295); // Max u32 value
    }

    #[test]
    fn read_leb128_u32_unexpected_eof() {
        let data = [0x80]; // Incomplete LEB128 encoding
        let test_instance = ByteReader::new(&data);
        let result = test_instance.read_leb128_u32();
        assert!(matches!(result, Err(Error::UnexpectedEndOfFile)));
    }

    #[test]
    fn read_leb128_u32_invalid_encoding() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF]; // Too many bytes for a valid u32
        let test_instance = ByteReader::new(&data);
        let result = test_instance.read_leb128_u32();
        assert!(matches!(result, Err(Error::InvalidLEB128Encoding)));
    }

    #[test]
    fn eof_empty_data() {
        let test_instance = ByteReader::new(&[]);
        assert!(test_instance.eof());
    }

    #[test]
    fn read_leb128_i32_positive_single_byte() {
        let data = [0x3F]; // 63 in LEB128
        let test_instance = ByteReader::new(&data);
        let result = test_instance.read_leb128_i32().unwrap();
        assert_eq!(result, 63);
        assert!(test_instance.eof());
    }

    #[test]
    fn read_leb128_i32_negative_single_byte() {
        let data = [0x41]; // -63 in LEB128
        let test_instance = ByteReader::new(&data);
        let result = test_instance.read_leb128_i32().unwrap();
        assert_eq!(result, -63);
        assert!(test_instance.eof());
    }

    #[test]
    fn read_leb128_i32_positive_multiple_bytes() {
        let data = [0xE5, 0x8E, 0x26]; // 624485 in LEB128
        let test_instance = ByteReader::new(&data);
        let result = test_instance.read_leb128_i32().unwrap();
        assert_eq!(result, 624485);
        assert!(test_instance.eof());
    }

    #[test]
    fn read_leb128_i32_negative_multiple_bytes() {
        let data = [0x9B, 0xF1, 0x59]; // -624485 in LEB128
        let test_instance = ByteReader::new(&data);
        let result = test_instance.read_leb128_i32().unwrap();
        assert_eq!(result, -624485);
        assert!(test_instance.eof());
    }

    #[test]
    fn read_leb128_i32_max_i32() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF, 0x07]; // Maximum i32 in LEB128 (2147483647)
        let test_instance = ByteReader::new(&data);
        let result = test_instance.read_leb128_i32().unwrap();
        assert_eq!(result, i32::MAX); // Max i32 value
        assert!(test_instance.eof());
    }

    #[test]
    fn read_leb128_i32_min_i32() {
        let data = [0x80, 0x80, 0x80, 0x80, 0x78]; // Minimum i32 in LEB128 (-2147483648)
        let test_instance = ByteReader::new(&data);
        let result = test_instance.read_leb128_i32().unwrap();
        assert_eq!(result, i32::MIN);
        assert!(test_instance.eof());
    }

    #[test]
    fn read_leb128_i32_unexpected_eof() {
        let data = [0x80]; // Incomplete LEB128 encoding
        let test_instance = ByteReader::new(&data);
        let result = test_instance.read_leb128_i32();
        assert!(matches!(result, Err(Error::UnexpectedEndOfFile)));
    }

    #[test]
    fn read_leb128_i32_invalid_encoding() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF]; // Too many bytes for a valid i32
        let test_instance = ByteReader::new(&data);
        let result = test_instance.read_leb128_i32();
        assert!(matches!(result, Err(Error::InvalidLEB128Encoding)));
    }

    #[test]
    fn read_leb128_i64_max_i64() {
        let data = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0]; // Maximum i64 in LEB128
        let test_instance = ByteReader::new(&data);
        let result = test_instance.read_leb128_i64().unwrap();
        assert_eq!(result, i64::MAX);
        assert!(test_instance.eof());
    }

    #[test]
    fn read_leb128_i64_min_i64() {
        let data = [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01]; // Minimum i64 in LEB128
        let test_instance = ByteReader::new(&data);
        let result = test_instance.read_leb128_i64().unwrap();
        assert_eq!(result, i64::MIN);
        assert!(test_instance.eof());
    }

    #[test]
    fn read_leb128_u64_max_u64() {
        let data = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]; // Maximum u64 in LEB128
        let test_instance = ByteReader::new(&data);
        let result = test_instance.read_leb128_u64().unwrap();
        assert_eq!(result, u64::MAX);
        assert!(test_instance.eof());
    }

    #[test]
    fn peek_range_within_bounds() {
        let given = [1, 2, 3, 4, 5];
        let test_instance = ByteReader::new(&given);
        let result = test_instance.peek_range(3).unwrap();
        assert_eq!(result, &[1, 2, 3]);
    }

    #[test]
    fn peek_range_past_end() {
        let given = [1, 2, 3, 4, 5];
        let test_instance = ByteReader::new(&given);
        let result = test_instance.peek_range(10).unwrap();
        assert_eq!(result, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn peek_range_empty_data() {
        let given = [];
        let test_instance = ByteReader::new(&given);
        let result = test_instance.peek_range(5).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn peek_range_zero_len() {
        let given = [1, 2, 3, 4, 5];
        let test_instance = ByteReader::new(&given);
        let result = test_instance.peek_range(0).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn peek_range_after_advancing_pos() {
        let given = [1, 2, 3, 4, 5];
        let test_instance = ByteReader::new(&given);
        *test_instance.pos.borrow_mut() = 2;
        let result = test_instance.peek_range(2).unwrap();
        assert_eq!(result, &[3, 4]);
    }

    #[test]
    fn peek_u8_at_start() {
        let test_instance = ByteReader::new(&[0x01, 0x02, 0x03]);
        assert_eq!(test_instance.peek_u8().unwrap(), 0x01);
        assert_eq!(test_instance.peek_u8().unwrap(), 0x01); // Position should not advance
    }

    #[test]
    fn peek_u8_past_end() {
        let test_instance = ByteReader::new(&[0x01, 0x02, 0x03]);
        test_instance.read_range(3).unwrap(); // Advance past the last byte
        assert!(test_instance.peek_u8().is_err());
    }

    #[test]
    fn peek_u8_empty_data() {
        let test_instance = ByteReader::new(&[]);
        assert!(test_instance.peek_u8().is_err()); // Should return an error
    }

    mod read_variable_length {
		use crate::ByteReader;

		#[test]
        fn test_u8() {
            let data: &[u8] = &[0x01, b'a', b'b', b'c', b'd', b'e', b'f'];
            let test_instance = ByteReader::new(data);

            let result = test_instance.read_variable_length::<u8>().unwrap();
            assert_eq!(result, b"a");
        }

        #[test]
        fn test_u16() {
            let data: &[u8] = &[0x02, 0x00, b'a', b'b', b'c', b'd', b'e', b'f'];
            let test_instance = ByteReader::new(data);

            let result = test_instance.read_variable_length::<u16>().unwrap();
            assert_eq!(result, b"ab");
        }

        #[test]
        fn test_u32() {
            let data: &[u8] = &[0x03, 0x00,0x00,0x00, b'a', b'b', b'c', b'd', b'e', b'f'];
            let test_instance = ByteReader::new(data);

            let result = test_instance.read_variable_length::<u32>().unwrap();
            assert_eq!(result, b"abc");
        }

        #[test]
        fn test_u64() {
            let data: &[u8] = &[0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, b'a', b'b', b'c', b'd', b'e', b'f'];
            let test_instance = ByteReader::new(data);

            let result = test_instance.read_variable_length::<u64>().unwrap();
            assert_eq!(result, b"abcd");
        }
    }
}
