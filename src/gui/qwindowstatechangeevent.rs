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
  // proto:  bool QWindowStateChangeEvent::isOverride();
  fn _ZNK23QWindowStateChangeEvent10isOverrideEv(qthis: *mut c_void) -> c_char;
  // proto:  void QWindowStateChangeEvent::~QWindowStateChangeEvent();
  fn _ZN23QWindowStateChangeEventD0Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QWindowStateChangeEvent)=1
pub struct QWindowStateChangeEvent {
  pub qclsinst: *mut c_void,
}

  // proto:  bool QWindowStateChangeEvent::isOverride();
impl /*struct*/ QWindowStateChangeEvent {
  pub fn isOverride<RetType, T: QWindowStateChangeEvent_isOverride<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isOverride(self);
    // return 1;
  }
}

pub trait QWindowStateChangeEvent_isOverride<RetType> {
  fn isOverride(self , rsthis: &mut QWindowStateChangeEvent) -> RetType;
}

  // proto:  bool QWindowStateChangeEvent::isOverride();
impl<'a> /*trait*/ QWindowStateChangeEvent_isOverride<i8> for () {
  fn isOverride(self , rsthis: &mut QWindowStateChangeEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QWindowStateChangeEvent10isOverrideEv()};
    let mut ret = unsafe {_ZNK23QWindowStateChangeEvent10isOverrideEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QWindowStateChangeEvent::~QWindowStateChangeEvent();
impl /*struct*/ QWindowStateChangeEvent {
  pub fn FreeQWindowStateChangeEvent<RetType, T: QWindowStateChangeEvent_FreeQWindowStateChangeEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQWindowStateChangeEvent(self);
    // return 1;
  }
}

pub trait QWindowStateChangeEvent_FreeQWindowStateChangeEvent<RetType> {
  fn FreeQWindowStateChangeEvent(self , rsthis: &mut QWindowStateChangeEvent) -> RetType;
}

  // proto:  void QWindowStateChangeEvent::~QWindowStateChangeEvent();
impl<'a> /*trait*/ QWindowStateChangeEvent_FreeQWindowStateChangeEvent<()> for () {
  fn FreeQWindowStateChangeEvent(self , rsthis: &mut QWindowStateChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QWindowStateChangeEventD0Ev()};
     unsafe {_ZN23QWindowStateChangeEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

