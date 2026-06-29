/// Type representing 2 possible values - `L` or `R`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn left(self) -> Option<L> {
        match self {
            Either::Left(value) => Some(value),
            Either::Right(_) => None,
        }
    }

    pub fn right(self) -> Option<R> {
        match self {
            Either::Left(_) => None,
            Either::Right(value) => Some(value),
        }
    }
}
