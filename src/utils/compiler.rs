use std::{fs, process::Command};

use crate::utils::parser::tokenize;

use super::parser::Token;

pub struct Compiler {
    pub ssa_id: usize,
}

impl Compiler {
    fn next(&mut self, base: &str) -> String {
        let name = format!("%{}{}", base, self.ssa_id);
        self.ssa_id += 1;
        name
    }
    pub fn compile(&mut self, tokens: &[Token], os: Option<String>, arch: Option<String>) {
        let mut body = String::new();
        for token in tokens {
            match token {
                Token::ShiftRight => {
                    let cur_ptr = self.next("cur_ptr");
                    let next_ptr = self.next("next_ptr");

                    let inst = format!(
                        r#"
                        ; >
                        {cur_ptr} = load ptr, ptr %head
                        {next_ptr} = getelementptr i8, ptr {cur_ptr}, i64 1
                        store ptr {next_ptr}, ptr %head
                        "#
                    );
                    body += &inst;
                }

                Token::ShiftLeft => {
                    let cur_ptr = self.next("cur_ptr");
                    let next_ptr = self.next("next_ptr");

                    let inst = format!(
                        r#"
                        ; >
                        {cur_ptr} = load ptr, ptr %head
                        {next_ptr} = getelementptr i8, ptr {cur_ptr}, i64 -1
                        store ptr {next_ptr}, ptr %head
                        "#
                    );
                    body += &inst;
                }

                Token::Increment => {
                    let cur_ptr = self.next("cur_ptr");
                    let val = self.next("val");
                    let inc = self.next("inc");
                    let inst = format!(
                        r#"
                        ; +
                        {cur_ptr} = load ptr, ptr %head              ; get pointer to current cell
                        {val} = load i8, ptr {cur_ptr}                ; load byte
                        {inc} = add i8 {val}, 1                       ; increment
                        store i8 {inc}, ptr {cur_ptr}                ; store it back
                    "#
                    );
                    body += &inst;
                }
                Token::Decrement => todo!(),
                Token::Output => {
                    let cur_ptr = self.next("cur_ptr");
                    let val = self.next("val");
                    let ext = self.next("ext");
                    let fmt_ptr = self.next("fmt_ptr");
                    let inst = format!(
                        r#"
                        {cur_ptr} = load ptr, ptr %head           ; load current head pointer
                        {val} = load i8, ptr {cur_ptr}             ; load byte at *head
                        {ext} = zext i8 {val} to i32               ; zero-extend to i32
                        {fmt_ptr} = getelementptr [3 x i8], ptr @fmt, i64 0, i64 0 ; get format string
                        call i32 (ptr, ...) @printf(ptr {fmt_ptr}, i32 {ext})
                    "#
                    );
                    body += &inst;
                }
                Token::Input => todo!(),
                Token::LoopStart => todo!(),
                Token::LoopEnd => todo!(),
            }
        }
        to_elf(&format!("{}{}{}", headers(), body, footer()), os, arch);
    }
}

pub fn headers() -> String {
    r#"
    @tape = global [30000 x i8] zeroinitializer
    @fmt = private constant [3 x i8] c"%c\00"

    declare i32 @printf(ptr, ...)
    declare i32 @putchar(i32)
    declare i32 @getchar()
    declare void @exit(i32)

    define i32 @main() {
    entry:
        ; Allocate mutable head pointer
        %head = alloca ptr
        %tape_ptr = getelementptr [30000 x i8], ptr @tape, i64 0, i64 0
        store ptr %tape_ptr, ptr %head
        "#
    .to_owned()
}

pub fn footer() -> String {
    r#"
        call void @exit(i32 0)
        ret i32 0
    }
    "#
    .to_owned()
}

pub fn to_elf(src: &str, os: Option<String>, arch: Option<String>) {
    // Step 1: Write IR to .ll file
    fs::write("gen.ll", src).expect("failed to write gen.ll");

    // Step 2: Construct target triple if arch & os are provided
    let target_triple = match (&arch, &os) {
        (Some(arch), Some(os)) => {
            let arch = normalize_arch(arch);
            let os = normalize_os(os);
            format!("{arch}-unknown-{os}")
        }
        _ => String::new(), // No target: compile for host
    };

    // Step 3: Build clang args
    let mut args = vec!["gen.ll", "-o", "output", "-O3", "-static"];
    if !target_triple.is_empty() {
        args.push("-target");
        args.push(&target_triple);
    }

    // Step 4: Call clang
    let status = Command::new("clang")
        .args(&args)
        .status()
        .expect("failed to execute clang");

    assert!(status.success(), "LLVM compilation failed");
}

fn normalize_arch(arch: &str) -> &str {
    match arch {
        "amd64" => "x86_64",
        "386" => "i386",
        "arm64" | "aarch64" => "aarch64",
        "arm" => "armv7a",
        _ => arch,
    }
}

fn normalize_os(os: &str) -> &str {
    match os {
        "macos" => "darwin",
        "windows" => "windows",
        "android" | "termux" => "linux-android",
        "linux" => "linux-gnu",
        _ => os,
    }
}

pub fn compile(file: std::path::PathBuf, os: Option<String>, arch: Option<String>) {
    println!("Compiling {}...", file.display());
    match tokenize(file) {
        Ok(tokens) => {
            let mut compiler = Compiler { ssa_id: 0 };
            compiler.compile(&tokens, os, arch);
            println!();
        }
        Err(err) => println!("{err}"),
    }
}
