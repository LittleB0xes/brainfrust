use std::string::String;
use std::{env, io, thread, time};
use std::str::FromStr;
use crossterm::{terminal, ClearType};

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
fn screen_output(stack: &Vec<u8>, index: usize, output: &String ) {
    let mut stack_line = String::new();
    let mut space = String::new();
    let mut index_line = String::new();
    terminal().clear(ClearType::All);
    for (i,cell) in stack.iter().enumerate() {
        stack_line = stack_line + &cell.to_string() + &' '.to_string();
        if *cell < 10 {
            space = " ".to_string();
        } else if *cell < 100 {
            space = "  ".to_string();
        } else {
            space = "   ".to_string();

        }
        if i == index {
            index_line = index_line+ &'^'.to_string() + &space;
        } else {
            index_line = index_line + &' '.to_string() + &space;
        }
    }
    println!("BrainFuck Interpreter by LittleBoxes"); 
    println!("{}\n{}\nOutput : {}", stack_line, index_line, output);
    //println!("{}", index_line);
    //println!("Output : {}", output);

}
fn interpreter(max_memory: usize, delay: u64) {
    let mut entry = String::new();
    let mut index: usize = 0;
    let mut stack: Vec<u8> = vec![0;max_memory];
    let mut code: Vec<char> = Vec::new();
    let mut output =  String::new();
    terminal().clear(ClearType::All);
    println!("BrainFuck Interpreter by LittleBoxes"); 
    entry = saisie();
    for op in entry.chars() {
        code.push(op);
    }
    let bracket_list: Vec<usize> = code_analyse(&code);
    
    let mut i: usize = 0;
    while i < code.len()  {
        thread::sleep(time::Duration::from_millis(delay));
        screen_output(&stack, index, &output);
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
                let letter = stack[index] as char;
                output += &letter.to_string();
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
                    i += 1;
                } else {
                    let bracket_score: usize = bracket_list[i];
                    i +=1;
                    while bracket_list[i] != bracket_score {
                        i +=1;
                    }
                    i +=1;
                }
            },
            ']' => {
                let bracket_score: usize = bracket_list[i];
                i -= 1;
                while bracket_list[i] != bracket_score {
                    i -=1;
                }
            },
            _ => {
                println!("Invalid operator");
                break;
            }
        }
    }
    thread::sleep(time::Duration::from_millis(delay));
    screen_output(&stack, index, &output);
}
fn main() {
    let mut max_memory: usize = 30;
    let mut delay: u64 = 500;
    let args: Vec<String> = env::args().collect();
    for i in 0..args.len() {
       if args[i] == "-m" {
            match usize::from_str(&args[i+1]) {
                Ok(memory) => {
                    max_memory = memory;
                },
                Err(_) => {}
            }
        } else if args[i] == "-d" {
            match u64::from_str(&args[i+1]) {
                Ok(delay_time) => {
                    delay = delay_time;
                },
                Err(_) => {}
            }

        } else if args[i] == "-e" {
            
        }
    
    }
    interpreter(max_memory, delay);
}
