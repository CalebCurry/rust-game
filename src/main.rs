use std::{collections::HashMap, fmt, io};

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive, ToPrimitive, Eq, PartialEq, Hash)]
enum Gem {
    Diamond = 1,
    Sapphire,
    Ruby,
    Topaz,
    Onyx,
    Jade,
}

impl fmt::Display for Gem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gem::Diamond => write!(f, "diamond"),
            Gem::Sapphire => write!(f, "sapphire"),
            Gem::Ruby => write!(f, "ruby"),
            Gem::Topaz => write!(f, "topaz"),
            Gem::Onyx => write!(f, "onyx"),
            Gem::Jade => write!(f, "jade"),
        }
    }
}

fn game(map: &mut [[u8; 5]; 5]) -> Vec<Gem> {
    let mut found: Vec<Gem> = Vec::new();

    while found.len() < 5 {
        println!("Search an X Y position 0-4 (example input: 5 3):");
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to read line");
                continue;
            }
        }

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() != 2 {
            println!("Two numbers required");
            continue;
        }

        let (x, y) = match (parts[0].parse::<u8>(), parts[1].parse::<u8>()) {
            (Ok(x), Ok(y)) => (x, y),
            _ => {
                println!("Something is wrong with the input");
                continue;
            }
        };

        if x >= 5 || y >= 5 {
            println!("Invalid index");
            continue;
        }

        let data = map[x as usize][y as usize];

        let gem = match Gem::from_u8(data) {
            Some(gem) => gem,
            None => {
                println!("No gem found at that position");
                continue;
            }
        };
        found.push(gem);
        map[x as usize][y as usize] = 0;
        println!("{found:?}");
    }
    found
}

fn main() {
    let mut map = [[0; 5]; 5];
    println!("{map:?}");

    map[4][2] = 1;
    map[1][2] = 2;
    map[3][3] = 3;
    map[0][2] = 4;
    map[1][4] = 5;

    for row in map {
        println!("{row:?}");
    }

    let mut found: Vec<Gem> = game(&mut map);

    println!("You've found them all! Congrats!");
    let mut gem_values: HashMap<Gem, f64> = HashMap::new();
    gem_values.insert(Gem::Diamond, 1000.00);
    gem_values.insert(Gem::Jade, 500.00);
    gem_values.insert(Gem::Onyx, 300.00);
    gem_values.insert(Gem::Ruby, 100.00);
    gem_values.insert(Gem::Sapphire, 15.50);
    gem_values.insert(Gem::Topaz, 9.99);

    let mut sum = 0.0;
    for gem in found {
        sum += gem_values[&gem];
    }

    println!("The total Gem Value is: {}", sum);
}
