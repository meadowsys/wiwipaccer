#[doc(hidden)]
pub mod __generated {
	wit_bindgen::generate!({
		path: "../plugin.wit",

		ownership: Borrowing {
			duplicate_if_necessary: true
		},
		pub_export_macro: true
	});
}

#[doc(inline)]
pub use self::__generated::exports::wiwi::wiwipaccer_provider_plugin::imp::Guest;
#[doc(inline)]
pub use self::__generated::wiwi::wiwipaccer_provider_plugin::types::{ PluginMetadata, PluginVersion };

#[macro_export]
macro_rules! export {
	($component:ident) => {
		$crate::__generated::export!($component with_types_in $crate::__generated);
	}
}
