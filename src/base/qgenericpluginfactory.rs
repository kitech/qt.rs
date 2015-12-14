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
use super::qstringlist::QStringList;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: static QObject * QGenericPluginFactory::create(const QString & , const QString & );
  fn _ZN21QGenericPluginFactory6createERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto: static QStringList QGenericPluginFactory::keys();
  fn _ZN21QGenericPluginFactory4keysEv() -> *mut c_void;
}

// body block begin
// class sizeof(QGenericPluginFactory)=1
pub struct QGenericPluginFactory {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGenericPluginFactory {
  pub fn create<T: QGenericPluginFactory_create>(&mut self, value: T) -> QObject {
    return value.create(self);
    // return 1;
  }
}

pub trait QGenericPluginFactory_create {
  fn create(self, rsthis: &mut QGenericPluginFactory) -> QObject;
}

// proto: static QObject * QGenericPluginFactory::create(const QString & , const QString & );
impl<'a> /*trait*/ QGenericPluginFactory_create for (&'a  QString, &'a  QString) {
  fn create(self, rsthis: &mut QGenericPluginFactory) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGenericPluginFactory6createERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QGenericPluginFactory6createERK7QStringS2_(arg0, arg1)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGenericPluginFactory {
  pub fn keys<T: QGenericPluginFactory_keys>(&mut self, value: T) -> QStringList {
    return value.keys(self);
    // return 1;
  }
}

pub trait QGenericPluginFactory_keys {
  fn keys(self, rsthis: &mut QGenericPluginFactory) -> QStringList;
}

// proto: static QStringList QGenericPluginFactory::keys();
impl<'a> /*trait*/ QGenericPluginFactory_keys for () {
  fn keys(self, rsthis: &mut QGenericPluginFactory) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGenericPluginFactory4keysEv()};
    let mut ret = unsafe {_ZN21QGenericPluginFactory4keysEv()};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

