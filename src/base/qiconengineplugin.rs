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
  // proto: const QMetaObject * QIconEnginePlugin::metaObject();
  fn _ZNK17QIconEnginePlugin10metaObjectEv() -> i32;
  // proto: void QIconEnginePlugin::NewQIconEnginePlugin(QObject * parent);
  fn _ZN17QIconEnginePluginC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QIconEngine * QIconEnginePlugin::create(const QString & filename);
  fn _ZN17QIconEnginePlugin6createERK7QString(arg0: *const c_void) -> i32;
  // proto: void QIconEnginePlugin::FreeQIconEnginePlugin();
  fn _ZN17QIconEnginePluginD0Ev() -> i32;
}

// body block begin
// class sizeof(QIconEnginePlugin)=1
pub struct QIconEnginePlugin {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QIconEnginePlugin {
  pub fn metaObject<T: QIconEnginePlugin_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QIconEnginePlugin_metaObject {
  fn metaObject(self, this: &mut QIconEnginePlugin) -> i32;
}

// proto: const QMetaObject * QIconEnginePlugin::metaObject();
impl<'a> /*trait*/ QIconEnginePlugin_metaObject for () {
  fn metaObject(self, this: &mut QIconEnginePlugin) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QIconEnginePlugin10metaObjectEv()};
    unsafe {_ZNK17QIconEnginePlugin10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QIconEnginePlugin {
  pub fn NewQIconEnginePlugin<T: QIconEnginePlugin_NewQIconEnginePlugin>(value: T) -> QIconEnginePlugin {
    let rsthis = value.NewQIconEnginePlugin();
    return rsthis;
    // return 1;
  }
}

pub trait QIconEnginePlugin_NewQIconEnginePlugin {
  fn NewQIconEnginePlugin(self) -> QIconEnginePlugin;
}

// proto: void QIconEnginePlugin::NewQIconEnginePlugin(QObject * parent);
impl<'a> /*trait*/ QIconEnginePlugin_NewQIconEnginePlugin for (&'a mut QObject) {
  fn NewQIconEnginePlugin(self) -> QIconEnginePlugin {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QIconEnginePluginC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QIconEnginePluginC1EP7QObject(qthis, arg0)};
    let rsthis = QIconEnginePlugin{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QIconEnginePlugin {
  pub fn create<T: QIconEnginePlugin_create>(&mut self, value: T) -> i32 {
    value.create(self);
    return 1;
  }
}

pub trait QIconEnginePlugin_create {
  fn create(self, this: &mut QIconEnginePlugin) -> i32;
}

// proto: QIconEngine * QIconEnginePlugin::create(const QString & filename);
impl<'a> /*trait*/ QIconEnginePlugin_create for (&'a  QString) {
  fn create(self, this: &mut QIconEnginePlugin) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QIconEnginePlugin6createERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QIconEnginePlugin6createERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QIconEnginePlugin {
  pub fn FreeQIconEnginePlugin<T: QIconEnginePlugin_FreeQIconEnginePlugin>(&mut self, value: T) -> i32 {
    value.FreeQIconEnginePlugin(self);
    return 1;
  }
}

pub trait QIconEnginePlugin_FreeQIconEnginePlugin {
  fn FreeQIconEnginePlugin(self, this: &mut QIconEnginePlugin) -> i32;
}

// proto: void QIconEnginePlugin::FreeQIconEnginePlugin();
impl<'a> /*trait*/ QIconEnginePlugin_FreeQIconEnginePlugin for () {
  fn FreeQIconEnginePlugin(self, this: &mut QIconEnginePlugin) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QIconEnginePluginD0Ev()};
    unsafe {_ZN17QIconEnginePluginD0Ev()};
    return 1;
  }
}

