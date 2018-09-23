use std::fmt::Debug;

use inspector::Inspectable;

#[derive(Debug)]
pub struct Grid<T>(Vec<Vec<T>>);

impl<T> Inspectable for Grid<T> where T: Debug {
    fn inspect(&self) {
        for row in &self.0 {
            println!("{:?}", row);
        }
    }
}

fn all_rows_equal<T>(grid: &[Vec<T>]) -> bool {
    grid.iter().fold((None, true), |(len, eq), row| match len {
        Some(len) => (Some(len), eq && len == row.len()),
        None => (Some(row.len()), true)
    }).1
}

impl<T> Grid<T> {
    pub fn new(vec: Vec<Vec<T>>) -> Grid<T> {
        assert!(all_rows_equal(&vec));

        Grid(vec)
    }
}