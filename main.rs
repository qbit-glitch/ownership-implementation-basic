fn main() {
    let str1 = String::from("Rust Course");
    let str2 = String::from(" by Rise-in.");
    let result = concatenate_strings(&str1, &str2);
    println!("Concatenated String : {}", result);
}

fn concatenate_strings(s1: &String, s2: &String) -> String{
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result    
}
