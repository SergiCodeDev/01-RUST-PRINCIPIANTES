fn main() {
    // let v: Vec<i32> = Vec::new();

    // let v = vec![1, 2, 3];

    /*
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    */
    /*
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // aqui podemos tener control sobre los errores (si no existe)
    let third: Option<&i32> = v.get(5);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    */
    /*
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    */
    /*
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    */
    /*
    let mut v = vec![100, 32, 57];
    let mut i = 0;
    while i != v.len() {
        if v[i] == 100 {
            // Si el elemento es igual a 100
            v.remove(i);
        } else {
            i += 1;
        }
    }
    for valor in v {
        println!("{valor}")
    } 
    */
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    for valor in row {
        match valor {
            // i,t y f podrian ser solo x porque son variables
            SpreadsheetCell::Int(i) => println!("Integer: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(t) => println!("Text: {}", t)
        }
    }

}
