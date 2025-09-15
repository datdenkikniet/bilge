use std::{num::ParseIntError, str::FromStr};

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BitSize(pub u8);

impl BitSize {
    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl FromStr for BitSize {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = u8::from_str(s)?;
        Ok(Self(value))
    }
}

impl std::fmt::Display for BitSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToTokens for BitSize {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // TODO: attach call site?
        let ident = syn::LitInt::new(&format!("{}", self.0), Span::call_site());
        ident.to_tokens(tokens);
    }
}
