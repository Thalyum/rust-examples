fn read_be_u32(input: &mut &[u8]) -> u32 {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u32>());
    *input = rest;
    u32::from_be_bytes(int_bytes.try_into().unwrap())
}

fn main() {
    let mut a = u32::to_be_bytes(24).to_vec();
    let mut b = u32::to_be_bytes(43 << 8).to_vec();
    let mut c = u32::to_be_bytes(17 << 16).to_vec();

    a.append(&mut b);
    a.append(&mut c);
    let v = a;
    println!("v ({:p}): {:x?}", &v, v);

    let mut r = &mut &v[..];
    let n = read_be_u32(&mut r);
    assert_eq!(n, 24);
    let n = read_be_u32(&mut r);
    assert_eq!(n, 43 << 8);
    let n = read_be_u32(&mut r);
    assert_eq!(n, 17 << 16);
}
