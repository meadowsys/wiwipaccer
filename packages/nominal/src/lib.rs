//! Generic, zero-cost struct that wraps data in a newtype so you
//! don't ex. accidentally pass a filepath ([`String`]) as an ID (also [`String`])
//! or something similar, which is something I (Vapor | Meadowsys) am always
//! paranoid about..
//!
//! The struct is `repr(transparent)` and have `#[inline]` on all functions, so
//! it all should be zero cost, optimised away by the rust compiler.
//!
//! Nominal typing is great c:

#![allow(clippy::partialeq_ne_impl)]

#[cfg(feature = "serde")]
use ::serde::{
	de::{ Deserialize, Deserializer },
	ser::{ Serialize, Serializer }
};
use ::std::cmp::Ordering;
use ::std::fmt::{ self, Debug, Display, Formatter };
use ::std::hash::{ Hash, Hasher };
use ::std::marker::PhantomData;
use ::std::future::Future;

#[macro_export]
macro_rules! nominal {
	($vis:vis $name:ident, marker: $marker:ident, inner: $( ref <$($lifetimes:lifetime),+> )? $type:ty) => {
		$vis struct $marker;
		$vis type $name$( <$($lifetimes),+> )? = $crate::Nominal<$type, $marker>;
	};
}

#[macro_export]
macro_rules! nominal_mod {
	{
		$(
			$mod_vis:vis mod $mod_name:ident {
				$( nominal!($item_vis:vis $name:ident, inner: $( ref <$($lifetimes:lifetime),+> )? $type:ty); )*
			}
		)*
	} => {
		$(
			$mod_vis mod $mod_name {
				pub mod marker {
					$( pub struct $name; )*
				}

				use super::*;
				$( $item_vis type $name$( <$($lifetimes),+> )? = $crate::Nominal<$type, marker::$name>; )*
			}
		)*
	}
}

#[repr(transparent)]
pub struct Nominal<T, M>(T, PhantomData<M>);

/// Dummy marker type for when it doesn't matter what the marker type is
/// (for example, in the middle of longer function chains)
pub struct Dummy;

impl<T, M> Nominal<T, M> {
	/// Wraps a value into a Nominal struct
	#[inline]
	pub fn new(value: T) -> Self {
		Self(value, PhantomData)
	}

	/// Unwraps and returns the value, consuming the wrapper
	#[inline]
	pub fn into_inner(self) -> T {
		self.0
	}

	/// Gets a reference to inner
	///
	/// Note: [`Deref`][`std::ops::Deref`] is not implemented on purpose,
	/// to prevent unintentional auto-derefs
	#[inline]
	pub fn ref_inner(&self) -> &T {
		&self.0
	}

	/// Gets a mutable reference to inner
	///
	/// Note: [`DerefMut`][`std::ops::DerefMut`] is not implemented on purpose,
	/// to prevent unintentional auto-derefs
	#[inline]
	pub fn mut_inner(&mut self) -> &mut T {
		&mut self.0
	}

	/// Consumes and rewraps self to another nominal type, where the two inner types
	/// are the same of course. Can be type inferred, so may lose a bit of explicitness,
	/// but it still explicitly calls a conversion, so should be clear still? :thinking:
	#[inline]
	pub fn transmute_nom<M2>(self) -> Nominal<T, M2> {
		Nominal(self.0, PhantomData)
	}

	#[inline]
	pub fn map_nom<M2, T2, F>(self, f: F) -> Nominal<T2, M2>
	where
		F: FnOnce(T) -> T2
	{
		Nominal(f(self.0), PhantomData)
	}

	#[inline]
	pub async fn async_map_nom<M2, T2, F, Fu>(self, f: F) -> Nominal<T2, M2>
	where
		F: FnOnce(T) -> Fu,
		Fu: Future<Output = T2>
	{
		Nominal(f(self.0).await, PhantomData)
	}
}

impl<T, M, E> Nominal<Result<T, E>, M> {
	#[inline]
	pub fn transpose(self) -> Result<Nominal<T, M>, E> {
		self.0.map(|val| Nominal(val, PhantomData))
	}

	#[inline]
	pub fn map_nom_ok<M2, T2, F>(self, f: F) -> Nominal<Result<T2, E>, M2>
	where
		F: FnOnce(T) -> T2
	{
		Nominal(self.0.map(f), PhantomData)
	}

	#[inline]
	pub fn map_nom_err<M2, E2, F>(self, f: F) -> Nominal<Result<T, E2>, M2>
	where
		F: FnOnce(E) -> E2
	{
		Nominal(self.0.map_err(f), PhantomData)
	}
}

impl<T, M> Nominal<Option<T>, M> {
	#[inline]
	pub fn transpose(self) -> Option<Nominal<T, M>> {
		self.0.map(|val| Nominal(val, PhantomData))
	}

