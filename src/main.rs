use std::string::String;
use std::{env, io, thread, time, fs};
use std::str::FromStr;
use crossterm::{terminal, ClearType};

fn saisie() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.to_string()
        }
        _ => {"".to_string()}
    }
}

fn code_cleaning(contents: String) -> Vec<char> {
    let mut code: Vec<char> = Vec::new();
    for op in contents.chars() {
        match op {
                 '>' | '<' | '+' | '-' | '[' | ']' | '.' | ',' => {
                    code.push(op);
                 },
                 _ => {}
             }
    }
    code
}

fn code_analyse(code: &Vec<char>) -> Vec<usize> {
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

fn screen_output(code: &Vec<char>, n: usize, stack: &Vec<u8>, index: usize, output: &String, exe: bool, display: bool) {
	let title = "______           _        ________          _   
| ___ \\         (_)      / _| ___ \\        | |  
| |_/ /_ __ __ _ _ _ __ | |_| |_/ /   _ ___| |_ 
| ___ \\ '__/ _` | | '_ \\|  _|    / | | / __| __|
| |_/ / | | (_| | | | | | | | |\\ \\ |_| \\__ \\ |_ 
\\____/|_|  \\__,_|_|_| |_|_| \\_| \\_\\__,_|___/\\__|
                                                ";
    let mut stack_line = String::new();
    let mut space: String;
    let mut index_line = String::new();
    let mut pointer_line = String::new();
    let mut code_line = String::new();
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

    for (i, op) in code.iter().enumerate() {
        code_line = code_line + &op.to_string();
        if i == n {
            pointer_line = pointer_line + &'^'.to_string();
        } else {
            pointer_line = pointer_line + &' '.to_string();
        }
    }
    println!("{}\n            a Brainfuck Interpreter Made In Rust\n", title);
    if exe {
        if display {
            println!("\n{}\n{}\n\n{}\n{}", code_line, pointer_line, stack_line, index_line);
        }
        println!("Output : {}", output);
    }
}

fn interpreter(contents: String, max_memory: usize, delay: u64, display: bool) {
    let entry: String;
    let mut index: usize = 0;
    let mut stack: Vec<u8> = vec![0;max_memory];
    let mut code: Vec<char> = Vec::new();
    let mut output =  String::new();
    terminal().clear(ClearType::All);
    if contents.len() == 0 {
         screen_output(&code, 0, &stack, index, &output, false, display);
         println!("Enter your code : ");
         entry = saisie();
         code = code_cleaning(entry);
     } else {
         code = code_cleaning(contents);
    }
    
    let bracket_list: Vec<usize> = code_analyse(&code);
    let mut i: usize = 0;
    while i < code.len()  {
        thread::sleep(time::Duration::from_millis(delay));
        screen_output(&code, i, &stack, index, &output, true, display);
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
                let entry = saisie();
                stack[index] = entry.chars().next().unwrap() as u8;
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
                i += 1;
            }
        }
    }
    thread::sleep(time::Duration::from_millis(delay));
    screen_output(&code, i, &stack, index, &output, true, display);
}
fn main() {
    let mut max_memory: usize = 30;
    let mut delay: u64 = 500;
    let mut display: bool = true;
    let mut contents = String::new();
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
            let filename = &args[i+1];
            contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
            println!("{}", filename);
        } else if args[i] == "-nodisplay" {
            display = false;
        }
    }
    interpreter(contents, max_memory, delay, display);
}
