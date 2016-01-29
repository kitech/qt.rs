// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtCore/qabstractnativeeventfilter.h
// dst-file: /src/core/qabstractnativeeventfilter.rs
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
use super::qbytearray::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QAbstractNativeEventFilter_Class_Size() -> c_int;
  // proto:  bool QAbstractNativeEventFilter::nativeEventFilter(const QByteArray & eventType, void * message, long * result);
  fn C_ZN26QAbstractNativeEventFilter17nativeEventFilterERK10QByteArrayPvPl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_long) -> c_char;
  // proto:  void QAbstractNativeEventFilter::QAbstractNativeEventFilter();
  fn C_ZN26QAbstractNativeEventFilterC2Ev() -> u64;
  // proto:  void QAbstractNativeEventFilter::~QAbstractNativeEventFilter();
  fn C_ZN26QAbstractNativeEventFilterD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractNativeEventFilter)=16
#[derive(Default)]
pub struct QAbstractNativeEventFilter {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QAbstractNativeEventFilter {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractNativeEventFilter {
    return QAbstractNativeEventFilter{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QAbstractNativeEventFilter::nativeEventFilter(const QByteArray & eventType, void * message, long * result);
impl /*struct*/ QAbstractNativeEventFilter {
  pub fn nativeEventFilter<RetType, T: QAbstractNativeEventFilter_nativeEventFilter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nativeEventFilter(self);
    // return 1;
  }
}

pub trait QAbstractNativeEventFilter_nativeEventFilter<RetType> {
  fn nativeEventFilter(self , rsthis: & QAbstractNativeEventFilter) -> RetType;
}

  // proto:  bool QAbstractNativeEventFilter::nativeEventFilter(const QByteArray & eventType, void * message, long * result);
impl<'a> /*trait*/ QAbstractNativeEventFilter_nativeEventFilter<i8> for (&'a QByteArray, *mut c_void, &'a mut Vec<i64>) {
  fn nativeEventFilter(self , rsthis: & QAbstractNativeEventFilter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAbstractNativeEventFilter17nativeEventFilterERK10QByteArrayPvPl()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as *mut c_void;
    let arg2 = self.2.as_ptr()  as *mut c_long;
    let mut ret = unsafe {C_ZN26QAbstractNativeEventFilter17nativeEventFilterERK10QByteArrayPvPl(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QAbstractNativeEventFilter::QAbstractNativeEventFilter();
impl /*struct*/ QAbstractNativeEventFilter {
  pub fn new<T: QAbstractNativeEventFilter_new>(value: T) -> QAbstractNativeEventFilter {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractNativeEventFilter_new {
  fn new(self) -> QAbstractNativeEventFilter;
}

  // proto:  void QAbstractNativeEventFilter::QAbstractNativeEventFilter();
impl<'a> /*trait*/ QAbstractNativeEventFilter_new for () {
  fn new(self) -> QAbstractNativeEventFilter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAbstractNativeEventFilterC2Ev()};
    let ctysz: c_int = unsafe{QAbstractNativeEventFilter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN26QAbstractNativeEventFilterC2Ev()};
    let rsthis = QAbstractNativeEventFilter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAbstractNativeEventFilter::~QAbstractNativeEventFilter();
impl /*struct*/ QAbstractNativeEventFilter {
  pub fn free<RetType, T: QAbstractNativeEventFilter_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QAbstractNativeEventFilter_free<RetType> {
  fn free(self , rsthis: & QAbstractNativeEventFilter) -> RetType;
}

  // proto:  void QAbstractNativeEventFilter::~QAbstractNativeEventFilter();
impl<'a> /*trait*/ QAbstractNativeEventFilter_free<()> for () {
  fn free(self , rsthis: & QAbstractNativeEventFilter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAbstractNativeEventFilterD2Ev()};
     unsafe {C_ZN26QAbstractNativeEventFilterD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

