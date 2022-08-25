use std::str::{self, from_utf8};
pub fn is_mine(c: &u8) -> bool {
    *c == b'*'
}

pub fn print_matrix(matrix: &Vec<String>) {
    for s in matrix {
        println!("{s}");
    }
}

pub fn print_matrix_str(matrix: &[&str]) {
    for s in matrix {
        println!("{s}");
    }
}

pub fn count_mines(minefield: &[&str], cur_row: &[u8],  j: usize, i:usize, row_len: usize, col_len: usize)-> u8 {
    let mut cnt: u8 = 0;
    if j > 0 && is_mine(&cur_row[j - 1]) {
        cnt += 1;
    }
    if j < row_len - 1 && is_mine(&cur_row[j + 1]) {
        cnt += 1;
    }
    if i > 0 {
        let top_row = minefield[i - 1].as_bytes();
        let top_char = top_row[j];
        if is_mine(&top_char) {
            cnt += 1;
        }
        if j > 0 && is_mine(&top_row[j-1]){
            // if {
            cnt += 1;
            // }
        }
        if j < row_len - 1 && is_mine(&top_row[j + 1]) {
            cnt += 1;
        }   
    }
    if i < col_len - 1 {
        let bottom_row = minefield[i + 1].as_bytes();
        let bottom_char = bottom_row[j];
        if is_mine(&bottom_char) {
            cnt += 1;
        }
        if j > 0 && is_mine(&bottom_row[j-1]){
            cnt += 1;
        }
        if j < row_len - 1 && is_mine(&bottom_row[j + 1]) {
            cnt += 1;
        }   
    }
    cnt
    
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    println!("INPUTTTTTT");
    print_matrix_str(minefield);
    let col_len = minefield.len();
    let mut to_ret: Vec<String> = Vec::new();
    for i in 0..col_len {
        // to_ret.push(String::new());
        let cur_row = minefield[i].as_bytes(); //.clone().as_mut();   //: &mut[u8]
        let mut temp_row = vec![];
        let row_len: usize = cur_row.len();
        for j in 0..row_len {
            if !is_mine(&cur_row[j]) {
                let cnt: u8 = count_mines(minefield,cur_row,j,i,row_len,col_len);
                if cnt > 0 {
                    // minefield[i].char_indices().nth(j) = cnt;
                    // cur_row[j] = cnt + 0x30;
                    temp_row.push(cnt + 0x30);
                } else {
                    temp_row.push(cur_row[j]);
                }
            } else {
                temp_row.push(cur_row[j]);
            }
        }
        let row_str = from_utf8(temp_row.as_slice()).unwrap().to_string();
        println!("row_str: {row_str}");
        to_ret.push(row_str);
        // minefield[i] = from_utf8(cur_row).unwrap();
    }
    print_matrix(&to_ret);
    to_ret
    // minefield.to_vec().into_iter().map(|x| x.to_string()).collect()
    // unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{:#?}\n", minefield);
}
