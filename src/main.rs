use std::os;
use std::result;
use std::vec::Vec;
use std::collections::{HashMap, TreeSet};
use std::io::{BufferedReader, File};
use std::io::stdio::{stdout, stderr};

use ip::IP;
use action::Action;
use error::ParserError;

mod ip;
mod action;
mod error;

#[deriving(Copy, Clone)]
pub enum ParserResult<T> {
    Ok(T),
    Err(ParserError)
}

impl<T> ParserResult<T> {
    fn then<R>(&self, f: |&T| -> ParserResult<R>) -> ParserResult<R> {
        match self {
            &Ok(ref t) => f(t),
            &Err(ref e) => Err(e.clone())
        }
    }

    fn finally<R>(&self, f: |&T| -> R) {
        match self {
            &Ok(ref t) => { f(t); },
            &Err(ref e) => exit(e.clone())
        }
    }
}

fn parse(filename: &String) {
    read_file(filename)
        .then(parse_code)
        .finally(write_output);
}


fn read_file(filename: &String) -> ParserResult<Vec<Vec<char>>> {
    let file = File::open(&Path::new(filename.as_slice()));

    if file.is_ok() {
        let mut reader = BufferedReader::new(file);

        let mut grid = Vec::new();
        grid.push(Vec::new());

        let mut max_len = 0;

        loop {
            match reader.read_char() {
                result::Ok('\n') => {
                    if grid.last().unwrap().len() > max_len {
                        max_len = grid.last().unwrap().len()
                    }
                    grid.push(Vec::new())
                },

                result::Ok(c) => grid.mut_last().unwrap().push(c),
                result::Err(_) => break
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

fn parse_code(code: &Vec<Vec<char>>) -> ParserResult<(Vec<Vec<action::Action>>, TreeSet<action::Action>)> {
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
                        actions.get_mut(state).push(action::TablePut);
                        used_actions.insert(action::TablePut);
                    },

                    'g' => {
                        actions.get_mut(state).push(action::TableGet);
                        used_actions.insert(action::TableGet);
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

fn write_first<W: Writer>(writer: &mut W, used_actions: &TreeSet<action::Action>) {
    writer.write_line("use std::char;");
    writer.write_line("use std::vec::Vec;");

    if used_actions.contains(&action::OutputChar) || used_actions.contains(&action::OutputNumber) {
        writer.write_line("use std::io::LineBufferedWriter;");
        writer.write_line("use std::io::stdio::{StdWriter, stdout};");
    }

    if used_actions.contains(&action::InputChar) || used_actions.contains(&action::InputNumber) {
        writer.write_line("use std::io::BufferedReader;");
        writer.write_line("use std::io::stdio::{StdReader, stdin};");
    }

    if used_actions.contains(&action::TableGet) || used_actions.contains(&action::TablePut) {
        writer.write_line("use std::collections::HashMap;");
    }

    writer.write_line("");

    if used_actions.contains(&action::Jump(Vec::new())) {
        writer.write_line("fn modulus(mut a: int, b: int) -> int {");
        writer.write_line("    while a < 0 {");
        writer.write_line("        a += b");
        writer.write_line("    }");
        writer.write_line("    a % b");
        writer.write_line("}\n");
    }

    writer.write_line("struct Program {");
    writer.write_line("    stack: Vec<int>,");

    if used_actions.contains(&action::OutputChar) || used_actions.contains(&action::OutputNumber) {
        writer.write_line("    output: LineBufferedWriter<StdWriter>,");
    }

    if used_actions.contains(&action::InputChar) || used_actions.contains(&action::InputNumber) {
        writer.write_line("    input: BufferedReader<StdReader>,");
    }

    if used_actions.contains(&action::TableGet) || used_actions.contains(&action::TablePut) {
        writer.write_line("    table: HashMap<(int, int), int>,");
    }

    writer.write_line("}\n");

    writer.write_line("impl Program {");
    writer.write_line("    fn run() {");
    writer.write_line("        let mut p = Program {");
    writer.write_line("            stack: Vec::new(),");

    if used_actions.contains(&action::OutputChar) || used_actions.contains(&action::OutputNumber) {
        writer.write_line("            output: stdout(),");
    }

    if used_actions.contains(&action::InputChar) || used_actions.contains(&action::InputNumber) {
        writer.write_line("            input: stdin(),");
    }

    if used_actions.contains(&action::TableGet) || used_actions.contains(&action::TablePut) {
        writer.write_line("            table: HashMap::new(),");
    }

    writer.write_line("        };");
    writer.write_line("");
    writer.write_line("        p.state0();");
    writer.write_line("    }");

    for act in used_actions.iter() {
        act.write_impl_to(writer);
    }
}

fn write_end<W: Writer>(writer: &mut W) {
    writer.write_line("}

fn main() {
    Program::run()
}");
}

fn write_output(&(ref actions, ref used_actions): &(Vec<Vec<action::Action>>, TreeSet<action::Action>)) {
    let mut writer = stdout();

    write_first(&mut writer, used_actions);

    let mut state = 0i;
    for v in actions.iter() {
        writer.write_line(format!("\n    fn state{}(&mut self) {{", state).as_slice());

        for action in v.iter() {
            action.write_to(&mut writer);
        }

        writer.write_line("    }");

        state += 1;
    }

    write_end(&mut writer);
}

fn exit(err: ParserError) {
    let mut out = stderr();
    write!(out, "Error: {}\n", err);
}

fn print_usage() {
    println!("Usage: befunge [input]")
}

fn main() {
    let args = os::args();

    if args.len() != 2 {
        print_usage();
    } else {
        let ref filename = args[1];
        parse(filename);
    }
}
