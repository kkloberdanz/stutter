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
use rpds::Vector;
use std::io;
use std::io::Write;

enum Input {
    Quit,
    None,
    Err(String),
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
    List,
    Index,
    Drop,
    Len,
    Take,
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
    List,
    Index,
    Drop,
    Len,
    Take,
    Func(String),
}

#[derive(Clone, Debug, PartialEq)]
enum StutterObject {
    Num(i64),
    Dec(f64),
    Id(String),
    Lambda(Vec<String>, ParseTree),
    List(Vec<StutterObject>),
    Nil,
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

fn count_chars(s: &String, c: char) -> i64 {
    let mut acc = 0;
    for letter in s.chars() {
        if letter == c {
            acc += 1;
        }
    }
    acc
}

fn prompt_user(prompt: &String) -> Input {
    print!("{}", prompt);
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read stdin");
    let as_string = user_input.trim().to_string();
    if count_chars(&as_string, '(') != count_chars(&as_string, ')') {
        return Input::Err(String::from("unmatched parens"));
    }

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
            "list" => Token::List,
            "take" => Token::Take,
            "index" => Token::Index,
            "drop" => Token::Drop,
            "len" => Token::Len,
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
        Token::List => Ok(Op::List),
        Token::Index => Ok(Op::Index),
        Token::Take => Ok(Op::Take),
        Token::Drop => Ok(Op::Drop),
        Token::Len => Ok(Op::Len),
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
                // TODO: fix this for calling lambdas
                ParseTree::Branch(_, _) => Err(format!(
                    "syntax error, expecting op not tree: {:?}",
                    op_leaf
                )),
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

fn param_to_string(
    param: &ParseTree,
) -> Result<&std::string::String, std::string::String> {
    match param {
        ParseTree::Leaf(tok) => match tok {
            Token::Id(name) => Ok(name),
            _ => Err(format!("error, expecting variable, got {:?}", tok)),
        },
        _ => Err(String::from("error, expecting variable, got tree")),
    }
}

fn get_params(params_as_tree: &ParseTree) -> Result<Vec<String>, String> {
    let mut params: Vec<String> = Vec::new();
    match params_as_tree {
        ParseTree::Branch(op, xs) => {
            let param = match op {
                Op::Func(name) => Ok(name.to_string()),
                _ => Err(String::from("expecting param name")),
            }?;
            params.push(param);
            for param in xs.iter() {
                let resolved_param = param_to_string(&param)?;
                params.push(resolved_param.to_string());
            }
        }
        _ => return Err(String::from("syntax error, expecting params")),
    }
    Ok(params)
}

fn eval_lambda_params(
    name: &String,
    val: &ParseTree,
    env: &HashTrieMap<String, StutterObject>,
) -> Result<(String, StutterObject), String> {
    match &val {
        ParseTree::Branch(tok, params_and_func) => match tok {
            Op::Func(f_name) => {
                if f_name == "lambda" {
                    let params = get_params(&params_and_func[0])?;
                    let func = &params_and_func[1];
                    Ok((
                        name.to_string(),
                        StutterObject::Lambda(params, func.clone()),
                    ))
                } else {
                    Err(String::from(format!(
                        "syntax error 2: expecting lambda, got: {:?}",
                        tok
                    )))
                }
            }
            _ => Err(String::from(format!(
                "syntax error 1: expecting lambda, got: {:?}",
                tok
            ))),
        },
        _ => Ok((name.to_string(), eval(&val, &env)?)),
    }
}

fn handle_let(
    xs: &Vec<ParseTree>,
    env: &HashTrieMap<String, StutterObject>,
) -> Result<StutterObject, String> {
    let expr = &xs[xs.len() - 1];
    let mut new_env = env.clone();
    if xs.len() < 2 {
        return Err(String::from(
            "expecting form of (let (VAR expr)...(expr))",
        ));
    }
    for branch in xs[..xs.len() - 1].iter() {
        let (var, val) = match branch {
            ParseTree::Branch(var_op, val_vec) => match var_op {
                Op::Func(name) => {
                    if val_vec.len() != 1 {
                        Err(format!("syntax error: {:?}", val_vec))
                    } else {
                        match &val_vec[0] {
                            ParseTree::Branch(Op::List, xs) => {
                                let v: Result<Vec<StutterObject>, String> = xs
                                    .par_iter()
                                    .map(|exp| eval(&exp, &env))
                                    .collect();
                                let resolved = v?;
                                let env = env.insert(
                                    name.to_string(),
                                    StutterObject::List(resolved),
                                );
                                return eval(&expr, &env);
                            }
                            _ => eval_lambda_params(&name, &val_vec[0], &env),
                        }
                    }
                }
                _ => Err(String::from("not a variable")),
            },
            _ => Err(String::from("expecting variable assignment")),
        }?;
        new_env = new_env.clone().insert(var.clone(), val.clone());
    }
    eval(&expr, &new_env)
}

fn handle_func(
    name: &String,
    xs: &Vec<ParseTree>,
    env: &HashTrieMap<String, StutterObject>,
) -> Result<StutterObject, String> {
    let func_body = env.get(name);
    match func_body {
        Some(body) => {
            match body {
                StutterObject::Lambda(params, expr) => {
                    let mut new_env = env.clone();
                    // TODO: use resolved instead of xs
                    for (param, arg) in params.iter().zip(xs) {
                        // TODO: multithread this
                        let resolved_arg = eval(&arg, &env)?;
                        new_env = new_env.clone().insert(
                            param.to_string(),
                            resolved_arg,
                        );
                    }
                    eval(&expr, &new_env)
                }
                _ => Ok(body.clone()),
            }
        }
        None => Err(format!("func: {} not in scope", name)),
    }
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

                Op::Func(name) => handle_func(&name, &xs, &env),

                // TODO: use resolved instead of xs here
                Op::Let => handle_let(&xs, &env),
                Op::List => {
                    let v = resolved?;
                    Ok(StutterObject::List(v))
                }
                Op::Index => {
                    let v = resolved?;
                    let i = &v[0];
                    let list = &v[1];
                    match (i, list) {
                        (StutterObject::Num(n), StutterObject::List(l)) =>  {
                            let size: usize = *n as usize;
                            Ok(l[size].clone())
                        }
                        _ => Err(String::from(
                            "type error: \
                             expected form (index NUM LIST)"))
                    }
                }
                Op::Take => {
                    let v = resolved?;
                    let i = &v[0];
                    let list = &v[1];
                    match (i, list) {
                        (StutterObject::Num(n), StutterObject::List(l)) =>  {
                            let size: usize = *n as usize;
                            Ok(StutterObject::List(l[..size].to_vec()))
                        }
                        _ => Err(String::from(
                            "type error: \
                             expected form (take NUM LIST)"))
                    }
                }
                Op::Drop => {
                    let v = resolved?;
                    let i = &v[0];
                    let list = &v[1];
                    match (i, list) {
                        (StutterObject::Num(n), StutterObject::List(l)) =>  {
                            let size: usize = *n as usize;
                            Ok(StutterObject::List(l[size..].to_vec()))
                        }
                        _ => Err(String::from(
                            "type error: \
                             expected form (drop NUM LIST)"))
                    }
                }
                Op::Len => {
                    let v = resolved?;
                    let list = &v[0];
                    match list {
                        StutterObject::List(l) =>  {
                            let len: i64 = l.len() as i64;
                            Ok(StutterObject::Num(len))
                        }
                        _ => Err(String::from(
                            "type error: \
                             expected form (len LIST)"))
                    }
                }
            }
        }
        ParseTree::Leaf(tok) => {
            let obj = token_to_stutterobject(&tok)?;
            match obj {
                StutterObject::Id(_) => lookup_env(&obj, &env),
                _ => Ok(obj),
            }
        }
    }
}

fn run(cmd: &String) -> Result<StutterObject, String> {
    let tokens = lex(&cmd)?;
    if tokens.len() == 0 {
        Ok(StutterObject::Nil)
    } else {
        let tree = parse(&tokens)?;
        let env = HashTrieMap::new();
        let result = eval(&tree, &env)?;
        Ok(result)
    }
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
            Input::Err(e) => {
                println!("lexical error: {}", e);
                continue;
            }
        };

        // Loop
    }
}

// test = (+ 1 2 3 (- 4 5) (* (- 6 7) 8))
