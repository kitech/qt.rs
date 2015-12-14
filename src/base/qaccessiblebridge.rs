// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qaccessibleevent::QAccessibleEvent;
use super::qaccessibleinterface::QAccessibleInterface;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QAccessibleBridge::FreeQAccessibleBridge();
  fn _ZN17QAccessibleBridgeD0Ev(qthis: *mut c_void) ;
  // proto:  void QAccessibleBridge::notifyAccessibilityUpdate(QAccessibleEvent * event);
  fn _ZN17QAccessibleBridge25notifyAccessibilityUpdateEP16QAccessibleEvent(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QAccessibleBridge::setRootObject(QAccessibleInterface * );
  fn _ZN17QAccessibleBridge13setRootObjectEP20QAccessibleInterface(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QAccessibleBridge)=8
pub struct QAccessibleBridge {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleBridge {
  pub fn FreeQAccessibleBridge<T: QAccessibleBridge_FreeQAccessibleBridge>(&mut self, value: T)  {
     value.FreeQAccessibleBridge(self);
    // return 1;
  }
}

pub trait QAccessibleBridge_FreeQAccessibleBridge {
  fn FreeQAccessibleBridge(self, rsthis: &mut QAccessibleBridge) ;
}

// proto:  void QAccessibleBridge::FreeQAccessibleBridge();
impl<'a> /*trait*/ QAccessibleBridge_FreeQAccessibleBridge for () {
  fn FreeQAccessibleBridge(self, rsthis: &mut QAccessibleBridge)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleBridgeD0Ev()};
     unsafe {_ZN17QAccessibleBridgeD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleBridge {
  pub fn notifyAccessibilityUpdate<T: QAccessibleBridge_notifyAccessibilityUpdate>(&mut self, value: T)  {
     value.notifyAccessibilityUpdate(self);
    // return 1;
  }
}

pub trait QAccessibleBridge_notifyAccessibilityUpdate {
  fn notifyAccessibilityUpdate(self, rsthis: &mut QAccessibleBridge) ;
}

// proto:  void QAccessibleBridge::notifyAccessibilityUpdate(QAccessibleEvent * event);
impl<'a> /*trait*/ QAccessibleBridge_notifyAccessibilityUpdate for (&'a mut QAccessibleEvent) {
  fn notifyAccessibilityUpdate(self, rsthis: &mut QAccessibleBridge)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleBridge25notifyAccessibilityUpdateEP16QAccessibleEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAccessibleBridge25notifyAccessibilityUpdateEP16QAccessibleEvent(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleBridge {
  pub fn setRootObject<T: QAccessibleBridge_setRootObject>(&mut self, value: T)  {
     value.setRootObject(self);
    // return 1;
  }
}

pub trait QAccessibleBridge_setRootObject {
  fn setRootObject(self, rsthis: &mut QAccessibleBridge) ;
}

// proto:  void QAccessibleBridge::setRootObject(QAccessibleInterface * );
impl<'a> /*trait*/ QAccessibleBridge_setRootObject for (&'a mut QAccessibleInterface) {
  fn setRootObject(self, rsthis: &mut QAccessibleBridge)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleBridge13setRootObjectEP20QAccessibleInterface()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAccessibleBridge13setRootObjectEP20QAccessibleInterface(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

