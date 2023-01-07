use std::{fs, process, env, io::Write};

mod lex;
mod data_types;
mod parse;
use data_types::*;

static mut DEBUG: bool = false;

// Produce x86 assembly from the program
fn produce_assembly(p: Prog, ass_f: String) -> std::io::Result<()>
{
    let Prog::Prog(f) = p;
    let Func::Func(f_name, stmt) = f;
    let Stmt::Return(expr) = stmt;
    
    let mut file = fs::File::create(ass_f)?;
    file.write_all(format!(".globl {f_name}\n").as_bytes()).expect("Failed to write .globl");
    file.write_all(format!("{f_name}:\n").as_bytes()).expect("Failed to write identifier");
    
    // Interpret the return value and its (possible) unary operators
    let mut next_expr = expr;
    let mut op_stack = Vec::new(); 
    loop {
        match next_expr {
            Expr::UnaryOp(op, exp) => {
                op_stack.push(op);
                next_expr = *exp;
            },
            Expr::IntLiteral(ret_val) => {
                file.write_all(format!("\tmov\t${ret_val}, %rax\n").as_bytes()).expect("Failed to write mov");
                break;
            },
        }
    }

    // Iterate over unary operation stack and write to assembly
    loop {
        match op_stack.pop() {
            Some(UnaryOp::Negation) => file.write_all(format!("\tneg\t%rax\n").as_bytes()).expect("Failed to write neg"),
            Some(UnaryOp::BitComp) => file.write_all(format!("\tnot\t%rax\n").as_bytes()).expect("Failed to write not"),
            Some(UnaryOp::LogNeg) => {
                file.write_all(format!("\tcmp\t$0, %rax\n").as_bytes()).expect("Failed to write cmp");
                file.write_all(format!("\tmov\t$0, %rax\n").as_bytes()).expect("Failed to write mov");
                file.write_all(format!("\tsete\t%al\n").as_bytes()).expect("Failed to write sete");
            },
            None => break, 
        }
    }

    // Return
    file.write_all(format!("\tret\n").as_bytes()).expect("Failed to write ret");

    Ok(())
}

// Print the input program "prettily"
#[allow(dead_code)]
fn pretty_print(p: Prog)
{
    println!("Pretty printing {:?} ...\n", p);

    let Prog::Prog(f) = p;
    let Func::Func(f_name, stmt) = f;
    let Stmt::Return(expr) = stmt;
    let Expr::IntLiteral(ret_val) = expr else {todo!()};

    println!("FUN INT {}:", f_name);
    println!("\tparams: ()");
    println!("\tbody:");
    println!("\t\tRETURN Int<{}>", ret_val);
   
}

fn main() 
{

    let mut path: String = String::from("stages/stage_2/valid/not_zero.c");
    let args: Vec<String> = env::args().collect();
    let ass_f: String = String::from("assembly.s");

    // Not so robust argument reading to automate testing
    // Last argument is always the path
    match args.len() {
        1 => (),
        2 => {
            if args.contains(&String::from("debug")) {
                unsafe {
                    DEBUG = true;
                }
            } else {
                path = args[args.len()-1].clone();
            }
        },
        3 => {
            unsafe {
                DEBUG = true;
            }
            path = args[args.len()-1].clone();
        },
        _ => {
            println!("Invalid argument setup, aborting...");
            process::exit(1);
        },
    }

    let p = parse::parse(lex::lex(path.clone()));

    // Produce the assembly
    match produce_assembly(p, ass_f.clone()) {
        Ok(_) => (),
        _ => {
            println!("Failed to produce assembly file!");
            process::exit(1);
        },
    }

    // Produce final name (location) of binary
    let binary_name = String::from(path.split(".").collect::<Vec<&str>>()[0])+".exe";

    // Produce binary
    process::Command::new("gcc")
                     .arg(ass_f.clone())
                     .arg("-o")
                     .arg(binary_name)
                     .output()
                     .expect("failed to run gcc on assembly file");

    // Delete assembly file
    fs::remove_file(ass_f.clone()).expect("Failed to remove assembly file");

}
