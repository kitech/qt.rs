// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrect::QRect;
use super::qpoint::QPoint;
use super::qmimedata::QMimeData;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QDragMoveEvent::accept(const QRect & r);
  fn _ZN14QDragMoveEvent6acceptERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRect QDragMoveEvent::answerRect();
  fn _ZNK14QDragMoveEvent10answerRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDragMoveEvent::ignore(const QRect & r);
  fn _ZN14QDragMoveEvent6ignoreERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDragMoveEvent::ignore();
  fn _ZN14QDragMoveEvent6ignoreEv(qthis: *mut c_void);
  // proto:  void QDragMoveEvent::~QDragMoveEvent();
  fn _ZN14QDragMoveEventD0Ev(qthis: *mut c_void);
  // proto:  void QDragMoveEvent::accept();
  fn _ZN14QDragMoveEvent6acceptEv(qthis: *mut c_void);
}

// body block begin
// class sizeof(QDragMoveEvent)=1
pub struct QDragMoveEvent {
  pub qclsinst: *mut c_void,
}

  // proto:  void QDragMoveEvent::accept(const QRect & r);
impl /*struct*/ QDragMoveEvent {
  pub fn accept<RetType, T: QDragMoveEvent_accept<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.accept(self);
    // return 1;
  }
}

pub trait QDragMoveEvent_accept<RetType> {
  fn accept(self , rsthis: &mut QDragMoveEvent) -> RetType;
}

  // proto:  void QDragMoveEvent::accept(const QRect & r);
impl<'a> /*trait*/ QDragMoveEvent_accept<()> for (QRect) {
  fn accept(self , rsthis: &mut QDragMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEvent6acceptERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QDragMoveEvent6acceptERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QDragMoveEvent::answerRect();
impl /*struct*/ QDragMoveEvent {
  pub fn answerRect<RetType, T: QDragMoveEvent_answerRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.answerRect(self);
    // return 1;
  }
}

pub trait QDragMoveEvent_answerRect<RetType> {
  fn answerRect(self , rsthis: &mut QDragMoveEvent) -> RetType;
}

  // proto:  QRect QDragMoveEvent::answerRect();
impl<'a> /*trait*/ QDragMoveEvent_answerRect<QRect> for () {
  fn answerRect(self , rsthis: &mut QDragMoveEvent) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDragMoveEvent10answerRectEv()};
    let mut ret = unsafe {_ZNK14QDragMoveEvent10answerRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QDragMoveEvent::ignore(const QRect & r);
impl /*struct*/ QDragMoveEvent {
  pub fn ignore<RetType, T: QDragMoveEvent_ignore<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.ignore(self);
    // return 1;
  }
}

pub trait QDragMoveEvent_ignore<RetType> {
  fn ignore(self , rsthis: &mut QDragMoveEvent) -> RetType;
}

  // proto:  void QDragMoveEvent::ignore(const QRect & r);
impl<'a> /*trait*/ QDragMoveEvent_ignore<()> for (QRect) {
  fn ignore(self , rsthis: &mut QDragMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEvent6ignoreERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QDragMoveEvent6ignoreERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QDragMoveEvent::ignore();
impl<'a> /*trait*/ QDragMoveEvent_ignore<()> for () {
  fn ignore(self , rsthis: &mut QDragMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEvent6ignoreEv()};
     unsafe {_ZN14QDragMoveEvent6ignoreEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDragMoveEvent::~QDragMoveEvent();
impl /*struct*/ QDragMoveEvent {
  pub fn FreeQDragMoveEvent<RetType, T: QDragMoveEvent_FreeQDragMoveEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQDragMoveEvent(self);
    // return 1;
  }
}

pub trait QDragMoveEvent_FreeQDragMoveEvent<RetType> {
  fn FreeQDragMoveEvent(self , rsthis: &mut QDragMoveEvent) -> RetType;
}

  // proto:  void QDragMoveEvent::~QDragMoveEvent();
impl<'a> /*trait*/ QDragMoveEvent_FreeQDragMoveEvent<()> for () {
  fn FreeQDragMoveEvent(self , rsthis: &mut QDragMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEventD0Ev()};
     unsafe {_ZN14QDragMoveEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDragMoveEvent::accept();
impl<'a> /*trait*/ QDragMoveEvent_accept<()> for () {
  fn accept(self , rsthis: &mut QDragMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDragMoveEvent6acceptEv()};
     unsafe {_ZN14QDragMoveEvent6acceptEv(rsthis.qclsinst)};
    // return 1;
  }
}

