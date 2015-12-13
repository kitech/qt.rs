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
  // proto: void QGenericPlugin::NewQGenericPlugin(QObject * parent);
  fn _ZN14QGenericPluginC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QObject * QGenericPlugin::create(const QString & name, const QString & spec);
  fn _ZN14QGenericPlugin6createERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QGenericPlugin::FreeQGenericPlugin();
  fn _ZN14QGenericPluginD0Ev() -> i32;
  // proto: const QMetaObject * QGenericPlugin::metaObject();
  fn _ZNK14QGenericPlugin10metaObjectEv() -> i32;
}

// body block begin
// class sizeof(QGenericPlugin)=1
pub struct QGenericPlugin {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGenericPlugin {
  pub fn NewQGenericPlugin<T: QGenericPlugin_NewQGenericPlugin>(value: T) -> QGenericPlugin {
    let rsthis = value.NewQGenericPlugin();
    return rsthis;
    // return 1;
  }
}

pub trait QGenericPlugin_NewQGenericPlugin {
  fn NewQGenericPlugin(self) -> QGenericPlugin;
}

// proto: void QGenericPlugin::NewQGenericPlugin(QObject * parent);
impl<'a> /*trait*/ QGenericPlugin_NewQGenericPlugin for (&'a mut QObject) {
  fn NewQGenericPlugin(self) -> QGenericPlugin {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGenericPluginC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QGenericPluginC1EP7QObject(qthis, arg0)};
    let rsthis = QGenericPlugin{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGenericPlugin {
  pub fn create<T: QGenericPlugin_create>(&mut self, value: T) -> i32 {
    value.create(self);
    return 1;
  }
}

pub trait QGenericPlugin_create {
  fn create(self, this: &mut QGenericPlugin) -> i32;
}

// proto: QObject * QGenericPlugin::create(const QString & name, const QString & spec);
impl<'a> /*trait*/ QGenericPlugin_create for (&'a  QString, &'a  QString) {
  fn create(self, this: &mut QGenericPlugin) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGenericPlugin6createERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN14QGenericPlugin6createERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGenericPlugin {
  pub fn FreeQGenericPlugin<T: QGenericPlugin_FreeQGenericPlugin>(&mut self, value: T) -> i32 {
    value.FreeQGenericPlugin(self);
    return 1;
  }
}

pub trait QGenericPlugin_FreeQGenericPlugin {
  fn FreeQGenericPlugin(self, this: &mut QGenericPlugin) -> i32;
}

// proto: void QGenericPlugin::FreeQGenericPlugin();
impl<'a> /*trait*/ QGenericPlugin_FreeQGenericPlugin for () {
  fn FreeQGenericPlugin(self, this: &mut QGenericPlugin) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGenericPluginD0Ev()};
    unsafe {_ZN14QGenericPluginD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGenericPlugin {
  pub fn metaObject<T: QGenericPlugin_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QGenericPlugin_metaObject {
  fn metaObject(self, this: &mut QGenericPlugin) -> i32;
}

// proto: const QMetaObject * QGenericPlugin::metaObject();
impl<'a> /*trait*/ QGenericPlugin_metaObject for () {
  fn metaObject(self, this: &mut QGenericPlugin) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGenericPlugin10metaObjectEv()};
    unsafe {_ZNK14QGenericPlugin10metaObjectEv()};
    return 1;
  }
}

