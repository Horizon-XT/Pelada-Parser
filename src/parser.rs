use std::str::Lines;

trait ParserStringOps {
    fn remove_marked_emoji(&self) -> String;
}

impl ParserStringOps for String {
    fn remove_marked_emoji(&self) -> String {
        self.replace("✅", "")
    }    
}

// Removes the green marked emoji, the numbers and trim the whitespaces
fn pre_process(raw_content: Lines) -> Vec<String> {
    raw_content
        .map(|line| -> String { line.replace("✅", "") })
        .map(|line| -> String {
            let filtered_names: Vec<&str> = line.split(".").collect();
            match filtered_names.last() {
                Some(name) => {
                    return name.to_string();
                }
                None => {
                    return "".to_string();
                }
            }
        })
        .map(|line| -> String { line.trim().to_string() })
        .collect()
}

pub fn run(content: &str) {
    let lines: Lines = content.lines();

    let parsed_lines: Vec<String> = pre_process(lines);

    for line in parsed_lines {
        println!("{}", line)
    }
}
