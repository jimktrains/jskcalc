#![feature(integer_sign_cast)]
mod datetime;
mod units;
use crate::datetime::Date;
use crate::units::{load_units, UnitExpr};
use regex::Regex;
use std::f64::consts::{E, FRAC_PI_2, FRAC_PI_4, LN_10, LN_2, PI, SQRT_2, TAU};
use std::io;
use std::ops::{BitAnd, BitOr, BitXor};
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Cell {
    Num(f64),
    Str(String),
    Word(i64),
    Date(Date),
    Days(i64),
}

impl Cell {
    fn type_name(&self) -> String {
        match self {
            Cell::Num(_) => "Num".to_owned(),
            Cell::Str(_) => "Str".to_owned(),
            Cell::Word(_) => "Word".to_owned(),
            Cell::Date(_) => "Date".to_owned(),
            Cell::Days(_) => "Days".to_owned(),
        }
    }

    fn as_num(&self) -> f64 {
        match self {
            Cell::Num(f) => *f,
            Cell::Str(_) => panic!("Str is not a number!"),
            Cell::Word(_) => panic!("Word is not a number!"),
            Cell::Date(_) => panic!("Date is not a number!"),
            Cell::Days(_) => panic!("Days is not a number!"),
        }
    }

    fn as_word(&self) -> i64 {
        match self {
            Cell::Num(_) => panic!("Num is not a word!"),
            Cell::Str(_) => panic!("Str is not a word!"),
            Cell::Word(w) => *w,
            Cell::Date(_) => panic!("Date is not a word!"),
            Cell::Days(_) => panic!("Days is not a word!"),
        }
    }

    fn as_date(&self) -> Date {
        match self {
            Cell::Num(_) => panic!("Num is not a date!"),
            Cell::Str(_) => panic!("Str is not a date!"),
            Cell::Word(_) => panic!("Word is not date!"),
            Cell::Date(d) => d.clone(),
            Cell::Days(_) => panic!("Days is not a date!"),
        }
    }

    fn as_days(&self) -> i64 {
        match self {
            Cell::Num(_) => panic!("Num is not a days!"),
            Cell::Str(_) => panic!("Str is not a days!"),
            Cell::Word(_) => panic!("Word is not a days!"),
            Cell::Date(_) => panic!("Date is not a days!"),
            Cell::Days(d) => *d,
        }
    }
}

type OpFcn<'a> = &'a dyn Fn(Vec<Cell>) -> Vec<Cell>;
type OpType<'a> = (&'a str, Vec<&'a str>, OpFcn<'a>);
type OpsType<'a> = Vec<(&'a str, Vec<&'a str>, &'a dyn Fn(Vec<Cell>) -> Vec<Cell>)>;
type StackType = Vec<Cell>;

fn find_op<'a>(
    opname: &'a str,
    ops: &'a OpsType,
    stack: &'a mut StackType,
) -> Option<(&'a OpType<'a>, &'a mut StackType)> {
    for op in ops {
        if opname != op.0 {
            continue;
        }
        if stack.len() < op.1.len() {
            continue;
        }
        for (i, t) in op.1.iter().enumerate() {
            if *t == "*" {
                continue;
            }
            if t.to_owned() != stack[stack.len() - i - 1].type_name() {
                continue;
            }
        }
        return Some((op, stack));
    }
    None
}

