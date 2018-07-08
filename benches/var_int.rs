#![feature(test)]
extern crate mc_varint;
extern crate test;

use mc_varint::{VarInt, VarIntRead, VarIntWrite, VarLong, VarLongRead, VarLongWrite};
use std::io::Cursor;
use test::{Bencher, black_box};

macro_rules! bench_impl {
    ($var_type: ident, $read_func: ident, $write_func: ident,
    $read_bench: ident, $write_bench: ident, $convert_bench: ident, $convert_type: ident,
    $sample_data: expr) => {
#[bench]
fn $read_bench(b: &mut Bencher) {
    let mut cur = Cursor::new($sample_data);
    b.iter(move || {
        let _ = black_box(cur.$read_func()).unwrap();
        cur.set_position(0);
    })
}

#[bench]
fn $write_bench(b: &mut Bencher) {
    let mut cur = Cursor::new($sample_data);
    b.iter(move || {
        let v = $var_type::from($convert_type::min_value());
        let _ = black_box(cur.$write_func(v)).unwrap();
        cur.set_position(0);
    })
}

#[bench]
fn $convert_bench(b: &mut Bencher) {
    b.iter(|| {
        let _ = $var_type::from(black_box($convert_type::min_value()));
    })
}

    };
}

bench_impl!(VarInt, read_var_int, write_var_int, var_int_read, var_int_write, var_int_convert,
            i32, vec![0xff, 0xff, 0xff, 0xff, 0x07]);
bench_impl!(VarLong, read_var_long, write_var_long, var_long_read, var_long_write, var_long_convert,
            i64, vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01]);
