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
  // proto:  const QPoint & QContextMenuEvent::globalPos();
  fn _ZNK17QContextMenuEvent9globalPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QContextMenuEvent::globalY();
  fn _ZNK17QContextMenuEvent7globalYEv(qthis: *mut c_void) -> c_int;
  // proto:  int QContextMenuEvent::globalX();
  fn _ZNK17QContextMenuEvent7globalXEv(qthis: *mut c_void) -> c_int;
  // proto:  const QPoint & QContextMenuEvent::pos();
  fn _ZNK17QContextMenuEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QContextMenuEvent::y();
  fn _ZNK17QContextMenuEvent1yEv(qthis: *mut c_void) ;
  // proto:  int QContextMenuEvent::x();
  fn _ZNK17QContextMenuEvent1xEv(qthis: *mut c_void) ;
  // proto:  void QContextMenuEvent::FreeQContextMenuEvent();
  fn _ZN17QContextMenuEventD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QContextMenuEvent)=1
pub struct QContextMenuEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QContextMenuEvent {
  pub fn globalPos<RetType, T: QContextMenuEvent_globalPos<RetType>>(&mut self, value: T) -> RetType {
    return value.globalPos(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_globalPos<RetType> {
  fn globalPos(self, rsthis: &mut QContextMenuEvent) -> RetType;
}

// proto:  const QPoint & QContextMenuEvent::globalPos();
impl<'a> /*trait*/ QContextMenuEvent_globalPos<QPoint> for () {
  fn globalPos(self, rsthis: &mut QContextMenuEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent9globalPosEv()};
    let mut ret = unsafe {_ZNK17QContextMenuEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QContextMenuEvent {
  pub fn globalY<RetType, T: QContextMenuEvent_globalY<RetType>>(&mut self, value: T) -> RetType {
    return value.globalY(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_globalY<RetType> {
  fn globalY(self, rsthis: &mut QContextMenuEvent) -> RetType;
}

// proto:  int QContextMenuEvent::globalY();
impl<'a> /*trait*/ QContextMenuEvent_globalY<i32> for () {
  fn globalY(self, rsthis: &mut QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent7globalYEv()};
    let mut ret = unsafe {_ZNK17QContextMenuEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QContextMenuEvent {
  pub fn globalX<RetType, T: QContextMenuEvent_globalX<RetType>>(&mut self, value: T) -> RetType {
    return value.globalX(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_globalX<RetType> {
  fn globalX(self, rsthis: &mut QContextMenuEvent) -> RetType;
}

// proto:  int QContextMenuEvent::globalX();
impl<'a> /*trait*/ QContextMenuEvent_globalX<i32> for () {
  fn globalX(self, rsthis: &mut QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent7globalXEv()};
    let mut ret = unsafe {_ZNK17QContextMenuEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QContextMenuEvent {
  pub fn pos<RetType, T: QContextMenuEvent_pos<RetType>>(&mut self, value: T) -> RetType {
    return value.pos(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_pos<RetType> {
  fn pos(self, rsthis: &mut QContextMenuEvent) -> RetType;
}

// proto:  const QPoint & QContextMenuEvent::pos();
impl<'a> /*trait*/ QContextMenuEvent_pos<QPoint> for () {
  fn pos(self, rsthis: &mut QContextMenuEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent3posEv()};
    let mut ret = unsafe {_ZNK17QContextMenuEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QContextMenuEvent {
  pub fn y<RetType, T: QContextMenuEvent_y<RetType>>(&mut self, value: T) -> RetType {
    return value.y(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_y<RetType> {
  fn y(self, rsthis: &mut QContextMenuEvent) -> RetType;
}

// proto:  int QContextMenuEvent::y();
impl<'a> /*trait*/ QContextMenuEvent_y<()> for () {
  fn y(self, rsthis: &mut QContextMenuEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent1yEv()};
     unsafe {_ZNK17QContextMenuEvent1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QContextMenuEvent {
  pub fn x<RetType, T: QContextMenuEvent_x<RetType>>(&mut self, value: T) -> RetType {
    return value.x(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_x<RetType> {
  fn x(self, rsthis: &mut QContextMenuEvent) -> RetType;
}

// proto:  int QContextMenuEvent::x();
impl<'a> /*trait*/ QContextMenuEvent_x<()> for () {
  fn x(self, rsthis: &mut QContextMenuEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent1xEv()};
     unsafe {_ZNK17QContextMenuEvent1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QContextMenuEvent {
  pub fn FreeQContextMenuEvent<RetType, T: QContextMenuEvent_FreeQContextMenuEvent<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQContextMenuEvent(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_FreeQContextMenuEvent<RetType> {
  fn FreeQContextMenuEvent(self, rsthis: &mut QContextMenuEvent) -> RetType;
}

// proto:  void QContextMenuEvent::FreeQContextMenuEvent();
impl<'a> /*trait*/ QContextMenuEvent_FreeQContextMenuEvent<()> for () {
  fn FreeQContextMenuEvent(self, rsthis: &mut QContextMenuEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QContextMenuEventD0Ev()};
     unsafe {_ZN17QContextMenuEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

