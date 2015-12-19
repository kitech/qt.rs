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

// proto: static bool QAccessible::isActive();
impl /*struct*/ QAccessible {
  pub fn isActive_s<RetType, T: QAccessible_isActive_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isActive_s();
    // return 1;
  }
}

pub trait QAccessible_isActive_s<RetType> {
  fn isActive_s(self ) -> RetType;
}

// proto: static bool QAccessible::isActive();
impl<'a> /*trait*/ QAccessible_isActive_s<i8> for () {
  fn isActive_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible8isActiveEv()};
    let mut ret = unsafe {_ZN11QAccessible8isActiveEv()};
    return ret as i8;
    // return 1;
  }
}

// proto: static unsigned int QAccessible::uniqueId(QAccessibleInterface * iface);
impl /*struct*/ QAccessible {
  pub fn uniqueId_s<RetType, T: QAccessible_uniqueId_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.uniqueId_s();
    // return 1;
  }
}

pub trait QAccessible_uniqueId_s<RetType> {
  fn uniqueId_s(self ) -> RetType;
}

// proto: static unsigned int QAccessible::uniqueId(QAccessibleInterface * iface);
impl<'a> /*trait*/ QAccessible_uniqueId_s<u32> for (&'a mut QAccessibleInterface) {
  fn uniqueId_s(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible8uniqueIdEP20QAccessibleInterface()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QAccessible8uniqueIdEP20QAccessibleInterface(arg0)};
    return ret as u32;
    // return 1;
  }
}

// proto: static unsigned int QAccessible::registerAccessibleInterface(QAccessibleInterface * iface);
impl /*struct*/ QAccessible {
  pub fn registerAccessibleInterface_s<RetType, T: QAccessible_registerAccessibleInterface_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.registerAccessibleInterface_s();
    // return 1;
  }
}

pub trait QAccessible_registerAccessibleInterface_s<RetType> {
  fn registerAccessibleInterface_s(self ) -> RetType;
}

// proto: static unsigned int QAccessible::registerAccessibleInterface(QAccessibleInterface * iface);
impl<'a> /*trait*/ QAccessible_registerAccessibleInterface_s<u32> for (&'a mut QAccessibleInterface) {
  fn registerAccessibleInterface_s(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible27registerAccessibleInterfaceEP20QAccessibleInterface()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QAccessible27registerAccessibleInterfaceEP20QAccessibleInterface(arg0)};
    return ret as u32;
    // return 1;
  }
}

// proto: static void QAccessible::setActive(bool active);
impl /*struct*/ QAccessible {
  pub fn setActive_s<RetType, T: QAccessible_setActive_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setActive_s();
    // return 1;
  }
}

pub trait QAccessible_setActive_s<RetType> {
  fn setActive_s(self ) -> RetType;
}

// proto: static void QAccessible::setActive(bool active);
impl<'a> /*trait*/ QAccessible_setActive_s<()> for (i8) {
  fn setActive_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible9setActiveEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QAccessible9setActiveEb(arg0)};
    // return 1;
  }
}

// proto: static QAccessibleInterface * QAccessible::queryAccessibleInterface(QObject * );
impl /*struct*/ QAccessible {
  pub fn queryAccessibleInterface_s<RetType, T: QAccessible_queryAccessibleInterface_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.queryAccessibleInterface_s();
    // return 1;
  }
}

pub trait QAccessible_queryAccessibleInterface_s<RetType> {
  fn queryAccessibleInterface_s(self ) -> RetType;
}

// proto: static QAccessibleInterface * QAccessible::queryAccessibleInterface(QObject * );
impl<'a> /*trait*/ QAccessible_queryAccessibleInterface_s<QAccessibleInterface> for (&'a mut QObject) {
  fn queryAccessibleInterface_s(self ) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible24queryAccessibleInterfaceEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QAccessible24queryAccessibleInterfaceEP7QObject(arg0)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static void QAccessible::updateAccessibility(QAccessibleEvent * event);
impl /*struct*/ QAccessible {
  pub fn updateAccessibility_s<RetType, T: QAccessible_updateAccessibility_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.updateAccessibility_s();
    // return 1;
  }
}

pub trait QAccessible_updateAccessibility_s<RetType> {
  fn updateAccessibility_s(self ) -> RetType;
}

// proto: static void QAccessible::updateAccessibility(QAccessibleEvent * event);
impl<'a> /*trait*/ QAccessible_updateAccessibility_s<()> for (&'a mut QAccessibleEvent) {
  fn updateAccessibility_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible19updateAccessibilityEP16QAccessibleEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QAccessible19updateAccessibilityEP16QAccessibleEvent(arg0)};
    // return 1;
  }
}

// proto: static void QAccessible::cleanup();
impl /*struct*/ QAccessible {
  pub fn cleanup_s<RetType, T: QAccessible_cleanup_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.cleanup_s();
    // return 1;
  }
}

pub trait QAccessible_cleanup_s<RetType> {
  fn cleanup_s(self ) -> RetType;
}

// proto: static void QAccessible::cleanup();
impl<'a> /*trait*/ QAccessible_cleanup_s<()> for () {
  fn cleanup_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible7cleanupEv()};
     unsafe {_ZN11QAccessible7cleanupEv()};
    // return 1;
  }
}

// proto: static void QAccessible::setRootObject(QObject * object);
impl /*struct*/ QAccessible {
  pub fn setRootObject_s<RetType, T: QAccessible_setRootObject_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setRootObject_s();
    // return 1;
  }
}

pub trait QAccessible_setRootObject_s<RetType> {
  fn setRootObject_s(self ) -> RetType;
}

// proto: static void QAccessible::setRootObject(QObject * object);
impl<'a> /*trait*/ QAccessible_setRootObject_s<()> for (&'a mut QObject) {
  fn setRootObject_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible13setRootObjectEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QAccessible13setRootObjectEP7QObject(arg0)};
    // return 1;
  }
}

// proto: static void QAccessible::deleteAccessibleInterface(Id uniqueId);
impl /*struct*/ QAccessible {
  pub fn deleteAccessibleInterface_s<RetType, T: QAccessible_deleteAccessibleInterface_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.deleteAccessibleInterface_s();
    // return 1;
  }
}

pub trait QAccessible_deleteAccessibleInterface_s<RetType> {
  fn deleteAccessibleInterface_s(self ) -> RetType;
}

// proto: static void QAccessible::deleteAccessibleInterface(Id uniqueId);
impl<'a> /*trait*/ QAccessible_deleteAccessibleInterface_s<()> for (u32) {
  fn deleteAccessibleInterface_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible25deleteAccessibleInterfaceEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN11QAccessible25deleteAccessibleInterfaceEj(arg0)};
    // return 1;
  }
}

// proto: static QAccessibleInterface * QAccessible::accessibleInterface(Id uniqueId);
impl /*struct*/ QAccessible {
  pub fn accessibleInterface_s<RetType, T: QAccessible_accessibleInterface_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.accessibleInterface_s();
    // return 1;
  }
}

pub trait QAccessible_accessibleInterface_s<RetType> {
  fn accessibleInterface_s(self ) -> RetType;
}

// proto: static QAccessibleInterface * QAccessible::accessibleInterface(Id uniqueId);
impl<'a> /*trait*/ QAccessible_accessibleInterface_s<QAccessibleInterface> for (u32) {
  fn accessibleInterface_s(self ) -> QAccessibleInterface {
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

