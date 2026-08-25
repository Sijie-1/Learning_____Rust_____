use std::io;

fn main() {
    const TARIFA_POR_HORA: f64 = 2.50;
    const DESCUENTO_MIEMBRO: f64 = 0.20;

    let mut horas_estacionado = String::new();

    println!("Ingrese las horas estacionadas:");
    io::stdin()
        .read_line(&mut horas_estacionado)
        .expect("Error al leer la linea");

    let horas_estacionado: f64 = match horas_estacionado.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Por favor, ingrese solo números"),
    };

    if horas_estacionado == 0.0 {
        println!("No hay cobro");
    } else if horas_estacionado > 12.0 {
        println!("Estadía larga");
    }

    let c_base: f64 = horas_estacionado * TARIFA_POR_HORA;

    loop {
        let mut es_miembro = String::new();

        println!("¿Usted tiene un pase de membresía? Responda solo con SI o NO:");
        io::stdin()
            .read_line(&mut es_miembro)
            .expect("Error al leer la linea");

        let es_miembro_txt = es_miembro.trim();

        if es_miembro_txt != "SI" && es_miembro_txt != "NO" {
            println!("Entrada no válida, intente de nuevo");
            continue;
        } else if es_miembro_txt == "SI" {
            let costo_final = c_base * (1.0 - DESCUENTO_MIEMBRO);
            println!("Horas estacionadas: {horas_estacionado}, Estado de membresia: {es_miembro_txt}");
            println!("Costo total a pagar: ${:.2}", costo_final);
            break;
        } else {
            println!("Horas estacionadas: {horas_estacionado}, Estado de membresia: {es_miembro_txt}");
            println!("Costo total a pagar: ${:.2}", c_base);
            break;
        }
    }
}
