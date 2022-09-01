use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};

use crate::{key::Key, mod26::Mod26};

type Position = Mod26;
type Mod26Map = HashMap<Position, Position>;

pub struct Rotor {
    forward_map: Mod26Map,
    reverse_map: Mod26Map,

    // インクリメントされたかどうか（初期位置から動いたかどうか）
    pub has_incremented: bool,
    initial_position: Mod26,
    pub offset: Mod26,
    // ※ ロータの回転状態に関するものはここで持つべきではないかも
    // todo: refactor
}

impl Rotor {
    pub fn new(map: Mod26Map) -> Result<Self, Error> {
        if map.len() != 26 {
            return Err(Error::new(ErrorKind::InvalidInput, "map length must be 26"));
        }
        let reverse_map: HashMap<Mod26, Mod26> = map.iter().map(|(&k, &v)| (v, k)).collect();
        if reverse_map.len() != 26 {
            return Err(Error::new(ErrorKind::InvalidInput, "map value duplicated"));
        }
        Ok(Self {
            initial_position: Mod26::default(),
            offset: Mod26::default(),
            forward_map: map,
            reverse_map,
            has_incremented: false,
        })
    }

    pub fn from_str(map_str: &str) -> Result<Self, Error> {
        if map_str.len() != 26 {
            return Err(Error::new(ErrorKind::InvalidInput, "str length must be 26"));
        }

        let mut map = HashMap::new();
        for (i, c) in map_str.chars().enumerate() {
            let key = Mod26::new(i as u64);

            let code = c as u8;
            if code < b'a' || code > b'z' {
                return Err(Error::new(ErrorKind::InvalidInput, "str must be lowercase"));
            }
            let value = Mod26::new((code - b'a') as u64);
            map.insert(key, value);
        }
        Self::new(map)
    }

    pub fn from_str_and_key(map_str: &str, key: Key) -> Result<Self, Error> {
        let mut rotor = Self::from_str(map_str)?;
        rotor.set_initial_position(key);
        Ok(rotor)
    }

    pub fn new_with_position(map: Mod26Map, initial_position: Key) -> Result<Self, Error> {
        let mut rotor = Self::new(map)?;
        rotor.set_initial_position(initial_position);
        Ok(rotor)
    }

    pub fn set_initial_position(&mut self, position: Key) {
        let position = position.to_mod26();
        self.initial_position = position;
        self.offset = position;
    }

    pub fn increment_offset(&mut self) {
        self.offset += Mod26(1);
        // todo: refactor
        self.has_incremented = true;
    }

    fn is_at_initial_position(&self) -> bool {
        self.offset == self.initial_position
    }

    pub fn is_rotated(&self) -> bool {
        self.has_incremented && self.is_at_initial_position()
    }

    pub fn substitute_from_forward(&self, input_position: Position) -> Position {
        let input = input_position + self.offset;
        self.forward_map.get(&input).cloned().unwrap() - self.offset
    }

    pub fn substitute_from_backward(&self, input_position: Position) -> Position {
        let input = input_position + self.offset;
        self.reverse_map.get(&input).cloned().unwrap() - self.offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 表と裏でマッピングが対応している() {
        let mut map = HashMap::new();
        // (i, 25-i) と対応するようなHashMap
        for i in 0..26 {
            map.insert(Mod26::new(i), Mod26::new(25 - i));
        }
        let rotor = Rotor::new(map).unwrap();

        let zero = Mod26::new(0);
        let twenty_five = Mod26::new(25);

        // 0 -> 25 に対応
        assert_eq!(rotor.forward_map[&zero], twenty_five);
        // 25 -> 0 に対応
        assert_eq!(rotor.reverse_map[&twenty_five], zero);
    }

    #[test]
    fn mapの要素数が25のとき初期化に失敗する() {
        let mut map = HashMap::new();
        for i in 0..25 {
            map.insert(Mod26::new(i), Mod26::new(i + 2));
        }
        let rotor = Rotor::new(map);
        assert!(rotor.is_err());
    }

    #[test]
    fn valueに重複があるとき初期化に失敗する() {
        let mut map = HashMap::new();
        for i in 0..26 {
            map.insert(Mod26::new(i), Mod26::new(0));
        }
        let rotor = Rotor::new(map);
        assert!(rotor.is_err());
    }

    #[test]
    fn strによる初期化() {
        // 正常系1
        let rotor = Rotor::from_str("abcdefghijklmnopqrstuvwxyz");
        assert!(rotor.is_ok());

        // 正常系2
        let rotor = Rotor::from_str("zyxwvutsrqponmlkjihgfedcba");
        assert!(rotor.is_ok());

        // 短い文字列のとき
        let rotor = Rotor::from_str("abc");
        assert!(rotor.is_err());

        // 長い文字列のとき
        let rotor = Rotor::from_str("abcdefghijklmnopqrstuvwxyza");
        assert!(rotor.is_err());

        // 小文字以外が混ざっているとき
        let rotor = Rotor::from_str("abcdeFGHIJKLMNOPQRSTUVWXYZ");
        assert!(rotor.is_err());

        // 重複があるとき
        let rotor = Rotor::from_str("aaaaaaaaaaaaaaaaaaaaaaaaaa");
        assert!(rotor.is_err());
    }

    #[test]
    fn offsetを26回インクリメントすると1回転する() {
        let key = Key::A;
        let mut map = HashMap::new();
        for i in 0..26 {
            map.insert(Mod26::new(i), Mod26::new(i + 2));
        }
        let mut rotor = Rotor::new_with_position(map, key).unwrap();

        assert!(rotor.is_at_initial_position());
        for _ in 0..26 {
            rotor.increment_offset();
        }

        assert!(rotor.is_at_initial_position());

        rotor.increment_offset();
        assert!(!rotor.is_at_initial_position());
    }

    #[test]
    fn offsetが0のとき換字できる() {
        let offset = Mod26::new(0);
        let zero = Mod26::new(0);
        let twenty_five = Mod26::new(25);
        let mut map = HashMap::new();
        // (i, 25-i) と対応するようなHashMap
        for i in 0..26 {
            map.insert(Mod26::new(i), Mod26::new(25 - i));
        }

        let rotor = Rotor::new_with_position(map, Key::from_mod26(offset)).unwrap();

        // 前から換字する
        assert_eq!(rotor.substitute_from_forward(zero), twenty_five);

        // 後ろから換字する
        assert_eq!(rotor.substitute_from_backward(twenty_five), zero);
    }

    #[test]
    fn offsetが2のとき換字できる() {
        let zero = Mod26::new(0);
        let twenty_one = Mod26::new(21);
        let mut map = HashMap::new();
        // (i, 25-i) と対応するようなHashMap
        for i in 0..26 {
            map.insert(Mod26::new(i), Mod26::new(25 - i));
        }
        let rotor = Rotor::new_with_position(map, Key::from_mod26(Mod26::new(2))).unwrap();

        // 前から換字すると、 0 -> (2 -> 23 ->) 21
        assert_eq!(rotor.substitute_from_forward(zero), twenty_one);

        // 後ろから換字すると、 21 -> (23 -> 2 ->) 0
        assert_eq!(rotor.substitute_from_backward(twenty_one), zero);
    }
}
