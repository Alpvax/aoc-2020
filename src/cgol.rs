use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

pub trait EntryType: /*fmt::Debug + */TryFrom<char> + Into<char> + Copy {
    fn to_char(self) -> char {
        self.into()
    }
}

pub enum GridLookup<'g, T: EntryType> {
    Value(&'g T),
    Empty,
    Invalid,
}
/*#[derive(Debug)]
struct GridEntry<'g, T: EntryType> {
    grid: &'g Grid<'g, T>,
    value: T,
    x: u16,
    y: u16,
}
impl<'g, T: EntryType> GridEntry<'g, T> {
    fn offset(&self, x: i32, y: i32) -> Option<&'g T> {
        match self.grid.get_entry(x, y) {
            GridLookup::Value(e) => Some(&e.value),
            _ => None,
        }
    }
}*/

#[derive(Debug)]
pub struct Grid<T: EntryType> {
    data: HashMap<(u16, u16), T>,
    width: u16,
    height: u16,
}
impl<'g, T: EntryType> Grid<T> {
    pub fn new(width: u16, height: u16) -> Self {
        Grid {
            data: HashMap::new(),
            width,
            height,
        }
    }
    pub fn get(&self, x: u16, y: u16) -> Option<&T> {
        if x < self.width && y < self.height {
            self.data.get(&(x, y))
        } else {
            None
        }
    }
    pub fn get_raw(&'g self, x: i32, y: i32) -> GridLookup<'g, T> {
        if x < 0 || x >= self.width.into() || y < 0 || y >= self.height.into() {
            GridLookup::Invalid
        } else {
            match self
                .data
                .get(&(u16::try_from(x).unwrap(), u16::try_from(y).unwrap()))
            {
                Some(e) => GridLookup::Value(e),
                None => GridLookup::Empty,
            }
        }
    }
    fn set_char(&'g mut self, x: u16, y: u16, c: char) -> Result<&T, T::Error> {
        let val = T::try_from(c)?;
        self.set(x, y, val);
        Ok(self.data.get(&(x, y)).unwrap())
    }
    pub fn set(&'g mut self, x: u16, y: u16, value: T) {
        self.data.insert((x, y), value);
    }
}
impl<'g, T: EntryType> FromStr for Grid<T> {
    type Err = T::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut g = Grid::new(0, 0);
        for (y, line) in s.split("\n").enumerate() {
            g.width = std::cmp::max(
                g.width,
                u16::try_from(line.chars().count()).expect("Wider than u16!"),
            );
            g.height += 1;
            for (x, c) in line.chars().enumerate() {
                g.set_char(u16::try_from(x).unwrap(), u16::try_from(y).unwrap(), c)?;
            }
        }
        Ok(g)
    }
}
impl<'g, T: EntryType> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output: String = (0..self.height)
            .map(|y| {
                let mut s = (0..self.width)
                    .map(|x| {
                        self.get(x, y)
                            .map(|v| v.to_char())
                            .expect("Missing space in grid!")
                    })
                    .collect::<String>();
                s.push('\n');
                s
            })
            .collect();
        write!(f, "{}", output)
    }
}