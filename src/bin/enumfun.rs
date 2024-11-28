use std::{error::Error, fmt::Display};

use derive_new::new;

enum TopTargets {
    Manual(u16),
    Auto,
}

#[derive(new, Debug)]
struct TopTargetsError;
impl Display for TopTargetsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.source() {
            Some(t) => f.write_str(&format!("Error e={}", t)),
            None => f.write_str("Holy cow batman!"),
        }
    }
}
impl Error for TopTargetsError {}

impl TopTargets {
    fn new(val: u16) -> Result<Self, TopTargetsError> {
        if val < 2 || val > 5 {
            Err(TopTargetsError::new())
        } else {
            Ok(Self::Manual(val))
        }
    }
}
pub fn main() {
    let targets = TopTargets::new(42);
    match targets {
        Ok(target) => {
            print_targets(&target);
        }
        Err(e) => println!("{}", e),
    }
    print_targets(&TopTargets::Auto);
}

fn print_targets(val: &TopTargets) {
    match val {
        TopTargets::Manual(count) => println!("{}", count),
        TopTargets::Auto => println!("Autoselect"),
    }
}
