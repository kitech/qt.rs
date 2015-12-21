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
  // proto:  void QAccessibleBridge::~QAccessibleBridge();
  fn _ZN17QAccessibleBridgeD0Ev(qthis: *mut c_void);
  // proto:  void QAccessibleBridge::notifyAccessibilityUpdate(QAccessibleEvent * event);
  fn _ZN17QAccessibleBridge25notifyAccessibilityUpdateEP16QAccessibleEvent(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAccessibleBridge::setRootObject(QAccessibleInterface * );
  fn _ZN17QAccessibleBridge13setRootObjectEP20QAccessibleInterface(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QAccessibleBridge)=8
pub struct QAccessibleBridge {
  pub qclsinst: *mut c_void,
}

  // proto:  void QAccessibleBridge::~QAccessibleBridge();
impl /*struct*/ QAccessibleBridge {
  pub fn FreeQAccessibleBridge<RetType, T: QAccessibleBridge_FreeQAccessibleBridge<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleBridge(self);
    // return 1;
  }
}

pub trait QAccessibleBridge_FreeQAccessibleBridge<RetType> {
  fn FreeQAccessibleBridge(self , rsthis: &mut QAccessibleBridge) -> RetType;
}

  // proto:  void QAccessibleBridge::~QAccessibleBridge();
impl<'a> /*trait*/ QAccessibleBridge_FreeQAccessibleBridge<()> for () {
  fn FreeQAccessibleBridge(self , rsthis: &mut QAccessibleBridge) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleBridgeD0Ev()};
     unsafe {_ZN17QAccessibleBridgeD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAccessibleBridge::notifyAccessibilityUpdate(QAccessibleEvent * event);
impl /*struct*/ QAccessibleBridge {
  pub fn notifyAccessibilityUpdate<RetType, T: QAccessibleBridge_notifyAccessibilityUpdate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.notifyAccessibilityUpdate(self);
    // return 1;
  }
}

pub trait QAccessibleBridge_notifyAccessibilityUpdate<RetType> {
  fn notifyAccessibilityUpdate(self , rsthis: &mut QAccessibleBridge) -> RetType;
}

  // proto:  void QAccessibleBridge::notifyAccessibilityUpdate(QAccessibleEvent * event);
impl<'a> /*trait*/ QAccessibleBridge_notifyAccessibilityUpdate<()> for (QAccessibleEvent) {
  fn notifyAccessibilityUpdate(self , rsthis: &mut QAccessibleBridge) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleBridge25notifyAccessibilityUpdateEP16QAccessibleEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAccessibleBridge25notifyAccessibilityUpdateEP16QAccessibleEvent(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAccessibleBridge::setRootObject(QAccessibleInterface * );
impl /*struct*/ QAccessibleBridge {
  pub fn setRootObject<RetType, T: QAccessibleBridge_setRootObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRootObject(self);
    // return 1;
  }
}

pub trait QAccessibleBridge_setRootObject<RetType> {
  fn setRootObject(self , rsthis: &mut QAccessibleBridge) -> RetType;
}

  // proto:  void QAccessibleBridge::setRootObject(QAccessibleInterface * );
impl<'a> /*trait*/ QAccessibleBridge_setRootObject<()> for (QAccessibleInterface) {
  fn setRootObject(self , rsthis: &mut QAccessibleBridge) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleBridge13setRootObjectEP20QAccessibleInterface()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QAccessibleBridge13setRootObjectEP20QAccessibleInterface(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

