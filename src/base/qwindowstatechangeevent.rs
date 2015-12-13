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
  // proto: bool QWindowStateChangeEvent::isOverride();
  fn _ZNK23QWindowStateChangeEvent10isOverrideEv() -> i32;
  // proto: void QWindowStateChangeEvent::FreeQWindowStateChangeEvent();
  fn _ZN23QWindowStateChangeEventD0Ev() -> i32;
}

// body block begin
// class sizeof(QWindowStateChangeEvent)=1
pub struct QWindowStateChangeEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWindowStateChangeEvent {
  pub fn isOverride<T: QWindowStateChangeEvent_isOverride>(&mut self, value: T) -> i32 {
    value.isOverride(self);
    return 1;
  }
}

pub trait QWindowStateChangeEvent_isOverride {
  fn isOverride(self, this: &mut QWindowStateChangeEvent) -> i32;
}

// proto: bool QWindowStateChangeEvent::isOverride();
impl<'a> /*trait*/ QWindowStateChangeEvent_isOverride for () {
  fn isOverride(self, this: &mut QWindowStateChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QWindowStateChangeEvent10isOverrideEv()};
    unsafe {_ZNK23QWindowStateChangeEvent10isOverrideEv()};
    return 1;
  }
}

impl /*struct*/ QWindowStateChangeEvent {
  pub fn FreeQWindowStateChangeEvent<T: QWindowStateChangeEvent_FreeQWindowStateChangeEvent>(&mut self, value: T) -> i32 {
    value.FreeQWindowStateChangeEvent(self);
    return 1;
  }
}

pub trait QWindowStateChangeEvent_FreeQWindowStateChangeEvent {
  fn FreeQWindowStateChangeEvent(self, this: &mut QWindowStateChangeEvent) -> i32;
}

// proto: void QWindowStateChangeEvent::FreeQWindowStateChangeEvent();
impl<'a> /*trait*/ QWindowStateChangeEvent_FreeQWindowStateChangeEvent for () {
  fn FreeQWindowStateChangeEvent(self, this: &mut QWindowStateChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QWindowStateChangeEventD0Ev()};
    unsafe {_ZN23QWindowStateChangeEventD0Ev()};
    return 1;
  }
}

