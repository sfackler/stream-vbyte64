#![feature(test)]
extern crate rand;
extern crate stream_vbyte64;
extern crate test;

use rand::{Rng, XorShiftRng};
use test::Bencher;

fn one_byte_values() -> Vec<u64> {
    (0..4096).map(|v| v & 0xff).collect()
}

fn random_values() -> Vec<u64> {
    let mut rng = XorShiftRng::new_unseeded();
    (0..4096)
        .map(|_| {
            let bytes = rng.gen_range(0, 9);
            rng.gen::<u64>() & ((1 << bytes * 8) - 1)
        })
        .collect()
}

fn scalar_encode(values: &[u64], b: &mut Bencher) {
    let mut keys = vec![0; stream_vbyte64::keys_len(values.len())];
    let mut data = vec![0; values.len() * 8];

    b.iter(|| unsafe {
        stream_vbyte64::encode_scalar(&values, &mut keys, &mut data)
    });
    b.bytes = 8 * values.len() as u64;
}

fn scalar_decode(values: &[u64], b: &mut Bencher) {
    let mut keys = vec![0; stream_vbyte64::keys_len(values.len())];
    let mut data = vec![0; values.len() * 8];
    let len = unsafe { stream_vbyte64::encode_scalar(&values, &mut keys, &mut data) };
    let mut decoded = vec![0; values.len()];

    b.iter(|| unsafe {
        stream_vbyte64::decode_scalar(&mut decoded, &keys, &data[..len])
    });
    b.bytes = 8 * values.len() as u64;
}

#[bench]
fn scalar_encode_one_byte(b: &mut Bencher) {
    let values = one_byte_values();
    scalar_encode(&values, b);
}

#[bench]
fn scalar_decode_one_byte(b: &mut Bencher) {
    let values = one_byte_values();
    scalar_decode(&values, b);
}

#[bench]
fn scalar_encode_random(b: &mut Bencher) {
    let values = random_values();
    scalar_encode(&values, b);
}

#[bench]
fn scalar_decode_random(b: &mut Bencher) {
    let values = random_values();
    scalar_decode(&values, b);
}
