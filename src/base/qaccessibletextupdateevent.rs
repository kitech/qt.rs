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
  // proto: QString QAccessibleTextUpdateEvent::textInserted();
  fn _ZNK26QAccessibleTextUpdateEvent12textInsertedEv() -> i32;
  // proto: void QAccessibleTextUpdateEvent::NewQAccessibleTextUpdateEvent(QAccessibleInterface * iface, int position, const QString & oldText, const QString & text);
  fn _ZN26QAccessibleTextUpdateEventC1EP20QAccessibleInterfaceiRK7QStringS4_(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *const c_void, arg3: *const c_void) -> i32;
  // proto: QString QAccessibleTextUpdateEvent::textRemoved();
  fn _ZNK26QAccessibleTextUpdateEvent11textRemovedEv() -> i32;
  // proto: int QAccessibleTextUpdateEvent::changePosition();
  fn _ZNK26QAccessibleTextUpdateEvent14changePositionEv() -> i32;
  // proto: void QAccessibleTextUpdateEvent::NewQAccessibleTextUpdateEvent(QObject * obj, int position, const QString & oldText, const QString & text);
  fn _ZN26QAccessibleTextUpdateEventC1EP7QObjectiRK7QStringS4_(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *const c_void, arg3: *const c_void) -> i32;
}

// body block begin
// class sizeof(QAccessibleTextUpdateEvent)=56
pub struct QAccessibleTextUpdateEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleTextUpdateEvent {
  pub fn textInserted<T: QAccessibleTextUpdateEvent_textInserted>(&mut self, value: T) -> i32 {
    value.textInserted(self);
    return 1;
  }
}

pub trait QAccessibleTextUpdateEvent_textInserted {
  fn textInserted(self, this: &mut QAccessibleTextUpdateEvent) -> i32;
}

// proto: QString QAccessibleTextUpdateEvent::textInserted();
impl<'a> /*trait*/ QAccessibleTextUpdateEvent_textInserted for () {
  fn textInserted(self, this: &mut QAccessibleTextUpdateEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK26QAccessibleTextUpdateEvent12textInsertedEv()};
    unsafe {_ZNK26QAccessibleTextUpdateEvent12textInsertedEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTextUpdateEvent {
  pub fn NewQAccessibleTextUpdateEvent<T: QAccessibleTextUpdateEvent_NewQAccessibleTextUpdateEvent>(value: T) -> QAccessibleTextUpdateEvent {
    let rsthis = value.NewQAccessibleTextUpdateEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextUpdateEvent_NewQAccessibleTextUpdateEvent {
  fn NewQAccessibleTextUpdateEvent(self) -> QAccessibleTextUpdateEvent;
}

// proto: void QAccessibleTextUpdateEvent::NewQAccessibleTextUpdateEvent(QAccessibleInterface * iface, int position, const QString & oldText, const QString & text);
impl<'a> /*trait*/ QAccessibleTextUpdateEvent_NewQAccessibleTextUpdateEvent for (&'a mut QAccessibleInterface, i32, &'a  QString, &'a  QString) {
  fn NewQAccessibleTextUpdateEvent(self) -> QAccessibleTextUpdateEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN26QAccessibleTextUpdateEventC1EP20QAccessibleInterfaceiRK7QStringS4_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    unsafe {_ZN26QAccessibleTextUpdateEventC1EP20QAccessibleInterfaceiRK7QStringS4_(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QAccessibleTextUpdateEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextUpdateEvent {
  pub fn textRemoved<T: QAccessibleTextUpdateEvent_textRemoved>(&mut self, value: T) -> i32 {
    value.textRemoved(self);
    return 1;
  }
}

pub trait QAccessibleTextUpdateEvent_textRemoved {
  fn textRemoved(self, this: &mut QAccessibleTextUpdateEvent) -> i32;
}

// proto: QString QAccessibleTextUpdateEvent::textRemoved();
impl<'a> /*trait*/ QAccessibleTextUpdateEvent_textRemoved for () {
  fn textRemoved(self, this: &mut QAccessibleTextUpdateEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK26QAccessibleTextUpdateEvent11textRemovedEv()};
    unsafe {_ZNK26QAccessibleTextUpdateEvent11textRemovedEv()};
    return 1;
  }
}

impl /*struct*/ QAccessibleTextUpdateEvent {
  pub fn changePosition<T: QAccessibleTextUpdateEvent_changePosition>(&mut self, value: T) -> i32 {
    value.changePosition(self);
    return 1;
  }
}

pub trait QAccessibleTextUpdateEvent_changePosition {
  fn changePosition(self, this: &mut QAccessibleTextUpdateEvent) -> i32;
}

// proto: int QAccessibleTextUpdateEvent::changePosition();
impl<'a> /*trait*/ QAccessibleTextUpdateEvent_changePosition for () {
  fn changePosition(self, this: &mut QAccessibleTextUpdateEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK26QAccessibleTextUpdateEvent14changePositionEv()};
    unsafe {_ZNK26QAccessibleTextUpdateEvent14changePositionEv()};
    return 1;
  }
}

// proto: void QAccessibleTextUpdateEvent::NewQAccessibleTextUpdateEvent(QObject * obj, int position, const QString & oldText, const QString & text);
impl<'a> /*trait*/ QAccessibleTextUpdateEvent_NewQAccessibleTextUpdateEvent for (&'a mut QObject, i32, &'a  QString, &'a  QString) {
  fn NewQAccessibleTextUpdateEvent(self) -> QAccessibleTextUpdateEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN26QAccessibleTextUpdateEventC1EP7QObjectiRK7QStringS4_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    unsafe {_ZN26QAccessibleTextUpdateEventC1EP7QObjectiRK7QStringS4_(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QAccessibleTextUpdateEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

