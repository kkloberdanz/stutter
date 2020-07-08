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

extern crate num_bigint;
extern crate num_traits;

use crate::num_bigint::ToBigInt;
use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;
use num_traits::pow;
use rpds::HashTrieMap;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io;
use std::io::Write;

enum Input {
    Quit,
    None,
    Command(String),
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Lparen,      // (
    Rparen,      // )
    Plus,        // +
    Minus,       // -
    Times,       // *
    Slash,       // /
    Percent,     // %
    Pow,         // pow
    Gt,          // >
    Lt,          // <
    Eq,          // =
    Gte,         // >=
    Lte,         // <=
    Let,         // let
    Def,         // def
    List,        // list
    Index,       // index
    Drop,        // drop
    Quote,       // quote
    Append,      // append
    Range,       // range
    Cat,         // cat
    Len,         // len
    Take,        // take
    If,          // if
    Int(BigInt), // Integer literal
    Dec(f64),    // Floating point literal
    Bool(bool),  // Boolean literal
    Id(String),  // identifier (variable name or function name)
}

#[derive(Clone, Debug, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Gt,
    Lt,
    Eq,
    Gte,
    Lte,
    Let,
    Def,
    List,
    Index,
    Drop,
    Quote,
    Append,
    Range,
    Cat,
    Len,
    Take,
    If,
    Func(String),
}

#[derive(Clone, Debug, PartialEq)]
enum StutterObject {
    Nil,
    Int(BigInt),
    Dec(f64),
    Bool(bool),
    Id(String),
    Lambda(Vec<String>, ParseTree),
    List(Vec<StutterObject>),
}

impl fmt::Display for StutterObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            StutterObject::Nil => write!(f, "Nil"),
            StutterObject::Int(i) => write!(f, "{}", i.to_string()),
            StutterObject::Dec(d) => write!(f, "{}", d.to_string()),
            StutterObject::Bool(b) => write!(
                f,
                "{}",
                if *b {
                    String::from("True")
                } else {
                    String::from("False")
                }
            ),
            StutterObject::Id(s) => write!(f, "{}", s),
            StutterObject::Lambda(_l, _tree) => write!(f, "{}", "<lambda>"),
            StutterObject::List(vec) => {
                let mut string_vec = Vec::new();
                for item in vec {
                    string_vec.push(format!("{}", item));
                }
                let string = string_vec.join(" ");
                write!(f, "({})", string)
            }
        }
    }
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
    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read stdin");
        let as_string = &line.trim().to_string();
        let new_input = format!(" {} ", as_string);
        user_input += &new_input;
        if count_chars(&user_input, '(') == count_chars(&user_input, ')') {
            break;
        }
    }
    match user_input.as_ref() {
        " q " => Input::Quit,
        "  " => Input::None,
        _ => Input::Command(user_input),
    }
}

fn to_token(s: &String) -> Token {
    let bytes = s.as_bytes();
    if let Some(t) = BigInt::parse_bytes(bytes, 10) {
        Token::Int(t)
    } else if let Ok(t) = s.parse::<f64>() {
        Token::Dec(t)
    } else if let Ok(t) = s.parse::<bool>() {
        Token::Bool(t)
    } else {
        match s.as_ref() {
            "(" => Token::Lparen,
            ")" => Token::Rparen,
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Times,
            "/" => Token::Slash,
            "%" => Token::Percent,
            "pow" => Token::Pow,
            "<" => Token::Lt,
            ">" => Token::Gt,
            "=" => Token::Eq,
            ">=" => Token::Gte,
            "<=" => Token::Lte,
            "let" => Token::Let,
            "def" => Token::Def,
            "list" => Token::List,
            "take" => Token::Take,
            "if" => Token::If,
            "index" => Token::Index,
            "drop" => Token::Drop,
            "quote" => Token::Quote,
            "append" => Token::Append,
            "range" => Token::Range,
            "cat" => Token::Cat,
            "len" => Token::Len,
            _ => Token::Id(s.to_string()),
        }
    }
}

