#[derive(Debug)]
struct ArrayFlexible<T, const N: usize> {
    contenedor: [T; N],
}

#[derive(Debug)]
struct Matriz<T, const M: usize, const N: usize> {
    data: [[T; M]; N],
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Constante genérica (generic const)");
    println!("--------------------");
    println!();

    let a1 = ArrayFlexible {
        contenedor: [10; 10],
    };

    let a2 = ArrayFlexible {
        contenedor: ["ping"; 4],
    };

    println!("{:?}", a1.contenedor);
    println!("{:?}", a2.contenedor);

    let mut m1 = Matriz {
        data: [[0.0; 3]; 3],
    };
    m1.data[0][0] = 1.0;
    println!("{:?}", m1);
}
