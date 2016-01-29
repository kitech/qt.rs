// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qsgtexture.h
// dst-file: /src/quick/qsgtexture.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::core::qrect::QRectF; // 771
use super::super::core::qsize::QSize; // 771
// use super::qsgtexture::QSGTexture; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSGTexture_Class_Size() -> c_int;
  // proto:  QSGTexture * QSGTexture::removedFromAtlas();
  fn _ZNK10QSGTexture16removedFromAtlasEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGTexture::updateBindOptions(bool force);
  fn _ZN10QSGTexture17updateBindOptionsEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QSGTexture::bind();
  fn _ZN10QSGTexture4bindEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QSGTexture::isAtlasTexture();
  fn _ZNK10QSGTexture14isAtlasTextureEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QSGTexture::~QSGTexture();
  fn _ZN10QSGTextureD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSGTexture::QSGTexture();
  fn _ZN10QSGTextureC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QSGTexture::metaObject();
  fn _ZNK10QSGTexture10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QRectF QSGTexture::normalizedTextureSubRect();
  fn _ZNK10QSGTexture24normalizedTextureSubRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QSGTexture::textureSize();
  fn _ZNK10QSGTexture11textureSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QSGTexture::hasAlphaChannel();
  fn _ZNK10QSGTexture15hasAlphaChannelEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QSGTexture::textureId();
  fn _ZNK10QSGTexture9textureIdEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QRectF QSGTexture::convertToNormalizedSourceRect(const QRectF & rect);
  fn _ZNK10QSGTexture29convertToNormalizedSourceRectERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QSGTexture::hasMipmaps();
  fn _ZNK10QSGTexture10hasMipmapsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QSGDynamicTexture_Class_Size() -> c_int;
  // proto:  const QMetaObject * QSGDynamicTexture::metaObject();
  fn _ZNK17QSGDynamicTexture10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QSGDynamicTexture::updateTexture();
  fn _ZN17QSGDynamicTexture13updateTextureEv(qthis: u64 /* *mut c_void*/) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QSGTexture)=1
