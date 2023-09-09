trait ParserStringOps {
    fn remove_new_lines(&self) -> String;
    fn remove_marked_emoji(&self) -> String;
}

impl ParserStringOps for String {
    fn remove_new_lines(&self) -> String {
        self.replace("\n", "")
    }

    fn remove_marked_emoji(&self) -> String {
        self.replace("✅", "")
    }
}

fn pre_process(raw_content: &str) -> String {
    //remove_marked_emoji(&remove_new_lines(raw_content))
    String::from(raw_content)
        .remove_new_lines()
        .remove_marked_emoji()
}

pub fn run(content: &str) {
    let processed: &str = &pre_process(content);

    println!("starting the parser\n\n{}", processed);
}
