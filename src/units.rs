use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::ops::{Div, Mul};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Rational {
    numerator: usize,
    denominator: usize,
}

impl Rational {
    fn new(numerator: usize, denominator: usize) -> Self {
        if denominator == 0 {
            panic!("Zero is an invalid denominator!");
        }
        let r = Self {
            numerator: numerator,
            denominator: denominator,
        };

        r.simplify()
    }
    pub fn as_f64(self) -> f64 {
        (self.numerator as f64) / (self.denominator as f64)
    }
    pub fn simplify(self) -> Self {
        let gcd = &self.gcd();
        Self {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }
    pub fn gcd(&self) -> usize {
        let mut x = self.numerator;
        let mut y = self.denominator;
        while y != 0 {
            let t = y;
            y = x % y;
            x = t;
        }
        x
    }
}

impl Div for Rational {
    // The division of rational numbers is a closed operation.
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.numerator == 0 {
            panic!("Cannot divide by zero-valued `Rational`!");
        }

        let numerator = self.numerator * rhs.denominator;
        let denominator = self.denominator * rhs.numerator;
        Self::new(numerator, denominator)
    }
}

impl Div<f64> for Rational {
    // The division of rational numbers is a closed operation.
    type Output = f64;

    fn div(self, rhs: f64) -> Self::Output {
        let numerator = self.numerator as f64;
        let denominator = self.denominator as f64;
        numerator / denominator / rhs
    }
}

impl Div<Rational> for f64 {
    type Output = Self;
    fn div(self, rhs: Rational) -> Self::Output {
        let numerator = rhs.numerator as f64;
        let denominator = rhs.denominator as f64;
        self * denominator / numerator
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Rational) -> Self::Output {
        let numerator = self.numerator * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        Self::new(numerator, denominator)
    }
}

impl Mul<f64> for Rational {
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        let numerator = self.numerator as f64;
        let denominator = self.denominator as f64;
        numerator / denominator * rhs
    }
}

impl Mul<Rational> for f64 {
    type Output = f64;

    fn mul(self, rhs: Rational) -> Self::Output {
        let numerator = rhs.numerator as f64;
        let denominator = rhs.denominator as f64;
        self * numerator / denominator
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Coef {
    Float(f64),
    Rational(Rational),
}

impl Coef {
    pub fn is_unit(&self) -> bool {
        match self {
            Coef::Rational(l) => l.numerator == l.denominator,
            _ => false,
        }
    }
    pub fn unit() -> Self {
        Coef::Rational(Rational::new(1, 1))
    }
    pub fn as_f64(self) -> f64 {
        match self {
            Coef::Float(f) => f,
            Coef::Rational(r) => r.as_f64(),
        }
    }
}

impl Div for Coef {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Coef::Float(l), Coef::Float(r)) => Coef::Float(l / r),
            (Coef::Rational(l), Coef::Float(r)) => Coef::Float(l / r),
            (Coef::Float(l), Coef::Rational(r)) => Coef::Float(l / r),
            (Coef::Rational(l), Coef::Rational(r)) => Coef::Rational(l / r),
        }
    }
}

impl Mul<Coef> for f64 {
    type Output = Coef;

    fn mul(self, rhs: Coef) -> Self::Output {
        Coef::Float(self) * rhs
    }
}

impl Mul<f64> for Coef {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Coef::Float(rhs) * self
    }
}

impl Mul for Coef {
    type Output = Self;

    fn mul(self, rhs: Coef) -> Self::Output {
        match (self, rhs) {
            (Coef::Float(l), Coef::Float(r)) => Coef::Float(l * r),
            (Coef::Rational(l), Coef::Float(r)) => Coef::Float(l * r),
            (Coef::Float(l), Coef::Rational(r)) => Coef::Float(l * r),
            (Coef::Rational(l), Coef::Rational(r)) => Coef::Rational(l * r),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Basic(String);

#[derive(Debug, PartialEq, Clone)]
pub enum UnitExpr {
    Basic(Basic),
    Div(Vec<Basic>, Vec<Basic>),
    Coef(Coef, Box<UnitExpr>),
}

impl fmt::Display for UnitExpr {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitExpr::Basic(b) => formatter.write_str(b.0.as_ref()),
            UnitExpr::Coef(c, e) => formatter.write_fmt(format_args!("{:?} {}", c, e)),
            UnitExpr::Div(n, d) => {
                if n.is_empty() {
                    if !d.is_empty() {
                        formatter.write_str("1 ")?;
                    }
                } else {
                    for e in n {
                        formatter.write_str(e.0.as_ref())?;
                        formatter.write_str(" ")?;
                    }
                }
                if !d.is_empty() {
                    formatter.write_str("/ ")?;
                    for e in d {
                        formatter.write_str(e.0.as_ref())?;
                        formatter.write_str(" ")?
                    }
                }
                Ok(())
            }
        }
    }
}

impl UnitExpr {
    pub fn factor_out_coef(self) -> (Coef, UnitExpr) {
        match self {
            UnitExpr::Basic(_) => (Coef::unit(), self),
            UnitExpr::Div(_, _) => (Coef::unit(), self),
            UnitExpr::Coef(x, e) => (x, *e),
        }
    }

