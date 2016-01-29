// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtGui/qopenglversionfunctions.h
// dst-file: /src/gui/qopenglversionfunctions.rs
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
use super::qopenglcontext::*; // 773
// use super::qopenglversionfunctions::QAbstractOpenGLFunctionsPrivate; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QOpenGLVersionFunctionsBackend_Class_Size() -> c_int;
  // proto:  void QOpenGLVersionFunctionsBackend::QOpenGLVersionFunctionsBackend(QOpenGLContext * ctx);
  fn C_ZN30QOpenGLVersionFunctionsBackendC2EP14QOpenGLContext(arg0: *mut c_void) -> u64;
  fn QAbstractOpenGLFunctions_Class_Size() -> c_int;
  // proto:  bool QAbstractOpenGLFunctions::initializeOpenGLFunctions();
  fn C_ZN24QAbstractOpenGLFunctions25initializeOpenGLFunctionsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QAbstractOpenGLFunctionsPrivate * QAbstractOpenGLFunctions::d_func();
  fn C_ZN24QAbstractOpenGLFunctions6d_funcEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAbstractOpenGLFunctions::~QAbstractOpenGLFunctions();
  fn C_ZN24QAbstractOpenGLFunctionsD2Ev(qthis: u64 /* *mut c_void*/);
  fn QOpenGLVersionStatus_Class_Size() -> c_int;
  // proto:  void QOpenGLVersionStatus::QOpenGLVersionStatus();
  fn C_ZN20QOpenGLVersionStatusC2Ev() -> u64;
} // <= ext block end

// body block begin =>
// class sizeof(QOpenGLVersionFunctionsBackend)=1
#[derive(Default)]
pub struct QOpenGLVersionFunctionsBackend {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QAbstractOpenGLFunctions)=16
#[derive(Default)]
pub struct QAbstractOpenGLFunctions {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLVersionStatus)=1
#[derive(Default)]
pub struct QOpenGLVersionStatus {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QOpenGLVersionFunctionsBackend {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLVersionFunctionsBackend {
    return QOpenGLVersionFunctionsBackend{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QOpenGLVersionFunctionsBackend::QOpenGLVersionFunctionsBackend(QOpenGLContext * ctx);
impl /*struct*/ QOpenGLVersionFunctionsBackend {
  pub fn new<T: QOpenGLVersionFunctionsBackend_new>(value: T) -> QOpenGLVersionFunctionsBackend {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLVersionFunctionsBackend_new {
  fn new(self) -> QOpenGLVersionFunctionsBackend;
}

  // proto:  void QOpenGLVersionFunctionsBackend::QOpenGLVersionFunctionsBackend(QOpenGLContext * ctx);
impl<'a> /*trait*/ QOpenGLVersionFunctionsBackend_new for (&'a QOpenGLContext) {
  fn new(self) -> QOpenGLVersionFunctionsBackend {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QOpenGLVersionFunctionsBackendC2EP14QOpenGLContext()};
    let ctysz: c_int = unsafe{QOpenGLVersionFunctionsBackend_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN30QOpenGLVersionFunctionsBackendC2EP14QOpenGLContext(arg0)};
    let rsthis = QOpenGLVersionFunctionsBackend{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAbstractOpenGLFunctions {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractOpenGLFunctions {
    return QAbstractOpenGLFunctions{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QAbstractOpenGLFunctions::initializeOpenGLFunctions();
impl /*struct*/ QAbstractOpenGLFunctions {
  pub fn initializeOpenGLFunctions<RetType, T: QAbstractOpenGLFunctions_initializeOpenGLFunctions<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.initializeOpenGLFunctions(self);
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctions_initializeOpenGLFunctions<RetType> {
  fn initializeOpenGLFunctions(self , rsthis: & QAbstractOpenGLFunctions) -> RetType;
}

  // proto:  bool QAbstractOpenGLFunctions::initializeOpenGLFunctions();
impl<'a> /*trait*/ QAbstractOpenGLFunctions_initializeOpenGLFunctions<i8> for () {
  fn initializeOpenGLFunctions(self , rsthis: & QAbstractOpenGLFunctions) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractOpenGLFunctions25initializeOpenGLFunctionsEv()};
    let mut ret = unsafe {C_ZN24QAbstractOpenGLFunctions25initializeOpenGLFunctionsEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QAbstractOpenGLFunctionsPrivate * QAbstractOpenGLFunctions::d_func();
impl /*struct*/ QAbstractOpenGLFunctions {
  pub fn d_func<RetType, T: QAbstractOpenGLFunctions_d_func<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.d_func(self);
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctions_d_func<RetType> {
  fn d_func(self , rsthis: & QAbstractOpenGLFunctions) -> RetType;
}

  // proto:  QAbstractOpenGLFunctionsPrivate * QAbstractOpenGLFunctions::d_func();
impl<'a> /*trait*/ QAbstractOpenGLFunctions_d_func<u64> for () {
  fn d_func(self , rsthis: & QAbstractOpenGLFunctions) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractOpenGLFunctions6d_funcEv()};
    let mut ret = unsafe {C_ZN24QAbstractOpenGLFunctions6d_funcEv(rsthis.qclsinst)};
    return ret as u64; // 2
    // return 1;
  }
}

  // proto:  void QAbstractOpenGLFunctions::~QAbstractOpenGLFunctions();
impl /*struct*/ QAbstractOpenGLFunctions {
  pub fn free<RetType, T: QAbstractOpenGLFunctions_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QAbstractOpenGLFunctions_free<RetType> {
  fn free(self , rsthis: & QAbstractOpenGLFunctions) -> RetType;
}

  // proto:  void QAbstractOpenGLFunctions::~QAbstractOpenGLFunctions();
impl<'a> /*trait*/ QAbstractOpenGLFunctions_free<()> for () {
  fn free(self , rsthis: & QAbstractOpenGLFunctions) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAbstractOpenGLFunctionsD2Ev()};
     unsafe {C_ZN24QAbstractOpenGLFunctionsD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLVersionStatus {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLVersionStatus {
    return QOpenGLVersionStatus{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QOpenGLVersionStatus::QOpenGLVersionStatus();
impl /*struct*/ QOpenGLVersionStatus {
  pub fn new<T: QOpenGLVersionStatus_new>(value: T) -> QOpenGLVersionStatus {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLVersionStatus_new {
  fn new(self) -> QOpenGLVersionStatus;
}

  // proto:  void QOpenGLVersionStatus::QOpenGLVersionStatus();
impl<'a> /*trait*/ QOpenGLVersionStatus_new for () {
  fn new(self) -> QOpenGLVersionStatus {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QOpenGLVersionStatusC2Ev()};
    let ctysz: c_int = unsafe{QOpenGLVersionStatus_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN20QOpenGLVersionStatusC2Ev()};
    let rsthis = QOpenGLVersionStatus{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

