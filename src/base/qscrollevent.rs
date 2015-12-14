// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QPointF QScrollEvent::contentPos();
  fn _ZNK12QScrollEvent10contentPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QScrollEvent::overshootDistance();
  fn _ZNK12QScrollEvent17overshootDistanceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QScrollEvent::FreeQScrollEvent();
  fn _ZN12QScrollEventD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QScrollEvent)=64
pub struct QScrollEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QScrollEvent {
  pub fn contentPos<T: QScrollEvent_contentPos>(&mut self, value: T) -> QPointF {
    return value.contentPos(self);
    // return 1;
  }
}

pub trait QScrollEvent_contentPos {
  fn contentPos(self, rsthis: &mut QScrollEvent) -> QPointF;
}

// proto:  QPointF QScrollEvent::contentPos();
impl<'a> /*trait*/ QScrollEvent_contentPos for () {
  fn contentPos(self, rsthis: &mut QScrollEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 64)};
    // unsafe{_ZNK12QScrollEvent10contentPosEv()};
    let mut ret = unsafe {_ZNK12QScrollEvent10contentPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScrollEvent {
  pub fn overshootDistance<T: QScrollEvent_overshootDistance>(&mut self, value: T) -> QPointF {
    return value.overshootDistance(self);
    // return 1;
  }
}

pub trait QScrollEvent_overshootDistance {
  fn overshootDistance(self, rsthis: &mut QScrollEvent) -> QPointF;
}

// proto:  QPointF QScrollEvent::overshootDistance();
impl<'a> /*trait*/ QScrollEvent_overshootDistance for () {
  fn overshootDistance(self, rsthis: &mut QScrollEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 64)};
    // unsafe{_ZNK12QScrollEvent17overshootDistanceEv()};
    let mut ret = unsafe {_ZNK12QScrollEvent17overshootDistanceEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScrollEvent {
  pub fn FreeQScrollEvent<T: QScrollEvent_FreeQScrollEvent>(&mut self, value: T)  {
     value.FreeQScrollEvent(self);
    // return 1;
  }
}

pub trait QScrollEvent_FreeQScrollEvent {
  fn FreeQScrollEvent(self, rsthis: &mut QScrollEvent) ;
}

// proto:  void QScrollEvent::FreeQScrollEvent();
impl<'a> /*trait*/ QScrollEvent_FreeQScrollEvent for () {
  fn FreeQScrollEvent(self, rsthis: &mut QScrollEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 64)};
    // unsafe{_ZN12QScrollEventD0Ev()};
     unsafe {_ZN12QScrollEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

