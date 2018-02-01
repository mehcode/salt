#![feature(proc_macro)]

#[macro_use]
extern crate quote;

extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn get(opts: TokenStream, item: TokenStream) -> TokenStream {
    impl_route_rewrite(syn::parse_expr("::shio::http::Method::GET").unwrap(), opts, item)
}

#[proc_macro_attribute]
pub fn post(opts: TokenStream, item: TokenStream) -> TokenStream {
    impl_route_rewrite(syn::parse_expr("::shio::http::Method::POST").unwrap(), opts, item)
}

#[proc_macro_attribute]
pub fn patch(opts: TokenStream, item: TokenStream) -> TokenStream {
    impl_route_rewrite(syn::parse_expr("::shio::http::Method::PATCH").unwrap(), opts, item)
}

#[proc_macro_attribute]
pub fn put(opts: TokenStream, item: TokenStream) -> TokenStream {
    impl_route_rewrite(syn::parse_expr("::shio::http::Method::PUT").unwrap(), opts, item)
}

#[proc_macro_attribute]
pub fn delete(opts: TokenStream, item: TokenStream) -> TokenStream {
    impl_route_rewrite(syn::parse_expr("::shio::http::Method::DELETE").unwrap(), opts, item)
}

#[proc_macro_attribute]
pub fn options(opts: TokenStream, item: TokenStream) -> TokenStream {
    impl_route_rewrite(syn::parse_expr("::shio::http::Method::OPTIONS").unwrap(), opts, item)
}

#[proc_macro_attribute]
pub fn head(opts: TokenStream, item: TokenStream) -> TokenStream {
    impl_route_rewrite(syn::parse_expr("::shio::http::Method::HEAD").unwrap(), opts, item)
}

fn impl_route_rewrite(meth: syn::Expr, opts: TokenStream, item: TokenStream) -> TokenStream {
    let item = item.to_string();
    let item = syn::parse_item(&item).expect("unable to parse item associated to get attribute");

    match item.node {
        syn::ItemKind::Fn(_, _, _, _, _, _) => {}
        _ => panic!("get attribute is only for functions"),
    }

    let opts = opts.to_string();
    let opts = syn::parse_token_trees(&opts).expect("unable to parse options of get attribute");
    let opts = &opts[0];

    let tts = match *opts {
        syn::TokenTree::Delimited(ref delim) => &delim.tts,
        _ => panic!("unvalid attribute options"),
    };
    let tt1 = &tts[0];
    let tok = match *tt1 {
        syn::TokenTree::Token(ref tok) => tok,
        _ => panic!("expected a token as first attribute option"),
    };
    let lit = match *tok {
        syn::Token::Literal(ref lit) => lit,
        _ => panic!("expected a literal as first attribute option"),
    };
    match *lit {
        syn::Lit::Str(_, _) => {}
        _ => panic!("expected a string literal as first attribute option"),
    };

    Route {
        handler: item,
        shio_method: meth,
        route: lit.clone(),
    }.create_new_token_stream()
}

struct Route {
    handler: syn::Item,
    shio_method: syn::Expr,
    route: syn::Lit,
}

impl Route {
    fn create_new_token_stream(mut self) -> TokenStream {
        let new_ident = syn::Ident::from(format!("__shio_handler_{}", self.handler.ident));
        let prev_ident = self.handler.ident.clone();
        self.handler.ident = new_ident.clone();

        let Route { handler, shio_method, route } = self;
        
        let tokens = quote! {
            #handler
            #[allow(non_camel_case_types)]
            pub struct #prev_ident;
            impl Into<::shio::router::Route> for #prev_ident {
                fn into(self) -> ::shio::router::Route {
                    (#shio_method, #route, #new_ident).into()
                }
            }
        };

        tokens.parse().unwrap()
    }
}
