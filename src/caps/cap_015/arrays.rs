pub fn run() {
    println!();
    println!("--------------------");
    println!("Arrays");
    println!("--------------------");
    println!();

    let a1: [i32; 10] = [1; 10];
    println!("{:?}", a1);

    let a2: [f64; 10] = [1.0; 10];
    println!("{:?}", a2);

    let a3: [f64; 5] = [1.0; 5];
    println!("{:?}", a3);

    // acceso mediante get()

    let indice = 5;
    // println!("El último elemento es {}", a3[indice]);

    match a3.get(indice) {
        Some(i) => println!("El elemento es {}", i),
        None => println!("Error, no se encontró ese elemento"),
    }

    // mutabilidad

    let mut semana_alienigena = ["domingo"; 7];
    semana_alienigena[0] = "viernes";
    semana_alienigena[1] = "sábado";
    println!("{:?}", semana_alienigena);

    match semana_alienigena.get_mut(0) {
        Some(dia) => *dia = "sábado",
        None => println!("Error, no se encontró ese elemento"),
    }

    println!("{:?}", semana_alienigena);

    // desestructuración
    let a1 = [1, 2, 3];
    let [uno, dos, tres] = a1;
    println!("Números: {} {} {}", uno, dos, tres);

    // a1 sigue existiendo
    println!("{:?}", a1);

    // comodines
    let a1 = [1, 2, 3];
    let [_, dos, _] = a1;
    println!("Dos: {}", dos);

    let a1 = [1, 2, 3];
    let [uno, resto @ ..] = a1;
    println!("Uno: {}", uno);
    println!("Resto: {:?}", resto);

    let semana_humana = [
        String::from("lunes"),
        String::from("lunes"),
        String::from("lunes"),
    ];

    let [dia1, dia2, dia3] = semana_humana;
    println!("Días: {} {} {}", dia1, dia2, dia3);

    // println!("{:?}", semana_humana );

    // pattern matching
    //
    let a1 = [1, 2, 3];

    match a1 {
        [1, a, b] => println!("Comienza con 1 - {} {}", a, b),
        [2, _, _] => println!("Comienza con 2"),
        [_, subarray @ ..] => println!("Default: {:?}", subarray),
    }

    // ownership
    let mut a1 = [1; 1000];
    let a2 = a1;

    a1[500] = 2;

    println!("Pos. 500: {}", a1[500]);
    println!("Pos. 500: {}", a2[500]);

    let semana_humana = [
        String::from("lunes"),
        String::from("lunes"),
        String::from("lunes"),
    ];

    // let tercer_dia = semana_humana[2];
    // println!("Tercer día {}", tercer_dia);
    let semana = semana_humana;

    println!("Semana: {:?}", semana);
    // println!("Semana humana: {:?}", semana_humana);

    let semana_humana = [
        String::from("lunes"),
        String::from("lunes"),
        String::from("lunes"),
    ];

    // clonamos el elemento
    // el elemento original del array sigue existiendo
    let segundo_dia = semana_humana[1].clone();
    println!("Segundo dia: {}", segundo_dia);

    // desestructuración del array original
    // el array original se consume
    // ahora tenemos la variable 'primer_dia'
    // y otro array: 'resto'
    let [primer_dia, resto @ ..] = semana_humana;
    println!("Segundo dia: {}", primer_dia);
    println!("Semana: {:?}", resto);

    // funciones
    fn funcion(datos: [i32; 3]) {
        println!("{:?}", datos);
    }

    let a1 = [1, 2, 3];
    funcion(a1);

    //let a2 = [1, 2, 3, 4];
    // funcion(a2);
}
