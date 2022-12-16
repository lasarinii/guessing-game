use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing Game!!!");
    'running: loop {

        //cria o número aleatório
        let rand_num = rand::thread_rng().gen_range(1..=100);
    
        loop {
            println!("Input your guess between 1 and 100: ");

            //atribui um espaço na memória para guess
            let mut guess = String::new();

            //guarda o valor na variavel guess
            io::stdin()
                .read_line(&mut guess)
                .expect("[ERROR]");

            //converte guess de String para u32
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            //compara guess com o número aleatório
            match guess.cmp(&rand_num) {
                Ordering::Less => println!("{guess} is too small!!!"),
                Ordering::Greater => println!("{guess} is too big!!!"),
                Ordering::Equal => {
                    println!("You win with {guess}!!!");
                    break;
                }
            }
        }
        loop {
            println!("Again? [S/N]");

            //condição para jogar novamente
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("[ERROR]");
            let choice = choice.trim();

            if choice == String::from("S") || choice == String::from("s") {
                println!("Running again...");
                break;
            } else if choice == String::from("N") || choice == String::from("n") {
                println!("Breaking...");
                break 'running;
            } else {
                continue;
            }
        }
    }
    
}
