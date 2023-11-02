use std::str::Lines;

// Removes the green mark emoji, the numbers and trim the whitespaces
fn pre_process(raw_content: Lines) -> Vec<String> {
    raw_content
        .map(|line| -> String {
            let no_check_emoji_name: String = line.replace("✅", "");

            let no_x_emoji_name: String = no_check_emoji_name.replace("❌", "");

            let no_colon_name: String = no_x_emoji_name.replace(":", "");

            let splitted_by_dot_name: Vec<&str> = no_colon_name.split(".").collect();

            match splitted_by_dot_name.last() {
                Some(name) => {
                    return name.trim().to_string();
                }
                None => {
                    return "".to_string();
                }
            }
        })
        .filter(|line| -> bool { line != "" })
        .collect()
}

pub fn run(content: &str) -> Vec<String> {
    let lines: Lines = content.lines();

    let mut parsed_list: Vec<String> = pre_process(lines);

    parsed_list.remove(0);

    parsed_list
}
