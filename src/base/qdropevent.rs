// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;
use super::qobject::QObject;
use super::qmimedata::QMimeData;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QDropEvent::FreeQDropEvent();
  fn _ZN10QDropEventD0Ev(qthis: *mut c_void) ;
  // proto:  QPoint QDropEvent::pos();
  fn _ZNK10QDropEvent3posEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QObject * QDropEvent::source();
  fn _ZNK10QDropEvent6sourceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMimeData * QDropEvent::mimeData();
  fn _ZNK10QDropEvent8mimeDataEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDropEvent::acceptProposedAction();
  fn _ZN10QDropEvent20acceptProposedActionEv(qthis: *mut c_void) ;
  // proto:  const QPointF & QDropEvent::posF();
  fn _ZNK10QDropEvent4posFEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QDropEvent)=1
pub struct QDropEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDropEvent {
  pub fn FreeQDropEvent<T: QDropEvent_FreeQDropEvent>(&mut self, value: T)  {
     value.FreeQDropEvent(self);
    // return 1;
  }
}

pub trait QDropEvent_FreeQDropEvent {
  fn FreeQDropEvent(self, rsthis: &mut QDropEvent) ;
}

// proto:  void QDropEvent::FreeQDropEvent();
impl<'a> /*trait*/ QDropEvent_FreeQDropEvent for () {
  fn FreeQDropEvent(self, rsthis: &mut QDropEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QDropEventD0Ev()};
     unsafe {_ZN10QDropEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDropEvent {
  pub fn pos<T: QDropEvent_pos>(&mut self, value: T) -> QPoint {
    return value.pos(self);
    // return 1;
  }
}

pub trait QDropEvent_pos {
  fn pos(self, rsthis: &mut QDropEvent) -> QPoint;
}

// proto:  QPoint QDropEvent::pos();
impl<'a> /*trait*/ QDropEvent_pos for () {
  fn pos(self, rsthis: &mut QDropEvent) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDropEvent3posEv()};
    let mut ret = unsafe {_ZNK10QDropEvent3posEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDropEvent {
  pub fn source<T: QDropEvent_source>(&mut self, value: T) -> QObject {
    return value.source(self);
    // return 1;
  }
}

pub trait QDropEvent_source {
  fn source(self, rsthis: &mut QDropEvent) -> QObject;
}

// proto:  QObject * QDropEvent::source();
impl<'a> /*trait*/ QDropEvent_source for () {
  fn source(self, rsthis: &mut QDropEvent) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDropEvent6sourceEv()};
    let mut ret = unsafe {_ZNK10QDropEvent6sourceEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDropEvent {
  pub fn mimeData<T: QDropEvent_mimeData>(&mut self, value: T) -> QMimeData {
    return value.mimeData(self);
    // return 1;
  }
}

pub trait QDropEvent_mimeData {
  fn mimeData(self, rsthis: &mut QDropEvent) -> QMimeData;
}

// proto:  const QMimeData * QDropEvent::mimeData();
impl<'a> /*trait*/ QDropEvent_mimeData for () {
  fn mimeData(self, rsthis: &mut QDropEvent) -> QMimeData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDropEvent8mimeDataEv()};
    let mut ret = unsafe {_ZNK10QDropEvent8mimeDataEv(rsthis.qclsinst)};
    let mut ret1 = QMimeData{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDropEvent {
  pub fn acceptProposedAction<T: QDropEvent_acceptProposedAction>(&mut self, value: T)  {
     value.acceptProposedAction(self);
    // return 1;
  }
}

pub trait QDropEvent_acceptProposedAction {
  fn acceptProposedAction(self, rsthis: &mut QDropEvent) ;
}

// proto:  void QDropEvent::acceptProposedAction();
impl<'a> /*trait*/ QDropEvent_acceptProposedAction for () {
  fn acceptProposedAction(self, rsthis: &mut QDropEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QDropEvent20acceptProposedActionEv()};
     unsafe {_ZN10QDropEvent20acceptProposedActionEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDropEvent {
  pub fn posF<T: QDropEvent_posF>(&mut self, value: T) -> QPointF {
    return value.posF(self);
    // return 1;
  }
}

pub trait QDropEvent_posF {
  fn posF(self, rsthis: &mut QDropEvent) -> QPointF;
}

// proto:  const QPointF & QDropEvent::posF();
impl<'a> /*trait*/ QDropEvent_posF for () {
  fn posF(self, rsthis: &mut QDropEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QDropEvent4posFEv()};
    let mut ret = unsafe {_ZNK10QDropEvent4posFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

