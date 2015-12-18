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
  // proto: static bool QAccessible::isActive();
  fn _ZN11QAccessible8isActiveEv() -> int8_t;
  // proto: static unsigned int QAccessible::uniqueId(QAccessibleInterface * iface);
  fn _ZN11QAccessible8uniqueIdEP20QAccessibleInterface(arg0: *mut c_void) -> c_uint;
  // proto: static unsigned int QAccessible::registerAccessibleInterface(QAccessibleInterface * iface);
  fn _ZN11QAccessible27registerAccessibleInterfaceEP20QAccessibleInterface(arg0: *mut c_void) -> c_uint;
  // proto: static void QAccessible::setActive(bool active);
  fn _ZN11QAccessible9setActiveEb(arg0: int8_t) ;
  // proto: static QAccessibleInterface * QAccessible::queryAccessibleInterface(QObject * );
  fn _ZN11QAccessible24queryAccessibleInterfaceEP7QObject(arg0: *mut c_void) -> *mut c_void;
  // proto: static void QAccessible::updateAccessibility(QAccessibleEvent * event);
  fn _ZN11QAccessible19updateAccessibilityEP16QAccessibleEvent(arg0: *mut c_void) ;
  // proto: static void QAccessible::cleanup();
  fn _ZN11QAccessible7cleanupEv() ;
  // proto: static void QAccessible::setRootObject(QObject * object);
  fn _ZN11QAccessible13setRootObjectEP7QObject(arg0: *mut c_void) ;
  // proto: static void QAccessible::deleteAccessibleInterface(Id uniqueId);
  fn _ZN11QAccessible25deleteAccessibleInterfaceEj(arg0: c_uint) ;
  // proto: static QAccessibleInterface * QAccessible::accessibleInterface(Id uniqueId);
  fn _ZN11QAccessible19accessibleInterfaceEj(arg0: c_uint) -> *mut c_void;
  // proto:  void QAccessible::NewQAccessible();
  fn _ZN11QAccessibleC1Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QAccessible)=1
pub struct QAccessible {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessible {
  pub fn isActive<RetType, T: QAccessible_isActive<RetType>>(&mut self, value: T) -> RetType {
    return value.isActive(self);
    // return 1;
  }
}

pub trait QAccessible_isActive<RetType> {
  fn isActive(self, rsthis: &mut QAccessible) -> RetType;
}

// proto: static bool QAccessible::isActive();
impl<'a> /*trait*/ QAccessible_isActive<i8> for () {
  fn isActive(self, rsthis: &mut QAccessible) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible8isActiveEv()};
    let mut ret = unsafe {_ZN11QAccessible8isActiveEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAccessible {
  pub fn uniqueId<RetType, T: QAccessible_uniqueId<RetType>>(&mut self, value: T) -> RetType {
    return value.uniqueId(self);
    // return 1;
  }
}

pub trait QAccessible_uniqueId<RetType> {
  fn uniqueId(self, rsthis: &mut QAccessible) -> RetType;
}

// proto: static unsigned int QAccessible::uniqueId(QAccessibleInterface * iface);
impl<'a> /*trait*/ QAccessible_uniqueId<u32> for (&'a mut QAccessibleInterface) {
  fn uniqueId(self, rsthis: &mut QAccessible) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible8uniqueIdEP20QAccessibleInterface()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QAccessible8uniqueIdEP20QAccessibleInterface(arg0)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QAccessible {
  pub fn registerAccessibleInterface<RetType, T: QAccessible_registerAccessibleInterface<RetType>>(&mut self, value: T) -> RetType {
    return value.registerAccessibleInterface(self);
    // return 1;
  }
}

pub trait QAccessible_registerAccessibleInterface<RetType> {
  fn registerAccessibleInterface(self, rsthis: &mut QAccessible) -> RetType;
}

// proto: static unsigned int QAccessible::registerAccessibleInterface(QAccessibleInterface * iface);
impl<'a> /*trait*/ QAccessible_registerAccessibleInterface<u32> for (&'a mut QAccessibleInterface) {
  fn registerAccessibleInterface(self, rsthis: &mut QAccessible) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible27registerAccessibleInterfaceEP20QAccessibleInterface()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QAccessible27registerAccessibleInterfaceEP20QAccessibleInterface(arg0)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QAccessible {
  pub fn setActive<RetType, T: QAccessible_setActive<RetType>>(&mut self, value: T) -> RetType {
    return value.setActive(self);
    // return 1;
  }
}

pub trait QAccessible_setActive<RetType> {
  fn setActive(self, rsthis: &mut QAccessible) -> RetType;
}

// proto: static void QAccessible::setActive(bool active);
impl<'a> /*trait*/ QAccessible_setActive<()> for (i8) {
  fn setActive(self, rsthis: &mut QAccessible) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible9setActiveEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QAccessible9setActiveEb(arg0)};
    // return 1;
  }
}

