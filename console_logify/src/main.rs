use ressa::{
    Parser,
};
use std::{
    io::Read,
    env::args,
    fs::read_to_string,
};
use resw::Writer;
use resast::prelude::*;

fn main() {
    let js = get_js().expect("Unable to get JavaScript");
    let parser = Parser::new(&js).expect("Unable to construct parser");
    let mut writer = Writer::new(::std::io::stdout());
    for part in parser.filter_map(|p| p.ok()).map(map_part) {
        writer.write_part(&part).expect("Failed to write part");
    }
}

fn map_part(part: ProgramPart) -> ProgramPart {
    match part {
        ProgramPart::Decl(ref decl) => ProgramPart::Decl(map_decl(decl)),
        ProgramPart::Stmt(ref stmt) => ProgramPart::Stmt(map_stmt(stmt)),
        ProgramPart::Dir(_) => part,
    }
}

fn map_decl(decl: &Decl) -> Decl {
    match decl {
        Decl::Function(ref f) => Decl::Function(map_func(f)),
        Decl::Class(ref class) => Decl::Class(map_class(class)),
        _ => decl.clone()
    }
}

fn map_stmt(stmt: &Stmt) -> Stmt {
    match stmt {
        Stmt::Expr(ref expr) => Stmt::Expr(map_expr(expr)),
        _ => stmt.clone(),
    }
}

fn map_expr(expr: &Expr) -> Expr {
    match expr {
        Expr::Function(ref f) => Expr::Function(map_func(f)),
        Expr::Class(ref c) => Expr::Class(map_class(c)),
        _ => expr.clone(),
    }
}

fn map_func(func: &Function) -> Function {
    let mut f = func.clone();
    let mut args = vec![];
    if let Some(ref name) = f.id {
        args.push(
            Expr::string(&format!("'{}'", name))
        );
    }
    for arg in f.params.iter().filter_map(|a| match a {
        FunctionArg::Expr(e) => match e {
            Expr::Ident(i)  => Some(i),
            _ => None,
        },
        FunctionArg::Pat(p) => match p {
            Pat::Identifier(i) => Some(i),
            _ => None,
        },
    }) {
        args.push(Expr::ident(arg));
    }
    f.body.insert(
        0,
        console_log(args),
    );
    f.body = f.body.into_iter().map(map_part).collect();
    f
}

fn map_class(class: &Class) -> Class {
    let mut class = class.clone();
    let prefix =  if let Some(ref id) = class.id {
        id.clone()
    } else {
        String::new()
    };

    class.body = class.body
                        .iter()
                        .map(|prop| map_class_prop(&prefix, prop))
                        .collect();
    class
}

fn map_class_prop(prefix: &str, prop: &Property) -> Property {
    let mut prop = prop.clone();
    let mut args = match prop.kind {
        PropertyKind::Ctor => {
            vec![Expr::string(&format!("'new {}'", prefix))]
        },
        PropertyKind::Get => {
            vec![
                Expr::string(&format!("'{}'", prefix)),
                Expr::string("get"),
            ]
        },
        PropertyKind::Set => {
            vec![
                Expr::string(&format!("'{}'", prefix)),
                Expr::string("set"),
            ]
        },
        PropertyKind::Method => {
            vec![
                Expr::string(&format!("'{}'", prefix)),
            ]
        },
        _ => vec![],
    };
    match &prop.key {
        PropertyKey::Expr(ref e) => {
            match e {
                Expr::Ident(i) => if i != "constructor" {
                    args.push(Expr::string(&format!("'{}'", i)));
                },
                _ => (),
            }
            
        },
        PropertyKey::Literal(ref l) => {
            match l {
                Literal::Boolean(ref b) => {
                    args.push(Expr::string(&format!("'{}'", b)));
                },
                Literal::Null => {
                    args.push(Expr::string("'null'"));
                },
                Literal::Number(ref n) => {
                    args.push(Expr::string(&format!("'{}'", n)));
                }
                Literal::RegEx(ref r) => {
                    args.push(Expr::string(&format!("'/{}/{}'", r.pattern, r.flags)));
                },
                Literal::String(ref s) => {
                    if s != "constructor" {
                        args.push(Expr::string(s));
                    }
                },
                _ => (),
            }
        },
        PropertyKey::Pat(ref p) => {
            match p {
                Pat::Identifier(ref i) => {
                    args.push(Expr::string(&format!("'{}'", i)));
                },
                _ => (),
            }
        },
    }
    if let PropertyValue::Expr(ref mut expr) = prop.value {
        match expr {
            Expr::Function(ref mut f) => {
                for ref arg in &f.params {
                    match arg {
                        FunctionArg::Expr(ref expr) => {
                            match expr {
                                Expr::Ident(_) => args.push(expr.clone()),
                                _ => (),
                            }
                        },
                        FunctionArg::Pat(ref pat) => {
                            match pat {
                                Pat::Identifier(ref ident) => {
                                    args.push(Expr::ident(ident))
                                },
                                _ => {},
                            }
                        }
                    }
                }
                f.body.insert(0,
                    console_log(args)
                )
            },
            _ => (),
        }
    }
    prop
}

fn console_log(args: Vec<Expr>) -> ProgramPart {
    ProgramPart::Stmt(
        Stmt::Expr(
            Expr::call(
                Expr::member(
                    Expr::ident("console"),
                    Expr::ident("log"),
                    false,
                ),
                args,
            )
        )
    )
}

fn get_js() -> Result<String, ::std::io::Error> {
    let mut cmd_args = args();
    let _ = cmd_args.next(); //discard bin name
    let js = if let Some(file_name) = cmd_args.next() {
        let js = read_to_string(file_name)?;
        js
    } else {
        let mut std_in = ::std::io::stdin();
        let mut ret = String::new();
        if atty::is(atty::Stream::Stdin) {
            return Ok(ret)
        }
        let _ = std_in.read_to_string(&mut ret);
        ret
    };
    Ok(js)
}