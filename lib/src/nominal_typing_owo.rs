//! Generic, zero-cost (`repr(transparent)`) that wrap data in a newtype so you
//! don't ex. accidentally pass a filepath ([`String`]) as an ID (also [`String`])
//! or something similar, which is something I (Vapor | Meadowsys) am always
//! paranoid about..
//!
//! The struct is `repr(transparent)` and have `#[inline]` on all functions, so
//! it all should be zero cost, optimised away by the rust compiler.

#![allow(clippy::partialeq_ne_impl)]

use serde::de::{ Deserialize, Deserializer, DeserializeOwned, DeserializeSeed };
use serde::ser::{ Serialize, Serializer };
use std::cmp::Ordering;
use std::fmt::{ self, Debug, Display, Formatter };
use std::hash::{ Hash, Hasher };
use std::marker::PhantomData;
use std::ops::{ Deref, DerefMut };

macro_rules! nominal {
	(pub name: $name:ident, marker: $marker:ident, type: $type:ty) => {
		pub struct $marker;
		pub type $name = crate::nominal_typing_owo::Nominal<$type, $marker>;
	};

	(name: $name:ident, marker: $marker:ident, type: $type:ident) => {
		struct $marker;
		type $name = crate::nominal_typing_owo::Nominal<$type, $marker>;
	};
}
pub(crate) use nominal;

#[repr(transparent)]
pub struct Nominal<T, M>(T, PhantomData<M>);

impl<T, M> Nominal<T, M> {
	#[inline]
	pub fn from(value: T) -> Self {
		Self(value, PhantomData)
	}

	#[inline]
	pub fn into(self) -> T {
		self.0
	}
}

impl<T, M> Deref for Nominal<T, M> {
	type Target = T;
	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T, M> DerefMut for Nominal<T, M> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

// delegate trait impls by just calling T's impl

impl<T, M> Clone for Nominal<T, M>
where
	T: Clone
{
	#[inline]
	fn clone(&self) -> Self {
		let t = <T as Clone>::clone(&self.0);
		Self(t, PhantomData)
	}

	#[inline]
	fn clone_from(&mut self, source: &Self) {
		<T as Clone>::clone_from(&mut self.0, source)
	}
}
impl<T, M> Copy for Nominal<T, M> where T: Copy {}

impl<T, M> Debug for Nominal<T, M>
where
	T: Debug
{
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		<T as Debug>::fmt(&self.0, f)
	}
}

impl<T, M> Display for Nominal<T, M>
where
	T: Display
{
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		<T as Display>::fmt(&self.0, f)
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
		<T as Hash>::hash(&self.0, state)
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
		<T as PartialEq>::eq(self, &other.0)
	}

	#[inline]
	fn ne(&self, other: &Nominal<T, M>) -> bool {
		// in case T has overridden ne
		<T as PartialEq>::ne(self, &other.0)
	}
}

impl<T, M> PartialEq<T> for Nominal<T, M>
where
	T: PartialEq<T>
{
	#[inline]
	fn eq(&self, other: &T) -> bool {
		<T as PartialEq>::eq(&self.0, other)
	}

	#[inline]
	fn ne(&self, other: &T) -> bool {
		// in case T has overridden ne
		<T as PartialEq>::ne(&self.0, other)
	}
}

// not possible:
// impl<T, M> PartialEq<Nominal<T, M>> for T

impl<T, M> Eq for Nominal<T, M> where T: Eq {}

impl<T, M> PartialOrd<Nominal<T, M>> for Nominal<T, M>
where
	T: PartialOrd<T>
{
	#[inline]
	fn partial_cmp(&self, other: &Nominal<T, M>) -> Option<Ordering> {
		<T as PartialOrd>::partial_cmp(&self.0, &other.0)
	}

	#[inline]
	fn lt(&self, other: &Nominal<T, M>) -> bool {
		<T as PartialOrd>::lt(&self.0, &other.0)
	}

	#[inline]
	fn le(&self, other: &Nominal<T, M>) -> bool {
		<T as PartialOrd>::le(&self.0, &other.0)
	}

	#[inline]
	fn gt(&self, other: &Nominal<T, M>) -> bool {
		<T as PartialOrd>::gt(&self.0, &other.0)
	}

	#[inline]
	fn ge(&self, other: &Nominal<T, M>) -> bool {
		<T as PartialOrd>::ge(&self.0, &other.0)
	}
}

impl<T, M> PartialOrd<T> for Nominal<T, M>
where
	T: PartialOrd<T>
{
	#[inline]
	fn partial_cmp(&self, other: &T) -> Option<Ordering> {
		<T as PartialOrd>::partial_cmp(&self.0, other)
	}

	#[inline]
	fn lt(&self, other: &T) -> bool {
		<T as PartialOrd>::lt(&self.0, other)
	}

	#[inline]
	fn le(&self, other: &T) -> bool {
		<T as PartialOrd>::le(&self.0, other)
	}

	#[inline]
	fn gt(&self, other: &T) -> bool {
		<T as PartialOrd>::gt(&self.0, other)
	}

	#[inline]
	fn ge(&self, other: &T) -> bool {
		<T as PartialOrd>::ge(&self.0, other)
	}
}

// not possible:
// impl<T, M> PartialOrd<Nominal<T, M>> for T

impl<T, M> Ord for Nominal<T, M>
where
	T: Ord
{
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {
		<T as Ord>::cmp(&self.0, &other.0)
	}

	#[inline]
	fn max(self, other: Self) -> Self
	where
		Self: Sized
	{
		let t = <T as Ord>::max(self.0, other.0);
		Self(t, PhantomData)
	}

	#[inline]
	fn min(self, other: Self) -> Self
	where
		Self: Sized
	{
		let t = <T as Ord>::min(self.0, other.0);
		Self(t, PhantomData)
	}

	#[inline]
	fn clamp(self, min: Self, max: Self) -> Self
	where
		Self: Sized
	{
		let t = <T as Ord>::clamp(self.0, min.0, max.0);
		Self(t, PhantomData)
	}
}

impl<'de, T, M> Deserialize<'de> for Nominal<T, M>
where
	T: Deserialize<'de>
{
	#[inline]
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		<T as Deserialize>::deserialize(deserializer)
			.map(|t| Self(t, PhantomData))
	}

	#[inline]
	fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
	where
		D: Deserializer<'de>
	{
		<T as Deserialize>::deserialize_in_place(deserializer, &mut place.0)
	}
}

impl<T, M> Serialize for Nominal<T, M>
where
	T: Serialize
{
	#[inline]
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		<T as Serialize>::serialize(&self.0, serializer)
	}
}
