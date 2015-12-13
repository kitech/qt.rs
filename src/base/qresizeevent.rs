// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: const QSize & QResizeEvent::oldSize();
  fn _ZNK12QResizeEvent7oldSizeEv() -> i32;
  // proto: const QSize & QResizeEvent::size();
  fn _ZNK12QResizeEvent4sizeEv() -> i32;
  // proto: void QResizeEvent::FreeQResizeEvent();
  fn _ZN12QResizeEventD0Ev() -> i32;
  // proto: void QResizeEvent::NewQResizeEvent(const QSize & size, const QSize & oldSize);
  fn _ZN12QResizeEventC1ERK5QSizeS2_(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
}

// body block begin
// class sizeof(QResizeEvent)=40
pub struct QResizeEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QResizeEvent {
  pub fn oldSize<T: QResizeEvent_oldSize>(&mut self, value: T) -> i32 {
    value.oldSize(self);
    return 1;
  }
}

pub trait QResizeEvent_oldSize {
  fn oldSize(self, this: &mut QResizeEvent) -> i32;
}

// proto: const QSize & QResizeEvent::oldSize();
impl<'a> /*trait*/ QResizeEvent_oldSize for () {
  fn oldSize(self, this: &mut QResizeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK12QResizeEvent7oldSizeEv()};
    unsafe {_ZNK12QResizeEvent7oldSizeEv()};
    return 1;
  }
}

impl /*struct*/ QResizeEvent {
  pub fn size<T: QResizeEvent_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QResizeEvent_size {
  fn size(self, this: &mut QResizeEvent) -> i32;
}

// proto: const QSize & QResizeEvent::size();
impl<'a> /*trait*/ QResizeEvent_size for () {
  fn size(self, this: &mut QResizeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK12QResizeEvent4sizeEv()};
    unsafe {_ZNK12QResizeEvent4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QResizeEvent {
  pub fn FreeQResizeEvent<T: QResizeEvent_FreeQResizeEvent>(&mut self, value: T) -> i32 {
    value.FreeQResizeEvent(self);
    return 1;
  }
}

pub trait QResizeEvent_FreeQResizeEvent {
  fn FreeQResizeEvent(self, this: &mut QResizeEvent) -> i32;
}

// proto: void QResizeEvent::FreeQResizeEvent();
impl<'a> /*trait*/ QResizeEvent_FreeQResizeEvent for () {
  fn FreeQResizeEvent(self, this: &mut QResizeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN12QResizeEventD0Ev()};
    unsafe {_ZN12QResizeEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QResizeEvent {
  pub fn NewQResizeEvent<T: QResizeEvent_NewQResizeEvent>(value: T) -> QResizeEvent {
    let rsthis = value.NewQResizeEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QResizeEvent_NewQResizeEvent {
  fn NewQResizeEvent(self) -> QResizeEvent;
}

// proto: void QResizeEvent::NewQResizeEvent(const QSize & size, const QSize & oldSize);
impl<'a> /*trait*/ QResizeEvent_NewQResizeEvent for (&'a  QSize, &'a  QSize) {
  fn NewQResizeEvent(self) -> QResizeEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN12QResizeEventC1ERK5QSizeS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN12QResizeEventC1ERK5QSizeS2_(qthis, arg0, arg1)};
    let rsthis = QResizeEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

