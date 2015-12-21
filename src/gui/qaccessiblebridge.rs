// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtGui/qaccessiblebridge.h
// dst-file: /src/gui/qaccessiblebridge.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::super::core::qobject::QObject; // 771
use super::super::core::qstring::QString; // 771
// use super::qaccessiblebridge::QAccessibleBridge; // 773
use super::qaccessible::QAccessibleEvent; // 773
use super::qaccessible::QAccessibleInterface; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QAccessibleBridgePlugin::QAccessibleBridgePlugin(QObject * parent);
  fn _ZN23QAccessibleBridgePluginC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QAccessibleBridge * QAccessibleBridgePlugin::create(const QString & key);
  fn _ZN23QAccessibleBridgePlugin6createERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleBridgePlugin::~QAccessibleBridgePlugin();
  fn _ZN23QAccessibleBridgePluginD0Ev(qthis: *mut c_void);
  // proto:  const QMetaObject * QAccessibleBridgePlugin::metaObject();
  fn _ZNK23QAccessibleBridgePlugin10metaObjectEv(qthis: *mut c_void);
  // proto:  void QAccessibleBridge::~QAccessibleBridge();
  fn _ZN17QAccessibleBridgeD0Ev(qthis: *mut c_void);
  // proto:  void QAccessibleBridge::notifyAccessibilityUpdate(QAccessibleEvent * event);
  fn _ZN17QAccessibleBridge25notifyAccessibilityUpdateEP16QAccessibleEvent(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAccessibleBridge::setRootObject(QAccessibleInterface * );
  fn _ZN17QAccessibleBridge13setRootObjectEP20QAccessibleInterface(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAccessibleBridgePlugin)=1
pub struct QAccessibleBridgePlugin {
  pub qclsinst: *mut c_void,
}

// class sizeof(QAccessibleBridge)=8
pub struct QAccessibleBridge {
  pub qclsinst: *mut c_void,
}

  // proto:  void QAccessibleBridgePlugin::QAccessibleBridgePlugin(QObject * parent);
impl /*struct*/ QAccessibleBridgePlugin {
  pub fn NewQAccessibleBridgePlugin<T: QAccessibleBridgePlugin_NewQAccessibleBridgePlugin>(value: T) -> QAccessibleBridgePlugin {
    let rsthis = value.NewQAccessibleBridgePlugin();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleBridgePlugin_NewQAccessibleBridgePlugin {
  fn NewQAccessibleBridgePlugin(self) -> QAccessibleBridgePlugin;
}

  // proto:  void QAccessibleBridgePlugin::QAccessibleBridgePlugin(QObject * parent);
impl<'a> /*trait*/ QAccessibleBridgePlugin_NewQAccessibleBridgePlugin for (QObject) {
  fn NewQAccessibleBridgePlugin(self) -> QAccessibleBridgePlugin {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QAccessibleBridgePluginC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QAccessibleBridgePluginC1EP7QObject(qthis, arg0)};
    let rsthis = QAccessibleBridgePlugin{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QAccessibleBridge * QAccessibleBridgePlugin::create(const QString & key);
impl /*struct*/ QAccessibleBridgePlugin {
  pub fn create<RetType, T: QAccessibleBridgePlugin_create<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QAccessibleBridgePlugin_create<RetType> {
  fn create(self , rsthis: &mut QAccessibleBridgePlugin) -> RetType;
}

  // proto:  QAccessibleBridge * QAccessibleBridgePlugin::create(const QString & key);
impl<'a> /*trait*/ QAccessibleBridgePlugin_create<QAccessibleBridge> for (QString) {
  fn create(self , rsthis: &mut QAccessibleBridgePlugin) -> QAccessibleBridge {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QAccessibleBridgePlugin6createERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN23QAccessibleBridgePlugin6createERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QAccessibleBridge{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QAccessibleBridgePlugin::~QAccessibleBridgePlugin();
impl /*struct*/ QAccessibleBridgePlugin {
  pub fn FreeQAccessibleBridgePlugin<RetType, T: QAccessibleBridgePlugin_FreeQAccessibleBridgePlugin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleBridgePlugin(self);
    // return 1;
  }
}

pub trait QAccessibleBridgePlugin_FreeQAccessibleBridgePlugin<RetType> {
  fn FreeQAccessibleBridgePlugin(self , rsthis: &mut QAccessibleBridgePlugin) -> RetType;
}

  // proto:  void QAccessibleBridgePlugin::~QAccessibleBridgePlugin();
impl<'a> /*trait*/ QAccessibleBridgePlugin_FreeQAccessibleBridgePlugin<()> for () {
  fn FreeQAccessibleBridgePlugin(self , rsthis: &mut QAccessibleBridgePlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QAccessibleBridgePluginD0Ev()};
     unsafe {_ZN23QAccessibleBridgePluginD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QAccessibleBridgePlugin::metaObject();
impl /*struct*/ QAccessibleBridgePlugin {
  pub fn metaObject<RetType, T: QAccessibleBridgePlugin_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAccessibleBridgePlugin_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QAccessibleBridgePlugin) -> RetType;
}

  // proto:  const QMetaObject * QAccessibleBridgePlugin::metaObject();
impl<'a> /*trait*/ QAccessibleBridgePlugin_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QAccessibleBridgePlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QAccessibleBridgePlugin10metaObjectEv()};
     unsafe {_ZNK23QAccessibleBridgePlugin10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
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

// <= body block end

