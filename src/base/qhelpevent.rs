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
  fn _ZNK10QHelpEvent1yEv(qthis: *mut c_void) ;
  // proto:  int QHelpEvent::globalY();
  fn _ZNK10QHelpEvent7globalYEv(qthis: *mut c_void) -> c_int;
  // proto:  int QHelpEvent::x();
  fn _ZNK10QHelpEvent1xEv(qthis: *mut c_void) ;
  // proto:  void QHelpEvent::FreeQHelpEvent();
  fn _ZN10QHelpEventD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QHelpEvent)=40
pub struct QHelpEvent {
  pub qclsinst: *mut c_void,
}

// proto:  const QPoint & QHelpEvent::globalPos();
impl /*struct*/ QHelpEvent {
  pub fn globalPos<RetType, T: QHelpEvent_globalPos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.globalPos(self);
    // return 1;
  }
}

pub trait QHelpEvent_globalPos<RetType> {
  fn globalPos(self , rsthis: &mut QHelpEvent) -> RetType;
}

// proto:  const QPoint & QHelpEvent::globalPos();
impl<'a> /*trait*/ QHelpEvent_globalPos<QPoint> for () {
  fn globalPos(self , rsthis: &mut QHelpEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent9globalPosEv()};
    let mut ret = unsafe {_ZNK10QHelpEvent9globalPosEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QHelpEvent::globalX();
impl /*struct*/ QHelpEvent {
  pub fn globalX<RetType, T: QHelpEvent_globalX<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.globalX(self);
    // return 1;
  }
}

pub trait QHelpEvent_globalX<RetType> {
  fn globalX(self , rsthis: &mut QHelpEvent) -> RetType;
}

// proto:  int QHelpEvent::globalX();
impl<'a> /*trait*/ QHelpEvent_globalX<i32> for () {
  fn globalX(self , rsthis: &mut QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent7globalXEv()};
    let mut ret = unsafe {_ZNK10QHelpEvent7globalXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  const QPoint & QHelpEvent::pos();
impl /*struct*/ QHelpEvent {
  pub fn pos<RetType, T: QHelpEvent_pos<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QHelpEvent_pos<RetType> {
  fn pos(self , rsthis: &mut QHelpEvent) -> RetType;
}

// proto:  const QPoint & QHelpEvent::pos();
impl<'a> /*trait*/ QHelpEvent_pos<QPoint> for () {
  fn pos(self , rsthis: &mut QHelpEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent3posEv()};
    let mut ret = unsafe {_ZNK10QHelpEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QHelpEvent::y();
impl /*struct*/ QHelpEvent {
  pub fn y<RetType, T: QHelpEvent_y<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QHelpEvent_y<RetType> {
  fn y(self , rsthis: &mut QHelpEvent) -> RetType;
}

// proto:  int QHelpEvent::y();
impl<'a> /*trait*/ QHelpEvent_y<()> for () {
  fn y(self , rsthis: &mut QHelpEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent1yEv()};
     unsafe {_ZNK10QHelpEvent1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  int QHelpEvent::globalY();
impl /*struct*/ QHelpEvent {
  pub fn globalY<RetType, T: QHelpEvent_globalY<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.globalY(self);
    // return 1;
  }
}

pub trait QHelpEvent_globalY<RetType> {
  fn globalY(self , rsthis: &mut QHelpEvent) -> RetType;
}

// proto:  int QHelpEvent::globalY();
impl<'a> /*trait*/ QHelpEvent_globalY<i32> for () {
  fn globalY(self , rsthis: &mut QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent7globalYEv()};
    let mut ret = unsafe {_ZNK10QHelpEvent7globalYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QHelpEvent::x();
impl /*struct*/ QHelpEvent {
  pub fn x<RetType, T: QHelpEvent_x<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QHelpEvent_x<RetType> {
  fn x(self , rsthis: &mut QHelpEvent) -> RetType;
}

// proto:  int QHelpEvent::x();
impl<'a> /*trait*/ QHelpEvent_x<()> for () {
  fn x(self , rsthis: &mut QHelpEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent1xEv()};
     unsafe {_ZNK10QHelpEvent1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QHelpEvent::FreeQHelpEvent();
impl /*struct*/ QHelpEvent {
  pub fn FreeQHelpEvent<RetType, T: QHelpEvent_FreeQHelpEvent<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQHelpEvent(self);
    // return 1;
  }
}

pub trait QHelpEvent_FreeQHelpEvent<RetType> {
  fn FreeQHelpEvent(self , rsthis: &mut QHelpEvent) -> RetType;
}

// proto:  void QHelpEvent::FreeQHelpEvent();
impl<'a> /*trait*/ QHelpEvent_FreeQHelpEvent<()> for () {
  fn FreeQHelpEvent(self , rsthis: &mut QHelpEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN10QHelpEventD0Ev()};
     unsafe {_ZN10QHelpEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

