pub fn get_input(prompt: Option<String>) -> String {
    use std::io;

    let mut inp = String::new();
    if prompt.is_some() {
        println!("{}", prompt.unwrap());
    };

    io::stdin()
        .read_line(&mut inp)
        .expect("Unable to get prompt");

    return inp;
}