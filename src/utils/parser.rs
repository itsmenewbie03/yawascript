pub fn _tokenize(file: std::path::PathBuf) {
    let contents = std::fs::read_to_string(file);
    println!("{}", contents.unwrap());
}
