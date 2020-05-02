use std::error::Error;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
struct MyError {
    message: String,
    file: String,
    line: u64,
    source: Option<Box<dyn Error>>,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(src) = &self.source {
            write!(f, "{}:{} {}: {}", self.file, self.line, self.message, src.as_ref())
        } else {
            write!(f, "{}:{} {}", self.file, self.line, self.message)
        }
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
    let result = std::fs::read_to_string(PathBuf::from("bad/path/fake/nonono"));
    // let boxed: Box<dyn std::error::Error> = result.err().unwrap().into();
    // let e = MyError {
    //     message: format!("Unable to open fake path '{}'", "bad/path/fake/nonono"),
    //     file: file!().to_string(),
    //     line: line!() as u64,
    //     source: Some(boxed),
    // };

    let x = wrap!(result.err().unwrap());
    // println!("{}", e);
    println!("{}", x);
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
#[macro_export]
macro_rules! wrap {
    // Base case:
    ($err:expr) => (MyError {
        message: "poo poo".to_string(),
        file: file!().to_string(),
        line: line!() as u64,
        source: Some($err.into()),
    });
        // let boxed: Box<dyn std::error::Error> = $err.into();
        // return MyError {
        //     message: format!("some message here"),
        //     file: file!().to_string(),
        //     line: line!() as u64,
        //     source: Some(boxed),
        // }
    // );
    // // `$x` followed by at least one `$y,`
    // ($x:expr, $($y:expr),+) => (
    //     // Call `find_min!` on the tail `$y`
    //     std::cmp::min($x, find_min!($($y),+))
    // )
}

// fn create_error<E>(source: E, message: &str, file: &str, line: u64) -> MyError
//     where E: std::error::Error, {
//     let boxed: Box<dyn std::error::Error> = source.into();
//     MyError {
//         message: message.to_string(),
//         file: file.to_string(),
//         line,
//         source: Some(boxed),
//     }
// }