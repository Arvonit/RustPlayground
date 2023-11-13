use std::{
    fs::{self, File},
    io::{Error, ErrorKind, Read},
};

pub fn run() {
    // This code causes a panic because the vector is only 3 elements long
    // let v = vec![1, 2, 3];
    // v[99];

    // Open a file or create it if it doesn't already exist
    let file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // If the file does not exist, create it
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // A simpler version with `unwrap_or_else()`
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // If we want to panic if there is an error, we can use the `unwrap()` method
    let file = File::open("hello.txt").unwrap();

    // Another method, `expect()`, lets use specify a panic error message
    let file = File::open("hello.txt").expect("Failed to open hello.txt");
}

// Sometimes we should propogate any errors that could happen inside a function
// to the caller. To do that, we should return a `Result` type.
fn read_username_from_file() -> Result<String, Error> {
    // Long version
    // let f = File::open("hello.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // Shorter version with ? operator
    // let mut f = File::open("hello.txt")?; // The ? operator causes `open` to return
    //                                       // if there is an error
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // We can also chain the ? operator
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    // We can make this even shorter by using the `read_to_string` function in fs
    fs::read_to_string("hello.txt")
}
