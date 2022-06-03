use std::io;

fn main() {
    // ! === macro
    println!("Advinhe o número!");
    println!("Digite o seu palpite.");

    let mut palpite = String::new();

    io::stdin().read_line(&mut palpite).expect("Falha ao ler entrada");

    println!("Voce dise: {}", palpite);
}
