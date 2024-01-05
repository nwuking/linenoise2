use linenoise2::linenoise2;


fn main() {
    // TODO
    loop {
        let _ = match linenoise2::linenoise2("propmt> ") {
            Some(lines) => {
                for line in lines {
                    println!("line: {}", line);
                }
            },
            None => {
                break;
            }
        };
    }
}