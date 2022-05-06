fn digit_num(num: u32) -> u32 {
    let mut p = 1;
    let mut tmp_num = num;
    loop {
        tmp_num /= 10;
        if tmp_num == 0 {
            break;
        }
        p += 1;
    }
    p
}

pub fn is_armstrong_number(num: u32) -> bool {
    let p = digit_num(num);
    let mut sum = 0;
    let mut num_mut = num;

    loop {
        let m = num_mut % 10;
        sum += m.pow(p);
        num_mut /= 10;
        if num_mut == 0 {
            break;
        }
    }

    sum == num
}
