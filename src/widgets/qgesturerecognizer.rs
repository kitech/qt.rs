// auto generated, do not modify.
// created: Wed Dec 30 23:22:52 2015
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
  fn _ZN18QGestureRecognizerD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGestureRecognizer::QGestureRecognizer();
  fn dector_ZN18QGestureRecognizerC1Ev() -> *mut c_void;
  fn _ZN18QGestureRecognizerC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGestureRecognizer::reset(QGesture * state);
  fn _ZN18QGestureRecognizer5resetEP8QGesture(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QGesture * QGestureRecognizer::create(QObject * target);
  fn _ZN18QGestureRecognizer6createEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
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
  pub fn Free<RetType, T: QGestureRecognizer_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QGestureRecognizer_Free<RetType> {
  fn Free(self , rsthis: & QGestureRecognizer) -> RetType;
}

  // proto:  void QGestureRecognizer::~QGestureRecognizer();
impl<'a> /*trait*/ QGestureRecognizer_Free<()> for () {
  fn Free(self , rsthis: & QGestureRecognizer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGestureRecognizerD0Ev()};
     unsafe {_ZN18QGestureRecognizerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGestureRecognizer::QGestureRecognizer();
impl /*struct*/ QGestureRecognizer {
  pub fn New<T: QGestureRecognizer_New>(value: T) -> QGestureRecognizer {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGestureRecognizer_New {
  fn New(self) -> QGestureRecognizer;
}

  // proto:  void QGestureRecognizer::QGestureRecognizer();
impl<'a> /*trait*/ QGestureRecognizer_New for () {
  fn New(self) -> QGestureRecognizer {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGestureRecognizerC1Ev()};
    let ctysz: c_int = unsafe{QGestureRecognizer_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN18QGestureRecognizerC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN18QGestureRecognizerC1Ev()} as u64;
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
     unsafe {_ZN18QGestureRecognizer5resetEP8QGesture(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZN18QGestureRecognizer6createEP7QObject(rsthis.qclsinst, arg0)};
    let mut ret1 = QGesture::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

