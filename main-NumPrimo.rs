use std::io;

fn main() {
    println!("Por favor, ingrese un número para verificar si es primo o no:");
    let mut n: String = String::new();
    io::stdin().read_line(&mut n).expect("Error al leer la linea");
    let n: u64 = n.trim().parse().expect("Error al transformar el número");

    let mut es_primo = n >= 2;
    let mut i = 2;
    while i * i <= n && es_primo {
        if n % i == 0 {
            es_primo = false;
        }
        i += 1;
    }

    if es_primo {
        println!("primo");
    } else {
        println!("No primo");
    }
}
