// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QSwipeGesture::FreeQSwipeGesture();
  fn _ZN13QSwipeGestureD0Ev(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QSwipeGesture::metaObject();
  fn _ZNK13QSwipeGesture10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QSwipeGesture::setSwipeAngle(qreal value);
  fn _ZN13QSwipeGesture13setSwipeAngleEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QSwipeGesture::NewQSwipeGesture(QObject * parent);
  fn _ZN13QSwipeGestureC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QSwipeGesture::swipeAngle();
  fn _ZNK13QSwipeGesture10swipeAngleEv(qthis: *mut c_void) -> c_double;
}

// body block begin
// class sizeof(QSwipeGesture)=1
pub struct QSwipeGesture {
  pub qclsinst: *mut c_void,
}

// proto:  void QSwipeGesture::FreeQSwipeGesture();
impl /*struct*/ QSwipeGesture {
  pub fn FreeQSwipeGesture<RetType, T: QSwipeGesture_FreeQSwipeGesture<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQSwipeGesture(self);
    // return 1;
  }
}

pub trait QSwipeGesture_FreeQSwipeGesture<RetType> {
  fn FreeQSwipeGesture(self , rsthis: &mut QSwipeGesture) -> RetType;
}

// proto:  void QSwipeGesture::FreeQSwipeGesture();
impl<'a> /*trait*/ QSwipeGesture_FreeQSwipeGesture<()> for () {
  fn FreeQSwipeGesture(self , rsthis: &mut QSwipeGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSwipeGestureD0Ev()};
     unsafe {_ZN13QSwipeGestureD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  const QMetaObject * QSwipeGesture::metaObject();
impl /*struct*/ QSwipeGesture {
  pub fn metaObject<RetType, T: QSwipeGesture_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSwipeGesture_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QSwipeGesture) -> RetType;
}

// proto:  const QMetaObject * QSwipeGesture::metaObject();
impl<'a> /*trait*/ QSwipeGesture_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QSwipeGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSwipeGesture10metaObjectEv()};
     unsafe {_ZNK13QSwipeGesture10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QSwipeGesture::setSwipeAngle(qreal value);
impl /*struct*/ QSwipeGesture {
  pub fn setSwipeAngle<RetType, T: QSwipeGesture_setSwipeAngle<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setSwipeAngle(self);
    // return 1;
  }
}

pub trait QSwipeGesture_setSwipeAngle<RetType> {
  fn setSwipeAngle(self , rsthis: &mut QSwipeGesture) -> RetType;
}

// proto:  void QSwipeGesture::setSwipeAngle(qreal value);
impl<'a> /*trait*/ QSwipeGesture_setSwipeAngle<()> for (f64) {
  fn setSwipeAngle(self , rsthis: &mut QSwipeGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSwipeGesture13setSwipeAngleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QSwipeGesture13setSwipeAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSwipeGesture {
  pub fn NewQSwipeGesture<T: QSwipeGesture_NewQSwipeGesture>(value: T) -> QSwipeGesture {
    let rsthis = value.NewQSwipeGesture();
    return rsthis;
    // return 1;
  }
}

pub trait QSwipeGesture_NewQSwipeGesture {
  fn NewQSwipeGesture(self) -> QSwipeGesture;
}

// proto: void QSwipeGesture::NewQSwipeGesture(QObject * parent);
impl<'a> /*trait*/ QSwipeGesture_NewQSwipeGesture for (&'a mut QObject) {
  fn NewQSwipeGesture(self) -> QSwipeGesture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSwipeGestureC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QSwipeGestureC1EP7QObject(qthis, arg0)};
    let rsthis = QSwipeGesture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  double QSwipeGesture::swipeAngle();
impl /*struct*/ QSwipeGesture {
  pub fn swipeAngle<RetType, T: QSwipeGesture_swipeAngle<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.swipeAngle(self);
    // return 1;
  }
}

pub trait QSwipeGesture_swipeAngle<RetType> {
  fn swipeAngle(self , rsthis: &mut QSwipeGesture) -> RetType;
}

// proto:  double QSwipeGesture::swipeAngle();
impl<'a> /*trait*/ QSwipeGesture_swipeAngle<f64> for () {
  fn swipeAngle(self , rsthis: &mut QSwipeGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSwipeGesture10swipeAngleEv()};
    let mut ret = unsafe {_ZNK13QSwipeGesture10swipeAngleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

