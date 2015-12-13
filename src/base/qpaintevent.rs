// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrect::QRect;
use super::qregion::QRegion;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QPaintEvent::FreeQPaintEvent();
  fn _ZN11QPaintEventD0Ev() -> i32;
  // proto: const QRect & QPaintEvent::rect();
  fn _ZNK11QPaintEvent4rectEv() -> i32;
  // proto: void QPaintEvent::NewQPaintEvent(const QRect & paintRect);
  fn _ZN11QPaintEventC1ERK5QRect(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: const QRegion & QPaintEvent::region();
  fn _ZNK11QPaintEvent6regionEv() -> i32;
  // proto: void QPaintEvent::NewQPaintEvent(const QRegion & paintRegion);
  fn _ZN11QPaintEventC1ERK7QRegion(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QPaintEvent)=56
pub struct QPaintEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPaintEvent {
  pub fn FreeQPaintEvent<T: QPaintEvent_FreeQPaintEvent>(&mut self, value: T) -> i32 {
    value.FreeQPaintEvent(self);
    return 1;
  }
}

pub trait QPaintEvent_FreeQPaintEvent {
  fn FreeQPaintEvent(self, this: &mut QPaintEvent) -> i32;
}

// proto: void QPaintEvent::FreeQPaintEvent();
impl<'a> /*trait*/ QPaintEvent_FreeQPaintEvent for () {
  fn FreeQPaintEvent(self, this: &mut QPaintEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN11QPaintEventD0Ev()};
    unsafe {_ZN11QPaintEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPaintEvent {
  pub fn rect<T: QPaintEvent_rect>(&mut self, value: T) -> i32 {
    value.rect(self);
    return 1;
  }
}

pub trait QPaintEvent_rect {
  fn rect(self, this: &mut QPaintEvent) -> i32;
}

// proto: const QRect & QPaintEvent::rect();
impl<'a> /*trait*/ QPaintEvent_rect for () {
  fn rect(self, this: &mut QPaintEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK11QPaintEvent4rectEv()};
    unsafe {_ZNK11QPaintEvent4rectEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEvent {
  pub fn NewQPaintEvent<T: QPaintEvent_NewQPaintEvent>(value: T) -> QPaintEvent {
    let rsthis = value.NewQPaintEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QPaintEvent_NewQPaintEvent {
  fn NewQPaintEvent(self) -> QPaintEvent;
}

// proto: void QPaintEvent::NewQPaintEvent(const QRect & paintRect);
impl<'a> /*trait*/ QPaintEvent_NewQPaintEvent for (&'a  QRect) {
  fn NewQPaintEvent(self) -> QPaintEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN11QPaintEventC1ERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QPaintEventC1ERK5QRect(qthis, arg0)};
    let rsthis = QPaintEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPaintEvent {
  pub fn region<T: QPaintEvent_region>(&mut self, value: T) -> i32 {
    value.region(self);
    return 1;
  }
}

pub trait QPaintEvent_region {
  fn region(self, this: &mut QPaintEvent) -> i32;
}

// proto: const QRegion & QPaintEvent::region();
impl<'a> /*trait*/ QPaintEvent_region for () {
  fn region(self, this: &mut QPaintEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK11QPaintEvent6regionEv()};
    unsafe {_ZNK11QPaintEvent6regionEv()};
    return 1;
  }
}

// proto: void QPaintEvent::NewQPaintEvent(const QRegion & paintRegion);
impl<'a> /*trait*/ QPaintEvent_NewQPaintEvent for (&'a  QRegion) {
  fn NewQPaintEvent(self) -> QPaintEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN11QPaintEventC1ERK7QRegion()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QPaintEventC1ERK7QRegion(qthis, arg0)};
    let rsthis = QPaintEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