	#[inline]
	pub fn map_nom_some<M2, T2, F>(self, f: F) -> Nominal<Option<T2>, M2>
	where
		F: FnOnce(T) -> T2
	{
		Nominal(self.0.map(f), PhantomData)
	}
}

// delegate trait impls by just calling T's impl

impl<T, M> Clone for Nominal<T, M>
where
	T: Clone
{
	#[inline]
	fn clone(&self) -> Self {
		let t = <T as Clone>::clone(self.ref_inner());
		Self(t, PhantomData)
	}

	#[inline]
	fn clone_from(&mut self, source: &Self) {
		<T as Clone>::clone_from(self.mut_inner(), source.ref_inner())
	}
}
impl<T, M> Copy for Nominal<T, M> where T: Copy {}

impl<T, M> Debug for Nominal<T, M>
where
	T: Debug
{
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		<T as Debug>::fmt(self.ref_inner(), f)
	}
}

impl<T, M> Display for Nominal<T, M>
where
	T: Display
{
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		<T as Display>::fmt(self.ref_inner(), f)
	}
}

impl<T, M> Default for Nominal<T, M>
where
	T: Default
{
	#[inline]
	fn default() -> Self {
		let t = <T as Default>::default();
		Self(t, PhantomData)
	}
}

impl<T, M> Hash for Nominal<T, M>
where
	T: Hash
{
	#[inline]
	fn hash<H: Hasher>(&self, state: &mut H) {
		<T as Hash>::hash(self.ref_inner(), state)
	}

	#[inline]
	fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
	where
		Self: Sized
	{
		// SAFETY: we're repr(transparent)
		let t_data = unsafe { &*(data as *const [Self] as *const [T]) };
		<T as Hash>::hash_slice(t_data, state)
	}
}

impl<T, M> PartialEq<Nominal<T, M>> for Nominal<T, M>
where
	T: PartialEq<T>
{
	#[inline]
	fn eq(&self, other: &Nominal<T, M>) -> bool {
		<T as PartialEq>::eq(self.ref_inner(), other.ref_inner())
	}

	#[inline]
	fn ne(&self, other: &Nominal<T, M>) -> bool {
		// in case T has overridden ne
		<T as PartialEq>::ne(self.ref_inner(), other.ref_inner())
	}
}

impl<T, M> Eq for Nominal<T, M> where T: Eq {}

impl<T, M> PartialOrd<Nominal<T, M>> for Nominal<T, M>
where
	T: PartialOrd<T>
{
	#[inline]
	fn partial_cmp(&self, other: &Nominal<T, M>) -> Option<Ordering> {
		<T as PartialOrd>::partial_cmp(self.ref_inner(), other.ref_inner())
	}

	#[inline]
	fn lt(&self, other: &Nominal<T, M>) -> bool {
		<T as PartialOrd>::lt(self.ref_inner(), other.ref_inner())
	}

	#[inline]
	fn le(&self, other: &Nominal<T, M>) -> bool {
		<T as PartialOrd>::le(self.ref_inner(), other.ref_inner())
	}

	#[inline]
	fn gt(&self, other: &Nominal<T, M>) -> bool {
		<T as PartialOrd>::gt(self.ref_inner(), other.ref_inner())
	}

	#[inline]
	fn ge(&self, other: &Nominal<T, M>) -> bool {
		<T as PartialOrd>::ge(self.ref_inner(), other.ref_inner())
	}
}

impl<T, M> Ord for Nominal<T, M>
where
	T: Ord
{
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {
		<T as Ord>::cmp(self.ref_inner(), other.ref_inner())
	}

	#[inline]
	fn max(self, other: Self) -> Self
	where
		Self: Sized
	{
		let t = <T as Ord>::max(self.into_inner(), other.into_inner());
		Self(t, PhantomData)
	}

	#[inline]
	fn min(self, other: Self) -> Self
	where
		Self: Sized
	{
		let t = <T as Ord>::min(self.into_inner(), other.into_inner());
		Self(t, PhantomData)
	}

	#[inline]
	fn clamp(self, min: Self, max: Self) -> Self
	where
		Self: Sized
	{
		let t = <T as Ord>::clamp(self.into_inner(), min.into_inner(), max.into_inner());
		Self(t, PhantomData)
	}
}

#[cfg(feature = "serde")]
impl<'de, T, M> Deserialize<'de> for Nominal<T, M>
where
	T: Deserialize<'de>
{
	#[inline]
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		<T as Deserialize>::deserialize::<D>(deserializer)
			.map(|t| Self(t, PhantomData))
	}

	#[inline]
	fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
	where
		D: Deserializer<'de>
	{
		<T as Deserialize>::deserialize_in_place::<D>(deserializer, place.mut_inner())
	}
}

#[cfg(feature = "serde")]
impl<T, M> Serialize for Nominal<T, M>
where
	T: Serialize
{
	#[inline]
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		<T as Serialize>::serialize::<S>(self.ref_inner(), serializer)
	}
}
