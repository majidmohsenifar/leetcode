fn main() {
    let string = String::from("DCXXI");
    let result = roman_to_int(string);
    println!("result: {}", result)
}

pub fn roman_to_int(s: String) -> i32 {
    let map = std::collections::HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let mut result: i32 = 0;
    let mut last_char = ' ';
    for ch in s.chars() {
        let added = map.get(&ch).unwrap();
        result += added;
        if ch != 'I' && last_char != ' ' && added > map.get(&last_char).unwrap() {
            match last_char {
                'I' | 'X' | 'C' => {
                    result -= 2 * map.get(&last_char).unwrap();
                }
                _ => {}
            }
        }
        last_char = ch;
    }
    result
}
