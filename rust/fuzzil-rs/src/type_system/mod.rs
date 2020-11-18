pub mod base_type;
use base_type::BaseType;

trait Type {
    fn definite_type(&self) -> i32;
    fn possible_type(&self) -> i32;
}

impl Type for base_type::Nothing {
    fn definite_type(&self) -> i32 {
        base_type::Nothing::new().raw_value()
    }

    fn possible_type(&self) -> i32 {
        base_type::Nothing::new().raw_value()
    }
}
impl Type for base_type::Undefined {
    fn definite_type(&self) -> i32 {
        base_type::Undefined::new().raw_value()
    }

    fn possible_type(&self) -> i32 {
        base_type::Nothing::new().raw_value()
    }
}

impl Type for base_type::Integer {
    fn definite_type(&self) -> i32 {
        base_type::Integer::new().raw_value()
    }

    fn possible_type(&self) -> i32 {
        base_type::Nothing::new().raw_value()
    }
}
impl Type for base_type::Float {
    fn definite_type(&self) -> i32 {
        base_type::Float::new().raw_value()
    }

    fn possible_type(&self) -> i32 {
        base_type::Nothing::new().raw_value()
    }
}
impl Type for base_type::JString {
    fn definite_type(&self) -> i32 {
        base_type::JString::new().raw_value()
    }

    fn possible_type(&self) -> i32 {
        base_type::Nothing::new().raw_value()
    }
}
impl Type for base_type::Boolean {
    fn definite_type(&self) -> i32 {
        base_type::Boolean::new().raw_value()
    }

    fn possible_type(&self) -> i32 {
        base_type::Nothing::new().raw_value()
    }
}
impl Type for base_type::Object {
    fn definite_type(&self) -> i32 {
        base_type::Object::new().raw_value()
    }

    fn possible_type(&self) -> i32 {
        base_type::Nothing::new().raw_value()
    }
}
impl Type for base_type::Function {
    fn definite_type(&self) -> i32 {
        base_type::Function::new().raw_value()
    }

    fn possible_type(&self) -> i32 {
        base_type::Nothing::new().raw_value()
    }
}
impl Type for base_type::Constructor {
    fn definite_type(&self) -> i32 {
        base_type::Constructor::new().raw_value()
    }

    fn possible_type(&self) -> i32 {
        base_type::Nothing::new().raw_value()
    }
}
impl Type for base_type::Unknown {
    fn definite_type(&self) -> i32 {
        base_type::Unknown::new().raw_value()
    }

    fn possible_type(&self) -> i32 {
        base_type::Nothing::new().raw_value()
    }
}
impl Type for base_type::BigInt {
    fn definite_type(&self) -> i32 {
        base_type::BigInt::new().raw_value()
    }

    fn possible_type(&self) -> i32 {
        base_type::Nothing::new().raw_value()
    }
}
impl Type for base_type::RegExp {
    fn definite_type(&self) -> i32 {
        base_type::RegExp::new().raw_value()
    }

    fn possible_type(&self) -> i32 {
        base_type::Nothing::new().raw_value()
    }
}
impl Type for base_type::Iter {
    fn definite_type(&self) -> i32 {
        base_type::Iter::new().raw_value()
    }

    fn possible_type(&self) -> i32 {
        base_type::Nothing::new().raw_value()
    }
}
impl Type for base_type::Opt {
    fn definite_type(&self) -> i32 {
        base_type::Opt::new().raw_value()
    }

    fn possible_type(&self) -> i32 {
        base_type::Nothing::new().raw_value()
    }
}
impl Type for base_type::List {
    fn definite_type(&self) -> i32 {
        base_type::List::new().raw_value()
    }

    fn possible_type(&self) -> i32 {
        base_type::Nothing::new().raw_value()
    }
}

#[cfg(test)]
mod tests {
    use super::base_type::*;
    use super::Type;

    #[test]
    fn consturctor() {
        assert_eq!(Constructor::new().definite_type(), Constructor::new().raw_value());
        assert_eq!(Constructor::new().possible_type(), Nothing::new().raw_value());
    }
    #[test]
    fn undefined() {
        assert_eq!(Undefined::new().definite_type(), Undefined::new().raw_value());
        assert_eq!(Undefined::new().possible_type(), Nothing::new().raw_value());
    }
    #[test]
    fn integer() {
        assert_eq!(Integer::new().definite_type(), Integer::new().raw_value());
        assert_eq!(Integer::new().possible_type(), Nothing::new().raw_value());
    }
    #[test]
    fn bigint() {
        assert_eq!(BigInt::new().definite_type(), BigInt::new().raw_value());
        assert_eq!(BigInt::new().possible_type(), Nothing::new().raw_value());
    }
    #[test]
    fn float() {
        assert_eq!(Float::new().definite_type(), Float::new().raw_value());
        assert_eq!(Float::new().possible_type(), Nothing::new().raw_value());
    }
    #[test]
    fn jstring() {
        assert_eq!(JString::new().definite_type(), JString::new().raw_value());
        assert_eq!(JString::new().possible_type(), Nothing::new().raw_value());
    }
    #[test]
    fn boolean() {
        assert_eq!(Boolean::new().definite_type(), Boolean::new().raw_value());
        assert_eq!(Boolean::new().possible_type(), Nothing::new().raw_value());
    }
    #[test]
    fn regexp() {
        assert_eq!(RegExp::new().definite_type(), RegExp::new().raw_value());
        assert_eq!(RegExp::new().possible_type(), Nothing::new().raw_value());
    }
    #[test]
    fn iterable() {
        assert_eq!(Iter::new().definite_type(), Iter::new().raw_value());
        assert_eq!(Iter::new().possible_type(), Nothing::new().raw_value());
    }
    #[test]
    fn unknown() {
        assert_eq!(Unknown::new().definite_type(), Unknown::new().raw_value());
        assert_eq!(Unknown::new().possible_type(), Nothing::new().raw_value());
    }
}
