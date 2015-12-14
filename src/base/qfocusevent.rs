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
  // proto:  bool QFocusEvent::lostFocus();
  fn _ZNK11QFocusEvent9lostFocusEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QFocusEvent::gotFocus();
  fn _ZNK11QFocusEvent8gotFocusEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFocusEvent::FreeQFocusEvent();
  fn _ZN11QFocusEventD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QFocusEvent)=24
pub struct QFocusEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFocusEvent {
  pub fn lostFocus<T: QFocusEvent_lostFocus>(&mut self, value: T) -> i8 {
    return value.lostFocus(self);
    // return 1;
  }
}

pub trait QFocusEvent_lostFocus {
  fn lostFocus(self, rsthis: &mut QFocusEvent) -> i8;
}

// proto:  bool QFocusEvent::lostFocus();
impl<'a> /*trait*/ QFocusEvent_lostFocus for () {
  fn lostFocus(self, rsthis: &mut QFocusEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFocusEvent9lostFocusEv()};
    let mut ret = unsafe {_ZNK11QFocusEvent9lostFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFocusEvent {
  pub fn gotFocus<T: QFocusEvent_gotFocus>(&mut self, value: T) -> i8 {
    return value.gotFocus(self);
    // return 1;
  }
}

pub trait QFocusEvent_gotFocus {
  fn gotFocus(self, rsthis: &mut QFocusEvent) -> i8;
}

// proto:  bool QFocusEvent::gotFocus();
impl<'a> /*trait*/ QFocusEvent_gotFocus for () {
  fn gotFocus(self, rsthis: &mut QFocusEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFocusEvent8gotFocusEv()};
    let mut ret = unsafe {_ZNK11QFocusEvent8gotFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFocusEvent {
  pub fn FreeQFocusEvent<T: QFocusEvent_FreeQFocusEvent>(&mut self, value: T)  {
     value.FreeQFocusEvent(self);
    // return 1;
  }
}

pub trait QFocusEvent_FreeQFocusEvent {
  fn FreeQFocusEvent(self, rsthis: &mut QFocusEvent) ;
}

// proto:  void QFocusEvent::FreeQFocusEvent();
impl<'a> /*trait*/ QFocusEvent_FreeQFocusEvent for () {
  fn FreeQFocusEvent(self, rsthis: &mut QFocusEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFocusEventD0Ev()};
     unsafe {_ZN11QFocusEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

