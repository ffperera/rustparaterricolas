// use thiserror::Error;
// use std::cell::RefCell;
// use std::rc::Rc;

// --- DOMINIO ---

// use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
enum NaveError {
    FalloMotor(String),
    ErrorOrientacion(String),
    ManiobraInvalida(String),
}

// Driven Ports
trait MotorPort {
    fn aplicar_impulso(&mut self, impulso: i32) -> Result<(), NaveError>;
    fn impulso_actual(&self) -> i32;
}

trait GiroscopioPort {
    fn fijar_rumbo(&mut self, coor: Coordenadas) -> Result<(), NaveError>;
    fn rumbo_actual(&self) -> Coordenadas;
}

// Driving ports
trait ComunicacionesPort {
    fn enviar_orden(&self, mensaje: String) -> Result<(), NaveError>;
}

#[allow(dead_code)]
#[derive(Debug)]
struct Nave<M, G> {
    nombre: String,
    motor: M,
    giro: G,
}

impl<M, G> Nave<M, G>
where
    M: MotorPort,
    G: GiroscopioPort,
{
    fn new(nombre: &str, motor: M, giro: G) -> Self {
        Self {
            nombre: nombre.to_string(),
            motor,
            giro,
        }
    }

    // casos de uso

    fn set_impulso(&mut self, impulso: i32) {
        self.motor.aplicar_impulso(impulso).unwrap();
    }

    fn set_orientacion(&mut self, orientacion: Coordenadas) {
        _ = self.giro.fijar_rumbo(orientacion);
    }

    fn impulso(&self) -> i32 {
        self.motor.impulso_actual()
    }

    fn rumbo(&self) -> Coordenadas {
        self.giro.rumbo_actual()
    }
}

impl<M, G> ComunicacionesPort for Nave<M, G> {
    fn enviar_orden(&self, mensaje: String) -> Result<(), NaveError> {
        println!("Orden {}", mensaje);
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Coordenadas(i32, i32, i32);

// posibles comandos
#[allow(dead_code)]
enum Comando {
    Rumbo(Coordenadas),
    Impulso(i32),
    ManiobraEvasiva,
    Mensaje(String),
}

// Adaptadores de infraestructura
// Motores + Giroscopio
struct AdaptadorMotoresElectricos {
    impulso: i32,
}

impl MotorPort for AdaptadorMotoresElectricos {
    fn aplicar_impulso(&mut self, impulso: i32) -> Result<(), NaveError> {
        if !(0..=100).contains(&impulso) {
            return Err(NaveError::FalloMotor(
                "Parámetro no válido para impulso".into(),
            ));
        }

        println!("Cambiando impulso al {}%", impulso);
        self.impulso = impulso;

        Ok(())
    }

    fn impulso_actual(&self) -> i32 {
        self.impulso
    }
}

struct AdaptadorGiroscopioLaser {
    coordenadas: Coordenadas,
}

impl GiroscopioPort for AdaptadorGiroscopioLaser {
    fn fijar_rumbo(&mut self, coor: Coordenadas) -> Result<(), NaveError> {
        self.coordenadas = coor.clone();

        println!("Cambiando rumbo a {:?}", coor);

        Ok(())
    }

    fn rumbo_actual(&self) -> Coordenadas {
        self.coordenadas.clone()
    }
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Arquitectura");
    println!("--------------------");
    println!();

    // Nave

    // 1. Instanciar adaptadores (infraestructura)
    let motor = AdaptadorMotoresElectricos { impulso: 0 };
    let giroscopio = AdaptadorGiroscopioLaser {
        coordenadas: Coordenadas(1, 1, 1),
    };

    let mut nave = Nave::new("Nave1", motor, giroscopio);

    nave.set_impulso(55);
    nave.set_orientacion(Coordenadas(2, 2, 2));
    nave.enviar_orden("Mensaje".into()).unwrap();

    println!("Impulso actual: {}", nave.impulso());
    println!("Rumbo actual: {:?}", nave.rumbo());
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AdaptadorMotorTest {
        impulso: i32,
    }

    impl MotorPort for AdaptadorMotorTest {
        fn aplicar_impulso(&mut self, impulso: i32) -> Result<(), NaveError> {
            if !(0..=100).contains(&impulso) {
                return Err(NaveError::FalloMotor(
                    "Parámetro no válido para impulso".into(),
                ));
            }

            self.impulso = impulso;

            Ok(())
        }

        fn impulso_actual(&self) -> i32 {
            self.impulso
        }
    }

    struct AdaptadorGiroTest {
        coordenadas: Coordenadas,
    }

    impl GiroscopioPort for AdaptadorGiroTest {
        fn fijar_rumbo(&mut self, coor: Coordenadas) -> Result<(), NaveError> {
            self.coordenadas = coor.clone();

            Ok(())
        }

        fn rumbo_actual(&self) -> Coordenadas {
            self.coordenadas.clone()
        }
    }

    #[test]
    fn aplicar_impulso_con_un_valor_valido() {
        let motor = AdaptadorMotorTest { impulso: 0 };
        let giroscopio = AdaptadorGiroTest {
            coordenadas: Coordenadas(1, 1, 1),
        };

        let mut nave = Nave::new("Test", motor, giroscopio);

        nave.set_impulso(55);

        assert_eq!(55, nave.impulso());
    }
}
