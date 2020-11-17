//! A simple type system designed for use during fuzzing.
//!
//! The goal of this type system is to be as simple as possible while still being able to express all the common
//! operations that one can peform on values of the target language, e.g. calling a function or constructor,
//! accessing properties, or calling methods.
//!
//! The type system is mainly used for two purposes:
//! 1. to obtain a variable of a certain type when generating code. E.g. the method call code generator will want
//!    a variable of type `.object()` as input because only that can have methods. Also, when generating function
//!    calls it can be necessary to find variables of the types that the function expects as arguments. This task
//!    is solved by defining a "Is a" relationship between types which can then be used to find suitable variables.
//! 2. to determine possible actions that can be performed on a value. E.g. when having a reference to something
//!    that is known to be a function, a function call can be performed. Also, the method call code generator will
//!    want to know the available methods that it can call on an object, which it can query from the type system.
//!
//! The following base types are defined:
//! - `.undefined`
//! - `.integer`
//! - `.float`
//! - `.string`
//! - `.boolean`
//! - `.object(ofGroup: G, withProperties: [...], withMethods: [...])`
//!   - something that (potentially) has properties and methods. Can also have a "group", which is simply a string.
//!   - groups can e.g. be used to store property and method type information for related objects.
//! - `.function(signature: S)`
//!   - something that can be invoked as a function
//! - `.constructor(signature: S)`
//!   - something that can be invoked as a constructor
//! - `.unknown`
//!   - a pseudotype to indicate that the real type is unknown
//!
//! Flag types are base types which never occur alone, but only in combination with other type. The flag types are:
//! - `.phi`  : indicates that the variable is a Phi which can be reassigned
//!   - this is simply a convenience feature which avoids the need to
//!   - track Phi variables seperately.
//! - `.list` : used to indicate rest parameters in a function signature.
//!
//! A type can be a union, essentially stating that it is one of multiple types. Union types occur in many scenarios, e.g.
//! when reassigning a variable, when computing type information following a conditionally executing piece of code, or due to
//! imprecise modelling of the environment, e.g. the + operation in JavaScript or return values of various APIs.
//!
//! Further, a type can also be a merged type, essentially stating that it is *all* of the contained types at the same type.
//! As an example of a merged type, think of regular JavaScript functions which can be called but also constructed. On the other hand,
//! some JavaScript builtins can be used as a function but not as a constructor or vice versa. As such, it is necessary to be able to
//! differentiate between functions and constructors. Strings are another example for merged types. On the one hand they are a primitive
//! type expected by various APIs, on the other hand they can be used like an object by accessing properties on them or invoking methods.
//! As such, a JavaScript string would be represented as a merge of `.string` and `.object(properties: [...], methods: [...])`.
//!
//! The operations that can be performed on types are then:
//!
//! 1. Unioning (|)     : this operation on two types expresses that the result can either
//!                       be the first or the second type.
//!
//! 2. Intersecting (&) : this operation computes the intersection, so the common type
//!                       between the two argument types. This is used by the "MayBe" query,
//!                       answering whether a value could potentially have the given type.
//!                       In contrast to the other two operations, itersecting will not create
//!                       new types.
//!
//! 3. Merging (+)      : this operation merges the two argument types into a single type
//!                          which is both types. Not all types can be merged, however.
//!
//! Finally, types define the "is a" (subsumption) relation (>=) which amounts to set inclusion. A type T1 subsumes
//! another type T2 if all instances of T2 are also instances of T1. Some examples:
//! - `.anything`, the union of all types, subsumes every other type
//! - `.nothing`, the empty type, is subsumed by all other types.`.nothing` occurs e.g. during intersection
//! - `.object()` subsumes all other object type. I.e. objects with a property "foo" are sill objects
//!       e.g. `.object()` >= `.object(withProperties: ["foo"])` and `.object()` >= `.object(withMethods: ["bar"])`
//! - `.object(withProperties: ["foo"])` subsumes all other object types that also have a property "foo"
//!       e.g. `.object(withProperties: ["foo"])` >= `.object(withProperties: ["foo", "bar"], withMethods: ["baz"])`
//! - `.object(ofGroup: G)` subsumes any other object of group G, but not of a different group
//! - `.function([.integer] => .integer)` only subsumes other functions with the same signature
//! - `.primitive`, the union of `.integer`, `.float`, and `.string`, subsumes its parts like every union
//! - `.functionAndConstructor()`, the merge of `.function()` and `.constructor()`, is subsumed by each of its parts like every merged type
//!
//! Internally, types are implemented as hierarchical bit sets, defining the definite type, indicating what it definitely
//! is, and the possible type, indicating what types it could potentially be. A union type is one where the possible type
//! is larger than the definite one.
//!
//! Examples:
//! ```md
//! .integer                      
//!     => definiteType = .integer,            possibleType = .integer
//!
//! .integer | .float             
//!     => definiteType = .nothing (0),        possibleType = .integer | .float
//!
//! .string + .object             
//!     => definiteType = .string | .object,   possibleType = .string | .object
//!
//! .string | (.string + .object)
//!     => definiteType = .string,             possibleType = .string | .object
//! ```

pub mod type_system;
