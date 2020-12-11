use rust_2020::parse_file;
use rust_2020::cgol::{EntryType, Grid};
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
enum Seat {
    None,
    Empty,
    Full,
}
impl EntryType for Seat {}
impl TryFrom<char> for Seat {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::None),
            'L' => Ok(Self::Empty),
            '#' => Ok(Self::Full),
            _ => Err(value),
        }
    }
}
impl From<Seat> for char {
    fn from(c: Seat) -> Self {
        match c {
            Seat::None => '.',
            Seat::Empty => 'L',
            Seat::Full => '#',
        }
    }
}

fn main() {
    let grid: Grid<Seat> = parse_file("puzzle-input/day11.txt", |s| s.parse().unwrap());
    println!("Grid: {0:?}\n\n{0}", grid);
    /* Test conversion to-from char
    println!(
        ". => {:?}\nL => {:?}\n# => {:?}\n------------\nNone => {:?}\nEmpty => {:?}\nFull => {:?}\n============\n",
        Seat::try_from('.'), Seat::try_from('L'), Seat::try_from('#'),
        char::from(Seat::None), char::from(Seat::Empty), char::from(Seat::Full),
    );*/
}
