pub fn run() {
    println!();
    println!("--------------------");
    println!("Expresiones");
    println!("--------------------");
    println!();

    let mut b = 3;
    println!("b: {:?}", b);

    let a = b = 4;

    println!("a: {:?}", a);
    println!("b: {:?}", b);

    let mut b = String::from("Pi");
    println!("b: {:?}", b);

    let a = b = String::from("3.14");

    println!("a: {:?}", a);
    println!("b: {:?}", b);

    {
        let a = 1;

        {
            println!("a: {a}");
        }
    }

    // ejemplo para demostrar que las funciones
    // definen bloques de scope cerrado
    fn funcion_externa() {
        let _a = 1;

        fn funcion_interna() {
            // ERROR
            // println!("a: {a}");
            println!("interior");
        }

        funcion_interna();
    }

    funcion_externa();

    // let a : es una sentencia
    let a = {
        let _b: i32; // < no puede ir sin ; 
    };

    println!("a: {:?}", a);

    let mut b = 5;
    println!("b: {:?}", b); // ()

    let a = { b = 2 };

    println!("a: {:?}", a); // ()
    println!("b: {:?}", b); // ()

    // ejemplo expresion
    let _a = 1;
    let _c = { _a + { 4 * 2 + { 1 } } };
}
