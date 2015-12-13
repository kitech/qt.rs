// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QAccessibleBridgePlugin::NewQAccessibleBridgePlugin(QObject * parent);
  fn _ZN23QAccessibleBridgePluginC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QAccessibleBridge * QAccessibleBridgePlugin::create(const QString & key);
  fn _ZN23QAccessibleBridgePlugin6createERK7QString(arg0: *const c_void) -> i32;
  // proto: void QAccessibleBridgePlugin::FreeQAccessibleBridgePlugin();
  fn _ZN23QAccessibleBridgePluginD0Ev() -> i32;
  // proto: const QMetaObject * QAccessibleBridgePlugin::metaObject();
  fn _ZNK23QAccessibleBridgePlugin10metaObjectEv() -> i32;
}

// body block begin
// class sizeof(QAccessibleBridgePlugin)=1
pub struct QAccessibleBridgePlugin {
  pub qclsinst: *mut c_void,
}

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

// proto: void QAccessibleBridgePlugin::NewQAccessibleBridgePlugin(QObject * parent);
impl<'a> /*trait*/ QAccessibleBridgePlugin_NewQAccessibleBridgePlugin for (&'a mut QObject) {
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

impl /*struct*/ QAccessibleBridgePlugin {
  pub fn create<T: QAccessibleBridgePlugin_create>(&mut self, value: T) -> i32 {
    value.create(self);
    return 1;
  }
}

pub trait QAccessibleBridgePlugin_create {
  fn create(self, this: &mut QAccessibleBridgePlugin) -> i32;
}

// proto: QAccessibleBridge * QAccessibleBridgePlugin::create(const QString & key);
impl<'a> /*trait*/ QAccessibleBridgePlugin_create for (&'a  QString) {
  fn create(self, this: &mut QAccessibleBridgePlugin) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QAccessibleBridgePlugin6createERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN23QAccessibleBridgePlugin6createERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QAccessibleBridgePlugin {
  pub fn FreeQAccessibleBridgePlugin<T: QAccessibleBridgePlugin_FreeQAccessibleBridgePlugin>(&mut self, value: T) -> i32 {
    value.FreeQAccessibleBridgePlugin(self);
    return 1;
  }
}

pub trait QAccessibleBridgePlugin_FreeQAccessibleBridgePlugin {
  fn FreeQAccessibleBridgePlugin(self, this: &mut QAccessibleBridgePlugin) -> i32;
}

// proto: void QAccessibleBridgePlugin::FreeQAccessibleBridgePlugin();
impl<'a> /*trait*/ QAccessibleBridgePlugin_FreeQAccessibleBridgePlugin for () {
  fn FreeQAccessibleBridgePlugin(self, this: &mut QAccessibleBridgePlugin) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QAccessibleBridgePluginD0Ev()};
    unsafe {_ZN23QAccessibleBridgePluginD0Ev()};
    return 1;
  }
}

impl /*struct*/ QAccessibleBridgePlugin {
  pub fn metaObject<T: QAccessibleBridgePlugin_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QAccessibleBridgePlugin_metaObject {
  fn metaObject(self, this: &mut QAccessibleBridgePlugin) -> i32;
}

// proto: const QMetaObject * QAccessibleBridgePlugin::metaObject();
impl<'a> /*trait*/ QAccessibleBridgePlugin_metaObject for () {
  fn metaObject(self, this: &mut QAccessibleBridgePlugin) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QAccessibleBridgePlugin10metaObjectEv()};
    unsafe {_ZNK23QAccessibleBridgePlugin10metaObjectEv()};
    return 1;
  }
}