    pub fn make(lf: Coef, rf: Coef, n: Vec<Basic>, d: Vec<Basic>) -> UnitExpr {
        let (n, d) = Self::cancel_units(n, d);
        let e = UnitExpr::Div(n, d);

        let c = lf / rf;
        if c.is_unit() {
            e
        } else {
            UnitExpr::Coef(c, Box::new(e))
        }
    }

    pub fn cancel_units(n: Vec<Basic>, d: Vec<Basic>) -> (Vec<Basic>, Vec<Basic>) {
        let mut n = n;
        let mut d = d;
        // I'm sure there's a better way to do this.
        let mut rem = None;
        let mut been_through = false;
        while !been_through || rem.is_some() {
            been_through = true;
            rem = None;
            for (ni, ne) in n.iter().enumerate() {
                for (di, de) in d.iter().enumerate() {
                    if ne == de {
                        rem = Some((ni, di));
                        break;
                    }
                }
                if rem.is_some() {
                    break;
                }
            }
            if let Some((ni, di)) = rem {
                n.remove(ni);
                d.remove(di);
            }
        }
        (n, d)
    }
}

impl Div for UnitExpr {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let (lf, l) = self.factor_out_coef();
        let (rf, r) = rhs.factor_out_coef();

        let (n, d) = match (l, r) {
            (UnitExpr::Div(mut ln, mut ld), UnitExpr::Div(mut rn, mut rd)) => {
                ln.append(&mut rd);
                ld.append(&mut rn);
                (ln, ld)
            }
            (UnitExpr::Basic(b), UnitExpr::Div(rn, mut rd)) => {
                rd.push(b);
                (rd, rn)
            }
            (UnitExpr::Div(ln, mut ld), UnitExpr::Basic(b)) => {
                ld.push(b);
                (ln, ld)
            }
            (UnitExpr::Basic(lb), UnitExpr::Basic(rb)) => (vec![lb], vec![rb]),
            _ => panic!("How'd we get here?"),
        };

        UnitExpr::make(lf, rf, n, d)
    }
}

impl Mul for UnitExpr {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let (lf, l) = self.factor_out_coef();
        let (rf, r) = rhs.factor_out_coef();

        let (n, d) = match (l, r) {
            (UnitExpr::Div(mut ln, mut ld), UnitExpr::Div(mut rn, mut rd)) => {
                ln.append(&mut rd);
                ld.append(&mut rn);
                (ln, ld)
            }
            (UnitExpr::Basic(b), UnitExpr::Div(mut rn, rd)) => {
                rn.push(b);
                (rn, rd)
            }
            (UnitExpr::Div(mut ln, ld), UnitExpr::Basic(b)) => {
                ln.push(b);
                (ln, ld)
            }
            (UnitExpr::Basic(lb), UnitExpr::Basic(rb)) => (vec![lb, rb], vec![]),
            _ => panic!("How'd we get here?"),
        };

        UnitExpr::make(lf, rf, n, d)
    }
}

