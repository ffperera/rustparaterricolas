trait Aterrizar {}

trait Transportar {}

trait Terraformar: Transportar + Aterrizar {}

struct TerraForm {
    nombre: String,
}

impl Aterrizar for TerraForm {}
impl Transportar for TerraForm {}
impl Terraformar for TerraForm {}

struct Carguero {
    nombre: String,
}

impl Transportar for Carguero {}

fn terraformar_planeta(p: &str, _nave: &impl Terraformar) {
    // métodos de Transportar
    // métodos de Aterrizar
    // métodos de Terraformar
    println!("Terraformando {}", p);
}

fn planificar_ruta(p: &str, _nave: &impl Transportar) {
    println!("Planificando ruta hacia {}", p);
}

// ejemplos con Despetar, Orbitar, Navegar

trait Despegar {}

trait Orbitar: Despegar {}

trait Navegar: Orbitar {}

struct Nave {}
impl Navegar for Nave {}
impl Orbitar for Nave {}
impl Despegar for Nave {}

fn transporta<T: Navegar>(_nave: &T) {
    //
}

fn despega<T: Despegar>(_nave: &T) {
    //
}

struct Orbitador {}
impl Orbitar for Orbitador {}
impl Despegar for Orbitador {}
// Orbitador no implementa Transportar

pub fn run() {
    println!();
    println!("--------------------");
    println!("Traits");
    println!("--------------------");
    println!();

    let terraform = TerraForm {
        nombre: "Terra".to_string(),
    };

    println!("{}", terraform.nombre);

    terraformar_planeta("Venus", &terraform);

    let cargo = Carguero {
        nombre: "Cargo".to_string(),
    };

    println!("{}", cargo.nombre);

    // terraformar_planeta("Venus", cargo); // ERROR

    planificar_ruta("Venus", &terraform);
    planificar_ruta("Venus", &cargo);

    let nave = Nave {};
    transporta(&nave);
    despega(&nave);

    let nave = Orbitador {};
    // transporta(&nave); // ERROR
    despega(&nave); // OK
}

