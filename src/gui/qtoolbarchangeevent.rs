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
  // proto:  void QToolBarChangeEvent::QToolBarChangeEvent(bool t);
  fn _ZN19QToolBarChangeEventC1Eb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QToolBarChangeEvent::~QToolBarChangeEvent();
  fn _ZN19QToolBarChangeEventD0Ev(qthis: *mut c_void);
  // proto:  bool QToolBarChangeEvent::toggle();
  fn _ZNK19QToolBarChangeEvent6toggleEv(qthis: *mut c_void) -> c_char;
}

// body block begin
// class sizeof(QToolBarChangeEvent)=24
pub struct QToolBarChangeEvent {
  pub qclsinst: *mut c_void,
}

  // proto:  void QToolBarChangeEvent::QToolBarChangeEvent(bool t);
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

  // proto:  void QToolBarChangeEvent::QToolBarChangeEvent(bool t);
impl<'a> /*trait*/ QToolBarChangeEvent_NewQToolBarChangeEvent for (i8) {
  fn NewQToolBarChangeEvent(self) -> QToolBarChangeEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QToolBarChangeEventC1Eb()};
    let arg0 = self  as c_char;
    unsafe {_ZN19QToolBarChangeEventC1Eb(qthis, arg0)};
    let rsthis = QToolBarChangeEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QToolBarChangeEvent::~QToolBarChangeEvent();
impl /*struct*/ QToolBarChangeEvent {
  pub fn FreeQToolBarChangeEvent<RetType, T: QToolBarChangeEvent_FreeQToolBarChangeEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQToolBarChangeEvent(self);
    // return 1;
  }
}

pub trait QToolBarChangeEvent_FreeQToolBarChangeEvent<RetType> {
  fn FreeQToolBarChangeEvent(self , rsthis: &mut QToolBarChangeEvent) -> RetType;
}

  // proto:  void QToolBarChangeEvent::~QToolBarChangeEvent();
impl<'a> /*trait*/ QToolBarChangeEvent_FreeQToolBarChangeEvent<()> for () {
  fn FreeQToolBarChangeEvent(self , rsthis: &mut QToolBarChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QToolBarChangeEventD0Ev()};
     unsafe {_ZN19QToolBarChangeEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QToolBarChangeEvent::toggle();
impl /*struct*/ QToolBarChangeEvent {
  pub fn toggle<RetType, T: QToolBarChangeEvent_toggle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toggle(self);
    // return 1;
  }
}

pub trait QToolBarChangeEvent_toggle<RetType> {
  fn toggle(self , rsthis: &mut QToolBarChangeEvent) -> RetType;
}

  // proto:  bool QToolBarChangeEvent::toggle();
impl<'a> /*trait*/ QToolBarChangeEvent_toggle<i8> for () {
  fn toggle(self , rsthis: &mut QToolBarChangeEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QToolBarChangeEvent6toggleEv()};
    let mut ret = unsafe {_ZNK19QToolBarChangeEvent6toggleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

