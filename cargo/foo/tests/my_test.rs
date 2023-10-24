#[cfg(test)]
mod tests {
    use std::fs::OpenOptions;
    use std::io::Write;

    #[test]
    fn test_file() {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("Ferris.txt")
            .expect("failed to open ferris.txt");

        for _ in 0..5 {
            file.write_all("Ferris\n".as_bytes())
                .expect("could not write to ferris.txt");
        }
    }

    #[test]
    fn test_file_also() {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("ferris.txt")
            .expect("failed to open ferris.txt");

        for _ in 0..5 {
            file.write_all("Corro\n".as_bytes())
                .expect("couldnot write to ferris.txt");
        }
    }
}
