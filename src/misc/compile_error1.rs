struct SomeValue {
    active: bool
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
    SomeValue(SomeValue)
}

fn main() {

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::SomeValue(SomeValue { active: true }),
    ];

    for i in &row {
        match i {
            SpreadsheetCell::Int(v) |
            SpreadsheetCell::Float(v) |
            SpreadsheetCell::Text(v) => println!("{v}"),
            SpreadsheetCell::SomeValue(v) => {
                let v = v.active;
                println!("{v}");
            },
        }
    }
}