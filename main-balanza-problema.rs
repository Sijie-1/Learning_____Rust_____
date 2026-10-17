// Este problema se conoce como el de los 9 objetos y la balanza

fn ordinal(posicion: usize) -> &'static str {
    match posicion {
        1 => "primer",
        2 => "segundo",
        3 => "tercer",
        4 => "cuarto",
        5 => "quinto",
        6 => "sexto",
        7 => "séptimo",
        8 => "octavo",
        9 => "noveno",
        _ => "desconocido",
    }
}

fn evaluar_grupo(grupo: &[f64], posicion: usize) {
    let (indice, peso) = if grupo[0] == grupo[1] {
        (posicion + 3, grupo[2])
    } else if grupo[0] > grupo[1] {
        (posicion + 2, grupo[1])
    } else {
        (posicion + 1, grupo[0])
    };

    println!(
        "El objeto menos pesado está en el {} puesto",
        ordinal(indice)
    );
    println!("Peso: {}", peso);
}

fn main() {
    let objetos: [f64; 9] = [
        1.0, 0.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ];

    let (objetos1, resto) = objetos.split_at(3);
    let (objetos2, objetos3) = resto.split_at(3);

    let total1: f64 = objetos1.iter().sum();
    let total2: f64 = objetos2.iter().sum();

    if total1 == total2 {
        evaluar_grupo(objetos3, 6);
    } else if total1 > total2 {
        evaluar_grupo(objetos2, 3);
    } else {
        evaluar_grupo(objetos1, 0);
    }

    println!("{:?}", objetos);
}
