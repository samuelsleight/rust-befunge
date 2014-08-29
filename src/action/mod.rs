#[deriving(Show)]
pub enum Action {
    PushChar(char),
    PushNumber(int),
    OutputChar,
    OutputNumber,
    InputChar,
    Duplicate,
    Add,
    Subtract,
    Divide,
    Multiply,
    Pop,
    Swap,
    If(uint, uint),
    Compare(uint, uint, uint),
    End
}

impl Action {
    pub fn write_to<W: Writer>(&self, writer: &mut W) {
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
            &Pop => writer.write_line("        self.stack.pop();"),
            &Swap => writer.write_line("        self.swap();"),

            &If(t, f) => {
                writer.write_line("        match self.stack.pop() {");
                writer.write_line(format!("            Some(0) | None => self.state{}(),", f).as_slice());
                writer.write_line(format!("            Some(_) => self.state{}(),", t).as_slice());
                writer.write_line("        }")
            },

            &Compare(s, l, r) => {
                writer.write_line("        match (self.stack.pop(), self.stack.pop()) {");
                writer.write_line(format!("            (Some(a), Some(b)) if a < b => self.state{}(),", l).as_slice());
                writer.write_line(format!("            (Some(a), Some(b)) if a > b => self.state{}(),", r).as_slice());
                writer.write_line(format!("            (Some(_), Some(_)) | (Some(0), None) | (None, Some(0)) | (None, None) => self.state{}(),", s).as_slice());
                writer.write_line(format!("            (None, _) => self.state{}(),", l).as_slice());
                writer.write_line(format!("            (_, None) => self.state{}()", r).as_slice());
                writer.write_line("        }")
            },

            &End => writer.write_line("        ()"),

            _ => writer.write_line("")
        };
    }
}

