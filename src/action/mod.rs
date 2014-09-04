use std::vec::Vec;
use std::io::IoError;

#[deriving(Show, PartialOrd, PartialEq, Ord, Eq)]
pub enum Action {
    PushChar(char),
    PushNumber(int),
    OutputChar,
    OutputNumber,
    InputChar,
    InputNumber,
    Duplicate,
    Add,
    Subtract,
    Divide,
    Multiply,
    Pop,
    Swap,
    Jump(Vec<uint>),
    If(uint, uint),
    Compare(uint, uint, uint),
    Clear,
    End,
    TablePut,
    TableGet
}

impl Action {
    pub fn write_to<W: Writer>(&self, writer: &mut W) -> Result<(), IoError> {
        match self {
            &PushChar(c) => writer.write_line(format!("        self.stack.push('{}' as int);", c).as_slice()),
            &PushNumber(n) => writer.write_line(format!("        self.stack.push({});", n).as_slice()),
            &OutputChar => writer.write_line("        self.output_char();"),
            &OutputNumber => writer.write_line("        self.output_number();"),
            &InputChar => writer.write_line("        self.input_char();"),
            &Duplicate => writer.write_line("        self.duplicate();"),
            &Add => writer.write_line("        self.add();"),
            &Subtract => writer.write_line("        self.subtract();"),
            &Multiply => writer.write_line("        self.multiply();"),
            &Divide => writer.write_line("        self.divide();"),
            &Pop => writer.write_line("        self.stack.pop();"),
            &Swap => writer.write_line("        self.swap();"),

            &Jump(ref v) => {
                writer.write_line("        match self.stack.pop() {")
                .and_then(|_| writer.write_line(format!("            Some(n) => match modulus(n, {}) {{", v.len()).as_slice()))
                .and_then(|_| range(0, v.len()).fold(Ok(()), |a, i| a.and_then(|_| writer.write_line(format!("                {} => self.state{}(),", i, v[i]).as_slice()))))
                .and_then(|_| writer.write_line("                _ => ()"))
                .and_then(|_| writer.write_line("            },"))
                .and_then(|_| writer.write_line(format!("            None => self.state{}()", v[0]).as_slice()))
                .and_then(|_| writer.write_line("        }"))
            },

            &If(t, f) => {
                writer.write_line("        match self.stack.pop() {")
                .and_then(|_| writer.write_line(format!("            Some(0) | None => self.state{}(),", f).as_slice()))
                .and_then(|_| writer.write_line(format!("            Some(_) => self.state{}(),", t).as_slice()))
                .and_then(|_| writer.write_line("        }"))
            },

            &Compare(s, l, r) => {
                writer.write_line("        match (self.stack.pop(), self.stack.pop()) {")
                .and_then(|_| writer.write_line(format!("            (Some(a), Some(b)) if a < b => self.state{}(),", l).as_slice()))
                .and_then(|_| writer.write_line(format!("            (Some(a), Some(b)) if a > b => self.state{}(),", r).as_slice()))
                .and_then(|_| writer.write_line(format!("            (Some(_), Some(_)) | (Some(0), None) | (None, Some(0)) | (None, None) => self.state{}(),", s).as_slice()))
                .and_then(|_| writer.write_line(format!("            (None, _) => self.state{}(),", l).as_slice()))
                .and_then(|_| writer.write_line(format!("            (_, None) => self.state{}()", r).as_slice()))
                .and_then(|_| writer.write_line("        }"))
            },

            &Clear => writer.write_line("        self.stack.clear();"),

            &End => writer.write_line("        ()"),

            &TableGet => writer.write_line("        self.table_get();"),
            &TablePut => writer.write_line("        self.table_put();"),

            _ => Ok(())
        }
    }

