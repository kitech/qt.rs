// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmlnetworkaccessmanagerfactory.h
// dst-file: /src/qml/qqmlnetworkaccessmanagerfactory.rs
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
use super::super::core::qobject::QObject; // 771
use super::super::network::qnetworkaccessmanager::QNetworkAccessManager; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlNetworkAccessManagerFactory_Class_Size() -> c_int;
  // proto:  QNetworkAccessManager * QQmlNetworkAccessManagerFactory::create(QObject * parent);
  fn _ZN31QQmlNetworkAccessManagerFactory6createEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QQmlNetworkAccessManagerFactory::~QQmlNetworkAccessManagerFactory();
  fn _ZN31QQmlNetworkAccessManagerFactoryD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QQmlNetworkAccessManagerFactory)=8
#[derive(Default)]
pub struct QQmlNetworkAccessManagerFactory {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QQmlNetworkAccessManagerFactory {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlNetworkAccessManagerFactory {
    return QQmlNetworkAccessManagerFactory{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QNetworkAccessManager * QQmlNetworkAccessManagerFactory::create(QObject * parent);
impl /*struct*/ QQmlNetworkAccessManagerFactory {
  pub fn create<RetType, T: QQmlNetworkAccessManagerFactory_create<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QQmlNetworkAccessManagerFactory_create<RetType> {
  fn create(self , rsthis: & QQmlNetworkAccessManagerFactory) -> RetType;
}

  // proto:  QNetworkAccessManager * QQmlNetworkAccessManagerFactory::create(QObject * parent);
impl<'a> /*trait*/ QQmlNetworkAccessManagerFactory_create<QNetworkAccessManager> for (&'a QObject) {
  fn create(self , rsthis: & QQmlNetworkAccessManagerFactory) -> QNetworkAccessManager {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QQmlNetworkAccessManagerFactory6createEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN31QQmlNetworkAccessManagerFactory6createEP7QObject(rsthis.qclsinst, arg0)};
    let mut ret1 = QNetworkAccessManager::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlNetworkAccessManagerFactory::~QQmlNetworkAccessManagerFactory();
impl /*struct*/ QQmlNetworkAccessManagerFactory {
  pub fn free<RetType, T: QQmlNetworkAccessManagerFactory_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlNetworkAccessManagerFactory_free<RetType> {
  fn free(self , rsthis: & QQmlNetworkAccessManagerFactory) -> RetType;
}

  // proto:  void QQmlNetworkAccessManagerFactory::~QQmlNetworkAccessManagerFactory();
impl<'a> /*trait*/ QQmlNetworkAccessManagerFactory_free<()> for () {
  fn free(self , rsthis: & QQmlNetworkAccessManagerFactory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QQmlNetworkAccessManagerFactoryD2Ev()};
     unsafe {_ZN31QQmlNetworkAccessManagerFactoryD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

