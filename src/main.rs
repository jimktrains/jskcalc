#![feature(integer_sign_cast)]
mod datetime;
use crate::datetime::Date;
use regex::Regex;
use std::f64::consts::{E, FRAC_PI_2, FRAC_PI_4, LN_10, LN_2, PI, SQRT_2, TAU};
use std::io;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Cell {
    Num(f64),
    Str(String),
    Word(i64),
    Date(Date),
}

fn main() -> io::Result<()> {
    let dbops: Vec<(&str, &dyn Fn(f64, Date) -> Date)> = vec![
        ("+days", &|a, b| b.add_days(a.floor().abs() as u16)),
        ("+months", &|a, b| b.add_months(a.floor().abs() as u16)),
    ];
    let duops: Vec<(&str, &dyn Fn(Date) -> Cell)> = vec![
        ("dow", &|a| Cell::Str(format!("{:?}", a.dow()))),
        ("doy", &|a| Cell::Num(a.doy() as f64)),
    ];
    let bops: Vec<(&str, &dyn Fn(f64, f64) -> f64)> = vec![
        ("+", &|a, b| b + a),
        ("-", &|a, b| b - a),
        ("*", &|a, b| b * a),
        ("/", &|a, b| b / a),
        ("pow", &|a, b| a.powf(b)),
        ("atan2", &f64::atan2),
        ("hypot", &f64::hypot),
    ];
    let uops: Vec<(&str, &dyn Fn(f64) -> f64)> = vec![
        ("sin", &f64::sin),
        ("cos", &f64::cos),
        ("tan", &f64::tan),
        ("asin", &f64::asin),
        ("acos", &f64::acos),
        ("atan", &f64::atan),
        ("sinh", &f64::sinh),
        ("cosh", &f64::cosh),
        ("tanh", &f64::tanh),
        ("asinh", &f64::asinh),
        ("acosh", &f64::acosh),
        ("atanh", &f64::atanh),
        ////
        ("d2r", &f64::to_radians),
        ("r2d", &f64::to_degrees),
        ////
        ("1/", &f64::recip),
        ("recip", &f64::recip),
        ////
        ("ln", &f64::ln),
        ("ln1+", &f64::ln_1p),
        ("log10", &f64::log10),
        ("log2", &f64::log2),
        ("exp", &f64::exp),
        ("exp-1", &f64::exp_m1),
        ("sqrt", &f64::sqrt),
        ("cbrt", &f64::cbrt),
        ("sq", &|a| a * a),
        ("cb", &|a| a * a * a),
        ////
        ("abs", &f64::abs),
        ("ceil", &f64::ceil),
        ("floor", &f64::floor),
    ];
    let consts: Vec<(&str, f64)> = vec![
        ("pi", PI),
        ("e", E),
        ("sqrt2", SQRT_2),
        ("ln2", LN_2),
        ("ln10", LN_10),
        ("tau", TAU),
        ("pi/2", FRAC_PI_2),
        ("pi/4", FRAC_PI_4),
    ];

    // Not a fan of the cloning.
    let stackop: Vec<(&str, usize, &dyn Fn(Vec<Cell>) -> Vec<Cell>)> = vec![
        ("drop", 1, &|_v| vec![]),
        ("swap", 2, &|v| vec![v[0].clone(), v[1].clone()]),
        ("rot", 3, &|v| {
            vec![v[0].clone(), v[2].clone(), v[1].clone()]
        }),
        ("dup", 1, &|v| vec![v[0].clone(), v[0].clone()]),
        ////
        ("w", 1, &|v| {
            if let Cell::Num(n) = v[0] {
                vec![Cell::Word(n as i64)]
            } else {
                let v = &v[0];
                println!("not a number: {v:?}; cannot convert to word");
                vec![]
            }
        }),
        ////
        ("dms2dd", 3, &|v| {
            if let Cell::Num(d) = v[2] {
                if let Cell::Num(m) = v[1] {
                    if let Cell::Num(s) = v[0] {
                        vec![Cell::Num(d + m / 60.0 + s / 3600.0)]
                    } else {
                        let s = &v[0];
                        println!("s was not a number: {s:?}");
                        vec![]
                    }
                } else {
                    let m = &v[1];
                    println!("m was not a number: {m:?}");
                    vec![]
                }
            } else {
                let d = &v[2];
                println!("d was not a number: {d:?}");
                vec![]
            }
        }),
        ("dd2dms", 1, &|v| {
            if let Cell::Num(mut v) = v[0] {
                let d = v.floor();
                v = (v - d) * 60.0;
                let m = v.floor();
                v = (v - m) * 60.0;
                let s = v;
                vec![Cell::Num(d), Cell::Num(m), Cell::Num(s)]
            } else {
                let d = &v[0];
                println!("d was not a number: {d:?}");
                vec![]
            }
        }),
    ];

    let wbops: Vec<(&str, &dyn Fn(i64, i64) -> i64)> = vec![
        ("w+", &|a, b| b.wrapping_add(a)),
        ("w-", &|a, b| b.wrapping_sub(a)),
        ("w*", &|a, b| b * a),
        ("w/", &|a, b| b / a),
        ("mod", &|a, b| b % a),
        ("&", &|a, b| b & a),
        ("^", &|a, b| b ^ a),
        ("|", &|a, b| b | a),
        ("sl", &|a, b| b << a),
        ("lsr", &|a, b| ((b as u64) >> a) as i64),
        ("asr", &|a, b| b >> a),
    ];
    let wuops: Vec<(&str, &dyn Fn(i64) -> i64)> = vec![("~", &|a| !a)];
    let mut stack = vec![];
    let stdin = io::stdin();
    let date_pattern = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
        let split = buffer.split(" ");
        for s in split {
            let s = s.trim();
            if s.len() == 0 {
                continue;
            }
            if let Ok(f) = f64::from_str(s) {
                stack.push(Cell::Num(f));
            } else if date_pattern.is_match(s) {
                let mut date_parts = s.split("-");
                stack.push(Cell::Date(Date::new(
                    i16::from_str(date_parts.next().unwrap()).unwrap(),
                    (u16::from_str(date_parts.next().unwrap()).unwrap() - 1).into(),
                    u8::from_str(date_parts.next().unwrap()).unwrap(),
                )));
            } else {
                if let Some(op) = bops.iter().position(|e| e.0 == s) {
                    let op = bops[op].1;
                    let a = stack.pop();
                    let b = stack.pop();
                    if let Some(Cell::Num(a)) = a {
                        if let Some(Cell::Num(b)) = b {
                            stack.push(Cell::Num(op(a, b)));
                        } else {
                            println!("2nd not a number: {b:?}");
                        }
                    } else {
                        println!("1st not a number: {a:?}");
                    }
                } else if let Some(op) = uops.iter().position(|e| e.0 == s) {
                    let op = uops[op].1;
                    let a = stack.pop();
                    if let Some(Cell::Num(a)) = a {
                        stack.push(Cell::Num(op(a)));
                    } else {
                        println!("1st not a number: {a:?}");
                    }
                } else if let Some(op) = wbops.iter().position(|e| e.0 == s) {
                    let op = wbops[op].1;
                    let a = stack.pop();
                    let b = stack.pop();
                    if let Some(Cell::Word(a)) = a {
                        if let Some(Cell::Word(b)) = b {
                            stack.push(Cell::Word(op(a, b)));
                        } else {
                            println!("2nd not a number: {b:?}");
                        }
                    } else {
                        println!("1st not a number: {a:?}");
                    }
                } else if let Some(op) = wuops.iter().position(|e| e.0 == s) {
                    let op = wuops[op].1;
                    let a = stack.pop();
                    if let Some(Cell::Word(a)) = a {
                        stack.push(Cell::Word(op(a)));
                    } else {
                        println!("1st not a number: {a:?}");
                    }
                } else if let Some(op) = dbops.iter().position(|e| e.0 == s) {
                    let op = dbops[op].1;
                    let a = stack.pop();
                    let b = stack.pop();
                    if let Some(Cell::Num(a)) = a {
                        if let Some(Cell::Date(b)) = b {
                            stack.push(Cell::Date(op(a, b)));
                        } else {
                            println!("2nd not a date: {b:?}");
                        }
                    } else {
                        println!("1st not a number: {a:?}");
                    }
                } else if let Some(op) = duops.iter().position(|e| e.0 == s) {
                    let op = duops[op].1;
                    let a = stack.pop();
                    if let Some(Cell::Date(a)) = a {
                        stack.push(op(a));
                    } else {
                        println!("1st not a date: {a:?}");
                    }
                } else if let Some(op) = consts.iter().position(|e| e.0 == s) {
                    stack.push(Cell::Num(consts[op].1));
                } else if let Some(op) = stackop.iter().position(|e| e.0 == s) {
                    let op = stackop[op];
                    let mut vals = vec![];
                    for i in 0..op.1 {
                        if let Some(v) = stack.pop() {
                            vals.push(v);
                        } else {
                            println!("No element at stack position {i}");
                            break;
                        }
                    }
                    if vals.len() == op.1 {
                        stack.append(&mut op.2(vals))
                    }
                } else if s == "pl" {
                    let v = stack.last();
                    if let Some(v) = v {
                        match v {
                            Cell::Num(v) => println!("N {v:?}"),
                            Cell::Date(v) => println!("D {v:?}"),
                            Cell::Word(v) => {
                                print!("W b");
                                for i in (0..=56).rev().step_by(8) {
                                    let p1 = (v >> (i + 4)) & 0xf;
                                    let p2 = (v >> i) & 0xf;
                                    print!("{p1:04b} {p2:04b}  ");
                                }
                                println!();
                                print!("W x");
                                for i in (0..=56).rev().step_by(8) {
                                    let p = (v >> i) & 0xff;
                                    print!("{p:02X} ");
                                }
                                println!();
                                println!("W {v}i64");
                                let u = v.cast_unsigned();
                                println!("W {u}u64");
                                // let f = f64::from_bits(v.cast_unsigned());
                                // println!("W {f}f64")
                            }
                            Cell::Str(v) => println!("S {v:?}"),
                        };
                    }
                } else if s == "p" || s == "pd" {
                    let v = if s == "pd" {
                        stack.pop()
                    } else {
                        // Barf
                        stack.last().map(|x| x.clone())
                    };
                    if let Some(v) = v {
                        match v {
                            Cell::Num(v) => println!("N {v:?}"),
                            Cell::Date(v) => println!("D {v:?}"),
                            Cell::Word(v) => {
                                print!("W b");
                                for i in (0..=56).rev().step_by(8) {
                                    let p1 = (v >> (i + 4)) & 0xf;
                                    let p2 = (v >> i) & 0xf;
                                    print!("{p1:04b} {p2:04b}  ");
                                }
                                println!();
                            }
                            Cell::Str(v) => println!("S {v:?}"),
                        };
                    }
                } else if s == "clr" {
                    while stack.pop().is_some() {}
                } else if s == "ps" {
                    let l = stack.len();
                    println!("Stack is {l} entries deep");
                    for (i, e) in stack.iter().rev().enumerate() {
                        println!(" {i}: {e:?}");
                    }
                    println!();
                } else if s == "q" {
                    return Ok(());
                } else {
                    stack.push(Cell::Str(s.to_owned()));
                }
            }
        }
    }
}
