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
  // proto:  void QGestureRecognizer::FreeQGestureRecognizer();
  fn _ZN18QGestureRecognizerD0Ev(qthis: *mut c_void) ;
  // proto:  void QGestureRecognizer::NewQGestureRecognizer();
  fn _ZN18QGestureRecognizerC1Ev(qthis: *mut c_void) ;
  // proto:  void QGestureRecognizer::reset(QGesture * state);
  fn _ZN18QGestureRecognizer5resetEP8QGesture(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QGesture * QGestureRecognizer::create(QObject * target);
  fn _ZN18QGestureRecognizer6createEP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QGestureRecognizer)=8
pub struct QGestureRecognizer {
  pub qclsinst: *mut c_void,
}

// proto:  void QGestureRecognizer::FreeQGestureRecognizer();
impl /*struct*/ QGestureRecognizer {
  pub fn FreeQGestureRecognizer<RetType, T: QGestureRecognizer_FreeQGestureRecognizer<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQGestureRecognizer(self);
    // return 1;
  }
}

pub trait QGestureRecognizer_FreeQGestureRecognizer<RetType> {
  fn FreeQGestureRecognizer(self , rsthis: &mut QGestureRecognizer) -> RetType;
}

// proto:  void QGestureRecognizer::FreeQGestureRecognizer();
impl<'a> /*trait*/ QGestureRecognizer_FreeQGestureRecognizer<()> for () {
  fn FreeQGestureRecognizer(self , rsthis: &mut QGestureRecognizer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGestureRecognizerD0Ev()};
     unsafe {_ZN18QGestureRecognizerD0Ev(rsthis.qclsinst)};
    // return 1;
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

// proto:  void QGestureRecognizer::reset(QGesture * state);
impl /*struct*/ QGestureRecognizer {
  pub fn reset<RetType, T: QGestureRecognizer_reset<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QGestureRecognizer_reset<RetType> {
  fn reset(self , rsthis: &mut QGestureRecognizer) -> RetType;
}

// proto:  void QGestureRecognizer::reset(QGesture * state);
impl<'a> /*trait*/ QGestureRecognizer_reset<()> for (&'a mut QGesture) {
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
  pub fn create<RetType, T: QGestureRecognizer_create<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QGestureRecognizer_create<RetType> {
  fn create(self , rsthis: &mut QGestureRecognizer) -> RetType;
}

// proto:  QGesture * QGestureRecognizer::create(QObject * target);
impl<'a> /*trait*/ QGestureRecognizer_create<QGesture> for (&'a mut QObject) {
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