pub fn load_units() -> HashMap<String, UnitExpr> {
    // taken from gnu units (/usr/share/units/definitions.units)
    let lines = vec![
        "cm !", // I have to stop this somewhere
        "inch                    2.54 cm",
        "in                      inch",
        "usgallon                231 in^3 # US liquid measure is derived from",
        "gallon                  usgallon",
        "gal                     gallon          # the British wine gallon of 1707.",
        "quart                   1|4 gallon      # See the \"winegallon\" entry below",
        "pint                    1|2 quart       # more historical information.",
        "gill                    1|4 pint",
        "usquart                 1|4 usgallon",
        "uspint                  1|2 usquart",
        "usgill                  1|4 uspint",
        "usfluidounce            1|16 uspint",
        "usfloz                  usfluidounce",
        "fluiddram               1|8 usfloz",
        "minimvolume             1|60 fluiddram",
        "qt                      quart",
        "pt                      pint",
        "uscup                   8 usfloz",
        "ustablespoon            1|16 uscup",
        "usteaspoon              1|3 ustablespoon",
        "ustbl                   ustablespoon",
        "ustbsp                  ustablespoon",
        "ustblsp                 ustablespoon",
        "ustsp                   usteaspoon",
    ];
    let mut units = HashMap::new();
    let units_line =
        Regex::new(r"(?<name>\S+)\s+((?<num>\d+(\.\d+)?)(\|(?<den>\d+))?\s+)?(?<def>[^#]*)(#.*)?")
            .unwrap();
    for line in lines {
        for c in units_line.captures_iter(line) {
            let def = c["def"].trim().to_owned();
            let name = c["name"].trim().to_owned();
            let mut coef = Coef::unit();
            let mut e = if def == "!" {
                UnitExpr::Basic(Basic(name.clone()))
            } else {
                UnitExpr::Div(
                    def.split(" ")
                        .flat_map(|x| {
                            if x.contains("^") {
                                let mut s = x.split("^");
                                let b = s.next().unwrap();
                                let n = usize::from_str(s.next().unwrap()).unwrap();
                                let mut v = vec![];
                                for _i in 0..n {
                                    v.push(b);
                                }
                                v
                            } else {
                                vec![x]
                            }
                        })
                        .flat_map(|x| {
                            if units.contains_key(x) {
                                match &units[x] {
                                    UnitExpr::Basic(b) => vec![b.clone()],
                                    UnitExpr::Div(n, _d) => n.clone(), // Need to handle the denom
                                    UnitExpr::Coef(c, e) => {
                                        coef = coef * *c;
                                        if let UnitExpr::Basic(b) = (**e).clone() {
                                            vec![b.clone()]
                                        } else if let UnitExpr::Div(n, _d) = (**e).clone() {
                                            n.clone() // Need to handle the denom
                                        } else {
                                            vec![]
                                        }
                                    }
                                }
                            } else {
                                println!("Couldn't find {x}");
                                vec![]
                            }
                        })
                        .collect(),
                    vec![],
                )
            };
            if let Some(nn) = c.name("num") {
                let nr = usize::from_str(nn.into());
                if let Ok(n) = nr {
                    let d =
                        usize::from_str(c.name("den").map(|c| c.into()).unwrap_or("1")).unwrap();
                    coef = coef * Coef::Rational(Rational::new(n, d));
                } else {
                    let n = Coef::Float(f64::from_str(nn.into()).unwrap());
                    coef = coef * n;
                }
            }
            if !coef.is_unit() {
                e = UnitExpr::Coef(coef, Box::new(e));
            }
            //println!("{name:<20}=> {e:?}");
            units.insert(name, e);
        }
    }
    units
}

#[cfg(test)]
mod units_test {
    use super::*;
    #[test]
    pub fn expr() {
        let m = UnitExpr::Coef(
            Coef::Float(10.0),
            Box::new(UnitExpr::Basic(Basic("m".to_owned()))),
        );
        let s = UnitExpr::Basic(Basic("s".to_owned()));
        let v = m.clone() / s.clone();
        println!("{m} /  {s} = {v}");

        let m = UnitExpr::Basic(Basic("m".to_owned()));
        let a = m.clone() * m.clone();
        println!("{m} *  {m} = {a}");

        let v2 = a.clone() / v.clone();
        println!("{a} /  {v} = {v2}");

        let v2 = v.clone() / a.clone();
        println!("{v} /  {a} = {v2}");
        let units = load_units();

        let tsp = units["ustsp"].clone();
        let tbsp = units["ustbsp"].clone();
        println!("{tsp:?}");
        println!("{tbsp:?}");
        let conv = tbsp.clone() / tsp.clone();
        println!("tbsp / tsp = {conv:?}");
        let conv = tsp / tbsp;
        println!("tsp / tbsp = {conv:?}");
    }
}
