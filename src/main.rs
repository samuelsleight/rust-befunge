use std::os;
use std::vec::Vec;
use std::collections::{HashMap, TreeSet};
use std::io::{BufferedReader, File, IoError};
use std::io::stdio::{stdout, stderr};

use ip::IP;
use action::Action;
use error::ParserError;

mod ip;
mod action;
mod error;

struct Parser {
    vars_enabled: bool,
    output_file: Option<String>
}

impl Parser {
    fn new(vars: bool, output: Option<String>) -> Parser {
        Parser {
            vars_enabled: vars,
            output_file: output
        }
    }

    fn parse(&self, filename: &String) -> Result<(), ParserError> {
        self.read_file(filename)
            .and_then(|a| self.parse_code(a))
            .and_then(|a| self.write_output(a))
    }

    fn read_file(&self, filename: &String) -> Result<Vec<Vec<char>>, ParserError> {
        let file = File::open(&Path::new(filename.as_slice()));

        if file.is_ok() {
            let mut reader = BufferedReader::new(file);

            let mut grid = Vec::new();
            grid.push(Vec::new());

            let mut max_len = 0;

            loop {
                match reader.read_char() {
                    Ok('\n') => {
                        if grid.last().unwrap().len() > max_len {
                            max_len = grid.last().unwrap().len()
                        }
                        grid.push(Vec::new())
                    },

                    Ok(c) => grid.mut_last().unwrap().push(c),
                    Err(_) => break
                }
            }

            let rows = grid.len() - 1;
            grid.truncate(rows);

            for row in grid.mut_iter() {
                let inc = max_len - row.len();
                if inc > 0 {
                    row.grow(inc, &' ')
                }
            }

            if grid.is_empty() {
                Err(error::FileEmptyError(filename.clone()))
            } else {
                Ok(grid)
            }
        } else {
            Err(error::FileReadError(filename.clone()))
        }
    }

