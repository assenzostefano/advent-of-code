use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/direction.txt").expect("Errore nella lettura del file");

    let mut x = 0;
    let mut y = 0;
    let mut visited_houses = HashSet::new();

    visited_houses.insert((x, y));

    for direction in input.chars() {
        match direction {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _   => {}
        }
        
        visited_houses.insert((x, y));
    }

    println!("Numero di case visitate: {}", visited_houses.len());
}
