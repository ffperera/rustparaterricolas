pub fn run() {
    println!();
    println!("--------------------");
    println!("Strings");
    println!("--------------------");
    println!();

    // contactenar ---------

    let saludo = "Saludos".to_owned();
    let terricolas = saludo + ", terrícolas";

    // 'saludo' ya no está disponible
    // 'terricolas' es el nuevo propietario (String)
    println!("{}", terricolas);

    // 'saludo' es propietario del contenido
    let mut saludo = "Saludos".to_owned();
    saludo = saludo + ", terrícolas";
    // 'saludo' vuelve a ser propietario
    // del contenido tras la operación

    println!("{}", saludo);

    let mut saludo = "Saludos".to_owned();
    let exclama = "!!";
    saludo = saludo + "," + "terrícolas" + exclama;

    println!("{}", saludo);

    // push_str
    let mut saludo = "Saludos".to_owned();
    let exclama = "!!";

    saludo.push_str(",");
    saludo.push_str("terricolas");
    saludo.push_str(exclama);

    println!("{}", saludo);

    // format
    let mut saludo = "Saludos".to_owned();
    let exclama = "!!";

    saludo = format!("{}{}{}", saludo, ", terrícolas", exclama);

    println!("{}", saludo);

    // - `trim()` devuelve una nueva cadena sin espacios en blanco al principio y al final.
    // - `trim_start()` devuelve una nueva cadena sin espacios en blanco al principio.
    // - `trim_end()` devuelve una nueva cadena sin espacios en blanco al final.
    // - `trim_matches('x')` devuelve una nueva cadena sin los caracteres `'x'` al principio y al final.
    // - `trim_start_matches('x')` devuelve una nueva cadena sin los caracteres `'x'` al principio.
    // - `trim_end_matches('x')` devuelve una nueva cadena sin los caracteres `'x'` al final.

    let saludo: &'static str = "   Saludos      ";

    println!("{}", saludo);
    println!("{}", saludo.trim());
    println!("{}", saludo.trim_start());
    println!("{}", saludo.trim_end());
    println!("{}", saludo.trim_matches(' '));
    println!("{}", saludo.trim_start_matches(' '));
    println!("{}", saludo.trim_end_matches(' '));

    let saludo: String = "    Saludos     ".to_owned();

    println!("{}", saludo);
    println!("{}", saludo.trim());
    println!("{}", saludo.trim_start());
    println!("{}", saludo.trim_end());
    println!("{}", saludo.trim_matches(' '));
    println!("{}", saludo.trim_start_matches(' '));
    println!("{}", saludo.trim_end_matches(' '));

    // - `len()` devuelve el número de bytes que ocupa la cadena o el slice.
    // - `chars()` devuelve un iterador sobre los caracteres (`char`) de la cadena (UTF-8).
    // - `chars().count()` devuelve el número de caracteres de la cadena (UTF-8).
    // - `bytes()` devuelve un iterador sobre los bytes (`u8`) de la cadena.
    // - `is_empty()` devuelve true si la cadena está vacía.
    // - `contains("search")` devuelve true si la cadena contiene el texto indicado.
    // - `find("search")` devuelve `Some(posicion)` la posición del texto indicado o `None` si no lo encuentra.
    // - `starts_with("search")` devuelve true si la cadena comienza con el texto indicado.
    // - `ends_with("search")` devuelve true si la cadena termina con el texto indicado.
    // - `replace("search", "replace")` devuelve una nueva cadena con el texto reemplazado.
    // - `replacen("search", "replace", n)` devuelve una nueva cadena con el texto reemplazado `n` veces.
    //
    let saludo: String = "Saludos".to_owned();

    println!("{}", saludo);
    // Saludos
    println!("{}", saludo.len());
    // 7    println!("{}", saludo.chars().count());
    println!("{}", saludo.chars().count());
    // 7
    println!("{}", saludo.chars().nth(3).unwrap());
    // u
    println!("{}", saludo.chars().last().unwrap());
    // s
    println!("{}", saludo.bytes().nth(3).unwrap());
    // 117
    println!("{}", saludo.bytes().last().unwrap());
    // 115
    println!("{}", saludo.is_empty());
    // false
    println!("{}", saludo.contains("Saludos"));
    // true

    // regex

    use regex::Regex;

    let patron = Regex::new(r"\d+").unwrap();

    let texto = "Detectadas 17 naves";

    let match_expr = patron.find(texto).unwrap();
    println!("{}", match_expr.as_str());
    // 17

    // literales de cadena

    let literal = "
        Los efectos \"secundarios\" de
        aprender Rust son...
    ";

    println!("{}", literal);

    //        Los efectos 'secundarios' de
    //        aprender Rust son...

    let literal = "\
        Los efectos \"secundarios\" de \
        aprender Rust son... \
    ";

    println!("{}", literal);

    // Los efectos "secundarios" de aprender Rust son...

    let literal = r"
Los efectos secundarios \n de \
aprender Rust son...
    ";

    println!("{}", literal);

    // Los efectos secundarios \n de
    // aprender Rust son...

    let literal = r#"
Los efectos "secundarios" de \
aprender Rust son... \n
    "#;

    println!("{}", literal);

    // Los efectos "secundarios" de \
    // aprender Rust son... \n

    let literal = r##"
Los efectos "secundarios"# de  
aprender Rust son... \n
    "##;

    println!("{}", literal);

    // Los efectos "secundarios"# de
    // aprender Rust son... \n

    // importar texto externo
    let literal = include_str!("../../../data/saludo.txt");

    println!("{}", literal);
}
