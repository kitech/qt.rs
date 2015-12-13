// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QDragMoveEvent::accept(const QRect & r);
  fn _ZN14QDragMoveEvent6acceptERK5QRect(arg0: *const c_void) -> i32;
  // proto: QRect QDragMoveEvent::answerRect();
  fn _ZNK14QDragMoveEvent10answerRectEv() -> i32;
  // proto: void QDragMoveEvent::ignore(const QRect & r);
  fn _ZN14QDragMoveEvent6ignoreERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QDragMoveEvent::ignore();
  fn _ZN14QDragMoveEvent6ignoreEv() -> i32;
  // proto: void QDragMoveEvent::FreeQDragMoveEvent();
  fn _ZN14QDragMoveEventD0Ev() -> i32;
  // proto: void QDragMoveEvent::accept();
  fn _ZN14QDragMoveEvent6acceptEv() -> i32;
}

// body block begin
// class sizeof(QDragMoveEvent)=1
pub struct QDragMoveEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDragMoveEvent {
  pub fn accept<T: QDragMoveEvent_accept>(&mut self, value: T) -> i32 {
    value.accept(self);
    return 1;
  }
}

pub trait QDragMoveEvent_accept {
  fn accept(self, this: &mut QDragMoveEvent) -> i32;
}

// proto: void QDragMoveEvent::accept(const QRect & r);
impl<'a> /*trait*/ QDragMoveEvent_accept for (&'a  QRect) {
  fn accept(self, this: &mut QDragMoveEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEvent6acceptERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QDragMoveEvent6acceptERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QDragMoveEvent {
  pub fn answerRect<T: QDragMoveEvent_answerRect>(&mut self, value: T) -> i32 {
    value.answerRect(self);
    return 1;
  }
}

pub trait QDragMoveEvent_answerRect {
  fn answerRect(self, this: &mut QDragMoveEvent) -> i32;
}

// proto: QRect QDragMoveEvent::answerRect();
impl<'a> /*trait*/ QDragMoveEvent_answerRect for () {
  fn answerRect(self, this: &mut QDragMoveEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDragMoveEvent10answerRectEv()};
    unsafe {_ZNK14QDragMoveEvent10answerRectEv()};
    return 1;
  }
}

impl /*struct*/ QDragMoveEvent {
  pub fn ignore<T: QDragMoveEvent_ignore>(&mut self, value: T) -> i32 {
    value.ignore(self);
    return 1;
  }
}

pub trait QDragMoveEvent_ignore {
  fn ignore(self, this: &mut QDragMoveEvent) -> i32;
}

// proto: void QDragMoveEvent::ignore(const QRect & r);
impl<'a> /*trait*/ QDragMoveEvent_ignore for (&'a  QRect) {
  fn ignore(self, this: &mut QDragMoveEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEvent6ignoreERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QDragMoveEvent6ignoreERK5QRect(arg0)};
    return 1;
  }
}

// proto: void QDragMoveEvent::ignore();
impl<'a> /*trait*/ QDragMoveEvent_ignore for () {
  fn ignore(self, this: &mut QDragMoveEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEvent6ignoreEv()};
    unsafe {_ZN14QDragMoveEvent6ignoreEv()};
    return 1;
  }
}

impl /*struct*/ QDragMoveEvent {
  pub fn FreeQDragMoveEvent<T: QDragMoveEvent_FreeQDragMoveEvent>(&mut self, value: T) -> i32 {
    value.FreeQDragMoveEvent(self);
    return 1;
  }
}

pub trait QDragMoveEvent_FreeQDragMoveEvent {
  fn FreeQDragMoveEvent(self, this: &mut QDragMoveEvent) -> i32;
}

// proto: void QDragMoveEvent::FreeQDragMoveEvent();
impl<'a> /*trait*/ QDragMoveEvent_FreeQDragMoveEvent for () {
  fn FreeQDragMoveEvent(self, this: &mut QDragMoveEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEventD0Ev()};
    unsafe {_ZN14QDragMoveEventD0Ev()};
    return 1;
  }
}

// proto: void QDragMoveEvent::accept();
impl<'a> /*trait*/ QDragMoveEvent_accept for () {
  fn accept(self, this: &mut QDragMoveEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEvent6acceptEv()};
    unsafe {_ZN14QDragMoveEvent6acceptEv()};
    return 1;
  }
}