fn token_to_stutterobject(tok: &Token) -> Result<StutterObject, String> {
    match tok {
        Token::Id(s) => Ok(StutterObject::Id(s.to_string())),
        Token::Int(i) => Ok(StutterObject::Int(i.clone())),
        Token::Dec(f) => Ok(StutterObject::Dec(*f)),
        Token::Bool(b) => Ok(StutterObject::Bool(*b)),
        _ => Err(format!("token: {:?} does not form a valid atom", tok)),
    }
}

fn token_to_op(tok: &Token) -> Result<Op, String> {
    match tok {
        Token::Plus => Ok(Op::Add),
        Token::Minus => Ok(Op::Sub),
        Token::Times => Ok(Op::Mul),
        Token::Slash => Ok(Op::Div),
        Token::Percent => Ok(Op::Mod),
        Token::Pow => Ok(Op::Pow),
        Token::Gt => Ok(Op::Gt),
        Token::Lt => Ok(Op::Lt),
        Token::Eq => Ok(Op::Eq),
        Token::Gte => Ok(Op::Gte),
        Token::Lte => Ok(Op::Lte),
        Token::Let => Ok(Op::Let),
        Token::Def => Ok(Op::Def),
        Token::List => Ok(Op::List),
        Token::Index => Ok(Op::Index),
        Token::Take => Ok(Op::Take),
        Token::If => Ok(Op::If),
        Token::Drop => Ok(Op::Drop),
        Token::Quote => Ok(Op::Quote),
        Token::Append => Ok(Op::Append),
        Token::Range => Ok(Op::Range),
        Token::Cat => Ok(Op::Cat),
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

fn lookup_global_env(
    variable_name: &String,
    global_env: &mut HashMap<String, StutterObject>,
) -> Result<StutterObject, String> {
    match global_env.get(variable_name) {
        Some(value) => Ok(value.clone()),
        None => Err(format!("'{}' not in scope", variable_name)),
    }
}

fn lookup_env(
    obj: &StutterObject,
    env: &HashTrieMap<String, StutterObject>,
    global_env: &mut HashMap<String, StutterObject>,
) -> Result<StutterObject, String> {
    match obj {
        StutterObject::Id(variable_name) => match env.get(variable_name) {
            Some(value) => Ok(value.clone()),
            None => lookup_global_env(&variable_name, global_env),
        },
        _ => Ok(obj.clone()),
    }
}

fn lookup_env_string(
    name: &String,
    env: &HashTrieMap<String, StutterObject>,
    global_env: &mut HashMap<String, StutterObject>,
) -> Result<StutterObject, String> {
    let res = env.get(name);
    match res {
        Some(x) => Ok(x.clone()),
        _ => match global_env.get(name) {
            Some(y) => Ok(y.clone()),
            _ => Err(format!("'{}' is not in scope", name)),
        },
    }
}

fn bigint_to_f64(n: &BigInt) -> Result<f64, String> {
    let opt_f = n.to_f64();
    match opt_f {
        Some(f) => Ok(f),
        None => Err(String::from("failed to represent BigInt as f64")),
    }
}

fn bigint_to_usize(n: &BigInt) -> Result<usize, String> {
    let opt_usize = n.to_usize();
    match opt_usize {
        Some(us) => Ok(us),
        None => Err(format!("failed to represent BigInt as usize: {:?}", n)),
    }
}

fn bigint_to_i64(n: &BigInt) -> Result<i64, String> {
    let opt_usize = n.to_i64();
    match opt_usize {
        Some(us) => Ok(us),
        None => Err(format!("failed to represent BigInt as i64: {:?}", n)),
    }
}

fn usize_to_bigint(n: usize) -> Result<BigInt, String> {
    let opt_bi = n.to_bigint();
    match opt_bi {
        Some(bi) => Ok(bi),
        None => Err(String::from("failed to represent usize as BigInt")),
    }
}

fn i64_to_bigint(n: i64) -> Result<BigInt, String> {
    let opt_bi = n.to_bigint();
    match opt_bi {
        Some(bi) => Ok(bi),
        None => Err(String::from("failed to represent usize as BigInt")),
    }
}

fn apply_op(
    op: &Op,
    acc: &StutterObject,
    operand: &StutterObject,
    env: &HashTrieMap<String, StutterObject>,
    global_env: &mut HashMap<String, StutterObject>,
) -> Result<StutterObject, String> {
    let resolved_operand = lookup_env(&operand, &env, global_env)?;
    let resolved_acc = lookup_env(&acc, &env, global_env)?;
    match (resolved_acc, resolved_operand) {
        (StutterObject::Int(n1), StutterObject::Int(n2)) => match op {
            Op::Add => Ok(StutterObject::Int(n1 + n2)),
            Op::Sub => Ok(StutterObject::Int(n1 - n2)),
            Op::Div => Ok(StutterObject::Dec(
                bigint_to_f64(&n1)? / bigint_to_f64(&n2)?,
            )),
            Op::Mod => Ok(StutterObject::Int(n1 % n2)),
            Op::Pow => Ok(StutterObject::Int(pow(n1, bigint_to_usize(&n2)?))),
            Op::Mul => Ok(StutterObject::Int(n1 * n2)),
            Op::Gt => Ok(StutterObject::Bool(n1 > n2)),
            Op::Lt => Ok(StutterObject::Bool(n1 < n2)),
            Op::Eq => Ok(StutterObject::Bool(n1 == n2)),
            Op::Gte => Ok(StutterObject::Bool(n1 >= n2)),
            Op::Lte => Ok(StutterObject::Bool(n1 <= n2)),
            _ => Err(format!("{:?} not implemented", op)),
        },
        (StutterObject::Dec(f1), StutterObject::Dec(f2)) => match op {
            Op::Add => Ok(StutterObject::Dec(f1 + f2)),
            Op::Sub => Ok(StutterObject::Dec(f1 - f2)),
            Op::Div => Ok(StutterObject::Dec(f1 / f2)),
            Op::Mod => Ok(StutterObject::Dec(f1 % f2)),
            Op::Pow => Ok(StutterObject::Dec(f1.powf(f2))),
            Op::Mul => Ok(StutterObject::Dec(f1 * f2)),
            Op::Gt => Ok(StutterObject::Bool(f1 > f2)),
            Op::Lt => Ok(StutterObject::Bool(f1 < f2)),
            Op::Eq => Ok(StutterObject::Bool(f1 == f2)),
            Op::Gte => Ok(StutterObject::Bool(f1 >= f2)),
            Op::Lte => Ok(StutterObject::Bool(f1 <= f2)),
            _ => Err(format!("{:?} not implemented", op)),
        },
        (StutterObject::Int(n1), StutterObject::Dec(f2)) => match op {
            Op::Add => Ok(StutterObject::Dec((bigint_to_f64(&n1)?) + f2)),
            Op::Sub => Ok(StutterObject::Dec((bigint_to_f64(&n1)?) - f2)),
            Op::Div => Ok(StutterObject::Dec((bigint_to_f64(&n1)?) / f2)),
            Op::Mod => Ok(StutterObject::Dec((bigint_to_f64(&n1)?) % f2)),
            Op::Pow => Ok(StutterObject::Dec((bigint_to_f64(&n1)?).powf(f2))),
            Op::Mul => Ok(StutterObject::Dec((bigint_to_f64(&n1)?) * f2)),
            Op::Gt => Ok(StutterObject::Bool((bigint_to_f64(&n1)?) > f2)),
            Op::Lt => Ok(StutterObject::Bool((bigint_to_f64(&n1)?) < f2)),
            Op::Eq => Ok(StutterObject::Bool((bigint_to_f64(&n1)?) == f2)),
            Op::Gte => Ok(StutterObject::Bool((bigint_to_f64(&n1)?) >= f2)),
            Op::Lte => Ok(StutterObject::Bool((bigint_to_f64(&n1)?) <= f2)),
            _ => Err(format!("{:?} not implemented", op)),
        },
        (StutterObject::Dec(f1), StutterObject::Int(n2)) => match op {
            Op::Add => Ok(StutterObject::Dec(f1 + (bigint_to_f64(&n2)?))),
            Op::Sub => Ok(StutterObject::Dec(f1 - (bigint_to_f64(&n2)?))),
            Op::Div => Ok(StutterObject::Dec(f1 / (bigint_to_f64(&n2)?))),
            Op::Mod => Ok(StutterObject::Dec(f1 % (bigint_to_f64(&n2)?))),
            Op::Pow => Ok(StutterObject::Dec(f1.powf(bigint_to_f64(&n2)?))),
            Op::Mul => Ok(StutterObject::Dec(f1 * (bigint_to_f64(&n2)?))),
            Op::Gt => Ok(StutterObject::Bool(f1 > (bigint_to_f64(&n2)?))),
            Op::Lt => Ok(StutterObject::Bool(f1 < (bigint_to_f64(&n2)?))),
            Op::Eq => Ok(StutterObject::Bool(f1 == (bigint_to_f64(&n2)?))),
            Op::Gte => Ok(StutterObject::Bool(f1 >= (bigint_to_f64(&n2)?))),
            Op::Lte => Ok(StutterObject::Bool(f1 <= (bigint_to_f64(&n2)?))),
            _ => Err(format!("{:?} not implemented", op)),
        },
        _ => {
            let msg = format!(
                "incompatible types: ({:?} {:?} {:?}) not supported",
                op,
                lookup_env(&acc, &env, global_env)?,
                lookup_env(&operand, &env, global_env)?
            );
            Err(msg)
        }
    }
}

fn reduce(
    op: &Op,
    list: &Vec<StutterObject>,
    env: &HashTrieMap<String, StutterObject>,
    global_env: &mut HashMap<String, StutterObject>,
) -> Result<StutterObject, String> {
    let mut acc = list[0].clone();
    for operand in list[1..].iter() {
        acc = apply_op(op, &acc, &operand, env, global_env)?;
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
    global_env: &mut HashMap<String, StutterObject>,
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
        _ => Ok((name.to_string(), eval(&val, &env, global_env, true)?)),
    }
}

fn eval_func(
    name: &String,
    xs: &Vec<ParseTree>,
    env: &HashTrieMap<String, StutterObject>,
    global_env: &mut HashMap<String, StutterObject>,
) -> Result<StutterObject, String> {
    let body = lookup_env_string(&name, &env, global_env)?;
    match body {
        StutterObject::Lambda(params, expr) => {
            let mut new_env = env.clone();
            // TODO: use resolved instead of xs
            for (param, arg) in params.iter().zip(xs) {
                // TODO: multithread this
                let resolved_arg = eval(&arg, &env, global_env, true)?;
                new_env =
                    new_env.clone().insert(param.to_string(), resolved_arg);
            }
            eval(&expr, &new_env, global_env, true)
        }
        _ => Ok(body.clone()),
    }
}

fn eval_let(
    xs: &Vec<ParseTree>,
    env: &HashTrieMap<String, StutterObject>,
    global_env: &mut HashMap<String, StutterObject>,
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
                                    .iter()
                                    .map(|exp| {
                                        eval(&exp, &env, global_env, true)
                                    })
                                    .collect();
                                let resolved = v?;
                                let env = env.insert(
                                    name.to_string(),
                                    StutterObject::List(resolved),
                                );
                                return eval(&expr, &env, global_env, true);
                            }
                            ParseTree::Branch(_func, _sub_expr) => {
                                let value =
                                    eval(&val_vec[0], &env, global_env, true)?;
                                Ok((name.to_string(), value))
                            }
                            _ => eval_lambda_params(
                                &name,
                                &val_vec[0],
                                &env,
                                global_env,
                            ),
                        }
                    }
                }
                _ => Err(String::from("not a variable")),
            },
            _ => Err(String::from("expecting variable assignment")),
        }?;
        new_env = new_env.clone().insert(var.clone(), val.clone());
    }
    eval(&expr, &new_env, global_env, true)
}

