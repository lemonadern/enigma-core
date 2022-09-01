use crate::{key::Key, plugboard::PlugBoard, reflector::Reflector, rotor::Rotor};

pub struct Scrambler {
    plugboard: PlugBoard,
    rotor1: Rotor,
    rotor2: Rotor,
    rotor3: Rotor,
    reflector: Reflector,
}

impl Scrambler {
    pub fn new(rotor1: Rotor, rotor2: Rotor, rotor3: Rotor, plugboard: PlugBoard) -> Self {
        Self {
            plugboard,
            rotor1,
            rotor2,
            rotor3,
            reflector: Reflector::default(),
        }
    }

    fn scramble_char_by_char(&mut self, input: char) -> char {
        let key = Key::from_char(input);

        let x = self.plugboard.substitute(key.to_mod26());

        let x = self.rotor1.substitute_from_forward(x);
        let x = self.rotor2.substitute_from_forward(x);
        let x = self.rotor3.substitute_from_forward(x);

        let x = self.reflector.substitute(x);

        let x = self.rotor3.substitute_from_backward(x);
        let x = self.rotor2.substitute_from_backward(x);
        let x = self.rotor1.substitute_from_backward(x);

        let x = self.plugboard.substitute(x);

        let encrypted_key = Key::from_mod26(x);

        if self.rotor2.is_rotated() {
            self.rotor3.increment_offset();
        }

        if self.rotor1.is_rotated() {
            self.rotor2.increment_offset();
        }

        self.rotor1.increment_offset();

        encrypted_key.to_char()
    }

    pub fn scramble(&mut self, input: &str) -> String {
        input
            .chars()
            .map(|c| self.scramble_char_by_char(c))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::mod26::Mod26;

    use super::*;

    #[test]
    fn scramble() {
        let raw = "helloworld";

        let rotor1 = Rotor::from_str(Mod26::new(0), "bcdefghijklmnopqrstuvwxyza").unwrap();
        let rotor2 = Rotor::from_str(Mod26::new(0), "cdefghijklmnopqrstuvwxyzab").unwrap();
        let rotor3 = Rotor::from_str(Mod26::new(0), "efghijklmnopqrstuvwxyzabcd").unwrap();

        let mut plugboard_pairs = HashMap::new();
        plugboard_pairs.insert(Mod26::new(0), Mod26::new(1));

        let plugboard = PlugBoard::new(plugboard_pairs).unwrap();

        let mut scrambler1 = Scrambler::new(rotor1, rotor2, rotor3, plugboard);

        let encrypted = scrambler1.scramble(raw);

        let rotor1 = Rotor::from_str(Mod26::new(0), "bcdefghijklmnopqrstuvwxyza").unwrap();
        let rotor2 = Rotor::from_str(Mod26::new(0), "cdefghijklmnopqrstuvwxyzab").unwrap();
        let rotor3 = Rotor::from_str(Mod26::new(0), "efghijklmnopqrstuvwxyzabcd").unwrap();

        let mut plugboard_pairs = HashMap::new();
        plugboard_pairs.insert(Mod26::new(0), Mod26::new(1));

        let plugboard = PlugBoard::new(plugboard_pairs).unwrap();

        let mut scrambler2 = Scrambler::new(rotor1, rotor2, rotor3, plugboard);
        let encrypted_twice = scrambler2.scramble(encrypted.as_str());

        assert_eq!(raw, encrypted_twice);
    }

    #[test]
    fn scramble_char_by_char() {
        let raw = 'a';

        let rotor1 = Rotor::from_str(Mod26::new(0), "bcdefghijklmnopqrstuvwxyza").unwrap();
        let rotor2 = Rotor::from_str(Mod26::new(0), "cdefghijklmnopqrstuvwxyzab").unwrap();
        let rotor3 = Rotor::from_str(Mod26::new(0), "efghijklmnopqrstuvwxyzabcd").unwrap();

        let mut plugboard_pairs = HashMap::new();
        plugboard_pairs.insert(Mod26::new(0), Mod26::new(1));

        let plugboard = PlugBoard::new(plugboard_pairs).unwrap();

        let mut scrambler1 = Scrambler::new(rotor1, rotor2, rotor3, plugboard);

        let encrypted = scrambler1.scramble_char_by_char(raw);

        let rotor1 = Rotor::from_str(Mod26::new(0), "bcdefghijklmnopqrstuvwxyza").unwrap();
        let rotor2 = Rotor::from_str(Mod26::new(0), "cdefghijklmnopqrstuvwxyzab").unwrap();
        let rotor3 = Rotor::from_str(Mod26::new(0), "efghijklmnopqrstuvwxyzabcd").unwrap();

        let mut plugboard_pairs = HashMap::new();
        plugboard_pairs.insert(Mod26::new(0), Mod26::new(1));

        let plugboard = PlugBoard::new(plugboard_pairs).unwrap();

        let mut scrambler2 = Scrambler::new(rotor1, rotor2, rotor3, plugboard);
        let encrypted_twice = scrambler2.scramble_char_by_char(encrypted);

        assert_eq!(raw, encrypted_twice);
    }
}
