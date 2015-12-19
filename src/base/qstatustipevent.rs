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
  // proto:  void QStatusTipEvent::FreeQStatusTipEvent();
  fn _ZN15QStatusTipEventD0Ev(qthis: *mut c_void) ;
  // proto:  QString QStatusTipEvent::tip();
  fn _ZNK15QStatusTipEvent3tipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStatusTipEvent::NewQStatusTipEvent(const QString & tip);
  fn _ZN15QStatusTipEventC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QStatusTipEvent)=32
pub struct QStatusTipEvent {
  pub qclsinst: *mut c_void,
}

// proto:  void QStatusTipEvent::FreeQStatusTipEvent();
impl /*struct*/ QStatusTipEvent {
  pub fn FreeQStatusTipEvent<RetType, T: QStatusTipEvent_FreeQStatusTipEvent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQStatusTipEvent(self);
    // return 1;
  }
}

pub trait QStatusTipEvent_FreeQStatusTipEvent<RetType> {
  fn FreeQStatusTipEvent(self , rsthis: &mut QStatusTipEvent) -> RetType;
}

// proto:  void QStatusTipEvent::FreeQStatusTipEvent();
impl<'a> /*trait*/ QStatusTipEvent_FreeQStatusTipEvent<()> for () {
  fn FreeQStatusTipEvent(self , rsthis: &mut QStatusTipEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QStatusTipEventD0Ev()};
     unsafe {_ZN15QStatusTipEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QString QStatusTipEvent::tip();
impl /*struct*/ QStatusTipEvent {
  pub fn tip<RetType, T: QStatusTipEvent_tip<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.tip(self);
    // return 1;
  }
}

pub trait QStatusTipEvent_tip<RetType> {
  fn tip(self , rsthis: &mut QStatusTipEvent) -> RetType;
}

// proto:  QString QStatusTipEvent::tip();
impl<'a> /*trait*/ QStatusTipEvent_tip<QString> for () {
  fn tip(self , rsthis: &mut QStatusTipEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QStatusTipEvent3tipEv()};
    let mut ret = unsafe {_ZNK15QStatusTipEvent3tipEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QStatusTipEventC1ERK7QString(qthis, arg0)};
    let rsthis = QStatusTipEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

