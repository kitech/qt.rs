// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qgesture::QGesture;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGestureRecognizer::FreeQGestureRecognizer();
  fn _ZN18QGestureRecognizerD0Ev() -> i32;
  // proto: void QGestureRecognizer::NewQGestureRecognizer();
  fn _ZN18QGestureRecognizerC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QGestureRecognizer::reset(QGesture * state);
  fn _ZN18QGestureRecognizer5resetEP8QGesture(arg0: *mut c_void) -> i32;
  // proto: QGesture * QGestureRecognizer::create(QObject * target);
  fn _ZN18QGestureRecognizer6createEP7QObject(arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QGestureRecognizer)=8
pub struct QGestureRecognizer {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGestureRecognizer {
  pub fn FreeQGestureRecognizer<T: QGestureRecognizer_FreeQGestureRecognizer>(&mut self, value: T) -> i32 {
    value.FreeQGestureRecognizer(self);
    return 1;
  }
}

pub trait QGestureRecognizer_FreeQGestureRecognizer {
  fn FreeQGestureRecognizer(self, this: &mut QGestureRecognizer) -> i32;
}

// proto: void QGestureRecognizer::FreeQGestureRecognizer();
impl<'a> /*trait*/ QGestureRecognizer_FreeQGestureRecognizer for () {
  fn FreeQGestureRecognizer(self, this: &mut QGestureRecognizer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGestureRecognizerD0Ev()};
    unsafe {_ZN18QGestureRecognizerD0Ev()};
    return 1;
  }
}

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

// proto: void QGestureRecognizer::NewQGestureRecognizer();
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

impl /*struct*/ QGestureRecognizer {
  pub fn reset<T: QGestureRecognizer_reset>(&mut self, value: T) -> i32 {
    value.reset(self);
    return 1;
  }
}

pub trait QGestureRecognizer_reset {
  fn reset(self, this: &mut QGestureRecognizer) -> i32;
}

// proto: void QGestureRecognizer::reset(QGesture * state);
impl<'a> /*trait*/ QGestureRecognizer_reset for (&'a mut QGesture) {
  fn reset(self, this: &mut QGestureRecognizer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGestureRecognizer5resetEP8QGesture()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QGestureRecognizer5resetEP8QGesture(arg0)};
    return 1;
  }
}

impl /*struct*/ QGestureRecognizer {
  pub fn create<T: QGestureRecognizer_create>(&mut self, value: T) -> i32 {
    value.create(self);
    return 1;
  }
}

pub trait QGestureRecognizer_create {
  fn create(self, this: &mut QGestureRecognizer) -> i32;
}

// proto: QGesture * QGestureRecognizer::create(QObject * target);
impl<'a> /*trait*/ QGestureRecognizer_create for (&'a mut QObject) {
  fn create(self, this: &mut QGestureRecognizer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGestureRecognizer6createEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QGestureRecognizer6createEP7QObject(arg0)};
    return 1;
  }
}

