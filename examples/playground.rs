use std::collections::HashMap;

use enigma_core::{key::Key, plugboard::PlugBoard, rotor::Rotor, scrambler::Scrambler};

fn main() {
    let raw = "hello, world! mr. enigma";

    let rotor1 = Rotor::from_str_and_key("zyxwvutsrqponmlkjihgfedcba", Key::W).unwrap();
    let rotor2 = Rotor::from_str_and_key("kyxwvutsrqponmlazihgfedcbj", Key::E).unwrap();
    let rotor3 = Rotor::from_str_and_key("zyxwvutsrqponmlkjihgfedcba", Key::B).unwrap();

    let mut plugboard_pairs = HashMap::new();
    plugboard_pairs.insert(Key::A, Key::B);
    plugboard_pairs.insert(Key::D, Key::Z);
    plugboard_pairs.insert(Key::U, Key::T);

    let plugboard = PlugBoard::new(plugboard_pairs).unwrap();

    let mut scrambler1 = Scrambler::new(rotor1, rotor2, rotor3, plugboard);

    let encrypted = scrambler1.scramble(raw);

    let rotor1 = Rotor::from_str_and_key("zyxwvutsrqponmlkjihgfedcba", Key::W).unwrap();
    let rotor2 = Rotor::from_str_and_key("kyxwvutsrqponmlazihgfedcbj", Key::E).unwrap();
    let rotor3 = Rotor::from_str_and_key("zyxwvutsrqponmlkjihgfedcba", Key::B).unwrap();

    let mut plugboard_pairs = HashMap::new();
    plugboard_pairs.insert(Key::A, Key::B);
    plugboard_pairs.insert(Key::D, Key::Z);
    plugboard_pairs.insert(Key::U, Key::T);

    let plugboard = PlugBoard::new(plugboard_pairs).unwrap();

    let mut scrambler2 = Scrambler::new(rotor1, rotor2, rotor3, plugboard);
    let encrypted_twice = scrambler2.scramble(encrypted.as_str());

    println!("raw text       : {}", raw);
    println!("encrypted text : {}", encrypted);
    println!("encrypted twice: {}", encrypted_twice);
}
