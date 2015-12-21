// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
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
use super::qgesture::QGesture; // 773
use super::super::core::qobject::QObject; // 771
use super::super::core::qcoreevent::QEvent; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QGestureRecognizer::~QGestureRecognizer();
  fn _ZN18QGestureRecognizerD0Ev(qthis: *mut c_void);
  // proto:  void QGestureRecognizer::QGestureRecognizer();
  fn _ZN18QGestureRecognizerC1Ev(qthis: *mut c_void);
  // proto:  void QGestureRecognizer::reset(QGesture * state);
  fn _ZN18QGestureRecognizer5resetEP8QGesture(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QGesture * QGestureRecognizer::create(QObject * target);
  fn _ZN18QGestureRecognizer6createEP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QGestureRecognizer)=8
pub struct QGestureRecognizer {
  pub qclsinst: *mut c_void,
}

  // proto:  void QGestureRecognizer::~QGestureRecognizer();
impl /*struct*/ QGestureRecognizer {
  pub fn FreeQGestureRecognizer<RetType, T: QGestureRecognizer_FreeQGestureRecognizer<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGestureRecognizer(self);
    // return 1;
  }
}

pub trait QGestureRecognizer_FreeQGestureRecognizer<RetType> {
  fn FreeQGestureRecognizer(self , rsthis: &mut QGestureRecognizer) -> RetType;
}

  // proto:  void QGestureRecognizer::~QGestureRecognizer();
impl<'a> /*trait*/ QGestureRecognizer_FreeQGestureRecognizer<()> for () {
  fn FreeQGestureRecognizer(self , rsthis: &mut QGestureRecognizer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGestureRecognizerD0Ev()};
     unsafe {_ZN18QGestureRecognizerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGestureRecognizer::QGestureRecognizer();
impl /*struct*/ QGestureRecognizer {
  pub fn NewQGestureRecognizer<T: QGestureRecognizer_NewQGestureRecognizer>(value: T) -> QGestureRecognizer {
    let rsthis = value.NewQGestureRecognizer();
    return rsthis;
    // return 1;
  }
}

pub trait QGestureRecognizer_NewQGestureRecognizer {
  fn NewQGestureRecognizer(self) -> QGestureRecognizer;
}

  // proto:  void QGestureRecognizer::QGestureRecognizer();
impl<'a> /*trait*/ QGestureRecognizer_NewQGestureRecognizer for () {
  fn NewQGestureRecognizer(self) -> QGestureRecognizer {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGestureRecognizerC1Ev()};
    unsafe {_ZN18QGestureRecognizerC1Ev(qthis)};
    let rsthis = QGestureRecognizer{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGestureRecognizer::reset(QGesture * state);
impl /*struct*/ QGestureRecognizer {
  pub fn reset<RetType, T: QGestureRecognizer_reset<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QGestureRecognizer_reset<RetType> {
  fn reset(self , rsthis: &mut QGestureRecognizer) -> RetType;
}

  // proto:  void QGestureRecognizer::reset(QGesture * state);
impl<'a> /*trait*/ QGestureRecognizer_reset<()> for (QGesture) {
  fn reset(self , rsthis: &mut QGestureRecognizer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGestureRecognizer5resetEP8QGesture()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QGestureRecognizer5resetEP8QGesture(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGesture * QGestureRecognizer::create(QObject * target);
impl /*struct*/ QGestureRecognizer {
  pub fn create<RetType, T: QGestureRecognizer_create<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QGestureRecognizer_create<RetType> {
  fn create(self , rsthis: &mut QGestureRecognizer) -> RetType;
}

  // proto:  QGesture * QGestureRecognizer::create(QObject * target);
impl<'a> /*trait*/ QGestureRecognizer_create<QGesture> for (QObject) {
  fn create(self , rsthis: &mut QGestureRecognizer) -> QGesture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGestureRecognizer6createEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QGestureRecognizer6createEP7QObject(rsthis.qclsinst, arg0)};
    let mut ret1 = QGesture{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// <= body block end

