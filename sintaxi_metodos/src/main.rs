/*
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/


/* 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    otro: u32,
}
// struct + impl y después para usarlo necesitamos una variable, para reutilizar codigo, clases python?
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height + self.otro
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
        otro: 8,
    };

    let medidas_pantalla = Rectangle {
        width: 1920,
        height: 1080,
        otro: 0,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Pixeles en pantalla {}", medidas_pantalla.area());
    // si queremos traer otras partes de medidas_pantalla debemos usar &self para poder usar mas datos sino se destrullen los demas (ownership)
    println!("Ancho {}", medidas_pantalla.width);
    println!("Alto {}", medidas_pantalla.height);
}

*/



/*
#[derive(Debug)]
struct Rectangle {
    width: u32,
    _height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        _height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

*/

/*
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, otro_rec: &Rectangle) -> bool {
        self.width > otro_rec.width && self.height > otro_rec.height
    }
    
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

*/
/* 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3);

    println!("Medidas cuadrado {} y {}", sq.width, sq.height)
}
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(x: u32, y: u32) -> Self {
        Self {
            width: x,
            height: y,
        }
    }
}

fn main() {
    let sq = Rectangle::square(5, 7);

    println!("Medidas cuadrado {} y {}", sq.width, sq.height)
}