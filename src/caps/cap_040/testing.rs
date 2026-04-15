#[allow(dead_code)]
pub struct Carguero {
    nombre: String,
    carga: i32,
}

#[allow(dead_code)]
impl Carguero {
    pub fn new(nombre: String, carga: i32) -> Self {
        if carga <= 0 {
            return Self { nombre, carga: 0 };
        }

        Self { nombre, carga }
    }

    pub fn nombre(&self) -> &str {
        self.nombre.as_str()
    }

    pub fn carga(&self) -> i32 {
        self.carga
    }

    pub fn descargar(&mut self, cantidad: i32) -> i32 {
        if cantidad < 0 {
            return self.carga;
        }

        if cantidad > self.carga {
            self.carga = 0;
        } else {
            self.carga -= cantidad;
        }

        self.carga
    }

    pub fn vaciar(&mut self) {
        self.carga = 0;
    }

    pub fn cargar(&mut self, cantidad: i32) -> i32 {
        if cantidad < 0 {
            return self.carga;
        }

        self.carga += cantidad;
        self.carga
    }
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Testing");
    println!("--------------------");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOMBRE_NAVE: &str = "C1";

    #[test]
    fn crear_carguero_con_valores_normales() {
        // Arrange
        let nombre = String::from(NOMBRE_NAVE);
        let carga_inicial = 1000;

        // Act
        let carguero = Carguero::new(nombre, carga_inicial);

        // Assert
        assert_eq!(carguero.carga(), 1000);
        assert_eq!(carguero.nombre(), NOMBRE_NAVE);
    }

    #[test]
    fn crear_carguero_con_cantidad_cero_mantiene_carga_a_cero() {
        let carguero = Carguero::new(String::from(NOMBRE_NAVE), 0);

        // Assert
        assert_eq!(carguero.carga(), 0);
    }

    #[test]
    fn crear_carguero_con_cantidad_negativa_establece_carga_a_cero() {
        let carguero = Carguero::new(String::from(NOMBRE_NAVE), -500);

        assert_eq!(carguero.carga(), 0);
    }

    #[test]
    fn descargar_carguero_disminuye_la_carga() {
        let mut carguero = Carguero::new(String::from(NOMBRE_NAVE), 1000);

        let carga_restante = carguero.descargar(300);

        assert_eq!(carga_restante, 700);
    }

    #[test]
    fn descargar_carguero_cantidad_negativa_equivale_a_no_hacer_nada() {
        let mut carguero = Carguero::new(String::from(NOMBRE_NAVE), 0);

        let carga_restante = carguero.descargar(-300);

        assert_eq!(carga_restante, 0);
    }

    #[test]
    fn no_es_posible_descargar_carguero_mas_de_la_carga_disponible() {
        let mut carguero = Carguero::new(String::from(NOMBRE_NAVE), 200);

        let carga_restante = carguero.descargar(500);

        assert_eq!(carga_restante, 0);
    }

    #[test]
    fn vaciar_carguero_establece_la_carga_a_cero() {
        let mut carguero = Carguero::new(String::from(NOMBRE_NAVE), 850);

        carguero.vaciar();

        assert_eq!(carguero.carga(), 0);
    }

    #[test]
    fn cargar_cantidad_positiva_aumenta_la_carga_en_carguero() {
        let mut carguero = Carguero::new(String::from(NOMBRE_NAVE), 400);

        let nueva_carga = carguero.cargar(250);

        assert_eq!(nueva_carga, 650);
    }

    #[test]
    fn cargar_cantidad_negativa_en_carguero_no_hace_nada() {
        let mut carguero = Carguero::new(String::from(NOMBRE_NAVE), 400);

        let nueva_carga = carguero.cargar(-250);

        assert_eq!(nueva_carga, 400);
    }

    // sólo para copiar como ejemplos al capítulo

    #[test]
    fn test_con_assert() {
        assert!(2 + 2 == 4);
        //             ^--- expresión booleana
    }

    #[test]
    fn test_con_assert_msg() {
        assert!(2 + 2 != 4, "Fallo en el universo matemático {}", 2 + 2);
        //             ^--- expresión booleana
    }

    #[test]
    fn test_con_result() -> Result<(), String> {
        if 2 + 2 == 5 {
            //   ^--- expresión que queremos evaluar

            Ok(())
        } else {
            Err(format!("Fallo de la Matrix {}", 2 + 2))
        }
    }

    #[test]
    fn test_con_assert_eq() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_con_assert_ne() {
        assert_ne!(2 + 2, 5);
    }

    #[test]
    fn test_assert_float() {
        assert_eq!(0.1 + 0.2, 0.3);
    }
}
