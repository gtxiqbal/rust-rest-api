use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, FnArg, ItemFn, Pat, PatType};

#[proc_macro_derive(TransactionDB)]
pub fn transaction_derive_macro(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;
    let gen = quote! {
        use crate::repositories::transaction::TransactionDB;

        impl TransactionDB for #name {
            async fn begin(&self) -> Result<sqlx::Transaction<Postgres>, ErrorApp> {
                let result = self.conn.begin().await;
                if let Ok(tx) = result {
                    return Ok(tx);
                }
                Err(ErrorApp::OtherErr(result.err().unwrap().to_string()))
            }

            async fn commit<'a>(&self, tx: sqlx::Transaction<'a, Postgres>) -> Result<(), ErrorApp> {
                match tx.commit().await {
                    Ok(_) => Ok(()),
                    Err(err) => Err(match err {
                        Error::Database(err_db) => match err_db.kind() {
                            ErrorKind::UniqueViolation => ErrorApp::DuplicateKey,
                            _ => ErrorApp::OtherErr(err_db.to_string()),
                        },
                        _ => ErrorApp::OtherErr(err.to_string()),
                    }),
                }
            }

            async fn rollback<'a>(&self, tx: sqlx::Transaction<'a, Postgres>) -> Result<(), ErrorApp> {
                match tx.rollback().await {
                    Ok(_) => Ok(()),
                    Err(err) => Err(ErrorApp::OtherErr(err.to_string())),
                }
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn transaction(_attr: TokenStream, input: TokenStream) -> TokenStream {
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

            let tx_manager = TX_MANAGER.get();
            let tx_result = tx_manager.db.begin().await;
            if let  Err(err) = tx_result {
                return Err(ErrorApp::OtherErr(err.to_string()));
            }

            let tx_manager = TxManager {
                db: tx_manager.db,
                tx: Arc::new(Mutex::new(Some(tx_result.unwrap()))),
                is_tx: false,
            };
            TX_MANAGER.scope(tx_manager, async {
                let result = #func_block;

                let tx_manager = TX_MANAGER.get();
                let tx_opt = tx_manager.tx.lock().await.take();
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
                
                result
            }).await
        }
    };
    gen.into()
}