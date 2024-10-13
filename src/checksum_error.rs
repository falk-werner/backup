#[derive(Debug, Clone)]
pub struct ChecksumError {
    pub file_names: Vec<String>
}

impl ChecksumError {
    pub fn new() -> Self {
        ChecksumError { file_names: vec!() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let error = ChecksumError::new();
        assert_eq!(0, error.file_names.len());
    }
}