// 1. Define the struct OUTSIDE the function (standard practice)
struct User<'a> {
    username: &'a str,
    email: &'a str,
}

pub fn print_struct_lifetime() {
    // 2. Create OWNED data (Lives on the heap, not static)
    let name_owner = String::from("Ram Krishna");
    let email_owner = String::from("email@gmail.com");

    // 3. Create the struct holding REFERENCES to that data
    let user = User {
        username: &name_owner, // 'user' borrows from 'name_owner'
        email: &email_owner,   // 'user' borrows from 'email_owner'
    };

    println!("The user is: {}", user.username);

    // This example is "Good" because if you dropped 'name_owner' here,
    // and tried to use 'user' afterwards, the compiler would stop you.
}
