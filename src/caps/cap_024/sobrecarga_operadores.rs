use std::ops::Add;
// use std::ops::AddAssign;

// Debug para poder mostrar los resultados con println!
#[derive(Debug)]
struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct Vector3Di {
    x: f32,
    y: f32,
    z: f32,
}

impl Add for Vector3D {
    type Output = Self;

    fn add(self, v2: Self) -> Self::Output {
        Vector3D {
            x: self.x + v2.x,
            y: self.y + v2.y,
            z: self.z + v2.z,
        }
    }
}

impl Add<Vector3Di> for Vector3D {
    type Output = Self;

    fn add(self, v2: Vector3Di) -> Self::Output {
        Vector3D {
            x: self.x + v2.x as f64,
            y: self.y + v2.y as f64,
            z: self.z + v2.z as f64,
        }
    }
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Sobrecarga de operadores");
    println!("--------------------");
    println!();

    let v1 = Vector3D {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let v2 = Vector3D {
        x: 2.0,
        y: 2.0,
        z: 2.0,
    };

    // la suma ya está definida, podemos hacer v1 + v2
    // podemos usar println directamente gracias a derive(Debug)
    println!("Suma de vectores 3D: {:?}", v1 + v2);

    let v1 = Vector3D {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let v3 = Vector3Di {
        x: 2.0,
        y: 2.0,
        z: 2.0,
    };
    println!("Suma de vectores 3D: {:?}", v1 + v3);

    let resultado = 12 + 8;
    println!("{}", resultado);

    // implementación para i32 + i32
    let resultado = Add::add(12, 8);
    println!("{}", resultado);

    let resultado = 12.add(8);
    println!("{}", resultado);
}
