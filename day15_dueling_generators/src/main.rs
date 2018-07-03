const A_START: u32 = 516;
const B_START: u32 = 190;
const A_FACTOR: u32 = 16807;
const B_FACTOR: u32 = 48271;

struct Sequence {
    cur: u32,
    factor: u32,
}

impl Sequence {
    fn new(starting_value: u32, factor: u32) -> Sequence {
        Sequence {
            cur: starting_value,
            factor,
        }
    }
}

impl Iterator for Sequence {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = ((self.cur as usize * self.factor as usize) % 2_147_483_647) as u32;
        self.cur = new_next;
        Some(new_next)
    }
}

fn main() {
    let sequence_a = Sequence::new(A_START, A_FACTOR);
    let sequence_b = Sequence::new(B_START, B_FACTOR);

    let num_matching = sequence_a
        .take(40_000_000)
        .zip(sequence_b.take(40_000_000))
        .filter(|(x, y)| lowest_16_bit_match(*x, *y))
        .count();

    println!("{}", num_matching);
}

fn lowest_16_bit_match(x: u32, y: u32) -> bool {
    (x & 0x0000_FFFF) == (y & 0x0000_FFFF)
}
