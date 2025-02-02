const SUPPORTED_CODECS: [&str; 3] = ["lz77", "lz78", "lzw"];

pub fn list() -> String {
    SUPPORTED_CODECS.join(" ")
}

pub fn noop_identity(data: &[u8]) -> Vec<u8> {
    data.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_test() {
        let codecs = list();
        assert_eq!(codecs, SUPPORTED_CODECS.join(" "));
    }
}
