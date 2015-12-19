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

// proto:  bool QFocusEvent::lostFocus();
impl /*struct*/ QFocusEvent {
  pub fn lostFocus<RetType, T: QFocusEvent_lostFocus<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.lostFocus(self);
    // return 1;
  }
}

pub trait QFocusEvent_lostFocus<RetType> {
  fn lostFocus(self , rsthis: &mut QFocusEvent) -> RetType;
}

// proto:  bool QFocusEvent::lostFocus();
impl<'a> /*trait*/ QFocusEvent_lostFocus<i8> for () {
  fn lostFocus(self , rsthis: &mut QFocusEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFocusEvent9lostFocusEv()};
    let mut ret = unsafe {_ZNK11QFocusEvent9lostFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QFocusEvent::gotFocus();
impl /*struct*/ QFocusEvent {
  pub fn gotFocus<RetType, T: QFocusEvent_gotFocus<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.gotFocus(self);
    // return 1;
  }
}

pub trait QFocusEvent_gotFocus<RetType> {
  fn gotFocus(self , rsthis: &mut QFocusEvent) -> RetType;
}

// proto:  bool QFocusEvent::gotFocus();
impl<'a> /*trait*/ QFocusEvent_gotFocus<i8> for () {
  fn gotFocus(self , rsthis: &mut QFocusEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFocusEvent8gotFocusEv()};
    let mut ret = unsafe {_ZNK11QFocusEvent8gotFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QFocusEvent::FreeQFocusEvent();
impl /*struct*/ QFocusEvent {
  pub fn FreeQFocusEvent<RetType, T: QFocusEvent_FreeQFocusEvent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQFocusEvent(self);
    // return 1;
  }
}

pub trait QFocusEvent_FreeQFocusEvent<RetType> {
  fn FreeQFocusEvent(self , rsthis: &mut QFocusEvent) -> RetType;
}

// proto:  void QFocusEvent::FreeQFocusEvent();
impl<'a> /*trait*/ QFocusEvent_FreeQFocusEvent<()> for () {
  fn FreeQFocusEvent(self , rsthis: &mut QFocusEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFocusEventD0Ev()};
     unsafe {_ZN11QFocusEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

