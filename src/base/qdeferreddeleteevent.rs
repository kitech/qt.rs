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
  fn _ZNK20QDeferredDeleteEvent9loopLevelEv() -> i32;
  fn _ZN20QDeferredDeleteEventD0Ev() -> i32;
  fn _ZN20QDeferredDeleteEventC1Ev(qthis: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QDeferredDeleteEvent)=24
pub struct QDeferredDeleteEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDeferredDeleteEvent {
  pub fn loopLevel<T: QDeferredDeleteEvent_loopLevel>(&mut self, value: T) -> i32 {
    value.loopLevel(self);
    return 1;
  }
}

pub trait QDeferredDeleteEvent_loopLevel {
  fn loopLevel(self, this: &mut QDeferredDeleteEvent) -> i32;
}

// proto: int QDeferredDeleteEvent::loopLevel();
impl<'a> /*trait*/ QDeferredDeleteEvent_loopLevel for () {
  fn loopLevel(self, this: &mut QDeferredDeleteEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QDeferredDeleteEvent9loopLevelEv()};
    unsafe {_ZNK20QDeferredDeleteEvent9loopLevelEv()};
    return 1;
  }
}

impl /*struct*/ QDeferredDeleteEvent {
  pub fn FreeQDeferredDeleteEvent<T: QDeferredDeleteEvent_FreeQDeferredDeleteEvent>(&mut self, value: T) -> i32 {
    value.FreeQDeferredDeleteEvent(self);
    return 1;
  }
}

pub trait QDeferredDeleteEvent_FreeQDeferredDeleteEvent {
  fn FreeQDeferredDeleteEvent(self, this: &mut QDeferredDeleteEvent) -> i32;
}

// proto: void QDeferredDeleteEvent::FreeQDeferredDeleteEvent();
impl<'a> /*trait*/ QDeferredDeleteEvent_FreeQDeferredDeleteEvent for () {
  fn FreeQDeferredDeleteEvent(self, this: &mut QDeferredDeleteEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QDeferredDeleteEventD0Ev()};
    unsafe {_ZN20QDeferredDeleteEventD0Ev()};
    return 1;
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

