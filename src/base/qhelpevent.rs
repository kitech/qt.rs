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
  // proto:  const QPoint & QHelpEvent::globalPos();
  fn _ZNK10QHelpEvent9globalPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QHelpEvent::globalX();
  fn _ZNK10QHelpEvent7globalXEv(qthis: *mut c_void) -> c_int;
  // proto:  const QPoint & QHelpEvent::pos();
  fn _ZNK10QHelpEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QHelpEvent::y();
  fn _ZNK10QHelpEvent1yEv(qthis: *mut c_void) -> c_int;
  // proto:  int QHelpEvent::globalY();
  fn _ZNK10QHelpEvent7globalYEv(qthis: *mut c_void) -> c_int;
  // proto:  int QHelpEvent::x();
  fn _ZNK10QHelpEvent1xEv(qthis: *mut c_void) -> c_int;
  // proto:  void QHelpEvent::FreeQHelpEvent();
  fn _ZN10QHelpEventD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QHelpEvent)=40
pub struct QHelpEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QHelpEvent {
  pub fn globalPos<T: QHelpEvent_globalPos>(&mut self, value: T) -> QPoint {
    return value.globalPos(self);
    // return 1;
  }
}

pub trait QHelpEvent_globalPos {
  fn globalPos(self, rsthis: &mut QHelpEvent) -> QPoint;
}

// proto:  const QPoint & QHelpEvent::globalPos();
impl<'a> /*trait*/ QHelpEvent_globalPos for () {
  fn globalPos(self, rsthis: &mut QHelpEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent9globalPosEv()};
    let mut ret = unsafe {_ZNK10QHelpEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QHelpEvent {
  pub fn globalX<T: QHelpEvent_globalX>(&mut self, value: T) -> i32 {
    return value.globalX(self);
    // return 1;
  }
}

pub trait QHelpEvent_globalX {
  fn globalX(self, rsthis: &mut QHelpEvent) -> i32;
}

// proto:  int QHelpEvent::globalX();
impl<'a> /*trait*/ QHelpEvent_globalX for () {
  fn globalX(self, rsthis: &mut QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent7globalXEv()};
    let mut ret = unsafe {_ZNK10QHelpEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHelpEvent {
  pub fn pos<T: QHelpEvent_pos>(&mut self, value: T) -> QPoint {
    return value.pos(self);
    // return 1;
  }
}

pub trait QHelpEvent_pos {
  fn pos(self, rsthis: &mut QHelpEvent) -> QPoint;
}

// proto:  const QPoint & QHelpEvent::pos();
impl<'a> /*trait*/ QHelpEvent_pos for () {
  fn pos(self, rsthis: &mut QHelpEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent3posEv()};
    let mut ret = unsafe {_ZNK10QHelpEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QHelpEvent {
  pub fn y<T: QHelpEvent_y>(&mut self, value: T) -> i32 {
    return value.y(self);
    // return 1;
  }
}

pub trait QHelpEvent_y {
  fn y(self, rsthis: &mut QHelpEvent) -> i32;
}

// proto:  int QHelpEvent::y();
impl<'a> /*trait*/ QHelpEvent_y for () {
  fn y(self, rsthis: &mut QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent1yEv()};
    let mut ret = unsafe {_ZNK10QHelpEvent1yEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHelpEvent {
  pub fn globalY<T: QHelpEvent_globalY>(&mut self, value: T) -> i32 {
    return value.globalY(self);
    // return 1;
  }
}

pub trait QHelpEvent_globalY {
  fn globalY(self, rsthis: &mut QHelpEvent) -> i32;
}

// proto:  int QHelpEvent::globalY();
impl<'a> /*trait*/ QHelpEvent_globalY for () {
  fn globalY(self, rsthis: &mut QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent7globalYEv()};
    let mut ret = unsafe {_ZNK10QHelpEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHelpEvent {
  pub fn x<T: QHelpEvent_x>(&mut self, value: T) -> i32 {
    return value.x(self);
    // return 1;
  }
}

pub trait QHelpEvent_x {
  fn x(self, rsthis: &mut QHelpEvent) -> i32;
}

// proto:  int QHelpEvent::x();
impl<'a> /*trait*/ QHelpEvent_x for () {
  fn x(self, rsthis: &mut QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent1xEv()};
    let mut ret = unsafe {_ZNK10QHelpEvent1xEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QHelpEvent {
  pub fn FreeQHelpEvent<T: QHelpEvent_FreeQHelpEvent>(&mut self, value: T)  {
     value.FreeQHelpEvent(self);
    // return 1;
  }
}

pub trait QHelpEvent_FreeQHelpEvent {
  fn FreeQHelpEvent(self, rsthis: &mut QHelpEvent) ;
}

// proto:  void QHelpEvent::FreeQHelpEvent();
impl<'a> /*trait*/ QHelpEvent_FreeQHelpEvent for () {
  fn FreeQHelpEvent(self, rsthis: &mut QHelpEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN10QHelpEventD0Ev()};
     unsafe {_ZN10QHelpEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

