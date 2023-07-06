use std::fs::read_to_string;


fn main() {
    let file = read_lines("./data/test.txt");
    let parser = Parser { lines: file };
    process_data(&parser);
}

struct Parser {
    lines: Vec<String>
}

fn process_data(parser: &Parser) {
    let unparsed_data = &parser.lines;
    for line in unparsed_data {
        println!("{}", line);
    }
    

}


struct Declaration {
    prefix: String,
    modifier: String
}


struct Prefix {

}

struct Modifier {

}


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
    .unwrap()
    .lines()
    .map(String::from)
    .collect()
}