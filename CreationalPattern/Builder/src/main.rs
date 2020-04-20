struct Builder {
    fn test(&self) => String {
        return String::from("blah")
    }
}

impl Builder {
    fn build(&self) -> &String {
        let return_val = String::from("test");
        return return_val;
    }
}

fn main() {
    let instance = Builder {};

    println!("{}", instance.build());
}
