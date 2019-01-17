use ressa::*;

static JS: &str = "
function Thing(stuff) {
    this.stuff = stuff;
}
";

fn main() {
    let parser = Parser::new(JS).expect("Failed to create parser");
    for part in parser {
        let part = part.expect("Failed to parse part");
        println!("{:?}", part);
    }
}