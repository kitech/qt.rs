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
  // proto:  void QIconDragEvent::FreeQIconDragEvent();
  fn _ZN14QIconDragEventD0Ev(qthis: *mut c_void) ;
  // proto:  void QIconDragEvent::NewQIconDragEvent();
  fn _ZN14QIconDragEventC1Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QIconDragEvent)=24
pub struct QIconDragEvent {
  pub qclsinst: *mut c_void,
}

// proto:  void QIconDragEvent::FreeQIconDragEvent();
impl /*struct*/ QIconDragEvent {
  pub fn FreeQIconDragEvent<RetType, T: QIconDragEvent_FreeQIconDragEvent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQIconDragEvent(self);
    // return 1;
  }
}

pub trait QIconDragEvent_FreeQIconDragEvent<RetType> {
  fn FreeQIconDragEvent(self , rsthis: &mut QIconDragEvent) -> RetType;
}

// proto:  void QIconDragEvent::FreeQIconDragEvent();
impl<'a> /*trait*/ QIconDragEvent_FreeQIconDragEvent<()> for () {
  fn FreeQIconDragEvent(self , rsthis: &mut QIconDragEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QIconDragEventD0Ev()};
     unsafe {_ZN14QIconDragEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QIconDragEvent {
  pub fn NewQIconDragEvent<T: QIconDragEvent_NewQIconDragEvent>(value: T) -> QIconDragEvent {
    let rsthis = value.NewQIconDragEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QIconDragEvent_NewQIconDragEvent {
  fn NewQIconDragEvent(self) -> QIconDragEvent;
}

// proto: void QIconDragEvent::NewQIconDragEvent();
impl<'a> /*trait*/ QIconDragEvent_NewQIconDragEvent for () {
  fn NewQIconDragEvent(self) -> QIconDragEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QIconDragEventC1Ev()};
    unsafe {_ZN14QIconDragEventC1Ev(qthis)};
    let rsthis = QIconDragEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

