// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTapAndHoldGesture::NewQTapAndHoldGesture(QObject * parent);
  fn _ZN18QTapAndHoldGestureC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTapAndHoldGesture::FreeQTapAndHoldGesture();
  fn _ZN18QTapAndHoldGestureD0Ev(qthis: *mut c_void) ;
  // proto:  QPointF QTapAndHoldGesture::position();
  fn _ZNK18QTapAndHoldGesture8positionEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static void QTapAndHoldGesture::setTimeout(int msecs);
  fn _ZN18QTapAndHoldGesture10setTimeoutEi(arg0: c_int) ;
  // proto: static int QTapAndHoldGesture::timeout();
  fn _ZN18QTapAndHoldGesture7timeoutEv() -> c_int;
  // proto:  const QMetaObject * QTapAndHoldGesture::metaObject();
  fn _ZNK18QTapAndHoldGesture10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QTapAndHoldGesture::setPosition(const QPointF & pos);
  fn _ZN18QTapAndHoldGesture11setPositionERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QTapAndHoldGesture)=1
pub struct QTapAndHoldGesture {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTapAndHoldGesture {
  pub fn NewQTapAndHoldGesture<T: QTapAndHoldGesture_NewQTapAndHoldGesture>(value: T) -> QTapAndHoldGesture {
    let rsthis = value.NewQTapAndHoldGesture();
    return rsthis;
    // return 1;
  }
}

pub trait QTapAndHoldGesture_NewQTapAndHoldGesture {
  fn NewQTapAndHoldGesture(self) -> QTapAndHoldGesture;
}

// proto: void QTapAndHoldGesture::NewQTapAndHoldGesture(QObject * parent);
impl<'a> /*trait*/ QTapAndHoldGesture_NewQTapAndHoldGesture for (&'a mut QObject) {
  fn NewQTapAndHoldGesture(self) -> QTapAndHoldGesture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QTapAndHoldGestureC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QTapAndHoldGestureC1EP7QObject(qthis, arg0)};
    let rsthis = QTapAndHoldGesture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTapAndHoldGesture {
  pub fn FreeQTapAndHoldGesture<RetType, T: QTapAndHoldGesture_FreeQTapAndHoldGesture<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQTapAndHoldGesture(self);
    // return 1;
  }
}

pub trait QTapAndHoldGesture_FreeQTapAndHoldGesture<RetType> {
  fn FreeQTapAndHoldGesture(self, rsthis: &mut QTapAndHoldGesture) -> RetType;
}

// proto:  void QTapAndHoldGesture::FreeQTapAndHoldGesture();
impl<'a> /*trait*/ QTapAndHoldGesture_FreeQTapAndHoldGesture<()> for () {
  fn FreeQTapAndHoldGesture(self, rsthis: &mut QTapAndHoldGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QTapAndHoldGestureD0Ev()};
     unsafe {_ZN18QTapAndHoldGestureD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTapAndHoldGesture {
  pub fn position<RetType, T: QTapAndHoldGesture_position<RetType>>(&mut self, value: T) -> RetType {
    return value.position(self);
    // return 1;
  }
}

pub trait QTapAndHoldGesture_position<RetType> {
  fn position(self, rsthis: &mut QTapAndHoldGesture) -> RetType;
}

// proto:  QPointF QTapAndHoldGesture::position();
impl<'a> /*trait*/ QTapAndHoldGesture_position<QPointF> for () {
  fn position(self, rsthis: &mut QTapAndHoldGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QTapAndHoldGesture8positionEv()};
    let mut ret = unsafe {_ZNK18QTapAndHoldGesture8positionEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTapAndHoldGesture {
  pub fn setTimeout<RetType, T: QTapAndHoldGesture_setTimeout<RetType>>(&mut self, value: T) -> RetType {
    return value.setTimeout(self);
    // return 1;
  }
}

pub trait QTapAndHoldGesture_setTimeout<RetType> {
  fn setTimeout(self, rsthis: &mut QTapAndHoldGesture) -> RetType;
}

// proto: static void QTapAndHoldGesture::setTimeout(int msecs);
impl<'a> /*trait*/ QTapAndHoldGesture_setTimeout<()> for (i32) {
  fn setTimeout(self, rsthis: &mut QTapAndHoldGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QTapAndHoldGesture10setTimeoutEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN18QTapAndHoldGesture10setTimeoutEi(arg0)};
    // return 1;
  }
}

impl /*struct*/ QTapAndHoldGesture {
  pub fn timeout<RetType, T: QTapAndHoldGesture_timeout<RetType>>(&mut self, value: T) -> RetType {
    return value.timeout(self);
    // return 1;
  }
}

pub trait QTapAndHoldGesture_timeout<RetType> {
  fn timeout(self, rsthis: &mut QTapAndHoldGesture) -> RetType;
}

// proto: static int QTapAndHoldGesture::timeout();
impl<'a> /*trait*/ QTapAndHoldGesture_timeout<i32> for () {
  fn timeout(self, rsthis: &mut QTapAndHoldGesture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QTapAndHoldGesture7timeoutEv()};
    let mut ret = unsafe {_ZN18QTapAndHoldGesture7timeoutEv()};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTapAndHoldGesture {
  pub fn metaObject<RetType, T: QTapAndHoldGesture_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QTapAndHoldGesture_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QTapAndHoldGesture) -> RetType;
}

// proto:  const QMetaObject * QTapAndHoldGesture::metaObject();
impl<'a> /*trait*/ QTapAndHoldGesture_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QTapAndHoldGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QTapAndHoldGesture10metaObjectEv()};
     unsafe {_ZNK18QTapAndHoldGesture10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTapAndHoldGesture {
  pub fn setPosition<RetType, T: QTapAndHoldGesture_setPosition<RetType>>(&mut self, value: T) -> RetType {
    return value.setPosition(self);
    // return 1;
  }
}

pub trait QTapAndHoldGesture_setPosition<RetType> {
  fn setPosition(self, rsthis: &mut QTapAndHoldGesture) -> RetType;
}

// proto:  void QTapAndHoldGesture::setPosition(const QPointF & pos);
impl<'a> /*trait*/ QTapAndHoldGesture_setPosition<()> for (&'a  QPointF) {
  fn setPosition(self, rsthis: &mut QTapAndHoldGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QTapAndHoldGesture11setPositionERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QTapAndHoldGesture11setPositionERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

