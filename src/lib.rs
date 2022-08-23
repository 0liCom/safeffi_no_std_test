#![no_std]

fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

/// Shared reference
pub fn cmp(left: &[u8], right: &[u8]) -> bool {
    let min_len = min(left.len(), right.len());

    for i in 0..min_len {
        if left[i] != right[i] {
            return false;
        }
    }

    true
}

/// Mut reference
pub fn copy_prefix(dest: &mut [u8], src: &[u8], len: usize) {
    let min_len = min(dest.len(), min(src.len(), len));

    for i in 0..min_len {
        dest[i] = src[i];
    }
}

#[repr(C)]
pub struct SomeStruct {
    array: [i32; 4],
}

/// Owned
pub fn spawn_struct(array: [i32; 4]) -> SomeStruct {
    SomeStruct { array }
}

pub fn convert(data: &[u8; 16]) -> [i32; 4] {
    let mut res = [0; 4];
    for i in 0..4 {
        res[i] = (data[i*4] as i32) + ((data[i*4 + 1] as i32) << 8) + ((data[i*4 + 2] as i32) << 16) + ((data[i*4 + 3] as i32) << 24);
    }
    res
}
