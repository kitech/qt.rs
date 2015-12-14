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
  // proto:  void QInputMethodQueryEvent::FreeQInputMethodQueryEvent();
  fn _ZN22QInputMethodQueryEventD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QInputMethodQueryEvent)=1
pub struct QInputMethodQueryEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QInputMethodQueryEvent {
  pub fn FreeQInputMethodQueryEvent<T: QInputMethodQueryEvent_FreeQInputMethodQueryEvent>(&mut self, value: T)  {
     value.FreeQInputMethodQueryEvent(self);
    // return 1;
  }
}

pub trait QInputMethodQueryEvent_FreeQInputMethodQueryEvent {
  fn FreeQInputMethodQueryEvent(self, rsthis: &mut QInputMethodQueryEvent) ;
}

// proto:  void QInputMethodQueryEvent::FreeQInputMethodQueryEvent();
impl<'a> /*trait*/ QInputMethodQueryEvent_FreeQInputMethodQueryEvent for () {
  fn FreeQInputMethodQueryEvent(self, rsthis: &mut QInputMethodQueryEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QInputMethodQueryEventD0Ev()};
     unsafe {_ZN22QInputMethodQueryEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

