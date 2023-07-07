use std::fs::read_to_string;
use regex::Regex;



fn main() {
    let file_parser = FileParser::new("./data/test.txt");
    println!("{:?}", file_parser.print_lines());
    println!("Keywords: {:?}", file_parser.parse_keywords());

}


struct FileParser {
    lines: Vec<String>
}

impl FileParser {
    pub fn new(filepath: &str) -> Self {
        let file_lines = read_to_string(filepath)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
        Self { lines: file_lines }
    }

    pub fn print_lines(&self) {
        for line in &self.lines {
            println!("{}", line);
        }
    }


    pub fn parse_keywords(&self) -> Vec<String> {
        let keywords: Vec<String> = Vec::new();
        let re_declaration = Regex::new(r"^(.*?)(?=[#'])[#']").unwrap();
        let re_encaser = Regex::new(r"(?<=declaration)(?:\[(.*?)\]|\{(.*?)\})").unwrap();
        let re_modifiers = Regex::new(r"(?<=declaration)(?:[-._>]+[^-.>_]*[-._>]+)+\.").unwrap();

        // Open the file

        // Variables to store the matches
        let mut declaration: Option<String> = None;
        let mut encaser: Option<String> = None;
        let mut modifiers: Option<String> = None;

    // Process each line
        for line_result in self.lines {
            if let Ok(line) = line_result {
                // Extract the first word of the line
                let first_word = line.split_whitespace().next();

                // Check if the first word is present
                if let Some(word) = first_word {
                    // Apply the regex patterns to the first word
                    if re_declaration.is_match(word) {
                        // Declaration pattern found
                        declaration = Some(word.to_string());
                    }
                    if let Some(captures) = re_encaser.captures(word) {
                        // Encaser pattern found
                        if let Some(encaser_match) = captures.get(1) {
                            encaser = Some(encaser_match.as_str().to_string());
                        } else if let Some(encaser_match) = captures.get(2) {
                            encaser = Some(encaser_match.as_str().to_string());
                        }
                    }
                    if re_modifiers.is_match(word) {
                        // Modifiers pattern found
                        modifiers = Some(word.to_string());
                    }
                }
            }
        }
        keywords
    }
}

#[derive(Debug)]
struct Declaration {
    prefix: String,
    modifier: String,
    body: String,
}

impl Declaration {
    pub fn new(line: String) -> Self {
        Self { prefix: line.clone(), modifier: line.clone(), body: line.clone() }
    }

    pub fn print_declaration(&self) {
        println!("{}, {}, {}", self.prefix, self.modifier, self.body);
    }
}



#[derive(Default)]
pub struct DeclarationBuilder {
    pub prefix: Option<String>,
    pub modifier: Option<String>,
    pub body: Option<String>
}    

impl DeclarationBuilder {
    pub fn new() -> Self {
        DeclarationBuilder::default()
    }

    pub fn prefix(&mut self, prefix: impl Into<String>) -> &mut Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn modifier(&mut self, modifier: impl Into<String>) -> &mut Self {
        self.modifier = Some(modifier.into());
        self
    }

    pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
        self.body = Some(body.into());
        self
    }


}


