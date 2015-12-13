// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QObject * QGenericPluginFactory::create(const QString & , const QString & );
  fn _ZN21QGenericPluginFactory6createERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QStringList QGenericPluginFactory::keys();
  fn _ZN21QGenericPluginFactory4keysEv() -> i32;
}

// body block begin
// class sizeof(QGenericPluginFactory)=1
pub struct QGenericPluginFactory {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGenericPluginFactory {
  pub fn create<T: QGenericPluginFactory_create>(&mut self, value: T) -> i32 {
    value.create(self);
    return 1;
  }
}

pub trait QGenericPluginFactory_create {
  fn create(self, this: &mut QGenericPluginFactory) -> i32;
}

// proto: QObject * QGenericPluginFactory::create(const QString & , const QString & );
impl<'a> /*trait*/ QGenericPluginFactory_create for (&'a  QString, &'a  QString) {
  fn create(self, this: &mut QGenericPluginFactory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGenericPluginFactory6createERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN21QGenericPluginFactory6createERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGenericPluginFactory {
  pub fn keys<T: QGenericPluginFactory_keys>(&mut self, value: T) -> i32 {
    value.keys(self);
    return 1;
  }
}

pub trait QGenericPluginFactory_keys {
  fn keys(self, this: &mut QGenericPluginFactory) -> i32;
}

// proto: QStringList QGenericPluginFactory::keys();
impl<'a> /*trait*/ QGenericPluginFactory_keys for () {
  fn keys(self, this: &mut QGenericPluginFactory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGenericPluginFactory4keysEv()};
    unsafe {_ZN21QGenericPluginFactory4keysEv()};
    return 1;
  }
}

