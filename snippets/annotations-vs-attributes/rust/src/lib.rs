use proc_macro::TokenStream;

// #region definition
#[proc_macro_attribute]
pub fn data_holder(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
// #endregion definition