#[derive(Default)]
pub struct QSGTexture {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QSGDynamicTexture)=1
#[derive(Default)]
pub struct QSGDynamicTexture {
  qbase: QSGTexture,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSGTexture {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGTexture {
    return QSGTexture{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSGTexture {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QSGTexture {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  QSGTexture * QSGTexture::removedFromAtlas();
impl /*struct*/ QSGTexture {
  pub fn removedFromAtlas<RetType, T: QSGTexture_removedFromAtlas<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removedFromAtlas(self);
    // return 1;
  }
}

pub trait QSGTexture_removedFromAtlas<RetType> {
  fn removedFromAtlas(self , rsthis: & QSGTexture) -> RetType;
}

  // proto:  QSGTexture * QSGTexture::removedFromAtlas();
impl<'a> /*trait*/ QSGTexture_removedFromAtlas<QSGTexture> for () {
  fn removedFromAtlas(self , rsthis: & QSGTexture) -> QSGTexture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSGTexture16removedFromAtlasEv()};
    let mut ret = unsafe {_ZNK10QSGTexture16removedFromAtlasEv(rsthis.qclsinst)};
    let mut ret1 = QSGTexture::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGTexture::updateBindOptions(bool force);
impl /*struct*/ QSGTexture {
  pub fn updateBindOptions<RetType, T: QSGTexture_updateBindOptions<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.updateBindOptions(self);
    // return 1;
  }
}

pub trait QSGTexture_updateBindOptions<RetType> {
  fn updateBindOptions(self , rsthis: & QSGTexture) -> RetType;
}

  // proto:  void QSGTexture::updateBindOptions(bool force);
impl<'a> /*trait*/ QSGTexture_updateBindOptions<()> for (i8) {
  fn updateBindOptions(self , rsthis: & QSGTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSGTexture17updateBindOptionsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QSGTexture17updateBindOptionsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSGTexture::bind();
impl /*struct*/ QSGTexture {
  pub fn bind<RetType, T: QSGTexture_bind<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bind(self);
    // return 1;
  }
}

pub trait QSGTexture_bind<RetType> {
  fn bind(self , rsthis: & QSGTexture) -> RetType;
}

  // proto:  void QSGTexture::bind();
impl<'a> /*trait*/ QSGTexture_bind<()> for () {
  fn bind(self , rsthis: & QSGTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSGTexture4bindEv()};
     unsafe {_ZN10QSGTexture4bindEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QSGTexture::isAtlasTexture();
impl /*struct*/ QSGTexture {
  pub fn isAtlasTexture<RetType, T: QSGTexture_isAtlasTexture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAtlasTexture(self);
    // return 1;
  }
}

pub trait QSGTexture_isAtlasTexture<RetType> {
  fn isAtlasTexture(self , rsthis: & QSGTexture) -> RetType;
}

  // proto:  bool QSGTexture::isAtlasTexture();
impl<'a> /*trait*/ QSGTexture_isAtlasTexture<i8> for () {
  fn isAtlasTexture(self , rsthis: & QSGTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSGTexture14isAtlasTextureEv()};
    let mut ret = unsafe {_ZNK10QSGTexture14isAtlasTextureEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSGTexture::~QSGTexture();
impl /*struct*/ QSGTexture {
  pub fn free<RetType, T: QSGTexture_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSGTexture_free<RetType> {
  fn free(self , rsthis: & QSGTexture) -> RetType;
}

  // proto:  void QSGTexture::~QSGTexture();
impl<'a> /*trait*/ QSGTexture_free<()> for () {
  fn free(self , rsthis: & QSGTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSGTextureD2Ev()};
     unsafe {_ZN10QSGTextureD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSGTexture::QSGTexture();
impl /*struct*/ QSGTexture {
  pub fn new<T: QSGTexture_new>(value: T) -> QSGTexture {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSGTexture_new {
  fn new(self) -> QSGTexture;
}

  // proto:  void QSGTexture::QSGTexture();
impl<'a> /*trait*/ QSGTexture_new for () {
  fn new(self) -> QSGTexture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QSGTextureC2Ev()};
    let ctysz: c_int = unsafe{QSGTexture_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN10QSGTextureC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSGTexture{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QSGTexture::metaObject();
impl /*struct*/ QSGTexture {
  pub fn metaObject<RetType, T: QSGTexture_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSGTexture_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSGTexture) -> RetType;
}

  // proto:  const QMetaObject * QSGTexture::metaObject();
impl<'a> /*trait*/ QSGTexture_metaObject<()> for () {
  fn metaObject(self , rsthis: & QSGTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSGTexture10metaObjectEv()};
     unsafe {_ZNK10QSGTexture10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRectF QSGTexture::normalizedTextureSubRect();
impl /*struct*/ QSGTexture {
  pub fn normalizedTextureSubRect<RetType, T: QSGTexture_normalizedTextureSubRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.normalizedTextureSubRect(self);
    // return 1;
  }
}

pub trait QSGTexture_normalizedTextureSubRect<RetType> {
  fn normalizedTextureSubRect(self , rsthis: & QSGTexture) -> RetType;
}

  // proto:  QRectF QSGTexture::normalizedTextureSubRect();
impl<'a> /*trait*/ QSGTexture_normalizedTextureSubRect<QRectF> for () {
  fn normalizedTextureSubRect(self , rsthis: & QSGTexture) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSGTexture24normalizedTextureSubRectEv()};
    let mut ret = unsafe {_ZNK10QSGTexture24normalizedTextureSubRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QSGTexture::textureSize();
impl /*struct*/ QSGTexture {
  pub fn textureSize<RetType, T: QSGTexture_textureSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textureSize(self);
    // return 1;
  }
}

pub trait QSGTexture_textureSize<RetType> {
  fn textureSize(self , rsthis: & QSGTexture) -> RetType;
}

  // proto:  QSize QSGTexture::textureSize();
impl<'a> /*trait*/ QSGTexture_textureSize<QSize> for () {
  fn textureSize(self , rsthis: & QSGTexture) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSGTexture11textureSizeEv()};
    let mut ret = unsafe {_ZNK10QSGTexture11textureSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSGTexture::hasAlphaChannel();
impl /*struct*/ QSGTexture {
  pub fn hasAlphaChannel<RetType, T: QSGTexture_hasAlphaChannel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasAlphaChannel(self);
    // return 1;
  }
}

pub trait QSGTexture_hasAlphaChannel<RetType> {
  fn hasAlphaChannel(self , rsthis: & QSGTexture) -> RetType;
}

  // proto:  bool QSGTexture::hasAlphaChannel();
impl<'a> /*trait*/ QSGTexture_hasAlphaChannel<i8> for () {
  fn hasAlphaChannel(self , rsthis: & QSGTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSGTexture15hasAlphaChannelEv()};
    let mut ret = unsafe {_ZNK10QSGTexture15hasAlphaChannelEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QSGTexture::textureId();
impl /*struct*/ QSGTexture {
  pub fn textureId<RetType, T: QSGTexture_textureId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textureId(self);
    // return 1;
  }
}

pub trait QSGTexture_textureId<RetType> {
  fn textureId(self , rsthis: & QSGTexture) -> RetType;
}

  // proto:  int QSGTexture::textureId();
impl<'a> /*trait*/ QSGTexture_textureId<i32> for () {
  fn textureId(self , rsthis: & QSGTexture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSGTexture9textureIdEv()};
    let mut ret = unsafe {_ZNK10QSGTexture9textureIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QRectF QSGTexture::convertToNormalizedSourceRect(const QRectF & rect);
impl /*struct*/ QSGTexture {
  pub fn convertToNormalizedSourceRect<RetType, T: QSGTexture_convertToNormalizedSourceRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.convertToNormalizedSourceRect(self);
    // return 1;
  }
}

pub trait QSGTexture_convertToNormalizedSourceRect<RetType> {
  fn convertToNormalizedSourceRect(self , rsthis: & QSGTexture) -> RetType;
}

  // proto:  QRectF QSGTexture::convertToNormalizedSourceRect(const QRectF & rect);
impl<'a> /*trait*/ QSGTexture_convertToNormalizedSourceRect<QRectF> for (&'a QRectF) {
  fn convertToNormalizedSourceRect(self , rsthis: & QSGTexture) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSGTexture29convertToNormalizedSourceRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QSGTexture29convertToNormalizedSourceRectERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSGTexture::hasMipmaps();
impl /*struct*/ QSGTexture {
  pub fn hasMipmaps<RetType, T: QSGTexture_hasMipmaps<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasMipmaps(self);
    // return 1;
  }
}

pub trait QSGTexture_hasMipmaps<RetType> {
  fn hasMipmaps(self , rsthis: & QSGTexture) -> RetType;
}

  // proto:  bool QSGTexture::hasMipmaps();
impl<'a> /*trait*/ QSGTexture_hasMipmaps<i8> for () {
  fn hasMipmaps(self , rsthis: & QSGTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QSGTexture10hasMipmapsEv()};
    let mut ret = unsafe {_ZNK10QSGTexture10hasMipmapsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSGDynamicTexture {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGDynamicTexture {
    return QSGDynamicTexture{qbase: QSGTexture::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSGDynamicTexture {
  type Target = QSGTexture;

  fn deref(&self) -> &QSGTexture {
    return & self.qbase;
  }
}
impl AsRef<QSGTexture> for QSGDynamicTexture {
  fn as_ref(& self) -> & QSGTexture {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QSGDynamicTexture::metaObject();
impl /*struct*/ QSGDynamicTexture {
  pub fn metaObject<RetType, T: QSGDynamicTexture_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSGDynamicTexture_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSGDynamicTexture) -> RetType;
}

  // proto:  const QMetaObject * QSGDynamicTexture::metaObject();
impl<'a> /*trait*/ QSGDynamicTexture_metaObject<()> for () {
  fn metaObject(self , rsthis: & QSGDynamicTexture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSGDynamicTexture10metaObjectEv()};
     unsafe {_ZNK17QSGDynamicTexture10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QSGDynamicTexture::updateTexture();
impl /*struct*/ QSGDynamicTexture {
  pub fn updateTexture<RetType, T: QSGDynamicTexture_updateTexture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.updateTexture(self);
    // return 1;
  }
}

pub trait QSGDynamicTexture_updateTexture<RetType> {
  fn updateTexture(self , rsthis: & QSGDynamicTexture) -> RetType;
}

  // proto:  bool QSGDynamicTexture::updateTexture();
impl<'a> /*trait*/ QSGDynamicTexture_updateTexture<i8> for () {
  fn updateTexture(self , rsthis: & QSGDynamicTexture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSGDynamicTexture13updateTextureEv()};
    let mut ret = unsafe {_ZN17QSGDynamicTexture13updateTextureEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

