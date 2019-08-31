use std::io;
use std::string::String;
use std::{thread, time};
use std::str::FromStr;

fn saisie() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.trim().to_string()
        }
        _ => {"".to_string()}
    }
}
fn input_byte() -> Option<u8> {
	let mut entree = String::new();						
	match io::stdin().read_line(&mut entree) {		
		Ok(_) => {								
			match u8::from_str(entree.trim()) {
				Ok(nombre) => Some(nombre),		
				Err(_) => {				
					println!("A number between 0 and 255, please...");		
					None						
				}
			}
		},
		_ => {										
				println!("Erreur à la saisie");
				None
			}
	}
}

fn code_analyse(code: &Vec<char>) -> Vec<usize> {
    // Analyse du code pour trouver les paires de crochet
    let mut bracket_list: Vec<usize> = vec![0; code.len()];
    let mut bracket_count: usize = 0;
    for (i, op) in code.iter().enumerate() {
        if *op == '[' {
            bracket_count += 1;
            bracket_list[i] = bracket_count;
        } else if *op == ']' {
            bracket_list[i] = bracket_count;
            bracket_count -= 1;
        }
    }
    bracket_list

}

fn main() {
    const MAX_MEMORY: usize= 30;
    let mut entry = String::new();
    let mut index: usize = 0;
    let mut stack: Vec<u8> = vec![0;30];
    let mut code: Vec<char> = Vec::new();
    println!("-----> BrainFuck <-----\n       v1.1      "); 
    entry = saisie();
    for op in entry.chars() {
        code.push(op);
    }
    let bracket_list: Vec<usize> = code_analyse(&code);

    
    let mut i: usize = 0;
    while i < code.len() {
        //thread::sleep(time::Duration::from_millis(100));
        println!("{:?}", stack);
        match code[i] {
            '>' => {
                i += 1;
                index += 1;
            },
            '<' => {
                i += 1;
                if index == 0 {
                    println!("Warning ! Memory Error");
                } else {
                    index -=1;
                }
            },
            '.' => {
                println!("Output : {}", stack[index] as char);
                i +=1;
            },
            ',' => {
                println!("Input : ");
                match input_byte() {
                    Some(nombre) => {
                        stack[index] = nombre;
                    }
                    None => {
                       println!("Entry error");
                    }
                }
                i += 1;
            },
            '+' => {
                i += 1;
                if stack[index] == 255 {
                    index = 0;
                } else {
                    stack[index] += 1;
                }
            },
            '-' => {    
                i += 1;
                if stack[index] == 0 {
                    stack[index] = 255;
                } else {
                    stack[index] -= 1;
                }
            },
            '[' => {
                if stack[index] != 0 {
                    // on avance d'un cran pour executer l'op suivant
                    i += 1;
                } else {
                    //si 0 alors on incrémente jusqu'au crochet fermant correspondant
                    let bracket_score: usize = bracket_list[i];
                    i +=1;
                    while bracket_list[i] != bracket_score {
                        i +=1;
                    }
                    // quand on a attient le bon crochet fermant, on passe à l'op juste après
                    i +=1;
                }
            },
            ']' => {
                // aller au précédent [
                let bracket_score: usize = bracket_list[i];
                i -= 1;
                while bracket_list[i] != bracket_score {
                    i -=1;
                }
            },
            _ => {}
        }
    }
    
    println!("{:?}", stack);
}
