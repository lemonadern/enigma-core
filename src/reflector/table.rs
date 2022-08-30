use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};

use crate::mod26::Mod26;

pub struct ReflectorTable(HashMap<Mod26, Mod26>);

impl ReflectorTable {
    pub fn new(map: HashMap<Mod26, Mod26>) -> Result<Self, Error> {
        if map.len() != 13 {
            return Err(Error::new(ErrorKind::InvalidInput, "map length is not 13"));
        }

        let mut reflector_table = HashMap::new();

        for (&key, &value) in &map {
                        reflector_table.insert(key, value);
        }
        for (&key, &value) in &map {
            // key, value が逆向きのペアを追加する
            // value - value 間、 key - value 間 の値の重複を検出する
            if let Some(v) = reflector_table.insert(value, key) {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("duplicate key: {}", v),
                ));
            }
        }
        Ok(Self(reflector_table))
    }

    // 機能上 get 以外は必要ないので、SliceIndexトレイトの実装はしていない
    pub fn get(&self, input_position: Mod26) -> Option<&Mod26> {
        self.0.get(&input_position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 正常系() {
        // map の要素が13個である
        // value に重複がない
        // value と key の間にも重複がない
        let mut map = HashMap::new();
        for i in 0..13 {
            map.insert(Mod26::new(i), Mod26::new(i + 13));
        }
        let table = ReflectorTable::new(map);
        assert!(table.is_ok());
    }

    #[test]
    fn mapの長さが1のとき初期化できない() {
        let mut map = HashMap::new();
        map.insert(Mod26::new(0), Mod26::new(1));

        let table = ReflectorTable::new(map);
        assert!(table.is_err());
    }

    #[test]
    fn mapの長さが14のとき初期化できない() {
        let mut map = HashMap::new();
        for i in 0..14 {
            map.insert(Mod26::new(i), Mod26::new(i + 13));
        }
        let table = ReflectorTable::new(map);
        assert!(table.is_err());
    }

    #[test]
    fn valueに重複があるとき初期化できない() {
        let mut map = HashMap::new();
        for i in 0..13 {
            map.insert(Mod26::new(i), Mod26::new(0));
        }
        let table = ReflectorTable::new(map);
        assert!(table.is_err());
    }

    #[test]
    fn valueとkeyの間に重複があるとき初期化できない() {
        let mut map = HashMap::new();
        for i in 0..13 {
            map.insert(Mod26::new(i), Mod26::new(i + 12));
        }
        let table = ReflectorTable::new(map);
        assert!(table.is_err());
    }
}