    fn parse_code(&self, code: Vec<Vec<char>>) -> Result<(Vec<Vec<action::Action>>, TreeSet<action::Action>), ParserError> {
        let mut ip_queue = vec![IP::new(0, 0, 1, 0)];

        let width = code[0].len();
        let height = code.len();

        let mut states = Vec::new();
        states.grow_fn(height, |_| {
            let mut v = Vec::new();
            v.grow(width, &HashMap::new());
            v
        });

        let mut actions = Vec::new();
        let mut used_actions = TreeSet::new();

        let mut stringmode = false;
        let mut state = 0u;
        let mut next_state = 1u;

        loop {
            if state >= ip_queue.len() {
                break
            }

            let mut ip = ip_queue[state].clone();
            states.get_mut(ip.y as uint).get_mut(ip.x as uint).find_or_insert(ip.delta(), state);

            actions.push(Vec::new());

            loop {
                if stringmode {
                    match code[ip.y as uint][ip.x as uint] {
                        '"' => stringmode = false,
                        c => { 
                            actions.get_mut(state).push(action::PushChar(c));
                            used_actions.insert(action::PushChar(' '));
                        }
                    }
                } else {
                    match code[ip.y as uint][ip.x as uint] {
                        '>' => ip.right(),
                        '<' => ip.left(),
                        '^' => ip.up(),
                        'v' => ip.down(),
                        '[' => ip.rotate_left(),
                        ']' => ip.rotate_right(),
                        'r' => ip.flip(),
                        '#' => ip.advance(width, height),
                        '"' => stringmode = true,

                        c @ '0'..'9' => { 
                            actions.get_mut(state).push(action::PushNumber(c.to_digit(10).unwrap() as int));
                            used_actions.insert(action::PushNumber(0));
                        },

                        c @ 'a'..'f' => {
                            actions.get_mut(state).push(action::PushNumber(c.to_digit(16).unwrap() as int));
                            used_actions.insert(action::PushNumber(0));
                        },

                        '~' => {
                            actions.get_mut(state).push(action::InputChar);
                            used_actions.insert(action::InputChar);
                        },

                        ',' => {
                            actions.get_mut(state).push(action::OutputChar);
                            used_actions.insert(action::OutputChar);
                        },

                        '.' => {
                            actions.get_mut(state).push(action::OutputNumber);
                            used_actions.insert(action::OutputNumber);
                        },

                        '+' => {
                            match (actions.get_mut(state).pop(), actions.get_mut(state).pop()) {
                                (Some(action::PushNumber(a)), Some(action::PushNumber(b))) => actions.get_mut(state).push(action::PushNumber(a + b)),
                                (Some(action::PushChar(c)), Some(action::PushNumber(n))) 
                              | (Some(action::PushNumber(n)), Some(action::PushChar(c))) => actions.get_mut(state).push(action::PushNumber(n + (c as int))),
                                (Some(action::PushChar(a)), Some(action::PushChar(b))) => actions.get_mut(state).push(action::PushNumber(a as int + b as int)),

                                (Some(a), Some(b)) => {
                                    actions.get_mut(state).push(b);
                                    actions.get_mut(state).push(a);
                                    actions.get_mut(state).push(action::Add);
                                    used_actions.insert(action::Add);
                                },

                                (None, Some(a)) | (Some(a), None) => {
                                    actions.get_mut(state).push(a);
                                    actions.get_mut(state).push(action::Add);
                                    used_actions.insert(action::Add);
                                },

                                (None, None) => {
                                    actions.get_mut(state).push(action::Add);
                                    used_actions.insert(action::Add);
                                },
                            }
                        },

                        '*' => {
                            match (actions.get_mut(state).pop(), actions.get_mut(state).pop()) {
                                (Some(action::PushNumber(a)), Some(action::PushNumber(b))) => actions.get_mut(state).push(action::PushNumber(a * b)),
                                (Some(action::PushChar(c)), Some(action::PushNumber(n))) 
                              | (Some(action::PushNumber(n)), Some(action::PushChar(c))) => actions.get_mut(state).push(action::PushNumber(n * (c as int))),
                                (Some(action::PushChar(a)), Some(action::PushChar(b))) => actions.get_mut(state).push(action::PushNumber(a as int * b as int)),

                                (Some(a), Some(b)) => {
                                    actions.get_mut(state).push(b);
                                    actions.get_mut(state).push(a);
                                    actions.get_mut(state).push(action::Multiply);
                                    used_actions.insert(action::Multiply);
                                },

                                (None, Some(a)) | (Some(a), None) => {
                                    actions.get_mut(state).push(a);
                                    actions.get_mut(state).push(action::Multiply);
                                    used_actions.insert(action::Multiply);
                                },

                                (None, None) => {
                                    actions.get_mut(state).push(action::Multiply);
                                    used_actions.insert(action::Multiply);
                                },
                            }
                        },

                        '-' => {
                            match (actions.get_mut(state).pop(), actions.get_mut(state).pop()) {
                                (Some(action::PushNumber(a)), Some(action::PushNumber(b))) => actions.get_mut(state).push(action::PushNumber(b - a)),
                                (Some(action::PushChar(a)), Some(action::PushNumber(b))) => actions.get_mut(state).push(action::PushNumber(b - (a as int))),
                                (Some(action::PushNumber(a)), Some(action::PushChar(b))) => actions.get_mut(state).push(action::PushNumber((b as int) - a)),
                                (Some(action::PushChar(a)), Some(action::PushChar(b))) => actions.get_mut(state).push(action::PushNumber(b as int - a as int)),

                                (Some(a), Some(b)) => {
                                    actions.get_mut(state).push(b);
                                    actions.get_mut(state).push(a);
                                    actions.get_mut(state).push(action::Subtract);
                                    used_actions.insert(action::Subtract);
                                },

                                (None, Some(a)) | (Some(a), None) => {
                                    actions.get_mut(state).push(a);
                                    actions.get_mut(state).push(action::Subtract);
                                    used_actions.insert(action::Subtract);
                                },

                                (None, None) => {
                                    actions.get_mut(state).push(action::Subtract);
                                    used_actions.insert(action::Subtract);
                                },
                            }
                        },

                        '/' => {
                            match (actions.get_mut(state).pop(), actions.get_mut(state).pop()) {
                                (Some(action::PushNumber(a)), Some(action::PushNumber(b))) => actions.get_mut(state).push(action::PushNumber(b / a)),
                                (Some(action::PushChar(a)), Some(action::PushNumber(b))) => actions.get_mut(state).push(action::PushNumber(b / (a as int))),
                                (Some(action::PushNumber(a)), Some(action::PushChar(b))) => actions.get_mut(state).push(action::PushNumber((b as int) / a)),
                                (Some(action::PushChar(a)), Some(action::PushChar(b))) => actions.get_mut(state).push(action::PushNumber(b as int / a as int)),

                                (Some(a), Some(b)) => {
                                    actions.get_mut(state).push(b);
                                    actions.get_mut(state).push(a);
                                    actions.get_mut(state).push(action::Divide);
                                    used_actions.insert(action::Divide);
                                },

                                (None, Some(a)) | (Some(a), None) => {
                                    actions.get_mut(state).push(a);
                                    actions.get_mut(state).push(action::Divide);
                                    used_actions.insert(action::Divide);
                                },

                                (None, None) => {
                                    actions.get_mut(state).push(action::Divide);
                                    used_actions.insert(action::Divide);
                                },
                            }
                        },

                        ':' => {
                            actions.get_mut(state).push(action::Duplicate);
                            used_actions.insert(action::Duplicate);
                        },

                        '$' => {
                            actions.get_mut(state).push(action::Pop);
                            used_actions.insert(action::Pop);
                        },

                        '\\' => {
                            actions.get_mut(state).push(action::Swap);
                            used_actions.insert(action::Swap);
                        },

                        '\'' => {
                            ip.advance(width, height);
                            actions.get_mut(state).push(action::PushChar(code[ip.y as uint][ip.x as uint]));
                            used_actions.insert(action::PushChar(' '));
                        },

                        'j' => {
                            let mut new_ip = ip.clone();
                            let mut jump_vec = Vec::new();

                            loop {
                                new_ip.advance(width, height);

                                let new_state = *states.get_mut(new_ip.y as uint).get_mut(new_ip.x as uint).find_or_insert(new_ip.delta(), next_state);
                                if new_state == next_state {
                                    ip_queue.push(new_ip);
                                    next_state += 1;
                                }
                                jump_vec.push(new_state);

                                if new_ip == ip {
                                    break
                                }
                            };

                            actions.get_mut(state).push(action::Jump(jump_vec));
                            used_actions.insert(action::Jump(Vec::new()));
                            break
                        },

                        c @ '_' | c @ '|' => {
                            let true_ip = if c == '_' { ip.new_left(width, height) } else { ip.new_up(width, height) };
                            let false_ip = if c == '_' { ip.new_right(width, height) } else { ip.new_down(width, height) };

                            let true_state = *states.get_mut(true_ip.y as uint).get_mut(true_ip.x as uint).find_or_insert(true_ip.delta(), next_state);
                            if true_state == next_state {
                                ip_queue.push(true_ip);
                                next_state += 1
                            }

                            let false_state = *states.get_mut(false_ip.y as uint).get_mut(false_ip.x as uint).find_or_insert(false_ip.delta(), next_state);
                            if false_state == next_state {
                                ip_queue.push(false_ip);
                                next_state += 1
                            }

                            actions.get_mut(state).push(action::If(true_state, false_state));
                            used_actions.insert(action::If(0, 0));
                            break
                        },

                        'w' => {
                            let s_ip = ip.new_straight(width, height);
                            let l_ip = ip.new_turn_left(width, height);
                            let r_ip = ip.new_turn_right(width, height);

                            let s_state = *states.get_mut(s_ip.y as uint).get_mut(s_ip.x as uint).find_or_insert(s_ip.delta(), next_state);
                            if s_state == next_state {
                                ip_queue.push(s_ip);
                                next_state += 1
                            }

                            let l_state = *states.get_mut(l_ip.y as uint).get_mut(l_ip.x as uint).find_or_insert(l_ip.delta(), next_state);
                            if l_state == next_state {
                                ip_queue.push(l_ip);
                                next_state += 1
                            }

                            let r_state = *states.get_mut(r_ip.y as uint).get_mut(r_ip.x as uint).find_or_insert(r_ip.delta(), next_state);
                            if r_state == next_state {
                                ip_queue.push(r_ip);
                                next_state += 1
                            }

                            actions.get_mut(state).push(action::Compare(s_state, l_state, r_state));
                            used_actions.insert(action::Compare(0, 0, 0));
                            break
                        },

                        'n' => {
                            actions.get_mut(state).push(action::Clear);
                            used_actions.insert(action::Clear);
                        },

                        '@' => {
                            actions.get_mut(state).push(action::End);
                            used_actions.insert(action::End);
                            break
                        },

                        'p' => {
                            if self.vars_enabled {
                                actions.get_mut(state).push(action::TablePut);
                                used_actions.insert(action::TablePut);
                            } else {
                                return Err(error::VarsDisabled)
                            }
                        },

                        'g' => {
                            if self.vars_enabled {
                                actions.get_mut(state).push(action::TableGet);
                                used_actions.insert(action::TableGet);
                            } else {
                                return Err(error::VarsDisabled)
                            }
                        },

                        ' ' => (),

                        c @ _ => return Err(error::UnexpectedChar(ip.x, ip.y, c))
                    }
                }

                ip.advance(width, height)
            }

            state += 1;
        }

        Ok((actions, used_actions))
    }

