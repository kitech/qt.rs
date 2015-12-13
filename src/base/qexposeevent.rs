// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qregion::QRegion;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QExposeEvent::NewQExposeEvent(const QRegion & rgn);
  fn _ZN12QExposeEventC1ERK7QRegion(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: const QRegion & QExposeEvent::region();
  fn _ZNK12QExposeEvent6regionEv() -> i32;
  // proto: void QExposeEvent::FreeQExposeEvent();
  fn _ZN12QExposeEventD0Ev() -> i32;
}

// body block begin
// class sizeof(QExposeEvent)=32
pub struct QExposeEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QExposeEvent {
  pub fn NewQExposeEvent<T: QExposeEvent_NewQExposeEvent>(value: T) -> QExposeEvent {
    let rsthis = value.NewQExposeEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QExposeEvent_NewQExposeEvent {
  fn NewQExposeEvent(self) -> QExposeEvent;
}

// proto: void QExposeEvent::NewQExposeEvent(const QRegion & rgn);
impl<'a> /*trait*/ QExposeEvent_NewQExposeEvent for (&'a  QRegion) {
  fn NewQExposeEvent(self) -> QExposeEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QExposeEventC1ERK7QRegion()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QExposeEventC1ERK7QRegion(qthis, arg0)};
    let rsthis = QExposeEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QExposeEvent {
  pub fn region<T: QExposeEvent_region>(&mut self, value: T) -> i32 {
    value.region(self);
    return 1;
  }
}

pub trait QExposeEvent_region {
  fn region(self, this: &mut QExposeEvent) -> i32;
}

// proto: const QRegion & QExposeEvent::region();
impl<'a> /*trait*/ QExposeEvent_region for () {
  fn region(self, this: &mut QExposeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QExposeEvent6regionEv()};
    unsafe {_ZNK12QExposeEvent6regionEv()};
    return 1;
  }
}

impl /*struct*/ QExposeEvent {
  pub fn FreeQExposeEvent<T: QExposeEvent_FreeQExposeEvent>(&mut self, value: T) -> i32 {
    value.FreeQExposeEvent(self);
    return 1;
  }
}

pub trait QExposeEvent_FreeQExposeEvent {
  fn FreeQExposeEvent(self, this: &mut QExposeEvent) -> i32;
}

// proto: void QExposeEvent::FreeQExposeEvent();
impl<'a> /*trait*/ QExposeEvent_FreeQExposeEvent for () {
  fn FreeQExposeEvent(self, this: &mut QExposeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QExposeEventD0Ev()};
    unsafe {_ZN12QExposeEventD0Ev()};
    return 1;
  }
}

