pub fn change_vec(vec: Vec<u16>) -> Vec<u16> {
    let mut new_vec: Vec<u16> = Vec::new();
    for i in 0..vec.len() {
        if vec[i] % 2 == 0 {
            new_vec.push(vec[i]);
        }
    }
    new_vec
}
