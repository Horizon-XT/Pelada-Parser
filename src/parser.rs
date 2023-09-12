use std::str::Lines;

trait ParserStringOps {
    fn remove_marked_emoji(&self) -> String;
}

impl ParserStringOps for String {
    fn remove_marked_emoji(&self) -> String {
        self.replace("✅", "")
    }    
}

// Removes the green mark emoji, the numbers and trim the whitespaces
fn pre_process(raw_content: Lines) -> Vec<String> {
    raw_content
        .map(|line| -> String {
            let no_emoji_name: String = line.replace("✅", "");
           
            let splitted_by_dot_name: Vec<&str> = no_emoji_name.split(".").collect();
            
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

pub fn run(content: &str) {
    let lines: Lines = content.lines();

    let parsed_lines: Vec<String> = pre_process(lines);

    for line in parsed_lines {
        println!("{}", line)
    }
}
