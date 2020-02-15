use super::ast::*;

peg::parser! {    
    grammar grammar() for str {
        rule comment() = "//" (!"\n" [_])* "\n"
        rule ws() = [' ' | '\n' | '\t']
        // zero or more whitespace.
        rule _() = quiet!{(ws()/comment())*}
        // one or more whitespace.
        rule __() = quiet!{ws()+}

        pub rule file() -> Vec<Func>
        = _ i:func() ** _ _ { i }

        rule func() -> Func
        = "func" __ n:ident()  _ "(" _ p:params() _ ")" _ r:expr()? _ b:block() { Func {
                name: n,
                params: p,
                ret_type: r,
                block: b,
            }
        }

        rule params() -> Vec<FuncParam>
        = p:param() ** ( _ "," _ ) { p }

        rule param() -> FuncParam
        = x:ident() _ ":" _ y:expr() { FuncParam{ name: x, acetype: y } }

        rule block() -> Block
        = "{" _ s:stmt() ** _ _ "}" { s }

        rule stmt() -> Stmt = precedence! {
            "return" _ e:expr()? _ ";" { Stmt::Return(e) }
            l:expr() _ "=" _ r:expr() _ ";" { Stmt::Assign(l, r) }
            e:expr() _ ";" { Stmt::Expr(e) }
        }

        rule expr() -> Expr = precedence! {
            x:(@) _ "+" _ y:@ { Expr::new_binop(x, Ops::Add, y) }
            x:(@) _ "-" _ y:@ { Expr::new_binop(x, Ops::Sub, y) }
            --
            x:(@) _ "*" _ y:@ { Expr::new_binop(x, Ops::Mul, y) }
            x:(@) _ "/" _ y:@ { Expr::new_binop(x, Ops::Div, y) }
            x:(@) _ "%" _ y:@ { Expr::new_binop(x, Ops::Mod, y) }
            --
            x:@ _ "(" _ args:expr() ** ( _ "," _ ) _ ")" { Expr::Call(Box::new(x), args) }
            x:@ _ "[" _ y:expr() _ "]" { Expr::new_binop(x, Ops::Index, y) }
            --
            n:int() { n }
            n:ident() { Expr::Ident(n) }
            "(" _ n:expr() _ ")" { n }
        }

        rule int() -> Expr
        = n:$(['0'..='9']+) { Expr::Int(n.parse().unwrap()) }

        rule ident() -> Ident
        = n:$(['a'..='z']+) { String::from(n) }
        
    }
}

pub fn parse(file: &str) {
    let res = grammar::file(file);
    dbg!(res.unwrap());
}