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
  // proto: void QIconDragEvent::FreeQIconDragEvent();
  fn _ZN14QIconDragEventD0Ev() -> i32;
  // proto: void QIconDragEvent::NewQIconDragEvent();
  fn _ZN14QIconDragEventC1Ev(qthis: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QIconDragEvent)=24
pub struct QIconDragEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QIconDragEvent {
  pub fn FreeQIconDragEvent<T: QIconDragEvent_FreeQIconDragEvent>(&mut self, value: T) -> i32 {
    value.FreeQIconDragEvent(self);
    return 1;
  }
}

pub trait QIconDragEvent_FreeQIconDragEvent {
  fn FreeQIconDragEvent(self, this: &mut QIconDragEvent) -> i32;
}

// proto: void QIconDragEvent::FreeQIconDragEvent();
impl<'a> /*trait*/ QIconDragEvent_FreeQIconDragEvent for () {
  fn FreeQIconDragEvent(self, this: &mut QIconDragEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QIconDragEventD0Ev()};
    unsafe {_ZN14QIconDragEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QIconDragEvent {
  pub fn NewQIconDragEvent<T: QIconDragEvent_NewQIconDragEvent>(value: T) -> QIconDragEvent {
    let rsthis = value.NewQIconDragEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QIconDragEvent_NewQIconDragEvent {
  fn NewQIconDragEvent(self) -> QIconDragEvent;
}

// proto: void QIconDragEvent::NewQIconDragEvent();
impl<'a> /*trait*/ QIconDragEvent_NewQIconDragEvent for () {
  fn NewQIconDragEvent(self) -> QIconDragEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QIconDragEventC1Ev()};
    unsafe {_ZN14QIconDragEventC1Ev(qthis)};
    let rsthis = QIconDragEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

