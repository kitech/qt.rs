// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QHoverEvent::FreeQHoverEvent();
  fn _ZN11QHoverEventD0Ev(qthis: *mut c_void) ;
  // proto:  const QPointF & QHoverEvent::posF();
  fn _ZNK11QHoverEvent4posFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QHoverEvent::oldPos();
  fn _ZNK11QHoverEvent6oldPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QPointF & QHoverEvent::oldPosF();
  fn _ZNK11QHoverEvent7oldPosFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QHoverEvent::pos();
  fn _ZNK11QHoverEvent3posEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QHoverEvent)=1
pub struct QHoverEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QHoverEvent {
  pub fn FreeQHoverEvent<RetType, T: QHoverEvent_FreeQHoverEvent<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQHoverEvent(self);
    // return 1;
  }
}

pub trait QHoverEvent_FreeQHoverEvent<RetType> {
  fn FreeQHoverEvent(self, rsthis: &mut QHoverEvent) -> RetType;
}

// proto:  void QHoverEvent::FreeQHoverEvent();
impl<'a> /*trait*/ QHoverEvent_FreeQHoverEvent<()> for () {
  fn FreeQHoverEvent(self, rsthis: &mut QHoverEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHoverEventD0Ev()};
     unsafe {_ZN11QHoverEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QHoverEvent {
  pub fn posF<RetType, T: QHoverEvent_posF<RetType>>(&mut self, value: T) -> RetType {
    return value.posF(self);
    // return 1;
  }
}

pub trait QHoverEvent_posF<RetType> {
  fn posF(self, rsthis: &mut QHoverEvent) -> RetType;
}

// proto:  const QPointF & QHoverEvent::posF();
impl<'a> /*trait*/ QHoverEvent_posF<QPointF> for () {
  fn posF(self, rsthis: &mut QHoverEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHoverEvent4posFEv()};
    let mut ret = unsafe {_ZNK11QHoverEvent4posFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QHoverEvent {
  pub fn oldPos<RetType, T: QHoverEvent_oldPos<RetType>>(&mut self, value: T) -> RetType {
    return value.oldPos(self);
    // return 1;
  }
}

pub trait QHoverEvent_oldPos<RetType> {
  fn oldPos(self, rsthis: &mut QHoverEvent) -> RetType;
}

// proto:  QPoint QHoverEvent::oldPos();
impl<'a> /*trait*/ QHoverEvent_oldPos<QPoint> for () {
  fn oldPos(self, rsthis: &mut QHoverEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHoverEvent6oldPosEv()};
    let mut ret = unsafe {_ZNK11QHoverEvent6oldPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QHoverEvent {
  pub fn oldPosF<RetType, T: QHoverEvent_oldPosF<RetType>>(&mut self, value: T) -> RetType {
    return value.oldPosF(self);
    // return 1;
  }
}

pub trait QHoverEvent_oldPosF<RetType> {
  fn oldPosF(self, rsthis: &mut QHoverEvent) -> RetType;
}

// proto:  const QPointF & QHoverEvent::oldPosF();
impl<'a> /*trait*/ QHoverEvent_oldPosF<QPointF> for () {
  fn oldPosF(self, rsthis: &mut QHoverEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHoverEvent7oldPosFEv()};
    let mut ret = unsafe {_ZNK11QHoverEvent7oldPosFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QHoverEvent {
  pub fn pos<RetType, T: QHoverEvent_pos<RetType>>(&mut self, value: T) -> RetType {
    return value.pos(self);
    // return 1;
  }
}

pub trait QHoverEvent_pos<RetType> {
  fn pos(self, rsthis: &mut QHoverEvent) -> RetType;
}

// proto:  QPoint QHoverEvent::pos();
impl<'a> /*trait*/ QHoverEvent_pos<QPoint> for () {
  fn pos(self, rsthis: &mut QHoverEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHoverEvent3posEv()};
    let mut ret = unsafe {_ZNK11QHoverEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

