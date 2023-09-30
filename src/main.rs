use std::{
    io::{self, Write},
    str::FromStr,
};

fn read_value<T>(prompt: &str) -> T
where
    T: FromStr,
{
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        match user_input.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("Ingrese un valor válido."),
        }
    }
}

fn main() {
    println!("Hello, world!");

    let (name1, views1, earnings1) = read_user_data("primer user");
    let (name2, views2, earnings2) = read_user_data("segundo user");

    let user1 = impresion_price(earnings1, views1, &name1, "Tech");
    let user2 = impresion_price(earnings2, views2, &name2, "Corralon, antirevenue");

    let parsed_user1 = parsed(&user1);
    let parsed_user2 = parsed(&user2);

    let diff = parsed_user1 - parsed_user2;

    let format_notation = format!("{:e}", diff);

    println!("Diferencia en notacion: {}", format_notation);
    println!("Diferencia en numero: {}", diff);
}

fn read_user_data(user_description: &str) -> (String, f64, f64) {
    let name: String = read_value(&format!("Ingrese nombre del {}:", user_description));
    let views: f64 = read_value(&format!(
        "Ingrese numero de visualizaciones para {}:",
        user_description
    ));
    let earnings: f64 = read_value(&format!("Ingrese ganancias para {}:", user_description));
    (name, views, earnings)
}
fn parsed(numero: &str) -> f64 {
    numero.parse().expect("No se pudo analizar el número uno.")
}
fn impresion_price(amount: f64, amount_imp: f64, name: &str, rubro: &str) -> String {
    let multiplicador = 1_000_000_000.0;
    let rev_per_imp = amount / (amount_imp * multiplicador);
    let notacion_cientifica = format!("{:e}", rev_per_imp);
    println!(
        "{} revenuew: {}, rubro: {}",
        name, notacion_cientifica, rubro
    );
    notacion_cientifica
}
