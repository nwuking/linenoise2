
pub mod linenoise2 {
    use std::collections::VecDeque;
    use std::fs::OpenOptions;
    use std::io::{BufReader, BufRead, stdout, Write, stdin, Read};
    use std::path::Path;
    use std::{u8};

    static mut S_COMPLETION_CALLBACK: fn() = || {};
    static mut S_HINTS_CALLBACK: fn() = || {};
    static mut S_HISTORY_MAX_LEN: usize = 100;
    static mut S_HISTORY: Option<VecDeque<String>> = None;

    struct Linenoise2State<'a> {
        // TODO
        propmt: &'a str,
        buf: String,
    }

    impl<'a> Linenoise2State<'a> {
        pub fn new(propmt: &'a str) -> Self {
            Self {
                propmt: propmt,
                buf: String::new(),
            }
        }
    }

    fn linenoise2_edit_insert(l: &mut Linenoise2State, c: u8) {
        // TODO
        l.buf.push(char::from(c));
    }

    pub fn linenoise2_set_completion_callback(callback: fn()) {
        unsafe {
            S_COMPLETION_CALLBACK = callback;
            // S_COMPLETION_CALLBACK();
        }
    }

    pub fn linenoise2_set_hints_callback(callback: fn()) {
        unsafe {
            S_HINTS_CALLBACK = callback;
            // S_HINTS_CALLBACK();
        }
    }

    pub fn linenoise2_history_add(line: &str) {
        // 将历史记录添加到历史队列中
        unsafe {
            if S_HISTORY_MAX_LEN == 0 {
                return;
            }

            let history = match &mut S_HISTORY {
                Some(history) => history,
                None => {
                    let history = VecDeque::new();
                    S_HISTORY = Some(history);
                    S_HISTORY.as_mut().unwrap()
                }
            };

            if history.len() == S_HISTORY_MAX_LEN {
                history.pop_front();
            }
            history.push_back(String::from(line));
        }
    }

    pub fn linenoise2_history_load(filename: &str) {
        // 打开文件
        let path = Path::new(filename);
        let file = match OpenOptions::new().read(true).write(true).create(true).open(path) {
            Ok(file) => file,
            Err(e) => panic!("file: {} open error: {}", path.display(), e),
        };

        // 读取文件内容
        let buf_reader = BufReader::new(file);
        for line in buf_reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(e) => panic!("file: {} read error: {}", path.display(), e),
            };
            linenoise2_history_add(line.as_str());
        }
    }

    pub fn linenoise2(propmt: &str) -> Option<String> {
        let mut l = Linenoise2State::new(propmt);
        let mut result: Option<String> = match stdout().write(l.propmt.as_bytes()) {
            Err(_) => None,
            _ => {
                // 刷新缓冲区
                stdout().flush().unwrap();
                Some(String::new())
            }
        };

        loop {
            // 一个字符一个字符的读取
            let mut buf: [u8; 1] = [0; 1];
            let n = match stdin().read(buf.as_mut()) {
                Err(_) => {
                    result = None;
                    break;
                },
                Ok(_) => buf[0]
            };

            let _ = match n {
                10 | 13 => {
                    // enter 
                    result = Some(l.buf);
                    break;
                }
                3 => {
                    // ctrl + c 退出程序
                    // panic!("ctrl + c");
                }
                4 => {
                    // ctrl + d
                    // todo!("ctrl + d");
                }
                8 | 127 => {
                    // backspace
                    // todo!("backspace");
                }
                27 => {
                    // esc
                    // todo!("esc");
                }
                9 => {
                    // tab
                    // todo!("tab");
                }
                _ => {
                    // other
                    linenoise2_edit_insert(&mut l, n);
                }
            };
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use crate::linenoise2;

    #[test]
    fn it_works() {
        // TODO
        let callback1 = || {
            println!("callback");
        };

        // let callback1 = test_callback;
        linenoise2::linenoise2_set_completion_callback(callback1);
        linenoise2::linenoise2_set_hints_callback(callback1);
        // let filename = "test";
        let filename = String::from("./test.txt");
        linenoise2::linenoise2_history_load(filename.as_str());
        // callback1();
    }

    #[test]
    fn test_linenoise2() {
        match linenoise2::linenoise2("propmt") {
            Some(line) => {
                println!("line: {}", line);
            },
            None => {
                println!("None");
            }
        };
    }
}
