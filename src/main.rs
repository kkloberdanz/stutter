use std::io;
use std::io::Write;

enum Input {
    Quit,
    Command(String),
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Clone, Debug)]
enum Atom {
    Nil,
    Num(i64),
    Id(String),
}

#[derive(Clone, Debug)]
enum AST {
    Leaf(Atom),
    Branch(Op, Vec<AST>),
}

fn prompt_user(prompt: &String) -> Input {
    print!("{}", prompt);
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("failed to read stdin");
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
        _ => None
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
    for c in cmd.chars() {
        match c {

            ')' => {
                if tok.len() > 0 {
                    tokens.push(to_token(&tok));
                    tok = String::new();
                }
                tokens.push(to_token(&String::from(")")));
            },

            '(' => {
                if tok.len() > 0 {
                    tokens.push(to_token(&tok));
                    tok = String::new();
                }
                tokens.push(to_token(&String::from("(")));
            },

            ' ' => {
                if tok.len() > 0 {
                    tokens.push(to_token(&tok));
                    tok = String::new();
                }
            },

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

fn parse(tokens: &Vec<Token>) -> AST {
    let mut stack = Vec::new();
    let mut tree = AST::Leaf(Atom::Nil);
    for tok in tokens.iter() {
        match tok {
            Token::Rparen => {
                let mut list = Vec::new();
                while let Some(t) = stack.pop() {
                    match t {
                        Token::Lparen => break,
                        _ => {
                            println!("adding to tree: {:?}", t);
                            match token_to_atom(&t) {
                                Some(atom) => list.push(AST::Leaf(atom)),
                                None => {
                                    list.push(tree);
                                    let op = token_to_op(&t);
                                    let branch = AST::Branch(op, list);
                                    println!("branch = {:?}", branch);
                                    tree = branch;
                                    list = Vec::new();
                                }
                            }
                        }
                    }
                }
                println!();
            },
            _ => stack.push(tok.clone())
        }
    }
    tree
}

fn main() {
    let prompt = String::from("> ");
    loop {
        // Read
        let cmd = prompt_user(&prompt);
        match cmd {
            Input::Command(s) => {

                // Eval
                let tokens = lex(&s);
                match tokens {
                    Ok(t) => {
                        println!("{:?}", t);
                        let tree = parse(&t);
                        println!("{:?}", tree);
                    },
                    Err(e) => println!("{}", e),
                }
                //let result = eval(tree);
            }
            Input::Quit => break,
        };


        // Print

        // Loop
    }
}
