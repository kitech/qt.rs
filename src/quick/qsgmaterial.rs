// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qsgmaterial.h
// dst-file: /src/quick/qsgmaterial.rs
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
// use super::qsgmaterial::QSGMaterialType; // 773
// use super::qsgmaterial::QSGMaterialShader; // 773
// use super::qsgmaterial::QSGMaterial; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSGMaterial_Class_Size() -> c_int;
  // proto:  void QSGMaterial::QSGMaterial();
  fn _ZN11QSGMaterialC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSGMaterial::QSGMaterial(const QSGMaterial & );
  fn _ZN11QSGMaterialC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSGMaterialType * QSGMaterial::type();
  fn _ZNK11QSGMaterial4typeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSGMaterialShader * QSGMaterial::createShader();
  fn _ZNK11QSGMaterial12createShaderEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QSGMaterial::compare(const QSGMaterial * other);
  fn _ZNK11QSGMaterial7compareEPKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  void QSGMaterial::~QSGMaterial();
  fn _ZN11QSGMaterialD2Ev(qthis: u64 /* *mut c_void*/);
  fn QSGMaterialShader_Class_Size() -> c_int;
  // proto:  void QSGMaterialShader::activate();
  fn _ZN17QSGMaterialShader8activateEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSGMaterialShader::QSGMaterialShader();
  fn _ZN17QSGMaterialShaderC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const char *const * QSGMaterialShader::attributeNames();
  fn _ZNK17QSGMaterialShader14attributeNamesEv(qthis: u64 /* *mut c_void*/) -> *mut c_char;
  // proto:  void QSGMaterialShader::~QSGMaterialShader();
  fn _ZN17QSGMaterialShaderD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSGMaterialShader::deactivate();
  fn _ZN17QSGMaterialShader10deactivateEv(qthis: u64 /* *mut c_void*/);
  // proto:  QOpenGLShaderProgram * QSGMaterialShader::program();
  fn _ZN17QSGMaterialShader7programEv(qthis: u64 /* *mut c_void*/);
  fn QSGMaterialType_Class_Size() -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QSGMaterial)=1
