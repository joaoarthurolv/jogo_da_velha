use std::io;

fn main(){
    let mut tabuleiro: [char; 9] = [' '; 9];
    let mut jogador_x = true;
    
    mostrar_nome_do_jogo();
    mostrar_tabuleiro(&tabuleiro);

    loop {
        if jogador_x {
            println!("Vez do jogador X:");
        } else {
            println!("Vez do jogador O:");
        }

        let mut entrada = String::new();

        io::stdin().read_line(&mut entrada)
                .expect("Falha ao ler jogada.");
        
        match entrada.trim().parse::<usize>() {
            Err(_) => {
                println!("Input inválido!");
                continue;
            }
            Ok(mut entrada) => {
                entrada = entrada - 1;
                
                if tabuleiro[entrada] != ' ' {
                    println!("Posição já ocupada!");
                    mostrar_tabuleiro(&tabuleiro);
                    continue;
                }

                if jogador_x {
                    tabuleiro[entrada] = 'X';
                    
                    let bool = checar_vitoria(&tabuleiro, 'X');

                    if bool {
                        mostrar_tabuleiro(&tabuleiro);
                        println!("O jogador X ganhou!");
                        break;
                    }
                    jogador_x = false;
                } else {
                    tabuleiro[entrada] = 'O';
                    
                    let bool = checar_vitoria(&tabuleiro, 'O');

                    if bool {
                        mostrar_tabuleiro(&tabuleiro);
                        println!("O jogador O ganhou!");
                        break;
                    }
                    jogador_x = true;
                }
            }
        }
        mostrar_tabuleiro(&tabuleiro);
        
    }
}

fn mostrar_tabuleiro(tabuleiro: &[char;9]){
    println!("  {}  |  {}  |  {}  ", tabuleiro[0], tabuleiro[1], tabuleiro[2]);
    println!("------------------");
    println!("  {}  |  {}  |  {}  ", tabuleiro[3], tabuleiro[4], tabuleiro[5]);
    println!("------------------");
    println!("  {}  |  {}  |  {}  ", tabuleiro[6], tabuleiro[7], tabuleiro[8]);
}

fn mostrar_nome_do_jogo(){
    println!("JOGO DA VELHA!");
}

fn checar_vitoria(tabuleiro: &[char;9], entrada: char) -> bool {
    // Checando vitória de forma horizontal
    for i in (0..8).step_by(3) {
        if entrada == tabuleiro[i] && entrada == tabuleiro[i+1] && entrada == tabuleiro[i+2] {
            return true;
        }
    }
    
    // Checando vitória de forma vertical
    for i in 0..3 {
        if entrada == tabuleiro[i] && entrada == tabuleiro[i+3] && entrada == tabuleiro[i+6] {
            return true;
        }
    }

    // Checando vitória de forma diagonal
    if entrada == tabuleiro[0] && entrada == tabuleiro[4] && entrada == tabuleiro[8] {
        return true;
    }

    if entrada == tabuleiro[2] && entrada == tabuleiro[4] && entrada == tabuleiro[6] {
        return true;
    }

    return false;    
}