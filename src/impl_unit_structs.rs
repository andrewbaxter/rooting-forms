use std::{
    cell::{
        Cell,
        RefCell,
    },
    marker::PhantomData,
    rc::Rc,
    sync::{
        Arc,
        Mutex,
    },
};
use crate::{
    FormWith,
    FormElements,
    FormState,
};

struct RcFormState<C, T: FormWith<C>>(Box<dyn FormState<T>>, PhantomData<C>);

impl<C, T: FormWith<C>> FormState<Rc<T>> for RcFormState<C, T> {
    fn parse(&self) -> Result<Rc<T>, ()> {
        return Ok(Rc::new(self.0.parse()?));
    }
}

impl<C: 'static, T: FormWith<C> + 'static> FormWith<C> for Rc<T> {
    fn new_form_with_(
        context: &C,
        field: &str,
        from: Option<&Self>,
        depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        let (elements, state) = T::new_form_with_(context, field, from.map(|x| x.as_ref()), depth);
        return (elements, Box::new(RcFormState(state, Default::default())));
    }
}

struct ArcFormState<C, T: FormWith<C>>(Box<dyn FormState<T>>, PhantomData<C>);

impl<C, T: FormWith<C>> FormState<Arc<T>> for ArcFormState<C, T> {
    fn parse(&self) -> Result<Arc<T>, ()> {
        return Ok(Arc::new(self.0.parse()?));
    }
}

impl<C: 'static, T: FormWith<C> + 'static> FormWith<C> for Arc<T> {
    fn new_form_with_(
        context: &C,
        field: &str,
        from: Option<&Self>,
        depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        let (elements, state) = T::new_form_with_(context, field, from.map(|x| x.as_ref()), depth);
        return (elements, Box::new(ArcFormState(state, Default::default())));
    }
}

struct CellFormState<C, T: FormWith<C>>(Box<dyn FormState<T>>, PhantomData<C>);

impl<C, T: FormWith<C> + Copy + Clone> FormState<Cell<T>> for CellFormState<C, T> {
    fn parse(&self) -> Result<Cell<T>, ()> {
        return Ok(Cell::new(self.0.parse()?));
    }
}

impl<C: 'static, T: FormWith<C> + Copy + Clone + 'static> FormWith<C> for Cell<T> {
    fn new_form_with_(
        context: &C,
        field: &str,
        from: Option<&Self>,
        depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        let (elements, state) = T::new_form_with_(context, field, from.map(|x| x.get()).as_ref(), depth);
        return (elements, Box::new(CellFormState(state, Default::default())));
    }
}

struct RefCellFormState<C, T: FormWith<C>>(Box<dyn FormState<T>>, PhantomData<C>);

impl<C, T: FormWith<C>> FormState<RefCell<T>> for RefCellFormState<C, T> {
    fn parse(&self) -> Result<RefCell<T>, ()> {
        return Ok(RefCell::new(self.0.parse()?));
    }
}

impl<C: 'static, T: FormWith<C> + 'static> FormWith<C> for RefCell<T> {
    fn new_form_with_(
        context: &C,
        field: &str,
        from: Option<&Self>,
        depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        let from = from.map(|x| x.borrow());
        let (elements, state) = T::new_form_with_(context, field, match &from {
            Some(x) => Some(&*x),
            None => None,
        }, depth);
        return (elements, Box::new(RefCellFormState(state, Default::default())));
    }
}

struct MutexFormState<C, T: FormWith<C>>(Box<dyn FormState<T>>, PhantomData<C>);

impl<C, T: FormWith<C>> FormState<Mutex<T>> for MutexFormState<C, T> {
    fn parse(&self) -> Result<Mutex<T>, ()> {
        return Ok(Mutex::new(self.0.parse()?));
    }
}

impl<C: 'static, T: FormWith<C> + 'static> FormWith<C> for Mutex<T> {
    fn new_form_with_(
        context: &C,
        field: &str,
        from: Option<&Self>,
        depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        let from = from.map(|x| x.lock().unwrap());
        let (elements, state) = T::new_form_with_(context, field, match &from {
            Some(x) => Some(&*x),
            None => None,
        }, depth);
        return (elements, Box::new(MutexFormState(state, Default::default())));
    }
}
