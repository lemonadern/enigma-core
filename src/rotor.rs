use std::{collections::HashMap,io::{Error, ErrorKind}};

use crate::mod26::Mod26;

type Position = Mod26;
type Mod26Map = HashMap<Position, Position>;

pub struct Rotor {
    offset: Mod26,
    forward_map: Mod26Map,
    reverse_map: Mod26Map,
}

impl Rotor {
    pub fn new(offset: Mod26, map: Mod26Map) -> Result<Self, Error> {
        if map.len() != 26 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "map length must be 26",
            ));
        }
        let reverse_map: HashMap<Mod26,Mod26> = map.iter().map(|(&k, &v)| (v, k)).collect();
        if reverse_map.len() != 26 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "map value duplicated",
            ));
        }
        Ok(Self {
            offset,
            forward_map: map,
            reverse_map,
        })
    }

    pub fn increment_offset(&mut self) {
        self.offset += Mod26(1);
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
    fn 正常系() {
        let offset = Mod26::new(0);
        let mut map = HashMap::new();
        // (i, i+2) と対応するようなHashMap
        for i in 0..26 {
            map.insert(Mod26::new(i), Mod26::new(i + 2));
        }
        let mut rotor = Rotor::new(offset, map).unwrap();


        let zero = Mod26::new(0);
        let one = Mod26::new(1);
        let two = Mod26::new(2);

        // 0 -> 2 に対応
        assert_eq!(rotor.forward_map[&zero], two);
        // 2 -> 0 に対応
        assert_eq!(rotor.reverse_map[&two], zero);

        rotor.increment_offset();
        assert_eq!(rotor.offset, one);
    }

    #[test]
    fn mapの要素数が25のとき初期化に失敗する() {
        let offset = Mod26::new(0);
        let mut map = HashMap::new();
        for i in 0..25 {
            map.insert(Mod26::new(i), Mod26::new(i + 2));
        }
        let rotor = Rotor::new(offset, map);
        assert!(rotor.is_err());
    }

    #[test]
    fn valueに重複があるとき初期化に失敗する() {
        let offset = Mod26::new(0);
        let mut map = HashMap::new();
        for i in 0..26 {
            map.insert(Mod26::new(i), Mod26::new(0));
        }
        let rotor = Rotor::new(offset, map);
        assert!(rotor.is_err());
    }

    #[test]
    fn offsetが0のとき_substitution() {
        let zero = Mod26::new(0);
        let one = Mod26::new(1);
        let mut map = HashMap::new();
        // (i, i+1) と対応するようなHashMap
        for i in 0..26 {
            map.insert(Mod26::new(i), Mod26::new(i + 1));
        }

        let rotor = Rotor::new(zero, map).unwrap();

        // 前から換字する
        assert_eq!(rotor.substitute_from_forward(zero), one);

        // 後ろから換字する
        assert_eq!(rotor.substitute_from_backward(one), zero);
    }

    #[test]
    fn offsetが2のとき_substitution() {
        let zero = Mod26::new(0);
        let one = Mod26::new(1);
        let two = Mod26::new(2);
        let mut map = HashMap::new();
        // (i, i+1) と対応するようなHashMap
        for i in 0..26 {
            map.insert(Mod26::new(i), Mod26::new(i + 1));
        }
        let rotor = Rotor::new(two, map).unwrap();

        //     offset = 2
        //     map = i -> i + 1

        // 0 -> | 2 ->\    2 |
        //      | 3    \-> 3 | -> 1
        //      | 4        4 |
        //      | 5        5 |

        // 前から換字すると、 0 -> 1
        assert_eq!(rotor.substitute_from_forward(zero), one);

        // 後ろから換字すると、 1 -> 0
        assert_eq!(rotor.substitute_from_backward(one), zero);
    }
}
