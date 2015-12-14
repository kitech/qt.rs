// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qstring::QString;
use super::qaccessibleinterface::QAccessibleInterface;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QAccessibleTextRemoveEvent::NewQAccessibleTextRemoveEvent(QObject * obj, int position, const QString & text);
  fn _ZN26QAccessibleTextRemoveEventC1EP7QObjectiRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) ;
  // proto:  QString QAccessibleTextRemoveEvent::textRemoved();
  fn _ZNK26QAccessibleTextRemoveEvent11textRemovedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleTextRemoveEvent::NewQAccessibleTextRemoveEvent(QAccessibleInterface * iface, int position, const QString & text);
  fn _ZN26QAccessibleTextRemoveEventC1EP20QAccessibleInterfaceiRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) ;
  // proto:  int QAccessibleTextRemoveEvent::changePosition();
  fn _ZNK26QAccessibleTextRemoveEvent14changePositionEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QAccessibleTextRemoveEvent)=48
pub struct QAccessibleTextRemoveEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleTextRemoveEvent {
  pub fn NewQAccessibleTextRemoveEvent<T: QAccessibleTextRemoveEvent_NewQAccessibleTextRemoveEvent>(value: T) -> QAccessibleTextRemoveEvent {
    let rsthis = value.NewQAccessibleTextRemoveEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextRemoveEvent_NewQAccessibleTextRemoveEvent {
  fn NewQAccessibleTextRemoveEvent(self) -> QAccessibleTextRemoveEvent;
}

// proto: void QAccessibleTextRemoveEvent::NewQAccessibleTextRemoveEvent(QObject * obj, int position, const QString & text);
impl<'a> /*trait*/ QAccessibleTextRemoveEvent_NewQAccessibleTextRemoveEvent for (&'a mut QObject, i32, &'a  QString) {
  fn NewQAccessibleTextRemoveEvent(self) -> QAccessibleTextRemoveEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN26QAccessibleTextRemoveEventC1EP7QObjectiRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN26QAccessibleTextRemoveEventC1EP7QObjectiRK7QString(qthis, arg0, arg1, arg2)};
    let rsthis = QAccessibleTextRemoveEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextRemoveEvent {
  pub fn textRemoved<T: QAccessibleTextRemoveEvent_textRemoved>(&mut self, value: T) -> QString {
    return value.textRemoved(self);
    // return 1;
  }
}

pub trait QAccessibleTextRemoveEvent_textRemoved {
  fn textRemoved(self, rsthis: &mut QAccessibleTextRemoveEvent) -> QString;
}

// proto:  QString QAccessibleTextRemoveEvent::textRemoved();
impl<'a> /*trait*/ QAccessibleTextRemoveEvent_textRemoved for () {
  fn textRemoved(self, rsthis: &mut QAccessibleTextRemoveEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK26QAccessibleTextRemoveEvent11textRemovedEv()};
    let mut ret = unsafe {_ZNK26QAccessibleTextRemoveEvent11textRemovedEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QAccessibleTextRemoveEvent::NewQAccessibleTextRemoveEvent(QAccessibleInterface * iface, int position, const QString & text);
impl<'a> /*trait*/ QAccessibleTextRemoveEvent_NewQAccessibleTextRemoveEvent for (&'a mut QAccessibleInterface, i32, &'a  QString) {
  fn NewQAccessibleTextRemoveEvent(self) -> QAccessibleTextRemoveEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN26QAccessibleTextRemoveEventC1EP20QAccessibleInterfaceiRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN26QAccessibleTextRemoveEventC1EP20QAccessibleInterfaceiRK7QString(qthis, arg0, arg1, arg2)};
    let rsthis = QAccessibleTextRemoveEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextRemoveEvent {
  pub fn changePosition<T: QAccessibleTextRemoveEvent_changePosition>(&mut self, value: T) -> i32 {
    return value.changePosition(self);
    // return 1;
  }
}

pub trait QAccessibleTextRemoveEvent_changePosition {
  fn changePosition(self, rsthis: &mut QAccessibleTextRemoveEvent) -> i32;
}

// proto:  int QAccessibleTextRemoveEvent::changePosition();
impl<'a> /*trait*/ QAccessibleTextRemoveEvent_changePosition for () {
  fn changePosition(self, rsthis: &mut QAccessibleTextRemoveEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK26QAccessibleTextRemoveEvent14changePositionEv()};
    let mut ret = unsafe {_ZNK26QAccessibleTextRemoveEvent14changePositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

