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
  // proto: bool QFocusEvent::lostFocus();
  fn _ZNK11QFocusEvent9lostFocusEv() -> i32;
  // proto: bool QFocusEvent::gotFocus();
  fn _ZNK11QFocusEvent8gotFocusEv() -> i32;
  // proto: void QFocusEvent::FreeQFocusEvent();
  fn _ZN11QFocusEventD0Ev() -> i32;
}

// body block begin
// class sizeof(QFocusEvent)=24
pub struct QFocusEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFocusEvent {
  pub fn lostFocus<T: QFocusEvent_lostFocus>(&mut self, value: T) -> i32 {
    value.lostFocus(self);
    return 1;
  }
}

pub trait QFocusEvent_lostFocus {
  fn lostFocus(self, this: &mut QFocusEvent) -> i32;
}

// proto: bool QFocusEvent::lostFocus();
impl<'a> /*trait*/ QFocusEvent_lostFocus for () {
  fn lostFocus(self, this: &mut QFocusEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFocusEvent9lostFocusEv()};
    unsafe {_ZNK11QFocusEvent9lostFocusEv()};
    return 1;
  }
}

impl /*struct*/ QFocusEvent {
  pub fn gotFocus<T: QFocusEvent_gotFocus>(&mut self, value: T) -> i32 {
    value.gotFocus(self);
    return 1;
  }
}

pub trait QFocusEvent_gotFocus {
  fn gotFocus(self, this: &mut QFocusEvent) -> i32;
}

// proto: bool QFocusEvent::gotFocus();
impl<'a> /*trait*/ QFocusEvent_gotFocus for () {
  fn gotFocus(self, this: &mut QFocusEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFocusEvent8gotFocusEv()};
    unsafe {_ZNK11QFocusEvent8gotFocusEv()};
    return 1;
  }
}

impl /*struct*/ QFocusEvent {
  pub fn FreeQFocusEvent<T: QFocusEvent_FreeQFocusEvent>(&mut self, value: T) -> i32 {
    value.FreeQFocusEvent(self);
    return 1;
  }
}

pub trait QFocusEvent_FreeQFocusEvent {
  fn FreeQFocusEvent(self, this: &mut QFocusEvent) -> i32;
}

// proto: void QFocusEvent::FreeQFocusEvent();
impl<'a> /*trait*/ QFocusEvent_FreeQFocusEvent for () {
  fn FreeQFocusEvent(self, this: &mut QFocusEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFocusEventD0Ev()};
    unsafe {_ZN11QFocusEventD0Ev()};
    return 1;
  }
}

