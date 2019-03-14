#![feature(test)]
extern crate test;
extern crate mc_varint;

use std::io;
use mc_varint::{VarIntRead, VarIntWrite};

// FF*8 [0, 7], 7F [8]

struct VarI64Gen {
    state: u8
}

impl VarI64Gen {
    fn new() -> Self {
        Self { state: 0 }
    }
}

impl io::Read for VarI64Gen {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        buf[0] = if self.state != 8 { 0xff } else { 0x7f };
        self.state = (self.state + 1) % 9;
        Ok(1)
    }
}
// FF*4 [0, 3], 7F [4]

struct VarI32Gen {
    state: u8
}

impl VarI32Gen {
    fn new() -> Self {
        Self { state: 0 }
    }
}

impl io::Read for VarI32Gen {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        buf[0] = if self.state != 4 { 0xff } else { 0x7f };
        self.state = (self.state + 1) % 5;
        Ok(1)
    }
}

#[bench]
fn read_i64(b: &mut test::Bencher) {
    b.iter(|| {
        let mut src = VarI64Gen::new();
        let _ = test::black_box(src.read_var_i64()).unwrap();
    })
}

#[bench]
fn read_u64(b: &mut test::Bencher) {
    b.iter(|| {
        let mut src = VarI64Gen::new();
        let _ = test::black_box(src.read_var_u64()).unwrap();
    })
}

#[bench]
fn read_i32(b: &mut test::Bencher) {
    b.iter(|| {
        let mut src = VarI32Gen::new();
        let _ = test::black_box(src.read_var_i32()).unwrap();
    })
}

#[bench]
fn read_u32(b: &mut test::Bencher) {
    b.iter(|| {
        let mut src = VarI32Gen::new();
        let _ = test::black_box(src.read_var_u32()).unwrap();
    })
}

struct Blackhole;

impl io::Write for Blackhole {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[bench]
fn write_i64(b: &mut test::Bencher) {
    b.iter(|| {
        let mut dst = Blackhole;
        let _ = test::black_box(dst.write_var_i64(i64::min_value())).unwrap();
    })
}

#[bench]
fn write_u64(b: &mut test::Bencher) {
    b.iter(|| {
        let mut dst = Blackhole;
        let _ = test::black_box(dst.write_var_u64(u64::min_value())).unwrap();
    })
}

#[bench]
fn write_i32(b: &mut test::Bencher) {
    b.iter(|| {
        let mut dst = Blackhole;
        let _ = test::black_box(dst.write_var_i32(i32::min_value())).unwrap();
    })
}

#[bench]
fn write_u32(b: &mut test::Bencher) {
    b.iter(|| {
        let mut dst = Blackhole;
        let _ = test::black_box(dst.write_var_u32(u32::min_value())).unwrap();
    })
}