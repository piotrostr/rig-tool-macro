use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, PatType, ReturnType, Type};

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}

// TODO add support for more types
// * vec doesnt work atm
// * check the format rig expects vectors/nested objects, make this recursive
fn get_json_type(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(type_path) => {
            let type_name = &type_path.path.segments[0].ident.to_string();
            match type_name.as_str() {
                "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64" => {
                    quote! { "type": "number" }
                }
                "String" | "str" => {
                    quote! { "type": "string" }
                }
                "bool" => {
                    quote! { "type": "boolean" }
                }
                // TODO add support for custom types (assuming they're objects)
                _ => {
                    quote! { "type": "object" }
                }
            }
        }
        _ => quote! { "type": "object" },
    }
}

#[proc_macro_attribute]
pub fn tool(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();
    let struct_name = quote::format_ident!("{}Tool", to_pascal_case(&fn_name_str));
    let static_name = quote::format_ident!("{}", to_pascal_case(&fn_name_str));
    let error_name = quote::format_ident!("{}Error", struct_name);

    // Extract return type
    let return_type = if let ReturnType::Type(_, ty) = &input_fn.sig.output {
        if let Type::Path(type_path) = ty.as_ref() {
            if type_path.path.segments[0].ident == "Result" {
                if let syn::PathArguments::AngleBracketed(args) =
                    &type_path.path.segments[0].arguments
                {
                    if let syn::GenericArgument::Type(t) = &args.args[0] {
                        t
                    } else {
                        panic!("Expected type argument in Result")
                    }
                } else {
                    panic!("Expected angle bracketed arguments in Result")
                }
            } else {
                ty.as_ref()
            }
        } else {
            ty.as_ref()
        }
    } else {
        panic!("Function must return a Result")
    };

    let args = input_fn.sig.inputs.iter().filter_map(|arg| {
        if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
            Some((pat, ty))
        } else {
            None
        }
    });

    let arg_names: Vec<_> = args.clone().map(|(pat, _)| pat).collect();
    let arg_types: Vec<_> = args.clone().map(|(_, ty)| ty).collect();
    let json_types: Vec<_> = arg_types.iter().map(|ty| get_json_type(ty)).collect();

    let args_struct_name = quote::format_ident!("{}Args", to_pascal_case(&fn_name_str));

    let expanded = quote! {
        #[derive(Debug, thiserror::Error)]
        pub enum #error_name {
            #[error("Tool execution failed: {0}")]
            ExecutionError(String),
        }

        #[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
        pub struct #struct_name;

        #[derive(Debug, serde::Deserialize, serde::Serialize)]
        pub struct #args_struct_name {
            #(#arg_names: #arg_types),*
        }

        #input_fn

        impl rig::tool::Tool for #struct_name {
            const NAME: &'static str = #fn_name_str;

            type Error = #error_name;
            type Args = #args_struct_name;
            type Output = #return_type;

            async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {
                rig::completion::ToolDefinition {
                    name: Self::NAME.to_string(),
                    description: format!("Function to {}", Self::NAME),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            #(
                                stringify!(#arg_names): {
                                    #json_types,
                                    "description": format!("Parameter {}", stringify!(#arg_names))
                                }
                            ),*
                        },
                    }),
                }
            }

            async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
                #fn_name(#(args.#arg_names),*)
                    .map_err(|e| Self::Error::ExecutionError(e.to_string()))
            }
        }

        pub static #static_name: #struct_name = #struct_name;
    };

    expanded.into()
}
