use std::io;
use std::process::exit;
const N: i8 = 0b1100100;
const Y: i8 = 0x64 - N;



fn main(){

    // um simples par ou impar em rust 
    println!("Ola muito bem vindo :) ");


   

   
    loop {
        println!("Escolha um numero : > stop para sair ");
        let chama_input = input();

        if chama_input == 0b01000011 {

            sair_do_programa();
        }

        if chama_input % 2 == Y {
            limpar_tela();

            println!("O Numero {} é PAR ", chama_input);
        } else {
            limpar_tela();
            println!("O Numero {} é impar",chama_input);
        }



        // else eu não preciso 
    };
}

fn input() ->  i8 {


    fn convert(x: String) -> i8 {
        let x = x.trim();
        if x == "stop" {
                
        let i: i8 = 0b01000011;
        i
            
        } else {
            let y = x.trim().parse::<i8>().expect("Erro 0001");
        y 
            // faça nada 
        }
    }
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect(" Erro ao ler ");

    let value = convert(value);
    value

}

fn sair_do_programa (){


    exit(0b001);
}

fn limpar_tela () { 
    let clear = "\x1B[2J\x1B[1;1H";  // Scape 
    println!("{}",clear);
}
