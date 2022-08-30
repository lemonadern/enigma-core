pub mod mod26;
pub mod plugboard;
pub mod reflector;
pub mod rotor;

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