    fn write_first<W: Writer>(&self, writer: &mut W, used_actions: &TreeSet<action::Action>) -> Result<(), IoError> {
        writer.write_line("use std::char;")
        .and_then(|_| writer.write_line("use std::vec::Vec;"))

        .and_then(|_| if used_actions.contains(&action::OutputChar) || used_actions.contains(&action::OutputNumber) {
            writer.write_line("use std::io::LineBufferedWriter;")
            .and_then(|_| writer.write_line("use std::io::stdio::{StdWriter, stdout};"))
        } else { Ok(()) })

        .and_then(|_| if used_actions.contains(&action::InputChar) || used_actions.contains(&action::InputNumber) {
            writer.write_line("use std::io::BufferedReader;")
            .and_then(|_| writer.write_line("use std::io::stdio::{StdReader, stdin};"))
        } else { Ok(()) })

        .and_then(|_| if used_actions.contains(&action::TableGet) || used_actions.contains(&action::TablePut) {
            writer.write_line("use std::collections::HashMap;")
        } else { Ok(()) })

        .and_then(|_| writer.write_line(""))

        .and_then(|_| if used_actions.contains(&action::Jump(Vec::new())) {
            writer.write_line("fn modulus(mut a: int, b: int) -> int {")
            .and_then(|_| writer.write_line("    while a < 0 {"))
            .and_then(|_| writer.write_line("        a += b"))
            .and_then(|_| writer.write_line("    }"))
            .and_then(|_| writer.write_line("    a % b"))
            .and_then(|_| writer.write_line("}\n"))
        } else { Ok(()) })

        .and_then(|_| writer.write_line("struct Program {"))
        .and_then(|_| writer.write_line("    stack: Vec<int>,"))

        .and_then(|_| if used_actions.contains(&action::OutputChar) || used_actions.contains(&action::OutputNumber) {
            writer.write_line("    output: LineBufferedWriter<StdWriter>,")
        } else { Ok(()) })

        .and_then(|_| if used_actions.contains(&action::InputChar) || used_actions.contains(&action::InputNumber) {
            writer.write_line("    input: BufferedReader<StdReader>,")
        } else { Ok(()) })

        .and_then(|_| if used_actions.contains(&action::TableGet) || used_actions.contains(&action::TablePut) {
            writer.write_line("    table: HashMap<(int, int), int>,")
        } else { Ok(()) })

        .and_then(|_| writer.write_line("}\n"))

        .and_then(|_| writer.write_line("impl Program {"))
        .and_then(|_| writer.write_line("    fn run() {"))
        .and_then(|_| writer.write_line("        let mut p = Program {"))
        .and_then(|_| writer.write_line("            stack: Vec::new(),"))

        .and_then(|_| if used_actions.contains(&action::OutputChar) || used_actions.contains(&action::OutputNumber) {
            writer.write_line("            output: stdout(),")
        } else { Ok(()) })

        .and_then(|_| if used_actions.contains(&action::InputChar) || used_actions.contains(&action::InputNumber) {
            writer.write_line("            input: stdin(),")
        } else { Ok(()) })

        .and_then(|_| if used_actions.contains(&action::TableGet) || used_actions.contains(&action::TablePut) {
            writer.write_line("            table: HashMap::new(),")
        } else { Ok(()) })

        .and_then(|_| writer.write_line("        };"))
        .and_then(|_| writer.write_line(""))
        .and_then(|_| writer.write_line("        p.state0();"))
        .and_then(|_| writer.write_line("    }"))

        .and_then(|_| used_actions.iter().fold(Ok(()), |acc, act| acc.and_then(|_| act.write_impl_to(writer))))
    }

