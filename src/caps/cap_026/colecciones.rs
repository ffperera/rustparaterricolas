use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::collections::VecDeque;

pub fn run() {
    println!();
    println!("--------------------");
    println!("Colecciones");
    println!("--------------------");
    println!();

    println!();
    println!("Vec");
    println!();

    let mut naves = Vec::new();

    naves.push(("Neptuno", 0));
    naves.push(("Mercurio", 0));
    naves.push(("Venus", 2));
    naves.push(("Saturno", 1));
    naves.push(("Tierra", 1));
    naves.push(("Marte", 6779));
    naves.push(("Tierra", 10));

    for (planeta, num) in &naves {
        println!("{} : {}", planeta, num);
    }

    println!();
    println!("HashMap");
    println!();

    // naves desplegadas en el sistema solar
    let mut naves = HashMap::new();

    naves.insert("Mercurio", 0);
    naves.insert("Venus", 2);
    naves.insert("Tierra", 1);
    naves.insert("Marte", 6779);

    naves.insert("Tierra", 10);

    // acceso mediante índice
    // (riesgo de panic si no existe)
    println!("En la Tierra: {}", naves["Tierra"]);

    // acceso mediante get()
    // (riesgo de panic si no existe)
    println!("En Mercurio: {}", naves.get("Mercurio").unwrap());

    // mediante get y match
    match naves.get("Marte") {
        Some(cuantas) => println!("Hay {} naves", cuantas),
        None => println!("No hay naves en ese planeta"),
    }

    // si sólo nos interesa cuando existe la clave
    if let Some(cuantas) = naves.get("Venus") {
        println!("Hay {} naves", cuantas);
    }

    println!("Total naves: {}", naves.len());

    // or_insert()
    // si no existe esa clave,
    // se crea una nueva entrada para la clave
    // con el contenido especificado
    let jupiter = naves.entry("Júpiter").or_insert(5);

    // devuelve referencia mutable:
    *jupiter += 1;

    println!("En Júpiter: {}", jupiter); // 6

    // and_modify()
    // en combinación con or_insert()
    let jupiter = naves
        .entry("Júpiter")
        .and_modify(|item| *item += 1)
        .or_insert(11);

    println!("En Júpiter: {}", jupiter);

    // or_insert_with()
    // en este caso la closure captura la variable jupiter
    let jupiter = *jupiter;
    let saturno = naves.entry("Saturno").or_insert_with(|| jupiter);

    println!("En Saturno: {}", saturno);

    // or_insert_with_key()
    let neptuno = naves
        .entry("Neptuno")
        .or_insert_with_key(|planeta| planeta.len());
    println!("En Neptuno: {}", neptuno);

    // iterador
    for (planeta, cuantas) in naves.iter() {
        println!("En {} : {}", planeta, cuantas);
    }

    println!();
    println!("HashSet");
    println!();

    // array con nombres de planetas duplicados
    let planetas_duplicados = [
        "Mercurio", "Marte", "Marte", "Venus", "Tierra", "Tierra", "Marte", "Júpiter", "Saturno",
        "Neptuno", "Urano", "Neptuno", "Plutón",
    ];

    let mut planetas = HashSet::new();

    for planeta in planetas_duplicados {
        planetas.insert(planeta);
    }

    // listado de planetas sin duplicados
    for planeta in &planetas {
        println!("{}", planeta);
    }

    // - `insert()` - añadir elemento (clave) al conjunto.
    // - `contains()` - devuelve `true` si existe ese elemento.
    // - `remove()` - elimina un elemento del conjunto. Devuelve `true` si el elemento estaba en el set.

    if planetas.contains("Tierra") {
        println!("Todavía existe.");
    }

    if planetas.remove("Plutón") {
        println!("Plutón eliminado.");
    }

    println!();
    println!("BTreeMap");
    println!();

    let mut naves = BTreeMap::new();

    naves.insert("Neptuno", 0);
    naves.insert("Mercurio", 0);
    naves.insert("Venus", 2);
    naves.insert("Saturno", 1);
    naves.insert("Tierra", 1);
    naves.insert("Marte", 6779);

    naves.insert("Tierra", 10);

    // los elementos se muestran ordenados
    // alfabéticamente por su clave
    for (planeta, num) in &naves {
        print!("{} : {}  | ", planeta, num);
    }
    // Marte .. Mercurio .. Neptuno .. Saturno ...

    println!();

    // extraer slice
    let grupo = naves.range("Marte".."Saturno");
    for (planeta, num) in grupo {
        print!("{} : {}  | ", planeta, num);
    }

    // Marte : 6779  | Mercurio : 0  | Neptuno : 0  |

    println!();

    println!();
    println!("BTreeSet");
    println!();

    // use std::collections::BTreeSet;

    let planetas_duplicados = [
        "Mercurio", "Marte", "Marte", "Venus", "Tierra", "Tierra", "Marte", "Júpiter", "Saturno",
        "Neptuno", "Urano", "Neptuno", "Plutón",
    ];

    let mut planetas = BTreeSet::new();

    for planeta in planetas_duplicados {
        planetas.insert(planeta);
    }

    // listado de planetas sin duplicados
    // y además ordenados alfabéticamente
    for planeta in &planetas {
        println!("{}", planeta);
    }

    println!();
    println!("VecDeque");
    println!();

    // cola doble vacía para misiones espaciales
    let mut misiones = VecDeque::new();

    // misiones al final de la cola
    misiones.push_back("Despegar");
    misiones.push_back("Orbitar Tierra");
    misiones.push_back("Rumbo a Marte");

    // agregar al inicio
    misiones.push_front("Checklist pre-lanzamiento");

    // completar primera misión
    let completada = misiones.pop_front();
    println!("Completada: {:?}\n", completada.unwrap());

    // nueva misión menos prioritaria
    // (al final de la cola)
    misiones.push_back("Desplegar satélites");

    // nueva misión prioritaria
    // (al inicio de la cola)
    misiones.push_front("Calibrar instrumentos");

    // misiones
    for mision in &misiones {
        println!("{}", mision);
    }

    // acceso a ambos extremos
    println!("Próxima misión: {}", misiones.front().unwrap());
    println!("Última misión: {}", misiones.back().unwrap());

    println!();
    println!("LinkedList");
    println!();

    // componentes de una nave
    let mut componentes = LinkedList::new();

    // añadir componentes al final de la lista
    componentes.push_back("Propulsor principal");
    componentes.push_back("Sistema de navegación");
    componentes.push_back("Computadora de vuelo");

    // componentes al inicio
    componentes.push_front("Escudo térmico");
    componentes.push_front("Sistema de emergencia");

    // instalar primer componente
    let instalado = componentes.pop_front();
    println!("Componente {:?}", instalado.unwrap());

    // componentes
    for comp in &componentes {
        println!("{}", comp);
    }

    // dividir la lista en dos partes
    let mitad = componentes.len() / 2;
    let mut nuevos_componentes = componentes.split_off(mitad);

    // componentes de la primera lista
    for comp in &componentes {
        print!("{} → ", comp);
    }

    println!();

    // componentes de la segunda lista
    for comp in &nuevos_componentes {
        print!("{} → ", comp);
    }

    println!();

    // combinar las dos listas
    componentes.append(&mut nuevos_componentes);
    println!("{:?}", componentes);

    println!();
    println!("BinaryHeap");
    println!();

    // implementar todos los traits necesarios para
    // comparación de elementos
    #[derive(Debug, Eq, PartialEq)]
    struct Alerta {
        gravedad: u32,
        mensaje: &'static str,
    }

    impl Alerta {
        fn new(gravedad: u32, mensaje: &'static str) -> Self {
            Alerta { gravedad, mensaje }
        }
    }

    // implementar trait Ord
    // mayor gravedad = mayor prioridad
    impl Ord for Alerta {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.gravedad.cmp(&other.gravedad)
        }
    }

    // implementar trait PartialOrd
    // mayor gravedad = mayor prioridad
    impl PartialOrd for Alerta {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    // gestionar alertas del sistema
    // interesa siempre la más prioritaria
    let mut alertas = BinaryHeap::new();

    // mayor valor = mayor prioridad
    alertas.push(Alerta::new(3, "Fallo sistema refrigeración"));
    alertas.push(Alerta::new(1, "Baja presión en módulo"));
    alertas.push(Alerta::new(5, "Pérdida de oxígeno en cabina"));
    alertas.push(Alerta::new(2, "Temperatura exterior crítica"));
    alertas.push(Alerta::new(4, "Fallo en escudos de radiación"));

    // esta alerta debería ser la más prioritaria
    alertas.push(Alerta::new(10, "Impacto de meteorito inminente"));

    // acceder a la alerta más prioritaria
    // (sin eliminarla de la lista)
    println!("Alerta máxima: {:?}", alertas.peek().unwrap());

    // gestionar alertas por orden de prioridad
    // con pop() se extrae el elemento prioritario
    // (se elimina de la lista)
    while let Some(alerta) = alertas.pop() {
        println!("{}: {}", alerta.gravedad, alerta.mensaje);
    }
}
