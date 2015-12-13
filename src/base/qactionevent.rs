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
  // proto: QAction * QActionEvent::before();
  fn _ZNK12QActionEvent6beforeEv() -> i32;
  // proto: QAction * QActionEvent::action();
  fn _ZNK12QActionEvent6actionEv() -> i32;
  // proto: void QActionEvent::FreeQActionEvent();
  fn _ZN12QActionEventD0Ev() -> i32;
}

// body block begin
// class sizeof(QActionEvent)=40
pub struct QActionEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QActionEvent {
  pub fn before<T: QActionEvent_before>(&mut self, value: T) -> i32 {
    value.before(self);
    return 1;
  }
}

pub trait QActionEvent_before {
  fn before(self, this: &mut QActionEvent) -> i32;
}

// proto: QAction * QActionEvent::before();
impl<'a> /*trait*/ QActionEvent_before for () {
  fn before(self, this: &mut QActionEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK12QActionEvent6beforeEv()};
    unsafe {_ZNK12QActionEvent6beforeEv()};
    return 1;
  }
}

impl /*struct*/ QActionEvent {
  pub fn action<T: QActionEvent_action>(&mut self, value: T) -> i32 {
    value.action(self);
    return 1;
  }
}

pub trait QActionEvent_action {
  fn action(self, this: &mut QActionEvent) -> i32;
}

// proto: QAction * QActionEvent::action();
impl<'a> /*trait*/ QActionEvent_action for () {
  fn action(self, this: &mut QActionEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK12QActionEvent6actionEv()};
    unsafe {_ZNK12QActionEvent6actionEv()};
    return 1;
  }
}

impl /*struct*/ QActionEvent {
  pub fn FreeQActionEvent<T: QActionEvent_FreeQActionEvent>(&mut self, value: T) -> i32 {
    value.FreeQActionEvent(self);
    return 1;
  }
}

pub trait QActionEvent_FreeQActionEvent {
  fn FreeQActionEvent(self, this: &mut QActionEvent) -> i32;
}

// proto: void QActionEvent::FreeQActionEvent();
impl<'a> /*trait*/ QActionEvent_FreeQActionEvent for () {
  fn FreeQActionEvent(self, this: &mut QActionEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN12QActionEventD0Ev()};
    unsafe {_ZN12QActionEventD0Ev()};
    return 1;
  }
}

