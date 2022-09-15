// Algorithm to quickly compute a^b mod c
// See https://www.youtube.com/watch?v=cbGB__V8MNk

use std::env;
use std::time::Instant;

fn main() -> () {
    let p: Vec<String> = env::args().collect();

    assert_eq!(p.len(), 4);

    let a: u32 = p[1]
        .parse()
        .expect(format!("{} is not a number", p[1]).as_str());
    let b: u32 = p[2]
        .parse()
        .expect(format!("{} is not a number", p[2]).as_str());
    let c: u32 = p[3]
        .parse()
        .expect(format!("{} is not a number", p[3]).as_str());

    let mut start: bool = false;
    let mut n = a;

    let now = Instant::now();

    for i in (0..31).rev() {
        let bit = b & (1 << i);

        if bit == 0 {
            if !start {
                continue;
            };
        }

        if start {
            n = n.pow(2) % c;

            if bit != 0 {
                n = n * a % c;
            };
        }

        start = true;
    }

    println!("{:?}", Instant::now() - now);
    println!("exponentation result: {}", n);
}
