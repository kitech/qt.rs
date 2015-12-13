// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QStyle * QStylePlugin::create(const QString & key);
  fn _ZN12QStylePlugin6createERK7QString(arg0: *const c_void) -> i32;
  // proto: const QMetaObject * QStylePlugin::metaObject();
  fn _ZNK12QStylePlugin10metaObjectEv() -> i32;
  // proto: void QStylePlugin::NewQStylePlugin(QObject * parent);
  fn _ZN12QStylePluginC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QStylePlugin::FreeQStylePlugin();
  fn _ZN12QStylePluginD0Ev() -> i32;
}

// body block begin
// class sizeof(QStylePlugin)=1
pub struct QStylePlugin {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStylePlugin {
  pub fn create<T: QStylePlugin_create>(&mut self, value: T) -> i32 {
    value.create(self);
    return 1;
  }
}

pub trait QStylePlugin_create {
  fn create(self, this: &mut QStylePlugin) -> i32;
}

// proto: QStyle * QStylePlugin::create(const QString & key);
impl<'a> /*trait*/ QStylePlugin_create for (&'a  QString) {
  fn create(self, this: &mut QStylePlugin) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStylePlugin6createERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QStylePlugin6createERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QStylePlugin {
  pub fn metaObject<T: QStylePlugin_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QStylePlugin_metaObject {
  fn metaObject(self, this: &mut QStylePlugin) -> i32;
}

// proto: const QMetaObject * QStylePlugin::metaObject();
impl<'a> /*trait*/ QStylePlugin_metaObject for () {
  fn metaObject(self, this: &mut QStylePlugin) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QStylePlugin10metaObjectEv()};
    unsafe {_ZNK12QStylePlugin10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QStylePlugin {
  pub fn NewQStylePlugin<T: QStylePlugin_NewQStylePlugin>(value: T) -> QStylePlugin {
    let rsthis = value.NewQStylePlugin();
    return rsthis;
    // return 1;
  }
}

pub trait QStylePlugin_NewQStylePlugin {
  fn NewQStylePlugin(self) -> QStylePlugin;
}

// proto: void QStylePlugin::NewQStylePlugin(QObject * parent);
impl<'a> /*trait*/ QStylePlugin_NewQStylePlugin for (&'a mut QObject) {
  fn NewQStylePlugin(self) -> QStylePlugin {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStylePluginC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QStylePluginC1EP7QObject(qthis, arg0)};
    let rsthis = QStylePlugin{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStylePlugin {
  pub fn FreeQStylePlugin<T: QStylePlugin_FreeQStylePlugin>(&mut self, value: T) -> i32 {
    value.FreeQStylePlugin(self);
    return 1;
  }
}

pub trait QStylePlugin_FreeQStylePlugin {
  fn FreeQStylePlugin(self, this: &mut QStylePlugin) -> i32;
}

// proto: void QStylePlugin::FreeQStylePlugin();
impl<'a> /*trait*/ QStylePlugin_FreeQStylePlugin for () {
  fn FreeQStylePlugin(self, this: &mut QStylePlugin) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStylePluginD0Ev()};
    unsafe {_ZN12QStylePluginD0Ev()};
    return 1;
  }
}

