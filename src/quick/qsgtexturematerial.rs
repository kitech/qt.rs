// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qsgtexturematerial.h
// dst-file: /src/quick/qsgtexturematerial.rs
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
// use super::qsgtexturematerial::QSGOpaqueTextureMaterial; // 773
use std::ops::Deref;
use super::qsgmaterial::QSGMaterialType; // 773
use super::qsgmaterial::QSGMaterialShader; // 773
use super::qsgmaterial::QSGMaterial; // 773
use super::qsgtexture::QSGTexture; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSGTextureMaterial_Class_Size() -> c_int;
  // proto:  QSGMaterialType * QSGTextureMaterial::type();
  fn _ZNK18QSGTextureMaterial4typeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSGMaterialShader * QSGTextureMaterial::createShader();
  fn _ZNK18QSGTextureMaterial12createShaderEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QSGOpaqueTextureMaterial_Class_Size() -> c_int;
  // proto:  QSGTexture * QSGOpaqueTextureMaterial::texture();
  fn _ZNK24QSGOpaqueTextureMaterial7textureEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGOpaqueTextureMaterial::setTexture(QSGTexture * texture);
  fn _ZN24QSGOpaqueTextureMaterial10setTextureEP10QSGTexture(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSGMaterialShader * QSGOpaqueTextureMaterial::createShader();
  fn _ZNK24QSGOpaqueTextureMaterial12createShaderEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QSGOpaqueTextureMaterial::compare(const QSGMaterial * other);
  fn _ZNK24QSGOpaqueTextureMaterial7compareEPK11QSGMaterial(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  QSGMaterialType * QSGOpaqueTextureMaterial::type();
  fn _ZNK24QSGOpaqueTextureMaterial4typeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGOpaqueTextureMaterial::QSGOpaqueTextureMaterial();
  fn _ZN24QSGOpaqueTextureMaterialC2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QSGTextureMaterial)=1
#[derive(Default)]
pub struct QSGTextureMaterial {
  qbase: QSGOpaqueTextureMaterial,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QSGOpaqueTextureMaterial)=1
#[derive(Default)]
pub struct QSGOpaqueTextureMaterial {
  qbase: QSGMaterial,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSGTextureMaterial {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGTextureMaterial {
    return QSGTextureMaterial{qbase: QSGOpaqueTextureMaterial::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSGTextureMaterial {
  type Target = QSGOpaqueTextureMaterial;

  fn deref(&self) -> &QSGOpaqueTextureMaterial {
    return & self.qbase;
  }
}
impl AsRef<QSGOpaqueTextureMaterial> for QSGTextureMaterial {
  fn as_ref(& self) -> & QSGOpaqueTextureMaterial {
    return & self.qbase;
  }
}
  // proto:  QSGMaterialType * QSGTextureMaterial::type();
impl /*struct*/ QSGTextureMaterial {
  pub fn type_<RetType, T: QSGTextureMaterial_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QSGTextureMaterial_type_<RetType> {
  fn type_(self , rsthis: & QSGTextureMaterial) -> RetType;
}

  // proto:  QSGMaterialType * QSGTextureMaterial::type();
impl<'a> /*trait*/ QSGTextureMaterial_type_<QSGMaterialType> for () {
  fn type_(self , rsthis: & QSGTextureMaterial) -> QSGMaterialType {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QSGTextureMaterial4typeEv()};
    let mut ret = unsafe {_ZNK18QSGTextureMaterial4typeEv(rsthis.qclsinst)};
    let mut ret1 = QSGMaterialType::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSGMaterialShader * QSGTextureMaterial::createShader();
impl /*struct*/ QSGTextureMaterial {
  pub fn createShader<RetType, T: QSGTextureMaterial_createShader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createShader(self);
    // return 1;
  }
}

pub trait QSGTextureMaterial_createShader<RetType> {
  fn createShader(self , rsthis: & QSGTextureMaterial) -> RetType;
}

  // proto:  QSGMaterialShader * QSGTextureMaterial::createShader();
impl<'a> /*trait*/ QSGTextureMaterial_createShader<QSGMaterialShader> for () {
  fn createShader(self , rsthis: & QSGTextureMaterial) -> QSGMaterialShader {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QSGTextureMaterial12createShaderEv()};
    let mut ret = unsafe {_ZNK18QSGTextureMaterial12createShaderEv(rsthis.qclsinst)};
    let mut ret1 = QSGMaterialShader::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSGOpaqueTextureMaterial {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGOpaqueTextureMaterial {
    return QSGOpaqueTextureMaterial{qbase: QSGMaterial::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSGOpaqueTextureMaterial {
  type Target = QSGMaterial;

  fn deref(&self) -> &QSGMaterial {
    return & self.qbase;
  }
}
impl AsRef<QSGMaterial> for QSGOpaqueTextureMaterial {
  fn as_ref(& self) -> & QSGMaterial {
    return & self.qbase;
  }
}
  // proto:  QSGTexture * QSGOpaqueTextureMaterial::texture();
impl /*struct*/ QSGOpaqueTextureMaterial {
  pub fn texture<RetType, T: QSGOpaqueTextureMaterial_texture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.texture(self);
    // return 1;
  }
}

pub trait QSGOpaqueTextureMaterial_texture<RetType> {
  fn texture(self , rsthis: & QSGOpaqueTextureMaterial) -> RetType;
}

  // proto:  QSGTexture * QSGOpaqueTextureMaterial::texture();
impl<'a> /*trait*/ QSGOpaqueTextureMaterial_texture<QSGTexture> for () {
  fn texture(self , rsthis: & QSGOpaqueTextureMaterial) -> QSGTexture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QSGOpaqueTextureMaterial7textureEv()};
    let mut ret = unsafe {_ZNK24QSGOpaqueTextureMaterial7textureEv(rsthis.qclsinst)};
    let mut ret1 = QSGTexture::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGOpaqueTextureMaterial::setTexture(QSGTexture * texture);
impl /*struct*/ QSGOpaqueTextureMaterial {
  pub fn setTexture<RetType, T: QSGOpaqueTextureMaterial_setTexture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTexture(self);
    // return 1;
  }
}

pub trait QSGOpaqueTextureMaterial_setTexture<RetType> {
  fn setTexture(self , rsthis: & QSGOpaqueTextureMaterial) -> RetType;
}

  // proto:  void QSGOpaqueTextureMaterial::setTexture(QSGTexture * texture);
impl<'a> /*trait*/ QSGOpaqueTextureMaterial_setTexture<()> for (&'a QSGTexture) {
  fn setTexture(self , rsthis: & QSGOpaqueTextureMaterial) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QSGOpaqueTextureMaterial10setTextureEP10QSGTexture()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN24QSGOpaqueTextureMaterial10setTextureEP10QSGTexture(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSGMaterialShader * QSGOpaqueTextureMaterial::createShader();
impl /*struct*/ QSGOpaqueTextureMaterial {
  pub fn createShader<RetType, T: QSGOpaqueTextureMaterial_createShader<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createShader(self);
    // return 1;
  }
}

pub trait QSGOpaqueTextureMaterial_createShader<RetType> {
  fn createShader(self , rsthis: & QSGOpaqueTextureMaterial) -> RetType;
}

  // proto:  QSGMaterialShader * QSGOpaqueTextureMaterial::createShader();
impl<'a> /*trait*/ QSGOpaqueTextureMaterial_createShader<QSGMaterialShader> for () {
  fn createShader(self , rsthis: & QSGOpaqueTextureMaterial) -> QSGMaterialShader {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QSGOpaqueTextureMaterial12createShaderEv()};
    let mut ret = unsafe {_ZNK24QSGOpaqueTextureMaterial12createShaderEv(rsthis.qclsinst)};
    let mut ret1 = QSGMaterialShader::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QSGOpaqueTextureMaterial::compare(const QSGMaterial * other);
impl /*struct*/ QSGOpaqueTextureMaterial {
  pub fn compare<RetType, T: QSGOpaqueTextureMaterial_compare<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.compare(self);
    // return 1;
  }
}

pub trait QSGOpaqueTextureMaterial_compare<RetType> {
  fn compare(self , rsthis: & QSGOpaqueTextureMaterial) -> RetType;
}

  // proto:  int QSGOpaqueTextureMaterial::compare(const QSGMaterial * other);
impl<'a> /*trait*/ QSGOpaqueTextureMaterial_compare<i32> for (&'a QSGMaterial) {
  fn compare(self , rsthis: & QSGOpaqueTextureMaterial) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QSGOpaqueTextureMaterial7compareEPK11QSGMaterial()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK24QSGOpaqueTextureMaterial7compareEPK11QSGMaterial(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSGMaterialType * QSGOpaqueTextureMaterial::type();
impl /*struct*/ QSGOpaqueTextureMaterial {
  pub fn type_<RetType, T: QSGOpaqueTextureMaterial_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QSGOpaqueTextureMaterial_type_<RetType> {
  fn type_(self , rsthis: & QSGOpaqueTextureMaterial) -> RetType;
}

  // proto:  QSGMaterialType * QSGOpaqueTextureMaterial::type();
impl<'a> /*trait*/ QSGOpaqueTextureMaterial_type_<QSGMaterialType> for () {
  fn type_(self , rsthis: & QSGOpaqueTextureMaterial) -> QSGMaterialType {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QSGOpaqueTextureMaterial4typeEv()};
    let mut ret = unsafe {_ZNK24QSGOpaqueTextureMaterial4typeEv(rsthis.qclsinst)};
    let mut ret1 = QSGMaterialType::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGOpaqueTextureMaterial::QSGOpaqueTextureMaterial();
impl /*struct*/ QSGOpaqueTextureMaterial {
  pub fn new<T: QSGOpaqueTextureMaterial_new>(value: T) -> QSGOpaqueTextureMaterial {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSGOpaqueTextureMaterial_new {
  fn new(self) -> QSGOpaqueTextureMaterial;
}

  // proto:  void QSGOpaqueTextureMaterial::QSGOpaqueTextureMaterial();
impl<'a> /*trait*/ QSGOpaqueTextureMaterial_new for () {
  fn new(self) -> QSGOpaqueTextureMaterial {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QSGOpaqueTextureMaterialC2Ev()};
    let ctysz: c_int = unsafe{QSGOpaqueTextureMaterial_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN24QSGOpaqueTextureMaterialC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSGOpaqueTextureMaterial{qbase: QSGMaterial::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

