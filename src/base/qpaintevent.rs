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
  // proto:  void QPaintEvent::FreeQPaintEvent();
  fn _ZN11QPaintEventD0Ev(qthis: *mut c_void) ;
  // proto:  const QRect & QPaintEvent::rect();
  fn _ZNK11QPaintEvent4rectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPaintEvent::NewQPaintEvent(const QRect & paintRect);
  fn _ZN11QPaintEventC1ERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QRegion & QPaintEvent::region();
  fn _ZNK11QPaintEvent6regionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPaintEvent::NewQPaintEvent(const QRegion & paintRegion);
  fn _ZN11QPaintEventC1ERK7QRegion(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QPaintEvent)=56
pub struct QPaintEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPaintEvent {
  pub fn FreeQPaintEvent<T: QPaintEvent_FreeQPaintEvent>(&mut self, value: T)  {
     value.FreeQPaintEvent(self);
    // return 1;
  }
}

pub trait QPaintEvent_FreeQPaintEvent {
  fn FreeQPaintEvent(self, rsthis: &mut QPaintEvent) ;
}

// proto:  void QPaintEvent::FreeQPaintEvent();
impl<'a> /*trait*/ QPaintEvent_FreeQPaintEvent for () {
  fn FreeQPaintEvent(self, rsthis: &mut QPaintEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN11QPaintEventD0Ev()};
     unsafe {_ZN11QPaintEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPaintEvent {
  pub fn rect<T: QPaintEvent_rect>(&mut self, value: T) -> QRect {
    return value.rect(self);
    // return 1;
  }
}

pub trait QPaintEvent_rect {
  fn rect(self, rsthis: &mut QPaintEvent) -> QRect;
}

// proto:  const QRect & QPaintEvent::rect();
impl<'a> /*trait*/ QPaintEvent_rect for () {
  fn rect(self, rsthis: &mut QPaintEvent) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK11QPaintEvent4rectEv()};
    let mut ret = unsafe {_ZNK11QPaintEvent4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QPaintEventC1ERK5QRect(qthis, arg0)};
    let rsthis = QPaintEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPaintEvent {
  pub fn region<T: QPaintEvent_region>(&mut self, value: T) -> QRegion {
    return value.region(self);
    // return 1;
  }
}

pub trait QPaintEvent_region {
  fn region(self, rsthis: &mut QPaintEvent) -> QRegion;
}

// proto:  const QRegion & QPaintEvent::region();
impl<'a> /*trait*/ QPaintEvent_region for () {
  fn region(self, rsthis: &mut QPaintEvent) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK11QPaintEvent6regionEv()};
    let mut ret = unsafe {_ZNK11QPaintEvent6regionEv(rsthis.qclsinst)};
    let mut ret1 = QRegion{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QPaintEvent::NewQPaintEvent(const QRegion & paintRegion);
impl<'a> /*trait*/ QPaintEvent_NewQPaintEvent for (&'a  QRegion) {
  fn NewQPaintEvent(self) -> QPaintEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN11QPaintEventC1ERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QPaintEventC1ERK7QRegion(qthis, arg0)};
    let rsthis = QPaintEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

