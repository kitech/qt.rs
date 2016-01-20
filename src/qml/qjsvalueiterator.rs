// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qjsvalueiterator.h
// dst-file: /src/qml/qjsvalueiterator.rs
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
use super::qjsvalue::QJSValue; // 773
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QJSValueIterator_Class_Size() -> c_int;
  // proto:  QJSValue QJSValueIterator::value();
  fn _ZNK16QJSValueIterator5valueEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QJSValueIterator::next();
  fn _ZN16QJSValueIterator4nextEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QJSValueIterator::~QJSValueIterator();
  fn _ZN16QJSValueIteratorD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QJSValueIterator::QJSValueIterator(const QJSValue & value);
  fn _ZN16QJSValueIteratorC2ERK8QJSValue(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QJSValueIterator::QJSValueIterator(const QJSValueIterator & );
  fn _ZN16QJSValueIteratorC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QJSValueIterator::hasNext();
  fn _ZNK16QJSValueIterator7hasNextEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QJSValueIterator::name();
  fn _ZNK16QJSValueIterator4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QJSValueIterator)=1
#[derive(Default)]
pub struct QJSValueIterator {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QJSValueIterator {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QJSValueIterator {
    return QJSValueIterator{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QJSValue QJSValueIterator::value();
impl /*struct*/ QJSValueIterator {
  pub fn value<RetType, T: QJSValueIterator_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QJSValueIterator_value<RetType> {
  fn value(self , rsthis: & QJSValueIterator) -> RetType;
}

  // proto:  QJSValue QJSValueIterator::value();
impl<'a> /*trait*/ QJSValueIterator_value<QJSValue> for () {
  fn value(self , rsthis: & QJSValueIterator) -> QJSValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QJSValueIterator5valueEv()};
    let mut ret = unsafe {_ZNK16QJSValueIterator5valueEv(rsthis.qclsinst)};
    let mut ret1 = QJSValue::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QJSValueIterator::next();
impl /*struct*/ QJSValueIterator {
  pub fn next<RetType, T: QJSValueIterator_next<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.next(self);
    // return 1;
  }
}

pub trait QJSValueIterator_next<RetType> {
  fn next(self , rsthis: & QJSValueIterator) -> RetType;
}

  // proto:  bool QJSValueIterator::next();
impl<'a> /*trait*/ QJSValueIterator_next<i8> for () {
  fn next(self , rsthis: & QJSValueIterator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QJSValueIterator4nextEv()};
    let mut ret = unsafe {_ZN16QJSValueIterator4nextEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QJSValueIterator::~QJSValueIterator();
impl /*struct*/ QJSValueIterator {
  pub fn free<RetType, T: QJSValueIterator_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QJSValueIterator_free<RetType> {
  fn free(self , rsthis: & QJSValueIterator) -> RetType;
}

  // proto:  void QJSValueIterator::~QJSValueIterator();
impl<'a> /*trait*/ QJSValueIterator_free<()> for () {
  fn free(self , rsthis: & QJSValueIterator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QJSValueIteratorD2Ev()};
     unsafe {_ZN16QJSValueIteratorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QJSValueIterator::QJSValueIterator(const QJSValue & value);
impl /*struct*/ QJSValueIterator {
  pub fn new<T: QJSValueIterator_new>(value: T) -> QJSValueIterator {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QJSValueIterator_new {
  fn new(self) -> QJSValueIterator;
}

  // proto:  void QJSValueIterator::QJSValueIterator(const QJSValue & value);
impl<'a> /*trait*/ QJSValueIterator_new for (&'a QJSValue) {
  fn new(self) -> QJSValueIterator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QJSValueIteratorC2ERK8QJSValue()};
    let ctysz: c_int = unsafe{QJSValueIterator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QJSValueIteratorC2ERK8QJSValue(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QJSValueIterator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QJSValueIterator::QJSValueIterator(const QJSValueIterator & );
impl<'a> /*trait*/ QJSValueIterator_new for (&'a QJSValueIterator) {
  fn new(self) -> QJSValueIterator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QJSValueIteratorC2ERKS_()};
    let ctysz: c_int = unsafe{QJSValueIterator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QJSValueIteratorC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QJSValueIterator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QJSValueIterator::hasNext();
impl /*struct*/ QJSValueIterator {
  pub fn hasNext<RetType, T: QJSValueIterator_hasNext<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasNext(self);
    // return 1;
  }
}

pub trait QJSValueIterator_hasNext<RetType> {
  fn hasNext(self , rsthis: & QJSValueIterator) -> RetType;
}

  // proto:  bool QJSValueIterator::hasNext();
impl<'a> /*trait*/ QJSValueIterator_hasNext<i8> for () {
  fn hasNext(self , rsthis: & QJSValueIterator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QJSValueIterator7hasNextEv()};
    let mut ret = unsafe {_ZNK16QJSValueIterator7hasNextEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QJSValueIterator::name();
impl /*struct*/ QJSValueIterator {
  pub fn name<RetType, T: QJSValueIterator_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QJSValueIterator_name<RetType> {
  fn name(self , rsthis: & QJSValueIterator) -> RetType;
}

  // proto:  QString QJSValueIterator::name();
impl<'a> /*trait*/ QJSValueIterator_name<QString> for () {
  fn name(self , rsthis: & QJSValueIterator) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QJSValueIterator4nameEv()};
    let mut ret = unsafe {_ZNK16QJSValueIterator4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