fn main() -> io::Result<()> {
    let ops: Vec<(&str, Vec<&str>, &dyn Fn(Vec<Cell>) -> Vec<Cell>)> = vec![
        ("days", vec!["Num"], &|v| {
            vec![Cell::Days(v[0].as_num().floor() as i64)]
        }),
        ("weeks", vec!["Num"], &|v| {
            vec![Cell::Days(7 * v[0].as_num().floor() as i64)]
        }),
        ("+", vec!["Days", "Date"], &|v| {
            vec![Cell::Date(v[1].as_date().add_days(v[0].as_days() as u16))]
        }),
        ("dow", vec!["Date"], &|a| {
            vec![Cell::Str(format!("{:?}", a[0].as_date().dow()))]
        }),
        ("doy", vec!["Date"], &|a| {
            vec![Cell::Num(a[0].as_date().doy() as f64)]
        }),
        ("+", vec!["Num", "Num"], &|v| {
            vec![Cell::Num(v[1].as_num() + v[0].as_num())]
        }),
        ("-", vec!["Num", "Num"], &|v| {
            vec![Cell::Num(v[1].as_num() - v[0].as_num())]
        }),
        ("*", vec!["Num", "Num"], &|v| {
            vec![Cell::Num(v[1].as_num() * v[0].as_num())]
        }),
        ("/", vec!["Num", "Num"], &|v| {
            vec![Cell::Num(v[1].as_num() / v[0].as_num())]
        }),
        ("pow", vec!["Num", "Num"], &|v| {
            vec![Cell::Num(f64::powf(v[0].as_num(), v[1].as_num()))]
        }),
        ("atan2", vec!["Num", "Num"], &|v| {
            vec![Cell::Num(f64::atan2(v[0].as_num(), v[1].as_num()))]
        }),
        ("hypot", vec!["Num", "Num"], &|v| {
            vec![Cell::Num(f64::hypot(v[0].as_num(), v[1].as_num()))]
        }),
        ("sin", vec!["Num"], &|v| {
            vec![Cell::Num(f64::sin(v[0].as_num()))]
        }),
        ("cos", vec!["Num"], &|v| {
            vec![Cell::Num(f64::cos(v[0].as_num()))]
        }),
        ("tan", vec!["Num"], &|v| {
            vec![Cell::Num(f64::tan(v[0].as_num()))]
        }),
        ("asin", vec!["Num"], &|v| {
            vec![Cell::Num(f64::asin(v[0].as_num()))]
        }),
        ("acos", vec!["Num"], &|v| {
            vec![Cell::Num(f64::acos(v[0].as_num()))]
        }),
        ("atan", vec!["Num"], &|v| {
            vec![Cell::Num(f64::atan(v[0].as_num()))]
        }),
        ("sinh", vec!["Num"], &|v| {
            vec![Cell::Num(f64::sinh(v[0].as_num()))]
        }),
        ("cosh", vec!["Num"], &|v| {
            vec![Cell::Num(f64::cosh(v[0].as_num()))]
        }),
        ("tanh", vec!["Num"], &|v| {
            vec![Cell::Num(f64::tanh(v[0].as_num()))]
        }),
        ("asinh", vec!["Num"], &|v| {
            vec![Cell::Num(f64::asinh(v[0].as_num()))]
        }),
        ("acosh", vec!["Num"], &|v| {
            vec![Cell::Num(f64::acosh(v[0].as_num()))]
        }),
        ("atanh", vec!["Num"], &|v| {
            vec![Cell::Num(f64::atanh(v[0].as_num()))]
        }),
        ////
        ("d2r", vec!["Num"], &|v| {
            vec![Cell::Num(f64::to_radians(v[0].as_num()))]
        }),
        ("r2d", vec!["Num"], &|v| {
            vec![Cell::Num(f64::to_degrees(v[0].as_num()))]
        }),
        ////
        ("1/", vec!["Num"], &|v| {
            vec![Cell::Num(f64::recip(v[0].as_num()))]
        }),
        ("recip", vec!["Num"], &|v| {
            vec![Cell::Num(f64::recip(v[0].as_num()))]
        }),
        ////
        ("ln", vec!["Num"], &|v| {
            vec![Cell::Num(f64::ln(v[0].as_num()))]
        }),
        ("ln1+", vec!["Num"], &|v| {
            vec![Cell::Num(f64::ln_1p(v[0].as_num()))]
        }),
        ("log10", vec!["Num"], &|v| {
            vec![Cell::Num(f64::log10(v[0].as_num()))]
        }),
        ("log2", vec!["Num"], &|v| {
            vec![Cell::Num(f64::log2(v[0].as_num()))]
        }),
        ("exp", vec!["Num"], &|v| {
            vec![Cell::Num(f64::exp(v[0].as_num()))]
        }),
        ("exp-1", vec!["Num"], &|v| {
            vec![Cell::Num(f64::exp_m1(v[0].as_num()))]
        }),
        ("sqrt", vec!["Num"], &|v| {
            vec![Cell::Num(f64::sqrt(v[0].as_num()))]
        }),
        ("cbrt", vec!["Num"], &|v| {
            vec![Cell::Num(f64::cbrt(v[0].as_num()))]
        }),
        ("sq", vec!["Num"], &|a| {
            vec![Cell::Num(a[0].as_num() * a[0].as_num())]
        }),
        ("cb", vec!["Num"], &|a| {
            vec![Cell::Num(a[0].as_num() * a[0].as_num() * a[0].as_num())]
        }),
        ////
        ("abs", vec!["Num"], &|v| {
            vec![Cell::Num(f64::abs(v[0].as_num()))]
        }),
        ("ceil", vec!["Num"], &|v| {
            vec![Cell::Num(f64::ceil(v[0].as_num()))]
        }),
        ("floor", vec!["Num"], &|v| {
            vec![Cell::Num(f64::floor(v[0].as_num()))]
        }),
        ("pi", vec![], &|_| vec![Cell::Num(PI)]),
        ("e", vec![], &|_| vec![Cell::Num(E)]),
        ("sqrt2", vec![], &|_| vec![Cell::Num(SQRT_2)]),
        ("ln2", vec![], &|_| vec![Cell::Num(LN_2)]),
        ("ln10", vec![], &|_| vec![Cell::Num(LN_10)]),
        ("tau", vec![], &|_| vec![Cell::Num(TAU)]),
        ("pi/2", vec![], &|_| vec![Cell::Num(FRAC_PI_2)]),
        ("pi/4", vec![], &|_| vec![Cell::Num(FRAC_PI_4)]),
        ("drop", vec!["*"], &|_v| vec![]),
        ("swap", vec!["*", "*"], &|v| {
            vec![v[0].clone(), v[1].clone()]
        }),
        ("rot", vec!["*", "*", "*"], &|v| {
            vec![v[0].clone(), v[2].clone(), v[1].clone()]
        }),
        ("dup", vec!["*"], &|v| vec![v[0].clone(), v[0].clone()]),
        ////
        ("w", vec!["Num"], &|v| {
            if let Cell::Num(n) = v[0] {
                vec![Cell::Word(n as i64)]
            } else {
                let v = &v[0];
                println!("not a number: {v:?}; cannot convert to word");
                vec![]
            }
        }),
        ////
        ("dms2dd", vec!["Num", "Num", "Num"], &|v| {
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
        ("dd2dms", vec!["Num"], &|v| {
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
        ("+", vec!["Num", "Num"], &|v| {
            vec![Cell::Word(i64::wrapping_add(
                v[1].as_word(),
                v[0].as_word(),
            ))]
        }),
        ("-", vec!["Word", "Word"], &|v| {
            vec![Cell::Word(v[1].as_word().wrapping_sub(v[0].as_word()))]
        }),
        ("*", vec!["Word", "Word"], &|v| {
            vec![Cell::Word(i64::wrapping_mul(
                v[1].as_word(),
                v[0].as_word(),
            ))]
        }),
        ("/", vec!["Word", "Word"], &|v| {
            vec![Cell::Word(v[1].as_word() / v[0].as_word())]
        }),
        ("mod", vec!["Word", "Word"], &|v| {
            vec![Cell::Word(v[1].as_word() % v[0].as_word())]
        }),
        ("&", vec!["Word", "Word"], &|v| {
            vec![Cell::Word(i64::bitand(v[1].as_word(), v[0].as_word()))]
        }),
        ("^", vec!["Word", "Word"], &|v| {
            vec![Cell::Word(i64::bitxor(v[1].as_word(), v[0].as_word()))]
        }),
        ("|", vec!["Word", "Word"], &|v| {
            vec![Cell::Word(i64::bitor(v[1].as_word(), v[0].as_word()))]
        }),
        ("sl", vec!["Word", "Word"], &|v| {
            vec![Cell::Word(v[1].as_word() << v[0].as_word())]
        }),
        ("asr", vec!["Word", "Word"], &|v| {
            vec![Cell::Word(v[1].as_word() >> v[0].as_word())]
        }),
        ("lsr", vec!["Num", "Num"], &|v| {
            vec![Cell::Word(i64::from_be_bytes(
                ((u64::from_be_bytes(v[1].as_word().to_be_bytes())) >> v[0].as_word())
                    .to_be_bytes(),
            ))]
        }),
        ("~", vec!["Word"], &|v| vec![Cell::Word(!v[0].as_word())]),
    ];

    let mut stack = vec![];
    let stdin = io::stdin();
    let date_pattern = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
    let units = load_units();
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
            } else if s.starts_with("0x") {
                stack.push(Cell::Word(i64::from_be_bytes(
                    u64::from_str_radix(s.strip_prefix("0x").unwrap(), 16)
                        .unwrap()
                        .to_be_bytes(),
                )));
            } else if date_pattern.is_match(s) {
                let mut date_parts = s.split("-");
                stack.push(Cell::Date(Date::new(
                    i16::from_str(date_parts.next().unwrap()).unwrap(),
                    (u16::from_str(date_parts.next().unwrap()).unwrap() - 1).into(),
                    u8::from_str(date_parts.next().unwrap()).unwrap(),
                )));
            } else if s.starts_with("'") {
                stack.push(Cell::Str(s[1..].to_string()));
            } else if let Some((op, stack)) = find_op(s, &ops, &mut stack) {
                let mut params = vec![];
                let plen = op.1.len();
                for _ in 1..=plen {
                    params.push(stack.pop().unwrap());
                }
                stack.append(&mut op.2(params));
            } else if s == "conv" {
                let a = stack.pop();
                let b = stack.pop();
                if let Some(Cell::Str(a)) = a {
                    if let Some(Cell::Str(b)) = b {
                        if let Some(ua) = units.get(&a) {
                            if let Some(ub) = units.get(&b) {
                                let u = ub.clone() / ua.clone();

                                if let UnitExpr::Coef(c, u) = u {
                                    stack.push(Cell::Num(c.as_f64()));
                                    let u = format!("{}", u);
                                    if u.len() > 0 {
                                        stack.push(Cell::Str(u));
                                    }
                                } else {
                                    stack.push(Cell::Str(format!("{}", u)));
                                }
                            } else {
                                println!("no unit found for {b}");
                            }
                        } else {
                            println!("no unit found for {a}");
                        }
                    } else {
                        println!("2nd not a String: {b:?}");
                    }
                } else {
                    println!("1st not a String: {a:?}");
                }
            } else if s == "pl" {
                let v = stack.last();
                if let Some(v) = v {
                    match v {
                        Cell::Num(v) => println!("N {v:?}"),
                        Cell::Date(v) => println!("D {v:?}"),
                        Cell::Days(v) => println!("Ds {v:?}"),
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
                        Cell::Days(v) => println!("Ds {v:?}"),
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
