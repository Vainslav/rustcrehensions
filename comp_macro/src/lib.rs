use quote::{ToTokens, quote};
use syn::{Expr, Pat, Token, parse::{Parse, ParseStream}, parse_macro_input};

// comp![ x for x in vec![1, 2, 3] ]
#[test]
fn test3() {
    let list: Vec<i32> = ::core::iter::IntoIterator::into_iter(vec![1, 2, 3])
        .flat_map(move |x| {
            (true).then(|| {x})
        })
        .collect();
    dbg!(list);
    assert!(false)
}


// comp![ (x, y) for x in vec![1, 2, 3, 4] if x > 1 for y in vec![1, 2, 3, 4] if x + y == 4 ]
#[test]
fn test() {
    let list: Vec<(i32, i32)> = ::core::iter::IntoIterator::into_iter(vec![1, 2, 3, 4])
        .flat_map(move |x| {
            (true && x > 1).then(|| {x})
        })
        .flat_map(move |x| {
            ::core::iter::IntoIterator::into_iter(vec![1, 2, 3, 4])
                .flat_map(move |y| {
                    (true && x + y == 4).then(|| {(x, y)})
                })
        })
        .collect();
    dbg!(list);
    assert!(false)
}

// comp![ comp![(x, y) for y in vec![1, 2, 3, 4] if x+y == 4] for x in vec![1, 2, 3, 4] if x > 1 ]
#[test]
fn test2() {
    let list: Vec<Vec<(i32, i32)>> = ::core::iter::IntoIterator::into_iter(vec![1, 2, 3, 4])
        .flat_map(move |x| {
            (true && x > 1).then(|| {
                    ::core::iter::IntoIterator::into_iter(vec![1, 2, 3, 4])
                        .flat_map(|y| {(true && x + y == 4).then(|| (x, y))})
                        .collect()
                }
            )
        })
        .collect();
    dbg!(list);
    assert!(false)
}

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

impl ToTokens for Comp {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Operation { 
            item, 
            iter, 
            conds 
        } = &self.op;

        let first_iter = iter.get(0);
        let _first_cond = conds.get(0);

        let expression = &self.exp;

        tokens.extend(
            quote! {
                ::core::iter::IntoIterator::into_iter(#first_iter).flat_map(move |#item| {
                    (true #(&& (#conds))*).then(|| {#expression})
                }).collect()
            }
        );
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

impl ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.item.to_tokens(tokens);
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

impl ToTokens for Iterable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.iter.to_tokens(tokens);
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

impl ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.cond.to_tokens(tokens);
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

#[proc_macro]
pub fn comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let comp = parse_macro_input!(input as Comp);
    quote!{ #comp }.into()
}