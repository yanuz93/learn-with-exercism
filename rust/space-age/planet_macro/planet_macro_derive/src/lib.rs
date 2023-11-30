use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Planet)]
pub fn planet_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_planet(&ast)
}

fn impl_planet(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Planet for #name {
            fn get_type_name() -> String {
                stringify!(#name).to_string()
            }
            
            fn years_during(d: &Duration) -> f64 {
                let name: String = Self::get_type_name();
                let earth_factor: f64 = match name.as_str() {
                    "Mercury" => 0.2408467,
                    "Venus" => 0.61519726,
                    "Earth" => 1.00,
                    "Mars" => 1.8808158,
                    "Jupiter" => 11.862615,
                    "Saturn" => 29.447498,
                    "Uranus" => 84.016846,
                    "Neptune" => 164.79132,
                    _ => 1.00,
                };

                d.earth_years / earth_factor
            }
        }
    };
    gen.into()
}
