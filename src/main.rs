// This file is part of Stutter.
//
// Stutter is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Stutter is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Stutter.  If not, see <https://www.gnu.org/licenses/>.

use rayon::prelude::*;
use std::io;
use std::io::Write;

enum Input {
    Quit,
    None,
    Command(String),
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Lparen,
    Rparen,
    Plus,
    Minus,
    Times,
    Slash,
    Num(i64),
    Id(String),
}

#[derive(Clone, Debug, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Func(String),
}

#[derive(Clone, Debug, PartialEq)]
enum Atom {
    Num(i64),
    Id(String),
}

#[derive(Clone, Debug, PartialEq)]
enum AST {
    Leaf(Atom),
    Branch(Op, Vec<AST>),
}

#[derive(Clone, Debug, PartialEq)]
enum ParseTree {
    Leaf(Token),
    Branch(Op, Vec<ParseTree>),
}

#[derive(Clone, Debug, PartialEq)]
enum Production {
    Tree(ParseTree),
    Tok(Token),
}

#[derive(Clone, Debug, PartialEq)]
enum StutterObject {
    Atom(Atom),
}

fn prompt_user(prompt: &String) -> Input {
    print!("{}", prompt);
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read stdin");
    let as_string = user_input.trim().to_string();

    match as_string.as_ref() {
        "q" => Input::Quit,
        "" => Input::None,
        _ => Input::Command(as_string),
    }
}

fn to_token(s: &String) -> Token {
    if let Ok(t) = s.parse::<i64>() {
        Token::Num(t)
    } else {
        match s.as_ref() {
            "(" => Token::Lparen,
            ")" => Token::Rparen,
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Times,
            "/" => Token::Slash,
            _ => Token::Id(s.to_string()),
        }
    }
}

fn token_to_atom(tok: &Token) -> Result<Atom, String> {
    match tok {
        Token::Id(s) => Ok(Atom::Id(s.to_string())),
        Token::Num(i) => Ok(Atom::Num(*i)),
        _ => Err(format!("token: {:?} does not form a valid atom", tok)),
    }
}

fn token_to_op(tok: &Token) -> Result<Op, String> {
    match tok {
        Token::Plus => Ok(Op::Add),
        Token::Minus => Ok(Op::Sub),
        Token::Times => Ok(Op::Mul),
        Token::Slash => Ok(Op::Div),
        Token::Id(s) => Ok(Op::Func(s.to_string())),
        _ => Err(format!("invalid op: {:?}", tok)),
    }
}

fn lex(cmd: &String) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut tok = String::new();
    let mut in_comment = false;
    for c in cmd.chars() {
        if in_comment {
            continue;
        }
        match c {
            ';' => in_comment = true,

            '\n' => in_comment = false,

            ')' => {
                if tok.len() > 0 {
                    tokens.push(to_token(&tok));
                    tok = String::new();
                }
                tokens.push(to_token(&String::from(")")));
            }

            '(' => {
                if tok.len() > 0 {
                    tokens.push(to_token(&tok));
                    tok = String::new();
                }
                tokens.push(to_token(&String::from("(")));
            }

            ' ' => {
                if tok.len() > 0 {
                    tokens.push(to_token(&tok));
                    tok = String::new();
                }
            }

            _ => {
                tok.push(c);
            }
        }
    }

    if tok.len() > 0 {
        Err(format!("Invalid syntax, token not matched: {:?}", tok))
    } else {
        Ok(tokens)
    }
}

fn push_tree(
    mut stack: Vec<Production>,
    mut list: Vec<ParseTree>,
) -> Result<Vec<Production>, String> {
    println!("list: {:?}", list);
    let op_option = list.pop();
    match op_option {
        Some(op_leaf) => {
            match op_leaf {
                // TODO: fix this for lambdas
                ParseTree::Branch(_, _) => {
                    return Err(String::from(
                        "syntax error, expecting op not tree: {:?}",
                    ));
                }
                ParseTree::Leaf(op_tok) => {
                    let op = token_to_op(&op_tok)?;
                    list.reverse();
                    let branch = ParseTree::Branch(op, list);
                    stack.push(Production::Tree(branch));
                    return Ok(stack);
                }
            }
        }
        None => return Err(String::from("syntax error")),
    }
}

