use std::io;
fn main(){
    loop{
        let mut input_str: String = String::new();
        println!("Enter a word: ");
        io::stdin().read_line(&mut input_str).expect("ERROR READING INPUT");
        let string_in_pig_latin= convert_to_pig_latin(&input_str);
        println!("pig latin => {}",string_in_pig_latin);
    }
}
fn convert_to_pig_latin(string: &String)->String{
    let inner_string: String = string.clone();
    let mut str_as_vector: Vec<char> = Vec::new();
    for c in inner_string.chars(){
        str_as_vector.push(c);
    }
    str_as_vector.pop();
    let _vowels: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
    let startswith_vowel: bool = check_first_letter(&_vowels, &str_as_vector[0]);
    let mut result: String = String::new();
    if startswith_vowel {
        for chars in "-hay".chars(){
            str_as_vector.push(chars);
        }
        for elem in str_as_vector.iter(){
            result.push(*elem);
        }
    }
    else{
        let mut new_vec: Vec<char> = Vec::new();
        for i in 1..str_as_vector.len(){
            new_vec.push(str_as_vector[i]);
        }
        new_vec.push('-');
        new_vec.push(str_as_vector[0]);
        for elem in "ay".chars(){
            new_vec.push(elem);
        }
        for elem in new_vec.iter(){
            result.push(*elem);
        }
    }
    result
}
fn check_first_letter(vec: &Vec<char>, character: &char)->bool{
    for elem in vec{
        if elem == character{
            return true;
        }
    }
    false
}