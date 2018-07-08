use std::io;

// All varint's should be checked, which means their `inner`'s are safe to write directly for
// readers to read.
#[derive(Debug, Eq, PartialEq)]
pub struct VarInt {
    inner: [u8; 5]
}

impl Default for VarInt {
    fn default() -> Self {
        VarInt {
            inner: [0u8; 5]
        }
    }
}

pub trait VarIntRead {
    fn read_var_int(&mut self) -> io::Result<VarInt>;
}

impl<R> VarIntRead for R where R: io::Read {
    fn read_var_int(&mut self) -> Result<VarInt, io::Error> {
        let mut ans = VarInt {
            inner: [0u8; 5]
        };
        let mut ptr = 0;
        let mut buf = [0u8];
        loop {
            self.read_exact(&mut buf)?;
            if ptr >= 5 {
                return Err(io::Error::new(io::ErrorKind::InvalidData,
                                          "varint too long (length > 5)"));
            }
            ans.inner[ptr] = buf[0];
            ptr += 1;
            if buf[0] & 0b1000_0000 == 0 {
                return Ok(ans);
            }
        }
    }
}

pub trait VarIntWrite {
    fn write_var_int(&mut self, n: VarInt) -> io::Result<()>;
}

impl<W> VarIntWrite for W where W: io::Write {
    fn write_var_int(&mut self, n: VarInt) -> io::Result<()> {
        let mut buf = [0x00];
        let mut ptr = 0;
        loop {
            if n.inner[ptr] == 0 {
                break;
            }
            buf[0] = n.inner[ptr];
            self.write(&buf)?;
            ptr += 1;
            if ptr >= 5 {
                break;
            }
        }
        // If no bytes written, that is, the varint is equal to 0
        if ptr == 0 {
            // At that time, `buf` is still [0x00], let's write it
            self.write(&buf)?;
        }
        Ok(())
    }
}

impl From<VarInt> for i32 {
    fn from(v: VarInt) -> Self {
        let mut ans = 0 as Self;
        let mut ptr = 0;
        loop {
            let value = (v.inner[ptr] & 0b0111_1111) as Self;
            ans |= value << (7 * ptr as Self);
            if v.inner[ptr] & 0b1000_0000 == 0 {
                return ans;
            }
            ptr += 1;
        }
    }
}

impl From<i32> for VarInt {
    fn from(n: i32) -> Self {
        let mut ans = VarInt {
            inner: [0u8; 5]
        };
        let mut n = n;
        let mut ptr = 0;
        loop {
            let mut tmp = (n & 0b0111_1111) as u8;
            // There isn't a logical right-shift operator in Rust
            n = (n >> 7) & (i32::max_value() >> 6);
            if n != 0 {
                tmp |= 0b1000_0000;
            }
            ans.inner[ptr] = tmp;
            ptr += 1;
            if n == 0 || ptr >= 5 {
                break;
            }
        }
        ans
    }
}