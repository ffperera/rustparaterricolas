// use crate::caps::c086_match::Planeta::*;

// use std::env::temp_dir;

#[derive(Debug)]
pub enum Planeta {
    Venus(i32),
    Marte(String),
    Jupiter(String, i32),
    Mercurio(i32),
    Tierra {
        nombre: String,
        radio: i32,
        temperatura: f64,
    },
    Planetoide,
    Asteroide,
}

pub fn expresion_match(p: &Planeta) {
    // para no escribir Planeta::
    use Planeta::*;
    match p {
        Venus(radio) | Mercurio(radio) => println!("Venus, radio: {} km", radio),
        Marte(nombre) => println!("Planeta {}", nombre),
        Jupiter(n, r) => println!("{} : {} km", n, r),
        Tierra {
            radio,
            nombre,
            temperatura,
        } => println!(
            "Tierra: {}, radio: {} km, temperatura: {} °C",
            nombre, radio, temperatura
        ),
        Planetoide | Asteroide => println!("Otro planeta"),
    };
}

pub fn radio_estimado(p: &Planeta) -> i32 {
    // para no escribir Planeta::
    use Planeta::*;
    match p {
        // como recibimos una referencia,
        // devolvemos el valor mediante desreferencia
        Venus(radio) | Mercurio(radio) => *radio,
        Marte(_) => 3389,
        Jupiter(_, r) => *r,
        Tierra { radio, .. } => *radio,
        Planetoide | Asteroide => 0,
    }
    // ^- no termina en ;
    // es una expresión, que devuelve i32
    // por lo tanto la función devuelve i32
}

// rama por defecto
pub fn expresion_match2(p: &Planeta) {
    // para no escribir Planeta::
    use Planeta::*;
    match p {
        Venus(radio) | Mercurio(radio) => println!("Venus, radio: {} km", radio),
        Marte(nombre) => println!("Planeta {}", nombre),
        Jupiter(n, r) => println!("{} : {} km", n, r),
        Tierra { nombre, .. } => println!("{}", nombre),
        n => println!("Otro planeta {:?}", n),
    };
}

/*
pub enum Planeta {
    Venus(i32),
    Marte(String),
    Jupiter(String, i32),
    Tierra,
    Otro,
}

pub fn expresion_match(p: &Planeta) {
    match p {
        Venus(radio) => println!("Venus, radio: {} km", radio),
        Marte(nombre) => println!("Planeta {}", nombre),
        Jupiter(n, r) => println!("{} : {} km", n, r),
        Tierra | Otro => println!("Otro planeta"),
    };
}
*/

fn es_planetoide(p: &Planeta) {
    match p {
        Planeta::Planetoide => println!("Es planetoide"),
        _ => println!("No es un planetoide"),
    };
}

fn es_planeta(p: &Planeta) {
    use Planeta::*;

    match p {
        Planetoide | Asteroide => println!("No es planeta"),
        _ => println!("Es un planeta"),
    };
}

fn es_venus(p: &Planeta) {
    use Planeta::*;

    match p {
        Venus(radio @ 6000..) => println!("Es Venus, y su radio es: {} km", radio),
        Venus(_) => println!("Parece un mini-Venus"),
        _ => println!("No es Venus"),
    };
}

fn es_tierra(p: &Planeta) {
    use Planeta::*;

    match p {
        Tierra {
            radio,
            nombre,
            temperatura,
        } => println!(
            "Tierra: {}, radio: {} km, temperatura: {} °C",
            nombre, radio, temperatura
        ),
        _ => println!("No es Tierra"),
    };
}

fn tipo_de_tierra(p: &Planeta) {
    use Planeta::*;

    match p {
        Tierra {
            radio: r @ 9000..,
            nombre,
            temperatura: _,
        } => println!("Supertierra: {}, radio: {} km", nombre, r),
        Tierra {
            radio: _,
            nombre,
            temperatura: t @ 120.0..,
        } => println!("Tierra caliente: {}, temperatura: {} °C", nombre, t),
        Tierra {
            radio: r,
            nombre: n,
            temperatura: t,
        } => println!("Tierra: {}, radio: {} km, temperatura: {} °C", n, r, t),

        _ => println!("No es Tierra"),
    };
}

fn es_planetoide_o_venus(p: &Planeta) {
    use Planeta::*;

    match p {
        Planetoide | Venus(9000..) => println!("No es planeta"),
        Venus(_) | Tierra { .. } => println!("Es Venus o Tierra"),
        _ => println!("Es un planeta"),
    };
}

