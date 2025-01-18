use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, PatType};

#[proc_macro_attribute]
pub fn tool(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let struct_name = quote::format_ident!("{}", fn_name.to_string().to_uppercase());
    let fn_name_str = fn_name.to_string();

    let args = input_fn.sig.inputs.iter().filter_map(|arg| {
        if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
            Some((pat, ty))
        } else {
            None
        }
    });

    let arg_names: Vec<_> = args.clone().map(|(pat, _)| pat).collect();
    let arg_types: Vec<_> = args.clone().map(|(_, ty)| ty).collect();

    let args_struct_name = quote::format_ident!("{}Args", struct_name);

    let expanded = quote! {
        #[derive(Debug, serde::Deserialize, serde::Serialize)]
        pub struct #struct_name;

        #[derive(Debug, serde::Deserialize, serde::Serialize)]
        pub struct #args_struct_name {
            #(#arg_names: #arg_types),*
        }

        #input_fn

        impl rig::tool::Tool for #struct_name {
            const NAME: &'static str = #fn_name_str;

            type Error = std::io::Error;
            type Args = #args_struct_name;
            type Output = u64;

            async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {
                rig::completion::ToolDefinition {
                    name: Self::NAME.to_string(),
                    description: format!("Function to {}", Self::NAME),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            #(
                                stringify!(#arg_names): {
                                    "type": "number",
                                    "description": format!("Parameter {}", stringify!(#arg_names))
                                }
                            ),*
                        }
                    }),
                }
            }

            async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
                #fn_name(#(args.#arg_names),*)  // Remove the Ok() wrapper since the function already returns Result
            }
        }
    };

    expanded.into()
}
