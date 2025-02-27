use std::env;
use std::io::{self, BufRead, Write};
use std::process;

#[derive(Debug)]
enum State {
    Normal,
    Escape,
    Csi,
}

fn convert_tilde_path(input_str: &str, homedir: &[u8]) -> Option<String> {
    let input = input_str.as_bytes();

    let mut result = Vec::new();
    let mut state = State::Normal;
    let mut init_done = false;

    for &b in input {
        match state {
            State::Normal => {
                if b == 0x1B { // ESC (start of an escape sequence)
                    state = State::Escape;
                    result.push(b); // append ESC byte
                } else {
                    //////////////////////////////////////////////
                    if init_done == false {
                        if b == '~' as u8 {
                            init_done = true;
                            result.extend_from_slice(homedir);
                        } else {
                            return None // mismatch detected
                        }
                    } else {
                        result.push(b); // write normal byte to result
                    }
                    //////////////////////////////////////////////
                }
            }
            State::Escape => {
                if b == 0x5B { // '[' (CSI start)
                    state = State::Csi;
                    result.push(b); // append '['
                } else {
                    state = State::Normal;
                    result.push(b); // normal byte
                }
            }
            State::Csi => {
                if b >= 0x40 && b < 0x80 { // CSI control byte range (end of escape sequence)
                    state = State::Normal;
                    result.push(b); // append CSI control byte
                } else {
                    result.push(b); // append non-control CSI bytes
                }
            }
        }
    }

    if init_done == false {
        return None;
    }

    match String::from_utf8(result) {
        Ok(str) => Some(str),
        Err(_e) => None,
    }
}

fn main() {
    let mut stdout = io::stdout();
    let home_dir = env::var("HOME").unwrap();
    let homedir_u8 = home_dir.as_bytes();

    let mut args = env::args();

    if args.len() >= 2 {
        args.next(); // skip exec name

        for arg in args {
            let res;
            if let Some(converted) = convert_tilde_path(&arg, homedir_u8) {
                res = writeln!(stdout, "{}", converted);
            } else {
                res = writeln!(stdout, "{}", arg);
            }
            if let Err(_) = res {
                process::exit(1);
            }
        }
    } else {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(ln) = line {
                let res;
                if let Some(converted) = convert_tilde_path(&ln, homedir_u8) {
                    res = writeln!(stdout, "{}", converted);
                } else {
                    res = writeln!(stdout, "{}", ln);
                }
                if let Err(_) = res {
                    process::exit(1);
                }
            }
        }
    }
}
