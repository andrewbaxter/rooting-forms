use {
    crate::{
        css::{
            err_el,
            ATTR_LABEL,
            CSS_CLASS_SMALL_INPUT,
        },
        FormElements,
        FormState,
        FormWith,
    },
    jiff::{
        civil::{
            Date,
            Time,
        },
        tz::TimeZone,
        Timestamp,
        Zoned,
    },
    rooting::{
        el,
        El,
    },
    wasm_bindgen::JsCast,
    web_sys::HtmlInputElement,
};

// Timestamp
const DATETIME_FORMAT: &str = "%Y-%m-%dT%H:%M";

pub struct TimestampFormState {
    pub err_el: El,
    pub el: El,
}

fn parse(t: &str) -> Result<Timestamp, String> {
    let t = match Zoned::strptime(DATETIME_FORMAT, t) {
        Ok(t) => t,
        Err(e) => {
            return Err(e.to_string());
        },
    };
    return Ok(t.timestamp());
}

impl FormState<Timestamp> for TimestampFormState {
    fn parse(&self) -> Result<Timestamp, ()> {
        match parse(&self.el.raw().dyn_ref::<HtmlInputElement>().unwrap().value()) {
            Ok(v) => {
                self.err_el.ref_text("");
                return Ok(v);
            },
            Err(e) => {
                self.err_el.ref_text(&e);
                return Err(());
            },
        }
    }
}

impl<C> FormWith<C> for Timestamp {
    fn new_form_with_(
        _context: &C,
        field: &str,
        from: Option<&Self>,
        _depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        let err_el = err_el();
        let input_el =
            el("input")
                .classes(&[CSS_CLASS_SMALL_INPUT])
                .attr(ATTR_LABEL, field)
                .attr("type", "datetime-local")
                .attr("value", &match from {
                    Some(v) => *v,
                    None => Timestamp::now(),
                }.to_zoned(jiff::tz::TimeZone::system()).strftime(DATETIME_FORMAT).to_string())
                .on("input", {
                    let err_el = err_el.clone();
                    move |ev| {
                        match parse(&ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value()) {
                            Ok(_) => {
                                err_el.ref_text("");
                            },
                            Err(e) => {
                                err_el.ref_text(&e);
                            },
                        }
                    }
                });
        return (FormElements {
            error: None,
            elements: vec![input_el.clone()],
        }, Box::new(TimestampFormState {
            el: input_el,
            err_el: err_el,
        }));
    }
}

// Date
const DATE_FORMAT: &str = "%Y-%m-%d";

pub struct DateFormState {
    pub err_el: El,
    pub el: El,
}

fn parse_date(t: &str) -> Result<Date, String> {
    return Date::strptime(DATE_FORMAT, t).map_err(|e| e.to_string());
}

impl FormState<Date> for DateFormState {
    fn parse(&self) -> Result<Date, ()> {
        match parse_date(&self.el.raw().dyn_ref::<HtmlInputElement>().unwrap().value()) {
            Ok(v) => {
                self.err_el.ref_text("");
                return Ok(v);
            },
            Err(e) => {
                self.err_el.ref_text(&e);
                return Err(());
            },
        }
    }
}

impl<C> FormWith<C> for Date {
    fn new_form_with_(
        _context: &C,
        field: &str,
        from: Option<&Self>,
        _depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        let err_el = err_el();
        let input_el =
            el("input")
                .classes(&[CSS_CLASS_SMALL_INPUT])
                .attr(ATTR_LABEL, field)
                .attr("type", "date")
                .attr("value", &match from {
                    Some(v) => *v,
                    None => Zoned::now().date(),
                }.strftime(DATE_FORMAT).to_string())
                .on("input", {
                    let err_el = err_el.clone();
                    move |ev| {
                        match parse_date(&ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value()) {
                            Ok(_) => {
                                err_el.ref_text("");
                            },
                            Err(e) => {
                                err_el.ref_text(&e);
                            },
                        }
                    }
                });
        return (FormElements {
            error: None,
            elements: vec![input_el.clone()],
        }, Box::new(DateFormState {
            el: input_el,
            err_el: err_el,
        }));
    }
}

// Time
/// Represents a time and the timezone the time is relative to.
#[derive(PartialEq, Eq, Clone)]
pub struct ZoneTime {
    pub zone: TimeZone,
    pub time: Time,
}

const TIME_FORMAT: &str = "%H:%M";

pub struct TimeFormState {
    pub err_el: El,
    pub el: El,
}

fn parse_zt(t: &str) -> Result<ZoneTime, String> {
    match Time::strptime(TIME_FORMAT, t) {
        Ok(t) => return Ok(ZoneTime {
            zone: jiff::tz::TimeZone::system(),
            time: t,
        }),
        Err(e) => return Err(e.to_string()),
    }
}

impl FormState<ZoneTime> for TimeFormState {
    fn parse(&self) -> Result<ZoneTime, ()> {
        match parse_zt(&self.el.raw().dyn_ref::<HtmlInputElement>().unwrap().value()) {
            Ok(v) => {
                self.err_el.ref_text("");
                return Ok(v);
            },
            Err(e) => {
                self.err_el.ref_text(&e);
                return Err(());
            },
        }
    }
}

impl<C> FormWith<C> for ZoneTime {
    fn new_form_with_(
        _context: &C,
        field: &str,
        from: Option<&Self>,
        _depth: usize,
    ) -> (FormElements, Box<dyn FormState<Self>>) {
        let err_el = err_el();
        let input_el =
            el("input")
                .classes(&[CSS_CLASS_SMALL_INPUT])
                .attr(ATTR_LABEL, field)
                .attr("type", "datetime-local")
                .attr("value", &match from {
                    Some(v) => v.time,
                    None => Zoned::now().time(),
                }.strftime(TIME_FORMAT).to_string())
                .on("input", {
                    let err_el = err_el.clone();
                    move |ev| {
                        match parse(&ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value()) {
                            Ok(_) => {
                                err_el.ref_text("");
                            },
                            Err(e) => {
                                err_el.ref_text(&e);
                            },
                        }
                    }
                });
        return (FormElements {
            error: None,
            elements: vec![input_el.clone()],
        }, Box::new(TimeFormState {
            el: input_el,
            err_el: err_el,
        }));
    }
}
