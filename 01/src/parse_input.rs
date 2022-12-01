pub type ParseOutput = Vec<i32>;

pub fn read_main() -> String {
    read_file("src/01.txt")
}

pub fn read_test() -> String {
    read_file("src/test.txt")
}

pub fn read_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap()
}

pub fn parse(file: &String) -> ParseOutput {
    return file.split("\n\n").map(|package| {package.lines().fold(0, |acc, item| acc + item.parse::<i32>().unwrap())}).collect();
}
