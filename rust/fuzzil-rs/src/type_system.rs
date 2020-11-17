#[allow(dead_code)]
extern crate hibitset;
use hibitset::{BitSet, BitSetAnd, BitSetLike, BitSetNot};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BaseTypes {
    Undefined,
    Integer,
    Float,
    Str,
    Boolean,
    Object,
    Function,
    Constructor,
    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Union,
    Intersect,
    Merge,
}

#[cfg(test)]
mod tests {
    use super::{BitSet, BitSetAnd};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn insert() {
        let mut c = BitSet::new();
        for i in 0..1_000 {
            assert!(!c.add(i));
            assert!(c.add(i));
        }

        for i in 0..1_000 {
            assert!(c.contains(i));
        }
    }

    #[test]
    fn insert_100k() {
        let mut c = BitSet::new();
        for i in 0..100_000 {
            assert!(!c.add(i));
            assert!(c.add(i));
        }

        for i in 0..100_000 {
            assert!(c.contains(i));
        }
    }
    #[test]
    fn remove() {
        let mut c = BitSet::new();
        for i in 0..1_000 {
            assert!(!c.add(i));
        }

        for i in 0..1_000 {
            assert!(c.contains(i));
            assert!(c.remove(i));
            assert!(!c.contains(i));
            assert!(!c.remove(i));
        }
    }

}