fn unpack_string_from_leaf(tree: &ParseTree) -> Result<String, String> {
    match tree {
        ParseTree::Leaf(id) => match id {
            Token::Id(s) => Ok(s.to_string()),
            _ => Err(String::from("expecting function name")),
        },
        ParseTree::Branch(_op, _v) => {
            Err(String::from("expecting function name, got branch"))
        }
    }
}

fn eval_def(
    xs: &Vec<ParseTree>,
    env: &HashTrieMap<String, StutterObject>,
    global_env: &mut HashMap<String, StutterObject>,
) -> Result<(String, StutterObject), String> {
    if xs.len() != 2 {
        return Err(String::from("expecting form of (def VAR EXPR)"));
    }
    let name = unpack_string_from_leaf(&xs[0])?;
    let expr = &xs[1];
    let value = eval(&expr, &env, global_env, true)?;
    Ok((name, value))
}

fn resolve_exprs(
    xs: &Vec<ParseTree>,
    env: &HashTrieMap<String, StutterObject>,
    global_env: &mut HashMap<String, StutterObject>,
) -> Result<Vec<StutterObject>, String> {
    xs.to_vec()
        .iter()
        .map(|expr| eval(&expr, &env, global_env, true))
        .collect()
}

fn eval_branch(
    op: &Op,
    xs: &Vec<ParseTree>,
    env: &HashTrieMap<String, StutterObject>,
    global_env: &mut HashMap<String, StutterObject>,
) -> Result<StutterObject, String> {
    match op {
        Op::Add
        | Op::Pow
        | Op::Sub
        | Op::Mul
        | Op::Div
        | Op::Mod
        | Op::Eq
        | Op::Gt
        | Op::Lt
        | Op::Gte
        | Op::Lte => reduce(
            op,
            &resolve_exprs(&xs, &env, global_env)?,
            &env,
            global_env,
        ),

        Op::Func(name) => eval_func(&name, &xs, &env, global_env),

        Op::Let => eval_let(&xs, &env, global_env),

        Op::Def => {
            let (name, value) = eval_def(&xs, &env, global_env)?;
            global_env.insert(name, value);
            Ok(StutterObject::Nil)
        }

        Op::Quote => {
            if xs.len() != 1 {
                Err(String::from("expecting: (quote ITEM)"))
            } else {
                let v = eval(&xs[0], &env, global_env, false)?;
                Ok(v)
            }
        }

        Op::List => {
            let v = resolve_exprs(&xs, &env, global_env)?;
            Ok(StutterObject::List(v))
        }
        Op::Index => {
            let v = resolve_exprs(&xs, &env, global_env)?;
            let i = &v[0];
            let list = &v[1];
            match (i, list) {
                (StutterObject::Int(n), StutterObject::List(l)) => {
                    let size: usize = bigint_to_usize(&n)?;
                    Ok(l[size].clone())
                }
                _ => Err(String::from(
                    "type error: expected form (index NUM LIST)",
                )),
            }
        }
        Op::Take => {
            let v = resolve_exprs(&xs, &env, global_env)?;
            let i = &v[0];
            let list = &v[1];
            match (i, list) {
                (StutterObject::Int(n), StutterObject::List(l)) => {
                    let size: usize = bigint_to_usize(&n)?;
                    Ok(StutterObject::List(l[..size].to_vec()))
                }
                _ => Err(String::from(
                    "type error: expected form (take NUM LIST)",
                )),
            }
        }
        Op::Drop => {
            let v = resolve_exprs(&xs, &env, global_env)?;
            let i = &v[0];
            let list = &v[1];
            match (i, list) {
                (StutterObject::Int(n), StutterObject::List(l)) => {
                    let big_zero: BigInt = 0.to_bigint().unwrap();
                    if n <= &big_zero {
                        Ok(StutterObject::List(Vec::new()))
                    } else {
                        let size: usize = bigint_to_usize(&n)?;
                        Ok(StutterObject::List(l[size..].to_vec()))
                    }
                }
                _ => Err(String::from(
                    "type error: expected form (drop NUM LIST)",
                )),
            }
        }
        Op::Append => {
            let v = resolve_exprs(&xs, &env, global_env)?;
            let i = &v[0];
            let list = &v[1];
            match list {
                StutterObject::List(l) => {
                    let mut vec = l.clone();
                    vec.push(i.clone());
                    Ok(StutterObject::List(vec))
                }
                _ => Err(String::from(
                    "type error: expected form (append ITEM LIST)",
                )),
            }
        }
        Op::Range => {
            let v = resolve_exprs(&xs, &env, global_env)?;
            let lower_bound = &v[0];
            let upper_bound = &v[1];
            match (lower_bound, upper_bound) {
                (StutterObject::Int(bi1), StutterObject::Int(bi2)) => {
                    if bi1 >= bi2 {
                        return Ok(StutterObject::List(Vec::new()));
                    }
                    let mut vector =
                        Vec::with_capacity(bigint_to_usize(&(bi2 - bi1))?);
                    let i1 = bigint_to_i64(bi1)?;
                    let i2 = bigint_to_i64(bi2)?;
                    for i in i1..i2 {
                        vector.push(StutterObject::Int(i64_to_bigint(i)?));
                    }
                    Ok(StutterObject::List(vector))
                }
                _ => Err(format!(
                    "unsupported types for range: {:?}, {:?}",
                    lower_bound, upper_bound
                )),
            }
        }
        Op::Cat => {
            let v = resolve_exprs(&xs, &env, global_env)?;
            let mut master_vec = Vec::new();
            for list in v.iter() {
                match list {
                    StutterObject::List(vec) => {
                        master_vec.append(&mut vec.clone());
                    }
                    _ => {
                        return Err(format!(
                            "cat: expecting list, got {:?}",
                            list
                        ))
                    }
                }
            }
            Ok(StutterObject::List(master_vec))
        }
        Op::Len => {
            let v = resolve_exprs(&xs, &env, global_env)?;
            let list = &v[0];
            match list {
                StutterObject::List(l) => {
                    let len: BigInt = usize_to_bigint(l.len())?;
                    Ok(StutterObject::Int(len))
                }
                _ => Err(String::from("type error: expected form (len LIST)")),
            }
        }
        Op::If => {
            let exprs = xs.to_vec();
            if exprs.len() != 3 {
                Err(String::from(
                    "expecting form of (if (CONDITION) (EXPR) (EXPR))",
                ))
            } else {
                let condition = eval(&exprs[0], &env, global_env, true)?;
                let true_path = &exprs[1];
                let false_path = &exprs[2];
                let path = match condition {
                    StutterObject::Bool(true) => Ok(true_path),
                    StutterObject::Bool(false) => Ok(false_path),
                    _ => Err(format!(
                        "expecting boolean expression, got {}",
                        condition
                    )),
                }?;
                let resolved = eval(&path, &env, global_env, true)?;
                Ok(resolved)
            }
        }
    }
}

