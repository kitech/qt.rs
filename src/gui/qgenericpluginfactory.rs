// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtGui/qgenericpluginfactory.h
// dst-file: /src/gui/qgenericpluginfactory.rs
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
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::super::core::qobject::QObject; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGenericPluginFactory_Class_Size() -> c_int;
  // proto: static QObject * QGenericPluginFactory::create(const QString & , const QString & );
  fn _ZN21QGenericPluginFactory6createERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto: static QStringList QGenericPluginFactory::keys();
  fn _ZN21QGenericPluginFactory4keysEv();
} // <= ext block end

// body block begin =>
// class sizeof(QGenericPluginFactory)=1
pub struct QGenericPluginFactory {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGenericPluginFactory {
  pub fn inheritFrom(qthis: *mut c_void) -> QGenericPluginFactory {
    return QGenericPluginFactory{qclsinst: qthis};
  }
}
  // proto: static QObject * QGenericPluginFactory::create(const QString & , const QString & );
impl /*struct*/ QGenericPluginFactory {
  pub fn create_s<RetType, T: QGenericPluginFactory_create_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.create_s();
    // return 1;
  }
}

pub trait QGenericPluginFactory_create_s<RetType> {
  fn create_s(self ) -> RetType;
}

  // proto: static QObject * QGenericPluginFactory::create(const QString & , const QString & );
impl<'a> /*trait*/ QGenericPluginFactory_create_s<QObject> for (&'a QString, &'a QString) {
  fn create_s(self ) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGenericPluginFactory6createERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QGenericPluginFactory6createERK7QStringS2_(arg0, arg1)};
    let mut ret1 = QObject::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QStringList QGenericPluginFactory::keys();
impl /*struct*/ QGenericPluginFactory {
  pub fn keys_s<RetType, T: QGenericPluginFactory_keys_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.keys_s();
    // return 1;
  }
}

pub trait QGenericPluginFactory_keys_s<RetType> {
  fn keys_s(self ) -> RetType;
}

  // proto: static QStringList QGenericPluginFactory::keys();
impl<'a> /*trait*/ QGenericPluginFactory_keys_s<()> for () {
  fn keys_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGenericPluginFactory4keysEv()};
     unsafe {_ZN21QGenericPluginFactory4keysEv()};
    // return 1;
  }
}

// <= body block end

