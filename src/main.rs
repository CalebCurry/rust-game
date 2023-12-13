use std::fmt;

#[derive(Debug)]
enum Gem {
    Diamond,
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

fn main() {
    let gems = [
        (Gem::Onyx, 25.00),
        (Gem::Diamond, 100.00),
        (Gem::Onyx, 50.00),
        (Gem::Ruby, 10.00),
    ];

    for gem in gems {
        println!("This {} is worth {}", gem.0, gem.1);
    }
}
