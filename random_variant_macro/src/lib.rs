//! Proc macros that can help with generating implements of
//! the EveryVariant trait for structs and enums
extern crate syn;

use quote::*;

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};

use proc_macro_error::{abort, proc_macro_error};
use syn::{
    punctuated::Punctuated, token::Comma, Field, GenericParam, Generics, Ident, Item, TraitBound,
    TraitBoundModifier, Type, TypeParamBound, TypePath,
};

#[derive(Debug)]
struct StructFieldGen {
    name: Ident,
    ty: TypePath,
}

#[derive(Debug)]
struct AnonStructFieldGen {
    id: Ident,
    ty: TypePath,
}

#[derive(Debug)]
struct EnumFieldGen {
    id: Ident,
    ty: Type,
    name: Option<Ident>,
}

fn do_enum_gen(var_id: &Ident, field_data: &Punctuated<Field, Comma>) -> TokenStream2 {
    let mut field_gen = Vec::new();
    for (idx, field) in field_data.iter().enumerate() {
        field_gen.push(EnumFieldGen {
            id: Ident::new(&format!("v{}", &idx.to_string()), Span::call_site()),
            ty: field.ty.clone(),
            name: field.ident.clone(),
        });
    }

    let mut named_fields = false;

    let names: Vec<TokenStream2> = field_gen
        .iter()
        .map(|field_gen| {
            let field_id = &field_gen.id;
            let field_name = &field_gen.name;
            if let Some(field_name) = field_name {
                named_fields = true;
                quote! {
                    #field_name: #field_id
                }
            } else {
                quote! {
                    #field_id
                }
            }
        })
        .collect();

    let mut enum_gen = if !named_fields {
        quote! {
            let s = Self :: #var_id (
              #( #names.clone() ),*
            );
            vec.push(s);
        }
    } else {
        quote! {
            let s = Self :: #var_id {
              #( #names.clone() ),*
            };
            vec.push(s);
        }
    };

    for field in field_gen.iter().rev() {
        let fname = &field.id;
        let ftype = &field.ty;

        enum_gen = quote! {
            for #fname in <#ftype as EveryVariant>::every_variant() {
                #enum_gen
            }
        };
    }

    let variant_gen = quote! {
        #enum_gen
    };
    println!("{}", variant_gen);

    variant_gen
}

fn do_bound_gen(generics: &Generics) -> Generics {
    let mut generics = generics.clone();

    fn make_bound(name: &str) -> TypeParamBound {
        TypeParamBound::Trait(TraitBound {
            paren_token: None,
            modifier: TraitBoundModifier::None,
            lifetimes: None,
            path: Ident::new(name, Span::call_site()).into(),
        })
    }

    let everyvariant_bound = make_bound("RandomVariant");
    // let clone_bound = make_bound("Clone");
    // let sized_bound = make_bound("Sized");

    for param in &mut generics.params {
        match param {
            GenericParam::Type(some_type) => {
                some_type.bounds.push(everyvariant_bound.clone());
                // some_type.bounds.push(clone_bound.clone());
                // some_type.bounds.push(sized_bound.clone());
            }
            _ => {}
        }
    }
    generics
}

