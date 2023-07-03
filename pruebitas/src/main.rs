fn main() -> () {
    fn encrypt_this(text: &str) -> String {
        let mut encrypted_sentence :Vec<String> = Vec::new();

        for word in text.split(" "){

        let mut encrypted = word.to_string();
        let fir_letter:String = String::from(&word[0..1]).chars().map(|x| (x as u8).to_string()).collect();
        if word.len() > 1 {
        let sec_letter = &word[1..2];
        let last_letter =  &word[word.len()-1..word.len()];
        encrypted.replace_range(1..2, last_letter);
        encrypted.replace_range(encrypted.len()-1..encrypted.len(), sec_letter);
        }
        encrypted.replace_range(0..1, &fir_letter);
        encrypted_sentence.push(encrypted);

    }
    println!("{}",  encrypted_sentence.join(" "));
    return encrypted_sentence.join(" ")
    }
    encrypt_this(&"A wise old owl lived in an oak");
}