use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};

use crate::{key::Key, mod26::Mod26};

pub struct PlugBoard {
    map: HashMap<Mod26, Mod26>,
}

impl PlugBoard {
    pub fn new(pairs: HashMap<Key, Key>) -> Result<Self, Error> {
        let mut map = HashMap::new();
        for (&k, &v) in &pairs {
            map.insert(k.to_mod26(), v.to_mod26());
        }
        for (&k, &v) in &pairs {
            if let Some(v) = map.insert(v.to_mod26(), k.to_mod26()) {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("duplicate key: {}", v),
                ));
            }
        }
        Ok(Self { map })
    }

    pub fn substitute(&self, input: Mod26) -> Mod26 {
        // 換字テーブルになければ短絡する
        self.map.get(&input).cloned().unwrap_or(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn ペアaとbを渡したとき_0を1に_1を0にそれぞれ換字できる() {
        let mut map = HashMap::new();
        map.insert(Key::A, Key::B);

        let plugboard = PlugBoard::new(map).unwrap();
        assert_eq!(plugboard.substitute(Mod26::new(0)), Mod26::new(1));
        assert_eq!(plugboard.substitute(Mod26::new(1)), Mod26::new(0));
    }

    #[test]
    fn valueが重複しているmapを渡したとき初期化に失敗する() {
        let mut map = HashMap::new();
        map.insert(Key::A, Key::C);
        map.insert(Key::B, Key::C);

        let plugboard = PlugBoard::new(map);
        assert!(plugboard.is_err());
    }

    #[test]
    fn keyとvalueが同じ値のペアがあるとき初期化に失敗する() {
        let mut map = HashMap::new();
        map.insert(Key::A, Key::A);

        let plugboard = PlugBoard::new(map);
        assert!(plugboard.is_err());
    }
}
