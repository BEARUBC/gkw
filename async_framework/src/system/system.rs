use std::marker::PhantomData;

pub struct System<M, T, A> {
    pdm: PhantomData<M>,
    pdt: PhantomData<T>,
    pda: PhantomData<A>,
}
