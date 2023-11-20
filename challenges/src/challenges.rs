
use std::collections::HashMap;

//In number theory and combinatorics, a partition of a positive integer n, 
//also called an integer partition, is a way of writing n as a sum of positive integers. 
//Two sums that differ only in the order of their summands are considered the same partition.
//For example, 4 can be partitioned in five distinct ways:
// 4, 3 + 1, 2 + 2, 2 + 1 + 1, 1 + 1 + 1 + 1.
pub fn part(n: i64) -> String {
  todo!()
}

// From an n x n array, return the snail path in clockwise
pub fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    let mut snail_path:Vec<i32> = Vec::new();
    if (matrix.is_empty()) || (matrix[0].len() < 1) {
        return snail_path
    }
    let mut start_row = 0;
    let mut start_col = 0;
    let mut end_row = matrix.len() - 1;
    let mut end_col = matrix[0].len() - 1;
    let mut laps = (matrix.len() as f32 / 2.0).ceil() as u8;
    while laps > 0{
        // top
        let top = &matrix[start_row][start_col..=end_col];
        snail_path.extend_from_slice(&top);
        // if it's odd len let the top to take the last row
        if matrix.len() % 2 != 0 && laps == 1{
            break;
        }
        // right side
        let right:Vec<i32> = (&matrix[start_row+1..=end_row-1]).to_owned().into_iter().map(|x| x[end_col]).collect();
        snail_path.extend_from_slice(&right);
        // bottom
        let bottom:Vec<i32> = (&matrix[end_row][start_col..=end_col]).to_owned().into_iter().rev().collect();
        snail_path.extend_from_slice(&bottom);
        // left side
        let left:Vec<i32> = (&matrix[start_row+1..=end_row-1]).to_owned().into_iter().map(|x| x[start_col]).rev().collect();
        snail_path.extend_from_slice(&left);

        // update vars
        start_row += 1;
        start_col += 1;
        end_row = if end_row == 0 {0} else {end_row-1};
        end_col = if end_col == 0 {0} else {end_col-1};
        laps-=1
    }
    return snail_path
} 

// From a range of number collect all the number that breaking them down into their digits 
// and powering each digit to its correspond position in the integer give the same number.
pub fn sum_dig_pow(a: u64, b: u64) -> Vec<u64>  {
    let mut result:Vec<u64> = Vec::new();
    for integer in a..=b{
        let digits = integer.to_string();
        let sum_digits:u64 = digits.chars().enumerate().map(|(k,v)| ((v.to_digit(10).unwrap() as u64).pow((k+1) as u32))).sum::<u64>() as u64;
        if sum_digits == integer{
            result.push(sum_digits);
        }
    };
    return result
} 

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