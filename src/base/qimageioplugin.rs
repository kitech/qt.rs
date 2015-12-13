// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qiodevice::QIODevice;
use super::qbytearray::QByteArray;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: const QMetaObject * QImageIOPlugin::metaObject();
  fn _ZNK14QImageIOPlugin10metaObjectEv() -> i32;
  // proto: void QImageIOPlugin::FreeQImageIOPlugin();
  fn _ZN14QImageIOPluginD0Ev() -> i32;
  // proto: QImageIOHandler * QImageIOPlugin::create(QIODevice * device, const QByteArray & format);
  fn _ZNK14QImageIOPlugin6createEP9QIODeviceRK10QByteArray(arg0: *mut c_void, arg1: *const c_void) -> i32;
  // proto: void QImageIOPlugin::NewQImageIOPlugin(QObject * parent);
  fn _ZN14QImageIOPluginC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QImageIOPlugin)=1
pub struct QImageIOPlugin {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QImageIOPlugin {
  pub fn metaObject<T: QImageIOPlugin_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QImageIOPlugin_metaObject {
  fn metaObject(self, this: &mut QImageIOPlugin) -> i32;
}

// proto: const QMetaObject * QImageIOPlugin::metaObject();
impl<'a> /*trait*/ QImageIOPlugin_metaObject for () {
  fn metaObject(self, this: &mut QImageIOPlugin) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QImageIOPlugin10metaObjectEv()};
    unsafe {_ZNK14QImageIOPlugin10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QImageIOPlugin {
  pub fn FreeQImageIOPlugin<T: QImageIOPlugin_FreeQImageIOPlugin>(&mut self, value: T) -> i32 {
    value.FreeQImageIOPlugin(self);
    return 1;
  }
}

pub trait QImageIOPlugin_FreeQImageIOPlugin {
  fn FreeQImageIOPlugin(self, this: &mut QImageIOPlugin) -> i32;
}

// proto: void QImageIOPlugin::FreeQImageIOPlugin();
impl<'a> /*trait*/ QImageIOPlugin_FreeQImageIOPlugin for () {
  fn FreeQImageIOPlugin(self, this: &mut QImageIOPlugin) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QImageIOPluginD0Ev()};
    unsafe {_ZN14QImageIOPluginD0Ev()};
    return 1;
  }
}

impl /*struct*/ QImageIOPlugin {
  pub fn create<T: QImageIOPlugin_create>(&mut self, value: T) -> i32 {
    value.create(self);
    return 1;
  }
}

pub trait QImageIOPlugin_create {
  fn create(self, this: &mut QImageIOPlugin) -> i32;
}

// proto: QImageIOHandler * QImageIOPlugin::create(QIODevice * device, const QByteArray & format);
impl<'a> /*trait*/ QImageIOPlugin_create for (&'a mut QIODevice, &'a  QByteArray) {
  fn create(self, this: &mut QImageIOPlugin) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QImageIOPlugin6createEP9QIODeviceRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK14QImageIOPlugin6createEP9QIODeviceRK10QByteArray(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QImageIOPlugin {
  pub fn NewQImageIOPlugin<T: QImageIOPlugin_NewQImageIOPlugin>(value: T) -> QImageIOPlugin {
    let rsthis = value.NewQImageIOPlugin();
    return rsthis;
    // return 1;
  }
}

pub trait QImageIOPlugin_NewQImageIOPlugin {
  fn NewQImageIOPlugin(self) -> QImageIOPlugin;
}

// proto: void QImageIOPlugin::NewQImageIOPlugin(QObject * parent);
impl<'a> /*trait*/ QImageIOPlugin_NewQImageIOPlugin for (&'a mut QObject) {
  fn NewQImageIOPlugin(self) -> QImageIOPlugin {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QImageIOPluginC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QImageIOPluginC1EP7QObject(qthis, arg0)};
    let rsthis = QImageIOPlugin{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

