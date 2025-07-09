fn main() {
    println!("{}", get_result())
}

fn get_result() -> String {
    let mut primes: [bool; 101] = [true; 101];
    primes[1] = false;
    let mut p = 2; 
    while p * p <= 100 {
        let mut i = p * p; 
        if primes[i] {
            while i <= 100 {
                primes[i] = false;
                i += p;
            }
        }
        p += 1;
    }
    let mut results: Vec<String> = vec![];
    let mut text = "".to_string();
    for i in (1..=100).rev() {
        text = "".to_string();
        if primes[i] {
            continue;
        } 
        if i % 3 == 0 {
            text += "Foo";
        }
        if i % 5 == 0 {
            text += "Bar";
        }
        if text.is_empty() {
            text += &i.to_string();
        }
        results.push(text);
    }
    results.join(", ")
}