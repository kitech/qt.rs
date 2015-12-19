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
  // proto:  void QExposeEvent::NewQExposeEvent(const QRegion & rgn);
  fn _ZN12QExposeEventC1ERK7QRegion(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QRegion & QExposeEvent::region();
  fn _ZNK12QExposeEvent6regionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QExposeEvent::FreeQExposeEvent();
  fn _ZN12QExposeEventD0Ev(qthis: *mut c_void) ;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QExposeEventC1ERK7QRegion(qthis, arg0)};
    let rsthis = QExposeEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  const QRegion & QExposeEvent::region();
impl /*struct*/ QExposeEvent {
  pub fn region<RetType, T: QExposeEvent_region<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.region(self);
    // return 1;
  }
}

pub trait QExposeEvent_region<RetType> {
  fn region(self , rsthis: &mut QExposeEvent) -> RetType;
}

// proto:  const QRegion & QExposeEvent::region();
impl<'a> /*trait*/ QExposeEvent_region<QRegion> for () {
  fn region(self , rsthis: &mut QExposeEvent) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QExposeEvent6regionEv()};
    let mut ret = unsafe {_ZNK12QExposeEvent6regionEv(rsthis.qclsinst)};
    let mut ret1 = QRegion{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QExposeEvent::FreeQExposeEvent();
impl /*struct*/ QExposeEvent {
  pub fn FreeQExposeEvent<RetType, T: QExposeEvent_FreeQExposeEvent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQExposeEvent(self);
    // return 1;
  }
}

pub trait QExposeEvent_FreeQExposeEvent<RetType> {
  fn FreeQExposeEvent(self , rsthis: &mut QExposeEvent) -> RetType;
}

// proto:  void QExposeEvent::FreeQExposeEvent();
impl<'a> /*trait*/ QExposeEvent_FreeQExposeEvent<()> for () {
  fn FreeQExposeEvent(self , rsthis: &mut QExposeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QExposeEventD0Ev()};
     unsafe {_ZN12QExposeEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

