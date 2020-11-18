/// A feeble attempt to define JavaSript as a rust trait system.
/// Gonna start out *verbose*
///
/// # https://developer.mozilla.org/en-US/docs/Web/JavaScript/Data_structures
/// The latest ECMAScript standard defines nine types:
///   | - Six Data Types that are primitives, checked by typeof operator:
///   |   - undefined : typeof instance === "undefined"
///   |   - Boolean   : typeof instance === "boolean"
///   |   - Number    : typeof instance === "number"
///   |   - String    : typeof instance === "string"
///   |   - BigInt    : typeof instance === "bigint"
///   |   - Symbol    : typeof instance === "symbol"
///   | - Structural Types:
///   |   - Object    : typeof instance === "object". Special non-data but Structural type for any
///   |                 constructed object instance also used as data structures: new Object, new
///   |                 Array, new Map, new Set, new WeakMap, new WeakSet, new Date and almost
///   |                 everything made with new keyword;
///   |   - Function  : a non-data structure, though it also answers for typeof operator: typeof instance === "function".
///   |                 This is merely a special shorthand for Functions, though every Function constructor is
///   |                 derived from Object constructor.
///   | - Structural Root Primitive:
///   |   - null      : typeof instance === "object". Special primitive type having additional usage for its value: if object
///   |                 is not inherited, then null is shown;
///
///
/// Trait nursery:
///   - (?) trait DataType<T>
///   - (?) trait StructuralType<T>
///   - trait Value properties
///   - trait Function properties
///   - trait Fundamental objects
///   - trait Error objects
///   - trait Numbers and dates
///   - trait Text processing
///   - trait Indexed collections
///   - trait Keyed collections
///   - trait Structured data
///   - trait Control abstraction objects
///   - trait Reflection
///   - trait Internationalization
///   - trait WebAssembly
///   - trait Other
///

//trait Seq<T> {
//    fn len(&self) -> u32;
//    fn elt_at(&self, n: u32) -> T;
//    fn iter<F>(&self, f: F) where F: Fn(T);
//}
//
//trait DataType {
//    fn as_str(&self) -> &'static str;
//}
//
//trait StructureType {
//    fn owned(&self) -> Vec<&'static str>;
//}

pub trait BaseType {
    //type BaseType;
    fn new() -> Self;
    fn raw_value(&self) -> i32;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Nothing(pub i32);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Undefined(pub i32);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Integer(pub i32);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Float(pub i32);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct JString(pub i32);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Boolean(pub i32);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Object(pub i32);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Function(pub i32);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Constructor(pub i32);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Unknown(pub i32);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct BigInt(pub i32);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RegExp(pub i32);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Iter(pub i32);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Opt(pub i32);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct List(pub i32);

impl BaseType for Nothing {
    fn new() -> Self {
        Self(0 << 0)
    }

    fn raw_value(&self) -> i32 {
        self.0
    }
}

impl BaseType for Undefined {
    fn new() -> Self {
        Self(1 << 0)
    }

    fn raw_value(&self) -> i32 {
        self.0
    }
}
impl BaseType for Integer {
    fn new() -> Self {
        Self(1 << 1)
    }

    fn raw_value(&self) -> i32 {
        self.0
    }
}
impl BaseType for Float {
    fn new() -> Self {
        Self(1 << 2)
    }

    fn raw_value(&self) -> i32 {
        self.0
    }
}
impl BaseType for JString {
    fn new() -> Self {
        Self(1 << 3)
    }

    fn raw_value(&self) -> i32 {
        self.0
    }
}
impl BaseType for Boolean {
    fn new() -> Self {
        Self(1 << 4)
    }

    fn raw_value(&self) -> i32 {
        self.0
    }
}
impl BaseType for Object {
    fn new() -> Self {
        Self(1 << 5)
    }

    fn raw_value(&self) -> i32 {
        self.0
    }
}

impl BaseType for Function {
    fn new() -> Self {
        Self(1 << 6)
    }

    fn raw_value(&self) -> i32 {
        self.0
    }
}

impl BaseType for Constructor {
    fn new() -> Self {
        Self(1 << 7)
    }

    fn raw_value(&self) -> i32 {
        self.0
    }
}

impl BaseType for Unknown {
    fn new() -> Self {
        Self(1 << 8)
    }

    fn raw_value(&self) -> i32 {
        self.0
    }
}

impl BaseType for BigInt {
    fn new() -> Self {
        Self(1 << 9)
    }

    fn raw_value(&self) -> i32 {
        self.0
    }
}

impl BaseType for RegExp {
    fn new() -> Self {
        Self(1 << 10)
    }

    fn raw_value(&self) -> i32 {
        self.0
    }
}

impl BaseType for Iter {
    fn new() -> Self {
        Self(1 << 11)
    }

    fn raw_value(&self) -> i32 {
        self.0
    }
}

impl BaseType for Opt {
    fn new() -> Self {
        Self(1 << 12)
    }

    fn raw_value(&self) -> i32 {
        self.0
    }
}

impl BaseType for List {
    fn new() -> Self {
        Self(1 << 13)
    }

    fn raw_value(&self) -> i32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing() {
        assert_eq!(Nothing::new().raw_value(), 0 << 0);
    }
    #[test]
    fn undefined() {
        assert_eq!(Undefined::new().raw_value(), 1 << 0);
    }
    #[test]
    fn integer() {
        assert_eq!(Integer::new().raw_value(), 1 << 1);
    }
    #[test]
    fn float() {
        assert_eq!(Float::new().raw_value(), 1 << 2);
    }
    #[test]
    fn jstring() {
        assert_eq!(JString::new().raw_value(), 1 << 3);
    }
    #[test]
    fn boolean() {
        assert_eq!(Boolean::new().raw_value(), 1 << 4);
    }
    #[test]
    fn object() {
        assert_eq!(Object::new().raw_value(), 1 << 5);
    }
    #[test]
    fn function() {
        assert_eq!(Function::new().raw_value(), 1 << 6);
    }
    #[test]
    fn constructor() {
        assert_eq!(Constructor::new().raw_value(), 1 << 7);
    }
    #[test]
    fn unknown() {
        assert_eq!(Unknown::new().raw_value(), 1 << 8);
    }
    #[test]
    fn bigint() {
        assert_eq!(BigInt::new().raw_value(), 1 << 9);
    }
    #[test]
    fn regexp() {
        assert_eq!(RegExp::new().raw_value(), 1 << 10);
    }
    #[test]
    fn iter() {
        assert_eq!(Iter::new().raw_value(), 1 << 11);
    }
    #[test]
    fn opt() {
        assert_eq!(Opt::new().raw_value(), 1 << 12);
    }
    #[test]
    fn list() {
        assert_eq!(List::new().raw_value(), 1 << 13);
    }
}
