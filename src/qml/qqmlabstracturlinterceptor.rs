// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmlabstracturlinterceptor.h
// dst-file: /src/qml/qqmlabstracturlinterceptor.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use std::ops::Deref;
use super::super::core::qurl::QUrl; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlAbstractUrlInterceptor_Class_Size() -> c_int;
  // proto:  void QQmlAbstractUrlInterceptor::~QQmlAbstractUrlInterceptor();
  fn _ZN26QQmlAbstractUrlInterceptorD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlAbstractUrlInterceptor::QQmlAbstractUrlInterceptor();
  fn _ZN26QQmlAbstractUrlInterceptorC2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QQmlAbstractUrlInterceptor)=8
#[derive(Default)]
pub struct QQmlAbstractUrlInterceptor {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QQmlAbstractUrlInterceptor {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlAbstractUrlInterceptor {
    return QQmlAbstractUrlInterceptor{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QQmlAbstractUrlInterceptor::~QQmlAbstractUrlInterceptor();
impl /*struct*/ QQmlAbstractUrlInterceptor {
  pub fn free<RetType, T: QQmlAbstractUrlInterceptor_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlAbstractUrlInterceptor_free<RetType> {
  fn free(self , rsthis: & QQmlAbstractUrlInterceptor) -> RetType;
}

  // proto:  void QQmlAbstractUrlInterceptor::~QQmlAbstractUrlInterceptor();
impl<'a> /*trait*/ QQmlAbstractUrlInterceptor_free<()> for () {
  fn free(self , rsthis: & QQmlAbstractUrlInterceptor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QQmlAbstractUrlInterceptorD2Ev()};
     unsafe {_ZN26QQmlAbstractUrlInterceptorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlAbstractUrlInterceptor::QQmlAbstractUrlInterceptor();
impl /*struct*/ QQmlAbstractUrlInterceptor {
  pub fn new<T: QQmlAbstractUrlInterceptor_new>(value: T) -> QQmlAbstractUrlInterceptor {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlAbstractUrlInterceptor_new {
  fn new(self) -> QQmlAbstractUrlInterceptor;
}

  // proto:  void QQmlAbstractUrlInterceptor::QQmlAbstractUrlInterceptor();
impl<'a> /*trait*/ QQmlAbstractUrlInterceptor_new for () {
  fn new(self) -> QQmlAbstractUrlInterceptor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QQmlAbstractUrlInterceptorC2Ev()};
    let ctysz: c_int = unsafe{QQmlAbstractUrlInterceptor_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN26QQmlAbstractUrlInterceptorC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlAbstractUrlInterceptor{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

