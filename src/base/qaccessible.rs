// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qaccessibleinterface::QAccessibleInterface;
use super::qobject::QObject;
use super::qaccessibleevent::QAccessibleEvent;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QAccessible::isActive();
  fn _ZN11QAccessible8isActiveEv() -> i32;
  // proto: unsigned int QAccessible::uniqueId(QAccessibleInterface * iface);
  fn _ZN11QAccessible8uniqueIdEP20QAccessibleInterface(arg0: *mut c_void) -> i32;
  // proto: unsigned int QAccessible::registerAccessibleInterface(QAccessibleInterface * iface);
  fn _ZN11QAccessible27registerAccessibleInterfaceEP20QAccessibleInterface(arg0: *mut c_void) -> i32;
  // proto: void QAccessible::setActive(bool active);
  fn _ZN11QAccessible9setActiveEb(arg0: int8_t) -> i32;
  // proto: QAccessibleInterface * QAccessible::queryAccessibleInterface(QObject * );
  fn _ZN11QAccessible24queryAccessibleInterfaceEP7QObject(arg0: *mut c_void) -> i32;
  // proto: void QAccessible::updateAccessibility(QAccessibleEvent * event);
  fn _ZN11QAccessible19updateAccessibilityEP16QAccessibleEvent(arg0: *mut c_void) -> i32;
  // proto: void QAccessible::cleanup();
  fn _ZN11QAccessible7cleanupEv() -> i32;
  // proto: void QAccessible::setRootObject(QObject * object);
  fn _ZN11QAccessible13setRootObjectEP7QObject(arg0: *mut c_void) -> i32;
  // proto: void QAccessible::deleteAccessibleInterface(Id uniqueId);
  fn _ZN11QAccessible25deleteAccessibleInterfaceEj(arg0: c_uint) -> i32;
  // proto: QAccessibleInterface * QAccessible::accessibleInterface(Id uniqueId);
  fn _ZN11QAccessible19accessibleInterfaceEj(arg0: c_uint) -> i32;
  // proto: void QAccessible::NewQAccessible();
  fn _ZN11QAccessibleC1Ev(qthis: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QAccessible)=1
pub struct QAccessible {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessible {
  pub fn isActive<T: QAccessible_isActive>(&mut self, value: T) -> i32 {
    value.isActive(self);
    return 1;
  }
}

pub trait QAccessible_isActive {
  fn isActive(self, this: &mut QAccessible) -> i32;
}

// proto: bool QAccessible::isActive();
impl<'a> /*trait*/ QAccessible_isActive for () {
  fn isActive(self, this: &mut QAccessible) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible8isActiveEv()};
    unsafe {_ZN11QAccessible8isActiveEv()};
    return 1;
  }
}

impl /*struct*/ QAccessible {
  pub fn uniqueId<T: QAccessible_uniqueId>(&mut self, value: T) -> i32 {
    value.uniqueId(self);
    return 1;
  }
}

pub trait QAccessible_uniqueId {
  fn uniqueId(self, this: &mut QAccessible) -> i32;
}

