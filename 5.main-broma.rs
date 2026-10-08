use std::io;

fn main() {
    println!("¿La licenciada Germania Benavides nos perdonara a todos?");
    println!("1. Si");
    println!("2. No");
    println!("Ingrese un número: ");
    
    let num: u8 = loop {
        let mut input_respuesta: String = String::new();
        io::stdin()
            .read_line(&mut input_respuesta)
            .expect("Error al leer la linea"); 
    
        match input_respuesta.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Error, ingrese solo números")
            }
            
        }
    };
    
    if num == 1 {
        println!("Bueno gracias");
    } else if num == 2 {
        println!("Los matare");
    } else {
        println!("Error, ingrese un numero entre 1-2");
    }
}
