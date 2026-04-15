use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
enum NaveError {
    FalloMotor(String),
    ErrorOrientacion(String),
    ManiobraInvalida(String),
    NaveNoEncontrada(String),
    NaveYaExiste(String),
}

// driven Ports
trait MotorPort {
    fn aplicar_impulso(&mut self, impulso: i32) -> Result<(), NaveError>;
    fn impulso_actual(&self) -> i32;
}

trait GiroscopioPort {
    fn fijar_rumbo(&mut self, coor: Coordenadas) -> Result<(), NaveError>;
    fn rumbo_actual(&self) -> Coordenadas;
}

trait NaveRepositorioPort {
    fn agregar_nave(&mut self, id: &str, nombre: &str) -> Result<(), NaveError>;
    fn obtener_nave(&self, id: &str) -> Result<String, NaveError>;
    fn obtener_todas(&self) -> Vec<(String, String)>;
    fn eliminar_nave(&mut self, id: &str) -> Result<(), NaveError>;
    // ...
}

// driving ports
trait ComunicacionesPort {
    fn enviar_orden(&self, mensaje: String) -> Result<(), NaveError>;
}

// dominio
// entidad Nave

// Dominio: Entidad Nave
#[derive(Debug)]
struct Nave<M, G> {
    id: String,
    nombre: String,
    motor: M,
    giro: G,
}

