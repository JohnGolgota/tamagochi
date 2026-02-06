use rand::prelude::*;

pub fn randnum() -> u8 {
    let rand_number: u8 = rand::thread_rng().gen_range(1..=10);
    rand_number
}
