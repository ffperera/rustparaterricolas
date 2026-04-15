use rand::Rng;

#[derive(Debug)]
enum ErrorNave {
    ErrorMotor,
    ErrorNav,
    ErrorSoporte,
    ErrorComm,
    ErrorSistema,
    ErrorCombustible(i32),
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
        }
    }
}

impl std::error::Error for ErrorNave {}

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

fn error_entrada_salida() -> Result<(), std::io::Error> {
    let f = std::fs::File::open("no_existe.txt")?;
    _ = f;
    Ok(())
}

fn checklist_nave_06() -> Result<(), Box<dyn std::error::Error>> {
    // estas funciones pueden generar
    // error de tipo ErrorNave
    check_combustible()?;
    check_motores()?;
    check_navegacion()?;
    check_soporte()?;
    check_comunicaciones()?;
    check_sistemas()?;

    // esta función puede generar
    // error de tipo std::io::Error
    error_entrada_salida()?;

    Ok(())
}

pub fn run() {
    println!();
    println!("--------------------");
    println!("Gestión de errores - dynamic dispatch");
    println!("--------------------");
    println!();

    match checklist_nave_06() {
        Ok(_) => println!("Todos los sistemas funcionando"),
        Err(e) => {
            if e.is::<ErrorNave>() {
                println!("Detectado error en: {}", e);
            }

            if e.is::<std::io::Error>() {
                println!("Error de entrada-salida: {}", e);
            }
        }
    }
}
