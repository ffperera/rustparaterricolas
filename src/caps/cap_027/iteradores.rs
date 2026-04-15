pub fn run() {

    println!("");
    println!("--------------------");
    println!("Iteradores");
    println!("--------------------");
    println!("");


    // // colección
    // // en este caso Vec
    // let vector = vec![1, 2, 3, 4, 5, 6, 7];
    //
    // // iterador necesita ser mutable porque next()
    // // modifica su estado interno
    // let mut iterador = vector.iter();
    //
    // if let Some(elemento) = iterador.next() {
    //
    //     // 'elemento' es una referencia 
    //     let b = *elemento + 10; 
    //
    //     println!("{}", b);  // 11
    // }
    //


let vector = vec![
        String::from("Mercurio"),
        String::from("Venus"),
        String::from("Tierra"),
        String::from("Marte"),
    ];


    // iterador necesita ser mutable porque next()
    // modifica su estado interno
    let mut iterador = vector.iter();

    if let Some(elemento) = iterador.next() {
        // 'elemento' es una referencia 
        println!("Primero: {}", elemento);
    }



    let iterador = vector.iter();

    for planeta in iterador {
        // 'planeta' es una referencia
        // al elemento actual de la colección
        println!("{}", planeta);
    }

    
    for planeta in &vector {
        // 'planeta' es una referencia
        // al elemento actual de la colección
        println!("{}", planeta);
    }






    let vector = vec![
        String::from("Mercurio"),
        String::from("Venus"),
        String::from("Tierra"),
        String::from("Marte"),
    ];

    // consume los elementos de la colección
    let mut iterador = vector.into_iter();

    // vector ya no está activo 
    // println!("{:?}", vector);

    if let Some(elemento) = iterador.next() {

        // 'elemento' es propietario del contenido
        println!("Primero: {}", elemento);

        // 'elemento' se consume aquí
    }


    let vector = vec![
        String::from("Mercurio"),
        String::from("Venus"),
        String::from("Tierra"),
        String::from("Marte"),
    ];

    for planeta in vector {
        // planeta es propietario del contenido
        // del elemento actual
        
        // puede hacer lo que quiera, por ejemplo
        // reasignar a una variable propietaria mutable 
        // para modificar el contenido
        let mut a = planeta;
        a.push_str("??");
        println!("{}", a);
    }

    // vector se ha consumido en el proceso




    let mut vector = vec![
        String::from("Mercurio"),
        String::from("Venus"),
        String::from("Tierra"),
        String::from("Marte"),
    ];

    // iterador devuelve una referencia exclusiva
    // a cada elemento de la colección
    // la variable propietaria de la colección 
    // tiene que ser mutable
    let mut iterador = vector.iter_mut();

    if let Some(elemento) = iterador.next() {

        // 'elemento' es una referencia mutable
        elemento.push_str("!!");
    }

    println!("{:?}", vector);
    // ["Mercurio!!", "Venus", "Tierra", "Marte"]


    // tiene que ser mutable para poder
    // generar referencias mutables
    let mut vector = vec![
        String::from("Mercurio"),
        String::from("Venus"),
        String::from("Tierra"),
        String::from("Marte"),
    ];

    let iterador = vector.iter_mut();

    for planeta in iterador {
        // 'planeta' es una referencia mutable
        planeta.push_str("!!");
        println!("{}", planeta);
    }


    println!("{:?}", vector);
    // ["Mercurio!!", "Venus!!", "Tierra!!", "Marte!!"]


    for planeta in &mut vector {
        // 'planeta' es una referencia mutable
        planeta.push_str("!");
        println!("{}", planeta);
    }


    // vector de elementos tipo copia

    let vector = vec![1, 2, 3, 4, 5, 6, 7];

    for i in vector {
        print!("{} ", i);
    }

    // vector se consume igualmente porque 
    // se mueve al bucle for in
    // println!("{:?}", vector);


    // Equivalentes
    
    let vector = vec![
        String::from("Mercurio"),
        String::from("Venus"),
        String::from("Tierra"),
        String::from("Marte"),
    ];

    println!("");
        
    for planeta in (&vector).into_iter() {
        // 'planeta' es una referencia al elemento
        print!("{}.", planeta);
    }


    println!("");

    // equivalente a 
    for planeta in vector.iter() {
        // 'planeta' es una referencia al elemento
        print!("{}.", planeta);
    }


    println!("");

    // equivalente a 
    for planeta in &vector {
        // 'planeta' es una referencia al elemento
        print!("{}.", planeta);
    }


    println!("");


    // rangos

    // se genera un iterador de forma implícita
    // (llamada a into_iter() implícita)
    for item in 1..20 {
        print!("{:?}.", item);
    }


    println!("");

    let range = 1..20;

    // también de forma implícita a partir de Range
    // (la variable range se consume, deja de estar disponible)
    for item in range {
        print!("{:?}.", item);
    }


    println!("");

    let range = 1..20;

    // de forma explícita con into_iter()
    // (la variable range se consume, deja de estar disponible)
    let iterator = range.into_iter();
    for item in iterator {
        print!("{:?}.", item);
    }

    println!("");



    // map
    let vector = vec![
        String::from("Mercurio"),
        String::from("Venus"),
        String::from("Tierra"),
        String::from("Marte"),
    ];

    let resultado = vector.iter().map(|planeta| {
       planeta.to_uppercase() 
    }).collect::<Vec<String>>();

    println!("{:?}", resultado);


    // collect
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // en el momento en que uno de los elementos 
    // no cumpla la condición, map() devolverá None
    // y no procesará el resto de elementos
    let res: Option<Vec<_>> = vector
        .iter()
        .map(|&x| {
            if x < 3 { return Some(String::from("pequeño")) };
            if x < 6 { return Some(String::from("mediano")) };
            if x == 6 { return Some(String::from("grande")) };
            None
        })
        .collect();

    // el resultado es None
    println!("{:?}", res);


    let resultado: Vec<Option<_>> = vector
        .iter()
        .map(|&x| {
            if x < 3 { return Some(String::from("pequeño")) };
            if x < 6 { return Some(String::from("mediano")) };
            if x == 6 { return Some(String::from("grande")) };
            None
        })
        .collect();

    // [Some("pequeño"), Some("pequeño")... None, None]
    println!("{:?}", resultado);


    let resultado = vector
        .iter()
        .map(|&x| {
            if x < 3 { return Some(String::from("pequeño")) };
            if x < 6 { return Some(String::from("mediano")) };
            if x == 6 { return Some(String::from("grande")) };
            None
        })
        .collect::<Vec<Option<_>>>();

    // [Some("pequeño"), Some("pequeño")... None, None]
    println!("{:?}", resultado);




    let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];

    vector
        .iter()
        .filter(|&&v| v <= 5)
        .for_each(|v| print!("{v} "));

    println!("");



    // ejemplos métodos iteradores
    //
    let planetas = [ "Marte", "VEnus", 
                    "TierrA", "JúPiter", 
                    "Saturno", "P9" ];
    
    let codigos = planetas
        .iter()
        .filter(|p| p.len() > 3)  // sólo nombres largos
        .map(|p| {
            // extraer mayúsculas
            p.chars()
                .filter(|c| c.is_ascii_uppercase())  
                .collect::<String>()
        })
        .enumerate()
        .map(|(i, codigo)| format!("{}-{}", codigo, i + 1))
        .collect::<Vec<_>>();

    println!("Códigos planetarios:");
    for codigo in codigos {
        print!("{} ", codigo);
    }

    // M-1 VE-2 TA-3 JP-4 S-5 

    println!("");


    let comunicacion = "ALERTA@Nivel4#Meteoritos@Trayectoria@Colision@Inminente";
    
    let palabras = comunicacion
        .split(|c: char| c == '@' || c == '#' || c == ' ')  // separadores
        .filter(|palabra| palabra.len() > 4)  // palabras significativas
        .map(|palabra| palabra.to_uppercase())  // a mayúsculas
        .collect::<Vec<_>>();

    println!("Palabras claves:");
    for palabra in palabras {
        println!("- {}", palabra);
    }

    // Verificar si es emergencia
    let es_emergencia = comunicacion
        .split_whitespace()
        .any(|palabra| palabra.contains("ALERTA") || 
                       palabra.contains("INMINENTE"));
    
    println!("Estado emergencia: {}", es_emergencia);


    println!("");
}
