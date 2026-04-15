pub fn run() {
    println!();
    println!("--------------------");
    println!("if - else if - else ");
    println!("--------------------");
    println!();

    // if incompleto devuelve ()
    let a = 2;
    let b = if a != 1 {
        6;
    };

    println!("a: {}", a);
    println!("b: {:?}", b); // ()

    // if completo sin else ?
    // no, siempre tiene que haber rama else
    // para que sea completo
    let a = 2;
    let b = if a != 1 {
        // 5
    } else if a == 1 {
        // 10
    };

    println!("a: {}", a);
    println!("b: {:?}", b); // ()
}
