use std::collections::HashMap;

use enigma_core::{key::Key, plugboard::PlugBoard, rotor::Rotor, scrambler::Scrambler};

fn main() {
    let raw = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    let rotor1 = Rotor::from_str("zyxwvutsrqponmlkjihgfedcba").unwrap();
    let rotor2 = Rotor::from_str("kyxwvutsrqponmlazihgfedcbj").unwrap();
    let rotor3 = Rotor::from_str("zyxwvutsrqponmlkjihgfedcba").unwrap();

    let mut plugboard_pairs = HashMap::new();
    plugboard_pairs.insert(Key::A, Key::B);

    let plugboard = PlugBoard::new(plugboard_pairs).unwrap();

    let mut scrambler1 = Scrambler::new(rotor1, rotor2, rotor3, plugboard);

    let encrypted = scrambler1.scramble(raw);

    let rotor1 = Rotor::from_str("zyxwvutsrqponmlkjihgfedcba").unwrap();
    let rotor2 = Rotor::from_str("kyxwvutsrqponmlazihgfedcbj").unwrap();
    let rotor3 = Rotor::from_str("zyxwvutsrqponmlkjihgfedcba").unwrap();

    let mut plugboard_pairs = HashMap::new();
    plugboard_pairs.insert(Key::A, Key::B);

    let plugboard = PlugBoard::new(plugboard_pairs).unwrap();

    let mut scrambler2 = Scrambler::new(rotor1, rotor2, rotor3, plugboard);
    let encrypted_twice = scrambler2.scramble(encrypted.as_str());

    println!("raw text       : {}", raw);
    println!("encrypted text : {}", encrypted);
    println!("encrypted twice: {}", encrypted_twice);
}
