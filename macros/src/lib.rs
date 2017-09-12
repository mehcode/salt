#![feature(proc_macro)]

#[macro_use]
extern crate quote;

extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn get(opts: TokenStream, item: TokenStream) -> TokenStream {
    impl_route_rewrite(syn::parse_expr("::shio::Method::Get").unwrap(), opts, item)
}

#[proc_macro_attribute]
pub fn post(opts: TokenStream, item: TokenStream) -> TokenStream {
    impl_route_rewrite(syn::parse_expr("::shio::Method::Post").unwrap(), opts, item)
}

fn impl_route_rewrite(meth: syn::Expr, opts: TokenStream, item: TokenStream) -> TokenStream {
    let item = item.to_string();
    let item = syn::parse_item(&item).expect("unable to parse item associated to get attribute");

    match &item.node {
        &syn::ItemKind::Fn(_, _, _, _, _, _) => {}
        _ => panic!("get attribute is only for functions"),
    }

    let opts = opts.to_string();
    let opts = syn::parse_token_trees(&opts).expect("unable to parse options of get attribute");
    let opts = &opts[0];

    let tts = match opts {
        &syn::TokenTree::Delimited(ref delim) => &delim.tts,
        _ => panic!("unvalid attribute options"),
    };
    let tt1 = &tts[0];
    let tok = match tt1 {
        &syn::TokenTree::Token(ref tok) => tok,
        _ => panic!("expected a token as first attribute option"),
    };
    let lit = match tok {
        &syn::Token::Literal(ref lit) => lit,
        _ => panic!("expected a literal as first attribute option"),
    };
    match lit {
        &syn::Lit::Str(_, _) => {}
        _ => panic!("expected a string literal as first attribute option"),
    };

    Route {
        handler: item,
        shio_method: meth,
        route: lit.clone(),
    }.output_token_stream()
}

struct Route {
    handler: syn::Item,
    shio_method: syn::Expr,
    route: syn::Lit,
}

impl Route {
    // output the token stream associated to this route
    fn output_token_stream(mut self) -> TokenStream {
        let nstruct = self.create_new_struct();
        let nstatic = self.create_static();
        let nimpl = self.create_into_impl();

        let new_ident = syn::Ident::from(format!("__shio_handler_{}", self.handler.ident));
        self.handler.ident = new_ident;
        let Route { handler, .. } = self;
        let handler = handler;
        let struct_ident = nstruct.ident.clone();

        let tokens = quote! {
            #handler
            #[allow(non_camel_case_types)]
            #nstruct
            impl Copy for #struct_ident {}
            impl Clone for #struct_ident {
                fn clone(&self) -> Self {
                    *self
                }
            }
            #[allow(non_upper_case_globals)]
            #nstatic
            #nimpl
        };

        tokens.parse().unwrap()
    }

    // Construct the syn object related to structure decl
    fn create_new_struct(&self) -> syn::Item {
        let variant_data_for_struct = syn::VariantData::Struct(vec![]);
        let struct_kind = syn::ItemKind::Struct(variant_data_for_struct, Default::default());

        let ident = self.handler.ident.clone();

        syn::Item {
            node: struct_kind,
            vis: self.handler.vis.clone(),
            attrs: vec![],
            ident: format!("__shio_route_{}", ident).into(),
        }
    }

    // Construct the static syn object
    fn create_static(&self) -> syn::Item {
        let struct_path = syn::Path::from(format!("__shio_route_{}", self.handler.ident));

        let struct_value_expr = syn::ExprKind::Struct(struct_path.clone(), vec![], None);

        syn::Item {
            ident: self.handler.ident.clone(),
            vis: self.handler.vis.clone(),
            attrs: vec![],
            node: syn::ItemKind::Static(
                Box::new(syn::Ty::Path(None, struct_path)),
                syn::Mutability::Immutable,
                Box::new(struct_value_expr.into()),
            ),
        }
    }

    // Construct the impl syn object
    fn create_into_impl(&self) -> syn::Item {
        let tup_data = vec![
            self.shio_method.clone(),
            syn::ExprKind::Lit(self.route.clone()).into(),
            syn::ExprKind::Path(
                None,
                syn::Path::from(format!("__shio_handler_{}", self.handler.ident)),
            ).into(),
        ];

        let tup_data = syn::ExprKind::Tup(tup_data);
        let method_call =
            syn::ExprKind::MethodCall(syn::Ident::from("into"), vec![], vec![tup_data.into()]);

        // block of `Into::into`
        let block = syn::Block {
            stmts: vec![syn::Stmt::Expr(Box::new(method_call.into()))],
        };

        // type of `Into::into`
        let fndecl = syn::FnDecl {
            variadic: false,
            inputs: vec![syn::FnArg::SelfValue(syn::Mutability::Immutable)],
            output: syn::FunctionRetTy::Ty(syn::Ty::Path(
                None,
                syn::Path::from("::shio::router::Route"),
            )),
        };

        // method `Into::into`
        let method_sig = syn::MethodSig {
            unsafety: syn::Unsafety::Normal,
            constness: syn::Constness::NotConst,
            abi: None,
            generics: syn::Generics::default(),
            decl: fndecl,
        };
        let method = syn::ImplItemKind::Method(method_sig, block);

        let impl_item = syn::ImplItem {
            ident: syn::Ident::from("into"),
            vis: syn::Visibility::Inherited,
            defaultness: syn::Defaultness::Final,
            attrs: vec![],
            node: method,
        };

        // trait impl'ed
        let trait_impled = Some(syn::Path::from("Into<::shio::router::Route>"));
        // type impl
        let type_impl = Box::new(syn::Ty::Path(
            None,
            syn::Path::from(format!("__shio_route_{}", self.handler.ident)),
        ));

        // the impl
        let the_impl = syn::ItemKind::Impl(
            syn::Unsafety::Normal,
            syn::ImplPolarity::Positive,
            syn::Generics::default(),
            trait_impled,
            type_impl,
            vec![impl_item],
        );

        syn::Item {
            ident: syn::Ident::from(""),
            vis: syn::Visibility::Inherited,
            attrs: vec![],
            node: the_impl,
        }
    }
}
