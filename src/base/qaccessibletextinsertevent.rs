// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qaccessibleinterface::QAccessibleInterface;
use super::qstring::QString;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QString QAccessibleTextInsertEvent::textInserted();
  fn _ZNK26QAccessibleTextInsertEvent12textInsertedEv() -> i32;
  // proto: int QAccessibleTextInsertEvent::changePosition();
  fn _ZNK26QAccessibleTextInsertEvent14changePositionEv() -> i32;
  // proto: void QAccessibleTextInsertEvent::NewQAccessibleTextInsertEvent(QAccessibleInterface * iface, int position, const QString & text);
  fn _ZN26QAccessibleTextInsertEventC1EP20QAccessibleInterfaceiRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *const c_void) -> i32;
  // proto: void QAccessibleTextInsertEvent::NewQAccessibleTextInsertEvent(QObject * obj, int position, const QString & text);
  fn _ZN26QAccessibleTextInsertEventC1EP7QObjectiRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *const c_void) -> i32;
}

// body block begin
// class sizeof(QAccessibleTextInsertEvent)=48
pub struct QAccessibleTextInsertEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleTextInsertEvent {
  pub fn textInserted<T: QAccessibleTextInsertEvent_textInserted>(&mut self, value: T) -> i32 {
    value.textInserted(self);
    return 1;
  }
}

pub trait QAccessibleTextInsertEvent_textInserted {
  fn textInserted(self, this: &mut QAccessibleTextInsertEvent) -> i32;
}

// proto: QString QAccessibleTextInsertEvent::textInserted();
impl<'a> /*trait*/ QAccessibleTextInsertEvent_textInserted for () {
  fn textInserted(self, this: &mut QAccessibleTextInsertEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK26QAccessibleTextInsertEvent12textInsertedEv()};
    unsafe {_ZNK26QAccessibleTextInsertEvent12textInsertedEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTextInsertEvent {
  pub fn changePosition<T: QAccessibleTextInsertEvent_changePosition>(&mut self, value: T) -> i32 {
    value.changePosition(self);
    return 1;
  }
}

pub trait QAccessibleTextInsertEvent_changePosition {
  fn changePosition(self, this: &mut QAccessibleTextInsertEvent) -> i32;
}

// proto: int QAccessibleTextInsertEvent::changePosition();
impl<'a> /*trait*/ QAccessibleTextInsertEvent_changePosition for () {
  fn changePosition(self, this: &mut QAccessibleTextInsertEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK26QAccessibleTextInsertEvent14changePositionEv()};
    unsafe {_ZNK26QAccessibleTextInsertEvent14changePositionEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTextInsertEvent {
  pub fn NewQAccessibleTextInsertEvent<T: QAccessibleTextInsertEvent_NewQAccessibleTextInsertEvent>(value: T) -> QAccessibleTextInsertEvent {
    let rsthis = value.NewQAccessibleTextInsertEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextInsertEvent_NewQAccessibleTextInsertEvent {
  fn NewQAccessibleTextInsertEvent(self) -> QAccessibleTextInsertEvent;
}

// proto: void QAccessibleTextInsertEvent::NewQAccessibleTextInsertEvent(QAccessibleInterface * iface, int position, const QString & text);
impl<'a> /*trait*/ QAccessibleTextInsertEvent_NewQAccessibleTextInsertEvent for (&'a mut QAccessibleInterface, i32, &'a  QString) {
  fn NewQAccessibleTextInsertEvent(self) -> QAccessibleTextInsertEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN26QAccessibleTextInsertEventC1EP20QAccessibleInterfaceiRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN26QAccessibleTextInsertEventC1EP20QAccessibleInterfaceiRK7QString(qthis, arg0, arg1, arg2)};
    let rsthis = QAccessibleTextInsertEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QAccessibleTextInsertEvent::NewQAccessibleTextInsertEvent(QObject * obj, int position, const QString & text);
impl<'a> /*trait*/ QAccessibleTextInsertEvent_NewQAccessibleTextInsertEvent for (&'a mut QObject, i32, &'a  QString) {
  fn NewQAccessibleTextInsertEvent(self) -> QAccessibleTextInsertEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN26QAccessibleTextInsertEventC1EP7QObjectiRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN26QAccessibleTextInsertEventC1EP7QObjectiRK7QString(qthis, arg0, arg1, arg2)};
    let rsthis = QAccessibleTextInsertEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

