#[doc(hidden)]
pub mod __generated_notouchie {
	wit_bindgen::generate!({
		path: "provider-plugin.wit",

		// todo toy with configs if necessary
		ownership: Borrowing {
			duplicate_if_necessary: true
		},
		pub_export_macro: true
	});
}

#[doc(inline)]
pub use self::__generated_notouchie::exports::wiwi::wiwipaccer_provider_plugin::imp::Guest;
#[doc(inline)]
pub use self::__generated_notouchie::wiwi::wiwipaccer_provider_plugin::types::{ Ctx, CtxMethods, PluginMeta };

#[macro_export]
macro_rules! export {
	($component:ident) => {
		$crate::__generated_notouchie::export! {
			$component
			with_types_in $crate::__generated_notouchie
		}
	}
}

#[macro_export]
macro_rules! cargo_pkg_version {
	() => { String::from(env!("CARGO_PKG_VERSION")) }
}
