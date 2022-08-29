#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Modulo26(pub u64);
impl Modulo26 {
    pub fn new(i: u64) -> Self {
        Self(i % 26)
    }
    pub fn add(self, other: u64) -> Self {
        let sum = self.0 + other;
        Self(sum % 26)
    }
}

impl std::ops::Add for Modulo26 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        self.add(other.0)
    }
}

impl std::ops::AddAssign for Modulo26 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl std::fmt::Display for Modulo26 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let m = Modulo26::new(0);
        assert_eq!(m, Modulo26(0));

        let m = Modulo26::new(26);
        assert_eq!(m, Modulo26(0));

        let added = m + Modulo26(26);
        assert_eq!(added, m);
    }
}
