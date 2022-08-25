use std::collections::HashMap;
// use int_enum::IntEnum;

// pub enum NT {
//     A: 0,
//     T: 1,
//     C: 2,
//     G: 3,
// }

pub fn is_invalid_NT(c: char) -> bool{
    c != 'A' && c != 'T' && c != 'C' && c != 'G'  
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if is_invalid_NT(nucleotide){
        return Err(nucleotide);
    }
    let mut c_cnt:usize = 0;
    for c in dna.chars(){
        if is_invalid_NT(c){
            return Err(c);
        }
        if c == nucleotide{
            c_cnt += 1;
        }
    }
    Ok(c_cnt)
    // unimplemented!(
    //     "How much of nucleotide type '{}' is contained inside DNA string '{}'?",
    //     nucleotide,
    //     dna
    // );
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut cnt = HashMap::new();
    cnt.insert('A',0);
    cnt.insert('T',0);
    cnt.insert('C',0);
    cnt.insert('G',0);
    for c in dna.chars(){
        println!("c: {c}");
        if is_invalid_NT(c){
            return Err(c);
        }
        *cnt.entry(c).or_insert(0)+= 1;  //.or_insert(0)
    }
    Ok(cnt)
    // unimplemented!(
    //     "How much of every nucleotide type is contained inside DNA string '{}'?",
    //     dna
    // );
}
