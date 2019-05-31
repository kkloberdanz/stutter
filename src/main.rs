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
use rpds::HashTrieMap;
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
    Let,
    Num(i64),
    Dec(f64),
    Id(String),
}

#[derive(Clone, Debug, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Let,
    Func(String),
}

#[derive(Clone, Debug, PartialEq)]
enum StutterObject {
    Num(i64),
    Dec(f64),
    Id(String),
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
    } else if let Ok(t) = s.parse::<f64>() {
        Token::Dec(t)
    } else {
        match s.as_ref() {
            "(" => Token::Lparen,
            ")" => Token::Rparen,
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Times,
            "/" => Token::Slash,
            "let" => Token::Let,
            _ => Token::Id(s.to_string()),
        }
    }
}

fn token_to_stutterobject(tok: &Token) -> Result<StutterObject, String> {
    match tok {
        Token::Id(s) => Ok(StutterObject::Id(s.to_string())),
        Token::Num(i) => Ok(StutterObject::Num(*i)),
        Token::Dec(f) => Ok(StutterObject::Dec(*f)),
        _ => Err(format!("token: {:?} does not form a valid atom", tok)),
    }
}

fn token_to_op(tok: &Token) -> Result<Op, String> {
    match tok {
        Token::Plus => Ok(Op::Add),
        Token::Minus => Ok(Op::Sub),
        Token::Times => Ok(Op::Mul),
        Token::Slash => Ok(Op::Div),
        Token::Let => Ok(Op::Let),
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

fn push_production(
    mut stack: Vec<Production>,
    mut list: Vec<ParseTree>,
) -> Result<Vec<Production>, String> {
    let op_option = list.pop();
    match op_option {
        Some(op_leaf) => {
            match op_leaf {
                // TODO: fix this for lambdas
                ParseTree::Branch(_, _) => {
                    Err(String::from(
                        "syntax error, expecting op not tree: {:?}",
                    ))
                }
                ParseTree::Leaf(op_tok) => {
                    let op = token_to_op(&op_tok)?;
                    list.reverse();
                    let branch = ParseTree::Branch(op, list);
                    stack.push(Production::Tree(branch));
                    Ok(stack)
                }
            }
        }
        None => Err(String::from("syntax error")),
    }
}

fn parse(tokens: &Vec<Token>) -> Result<ParseTree, String> {
    let mut stack = Vec::new();
    for tok in tokens.iter() {
        match tok {
            Token::Rparen => {
                let mut list: Vec<ParseTree> = Vec::new();
                while let Some(v) = stack.pop() {
                    match v {
                        Production::Tok(t) => match t {
                            Token::Lparen => {
                                stack = push_production(stack, list)?;
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

fn lookup_env(
    obj: &StutterObject,
    env: &HashTrieMap<String, StutterObject>,
) -> Result<StutterObject, String> {
    match obj {
        StutterObject::Id(variable_name) => match env.get(variable_name) {
            Some(value) => Ok(value.clone()),
            None => Err(format!("{} not in scope", variable_name)),
        },
        _ => Ok(obj.clone()),
    }
}

fn apply_op(
    op: &Op,
    acc: &StutterObject,
    operand: &StutterObject,
    env: &HashTrieMap<String, StutterObject>,
) -> Result<StutterObject, String> {
    let resolved_operand = lookup_env(&operand, &env)?;
    let resolved_acc = lookup_env(&acc, &env)?;
    match (resolved_acc, resolved_operand) {
        (StutterObject::Num(n1), StutterObject::Num(n2)) => match op {
            Op::Add => Ok(StutterObject::Num(n1 + n2)),
            Op::Sub => Ok(StutterObject::Num(n1 - n2)),
            Op::Div => Ok(StutterObject::Num(n1 / n2)),
            Op::Mul => Ok(StutterObject::Num(n1 * n2)),
            _ => Err(format!("{:?} not implemented", op)),
        },
        (StutterObject::Dec(f1), StutterObject::Dec(f2)) => match op {
            Op::Add => Ok(StutterObject::Dec(f1 + f2)),
            Op::Sub => Ok(StutterObject::Dec(f1 - f2)),
            Op::Div => Ok(StutterObject::Dec(f1 / f2)),
            Op::Mul => Ok(StutterObject::Dec(f1 * f2)),
            _ => Err(format!("{:?} not implemented", op)),
        },
        _ => {
            let msg = format!(
                "type error: ({:?} {:?} {:?}) not supported",
                op,
                lookup_env(&acc, &env)?,
                lookup_env(&operand, &env)?
            );
            Err(msg)
        }
    }
}

fn reduce(
    op: &Op,
    list: &Vec<StutterObject>,
    env: &HashTrieMap<String, StutterObject>,
) -> Result<StutterObject, String> {
    let mut acc = list[0].clone();
    for operand in list[1..].iter() {
        acc = apply_op(op, &acc, &operand, env)?;
    }
    Ok(acc)
}

fn eval(
    tree: &ParseTree,
    env: &HashTrieMap<String, StutterObject>,
) -> Result<StutterObject, String> {
    match tree {
        ParseTree::Branch(op, xs) => {
            let v = xs.to_vec();
            let resolved: Result<Vec<StutterObject>, String> =
                v.par_iter().map(|expr| eval(&expr, &env)).collect();

            match op {
                Op::Add | Op::Sub | Op::Mul | Op::Div => {
                    reduce(op, &resolved?, &env)
                }
                Op::Func(name) => {
                    Err(format!("funcitons not implemented, got {:?}", name))
                }
                Op::Let => {
                    if xs.len() != 3 {
                        let msg =
                            format!(
                            "syntax error, expecting (let VAR VALUE IN_EXPR) \
                             assignment expression: ({:?} {:?})", op, xs);
                        Err(msg)
                    } else {
                        let variable = &xs[0];
                        let value = eval(&xs[1], &env)?;
                        let expr = &xs[2];
                        match variable {
                            ParseTree::Leaf(tok) => match tok {
                                Token::Id(name) => {
                                    eval(expr,
                                         &env.insert(name.clone().to_string(),
                                                     value.clone()))
                                }
                                _ => Err(format!(
                                    "error: invalid type for variable: {:?}",
                                    variable
                                )),
                            },
                            _ => Err(format!(
                                "error: invalid type for variable: {:?}",
                                variable
                            )),
                        }
                    }
                }
            }
        }
        ParseTree::Leaf(tok) => {
            token_to_stutterobject(&tok)
        }
    }
}

fn run(cmd: &String) -> Result<StutterObject, String> {
    let tokens = lex(&cmd)?;
    let tree = parse(&tokens)?;
    let env = HashTrieMap::new();
    let result = eval(&tree, &env)?;
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
