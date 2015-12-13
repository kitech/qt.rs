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
  // proto: void QDropEvent::FreeQDropEvent();
  fn _ZN10QDropEventD0Ev() -> i32;
  // proto: QPoint QDropEvent::pos();
  fn _ZNK10QDropEvent3posEv() -> i32;
  // proto: QObject * QDropEvent::source();
  fn _ZNK10QDropEvent6sourceEv() -> i32;
  // proto: const QMimeData * QDropEvent::mimeData();
  fn _ZNK10QDropEvent8mimeDataEv() -> i32;
  // proto: void QDropEvent::acceptProposedAction();
  fn _ZN10QDropEvent20acceptProposedActionEv() -> i32;
  // proto: const QPointF & QDropEvent::posF();
  fn _ZNK10QDropEvent4posFEv() -> i32;
}

// body block begin
// class sizeof(QDropEvent)=1
pub struct QDropEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDropEvent {
  pub fn FreeQDropEvent<T: QDropEvent_FreeQDropEvent>(&mut self, value: T) -> i32 {
    value.FreeQDropEvent(self);
    return 1;
  }
}

pub trait QDropEvent_FreeQDropEvent {
  fn FreeQDropEvent(self, this: &mut QDropEvent) -> i32;
}

// proto: void QDropEvent::FreeQDropEvent();
impl<'a> /*trait*/ QDropEvent_FreeQDropEvent for () {
  fn FreeQDropEvent(self, this: &mut QDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QDropEventD0Ev()};
    unsafe {_ZN10QDropEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QDropEvent {
  pub fn pos<T: QDropEvent_pos>(&mut self, value: T) -> i32 {
    value.pos(self);
    return 1;
  }
}

pub trait QDropEvent_pos {
  fn pos(self, this: &mut QDropEvent) -> i32;
}

// proto: QPoint QDropEvent::pos();
impl<'a> /*trait*/ QDropEvent_pos for () {
  fn pos(self, this: &mut QDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDropEvent3posEv()};
    unsafe {_ZNK10QDropEvent3posEv()};
    return 1;
  }
}

impl /*struct*/ QDropEvent {
  pub fn source<T: QDropEvent_source>(&mut self, value: T) -> i32 {
    value.source(self);
    return 1;
  }
}

pub trait QDropEvent_source {
  fn source(self, this: &mut QDropEvent) -> i32;
}

// proto: QObject * QDropEvent::source();
impl<'a> /*trait*/ QDropEvent_source for () {
  fn source(self, this: &mut QDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDropEvent6sourceEv()};
    unsafe {_ZNK10QDropEvent6sourceEv()};
    return 1;
  }
}

impl /*struct*/ QDropEvent {
  pub fn mimeData<T: QDropEvent_mimeData>(&mut self, value: T) -> i32 {
    value.mimeData(self);
    return 1;
  }
}

pub trait QDropEvent_mimeData {
  fn mimeData(self, this: &mut QDropEvent) -> i32;
}

// proto: const QMimeData * QDropEvent::mimeData();
impl<'a> /*trait*/ QDropEvent_mimeData for () {
  fn mimeData(self, this: &mut QDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDropEvent8mimeDataEv()};
    unsafe {_ZNK10QDropEvent8mimeDataEv()};
    return 1;
  }
}

impl /*struct*/ QDropEvent {
  pub fn acceptProposedAction<T: QDropEvent_acceptProposedAction>(&mut self, value: T) -> i32 {
    value.acceptProposedAction(self);
    return 1;
  }
}

pub trait QDropEvent_acceptProposedAction {
  fn acceptProposedAction(self, this: &mut QDropEvent) -> i32;
}

// proto: void QDropEvent::acceptProposedAction();
impl<'a> /*trait*/ QDropEvent_acceptProposedAction for () {
  fn acceptProposedAction(self, this: &mut QDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QDropEvent20acceptProposedActionEv()};
    unsafe {_ZN10QDropEvent20acceptProposedActionEv()};
    return 1;
  }
}

impl /*struct*/ QDropEvent {
  pub fn posF<T: QDropEvent_posF>(&mut self, value: T) -> i32 {
    value.posF(self);
    return 1;
  }
}

pub trait QDropEvent_posF {
  fn posF(self, this: &mut QDropEvent) -> i32;
}

// proto: const QPointF & QDropEvent::posF();
impl<'a> /*trait*/ QDropEvent_posF for () {
  fn posF(self, this: &mut QDropEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDropEvent4posFEv()};
    unsafe {_ZNK10QDropEvent4posFEv()};
    return 1;
  }
}

