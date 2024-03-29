use crate::mod26::Mod26;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Key {
    #[deprecated]
    pub fn from_mod26(mod26: Mod26) -> Self {
        match mod26.0 {
            0 => Key::A,
            1 => Key::B,
            2 => Key::C,
            3 => Key::D,
            4 => Key::E,
            5 => Key::F,
            6 => Key::G,
            7 => Key::H,
            8 => Key::I,
            9 => Key::J,
            10 => Key::K,
            11 => Key::L,
            12 => Key::M,
            13 => Key::N,
            14 => Key::O,
            15 => Key::P,
            16 => Key::Q,
            17 => Key::R,
            18 => Key::S,
            19 => Key::T,
            20 => Key::U,
            21 => Key::V,
            22 => Key::W,
            23 => Key::X,
            24 => Key::Y,
            25 => Key::Z,
            _ => panic!("mod26 must be 0..26"),
        }
    }

    #[deprecated]
    pub fn from_char(c: char) -> Self {
        match c {
            'a' => Key::A,
            'b' => Key::B,
            'c' => Key::C,
            'd' => Key::D,
            'e' => Key::E,
            'f' => Key::F,
            'g' => Key::G,
            'h' => Key::H,
            'i' => Key::I,
            'j' => Key::J,
            'k' => Key::K,
            'l' => Key::L,
            'm' => Key::M,
            'n' => Key::N,
            'o' => Key::O,
            'p' => Key::P,
            'q' => Key::Q,
            'r' => Key::R,
            's' => Key::S,
            't' => Key::T,
            'u' => Key::U,
            'v' => Key::V,
            'w' => Key::W,
            'x' => Key::X,
            'y' => Key::Y,
            'z' => Key::Z,
            _ => panic!("character must be a..z"),
        }
    }

    pub fn to_mod26(&self) -> Mod26 {
        match self {
            Key::A => Mod26::new(0),
            Key::B => Mod26::new(1),
            Key::C => Mod26::new(2),
            Key::D => Mod26::new(3),
            Key::E => Mod26::new(4),
            Key::F => Mod26::new(5),
            Key::G => Mod26::new(6),
            Key::H => Mod26::new(7),
            Key::I => Mod26::new(8),
            Key::J => Mod26::new(9),
            Key::K => Mod26::new(10),
            Key::L => Mod26::new(11),
            Key::M => Mod26::new(12),
            Key::N => Mod26::new(13),
            Key::O => Mod26::new(14),
            Key::P => Mod26::new(15),
            Key::Q => Mod26::new(16),
            Key::R => Mod26::new(17),
            Key::S => Mod26::new(18),
            Key::T => Mod26::new(19),
            Key::U => Mod26::new(20),
            Key::V => Mod26::new(21),
            Key::W => Mod26::new(22),
            Key::X => Mod26::new(23),
            Key::Y => Mod26::new(24),
            Key::Z => Mod26::new(25),
        }
    }

    #[deprecated]
    pub fn to_char(&self) -> char {
        match self {
            Key::A => 'a',
            Key::B => 'b',
            Key::C => 'c',
            Key::D => 'd',
            Key::E => 'e',
            Key::F => 'f',
            Key::G => 'g',
            Key::H => 'h',
            Key::I => 'i',
            Key::J => 'j',
            Key::K => 'k',
            Key::L => 'l',
            Key::M => 'm',
            Key::N => 'n',
            Key::O => 'o',
            Key::P => 'p',
            Key::Q => 'q',
            Key::R => 'r',
            Key::S => 's',
            Key::T => 't',
            Key::U => 'u',
            Key::V => 'v',
            Key::W => 'w',
            Key::X => 'x',
            Key::Y => 'y',
            Key::Z => 'z',
        }
    }
}

