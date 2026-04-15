pub fn run() {
    println!();
    println!("--------------------");
    println!("Vectores");
    println!("--------------------");
    println!();

    // declaración y método push()

    struct PlanetaAlienigena {
        nombre: String,
        distancia: i32,
    }

    // todavía no hay asignación de memoria en el heap
    let mut planetas_alienigenas = Vec::new();

    // con la primera llamada a push() el compilador determina
    // el tipo de cada item y reserva memoria en el heap

    // Se guarda el primer item en vector
    planetas_alienigenas.push(PlanetaAlienigena {
        nombre: "Prust I".to_string(),
        distancia: 54,
    });

    let p2 = PlanetaAlienigena {
        nombre: "Drust III".to_string(),
        distancia: 76,
    };

    // Se guarda el segundo item en vector
    // mediante una variable intermedia, que se consume
    planetas_alienigenas.push(p2);

    for planeta in planetas_alienigenas {
        println!("{} {}", planeta.nombre, planeta.distancia);
    }

    // acceder a elementos de un vector
    //
    let vector = vec![0, 1, 2, 3, 4, 5, 6];

    // simular algún índice arbitrario
    let i = "texto".len() - 4;

    // comprobar si el índice está en rango
    // como i es de tipo usize, no hay que comprobar si
    // es menor que cero
    if i < vector.len() {
        println!("{}", vector[i]);
    }

    // sin necesidad de comprobar el rango, a través de get(i)
    // quitar la envoltura Option para acceder al item (si existe)
    if let Some(num) = vector.get(5) {
        println!("{}", num);
    }

    // equivalente con match si quisiéramos gestionar la rama None
    match vector.get(i) {
        Some(num) => println!("{}", num),
        None => println!("No existe ese elemento."),
    }

    // slices

    let vector = vec![0, 1, 2, 3, 4, 5, 6];

    // si los índices no son válidos: panic
    let slice = &vector[2..6];
    println!("{:?}", slice); // [2, 3, 4, 5]

    // devuelve None si los índices no son válidos
    let slice = vector.get(2..6);
    println!("{:?}", slice); // Some([2, 3, 4, 5])

    // métodos asociados a slices

    let mut numeros = vec![1, 2, 3, 4, 5];
    let slice = &numeros[..];

    // len()
    println!("Longitud: {}", slice.len());

    // is_empty()
    println!("¿Está vacío? {}", slice.is_empty());

    // first() / last()
    println!("Primero: {:?}", slice.first());
    println!("Último: {:?}", slice.last());

    // split_at()
    let (left, right) = slice.split_at(3);
    println!("Left: {:?}, Right: {:?}", left, right);

    // sort() - Ordenar el vector
    let mut desordenados = vec![3, 1, 4, 2, 5];
    let slice = &mut desordenados[..];
    slice.sort();
    println!("Ordenados: {:?}", desordenados);

    // reverse()
    let slice_mut = &mut numeros[..];
    slice_mut.reverse();
    println!("Vector después de reverse(): {:?}", numeros);

    // fill()
    let mut vector_inicial = vec![1, 2, 3, 4, 5];
    let slice = &mut vector_inicial[..];
    slice.fill(0);
    println!("Todo ceros: {:?}", vector_inicial);

    // split_at()
    let numeros = vec![1, 2, 3, 4, 5];
    let slice = &numeros[..];
    let (left, right) = slice.split_at(3);
    println!("Izquierda: {:?}, Derecha: {:?}", left, right);

    // capacidad
    let mut vector: Vec<i32> = Vec::new();
    // Capacidad: 0, Tamaño: 0
    println!("Capacidad: {}", vector.capacity());
    println!("Tamaño: {}", vector.len());

    vector.push(1);
    // Capacidad: 4, Tamaño: 1
    // (el valor de capacidad puede depender
    // de la versión de Rust y otros factores)
    println!("Capacidad: {}", vector.capacity());
    println!("Tamaño: {}", vector.len());

    let mut vector: Vec<i32> = vec![];
    // Capacidad: 0, Tamaño: 0
    println!("Capacidad: {}", vector.capacity());
    println!("Tamaño: {}", vector.len());

    vector.push(1);
    // Capacidad: 4, Tamaño: 1
    println!("Capacidad: {}", vector.capacity());
    println!("Tamaño: {}", vector.len());
}
