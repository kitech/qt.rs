// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: const QPoint & QHelpEvent::globalPos();
  fn _ZNK10QHelpEvent9globalPosEv() -> i32;
  // proto: int QHelpEvent::globalX();
  fn _ZNK10QHelpEvent7globalXEv() -> i32;
  // proto: const QPoint & QHelpEvent::pos();
  fn _ZNK10QHelpEvent3posEv() -> i32;
  // proto: int QHelpEvent::y();
  fn _ZNK10QHelpEvent1yEv() -> i32;
  // proto: int QHelpEvent::globalY();
  fn _ZNK10QHelpEvent7globalYEv() -> i32;
  // proto: int QHelpEvent::x();
  fn _ZNK10QHelpEvent1xEv() -> i32;
  // proto: void QHelpEvent::FreeQHelpEvent();
  fn _ZN10QHelpEventD0Ev() -> i32;
}

// body block begin
// class sizeof(QHelpEvent)=40
pub struct QHelpEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QHelpEvent {
  pub fn globalPos<T: QHelpEvent_globalPos>(&mut self, value: T) -> i32 {
    value.globalPos(self);
    return 1;
  }
}

pub trait QHelpEvent_globalPos {
  fn globalPos(self, this: &mut QHelpEvent) -> i32;
}

// proto: const QPoint & QHelpEvent::globalPos();
impl<'a> /*trait*/ QHelpEvent_globalPos for () {
  fn globalPos(self, this: &mut QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent9globalPosEv()};
    unsafe {_ZNK10QHelpEvent9globalPosEv()};
    return 1;
  }
}

impl /*struct*/ QHelpEvent {
  pub fn globalX<T: QHelpEvent_globalX>(&mut self, value: T) -> i32 {
    value.globalX(self);
    return 1;
  }
}

pub trait QHelpEvent_globalX {
  fn globalX(self, this: &mut QHelpEvent) -> i32;
}

// proto: int QHelpEvent::globalX();
impl<'a> /*trait*/ QHelpEvent_globalX for () {
  fn globalX(self, this: &mut QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent7globalXEv()};
    unsafe {_ZNK10QHelpEvent7globalXEv()};
    return 1;
  }
}

impl /*struct*/ QHelpEvent {
  pub fn pos<T: QHelpEvent_pos>(&mut self, value: T) -> i32 {
    value.pos(self);
    return 1;
  }
}

pub trait QHelpEvent_pos {
  fn pos(self, this: &mut QHelpEvent) -> i32;
}

// proto: const QPoint & QHelpEvent::pos();
impl<'a> /*trait*/ QHelpEvent_pos for () {
  fn pos(self, this: &mut QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent3posEv()};
    unsafe {_ZNK10QHelpEvent3posEv()};
    return 1;
  }
}

impl /*struct*/ QHelpEvent {
  pub fn y<T: QHelpEvent_y>(&mut self, value: T) -> i32 {
    value.y(self);
    return 1;
  }
}

pub trait QHelpEvent_y {
  fn y(self, this: &mut QHelpEvent) -> i32;
}

// proto: int QHelpEvent::y();
impl<'a> /*trait*/ QHelpEvent_y for () {
  fn y(self, this: &mut QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent1yEv()};
    unsafe {_ZNK10QHelpEvent1yEv()};
    return 1;
  }
}

impl /*struct*/ QHelpEvent {
  pub fn globalY<T: QHelpEvent_globalY>(&mut self, value: T) -> i32 {
    value.globalY(self);
    return 1;
  }
}

pub trait QHelpEvent_globalY {
  fn globalY(self, this: &mut QHelpEvent) -> i32;
}

// proto: int QHelpEvent::globalY();
impl<'a> /*trait*/ QHelpEvent_globalY for () {
  fn globalY(self, this: &mut QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent7globalYEv()};
    unsafe {_ZNK10QHelpEvent7globalYEv()};
    return 1;
  }
}

impl /*struct*/ QHelpEvent {
  pub fn x<T: QHelpEvent_x>(&mut self, value: T) -> i32 {
    value.x(self);
    return 1;
  }
}

pub trait QHelpEvent_x {
  fn x(self, this: &mut QHelpEvent) -> i32;
}

// proto: int QHelpEvent::x();
impl<'a> /*trait*/ QHelpEvent_x for () {
  fn x(self, this: &mut QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK10QHelpEvent1xEv()};
    unsafe {_ZNK10QHelpEvent1xEv()};
    return 1;
  }
}

impl /*struct*/ QHelpEvent {
  pub fn FreeQHelpEvent<T: QHelpEvent_FreeQHelpEvent>(&mut self, value: T) -> i32 {
    value.FreeQHelpEvent(self);
    return 1;
  }
}

pub trait QHelpEvent_FreeQHelpEvent {
  fn FreeQHelpEvent(self, this: &mut QHelpEvent) -> i32;
}

// proto: void QHelpEvent::FreeQHelpEvent();
impl<'a> /*trait*/ QHelpEvent_FreeQHelpEvent for () {
  fn FreeQHelpEvent(self, this: &mut QHelpEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN10QHelpEventD0Ev()};
    unsafe {_ZN10QHelpEventD0Ev()};
    return 1;
  }
}

