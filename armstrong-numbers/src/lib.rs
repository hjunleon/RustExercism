pub fn get_num_digits(num:u32)->u32 {
    let mut t_num = num;
    let mut to_ret: u32 = 0;
    while t_num > 0{
        t_num /= 10;
        to_ret += 1;
    }
    to_ret
}
pub fn get_armstrong(num: u32, num_of_digits: u32) -> u32 {
    let mut t_num = num;
    let mut to_ret: u32 = 0;
    while t_num > 0{
        let digit = t_num % 10;
        to_ret += digit.pow(num_of_digits);
        t_num /= 10;
    }
    to_ret
}
pub fn is_armstrong_number(num: u32) -> bool {
    let num_of_digits: u32 = get_num_digits(num);

    let expected: u32 = get_armstrong(num, num_of_digits);

    if num == expected{
        return true;
    }
    false
    // unimplemented!("true if {} is an armstrong number", num)
}
