use std::fmt::Display;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Handle(u64);

impl Display for Handle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Handle {
    pub fn new(pointer: u32, generation: u32) -> Self {
        let pointer = pointer as u64;
        let generation = generation as u64;

        Self((generation << 32) & pointer)
    }

    pub fn pointer(&self) -> u32 {
        let mask = u32::MAX as u64;
        (self.0 & mask) as u32
    }

    pub fn generation(&self) -> u32 {
        let mask = (u32::MAX as u64) << 32;
        ((self.0 & mask) >> 32) as u32
    }
}

#[cfg(test)]
mod test {
    use crate::Handle;

    fn test_handle(pointer: u32, generation: u32) {
        let handle = Handle::new(pointer, generation);

        assert_eq!(handle.pointer(), pointer);
        assert_eq!(handle.generation(), generation);
    }

    #[test]
    fn zero_zero() {
        test_handle(0, 0);
    }

    #[test]
    fn zero_gen() {
        test_handle(42, 0);
    }

    #[test]
    fn zero_pointer() {
        test_handle(0, 42);
    }

    #[test]
    fn regular() {
        test_handle(42, 69);
    }

    #[test]
    fn max_gen() {
        test_handle(42, u32::MAX);
    }

    #[test]
    fn max_pointer() {
        test_handle(u32::MAX, 42);
    }
}
