// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  const QSize & QResizeEvent::oldSize();
  fn _ZNK12QResizeEvent7oldSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QSize & QResizeEvent::size();
  fn _ZNK12QResizeEvent4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QResizeEvent::FreeQResizeEvent();
  fn _ZN12QResizeEventD0Ev(qthis: *mut c_void) ;
  // proto:  void QResizeEvent::NewQResizeEvent(const QSize & size, const QSize & oldSize);
  fn _ZN12QResizeEventC1ERK5QSizeS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
}

// body block begin
// class sizeof(QResizeEvent)=40
pub struct QResizeEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QResizeEvent {
  pub fn oldSize<T: QResizeEvent_oldSize>(&mut self, value: T) -> QSize {
    return value.oldSize(self);
    // return 1;
  }
}

pub trait QResizeEvent_oldSize {
  fn oldSize(self, rsthis: &mut QResizeEvent) -> QSize;
}

// proto:  const QSize & QResizeEvent::oldSize();
impl<'a> /*trait*/ QResizeEvent_oldSize for () {
  fn oldSize(self, rsthis: &mut QResizeEvent) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK12QResizeEvent7oldSizeEv()};
    let mut ret = unsafe {_ZNK12QResizeEvent7oldSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QResizeEvent {
  pub fn size<T: QResizeEvent_size>(&mut self, value: T) -> QSize {
    return value.size(self);
    // return 1;
  }
}

pub trait QResizeEvent_size {
  fn size(self, rsthis: &mut QResizeEvent) -> QSize;
}

// proto:  const QSize & QResizeEvent::size();
impl<'a> /*trait*/ QResizeEvent_size for () {
  fn size(self, rsthis: &mut QResizeEvent) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK12QResizeEvent4sizeEv()};
    let mut ret = unsafe {_ZNK12QResizeEvent4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QResizeEvent {
  pub fn FreeQResizeEvent<T: QResizeEvent_FreeQResizeEvent>(&mut self, value: T)  {
     value.FreeQResizeEvent(self);
    // return 1;
  }
}

pub trait QResizeEvent_FreeQResizeEvent {
  fn FreeQResizeEvent(self, rsthis: &mut QResizeEvent) ;
}

// proto:  void QResizeEvent::FreeQResizeEvent();
impl<'a> /*trait*/ QResizeEvent_FreeQResizeEvent for () {
  fn FreeQResizeEvent(self, rsthis: &mut QResizeEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN12QResizeEventD0Ev()};
     unsafe {_ZN12QResizeEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QResizeEvent {
  pub fn NewQResizeEvent<T: QResizeEvent_NewQResizeEvent>(value: T) -> QResizeEvent {
    let rsthis = value.NewQResizeEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QResizeEvent_NewQResizeEvent {
  fn NewQResizeEvent(self) -> QResizeEvent;
}

// proto: void QResizeEvent::NewQResizeEvent(const QSize & size, const QSize & oldSize);
impl<'a> /*trait*/ QResizeEvent_NewQResizeEvent for (&'a  QSize, &'a  QSize) {
  fn NewQResizeEvent(self) -> QResizeEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN12QResizeEventC1ERK5QSizeS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QResizeEventC1ERK5QSizeS2_(qthis, arg0, arg1)};
    let rsthis = QResizeEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

