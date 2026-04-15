use std::mem::size_of;

#[derive(Debug)]
pub enum PlanetaEnum {
    Mercurio,
    Venus,
    Tierra,
    Marte,
    Jupiter,
    Saturno,
    Urano,
    Neptuno,
}

#[derive(Debug)]
pub enum Planeta {
    Venus(i32),
    Marte(String),
    Jupiter(String, i32),
    Otro,
}

pub fn expresion_match(p: &Planeta) {
    match p {
        Planeta::Venus(radio) => println!("Venus, radio: {} km", radio),
        Planeta::Marte(nombre) => println!("Planeta {}", nombre),
        Planeta::Jupiter(n, r) => println!("{} : {} km", n, r),
        Planeta::Otro => println!("Otro planeta"),
    };
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Enums");
    println!("--------------------");
    println!();

    let marte = PlanetaEnum::Marte;
    let urano = PlanetaEnum::Urano;

    let p3 = PlanetaEnum::Jupiter;
    let p4 = PlanetaEnum::Neptuno;
    let p5 = PlanetaEnum::Tierra;
    let p6 = PlanetaEnum::Mercurio;
    let p7 = PlanetaEnum::Saturno;
    let p8 = PlanetaEnum::Venus;

    println!("Enum simple {:?} {:?}", marte, urano);
    println!("{:?}  {:?} {:?} {:?} {:?} {:?}", p3, p4, p5, p6, p7, p8);

    let p1 = Planeta::Marte(String::from("Marte"));
    let p2 = Planeta::Venus(6052);
    let p3 = Planeta::Jupiter(String::from("Jupiter"), 69911);
    let p4 = Planeta::Otro;

    println!("Variante {:?}", &p1);
    println!("Variante {:?}", &p2);
    println!("Variante {:?}", &p3);
    println!("Variante {:?}", &p4);

    expresion_match(&p1);
    expresion_match(&p2);
    expresion_match(&p3);
    expresion_match(&p4);

    // println!("Variante {:?}", &p4);
    println!("Tamaño Planeta (bytes) {:?}", size_of::<Planeta>());
    println!("Tamaño String (bytes) {:?}", size_of::<String>());
    println!("Tamaño i32 (bytes) {:?}", size_of::<i32>());
}
