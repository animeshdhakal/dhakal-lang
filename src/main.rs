use dhakal_lang::lexer::Lexer;

fn main() {
    let s = r#"
        let hello = 12;
    "#;

    let mut l = Lexer::new(s.to_string());

    for i in 0..7 {
        println!("{:?}", l.next_token());
    }

    println!("Hello, world!");
}
