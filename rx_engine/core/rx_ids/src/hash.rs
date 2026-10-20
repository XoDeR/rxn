const STRING_HASH_SEED: u64 = 0xA0761D6478BD642F;
const STRING_HASH_PRIME: u64 = 0xE7037ED1A0B428DB;
const EMPTY_STRING_HASH: u64 = mix64(STRING_HASH_SEED.wrapping_mul(STRING_HASH_SEED));

#[inline]
pub const fn string_to_u64(s: &str) -> u64 {
    let bytes = s.as_bytes();
    let len = bytes.len();
    if len == 0 {
        return EMPTY_STRING_HASH;
    }

    let mut hash = STRING_HASH_SEED ^ (len as u64).wrapping_mul(STRING_HASH_PRIME);
    let mut i = 0usize;

    while i + 8 <= len {
        hash ^= read_u64_le(bytes, i);
        hash = hash.wrapping_mul(STRING_HASH_PRIME);
        hash ^= hash >> 32;
        i += 8;
    }

    let mut tail = 0u64;
    let mut shift = 0u32;
    while i < len {
        tail |= (bytes[i] as u64) << shift;
        shift += 8;
        i += 1;
    }

    hash ^= tail;
    hash = hash.wrapping_mul(STRING_HASH_SEED);
    mix64(hash)
}

const fn read_u64_le(bytes: &[u8], offset: usize) -> u64 {
    (bytes[offset] as u64)
        | ((bytes[offset + 1] as u64) << 8)
        | ((bytes[offset + 2] as u64) << 16)
        | ((bytes[offset + 3] as u64) << 24)
        | ((bytes[offset + 4] as u64) << 32)
        | ((bytes[offset + 5] as u64) << 40)
        | ((bytes[offset + 6] as u64) << 48)
        | ((bytes[offset + 7] as u64) << 56)
}

pub fn parse_hashed_source_uri(s: &str) -> Option<u64> {
    if s.as_bytes().iter().all(|b| b.is_ascii_digit()) {
        return s.parse::<u64>().ok();
    }
    None
}

pub const fn mix64(mut x: u64) -> u64 {
    x ^= x >> 30;
    x = x.wrapping_mul(0xBF58476D1CE4E5B9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94D049BB133111EB);
    x ^= x >> 31;
    x
}
