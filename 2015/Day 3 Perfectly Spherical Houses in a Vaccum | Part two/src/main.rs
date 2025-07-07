use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/direction.txt").expect("Errore nella lettura del file");

    // Coordinate per Babbo Natale e Robo-Natale
    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robo_x = 0;
    let mut robo_y = 0;

    let mut visited_houses = HashSet::new();

    visited_houses.insert((0, 0));

    for (i, direction) in input.chars().enumerate() {
        if i % 2 == 0 {
            match direction {
                '^' => santa_y += 1,
                'v' => santa_y -= 1,
                '>' => santa_x += 1,
                '<' => santa_x -= 1,
                _ => {}
            }
            visited_houses.insert((santa_x, santa_y));
        } 
        else {
            match direction {
                '^' => robo_y += 1,
                'v' => robo_y -= 1,
                '>' => robo_x += 1,
                '<' => robo_x -= 1,
                _ => {}
            }
            visited_houses.insert((robo_x, robo_y));
        }
    }

    println!("Numero di case visitate da Babbo Natale o Robo-Natale: {}", visited_houses.len());
}
