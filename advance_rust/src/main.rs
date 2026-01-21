mod generic;
mod hashmaps;
mod iterators;
mod strings;
mod struct_lifetime;
mod traits;
mod vectors;
fn main() {
    let vec = vec![10, 13, 18, 17, 20, 23, 27, 29, 30, 40, 50];
    let new_vec = vectors::change_vec(vec);
    println!("{:?}", new_vec);
    let hm_vec: Vec<(String, u32)> = vec![
        ("Harkirat".to_string(), 21),
        ("Ram".to_string(), 20),
        ("Krishna".to_string(), 30),
    ];
    let hm = hashmaps::vec_to_hm(hm_vec);
    println!("Converted HashMap : {:?} ", hm);
    iterators::run_iterator();
    println!("Here is the execution of String File : ");
    strings::print_string();
    println!("Here is the execution of Generics File : ");
    generic::print_generic();
    println!("Here is the execution of Trait File : ");
    traits::print_trait();
    println!("Here is the execution of Lifetime File : ");
    lifetime::print_lifetime();
    struct_lifetime::print_struct_lifetime();
}

// The Definition
struct User<'a> {
    // 1. We have a generic lifetime 'a
    // 2. We attach it to the reference
    username: &'a str,
}

// The Translation:
// "Any instance of 'User' cannot outlive the
// string data stored in 'username'."
