// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmlextensioninterface.h
// dst-file: /src/qml/qqmlextensioninterface.rs
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
// use super::qqmlextensioninterface::QQmlTypesExtensionInterface; // 773
use super::qqmlengine::QQmlEngine; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlTypesExtensionInterface_Class_Size() -> c_int;
  // proto:  void QQmlTypesExtensionInterface::registerTypes(const char * uri);
  fn _ZN27QQmlTypesExtensionInterface13registerTypesEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char);
  // proto:  void QQmlTypesExtensionInterface::~QQmlTypesExtensionInterface();
  fn _ZN27QQmlTypesExtensionInterfaceD2Ev(qthis: u64 /* *mut c_void*/);
  fn QQmlExtensionInterface_Class_Size() -> c_int;
  // proto:  void QQmlExtensionInterface::~QQmlExtensionInterface();
  fn _ZN22QQmlExtensionInterfaceD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlExtensionInterface::initializeEngine(QQmlEngine * engine, const char * uri);
  fn _ZN22QQmlExtensionInterface16initializeEngineEP10QQmlEnginePKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char);
} // <= ext block end

// body block begin =>
// class sizeof(QQmlTypesExtensionInterface)=8
#[derive(Default)]
pub struct QQmlTypesExtensionInterface {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QQmlExtensionInterface)=8
#[derive(Default)]
pub struct QQmlExtensionInterface {
  qbase: QQmlTypesExtensionInterface,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QQmlTypesExtensionInterface {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlTypesExtensionInterface {
    return QQmlTypesExtensionInterface{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QQmlTypesExtensionInterface::registerTypes(const char * uri);
impl /*struct*/ QQmlTypesExtensionInterface {
  pub fn registerTypes<RetType, T: QQmlTypesExtensionInterface_registerTypes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.registerTypes(self);
    // return 1;
  }
}

pub trait QQmlTypesExtensionInterface_registerTypes<RetType> {
  fn registerTypes(self , rsthis: & QQmlTypesExtensionInterface) -> RetType;
}

  // proto:  void QQmlTypesExtensionInterface::registerTypes(const char * uri);
impl<'a> /*trait*/ QQmlTypesExtensionInterface_registerTypes<()> for (&'a  String) {
  fn registerTypes(self , rsthis: & QQmlTypesExtensionInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QQmlTypesExtensionInterface13registerTypesEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZN27QQmlTypesExtensionInterface13registerTypesEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQmlTypesExtensionInterface::~QQmlTypesExtensionInterface();
impl /*struct*/ QQmlTypesExtensionInterface {
  pub fn free<RetType, T: QQmlTypesExtensionInterface_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlTypesExtensionInterface_free<RetType> {
  fn free(self , rsthis: & QQmlTypesExtensionInterface) -> RetType;
}

  // proto:  void QQmlTypesExtensionInterface::~QQmlTypesExtensionInterface();
impl<'a> /*trait*/ QQmlTypesExtensionInterface_free<()> for () {
  fn free(self , rsthis: & QQmlTypesExtensionInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QQmlTypesExtensionInterfaceD2Ev()};
     unsafe {_ZN27QQmlTypesExtensionInterfaceD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QQmlExtensionInterface {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlExtensionInterface {
    return QQmlExtensionInterface{qbase: QQmlTypesExtensionInterface::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQmlExtensionInterface {
  type Target = QQmlTypesExtensionInterface;

  fn deref(&self) -> &QQmlTypesExtensionInterface {
    return & self.qbase;
  }
}
impl AsRef<QQmlTypesExtensionInterface> for QQmlExtensionInterface {
  fn as_ref(& self) -> & QQmlTypesExtensionInterface {
    return & self.qbase;
  }
}
  // proto:  void QQmlExtensionInterface::~QQmlExtensionInterface();
impl /*struct*/ QQmlExtensionInterface {
  pub fn free<RetType, T: QQmlExtensionInterface_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlExtensionInterface_free<RetType> {
  fn free(self , rsthis: & QQmlExtensionInterface) -> RetType;
}

  // proto:  void QQmlExtensionInterface::~QQmlExtensionInterface();
impl<'a> /*trait*/ QQmlExtensionInterface_free<()> for () {
  fn free(self , rsthis: & QQmlExtensionInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QQmlExtensionInterfaceD2Ev()};
     unsafe {_ZN22QQmlExtensionInterfaceD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlExtensionInterface::initializeEngine(QQmlEngine * engine, const char * uri);
impl /*struct*/ QQmlExtensionInterface {
  pub fn initializeEngine<RetType, T: QQmlExtensionInterface_initializeEngine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.initializeEngine(self);
    // return 1;
  }
}

pub trait QQmlExtensionInterface_initializeEngine<RetType> {
  fn initializeEngine(self , rsthis: & QQmlExtensionInterface) -> RetType;
}

  // proto:  void QQmlExtensionInterface::initializeEngine(QQmlEngine * engine, const char * uri);
impl<'a> /*trait*/ QQmlExtensionInterface_initializeEngine<()> for (&'a QQmlEngine, &'a  String) {
  fn initializeEngine(self , rsthis: & QQmlExtensionInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QQmlExtensionInterface16initializeEngineEP10QQmlEnginePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {_ZN22QQmlExtensionInterface16initializeEngineEP10QQmlEnginePKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// <= body block end

