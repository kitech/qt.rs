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
  // proto: void QAccessiblePlugin::NewQAccessiblePlugin(QObject * parent);
  fn _ZN17QAccessiblePluginC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: const QMetaObject * QAccessiblePlugin::metaObject();
  fn _ZNK17QAccessiblePlugin10metaObjectEv() -> i32;
  // proto: void QAccessiblePlugin::FreeQAccessiblePlugin();
  fn _ZN17QAccessiblePluginD0Ev() -> i32;
  // proto: QAccessibleInterface * QAccessiblePlugin::create(const QString & key, QObject * object);
  fn _ZN17QAccessiblePlugin6createERK7QStringP7QObject(arg0: *const c_void, arg1: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QAccessiblePlugin)=1
pub struct QAccessiblePlugin {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessiblePlugin {
  pub fn NewQAccessiblePlugin<T: QAccessiblePlugin_NewQAccessiblePlugin>(value: T) -> QAccessiblePlugin {
    let rsthis = value.NewQAccessiblePlugin();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessiblePlugin_NewQAccessiblePlugin {
  fn NewQAccessiblePlugin(self) -> QAccessiblePlugin;
}

// proto: void QAccessiblePlugin::NewQAccessiblePlugin(QObject * parent);
impl<'a> /*trait*/ QAccessiblePlugin_NewQAccessiblePlugin for (&'a mut QObject) {
  fn NewQAccessiblePlugin(self) -> QAccessiblePlugin {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessiblePluginC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QAccessiblePluginC1EP7QObject(qthis, arg0)};
    let rsthis = QAccessiblePlugin{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessiblePlugin {
  pub fn metaObject<T: QAccessiblePlugin_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QAccessiblePlugin_metaObject {
  fn metaObject(self, this: &mut QAccessiblePlugin) -> i32;
}

// proto: const QMetaObject * QAccessiblePlugin::metaObject();
impl<'a> /*trait*/ QAccessiblePlugin_metaObject for () {
  fn metaObject(self, this: &mut QAccessiblePlugin) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessiblePlugin10metaObjectEv()};
    unsafe {_ZNK17QAccessiblePlugin10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QAccessiblePlugin {
  pub fn FreeQAccessiblePlugin<T: QAccessiblePlugin_FreeQAccessiblePlugin>(&mut self, value: T) -> i32 {
    value.FreeQAccessiblePlugin(self);
    return 1;
  }
}

pub trait QAccessiblePlugin_FreeQAccessiblePlugin {
  fn FreeQAccessiblePlugin(self, this: &mut QAccessiblePlugin) -> i32;
}

// proto: void QAccessiblePlugin::FreeQAccessiblePlugin();
impl<'a> /*trait*/ QAccessiblePlugin_FreeQAccessiblePlugin for () {
  fn FreeQAccessiblePlugin(self, this: &mut QAccessiblePlugin) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessiblePluginD0Ev()};
    unsafe {_ZN17QAccessiblePluginD0Ev()};
    return 1;
  }
}

impl /*struct*/ QAccessiblePlugin {
  pub fn create<T: QAccessiblePlugin_create>(&mut self, value: T) -> i32 {
    value.create(self);
    return 1;
  }
}

pub trait QAccessiblePlugin_create {
  fn create(self, this: &mut QAccessiblePlugin) -> i32;
}

// proto: QAccessibleInterface * QAccessiblePlugin::create(const QString & key, QObject * object);
impl<'a> /*trait*/ QAccessiblePlugin_create for (&'a  QString, &'a mut QObject) {
  fn create(self, this: &mut QAccessiblePlugin) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessiblePlugin6createERK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN17QAccessiblePlugin6createERK7QStringP7QObject(arg0, arg1)};
    return 1;
  }
}

