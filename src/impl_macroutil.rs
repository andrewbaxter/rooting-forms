use std::{
    cell::RefCell,
    marker::PhantomData,
    rc::Rc,
};
use crate::{
    FormElements,
    FormState,
    FormWith,
};

struct LazySubform_<C, T: FormWith<C>> {
    data: RefCell<Option<Rc<(FormElements, Box<dyn FormState<T>>)>>>,
    initializer: Box<dyn Fn() -> (FormElements, Box<dyn FormState<T>>)>,
    _pd: PhantomData<C>,
}

pub struct LazySubform<C, T: FormWith<C>>(Rc<LazySubform_<C, T>>);

impl<C, T: FormWith<C>> Clone for LazySubform<C, T> {
    fn clone(&self) -> Self {
        return Self(self.0.clone());
    }
}

impl<C, T: FormWith<C>> LazySubform<C, T> {
    pub fn new(f: impl 'static + Fn() -> (FormElements, Box<dyn FormState<T>>)) -> Self {
        return Self(Rc::new(LazySubform_ {
            data: RefCell::new(None),
            initializer: Box::new(f),
            _pd: Default::default(),
        }));
    }

    fn init(&self) -> Rc<(FormElements, Box<dyn FormState<T>>)> {
        return self.0.data.borrow_mut().get_or_insert_with(|| Rc::new((self.0.initializer)())).clone();
    }

    pub fn elements(&self) -> Vec<rooting::El> {
        let init = self.init();
        let mut out = vec![];
        if let Some(error) = &init.0.error {
            out.push(error.clone());
        }
        out.extend(init.0.elements.clone());
        return out;
    }

    pub fn parse(&self) -> Result<T, ()> {
        return self.init().1.parse();
    }
}

pub struct VariantUnitFormState<C: 'static + Clone, T: FormWith<C>>(pub fn() -> T, pub PhantomData<C>);

impl<C: 'static + Clone, T: FormWith<C>> VariantUnitFormState<C, T> {
    pub fn new(f: fn() -> T) -> Self {
        return Self(f, Default::default());
    }
}

impl<C: 'static + Clone, T: FormWith<C>> FormState<T> for VariantUnitFormState<C, T> {
    fn parse(&self) -> Result<T, ()> {
        return Ok((self.0)());
    }
}

pub struct VariantWrapFormState<C: 'static + Clone, V: FormWith<C>, T: FormWith<C>> {
    inner: Box<dyn FormState<V>>,
    wrap: fn(V) -> T,
    _pd: PhantomData<C>,
}

impl<C: 'static + Clone, V: FormWith<C>, T: FormWith<C>> VariantWrapFormState<C, V, T> {
    pub fn new(inner: Box<dyn FormState<V>>, wrap: fn(V) -> T) -> Self {
        return Self {
            inner: inner,
            wrap,
            _pd: Default::default(),
        };
    }
}

impl<C: 'static + Clone, V: FormWith<C>, T: FormWith<C>> FormState<T> for VariantWrapFormState<C, V, T> {
    fn parse(&self) -> Result<T, ()> {
        return Ok((self.wrap)(self.inner.parse()?));
    }
}
