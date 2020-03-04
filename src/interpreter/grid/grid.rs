use std::{
    ops::Index,
    fmt::Debug
};

use crate::{
    inspector::Inspectable,
    interpreter::Ip,
};

#[derive(Debug, Clone)]
pub struct Grid<T>(Box<[Box<[T]>]>);

impl<T> Inspectable for Grid<T> where T: Debug {
    fn inspect(&self) {
        for row in self.0.iter() {
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
    pub fn new(vec: Vec<Vec<T>>) -> Self {
        assert!(all_rows_equal(&vec));

        Grid(vec.into_iter()
            .map(|v| v.into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice())
    }

    pub fn ip(&self) -> Ip {
        Ip::new(0, 0, self.0[0].len(), self.0.len())
    }
}

impl<T> Index<Ip> for Grid<T> {
    type Output = T;

    fn index(&self, index: Ip) -> &Self::Output {
        &self.0[index.col() as usize][index.row() as usize]
    }
}
