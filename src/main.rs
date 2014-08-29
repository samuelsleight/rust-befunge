use std::os;
use std::vec::Vec;
use std::collections::HashMap;
use std::io::{BufferedReader, File};
use std::io::stdio::stdout;

use ip::IP;
use action::Action;

mod ip;
mod action;

fn write_first<W: Writer>(writer: &mut W) {
    writer.write_line("use std::char;
use std::vec::Vec;
use std::io::{LineBufferedWriter, BufferedReader};
use std::io::stdio::{StdWriter, stdout, StdReader, stdin};

struct Program {
    stack: Vec<int>,
    output: LineBufferedWriter<StdWriter>,
    input: BufferedReader<StdReader>
}

impl Program {
    fn run() {
        let mut p = Program { 
            stack: Vec::new(),
            output: stdout(),
            input: stdin(),
        };
        
        p.state0();
    }

    fn duplicate(&mut self) {
        match self.stack.pop() {
            Some(c) => {
                self.stack.push(c);
                self.stack.push(c);
            },
            None => ()
        };
    }

    fn add(&mut self) {
        match (self.stack.pop(), self.stack.pop()) {
                (Some(a), Some(b)) => self.stack.push(a + b),
                (Some(a), None) | (None, Some(a)) => self.stack.push(a),
                (None, None) => self.stack.push(0)
        };
    }

    fn subtract(&mut self) {
        match (self.stack.pop(), self.stack.pop()) {
                (Some(a), Some(b)) => self.stack.push(b - a),
                (Some(a), None) => self.stack.push(-a),
                (None, Some(a)) => self.stack.push(a),
                (None, None) => self.stack.push(0)
        };
    }

    fn multiply(&mut self) {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(a), Some(b)) => self.stack.push(a * b),
            (_, None) | (None, _) => self.stack.push(0)
        };
    }

    fn swap(&mut self) {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(a), Some(b)) => {
                self.stack.push(a);
                self.stack.push(b);
            },

            (Some(a), None) => self.stack.push(a),

            (None, Some(a)) => {
                self.stack.push(0);
                self.stack.push(a);
            },

            _ => self.stack.push(0)
        }
    }

    fn output_char(&mut self) {
        match self.stack.pop() {
            Some(c) if char::from_u32(c as u32).is_some() => self.output.write_char(char::from_u32(c as u32).unwrap()),
            _ => self.output.write_char(0 as char)
        };

        self.output.flush();
    }

    fn output_number(&mut self) {
        match self.stack.pop() {
            Some(n) => self.output.write_int(n),
            None => self.output.write_int(0)
        };
    }
    
    fn input_char(&mut self) {
        self.stack.push(self.input.read_char().unwrap() as int);
    }");
}

