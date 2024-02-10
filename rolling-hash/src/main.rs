use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
};

// values will bze computed module a prime 'p'
// the larger 'p' is, the smaller the probability of having collision
// but if 'p' is too large, computation will overflow: waht is the macimum p value we can take ?

// maximum byte value = 255 => 8 bits
// regiuster size = 64 bits (unsigned)
// maximum value that will be held in a register: (2^8) * (p - 1) < 2^64
// (any larger value being modulo'd out by 'p')
// so prime 'p' must be < 2^64 / (2^8) = 2^56
const PRIME: u64 = 72057594037927931;
const DOMAIN: u64 = 256;

// make sur to get a positive output when using modulo
// println!("{}", -1i32 % 4);                // -1
// println!("{}", (-21i32).rem_euclid(4));   // 3

fn roll_once(old_hash: u64, remove_digit: u8, last_position: u64, add_digit: u8) -> u64 {
    // remove old value
    let mut new_hash = _roll_once_remove_digit(last_position, remove_digit, old_hash);

    // shift left (in the hash DOMAIN)
    new_hash = new_hash.checked_mul(DOMAIN).unwrap().rem_euclid(PRIME);

    // add new value
    new_hash = new_hash
        .checked_add(add_digit as u64)
        .unwrap()
        .rem_euclid(PRIME);

    new_hash
}

fn _roll_once_remove_digit(last_position: u64, remove_digit: u8, old_hash: u64) -> u64 {
    let remove_value = last_position
        .checked_mul(remove_digit as u64)
        .unwrap()
        .rem_euclid(PRIME);
    if remove_value > old_hash {
        PRIME - remove_value + old_hash
    } else {
        old_hash - remove_value
    }
}

fn compute_last_pos(window_size: usize) -> u64 {
    let mut last_pos: u64 = 1;
    for _ in 0..(window_size - 1) {
        last_pos = last_pos.checked_mul(DOMAIN).unwrap().rem_euclid(PRIME);
    }
    last_pos
}

fn init_hash(input_vector: &[u8]) -> u64 {
    let mut hash = 0;
    for i in 0..input_vector.len() {
        hash = roll_once(hash, 0, 0, input_vector[i]);
    }
    hash
}

fn main() {
    let mut target = File::open("target").unwrap();
    let mut target_buf = vec![];
    target.read_to_end(&mut target_buf).unwrap();
    let target_sz = target_buf.len();

    let last_pos = compute_last_pos(target_sz);

    // hash target
    let target_hash = init_hash(&target_buf);
    dbg!(&target_hash);

    let input = File::open("input").unwrap();
    let mut reader = BufReader::new(input);

    // init hash
    let mut window = vec![0u8; target_sz];
    reader.read_exact(&mut window).unwrap();
    let mut roll_hash = init_hash(&window);

    let mut global_idx = 0;
    loop {
        if target_hash == roll_hash {
            break;
        }
        reader.seek(SeekFrom::Start(global_idx)).unwrap();
        reader.read_exact(&mut window).unwrap();
        roll_hash = roll_once(roll_hash, window[0], last_pos, window[target_sz - 1]);
        if global_idx == 8 * 1024 {
            dbg!(&roll_hash);
        }
        global_idx += 1;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_overflow_smallest() {
        let size = std::u8::MAX as usize;
        let _ = roll_once(
            PRIME - 1,
            std::u8::MAX,
            compute_last_pos(size),
            std::u8::MAX,
        );
    }

    #[test]
    fn test_overflow_small() {
        let size = std::u16::MAX as usize;
        let _ = roll_once(
            PRIME - 1,
            std::u8::MAX,
            compute_last_pos(size),
            std::u8::MAX,
        );
    }

    // Very Long
    // #[test]
    // fn test_overflow_medium() {
    //     let size = std::u32::MAX as usize;
    //     let _ = roll_once(
    //         std::u64::MAX,
    //         std::u8::MAX,
    //         compute_last_pos(size),
    //         std::u8::MAX,
    //     );
    // }

    // WAY TOO LONG
    // #[test]
    // fn test_overflow_large() {
    //     let size = std::u64::MAX as usize;
    //     let _ = roll_once(
    //         std::u64::MAX,
    //         std::u8::MAX,
    //         compute_last_pos(size),
    //         std::u8::MAX,
    //     );
    // }

    #[test]
    fn test_init() {
        let v = vec![16, 27, 89, 12];
        let expected: u64 = (16 * DOMAIN.pow(3)).rem_euclid(PRIME)
            + (27 * DOMAIN.pow(2)).rem_euclid(PRIME)
            + (89 * DOMAIN.pow(1)).rem_euclid(PRIME)
            + 12;
        let hash = init_hash(&v);
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_last_pos_small() {
        let v = vec![0u8; 16];
        let val = compute_last_pos(v.len());
        let expected: u64 = 6400; // 256**(16 - 1) % PRIME
        assert_eq!(val, expected);
    }

    #[test]
    fn test_last_pos_large() {
        let v = vec![0u8; std::u16::MAX as usize];
        let val = compute_last_pos(v.len());
        let expected: u64 = 440322404861847; // 256**(2**16 - 1 - 1) % PRIME
        assert_eq!(val, expected);
    }

    #[test]
    fn test_remove_digit() {
        let v = vec![16, 27, 89, 12];
        let expected: u64 =
            (27 * DOMAIN.pow(2)).rem_euclid(PRIME) + (89 * DOMAIN.pow(1)).rem_euclid(PRIME) + 12;
        let hash = init_hash(&v);
        let value = _roll_once_remove_digit(compute_last_pos(v.len()), 16, hash);
        assert_eq!(value, expected);
    }

    #[test]
    fn test_rolling_once() {
        let v = vec![16, 27, 89, 12, 35];
        let expected: u64 = (27 * DOMAIN.pow(3)).rem_euclid(PRIME)
            + (89 * DOMAIN.pow(2)).rem_euclid(PRIME)
            + (12 * DOMAIN.pow(1)).rem_euclid(PRIME)
            + 35;
        let window = &v[0..4];
        let mut hash = init_hash(window);
        hash = roll_once(hash, 16, compute_last_pos(window.len()), 35);
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_rolling_twice() {
        let v = vec![16, 27, 89, 12, 35, 78];
        let expected: u64 = (89 * DOMAIN.pow(3)).rem_euclid(PRIME)
            + (12 * DOMAIN.pow(2)).rem_euclid(PRIME)
            + (35 * DOMAIN.pow(1)).rem_euclid(PRIME)
            + 78;
        let window = &v[0..4];
        let mut hash = init_hash(window);
        hash = roll_once(hash, 16, compute_last_pos(window.len()), 35);
        hash = roll_once(hash, 27, compute_last_pos(window.len()), 78);
        assert_eq!(hash, expected);
    }
}
