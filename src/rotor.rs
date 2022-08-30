use std::collections::HashMap;

use crate::mod26::Mod26;

pub struct Rotor {
    offset: Mod26,
    forward_map: HashMap<Mod26, Mod26>,
    reverse_map: HashMap<Mod26, Mod26>,
}

type Position = Mod26;

impl Rotor {
    pub fn new(offset: Mod26, map: HashMap<Mod26, Mod26>) -> Self {
        let reverse_map = map.iter().map(|(&k, &v)| (v, k)).collect();
        Self {
            offset,
            forward_map: map,
            reverse_map,
        }
    }

    pub fn increment_offset(&mut self) {
        self.offset += Mod26(1);
    }

    pub fn substitute_from_forward(&self, input_position: Position) -> Position {
        let input = input_position + self.offset;
        /*
        todo: unwrapの扱い 要検討
        */
        self.forward_map.get(&input).cloned().unwrap_or(input) - self.offset
    }

    pub fn substitute_from_backward(&self, input_position: Position) -> Position {
        let input = input_position + self.offset;
        /*
        todo: unwrapの扱い 要検討
        */
 
        self.reverse_map.get(&input).cloned().unwrap_or(input) - self.offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotorの初期化() {
        let offset = Mod26::new(0);
        let mut map = HashMap::new();
        // (i, i+2) と対応するようなHashMap
        for i in 0..26 {
            map.insert(Mod26::new(i), Mod26::new(i + 2));
        }
        let mut rotor = Rotor::new(offset, map);

        let map_length = rotor.forward_map.len();
        // HashMap の要素が26個
        assert_eq!(map_length, 26);

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
    fn offsetをインクリメントできる() {
        let zero = Mod26::new(0);
        let one = Mod26::new(1);
        let map = HashMap::new();
        let mut rotor = Rotor::new(zero, map);
        rotor.increment_offset();
        // 0 -> 1 になる
        assert_eq!(rotor.offset, one);

        let twenty_five = Mod26::new(25);
        let map = HashMap::new();
        let mut rotor = Rotor::new(twenty_five, map);
        rotor.increment_offset();
        // 25 -> 0 になる
        assert_eq!(rotor.offset, zero);
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

        let rotor = Rotor::new(zero, map);

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
        let rotor = Rotor::new(two, map);

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
