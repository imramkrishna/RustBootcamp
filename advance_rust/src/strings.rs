pub fn print_string() {
    let name = String::from("Ram Krishna Yadav");
    let word2 = get_first_word(&name);
    println!("The Name Word : {} ", name);
    println!("The First Word : {} ", word2)
}

//Here this function returs a string slices which prevents creation of new string in heap memory and also works if the string changes for that changed string .THis prevents the issue of dangling pointers. Thats why &str commonly known as String slices was used in this code.

// Rust automatically converts &String to &str when needed. This is called "deref coercion".
fn get_first_word(word: &str) -> &str {
    let mut index_count = 0;
    for i in word.chars() {
        if i == ' ' {
            break;
        }
        index_count += 1;
    }
    &word[0..index_count]
}
