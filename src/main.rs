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
}

#[derive(Clone, Debug, PartialEq)]
enum AST {
    Leaf(Atom),
    List(Vec<AST>),
    Branch(Op, Vec<AST>),
}

#[derive(Clone, Debug, PartialEq)]
enum Production {
    Tree(AST),
    Tok(Token),
}

#[derive(Clone, Debug, PartialEq)]
enum StutterObject {
    Nil,
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

fn eval(ast: &AST) -> Result<StutterObject, String> {
    //Ok(StutterObject::Nil)
    match ast {
        AST::Branch(op, xs) => {
            println!("op = {:?}", op);
            eval(&AST::List(xs.to_vec()))
        }
        AST::List(v) => {
            let mut rec_v = Vec::new();
            for item in v.iter() {
                println!("item = {:?}", item);
                rec_v.push(eval(&item)?);
            }
            //Ok(AST::List(rec_v))
            Ok(StutterObject::Nil)
        }
        AST::Leaf(atom) => {
            println!("atom = {:?}", atom);
            Ok(StutterObject::Nil)
        }
    }
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
                        println!("tokens = {:?}", t);
                        let tree = parse(&t);
                        match tree {
                            Ok(t) => {
                                println!("ast = {:?}", t);
                                let result = eval(&t);
                                match result {

                                    // Print
                                    Ok(so) => {
                                        println!("retult = {:?}", so);
                                    },
                                    Err(e) => {
                                        println!("runtime error: {:?}", e);
                                    }
                                }
                            },
                            Err(e) => println!("syntax error: {}", e),
                        }
                    }
                    Err(e) => println!("lexical error: {}", e),
                }
            }
            Input::Quit => break,
        };

        // Loop
    }
}

// test = (+ 1 2 3 (- 4 5) (* (/ 6 7) 8))
