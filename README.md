# Brainfrust
A Brainfuck Interpreter Made in Rust and [Crossterm](https://github.com/crossterm-rs/crossterm) for the terminal rendering.


## Launching arguments
This is a simple Brainfuck interpreter. He accept some argument at execution
* -m : set the stack size of the interprter (ex : brainfuck -m 50,  set a 50 bytes memory). Default value is 30.
* -d : set a time delay between operation (ex : brainfuck -d 100, set a 100ms delay). Default vaulue is 500ms.
* -e : execute an extern brainfuck code file (ex : brainfuck -e hello.bf).
* -nodisplay : hide stack and pointer during the execution.

                                                
                                                

