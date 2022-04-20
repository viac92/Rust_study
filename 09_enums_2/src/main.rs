fn count_sheep(n: u32) -> String {
    let mut string_out: String = String::new();
    
    if n == 0 {
        string_out.to_string()
    } else {
        string_out = "1 sheep...".to_string();
        for i in 1..n {
            let string_add = format!("{} sheep...", i + 1);
            string_out = format!("{} {}", string_out, string_add);
        }
        string_out
    }
}
fn main() {
    println!("{}", count_sheep(0));
    println!("{}", count_sheep(2));
    println!("{}", count_sheep(4));

    assert_eq!(count_sheep(1), "1 sheep...");
}
