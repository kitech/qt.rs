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
  // proto:  QPointF QTapGesture::position();
  fn _ZNK11QTapGesture8positionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTapGesture::setPosition(const QPointF & pos);
  fn _ZN11QTapGesture11setPositionERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTapGesture::NewQTapGesture(QObject * parent);
  fn _ZN11QTapGestureC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QTapGesture::metaObject();
  fn _ZNK11QTapGesture10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QTapGesture::FreeQTapGesture();
  fn _ZN11QTapGestureD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QTapGesture)=1
pub struct QTapGesture {
  pub qclsinst: *mut c_void,
}

// proto:  QPointF QTapGesture::position();
impl /*struct*/ QTapGesture {
  pub fn position<RetType, T: QTapGesture_position<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QTapGesture_position<RetType> {
  fn position(self , rsthis: &mut QTapGesture) -> RetType;
}

// proto:  QPointF QTapGesture::position();
impl<'a> /*trait*/ QTapGesture_position<QPointF> for () {
  fn position(self , rsthis: &mut QTapGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTapGesture8positionEv()};
    let mut ret = unsafe {_ZNK11QTapGesture8positionEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTapGesture::setPosition(const QPointF & pos);
impl /*struct*/ QTapGesture {
  pub fn setPosition<RetType, T: QTapGesture_setPosition<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setPosition(self);
    // return 1;
  }
}

pub trait QTapGesture_setPosition<RetType> {
  fn setPosition(self , rsthis: &mut QTapGesture) -> RetType;
}

// proto:  void QTapGesture::setPosition(const QPointF & pos);
impl<'a> /*trait*/ QTapGesture_setPosition<()> for (&'a  QPointF) {
  fn setPosition(self , rsthis: &mut QTapGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTapGesture11setPositionERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTapGesture11setPositionERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTapGesture {
  pub fn NewQTapGesture<T: QTapGesture_NewQTapGesture>(value: T) -> QTapGesture {
    let rsthis = value.NewQTapGesture();
    return rsthis;
    // return 1;
  }
}

pub trait QTapGesture_NewQTapGesture {
  fn NewQTapGesture(self) -> QTapGesture;
}

// proto: void QTapGesture::NewQTapGesture(QObject * parent);
impl<'a> /*trait*/ QTapGesture_NewQTapGesture for (&'a mut QObject) {
  fn NewQTapGesture(self) -> QTapGesture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTapGestureC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTapGestureC1EP7QObject(qthis, arg0)};
    let rsthis = QTapGesture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  const QMetaObject * QTapGesture::metaObject();
impl /*struct*/ QTapGesture {
  pub fn metaObject<RetType, T: QTapGesture_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTapGesture_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QTapGesture) -> RetType;
}

// proto:  const QMetaObject * QTapGesture::metaObject();
impl<'a> /*trait*/ QTapGesture_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QTapGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTapGesture10metaObjectEv()};
     unsafe {_ZNK11QTapGesture10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QTapGesture::FreeQTapGesture();
impl /*struct*/ QTapGesture {
  pub fn FreeQTapGesture<RetType, T: QTapGesture_FreeQTapGesture<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQTapGesture(self);
    // return 1;
  }
}

pub trait QTapGesture_FreeQTapGesture<RetType> {
  fn FreeQTapGesture(self , rsthis: &mut QTapGesture) -> RetType;
}

// proto:  void QTapGesture::FreeQTapGesture();
impl<'a> /*trait*/ QTapGesture_FreeQTapGesture<()> for () {
  fn FreeQTapGesture(self , rsthis: &mut QTapGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTapGestureD0Ev()};
     unsafe {_ZN11QTapGestureD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

