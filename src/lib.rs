pub mod delta_instruction;
pub mod delta_instruction_error;
pub mod delta_instruction_traits;
pub mod delta_patch;
pub mod delta_patch_error;

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
