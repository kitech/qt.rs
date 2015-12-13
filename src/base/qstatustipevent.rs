// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QStatusTipEvent::FreeQStatusTipEvent();
  fn _ZN15QStatusTipEventD0Ev() -> i32;
  // proto: QString QStatusTipEvent::tip();
  fn _ZNK15QStatusTipEvent3tipEv() -> i32;
  // proto: void QStatusTipEvent::NewQStatusTipEvent(const QString & tip);
  fn _ZN15QStatusTipEventC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QStatusTipEvent)=32
pub struct QStatusTipEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStatusTipEvent {
  pub fn FreeQStatusTipEvent<T: QStatusTipEvent_FreeQStatusTipEvent>(&mut self, value: T) -> i32 {
    value.FreeQStatusTipEvent(self);
    return 1;
  }
}

pub trait QStatusTipEvent_FreeQStatusTipEvent {
  fn FreeQStatusTipEvent(self, this: &mut QStatusTipEvent) -> i32;
}

// proto: void QStatusTipEvent::FreeQStatusTipEvent();
impl<'a> /*trait*/ QStatusTipEvent_FreeQStatusTipEvent for () {
  fn FreeQStatusTipEvent(self, this: &mut QStatusTipEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QStatusTipEventD0Ev()};
    unsafe {_ZN15QStatusTipEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QStatusTipEvent {
  pub fn tip<T: QStatusTipEvent_tip>(&mut self, value: T) -> i32 {
    value.tip(self);
    return 1;
  }
}

pub trait QStatusTipEvent_tip {
  fn tip(self, this: &mut QStatusTipEvent) -> i32;
}

// proto: QString QStatusTipEvent::tip();
impl<'a> /*trait*/ QStatusTipEvent_tip for () {
  fn tip(self, this: &mut QStatusTipEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QStatusTipEvent3tipEv()};
    unsafe {_ZNK15QStatusTipEvent3tipEv()};
    return 1;
  }
}

impl /*struct*/ QStatusTipEvent {
  pub fn NewQStatusTipEvent<T: QStatusTipEvent_NewQStatusTipEvent>(value: T) -> QStatusTipEvent {
    let rsthis = value.NewQStatusTipEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QStatusTipEvent_NewQStatusTipEvent {
  fn NewQStatusTipEvent(self) -> QStatusTipEvent;
}

// proto: void QStatusTipEvent::NewQStatusTipEvent(const QString & tip);
impl<'a> /*trait*/ QStatusTipEvent_NewQStatusTipEvent for (&'a  QString) {
  fn NewQStatusTipEvent(self) -> QStatusTipEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QStatusTipEventC1ERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QStatusTipEventC1ERK7QString(qthis, arg0)};
    let rsthis = QStatusTipEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

