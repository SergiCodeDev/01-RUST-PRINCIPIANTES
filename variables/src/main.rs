fn main() {
    // escribir mut para que la variable pueda cambiar sino sera como si fuera const
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // contantes
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of x is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let s = 5;
    println!("The value of x is: {s}");

    let s = s + 1;

    {
        let s = s * 2;
        println!("The value of x in the inner scope is: {s}");
    }

    println!("The value of x is: {s}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("Numero de espacios {spaces}");

    //error
    /*
    let mut spaces = "   ";
    spaces = spaces.len();
     */

    // tipo de datos
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Numero {guess}");

    // Tipos de Enteros + y -
    /*
    Tamaño	Signed	Unsigned
    8-bit	i8	    u8
    16-bit	i16	    u16
    32-bit	i32	    u32
    64-bit	i64	    u64
    128-bit	i128	u128
    arch	isize	usize
     */

    //Tipos de punto flotante (decimales)
    let j = 2.0; // f64

    let h: f32 = 3.0; // f32

    println!("Numero {j} + {h}");

    // Operaciones numéricas
    /*
       // addition
       let sum = 5 + 10;

       // subtraction
       let difference = 95.5 - 4.3;

       // multiplication
       let product = 4 * 30;

       // division
       let quotient = 56.7 / 32.2;
       let truncated = -5 / 3; // Results in -1

       // remainder
       let remainder = 43 % 5;
    */

    // El tipo booleano

    /*
    let t = true;

    let f: bool = false; // with explicit type annotation
     */

    // El tipo de carácter

    /*
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
     */

    // Tipos compuestos
    // tuplas una vez declaradas, su tamaño no puede aumentar ni disminuir
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    /*
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    // aceder a la posicion con un . y numero de posicion
    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
     */
    //El Tipo Arreglo A diferencia de una tupla, cada elemento de un arreglo debe tener el mismo tipo.
    /*
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // esto es let a = [3, 3, 3, 3, 3]; valor / veces
    let a = [3; 5];

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    */
    /* 
    use std::io; fuera de la funcion
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    */
}
