extern crate proc_macro;

use quote::quote;
use syn::parse_macro_input;

/// This example demonstrates a useful pattern. Not only can you use
/// `#[access_control(CTRL_FN)]` to ensure any invariants or preconditions hold prior to
/// executing an instruction, but also it can be used to finish any validation
/// on the `Accounts` struct, particularly when instruction arguments are
/// needed. Here, we use the given `bump_seed` to verify it creates a valid
/// program-derived address.
#[proc_macro_attribute]
pub fn access_ctrl(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut args = args.to_string();
    args.retain(|c| !c.is_whitespace());
    let access_control: Vec<proc_macro2::TokenStream> = args
        .split('+')
        .filter(|ac| !ac.is_empty())
        .map(|ac| format!("{}?;", ac)) // Add `?;` syntax.
        .map(|ac| ac.parse().unwrap())
        .collect();

    let item_fn = parse_macro_input!(input as syn::ItemFn);

    let fn_attrs = item_fn.attrs;
    let fn_vis = item_fn.vis;
    let fn_sig = item_fn.sig;
    let fn_block = item_fn.block;

    let fn_stmts = fn_block.stmts;

    proc_macro::TokenStream::from(quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {

            #(#access_control)*

            #(#fn_stmts)*
        }
    })
}

#[proc_macro_attribute]
pub fn config_item(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input_struct = parse_macro_input!(input as syn::ItemStruct);
    let config_name = &input_struct.ident;
    proc_macro::TokenStream::from(quote! {
        pub const CONFIG: Item<#config_name> = Item::new("CONFIG");

        #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
        #[serde(rename_all = "snake_case")]
        #input_struct
    })
}
