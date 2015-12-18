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
  // proto: static QObject * QGenericPluginFactory::create(const QString & , const QString & );
  fn _ZN21QGenericPluginFactory6createERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto: static QStringList QGenericPluginFactory::keys();
  fn _ZN21QGenericPluginFactory4keysEv() ;
}

// body block begin
// class sizeof(QGenericPluginFactory)=1
pub struct QGenericPluginFactory {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGenericPluginFactory {
  pub fn create<RetType, T: QGenericPluginFactory_create<RetType>>(&mut self, value: T) -> RetType {
    return value.create(self);
    // return 1;
  }
}

pub trait QGenericPluginFactory_create<RetType> {
  fn create(self, rsthis: &mut QGenericPluginFactory) -> RetType;
}

// proto: static QObject * QGenericPluginFactory::create(const QString & , const QString & );
impl<'a> /*trait*/ QGenericPluginFactory_create<QObject> for (&'a  QString, &'a  QString) {
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
  pub fn keys<RetType, T: QGenericPluginFactory_keys<RetType>>(&mut self, value: T) -> RetType {
    return value.keys(self);
    // return 1;
  }
}

pub trait QGenericPluginFactory_keys<RetType> {
  fn keys(self, rsthis: &mut QGenericPluginFactory) -> RetType;
}

// proto: static QStringList QGenericPluginFactory::keys();
impl<'a> /*trait*/ QGenericPluginFactory_keys<()> for () {
  fn keys(self, rsthis: &mut QGenericPluginFactory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGenericPluginFactory4keysEv()};
     unsafe {_ZN21QGenericPluginFactory4keysEv()};
    // return 1;
  }
}

