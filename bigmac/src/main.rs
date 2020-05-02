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

    let original_error = result.err().unwrap();
    let raised_err = raise!("foo is a {}", "bishop");

    let x = wrap!(raised_err, "my message {} {}", "and poo", "and foo");
    // println!("{}", e);
    println!("{}", x);
    // yo!("poo");
    // yall!("poo", "foo", "loo");
}

#[macro_export]
macro_rules! wrap {
    // Base case:
    ($err:expr) => (MyError {
        message: "an error occurred".to_string(),
        file: file!().to_string(),
        line: line!() as u64,
        source: Some($err.into()),
    });
    ($err:expr, $msg:expr) => (MyError {
        message: $msg.to_string(),
        file: file!().to_string(),
        line: line!() as u64,
        source: Some($err.into()),
    });
    ($err:expr, $fmt:expr, $($arg:expr),+) => (MyError {
        message: format!($fmt, $($arg),+),
        file: file!().to_string(),
        line: line!() as u64,
        source: Some($err.into()),
    });
}

#[macro_export]
macro_rules! raise {
    // Base case:
    ($msg:expr) => (MyError {
        message: msg.to_string(),
        file: file!().to_string(),
        line: line!() as u64,
        source: None,
    });
    ($fmt:expr, $($arg:expr),+) => (MyError {
        message: format!($fmt, $($arg),+),
        file: file!().to_string(),
        line: line!() as u64,
        source: None,
    });
}
