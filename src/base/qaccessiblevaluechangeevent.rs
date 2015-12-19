// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qvariant::QVariant;
use super::qaccessibleinterface::QAccessibleInterface;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QAccessibleValueChangeEvent::NewQAccessibleValueChangeEvent(QObject * obj, const QVariant & val);
  fn _ZN27QAccessibleValueChangeEventC1EP7QObjectRK8QVariant(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QAccessibleValueChangeEvent::NewQAccessibleValueChangeEvent(QAccessibleInterface * iface, const QVariant & val);
  fn _ZN27QAccessibleValueChangeEventC1EP20QAccessibleInterfaceRK8QVariant(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QAccessibleValueChangeEvent::setValue(const QVariant & val);
  fn _ZN27QAccessibleValueChangeEvent8setValueERK8QVariant(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QVariant QAccessibleValueChangeEvent::value();
  fn _ZNK27QAccessibleValueChangeEvent5valueEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QAccessibleValueChangeEvent)=48
pub struct QAccessibleValueChangeEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleValueChangeEvent {
  pub fn NewQAccessibleValueChangeEvent<T: QAccessibleValueChangeEvent_NewQAccessibleValueChangeEvent>(value: T) -> QAccessibleValueChangeEvent {
    let rsthis = value.NewQAccessibleValueChangeEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleValueChangeEvent_NewQAccessibleValueChangeEvent {
  fn NewQAccessibleValueChangeEvent(self) -> QAccessibleValueChangeEvent;
}

// proto: void QAccessibleValueChangeEvent::NewQAccessibleValueChangeEvent(QObject * obj, const QVariant & val);
impl<'a> /*trait*/ QAccessibleValueChangeEvent_NewQAccessibleValueChangeEvent for (&'a mut QObject, &'a  QVariant) {
  fn NewQAccessibleValueChangeEvent(self) -> QAccessibleValueChangeEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN27QAccessibleValueChangeEventC1EP7QObjectRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN27QAccessibleValueChangeEventC1EP7QObjectRK8QVariant(qthis, arg0, arg1)};
    let rsthis = QAccessibleValueChangeEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QAccessibleValueChangeEvent::NewQAccessibleValueChangeEvent(QAccessibleInterface * iface, const QVariant & val);
impl<'a> /*trait*/ QAccessibleValueChangeEvent_NewQAccessibleValueChangeEvent for (&'a mut QAccessibleInterface, &'a  QVariant) {
  fn NewQAccessibleValueChangeEvent(self) -> QAccessibleValueChangeEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN27QAccessibleValueChangeEventC1EP20QAccessibleInterfaceRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN27QAccessibleValueChangeEventC1EP20QAccessibleInterfaceRK8QVariant(qthis, arg0, arg1)};
    let rsthis = QAccessibleValueChangeEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QAccessibleValueChangeEvent::setValue(const QVariant & val);
impl /*struct*/ QAccessibleValueChangeEvent {
  pub fn setValue<RetType, T: QAccessibleValueChangeEvent_setValue<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setValue(self);
    // return 1;
  }
}

pub trait QAccessibleValueChangeEvent_setValue<RetType> {
  fn setValue(self , rsthis: &mut QAccessibleValueChangeEvent) -> RetType;
}

// proto:  void QAccessibleValueChangeEvent::setValue(const QVariant & val);
impl<'a> /*trait*/ QAccessibleValueChangeEvent_setValue<()> for (&'a  QVariant) {
  fn setValue(self , rsthis: &mut QAccessibleValueChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN27QAccessibleValueChangeEvent8setValueERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QAccessibleValueChangeEvent8setValueERK8QVariant(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QVariant QAccessibleValueChangeEvent::value();
impl /*struct*/ QAccessibleValueChangeEvent {
  pub fn value<RetType, T: QAccessibleValueChangeEvent_value<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QAccessibleValueChangeEvent_value<RetType> {
  fn value(self , rsthis: &mut QAccessibleValueChangeEvent) -> RetType;
}

// proto:  QVariant QAccessibleValueChangeEvent::value();
impl<'a> /*trait*/ QAccessibleValueChangeEvent_value<QVariant> for () {
  fn value(self , rsthis: &mut QAccessibleValueChangeEvent) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK27QAccessibleValueChangeEvent5valueEv()};
    let mut ret = unsafe {_ZNK27QAccessibleValueChangeEvent5valueEv(rsthis.qclsinst)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

