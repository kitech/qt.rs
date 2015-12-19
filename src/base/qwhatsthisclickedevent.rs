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
  // proto:  QString QWhatsThisClickedEvent::href();
  fn _ZNK22QWhatsThisClickedEvent4hrefEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWhatsThisClickedEvent::FreeQWhatsThisClickedEvent();
  fn _ZN22QWhatsThisClickedEventD0Ev(qthis: *mut c_void) ;
  // proto:  void QWhatsThisClickedEvent::NewQWhatsThisClickedEvent(const QString & href);
  fn _ZN22QWhatsThisClickedEventC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QWhatsThisClickedEvent)=32
pub struct QWhatsThisClickedEvent {
  pub qclsinst: *mut c_void,
}

// proto:  QString QWhatsThisClickedEvent::href();
impl /*struct*/ QWhatsThisClickedEvent {
  pub fn href<RetType, T: QWhatsThisClickedEvent_href<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.href(self);
    // return 1;
  }
}

pub trait QWhatsThisClickedEvent_href<RetType> {
  fn href(self , rsthis: &mut QWhatsThisClickedEvent) -> RetType;
}

// proto:  QString QWhatsThisClickedEvent::href();
impl<'a> /*trait*/ QWhatsThisClickedEvent_href<QString> for () {
  fn href(self , rsthis: &mut QWhatsThisClickedEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QWhatsThisClickedEvent4hrefEv()};
    let mut ret = unsafe {_ZNK22QWhatsThisClickedEvent4hrefEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QWhatsThisClickedEvent::FreeQWhatsThisClickedEvent();
impl /*struct*/ QWhatsThisClickedEvent {
  pub fn FreeQWhatsThisClickedEvent<RetType, T: QWhatsThisClickedEvent_FreeQWhatsThisClickedEvent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQWhatsThisClickedEvent(self);
    // return 1;
  }
}

pub trait QWhatsThisClickedEvent_FreeQWhatsThisClickedEvent<RetType> {
  fn FreeQWhatsThisClickedEvent(self , rsthis: &mut QWhatsThisClickedEvent) -> RetType;
}

// proto:  void QWhatsThisClickedEvent::FreeQWhatsThisClickedEvent();
impl<'a> /*trait*/ QWhatsThisClickedEvent_FreeQWhatsThisClickedEvent<()> for () {
  fn FreeQWhatsThisClickedEvent(self , rsthis: &mut QWhatsThisClickedEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QWhatsThisClickedEventD0Ev()};
     unsafe {_ZN22QWhatsThisClickedEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWhatsThisClickedEvent {
  pub fn NewQWhatsThisClickedEvent<T: QWhatsThisClickedEvent_NewQWhatsThisClickedEvent>(value: T) -> QWhatsThisClickedEvent {
    let rsthis = value.NewQWhatsThisClickedEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QWhatsThisClickedEvent_NewQWhatsThisClickedEvent {
  fn NewQWhatsThisClickedEvent(self) -> QWhatsThisClickedEvent;
}

// proto: void QWhatsThisClickedEvent::NewQWhatsThisClickedEvent(const QString & href);
impl<'a> /*trait*/ QWhatsThisClickedEvent_NewQWhatsThisClickedEvent for (&'a  QString) {
  fn NewQWhatsThisClickedEvent(self) -> QWhatsThisClickedEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QWhatsThisClickedEventC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QWhatsThisClickedEventC1ERK7QString(qthis, arg0)};
    let rsthis = QWhatsThisClickedEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

