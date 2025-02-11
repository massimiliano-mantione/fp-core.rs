use crate::applicative::Applicative;
use crate::chain::Chain;

pub trait Monad<A, B>: Chain<B> + Applicative<A, B> {}

impl<A, B> Monad<A, B> for Option<A> where {}
