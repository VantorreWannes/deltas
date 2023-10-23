mod instructions;
mod lcs;
pub mod patch;

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::patch::Patch;

    #[test]
    fn speed() {
        let source = fs::read("files/source.txt").unwrap();
        let target = fs::read("files/target.txt").unwrap();
        let patch = Patch::new(&source, &target);
        let patch_bytes = patch.to_bytes();
        fs::write("files/raw_patch", patch_bytes).unwrap();
    }

    #[test]
    fn it_works() {
        let source = fs::read("files/source.txt").unwrap();
        let target = fs::read("files/target.txt").unwrap();
        let patch = Patch::new(&source, &target);
        let result = patch.apply(&source).unwrap();
        assert_eq!(result, target);
    }

    #[test]
    fn patch_bytes() {
        let source = fs::read("files/source.txt").unwrap();
        let target = fs::read("files/target.txt").unwrap();
        let patch = Patch::new(&source, &target);
        let patch_bytes = patch.to_bytes();
        let constructed_patch = Patch::try_from_bytes(&patch_bytes).unwrap();
        assert_eq!(patch, constructed_patch);
    }
}
