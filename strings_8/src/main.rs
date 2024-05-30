fn main() {
    // let mut s = String::new();
    /*
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    */

    // let s = String::from("initial contents");
    /*
       let uno = String::from("Hola mundo");
       let dos = String::from("dos mundo");
       let union_final = uno + " " + &dos;
       println!("{union_final}, {dos}")
    */
    /*
       let uno = String::from("Hola mundo");
       let dos = String::from("dos mundo");
       // no hace falta hacer referencia
       let union_final = format!("{uno} {dos}");
       println!("{union_final}, {dos} , {uno}")
    */
    /*
    let mut s = String::from("Hola");
    s.push_str(", mundo!");
    println!("{}", s);
    */
    /*
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    */
    /* 
    let mut s = String::from("lo");
    s.push('l');
    println!("{s}") 
    */
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }

}
