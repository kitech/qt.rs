// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QPointF QScrollEvent::contentPos();
  fn _ZNK12QScrollEvent10contentPosEv() -> i32;
  // proto: QPointF QScrollEvent::overshootDistance();
  fn _ZNK12QScrollEvent17overshootDistanceEv() -> i32;
  // proto: void QScrollEvent::FreeQScrollEvent();
  fn _ZN12QScrollEventD0Ev() -> i32;
}

// body block begin
// class sizeof(QScrollEvent)=64
pub struct QScrollEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QScrollEvent {
  pub fn contentPos<T: QScrollEvent_contentPos>(&mut self, value: T) -> i32 {
    value.contentPos(self);
    return 1;
  }
}

pub trait QScrollEvent_contentPos {
  fn contentPos(self, this: &mut QScrollEvent) -> i32;
}

// proto: QPointF QScrollEvent::contentPos();
impl<'a> /*trait*/ QScrollEvent_contentPos for () {
  fn contentPos(self, this: &mut QScrollEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 64)};
    // unsafe{_ZNK12QScrollEvent10contentPosEv()};
    unsafe {_ZNK12QScrollEvent10contentPosEv()};
    return 1;
  }
}

impl /*struct*/ QScrollEvent {
  pub fn overshootDistance<T: QScrollEvent_overshootDistance>(&mut self, value: T) -> i32 {
    value.overshootDistance(self);
    return 1;
  }
}

pub trait QScrollEvent_overshootDistance {
  fn overshootDistance(self, this: &mut QScrollEvent) -> i32;
}

// proto: QPointF QScrollEvent::overshootDistance();
impl<'a> /*trait*/ QScrollEvent_overshootDistance for () {
  fn overshootDistance(self, this: &mut QScrollEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 64)};
    // unsafe{_ZNK12QScrollEvent17overshootDistanceEv()};
    unsafe {_ZNK12QScrollEvent17overshootDistanceEv()};
    return 1;
  }
}

impl /*struct*/ QScrollEvent {
  pub fn FreeQScrollEvent<T: QScrollEvent_FreeQScrollEvent>(&mut self, value: T) -> i32 {
    value.FreeQScrollEvent(self);
    return 1;
  }
}

pub trait QScrollEvent_FreeQScrollEvent {
  fn FreeQScrollEvent(self, this: &mut QScrollEvent) -> i32;
}

// proto: void QScrollEvent::FreeQScrollEvent();
impl<'a> /*trait*/ QScrollEvent_FreeQScrollEvent for () {
  fn FreeQScrollEvent(self, this: &mut QScrollEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 64)};
    // unsafe{_ZN12QScrollEventD0Ev()};
    unsafe {_ZN12QScrollEventD0Ev()};
    return 1;
  }
}

