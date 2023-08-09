
use std::collections::HashMap;


// Cut a string into chunks of size c (ignore the last chunk if its size is less than c).
// If a chunk represents an integer such as the sum of the cubes of its digits is divisible by 2, 
// reverse that chunk; otherwise rotate it to the left by one position.
// Put together these modified chunks and return the result as a string.
pub fn revrot(s: &str, c: usize) -> String {
    if (s.is_empty()) | (c <= 0 )| (c > s.len()){
        return String::from("")
    }
    
    // String to return 
    let mut result = String::new();
    // First character
    let mut first_char = 0;
    // Last character
    let mut last_char = c;
    
    for _chunk in 1..=(s.len()/c) {
        let mut substring = (&s[first_char..last_char]).to_owned();
        let substring_sum:u32 = substring.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>().iter().sum();
        if (substring_sum % 2) == 0{
            substring = substring.chars().rev().collect::<String>();
        }
        else {
            substring = substring[1..].to_owned() + &substring[0..1];
        }
        result += &substring;
        first_char += c;
        last_char += c;
    }
    return result
}

// Could you make a program that makes a string uppercase
// gives it sorted in alphabetical order by last name.
pub fn meeting(s: &str) -> String {
    let names = s.split(";");
    let mut list_names : Vec<String>= Vec::new();
    for name in names {
        let name_ordered:Vec<String> = name.split(":").map(|x| x.to_uppercase()).collect();
        let full_name = name_ordered.into_iter().rev().collect::<Vec<String>>().join(", ");
        list_names.push(full_name);
    }
    list_names.sort_unstable();
    let names_paren:String = list_names.iter().map(|x| "(".to_owned() + x + ")").collect();
    return names_paren
}

// The main idea is to count all the occurring characters in a string. 
// If you have a string like aba, then the result should be {'a': 2, 'b': 1}.
pub fn count(input: &str) -> HashMap<char, i32> {
    
    let mut dict:HashMap<char, i32> = HashMap::new();
    for letter in input.chars(){
        if !(dict.contains_key(&letter)){
            dict.insert(letter, 1);
        }
        else {
            if let Some(x) = dict.get_mut(&letter){
                *x += 1;
            }
        }
    }
    return dict;
}

// Create table lenXlen
pub fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
    
    let mut table:Vec<Vec<usize>> = Vec::new();
    for i in 1..=len{
        let row: Vec<usize> = (1..=len).map(|x| x*i).collect();
        table.push(row);
    }
    return table
}

// m is a volume i have to return n  
// where  n3 + (n-1)3 ... = m
pub fn find_nb(m: u64) -> i32 {
    
    let mut cubes:u64 = 0;
    let mut volume:u64 = 0;
    loop {
        cubes+=1;
        volume += cubes.pow(3);
        if volume > m  {
        return -1;
        }
        else if volume == m {
        return cubes as i32
        }
    }
}