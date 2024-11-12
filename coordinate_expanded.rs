#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::vec;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
mod jitter {
    use crate::MyBotDetection;
    impl MyBotDetection {
        /// Returns a number between 0.0 and 1.0, with higher number meaning there
        /// was more jitter.
        pub(crate) fn jitter(&self) -> f32 {
            if self.events.len() < 2 {
                return 0.0;
            }
            let mut diffs = Vec::new();
            for i in 1..self.events.len() {
                let dx = (self.events[i].coordinate.x - self.events[i - 1].coordinate.x)
                    as f32;
                let dy = (self.events[i].coordinate.y - self.events[i - 1].coordinate.y)
                    as f32;
                let dist = (dx * dx + dy * dy).sqrt();
                diffs.push(dist);
            }
            let mean: f32 = diffs.iter().sum::<f32>() / diffs.len() as f32;
            let variance: f32 = diffs.iter().map(|&d| (d - mean).powi(2)).sum::<f32>()
                / diffs.len() as f32;
            (variance / 1000.0).min(1.0)
        }
    }
}
mod web_utils {
    #[allow(unused)]
    pub(crate) use println;
}
pub type Timestamp = f64;
pub struct MyBotDetection {
    events: Vec<Event>,
    saved_results: Vec<BotDetectionOutput>,
}
#[automatically_derived]
impl wasm_bindgen::describe::WasmDescribe for MyBotDetection {
    fn describe() {
        use wasm_bindgen::__wbindgen_if_not_std;
        use wasm_bindgen::describe::*;
        inform(RUST_STRUCT);
        inform(14u32);
        inform(77u32);
        inform(121u32);
        inform(66u32);
        inform(111u32);
        inform(116u32);
        inform(68u32);
        inform(101u32);
        inform(116u32);
        inform(101u32);
        inform(99u32);
        inform(116u32);
        inform(105u32);
        inform(111u32);
        inform(110u32);
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::IntoWasmAbi for MyBotDetection {
    type Abi = u32;
    fn into_abi(self) -> u32 {
        use wasm_bindgen::__rt::alloc::rc::Rc;
        use wasm_bindgen::__rt::WasmRefCell;
        Rc::into_raw(Rc::new(WasmRefCell::new(self))) as u32
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::FromWasmAbi for MyBotDetection {
    type Abi = u32;
    unsafe fn from_abi(js: u32) -> Self {
        use wasm_bindgen::__rt::alloc::rc::Rc;
        use wasm_bindgen::__rt::core::result::Result::{Ok, Err};
        use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};
        let ptr = js as *mut WasmRefCell<MyBotDetection>;
        assert_not_null(ptr);
        let rc = Rc::from_raw(ptr);
        match Rc::try_unwrap(rc) {
            Ok(cell) => cell.into_inner(),
            Err(_) => {
                wasm_bindgen::throw_str(
                    "attempted to take ownership of Rust value while it was borrowed",
                )
            }
        }
    }
}
#[automatically_derived]
impl wasm_bindgen::__rt::core::convert::From<MyBotDetection> for wasm_bindgen::JsValue {
    fn from(value: MyBotDetection) -> Self {
        let ptr = wasm_bindgen::convert::IntoWasmAbi::into_abi(value);
        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        unsafe fn __wbg_mybotdetection_new(_: u32) -> u32 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("cannot convert to JsValue outside of the Wasm target"),
                );
            }
        }
        unsafe {
            <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                __wbg_mybotdetection_new(ptr),
            )
        }
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::RefFromWasmAbi for MyBotDetection {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::RcRef<MyBotDetection>;
    unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
        use wasm_bindgen::__rt::alloc::rc::Rc;
        let js = js as *mut wasm_bindgen::__rt::WasmRefCell<MyBotDetection>;
        wasm_bindgen::__rt::assert_not_null(js);
        Rc::increment_strong_count(js);
        let rc = Rc::from_raw(js);
        wasm_bindgen::__rt::RcRef::new(rc)
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::RefMutFromWasmAbi for MyBotDetection {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::RcRefMut<MyBotDetection>;
    unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor {
        use wasm_bindgen::__rt::alloc::rc::Rc;
        let js = js as *mut wasm_bindgen::__rt::WasmRefCell<MyBotDetection>;
        wasm_bindgen::__rt::assert_not_null(js);
        Rc::increment_strong_count(js);
        let rc = Rc::from_raw(js);
        wasm_bindgen::__rt::RcRefMut::new(rc)
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::LongRefFromWasmAbi for MyBotDetection {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::RcRef<MyBotDetection>;
    unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
        <Self as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(js)
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::OptionIntoWasmAbi for MyBotDetection {
    #[inline]
    fn none() -> Self::Abi {
        0
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::OptionFromWasmAbi for MyBotDetection {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        *abi == 0
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::TryFromJsValue for MyBotDetection {
    type Error = wasm_bindgen::JsValue;
    fn try_from_js_value(
        value: wasm_bindgen::JsValue,
    ) -> wasm_bindgen::__rt::core::result::Result<Self, Self::Error> {
        let idx = wasm_bindgen::convert::IntoWasmAbi::into_abi(&value);
        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        unsafe fn __wbg_mybotdetection_unwrap(_: u32) -> u32 {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "cannot convert from JsValue outside of the Wasm target",
                    ),
                );
            }
        }
        let ptr = unsafe { __wbg_mybotdetection_unwrap(idx) };
        if ptr == 0 {
            wasm_bindgen::__rt::core::result::Result::Err(value)
        } else {
            #[allow(clippy::mem_forget)] wasm_bindgen::__rt::core::mem::forget(value);
            unsafe {
                wasm_bindgen::__rt::core::result::Result::Ok(
                    <Self as wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr),
                )
            }
        }
    }
}
#[automatically_derived]
impl wasm_bindgen::describe::WasmDescribeVector for MyBotDetection {
    fn describe_vector() {
        use wasm_bindgen::describe::*;
        inform(VECTOR);
        inform(NAMED_EXTERNREF);
        inform(14u32);
        inform(77u32);
        inform(121u32);
        inform(66u32);
        inform(111u32);
        inform(116u32);
        inform(68u32);
        inform(101u32);
        inform(116u32);
        inform(101u32);
        inform(99u32);
        inform(116u32);
        inform(105u32);
        inform(111u32);
        inform(110u32);
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::VectorIntoWasmAbi for MyBotDetection {
    type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
        [wasm_bindgen::JsValue],
    > as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    fn vector_into_abi(
        vector: wasm_bindgen::__rt::alloc::boxed::Box<[MyBotDetection]>,
    ) -> Self::Abi {
        wasm_bindgen::convert::js_value_vector_into_abi(vector)
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::VectorFromWasmAbi for MyBotDetection {
    type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
        [wasm_bindgen::JsValue],
    > as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn vector_from_abi(
        js: Self::Abi,
    ) -> wasm_bindgen::__rt::alloc::boxed::Box<[MyBotDetection]> {
        wasm_bindgen::convert::js_value_vector_from_abi(js)
    }
}
#[automatically_derived]
impl wasm_bindgen::__rt::VectorIntoJsValue for MyBotDetection {
    fn vector_into_jsvalue(
        vector: wasm_bindgen::__rt::alloc::boxed::Box<[MyBotDetection]>,
    ) -> wasm_bindgen::JsValue {
        wasm_bindgen::__rt::js_value_vector_into_jsvalue(vector)
    }
}
pub struct Event {
    pub timestamp: Timestamp,
    pub coordinate: Coordinate,
}
#[automatically_derived]
impl wasm_bindgen::describe::WasmDescribe for Event {
    fn describe() {
        use wasm_bindgen::__wbindgen_if_not_std;
        use wasm_bindgen::describe::*;
        inform(RUST_STRUCT);
        inform(5u32);
        inform(69u32);
        inform(118u32);
        inform(101u32);
        inform(110u32);
        inform(116u32);
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::IntoWasmAbi for Event {
    type Abi = u32;
    fn into_abi(self) -> u32 {
        use wasm_bindgen::__rt::alloc::rc::Rc;
        use wasm_bindgen::__rt::WasmRefCell;
        Rc::into_raw(Rc::new(WasmRefCell::new(self))) as u32
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::FromWasmAbi for Event {
    type Abi = u32;
    unsafe fn from_abi(js: u32) -> Self {
        use wasm_bindgen::__rt::alloc::rc::Rc;
        use wasm_bindgen::__rt::core::result::Result::{Ok, Err};
        use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};
        let ptr = js as *mut WasmRefCell<Event>;
        assert_not_null(ptr);
        let rc = Rc::from_raw(ptr);
        match Rc::try_unwrap(rc) {
            Ok(cell) => cell.into_inner(),
            Err(_) => {
                wasm_bindgen::throw_str(
                    "attempted to take ownership of Rust value while it was borrowed",
                )
            }
        }
    }
}
#[automatically_derived]
impl wasm_bindgen::__rt::core::convert::From<Event> for wasm_bindgen::JsValue {
    fn from(value: Event) -> Self {
        let ptr = wasm_bindgen::convert::IntoWasmAbi::into_abi(value);
        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        unsafe fn __wbg_event_new(_: u32) -> u32 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("cannot convert to JsValue outside of the Wasm target"),
                );
            }
        }
        unsafe {
            <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                __wbg_event_new(ptr),
            )
        }
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::RefFromWasmAbi for Event {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::RcRef<Event>;
    unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
        use wasm_bindgen::__rt::alloc::rc::Rc;
        let js = js as *mut wasm_bindgen::__rt::WasmRefCell<Event>;
        wasm_bindgen::__rt::assert_not_null(js);
        Rc::increment_strong_count(js);
        let rc = Rc::from_raw(js);
        wasm_bindgen::__rt::RcRef::new(rc)
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::RefMutFromWasmAbi for Event {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::RcRefMut<Event>;
    unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor {
        use wasm_bindgen::__rt::alloc::rc::Rc;
        let js = js as *mut wasm_bindgen::__rt::WasmRefCell<Event>;
        wasm_bindgen::__rt::assert_not_null(js);
        Rc::increment_strong_count(js);
        let rc = Rc::from_raw(js);
        wasm_bindgen::__rt::RcRefMut::new(rc)
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::LongRefFromWasmAbi for Event {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::RcRef<Event>;
    unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
        <Self as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(js)
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::OptionIntoWasmAbi for Event {
    #[inline]
    fn none() -> Self::Abi {
        0
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::OptionFromWasmAbi for Event {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        *abi == 0
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::TryFromJsValue for Event {
    type Error = wasm_bindgen::JsValue;
    fn try_from_js_value(
        value: wasm_bindgen::JsValue,
    ) -> wasm_bindgen::__rt::core::result::Result<Self, Self::Error> {
        let idx = wasm_bindgen::convert::IntoWasmAbi::into_abi(&value);
        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        unsafe fn __wbg_event_unwrap(_: u32) -> u32 {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "cannot convert from JsValue outside of the Wasm target",
                    ),
                );
            }
        }
        let ptr = unsafe { __wbg_event_unwrap(idx) };
        if ptr == 0 {
            wasm_bindgen::__rt::core::result::Result::Err(value)
        } else {
            #[allow(clippy::mem_forget)] wasm_bindgen::__rt::core::mem::forget(value);
            unsafe {
                wasm_bindgen::__rt::core::result::Result::Ok(
                    <Self as wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr),
                )
            }
        }
    }
}
#[automatically_derived]
impl wasm_bindgen::describe::WasmDescribeVector for Event {
    fn describe_vector() {
        use wasm_bindgen::describe::*;
        inform(VECTOR);
        inform(NAMED_EXTERNREF);
        inform(5u32);
        inform(69u32);
        inform(118u32);
        inform(101u32);
        inform(110u32);
        inform(116u32);
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::VectorIntoWasmAbi for Event {
    type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
        [wasm_bindgen::JsValue],
    > as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    fn vector_into_abi(
        vector: wasm_bindgen::__rt::alloc::boxed::Box<[Event]>,
    ) -> Self::Abi {
        wasm_bindgen::convert::js_value_vector_into_abi(vector)
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::VectorFromWasmAbi for Event {
    type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
        [wasm_bindgen::JsValue],
    > as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn vector_from_abi(
        js: Self::Abi,
    ) -> wasm_bindgen::__rt::alloc::boxed::Box<[Event]> {
        wasm_bindgen::convert::js_value_vector_from_abi(js)
    }
}
#[automatically_derived]
impl wasm_bindgen::__rt::VectorIntoJsValue for Event {
    fn vector_into_jsvalue(
        vector: wasm_bindgen::__rt::alloc::boxed::Box<[Event]>,
    ) -> wasm_bindgen::JsValue {
        wasm_bindgen::__rt::js_value_vector_into_jsvalue(vector)
    }
}
#[automatically_derived]
const _: () = {
    #[doc(hidden)]
    pub unsafe extern "C" fn __wbg_get_event_timestamp(
        js: u32,
    ) -> wasm_bindgen::convert::WasmRet<
        <Timestamp as wasm_bindgen::convert::IntoWasmAbi>::Abi,
    > {
        use wasm_bindgen::__rt::{WasmRefCell, assert_not_null};
        use wasm_bindgen::convert::IntoWasmAbi;
        fn assert_copy<T: Copy>() {}
        assert_copy::<Timestamp>();
        let js = js as *mut WasmRefCell<Event>;
        assert_not_null(js);
        let val = (*js).borrow().timestamp;
        <Timestamp as IntoWasmAbi>::into_abi(val).into()
    }
};
#[automatically_derived]
const _: () = {
    #[doc(hidden)]
    pub unsafe extern "C" fn __wbg_get_event_coordinate(
        js: u32,
    ) -> wasm_bindgen::convert::WasmRet<
        <Coordinate as wasm_bindgen::convert::IntoWasmAbi>::Abi,
    > {
        use wasm_bindgen::__rt::{WasmRefCell, assert_not_null};
        use wasm_bindgen::convert::IntoWasmAbi;
        fn assert_copy<T: Copy>() {}
        assert_copy::<Coordinate>();
        let js = js as *mut WasmRefCell<Event>;
        assert_not_null(js);
        let val = (*js).borrow().coordinate;
        <Coordinate as IntoWasmAbi>::into_abi(val).into()
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for Event {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Event",
            "timestamp",
            &self.timestamp,
            "coordinate",
            &&self.coordinate,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Event {
    #[inline]
    fn clone(&self) -> Event {
        Event {
            timestamp: ::core::clone::Clone::clone(&self.timestamp),
            coordinate: ::core::clone::Clone::clone(&self.coordinate),
        }
    }
}
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}
#[automatically_derived]
impl wasm_bindgen::describe::WasmDescribe for Coordinate {
    fn describe() {
        use wasm_bindgen::__wbindgen_if_not_std;
        use wasm_bindgen::describe::*;
        inform(RUST_STRUCT);
        inform(10u32);
        inform(67u32);
        inform(111u32);
        inform(111u32);
        inform(114u32);
        inform(100u32);
        inform(105u32);
        inform(110u32);
        inform(97u32);
        inform(116u32);
        inform(101u32);
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::IntoWasmAbi for Coordinate {
    type Abi = u32;
    fn into_abi(self) -> u32 {
        use wasm_bindgen::__rt::alloc::rc::Rc;
        use wasm_bindgen::__rt::WasmRefCell;
        Rc::into_raw(Rc::new(WasmRefCell::new(self))) as u32
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::FromWasmAbi for Coordinate {
    type Abi = u32;
    unsafe fn from_abi(js: u32) -> Self {
        use wasm_bindgen::__rt::alloc::rc::Rc;
        use wasm_bindgen::__rt::core::result::Result::{Ok, Err};
        use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};
        let ptr = js as *mut WasmRefCell<Coordinate>;
        assert_not_null(ptr);
        let rc = Rc::from_raw(ptr);
        match Rc::try_unwrap(rc) {
            Ok(cell) => cell.into_inner(),
            Err(_) => {
                wasm_bindgen::throw_str(
                    "attempted to take ownership of Rust value while it was borrowed",
                )
            }
        }
    }
}
#[automatically_derived]
impl wasm_bindgen::__rt::core::convert::From<Coordinate> for wasm_bindgen::JsValue {
    fn from(value: Coordinate) -> Self {
        let ptr = wasm_bindgen::convert::IntoWasmAbi::into_abi(value);
        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        unsafe fn __wbg_coordinate_new(_: u32) -> u32 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("cannot convert to JsValue outside of the Wasm target"),
                );
            }
        }
        unsafe {
            <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                __wbg_coordinate_new(ptr),
            )
        }
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::RefFromWasmAbi for Coordinate {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::RcRef<Coordinate>;
    unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
        use wasm_bindgen::__rt::alloc::rc::Rc;
        let js = js as *mut wasm_bindgen::__rt::WasmRefCell<Coordinate>;
        wasm_bindgen::__rt::assert_not_null(js);
        Rc::increment_strong_count(js);
        let rc = Rc::from_raw(js);
        wasm_bindgen::__rt::RcRef::new(rc)
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::RefMutFromWasmAbi for Coordinate {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::RcRefMut<Coordinate>;
    unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor {
        use wasm_bindgen::__rt::alloc::rc::Rc;
        let js = js as *mut wasm_bindgen::__rt::WasmRefCell<Coordinate>;
        wasm_bindgen::__rt::assert_not_null(js);
        Rc::increment_strong_count(js);
        let rc = Rc::from_raw(js);
        wasm_bindgen::__rt::RcRefMut::new(rc)
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::LongRefFromWasmAbi for Coordinate {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::RcRef<Coordinate>;
    unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
        <Self as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(js)
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::OptionIntoWasmAbi for Coordinate {
    #[inline]
    fn none() -> Self::Abi {
        0
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::OptionFromWasmAbi for Coordinate {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        *abi == 0
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::TryFromJsValue for Coordinate {
    type Error = wasm_bindgen::JsValue;
    fn try_from_js_value(
        value: wasm_bindgen::JsValue,
    ) -> wasm_bindgen::__rt::core::result::Result<Self, Self::Error> {
        let idx = wasm_bindgen::convert::IntoWasmAbi::into_abi(&value);
        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        unsafe fn __wbg_coordinate_unwrap(_: u32) -> u32 {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "cannot convert from JsValue outside of the Wasm target",
                    ),
                );
            }
        }
        let ptr = unsafe { __wbg_coordinate_unwrap(idx) };
        if ptr == 0 {
            wasm_bindgen::__rt::core::result::Result::Err(value)
        } else {
            #[allow(clippy::mem_forget)] wasm_bindgen::__rt::core::mem::forget(value);
            unsafe {
                wasm_bindgen::__rt::core::result::Result::Ok(
                    <Self as wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr),
                )
            }
        }
    }
}
#[automatically_derived]
impl wasm_bindgen::describe::WasmDescribeVector for Coordinate {
    fn describe_vector() {
        use wasm_bindgen::describe::*;
        inform(VECTOR);
        inform(NAMED_EXTERNREF);
        inform(10u32);
        inform(67u32);
        inform(111u32);
        inform(111u32);
        inform(114u32);
        inform(100u32);
        inform(105u32);
        inform(110u32);
        inform(97u32);
        inform(116u32);
        inform(101u32);
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::VectorIntoWasmAbi for Coordinate {
    type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
        [wasm_bindgen::JsValue],
    > as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    fn vector_into_abi(
        vector: wasm_bindgen::__rt::alloc::boxed::Box<[Coordinate]>,
    ) -> Self::Abi {
        wasm_bindgen::convert::js_value_vector_into_abi(vector)
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::VectorFromWasmAbi for Coordinate {
    type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
        [wasm_bindgen::JsValue],
    > as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn vector_from_abi(
        js: Self::Abi,
    ) -> wasm_bindgen::__rt::alloc::boxed::Box<[Coordinate]> {
        wasm_bindgen::convert::js_value_vector_from_abi(js)
    }
}
#[automatically_derived]
impl wasm_bindgen::__rt::VectorIntoJsValue for Coordinate {
    fn vector_into_jsvalue(
        vector: wasm_bindgen::__rt::alloc::boxed::Box<[Coordinate]>,
    ) -> wasm_bindgen::JsValue {
        wasm_bindgen::__rt::js_value_vector_into_jsvalue(vector)
    }
}
#[automatically_derived]
const _: () = {
    #[doc(hidden)]
    pub unsafe extern "C" fn __wbg_get_coordinate_x(
        js: u32,
    ) -> wasm_bindgen::convert::WasmRet<
        <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
    > {
        use wasm_bindgen::__rt::{WasmRefCell, assert_not_null};
        use wasm_bindgen::convert::IntoWasmAbi;
        fn assert_copy<T: Copy>() {}
        assert_copy::<i32>();
        let js = js as *mut WasmRefCell<Coordinate>;
        assert_not_null(js);
        let val = (*js).borrow().x;
        <i32 as IntoWasmAbi>::into_abi(val).into()
    }
};
#[automatically_derived]
const _: () = {
    #[doc(hidden)]
    pub unsafe extern "C" fn __wbg_get_coordinate_y(
        js: u32,
    ) -> wasm_bindgen::convert::WasmRet<
        <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
    > {
        use wasm_bindgen::__rt::{WasmRefCell, assert_not_null};
        use wasm_bindgen::convert::IntoWasmAbi;
        fn assert_copy<T: Copy>() {}
        assert_copy::<i32>();
        let js = js as *mut WasmRefCell<Coordinate>;
        assert_not_null(js);
        let val = (*js).borrow().y;
        <i32 as IntoWasmAbi>::into_abi(val).into()
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for Coordinate {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Coordinate",
            "x",
            &self.x,
            "y",
            &&self.y,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Coordinate {
    #[inline]
    fn clone(&self) -> Coordinate {
        let _: ::core::clone::AssertParamIsClone<i32>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Coordinate {}
pub struct BotDetectionOutput {
    pub human_score: f32,
    pub timestamp: Timestamp,
    result_text: String,
}
#[automatically_derived]
impl wasm_bindgen::describe::WasmDescribe for BotDetectionOutput {
    fn describe() {
        use wasm_bindgen::__wbindgen_if_not_std;
        use wasm_bindgen::describe::*;
        inform(RUST_STRUCT);
        inform(18u32);
        inform(66u32);
        inform(111u32);
        inform(116u32);
        inform(68u32);
        inform(101u32);
        inform(116u32);
        inform(101u32);
        inform(99u32);
        inform(116u32);
        inform(105u32);
        inform(111u32);
        inform(110u32);
        inform(79u32);
        inform(117u32);
        inform(116u32);
        inform(112u32);
        inform(117u32);
        inform(116u32);
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::IntoWasmAbi for BotDetectionOutput {
    type Abi = u32;
    fn into_abi(self) -> u32 {
        use wasm_bindgen::__rt::alloc::rc::Rc;
        use wasm_bindgen::__rt::WasmRefCell;
        Rc::into_raw(Rc::new(WasmRefCell::new(self))) as u32
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::FromWasmAbi for BotDetectionOutput {
    type Abi = u32;
    unsafe fn from_abi(js: u32) -> Self {
        use wasm_bindgen::__rt::alloc::rc::Rc;
        use wasm_bindgen::__rt::core::result::Result::{Ok, Err};
        use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};
        let ptr = js as *mut WasmRefCell<BotDetectionOutput>;
        assert_not_null(ptr);
        let rc = Rc::from_raw(ptr);
        match Rc::try_unwrap(rc) {
            Ok(cell) => cell.into_inner(),
            Err(_) => {
                wasm_bindgen::throw_str(
                    "attempted to take ownership of Rust value while it was borrowed",
                )
            }
        }
    }
}
#[automatically_derived]
impl wasm_bindgen::__rt::core::convert::From<BotDetectionOutput>
for wasm_bindgen::JsValue {
    fn from(value: BotDetectionOutput) -> Self {
        let ptr = wasm_bindgen::convert::IntoWasmAbi::into_abi(value);
        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        unsafe fn __wbg_botdetectionoutput_new(_: u32) -> u32 {
            {
                ::core::panicking::panic_fmt(
                    format_args!("cannot convert to JsValue outside of the Wasm target"),
                );
            }
        }
        unsafe {
            <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                __wbg_botdetectionoutput_new(ptr),
            )
        }
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::RefFromWasmAbi for BotDetectionOutput {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::RcRef<BotDetectionOutput>;
    unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
        use wasm_bindgen::__rt::alloc::rc::Rc;
        let js = js as *mut wasm_bindgen::__rt::WasmRefCell<BotDetectionOutput>;
        wasm_bindgen::__rt::assert_not_null(js);
        Rc::increment_strong_count(js);
        let rc = Rc::from_raw(js);
        wasm_bindgen::__rt::RcRef::new(rc)
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::RefMutFromWasmAbi for BotDetectionOutput {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::RcRefMut<BotDetectionOutput>;
    unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor {
        use wasm_bindgen::__rt::alloc::rc::Rc;
        let js = js as *mut wasm_bindgen::__rt::WasmRefCell<BotDetectionOutput>;
        wasm_bindgen::__rt::assert_not_null(js);
        Rc::increment_strong_count(js);
        let rc = Rc::from_raw(js);
        wasm_bindgen::__rt::RcRefMut::new(rc)
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::LongRefFromWasmAbi for BotDetectionOutput {
    type Abi = u32;
    type Anchor = wasm_bindgen::__rt::RcRef<BotDetectionOutput>;
    unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
        <Self as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(js)
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::OptionIntoWasmAbi for BotDetectionOutput {
    #[inline]
    fn none() -> Self::Abi {
        0
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::OptionFromWasmAbi for BotDetectionOutput {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        *abi == 0
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::TryFromJsValue for BotDetectionOutput {
    type Error = wasm_bindgen::JsValue;
    fn try_from_js_value(
        value: wasm_bindgen::JsValue,
    ) -> wasm_bindgen::__rt::core::result::Result<Self, Self::Error> {
        let idx = wasm_bindgen::convert::IntoWasmAbi::into_abi(&value);
        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        unsafe fn __wbg_botdetectionoutput_unwrap(_: u32) -> u32 {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "cannot convert from JsValue outside of the Wasm target",
                    ),
                );
            }
        }
        let ptr = unsafe { __wbg_botdetectionoutput_unwrap(idx) };
        if ptr == 0 {
            wasm_bindgen::__rt::core::result::Result::Err(value)
        } else {
            #[allow(clippy::mem_forget)] wasm_bindgen::__rt::core::mem::forget(value);
            unsafe {
                wasm_bindgen::__rt::core::result::Result::Ok(
                    <Self as wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr),
                )
            }
        }
    }
}
#[automatically_derived]
impl wasm_bindgen::describe::WasmDescribeVector for BotDetectionOutput {
    fn describe_vector() {
        use wasm_bindgen::describe::*;
        inform(VECTOR);
        inform(NAMED_EXTERNREF);
        inform(18u32);
        inform(66u32);
        inform(111u32);
        inform(116u32);
        inform(68u32);
        inform(101u32);
        inform(116u32);
        inform(101u32);
        inform(99u32);
        inform(116u32);
        inform(105u32);
        inform(111u32);
        inform(110u32);
        inform(79u32);
        inform(117u32);
        inform(116u32);
        inform(112u32);
        inform(117u32);
        inform(116u32);
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::VectorIntoWasmAbi for BotDetectionOutput {
    type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
        [wasm_bindgen::JsValue],
    > as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    fn vector_into_abi(
        vector: wasm_bindgen::__rt::alloc::boxed::Box<[BotDetectionOutput]>,
    ) -> Self::Abi {
        wasm_bindgen::convert::js_value_vector_into_abi(vector)
    }
}
#[automatically_derived]
impl wasm_bindgen::convert::VectorFromWasmAbi for BotDetectionOutput {
    type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
        [wasm_bindgen::JsValue],
    > as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn vector_from_abi(
        js: Self::Abi,
    ) -> wasm_bindgen::__rt::alloc::boxed::Box<[BotDetectionOutput]> {
        wasm_bindgen::convert::js_value_vector_from_abi(js)
    }
}
#[automatically_derived]
impl wasm_bindgen::__rt::VectorIntoJsValue for BotDetectionOutput {
    fn vector_into_jsvalue(
        vector: wasm_bindgen::__rt::alloc::boxed::Box<[BotDetectionOutput]>,
    ) -> wasm_bindgen::JsValue {
        wasm_bindgen::__rt::js_value_vector_into_jsvalue(vector)
    }
}
#[automatically_derived]
const _: () = {
    #[doc(hidden)]
    pub unsafe extern "C" fn __wbg_get_botdetectionoutput_humanScore(
        js: u32,
    ) -> wasm_bindgen::convert::WasmRet<
        <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
    > {
        use wasm_bindgen::__rt::{WasmRefCell, assert_not_null};
        use wasm_bindgen::convert::IntoWasmAbi;
        fn assert_copy<T: Copy>() {}
        assert_copy::<f32>();
        let js = js as *mut WasmRefCell<BotDetectionOutput>;
        assert_not_null(js);
        let val = (*js).borrow().human_score;
        <f32 as IntoWasmAbi>::into_abi(val).into()
    }
};
#[automatically_derived]
const _: () = {
    #[doc(hidden)]
    pub unsafe extern "C" fn __wbg_get_botdetectionoutput_timestamp(
        js: u32,
    ) -> wasm_bindgen::convert::WasmRet<
        <Timestamp as wasm_bindgen::convert::IntoWasmAbi>::Abi,
    > {
        use wasm_bindgen::__rt::{WasmRefCell, assert_not_null};
        use wasm_bindgen::convert::IntoWasmAbi;
        fn assert_copy<T: Copy>() {}
        assert_copy::<Timestamp>();
        let js = js as *mut WasmRefCell<BotDetectionOutput>;
        assert_not_null(js);
        let val = (*js).borrow().timestamp;
        <Timestamp as IntoWasmAbi>::into_abi(val).into()
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for BotDetectionOutput {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "BotDetectionOutput",
            "human_score",
            &self.human_score,
            "timestamp",
            &self.timestamp,
            "result_text",
            &&self.result_text,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for BotDetectionOutput {
    #[inline]
    fn clone(&self) -> BotDetectionOutput {
        BotDetectionOutput {
            human_score: ::core::clone::Clone::clone(&self.human_score),
            timestamp: ::core::clone::Clone::clone(&self.timestamp),
            result_text: ::core::clone::Clone::clone(&self.result_text),
        }
    }
}
impl MyBotDetection {
    pub fn new() -> Self {
        #[automatically_derived]
        const _: () = {
            pub unsafe extern "C" fn __wasm_bindgen_generated_MyBotDetection_new() -> wasm_bindgen::convert::WasmRet<
                <MyBotDetection as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = {
                    let _ret = MyBotDetection::new();
                    _ret
                };
                <MyBotDetection as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(
                        _ret,
                    )
                    .into()
            }
        };
        Self {
            events: ::alloc::vec::Vec::new(),
            saved_results: ::alloc::vec::Vec::new(),
        }
    }
    pub fn from_events(events: Vec<Event>) -> Self {
        #[automatically_derived]
        const _: () = {
            pub unsafe extern "C" fn __wasm_bindgen_generated_MyBotDetection_fromEvents(
                arg0_1: <<Vec<
                    Event,
                > as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
                arg0_2: <<Vec<
                    Event,
                > as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
                arg0_3: <<Vec<
                    Event,
                > as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
                arg0_4: <<Vec<
                    Event,
                > as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            ) -> wasm_bindgen::convert::WasmRet<
                <MyBotDetection as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = {
                    let arg0 = unsafe {
                        <Vec<
                            Event,
                        > as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                            <<Vec<
                                Event,
                            > as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                arg0_1,
                                arg0_2,
                                arg0_3,
                                arg0_4,
                            ),
                        )
                    };
                    let _ret = MyBotDetection::from_events(arg0);
                    _ret
                };
                <MyBotDetection as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(
                        _ret,
                    )
                    .into()
            }
        };
        Self { events, ..Self::new() }
    }
    pub fn add_event(
        &mut self,
        timestamp: Timestamp,
        event: web_sys::MouseEvent,
    ) -> Result<(), JsValue> {
        #[automatically_derived]
        const _: () = {
            pub unsafe extern "C" fn __wasm_bindgen_generated_MyBotDetection_addEvent(
                me: u32,
                arg1_1: <<Timestamp as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
                arg1_2: <<Timestamp as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
                arg1_3: <<Timestamp as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
                arg1_4: <<Timestamp as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
                arg2_1: <<web_sys::MouseEvent as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
                arg2_2: <<web_sys::MouseEvent as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
                arg2_3: <<web_sys::MouseEvent as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
                arg2_4: <<web_sys::MouseEvent as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            ) -> wasm_bindgen::convert::WasmRet<
                <Result<(), JsValue> as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = {
                    let mut me = unsafe {
                        <MyBotDetection as wasm_bindgen::convert::RefMutFromWasmAbi>::ref_mut_from_abi(
                            me,
                        )
                    };
                    let me = &mut *me;
                    let arg1 = unsafe {
                        <Timestamp as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                            <<Timestamp as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                arg1_1,
                                arg1_2,
                                arg1_3,
                                arg1_4,
                            ),
                        )
                    };
                    let arg2 = unsafe {
                        <web_sys::MouseEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                            <<web_sys::MouseEvent as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                arg2_1,
                                arg2_2,
                                arg2_3,
                                arg2_4,
                            ),
                        )
                    };
                    let _ret = me.add_event(arg1, arg2);
                    _ret
                };
                <Result<
                    (),
                    JsValue,
                > as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                    .into()
            }
        };
        let new_event = Event {
            timestamp,
            coordinate: Coordinate {
                x: event.client_x(),
                y: event.client_y(),
            },
        };
        self.events.push(new_event);
        Ok(())
    }
    pub fn save_result(&mut self, result: BotDetectionOutput) {
        #[automatically_derived]
        const _: () = {
            pub unsafe extern "C" fn __wasm_bindgen_generated_MyBotDetection_saveResult(
                me: u32,
                arg1_1: <<BotDetectionOutput as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
                arg1_2: <<BotDetectionOutput as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
                arg1_3: <<BotDetectionOutput as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
                arg1_4: <<BotDetectionOutput as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            ) -> wasm_bindgen::convert::WasmRet<
                <() as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = {
                    let mut me = unsafe {
                        <MyBotDetection as wasm_bindgen::convert::RefMutFromWasmAbi>::ref_mut_from_abi(
                            me,
                        )
                    };
                    let me = &mut *me;
                    let arg1 = unsafe {
                        <BotDetectionOutput as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                            <<BotDetectionOutput as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                arg1_1,
                                arg1_2,
                                arg1_3,
                                arg1_4,
                            ),
                        )
                    };
                    let _ret = me.save_result(arg1);
                    _ret
                };
                <() as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret).into()
            }
        };
        self.saved_results.push(result);
    }
    pub fn save_borrowed_result(&mut self, result: &BotDetectionOutput) {
        #[automatically_derived]
        const _: () = {
            pub unsafe extern "C" fn __wasm_bindgen_generated_MyBotDetection_saveBorrowedResult(
                me: u32,
                arg1_1: <<BotDetectionOutput as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
                arg1_2: <<BotDetectionOutput as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
                arg1_3: <<BotDetectionOutput as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
                arg1_4: <<BotDetectionOutput as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            ) -> wasm_bindgen::convert::WasmRet<
                <() as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = {
                    let mut me = unsafe {
                        <MyBotDetection as wasm_bindgen::convert::RefMutFromWasmAbi>::ref_mut_from_abi(
                            me,
                        )
                    };
                    let me = &mut *me;
                    let arg1 = unsafe {
                        <BotDetectionOutput as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                            <<BotDetectionOutput as wasm_bindgen::convert::RefFromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                arg1_1,
                                arg1_2,
                                arg1_3,
                                arg1_4,
                            ),
                        )
                    };
                    let arg1 = &*arg1;
                    let _ret = me.save_borrowed_result(arg1);
                    _ret
                };
                <() as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret).into()
            }
        };
        self.saved_results.push(result.clone());
    }
    pub fn events(&self, start: usize, end: usize) -> Vec<Event> {
        #[automatically_derived]
        const _: () = {
            pub unsafe extern "C" fn __wasm_bindgen_generated_MyBotDetection_events(
                me: u32,
                arg1_1: <<usize as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
                arg1_2: <<usize as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
                arg1_3: <<usize as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
                arg1_4: <<usize as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
                arg2_1: <<usize as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
                arg2_2: <<usize as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
                arg2_3: <<usize as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
                arg2_4: <<usize as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            ) -> wasm_bindgen::convert::WasmRet<
                <Vec<Event> as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = {
                    let me = unsafe {
                        <MyBotDetection as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                            me,
                        )
                    };
                    let me = &*me;
                    let arg1 = unsafe {
                        <usize as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                            <<usize as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                arg1_1,
                                arg1_2,
                                arg1_3,
                                arg1_4,
                            ),
                        )
                    };
                    let arg2 = unsafe {
                        <usize as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                            <<usize as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                arg2_1,
                                arg2_2,
                                arg2_3,
                                arg2_4,
                            ),
                        )
                    };
                    let _ret = me.events(arg1, arg2);
                    _ret
                };
                <Vec<Event> as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                    .into()
            }
        };
        self.events[start..end].to_vec()
    }
    pub fn num_events(&self) -> usize {
        #[automatically_derived]
        const _: () = {
            pub unsafe extern "C" fn __wasm_bindgen_generated_MyBotDetection_num_events(
                me: u32,
            ) -> wasm_bindgen::convert::WasmRet<
                <usize as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = {
                    let me = unsafe {
                        <MyBotDetection as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                            me,
                        )
                    };
                    let me = &*me;
                    let _ret = me.num_events();
                    _ret
                };
                <usize as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret).into()
            }
        };
        self.events.len()
    }
    pub fn results(&self) -> Vec<BotDetectionOutput> {
        #[automatically_derived]
        const _: () = {
            pub unsafe extern "C" fn __wasm_bindgen_generated_MyBotDetection_results(
                me: u32,
            ) -> wasm_bindgen::convert::WasmRet<
                <Vec<BotDetectionOutput> as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = {
                    let me = unsafe {
                        <MyBotDetection as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                            me,
                        )
                    };
                    let me = &*me;
                    let _ret = me.results();
                    _ret
                };
                <Vec<
                    BotDetectionOutput,
                > as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                    .into()
            }
        };
        self.saved_results.clone()
    }
    pub fn is_bot(&self) -> BotDetectionOutput {
        #[automatically_derived]
        const _: () = {
            pub unsafe extern "C" fn __wasm_bindgen_generated_MyBotDetection_isBot(
                me: u32,
            ) -> wasm_bindgen::convert::WasmRet<
                <BotDetectionOutput as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = {
                    let me = unsafe {
                        <MyBotDetection as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                            me,
                        )
                    };
                    let me = &*me;
                    let _ret = me.is_bot();
                    _ret
                };
                <BotDetectionOutput as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(
                        _ret,
                    )
                    .into()
            }
        };
        let human_score = self.jitter();
        let result_text = if human_score < 0.5 { "Robot" } else { "Human" }.to_owned();
        let timestamp = self.events.last().map(|e| e.timestamp).unwrap_or_default();
        BotDetectionOutput {
            timestamp,
            human_score,
            result_text,
        }
    }
}
impl BotDetectionOutput {
    pub fn text(&self) -> String {
        #[automatically_derived]
        const _: () = {
            pub unsafe extern "C" fn __wasm_bindgen_generated_BotDetectionOutput_text(
                me: u32,
            ) -> wasm_bindgen::convert::WasmRet<
                <String as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = {
                    let me = unsafe {
                        <BotDetectionOutput as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(
                            me,
                        )
                    };
                    let me = &*me;
                    let _ret = me.text();
                    _ret
                };
                <String as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret).into()
            }
        };
        self.result_text.clone()
    }
}
impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        #[automatically_derived]
        const _: () = {
            pub unsafe extern "C" fn __wasm_bindgen_generated_Coordinate_new(
                arg0_1: <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
                arg0_2: <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
                arg0_3: <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
                arg0_4: <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
                arg1_1: <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
                arg1_2: <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
                arg1_3: <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
                arg1_4: <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            ) -> wasm_bindgen::convert::WasmRet<
                <Coordinate as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = {
                    let arg0 = unsafe {
                        <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                            <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                arg0_1,
                                arg0_2,
                                arg0_3,
                                arg0_4,
                            ),
                        )
                    };
                    let arg1 = unsafe {
                        <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                            <<i32 as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                arg1_1,
                                arg1_2,
                                arg1_3,
                                arg1_4,
                            ),
                        )
                    };
                    let _ret = Coordinate::new(arg0, arg1);
                    _ret
                };
                <Coordinate as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
                    .into()
            }
        };
        Self { x, y }
    }
}
