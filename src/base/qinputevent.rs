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
  // proto:  void QInputEvent::setTimestamp(ulong atimestamp);
  fn _ZN11QInputEvent12setTimestampEm(qthis: *mut c_void, arg0: c_ulong) ;
  // proto:  unsigned long QInputEvent::timestamp();
  fn _ZNK11QInputEvent9timestampEv(qthis: *mut c_void) -> c_ulong;
  // proto:  void QInputEvent::FreeQInputEvent();
  fn _ZN11QInputEventD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QInputEvent)=1
pub struct QInputEvent {
  pub qclsinst: *mut c_void,
}

// proto:  void QInputEvent::setTimestamp(ulong atimestamp);
impl /*struct*/ QInputEvent {
  pub fn setTimestamp<RetType, T: QInputEvent_setTimestamp<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setTimestamp(self);
    // return 1;
  }
}

pub trait QInputEvent_setTimestamp<RetType> {
  fn setTimestamp(self , rsthis: &mut QInputEvent) -> RetType;
}

// proto:  void QInputEvent::setTimestamp(ulong atimestamp);
impl<'a> /*trait*/ QInputEvent_setTimestamp<()> for (u32) {
  fn setTimestamp(self , rsthis: &mut QInputEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QInputEvent12setTimestampEm()};
    let arg0 = self  as c_ulong;
     unsafe {_ZN11QInputEvent12setTimestampEm(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  unsigned long QInputEvent::timestamp();
impl /*struct*/ QInputEvent {
  pub fn timestamp<RetType, T: QInputEvent_timestamp<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.timestamp(self);
    // return 1;
  }
}

pub trait QInputEvent_timestamp<RetType> {
  fn timestamp(self , rsthis: &mut QInputEvent) -> RetType;
}

// proto:  unsigned long QInputEvent::timestamp();
impl<'a> /*trait*/ QInputEvent_timestamp<u64> for () {
  fn timestamp(self , rsthis: &mut QInputEvent) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QInputEvent9timestampEv()};
    let mut ret = unsafe {_ZNK11QInputEvent9timestampEv(rsthis.qclsinst)};
    return ret as u64;
    // return 1;
  }
}

// proto:  void QInputEvent::FreeQInputEvent();
impl /*struct*/ QInputEvent {
  pub fn FreeQInputEvent<RetType, T: QInputEvent_FreeQInputEvent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQInputEvent(self);
    // return 1;
  }
}

pub trait QInputEvent_FreeQInputEvent<RetType> {
  fn FreeQInputEvent(self , rsthis: &mut QInputEvent) -> RetType;
}

// proto:  void QInputEvent::FreeQInputEvent();
impl<'a> /*trait*/ QInputEvent_FreeQInputEvent<()> for () {
  fn FreeQInputEvent(self , rsthis: &mut QInputEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QInputEventD0Ev()};
     unsafe {_ZN11QInputEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

