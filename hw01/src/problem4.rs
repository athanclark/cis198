#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

pub type Move = (Peg, Peg);

pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    match num_discs {
        1 => vec![(src, dst)],
        n => {
            let mut prefix: Vec<Move> = hanoi(n-1, src, dst, aux);
            let suffix: Vec<Move> = hanoi(n-1, aux, src, dst);
            prefix.push((src, dst));
            prefix.extend(suffix);
            prefix
        }
    }
}
