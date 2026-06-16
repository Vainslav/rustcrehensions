use std::iter::once;

use quote::{ToTokens, quote};
use syn::{
    Expr, Pat, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct Comp {
    mapping: Expr,
    op: Vec<Iteration>,
}

impl Parse for Comp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            mapping: input.parse()?,
            op: parse_one_or_many(input),
        })
    }
}

impl ToTokens for Comp {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let iterations = &self.op;

        let mut iterations_size = iterations.len();

        let Iteration { item, iter, conds } =
            iterations.first().expect("Always at least one iteration");

        let expression = &self.mapping;

        let mut generated_tokens = if iterations_size == 1 {
            quote! {
                ::core::iter::IntoIterator::into_iter(#iter).flat_map(move |#item| {
                    (true #(&& (#conds))*).then(|| {#expression})
                })
            }
        } else {
            quote! {
                ::core::iter::IntoIterator::into_iter(#iter).flat_map(move |#item| {
                    (true #(&& (#conds))*).then(|| {(#item)})
                })
            }
        };

        iterations_size -= 1;

        let mut accumulated_items = vec![item];

        for Iteration { item, iter, conds } in iterations.iter().skip(1) {
            let iteration_tokens = if iterations_size == 1 {
                quote! {
                    .flat_map(move |(#(#accumulated_items),*)| {
                        ::core::iter::IntoIterator::into_iter(#iter).flat_map(move |#item| {
                            (true #(&& (#conds))*).then(|| {#expression})
                        }).collect::<Vec<_>>()
                    })
                }
            } else {
                let all_items: Vec<_> = accumulated_items
                    .iter()
                    .chain(once(&item))
                    .copied()
                    .collect();
                quote! {
                    .flat_map(move |(#(#accumulated_items),*)| {
                        ::core::iter::IntoIterator::into_iter(#iter).flat_map(move |#item| {
                            (true #(&& (#conds))*).then(|| {(#(#all_items),*)})
                        }).collect::<Vec<_>>()
                    })
                }
            };
            generated_tokens.extend(iteration_tokens);
            iterations_size -= 1;
            accumulated_items.push(item);
        }
        generated_tokens.extend(quote! {.collect()});
        tokens.extend(generated_tokens);
    }
}

struct Iteration {
    item: Pattern,
    iter: Iterable,
    conds: Vec<Condition>,
}

impl Parse for Iteration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            item: input.parse()?,
            iter: input.parse()?,
            conds: parse_one_or_many(input),
        })
    }
}

struct Pattern {
    item: Pat,
}

impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Token![for] = input.parse()?;
        Ok(Self {
            item: Pat::parse_single(input)?,
        })
    }
}

impl ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.item.to_tokens(tokens);
    }
}

struct Iterable {
    iter: Expr,
}

impl Parse for Iterable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Token![in] = input.parse()?;
        Ok(Self {
            iter: input.parse()?,
        })
    }
}

impl ToTokens for Iterable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.iter.to_tokens(tokens);
    }
}

struct Condition {
    cond: Expr,
}

impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Token![if] = input.parse()?;
        Ok(Self {
            cond: input.parse()?,
        })
    }
}

impl ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.cond.to_tokens(tokens);
    }
}

fn parse_one_or_many<T>(input: ParseStream) -> Vec<T>
where
    T: Parse,
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
    quote! { #comp }.into()
}
