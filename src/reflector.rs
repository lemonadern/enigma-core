mod table;

use crate::mod26::Mod26;

use self::table::ReflectorTable;
pub struct Reflector {
    map: ReflectorTable,
}

type Position = Mod26;

impl Reflector {
    pub fn new(map: ReflectorTable) -> Self {
        Self { map }
    }

    pub fn substitute(&self, input_position: Position) -> Position {
        self.map.get(input_position).cloned().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn リフレクタの初期化() {
        let mut map = HashMap::new();
        // (i, 25 - i) i = 0..13
        for i in 0..13 {
            map.insert(Mod26::new(i), Mod26::new(25 - i));
        }

        let table = ReflectorTable::new(map).unwrap();
        let reflector = Reflector::new(table);

        // 0 -> 25
        assert_eq!(reflector.substitute(Mod26::new(0)), Mod26::new(25));

        // 25 -> 0
        assert_eq!(reflector.substitute(Mod26::new(25)), Mod26::new(0));

        // 12 -> 13
        assert_eq!(reflector.substitute(Mod26::new(12)), Mod26::new(13));

        // 13 -> 12
        assert_eq!(reflector.substitute(Mod26::new(13)), Mod26::new(12));
    }
}
