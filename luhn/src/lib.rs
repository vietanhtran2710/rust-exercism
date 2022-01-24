/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.len() < 2 {
        return false;
    }
    let mut payload: Vec<u32> = Vec::new();
    for ch in code.chars() {
        if ch != ' ' {
            if ch.is_ascii_digit() {
                payload.push(ch.to_digit(10).unwrap())
            }
            else {
                return false;
            }
        }
    }
    if payload.len() < 2 {
        return false;
    }
    let (mut sum, mut i) = (payload[payload.len() - 1], payload.len() - 2);
    loop {
        payload[i] *= 2;
        if payload[i] > 9 {
            payload[i] -= 9
        }
        sum += payload[i];
        if i > 0 {
            sum += payload[i - 1];    
        }
        if i <= 1 {
            break;
        }
        i -= 2;
    }
    println!("{}", sum);
    sum % 10 == 0
}
