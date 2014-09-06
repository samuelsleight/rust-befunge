use std::io::File;
use std::io::Command;

use super::Parser;

#[cfg(test)]
fn compilation_test(bf: &str, file: &str, expected: &str, vars: bool) {
    let bf_filename = format!("{}.b98", file);
    let rs_filename = format!("{}.rs", file);

    {
        let mut bf_file = File::create(&Path::new(bf_filename.as_slice()));
        match bf_file.write_line(bf) {
            Err(e) => fail!("Error creating test befunge: {}", e),
            _ => ()
        }
    }

    let p = Parser::new(vars, true, Some(rs_filename.to_string()));
    match p.parse(&bf_filename.to_string()) {
        Err(e) => {
            clean_files(file);
            fail!("Error parsing befunge: {}", e)
        },
        _ => ()
    }

    match Command::new("rustc").arg(rs_filename.as_slice()).output() {
        Err(e) => {
            clean_files(file);
            fail!("Compilation process error: {}", e)
        },

        _ => ()
    }

    match Command::new(format!("./{}", file)).output() {
        Ok(output) => assert_eq!(output.output.as_slice(), expected.as_bytes()),

        Err(e) => {
            clean_files(file);
            fail!("Error running compiled program: {}", e)
        }
    }

    clean_files(file)
}

#[cfg(test)]
fn clean_files(file: &str) {
    let bf_filename = format!("{}.b98", file);
    let rs_filename = format!("{}.rs", file);

    for f in vec![bf_filename.as_slice(), rs_filename.as_slice(), file].iter() {
        Command::new("rm").arg(*f).spawn();
    }
}

#[test]
fn test_simple() {
    compilation_test("0\"olleH\">:#,_@", "simp", "Hello", false);
}

#[test]
#[should_fail]
fn test_invalid_char() {
    compilation_test("m@", "inv_char", "", false);
}

#[test]
#[should_fail]
fn test_disable_var() {
    compilation_test("555p55g.@", "var_dis", "", false);
}

#[test]
fn test_enable_var() {
    compilation_test("555p55g.@", "var_en", "5", true);
}
