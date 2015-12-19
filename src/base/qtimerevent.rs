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
  // proto:  void QTimerEvent::NewQTimerEvent(int timerId);
  fn _ZN11QTimerEventC1Ei(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTimerEvent::FreeQTimerEvent();
  fn _ZN11QTimerEventD0Ev(qthis: *mut c_void) ;
  // proto:  int QTimerEvent::timerId();
  fn _ZNK11QTimerEvent7timerIdEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QTimerEvent)=24
pub struct QTimerEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTimerEvent {
  pub fn NewQTimerEvent<T: QTimerEvent_NewQTimerEvent>(value: T) -> QTimerEvent {
    let rsthis = value.NewQTimerEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QTimerEvent_NewQTimerEvent {
  fn NewQTimerEvent(self) -> QTimerEvent;
}

// proto: void QTimerEvent::NewQTimerEvent(int timerId);
impl<'a> /*trait*/ QTimerEvent_NewQTimerEvent for (i32) {
  fn NewQTimerEvent(self) -> QTimerEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTimerEventC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QTimerEventC1Ei(qthis, arg0)};
    let rsthis = QTimerEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QTimerEvent::FreeQTimerEvent();
impl /*struct*/ QTimerEvent {
  pub fn FreeQTimerEvent<RetType, T: QTimerEvent_FreeQTimerEvent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQTimerEvent(self);
    // return 1;
  }
}

pub trait QTimerEvent_FreeQTimerEvent<RetType> {
  fn FreeQTimerEvent(self , rsthis: &mut QTimerEvent) -> RetType;
}

// proto:  void QTimerEvent::FreeQTimerEvent();
impl<'a> /*trait*/ QTimerEvent_FreeQTimerEvent<()> for () {
  fn FreeQTimerEvent(self , rsthis: &mut QTimerEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTimerEventD0Ev()};
     unsafe {_ZN11QTimerEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  int QTimerEvent::timerId();
impl /*struct*/ QTimerEvent {
  pub fn timerId<RetType, T: QTimerEvent_timerId<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.timerId(self);
    // return 1;
  }
}

pub trait QTimerEvent_timerId<RetType> {
  fn timerId(self , rsthis: &mut QTimerEvent) -> RetType;
}

// proto:  int QTimerEvent::timerId();
impl<'a> /*trait*/ QTimerEvent_timerId<i32> for () {
  fn timerId(self , rsthis: &mut QTimerEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTimerEvent7timerIdEv()};
    let mut ret = unsafe {_ZNK11QTimerEvent7timerIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

