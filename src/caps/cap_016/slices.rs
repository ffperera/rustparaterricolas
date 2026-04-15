pub fn run() {
    println!();
    println!("--------------------");
    println!("Slices");
    println!("--------------------");
    println!();

    // hit - histograma inteligencia terrícola
    let hit = [0, 1, 5, 10, 12, 10, 5, 1, 0];

    let humano_bebe = &hit[0..3];
    let humano_adulto = &hit[3..=5];
    let casi_alien = &hit[6..];

    println!("{:?}", humano_bebe); // [0, 1, 5]
    println!("{:?}", humano_adulto); // [10, 12, 10]
    println!("{:?}", casi_alien); // [5, 1, 0]

    // un rango es un tipo de dato
    // la variable 'rango' es ahora propietaria
    let rango = 3..=5;

    // la variable 'rango' se consume aquí
    println!("{:?}", &hit[rango]);

    let hit = [0, 1, 5, 10, 12, 10, 5, 1, 0];

    // intentamos obtener un slice que sobrepasa
    // la longitud de la colección
    // panic - error en tiempo de ejecución
    // index out of range
    // let humano_adulto =  &hit[3..=12];  // <- ERROR

    // todo el rango del array original
    let humano = &hit[..];

    // intentamos acceder a un índice
    // que no pertenece al slice
    // panic -  error en tiempo de ejecución
    // index out of bounds
    // println!("{}", humano[12]);  // <- ERROR

    println!("{:?}", humano);

    //
    let humano: &[i32];

    {
        let hit = [0, 1, 5, 10, 12, 10, 5, 1, 0];
        humano = &hit[..];
        println!("{:?}", humano);
    }

    // ERROR
    // println!("{:?}", humano);

    // paso de parámetros
    fn print_hit(tramo: &[i32]) {
        for i in tramo {
            println!("{}", i);
        }
    }

    let hit = [0, 1, 5, 10, 12, 10, 5, 1, 0];
    let adulto = &hit[3..=5];
    print_hit(adulto);

    // generar slices a través de get()
    let hit = [0, 1, 5, 10, 12, 10, 5, 1, 0];

    // let humano_adulto =  &hit[3..=12];  // <- ERROR

    let humano_adulto = hit.get(3..=5);
    println!("{:?}", humano_adulto);

    let humano_adulto = hit.get(3..=12);
    println!("{:?}", humano_adulto);

    // acceder a elementos de un slice a través de get()
    let hit = [0, 1, 5, 10, 12, 10, 5, 1, 0];

    let humano_adulto = hit.get(3..=5).unwrap();
    // &[10, 12, 10]

    let inteligencia = humano_adulto.get(0);
    println!("{:?}", inteligencia);
    // Some(10)

    let inteligencia = humano_adulto.get(3);
    println!("{:?}", inteligencia);
    // None

    // slices mutables

    let mut hit = [0, 1, 5, 10, 12, 10, 5, 1, 0];

    // transformar la inteligencia humana
    // para conquistar la Tierra

    let humano_adulto = &mut hit[3..=5];

    humano_adulto[0] = 7; // pos 3 del array
    humano_adulto[1] = 7;
    humano_adulto[2] = 7; // pos 5 del array

    println!("{:?}", hit);
    // [0, 1, 5, 7, 7, 7, 5, 1, 0]

    // - `len()` : Devuelve el número de elementos del *slice*.
    // - `first()` : Devuelve una referencia al primer elemento del *slice*, envuelta en `Option`.
    // - `last()` : Devuelve una referencia al último elemento del *slice*, envuelta en `Option`.
    // - `get(i)` : Devuelve una referencia al elemento asociado al índice `i`, envuelta en `Option`.
    // - `get(a..b)` : Devuelve un *slice* a partir del *slice* actual, envuelto en un `Option`.
    // - `get_mut(i)` : Devuelve una referencia exclusiva al elemento asociado al índice, envuelta en `Option`.
    // - `get_mut(a..b)` : Devuelve un *slice* mutable a partir del *slice* actual, envuelto en un `Option`.

    let mut hit = [0, 1, 5, 10, 12, 10, 5, 1, 0];
    let humano_adulto = &mut hit[3..=5];

    println!("Longitud: {}", humano_adulto.len());
    println!("Primero: {}", humano_adulto.first().unwrap());
    println!("Último: {}", humano_adulto.last().unwrap());

    // extraer un slice de un slice
    let sub_slice = humano_adulto.get(1..).unwrap();
    println!("Slice de slice: {:?}", sub_slice);

    let sub_slice = humano_adulto.get_mut(1..).unwrap();
    sub_slice[0] += 10;
    sub_slice[1] += 10;
    println!("Array modificado {:?}", hit);
    // [0, 1, 5, 10, 22, 20, 5, 1, 0]

    // métodos muy utilizados con slices

    let mut hit = [0, 1, 5, 10, 12, 10, 5, 1, 0];
    let humano_adulto = &mut hit[3..=5];

    println!("Longitud: {}", humano_adulto.len());
    println!("Primero: {}", humano_adulto.first().unwrap());
    println!("Último: {}", humano_adulto.last().unwrap());

    // extraer un slice de un slice
    let sub_slice = humano_adulto.get(1..).unwrap();
    println!("Slice de slice: {:?}", sub_slice);

    let sub_slice = humano_adulto.get_mut(1..).unwrap();
    sub_slice[0] += 10;
    sub_slice[1] += 10;
    println!("Array modificado {:?}", hit);
    // [0, 1, 5, 10, 22, 20, 5, 1, 0]

    // ejemplos con `reverse()` y con `fill()`

    let mut cuenta_adelante = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let cuenta = &mut cuenta_adelante[..];

    // reverse afecta al contenido real de la colección
    // el slice cuenta es simplemente una 'vista' a través
    // de la cual accedemos al contenido real
    cuenta.reverse();

    println!("Cuenta atrás: {:?} ", cuenta);
    // Cuenta atrás: [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]

    // lo mismo ocurre con fill()
    // afecta al contenido real de la colección
    cuenta.fill(1);
    println!("Todo unos: {:?} ", cuenta);
    // Todo unos: [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
}
