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

impl /*struct*/ QInputEvent {
  pub fn setTimestamp<T: QInputEvent_setTimestamp>(&mut self, value: T)  {
     value.setTimestamp(self);
    // return 1;
  }
}

pub trait QInputEvent_setTimestamp {
  fn setTimestamp(self, rsthis: &mut QInputEvent) ;
}

// proto:  void QInputEvent::setTimestamp(ulong atimestamp);
impl<'a> /*trait*/ QInputEvent_setTimestamp for (u32) {
  fn setTimestamp(self, rsthis: &mut QInputEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QInputEvent12setTimestampEm()};
    let arg0 = self  as c_ulong;
     unsafe {_ZN11QInputEvent12setTimestampEm(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QInputEvent {
  pub fn timestamp<T: QInputEvent_timestamp>(&mut self, value: T) -> u32 {
    return value.timestamp(self);
    // return 1;
  }
}

pub trait QInputEvent_timestamp {
  fn timestamp(self, rsthis: &mut QInputEvent) -> u32;
}

// proto:  unsigned long QInputEvent::timestamp();
impl<'a> /*trait*/ QInputEvent_timestamp for () {
  fn timestamp(self, rsthis: &mut QInputEvent) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QInputEvent9timestampEv()};
    let mut ret = unsafe {_ZNK11QInputEvent9timestampEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QInputEvent {
  pub fn FreeQInputEvent<T: QInputEvent_FreeQInputEvent>(&mut self, value: T)  {
     value.FreeQInputEvent(self);
    // return 1;
  }
}

pub trait QInputEvent_FreeQInputEvent {
  fn FreeQInputEvent(self, rsthis: &mut QInputEvent) ;
}

// proto:  void QInputEvent::FreeQInputEvent();
impl<'a> /*trait*/ QInputEvent_FreeQInputEvent for () {
  fn FreeQInputEvent(self, rsthis: &mut QInputEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QInputEventD0Ev()};
     unsafe {_ZN11QInputEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