fn params_to_string(params: &ParseTree) -> Result<Vec<String>, String> {
    let mut params_vec = Vec::new();
    match params {
        ParseTree::Branch(op, term_vec) => {
            match op {
                Op::Func(first_term) => params_vec.push(first_term.clone()),
                _ => return Err(String::from("expecting first param")),
            };
            for item in term_vec.iter() {
                match item {
                    ParseTree::Leaf(Token::Id(s)) => {
                        params_vec.push(s.to_string())
                    }
                    _ => return Err(String::from("expecting param list")),
                }
            }
            Ok(params_vec)
        }
        _ => Err(String::from("problem getting params")),
    }
}

fn eval(
    tree: &ParseTree,
    env: &HashTrieMap<String, StutterObject>,
    global_env: &mut HashMap<String, StutterObject>,
    fully_eval_lambda: bool,
) -> Result<StutterObject, String> {
    match tree {
        ParseTree::Branch(op, xs) => match op {
            Op::Func(s) => {
                if fully_eval_lambda && s != "lambda" {
                    eval_branch(&op, &xs, &env, global_env)
                } else {
                    let params = &xs[0];
                    let params_as_string = params_to_string(&params)?;
                    let expr = &xs[1];
                    Ok(StutterObject::Lambda(params_as_string, expr.clone()))
                }
            }
            _ => {
                if fully_eval_lambda {
                    eval_branch(&op, &xs, &env, global_env)
                } else {
                    Err(String::from("could not evaluate branch"))
                }
            }
        },
        ParseTree::Leaf(tok) => {
            let obj = token_to_stutterobject(&tok)?;
            match obj {
                StutterObject::Id(_) => lookup_env(&obj, &env, global_env),
                _ => Ok(obj),
            }
        }
    }
}

