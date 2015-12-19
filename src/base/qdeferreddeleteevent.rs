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
  // proto:  int QDeferredDeleteEvent::loopLevel();
  fn _ZNK20QDeferredDeleteEvent9loopLevelEv(qthis: *mut c_void) -> c_int;
  // proto:  void QDeferredDeleteEvent::FreeQDeferredDeleteEvent();
  fn _ZN20QDeferredDeleteEventD0Ev(qthis: *mut c_void) ;
  // proto:  void QDeferredDeleteEvent::NewQDeferredDeleteEvent();
  fn _ZN20QDeferredDeleteEventC1Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QDeferredDeleteEvent)=24
pub struct QDeferredDeleteEvent {
  pub qclsinst: *mut c_void,
}

// proto:  int QDeferredDeleteEvent::loopLevel();
impl /*struct*/ QDeferredDeleteEvent {
  pub fn loopLevel<RetType, T: QDeferredDeleteEvent_loopLevel<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.loopLevel(self);
    // return 1;
  }
}

pub trait QDeferredDeleteEvent_loopLevel<RetType> {
  fn loopLevel(self , rsthis: &mut QDeferredDeleteEvent) -> RetType;
}

// proto:  int QDeferredDeleteEvent::loopLevel();
impl<'a> /*trait*/ QDeferredDeleteEvent_loopLevel<i32> for () {
  fn loopLevel(self , rsthis: &mut QDeferredDeleteEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QDeferredDeleteEvent9loopLevelEv()};
    let mut ret = unsafe {_ZNK20QDeferredDeleteEvent9loopLevelEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QDeferredDeleteEvent::FreeQDeferredDeleteEvent();
impl /*struct*/ QDeferredDeleteEvent {
  pub fn FreeQDeferredDeleteEvent<RetType, T: QDeferredDeleteEvent_FreeQDeferredDeleteEvent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQDeferredDeleteEvent(self);
    // return 1;
  }
}

pub trait QDeferredDeleteEvent_FreeQDeferredDeleteEvent<RetType> {
  fn FreeQDeferredDeleteEvent(self , rsthis: &mut QDeferredDeleteEvent) -> RetType;
}

// proto:  void QDeferredDeleteEvent::FreeQDeferredDeleteEvent();
impl<'a> /*trait*/ QDeferredDeleteEvent_FreeQDeferredDeleteEvent<()> for () {
  fn FreeQDeferredDeleteEvent(self , rsthis: &mut QDeferredDeleteEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QDeferredDeleteEventD0Ev()};
     unsafe {_ZN20QDeferredDeleteEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDeferredDeleteEvent {
  pub fn NewQDeferredDeleteEvent<T: QDeferredDeleteEvent_NewQDeferredDeleteEvent>(value: T) -> QDeferredDeleteEvent {
    let rsthis = value.NewQDeferredDeleteEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QDeferredDeleteEvent_NewQDeferredDeleteEvent {
  fn NewQDeferredDeleteEvent(self) -> QDeferredDeleteEvent;
}

// proto: void QDeferredDeleteEvent::NewQDeferredDeleteEvent();
impl<'a> /*trait*/ QDeferredDeleteEvent_NewQDeferredDeleteEvent for () {
  fn NewQDeferredDeleteEvent(self) -> QDeferredDeleteEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QDeferredDeleteEventC1Ev()};
    unsafe {_ZN20QDeferredDeleteEventC1Ev(qthis)};
    let rsthis = QDeferredDeleteEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

