pub mod mod26;
pub mod reflector;
pub mod rotor;
pub mod plugboard;

#[cfg(test)]
extern crate speculate;

mod tests {
    use speculate::speculate;

    speculate! {
        describe "add 関数" {
            it "1 + 1 = 2" {
                assert_eq!(1+1,2)
            }

            #[should_panic]
            it "パニックする" {
                assert_eq!(1+1 , 3)
            }
        }
    }
}
