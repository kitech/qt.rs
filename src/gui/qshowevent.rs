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
  // proto:  void QShowEvent::~QShowEvent();
  fn _ZN10QShowEventD0Ev(qthis: *mut c_void);
  // proto:  void QShowEvent::QShowEvent();
  fn _ZN10QShowEventC1Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QShowEvent)=24
pub struct QShowEvent {
  pub qclsinst: *mut c_void,
}

  // proto:  void QShowEvent::~QShowEvent();
impl /*struct*/ QShowEvent {
  pub fn FreeQShowEvent<RetType, T: QShowEvent_FreeQShowEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQShowEvent(self);
    // return 1;
  }
}

pub trait QShowEvent_FreeQShowEvent<RetType> {
  fn FreeQShowEvent(self , rsthis: &mut QShowEvent) -> RetType;
}

  // proto:  void QShowEvent::~QShowEvent();
impl<'a> /*trait*/ QShowEvent_FreeQShowEvent<()> for () {
  fn FreeQShowEvent(self , rsthis: &mut QShowEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QShowEventD0Ev()};
     unsafe {_ZN10QShowEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QShowEvent::QShowEvent();
impl /*struct*/ QShowEvent {
  pub fn NewQShowEvent<T: QShowEvent_NewQShowEvent>(value: T) -> QShowEvent {
    let rsthis = value.NewQShowEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QShowEvent_NewQShowEvent {
  fn NewQShowEvent(self) -> QShowEvent;
}

  // proto:  void QShowEvent::QShowEvent();
impl<'a> /*trait*/ QShowEvent_NewQShowEvent for () {
  fn NewQShowEvent(self) -> QShowEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QShowEventC1Ev()};
    unsafe {_ZN10QShowEventC1Ev(qthis)};
    let rsthis = QShowEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

