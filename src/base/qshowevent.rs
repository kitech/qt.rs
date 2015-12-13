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
  // proto: void QShowEvent::FreeQShowEvent();
  fn _ZN10QShowEventD0Ev() -> i32;
  // proto: void QShowEvent::NewQShowEvent();
  fn _ZN10QShowEventC1Ev(qthis: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QShowEvent)=24
pub struct QShowEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QShowEvent {
  pub fn FreeQShowEvent<T: QShowEvent_FreeQShowEvent>(&mut self, value: T) -> i32 {
    value.FreeQShowEvent(self);
    return 1;
  }
}

pub trait QShowEvent_FreeQShowEvent {
  fn FreeQShowEvent(self, this: &mut QShowEvent) -> i32;
}

// proto: void QShowEvent::FreeQShowEvent();
impl<'a> /*trait*/ QShowEvent_FreeQShowEvent for () {
  fn FreeQShowEvent(self, this: &mut QShowEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QShowEventD0Ev()};
    unsafe {_ZN10QShowEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QShowEvent {
  pub fn NewQShowEvent<T: QShowEvent_NewQShowEvent>(value: T) -> QShowEvent {
    let rsthis = value.NewQShowEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QShowEvent_NewQShowEvent {
  fn NewQShowEvent(self) -> QShowEvent;
}

// proto: void QShowEvent::NewQShowEvent();
impl<'a> /*trait*/ QShowEvent_NewQShowEvent for () {
  fn NewQShowEvent(self) -> QShowEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QShowEventC1Ev()};
    unsafe {_ZN10QShowEventC1Ev(qthis)};
    let rsthis = QShowEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

