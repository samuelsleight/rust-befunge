use std::{
    cmp,
    path::Path,
    fs::File,
    marker::PhantomData,
    io::{BufRead, BufReader},
};

use crate::{
    error::Error,
    interpreter::Grid,
};

use pipeline::Stage;

pub struct FileReader<P>(PhantomData<P>);

impl<P> FileReader<P> {
    pub fn new() -> Self {
        FileReader(PhantomData)
    }
}

impl<P> Stage<Error> for FileReader<P> where P: AsRef<Path> {
    type Input = P;
    type Output = Grid<char>;

    fn run(self, path: Self::Input) -> Result<Self::Output, Error> {
        let mut len = 0;

        File::open(path)
            .and_then(|file| BufReader::new(file).lines().collect::<Result<Vec<_>, _>>())
            .map(|strings| strings.into_iter()
                .map(|string| {
                    let vec = string.chars().collect::<Vec<_>>();
                    len = cmp::max(vec.len(), len);
                    vec
                })
                .collect::<Vec<_>>()
                .into_iter()
                .map(|mut vec| {
                    vec.resize(len, ' ');
                    vec
                })
                .collect())
            .map(Grid::new)
            .map_err(Error::IO)
    }
}
