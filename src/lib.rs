use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, FnArg, Ident, ItemFn, LitStr, Pat, PatType, Token};

#[allow(non_snake_case)]
struct KeyValue {
    key: Ident,
    _eq: Token![=],
    value: LitStr,
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(KeyValue {
            key: input.parse()?,
            _eq: input.parse()?,
            value: input.parse()?,
        })
    }
}

struct TransactionArgs {
    pairs: Vec<KeyValue>,
}

impl Parse for TransactionArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut pairs = Vec::new();

        while !input.is_empty() {
            pairs.push(input.parse()?);
            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            }
        }

        Ok(TransactionArgs {pairs})
    }
}

#[proc_macro_attribute]
pub fn transaction(attr: TokenStream, input: TokenStream) -> TokenStream {
    let tx_arg = parse_macro_input!(attr as TransactionArgs);
    let mut db = Ident::new("db", proc_macro2::Span::call_site());
    let mut tx = Ident::new("tx", proc_macro2::Span::call_site());
    let mut is_tx = Ident::new("is_tx", proc_macro2::Span::call_site());
    let mut propagation = String::from("REQUIRED");

    if !tx_arg.pairs.is_empty() {
        for pair in tx_arg.pairs {
            let key = pair.key;
            let value = pair.value.value();
            match key.to_string().as_str() {
                "txManager" => {
                    if !value.eq("tx") {
                        db = Ident::new(format!("db_{value}").as_str(), key.span());
                        tx = Ident::new(format!("tx_{value}").as_str(), key.span());
                        is_tx = Ident::new(format!("is_tx_{value}").as_str(), key.span());
                    }
                },
                "propagation" => propagation = value,
                _ => panic!("unknown attribute transaction"),
            }
        }
    }



    let func = parse_macro_input!(input as ItemFn);
    let func_vis = &func.vis; // like pub
    let func_block = &func.block; // { some statement or expression here }


    let func_decl = func.sig;
    let func_async = &func_decl.asyncness.unwrap();
    let func_name = &func_decl.ident; // function name
    let func_generics = &func_decl.generics;
    let func_where_clause = func_generics.where_clause.as_ref();
    let func_inputs = &func_decl.inputs;
    let func_output = &func_decl.output;

    // Extract parameter names and types
    let mut param_names = Vec::new();
    let mut param_types = Vec::new();

    for input in func_inputs {
        if let FnArg::Typed(PatType { pat, ty, .. }) = input {
            if let Pat::Ident(pat_ident) = &**pat {
                param_names.push(&pat_ident.ident);
                param_types.push(&**ty);
            }
        }
    }

    let gen = quote! {
        #func_vis #func_async fn #func_name #func_generics(#func_inputs) #func_output #func_where_clause {
            use std::sync::Arc;
            use tokio::sync::Mutex;
            use crate::utils::context::{TxManager, TX_MANAGER};
            use log::info;

            let mut tx_manager = TX_MANAGER.get();
            let is_tx = tx_manager.is_tx;

            if (#propagation.eq("REQUIRED") && !is_tx) || #propagation.eq("REQUIRED_NEW") {
                let tx_result = tx_manager.#db.begin().await;
                if let  Err(err) = tx_result {
                    return Err(ErrorApp::OtherErr(err.to_string()));
                }

                tx_manager = TxManager {
                    db: tx_manager.db,
                    tx: Default::default(),
                    is_tx: false,
                };
                tx_manager.#tx = Arc::new(Mutex::new(Some(tx_result.unwrap())));
                tx_manager.#is_tx = true;
            }
            TX_MANAGER.scope(tx_manager, async {
                let result = #func_block;

                if (#propagation.eq("REQUIRED") && !is_tx) || #propagation.eq("REQUIRED_NEW") {
                    let tx_manager = TX_MANAGER.get();
                    let tx_opt = tx_manager.#tx.lock().await.take();
                    if let None = tx_opt {
                        return Err(ErrorApp::OtherErr("tx not found when finished".to_string()));
                    }
                    let tx = tx_opt.unwrap();
                    if let Err(err) = result {
                        if let Err(err_tx) = tx.rollback().await {
                            return Err(ErrorApp::OtherErr(format!("{}: {}", err, err_tx)));
                        }
                        return Err(err);
                    }

                    if let Err(err_tx) = tx.commit().await {
                        return Err(ErrorApp::OtherErr(format!("{}", err_tx)));
                    }
                }

                result
            }).await
        }
    };
    gen.into()
}