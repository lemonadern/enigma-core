mod tests {
    use std::collections::HashMap;

    use enigma_core::{key::Key, plugboard::PlugBoard, rotor::Rotor, scrambler::Scrambler};

    #[test]
    fn it_works() {
        let rotor1 = Rotor::from_str_and_key("zyxwvutsrqponmlkjihgfedcba", Key::T).unwrap();
        let rotor2 = Rotor::from_str_and_key("kyxwvutsrqponmlazihgfedcbj", Key::E).unwrap();
        let rotor3 = Rotor::from_str_and_key("zyxwvutsrqponmlkjihgfedcba", Key::B).unwrap();

        let plugboard_pairs = HashMap::from([(Key::A, Key::B), (Key::D, Key::Z), (Key::U, Key::T)]);
        let plugboard = PlugBoard::new(plugboard_pairs).unwrap();

        let mut encrypter = Scrambler::new(rotor1, rotor2, rotor3, plugboard);

        let rotor1 = Rotor::from_str_and_key("zyxwvutsrqponmlkjihgfedcba", Key::T).unwrap();
        let rotor2 = Rotor::from_str_and_key("kyxwvutsrqponmlazihgfedcbj", Key::E).unwrap();
        let rotor3 = Rotor::from_str_and_key("zyxwvutsrqponmlkjihgfedcba", Key::B).unwrap();

        let plugboard_pairs = HashMap::from([(Key::A, Key::B), (Key::D, Key::Z), (Key::U, Key::T)]);
        let plugboard = PlugBoard::new(plugboard_pairs).unwrap();

        let mut decrypter = Scrambler::new(rotor1, rotor2, rotor3, plugboard);

        let plain = "hello, world! mr. enigma";
        let encrypted = encrypter.scramble(plain);
        let decrypted = decrypter.scramble(&encrypted);

        assert_eq!(plain, decrypted.as_str());
    }
}
