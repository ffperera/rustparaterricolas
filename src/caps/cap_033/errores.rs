use rand::Rng;

#[derive(Debug)]
enum ErrorNave {
    ErrorMotor,
    ErrorNav,
    ErrorSoporte,
    ErrorComm,
    ErrorSistema,
    ErrorCombustible(i32),

    ErrorIO(std::io::Error),
}

impl std::fmt::Display for ErrorNave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorNave::ErrorMotor => write!(f, "error en motores"),
            ErrorNave::ErrorNav => write!(f, "error en navegación"),
            ErrorNave::ErrorSoporte => write!(f, "error en soporte"),
            ErrorNave::ErrorComm => write!(f, "error en comunicaciones"),
            ErrorNave::ErrorSistema => write!(f, "error en sistemas"),
            ErrorNave::ErrorCombustible(porcentaje) => {
                write!(f, "falta combustible {}%", porcentaje)
            }
            ErrorNave::ErrorIO(io) => write!(f, "error IO: {}", io),
        }
    }
}

impl std::error::Error for ErrorNave {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ErrorNave::ErrorIO(io) => Some(io),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ErrorNave {
    fn from(err: std::io::Error) -> ErrorNave {
        ErrorNave::ErrorIO(err)
    }
}

/*
fn check_motores() -> Result<(), String> {
    Ok(())
}

fn check_navegacion()-> Result<(), String> {
    Ok(())
}

fn check_soporte()-> Result<(), String> {
    Ok(())
}

fn check_comunicaciones()-> Result<(), String> {
    Ok(())
}

fn check_sistemas()-> Result<(), String> {
    Ok(())
}

fn check_combustible() -> Result<i32, String> {
    Ok(85)
}
*/

fn check_motores() -> Result<(), ErrorNave> {
    let mut rng = rand::rng();
    if rng.random_range(1..100) < 10 {
        return Err(ErrorNave::ErrorMotor);
    }

    Ok(())
}

fn check_navegacion() -> Result<(), ErrorNave> {
    let mut rng = rand::rng();
    if rng.random_range(1..100) > 80 {
        return Err(ErrorNave::ErrorNav);
    }

    Ok(())
}

fn check_soporte() -> Result<(), ErrorNave> {
    let mut rng = rand::rng();
    if rng.random_range(1..100) > 85 {
        return Err(ErrorNave::ErrorSoporte);
    }

    Ok(())
}

fn check_comunicaciones() -> Result<(), ErrorNave> {
    let mut rng = rand::rng();
    if rng.random_range(1..100) < 20 {
        return Err(ErrorNave::ErrorComm);
    }

    Ok(())
}

fn check_sistemas() -> Result<(), ErrorNave> {
    let mut rng = rand::rng();
    if rng.random_range(1..100) > 80 {
        return Err(ErrorNave::ErrorSistema);
    }

    Ok(())
}

fn check_combustible() -> Result<i32, ErrorNave> {
    let mut rng = rand::rng();
    let nivel = 100 - rng.random_range(1..6);
    if nivel < 96 {
        return Err(ErrorNave::ErrorCombustible(nivel));
    }

    Ok(nivel)
}

/*
fn checklist_nave_01() -> Result<(), String> {

    if let Err(err) = check_motores() {
        return Err(err);
    }

    if let Err(err) = check_navegacion() {
        return Err(err);
    }

    if let Err(err) = check_soporte() {
        return Err(err);
    }

    if let Err(err) = check_comunicaciones() {
        return Err(err);
    }

    if let Err(err) = check_sistemas() {
        return Err(err);
    }

    Ok(())
}


fn checklist_nave_02() -> Result<(), String> {

    check_motores()?;
    check_navegacion()?;
    check_soporte()?;
    check_comunicaciones()?;
    check_sistemas()?;

    Ok(())? operator
}

fn checklist_nave_03() -> Result<(), String> {

    let combustible = check_combustible()?;

    if combustible < 98 {
        return Err(format!("Combustible al {}%",
                    combustible)
        );
    }

    check_motores()?;
    check_navegacion()?;
    check_soporte()?;
    check_comunicaciones()?;
    check_sistemas()?;

    Ok(())
}



fn checklist_nave_04() -> Result<(), ErrorNave> {

    let combustible = check_combustible()?;

    if combustible < 98 {
        return Err(ErrorNave::ErrorCombustible(combustible));
    }

    check_motores()?;
    check_navegacion()?;
    check_soporte()?;
    check_comunicaciones()?;
    check_sistemas()?;

    Ok(())
}

*/

fn checklist_nave_05() -> Result<(), ErrorNave> {
    check_combustible()?;
    check_motores()?;
    check_navegacion()?;
    check_soporte()?;
    check_comunicaciones()?;
    check_sistemas()?;

    Ok(())
}

fn checklist_nave_06() -> Result<(), Box<dyn std::error::Error>> {
    check_combustible()?;
    check_motores()?;
    check_navegacion()?;
    check_soporte()?;
    check_comunicaciones()?;
    check_sistemas()?;

    // posibles errores de entrada-salida

    Ok(())
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Gestión de errores");
    println!("--------------------");
    println!();

    /*
        if let Err(err) = checklist_nave_01() {
            println!("Se suspende lanzamiento: {}", err);
        } else {
            println!("Preparados para la cuenta atrás.");
        }

        if let Err(err) = checklist_nave_02() {
            println!("Se suspende lanzamiento: {}", err);
        } else {
            println!("Preparados para la cuenta atrás.");
        }

        if let Err(err) = checklist_nave_03() {
            println!("Se suspende lanzamiento: {}", err);
        } else {
            println!("Preparados para la cuenta atrás.");
        }



        match checklist_nave_04() {

            Ok(_) => println!("Cuenta atrás"),
            Err(e) => {

                print!("Detectado error en: ");

                match e {
                    ErrorNave::ErrorMotor => println!("Motor"),
                    ErrorNave::ErrorNav   => println!("Navegación"),
                    ErrorNave::ErrorSoporte => println!("Soporte"),
                    ErrorNave::ErrorComm => println!("Comunicaciones"),
                    ErrorNave::ErrorSistema => println!("Sistema"),
                    ErrorNave::ErrorCombustible(porcentaje) => {
                        println!("Combustible {}%", porcentaje);
                    },
                }
            }
        }
    */

    match checklist_nave_05() {
        Ok(_) => println!("Cuenta atrás"),
        Err(e) => {
            println!("Detectado error en: {} ", e);
        }
    }

    match checklist_nave_06() {
        Ok(_) => println!("Cuenta atrás"),
        Err(e) => {
            println!("Detectado error en: {} ", e);
            if e.source().is_some() {
                println!("Origen: {}", e.source().unwrap());
            }
        }
    }
}
