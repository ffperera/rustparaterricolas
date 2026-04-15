/// Functional core: dominio puro de la aplicación.
///
/// En este módulo definimos las entidades del dominio, lógica de negocio pura,
/// tipos de error y funciones sin efectos secundarios. Es el núcleo funcional
/// de la arquitectura.
mod core {

    #[derive(Clone, Debug, PartialEq)]
    pub struct IdentificadorNave(String);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Nave {
        pub id: IdentificadorNave,
        pub nombre: String,
        pub estado: EstadoNave,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct EstadoNave {
        pub impulso: i32,
        pub rumbo: Coordenadas,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Coordenadas(pub i32, pub i32, pub i32);

    #[allow(dead_code)]
    #[derive(Debug, PartialEq)]
    pub enum NaveError {
        ImpulsoFueraDeRango { min: i32, max: i32, valor: i32 },
        RumboInvalido(String),
        NaveNoEncontrada(String),
        NaveYaExiste(String),
        ManiobraInvalida(String),
    }

    impl Nave {
        #[allow(dead_code)]
        pub fn new(id: IdentificadorNave, nombre: &str) -> Self {
            Self {
                id,
                nombre: nombre.to_string(),
                estado: EstadoNave {
                    impulso: 0,
                    rumbo: Coordenadas(0, 0, 0),
                },
            }
        }

        pub fn con_estado(id: IdentificadorNave, nombre: &str, estado: EstadoNave) -> Self {
            Self {
                id,
                nombre: nombre.to_string(),
                estado,
            }
        }

        pub fn cambiar_impulso(self, impulso: i32) -> Result<Nave, NaveError> {
            if !(0..=100).contains(&impulso) {
                return Err(NaveError::ImpulsoFueraDeRango {
                    min: 0,
                    max: 100,
                    valor: impulso,
                });
            }

            Ok(Nave {
                estado: EstadoNave {
                    impulso,
                    rumbo: self.estado.rumbo,
                },
                ..self
            })
        }

        pub fn cambiar_rumbo(self, rumbo: Coordenadas) -> Result<Nave, NaveError> {
            // validación de rumbo

            // ok
            Ok(Nave {
                estado: EstadoNave {
                    impulso: self.estado.impulso,
                    rumbo: rumbo.clone(),
                },
                ..self
            })
        }
    }

    impl IdentificadorNave {
        pub fn new(id: &str) -> Self {
            Self(String::from(id))
        }

        pub fn as_str(&self) -> &str {
            self.as_ref()
        }
    }

    impl std::fmt::Display for IdentificadorNave {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl AsRef<str> for IdentificadorNave {
        fn as_ref(&self) -> &str {
            self.0.as_str()
        }
    }
}

/// Imperative shell: capa de aplicación e infraestructura.
///
/// En este módulo implementamos los casos de uso, adaptación del dominio al mundo exterior,
/// gestión de estado, persistencia y comunicación con sistemas externos. Es la capa impura
/// que orquesta el núcleo funcional.
mod shell {

    use super::core::{Coordenadas, EstadoNave, IdentificadorNave, Nave, NaveError};
    use std::collections::HashMap;
    // use tokio::task::Id;
    use uuid::Uuid;

    pub struct Flota {
        naves: HashMap<String, (String, i32, Coordenadas)>,
    }

    impl Flota {
        pub fn new() -> Self {
            Self {
                naves: HashMap::new(),
            }
        }

        // caso de uso: crear nueva nave
        pub fn crear_nave(
            &mut self,
            nombre: &str,
            impulso: i32,
            rumbo: Coordenadas,
        ) -> Result<Nave, NaveError> {
            let id = Uuid::new_v4();
            self.naves
                .insert(id.to_string(), (nombre.to_string(), impulso, rumbo.clone()));

            Ok(Nave {
                id: IdentificadorNave::new(id.to_string().as_str()),
                nombre: nombre.to_string(),
                estado: EstadoNave {
                    impulso,
                    rumbo: rumbo.clone(),
                },
            })
        }

        // caso de uso: instanciar una nave desde la flota
        pub fn obtener_instancia_nave(&self, id: &str) -> Result<Nave, NaveError> {
            match self.naves.get(id) {
                Some((nombre, impulso, rumbo)) => Ok(Nave::con_estado(
                    IdentificadorNave::new(id),
                    nombre,
                    EstadoNave {
                        impulso: *impulso,
                        rumbo: rumbo.clone(),
                    },
                )),

                None => Err(NaveError::NaveNoEncontrada(id.to_string())),
            }
        }

        // caso de uso: cambiar impulso de una nave
        pub fn cambiar_impulso(
            &mut self,
            id: &str,
            impulso: i32,
            motor: impl Fn(i32) -> i32,
        ) -> Result<Nave, NaveError> {
            match self.obtener_instancia_nave(id) {
                Ok(nave) => nave
                    .cambiar_impulso(motor(impulso))
                    .and_then(|n| self.actualizar_estado(n)),
                Err(_) => Err(NaveError::NaveNoEncontrada(id.to_string())),
            }
        }

        // caso de uso: cambiar rumbo de una nave
        pub fn cambiar_rumbo(
            &mut self,
            id: &str,
            rumbo: Coordenadas,
            giro: impl Fn(Coordenadas) -> Coordenadas,
        ) -> Result<Nave, NaveError> {
            match self.obtener_instancia_nave(id) {
                Ok(nave) => nave
                    .cambiar_rumbo(giro(rumbo))
                    .and_then(|n| self.actualizar_estado(n)),
                Err(_) => Err(NaveError::NaveNoEncontrada(id.to_string())),
            }
        }

        // caso de uso: listar todas las naves
        pub fn listar_naves(&self) -> Vec<Nave> {
            self.naves
                .iter()
                .map(|(id, (n, i, r))| Nave {
                    id: IdentificadorNave::new(id),
                    nombre: n.clone(),
                    estado: EstadoNave {
                        impulso: *i,
                        rumbo: r.clone(),
                    },
                })
                .collect()
        }

        pub fn actualizar_estado(&mut self, nave: Nave) -> Result<Nave, NaveError> {
            self.naves.insert(
                nave.id.to_string(),
                (
                    nave.nombre.clone(),
                    nave.estado.impulso,
                    nave.estado.rumbo.clone(),
                ),
            );

            Ok(nave)
        }

        // caso de uso: eliminar nave
        pub fn _eliminar_nave(&mut self, id: &str) -> Result<(), NaveError> {
            self.naves.remove(id).unwrap();
            Ok(())
        }
    }
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Arquitectura - Functional core, imperative shell");
    println!("--------------------");
    println!();

    use core::Coordenadas;

    fn motor_ionico(impulso: i32) -> i32 {
        match impulso {
            ..0 => 0,
            101.. => 100,
            i => i,
        }
    }

    let motor_quimico = |i: i32| i.clamp(0, 90);

    fn giroscopio_laser(rumbo: Coordenadas) -> Coordenadas {
        // validación de coordenadas
        rumbo
    }

    let mut flota = shell::Flota::new();

    let n1 = flota
        .crear_nave("Rocinante", 55, Coordenadas(1, 1, 1))
        .unwrap();

    let n2 = flota
        .crear_nave("Nostromo", 2, Coordenadas(100, 100, 100))
        .unwrap();

    let id1 = n1.id.as_str();
    let id2 = n2.id.as_str();

    println!("\nNaves antes de actualizar estado flota:");

    flota
        .listar_naves()
        .iter()
        .for_each(|n| println!("{:?}", n));

    println!();

    let n1 = flota.cambiar_impulso(id1, 75, motor_ionico).unwrap();
    println!("Rocinante: {:?}", n1);

    let n2 = flota
        .cambiar_rumbo(id2, Coordenadas(10, 20, 30), giroscopio_laser)
        .and_then(|_| flota.cambiar_impulso(id2, 97, motor_quimico))
        .unwrap();

    println!("Nostromo: {:?}", n2);

    // flota.actualizar_estado(n1).unwrap();

    println!("\nNaves después de actualizar estado flota:");

    flota
        .listar_naves()
        .iter()
        .for_each(|n| println!("{:?}", n));
}
