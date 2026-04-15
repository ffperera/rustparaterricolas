fn info_marte() -> (String, i32) {
    (String::from("Marte"), 3390)
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Tuplas");
    println!("--------------------");
    println!();

    let planetas = ("Urano", "Neptuno", "Saturno");
    println!("Tupla con planetas: {:?}", planetas);

    let cosas = (27, 'ñ', String::from("space"));
    println!("Tupla con cosas: {:?}", cosas);

    let rgb: (i32, i32, i32) = (255, 255, 0);
    println!("Tupla rgb: {:?}", rgb);

    let mut planetas = ("Urano", "Neptuno", "Saturno");
    planetas.2 = "Mercurio";
    println!("{} {} {}", planetas.0, planetas.1, planetas.2);

    let coordenadas = (1.1, 2.2, 3.3);
    // desestructurar
    let (x, y, z) = coordenadas;

    println!("Tupla original: {:?}", coordenadas);
    println!("Variables: {} {} {} ", x, y, z);

    let (x, _, _) = coordenadas;
    println!("Componente x: {}", x);

    let marte = String::from("Marte");
    let venus = String::from("Venus");
    let jupiter = String::from("Jupiter");

    let planetas = (marte, venus, jupiter);
    println!("Planetas (ownership): {:?}", planetas);

    // la variable 'marge' se consumió en la
    // creación de la tupla
    // println!("Marte (ownership): {:?}", marte);

    // radio de los planetas, en km
    let marte = ("Marte", 3390);
    let venus = ("Venus", 6052);
    let jupiter = ("Jupiter", 69911);

    // tupla de tuplas
    let planetas = (marte, venus, jupiter);

    println!("Planetas: {:?}", planetas);
    println!("{} : {} km ", planetas.1.0, planetas.1.1);

    // let indice:usize = 1;
    // println!("{:?}", planetas[indice]);
    // println!("{:?}", planetas.indice);

    let (nombre, radio) = info_marte();
    println!("{} : {} km ", nombre, radio);

    fn print(color: (i32, i32, i32)) {
        println!("RGB: {}, {}, {}", color.0, color.1, color.2);
    }

    let rojo = (255, 0, 0);
    let verde = (0, 255, 0);
    let azul = (0, 0, 255);
    print(rojo);
    print(verde);
    print(azul);
}