#[proc_macro_error]
#[proc_macro_derive(RandomVariant)]
pub fn derive_every_variant(item: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(item).expect("Failed to parse input item ");

    match item {
        Item::Enum(ref it) => {
            //println!("Enum: {}", it.ident);
            let _attrs = &it.attrs;

            let name = &it.ident;

            let variants = &it.variants;

            let mut variant_generators = Vec::new();
            for (i, var) in variants.iter().enumerate() {
                let varid = &var.ident;

                match var.fields {
                    syn::Fields::Unnamed(ref fields) => {
                        let variant_gen = do_enum_gen(&varid, &fields.unnamed);
                        variant_generators.push(variant_gen);
                        //println!("quote: {:?}", layeridarm.to_string());
                    }
                    syn::Fields::Unit => {
                        // Generate a parsing arm for unit structs
                        let variant_gen = quote! {
                            #i => Self::#varid,
                        };
                        variant_generators.push(variant_gen);
                    }
                    syn::Fields::Named(ref fields) => {
                        let variant_gen = do_enum_gen(&varid, &fields.named);
                        variant_generators.push(variant_gen);
                    }
                }
            }

            let bounded_generics = do_bound_gen(&it.generics);
            let (impl_generics, ty_generics, where_clause) = bounded_generics.split_for_impl();

            let number_variants: usize = 2;

            let out = quote! {
                impl #impl_generics RandomVariant for #name #ty_generics #where_clause {
                    fn random_variant<R: rand::Rng>(rng: &mut R) -> Self {
                        let u: usize = rng.gen_range(0..=#number_variants);
                        match u {
                            #( #variant_generators )*
                           _ => {
                             panic!("RandomVariant macro calculated to many variants ");
                           }
                        }
                    }
                }
            };

            println!("{}", out);
            out.into()
        }
        Item::Struct(ref it) => {
            // println!("struct: {:#?}", it);

            let name = &it.ident;

            //let mut member_generator = Vec::new();

            // This is used, but the compiler does not seem to detect it?
            #[allow(unused_assignments)]
            let mut structgen = quote! {};

            if it.fields.iter().any(|f| f.ident.is_none()) {
                let mut fieldgens = Vec::new();
                // Here we come if its an struct with anonymous fields
                for (idx, field) in it.fields.iter().enumerate() {
                    if let syn::Type::Path(path) = field.ty.clone() {
                        fieldgens.push(AnonStructFieldGen {
                            id: Ident::new(&format!("v{}", &idx.to_string()), Span::call_site()),
                            ty: path.clone(),
                        });
                    } else {
                        abort!(field, "Ident is missing ");
                    }
                }

                let names: Vec<Ident> = fieldgens.iter().map(|f| f.id.clone()).collect();
                structgen = quote! {
                    let s = #name(
                      #( #names.clone() ),*
                    );
                    vec.push(s);
                };

                for field in fieldgens.iter().rev() {
                    let fname = &field.id;
                    let ftype = &field.ty;

                    structgen = quote! {
                        for #fname in <#ftype as EveryVariant>::every_variant() {
                            #structgen
                        }
                    };
                }
                // println!("{}", structgen);
            } else {
                let mut fieldgens = Vec::new();
                for field in it.fields.iter() {
                    if let Some(name) = field.ident.clone() {
                        if let syn::Type::Path(path) = field.ty.clone() {
                            let fieldgen = StructFieldGen { name, ty: path };
                            fieldgens.push(fieldgen);
                        } else {
                            abort!(field, "Ident is missing ");
                        }
                    }
                }

                let names: Vec<Ident> = fieldgens.iter().map(|f| f.name.clone()).collect();
                structgen = quote! {
                      #( #names: RandomVariant::random_variant(rng) ),*
                };

                // for field in fieldgens.iter().rev() {
                //     let fname = &field.name;
                //     let ftype = &field.ty;
                //     // println!("type: {}, dbg: {:?}", ftype.to_token_stream(), ftype);

                //     structgen = quote! {
                //         #ftype as EveryVariant>::every_variant() {
                //             #structgen
                //         }
                //     };
                // }
            }

            let bounded_generics = do_bound_gen(&it.generics);
            let (impl_generics, ty_generics, where_clause) = bounded_generics.split_for_impl();

            let out = quote! {
                impl #impl_generics RandomVariant for #name #ty_generics #where_clause {
                    fn random_variant<R: rand::Rng>(rng: &mut R) -> Self {
                        Self {
                            #structgen
                        }
                    }
                }
            };

            println!("{}", out);
            out.into()
        }
        _ => {
            abort!(item, "Only has an effect on enums and structs ");
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
