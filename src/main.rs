use std::io::prelude::*;
use std::fs::File;
use std::iter::Peekable;

#[derive(Debug)]
enum Cmd {Move(i32), Set(i32), Output, Input, Loop(Vec<Cmd>)}

macro_rules! count {
    ($iter:ident, $ch:expr, $cmd:path, $step:expr) => {{
        let mut n = $step;
        while let Some(&$ch)=$iter.peek() {$iter.next();n+=$step}
        Some($cmd(n))
    }}
}

fn make_program<I>(iter: &mut Peekable<I>) -> Vec<Cmd>
    where I: Iterator<Item = u8>
{
    let mut prog = Vec::new();
    while let Some(b) = iter.next() {
        if let Some(cmd) = match b {
            b'>' => count!(iter, b'>', Cmd::Move,  1),
            b'<' => count!(iter, b'<', Cmd::Move, -1),
            b'+' => count!(iter, b'+', Cmd::Set,   1),
            b'-' => count!(iter, b'-', Cmd::Set,  -1),
            b'.' => Some(Cmd::Output),
            b',' => Some(Cmd::Input),
            b'[' => Some(Cmd::Loop(make_program(iter))),
            b']' => break,
            _ => None
        } {prog.push(cmd)}
    }
    prog
}

fn run_program(prog: &Vec<Cmd>, data: &mut [u8], ptr: &mut usize) {
    for cmd in prog {
        match *cmd {
            Cmd::Move(n) => *ptr = ((*ptr as i32) + n) as usize,
            Cmd::Set(n) => data[*ptr] = ((data[*ptr] as i32) + n) as u8,
            Cmd::Output => print!("{}", data[*ptr] as char),
            Cmd::Input =>
                std::io::stdin().read_exact(&mut data[*ptr..*ptr+1])
                .expect("STDIO error"),
            Cmd::Loop(ref p) => while data[*ptr] != 0 {
                run_program(p, data, ptr)
            },
        }
    }
}

fn main() {
    let filename = std::env::args()
        .nth(1).expect("Input file must be specified");
    let mut iter = File::open(filename)
        .expect("Can't open file").bytes()
        .map(|r| r.unwrap_or(b' ')).peekable();
    let prog = make_program(&mut iter);
    let mut data = vec![0 as u8; 32 * 1024];
    run_program(&prog, &mut data, &mut (16 * 1024));
}
