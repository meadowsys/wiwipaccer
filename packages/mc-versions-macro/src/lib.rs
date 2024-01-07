use ::proc_macro::TokenStream;

mod generate;

#[proc_macro]
pub fn inject_generated_mc_versions(input: TokenStream) -> TokenStream {
	generate::inject_generated_mc_versions(input.into()).into()
}
