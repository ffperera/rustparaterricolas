pub fn run() {
    println!();
    println!("--------------------");
    println!("Casting de datos primitivos");
    println!("--------------------");
    println!();

    let big = 240u8;
    let small = big as i8;

    // unsigned a signed
    // reinterpretación: 240 : -16
    println!("{} : {}", big, small);

    let big = 1240u16;
    let small = big as u8;

    // trunacamiento: 1240 : 216
    println!("{} : {}", big, small);

    // sin pérdida
    println!("{} : {}", 255u8, 255u8 as u16);

    // sin pérdida
    println!("{} : {}", 120i8, 120i8 as i32);

    // sin pérdida
    println!("{} : {}", 11.473f32, 11.473f32 as f64);

    // sin pérdida: inf : inf
    println!("{} : {}", f64::INFINITY as f32, f32::INFINITY);

    // pérdida de precisión:  22.0000001 : 22
    println!("{} : {}", 22.0000001f64, 22.0000001f64 as f32);

    // booleano a entero
    println!("{} : {}", true, true as i32);
    println!("{} : {}", false, false as u8);

    // entero a booleano a través de comparación
    // println!("{} : {}", 1, 1 == 1);

    println!("{} : {}", 'ñ', 'ñ' as u32);
    println!("{} : {}", 'ü', 'ü' as u32);
}

#[cfg(test)]
mod testing {

    #[test]
    fn test_casting_u8_to_i8() {
        let big = 240u8;
        let small = big as i8;
        assert_eq!(small, -16);
    }

    #[test]
    fn test_casting_u16_to_u8() {
        let big = 1240u16;
        let small = big as u8;
        assert_eq!(small, 216);
    }

    #[test]
    fn test_casting_sin_perdida_u8_u16() {
        let small = 255u8;
        let big = small as u16;
        assert_eq!(big, 255u16);
    }

    #[test]
    fn test_casting_sin_perdida_i8_i32() {
        let small = 120i8;
        let big = small as i32;
        assert_eq!(big, 120i32);
    }

    #[test]
    fn test_casting_sin_perdida_f32_f64() {
        let small = 11.473f32;
        let big = small as f64;

        // test if diff is lower than delta: 0.0000000001
        assert!(big - 11.473f64 < 0.000000001);
    }

    #[test]
    fn test_infinity_f32_f64() {
        let inf64 = f64::INFINITY;
        let inf32 = inf64 as f32;

        assert_eq!(inf32, f32::INFINITY);
    }

    #[test]
    fn test_casting_con_perdida_precision_float() {
        let diff = f32::EPSILON;

        assert!((22.0000001f32 - 22.0000001f64 as f32) < diff);
    }
}
