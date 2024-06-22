macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.into()),*]);
}

fn main() {
    println!("Hello, world!");
    println!("the list is: {:?}", listOfUsers());
}

fn listOfUsers() -> Vec<String> {
    return vec_of_strings!["mohamed", "ahmed", "Khalid"];
}