#[derive(Default)]
pub struct QSGMaterial {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QSGMaterialShader)=1
#[derive(Default)]
pub struct QSGMaterialShader {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QSGMaterialType)=1
#[derive(Default)]
pub struct QSGMaterialType {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSGMaterial {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGMaterial {
    return QSGMaterial{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QSGMaterial::QSGMaterial();
impl /*struct*/ QSGMaterial {
  pub fn new<T: QSGMaterial_new>(value: T) -> QSGMaterial {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSGMaterial_new {
  fn new(self) -> QSGMaterial;
}

  // proto:  void QSGMaterial::QSGMaterial();
impl<'a> /*trait*/ QSGMaterial_new for () {
  fn new(self) -> QSGMaterial {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSGMaterialC2Ev()};
    let ctysz: c_int = unsafe{QSGMaterial_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN11QSGMaterialC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSGMaterial{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSGMaterial::QSGMaterial(const QSGMaterial & );
impl<'a> /*trait*/ QSGMaterial_new for (&'a QSGMaterial) {
  fn new(self) -> QSGMaterial {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSGMaterialC2ERKS_()};
    let ctysz: c_int = unsafe{QSGMaterial_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QSGMaterialC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSGMaterial{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSGMaterialType * QSGMaterial::type();
impl /*struct*/ QSGMaterial {
  pub fn type_<RetType, T: QSGMaterial_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QSGMaterial_type_<RetType> {
  fn type_(self , rsthis: & QSGMaterial) -> RetType;
}

  // proto:  QSGMaterialType * QSGMaterial::type();
impl<'a> /*trait*/ QSGMaterial_type_<QSGMaterialType> for () {
  fn type_(self , rsthis: & QSGMaterial) -> QSGMaterialType {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSGMaterial4typeEv()};
    let mut ret = unsafe {_ZNK11QSGMaterial4typeEv(rsthis.qclsinst)};
    let mut ret1 = QSGMaterialType::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSGMaterialShader * QSGMaterial::createShader();
impl /*struct*/ QSGMaterial {
  pub fn createShader<RetType, T: QSGMaterial_createShader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createShader(self);
    // return 1;
  }
}

pub trait QSGMaterial_createShader<RetType> {
  fn createShader(self , rsthis: & QSGMaterial) -> RetType;
}

  // proto:  QSGMaterialShader * QSGMaterial::createShader();
impl<'a> /*trait*/ QSGMaterial_createShader<QSGMaterialShader> for () {
  fn createShader(self , rsthis: & QSGMaterial) -> QSGMaterialShader {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSGMaterial12createShaderEv()};
    let mut ret = unsafe {_ZNK11QSGMaterial12createShaderEv(rsthis.qclsinst)};
    let mut ret1 = QSGMaterialShader::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QSGMaterial::compare(const QSGMaterial * other);
impl /*struct*/ QSGMaterial {
  pub fn compare<RetType, T: QSGMaterial_compare<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.compare(self);
    // return 1;
  }
}

pub trait QSGMaterial_compare<RetType> {
  fn compare(self , rsthis: & QSGMaterial) -> RetType;
}

  // proto:  int QSGMaterial::compare(const QSGMaterial * other);
impl<'a> /*trait*/ QSGMaterial_compare<i32> for (&'a QSGMaterial) {
  fn compare(self , rsthis: & QSGMaterial) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSGMaterial7compareEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QSGMaterial7compareEPKS_(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSGMaterial::~QSGMaterial();
impl /*struct*/ QSGMaterial {
  pub fn free<RetType, T: QSGMaterial_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSGMaterial_free<RetType> {
  fn free(self , rsthis: & QSGMaterial) -> RetType;
}

  // proto:  void QSGMaterial::~QSGMaterial();
impl<'a> /*trait*/ QSGMaterial_free<()> for () {
  fn free(self , rsthis: & QSGMaterial) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSGMaterialD2Ev()};
     unsafe {_ZN11QSGMaterialD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSGMaterialShader {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGMaterialShader {
    return QSGMaterialShader{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QSGMaterialShader::activate();
impl /*struct*/ QSGMaterialShader {
  pub fn activate<RetType, T: QSGMaterialShader_activate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activate(self);
    // return 1;
  }
}

pub trait QSGMaterialShader_activate<RetType> {
  fn activate(self , rsthis: & QSGMaterialShader) -> RetType;
}

  // proto:  void QSGMaterialShader::activate();
impl<'a> /*trait*/ QSGMaterialShader_activate<()> for () {
  fn activate(self , rsthis: & QSGMaterialShader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSGMaterialShader8activateEv()};
     unsafe {_ZN17QSGMaterialShader8activateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSGMaterialShader::QSGMaterialShader();
impl /*struct*/ QSGMaterialShader {
  pub fn new<T: QSGMaterialShader_new>(value: T) -> QSGMaterialShader {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSGMaterialShader_new {
  fn new(self) -> QSGMaterialShader;
}

  // proto:  void QSGMaterialShader::QSGMaterialShader();
impl<'a> /*trait*/ QSGMaterialShader_new for () {
  fn new(self) -> QSGMaterialShader {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSGMaterialShaderC2Ev()};
    let ctysz: c_int = unsafe{QSGMaterialShader_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN17QSGMaterialShaderC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSGMaterialShader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const char *const * QSGMaterialShader::attributeNames();
impl /*struct*/ QSGMaterialShader {
  pub fn attributeNames<RetType, T: QSGMaterialShader_attributeNames<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.attributeNames(self);
    // return 1;
  }
}

pub trait QSGMaterialShader_attributeNames<RetType> {
  fn attributeNames(self , rsthis: & QSGMaterialShader) -> RetType;
}

  // proto:  const char *const * QSGMaterialShader::attributeNames();
impl<'a> /*trait*/ QSGMaterialShader_attributeNames<Vec<String>> for () {
  fn attributeNames(self , rsthis: & QSGMaterialShader) -> Vec<String> {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSGMaterialShader14attributeNamesEv()};
      let mut ret = unsafe {_ZNK17QSGMaterialShader14attributeNamesEv(rsthis.qclsinst)};
      let ret1: Vec<String> = Vec::new();
      return ret1 as Vec<String>;
    // return 1;
  }
}

  // proto:  void QSGMaterialShader::~QSGMaterialShader();
impl /*struct*/ QSGMaterialShader {
  pub fn free<RetType, T: QSGMaterialShader_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSGMaterialShader_free<RetType> {
  fn free(self , rsthis: & QSGMaterialShader) -> RetType;
}

  // proto:  void QSGMaterialShader::~QSGMaterialShader();
impl<'a> /*trait*/ QSGMaterialShader_free<()> for () {
  fn free(self , rsthis: & QSGMaterialShader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSGMaterialShaderD2Ev()};
     unsafe {_ZN17QSGMaterialShaderD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSGMaterialShader::deactivate();
impl /*struct*/ QSGMaterialShader {
  pub fn deactivate<RetType, T: QSGMaterialShader_deactivate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.deactivate(self);
    // return 1;
  }
}

pub trait QSGMaterialShader_deactivate<RetType> {
  fn deactivate(self , rsthis: & QSGMaterialShader) -> RetType;
}

  // proto:  void QSGMaterialShader::deactivate();
impl<'a> /*trait*/ QSGMaterialShader_deactivate<()> for () {
  fn deactivate(self , rsthis: & QSGMaterialShader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSGMaterialShader10deactivateEv()};
     unsafe {_ZN17QSGMaterialShader10deactivateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QOpenGLShaderProgram * QSGMaterialShader::program();
impl /*struct*/ QSGMaterialShader {
  pub fn program<RetType, T: QSGMaterialShader_program<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.program(self);
    // return 1;
  }
}

pub trait QSGMaterialShader_program<RetType> {
  fn program(self , rsthis: & QSGMaterialShader) -> RetType;
}

  // proto:  QOpenGLShaderProgram * QSGMaterialShader::program();
impl<'a> /*trait*/ QSGMaterialShader_program<()> for () {
  fn program(self , rsthis: & QSGMaterialShader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSGMaterialShader7programEv()};
     unsafe {_ZN17QSGMaterialShader7programEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSGMaterialType {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGMaterialType {
    return QSGMaterialType{qclsinst: qthis, ..Default::default()};
  }
}
// <= body block end