fn write_end<W: Writer>(writer: &mut W) {
    writer.write_line("}

fn main() {
    Program::run()
}");
}

#[deriving(Show)]
struct Program {
    code: Vec<Vec<char>>,
}

fn print_usage() {
    println!("Usage: befunge [input]")
}

impl Program {
    fn load_from_file(filename: &String) -> Program {
        let file = File::open(&Path::new(filename.as_slice()));
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

        Program {
            code: grid
        }
    }

    fn parse(&mut self) {
        let mut ip_queue = vec![IP::new(0, 0, 1, 0)];

        let width = self.code[0].len();
        let height = self.code.len();

        let mut normal_states = Vec::new();
        normal_states.grow_fn(height, |_| {
            let mut v = Vec::new();
            v.grow(width, &HashMap::new());
            v
        });

        let mut string_states = Vec::new();
        string_states.grow_fn(height, |_| {
            let mut v = Vec::new();
            v.grow(width, &HashMap::new());
            v
        });

        let mut actions = Vec::new();

        let mut stringmode = false;
        let mut state = 0u;
        let mut next_state = 1u;

        loop {
            if state >= ip_queue.len() {
                break
            }

            let mut ip = ip_queue[state].clone();

            actions.push(Vec::new());

            loop {
                if stringmode {
                    string_states.get_mut(ip.y as uint).get_mut(ip.x as uint).find_or_insert(ip.delta(), state);

                    match self.code[ip.y as uint][ip.x as uint] {
                        '"' => stringmode = false,
                        c => actions.get_mut(state).push(action::PushChar(c))
                    }
                } else {
                    normal_states.get_mut(ip.y as uint).get_mut(ip.x as uint).find_or_insert(ip.delta(), state);

                    match self.code[ip.y as uint][ip.x as uint] {
                        '>' => ip.right(),
                        '<' => ip.left(),
                        '^' => ip.up(),
                        'v' => ip.down(),
                        '[' => ip.rotate_left(),
                        ']' => ip.rotate_right(),
                        'r' => ip.flip(),
                        '#' => ip.advance(width, height),
                        '"' => stringmode = true,
                        c @ '0'..'9' => actions.get_mut(state).push(action::PushNumber(c.to_digit(10).unwrap() as int)),
                        c @ 'a'..'f' => actions.get_mut(state).push(action::PushNumber(c.to_digit(16).unwrap() as int)),
                        '~' => actions.get_mut(state).push(action::InputChar),
                        ',' => actions.get_mut(state).push(action::OutputChar),
                        '.' => actions.get_mut(state).push(action::OutputNumber),
                        '+' => actions.get_mut(state).push(action::Add),
                        '*' => actions.get_mut(state).push(action::Multiply),
                        '-' => actions.get_mut(state).push(action::Subtract),
                        '/' => actions.get_mut(state).push(action::Divide),
                        ':' => actions.get_mut(state).push(action::Duplicate),
                        '$' => actions.get_mut(state).push(action::Pop),
                        '\\' => actions.get_mut(state).push(action::Swap),

                        '\'' => {
                            ip.advance(width, height);
                            actions.get_mut(state).push(action::PushChar(self.code[ip.y as uint][ip.x as uint]));
                        },

                        c @ '_' | c @ '|' => {
                            let true_ip = if c == '_' { ip.new_left(width, height) } else { ip.new_up(width, height) };
                            let false_ip = if c == '_' { ip.new_right(width, height) } else { ip.new_down(width, height) };

                            let true_state = *normal_states.get_mut(true_ip.y as uint).get_mut(true_ip.x as uint).find_or_insert(true_ip.delta(), next_state);
                            if true_state == next_state {
                                ip_queue.push(true_ip);
                                next_state += 1
                            }

                            let false_state = *normal_states.get_mut(false_ip.y as uint).get_mut(false_ip.x as uint).find_or_insert(false_ip.delta(), next_state);
                            if false_state == next_state {
                                ip_queue.push(false_ip);
                                next_state += 1
                            }

                            actions.get_mut(state).push(action::If(true_state, false_state));
                            break
                        },

                        'w' => {
                            let s_ip = ip.new_straight(width, height);
                            let l_ip = ip.new_turn_left(width, height);
                            let r_ip = ip.new_turn_right(width, height);

                            let s_state = *normal_states.get_mut(s_ip.y as uint).get_mut(s_ip.x as uint).find_or_insert(s_ip.delta(), next_state);
                            if s_state == next_state {
                                ip_queue.push(s_ip);
                                next_state += 1
                            }

                            let l_state = *normal_states.get_mut(l_ip.y as uint).get_mut(l_ip.x as uint).find_or_insert(l_ip.delta(), next_state);
                            if l_state == next_state {
                                ip_queue.push(l_ip);
                                next_state += 1
                            }

                            let r_state = *normal_states.get_mut(r_ip.y as uint).get_mut(r_ip.x as uint).find_or_insert(r_ip.delta(), next_state);
                            if r_state == next_state {
                                ip_queue.push(r_ip);
                                next_state += 1
                            }

                            actions.get_mut(state).push(action::Compare(s_state, l_state, r_state));
                            break
                        },

                        '@' => {
                            actions.get_mut(state).push(action::End);
                            break
                        },

                        _ => (),
                    }
                }

                ip.advance(width, height)
            }

            state += 1;
        }

        let mut writer = stdout();
        write_first(&mut writer);
        state = 0;
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
}

fn main() {
    let args = os::args();

    if args.len() != 2 {
        print_usage();
    } else {
        let ref filename = args[1];
        let mut prog = Program::load_from_file(filename);
        prog.parse();
    }
}