impl<M, G> Nave<M, G>
where
    M: MotorPort,
    G: GiroscopioPort,
{
    fn new(id: &str, nombre: &str, motor: M, giro: G) -> Self {
        Self {
            id: id.to_string(),
            nombre: nombre.to_string(),
            motor,
            giro,
        }
    }

    #[allow(dead_code)]
    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn nombre(&self) -> &str {
        self.nombre.as_str()
    }

    fn set_impulso(&mut self, impulso: i32) -> Result<(), NaveError> {
        self.motor.aplicar_impulso(impulso)
    }

    fn set_orientacion(&mut self, orientacion: Coordenadas) -> Result<(), NaveError> {
        self.giro.fijar_rumbo(orientacion)
    }

    fn impulso(&self) -> i32 {
        self.motor.impulso_actual()
    }

    fn rumbo(&self) -> Coordenadas {
        self.giro.rumbo_actual()
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Coordenadas(i32, i32, i32);

impl<M, G> ComunicacionesPort for Nave<M, G> {
    fn enviar_orden(&self, mensaje: String) -> Result<(), NaveError> {
        println!(
            "Nave {} [ID: {}]: Orden recibida: {}",
            self.nombre, self.id, mensaje
        );
        Ok(())
    }
}

// servicio de aplicación
// casos de uso
struct ServicioNave<M, G, R>
where
    M: MotorPort + Clone,
    G: GiroscopioPort + Clone,
    R: NaveRepositorioPort,
{
    repositorio: R,
    motor: fn() -> M,
    giro: fn() -> G,
}

impl<M, G, R> ServicioNave<M, G, R>
where
    M: MotorPort + Clone,
    G: GiroscopioPort + Clone,
    R: NaveRepositorioPort,
{
    fn new(repositorio: R, motor: fn() -> M, giro: fn() -> G) -> Self {
        Self {
            repositorio,
            motor,
            giro,
        }
    }

    // caso de uso: crear nueva nave
    fn crear_nave(&mut self, id: &str, nombre: &str) -> Result<String, NaveError> {
        // Verificar si ya existe una nave con ese nombre
        // let naves = self.repositorio.obtener_todas();
        // if naves.iter().any(|n| n == nombre) {
        //     return Err(NaveError::NaveYaExiste(nombre.to_string()));
        // }
        //
        self.repositorio.agregar_nave(id, nombre)?;
        Ok(id.to_string())
    }

    // caso de uso: instanciar una nave desde el repositorio
    fn obtener_instancia_nave(&self, id: &str) -> Result<Nave<M, G>, NaveError> {
        let nombre = self.repositorio.obtener_nave(id)?;

        let motor = (self.motor)();
        let giro = (self.giro)();

        Ok(Nave::new(id, &nombre, motor, giro))
    }

    // caso de uso: cambiar impulso de una nave
    fn cambiar_impulso(&self, id: &str, impulso: i32) -> Result<(), NaveError> {
        let mut nave = self.obtener_instancia_nave(id)?;
        nave.set_impulso(impulso)?;
        println!("Nave {}: Impulso cambiado a {}", id, impulso);
        Ok(())
    }

    // caso de uso: cambiar rumbo de una nave
    fn cambiar_rumbo(&self, id: &str, rumbo: Coordenadas) -> Result<(), NaveError> {
        let mut nave = self.obtener_instancia_nave(id)?;
        nave.set_orientacion(rumbo.clone())?;
        println!("Nave {}: Rumbo cambiado a {:?}", id, rumbo);
        Ok(())
    }

    // Caso de uso: Enviar orden a una nave
    fn enviar_orden(&self, id: &str, mensaje: String) -> Result<(), NaveError> {
        let nave = self.obtener_instancia_nave(id)?;
        nave.enviar_orden(mensaje)
    }

    // Caso de uso: Obtener estado de una nave
    fn obtener_estado(&self, id: &str) -> Result<(String, i32, Coordenadas), NaveError> {
        let nave = self.obtener_instancia_nave(id)?;
        Ok((nave.nombre().to_string(), nave.impulso(), nave.rumbo()))
    }

    // Caso de uso: Listar todas las naves
    fn listar_naves(&self) -> Vec<(String, String)> {
        self.repositorio.obtener_todas()
    }

    // Caso de uso: Eliminar nave
    fn eliminar_nave(&mut self, id: &str) -> Result<(), NaveError> {
        self.repositorio.eliminar_nave(id)
    }
}

// Implementación concreta del repositorio
struct NavesDB {
    naves: HashMap<String, String>,
}

impl NavesDB {
    fn new() -> Self {
        Self {
            naves: HashMap::new(),
        }
    }
}

impl NaveRepositorioPort for NavesDB {
    fn agregar_nave(&mut self, id: &str, nombre: &str) -> Result<(), NaveError> {
        self.naves.insert(id.to_string(), nombre.to_string());
        Ok(())
    }

    fn obtener_nave(&self, id: &str) -> Result<String, NaveError> {
        self.naves
            .get(id)
            .cloned()
            .ok_or(NaveError::NaveNoEncontrada(id.to_string()))
    }

    fn obtener_todas(&self) -> Vec<(String, String)> {
        self.naves
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    fn eliminar_nave(&mut self, id: &str) -> Result<(), NaveError> {
        self.naves.remove(id);
        Ok(())
    }
}

// Adaptadores de infraestructura
struct AdaptadorMotoresElectricos {
    impulso: i32,
}

impl AdaptadorMotoresElectricos {
    fn new() -> Self {
        Self { impulso: 0 }
    }
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

impl Clone for AdaptadorMotoresElectricos {
    fn clone(&self) -> Self {
        Self {
            impulso: self.impulso,
        }
    }
}

struct AdaptadorGiroscopioLaser {
    coordenadas: Coordenadas,
}

impl AdaptadorGiroscopioLaser {
    fn new() -> Self {
        Self {
            coordenadas: Coordenadas(0, 0, 0),
        }
    }
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

impl Clone for AdaptadorGiroscopioLaser {
    fn clone(&self) -> Self {
        Self {
            coordenadas: self.coordenadas.clone(),
        }
    }
}

// Función principal de ejemplo
pub fn run() {
    // Crear repositorio
    let repositorio = NavesDB::new();

    // Crear servicio
    let mut servicio = ServicioNave::new(
        repositorio,
        AdaptadorMotoresElectricos::new,
        AdaptadorGiroscopioLaser::new,
    );

    // Casos de uso
    println!("=== Creando naves ===");
    let id1 = servicio.crear_nave("roci", "Rocinante").unwrap();
    let id2 = servicio.crear_nave("nostro", "Nostromo").unwrap();
    let id3 = servicio.crear_nave("leo", "Leonov").unwrap();

    println!("Naves creadas con IDs: {}, {}, {}", id1, id2, id3);

    println!("\n=== Listando naves ===");
    for (id, nombre) in servicio.listar_naves() {
        println!("ID {}: {}", id, nombre);
    }

    println!("\n=== Modificando naves ===");
    servicio.cambiar_impulso(id1.as_str(), 75).unwrap();
    servicio
        .cambiar_rumbo(id2.as_str(), Coordenadas(10, 20, 30))
        .unwrap();
    servicio
        .enviar_orden(id3.as_str(), "Iniciar secuencia de despegue".to_string())
        .unwrap();

    println!("\n=== Obteniendo estado de naves ===");
    for (id, _) in servicio.listar_naves() {
        match servicio.obtener_estado(id.as_str()) {
            Ok((nombre, impulso, rumbo)) => {
                println!(
                    "Nave {}: {}, Impulso: {}, Rumbo: {:?}",
                    id, nombre, impulso, rumbo
                );
            }
            Err(e) => println!("Error obteniendo estado de nave {}: {:?}", id, e),
        }
    }

    println!("\n=== Eliminando nave ===");
    servicio.eliminar_nave(id2.as_str()).unwrap();
    println!(
        "Nave eliminada. Total de naves: {}",
        servicio.listar_naves().len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock para pruebas
    struct MockMotor {
        impulso: i32,
    }

    impl MockMotor {
        fn new() -> Self {
            Self { impulso: 0 }
        }
    }

    impl MotorPort for MockMotor {
        fn aplicar_impulso(&mut self, impulso: i32) -> Result<(), NaveError> {
            self.impulso = impulso;
            Ok(())
        }

        fn impulso_actual(&self) -> i32 {
            self.impulso
        }
    }

    impl Clone for MockMotor {
        fn clone(&self) -> Self {
            Self {
                impulso: self.impulso,
            }
        }
    }

    struct MockGiroscopio {
        coordenadas: Coordenadas,
    }

    impl MockGiroscopio {
        fn new() -> Self {
            Self {
                coordenadas: Coordenadas(0, 0, 0),
            }
        }
    }

    impl GiroscopioPort for MockGiroscopio {
        fn fijar_rumbo(&mut self, coor: Coordenadas) -> Result<(), NaveError> {
            self.coordenadas = coor;
            Ok(())
        }

        fn rumbo_actual(&self) -> Coordenadas {
            self.coordenadas.clone()
        }
    }

    impl Clone for MockGiroscopio {
        fn clone(&self) -> Self {
            Self {
                coordenadas: self.coordenadas.clone(),
            }
        }
    }

    #[test]
    fn test_crear_y_obtener_nave() {
        let repositorio = NavesDB::new();
        let mut servicio = ServicioNave::new(repositorio, MockMotor::new, MockGiroscopio::new);

        let id = servicio.crear_nave("uno", "Test Ship").unwrap();
        let nave = servicio.obtener_instancia_nave(id.as_str()).unwrap();

        assert_eq!(nave.id(), id);
        assert_eq!(nave.nombre(), "Test Ship");
    }

    #[test]
    fn test_cambiar_impulso() {
        let repositorio = NavesDB::new();
        let mut servicio = ServicioNave::new(repositorio, MockMotor::new, MockGiroscopio::new);

        let id = servicio.crear_nave("uno", "Test Ship").unwrap();
        servicio.cambiar_impulso(id.as_str(), 50).unwrap();

        let (_, impulso, _) = servicio.obtener_estado(id.as_str()).unwrap();
        assert_eq!(impulso, 50);
    }

    #[test]
    fn test_nombre_duplicado() {
        let repositorio = NavesDB::new();
        let mut servicio = ServicioNave::new(repositorio, MockMotor::new, MockGiroscopio::new);

        servicio.crear_nave("uno", "Enterprise").unwrap();
        let result = servicio.crear_nave("uno", "Enterprise");

        assert!(matches!(result, Err(NaveError::NaveYaExiste(_))));
    }
}
