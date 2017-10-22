extern crate stdsimd;

use std::ptr;

pub fn keys_len(values: usize) -> usize {
    ((values + 7) / 8) * 3
}

unsafe fn encode_single(value: u64, out: &mut *mut u8) -> u8 {
    let value = value.to_le();
    if value < 1 << 8 {
        **out = value as u8;
        *out = out.offset(1);
        0
    } else if value < 1 << 16 {
        ptr::copy_nonoverlapping(&value as *const u64 as *const u8, *out, 2);
        *out = out.offset(2);
        1
    } else if value < 1 << 24 {
        ptr::copy_nonoverlapping(&value as *const u64 as *const u8, *out, 3);
        *out = out.offset(3);
        2
    } else if value < 1 << 32 {
        ptr::copy_nonoverlapping(&value as *const u64 as *const u8, *out, 4);
        *out = out.offset(4);
        3
    } else if value < 1 << 40 {
        ptr::copy_nonoverlapping(&value as *const u64 as *const u8, *out, 5);
        *out = out.offset(5);
        4
    } else if value < 1 << 48 {
        ptr::copy_nonoverlapping(&value as *const u64 as *const u8, *out, 6);
        *out = out.offset(6);
        5
    } else if value < 1 << 56 {
        ptr::copy_nonoverlapping(&value as *const u64 as *const u8, *out, 7);
        *out = out.offset(7);
        6
    } else {
        ptr::copy_nonoverlapping(&value as *const u64 as *const u8, *out, 8);
        *out = out.offset(8);
        7
    }
}

pub unsafe fn encode_scalar(input: &[u64], keys: &mut [u8], data: &mut [u8]) -> usize {
    debug_assert!(keys.len() >= keys_len(input.len()));

    if input.is_empty() {
        return 0;
    }

    let mut keyptr = keys.as_mut_ptr();
    let mut dataptr = data.as_mut_ptr();

    let mut shift = 0;
    let mut key = 0u32;

    for &value in input {
        if shift == 24 {
            key = key.to_le();
            ptr::copy_nonoverlapping(&key as *const u32 as *const u8, keyptr, 3);
            keyptr = keyptr.offset(3);
            shift = 0;
            key = 0;
        }
        let code = encode_single(value, &mut dataptr);
        key |= (code as u32) << shift;
        shift += 3;
    }

    key = key.to_le();
    ptr::copy_nonoverlapping(&key as *const u32 as *const u8, keyptr, 3);

    let written = dataptr as usize - data.as_mut_ptr() as usize;
    debug_assert!(written <= data.len());
    written
}

unsafe fn decode_single(ptr: &mut *const u8, code: u8) -> u64 {
    let mut value = 0;
    match code {
        0 => {
            value = **ptr as u64;
            *ptr = ptr.offset(1);
        }
        1 => {
            ptr::copy_nonoverlapping(*ptr, &mut value as *mut u64 as *mut u8, 2);
            value = u64::from_le(value);
            *ptr = ptr.offset(2);
        }
        2 => {
            ptr::copy_nonoverlapping(*ptr, &mut value as *mut u64 as *mut u8, 3);
            value = u64::from_le(value);
            *ptr = ptr.offset(3);
        }
        3 => {
            ptr::copy_nonoverlapping(*ptr, &mut value as *mut u64 as *mut u8, 4);
            value = u64::from_le(value);
            *ptr = ptr.offset(4);
        }
        4 => {
            ptr::copy_nonoverlapping(*ptr, &mut value as *mut u64 as *mut u8, 5);
            value = u64::from_le(value);
            *ptr = ptr.offset(5);
        }
        5 => {
            ptr::copy_nonoverlapping(*ptr, &mut value as *mut u64 as *mut u8, 6);
            value = u64::from_le(value);
            *ptr = ptr.offset(6);
        }
        6 => {
            ptr::copy_nonoverlapping(*ptr, &mut value as *mut u64 as *mut u8, 7);
            value = u64::from_le(value);
            *ptr = ptr.offset(7);
        }
        _ => {
            ptr::copy_nonoverlapping(*ptr, &mut value as *mut u64 as *mut u8, 8);
            value = u64::from_le(value);
            *ptr = ptr.offset(8);
        }
    }
    value
}

pub unsafe fn decode_scalar(output: &mut [u64], keys: &[u8], data: &[u8]) -> usize {
    debug_assert!(keys.len() >= keys_len(output.len()));

    if output.is_empty() {
        return 0;
    }

    let mut keyptr = keys.as_ptr();
    let mut dataptr = data.as_ptr();

    let mut shift = 0;
    let mut key = 0;
    ptr::copy_nonoverlapping(keyptr, &mut key as *mut u32 as *mut u8, 3);
    key = u32::from_le(key);
    keyptr = keyptr.offset(3);

    for output in output {
        if shift == 24 {
            shift = 0;
            ptr::copy_nonoverlapping(keyptr, &mut key as *mut u32 as *mut u8, 3);
            key = u32::from_le(key);
            keyptr = keyptr.offset(3);
        }
        let code = (key >> shift) & 0b111;
        *output = decode_single(&mut dataptr, code as u8);
        shift += 3;
    }

    let read = dataptr as usize - data.as_ptr() as usize;
    debug_assert!(data.len() >= read);
    read
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scalar_round_trip() {
        unsafe {
            let values = (0..4090)
                .map(|v| v * (u64::max_value() / 4090))
                .collect::<Vec<_>>();
            let mut keys = vec![0; keys_len(values.len())];
            let mut data = vec![0; values.len() * 8];

            let written = encode_scalar(&values, &mut keys, &mut data);
            let mut out = vec![0; values.len()];
            let read = decode_scalar(&mut out, &keys, &data[..written]);
            assert_eq!(read, written);
            assert_eq!(values, out);
        }
    }

    #[test]
    fn single_round_trip() {
        let tests = [
            0,
            5,
            5 << 8 | 2,
            5 << 16 | 2,
            5 << 24 | 2,
            5 << 32 | 2,
            5 << 40 | 2,
            5 << 48 | 2,
            5 << 56 | 2,
        ];
        for &test in &tests {
            unsafe {
                let mut buf = [0; 8];
                let mut write_ptr = buf.as_mut_ptr();
                let code = encode_single(test, &mut write_ptr);
                let mut read_ptr = buf.as_ptr();
                let out = decode_single(&mut read_ptr, code);
                assert_eq!(write_ptr as *const u8, read_ptr);
                assert_eq!(test, out);
            }
        }
    }
}
