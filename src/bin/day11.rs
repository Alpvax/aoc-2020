use rust_2020::grid::{Grid, Surroundings};
use rust_2020::parse_file;

fn main() {
    /*let test = "L.LL.LL.LL\n\
    LLLLLLL.LL\n\
    L.L.L..L..\n\
    LLLL.LL.LL\n\
    L.LL.LL.LL\n\
    L.LLLLL.LL\n\
    ..L.L.....\n\
    LLLLLLLLLL\n\
    L.LLLLLL.L\n\
    L.LLLLL.LL";
    let mut grid: Grid = test.parse().unwrap();*/
    let mut grid: Grid = parse_file("puzzle-input/day11.txt", |s| s.parse().unwrap());
    grid.default_char = '.';
    let grid = grid; //Remove mutability
                     /*println!("Grid: {0:?}\n\n{0}", grid);
                     for (i, g) in grid.iter_changes(&map_seat).enumerate() {
                         println!("\nAfter {}:\n{}", i + 1, g);
                     }*/
    let result = grid.iter_changes(&map_seat).last().unwrap();
    println!("Part 1: {}\nPart 2: {}", result.count_char('#'), 0);
}

fn map_seat(s: Surroundings) -> char {
    if s.value == 'L' && s.get_neighbour_count('#') == 0 {
        '#'
    } else if s.value == '#' && s.get_neighbour_count('#') >= 4 {
        'L'
    } else {
        s.value
    }
}
