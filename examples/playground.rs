use std::collections::HashMap;

use enigma_core::{mod26::Mod26, plugboard::PlugBoard, rotor::Rotor, scrambler::Scrambler, key::Key};

fn main() {
    let raw = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    let rotor1 = Rotor::from_str(Mod26::new(4), "zyxwvutsrqponmlkjihgfedcba").unwrap();
    let rotor2 = Rotor::from_str(Mod26::new(0), "kyxwvutsrqponmlazihgfedcbj").unwrap();
    let rotor3 = Rotor::from_str(Mod26::new(2), "zyxwvutsrqponmlkjihgfedcba").unwrap();

    let mut plugboard_pairs = HashMap::new();
    plugboard_pairs.insert(Key::A, Key::B);

    let plugboard = PlugBoard::new(plugboard_pairs).unwrap();

    let mut scrambler1 = Scrambler::new(rotor1, rotor2, rotor3, plugboard);

    let encrypted = scrambler1.scramble(raw);

    let rotor1 = Rotor::from_str(Mod26::new(4), "zyxwvutsrqponmlkjihgfedcba").unwrap();
    let rotor2 = Rotor::from_str(Mod26::new(0), "kyxwvutsrqponmlazihgfedcbj").unwrap();
    let rotor3 = Rotor::from_str(Mod26::new(2), "zyxwvutsrqponmlkjihgfedcba").unwrap();

    let mut plugboard_pairs = HashMap::new();
    plugboard_pairs.insert(Key::A, Key::B);

    let plugboard = PlugBoard::new(plugboard_pairs).unwrap();

    let mut scrambler2 = Scrambler::new(rotor1, rotor2, rotor3, plugboard);
    let encrypted_twice = scrambler2.scramble(encrypted.as_str());

    println!("raw text       : {}", raw);
    println!("encrypted text : {}", encrypted);
    println!("encrypted twice: {}", encrypted_twice);
}
