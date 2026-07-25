extern crate proc_macro;

use crate::proc_macro::TokenStream;

#[proc_macro_derive(Lens)]
pub fn lens_derive(input: TokenStream) -> TokenStream
{
    let proc_macro_crate::FoundCrate::Name(grui) = proc_macro_crate::crate_name("gru-ui")
        .expect("gru-ui is present in `Cargo.toml`")
    else { panic!("package schenanigans") };
    let grui = quote::format_ident!("{grui}");

    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let mut lenses = quote::quote!();
    if let syn::Data::Struct(data) = input.data
    {
        if let syn::Fields::Named(fields) = data.fields
        {
            for field in fields.named
            {
                if let Some(attribute) = field.ident
                {
                    let lens = quote::format_ident!("{}_{}_{}", "Lens", name, attribute);
                    let ty = field.ty;
                    lenses.extend(quote::quote!
                    (
                        #[allow(non_camel_case_types)]
                        #[derive(Clone, Copy)]
                        pub struct #lens;

                        impl #generics #grui::lens::Projector<#name #generics, #ty> for #lens
                        {
                            #[inline]
                            fn project(self, data: &#name #generics) -> &#ty
                            {
                                &data.#attribute
                            }

                            #[inline]
                            fn project_mut(self, data: &mut #name #generics) -> &mut #ty
                            {
                                &mut data.#attribute
                            }
                        }

                        impl #generics #grui::lens::Lens<#name #generics, #ty> for #lens
                        {
                            #[inline]
                            fn with<A, F: FnOnce(&#ty) -> A>(&mut self, data: &#name #generics, f: F) -> A
                            {
                                f(&data.#attribute)
                            }

                            #[inline]
                            fn with_mut<A, F: FnOnce(&mut #ty) -> A>(&mut self, data: &mut #name #generics, f: F) -> A
                            {
                                f(&mut data.#attribute)
                            }
                        }

                        impl #generics #name #generics
                        {
                            #[allow(non_upper_case_globals)]
                            pub const #attribute: #lens = #lens;
                        }
                    ));
                } else { panic!("Only named fields allowed."); }
            }
        } else { panic!("Only named fields allowed."); }
    } else { panic!("Only structs allowed."); }
    //println!("{}", lenses);
    TokenStream::from(lenses)
}
