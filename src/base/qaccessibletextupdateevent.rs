// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qaccessibleinterface::QAccessibleInterface;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QString QAccessibleTextUpdateEvent::textInserted();
  fn _ZNK26QAccessibleTextUpdateEvent12textInsertedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleTextUpdateEvent::NewQAccessibleTextUpdateEvent(QAccessibleInterface * iface, int position, const QString & oldText, const QString & text);
  fn _ZN26QAccessibleTextUpdateEventC1EP20QAccessibleInterfaceiRK7QStringS4_(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void, arg3: *mut c_void) ;
  // proto:  QString QAccessibleTextUpdateEvent::textRemoved();
  fn _ZNK26QAccessibleTextUpdateEvent11textRemovedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QAccessibleTextUpdateEvent::changePosition();
  fn _ZNK26QAccessibleTextUpdateEvent14changePositionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTextUpdateEvent::NewQAccessibleTextUpdateEvent(QObject * obj, int position, const QString & oldText, const QString & text);
  fn _ZN26QAccessibleTextUpdateEventC1EP7QObjectiRK7QStringS4_(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void, arg3: *mut c_void) ;
}

// body block begin
// class sizeof(QAccessibleTextUpdateEvent)=56
pub struct QAccessibleTextUpdateEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleTextUpdateEvent {
  pub fn textInserted<RetType, T: QAccessibleTextUpdateEvent_textInserted<RetType>>(&mut self, value: T) -> RetType {
    return value.textInserted(self);
    // return 1;
  }
}

pub trait QAccessibleTextUpdateEvent_textInserted<RetType> {
  fn textInserted(self, rsthis: &mut QAccessibleTextUpdateEvent) -> RetType;
}

// proto:  QString QAccessibleTextUpdateEvent::textInserted();
impl<'a> /*trait*/ QAccessibleTextUpdateEvent_textInserted<QString> for () {
  fn textInserted(self, rsthis: &mut QAccessibleTextUpdateEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK26QAccessibleTextUpdateEvent12textInsertedEv()};
    let mut ret = unsafe {_ZNK26QAccessibleTextUpdateEvent12textInsertedEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
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
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZN26QAccessibleTextUpdateEventC1EP20QAccessibleInterfaceiRK7QStringS4_(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QAccessibleTextUpdateEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextUpdateEvent {
  pub fn textRemoved<RetType, T: QAccessibleTextUpdateEvent_textRemoved<RetType>>(&mut self, value: T) -> RetType {
    return value.textRemoved(self);
    // return 1;
  }
}

pub trait QAccessibleTextUpdateEvent_textRemoved<RetType> {
  fn textRemoved(self, rsthis: &mut QAccessibleTextUpdateEvent) -> RetType;
}

// proto:  QString QAccessibleTextUpdateEvent::textRemoved();
impl<'a> /*trait*/ QAccessibleTextUpdateEvent_textRemoved<QString> for () {
  fn textRemoved(self, rsthis: &mut QAccessibleTextUpdateEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK26QAccessibleTextUpdateEvent11textRemovedEv()};
    let mut ret = unsafe {_ZNK26QAccessibleTextUpdateEvent11textRemovedEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextUpdateEvent {
  pub fn changePosition<RetType, T: QAccessibleTextUpdateEvent_changePosition<RetType>>(&mut self, value: T) -> RetType {
    return value.changePosition(self);
    // return 1;
  }
}

pub trait QAccessibleTextUpdateEvent_changePosition<RetType> {
  fn changePosition(self, rsthis: &mut QAccessibleTextUpdateEvent) -> RetType;
}

// proto:  int QAccessibleTextUpdateEvent::changePosition();
impl<'a> /*trait*/ QAccessibleTextUpdateEvent_changePosition<i32> for () {
  fn changePosition(self, rsthis: &mut QAccessibleTextUpdateEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK26QAccessibleTextUpdateEvent14changePositionEv()};
    let mut ret = unsafe {_ZNK26QAccessibleTextUpdateEvent14changePositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QAccessibleTextUpdateEvent::NewQAccessibleTextUpdateEvent(QObject * obj, int position, const QString & oldText, const QString & text);
impl<'a> /*trait*/ QAccessibleTextUpdateEvent_NewQAccessibleTextUpdateEvent for (&'a mut QObject, i32, &'a  QString, &'a  QString) {
  fn NewQAccessibleTextUpdateEvent(self) -> QAccessibleTextUpdateEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN26QAccessibleTextUpdateEventC1EP7QObjectiRK7QStringS4_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZN26QAccessibleTextUpdateEventC1EP7QObjectiRK7QStringS4_(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QAccessibleTextUpdateEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

