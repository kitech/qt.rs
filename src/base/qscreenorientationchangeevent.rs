// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qscreen::QScreen;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QScreen * QScreenOrientationChangeEvent::screen();
  fn _ZNK29QScreenOrientationChangeEvent6screenEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QScreenOrientationChangeEvent::FreeQScreenOrientationChangeEvent();
  fn _ZN29QScreenOrientationChangeEventD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QScreenOrientationChangeEvent)=40
pub struct QScreenOrientationChangeEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QScreenOrientationChangeEvent {
  pub fn screen<T: QScreenOrientationChangeEvent_screen>(&mut self, value: T) -> QScreen {
    return value.screen(self);
    // return 1;
  }
}

pub trait QScreenOrientationChangeEvent_screen {
  fn screen(self, rsthis: &mut QScreenOrientationChangeEvent) -> QScreen;
}

// proto:  QScreen * QScreenOrientationChangeEvent::screen();
impl<'a> /*trait*/ QScreenOrientationChangeEvent_screen for () {
  fn screen(self, rsthis: &mut QScreenOrientationChangeEvent) -> QScreen {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK29QScreenOrientationChangeEvent6screenEv()};
    let mut ret = unsafe {_ZNK29QScreenOrientationChangeEvent6screenEv(rsthis.qclsinst)};
    let mut ret1 = QScreen{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScreenOrientationChangeEvent {
  pub fn FreeQScreenOrientationChangeEvent<T: QScreenOrientationChangeEvent_FreeQScreenOrientationChangeEvent>(&mut self, value: T)  {
     value.FreeQScreenOrientationChangeEvent(self);
    // return 1;
  }
}

pub trait QScreenOrientationChangeEvent_FreeQScreenOrientationChangeEvent {
  fn FreeQScreenOrientationChangeEvent(self, rsthis: &mut QScreenOrientationChangeEvent) ;
}

// proto:  void QScreenOrientationChangeEvent::FreeQScreenOrientationChangeEvent();
impl<'a> /*trait*/ QScreenOrientationChangeEvent_FreeQScreenOrientationChangeEvent for () {
  fn FreeQScreenOrientationChangeEvent(self, rsthis: &mut QScreenOrientationChangeEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN29QScreenOrientationChangeEventD0Ev()};
     unsafe {_ZN29QScreenOrientationChangeEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

