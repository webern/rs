use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError {
    message: String,
    file: String,
    line: u64,
    source: Option<Box<dyn Error>>,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Some(src) = &self.source {
            return Some(src.as_ref());
        }
        None
    }
}

fn main() {
    println!("hello");
    // yo!("poo");
    // yall!("poo", "foo", "loo");
}

// #[macro_export]
// macro_rules! yo {
//   ($name:expr) => {
//      println!("Yo {}!", $name)
//   };
// }
//
//
// #[macro_export]
// macro_rules! yall {
//   (($name:expr),*) => {
//      println!("Yo {}!", $name)
//   };
// }
