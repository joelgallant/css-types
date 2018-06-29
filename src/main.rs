use std::fs::File;
use std::io::Read;

#[macro_use] extern crate nom;

// A
named!(type_name(&str) -> &str, ws!(nom::alphanumeric));

// type A
named!(type_start(&str) -> &str,
    fold_many1!(
        tuple!(
            tag!("type"),
            tag!(" "),
            type_name
        ),
        "",
        | _, (_, _, name) | name
    )
);

// extends A, B, C
named!(type_extends(&str) -> Vec<String>,
    fold_many1!(
        pair!(
            ws!(tag!("extends")),
            many1!(
                pair!(
                    type_name,
                    opt!(ws!(tag!(",")))
                )
            )
        ),
        Vec::new(),
        | _, (_, list): (&str, Vec<(&str, Option<&str>)>) | {
            list.iter().map(| (name, _) | name.to_string()).collect()
        }
    )
);

#[derive(Debug)]
struct TypeDef {
    name: String,
    extends: Vec<String>,
}

named!(type_defs(&str) -> Vec<TypeDef>,
    fold_many1!(
        complete!(ws!(tuple!(
            type_start,
            opt!(type_extends),
            ws!(tag!(";"))
        ))),
        Vec::new(),
        | mut acc: Vec<TypeDef>, (name, extends, _): (&str, Option<Vec<String>>, &str) | {
            acc.push(
                TypeDef {
                    name: name.to_string(),
                    extends: match extends {
                        Some(extends) => extends,
                        None => vec![]
                    }
                }
            );
            acc
        }
    )
);

fn main() {
    let mut file = File::open("test.tcss").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    match type_defs(&contents) {
        Ok((_, res)) => println!("{:?}", res),
        Err(err) => println!("err: {:?}", err),
    }
}
