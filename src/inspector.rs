use pipeline::Stage;

use crate::error::Error;

use std::{
    path::PathBuf,
    marker::PhantomData,
};

pub struct Inspector<T> {
    enabled: bool,
    _phantom: PhantomData<T>
}

impl<T> Inspector<T> {
    pub fn new(enabled: bool) -> Inspector<T> {
        Inspector {
            enabled,
            _phantom: PhantomData
        }
    }
}

pub trait Inspectable {
    fn inspect(&self);
}

impl Inspectable for PathBuf {
    fn inspect(&self) {
        println!("{:#?}", self);
    }
}

impl<T> Stage<Error> for Inspector<T> where T: Inspectable {
    type Input = T;
    type Output = T;

    fn run(self, input: Self::Input) -> Result<Self::Output, Error> {
        if self.enabled
        {
            input.inspect();
        }

        Ok(input)
    }
}