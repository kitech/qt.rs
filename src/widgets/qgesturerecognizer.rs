// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtWidgets/qgesturerecognizer.h
// dst-file: /src/widgets/qgesturerecognizer.rs
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
use super::qgesture::QGesture; // 773
use super::super::core::qobject::QObject; // 771
use super::super::core::qcoreevent::QEvent; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGestureRecognizer_Class_Size() -> c_int;
  // proto:  void QGestureRecognizer::~QGestureRecognizer();
  fn C_ZN18QGestureRecognizerD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGestureRecognizer::QGestureRecognizer();
  fn C_ZN18QGestureRecognizerC2Ev() -> u64;
  // proto:  void QGestureRecognizer::reset(QGesture * state);
  fn C_ZN18QGestureRecognizer5resetEP8QGesture(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QGesture * QGestureRecognizer::create(QObject * target);
  fn C_ZN18QGestureRecognizer6createEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QGestureRecognizer)=8
#[derive(Default)]
pub struct QGestureRecognizer {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QGestureRecognizer {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGestureRecognizer {
    return QGestureRecognizer{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QGestureRecognizer::~QGestureRecognizer();
impl /*struct*/ QGestureRecognizer {
  pub fn free<RetType, T: QGestureRecognizer_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGestureRecognizer_free<RetType> {
  fn free(self , rsthis: & QGestureRecognizer) -> RetType;
}

  // proto:  void QGestureRecognizer::~QGestureRecognizer();
impl<'a> /*trait*/ QGestureRecognizer_free<()> for () {
  fn free(self , rsthis: & QGestureRecognizer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGestureRecognizerD2Ev()};
     unsafe {C_ZN18QGestureRecognizerD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGestureRecognizer::QGestureRecognizer();
impl /*struct*/ QGestureRecognizer {
  pub fn new<T: QGestureRecognizer_new>(value: T) -> QGestureRecognizer {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGestureRecognizer_new {
  fn new(self) -> QGestureRecognizer;
}

  // proto:  void QGestureRecognizer::QGestureRecognizer();
impl<'a> /*trait*/ QGestureRecognizer_new for () {
  fn new(self) -> QGestureRecognizer {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGestureRecognizerC2Ev()};
    let ctysz: c_int = unsafe{QGestureRecognizer_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN18QGestureRecognizerC2Ev()};
    let rsthis = QGestureRecognizer{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGestureRecognizer::reset(QGesture * state);
impl /*struct*/ QGestureRecognizer {
  pub fn reset<RetType, T: QGestureRecognizer_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QGestureRecognizer_reset<RetType> {
  fn reset(self , rsthis: & QGestureRecognizer) -> RetType;
}

  // proto:  void QGestureRecognizer::reset(QGesture * state);
impl<'a> /*trait*/ QGestureRecognizer_reset<()> for (&'a QGesture) {
  fn reset(self , rsthis: & QGestureRecognizer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGestureRecognizer5resetEP8QGesture()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QGestureRecognizer5resetEP8QGesture(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGesture * QGestureRecognizer::create(QObject * target);
impl /*struct*/ QGestureRecognizer {
  pub fn create<RetType, T: QGestureRecognizer_create<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QGestureRecognizer_create<RetType> {
  fn create(self , rsthis: & QGestureRecognizer) -> RetType;
}

  // proto:  QGesture * QGestureRecognizer::create(QObject * target);
impl<'a> /*trait*/ QGestureRecognizer_create<QGesture> for (&'a QObject) {
  fn create(self , rsthis: & QGestureRecognizer) -> QGesture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGestureRecognizer6createEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN18QGestureRecognizer6createEP7QObject(rsthis.qclsinst, arg0)};
    let mut ret1 = QGesture::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

