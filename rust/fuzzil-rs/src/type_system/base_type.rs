#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BaseType(pub i32);

impl BaseType {
    pub fn nothing() -> Self {
        BaseType(0 << 0)
    }

    pub fn undefined() -> Self {
        BaseType(1 << 0)
    }

    pub fn integer() -> Self {
        BaseType(1 << 1)
    }

    pub fn float() -> Self {
        BaseType(1 << 2)
    }

    pub fn string() -> Self {
        BaseType(1 << 3)
    }

    pub fn boolean() -> Self {
        BaseType(1 << 4)
    }

    pub fn object() -> Self {
        BaseType(1 << 5)
    }

    pub fn function() -> Self {
        BaseType(1 << 6)
    }

    pub fn constructor() -> Self {
        BaseType(1 << 7)
    }
    pub fn unknown() -> Self {
        BaseType(1 << 8)
    }

    pub fn bigint() -> Self {
        BaseType(1 << 9)
    }

    pub fn regexp() -> Self {
        BaseType(1 << 10)
    }

    pub fn iterable() -> Self {
        BaseType(1 << 11)
    }

    pub fn optional() -> Self {
        BaseType(1 << 12)
    }

    pub fn list() -> Self {
        BaseType(1 << 13)
    }
}

#[cfg(test)]
mod tests {
    use super::BaseType;

    #[test]
    fn base_type_nothing() {
        assert_eq!(BaseType::nothing().0, 0 << 0);
    }

    #[test]
    fn base_type_undefined() {
        assert_eq!(BaseType::undefined().0, 1 << 0);
    }

    #[test]
    fn base_type_integer() {
        assert_eq!(BaseType::integer().0, 1 << 1);
    }

    #[test]
    fn base_type_float() {
        assert_eq!(BaseType::float().0, 1 << 2);
    }

    #[test]
    fn base_type_string() {
        assert_eq!(BaseType::string().0, 1 << 3);
    }

    #[test]
    fn base_type_boolean() {
        assert_eq!(BaseType::boolean().0, 1 << 4);
    }

    #[test]
    fn base_type_object() {
        assert_eq!(BaseType::object().0, 1 << 5);
    }

    #[test]
    fn base_type_function() {
        assert_eq!(BaseType::function().0, 1 << 6);
    }

    #[test]
    fn base_type_constructor() {
        assert_eq!(BaseType::constructor().0, 1 << 7);
    }

    #[test]
    fn base_type_unknown() {
        assert_eq!(BaseType::unknown().0, 1 << 8);
    }

    #[test]
    fn base_type_bigint() {
        assert_eq!(BaseType::bigint().0, 1 << 9);
    }

    #[test]
    fn base_type_regexp() {
        assert_eq!(BaseType::regexp().0, 1 << 10);
    }

    #[test]
    fn base_type_iterable() {
        assert_eq!(BaseType::iterable().0, 1 << 11);
    }

    #[test]
    fn base_type_optional() {
        assert_eq!(BaseType::optional().0, 1 << 12);
    }

    #[test]
    fn base_type_list() {
        assert_eq!(BaseType::list().0, 1 << 13);
    }
}
