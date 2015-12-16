// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QDynamicPropertyChangeEvent::FreeQDynamicPropertyChangeEvent();
  fn _ZN27QDynamicPropertyChangeEventD0Ev(qthis: *mut c_void) ;
  // proto:  void QDynamicPropertyChangeEvent::NewQDynamicPropertyChangeEvent(const QByteArray & name);
  fn _ZN27QDynamicPropertyChangeEventC1ERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QByteArray QDynamicPropertyChangeEvent::propertyName();
  fn _ZNK27QDynamicPropertyChangeEvent12propertyNameEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QDynamicPropertyChangeEvent)=32
pub struct QDynamicPropertyChangeEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDynamicPropertyChangeEvent {
  pub fn FreeQDynamicPropertyChangeEvent<T: QDynamicPropertyChangeEvent_FreeQDynamicPropertyChangeEvent>(&mut self, value: T)  {
     value.FreeQDynamicPropertyChangeEvent(self);
    // return 1;
  }
}

pub trait QDynamicPropertyChangeEvent_FreeQDynamicPropertyChangeEvent {
  fn FreeQDynamicPropertyChangeEvent(self, rsthis: &mut QDynamicPropertyChangeEvent) ;
}

// proto:  void QDynamicPropertyChangeEvent::FreeQDynamicPropertyChangeEvent();
impl<'a> /*trait*/ QDynamicPropertyChangeEvent_FreeQDynamicPropertyChangeEvent for () {
  fn FreeQDynamicPropertyChangeEvent(self, rsthis: &mut QDynamicPropertyChangeEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QDynamicPropertyChangeEventD0Ev()};
     unsafe {_ZN27QDynamicPropertyChangeEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDynamicPropertyChangeEvent {
  pub fn NewQDynamicPropertyChangeEvent<T: QDynamicPropertyChangeEvent_NewQDynamicPropertyChangeEvent>(value: T) -> QDynamicPropertyChangeEvent {
    let rsthis = value.NewQDynamicPropertyChangeEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QDynamicPropertyChangeEvent_NewQDynamicPropertyChangeEvent {
  fn NewQDynamicPropertyChangeEvent(self) -> QDynamicPropertyChangeEvent;
}

// proto: void QDynamicPropertyChangeEvent::NewQDynamicPropertyChangeEvent(const QByteArray & name);
impl<'a> /*trait*/ QDynamicPropertyChangeEvent_NewQDynamicPropertyChangeEvent for (&'a  QByteArray) {
  fn NewQDynamicPropertyChangeEvent(self) -> QDynamicPropertyChangeEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QDynamicPropertyChangeEventC1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN27QDynamicPropertyChangeEventC1ERK10QByteArray(qthis, arg0)};
    let rsthis = QDynamicPropertyChangeEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDynamicPropertyChangeEvent {
  pub fn propertyName<T: QDynamicPropertyChangeEvent_propertyName>(&mut self, value: T) -> QByteArray {
    return value.propertyName(self);
    // return 1;
  }
}

pub trait QDynamicPropertyChangeEvent_propertyName {
  fn propertyName(self, rsthis: &mut QDynamicPropertyChangeEvent) -> QByteArray;
}

// proto:  QByteArray QDynamicPropertyChangeEvent::propertyName();
impl<'a> /*trait*/ QDynamicPropertyChangeEvent_propertyName for () {
  fn propertyName(self, rsthis: &mut QDynamicPropertyChangeEvent) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QDynamicPropertyChangeEvent12propertyNameEv()};
    let mut ret = unsafe {_ZNK27QDynamicPropertyChangeEvent12propertyNameEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

