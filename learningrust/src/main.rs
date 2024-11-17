use rocket::{fs::FileServer, response::Redirect, uri};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct ErrorResponser {
    message: String,
    error_code: u16,
}


fn main() {
    let err = ErrorResponser {message: String::from("Hello error!"), error_code: 200};

    println!("{:?}", err);
}