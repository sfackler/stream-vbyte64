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

fn compressed_data_len(values: &[u64], b: &mut Bencher) {
    let data_len = stream_vbyte64::max_compressed_len(values.len());
    let mut buf = vec![0; data_len];
    stream_vbyte64::encode(values, &mut buf);
    b.iter(|| stream_vbyte64::compressed_data_len(values.len(), &buf));
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
    unsafe { stream_vbyte64::encode_scalar(&values, &mut keys, &mut data) };
    let mut decoded = vec![0; values.len()];

    b.iter(|| unsafe {
        stream_vbyte64::decode_scalar(&mut decoded, &keys, &data)
    });
    b.bytes = 8 * values.len() as u64;
}

fn encode(values: &[u64], b: &mut Bencher) {
    let data_len = stream_vbyte64::max_compressed_len(values.len());
    let mut buf = vec![0; data_len];

    b.iter(|| stream_vbyte64::encode(&values, &mut buf));
    b.bytes = 8 * values.len() as u64
}

fn decode(values: &[u64], b: &mut Bencher) {
    let data_len = stream_vbyte64::max_compressed_len(values.len());
    let mut buf = vec![0; data_len];
    stream_vbyte64::encode(&values, &mut buf);
    let mut decoded = vec![0; values.len()];

    b.iter(|| stream_vbyte64::decode(&mut decoded, &buf));
    b.bytes = 8 * values.len() as u64
}

#[bench]
fn compressed_data_len_one_byte(b: &mut Bencher) {
    let values = one_byte_values();
    compressed_data_len(&values, b);
}

#[bench]
fn compressed_data_len_random(b: &mut Bencher) {
    let values = random_values();
    compressed_data_len(&values, b);
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

#[bench]
fn encode_one_byte(b: &mut Bencher) {
    let values = one_byte_values();
    encode(&values, b);
}

#[bench]
fn decode_one_byte(b: &mut Bencher) {
    let values = one_byte_values();
    decode(&values, b);
}

#[bench]
fn encode_random(b: &mut Bencher) {
    let values = random_values();
    encode(&values, b);
}

#[bench]
fn decode_random(b: &mut Bencher) {
    let values = random_values();
    decode(&values, b);
}
