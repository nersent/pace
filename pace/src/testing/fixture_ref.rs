use std::io::Error;

pub trait Fixture {
    fn get_name(&self) -> String;
}

pub trait FixtureComparator {
    fn compare(&self, other: &dyn Fixture) -> bool;
}

pub trait FixtureLoader {
    fn load(path: &str) -> Result<Box<dyn Fixture>, Error>;
}
