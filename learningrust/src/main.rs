struct Data {
    data: String,
}

impl Drop for Data {
    fn drop(&mut self) {
        println!("Dropping Data: {}", self.data);
    }
}

fn main() {
    let x = Data {
        data: String::from("hello!"),
    };

    let y = Data {
        data: String::from("world!"),
    };

    drop(x);

    println!("Exiting main()");
}