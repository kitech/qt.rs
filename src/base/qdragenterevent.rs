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
  // proto:  void QDragEnterEvent::FreeQDragEnterEvent();
  fn _ZN15QDragEnterEventD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QDragEnterEvent)=1
pub struct QDragEnterEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDragEnterEvent {
  pub fn FreeQDragEnterEvent<T: QDragEnterEvent_FreeQDragEnterEvent>(&mut self, value: T)  {
     value.FreeQDragEnterEvent(self);
    // return 1;
  }
}

pub trait QDragEnterEvent_FreeQDragEnterEvent {
  fn FreeQDragEnterEvent(self, rsthis: &mut QDragEnterEvent) ;
}

// proto:  void QDragEnterEvent::FreeQDragEnterEvent();
impl<'a> /*trait*/ QDragEnterEvent_FreeQDragEnterEvent for () {
  fn FreeQDragEnterEvent(self, rsthis: &mut QDragEnterEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QDragEnterEventD0Ev()};
     unsafe {_ZN15QDragEnterEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

