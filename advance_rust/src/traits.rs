trait Summary {
    fn summarize(&self) -> String;
}
struct User {
    name: String,
    age: i32,
}
impl Summary for User {
    fn summarize(&self) -> String {
        format!("User is {} and is {} years old", &self.name, &self.age)
    }
}
pub fn print_trait() {
    let user = User {
        name: "Harkirat".to_string(),
        age: 32,
    };
    println!("{}", user.summarize());
}
