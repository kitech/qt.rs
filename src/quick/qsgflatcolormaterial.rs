// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qsgflatcolormaterial.h
// dst-file: /src/quick/qsgflatcolormaterial.rs
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
use super::qsgmaterial::QSGMaterial; // 773
use std::ops::Deref;
use super::super::gui::qcolor::QColor; // 771
use super::qsgmaterial::QSGMaterialShader; // 773
use super::qsgmaterial::QSGMaterialType; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSGFlatColorMaterial_Class_Size() -> c_int;
  // proto:  void QSGFlatColorMaterial::setColor(const QColor & color);
  fn _ZN20QSGFlatColorMaterial8setColorERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSGFlatColorMaterial::QSGFlatColorMaterial();
  fn _ZN20QSGFlatColorMaterialC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QSGFlatColorMaterial::compare(const QSGMaterial * other);
  fn _ZNK20QSGFlatColorMaterial7compareEPK11QSGMaterial(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  QSGMaterialShader * QSGFlatColorMaterial::createShader();
  fn _ZNK20QSGFlatColorMaterial12createShaderEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSGMaterialType * QSGFlatColorMaterial::type();
  fn _ZNK20QSGFlatColorMaterial4typeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QColor & QSGFlatColorMaterial::color();
  fn _ZNK20QSGFlatColorMaterial5colorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QSGFlatColorMaterial)=1
#[derive(Default)]
pub struct QSGFlatColorMaterial {
  qbase: QSGMaterial,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSGFlatColorMaterial {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGFlatColorMaterial {
    return QSGFlatColorMaterial{qbase: QSGMaterial::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSGFlatColorMaterial {
  type Target = QSGMaterial;

  fn deref(&self) -> &QSGMaterial {
    return & self.qbase;
  }
}
impl AsRef<QSGMaterial> for QSGFlatColorMaterial {
  fn as_ref(& self) -> & QSGMaterial {
    return & self.qbase;
  }
}
  // proto:  void QSGFlatColorMaterial::setColor(const QColor & color);
impl /*struct*/ QSGFlatColorMaterial {
  pub fn setColor<RetType, T: QSGFlatColorMaterial_setColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColor(self);
    // return 1;
  }
}

pub trait QSGFlatColorMaterial_setColor<RetType> {
  fn setColor(self , rsthis: & QSGFlatColorMaterial) -> RetType;
}

  // proto:  void QSGFlatColorMaterial::setColor(const QColor & color);
impl<'a> /*trait*/ QSGFlatColorMaterial_setColor<()> for (&'a QColor) {
  fn setColor(self , rsthis: & QSGFlatColorMaterial) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QSGFlatColorMaterial8setColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QSGFlatColorMaterial8setColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSGFlatColorMaterial::QSGFlatColorMaterial();
impl /*struct*/ QSGFlatColorMaterial {
  pub fn new<T: QSGFlatColorMaterial_new>(value: T) -> QSGFlatColorMaterial {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSGFlatColorMaterial_new {
  fn new(self) -> QSGFlatColorMaterial;
}

  // proto:  void QSGFlatColorMaterial::QSGFlatColorMaterial();
impl<'a> /*trait*/ QSGFlatColorMaterial_new for () {
  fn new(self) -> QSGFlatColorMaterial {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QSGFlatColorMaterialC2Ev()};
    let ctysz: c_int = unsafe{QSGFlatColorMaterial_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN20QSGFlatColorMaterialC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSGFlatColorMaterial{qbase: QSGMaterial::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QSGFlatColorMaterial::compare(const QSGMaterial * other);
impl /*struct*/ QSGFlatColorMaterial {
  pub fn compare<RetType, T: QSGFlatColorMaterial_compare<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.compare(self);
    // return 1;
  }
}

pub trait QSGFlatColorMaterial_compare<RetType> {
  fn compare(self , rsthis: & QSGFlatColorMaterial) -> RetType;
}

  // proto:  int QSGFlatColorMaterial::compare(const QSGMaterial * other);
impl<'a> /*trait*/ QSGFlatColorMaterial_compare<i32> for (&'a QSGMaterial) {
  fn compare(self , rsthis: & QSGFlatColorMaterial) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QSGFlatColorMaterial7compareEPK11QSGMaterial()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QSGFlatColorMaterial7compareEPK11QSGMaterial(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSGMaterialShader * QSGFlatColorMaterial::createShader();
impl /*struct*/ QSGFlatColorMaterial {
  pub fn createShader<RetType, T: QSGFlatColorMaterial_createShader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createShader(self);
    // return 1;
  }
}

pub trait QSGFlatColorMaterial_createShader<RetType> {
  fn createShader(self , rsthis: & QSGFlatColorMaterial) -> RetType;
}

  // proto:  QSGMaterialShader * QSGFlatColorMaterial::createShader();
impl<'a> /*trait*/ QSGFlatColorMaterial_createShader<QSGMaterialShader> for () {
  fn createShader(self , rsthis: & QSGFlatColorMaterial) -> QSGMaterialShader {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QSGFlatColorMaterial12createShaderEv()};
    let mut ret = unsafe {_ZNK20QSGFlatColorMaterial12createShaderEv(rsthis.qclsinst)};
    let mut ret1 = QSGMaterialShader::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSGMaterialType * QSGFlatColorMaterial::type();
impl /*struct*/ QSGFlatColorMaterial {
  pub fn type_<RetType, T: QSGFlatColorMaterial_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QSGFlatColorMaterial_type_<RetType> {
  fn type_(self , rsthis: & QSGFlatColorMaterial) -> RetType;
}

  // proto:  QSGMaterialType * QSGFlatColorMaterial::type();
impl<'a> /*trait*/ QSGFlatColorMaterial_type_<QSGMaterialType> for () {
  fn type_(self , rsthis: & QSGFlatColorMaterial) -> QSGMaterialType {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QSGFlatColorMaterial4typeEv()};
    let mut ret = unsafe {_ZNK20QSGFlatColorMaterial4typeEv(rsthis.qclsinst)};
    let mut ret1 = QSGMaterialType::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QColor & QSGFlatColorMaterial::color();
impl /*struct*/ QSGFlatColorMaterial {
  pub fn color<RetType, T: QSGFlatColorMaterial_color<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.color(self);
    // return 1;
  }
}

pub trait QSGFlatColorMaterial_color<RetType> {
  fn color(self , rsthis: & QSGFlatColorMaterial) -> RetType;
}

  // proto:  const QColor & QSGFlatColorMaterial::color();
impl<'a> /*trait*/ QSGFlatColorMaterial_color<QColor> for () {
  fn color(self , rsthis: & QSGFlatColorMaterial) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QSGFlatColorMaterial5colorEv()};
    let mut ret = unsafe {_ZNK20QSGFlatColorMaterial5colorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

