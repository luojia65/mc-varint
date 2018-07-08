extern crate mc_varint;

use mc_varint::{VarInt, VarIntRead, VarIntWrite};
use std::io::{Cursor, ErrorKind};

#[test]
fn var_int_default() {
    let d: VarInt = Default::default();
    assert_eq!(d, Cursor::new(vec![0x00]).read_var_int().unwrap());
}

#[test]
fn var_int_size() {
    use std::mem::size_of;
    assert_eq!(5, size_of::<VarInt>());
}
#[test]
fn var_int_read_exact() {
    let cond = [
        (0,             vec![0x00]),
        (1,             vec![0x01]),
        (2,             vec![0x02]),
        (127,           vec![0x7f]),
        (128,           vec![0x80, 0x01]),
        (255,           vec![0xff, 0x01]),
        (2147483647,    vec![0xff, 0xff, 0xff, 0xff, 0x07]),
        (-1,            vec![0xff, 0xff, 0xff, 0xff, 0x0f]),
        (-2147483648,   vec![0x80, 0x80, 0x80, 0x80, 0x08]),
    ];
    for (num, raw) in cond.iter() {
        assert_eq!(*num, i32::from(Cursor::new(raw).read_var_int().unwrap()));
    }
}

#[test]
fn var_int_read_oversize() {
    let cond = [
        // The `0xff`'s are filled for data remaining after varint read
        (0,             vec![0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]),
        (1,             vec![0x01, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]),
        (-2147483648,   vec![0x80, 0x80, 0x80, 0x80, 0x08, 0xff, 0xff, 0xff]),
    ];
    for (num, raw) in cond.iter() {
        assert_eq!(*num, i32::from(Cursor::new(raw).read_var_int().unwrap()));
    }
}

#[test]
fn var_int_read_err() {
    let cond = [
        (ErrorKind::UnexpectedEof, vec![0xff]), // EOF
        (ErrorKind::UnexpectedEof, vec![0xff, 0xff, 0xff, 0xff, 0xff]), // EOF
        (ErrorKind::InvalidData,   vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff]), // Too long
    ];
    for (kind, raw) in cond.iter() {
        assert_eq!(*kind, Cursor::new(raw).read_var_int().unwrap_err().kind());
    }
}

#[test]
fn var_int_write_exact() {
    let cond = [
        (0,             vec![0x00]),
        (1,             vec![0x01]),
        (2,             vec![0x02]),
        (127,           vec![0x7f]),
        (128,           vec![0x80, 0x01]),
        (255,           vec![0xff, 0x01]),
        (2147483647,    vec![0xff, 0xff, 0xff, 0xff, 0x07]),
        (-1,            vec![0xff, 0xff, 0xff, 0xff, 0x0f]),
        (-2147483648,   vec![0x80, 0x80, 0x80, 0x80, 0x08]),
    ];
    for (num, raw) in cond.iter() {
        let mut cur = Cursor::new(Vec::new());
        let var_int = VarInt::from(*num);
        cur.write_var_int(var_int).unwrap();
        assert_eq!(cur.into_inner(), *raw);
    }
}

#[test]
fn beat_matching() {
    let cond = [
        -2147483648, -123456789, -12345678, -1234567, -123456, -12345, -1234, -123, -12, -1,
        0, 1, 12, 123, 1234, 12345, 123456, 1234567, 12345678, 123456789, 2147483647,
        127, 128, 129, -127, -128, -129, -256, -255, -254, 254, 255, 256,
        16383, 16384, 16385, -16383, -16384, -16385, 32767, 32768, 32769, -32767, -32768, -32769
    ];
    for num in cond.iter() {
        let mut cur = Cursor::new(Vec::new());
        let var_int = VarInt::from(*num);
        cur.write_var_int(var_int).unwrap();
        let vec = cur.into_inner();

        let num1 = i32::from(Cursor::new(vec).read_var_int().unwrap());
        assert_eq!(*num, num1);
    }
}