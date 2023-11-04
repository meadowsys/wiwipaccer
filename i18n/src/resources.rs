use ahash::RandomState;
use fluent_syntax::ast;
use fluent_syntax::parser::Slice;
use hashbrown::HashMap;
use std::rc::Rc;

struct Resources(Map<String, Resource>);

struct Resource(Map<String, Entry>);

enum Entry {
	Message(Message),
	Term(Term),
	Attrs(Attrs)
}

struct Message {
	content: Pattern,
	attrs: Option<Attrs>
}

struct Term {
	content: Pattern,
	attrs: Option<Attrs>
}

struct Attrs(Map<String, Pattern>);

enum Pattern {
	Text(PatternText),
	Expression(PatternExpression),
	Complex(PatternComplex)
}

struct PatternText(String);

struct PatternExpression(Expr);

struct PatternComplex(Vec<StringOrExpr>);

enum StringOrExpr {
	String(String),
	Expr(Expr)
}

enum Expr {
	Inline(InlineExpr),
	Select {
		selector: InlineExpr,
		variants: Map<SelectVariantKey, Rc<SelectVariant>>,
		default_variant: Rc<SelectVariant>
	}
}

enum InlineExpr {
	String(String),
	Number(String), // TODO should this be parsed into a number already? or string is fine? should i publish my bignum package?
	FunctionRef, // TODO
	MesssageRef, // TODO
	TermRef, // TODO
	VarRef, // TODO
	Expr(Box<Expr>)
}

enum SelectVariantKey {
	String(String),
	Number(String) // TODO see above about Number(String)
}

struct SelectVariant(Pattern);

impl<'h, S: Slice<'h>> From<ast::Resource<S>> for Resource {
	fn from(value: ast::Resource<S>) -> Self {
		let entries = value.body.into_iter()
			.flat_map(|entry| match entry {
				ast::Entry::Message(message) => {
					let ast::Message { id, value, attributes, comment: _ } = message;

					let id = id.name.as_ref().to_string();
					// value: Option<_>
					// attributes: Vec<_>

					// Some((id, entry))
					todo!()
				}

				ast::Entry::Term(term) => {
					let ast::Term { id, value, attributes, comment: _ } = term;

					let id = id.name.as_ref().to_string();
					// value: Option<_>
					// attributes: Vec<_>

					// Some((id, entry))
					todo!()
				}

				ast::Entry::Comment(_)
					| ast::Entry::GroupComment(_)
					| ast::Entry::ResourceComment(_)
					| ast::Entry::Junk { content: _ }
				=> { None }
			})
			.collect();

		Self(entries)
	}
}

// type MessageWrap<S> = Wrap<(Option<ast::Pattern<S>>, Vec<ast::Attribute<S>>, MessageMarker)>;
// impl<'h, S: Slice<'h>> From<MessageWrap<S>> for Pattern {
// 	fn from(Wrap((value, attrs, _)): MessageWrap<S>) -> Self {
// 		todo!()
// 	}
// }

// type TermWrap<S> = Wrap<(ast::Pattern<S>, Vec<ast::Attribute<S>>, TermMarker)>;
// impl<'h, S: Slice<'h>> From<TermWrap<S>> for Pattern {
// 	fn from(Wrap((value, attrs, _)): TermWrap<S>) -> Self {
// 		todo!()
// 	}
// }

// enum Expression {
// 	Inline(InlineExpression),
// 	Select(SelectExpression)
// }

// enum InlineExpression {
// 	StringLiteral(String),
// 	NumberLiteral(String),
// 	FunctionReference, // TODO
// 	MessageReference, // TODO
// 	TermReference, // TODO
// 	VariableReference, // TODO
// 	Placeable // TODO
// }

// struct SelectExpression {
// 	selector: InlineExpression,
// 	variants: Map<VariantKey, Pattern>
// }

// impl<'h, S: Slice<'h>> From<ast::Resource<S>> for Resource {
// 	fn from(value: ast::Resource<S>) -> Self {
// 		let entries = value.body.into_iter()
// 			.filter_map(|entry| match entry {
// 				ast::Entry::Message(message) => {
// 					Some(Into::<Wrap<_>>::into(message).0)
// 				}

// 				ast::Entry::Term(term) => {
// 					Some(Into::<Wrap<_>>::into(term).0)
// 				}

// 				ast::Entry::Comment(_)
// 					| ast::Entry::GroupComment(_)
// 					| ast::Entry::ResourceComment(_)
// 					| ast::Entry::Junk { content: _ }
// 				=> { None }
// 			})
// 			.collect();

// 		Self { entries }
// 	}
// }

// impl<'h, S: Slice<'h>> From<ast::Message<S>> for Wrap<(String, Pattern)> {
// 	fn from(value: ast::Message<S>) -> Self {
// 		let ast::Message { id, value, attributes, comment: _ } = value;

// 		let id = id.name.as_ref().to_string();

// 		match (value, attributes.len()) {
// 			(Some(value), 0) => {
// 				// MessageSimpleText, MessageSimplePlaceable, Message
// 				Wrap((id, value.into()))
// 			}
// 			(Some(value), 1..) => {
// 				todo!()
// 			}
// 			(None, 1..) => {
// 				todo!()
// 			}
// 			// TODO is there a possibility to have this?
// 			_ => { unimplemented!() }
// 		}
// 	}
// }

// impl<'h, S: Slice<'h>> From<ast::Term<S>> for Wrap<(String, Pattern)> {
// 	fn from(value: ast::Term<S>) -> Self {
// 		let ast::Term { id, value, attributes, comment: _ } = value;

// 		let id = id.name.as_ref().to_string();

// 		if attributes.is_empty() {
// 			Wrap((id, value.into()))
// 		} else {
// 			todo!()
// 		}
// 	}
// }

// impl<'h, S: Slice<'h>> From<ast::Pattern<S>> for Pattern {
// 	fn from(value: ast::Pattern<S>) -> Self {
// 		todo!()
// 	}
// }

/// wrapper that's a local type so I can implement things onto it. Nothing
/// more than a zero cost wrapper on the stuff inside
#[repr(transparent)]
struct Wrap<T>(T);

struct MessageMarker;
struct TermMarker;

type Map<K, V> = HashMap<K, V, RandomState>;

#[inline]
fn map<K, V>() -> Map<K, V> {
	Map::with_hasher(RandomState::new())
}
