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
    fn elements(&self) -> FormElements {
        return self.0.elements();
    }

    fn parse(&self) -> Result<Rc<T>, ()> {
        return Ok(Rc::new(self.0.parse()?));
    }
}

impl<C: 'static, T: FormWith<C> + 'static> FormWith<C> for Rc<T> {
    fn new_form(context: &C, field: &str, from: Option<&Self>) -> Box<dyn FormState<Self>> {
        return Box::new(RcFormState(T::new_form(context, field, from.map(|x| x.as_ref())), Default::default()));
    }
}

struct ArcFormState<C, T: FormWith<C>>(Box<dyn FormState<T>>, PhantomData<C>);

impl<C, T: FormWith<C>> FormState<Arc<T>> for ArcFormState<C, T> {
    fn elements(&self) -> FormElements {
        return self.0.elements();
    }

    fn parse(&self) -> Result<Arc<T>, ()> {
        return Ok(Arc::new(self.0.parse()?));
    }
}

impl<C: 'static, T: FormWith<C> + 'static> FormWith<C> for Arc<T> {
    fn new_form(context: &C, field: &str, from: Option<&Self>) -> Box<dyn FormState<Self>> {
        return Box::new(ArcFormState(T::new_form(context, field, from.map(|x| x.as_ref())), Default::default()));
    }
}

struct CellFormState<C, T: FormWith<C>>(Box<dyn FormState<T>>, PhantomData<C>);

impl<C, T: FormWith<C> + Copy + Clone> FormState<Cell<T>> for CellFormState<C, T> {
    fn elements(&self) -> FormElements {
        return self.0.elements();
    }

    fn parse(&self) -> Result<Cell<T>, ()> {
        return Ok(Cell::new(self.0.parse()?));
    }
}

impl<C: 'static, T: FormWith<C> + Copy + Clone + 'static> FormWith<C> for Cell<T> {
    fn new_form(context: &C, field: &str, from: Option<&Self>) -> Box<dyn FormState<Self>> {
        return Box::new(
            CellFormState(T::new_form(context, field, from.map(|x| x.get()).as_ref()), Default::default()),
        );
    }
}

struct RefCellFormState<C, T: FormWith<C>>(Box<dyn FormState<T>>, PhantomData<C>);

impl<C, T: FormWith<C>> FormState<RefCell<T>> for RefCellFormState<C, T> {
    fn elements(&self) -> FormElements {
        return self.0.elements();
    }

    fn parse(&self) -> Result<RefCell<T>, ()> {
        return Ok(RefCell::new(self.0.parse()?));
    }
}

impl<C: 'static, T: FormWith<C> + 'static> FormWith<C> for RefCell<T> {
    fn new_form(context: &C, field: &str, from: Option<&Self>) -> Box<dyn FormState<Self>> {
        let from = from.map(|x| x.borrow());
        return Box::new(RefCellFormState(T::new_form(context, field, match &from {
            Some(x) => Some(&*x),
            None => None,
        }), Default::default()));
    }
}

struct MutexFormState<C, T: FormWith<C>>(Box<dyn FormState<T>>, PhantomData<C>);

impl<C, T: FormWith<C>> FormState<Mutex<T>> for MutexFormState<C, T> {
    fn elements(&self) -> FormElements {
        return self.0.elements();
    }

    fn parse(&self) -> Result<Mutex<T>, ()> {
        return Ok(Mutex::new(self.0.parse()?));
    }
}

impl<C: 'static, T: FormWith<C> + 'static> FormWith<C> for Mutex<T> {
    fn new_form(context: &C, field: &str, from: Option<&Self>) -> Box<dyn FormState<Self>> {
        let from = from.map(|x| x.lock().unwrap());
        return Box::new(MutexFormState(T::new_form(context, field, match &from {
            Some(x) => Some(&*x),
            None => None,
        }), Default::default()));
    }
}
