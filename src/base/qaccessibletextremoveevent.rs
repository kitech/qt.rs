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
  // proto: void QAccessibleTextRemoveEvent::NewQAccessibleTextRemoveEvent(QObject * obj, int position, const QString & text);
  fn _ZN26QAccessibleTextRemoveEventC1EP7QObjectiRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *const c_void) -> i32;
  // proto: QString QAccessibleTextRemoveEvent::textRemoved();
  fn _ZNK26QAccessibleTextRemoveEvent11textRemovedEv() -> i32;
  // proto: void QAccessibleTextRemoveEvent::NewQAccessibleTextRemoveEvent(QAccessibleInterface * iface, int position, const QString & text);
  fn _ZN26QAccessibleTextRemoveEventC1EP20QAccessibleInterfaceiRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *const c_void) -> i32;
  // proto: int QAccessibleTextRemoveEvent::changePosition();
  fn _ZNK26QAccessibleTextRemoveEvent14changePositionEv() -> i32;
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
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN26QAccessibleTextRemoveEventC1EP7QObjectiRK7QString(qthis, arg0, arg1, arg2)};
    let rsthis = QAccessibleTextRemoveEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextRemoveEvent {
  pub fn textRemoved<T: QAccessibleTextRemoveEvent_textRemoved>(&mut self, value: T) -> i32 {
    value.textRemoved(self);
    return 1;
  }
}

pub trait QAccessibleTextRemoveEvent_textRemoved {
  fn textRemoved(self, this: &mut QAccessibleTextRemoveEvent) -> i32;
}

// proto: QString QAccessibleTextRemoveEvent::textRemoved();
impl<'a> /*trait*/ QAccessibleTextRemoveEvent_textRemoved for () {
  fn textRemoved(self, this: &mut QAccessibleTextRemoveEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK26QAccessibleTextRemoveEvent11textRemovedEv()};
    unsafe {_ZNK26QAccessibleTextRemoveEvent11textRemovedEv()};
    return 1;
  }
}

// proto: void QAccessibleTextRemoveEvent::NewQAccessibleTextRemoveEvent(QAccessibleInterface * iface, int position, const QString & text);
impl<'a> /*trait*/ QAccessibleTextRemoveEvent_NewQAccessibleTextRemoveEvent for (&'a mut QAccessibleInterface, i32, &'a  QString) {
  fn NewQAccessibleTextRemoveEvent(self) -> QAccessibleTextRemoveEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN26QAccessibleTextRemoveEventC1EP20QAccessibleInterfaceiRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN26QAccessibleTextRemoveEventC1EP20QAccessibleInterfaceiRK7QString(qthis, arg0, arg1, arg2)};
    let rsthis = QAccessibleTextRemoveEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextRemoveEvent {
  pub fn changePosition<T: QAccessibleTextRemoveEvent_changePosition>(&mut self, value: T) -> i32 {
    value.changePosition(self);
    return 1;
  }
}

pub trait QAccessibleTextRemoveEvent_changePosition {
  fn changePosition(self, this: &mut QAccessibleTextRemoveEvent) -> i32;
}

// proto: int QAccessibleTextRemoveEvent::changePosition();
impl<'a> /*trait*/ QAccessibleTextRemoveEvent_changePosition for () {
  fn changePosition(self, this: &mut QAccessibleTextRemoveEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK26QAccessibleTextRemoveEvent14changePositionEv()};
    unsafe {_ZNK26QAccessibleTextRemoveEvent14changePositionEv()};
    return 1;
  }
}

