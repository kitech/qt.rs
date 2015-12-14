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
  fn _ZNK17QContextMenuEvent1yEv(qthis: *mut c_void) -> c_int;
  // proto:  int QContextMenuEvent::x();
  fn _ZNK17QContextMenuEvent1xEv(qthis: *mut c_void) -> c_int;
  // proto:  void QContextMenuEvent::FreeQContextMenuEvent();
  fn _ZN17QContextMenuEventD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QContextMenuEvent)=1
pub struct QContextMenuEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QContextMenuEvent {
  pub fn globalPos<T: QContextMenuEvent_globalPos>(&mut self, value: T) -> QPoint {
    return value.globalPos(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_globalPos {
  fn globalPos(self, rsthis: &mut QContextMenuEvent) -> QPoint;
}

// proto:  const QPoint & QContextMenuEvent::globalPos();
impl<'a> /*trait*/ QContextMenuEvent_globalPos for () {
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
  pub fn globalY<T: QContextMenuEvent_globalY>(&mut self, value: T) -> i32 {
    return value.globalY(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_globalY {
  fn globalY(self, rsthis: &mut QContextMenuEvent) -> i32;
}

// proto:  int QContextMenuEvent::globalY();
impl<'a> /*trait*/ QContextMenuEvent_globalY for () {
  fn globalY(self, rsthis: &mut QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent7globalYEv()};
    let mut ret = unsafe {_ZNK17QContextMenuEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QContextMenuEvent {
  pub fn globalX<T: QContextMenuEvent_globalX>(&mut self, value: T) -> i32 {
    return value.globalX(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_globalX {
  fn globalX(self, rsthis: &mut QContextMenuEvent) -> i32;
}

// proto:  int QContextMenuEvent::globalX();
impl<'a> /*trait*/ QContextMenuEvent_globalX for () {
  fn globalX(self, rsthis: &mut QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent7globalXEv()};
    let mut ret = unsafe {_ZNK17QContextMenuEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QContextMenuEvent {
  pub fn pos<T: QContextMenuEvent_pos>(&mut self, value: T) -> QPoint {
    return value.pos(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_pos {
  fn pos(self, rsthis: &mut QContextMenuEvent) -> QPoint;
}

// proto:  const QPoint & QContextMenuEvent::pos();
impl<'a> /*trait*/ QContextMenuEvent_pos for () {
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
  pub fn y<T: QContextMenuEvent_y>(&mut self, value: T) -> i32 {
    return value.y(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_y {
  fn y(self, rsthis: &mut QContextMenuEvent) -> i32;
}

// proto:  int QContextMenuEvent::y();
impl<'a> /*trait*/ QContextMenuEvent_y for () {
  fn y(self, rsthis: &mut QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent1yEv()};
    let mut ret = unsafe {_ZNK17QContextMenuEvent1yEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QContextMenuEvent {
  pub fn x<T: QContextMenuEvent_x>(&mut self, value: T) -> i32 {
    return value.x(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_x {
  fn x(self, rsthis: &mut QContextMenuEvent) -> i32;
}

// proto:  int QContextMenuEvent::x();
impl<'a> /*trait*/ QContextMenuEvent_x for () {
  fn x(self, rsthis: &mut QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent1xEv()};
    let mut ret = unsafe {_ZNK17QContextMenuEvent1xEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QContextMenuEvent {
  pub fn FreeQContextMenuEvent<T: QContextMenuEvent_FreeQContextMenuEvent>(&mut self, value: T)  {
     value.FreeQContextMenuEvent(self);
    // return 1;
  }
}

pub trait QContextMenuEvent_FreeQContextMenuEvent {
  fn FreeQContextMenuEvent(self, rsthis: &mut QContextMenuEvent) ;
}

// proto:  void QContextMenuEvent::FreeQContextMenuEvent();
impl<'a> /*trait*/ QContextMenuEvent_FreeQContextMenuEvent for () {
  fn FreeQContextMenuEvent(self, rsthis: &mut QContextMenuEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QContextMenuEventD0Ev()};
     unsafe {_ZN17QContextMenuEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

