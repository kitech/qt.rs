// auto generated, do not modify.
// created: Wed Dec 30 23:22:52 2015
// src-file: /QtGui/qopenglframebufferobject.h
// dst-file: /src/gui/qopenglframebufferobject.rs
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
use super::super::core::qsize::QSize; // 771
// use super::qopenglframebufferobject::QOpenGLFramebufferObjectFormat; // 773
use super::qimage::QImage; // 773
use super::super::core::qrect::QRect; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QOpenGLFramebufferObjectFormat_Class_Size() -> c_int;
  // proto:  void QOpenGLFramebufferObjectFormat::~QOpenGLFramebufferObjectFormat();
  fn _ZN30QOpenGLFramebufferObjectFormatD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QOpenGLFramebufferObjectFormat::setTextureTarget(GLenum target);
  fn _ZN30QOpenGLFramebufferObjectFormat16setTextureTargetEj(qthis: u64 /* *mut c_void*/, arg0: c_uint);
  // proto:  void QOpenGLFramebufferObjectFormat::setInternalTextureFormat(GLenum internalTextureFormat);
  fn _ZN30QOpenGLFramebufferObjectFormat24setInternalTextureFormatEj(qthis: u64 /* *mut c_void*/, arg0: c_uint);
  // proto:  void QOpenGLFramebufferObjectFormat::QOpenGLFramebufferObjectFormat(const QOpenGLFramebufferObjectFormat & other);
  fn dector_ZN30QOpenGLFramebufferObjectFormatC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN30QOpenGLFramebufferObjectFormatC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QOpenGLFramebufferObjectFormat::mipmap();
  fn _ZNK30QOpenGLFramebufferObjectFormat6mipmapEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  GLenum QOpenGLFramebufferObjectFormat::textureTarget();
  fn _ZNK30QOpenGLFramebufferObjectFormat13textureTargetEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  void QOpenGLFramebufferObjectFormat::setSamples(int samples);
  fn _ZN30QOpenGLFramebufferObjectFormat10setSamplesEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QOpenGLFramebufferObjectFormat::QOpenGLFramebufferObjectFormat();
  fn dector_ZN30QOpenGLFramebufferObjectFormatC1Ev() -> *mut c_void;
  fn _ZN30QOpenGLFramebufferObjectFormatC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QOpenGLFramebufferObjectFormat::setMipmap(bool enabled);
  fn _ZN30QOpenGLFramebufferObjectFormat9setMipmapEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  GLenum QOpenGLFramebufferObjectFormat::internalTextureFormat();
  fn _ZNK30QOpenGLFramebufferObjectFormat21internalTextureFormatEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  int QOpenGLFramebufferObjectFormat::samples();
  fn _ZNK30QOpenGLFramebufferObjectFormat7samplesEv(qthis: u64 /* *mut c_void*/) -> c_int;
  fn QOpenGLFramebufferObject_Class_Size() -> c_int;
  // proto:  bool QOpenGLFramebufferObject::isValid();
  fn _ZNK24QOpenGLFramebufferObject7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  GLuint QOpenGLFramebufferObject::takeTexture();
  fn _ZN24QOpenGLFramebufferObject11takeTextureEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  void QOpenGLFramebufferObject::QOpenGLFramebufferObject(const QSize & size, const QOpenGLFramebufferObjectFormat & format);
  fn dector_ZN24QOpenGLFramebufferObjectC1ERK5QSizeRK30QOpenGLFramebufferObjectFormat(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN24QOpenGLFramebufferObjectC1ERK5QSizeRK30QOpenGLFramebufferObjectFormat(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto: static bool QOpenGLFramebufferObject::bindDefault();
  fn _ZN24QOpenGLFramebufferObject11bindDefaultEv() -> c_char;
  // proto: static bool QOpenGLFramebufferObject::hasOpenGLFramebufferBlit();
  fn _ZN24QOpenGLFramebufferObject24hasOpenGLFramebufferBlitEv() -> c_char;
  // proto:  GLuint QOpenGLFramebufferObject::texture();
  fn _ZNK24QOpenGLFramebufferObject7textureEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  void QOpenGLFramebufferObject::QOpenGLFramebufferObject(const QSize & size, GLenum target);
  fn dector_ZN24QOpenGLFramebufferObjectC1ERK5QSizej(arg0: *mut c_void, arg1: c_uint) -> *mut c_void;
  fn _ZN24QOpenGLFramebufferObjectC1ERK5QSizej(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_uint);
  // proto:  bool QOpenGLFramebufferObject::release();
  fn _ZN24QOpenGLFramebufferObject7releaseEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static bool QOpenGLFramebufferObject::hasOpenGLFramebufferObjects();
  fn _ZN24QOpenGLFramebufferObject27hasOpenGLFramebufferObjectsEv() -> c_char;
  // proto:  QImage QOpenGLFramebufferObject::toImage(bool flipped);
  fn _ZNK24QOpenGLFramebufferObject7toImageEb(qthis: u64 /* *mut c_void*/, arg0: c_char) -> *mut c_void;
  // proto:  GLuint QOpenGLFramebufferObject::handle();
  fn _ZNK24QOpenGLFramebufferObject6handleEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  int QOpenGLFramebufferObject::height();
  fn _ZNK24QOpenGLFramebufferObject6heightEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto: static void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject * target, QOpenGLFramebufferObject * source, GLbitfield buffers, GLenum filter);
  fn _ZN24QOpenGLFramebufferObject15blitFramebufferEPS_S0_jj(arg0: *mut c_void, arg1: *mut c_void, arg2: c_uint, arg3: c_uint);
  // proto:  void QOpenGLFramebufferObject::QOpenGLFramebufferObject(int width, int height, const QOpenGLFramebufferObjectFormat & format);
  fn dector_ZN24QOpenGLFramebufferObjectC1EiiRK30QOpenGLFramebufferObjectFormat(arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  fn _ZN24QOpenGLFramebufferObjectC1EiiRK30QOpenGLFramebufferObjectFormat(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void);
  // proto: static void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject * target, const QRect & targetRect, QOpenGLFramebufferObject * source, const QRect & sourceRect, GLbitfield buffers, GLenum filter);
  fn _ZN24QOpenGLFramebufferObject15blitFramebufferEPS_RK5QRectS0_S3_jj(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: c_uint, arg5: c_uint);
  // proto:  QImage QOpenGLFramebufferObject::toImage();
  fn _ZNK24QOpenGLFramebufferObject7toImageEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QOpenGLFramebufferObject::size();
  fn _ZNK24QOpenGLFramebufferObject4sizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QOpenGLFramebufferObject::QOpenGLFramebufferObject(const QOpenGLFramebufferObject & );
  fn dector_ZN24QOpenGLFramebufferObjectC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN24QOpenGLFramebufferObjectC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QOpenGLFramebufferObject::~QOpenGLFramebufferObject();
  fn _ZN24QOpenGLFramebufferObjectD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QOpenGLFramebufferObjectFormat QOpenGLFramebufferObject::format();
  fn _ZNK24QOpenGLFramebufferObject6formatEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QOpenGLFramebufferObject::bind();
  fn _ZN24QOpenGLFramebufferObject4bindEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QOpenGLFramebufferObject::isBound();
  fn _ZNK24QOpenGLFramebufferObject7isBoundEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QOpenGLFramebufferObject::width();
  fn _ZNK24QOpenGLFramebufferObject5widthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QOpenGLFramebufferObject::QOpenGLFramebufferObject(int width, int height, GLenum target);
  fn dector_ZN24QOpenGLFramebufferObjectC1Eiij(arg0: c_int, arg1: c_int, arg2: c_uint) -> *mut c_void;
  fn _ZN24QOpenGLFramebufferObjectC1Eiij(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_uint);
} // <= ext block end

// body block begin =>
// class sizeof(QOpenGLFramebufferObjectFormat)=8
#[derive(Default)]
pub struct QOpenGLFramebufferObjectFormat {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLFramebufferObject)=1
#[derive(Default)]
pub struct QOpenGLFramebufferObject {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFramebufferObjectFormat {
    return QOpenGLFramebufferObjectFormat{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QOpenGLFramebufferObjectFormat::~QOpenGLFramebufferObjectFormat();
impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn Free<RetType, T: QOpenGLFramebufferObjectFormat_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_Free<RetType> {
  fn Free(self , rsthis: & QOpenGLFramebufferObjectFormat) -> RetType;
}

  // proto:  void QOpenGLFramebufferObjectFormat::~QOpenGLFramebufferObjectFormat();
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_Free<()> for () {
  fn Free(self , rsthis: & QOpenGLFramebufferObjectFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QOpenGLFramebufferObjectFormatD0Ev()};
     unsafe {_ZN30QOpenGLFramebufferObjectFormatD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLFramebufferObjectFormat::setTextureTarget(GLenum target);
impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn setTextureTarget<RetType, T: QOpenGLFramebufferObjectFormat_setTextureTarget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextureTarget(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_setTextureTarget<RetType> {
  fn setTextureTarget(self , rsthis: & QOpenGLFramebufferObjectFormat) -> RetType;
}

  // proto:  void QOpenGLFramebufferObjectFormat::setTextureTarget(GLenum target);
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_setTextureTarget<()> for (u32) {
  fn setTextureTarget(self , rsthis: & QOpenGLFramebufferObjectFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QOpenGLFramebufferObjectFormat16setTextureTargetEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN30QOpenGLFramebufferObjectFormat16setTextureTargetEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFramebufferObjectFormat::setInternalTextureFormat(GLenum internalTextureFormat);
impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn setInternalTextureFormat<RetType, T: QOpenGLFramebufferObjectFormat_setInternalTextureFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setInternalTextureFormat(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_setInternalTextureFormat<RetType> {
  fn setInternalTextureFormat(self , rsthis: & QOpenGLFramebufferObjectFormat) -> RetType;
}

  // proto:  void QOpenGLFramebufferObjectFormat::setInternalTextureFormat(GLenum internalTextureFormat);
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_setInternalTextureFormat<()> for (u32) {
  fn setInternalTextureFormat(self , rsthis: & QOpenGLFramebufferObjectFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QOpenGLFramebufferObjectFormat24setInternalTextureFormatEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN30QOpenGLFramebufferObjectFormat24setInternalTextureFormatEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFramebufferObjectFormat::QOpenGLFramebufferObjectFormat(const QOpenGLFramebufferObjectFormat & other);
impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn New<T: QOpenGLFramebufferObjectFormat_New>(value: T) -> QOpenGLFramebufferObjectFormat {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_New {
  fn New(self) -> QOpenGLFramebufferObjectFormat;
}

  // proto:  void QOpenGLFramebufferObjectFormat::QOpenGLFramebufferObjectFormat(const QOpenGLFramebufferObjectFormat & other);
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_New for (&'a QOpenGLFramebufferObjectFormat) {
  fn New(self) -> QOpenGLFramebufferObjectFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QOpenGLFramebufferObjectFormatC1ERKS_()};
    let ctysz: c_int = unsafe{QOpenGLFramebufferObjectFormat_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN30QOpenGLFramebufferObjectFormatC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN30QOpenGLFramebufferObjectFormatC1ERKS_(arg0)} as u64;
    let rsthis = QOpenGLFramebufferObjectFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QOpenGLFramebufferObjectFormat::mipmap();
impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn mipmap<RetType, T: QOpenGLFramebufferObjectFormat_mipmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mipmap(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_mipmap<RetType> {
  fn mipmap(self , rsthis: & QOpenGLFramebufferObjectFormat) -> RetType;
}

  // proto:  bool QOpenGLFramebufferObjectFormat::mipmap();
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_mipmap<i8> for () {
  fn mipmap(self , rsthis: & QOpenGLFramebufferObjectFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK30QOpenGLFramebufferObjectFormat6mipmapEv()};
    let mut ret = unsafe {_ZNK30QOpenGLFramebufferObjectFormat6mipmapEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  GLenum QOpenGLFramebufferObjectFormat::textureTarget();
impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn textureTarget<RetType, T: QOpenGLFramebufferObjectFormat_textureTarget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textureTarget(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_textureTarget<RetType> {
  fn textureTarget(self , rsthis: & QOpenGLFramebufferObjectFormat) -> RetType;
}

  // proto:  GLenum QOpenGLFramebufferObjectFormat::textureTarget();
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_textureTarget<u32> for () {
  fn textureTarget(self , rsthis: & QOpenGLFramebufferObjectFormat) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK30QOpenGLFramebufferObjectFormat13textureTargetEv()};
    let mut ret = unsafe {_ZNK30QOpenGLFramebufferObjectFormat13textureTargetEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  void QOpenGLFramebufferObjectFormat::setSamples(int samples);
impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn setSamples<RetType, T: QOpenGLFramebufferObjectFormat_setSamples<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSamples(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_setSamples<RetType> {
  fn setSamples(self , rsthis: & QOpenGLFramebufferObjectFormat) -> RetType;
}

  // proto:  void QOpenGLFramebufferObjectFormat::setSamples(int samples);
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_setSamples<()> for (i32) {
  fn setSamples(self , rsthis: & QOpenGLFramebufferObjectFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QOpenGLFramebufferObjectFormat10setSamplesEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN30QOpenGLFramebufferObjectFormat10setSamplesEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLFramebufferObjectFormat::QOpenGLFramebufferObjectFormat();
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_New for () {
  fn New(self) -> QOpenGLFramebufferObjectFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QOpenGLFramebufferObjectFormatC1Ev()};
    let ctysz: c_int = unsafe{QOpenGLFramebufferObjectFormat_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN30QOpenGLFramebufferObjectFormatC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN30QOpenGLFramebufferObjectFormatC1Ev()} as u64;
    let rsthis = QOpenGLFramebufferObjectFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLFramebufferObjectFormat::setMipmap(bool enabled);
impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn setMipmap<RetType, T: QOpenGLFramebufferObjectFormat_setMipmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMipmap(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_setMipmap<RetType> {
  fn setMipmap(self , rsthis: & QOpenGLFramebufferObjectFormat) -> RetType;
}

  // proto:  void QOpenGLFramebufferObjectFormat::setMipmap(bool enabled);
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_setMipmap<()> for (i8) {
  fn setMipmap(self , rsthis: & QOpenGLFramebufferObjectFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN30QOpenGLFramebufferObjectFormat9setMipmapEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN30QOpenGLFramebufferObjectFormat9setMipmapEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  GLenum QOpenGLFramebufferObjectFormat::internalTextureFormat();
impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn internalTextureFormat<RetType, T: QOpenGLFramebufferObjectFormat_internalTextureFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.internalTextureFormat(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_internalTextureFormat<RetType> {
  fn internalTextureFormat(self , rsthis: & QOpenGLFramebufferObjectFormat) -> RetType;
}

  // proto:  GLenum QOpenGLFramebufferObjectFormat::internalTextureFormat();
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_internalTextureFormat<u32> for () {
  fn internalTextureFormat(self , rsthis: & QOpenGLFramebufferObjectFormat) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK30QOpenGLFramebufferObjectFormat21internalTextureFormatEv()};
    let mut ret = unsafe {_ZNK30QOpenGLFramebufferObjectFormat21internalTextureFormatEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  int QOpenGLFramebufferObjectFormat::samples();
impl /*struct*/ QOpenGLFramebufferObjectFormat {
  pub fn samples<RetType, T: QOpenGLFramebufferObjectFormat_samples<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.samples(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObjectFormat_samples<RetType> {
  fn samples(self , rsthis: & QOpenGLFramebufferObjectFormat) -> RetType;
}

  // proto:  int QOpenGLFramebufferObjectFormat::samples();
impl<'a> /*trait*/ QOpenGLFramebufferObjectFormat_samples<i32> for () {
  fn samples(self , rsthis: & QOpenGLFramebufferObjectFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK30QOpenGLFramebufferObjectFormat7samplesEv()};
    let mut ret = unsafe {_ZNK30QOpenGLFramebufferObjectFormat7samplesEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFramebufferObject {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLFramebufferObject {
    return QOpenGLFramebufferObject{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QOpenGLFramebufferObject::isValid();
impl /*struct*/ QOpenGLFramebufferObject {
  pub fn isValid<RetType, T: QOpenGLFramebufferObject_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_isValid<RetType> {
  fn isValid(self , rsthis: & QOpenGLFramebufferObject) -> RetType;
}

  // proto:  bool QOpenGLFramebufferObject::isValid();
impl<'a> /*trait*/ QOpenGLFramebufferObject_isValid<i8> for () {
  fn isValid(self , rsthis: & QOpenGLFramebufferObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject7isValidEv()};
    let mut ret = unsafe {_ZNK24QOpenGLFramebufferObject7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  GLuint QOpenGLFramebufferObject::takeTexture();
impl /*struct*/ QOpenGLFramebufferObject {
  pub fn takeTexture<RetType, T: QOpenGLFramebufferObject_takeTexture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeTexture(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_takeTexture<RetType> {
  fn takeTexture(self , rsthis: & QOpenGLFramebufferObject) -> RetType;
}

  // proto:  GLuint QOpenGLFramebufferObject::takeTexture();
impl<'a> /*trait*/ QOpenGLFramebufferObject_takeTexture<u32> for () {
  fn takeTexture(self , rsthis: & QOpenGLFramebufferObject) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject11takeTextureEv()};
    let mut ret = unsafe {_ZN24QOpenGLFramebufferObject11takeTextureEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  void QOpenGLFramebufferObject::QOpenGLFramebufferObject(const QSize & size, const QOpenGLFramebufferObjectFormat & format);
impl /*struct*/ QOpenGLFramebufferObject {
  pub fn New<T: QOpenGLFramebufferObject_New>(value: T) -> QOpenGLFramebufferObject {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_New {
  fn New(self) -> QOpenGLFramebufferObject;
}

  // proto:  void QOpenGLFramebufferObject::QOpenGLFramebufferObject(const QSize & size, const QOpenGLFramebufferObjectFormat & format);
impl<'a> /*trait*/ QOpenGLFramebufferObject_New for (&'a QSize, &'a QOpenGLFramebufferObjectFormat) {
  fn New(self) -> QOpenGLFramebufferObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObjectC1ERK5QSizeRK30QOpenGLFramebufferObjectFormat()};
    let ctysz: c_int = unsafe{QOpenGLFramebufferObject_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN24QOpenGLFramebufferObjectC1ERK5QSizeRK30QOpenGLFramebufferObjectFormat(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN24QOpenGLFramebufferObjectC1ERK5QSizeRK30QOpenGLFramebufferObjectFormat(arg0, arg1)} as u64;
    let rsthis = QOpenGLFramebufferObject{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static bool QOpenGLFramebufferObject::bindDefault();
impl /*struct*/ QOpenGLFramebufferObject {
  pub fn bindDefault_s<RetType, T: QOpenGLFramebufferObject_bindDefault_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.bindDefault_s();
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_bindDefault_s<RetType> {
  fn bindDefault_s(self ) -> RetType;
}

  // proto: static bool QOpenGLFramebufferObject::bindDefault();
impl<'a> /*trait*/ QOpenGLFramebufferObject_bindDefault_s<i8> for () {
  fn bindDefault_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject11bindDefaultEv()};
    let mut ret = unsafe {_ZN24QOpenGLFramebufferObject11bindDefaultEv()};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QOpenGLFramebufferObject::hasOpenGLFramebufferBlit();
impl /*struct*/ QOpenGLFramebufferObject {
  pub fn hasOpenGLFramebufferBlit_s<RetType, T: QOpenGLFramebufferObject_hasOpenGLFramebufferBlit_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.hasOpenGLFramebufferBlit_s();
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_hasOpenGLFramebufferBlit_s<RetType> {
  fn hasOpenGLFramebufferBlit_s(self ) -> RetType;
}

  // proto: static bool QOpenGLFramebufferObject::hasOpenGLFramebufferBlit();
impl<'a> /*trait*/ QOpenGLFramebufferObject_hasOpenGLFramebufferBlit_s<i8> for () {
  fn hasOpenGLFramebufferBlit_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject24hasOpenGLFramebufferBlitEv()};
    let mut ret = unsafe {_ZN24QOpenGLFramebufferObject24hasOpenGLFramebufferBlitEv()};
    return ret as i8;
    // return 1;
  }
}

  // proto:  GLuint QOpenGLFramebufferObject::texture();
impl /*struct*/ QOpenGLFramebufferObject {
  pub fn texture<RetType, T: QOpenGLFramebufferObject_texture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.texture(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_texture<RetType> {
  fn texture(self , rsthis: & QOpenGLFramebufferObject) -> RetType;
}

  // proto:  GLuint QOpenGLFramebufferObject::texture();
impl<'a> /*trait*/ QOpenGLFramebufferObject_texture<u32> for () {
  fn texture(self , rsthis: & QOpenGLFramebufferObject) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject7textureEv()};
    let mut ret = unsafe {_ZNK24QOpenGLFramebufferObject7textureEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  void QOpenGLFramebufferObject::QOpenGLFramebufferObject(const QSize & size, GLenum target);
impl<'a> /*trait*/ QOpenGLFramebufferObject_New for (&'a QSize, u32) {
  fn New(self) -> QOpenGLFramebufferObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObjectC1ERK5QSizej()};
    let ctysz: c_int = unsafe{QOpenGLFramebufferObject_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_uint;
    // unsafe {_ZN24QOpenGLFramebufferObjectC1ERK5QSizej(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN24QOpenGLFramebufferObjectC1ERK5QSizej(arg0, arg1)} as u64;
    let rsthis = QOpenGLFramebufferObject{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QOpenGLFramebufferObject::release();
impl /*struct*/ QOpenGLFramebufferObject {
  pub fn release<RetType, T: QOpenGLFramebufferObject_release<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.release(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_release<RetType> {
  fn release(self , rsthis: & QOpenGLFramebufferObject) -> RetType;
}

  // proto:  bool QOpenGLFramebufferObject::release();
impl<'a> /*trait*/ QOpenGLFramebufferObject_release<i8> for () {
  fn release(self , rsthis: & QOpenGLFramebufferObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject7releaseEv()};
    let mut ret = unsafe {_ZN24QOpenGLFramebufferObject7releaseEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QOpenGLFramebufferObject::hasOpenGLFramebufferObjects();
impl /*struct*/ QOpenGLFramebufferObject {
  pub fn hasOpenGLFramebufferObjects_s<RetType, T: QOpenGLFramebufferObject_hasOpenGLFramebufferObjects_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.hasOpenGLFramebufferObjects_s();
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_hasOpenGLFramebufferObjects_s<RetType> {
  fn hasOpenGLFramebufferObjects_s(self ) -> RetType;
}

  // proto: static bool QOpenGLFramebufferObject::hasOpenGLFramebufferObjects();
impl<'a> /*trait*/ QOpenGLFramebufferObject_hasOpenGLFramebufferObjects_s<i8> for () {
  fn hasOpenGLFramebufferObjects_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject27hasOpenGLFramebufferObjectsEv()};
    let mut ret = unsafe {_ZN24QOpenGLFramebufferObject27hasOpenGLFramebufferObjectsEv()};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QImage QOpenGLFramebufferObject::toImage(bool flipped);
impl /*struct*/ QOpenGLFramebufferObject {
  pub fn toImage<RetType, T: QOpenGLFramebufferObject_toImage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toImage(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_toImage<RetType> {
  fn toImage(self , rsthis: & QOpenGLFramebufferObject) -> RetType;
}

  // proto:  QImage QOpenGLFramebufferObject::toImage(bool flipped);
impl<'a> /*trait*/ QOpenGLFramebufferObject_toImage<QImage> for (i8) {
  fn toImage(self , rsthis: & QOpenGLFramebufferObject) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject7toImageEb()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZNK24QOpenGLFramebufferObject7toImageEb(rsthis.qclsinst, arg0)};
    let mut ret1 = QImage::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  GLuint QOpenGLFramebufferObject::handle();
impl /*struct*/ QOpenGLFramebufferObject {
  pub fn handle<RetType, T: QOpenGLFramebufferObject_handle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.handle(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_handle<RetType> {
  fn handle(self , rsthis: & QOpenGLFramebufferObject) -> RetType;
}

  // proto:  GLuint QOpenGLFramebufferObject::handle();
impl<'a> /*trait*/ QOpenGLFramebufferObject_handle<u32> for () {
  fn handle(self , rsthis: & QOpenGLFramebufferObject) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject6handleEv()};
    let mut ret = unsafe {_ZNK24QOpenGLFramebufferObject6handleEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  int QOpenGLFramebufferObject::height();
impl /*struct*/ QOpenGLFramebufferObject {
  pub fn height<RetType, T: QOpenGLFramebufferObject_height<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_height<RetType> {
  fn height(self , rsthis: & QOpenGLFramebufferObject) -> RetType;
}

  // proto:  int QOpenGLFramebufferObject::height();
impl<'a> /*trait*/ QOpenGLFramebufferObject_height<i32> for () {
  fn height(self , rsthis: & QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject6heightEv()};
    let mut ret = unsafe {_ZNK24QOpenGLFramebufferObject6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject * target, QOpenGLFramebufferObject * source, GLbitfield buffers, GLenum filter);
impl /*struct*/ QOpenGLFramebufferObject {
  pub fn blitFramebuffer_s<RetType, T: QOpenGLFramebufferObject_blitFramebuffer_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.blitFramebuffer_s();
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_blitFramebuffer_s<RetType> {
  fn blitFramebuffer_s(self ) -> RetType;
}

  // proto: static void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject * target, QOpenGLFramebufferObject * source, GLbitfield buffers, GLenum filter);
impl<'a> /*trait*/ QOpenGLFramebufferObject_blitFramebuffer_s<()> for (&'a QOpenGLFramebufferObject, &'a QOpenGLFramebufferObject, u32, u32) {
  fn blitFramebuffer_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject15blitFramebufferEPS_S0_jj()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_uint;
    let arg3 = self.3  as c_uint;
     unsafe {_ZN24QOpenGLFramebufferObject15blitFramebufferEPS_S0_jj(arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QOpenGLFramebufferObject::QOpenGLFramebufferObject(int width, int height, const QOpenGLFramebufferObjectFormat & format);
impl<'a> /*trait*/ QOpenGLFramebufferObject_New for (i32, i32, &'a QOpenGLFramebufferObjectFormat) {
  fn New(self) -> QOpenGLFramebufferObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObjectC1EiiRK30QOpenGLFramebufferObjectFormat()};
    let ctysz: c_int = unsafe{QOpenGLFramebufferObject_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    // unsafe {_ZN24QOpenGLFramebufferObjectC1EiiRK30QOpenGLFramebufferObjectFormat(qthis, arg0, arg1, arg2)};
    let qthis: u64 = unsafe {dector_ZN24QOpenGLFramebufferObjectC1EiiRK30QOpenGLFramebufferObjectFormat(arg0, arg1, arg2)} as u64;
    let rsthis = QOpenGLFramebufferObject{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject * target, const QRect & targetRect, QOpenGLFramebufferObject * source, const QRect & sourceRect, GLbitfield buffers, GLenum filter);
impl<'a> /*trait*/ QOpenGLFramebufferObject_blitFramebuffer_s<()> for (&'a QOpenGLFramebufferObject, &'a QRect, &'a QOpenGLFramebufferObject, &'a QRect, u32, u32) {
  fn blitFramebuffer_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject15blitFramebufferEPS_RK5QRectS0_S3_jj()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4  as c_uint;
    let arg5 = self.5  as c_uint;
     unsafe {_ZN24QOpenGLFramebufferObject15blitFramebufferEPS_RK5QRectS0_S3_jj(arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  QImage QOpenGLFramebufferObject::toImage();
impl<'a> /*trait*/ QOpenGLFramebufferObject_toImage<QImage> for () {
  fn toImage(self , rsthis: & QOpenGLFramebufferObject) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject7toImageEv()};
    let mut ret = unsafe {_ZNK24QOpenGLFramebufferObject7toImageEv(rsthis.qclsinst)};
    let mut ret1 = QImage::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QOpenGLFramebufferObject::size();
impl /*struct*/ QOpenGLFramebufferObject {
  pub fn size<RetType, T: QOpenGLFramebufferObject_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_size<RetType> {
  fn size(self , rsthis: & QOpenGLFramebufferObject) -> RetType;
}

  // proto:  QSize QOpenGLFramebufferObject::size();
impl<'a> /*trait*/ QOpenGLFramebufferObject_size<QSize> for () {
  fn size(self , rsthis: & QOpenGLFramebufferObject) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject4sizeEv()};
    let mut ret = unsafe {_ZNK24QOpenGLFramebufferObject4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QOpenGLFramebufferObject::QOpenGLFramebufferObject(const QOpenGLFramebufferObject & );
impl<'a> /*trait*/ QOpenGLFramebufferObject_New for (&'a QOpenGLFramebufferObject) {
  fn New(self) -> QOpenGLFramebufferObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObjectC1ERKS_()};
    let ctysz: c_int = unsafe{QOpenGLFramebufferObject_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN24QOpenGLFramebufferObjectC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN24QOpenGLFramebufferObjectC1ERKS_(arg0)} as u64;
    let rsthis = QOpenGLFramebufferObject{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLFramebufferObject::~QOpenGLFramebufferObject();
impl /*struct*/ QOpenGLFramebufferObject {
  pub fn Free<RetType, T: QOpenGLFramebufferObject_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_Free<RetType> {
  fn Free(self , rsthis: & QOpenGLFramebufferObject) -> RetType;
}

  // proto:  void QOpenGLFramebufferObject::~QOpenGLFramebufferObject();
impl<'a> /*trait*/ QOpenGLFramebufferObject_Free<()> for () {
  fn Free(self , rsthis: & QOpenGLFramebufferObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObjectD0Ev()};
     unsafe {_ZN24QOpenGLFramebufferObjectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QOpenGLFramebufferObjectFormat QOpenGLFramebufferObject::format();
impl /*struct*/ QOpenGLFramebufferObject {
  pub fn format<RetType, T: QOpenGLFramebufferObject_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_format<RetType> {
  fn format(self , rsthis: & QOpenGLFramebufferObject) -> RetType;
}

  // proto:  QOpenGLFramebufferObjectFormat QOpenGLFramebufferObject::format();
impl<'a> /*trait*/ QOpenGLFramebufferObject_format<()> for () {
  fn format(self , rsthis: & QOpenGLFramebufferObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject6formatEv()};
     unsafe {_ZNK24QOpenGLFramebufferObject6formatEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLFramebufferObject::bind();
impl /*struct*/ QOpenGLFramebufferObject {
  pub fn bind<RetType, T: QOpenGLFramebufferObject_bind<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bind(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_bind<RetType> {
  fn bind(self , rsthis: & QOpenGLFramebufferObject) -> RetType;
}

  // proto:  bool QOpenGLFramebufferObject::bind();
impl<'a> /*trait*/ QOpenGLFramebufferObject_bind<i8> for () {
  fn bind(self , rsthis: & QOpenGLFramebufferObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObject4bindEv()};
    let mut ret = unsafe {_ZN24QOpenGLFramebufferObject4bindEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QOpenGLFramebufferObject::isBound();
impl /*struct*/ QOpenGLFramebufferObject {
  pub fn isBound<RetType, T: QOpenGLFramebufferObject_isBound<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isBound(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_isBound<RetType> {
  fn isBound(self , rsthis: & QOpenGLFramebufferObject) -> RetType;
}

  // proto:  bool QOpenGLFramebufferObject::isBound();
impl<'a> /*trait*/ QOpenGLFramebufferObject_isBound<i8> for () {
  fn isBound(self , rsthis: & QOpenGLFramebufferObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject7isBoundEv()};
    let mut ret = unsafe {_ZNK24QOpenGLFramebufferObject7isBoundEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QOpenGLFramebufferObject::width();
impl /*struct*/ QOpenGLFramebufferObject {
  pub fn width<RetType, T: QOpenGLFramebufferObject_width<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QOpenGLFramebufferObject_width<RetType> {
  fn width(self , rsthis: & QOpenGLFramebufferObject) -> RetType;
}

  // proto:  int QOpenGLFramebufferObject::width();
impl<'a> /*trait*/ QOpenGLFramebufferObject_width<i32> for () {
  fn width(self , rsthis: & QOpenGLFramebufferObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLFramebufferObject5widthEv()};
    let mut ret = unsafe {_ZNK24QOpenGLFramebufferObject5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QOpenGLFramebufferObject::QOpenGLFramebufferObject(int width, int height, GLenum target);
impl<'a> /*trait*/ QOpenGLFramebufferObject_New for (i32, i32, u32) {
  fn New(self) -> QOpenGLFramebufferObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLFramebufferObjectC1Eiij()};
    let ctysz: c_int = unsafe{QOpenGLFramebufferObject_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uint;
    // unsafe {_ZN24QOpenGLFramebufferObjectC1Eiij(qthis, arg0, arg1, arg2)};
    let qthis: u64 = unsafe {dector_ZN24QOpenGLFramebufferObjectC1Eiij(arg0, arg1, arg2)} as u64;
    let rsthis = QOpenGLFramebufferObject{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

