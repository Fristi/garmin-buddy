use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::prelude::*;


fn main() {
    parse("600m + 4 * (400m + 800m) + 600m");
    parse("1600m + 1400m + 1200m + 1000m + 800m");
}

fn parse(input: &str) {
    let res = parser().parse(input.trim()).unwrap();

    let d = total_distance(&res).unwrap();
    let i = instruction(&res).unwrap();

    println!("{}m ({})", d, i);
}


fn total_distance(w: &Workout) -> Result<u32, String> {
    match w {
        Workout::Num(x) => Ok(*x),
        Workout::Meters(x) => Ok(*x),
        Workout::Plus(a, b) => {
            let x = total_distance(a)?;
            let y = total_distance(b)?;
            Ok(x + y)
        },
        Workout::Times(a, b ) => {
            let x = total_distance(a)?;
            let y = total_distance(b)?;
            Ok(x * y)
        },
        Workout::Annotation(_, _) => Ok(0)
    }
}

fn instruction(w: &Workout) -> Result<String, String> {
    match w {
        Workout::Num(x) => Ok(format!("{}", *x)),
        Workout::Meters(x) => Ok(format!("{}m", *x)),
        Workout::Plus(a, b) => {
            let x = instruction(a)?;
            let y = instruction(b)?;
            Ok(format!("{} + {}", x, y))
        },
        Workout::Times(a, b ) => {
            let x = instruction(a)?;
            let y = instruction(b)?;
            Ok(format!("{} * {}", x, y))
        },
        Workout::Annotation(instr, a) => {
            let expr = instruction(a)?;
            Ok(format!("{}@{}", expr, instr))
        }
    }
}

fn parser<'a>() -> impl Parser<char, Workout, Error = Simple<char>> {
    recursive(|workout| {
        let num = text::int(10)
            .map(|s: String| Workout::Num(s.parse().unwrap()))
            .labelled("num")
            .padded();

        let meters = text::int(10).then_ignore(just('m'))
            .map(|s: String| Workout::Meters(s.parse().unwrap()))
            .labelled("meters")
            .padded();


        // let string = just('"')
        //     .ignore_then(none_of('"').repeated())
        //     .then_ignore(just('"'))
        //     .map(|c| String::from_iter(c));

        let lit = meters.or(num);

        let atom = lit
            .or(workout.clone().delimited_by(just('('), just(')')))
            .padded();

        let op = |c| just(c).padded();


        let times = atom.clone()
            .then(op('*').to(Workout::Times as fn(_,_) -> _).then(atom).repeated())
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let sum = times.clone()
            .then(op('+').to(Workout::Plus as fn(_,_) -> _).then(times).repeated())
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        // let annotation = sum.clone()
        //     .then(workout.then_ignore(just('@')).then(string).map(|(w, a)| Workout::Annotation(a, Box::new(w))))


        sum
    })
    .then_ignore(end())

}

#[derive(Clone, Debug)]
enum Workout {
    Plus(Box<Workout>, Box<Workout>),
    Num(u32),
    Meters(u32),
    Times(Box<Workout>, Box<Workout>),
    Annotation(String, Box<Workout>)
}

