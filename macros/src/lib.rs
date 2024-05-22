use std::collections::HashSet;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse,
    parse_macro_input,
    AttrStyle, Attribute, Item, ItemFn, Stmt,
};

#[proc_macro_attribute]
pub fn local_static(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut f = parse_macro_input!(input as ItemFn);

    // check the function signature
    // let valid_signature = f.sig.constness.is_none()
    //     && f.vis == Visibility::Inherited
    //     && f.sig.abi.is_none()
    //     && f.sig.inputs.is_empty()
    //     && f.sig.generics.params.is_empty()
    //     && f.sig.generics.where_clause.is_none()
    //     && f.sig.variadic.is_none()
    //     && match f.sig.output {
    //         ReturnType::Default => false,
    //         ReturnType::Type(_, ref ty) => matches!(**ty, Type::Never(_)),
    //     };

    // if !valid_signature {
    //     return parse::Error::new(
    //         f.span(),
    //         "`#[entry]` function must have signature `[unsafe] fn() -> !`",
    //     )
    //     .to_compile_error()
    //     .into();
    // }

    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    // XXX should we blacklist other attributes?
    let stmts = match convert_static_muts(f.block.stmts) {
        Err(e) => return e.to_compile_error().into(),
        Ok(x) => x,
    };

    f.block.stmts = stmts;

    return quote!( #f ).into();

    // XXX should we blacklist other attributes?
    // let (statics, stmts) = match extract_static_muts(f.block.stmts) {
    //     Err(e) => return e.to_compile_error().into(),
    //     Ok(x) => x,
    // };

    // f.sig.ident = Ident::new(&format!("__cortex_m_rt_{}", f.sig.ident), Span::call_site());
    // f.sig.inputs.extend(statics.iter().map(|statik| {
    //     let ident = &statik.ident;
    //     let ty = &statik.ty;
    //     let attrs = &statik.attrs;

    //     // Note that we use an explicit `'static` lifetime for the entry point arguments. This makes
    //     // it more flexible, and is sound here, since the entry will not be called again, ever.
    //     syn::parse::<FnArg>(
    //         quote!(#[allow(non_snake_case)] #(#attrs)* #ident: &'static mut #ty).into(),
    //     )
    //     .unwrap()
    // }));
    // f.block.stmts = stmts;

    // let tramp_ident = Ident::new(&format!("{}_trampoline", f.sig.ident), Span::call_site());
    // let ident = &f.sig.ident;

    // let resource_args = statics
    //     .iter()
    //     .map(|statik| {
    //         let (ref cfgs, ref attrs) = extract_cfgs(statik.attrs.clone());
    //         let ident = &statik.ident;
    //         let ty = &statik.ty;
    //         let expr = &statik.expr;
    //         quote! {
    //             #(#cfgs)*
    //             {
    //                 #(#attrs)*
    //                 static mut #ident: #ty = #expr;
    //                 &mut #ident
    //             }
    //         }
    //     })
    //     .collect::<Vec<_>>();

    // if let Err(error) = check_attr_whitelist(&f.attrs, WhiteListCaller::Entry) {
    //     return error;
    // }

    // let (ref cfgs, ref attrs) = extract_cfgs(f.attrs.clone());

    // quote!(
    //     #(#cfgs)*
    //     #(#attrs)*
    //     #[doc(hidden)]
    //     #[export_name = "main"]
    //     pub unsafe extern "C" fn #tramp_ident() {
    //         #ident(
    //             #(#resource_args),*
    //         )
    //     }

    //     #f
    // )
    // .into()
}


/// Convert `static mut` vars to `&'static mut`
fn convert_static_muts(
    stmts: impl IntoIterator<Item = Stmt>,
) -> Result<Vec<Stmt>, parse::Error> {
    let mut istmts = stmts.into_iter();

    let mut stmts = vec![];

    let converted = syn::parse::<Stmt>(quote!{
        {
            use portable_atomic::{AtomicBool, Ordering};

            static ENTER_ONCE: AtomicBool = AtomicBool::new(false);

            if ENTER_ONCE.swap(true, Ordering::SeqCst) == true {
                panic!("function with local_static can only be called once");
            }

        }
    }.into())?;
    stmts.push(converted);

    let mut seen = HashSet::new();
    for stmt in istmts.by_ref() {
        match &stmt {
            Stmt::Item(Item::Static(var)) => match var.mutability {
                syn::StaticMutability::Mut(_) => {

                    let ident = var.ident.clone();
                    let (cfgs, _) = extract_cfgs(var.attrs.clone());
                    let converted = syn::parse::<Stmt>(quote!{
                        #(#cfgs)*
                        #[allow(non_snake_case)]
                        let #ident = {
                            #stmt;
                            unsafe { &mut #ident }
                        };
                    }.into())?;
                    stmts.push(converted);

                    // Test for duplicate definitions.
                    // NOTE: This will have false positives with mutually exclusive #[cfg] attributes pairs on static muts with the same name.
                    if seen.contains(&var.ident) {
                        let compile_error = parse::Error::new(
                            var.ident.span(),
                            format!("the name `{}` is defined multiple times", var.ident),
                        ).into_compile_error();
                        let duplicate_err = syn::parse::<Stmt>(quote!{
                            #compile_error
                        }.into())?;
                        stmts.push(duplicate_err);
                    }
                    seen.insert(var.ident.clone());
                }
                _ => stmts.push(Stmt::Item(Item::Static(var.clone()))),
            },
            _ => {
                stmts.push(stmt);
            }
        }
    }

    Ok(stmts)
}

fn extract_cfgs(attrs: Vec<Attribute>) -> (Vec<Attribute>, Vec<Attribute>) {
    let mut cfgs = vec![];
    let mut not_cfgs = vec![];

    for attr in attrs {
        if eq(&attr, "cfg") {
            cfgs.push(attr);
        } else {
            not_cfgs.push(attr);
        }
    }

    (cfgs, not_cfgs)
}

// fn check_attr_whitelist(attrs: &[Attribute]) -> Result<(), TokenStream> {
//     let whitelist = &[
//         "doc",
//         "link_section",
//         "cfg",
//         "allow",
//         "warn",
//         "deny",
//         "forbid",
//         //"cold",
//         //"naked",
//     ];

//     'o: for attr in attrs {
//         for val in whitelist {
//             if eq(attr, val) {
//                 continue 'o;
//             }
//         }

//         return Err(parse::Error::new(attr.span(), "this attribute is not allowed on a static mut within a local_static function")
//             .to_compile_error()
//             .into());
//     }

//     Ok(())
// }

/// Returns `true` if `attr.path` matches `name`
fn eq(attr: &Attribute, name: &str) -> bool {
    attr.style == AttrStyle::Outer && attr.path().is_ident(name)
}
