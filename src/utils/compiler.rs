use std::process::Command;

use crate::utils::parser::tokenize;

use super::parser::Token;

pub struct Compiler;

impl Compiler {
    pub fn compile(tokens: &[Token]) {
        let mut body = String::new();
        for token in tokens {
            match token {
                Token::ShiftRight => {
                    let inst = "
                        add     DWORD PTR [rbp-4], 1
                    ";
                    body += inst;
                }
                Token::ShiftLeft => {
                    // TODO: check for underflow, for now we don't care
                    let inst = "
                        sub    DWORD PTR [rbp-4], 1
                    ";
                    body += inst;
                }
                Token::Increment => {
                    let inst = "
                        mov     eax, DWORD PTR [rbp-4]
                        cdqe
                        movzx   eax, BYTE PTR [rbp-30016+rax]
                        lea     edx, [rax+1]
                        mov     eax, DWORD PTR [rbp-4]
                        cdqe
                        mov     BYTE PTR [rbp-30016+rax], dl
                    ";
                    body += inst;
                }
                Token::Decrement => todo!(),
                Token::Output => {
                    let inst = "
                        mov     eax, DWORD PTR [rbp-4]
                        cdqe
                        movzx   eax, BYTE PTR [rbp-30016+rax]
                        movzx   eax, al
                        mov     esi, eax
                        lea     rdi, [rip + fmt]
                        mov     eax, 0
                        call    printf
                    ";
                    body += inst;
                }
                Token::Input => todo!(),
                Token::LoopStart => todo!(),
                Token::LoopEnd => todo!(),
            }
        }
        to_elf(&format!("{}{}{}", headers(), body, footer()));
    }
}

pub fn headers() -> String {
    r#"
    .global main 
    .intel_syntax noprefix
    .section .text
    main:
        push    rbp
        mov     rbp, rsp
        sub     rsp, 30016
        lea     rax, [rbp-30016]
        mov     edx, 30000
        mov     esi, 0
        mov     rdi, rax
        call    memset
        mov     DWORD PTR [rbp-4], 0
        
    "#
    .to_owned()
}

pub fn footer() -> String {
    r#"
        mov     eax, 0
        leave
        ret
    .section .data
        fmt: .asciz "%c"
        
    "#
    .to_owned()
}

pub fn to_elf(src: &str) {
    std::fs::write("./gen.asm", src).expect("failed to generate asm file");
    // as hello_world.asm --64 -o hello_world.o
    let _res = Command::new("as")
        .args(["./gen.asm", "--64", "-o", "gen.o"])
        .output()
        .expect("failed to execute `as` command");
    // gcc -o hello_world.elf -m64 hello_world.o  -nostdlib
    let _res = Command::new("gcc")
        .args(["-o", "output", "-m64", "./gen.o"])
        .output()
        .expect("failed to execute `gcc` command");
}

pub fn compile(file: std::path::PathBuf) {
    println!("Compiling {}...", file.display());
    match tokenize(file) {
        Ok(tokens) => {
            Compiler::compile(&tokens);
            println!();
        }
        Err(err) => println!("{err}"),
    }
}
