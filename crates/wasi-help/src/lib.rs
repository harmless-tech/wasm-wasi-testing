pub fn u32_to_i32(i: u32) -> i32 {
    unsafe { std::mem::transmute(i) }
}

pub fn i32_to_u32(i: i32) -> u32 {
    unsafe { std::mem::transmute(i) }
}

pub fn u64_to_i64(i: u64) -> i64 {
    unsafe { std::mem::transmute(i) }
}

pub fn i64_to_u64(i: i64) -> u64 {
    unsafe { std::mem::transmute(i) }
}
