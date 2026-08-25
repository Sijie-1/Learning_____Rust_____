use std::io;

fn main() {
    let mut arre: [f64; 3] = [0.0; 3];
    let mut nombre: String = String::new();
    let mut nota_valida: [bool; 3] = [false; 3];

    println!("Por favor, ingrese su nombre:");
    io::stdin()
        .read_line(&mut nombre)
        .expect("Error al leer la linea");

    nombre = nombre.trim().to_string();

    for i in 0..arre.len() {
        println!("Ingrese la nota N{}", i+1);
        let mut nota: String = String::new();
        io::stdin()
            .read_line(&mut nota)
            .expect("Error al leer la linea");

        let nota: f64 = nota
            .trim()
            .parse()
            .expect("Error al transformar el número");

        if nota >= 0.0 && nota <= 10.0 {
            arre[i] = nota;
            nota_valida[i] = true;
        } else {
            println!("Por favor, ingrese una nota entre el rango 0-10");
            break;
        }
    }
    if nota_valida[0] == true && nota_valida[1] == true && nota_valida[2] == true{
        println!("");
        recibir_el_arrgelo(arre, nombre);
    }
}

fn recibir_el_arrgelo (arre: [f64; 3], nombre:String) {
    let arre_sum:f64 = arre[0] + arre[1] + arre[2]; 
    let arre_prom: f64 = arre_sum / 3.0;
    let mut aprobado: bool = false;
    
    if arre_prom >= 7.0 {
        aprobado = true;
    }
    
    let tupla: (String, f64, bool) = (nombre, arre_prom, aprobado);
    println!("Nombre del estudiante: {}", tupla.0);
    println!("Promedio final con {}", tupla.1);
    if aprobado{
        println!("Aprobado")
    } else {
        println!("Reprobado")
    }
}
