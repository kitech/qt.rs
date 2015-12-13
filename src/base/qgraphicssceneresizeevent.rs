// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsizef::QSizeF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QSizeF QGraphicsSceneResizeEvent::newSize();
  fn _ZNK25QGraphicsSceneResizeEvent7newSizeEv() -> i32;
  // proto: QSizeF QGraphicsSceneResizeEvent::oldSize();
  fn _ZNK25QGraphicsSceneResizeEvent7oldSizeEv() -> i32;
  // proto: void QGraphicsSceneResizeEvent::FreeQGraphicsSceneResizeEvent();
  fn _ZN25QGraphicsSceneResizeEventD0Ev() -> i32;
  // proto: void QGraphicsSceneResizeEvent::setNewSize(const QSizeF & size);
  fn _ZN25QGraphicsSceneResizeEvent10setNewSizeERK6QSizeF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsSceneResizeEvent::NewQGraphicsSceneResizeEvent();
  fn _ZN25QGraphicsSceneResizeEventC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QGraphicsSceneResizeEvent::setOldSize(const QSizeF & size);
  fn _ZN25QGraphicsSceneResizeEvent10setOldSizeERK6QSizeF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsSceneResizeEvent::NewQGraphicsSceneResizeEvent(const QGraphicsSceneResizeEvent & );
  fn _ZN25QGraphicsSceneResizeEventC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QGraphicsSceneResizeEvent)=1
pub struct QGraphicsSceneResizeEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn newSize<T: QGraphicsSceneResizeEvent_newSize>(&mut self, value: T) -> i32 {
    value.newSize(self);
    return 1;
  }
}

pub trait QGraphicsSceneResizeEvent_newSize {
  fn newSize(self, this: &mut QGraphicsSceneResizeEvent) -> i32;
}

// proto: QSizeF QGraphicsSceneResizeEvent::newSize();
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_newSize for () {
  fn newSize(self, this: &mut QGraphicsSceneResizeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsSceneResizeEvent7newSizeEv()};
    unsafe {_ZNK25QGraphicsSceneResizeEvent7newSizeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn oldSize<T: QGraphicsSceneResizeEvent_oldSize>(&mut self, value: T) -> i32 {
    value.oldSize(self);
    return 1;
  }
}

pub trait QGraphicsSceneResizeEvent_oldSize {
  fn oldSize(self, this: &mut QGraphicsSceneResizeEvent) -> i32;
}

// proto: QSizeF QGraphicsSceneResizeEvent::oldSize();
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_oldSize for () {
  fn oldSize(self, this: &mut QGraphicsSceneResizeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsSceneResizeEvent7oldSizeEv()};
    unsafe {_ZNK25QGraphicsSceneResizeEvent7oldSizeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn FreeQGraphicsSceneResizeEvent<T: QGraphicsSceneResizeEvent_FreeQGraphicsSceneResizeEvent>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsSceneResizeEvent(self);
    return 1;
  }
}

pub trait QGraphicsSceneResizeEvent_FreeQGraphicsSceneResizeEvent {
  fn FreeQGraphicsSceneResizeEvent(self, this: &mut QGraphicsSceneResizeEvent) -> i32;
}

// proto: void QGraphicsSceneResizeEvent::FreeQGraphicsSceneResizeEvent();
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_FreeQGraphicsSceneResizeEvent for () {
  fn FreeQGraphicsSceneResizeEvent(self, this: &mut QGraphicsSceneResizeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsSceneResizeEventD0Ev()};
    unsafe {_ZN25QGraphicsSceneResizeEventD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn setNewSize<T: QGraphicsSceneResizeEvent_setNewSize>(&mut self, value: T) -> i32 {
    value.setNewSize(self);
    return 1;
  }
}

pub trait QGraphicsSceneResizeEvent_setNewSize {
  fn setNewSize(self, this: &mut QGraphicsSceneResizeEvent) -> i32;
}

// proto: void QGraphicsSceneResizeEvent::setNewSize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_setNewSize for (&'a  QSizeF) {
  fn setNewSize(self, this: &mut QGraphicsSceneResizeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsSceneResizeEvent10setNewSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN25QGraphicsSceneResizeEvent10setNewSizeERK6QSizeF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn NewQGraphicsSceneResizeEvent<T: QGraphicsSceneResizeEvent_NewQGraphicsSceneResizeEvent>(value: T) -> QGraphicsSceneResizeEvent {
    let rsthis = value.NewQGraphicsSceneResizeEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSceneResizeEvent_NewQGraphicsSceneResizeEvent {
  fn NewQGraphicsSceneResizeEvent(self) -> QGraphicsSceneResizeEvent;
}

// proto: void QGraphicsSceneResizeEvent::NewQGraphicsSceneResizeEvent();
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_NewQGraphicsSceneResizeEvent for () {
  fn NewQGraphicsSceneResizeEvent(self) -> QGraphicsSceneResizeEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsSceneResizeEventC1Ev()};
    unsafe {_ZN25QGraphicsSceneResizeEventC1Ev(qthis)};
    let rsthis = QGraphicsSceneResizeEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn setOldSize<T: QGraphicsSceneResizeEvent_setOldSize>(&mut self, value: T) -> i32 {
    value.setOldSize(self);
    return 1;
  }
}

pub trait QGraphicsSceneResizeEvent_setOldSize {
  fn setOldSize(self, this: &mut QGraphicsSceneResizeEvent) -> i32;
}

// proto: void QGraphicsSceneResizeEvent::setOldSize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_setOldSize for (&'a  QSizeF) {
  fn setOldSize(self, this: &mut QGraphicsSceneResizeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsSceneResizeEvent10setOldSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN25QGraphicsSceneResizeEvent10setOldSizeERK6QSizeF(arg0)};
    return 1;
  }
}

// proto: void QGraphicsSceneResizeEvent::NewQGraphicsSceneResizeEvent(const QGraphicsSceneResizeEvent & );
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_NewQGraphicsSceneResizeEvent for (&'a  QGraphicsSceneResizeEvent) {
  fn NewQGraphicsSceneResizeEvent(self) -> QGraphicsSceneResizeEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsSceneResizeEventC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN25QGraphicsSceneResizeEventC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsSceneResizeEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

