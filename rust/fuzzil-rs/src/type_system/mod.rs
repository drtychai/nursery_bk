pub mod base_type;
use base_type::BaseType;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Type {
    definite_type: BaseType,
    possible_type: BaseType,
}

impl Type {
    pub fn new(definite_type: BaseType, possible_type: BaseType) -> Self {
        Type {
            definite_type: definite_type,
            possible_type: possible_type,
        }
    }

    pub fn undefined() -> Self {
        Type {
            definite_type: BaseType::undefined(),
            possible_type: BaseType::nothing(),
        }
    }

    pub fn integer() -> Self {
        Type {
            definite_type: BaseType::integer(),
            possible_type: BaseType::nothing(),
        }
    }

    pub fn bigint() -> Self {
        Type {
            definite_type: BaseType::bigint(),
            possible_type: BaseType::nothing(),
        }
    }

    pub fn float() -> Self {
        Type {
            definite_type: BaseType::float(),
            possible_type: BaseType::nothing(),
        }
    }

    pub fn string() -> Self {
        Type {
            definite_type: BaseType::string(),
            possible_type: BaseType::nothing(),
        }
    }

    pub fn boolean() -> Self {
        Type {
            definite_type: BaseType::boolean(),
            possible_type: BaseType::nothing(),
        }
    }

    pub fn regexp() -> Self {
        Type {
            definite_type: BaseType::regexp(),
            possible_type: BaseType::nothing(),
        }
    }

    pub fn iterable() -> Self {
        Type {
            definite_type: BaseType::iterable(),
            possible_type: BaseType::nothing(),
        }
    }

    pub fn unknown() -> Self {
        Type {
            definite_type: BaseType::unknown(),
            possible_type: BaseType::nothing(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BaseType, Type};

    #[test]
    fn consturctor() {
        let t = Type::new(BaseType::integer(), BaseType::float());
        assert_eq!(t.definite_type.0, BaseType::integer().0);
        assert_eq!(t.possible_type.0, BaseType::float().0);
    }
    #[test]
    fn undefined() {
        let t = Type::undefined();
        assert_eq!(t.definite_type.0, BaseType::undefined().0);
        assert_eq!(t.possible_type.0, BaseType::nothing().0);
    }
    #[test]
    fn integer() {
        let t = Type::integer();
        assert_eq!(t.definite_type.0, BaseType::integer().0);
        assert_eq!(t.possible_type.0, BaseType::nothing().0);
    }
    #[test]
    fn bigint() {
        let t = Type::bigint();
        assert_eq!(t.definite_type.0, BaseType::bigint().0);
        assert_eq!(t.possible_type.0, BaseType::nothing().0);
    }
    #[test]
    fn float() {
        let t = Type::float();
        assert_eq!(t.definite_type.0, BaseType::float().0);
        assert_eq!(t.possible_type.0, BaseType::nothing().0);
    }
    #[test]
    fn string() {
        let t = Type::string();
        assert_eq!(t.definite_type.0, BaseType::string().0);
        assert_eq!(t.possible_type.0, BaseType::nothing().0);
    }
    #[test]
    fn boolean() {
        let t = Type::boolean();
        assert_eq!(t.definite_type.0, BaseType::boolean().0);
        assert_eq!(t.possible_type.0, BaseType::nothing().0);
    }
    #[test]
    fn regexp() {
        let t = Type::regexp();
        assert_eq!(t.definite_type.0, BaseType::regexp().0);
        assert_eq!(t.possible_type.0, BaseType::nothing().0);
    }
    #[test]
    fn iterable() {
        let t = Type::iterable();
        assert_eq!(t.definite_type.0, BaseType::iterable().0);
        assert_eq!(t.possible_type.0, BaseType::nothing().0);
    }
    #[test]
    fn unknown() {
        let t = Type::unknown();
        assert_eq!(t.definite_type.0, BaseType::unknown().0);
        assert_eq!(t.possible_type.0, BaseType::nothing().0);
    }
}