    fn write_end<W: Writer>(&self, writer: &mut W) -> Result<(), IoError> {
        writer.write_line("}

fn main() {
    Program::run()
}")
    }

    fn write_output(&self, (actions, used_actions): (Vec<Vec<action::Action>>, TreeSet<action::Action>)) -> Result<(), ParserError> {
        let mut state = 0i;

        match self.output_file {
            Some(ref f) => {
                let mut writer = File::create(&Path::new(f.clone()));
                self.write_first(&mut writer, &used_actions)

                .and_then(|_| actions.iter().fold(Ok(()), |acc, vec| acc.and_then(|_| {
                    writer.write_line(format!("\n    fn state{}(&mut self) {{", state).as_slice())
                    .and_then(|_| vec.iter().fold(Ok(()), |acc2, act| acc2.and_then(|_| act.write_to(&mut writer))))
                    .and_then(|_| writer.write_line("    }"))
                    .map(|_| state += 1)
                })))

                .and_then(|_| self.write_end(&mut writer))
                .map_err(|_| error::OutputError)
            },
                
            None => {
                let mut writer = stdout(); 
                self.write_first(&mut writer, &used_actions)

                .and_then(|_| actions.iter().fold(Ok(()), |acc, vec| acc.and_then(|_| {
                    writer.write_line(format!("\n    fn state{}(&mut self) {{", state).as_slice())
                    .and_then(|_| vec.iter().fold(Ok(()), |acc2, act| acc2.and_then(|_| act.write_to(&mut writer))))
                    .and_then(|_| writer.write_line("    }"))
                    .map(|_| state += 1)
                })))

                .and_then(|_| self.write_end(&mut writer))
                .map_err(|_| error::OutputError)
            }
        }
    }
}

fn exit(err: ParserError) {
    let mut out = stderr();
    if write!(out, "Error: {}\n", err).is_err() {
        fail!("Error reporting error")
    }
}

fn print_usage() {
    println!("Usage: befunge [options] input")
}

fn main() {
    let args = os::args();

    let mut help = false;
    let mut vars = false;
    let mut filename = None;
    let mut output = None;

    let mut i = 1u;
    loop {
        if i >= args.len() {
            break
        }

        match args[i].as_slice() {
            "-h" | "--help" => {
                help = true;
                break
            },

            "-o" | "--output" => {
                output = Some(args[i + 1].clone());
                i += 1
            },

            "--enable-vars" => vars = true,

            s => filename = Some(s.to_string())
        }

        i += 1
    }

    if help || filename.is_none() {
        return print_usage()
    }

    let parser = Parser::new(vars, output);

    match parser.parse(&filename.unwrap()) {
        Err(e) => exit(e),
        _ => ()
    }
}
