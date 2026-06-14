use syn::{Expr, Pat, Token, parse::{Parse, ParseStream}};

struct Comp {
    exp: Expr,
    op: Operation,
}

impl Parse for Comp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            exp: input.parse()?,
            op: input.parse()?
        })
    }
}

struct Operation {
    item: Pattern,
    iter: Vec<Iterable>,
    conds: Vec<Condition>
}

impl Parse for Operation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            item: input.parse()?,
            iter: parse_one_or_many(input),
            conds: parse_one_or_many(input)
        })
    }
}

struct Pattern {
    item: Pat
}

impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Token![for] = input.parse()?;
        Ok(Self {
            item: Pat::parse_single(input)?
        })
    }
}

struct Iterable {
    iter: Expr
}

impl Parse for Iterable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _ : Token![in] = input.parse()?;
        Ok(Self {
            iter: input.parse()?
        })
    }
}

struct Condition {
    cond: Expr
}

impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _ : Token![if] = input.parse()?;
        Ok(Self {
            cond: input.parse()?
        })
    }
}

fn parse_one_or_many<T>(input: ParseStream) -> Vec<T>
where T: Parse
{
    let mut v: Vec<T> = Vec::new();
    while let Ok(parsed) = input.parse() {
        v.push(parsed);
    }
    v
}