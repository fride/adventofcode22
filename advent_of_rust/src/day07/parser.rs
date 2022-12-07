use anyhow::{anyhow, Error};

use chumsky::{Parser};





use crate::day07::Expr;
use chumsky::prelude::*;


pub fn parse_input(input: Vec<String>) -> Result<Vec<Expr>, Error> {
    let mut res = vec![];
    let parser = parser();
    for (idx, line) in input.iter().enumerate() {
        res.push(
            parser
                .parse(line.trim())
                .map_err(|_err| anyhow!("Failed to parse line {}:{}", idx, &line))?,
        );
    }
    Ok(res)
}

// TODO _ I dont get how to repeat this with newlines!?
fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    let command = just('$').padded();

    let file_name = filter(|c: &char| !c.is_ascii_whitespace())
        .repeated()
        .at_least(1)
        .then_ignore(end()) // why?
        .labelled("file-name")
        .collect::<String>();

    let int = text::int(10)
        .map(|x: String| x.parse::<u64>().unwrap())
        .labelled("integer");

    let cd = command
        .clone()
        .then(text::keyword("cd"))
        .padded()
        .ignore_then(file_name.clone())
        .labelled("cd")
        .map(|file_name| Expr::Cd(file_name));

    let ls = command
        .clone()
        .then(text::keyword("ls"))
        .labelled("ls")
        .map(|_| Expr::Ls);

    let dir = text::keyword("dir")
        .padded()
        .ignore_then(file_name.clone())
        .labelled("dir")
        .map(|file| Expr::Dir(file));

    let file = int
        .padded()
        .then(file_name.clone())
        .map(|(size, name)| Expr::File(size, name));
    choice((cd, ls, dir, file))
}
