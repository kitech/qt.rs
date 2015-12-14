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
  // proto:  void QToolBarChangeEvent::NewQToolBarChangeEvent(bool t);
  fn _ZN19QToolBarChangeEventC1Eb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QToolBarChangeEvent::FreeQToolBarChangeEvent();
  fn _ZN19QToolBarChangeEventD0Ev(qthis: *mut c_void) ;
  // proto:  bool QToolBarChangeEvent::toggle();
  fn _ZNK19QToolBarChangeEvent6toggleEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QToolBarChangeEvent)=24
pub struct QToolBarChangeEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QToolBarChangeEvent {
  pub fn NewQToolBarChangeEvent<T: QToolBarChangeEvent_NewQToolBarChangeEvent>(value: T) -> QToolBarChangeEvent {
    let rsthis = value.NewQToolBarChangeEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QToolBarChangeEvent_NewQToolBarChangeEvent {
  fn NewQToolBarChangeEvent(self) -> QToolBarChangeEvent;
}

// proto: void QToolBarChangeEvent::NewQToolBarChangeEvent(bool t);
impl<'a> /*trait*/ QToolBarChangeEvent_NewQToolBarChangeEvent for (i8) {
  fn NewQToolBarChangeEvent(self) -> QToolBarChangeEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QToolBarChangeEventC1Eb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN19QToolBarChangeEventC1Eb(qthis, arg0)};
    let rsthis = QToolBarChangeEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QToolBarChangeEvent {
  pub fn FreeQToolBarChangeEvent<T: QToolBarChangeEvent_FreeQToolBarChangeEvent>(&mut self, value: T)  {
     value.FreeQToolBarChangeEvent(self);
    // return 1;
  }
}

pub trait QToolBarChangeEvent_FreeQToolBarChangeEvent {
  fn FreeQToolBarChangeEvent(self, rsthis: &mut QToolBarChangeEvent) ;
}

// proto:  void QToolBarChangeEvent::FreeQToolBarChangeEvent();
impl<'a> /*trait*/ QToolBarChangeEvent_FreeQToolBarChangeEvent for () {
  fn FreeQToolBarChangeEvent(self, rsthis: &mut QToolBarChangeEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QToolBarChangeEventD0Ev()};
     unsafe {_ZN19QToolBarChangeEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QToolBarChangeEvent {
  pub fn toggle<T: QToolBarChangeEvent_toggle>(&mut self, value: T) -> i8 {
    return value.toggle(self);
    // return 1;
  }
}

pub trait QToolBarChangeEvent_toggle {
  fn toggle(self, rsthis: &mut QToolBarChangeEvent) -> i8;
}

// proto:  bool QToolBarChangeEvent::toggle();
impl<'a> /*trait*/ QToolBarChangeEvent_toggle for () {
  fn toggle(self, rsthis: &mut QToolBarChangeEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QToolBarChangeEvent6toggleEv()};
    let mut ret = unsafe {_ZNK19QToolBarChangeEvent6toggleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

