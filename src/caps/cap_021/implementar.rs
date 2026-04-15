#[derive(Debug)]
struct Carguero {
    nombre: String,
    capacidad: i32,
    carga_actual: i32,
}

impl Carguero {
    fn new(n: &str) -> Self {
        Self {
            nombre: String::from(n),
            capacidad: 1000,
            carga_actual: 0,
        }
    }

    fn with_capacity(n: &str, c: i32) -> Self {
        Self {
            nombre: String::from(n),
            capacidad: c,
            carga_actual: 0,
        }
    }

    fn nombre(&self) -> &str {
        self.nombre.as_str()
    }
}

impl Carguero {
    fn add(&mut self, carga: i32) {
        self.carga_actual += carga;
        if self.carga_actual > self.capacidad {
            self.carga_actual = self.capacidad;
        }
    }

    fn sub(&mut self, carga: i32) {
        self.carga_actual -= carga;
        if self.carga_actual < 0 {
            self.carga_actual = 0;
        }
    }
}

impl Default for Carguero {
    fn default() -> Self {
        Self {
            nombre: String::from("C"),
            capacidad: 1000,
            carga_actual: 0,
        }
    }
}

fn posible_carguero(n: &str) -> Option<Carguero> {
    if n.len() > 1 {
        Some(Carguero {
            nombre: String::from(n),
            capacidad: 2000,
            carga_actual: 0,
        })
    } else {
        None
    }
}

// fluent interface / setters
impl Carguero {
    fn con_nombre(self, n: &str) -> Self {
        let mut cargo = self;
        //     ------^
        // como somos propietarios, podemos mover
        // la instancia a una variable local mutable

        // actualizamos
        cargo.nombre = n.to_string();

        // devolvemos la variable (move)
        cargo
    }

    fn con_capacidad(self, cap: i32) -> Self {
        let mut cargo = self;
        cargo.capacidad = cap;
        cargo
    }
}

#[derive(Default, Debug)]
struct CargueroBuilder {
    nombre: Option<String>,
    capacidad: Option<i32>,
    carga_actual: Option<i32>,
}

impl CargueroBuilder {
    fn nombre(self, n: &str) -> Self {
        let mut cargo = self;
        cargo.nombre = Some(n.to_string());
        cargo
    }

    fn capacidad(self, cap: i32) -> Self {
        let mut cargo = self;
        cargo.capacidad = Some(cap);
        cargo
    }

    fn carga(self, ca: i32) -> Self {
        let mut cargo = self;
        cargo.carga_actual = Some(ca);
        cargo
    }

    fn build(self) -> Carguero {
        Carguero {
            nombre: self.nombre.expect("Error: nombre"),
            capacidad: self.capacidad.expect("Error: capacidad"),
            carga_actual: self.carga_actual.expect("Error: carga"),
        }
    }
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Implementar");
    println!("--------------------");
    println!();

    let cargo = Carguero::new("C1");
    println!("{:?}", cargo);

    let mut cargo = Carguero::with_capacity("C2", 2000);

    cargo.add(250);
    cargo.sub(125);

    println!("{:?}", cargo);
    println!(
        "{}, {}, {}",
        cargo.nombre(),
        cargo.capacidad,
        cargo.carga_actual
    );

    let cargo = posible_carguero("CargoMax").unwrap_or_default();
    println!("Carguero: {:?}", cargo);

    let cargo = posible_carguero("1").unwrap_or_default();
    println!("Carguero: {:?}", cargo);

    let carguero_parcial = Carguero {
        nombre: "CP1".to_string(),
        ..Carguero::default()
    };

    println!("Carguero parcial: {:?}", carguero_parcial);

    // fluent interface
    let c1 = Carguero::default()
        .con_nombre("Super C1")
        .con_capacidad(2500);

    println!("Con fluent interface / setters: {:?}", c1);

    // builder
    let cargo = CargueroBuilder::default()
        .nombre("Super C1")
        .capacidad(2500)
        .carga(895)
        .build();

    println!("Carguero con builder: {:?}", cargo);

    let builder = CargueroBuilder::default();
    println!("Builder default {:?}", builder);

    let builder = builder.nombre("Super C1");
    let builder = builder.capacidad(2500);
    let builder = builder.carga(895);

    let cargo = builder.build();
    println!("Carguero by CargoBuilder: {:?}", cargo);

    //.capacidad(2500).carga(895);
}
