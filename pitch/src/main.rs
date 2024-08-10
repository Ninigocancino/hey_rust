use std::io;

fn main() {
    println!("{}", "__".repeat(15));

    let mut adj_1 = String::new();
    let mut verbo_1 = String::new();
    let mut organizacion = String::new();
    let mut verbo_2 = String::new();
    let mut sustantivo_plural = String::new();

    println!("Ingresa un adjetivo: ");
    io::stdin().read_line(&mut adj_1).expect("Error al leer entrada");
    let adj_1 = adj_1.trim();

    println!("Ingresa un verbo: ");
    io::stdin().read_line(&mut verbo_1).expect("Error al leer entrada");
    let verbo_1 = verbo_1.trim();

    println!("Ingresa una organización: ");
    io::stdin().read_line(&mut organizacion).expect("Error al leer entrada");
    let organizacion = organizacion.trim();

    println!("Ingresa otro verbo: ");
    io::stdin().read_line(&mut verbo_2).expect("Error al leer entrada");
    let verbo_2 = verbo_2.trim();

    println!("Agrega un sustantivo en plural: ");
    io::stdin().read_line(&mut sustantivo_plural).expect("Error al leer entrada");
    let sustantivo_plural = sustantivo_plural.trim();

    println!(" ");

    let madlib = format!(
        "¡Programar es tan adj {}! siempre me emociona porque me encanta {} problemas. ¡Aprende a {} con {} y alcanza tus {}!",
        adj_1, verbo_1, verbo_2, organizacion, sustantivo_plural
    );

    println!(" ");
    println!("{}", madlib);
}