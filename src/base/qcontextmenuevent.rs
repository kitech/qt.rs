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
  // proto: const QPoint & QContextMenuEvent::globalPos();
  fn _ZNK17QContextMenuEvent9globalPosEv() -> i32;
  // proto: int QContextMenuEvent::globalY();
  fn _ZNK17QContextMenuEvent7globalYEv() -> i32;
  // proto: int QContextMenuEvent::globalX();
  fn _ZNK17QContextMenuEvent7globalXEv() -> i32;
  // proto: const QPoint & QContextMenuEvent::pos();
  fn _ZNK17QContextMenuEvent3posEv() -> i32;
  // proto: int QContextMenuEvent::y();
  fn _ZNK17QContextMenuEvent1yEv() -> i32;
  // proto: int QContextMenuEvent::x();
  fn _ZNK17QContextMenuEvent1xEv() -> i32;
  // proto: void QContextMenuEvent::FreeQContextMenuEvent();
  fn _ZN17QContextMenuEventD0Ev() -> i32;
}

// body block begin
// class sizeof(QContextMenuEvent)=1
pub struct QContextMenuEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QContextMenuEvent {
  pub fn globalPos<T: QContextMenuEvent_globalPos>(&mut self, value: T) -> i32 {
    value.globalPos(self);
    return 1;
  }
}

pub trait QContextMenuEvent_globalPos {
  fn globalPos(self, this: &mut QContextMenuEvent) -> i32;
}

// proto: const QPoint & QContextMenuEvent::globalPos();
impl<'a> /*trait*/ QContextMenuEvent_globalPos for () {
  fn globalPos(self, this: &mut QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent9globalPosEv()};
    unsafe {_ZNK17QContextMenuEvent9globalPosEv()};
    return 1;
  }
}

impl /*struct*/ QContextMenuEvent {
  pub fn globalY<T: QContextMenuEvent_globalY>(&mut self, value: T) -> i32 {
    value.globalY(self);
    return 1;
  }
}

pub trait QContextMenuEvent_globalY {
  fn globalY(self, this: &mut QContextMenuEvent) -> i32;
}

// proto: int QContextMenuEvent::globalY();
impl<'a> /*trait*/ QContextMenuEvent_globalY for () {
  fn globalY(self, this: &mut QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent7globalYEv()};
    unsafe {_ZNK17QContextMenuEvent7globalYEv()};
    return 1;
  }
}

impl /*struct*/ QContextMenuEvent {
  pub fn globalX<T: QContextMenuEvent_globalX>(&mut self, value: T) -> i32 {
    value.globalX(self);
    return 1;
  }
}

pub trait QContextMenuEvent_globalX {
  fn globalX(self, this: &mut QContextMenuEvent) -> i32;
}

// proto: int QContextMenuEvent::globalX();
impl<'a> /*trait*/ QContextMenuEvent_globalX for () {
  fn globalX(self, this: &mut QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent7globalXEv()};
    unsafe {_ZNK17QContextMenuEvent7globalXEv()};
    return 1;
  }
}

impl /*struct*/ QContextMenuEvent {
  pub fn pos<T: QContextMenuEvent_pos>(&mut self, value: T) -> i32 {
    value.pos(self);
    return 1;
  }
}

pub trait QContextMenuEvent_pos {
  fn pos(self, this: &mut QContextMenuEvent) -> i32;
}

// proto: const QPoint & QContextMenuEvent::pos();
impl<'a> /*trait*/ QContextMenuEvent_pos for () {
  fn pos(self, this: &mut QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent3posEv()};
    unsafe {_ZNK17QContextMenuEvent3posEv()};
    return 1;
  }
}

impl /*struct*/ QContextMenuEvent {
  pub fn y<T: QContextMenuEvent_y>(&mut self, value: T) -> i32 {
    value.y(self);
    return 1;
  }
}

pub trait QContextMenuEvent_y {
  fn y(self, this: &mut QContextMenuEvent) -> i32;
}

// proto: int QContextMenuEvent::y();
impl<'a> /*trait*/ QContextMenuEvent_y for () {
  fn y(self, this: &mut QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent1yEv()};
    unsafe {_ZNK17QContextMenuEvent1yEv()};
    return 1;
  }
}

impl /*struct*/ QContextMenuEvent {
  pub fn x<T: QContextMenuEvent_x>(&mut self, value: T) -> i32 {
    value.x(self);
    return 1;
  }
}

pub trait QContextMenuEvent_x {
  fn x(self, this: &mut QContextMenuEvent) -> i32;
}

// proto: int QContextMenuEvent::x();
impl<'a> /*trait*/ QContextMenuEvent_x for () {
  fn x(self, this: &mut QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QContextMenuEvent1xEv()};
    unsafe {_ZNK17QContextMenuEvent1xEv()};
    return 1;
  }
}

impl /*struct*/ QContextMenuEvent {
  pub fn FreeQContextMenuEvent<T: QContextMenuEvent_FreeQContextMenuEvent>(&mut self, value: T) -> i32 {
    value.FreeQContextMenuEvent(self);
    return 1;
  }
}

pub trait QContextMenuEvent_FreeQContextMenuEvent {
  fn FreeQContextMenuEvent(self, this: &mut QContextMenuEvent) -> i32;
}

// proto: void QContextMenuEvent::FreeQContextMenuEvent();
impl<'a> /*trait*/ QContextMenuEvent_FreeQContextMenuEvent for () {
  fn FreeQContextMenuEvent(self, this: &mut QContextMenuEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QContextMenuEventD0Ev()};
    unsafe {_ZN17QContextMenuEventD0Ev()};
    return 1;
  }
}

