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
  // proto:  bool QChildEvent::added();
  fn _ZNK11QChildEvent5addedEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QChildEvent::polished();
  fn _ZNK11QChildEvent8polishedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QChildEvent::FreeQChildEvent();
  fn _ZN11QChildEventD0Ev(qthis: *mut c_void) ;
  // proto:  bool QChildEvent::removed();
  fn _ZNK11QChildEvent7removedEv(qthis: *mut c_void) -> int8_t;
  // proto:  QObject * QChildEvent::child();
  fn _ZNK11QChildEvent5childEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QChildEvent)=32
pub struct QChildEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QChildEvent {
  pub fn added<RetType, T: QChildEvent_added<RetType>>(&mut self, value: T) -> RetType {
    return value.added(self);
    // return 1;
  }
}

pub trait QChildEvent_added<RetType> {
  fn added(self, rsthis: &mut QChildEvent) -> RetType;
}

// proto:  bool QChildEvent::added();
impl<'a> /*trait*/ QChildEvent_added<i8> for () {
  fn added(self, rsthis: &mut QChildEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QChildEvent5addedEv()};
    let mut ret = unsafe {_ZNK11QChildEvent5addedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChildEvent {
  pub fn polished<RetType, T: QChildEvent_polished<RetType>>(&mut self, value: T) -> RetType {
    return value.polished(self);
    // return 1;
  }
}

pub trait QChildEvent_polished<RetType> {
  fn polished(self, rsthis: &mut QChildEvent) -> RetType;
}

// proto:  bool QChildEvent::polished();
impl<'a> /*trait*/ QChildEvent_polished<i8> for () {
  fn polished(self, rsthis: &mut QChildEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QChildEvent8polishedEv()};
    let mut ret = unsafe {_ZNK11QChildEvent8polishedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChildEvent {
  pub fn FreeQChildEvent<RetType, T: QChildEvent_FreeQChildEvent<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQChildEvent(self);
    // return 1;
  }
}

pub trait QChildEvent_FreeQChildEvent<RetType> {
  fn FreeQChildEvent(self, rsthis: &mut QChildEvent) -> RetType;
}

// proto:  void QChildEvent::FreeQChildEvent();
impl<'a> /*trait*/ QChildEvent_FreeQChildEvent<()> for () {
  fn FreeQChildEvent(self, rsthis: &mut QChildEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QChildEventD0Ev()};
     unsafe {_ZN11QChildEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QChildEvent {
  pub fn removed<RetType, T: QChildEvent_removed<RetType>>(&mut self, value: T) -> RetType {
    return value.removed(self);
    // return 1;
  }
}

pub trait QChildEvent_removed<RetType> {
  fn removed(self, rsthis: &mut QChildEvent) -> RetType;
}

// proto:  bool QChildEvent::removed();
impl<'a> /*trait*/ QChildEvent_removed<i8> for () {
  fn removed(self, rsthis: &mut QChildEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QChildEvent7removedEv()};
    let mut ret = unsafe {_ZNK11QChildEvent7removedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QChildEvent {
  pub fn child<RetType, T: QChildEvent_child<RetType>>(&mut self, value: T) -> RetType {
    return value.child(self);
    // return 1;
  }
}

pub trait QChildEvent_child<RetType> {
  fn child(self, rsthis: &mut QChildEvent) -> RetType;
}

// proto:  QObject * QChildEvent::child();
impl<'a> /*trait*/ QChildEvent_child<QObject> for () {
  fn child(self, rsthis: &mut QChildEvent) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QChildEvent5childEv()};
    let mut ret = unsafe {_ZNK11QChildEvent5childEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

