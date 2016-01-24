// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtGui/qimagereader.h
// dst-file: /src/gui/qimagereader.rs
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
use super::super::core::qsize::QSize; // 771
use super::qimage::QImage; // 773
use super::super::core::qrect::QRect; // 771
use super::super::core::qiodevice::QIODevice; // 771
use super::super::core::qbytearray::QByteArray; // 771
use super::qcolor::QColor; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QImageReader_Class_Size() -> c_int;
  // proto:  QString QImageReader::errorString();
  fn C_ZNK12QImageReader11errorStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QImageReader::canRead();
  fn C_ZNK12QImageReader7canReadEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QImageReader::~QImageReader();
  fn C_ZN12QImageReaderD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QImageReader::setScaledSize(const QSize & size);
  fn C_ZN12QImageReader13setScaledSizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QImageReader::read(QImage * image);
  fn C_ZN12QImageReader4readEP6QImage(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QImageReader::setScaledClipRect(const QRect & rect);
  fn C_ZN12QImageReader17setScaledClipRectERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QImageReader::imageCount();
  fn C_ZNK12QImageReader10imageCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QStringList QImageReader::textKeys();
  fn C_ZNK12QImageReader8textKeysEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QImageReader::decideFormatFromContent();
  fn C_ZNK12QImageReader23decideFormatFromContentEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QIODevice * QImageReader::device();
  fn C_ZNK12QImageReader6deviceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QImageReader::autoTransform();
  fn C_ZNK12QImageReader13autoTransformEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QImageReader::jumpToNextImage();
  fn C_ZN12QImageReader15jumpToNextImageEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static QByteArray QImageReader::imageFormat(const QString & fileName);
  fn C_ZN12QImageReader11imageFormatERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  QList<QByteArray> QImageReader::supportedSubTypes();
  fn C_ZNK12QImageReader17supportedSubTypesEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QImageReader::size();
  fn C_ZNK12QImageReader4sizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QColor QImageReader::backgroundColor();
  fn C_ZNK12QImageReader15backgroundColorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QByteArray QImageReader::subType();
  fn C_ZNK12QImageReader7subTypeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QImageReader::currentImageNumber();
  fn C_ZNK12QImageReader18currentImageNumberEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto: static QList<QByteArray> QImageReader::supportedImageFormats();
  fn C_ZN12QImageReader21supportedImageFormatsEv();
  // proto:  int QImageReader::loopCount();
  fn C_ZNK12QImageReader9loopCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QImageReader::setDecideFormatFromContent(bool ignored);
  fn C_ZN12QImageReader26setDecideFormatFromContentEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QRect QImageReader::scaledClipRect();
  fn C_ZNK12QImageReader14scaledClipRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QList<QByteArray> QImageReader::supportedMimeTypes();
  fn C_ZN12QImageReader18supportedMimeTypesEv();
  // proto:  QString QImageReader::text(const QString & key);
  fn C_ZNK12QImageReader4textERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QImageReader::nextImageDelay();
  fn C_ZNK12QImageReader14nextImageDelayEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QImage QImageReader::read();
  fn C_ZN12QImageReader4readEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QImageReader::supportsAnimation();
  fn C_ZNK12QImageReader17supportsAnimationEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QImageReader::jumpToImage(int imageNumber);
  fn C_ZN12QImageReader11jumpToImageEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QImageReader::setFileName(const QString & fileName);
  fn C_ZN12QImageReader11setFileNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSize QImageReader::scaledSize();
  fn C_ZNK12QImageReader10scaledSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QImageReader::setAutoTransform(bool enabled);
  fn C_ZN12QImageReader16setAutoTransformEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QImageReader::setClipRect(const QRect & rect);
  fn C_ZN12QImageReader11setClipRectERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QImageReader::autoDetectImageFormat();
  fn C_ZNK12QImageReader21autoDetectImageFormatEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QRect QImageReader::currentImageRect();
  fn C_ZNK12QImageReader16currentImageRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QImageReader::QImageReader(const QString & fileName, const QByteArray & format);
  fn C_ZN12QImageReaderC2ERK7QStringRK10QByteArray(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto: static QByteArray QImageReader::imageFormat(QIODevice * device);
  fn C_ZN12QImageReader11imageFormatEP9QIODevice(arg0: *mut c_void) -> *mut c_void;
  // proto:  int QImageReader::quality();
  fn C_ZNK12QImageReader7qualityEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QImageReader::setDevice(QIODevice * device);
  fn C_ZN12QImageReader9setDeviceEP9QIODevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QImageReader::setBackgroundColor(const QColor & color);
  fn C_ZN12QImageReader18setBackgroundColorERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QImageReader::setQuality(int quality);
  fn C_ZN12QImageReader10setQualityEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QImageReader::QImageReader(QIODevice * device, const QByteArray & format);
  fn C_ZN12QImageReaderC2EP9QIODeviceRK10QByteArray(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  void QImageReader::setAutoDetectImageFormat(bool enabled);
  fn C_ZN12QImageReader24setAutoDetectImageFormatEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QImageReader::QImageReader();
  fn C_ZN12QImageReaderC2Ev() -> u64;
  // proto:  void QImageReader::setFormat(const QByteArray & format);
  fn C_ZN12QImageReader9setFormatERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QImageReader::fileName();
  fn C_ZNK12QImageReader8fileNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRect QImageReader::clipRect();
  fn C_ZNK12QImageReader8clipRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QByteArray QImageReader::format();
  fn C_ZNK12QImageReader6formatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QImageReader)=8
#[derive(Default)]
pub struct QImageReader {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QImageReader {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QImageReader {
    return QImageReader{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QString QImageReader::errorString();
impl /*struct*/ QImageReader {
  pub fn errorString<RetType, T: QImageReader_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QImageReader_errorString<RetType> {
  fn errorString(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  QString QImageReader::errorString();
impl<'a> /*trait*/ QImageReader_errorString<QString> for () {
  fn errorString(self , rsthis: & QImageReader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader11errorStringEv()};
    let mut ret = unsafe {C_ZNK12QImageReader11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QImageReader::canRead();
impl /*struct*/ QImageReader {
  pub fn canRead<RetType, T: QImageReader_canRead<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canRead(self);
    // return 1;
  }
}

pub trait QImageReader_canRead<RetType> {
  fn canRead(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  bool QImageReader::canRead();
impl<'a> /*trait*/ QImageReader_canRead<i8> for () {
  fn canRead(self , rsthis: & QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader7canReadEv()};
    let mut ret = unsafe {C_ZNK12QImageReader7canReadEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QImageReader::~QImageReader();
impl /*struct*/ QImageReader {
  pub fn free<RetType, T: QImageReader_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QImageReader_free<RetType> {
  fn free(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  void QImageReader::~QImageReader();
impl<'a> /*trait*/ QImageReader_free<()> for () {
  fn free(self , rsthis: & QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReaderD2Ev()};
     unsafe {C_ZN12QImageReaderD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QImageReader::setScaledSize(const QSize & size);
impl /*struct*/ QImageReader {
  pub fn setScaledSize<RetType, T: QImageReader_setScaledSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScaledSize(self);
    // return 1;
  }
}

pub trait QImageReader_setScaledSize<RetType> {
  fn setScaledSize(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  void QImageReader::setScaledSize(const QSize & size);
impl<'a> /*trait*/ QImageReader_setScaledSize<()> for (&'a QSize) {
  fn setScaledSize(self , rsthis: & QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader13setScaledSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QImageReader13setScaledSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QImageReader::read(QImage * image);
impl /*struct*/ QImageReader {
  pub fn read<RetType, T: QImageReader_read<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.read(self);
    // return 1;
  }
}

pub trait QImageReader_read<RetType> {
  fn read(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  bool QImageReader::read(QImage * image);
impl<'a> /*trait*/ QImageReader_read<i8> for (&'a QImage) {
  fn read(self , rsthis: & QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader4readEP6QImage()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN12QImageReader4readEP6QImage(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QImageReader::setScaledClipRect(const QRect & rect);
impl /*struct*/ QImageReader {
  pub fn setScaledClipRect<RetType, T: QImageReader_setScaledClipRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScaledClipRect(self);
    // return 1;
  }
}

pub trait QImageReader_setScaledClipRect<RetType> {
  fn setScaledClipRect(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  void QImageReader::setScaledClipRect(const QRect & rect);
impl<'a> /*trait*/ QImageReader_setScaledClipRect<()> for (&'a QRect) {
  fn setScaledClipRect(self , rsthis: & QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader17setScaledClipRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QImageReader17setScaledClipRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QImageReader::imageCount();
impl /*struct*/ QImageReader {
  pub fn imageCount<RetType, T: QImageReader_imageCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.imageCount(self);
    // return 1;
  }
}

pub trait QImageReader_imageCount<RetType> {
  fn imageCount(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  int QImageReader::imageCount();
impl<'a> /*trait*/ QImageReader_imageCount<i32> for () {
  fn imageCount(self , rsthis: & QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader10imageCountEv()};
    let mut ret = unsafe {C_ZNK12QImageReader10imageCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QStringList QImageReader::textKeys();
impl /*struct*/ QImageReader {
  pub fn textKeys<RetType, T: QImageReader_textKeys<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textKeys(self);
    // return 1;
  }
}

pub trait QImageReader_textKeys<RetType> {
  fn textKeys(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  QStringList QImageReader::textKeys();
impl<'a> /*trait*/ QImageReader_textKeys<()> for () {
  fn textKeys(self , rsthis: & QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader8textKeysEv()};
     unsafe {C_ZNK12QImageReader8textKeysEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QImageReader::decideFormatFromContent();
impl /*struct*/ QImageReader {
  pub fn decideFormatFromContent<RetType, T: QImageReader_decideFormatFromContent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.decideFormatFromContent(self);
    // return 1;
  }
}

pub trait QImageReader_decideFormatFromContent<RetType> {
  fn decideFormatFromContent(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  bool QImageReader::decideFormatFromContent();
impl<'a> /*trait*/ QImageReader_decideFormatFromContent<i8> for () {
  fn decideFormatFromContent(self , rsthis: & QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader23decideFormatFromContentEv()};
    let mut ret = unsafe {C_ZNK12QImageReader23decideFormatFromContentEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QIODevice * QImageReader::device();
impl /*struct*/ QImageReader {
  pub fn device<RetType, T: QImageReader_device<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.device(self);
    // return 1;
  }
}

pub trait QImageReader_device<RetType> {
  fn device(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  QIODevice * QImageReader::device();
impl<'a> /*trait*/ QImageReader_device<QIODevice> for () {
  fn device(self , rsthis: & QImageReader) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader6deviceEv()};
    let mut ret = unsafe {C_ZNK12QImageReader6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QImageReader::autoTransform();
impl /*struct*/ QImageReader {
  pub fn autoTransform<RetType, T: QImageReader_autoTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoTransform(self);
    // return 1;
  }
}

pub trait QImageReader_autoTransform<RetType> {
  fn autoTransform(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  bool QImageReader::autoTransform();
impl<'a> /*trait*/ QImageReader_autoTransform<i8> for () {
  fn autoTransform(self , rsthis: & QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader13autoTransformEv()};
    let mut ret = unsafe {C_ZNK12QImageReader13autoTransformEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QImageReader::jumpToNextImage();
impl /*struct*/ QImageReader {
  pub fn jumpToNextImage<RetType, T: QImageReader_jumpToNextImage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.jumpToNextImage(self);
    // return 1;
  }
}

pub trait QImageReader_jumpToNextImage<RetType> {
  fn jumpToNextImage(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  bool QImageReader::jumpToNextImage();
impl<'a> /*trait*/ QImageReader_jumpToNextImage<i8> for () {
  fn jumpToNextImage(self , rsthis: & QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader15jumpToNextImageEv()};
    let mut ret = unsafe {C_ZN12QImageReader15jumpToNextImageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QByteArray QImageReader::imageFormat(const QString & fileName);
impl /*struct*/ QImageReader {
  pub fn imageFormat_s<RetType, T: QImageReader_imageFormat_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.imageFormat_s();
    // return 1;
  }
}

pub trait QImageReader_imageFormat_s<RetType> {
  fn imageFormat_s(self ) -> RetType;
}

  // proto: static QByteArray QImageReader::imageFormat(const QString & fileName);
impl<'a> /*trait*/ QImageReader_imageFormat_s<QByteArray> for (&'a QString) {
  fn imageFormat_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11imageFormatERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN12QImageReader11imageFormatERK7QString(arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QByteArray> QImageReader::supportedSubTypes();
impl /*struct*/ QImageReader {
  pub fn supportedSubTypes<RetType, T: QImageReader_supportedSubTypes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.supportedSubTypes(self);
    // return 1;
  }
}

pub trait QImageReader_supportedSubTypes<RetType> {
  fn supportedSubTypes(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  QList<QByteArray> QImageReader::supportedSubTypes();
impl<'a> /*trait*/ QImageReader_supportedSubTypes<()> for () {
  fn supportedSubTypes(self , rsthis: & QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader17supportedSubTypesEv()};
     unsafe {C_ZNK12QImageReader17supportedSubTypesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QImageReader::size();
impl /*struct*/ QImageReader {
  pub fn size<RetType, T: QImageReader_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QImageReader_size<RetType> {
  fn size(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  QSize QImageReader::size();
impl<'a> /*trait*/ QImageReader_size<QSize> for () {
  fn size(self , rsthis: & QImageReader) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader4sizeEv()};
    let mut ret = unsafe {C_ZNK12QImageReader4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QColor QImageReader::backgroundColor();
impl /*struct*/ QImageReader {
  pub fn backgroundColor<RetType, T: QImageReader_backgroundColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.backgroundColor(self);
    // return 1;
  }
}

pub trait QImageReader_backgroundColor<RetType> {
  fn backgroundColor(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  QColor QImageReader::backgroundColor();
impl<'a> /*trait*/ QImageReader_backgroundColor<QColor> for () {
  fn backgroundColor(self , rsthis: & QImageReader) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader15backgroundColorEv()};
    let mut ret = unsafe {C_ZNK12QImageReader15backgroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QImageReader::subType();
impl /*struct*/ QImageReader {
  pub fn subType<RetType, T: QImageReader_subType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.subType(self);
    // return 1;
  }
}

pub trait QImageReader_subType<RetType> {
  fn subType(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  QByteArray QImageReader::subType();
impl<'a> /*trait*/ QImageReader_subType<QByteArray> for () {
  fn subType(self , rsthis: & QImageReader) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader7subTypeEv()};
    let mut ret = unsafe {C_ZNK12QImageReader7subTypeEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QImageReader::currentImageNumber();
impl /*struct*/ QImageReader {
  pub fn currentImageNumber<RetType, T: QImageReader_currentImageNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentImageNumber(self);
    // return 1;
  }
}

pub trait QImageReader_currentImageNumber<RetType> {
  fn currentImageNumber(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  int QImageReader::currentImageNumber();
impl<'a> /*trait*/ QImageReader_currentImageNumber<i32> for () {
  fn currentImageNumber(self , rsthis: & QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader18currentImageNumberEv()};
    let mut ret = unsafe {C_ZNK12QImageReader18currentImageNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static QList<QByteArray> QImageReader::supportedImageFormats();
impl /*struct*/ QImageReader {
  pub fn supportedImageFormats_s<RetType, T: QImageReader_supportedImageFormats_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.supportedImageFormats_s();
    // return 1;
  }
}

pub trait QImageReader_supportedImageFormats_s<RetType> {
  fn supportedImageFormats_s(self ) -> RetType;
}

  // proto: static QList<QByteArray> QImageReader::supportedImageFormats();
impl<'a> /*trait*/ QImageReader_supportedImageFormats_s<()> for () {
  fn supportedImageFormats_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader21supportedImageFormatsEv()};
     unsafe {C_ZN12QImageReader21supportedImageFormatsEv()};
    // return 1;
  }
}

  // proto:  int QImageReader::loopCount();
impl /*struct*/ QImageReader {
  pub fn loopCount<RetType, T: QImageReader_loopCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.loopCount(self);
    // return 1;
  }
}

pub trait QImageReader_loopCount<RetType> {
  fn loopCount(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  int QImageReader::loopCount();
impl<'a> /*trait*/ QImageReader_loopCount<i32> for () {
  fn loopCount(self , rsthis: & QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader9loopCountEv()};
    let mut ret = unsafe {C_ZNK12QImageReader9loopCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QImageReader::setDecideFormatFromContent(bool ignored);
impl /*struct*/ QImageReader {
  pub fn setDecideFormatFromContent<RetType, T: QImageReader_setDecideFormatFromContent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDecideFormatFromContent(self);
    // return 1;
  }
}

pub trait QImageReader_setDecideFormatFromContent<RetType> {
  fn setDecideFormatFromContent(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  void QImageReader::setDecideFormatFromContent(bool ignored);
impl<'a> /*trait*/ QImageReader_setDecideFormatFromContent<()> for (i8) {
  fn setDecideFormatFromContent(self , rsthis: & QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader26setDecideFormatFromContentEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN12QImageReader26setDecideFormatFromContentEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QImageReader::scaledClipRect();
impl /*struct*/ QImageReader {
  pub fn scaledClipRect<RetType, T: QImageReader_scaledClipRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scaledClipRect(self);
    // return 1;
  }
}

pub trait QImageReader_scaledClipRect<RetType> {
  fn scaledClipRect(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  QRect QImageReader::scaledClipRect();
impl<'a> /*trait*/ QImageReader_scaledClipRect<QRect> for () {
  fn scaledClipRect(self , rsthis: & QImageReader) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader14scaledClipRectEv()};
    let mut ret = unsafe {C_ZNK12QImageReader14scaledClipRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QList<QByteArray> QImageReader::supportedMimeTypes();
impl /*struct*/ QImageReader {
  pub fn supportedMimeTypes_s<RetType, T: QImageReader_supportedMimeTypes_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.supportedMimeTypes_s();
    // return 1;
  }
}

pub trait QImageReader_supportedMimeTypes_s<RetType> {
  fn supportedMimeTypes_s(self ) -> RetType;
}

  // proto: static QList<QByteArray> QImageReader::supportedMimeTypes();
impl<'a> /*trait*/ QImageReader_supportedMimeTypes_s<()> for () {
  fn supportedMimeTypes_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader18supportedMimeTypesEv()};
     unsafe {C_ZN12QImageReader18supportedMimeTypesEv()};
    // return 1;
  }
}

  // proto:  QString QImageReader::text(const QString & key);
impl /*struct*/ QImageReader {
  pub fn text<RetType, T: QImageReader_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QImageReader_text<RetType> {
  fn text(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  QString QImageReader::text(const QString & key);
impl<'a> /*trait*/ QImageReader_text<QString> for (&'a QString) {
  fn text(self , rsthis: & QImageReader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader4textERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK12QImageReader4textERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QImageReader::nextImageDelay();
impl /*struct*/ QImageReader {
  pub fn nextImageDelay<RetType, T: QImageReader_nextImageDelay<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nextImageDelay(self);
    // return 1;
  }
}

pub trait QImageReader_nextImageDelay<RetType> {
  fn nextImageDelay(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  int QImageReader::nextImageDelay();
impl<'a> /*trait*/ QImageReader_nextImageDelay<i32> for () {
  fn nextImageDelay(self , rsthis: & QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader14nextImageDelayEv()};
    let mut ret = unsafe {C_ZNK12QImageReader14nextImageDelayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QImage QImageReader::read();
impl<'a> /*trait*/ QImageReader_read<QImage> for () {
  fn read(self , rsthis: & QImageReader) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader4readEv()};
    let mut ret = unsafe {C_ZN12QImageReader4readEv(rsthis.qclsinst)};
    let mut ret1 = QImage::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QImageReader::supportsAnimation();
impl /*struct*/ QImageReader {
  pub fn supportsAnimation<RetType, T: QImageReader_supportsAnimation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.supportsAnimation(self);
    // return 1;
  }
}

pub trait QImageReader_supportsAnimation<RetType> {
  fn supportsAnimation(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  bool QImageReader::supportsAnimation();
impl<'a> /*trait*/ QImageReader_supportsAnimation<i8> for () {
  fn supportsAnimation(self , rsthis: & QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader17supportsAnimationEv()};
    let mut ret = unsafe {C_ZNK12QImageReader17supportsAnimationEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QImageReader::jumpToImage(int imageNumber);
impl /*struct*/ QImageReader {
  pub fn jumpToImage<RetType, T: QImageReader_jumpToImage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.jumpToImage(self);
    // return 1;
  }
}

pub trait QImageReader_jumpToImage<RetType> {
  fn jumpToImage(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  bool QImageReader::jumpToImage(int imageNumber);
impl<'a> /*trait*/ QImageReader_jumpToImage<i8> for (i32) {
  fn jumpToImage(self , rsthis: & QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11jumpToImageEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN12QImageReader11jumpToImageEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QImageReader::setFileName(const QString & fileName);
impl /*struct*/ QImageReader {
  pub fn setFileName<RetType, T: QImageReader_setFileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFileName(self);
    // return 1;
  }
}

pub trait QImageReader_setFileName<RetType> {
  fn setFileName(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  void QImageReader::setFileName(const QString & fileName);
impl<'a> /*trait*/ QImageReader_setFileName<()> for (&'a QString) {
  fn setFileName(self , rsthis: & QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QImageReader11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QImageReader::scaledSize();
impl /*struct*/ QImageReader {
  pub fn scaledSize<RetType, T: QImageReader_scaledSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scaledSize(self);
    // return 1;
  }
}

pub trait QImageReader_scaledSize<RetType> {
  fn scaledSize(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  QSize QImageReader::scaledSize();
impl<'a> /*trait*/ QImageReader_scaledSize<QSize> for () {
  fn scaledSize(self , rsthis: & QImageReader) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader10scaledSizeEv()};
    let mut ret = unsafe {C_ZNK12QImageReader10scaledSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QImageReader::setAutoTransform(bool enabled);
impl /*struct*/ QImageReader {
  pub fn setAutoTransform<RetType, T: QImageReader_setAutoTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoTransform(self);
    // return 1;
  }
}

pub trait QImageReader_setAutoTransform<RetType> {
  fn setAutoTransform(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  void QImageReader::setAutoTransform(bool enabled);
impl<'a> /*trait*/ QImageReader_setAutoTransform<()> for (i8) {
  fn setAutoTransform(self , rsthis: & QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader16setAutoTransformEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN12QImageReader16setAutoTransformEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QImageReader::setClipRect(const QRect & rect);
impl /*struct*/ QImageReader {
  pub fn setClipRect<RetType, T: QImageReader_setClipRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setClipRect(self);
    // return 1;
  }
}

pub trait QImageReader_setClipRect<RetType> {
  fn setClipRect(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  void QImageReader::setClipRect(const QRect & rect);
impl<'a> /*trait*/ QImageReader_setClipRect<()> for (&'a QRect) {
  fn setClipRect(self , rsthis: & QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11setClipRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QImageReader11setClipRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QImageReader::autoDetectImageFormat();
impl /*struct*/ QImageReader {
  pub fn autoDetectImageFormat<RetType, T: QImageReader_autoDetectImageFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoDetectImageFormat(self);
    // return 1;
  }
}

pub trait QImageReader_autoDetectImageFormat<RetType> {
  fn autoDetectImageFormat(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  bool QImageReader::autoDetectImageFormat();
impl<'a> /*trait*/ QImageReader_autoDetectImageFormat<i8> for () {
  fn autoDetectImageFormat(self , rsthis: & QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader21autoDetectImageFormatEv()};
    let mut ret = unsafe {C_ZNK12QImageReader21autoDetectImageFormatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRect QImageReader::currentImageRect();
impl /*struct*/ QImageReader {
  pub fn currentImageRect<RetType, T: QImageReader_currentImageRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentImageRect(self);
    // return 1;
  }
}

pub trait QImageReader_currentImageRect<RetType> {
  fn currentImageRect(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  QRect QImageReader::currentImageRect();
impl<'a> /*trait*/ QImageReader_currentImageRect<QRect> for () {
  fn currentImageRect(self , rsthis: & QImageReader) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader16currentImageRectEv()};
    let mut ret = unsafe {C_ZNK12QImageReader16currentImageRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QImageReader::QImageReader(const QString & fileName, const QByteArray & format);
impl /*struct*/ QImageReader {
  pub fn new<T: QImageReader_new>(value: T) -> QImageReader {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QImageReader_new {
  fn new(self) -> QImageReader;
}

  // proto:  void QImageReader::QImageReader(const QString & fileName, const QByteArray & format);
impl<'a> /*trait*/ QImageReader_new for (&'a QString, &'a QByteArray) {
  fn new(self) -> QImageReader {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReaderC2ERK7QStringRK10QByteArray()};
    let ctysz: c_int = unsafe{QImageReader_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QImageReaderC2ERK7QStringRK10QByteArray(arg0, arg1)};
    let rsthis = QImageReader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static QByteArray QImageReader::imageFormat(QIODevice * device);
impl<'a> /*trait*/ QImageReader_imageFormat_s<QByteArray> for (&'a QIODevice) {
  fn imageFormat_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11imageFormatEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN12QImageReader11imageFormatEP9QIODevice(arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QImageReader::quality();
impl /*struct*/ QImageReader {
  pub fn quality<RetType, T: QImageReader_quality<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.quality(self);
    // return 1;
  }
}

pub trait QImageReader_quality<RetType> {
  fn quality(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  int QImageReader::quality();
impl<'a> /*trait*/ QImageReader_quality<i32> for () {
  fn quality(self , rsthis: & QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader7qualityEv()};
    let mut ret = unsafe {C_ZNK12QImageReader7qualityEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QImageReader::setDevice(QIODevice * device);
impl /*struct*/ QImageReader {
  pub fn setDevice<RetType, T: QImageReader_setDevice<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDevice(self);
    // return 1;
  }
}

pub trait QImageReader_setDevice<RetType> {
  fn setDevice(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  void QImageReader::setDevice(QIODevice * device);
impl<'a> /*trait*/ QImageReader_setDevice<()> for (&'a QIODevice) {
  fn setDevice(self , rsthis: & QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QImageReader9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QImageReader::setBackgroundColor(const QColor & color);
impl /*struct*/ QImageReader {
  pub fn setBackgroundColor<RetType, T: QImageReader_setBackgroundColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBackgroundColor(self);
    // return 1;
  }
}

pub trait QImageReader_setBackgroundColor<RetType> {
  fn setBackgroundColor(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  void QImageReader::setBackgroundColor(const QColor & color);
impl<'a> /*trait*/ QImageReader_setBackgroundColor<()> for (&'a QColor) {
  fn setBackgroundColor(self , rsthis: & QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader18setBackgroundColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QImageReader18setBackgroundColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QImageReader::setQuality(int quality);
impl /*struct*/ QImageReader {
  pub fn setQuality<RetType, T: QImageReader_setQuality<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setQuality(self);
    // return 1;
  }
}

pub trait QImageReader_setQuality<RetType> {
  fn setQuality(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  void QImageReader::setQuality(int quality);
impl<'a> /*trait*/ QImageReader_setQuality<()> for (i32) {
  fn setQuality(self , rsthis: & QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader10setQualityEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QImageReader10setQualityEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QImageReader::QImageReader(QIODevice * device, const QByteArray & format);
impl<'a> /*trait*/ QImageReader_new for (&'a QIODevice, &'a QByteArray) {
  fn new(self) -> QImageReader {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReaderC2EP9QIODeviceRK10QByteArray()};
    let ctysz: c_int = unsafe{QImageReader_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QImageReaderC2EP9QIODeviceRK10QByteArray(arg0, arg1)};
    let rsthis = QImageReader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QImageReader::setAutoDetectImageFormat(bool enabled);
impl /*struct*/ QImageReader {
  pub fn setAutoDetectImageFormat<RetType, T: QImageReader_setAutoDetectImageFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoDetectImageFormat(self);
    // return 1;
  }
}

pub trait QImageReader_setAutoDetectImageFormat<RetType> {
  fn setAutoDetectImageFormat(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  void QImageReader::setAutoDetectImageFormat(bool enabled);
impl<'a> /*trait*/ QImageReader_setAutoDetectImageFormat<()> for (i8) {
  fn setAutoDetectImageFormat(self , rsthis: & QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader24setAutoDetectImageFormatEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN12QImageReader24setAutoDetectImageFormatEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QImageReader::QImageReader();
impl<'a> /*trait*/ QImageReader_new for () {
  fn new(self) -> QImageReader {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReaderC2Ev()};
    let ctysz: c_int = unsafe{QImageReader_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN12QImageReaderC2Ev()};
    let rsthis = QImageReader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QImageReader::setFormat(const QByteArray & format);
impl /*struct*/ QImageReader {
  pub fn setFormat<RetType, T: QImageReader_setFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QImageReader_setFormat<RetType> {
  fn setFormat(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  void QImageReader::setFormat(const QByteArray & format);
impl<'a> /*trait*/ QImageReader_setFormat<()> for (&'a QByteArray) {
  fn setFormat(self , rsthis: & QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader9setFormatERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QImageReader9setFormatERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QImageReader::fileName();
impl /*struct*/ QImageReader {
  pub fn fileName<RetType, T: QImageReader_fileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QImageReader_fileName<RetType> {
  fn fileName(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  QString QImageReader::fileName();
impl<'a> /*trait*/ QImageReader_fileName<QString> for () {
  fn fileName(self , rsthis: & QImageReader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader8fileNameEv()};
    let mut ret = unsafe {C_ZNK12QImageReader8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QImageReader::clipRect();
impl /*struct*/ QImageReader {
  pub fn clipRect<RetType, T: QImageReader_clipRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clipRect(self);
    // return 1;
  }
}

pub trait QImageReader_clipRect<RetType> {
  fn clipRect(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  QRect QImageReader::clipRect();
impl<'a> /*trait*/ QImageReader_clipRect<QRect> for () {
  fn clipRect(self , rsthis: & QImageReader) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader8clipRectEv()};
    let mut ret = unsafe {C_ZNK12QImageReader8clipRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QImageReader::format();
impl /*struct*/ QImageReader {
  pub fn format<RetType, T: QImageReader_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QImageReader_format<RetType> {
  fn format(self , rsthis: & QImageReader) -> RetType;
}

  // proto:  QByteArray QImageReader::format();
impl<'a> /*trait*/ QImageReader_format<QByteArray> for () {
  fn format(self , rsthis: & QImageReader) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader6formatEv()};
    let mut ret = unsafe {C_ZNK12QImageReader6formatEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

