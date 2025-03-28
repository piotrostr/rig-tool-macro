use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::{parse_macro_input, FnArg, ItemFn, LitStr, PatType, ReturnType, Type};

// Add this struct to parse the description attribute
struct ToolAttr {
    description: Option<String>,
}

impl Parse for ToolAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut description = None;

        if !input.is_empty() {
            let name: syn::Ident = input.parse()?;
            if name == "description" {
                let _: syn::Token![=] = input.parse()?;
                let desc: LitStr = input.parse()?;
                description = Some(desc.value());
            }
        }

        Ok(ToolAttr { description })
    }
}

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

fn get_json_type(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(type_path) => {
            let segment = &type_path.path.segments[0];
            let type_name = segment.ident.to_string();

            // Handle Vec types
            if type_name == "Vec" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let syn::GenericArgument::Type(inner_type) = &args.args[0] {
                        let inner_json_type = get_json_type(inner_type);
                        return quote! {
                            "type": "array",
                            "items": { #inner_json_type }
                        };
                    }
                }
                return quote! { "type": "array", "description": " " }; // TODO add those descriptions
            }

            // Handle primitive types
            match type_name.as_str() {
                "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64" => {
                    quote! { "type": "number", "description": " " }
                }
                "String" | "str" => {
                    quote! { "type": "string", "description": " " }
                }
                "bool" => {
                    quote! { "type": "boolean", "description": " " }
                }
                // Handle other types as objects
                _ => {
                    quote! { "type": "object", "description": " " }
                }
            }
        }
        _ => {
            quote! { "type": "object", "description": " " }
        }
    }
}

#[proc_macro_attribute]
pub fn tool(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as ToolAttr);
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

    let call_impl = if input_fn.sig.asyncness.is_some() {
        quote! {
            async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
                #fn_name(#(args.#arg_names),*).await
                    .map_err(|e| Self::Error::ExecutionError(e.to_string()))
            }
        }
    } else {
        quote! {
            async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
                #fn_name(#(args.#arg_names),*)
                    .map_err(|e| Self::Error::ExecutionError(e.to_string()))
            }
        }
    };

    // Modify the definition implementation to use the description
    let description = match attr.description {
        Some(desc) => quote! { #desc.to_string() },
        None => quote! { format!("Function to {}", Self::NAME) },
    };

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
                    description: #description,
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

            #call_impl
        }

        pub static #static_name: #struct_name = #struct_name;
    };

    expanded.into()
}