// proto: unsigned int QAccessible::uniqueId(QAccessibleInterface * iface);
impl<'a> /*trait*/ QAccessible_uniqueId for (&'a mut QAccessibleInterface) {
  fn uniqueId(self, this: &mut QAccessible) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible8uniqueIdEP20QAccessibleInterface()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QAccessible8uniqueIdEP20QAccessibleInterface(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessible {
  pub fn registerAccessibleInterface<T: QAccessible_registerAccessibleInterface>(&mut self, value: T) -> i32 {
    value.registerAccessibleInterface(self);
    return 1;
  }
}

pub trait QAccessible_registerAccessibleInterface {
  fn registerAccessibleInterface(self, this: &mut QAccessible) -> i32;
}

// proto: unsigned int QAccessible::registerAccessibleInterface(QAccessibleInterface * iface);
impl<'a> /*trait*/ QAccessible_registerAccessibleInterface for (&'a mut QAccessibleInterface) {
  fn registerAccessibleInterface(self, this: &mut QAccessible) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible27registerAccessibleInterfaceEP20QAccessibleInterface()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QAccessible27registerAccessibleInterfaceEP20QAccessibleInterface(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessible {
  pub fn setActive<T: QAccessible_setActive>(&mut self, value: T) -> i32 {
    value.setActive(self);
    return 1;
  }
}

pub trait QAccessible_setActive {
  fn setActive(self, this: &mut QAccessible) -> i32;
}

// proto: void QAccessible::setActive(bool active);
impl<'a> /*trait*/ QAccessible_setActive for (i8) {
  fn setActive(self, this: &mut QAccessible) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible9setActiveEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QAccessible9setActiveEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessible {
  pub fn queryAccessibleInterface<T: QAccessible_queryAccessibleInterface>(&mut self, value: T) -> i32 {
    value.queryAccessibleInterface(self);
    return 1;
  }
}

pub trait QAccessible_queryAccessibleInterface {
  fn queryAccessibleInterface(self, this: &mut QAccessible) -> i32;
}

// proto: QAccessibleInterface * QAccessible::queryAccessibleInterface(QObject * );
impl<'a> /*trait*/ QAccessible_queryAccessibleInterface for (&'a mut QObject) {
  fn queryAccessibleInterface(self, this: &mut QAccessible) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible24queryAccessibleInterfaceEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QAccessible24queryAccessibleInterfaceEP7QObject(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessible {
  pub fn updateAccessibility<T: QAccessible_updateAccessibility>(&mut self, value: T) -> i32 {
    value.updateAccessibility(self);
    return 1;
  }
}

pub trait QAccessible_updateAccessibility {
  fn updateAccessibility(self, this: &mut QAccessible) -> i32;
}

// proto: void QAccessible::updateAccessibility(QAccessibleEvent * event);
impl<'a> /*trait*/ QAccessible_updateAccessibility for (&'a mut QAccessibleEvent) {
  fn updateAccessibility(self, this: &mut QAccessible) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible19updateAccessibilityEP16QAccessibleEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QAccessible19updateAccessibilityEP16QAccessibleEvent(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessible {
  pub fn cleanup<T: QAccessible_cleanup>(&mut self, value: T) -> i32 {
    value.cleanup(self);
    return 1;
  }
}

pub trait QAccessible_cleanup {
  fn cleanup(self, this: &mut QAccessible) -> i32;
}

// proto: void QAccessible::cleanup();
impl<'a> /*trait*/ QAccessible_cleanup for () {
  fn cleanup(self, this: &mut QAccessible) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible7cleanupEv()};
    unsafe {_ZN11QAccessible7cleanupEv()};
    return 1;
  }
}

impl /*struct*/ QAccessible {
  pub fn setRootObject<T: QAccessible_setRootObject>(&mut self, value: T) -> i32 {
    value.setRootObject(self);
    return 1;
  }
}

pub trait QAccessible_setRootObject {
  fn setRootObject(self, this: &mut QAccessible) -> i32;
}

// proto: void QAccessible::setRootObject(QObject * object);
impl<'a> /*trait*/ QAccessible_setRootObject for (&'a mut QObject) {
  fn setRootObject(self, this: &mut QAccessible) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible13setRootObjectEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QAccessible13setRootObjectEP7QObject(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessible {
  pub fn deleteAccessibleInterface<T: QAccessible_deleteAccessibleInterface>(&mut self, value: T) -> i32 {
    value.deleteAccessibleInterface(self);
    return 1;
  }
}

pub trait QAccessible_deleteAccessibleInterface {
  fn deleteAccessibleInterface(self, this: &mut QAccessible) -> i32;
}

// proto: void QAccessible::deleteAccessibleInterface(Id uniqueId);
impl<'a> /*trait*/ QAccessible_deleteAccessibleInterface for (u32) {
  fn deleteAccessibleInterface(self, this: &mut QAccessible) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible25deleteAccessibleInterfaceEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN11QAccessible25deleteAccessibleInterfaceEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessible {
  pub fn accessibleInterface<T: QAccessible_accessibleInterface>(&mut self, value: T) -> i32 {
    value.accessibleInterface(self);
    return 1;
  }
}

pub trait QAccessible_accessibleInterface {
  fn accessibleInterface(self, this: &mut QAccessible) -> i32;
}

// proto: QAccessibleInterface * QAccessible::accessibleInterface(Id uniqueId);
impl<'a> /*trait*/ QAccessible_accessibleInterface for (u32) {
  fn accessibleInterface(self, this: &mut QAccessible) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible19accessibleInterfaceEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN11QAccessible19accessibleInterfaceEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessible {
  pub fn NewQAccessible<T: QAccessible_NewQAccessible>(value: T) -> QAccessible {
    let rsthis = value.NewQAccessible();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessible_NewQAccessible {
  fn NewQAccessible(self) -> QAccessible;
}

// proto: void QAccessible::NewQAccessible();
impl<'a> /*trait*/ QAccessible_NewQAccessible for () {
  fn NewQAccessible(self) -> QAccessible {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessibleC1Ev()};
    unsafe {_ZN11QAccessibleC1Ev(qthis)};
    let rsthis = QAccessible{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

