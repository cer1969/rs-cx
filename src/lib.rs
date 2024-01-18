// Cristian Echeverría

mod k;
mod category;
mod conductor;
mod current_calc;

pub use k::*;
pub use category::*;
pub use conductor::Conductor;
pub use current_calc::CurrentCalc;

//----------------------------------------------------------------------------------------

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