struct PlanetaAlienigena {
    pub nombre: String,
    pub distancia: i32,
}

fn planeta_alienigena(p: &PlanetaAlienigena) {
    match p {
        PlanetaAlienigena {
            distancia: d @ 1000..,
            nombre,
        } => println!("Exoplaneta lejano {}, a {} años luz", nombre, d),
        PlanetaAlienigena {
            distancia: d,
            nombre: n,
        } if *d < 100 => println!("Exoplaneta cercano {}, a {} años luz", n, d),

        PlanetaAlienigena {
            distancia: d,
            nombre: n,
        } => println!("Exoplaneta {}, a {} años luz", n, d),
    }
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Match");
    println!("--------------------");
    println!();

    let p1 = Planeta::Marte(String::from("Marte"));
    let p2 = Planeta::Venus(6052);
    let p3 = Planeta::Jupiter(String::from("Jupiter"), 69911);
    let p4 = Planeta::Planetoide;
    let p5 = Planeta::Asteroide;

    let tierra = Planeta::Tierra {
        nombre: String::from("Tierra"),
        radio: 6378,
        temperatura: 15.2,
    };

    let p6 = Planeta::Mercurio(2439);

    // println!("Variante {:?}", &p1);
    // println!("Variante {:?}", &p2);
    // println!("Variante {:?}", &p3);
    // println!("Variante {:?}", &p4);
    //
    expresion_match(&p1);
    expresion_match(&p2);
    expresion_match(&p3);
    expresion_match(&p4);
    expresion_match(&p5);
    expresion_match(&tierra);
    expresion_match(&p6);

    let radio = radio_estimado(&p1);
    println!("Marte: {}", radio);

    // ------- rama por defecto
    //
    let x = 1;
    match x {
        12 => println!("Bingo!"),
        valor => println!("Valor: {}", valor),
    }

    expresion_match2(&p1);
    expresion_match2(&p2);
    expresion_match2(&p3);
    expresion_match2(&p4);
    expresion_match2(&tierra);
    expresion_match2(&p6);

    let x = 1;
    match x {
        valor_exacto @ 10..15 => {
            println!("Bingo! {}", valor_exacto);
        }
        fuera_rango => println!("Fallo: {}", fuera_rango),
    }

    // ( a , b )
    // (entero, float)
    let _x = (20, 35.8);
    let x = (0, -10.0);
    match x {
        (15..25, n) => {
            println!("'a' en [15, 24], 'b': {}", n);
        }
        tup @ (_, ..0.0) => {
            println!("'a' fuera de [15, 24]");
            println!("'b' < 0.0 {:?}", tup);
        }
        otro => println!("Tupla: {} {}", otro.0, otro.1),
    }

    let x = (20, 35.8);

    if let (15..25, n) = x {
        println!("'a' en [15, 24], 'b': {}", n);
    };

    // equivalente a
    match x {
        (15..25, n) => {
            println!("'a' en [15, 24], 'b': {}", n);
        }
        _ => println!("No me interesa"),
    }

    if let (15..25, n) = x {
        println!("'a' en [15, 24], 'b': {}", n);
    } else if let tup @ (_, ..0.0) = x {
        println!("'a' fuera de [15, 24]");
        println!("'b' < 0.0 {:?}", tup);
    } else {
        let otro = x;
        println!("Tupla: {} {}", otro.0, otro.1)
    }

    let planeta = Planeta::Planetoide;
    es_planetoide(&planeta);

    es_planeta(&planeta);

    es_venus(&p2);

    let mini_venus = Planeta::Venus(1000);
    es_venus(&mini_venus);

    es_tierra(&tierra);

    let tierra = Planeta::Tierra {
        nombre: String::from("Solaris A"),
        radio: 7890,
        temperatura: 182.0,
    };

    tipo_de_tierra(&tierra);

    let planeta = PlanetaAlienigena {
        nombre: "Solaris".to_string(),
        distancia: 44,
    };

    planeta_alienigena(&planeta);

    let planeta = PlanetaAlienigena {
        nombre: "Faraway".to_string(),
        distancia: 9099,
    };

    planeta_alienigena(&planeta);

    let planeta = Planeta::Tierra {
        nombre: String::from("Solaris A"),
        radio: 7890,
        temperatura: 182.0,
    };

    es_planetoide_o_venus(&planeta);
}
