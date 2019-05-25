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

use std::io;
use std::io::Write;

enum Input {
    Quit,
    Command(String),
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Null,
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
}

#[derive(Clone, Debug, PartialEq)]
enum Atom {
    Num(i64),
    Id(String),
    Nil,
}

#[derive(Clone, Debug, PartialEq)]
enum AST {
    Leaf(Atom),
    Branch(Op, Vec<AST>),
}

#[derive(Clone, Debug, PartialEq)]
enum Production {
    Tree(AST),
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

fn token_to_atom(tok: &Token) -> Option<Atom> {
    match tok {
        Token::Id(s) => Some(Atom::Id(s.to_string())),
        Token::Num(i) => Some(Atom::Num(*i)),
        _ => None,
    }
}

fn token_to_op(tok: &Token) -> Op {
    match tok {
        Token::Plus => Op::Add,
        Token::Minus => Op::Sub,
        Token::Times => Op::Mul,
        Token::Slash => Op::Div,
        _ => panic!("invalid op: {:?}", tok),
    }
}

fn lex(cmd: &String) -> Result<Vec<Token>, &str> {
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
        Err("Invalid syntax, trailing characters after final ')'")
    } else {
        Ok(tokens)
    }
}

fn parse(tokens: &Vec<Token>) -> Result<AST, String> {
    let mut stack = Vec::new();
    for tok in tokens.iter() {
        //println!("token = {:?}", tok);
        match tok {
            Token::Rparen => {
                let mut list = Vec::new();
                while let Some(v) = stack.pop() {
                    //println!("1 stack == {:?}", stack);
                    match v {
                        Production::Tok(t) => match t {
                            Token::Lparen => {
                                break;
                            }
                            _ => {
                                //println!("adding to tree: {:?}", t);
                                match token_to_atom(&t) {
                                    Some(atom) => list.push(AST::Leaf(atom)),
                                    None => {
                                        let op = token_to_op(&t);
                                        list.reverse();
                                        let branch = AST::Branch(op, list);
                                        //println!("branch = {:?}", branch);
                                        let top = stack.pop().unwrap();
                                        if top
                                            != Production::Tok(Token::Lparen)
                                        {
                                            return Err(String::from(
                                                "expected '('",
                                            ));
                                        }
                                        //println!("top = {:?}", top);
                                        stack.push(Production::Tree(branch));
                                        //println!("2 stack == {:?}", stack);
                                        break;
                                    }
                                }
                            }
                        },
                        Production::Tree(tree) => {
                            if stack.len() == 0 && tokens.len() == 0 {
                                println!("returning tree early");
                                return Ok(tree);
                            } else {
                                //println!("pushing to list: {:?}", tree);
                                list.push(tree);
                            }
                        }
                    }
                }
                //println!();
            }
            _ => stack.push(Production::Tok(tok.clone())),
        }
    }
    //println!("stack.len() == {}", stack.len());
    //println!("3 stack == {:#?}", stack);
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

fn apply_op(op: &Op, acc: &i64, operand: &i64) -> i64 {
    match op {
        Op::Add => acc + operand,
        Op::Sub => acc - operand,
        Op::Div => acc / operand,
        Op::Mul => acc * operand,
    }
}

fn reduce(op: &Op, list: &Vec<StutterObject>) -> StutterObject {
    let mut acc = list[0].clone();
    for so in list[1..].iter() {
        match (&mut acc, so) {
            (StutterObject::Atom(acc_atom), StutterObject::Atom(a)) => {
                let (acc_val, operand) = match (acc_atom, a) {
                    (Atom::Num(n1), Atom::Num(n2)) => (n1, n2),
                    _ => panic!("unhandled StutterObject: {:?}", a),
                };
                let acc_update = apply_op(op, &acc_val, operand);
                acc = StutterObject::Atom(Atom::Num(acc_update))
            }
            _ => panic!("incompatible types: {:?}, {:?}", acc, so),
        }
    }
    acc.clone()
}

fn eval(ast: &AST) -> Result<StutterObject, String> {
    //Ok(StutterObject::Nil)
    match ast {
        AST::Branch(op, xs) => {
            println!("op = {:?}", op);
            let v = xs.to_vec();
            let resolved: Vec<StutterObject> =
                v.iter().map(|x| eval(x).unwrap()).collect();
            println!("resolved = {:?}", resolved);
            let ans = reduce(op, &resolved);
            Ok(ans)
        }
        AST::Leaf(atom) => {
            println!("atom = {:?}", atom);
            Ok(StutterObject::Atom(atom.clone()))
        }
    }
}

fn run(cmd: &String) -> Result<StutterObject, String> {
    let tokens = lex(&cmd)?;
    let tree = parse(&tokens)?;
    let result = eval(&tree)?;
    Ok(result)
}

fn main() {
    let prompt = String::from("> ");
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
            Input::Quit => break,
        };

        // Loop
    }
}

// test = (+ 1 2 3 (- 4 5) (* (- 6 7) 8))
