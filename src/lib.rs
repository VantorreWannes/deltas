mod instructions;
mod lcs;
pub mod patch;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {

    use std::fs;

    use crate::patch::Patch;

    #[test]
    fn it_works() {
        let source = fs::read("files/source.txt").unwrap();
        let target = fs::read("files/target.txt").unwrap();
        let patch = Patch::new(&source, &target);
        let result = patch.apply(&source).unwrap();
        assert_eq!(result, target);
    }
}
