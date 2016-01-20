// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qquickimageprovider.h
// dst-file: /src/quick/qquickimageprovider.rs
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
use super::super::qml::qqmlengine::QQmlImageProviderBase; // 771
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::super::core::qsize::QSize; // 771
use super::super::gui::qimage::QImage; // 771
// use super::qquickimageprovider::QQuickTextureFactory; // 773
use super::super::gui::qpixmap::QPixmap; // 771
use super::super::core::qobject::QObject; // 771
use super::qquickwindow::QQuickWindow; // 773
use super::qsgtexture::QSGTexture; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQuickImageProvider_Class_Size() -> c_int;
  // proto:  QImage QQuickImageProvider::requestImage(const QString & id, QSize * size, const QSize & requestedSize);
  fn _ZN19QQuickImageProvider12requestImageERK7QStringP5QSizeRKS3_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  QQuickTextureFactory * QQuickImageProvider::requestTexture(const QString & id, QSize * size, const QSize & requestedSize);
  fn _ZN19QQuickImageProvider14requestTextureERK7QStringP5QSizeRKS3_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QQuickImageProvider::~QQuickImageProvider();
  fn _ZN19QQuickImageProviderD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QPixmap QQuickImageProvider::requestPixmap(const QString & id, QSize * size, const QSize & requestedSize);
  fn _ZN19QQuickImageProvider13requestPixmapERK7QStringP5QSizeRKS3_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  fn QQuickTextureFactory_Class_Size() -> c_int;
  // proto:  void QQuickTextureFactory::~QQuickTextureFactory();
  fn _ZN20QQuickTextureFactoryD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QImage QQuickTextureFactory::image();
  fn _ZNK20QQuickTextureFactory5imageEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSGTexture * QQuickTextureFactory::createTexture(QQuickWindow * window);
  fn _ZNK20QQuickTextureFactory13createTextureEP12QQuickWindow(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QQuickTextureFactory::textureByteCount();
  fn _ZNK20QQuickTextureFactory16textureByteCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QSize QQuickTextureFactory::textureSize();
  fn _ZNK20QQuickTextureFactory11textureSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQuickTextureFactory::QQuickTextureFactory();
  fn _ZN20QQuickTextureFactoryC2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QQuickImageProvider)=16
#[derive(Default)]
pub struct QQuickImageProvider {
  qbase: QQmlImageProviderBase,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QQuickTextureFactory)=1
#[derive(Default)]
pub struct QQuickTextureFactory {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QQuickImageProvider {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQuickImageProvider {
    return QQuickImageProvider{qbase: QQmlImageProviderBase::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQuickImageProvider {
  type Target = QQmlImageProviderBase;

  fn deref(&self) -> &QQmlImageProviderBase {
    return & self.qbase;
  }
}
impl AsRef<QQmlImageProviderBase> for QQuickImageProvider {
  fn as_ref(& self) -> & QQmlImageProviderBase {
    return & self.qbase;
  }
}
  // proto:  QImage QQuickImageProvider::requestImage(const QString & id, QSize * size, const QSize & requestedSize);
impl /*struct*/ QQuickImageProvider {
  pub fn requestImage<RetType, T: QQuickImageProvider_requestImage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.requestImage(self);
    // return 1;
  }
}

pub trait QQuickImageProvider_requestImage<RetType> {
  fn requestImage(self , rsthis: & QQuickImageProvider) -> RetType;
}

  // proto:  QImage QQuickImageProvider::requestImage(const QString & id, QSize * size, const QSize & requestedSize);
impl<'a> /*trait*/ QQuickImageProvider_requestImage<QImage> for (&'a QString, &'a QSize, &'a QSize) {
  fn requestImage(self , rsthis: & QQuickImageProvider) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQuickImageProvider12requestImageERK7QStringP5QSizeRKS3_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN19QQuickImageProvider12requestImageERK7QStringP5QSizeRKS3_(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QImage::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QQuickTextureFactory * QQuickImageProvider::requestTexture(const QString & id, QSize * size, const QSize & requestedSize);
impl /*struct*/ QQuickImageProvider {
  pub fn requestTexture<RetType, T: QQuickImageProvider_requestTexture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.requestTexture(self);
    // return 1;
  }
}

pub trait QQuickImageProvider_requestTexture<RetType> {
  fn requestTexture(self , rsthis: & QQuickImageProvider) -> RetType;
}

