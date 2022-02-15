pub mod common;

pub use common::cook::cook;

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    fn open(path: &str) -> String {
        let mut full_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        full_path.push(path);
        fs::read_to_string(full_path).expect(path)
    }

    #[test]
    fn cook_test() {
        let input = open("assets/test.txt");
        let expected_output = open("assets/test_mod.txt");
        let expected_trimed_output = open("assets/test_mod_trimmed.txt");
        assert_eq!(expected_output, cook(&input, false));
        assert_eq!(expected_trimed_output, cook(&input, true));
    }
}
