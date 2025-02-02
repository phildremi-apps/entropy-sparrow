use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

pub fn read_file(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn write_file(path: &str, data: &[u8]) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn extremely_suboptimal_read_write_test() {
        // That's actually an integration test - but it won't be here for long, anyway
        let input_file_path = "../LICENSE";
        let output_file_path = "LICENSE.copy.txt";
        let file_contents = read_file(input_file_path).expect("Failed to read {input_file_path}");
        write_file(output_file_path, &file_contents).expect("Failed to write {output_file_path}");
        fs::remove_file(output_file_path).expect("Failed to unlink {output_file_path}");
    }
}
