#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Mod26(pub u64);
impl Mod26 {
    pub fn new(i: u64) -> Self {
        Self(i % 26)
    }

    fn add(self, other: u64) -> Self {
        let sum = self.0 + other;
        Self(sum % 26)
    }

    fn sub(self, other: u64) -> Self {
        let diff = self.0 as i64 - other as i64;
        let diff = if diff < 0 { diff + 26 } else { diff };
        Self(diff as u64)
    }
}

impl std::ops::Add for Mod26 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        self.add(other.0)
    }
}

impl std::ops::Sub for Mod26 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        self.sub(other.0)
    }
}

impl std::ops::AddAssign for Mod26 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl std::fmt::Display for Mod26 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization() {
        // 0 = 0
        let zero = Mod26::new(0);
        assert_eq!(zero, Mod26(0));

        // 26 = 0
        let twenty_six = Mod26::new(26);
        assert_eq!(twenty_six, zero);
    }

    #[test]
    fn addiction() {
        let zero = Mod26::new(0);
        let one = Mod26::new(1);
        let twenty_five = Mod26::new(25);

        // 0 + 1 = 1
        assert_eq!(zero + one, one);
        // 25 + 1 = 0
        assert_eq!(twenty_five + one, zero);
    }

    #[test]
    fn subtraction() {
        let zero = Mod26::new(0);
        let one = Mod26::new(1);
        let two = Mod26::new(2);

        // 2 - 1 = 1
        assert_eq!(two - one, one);
        // 0 - 1 = 25
        assert_eq!(zero - one, Mod26(25));
    }
}
