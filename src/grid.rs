use std::collections::HashMap;
use std::fmt;
use std::iter::FromIterator;

const DEFAULT_CHAR: char = '.'; // Default to '.' as empty character

#[derive(Debug)]
pub struct Grid {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
    pub default_char: char,
}

pub struct Surroundings<'g> {
    grid: &'g Grid,
    x: usize,
    y: usize,
    pub value: char,
    neighbours: HashMap<char, u8>,
}
impl Surroundings<'_> {
    pub fn get_neighbour_count(&self, c: char) -> u8 {
        match self.neighbours.get(&c) {
            Some(&n) => n,
            None => 0,
        }
    }
}

impl Grid {
    pub fn new(data: Vec<Vec<char>>, default: char) -> Grid {
        let width = data.iter().map(|row| row.len()).max().unwrap();
        let height = data.len();
        Grid {
            data,
            width,
            height,
            default_char: default,
        }
    }
    fn get_char(&self, x: usize, y: usize) -> char {
        self.data[y][x]
    }
    fn get_surroundings<'g>(&'g self, x: usize, y: usize) -> Surroundings<'g> {
        let x_r = [
            if x == 0 { None } else { Some(x - 1) },
            Some(x),
            if x >= self.width - 1 {
                None
            } else {
                Some(x + 1)
            },
        ];
        let y_r = [
            if y == 0 { None } else { Some(y - 1) },
            Some(y),
            if y >= self.height - 1 {
                None
            } else {
                Some(y + 1)
            },
        ];
        let mut map = HashMap::new();
        let mut inc = |c: char| *map.entry(c).or_insert(0) += 1;
        for row in y_r.iter() {
            for col in x_r.iter() {
                match (col, row) {
                    (Some(i), Some(j)) => {
                        if *i != x || *j != y {
                            inc(self.get_char(*i, *j))
                        }
                    }
                    _ => inc(self.default_char),
                }
            }
        }
        Surroundings {
            grid: self,
            x,
            y,
            value: self.get_char(x, y),
            neighbours: map,
        }
    }
    pub fn iter_surroundings(&self) -> SurroundingsIter {
        SurroundingsIter::new(self)
    }
    pub fn map_surroundings(&self, f: &dyn Fn(Surroundings) -> char) -> Option<Grid> {
        let mut flag = false;
        let grid = self
            .iter_surroundings()
            .flat_map(|s| {
                let mut res = if s.x == 0 && s.y > 0 {
                    vec!['\n']
                } else {
                    Vec::new()
                };
                let old = s.value;
                let new = f(s);
                if !flag && new != old {
                    flag = true;
                }
                res.push(new);
                res.into_iter()
            })
            .collect();
        if flag {
            Some(grid)
        } else {
            None
        }
    }
    pub fn iter_changes<'f>(&self, f: &'f dyn Fn(Surroundings) -> char) -> GameOfLifeIter<'f> {
        GameOfLifeIter::new(self, f)
    }
    pub fn count_char(&self, c: char) -> usize {
        self.data
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&chr| *chr == c)
            .count()
    }
}

impl std::str::FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //let data = s.split("\n").map(|line| line.chars().collect()).collect();
        Ok(s.chars().collect())
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|row| {
                    let mut line = row.iter().map(|&c| char::from(c)).collect::<String>();
                    line.push('\n');
                    line
                })
                .collect::<String>()
        )
    }
}

impl<'g> FromIterator<Surroundings<'g>> for Grid {
    fn from_iter<T: IntoIterator<Item = Surroundings<'g>>>(iter: T) -> Self {
        let mut data = Vec::new();
        let mut current = Vec::new();
        let mut line = 0;
        let mut default = DEFAULT_CHAR;
        for s in iter {
            default = s.grid.default_char;
            if s.y > line {
                data.push(current);
                current = Vec::new();
                line = s.y;
            }
            current.push(s.value);
        }
        if current.len() > 0 {
            data.push(current);
        }
        Grid::new(data, default)
    }
}
impl FromIterator<char> for Grid {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut data = Vec::new();
        let mut current = Vec::new();
        for c in iter {
            if c == '\n' {
                data.push(current);
                current = Vec::new();
            } else {
                current.push(c);
            }
        }
        if current.len() > 0 {
            data.push(current);
        }
        Grid::new(data, DEFAULT_CHAR)
    }
}
impl Clone for Grid {
    fn clone(&self) -> Self {
        let mut g: Grid = self
            .data
            .iter()
            .flat_map(|row| {
                let mut copy: Vec<_> = row.iter().map(|&c| c).collect();
                copy.push('\n');
                copy.into_iter()
            })
            .collect();
        g.default_char = self.default_char;
        g
    }
}

pub struct SurroundingsIter<'g> {
    grid: &'g Grid,
    x: usize,
    y: usize,
}
impl SurroundingsIter<'_> {
    fn new<'g>(grid: &'g Grid) -> SurroundingsIter<'g> {
        SurroundingsIter { grid, x: 0, y: 0 }
    }
}
impl<'g> Iterator for SurroundingsIter<'g> {
    type Item = Surroundings<'g>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.height {
            return None;
        }
        let s = self.grid.get_surroundings(self.x, self.y);
        self.x += 1;
        if self.x >= self.grid.width {
            self.y += 1;
            self.x = 0;
        }
        Some(s)
    }
}

pub struct GameOfLifeIter<'f> {
    grid: Grid,
    mapper: &'f dyn Fn(Surroundings) -> char,
}
impl<'f> GameOfLifeIter<'f> {
    fn new(grid: &Grid, f: &'f dyn Fn(Surroundings) -> char) -> GameOfLifeIter<'f> {
        GameOfLifeIter {
            grid: grid.clone(),
            mapper: f,
        }
    }
}
impl<'f> Iterator for GameOfLifeIter<'f> {
    type Item = Grid;

    fn next(&mut self) -> Option<Self::Item> {
        self.grid.map_surroundings(self.mapper).map(|g| {
            self.grid = g.clone();
            g
        })
    }
}
