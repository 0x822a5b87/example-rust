use std::fs::read_to_string;

pub fn read_lines(filename: &String) -> Vec<String> {
    let lines = match read_to_string(filename) {
        Ok(v) => v,
        Err(e) => {
            println!("error opening file {}, error = {}", filename, e.to_string());
            return Vec::new();
        }
    };

    let mut result = Vec::new();
    for line in lines.lines() {
        result.push(line.to_string())
    }

    result
}
