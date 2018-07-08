extern crate mc_varint;

use mc_varint::{VarInt, VarIntRead, VarIntWrite, VarLong, VarLongRead, VarLongWrite};
use std::io::{Cursor, ErrorKind};
use std::collections::HashSet;

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

macro_rules! beat_match {
    ($bm_func: ident, $var_type: ident, $conversation_type: ident,
    $write_func: ident, $read_func: ident) => {
#[test]
fn $bm_func() {
    // Generate conditions
    let mut cond = HashSet::new();
    let mut i = 1;
    loop {
        cond.insert(i - 1);
        cond.insert(i);
        cond.insert(i + 1);
        cond.insert(- i - 1);
        cond.insert(- i);
        cond.insert(- i + 1);
        if i <= $conversation_type::max_value() / 2 {
            i *= 2;
        } else {
            break;
        }
    }
    cond.insert(0);
    cond.insert($conversation_type::max_value());
    cond.insert($conversation_type::min_value());
    // Begin iteration
    for num in cond.iter() {
        let mut cur = Cursor::new(Vec::new());
        cur.$write_func($var_type::from(*num)).unwrap();
        let vec = cur.into_inner();

        let num1 = $conversation_type::from(Cursor::new(vec).$read_func().unwrap());
        assert_eq!(*num, num1);
    }
}
    };
}

beat_match!(var_int_beat_matching, VarInt, i32, write_var_int, read_var_int);
beat_match!(var_long_beat_matching, VarLong, i64, write_var_long, read_var_long);