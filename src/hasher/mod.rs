use keccak_hash;



pub fn keccak_256(word: &str) -> String {
    let mut result = [0u8; 32];
    let bytes = word.as_bytes();

    keccak_hash::keccak_256(bytes,&mut result);

    let hash = result.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    return hash;
}


pub fn hash_words(words: Vec<&str>) -> Vec<String> {
    let mut hashes = Vec::new();
    for word in words {
        let hash = keccak_256(word);
        hashes.push(hash);
    }
    return hashes;
}

pub fn hash_combination(left: &String, right: &String) -> String {
    let formatted = &format!("{}{}", left, right);
    return keccak_256(formatted);
} 
