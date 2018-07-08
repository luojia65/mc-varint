use std::io;

macro_rules! var_impl {
    ($store_struct: ident, $read_trait: ident, $write_trait: ident, $read_func: ident, $write_func: ident,
    $conversation_type: ident, $size: expr, $error_too_long: expr) => {

// All $store_struct's should be checked, which means their `inner`'s are safe to write directly for
// readers to read.
#[derive(Debug, Eq, PartialEq)]
pub struct $store_struct {
    inner: [u8; $size]
}

impl Default for $store_struct {
    fn default() -> Self {
        $store_struct {
            inner: [0u8; $size]
        }
    }
}

pub trait $read_trait {
    fn $read_func(&mut self) -> io::Result<$store_struct>;
}

impl<R> $read_trait for R where R: io::Read {
    fn $read_func(&mut self) -> Result<$store_struct, io::Error> {
        let mut ans = $store_struct {
            inner: [0u8; $size]
        };
        let mut ptr = 0;
        let mut buf = [0u8];
        loop {
            self.read_exact(&mut buf)?;
            if ptr >= $size {
                return Err(io::Error::new(io::ErrorKind::InvalidData, $error_too_long));
            }
            ans.inner[ptr] = buf[0];
            ptr += 1;
            if buf[0] & 0b1000_0000 == 0 {
                return Ok(ans);
            }
        }
    }
}

pub trait $write_trait {
    fn $write_func(&mut self, n: $store_struct) -> io::Result<()>;
}

impl<W> $write_trait for W where W: io::Write {
    fn $write_func(&mut self, n: $store_struct) -> io::Result<()> {
        let mut buf = [0x00];
        let mut ptr = 0;
        loop {
            if n.inner[ptr] == 0 {
                break;
            }
            buf[0] = n.inner[ptr];
            self.write(&buf)?;
            ptr += 1;
            if ptr >= $size {
                break;
            }
        }
        // If no bytes written, that is, the $store_struct is equal to 0
        if ptr == 0 {
            // At that time, `buf` is still [0x00], let's write it
            self.write(&buf)?;
        }
        Ok(())
    }
}

impl From<$store_struct> for $conversation_type {
    fn from(v: $store_struct) -> Self {
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

impl From<$conversation_type> for $store_struct {
    fn from(n: $conversation_type) -> Self {
        let mut ans = $store_struct {
            inner: [0u8; $size]
        };
        let mut n = n;
        let mut ptr = 0;
        loop {
            let mut tmp = (n & 0b0111_1111) as u8;
            // There isn't a logical right-shift operator in Rust
            n = (n >> 7) & ($conversation_type::max_value() >> 6);
            if n != 0 {
                tmp |= 0b1000_0000;
            }
            ans.inner[ptr] = tmp;
            ptr += 1;
            if n == 0 || ptr >= $size {
                break;
            }
        }
        ans
    }
}
    };
}

var_impl!(VarInt, VarIntRead, VarIntWrite, read_var_int, write_var_int,
            i32, 5, "varint too long (length > 5)");
var_impl!(VarLong, VarLongRead, VarLongWrite, read_var_long, write_var_long,
            i64, 10, "varlong too long (length > 10)");

