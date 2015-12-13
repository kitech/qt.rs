// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QPointF QGesture::hotSpot();
  fn _ZNK8QGesture7hotSpotEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGesture::hasHotSpot();
  fn _ZNK8QGesture10hasHotSpotEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QGesture::unsetHotSpot();
  fn _ZN8QGesture12unsetHotSpotEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QGesture::metaObject();
  fn _ZNK8QGesture10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QGesture::NewQGesture(QObject * parent);
  fn _ZN8QGestureC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGesture::setHotSpot(const QPointF & value);
  fn _ZN8QGesture10setHotSpotERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGesture::FreeQGesture();
  fn _ZN8QGestureD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QGesture)=1
pub struct QGesture {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGesture {
  pub fn hotSpot<T: QGesture_hotSpot>(&mut self, value: T) -> QPointF {
    return value.hotSpot(self);
    // return 1;
  }
}

pub trait QGesture_hotSpot {
  fn hotSpot(self, rsthis: &mut QGesture) -> QPointF;
}

// proto:  QPointF QGesture::hotSpot();
impl<'a> /*trait*/ QGesture_hotSpot for () {
  fn hotSpot(self, rsthis: &mut QGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QGesture7hotSpotEv()};
    let mut ret = unsafe {_ZNK8QGesture7hotSpotEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGesture {
  pub fn hasHotSpot<T: QGesture_hasHotSpot>(&mut self, value: T) -> i8 {
    return value.hasHotSpot(self);
    // return 1;
  }
}

pub trait QGesture_hasHotSpot {
  fn hasHotSpot(self, rsthis: &mut QGesture) -> i8;
}

// proto:  bool QGesture::hasHotSpot();
impl<'a> /*trait*/ QGesture_hasHotSpot for () {
  fn hasHotSpot(self, rsthis: &mut QGesture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QGesture10hasHotSpotEv()};
    let mut ret = unsafe {_ZNK8QGesture10hasHotSpotEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGesture {
  pub fn unsetHotSpot<T: QGesture_unsetHotSpot>(&mut self, value: T)  {
     value.unsetHotSpot(self);
    // return 1;
  }
}

pub trait QGesture_unsetHotSpot {
  fn unsetHotSpot(self, rsthis: &mut QGesture) ;
}

// proto:  void QGesture::unsetHotSpot();
impl<'a> /*trait*/ QGesture_unsetHotSpot for () {
  fn unsetHotSpot(self, rsthis: &mut QGesture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGesture12unsetHotSpotEv()};
     unsafe {_ZN8QGesture12unsetHotSpotEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGesture {
  pub fn metaObject<T: QGesture_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QGesture_metaObject {
  fn metaObject(self, rsthis: &mut QGesture) ;
}

// proto:  const QMetaObject * QGesture::metaObject();
impl<'a> /*trait*/ QGesture_metaObject for () {
  fn metaObject(self, rsthis: &mut QGesture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QGesture10metaObjectEv()};
     unsafe {_ZNK8QGesture10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGesture {
  pub fn NewQGesture<T: QGesture_NewQGesture>(value: T) -> QGesture {
    let rsthis = value.NewQGesture();
    return rsthis;
    // return 1;
  }
}

pub trait QGesture_NewQGesture {
  fn NewQGesture(self) -> QGesture;
}

// proto: void QGesture::NewQGesture(QObject * parent);
impl<'a> /*trait*/ QGesture_NewQGesture for (&'a mut QObject) {
  fn NewQGesture(self) -> QGesture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGestureC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QGestureC1EP7QObject(qthis, arg0)};
    let rsthis = QGesture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGesture {
  pub fn setHotSpot<T: QGesture_setHotSpot>(&mut self, value: T)  {
     value.setHotSpot(self);
    // return 1;
  }
}

pub trait QGesture_setHotSpot {
  fn setHotSpot(self, rsthis: &mut QGesture) ;
}

// proto:  void QGesture::setHotSpot(const QPointF & value);
impl<'a> /*trait*/ QGesture_setHotSpot for (&'a  QPointF) {
  fn setHotSpot(self, rsthis: &mut QGesture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGesture10setHotSpotERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QGesture10setHotSpotERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGesture {
  pub fn FreeQGesture<T: QGesture_FreeQGesture>(&mut self, value: T)  {
     value.FreeQGesture(self);
    // return 1;
  }
}

pub trait QGesture_FreeQGesture {
  fn FreeQGesture(self, rsthis: &mut QGesture) ;
}

// proto:  void QGesture::FreeQGesture();
impl<'a> /*trait*/ QGesture_FreeQGesture for () {
  fn FreeQGesture(self, rsthis: &mut QGesture)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGestureD0Ev()};
     unsafe {_ZN8QGestureD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

