//! provides a macro used for compile-time parsing of public key strings into byte arrays for near 0-cost static public keys.

use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, LitByte, LitStr};


/// parses a string literal public key into a byte array public key
///
/// # Arguments
///
/// * `input` - A public key string
///
/// # Examples
///
/// ```
/// use static_pubkey::static_pubkey;
/// let key = static_pubkey!("GjphYQcbP1m3FuDyCTUJf2mUMxKPE3j6feWU1rxvC7Ps");
/// assert!(key.to_string() == "GjphYQcbP1m3FuDyCTUJf2mUMxKPE3j6feWU1rxvC7Ps");
/// ```
#[proc_macro]
pub fn static_pubkey(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use std::convert::TryFrom;
    let id_literal = parse_macro_input!(input as LitStr);
    let id_vec = bs58::decode(id_literal.value())
        .into_vec()
        .map_err(|_| syn::Error::new_spanned(&id_literal, "failed to decode base58 string"))
        .unwrap();
    let id_array = <[u8; 32]>::try_from(<&[u8]>::clone(&&id_vec[..]))
        .map_err(|_| {
            syn::Error::new_spanned(
                &id_literal,
                format!("pubkey array is not 32 bytes long: len={}", id_vec.len()),
            )
        })
        .unwrap();
    let bytes = id_array.iter().map(|b| LitByte::new(*b, Span::call_site()));
    let output = quote! {
        solana_program::pubkey::Pubkey::new_from_array(
            [#(#bytes,)*]
        )
    };
    output.into()
}