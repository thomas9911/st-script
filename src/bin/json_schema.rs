use st_script::json_schema;

const HELP: &'static str = r#"Prints the json schema of the syntax tree / input.
This can be used to implement validation in another programming language.
cargo run --bin json_schema > schema.json"#;

fn main() {
    if std::env::args().any(|x: String| ["--help", "-h", "help", "h"].contains(&x.as_ref())) {
        println!("{}", HELP);
    } else {
        println!("{}", json_schema());
    }
}
