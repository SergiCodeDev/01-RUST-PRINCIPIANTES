fn main() {
    /*     use std::collections::HashMap;

       let mut scores = HashMap::new();

       scores.insert(String::from("Blue"), 10);
       scores.insert(String::from("Yellow"), 50);

       let team_name = String::from("Blue");
       // .unwrap_or(0); devuelve el valor si existe y el (0) es un valor por defecto si no devuelve nada
       let score = scores.get(&team_name).copied().unwrap_or(0);
    */
    /*
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
     */
    /*
       // Reemplazando un valor
       use std::collections::HashMap;

       let mut scores = HashMap::new();

       scores.insert(String::from("Blue"), 10);
       scores.insert(String::from("Blue"), 25);

       println!("{:?}", scores);
    */
    // Insertando una clave y un valor solo si una clave no está presente
    /*     use std::collections::HashMap;

       let mut scores = HashMap::new();
       scores.insert(String::from("Blue"), 10);

       scores.entry(String::from("Yellow")).or_insert(50);
       scores.entry(String::from("Blue")).or_insert(50);

       println!("{:?}", scores);
    */
    // Actualizando un valor basado en el valor anterior

    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    // El método split_whitespace devuelve un iterator sobre sub-slices, separados por espacios en blanco
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
