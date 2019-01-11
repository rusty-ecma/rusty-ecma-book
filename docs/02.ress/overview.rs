use ress::*;

main() {
    let js = "var i = 0;";
    let scanner = Scanner::new(js);
    for token in scanner {
        println!("{:#?}", token);
    }
}