fn run(
    cmd: &String,
    global_env: &mut HashMap<String, StutterObject>,
) -> Result<StutterObject, String> {
    let tokens = lex(&cmd)?;
    if tokens.len() == 0 {
        Ok(StutterObject::Nil)
    } else {
        let tree = parse(&tokens)?;
        let env = HashTrieMap::new();
        let result = eval(&tree, &env, global_env, true)?;
        Ok(result)
    }
}

fn read_stdlib(
    global_env: &mut HashMap<String, StutterObject>,
) -> Result<StutterObject, String> {
    let home = env!("HOME");
    let filename = format!("{}/.stutter/stdlib.lisp", home);
    let err_msg = format!(
        "FAILED TO READ STDLIB, PLEASE PUT STDLIB.LISP IN {}",
        filename
    );
    let contents = fs::read_to_string(filename).expect(&err_msg);

    let mut num_lparen = 0;
    let mut num_rparen = 0;
    let mut expr = String::new();
    let mut in_comment = false;
    for c in contents.chars() {
        if c == '(' {
            num_lparen += 1;
            if !in_comment {
                expr.push(c);
            }
        } else if c == ')' {
            num_rparen += 1;
            if !in_comment {
                expr.push(c);
            }
            if num_lparen == num_rparen {
                num_lparen = 0;
                num_rparen = 0;
                let _result = run(&expr, global_env)?;
                expr = String::new();
            }
        } else if c == ';' {
            in_comment = true;
        } else if c == '\n' {
            in_comment = false;
        } else {
            if !in_comment {
                expr.push(c);
            }
        }
    }
    Ok(StutterObject::Nil)
}

fn main() {
    let prompt = String::from("λ ");
    let mut global_env = HashMap::new();
    read_stdlib(&mut global_env).unwrap();
    loop {
        // Read
        let cmd = prompt_user(&prompt);
        match cmd {
            Input::Command(s) => {
                // Eval
                let result = run(&s, &mut global_env);

                // Print
                match result {
                    Ok(r) => println!("{}", r),
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
// (def f (lambda (x y z) (+ x y z)))
// (if (= 0 0) (+ 1 2) (- 3 4))
// (let (myfunc (lambda (myvar) (> myvar 5))) (filter myfunc (list 3 3 54 54 3 2 4 4325 4365 3645)))
// (map (lambda (x) (+ 1 x)) (range 0 10))
