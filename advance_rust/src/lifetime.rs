pub fn print_lifetime() {
    let str1 = String::from("Small");
    let long;
    {
        let str2 = String::from("Longest");
        long = longest(&str1, &str2);
    }
    println!("The longest string is : {}", long)
}
fn longest<'a>(str1: &'a String, str2: &'a String) -> &'a String {
    if str1.len() > str2.len() { str1 } else { str2 }
}
