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
    TraitBoundModifier, TypeParamBound,
};

fn do_enum_gen(i: usize, var_id: &Ident, field_data: &Punctuated<Field, Comma>) -> TokenStream2 {
    let mut named_fields = false;

    let names: Vec<TokenStream2> = field_data
        .iter()
        .map(|field_gen| {
            let field_name = &field_gen.ident;
            let field_type = &field_gen.ty;
            if let Some(field_name) = field_name {
                named_fields = true;
                quote! {
                    #field_name: RandomVariant::random_variant(rng)
                }
            } else {
                quote! {
                    <#field_type>::random_variant(rng)
                }
            }
        })
        .collect();

    let enum_gen = if !named_fields {
        quote! {
            #i => {
                Self :: #var_id (
                  #( #names ),*
                )
            },
        }
    } else {
        quote! {
            #i => {
                Self :: #var_id {
                  #( #names ),*
                }
            },
        }
    };
    // println!("{}", enum_gen);
    enum_gen
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
                        let variant_gen = do_enum_gen(i, &varid, &fields.unnamed);
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
                        let variant_gen = do_enum_gen(i, &varid, &fields.named);
                        variant_generators.push(variant_gen);
                    }
                }
            }

            let bounded_generics = do_bound_gen(&it.generics);
            let (impl_generics, ty_generics, where_clause) = bounded_generics.split_for_impl();

            let number_variants: usize = variant_generators.len();

            let out = quote! {
                impl #impl_generics RandomVariant for #name #ty_generics #where_clause {
                    fn random_variant<R: random_variant::rand::Rng>(rng: &mut R) -> Self {
                        let u: usize = rng.gen_range(0..#number_variants);
                        match u {
                            #( #variant_generators )*
                           _ => {
                             panic!("RandomVariant macro calculated to many variants ");
                           }
                        }
                    }
                }
            };

            // println!("{}", out);
            out.into()
        }
        Item::Struct(ref it) => {
            // println!("struct: {:#?}", it);

            let name = &it.ident;

            //let mut member_generator = Vec::new();

            // This is used, but the compiler does not seem to detect it?
            #[allow(unused_assignments)]
            let mut structgen = quote! {};

            let mut use_brackets = false;
            // No identity
            if it.fields.iter().any(|f| f.ident.is_none()) {
                let mut variants: Vec<TokenStream2> = Vec::new();
                // Here we come if its an struct with anonymous fields
                for field in it.fields.iter() {
                    if let syn::Type::Path(_path) = field.ty.clone() {
                        variants.push(quote! {
                            RandomVariant::random_variant(rng),
                        })
                    } else {
                        abort!(field, "Ident is missing ");
                    }
                }

                structgen = quote!(
                    #( #variants )*
                )
            } else {
                let fieldgens: Vec<Ident> = it
                    .fields
                    .iter()
                    .filter_map(|field| field.ident.clone())
                    .collect();

                let mut variants: Vec<TokenStream2> = Vec::new();
                for field in fieldgens {
                    variants.push(quote! {
                       #field:  RandomVariant::random_variant(rng),
                    });
                }
                use_brackets = true;
                structgen = quote!(
                    #( #variants )*
                )
            }
            // println!("{}", structgen);

            let bounded_generics = do_bound_gen(&it.generics);
            let (impl_generics, ty_generics, where_clause) = bounded_generics.split_for_impl();
            let out = if use_brackets {
                quote! {
                    impl #impl_generics RandomVariant for #name #ty_generics #where_clause {
                        fn random_variant<R: random_variant::rand::Rng>(rng: &mut R) -> Self {
                            Self {
                                #structgen
                            }

                        }
                    }
                }
            } else {
                quote! {
                    impl #impl_generics RandomVariant for #name #ty_generics #where_clause {
                        fn random_variant<R: random_variant::rand::Rng>(rng: &mut R) -> Self {
                            Self (
                                #structgen
                            )
                        }
                    }
                }
            };

            // println!("{}", out);
            out.into()
        }
        _ => {
            abort!(item, "Only has an effect on enums and structs ");
        }
    }
}
