use std::fs::File;
use std::io::Read;

#[macro_use] extern crate nom;

type TypeName<'a> = &'a str;
named!(type_name(&str) -> TypeName, ws!(nom::alphanumeric));

#[derive(Debug)]
struct TypeDef {
    name: String,
    extends: Vec<String>,
}

named!(type_def<&str, TypeDef>,
    fold_many0!(
        pair!(
            pair!(
                ws!(tag!("type")),
                type_name // TypeName
            ),
            opt!(pair!(
                ws!(tag!("extends")),
                many0!( // BaseNameA, BaseNameB
                    pair!(
                        type_name,
                        opt!(tag!(","))
                    )
                )
            ))
        ),
        TypeDef { name: String::new(), extends: vec![] },
        | _, ((_, name), extends): ((&str, &str), Option<(&str, Vec<(&str, Option<&str>)>)>) | -> TypeDef {
            TypeDef {
                name: name.to_string(),
                extends: match extends {
                    Some((_, extends)) => {
                        extends.iter().map(| (name, _) | name.to_string()).collect()
                    },
                    None => vec![],
                },
            }
        }
    )
);

fn main() {
    let mut file = File::open("test.tcss").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("{:?}", type_def(&contents).unwrap().1);
}
