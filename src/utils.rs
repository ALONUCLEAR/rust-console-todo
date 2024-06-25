use std::{
    fs::{read_to_string, OpenOptions},
    io::Write,
};

pub fn get_input(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {}
    }

    return input.trim().to_string();
}

pub fn print_vector(vector: Vec<String>) {
    for element in vector.iter() {
        println!("{}", element);
    }
}

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    return result;
}

pub fn count_lines(filename: &str) -> u32 {
    let count = read_to_string(filename).unwrap().lines().count();

    return u32::try_from(count).unwrap();
}

pub fn add_line_at(filename: &str, line_number: usize, content: &str) {
    let mut full_content = read_lines(filename);

    if line_number >= full_content.len() {
        let mut file = OpenOptions::new().append(true).open(filename).unwrap();
        write!(&mut file, "\n{content}").unwrap();

        return;
    }

    let after_line = full_content.split_off(line_number);
    full_content.push(content.to_string());
    full_content.extend(after_line);
    let str_content = full_content.join("\n");

    let mut file = OpenOptions::new().write(true).open(filename).unwrap();
    write!(&mut file, "{str_content}").unwrap();
}

pub fn remove_line_at(filename: &str, line_number: usize) {
    let mut full_content = read_lines(filename);

    if line_number >= full_content.len() {
        println!("\nError - tried to remove a line that doesn't exist");

        return;
    }

    let mut after_line = full_content.split_off(line_number);
    after_line.remove(0);
    full_content.extend(after_line);
    let str_content = full_content.join("\n");

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename)
        .unwrap();
    write!(&mut file, "{str_content}").unwrap();
}

pub fn update_line_at(filename: &str, line_number: usize, updated_line: &str) {
    let mut full_content = read_lines(filename);

    if line_number >= full_content.len() {
        println!("\nError - tried to edit a line that doesn't exist.\nDid you mean to add?");

        return;
    }

    let mut after_line = full_content.split_off(line_number);
    after_line.remove(0);
    full_content.push(updated_line.to_string());
    full_content.extend(after_line);
    let str_content = full_content.join("\n");

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename)
        .unwrap();
    write!(&mut file, "{str_content}").unwrap();
}
