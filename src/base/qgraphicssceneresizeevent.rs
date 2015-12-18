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
  // proto:  QSizeF QGraphicsSceneResizeEvent::newSize();
  fn _ZNK25QGraphicsSceneResizeEvent7newSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSizeF QGraphicsSceneResizeEvent::oldSize();
  fn _ZNK25QGraphicsSceneResizeEvent7oldSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsSceneResizeEvent::FreeQGraphicsSceneResizeEvent();
  fn _ZN25QGraphicsSceneResizeEventD0Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsSceneResizeEvent::setNewSize(const QSizeF & size);
  fn _ZN25QGraphicsSceneResizeEvent10setNewSizeERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsSceneResizeEvent::NewQGraphicsSceneResizeEvent();
  fn _ZN25QGraphicsSceneResizeEventC1Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsSceneResizeEvent::setOldSize(const QSizeF & size);
  fn _ZN25QGraphicsSceneResizeEvent10setOldSizeERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsSceneResizeEvent::NewQGraphicsSceneResizeEvent(const QGraphicsSceneResizeEvent & );
  fn _ZN25QGraphicsSceneResizeEventC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QGraphicsSceneResizeEvent)=1
pub struct QGraphicsSceneResizeEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn newSize<RetType, T: QGraphicsSceneResizeEvent_newSize<RetType>>(&mut self, value: T) -> RetType {
    return value.newSize(self);
    // return 1;
  }
}

pub trait QGraphicsSceneResizeEvent_newSize<RetType> {
  fn newSize(self, rsthis: &mut QGraphicsSceneResizeEvent) -> RetType;
}

// proto:  QSizeF QGraphicsSceneResizeEvent::newSize();
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_newSize<QSizeF> for () {
  fn newSize(self, rsthis: &mut QGraphicsSceneResizeEvent) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsSceneResizeEvent7newSizeEv()};
    let mut ret = unsafe {_ZNK25QGraphicsSceneResizeEvent7newSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn oldSize<RetType, T: QGraphicsSceneResizeEvent_oldSize<RetType>>(&mut self, value: T) -> RetType {
    return value.oldSize(self);
    // return 1;
  }
}

pub trait QGraphicsSceneResizeEvent_oldSize<RetType> {
  fn oldSize(self, rsthis: &mut QGraphicsSceneResizeEvent) -> RetType;
}

// proto:  QSizeF QGraphicsSceneResizeEvent::oldSize();
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_oldSize<QSizeF> for () {
  fn oldSize(self, rsthis: &mut QGraphicsSceneResizeEvent) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsSceneResizeEvent7oldSizeEv()};
    let mut ret = unsafe {_ZNK25QGraphicsSceneResizeEvent7oldSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn FreeQGraphicsSceneResizeEvent<RetType, T: QGraphicsSceneResizeEvent_FreeQGraphicsSceneResizeEvent<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQGraphicsSceneResizeEvent(self);
    // return 1;
  }
}

pub trait QGraphicsSceneResizeEvent_FreeQGraphicsSceneResizeEvent<RetType> {
  fn FreeQGraphicsSceneResizeEvent(self, rsthis: &mut QGraphicsSceneResizeEvent) -> RetType;
}

// proto:  void QGraphicsSceneResizeEvent::FreeQGraphicsSceneResizeEvent();
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_FreeQGraphicsSceneResizeEvent<()> for () {
  fn FreeQGraphicsSceneResizeEvent(self, rsthis: &mut QGraphicsSceneResizeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsSceneResizeEventD0Ev()};
     unsafe {_ZN25QGraphicsSceneResizeEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsSceneResizeEvent {
  pub fn setNewSize<RetType, T: QGraphicsSceneResizeEvent_setNewSize<RetType>>(&mut self, value: T) -> RetType {
    return value.setNewSize(self);
    // return 1;
  }
}

pub trait QGraphicsSceneResizeEvent_setNewSize<RetType> {
  fn setNewSize(self, rsthis: &mut QGraphicsSceneResizeEvent) -> RetType;
}

// proto:  void QGraphicsSceneResizeEvent::setNewSize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_setNewSize<()> for (&'a  QSizeF) {
  fn setNewSize(self, rsthis: &mut QGraphicsSceneResizeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsSceneResizeEvent10setNewSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN25QGraphicsSceneResizeEvent10setNewSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
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
  pub fn setOldSize<RetType, T: QGraphicsSceneResizeEvent_setOldSize<RetType>>(&mut self, value: T) -> RetType {
    return value.setOldSize(self);
    // return 1;
  }
}

pub trait QGraphicsSceneResizeEvent_setOldSize<RetType> {
  fn setOldSize(self, rsthis: &mut QGraphicsSceneResizeEvent) -> RetType;
}

// proto:  void QGraphicsSceneResizeEvent::setOldSize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_setOldSize<()> for (&'a  QSizeF) {
  fn setOldSize(self, rsthis: &mut QGraphicsSceneResizeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsSceneResizeEvent10setOldSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN25QGraphicsSceneResizeEvent10setOldSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QGraphicsSceneResizeEvent::NewQGraphicsSceneResizeEvent(const QGraphicsSceneResizeEvent & );
impl<'a> /*trait*/ QGraphicsSceneResizeEvent_NewQGraphicsSceneResizeEvent for (&'a  QGraphicsSceneResizeEvent) {
  fn NewQGraphicsSceneResizeEvent(self) -> QGraphicsSceneResizeEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsSceneResizeEventC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN25QGraphicsSceneResizeEventC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsSceneResizeEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

