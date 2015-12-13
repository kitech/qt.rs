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
  // proto: QScreen * QScreenOrientationChangeEvent::screen();
  fn _ZNK29QScreenOrientationChangeEvent6screenEv() -> i32;
  // proto: void QScreenOrientationChangeEvent::FreeQScreenOrientationChangeEvent();
  fn _ZN29QScreenOrientationChangeEventD0Ev() -> i32;
}

// body block begin
// class sizeof(QScreenOrientationChangeEvent)=40
pub struct QScreenOrientationChangeEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QScreenOrientationChangeEvent {
  pub fn screen<T: QScreenOrientationChangeEvent_screen>(&mut self, value: T) -> i32 {
    value.screen(self);
    return 1;
  }
}

pub trait QScreenOrientationChangeEvent_screen {
  fn screen(self, this: &mut QScreenOrientationChangeEvent) -> i32;
}

// proto: QScreen * QScreenOrientationChangeEvent::screen();
impl<'a> /*trait*/ QScreenOrientationChangeEvent_screen for () {
  fn screen(self, this: &mut QScreenOrientationChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK29QScreenOrientationChangeEvent6screenEv()};
    unsafe {_ZNK29QScreenOrientationChangeEvent6screenEv()};
    return 1;
  }
}

impl /*struct*/ QScreenOrientationChangeEvent {
  pub fn FreeQScreenOrientationChangeEvent<T: QScreenOrientationChangeEvent_FreeQScreenOrientationChangeEvent>(&mut self, value: T) -> i32 {
    value.FreeQScreenOrientationChangeEvent(self);
    return 1;
  }
}

pub trait QScreenOrientationChangeEvent_FreeQScreenOrientationChangeEvent {
  fn FreeQScreenOrientationChangeEvent(self, this: &mut QScreenOrientationChangeEvent) -> i32;
}

// proto: void QScreenOrientationChangeEvent::FreeQScreenOrientationChangeEvent();
impl<'a> /*trait*/ QScreenOrientationChangeEvent_FreeQScreenOrientationChangeEvent for () {
  fn FreeQScreenOrientationChangeEvent(self, this: &mut QScreenOrientationChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN29QScreenOrientationChangeEventD0Ev()};
    unsafe {_ZN29QScreenOrientationChangeEventD0Ev()};
    return 1;
  }
}