impl From<Mod26> for Key {
    fn from(value: Mod26) -> Self {
        match value.0 {
            0 => Key::A,
            1 => Key::B,
            2 => Key::C,
            3 => Key::D,
            4 => Key::E,
            5 => Key::F,
            6 => Key::G,
            7 => Key::H,
            8 => Key::I,
            9 => Key::J,
            10 => Key::K,
            11 => Key::L,
            12 => Key::M,
            13 => Key::N,
            14 => Key::O,
            15 => Key::P,
            16 => Key::Q,
            17 => Key::R,
            18 => Key::S,
            19 => Key::T,
            20 => Key::U,
            21 => Key::V,
            22 => Key::W,
            23 => Key::X,
            24 => Key::Y,
            25 => Key::Z,
            n @ _ => panic!(
                // never
                "mod26 never becomes an integer greater than or equal to 26. But got {}.",
                n
            ),
        }
    }
}

impl TryFrom<char> for Key {
    // TODO: define and use approriate error type
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' => Ok(Key::A),
            'b' => Ok(Key::B),
            'c' => Ok(Key::C),
            'd' => Ok(Key::D),
            'e' => Ok(Key::E),
            'f' => Ok(Key::F),
            'g' => Ok(Key::G),
            'h' => Ok(Key::H),
            'i' => Ok(Key::I),
            'j' => Ok(Key::J),
            'k' => Ok(Key::K),
            'l' => Ok(Key::L),
            'm' => Ok(Key::M),
            'n' => Ok(Key::N),
            'o' => Ok(Key::O),
            'p' => Ok(Key::P),
            'q' => Ok(Key::Q),
            'r' => Ok(Key::R),
            's' => Ok(Key::S),
            't' => Ok(Key::T),
            'u' => Ok(Key::U),
            'v' => Ok(Key::V),
            'w' => Ok(Key::W),
            'x' => Ok(Key::X),
            'y' => Ok(Key::Y),
            'z' => Ok(Key::Z),
            _ => Err(()),
        }
    }
}

impl Into<char> for Key {
    fn into(self) -> char {
        match self {
            Key::A => 'a',
            Key::B => 'b',
            Key::C => 'c',
            Key::D => 'd',
            Key::E => 'e',
            Key::F => 'f',
            Key::G => 'g',
            Key::H => 'h',
            Key::I => 'i',
            Key::J => 'j',
            Key::K => 'k',
            Key::L => 'l',
            Key::M => 'm',
            Key::N => 'n',
            Key::O => 'o',
            Key::P => 'p',
            Key::Q => 'q',
            Key::R => 'r',
            Key::S => 's',
            Key::T => 't',
            Key::U => 'u',
            Key::V => 'v',
            Key::W => 'w',
            Key::X => 'x',
            Key::Y => 'y',
            Key::Z => 'z',
        }
    }
}

impl Into<char> for &Key {
    fn into(self) -> char {
        Into::into(*self)
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = self.into();
        write!(f, "{}", c)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[deprecated]
    #[test]
    fn mod26からkeyに変換できる() {
        let zero = Mod26::new(0);
        let key = Key::from_mod26(zero);
        assert_eq!(key, Key::A);
    }

    #[test]
    fn convert_from_mod26() {
        let zero = Mod26::new(0);
        assert_eq!(Key::from(zero), Key::A);
    }

    #[deprecated]
    #[test]
    fn keyからmod26に変換できる() {
        let a = Key::A;
        let mod26 = a.to_mod26();
        assert_eq!(mod26, Mod26::new(0));
    }

    #[deprecated]
    #[test]
    fn charからkeyに変換できる() {
        let c = 'a';
        let key = Key::from_char(c);
        assert_eq!(key, Key::A);
    }

    #[test]
    fn try_convertion_from_char() {
        assert_eq!(Key::try_from('c'), Ok(Key::C));

        assert_eq!(Key::try_from('1'), Err(()));
    }

    #[deprecated]
    #[test]
    fn charに変換できる() {
        let a = Key::A;
        let char = a.to_char();
        assert_eq!(char, 'a');
    }

    #[test]
    fn convert_into_char() {
        let v: char = Key::C.into();
        assert_eq!(v, 'c');

        let ref_key = &Key::C;
        let w: char = ref_key.into();
        assert_eq!(w, 'c')
    }
}
