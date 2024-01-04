use linenoise2::linenoise2;


fn main() {
    // TODO
    loop {
        let _ = match linenoise2::linenoise2("propmt> ") {
            Some(line) => {
                println!("line: {}", line);
                line
            },
            None => {
                break;
            }
        };
    }
}