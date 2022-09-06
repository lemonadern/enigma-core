pub mod key;
mod mod26;
pub mod plugboard;
mod reflector;
pub mod rotor;
pub mod scrambler;

use key::Key;
use rotor::Rotor;
use scrambler::Scrambler;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, enigma-wasm!");
}

#[wasm_bindgen]
pub fn enigma_encrypt_with_settings_inline(
    rotor1_map: &str,
    rotor2_map: &str,
    rotor3_map: &str,
    rotor1_key: u64,
    rotor2_key: u64,
    rotor3_key: u64,
    pair1_left: u64,
    pair1_right: u64,
    pair2_left: u64,
    pair2_right: u64,
    pair3_left: u64,
    pair3_right: u64,
    raw: &str,
) -> String {
    let rotor1 = Rotor::from_str_and_key(rotor1_map, Key::from_u64(rotor1_key)).unwrap();
    let rotor2 = Rotor::from_str_and_key(rotor2_map, Key::from_u64(rotor2_key)).unwrap();
    let rotor3 = Rotor::from_str_and_key(rotor3_map, Key::from_u64(rotor3_key)).unwrap();
    let mut plugboard_pairs = std::collections::HashMap::new();
    plugboard_pairs.insert(Key::from_u64(pair1_left), Key::from_u64(pair1_right));
    plugboard_pairs.insert(Key::from_u64(pair2_left), Key::from_u64(pair2_right));
    plugboard_pairs.insert(Key::from_u64(pair3_left), Key::from_u64(pair3_right));
    let plugboard = plugboard::PlugBoard::new(plugboard_pairs).unwrap();
    let mut scrambler = Scrambler::new(rotor1, rotor2, rotor3, plugboard);

    let encrypted = scrambler.scramble(raw);
    encrypted
}
