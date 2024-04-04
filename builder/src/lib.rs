
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let input_name = input.ident.clone();

    let builder_name = format_ident!("{}Builder", input.ident.clone());
    
    let mut fields = match input.data.clone() {
        Data::Struct(ds) => {
            match ds.fields {
                Fields::Named(fields) => {
                    fields.named
                },
                _=> {
                    panic!("Builder only works on sruct with named fields")
                }
            }
        },
        _=> {
            panic!("Builder only works on structs")
        }
        
    };

    let fields_names = fields.iter().map(|fd| fd.clone().ident.expect("extarcting field names")).collect::<Vec<_>>();
    let fields_types = fields.iter().map(|fd| fd.clone().ty).collect::<Vec<_>>();

    fields.iter_mut().for_each(|f| {
        let kind = &mut f.ty;
        *kind = Type::Verbatim(
            quote!( Option<#kind>)
        );
    });

    let mut build_fields = fields.clone();
    build_fields.iter_mut().for_each(|fd|{
        fd.ty = Type::Verbatim(
            quote!( None)
        );
    });

    


    let tokens = quote!{

        impl #input_name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #build_fields
                }
            }
        }

        pub struct #builder_name {
            #fields
        }
        
        impl #builder_name {

            #(
                pub fn #fields_names (&mut self, var : #fields_types ) {}

            )*
        }
    };

    tokens.into()
}
