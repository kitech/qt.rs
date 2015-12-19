// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QMoveEvent::FreeQMoveEvent();
  fn _ZN10QMoveEventD0Ev(qthis: *mut c_void) ;
  // proto:  const QPoint & QMoveEvent::oldPos();
  fn _ZNK10QMoveEvent6oldPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMoveEvent::NewQMoveEvent(const QPoint & pos, const QPoint & oldPos);
  fn _ZN10QMoveEventC1ERK6QPointS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  const QPoint & QMoveEvent::pos();
  fn _ZNK10QMoveEvent3posEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QMoveEvent)=40
pub struct QMoveEvent {
  pub qclsinst: *mut c_void,
}

// proto:  void QMoveEvent::FreeQMoveEvent();
impl /*struct*/ QMoveEvent {
  pub fn FreeQMoveEvent<RetType, T: QMoveEvent_FreeQMoveEvent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQMoveEvent(self);
    // return 1;
  }
}

pub trait QMoveEvent_FreeQMoveEvent<RetType> {
  fn FreeQMoveEvent(self , rsthis: &mut QMoveEvent) -> RetType;
}

// proto:  void QMoveEvent::FreeQMoveEvent();
impl<'a> /*trait*/ QMoveEvent_FreeQMoveEvent<()> for () {
  fn FreeQMoveEvent(self , rsthis: &mut QMoveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN10QMoveEventD0Ev()};
     unsafe {_ZN10QMoveEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  const QPoint & QMoveEvent::oldPos();
impl /*struct*/ QMoveEvent {
  pub fn oldPos<RetType, T: QMoveEvent_oldPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.oldPos(self);
    // return 1;
  }
}

pub trait QMoveEvent_oldPos<RetType> {
  fn oldPos(self , rsthis: &mut QMoveEvent) -> RetType;
}

// proto:  const QPoint & QMoveEvent::oldPos();
impl<'a> /*trait*/ QMoveEvent_oldPos<QPoint> for () {
  fn oldPos(self , rsthis: &mut QMoveEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QMoveEvent6oldPosEv()};
    let mut ret = unsafe {_ZNK10QMoveEvent6oldPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMoveEvent {
  pub fn NewQMoveEvent<T: QMoveEvent_NewQMoveEvent>(value: T) -> QMoveEvent {
    let rsthis = value.NewQMoveEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QMoveEvent_NewQMoveEvent {
  fn NewQMoveEvent(self) -> QMoveEvent;
}

// proto: void QMoveEvent::NewQMoveEvent(const QPoint & pos, const QPoint & oldPos);
impl<'a> /*trait*/ QMoveEvent_NewQMoveEvent for (&'a  QPoint, &'a  QPoint) {
  fn NewQMoveEvent(self) -> QMoveEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN10QMoveEventC1ERK6QPointS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN10QMoveEventC1ERK6QPointS2_(qthis, arg0, arg1)};
    let rsthis = QMoveEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  const QPoint & QMoveEvent::pos();
impl /*struct*/ QMoveEvent {
  pub fn pos<RetType, T: QMoveEvent_pos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QMoveEvent_pos<RetType> {
  fn pos(self , rsthis: &mut QMoveEvent) -> RetType;
}

// proto:  const QPoint & QMoveEvent::pos();
impl<'a> /*trait*/ QMoveEvent_pos<QPoint> for () {
  fn pos(self , rsthis: &mut QMoveEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QMoveEvent3posEv()};
    let mut ret = unsafe {_ZNK10QMoveEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

