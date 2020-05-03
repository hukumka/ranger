use std::marker::PhantomData;

pub trait Lens<I: ?Sized, O: ?Sized> {
    fn get<R, F: FnOnce(&O) -> R>(&mut self, input: &I, func: F) -> R;
}

pub fn lens_fn<I: ?Sized, O: ?Sized, T: AsRef<O>>(f: impl FnMut(&I) -> T) -> impl Lens<I, O> {
    FnLens::<I, O, _, T> {
        f,
        _m1: PhantomData::default(),
        _m2: PhantomData::default(),
        _m3: PhantomData::default(),
    }
}

pub fn lens_fn_ref<I: ?Sized, O: ?Sized>(f: impl FnMut(&I) -> &O) -> impl Lens<I, O> {
    FnRefLens::<I, O, _> {
        f,
        _m1: PhantomData::default(),
        _m2: PhantomData::default(),
    }
}

pub struct FnLens<I: ?Sized, O: ?Sized, F, T> {
    f: F,
    _m1: PhantomData<I>,
    _m2: PhantomData<O>,
    _m3: PhantomData<T>,
}

impl<I, O, F, T> Lens<I, O> for FnLens<I, O, F, T>
where
    I: ?Sized,
    O: ?Sized,
    F: FnMut(&I) -> T,
    T: AsRef<O>,
{
    fn get<R, F2: FnOnce(&O) -> R>(&mut self, input: &I, func: F2) -> R {
        func((self.f)(input).as_ref())
    }
}

pub struct FnRefLens<I: ?Sized, O: ?Sized, F> {
    f: F,
    _m1: PhantomData<I>,
    _m2: PhantomData<O>,
}

impl<I, O, F> Lens<I, O> for FnRefLens<I, O, F>
where
    I: ?Sized,
    O: ?Sized,
    F: FnMut(&I) -> &O,
{
    fn get<R, F2: FnOnce(&O) -> R>(&mut self, input: &I, func: F2) -> R {
        func((self.f)(input))
    }
}
