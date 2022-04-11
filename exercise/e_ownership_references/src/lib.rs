pub fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("Plural");
    } else {
        println!("Singular");
    }
}

pub fn change(s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s")
    }
}

pub fn eat(s: String) -> bool {
    let val: bool;
    if s.starts_with("b") && s.contains("a") {
        val = true;
    } else {
        val = false;
    }
    return val;
}

pub fn add(x: &i32, y: &i32) -> i32 {
    return (*x) + (*y);
}