  // proto:  QQuickTextureFactory * QQuickImageProvider::requestTexture(const QString & id, QSize * size, const QSize & requestedSize);
impl<'a> /*trait*/ QQuickImageProvider_requestTexture<QQuickTextureFactory> for (&'a QString, &'a QSize, &'a QSize) {
  fn requestTexture(self , rsthis: & QQuickImageProvider) -> QQuickTextureFactory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQuickImageProvider14requestTextureERK7QStringP5QSizeRKS3_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN19QQuickImageProvider14requestTextureERK7QStringP5QSizeRKS3_(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QQuickTextureFactory::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickImageProvider::~QQuickImageProvider();
impl /*struct*/ QQuickImageProvider {
  pub fn free<RetType, T: QQuickImageProvider_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQuickImageProvider_free<RetType> {
  fn free(self , rsthis: & QQuickImageProvider) -> RetType;
}

  // proto:  void QQuickImageProvider::~QQuickImageProvider();
impl<'a> /*trait*/ QQuickImageProvider_free<()> for () {
  fn free(self , rsthis: & QQuickImageProvider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQuickImageProviderD2Ev()};
     unsafe {_ZN19QQuickImageProviderD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPixmap QQuickImageProvider::requestPixmap(const QString & id, QSize * size, const QSize & requestedSize);
impl /*struct*/ QQuickImageProvider {
  pub fn requestPixmap<RetType, T: QQuickImageProvider_requestPixmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.requestPixmap(self);
    // return 1;
  }
}

pub trait QQuickImageProvider_requestPixmap<RetType> {
  fn requestPixmap(self , rsthis: & QQuickImageProvider) -> RetType;
}

  // proto:  QPixmap QQuickImageProvider::requestPixmap(const QString & id, QSize * size, const QSize & requestedSize);
impl<'a> /*trait*/ QQuickImageProvider_requestPixmap<QPixmap> for (&'a QString, &'a QSize, &'a QSize) {
  fn requestPixmap(self , rsthis: & QQuickImageProvider) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQuickImageProvider13requestPixmapERK7QStringP5QSizeRKS3_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN19QQuickImageProvider13requestPixmapERK7QStringP5QSizeRKS3_(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QPixmap::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QQuickTextureFactory {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQuickTextureFactory {
    return QQuickTextureFactory{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQuickTextureFactory {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QQuickTextureFactory {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QQuickTextureFactory::~QQuickTextureFactory();
impl /*struct*/ QQuickTextureFactory {
  pub fn free<RetType, T: QQuickTextureFactory_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQuickTextureFactory_free<RetType> {
  fn free(self , rsthis: & QQuickTextureFactory) -> RetType;
}

  // proto:  void QQuickTextureFactory::~QQuickTextureFactory();
impl<'a> /*trait*/ QQuickTextureFactory_free<()> for () {
  fn free(self , rsthis: & QQuickTextureFactory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QQuickTextureFactoryD2Ev()};
     unsafe {_ZN20QQuickTextureFactoryD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QImage QQuickTextureFactory::image();
impl /*struct*/ QQuickTextureFactory {
  pub fn image<RetType, T: QQuickTextureFactory_image<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.image(self);
    // return 1;
  }
}

pub trait QQuickTextureFactory_image<RetType> {
  fn image(self , rsthis: & QQuickTextureFactory) -> RetType;
}

  // proto:  QImage QQuickTextureFactory::image();
impl<'a> /*trait*/ QQuickTextureFactory_image<QImage> for () {
  fn image(self , rsthis: & QQuickTextureFactory) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QQuickTextureFactory5imageEv()};
    let mut ret = unsafe {_ZNK20QQuickTextureFactory5imageEv(rsthis.qclsinst)};
    let mut ret1 = QImage::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSGTexture * QQuickTextureFactory::createTexture(QQuickWindow * window);
impl /*struct*/ QQuickTextureFactory {
  pub fn createTexture<RetType, T: QQuickTextureFactory_createTexture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createTexture(self);
    // return 1;
  }
}

pub trait QQuickTextureFactory_createTexture<RetType> {
  fn createTexture(self , rsthis: & QQuickTextureFactory) -> RetType;
}

  // proto:  QSGTexture * QQuickTextureFactory::createTexture(QQuickWindow * window);
impl<'a> /*trait*/ QQuickTextureFactory_createTexture<QSGTexture> for (&'a QQuickWindow) {
  fn createTexture(self , rsthis: & QQuickTextureFactory) -> QSGTexture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QQuickTextureFactory13createTextureEP12QQuickWindow()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QQuickTextureFactory13createTextureEP12QQuickWindow(rsthis.qclsinst, arg0)};
    let mut ret1 = QSGTexture::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QQuickTextureFactory::textureByteCount();
impl /*struct*/ QQuickTextureFactory {
  pub fn textureByteCount<RetType, T: QQuickTextureFactory_textureByteCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textureByteCount(self);
    // return 1;
  }
}

pub trait QQuickTextureFactory_textureByteCount<RetType> {
  fn textureByteCount(self , rsthis: & QQuickTextureFactory) -> RetType;
}

  // proto:  int QQuickTextureFactory::textureByteCount();
impl<'a> /*trait*/ QQuickTextureFactory_textureByteCount<i32> for () {
  fn textureByteCount(self , rsthis: & QQuickTextureFactory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QQuickTextureFactory16textureByteCountEv()};
    let mut ret = unsafe {_ZNK20QQuickTextureFactory16textureByteCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSize QQuickTextureFactory::textureSize();
impl /*struct*/ QQuickTextureFactory {
  pub fn textureSize<RetType, T: QQuickTextureFactory_textureSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textureSize(self);
    // return 1;
  }
}

pub trait QQuickTextureFactory_textureSize<RetType> {
  fn textureSize(self , rsthis: & QQuickTextureFactory) -> RetType;
}

  // proto:  QSize QQuickTextureFactory::textureSize();
impl<'a> /*trait*/ QQuickTextureFactory_textureSize<QSize> for () {
  fn textureSize(self , rsthis: & QQuickTextureFactory) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QQuickTextureFactory11textureSizeEv()};
    let mut ret = unsafe {_ZNK20QQuickTextureFactory11textureSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickTextureFactory::QQuickTextureFactory();
impl /*struct*/ QQuickTextureFactory {
  pub fn new<T: QQuickTextureFactory_new>(value: T) -> QQuickTextureFactory {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQuickTextureFactory_new {
  fn new(self) -> QQuickTextureFactory;
}

  // proto:  void QQuickTextureFactory::QQuickTextureFactory();
impl<'a> /*trait*/ QQuickTextureFactory_new for () {
  fn new(self) -> QQuickTextureFactory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QQuickTextureFactoryC2Ev()};
    let ctysz: c_int = unsafe{QQuickTextureFactory_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN20QQuickTextureFactoryC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQuickTextureFactory{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