fn parse(tokens: &Vec<Token>) -> Result<ParseTree, String> {
    let mut stack = Vec::new();
    println!("tokens: {:?}", tokens);
    for tok in tokens.iter() {
        println!("stack: {:?}", stack);
        match tok {
            Token::Rparen => {
                let mut list: Vec<ParseTree> = Vec::new();
                while let Some(v) = stack.pop() {
                    println!("v: {:?}", v);
                    match v {
                        Production::Tok(t) => match t {
                            Token::Lparen => {
                                stack = push_tree(stack, list)?;
                                break;
                            }
                            _ => list.push(ParseTree::Leaf(t)),
                        },
                        Production::Tree(tree) => {
                            list.push(tree);
                        }
                    }
                }
            }
            _ => stack.push(Production::Tok(tok.clone())),
        }
    }
    if stack.len() == 1 {
        let top = stack[0].clone();
        match top {
            Production::Tree(t) => Ok(t),
            Production::Tok(tok) => Err(format!(
                "syntax error, ended with unmatched token: {:?}",
                tok
            )),
        }
    } else {
        Err(String::from("syntax error, failed to parse"))
    }
}

fn apply_op(op: &Op, acc: &i64, operand: &i64) -> Result<i64, String> {
    match op {
        Op::Add => Ok(acc + operand),
        Op::Sub => Ok(acc - operand),
        Op::Div => Ok(acc / operand),
        Op::Mul => Ok(acc * operand),
        Op::Func(name) => Err(format!("func not yet implemented, got: {:?}", name)),
    }
}

fn reduce(
    op: &Op,
    list: &Vec<StutterObject>,
) -> Result<StutterObject, String> {
    let mut acc = list[0].clone();
    for so in list[1..].iter() {
        match (&mut acc, so) {
            (StutterObject::Atom(acc_atom), StutterObject::Atom(a)) => {
                let (acc_val, operand) = match (acc_atom, a) {
                    (Atom::Num(n1), Atom::Num(n2)) => (n1, n2),
                    _ => {
                        return Err(format!(
                            "unknown object: {:?}",
                            a
                        ))
                    }
                };
                let acc_update = apply_op(op, &acc_val, operand)?;
                acc = StutterObject::Atom(Atom::Num(acc_update))
            }
        }
    }
    Ok(acc)
}

/*
fn eval(ast: &AST) -> Result<StutterObject, String> {
    match ast {
        AST::Branch(op, xs) => {
            let v = xs.to_vec();
            let resolved: Vec<StutterObject> =
                v.par_iter().map(|x| eval(x).unwrap()).collect();
            let ans = reduce(op, &resolved)?;
            Ok(ans)
        }
        AST::Leaf(atom) => Ok(StutterObject::Atom(atom.clone())),
    }
}
*/

fn eval(tree: &ParseTree) -> Result<StutterObject, String> {
    match tree {
        ParseTree::Branch(op, xs) => {
            let v = xs.to_vec();
            let resolved: Vec<StutterObject> =
                v.par_iter().map(|x| eval(x).unwrap()).collect();
            let ans = reduce(op, &resolved)?;
            Ok(ans)
        },
        ParseTree::Leaf(tok) => {
            let atom = token_to_atom(&tok)?;
            Ok(StutterObject::Atom(atom))
        },
    }
}

fn run(cmd: &String) -> Result<StutterObject, String> {
    let tokens = lex(&cmd)?;
    let tree = parse(&tokens)?;
    println!("tree: {:#?}", tree);
    let result = eval(&tree)?;
    Ok(result)
}

fn main() {
    let prompt = String::from("λ ");
    loop {
        // Read
        let cmd = prompt_user(&prompt);
        match cmd {
            Input::Command(s) => {
                // Eval
                let result = run(&s);

                // Print
                match result {
                    Ok(r) => println!("{:?}", r),
                    Err(e) => println!("error: {}", e),
                }
            }
            Input::None => continue,
            Input::Quit => break,
        };

        // Loop
    }
}

// test = (+ 1 2 3 (- 4 5) (* (- 6 7) 8))