impl /*struct*/ QAccessible {
  pub fn queryAccessibleInterface<RetType, T: QAccessible_queryAccessibleInterface<RetType>>(&mut self, value: T) -> RetType {
    return value.queryAccessibleInterface(self);
    // return 1;
  }
}

pub trait QAccessible_queryAccessibleInterface<RetType> {
  fn queryAccessibleInterface(self, rsthis: &mut QAccessible) -> RetType;
}

// proto: static QAccessibleInterface * QAccessible::queryAccessibleInterface(QObject * );
impl<'a> /*trait*/ QAccessible_queryAccessibleInterface<QAccessibleInterface> for (&'a mut QObject) {
  fn queryAccessibleInterface(self, rsthis: &mut QAccessible) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible24queryAccessibleInterfaceEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QAccessible24queryAccessibleInterfaceEP7QObject(arg0)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessible {
  pub fn updateAccessibility<RetType, T: QAccessible_updateAccessibility<RetType>>(&mut self, value: T) -> RetType {
    return value.updateAccessibility(self);
    // return 1;
  }
}

pub trait QAccessible_updateAccessibility<RetType> {
  fn updateAccessibility(self, rsthis: &mut QAccessible) -> RetType;
}

// proto: static void QAccessible::updateAccessibility(QAccessibleEvent * event);
impl<'a> /*trait*/ QAccessible_updateAccessibility<()> for (&'a mut QAccessibleEvent) {
  fn updateAccessibility(self, rsthis: &mut QAccessible) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible19updateAccessibilityEP16QAccessibleEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QAccessible19updateAccessibilityEP16QAccessibleEvent(arg0)};
    // return 1;
  }
}

impl /*struct*/ QAccessible {
  pub fn cleanup<RetType, T: QAccessible_cleanup<RetType>>(&mut self, value: T) -> RetType {
    return value.cleanup(self);
    // return 1;
  }
}

pub trait QAccessible_cleanup<RetType> {
  fn cleanup(self, rsthis: &mut QAccessible) -> RetType;
}

// proto: static void QAccessible::cleanup();
impl<'a> /*trait*/ QAccessible_cleanup<()> for () {
  fn cleanup(self, rsthis: &mut QAccessible) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible7cleanupEv()};
     unsafe {_ZN11QAccessible7cleanupEv()};
    // return 1;
  }
}

impl /*struct*/ QAccessible {
  pub fn setRootObject<RetType, T: QAccessible_setRootObject<RetType>>(&mut self, value: T) -> RetType {
    return value.setRootObject(self);
    // return 1;
  }
}

pub trait QAccessible_setRootObject<RetType> {
  fn setRootObject(self, rsthis: &mut QAccessible) -> RetType;
}

// proto: static void QAccessible::setRootObject(QObject * object);
impl<'a> /*trait*/ QAccessible_setRootObject<()> for (&'a mut QObject) {
  fn setRootObject(self, rsthis: &mut QAccessible) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible13setRootObjectEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QAccessible13setRootObjectEP7QObject(arg0)};
    // return 1;
  }
}

impl /*struct*/ QAccessible {
  pub fn deleteAccessibleInterface<RetType, T: QAccessible_deleteAccessibleInterface<RetType>>(&mut self, value: T) -> RetType {
    return value.deleteAccessibleInterface(self);
    // return 1;
  }
}

pub trait QAccessible_deleteAccessibleInterface<RetType> {
  fn deleteAccessibleInterface(self, rsthis: &mut QAccessible) -> RetType;
}

// proto: static void QAccessible::deleteAccessibleInterface(Id uniqueId);
impl<'a> /*trait*/ QAccessible_deleteAccessibleInterface<()> for (u32) {
  fn deleteAccessibleInterface(self, rsthis: &mut QAccessible) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible25deleteAccessibleInterfaceEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN11QAccessible25deleteAccessibleInterfaceEj(arg0)};
    // return 1;
  }
}

impl /*struct*/ QAccessible {
  pub fn accessibleInterface<RetType, T: QAccessible_accessibleInterface<RetType>>(&mut self, value: T) -> RetType {
    return value.accessibleInterface(self);
    // return 1;
  }
}

pub trait QAccessible_accessibleInterface<RetType> {
  fn accessibleInterface(self, rsthis: &mut QAccessible) -> RetType;
}

// proto: static QAccessibleInterface * QAccessible::accessibleInterface(Id uniqueId);
impl<'a> /*trait*/ QAccessible_accessibleInterface<QAccessibleInterface> for (u32) {
  fn accessibleInterface(self, rsthis: &mut QAccessible) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible19accessibleInterfaceEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN11QAccessible19accessibleInterfaceEj(arg0)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
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

