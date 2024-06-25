mod utils;
use utils::*;

fn main() {
    help_user("files/example.txt");
}

fn help_user(list_file_path: &str) {
    const QUIT_OPTION: &str = "6";
    let prompt = format!(
        "Choose your action:
    1. View list
    2. Add to list
    3. Remove from list
    4. Update list element
    5. Count the elements in the list
    {QUIT_OPTION}. Quit"
    );
    let choice = get_input(prompt.as_str());
    println!(""); //throw an empty line before any response

    // match is rust's switch case
    match choice.as_str() {
        "1" => {
            let tasks = get_task_list(list_file_path);
            println!("These are all the tasks you currently have listed:");
            print_vector(tasks);
        }
        "2" => {
            add_to_list(list_file_path);
        }
        "3" => {
            remove_from_list(list_file_path);
        }
        "4" => {
            update_list_element(list_file_path);
        }
        "5" => {
            let count = count_lines(list_file_path);
            println!("The list currently contains {count} lines.");
        }
        QUIT_OPTION => {
            println!("Thanks for using my console todo. See you soon :)");
        }
        _ => {
            println!("Invalid action. Showing the menu again...");
        }
    }

    if choice != QUIT_OPTION {
        println!("\n\n\n\n");
        help_user(list_file_path);
    }
}

fn get_task_list(filename: &str) -> Vec<String> {
    let lines = read_lines(filename);

    return lines
        .iter()
        .enumerate()
        .map(|(index, line)| format!("{}. {}", index + 1, line))
        .collect::<Vec<String>>();
}

fn get_line_in_file_from_user(filename: &str, top_offset: u32, prompt: &str) -> u32 {
    let line_count = count_lines(filename);
    let upper_limit = u32::min(line_count + top_offset, u32::try_from(u16::MAX).unwrap());
    let line = get_input(prompt);
    let line_num = match line.parse::<u32>() {
        Ok(val) => val,
        Err(_) => u32::MAX,
    };

    if line_num > upper_limit || line_num < 1 {
        println!("!!!!!\nInvalid line number - it should be between 1 and {upper_limit}!\n!!!!!");

        return u32::MAX;
    }

    return line_num;
}

fn add_to_list(filename: &str) {
    let line_num = get_line_in_file_from_user(
        filename,
        1,
        "Enter the number of the task you want to add: ",
    );

    if line_num == u32::MAX {
        return;
    }

    let num = usize::try_from(line_num).unwrap();
    let prompt = format!("\nWhat do you want your {num}th task to be called?");
    let content = get_input(prompt.as_str());

    add_line_at(filename, num - 1, content.as_str());

    println!("\nTask added successfully!");
}

fn remove_from_list(filename: &str) {
    let line_num = get_line_in_file_from_user(
        filename,
        0,
        "Enter the number of the task you want to remove: ",
    );

    if line_num == u32::MAX {
        return;
    }

    let num = usize::try_from(line_num).unwrap();
    remove_line_at(filename, num - 1);

    println!("\nTask removed successfully!");
}

fn update_list_element(filename: &str) {
    let line_num = get_line_in_file_from_user(
        filename,
        0,
        "Enter the number of the task you want to update: ",
    );

    if line_num == u32::MAX {
        return;
    }

    let index = usize::try_from(line_num).unwrap() - 1;
    let lines = read_lines(filename);
    let line_to_update = lines.get(index).unwrap();
    let prompt = format!(
        "\nYour {}th task is currently called: \"{}\"\nWhat do you want it to be called now?",
        index + 1,
        line_to_update
    );
    let new_task_name = get_input(prompt.as_str());
    update_line_at(filename, index, new_task_name.as_str());

    println!("\nTask updated successfully!");
}