    pub fn write_impl_to<W: Writer>(&self, writer: &mut W) -> Result<(), IoError> {
        match self {
            &Duplicate => {
                writer.write_line("")
                .and_then(|_| writer.write_line("    fn duplicate(&mut self) {"))
                .and_then(|_| writer.write_line("        match self.stack.pop() {"))
                .and_then(|_| writer.write_line("            Some(c) => {"))
                .and_then(|_| writer.write_line("                self.stack.push(c);"))
                .and_then(|_| writer.write_line("                self.stack.push(c);"))
                .and_then(|_| writer.write_line("            },"))
                .and_then(|_| writer.write_line("            None => ()"))
                .and_then(|_| writer.write_line("        }"))
                .and_then(|_| writer.write_line("    }"))
            },

            &Add => {
                writer.write_line("")
                .and_then(|_| writer.write_line("    fn add(&mut self) {"))
                .and_then(|_| writer.write_line("        match (self.stack.pop(), self.stack.pop()) {"))
                .and_then(|_| writer.write_line("            (Some(a), Some(b)) => self.stack.push(a + b),"))
                .and_then(|_| writer.write_line("            (Some(a), None) | (None, Some(a)) => self.stack.push(a),"))
                .and_then(|_| writer.write_line("            (None, None) => self.stack.push(0)"))
                .and_then(|_| writer.write_line("        };"))
                .and_then(|_| writer.write_line("    }"))
            },


            &Subtract => {
                writer.write_line("")
                .and_then(|_| writer.write_line("    fn subtract(&mut self) {"))
                .and_then(|_| writer.write_line("        match (self.stack.pop(), self.stack.pop()) {"))
                .and_then(|_| writer.write_line("            (Some(a), Some(b)) => self.stack.push(b - a),"))
                .and_then(|_| writer.write_line("            (Some(a), None) => self.stack.push(-a),"))
                .and_then(|_| writer.write_line("            (None, Some(a)) => self.stack.push(a),"))
                .and_then(|_| writer.write_line("            (None, None) => self.stack.push(0)"))
                .and_then(|_| writer.write_line("        };"))
                .and_then(|_| writer.write_line("    }"))
            },

            &Multiply => {
                writer.write_line("")
                .and_then(|_| writer.write_line("    fn multiply(&mut self) {"))
                .and_then(|_| writer.write_line("        match (self.stack.pop(), self.stack.pop()) {"))
                .and_then(|_| writer.write_line("            (Some(a), Some(b)) => self.stack.push(a * b),"))
                .and_then(|_| writer.write_line("            (_, None) | (None, _) => self.stack.push(0)"))
                .and_then(|_| writer.write_line("        };"))
                .and_then(|_| writer.write_line("    }"))
            },

            &Divide => {
                writer.write_line("")
                .and_then(|_| writer.write_line("    fn divide(&mut self) {"))
                .and_then(|_| writer.write_line("        match (self.stack.pop(), self.stack.pop()) {"))
                .and_then(|_| writer.write_line("            (Some(a), Some(b)) => self.stack.push(b / a),"))
                .and_then(|_| writer.write_line("            (Some(a), None) => self.stack.push(0 / a),"))
                .and_then(|_| writer.write_line("            (None, Some(a)) => self.stack.push(a / 0),"))
                .and_then(|_| writer.write_line("            (None, None) => self.stack.push(0 / 0)"))
                .and_then(|_| writer.write_line("        };"))
                .and_then(|_| writer.write_line("    }"))
            },

            &Swap => {
                writer.write_line("")
                .and_then(|_| writer.write_line("    fn swap(&mut self) {"))
                .and_then(|_| writer.write_line("        match (self.stack.pop(), self.stack.pop()) {"))
                .and_then(|_| writer.write_line("            (Some(a), Some(b)) => {"))
                .and_then(|_| writer.write_line("                self.stack.push(a);"))
                .and_then(|_| writer.write_line("                self.stack.push(b);"))
                .and_then(|_| writer.write_line("            },"))
                .and_then(|_| writer.write_line(""))
                .and_then(|_| writer.write_line("            (Some(a), None) => self.stack.push(a),"))
                .and_then(|_| writer.write_line(""))
                .and_then(|_| writer.write_line("            (None, Some(a)) => {"))
                .and_then(|_| writer.write_line("                self.stack.push(0);"))
                .and_then(|_| writer.write_line("                self.stack.push(a);"))
                .and_then(|_| writer.write_line("            },"))
                .and_then(|_| writer.write_line(""))
                .and_then(|_| writer.write_line("            _ => self.stack.push(0)"))
                .and_then(|_| writer.write_line("        }"))
                .and_then(|_| writer.write_line("    }"))
            },

            &OutputChar => {
                writer.write_line("")
                .and_then(|_| writer.write_line("    fn output_char(&mut self) {"))
                .and_then(|_| writer.write_line("        match self.stack.pop() {"))
                .and_then(|_| writer.write_line("            Some(c) if char::from_u32(c as u32).is_some() => self.output.write_char(char::from_u32(c as u32).unwrap()),"))
                .and_then(|_| writer.write_line("            _ => self.output.write_char(0 as char)"))
                .and_then(|_| writer.write_line("        };"))
                .and_then(|_| writer.write_line(""))
                .and_then(|_| writer.write_line("        self.output.flush();"))
                .and_then(|_| writer.write_line("    }"))
            },

            &OutputNumber => {
                writer.write_line("")
                .and_then(|_| writer.write_line("    fn output_number(&mut self) {"))
                .and_then(|_| writer.write_line("        match self.stack.pop() {"))
                .and_then(|_| writer.write_line("            Some(n) => self.output.write_int(n),"))
                .and_then(|_| writer.write_line("            None => self.output.write_int(0)"))
                .and_then(|_| writer.write_line("        };"))
                .and_then(|_| writer.write_line("    }"))
            },
    
            &InputChar => {
                writer.write_line("")
                .and_then(|_| writer.write_line("    fn input_char(&mut self) {"))
                .and_then(|_| writer.write_line("        self.stack.push(self.input.read_char().unwrap() as int);"))
                .and_then(|_| writer.write_line("    }"))
            },

            &TableGet => {
                writer.write_line("")
                .and_then(|_| writer.write_line("    fn table_get(&mut self) {"))
                .and_then(|_| writer.write_line("        match(self.stack.pop(), self.stack.pop()) {"))
                .and_then(|_| writer.write_line("            (Some(y), Some(x)) => self.stack.push(self.table.find_or_insert((x, y), ' ' as int).clone()),"))
                .and_then(|_| writer.write_line("            (None, Some(x)) => self.stack.push(self.table.find_or_insert((x, 0), ' ' as int).clone()),"))
                .and_then(|_| writer.write_line("            (Some(y), None) => self.stack.push(self.table.find_or_insert((0, y), ' ' as int).clone()),"))
                .and_then(|_| writer.write_line("            (None, None) => self.stack.push(self.table.find_or_insert((0, 0), ' ' as int).clone())"))
                .and_then(|_| writer.write_line("        };"))
                .and_then(|_| writer.write_line("    }"))
            },

            &TablePut => {
                writer.write_line("")
                .and_then(|_| writer.write_line("    fn table_put(&mut self) {"))
                .and_then(|_| writer.write_line("        match(self.stack.pop(), self.stack.pop()) {"))
                .and_then(|_| writer.write_line("            (Some(y), Some(x)) => self.table.insert((x, y), self.stack.pop().unwrap_or(0)),"))
                .and_then(|_| writer.write_line("            (None, Some(x)) => self.table.insert((x, 0), self.stack.pop().unwrap_or(0)),"))
                .and_then(|_| writer.write_line("            (Some(y), None) => self.table.insert((0, y), self.stack.pop().unwrap_or(0)),"))
                .and_then(|_| writer.write_line("            (None, None) => self.table.insert((0, 0), self.stack.pop().unwrap_or(0))"))
                .and_then(|_| writer.write_line("        };"))
                .and_then(|_| writer.write_line("    }"))
            },

            _ => Ok(())
        }
    }
}

