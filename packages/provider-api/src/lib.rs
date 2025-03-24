#[doc(hidden)]
pub mod __generated_notouchie {
	wit_bindgen::generate!({
		path: "provider-plugin.wit",

		ownership: Borrowing {
			duplicate_if_necessary: true
		},
		pub_export_macro: true
	});
}

#[doc(inline)]
pub use self::__generated_notouchie::wiwi::wiwipaccer_provider_plugin::types::PluginMeta;

pub trait Guest {
	fn meta() -> PluginMeta;
}

impl<T> self::__generated_notouchie::exports::wiwi::wiwipaccer_provider_plugin::imp::Guest for T
where
	T: Guest
{
	#[inline]
	fn meta() -> PluginMeta {
		<T as Guest>::meta()
	}
}

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
