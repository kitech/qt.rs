// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtGui/qimagewriter.h
// dst-file: /src/gui/qimagewriter.rs
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
use super::super::core::qstring::*; // 771
use super::super::core::qiodevice::*; // 771
use super::super::core::qbytearray::*; // 771
// use super::qlist::*; // 775
use super::qimage::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QImageWriter_Class_Size() -> c_int;
  // proto:  void QImageWriter::setText(const QString & key, const QString & text);
  fn C_ZN12QImageWriter7setTextERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QImageWriter::setGamma(float gamma);
  fn C_ZN12QImageWriter8setGammaEf(qthis: u64 /* *mut c_void*/, arg0: c_float);
  // proto:  void QImageWriter::setFileName(const QString & fileName);
  fn C_ZN12QImageWriter11setFileNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QImageWriter::optimizedWrite();
  fn C_ZNK12QImageWriter14optimizedWriteEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QImageWriter::~QImageWriter();
  fn C_ZN12QImageWriterD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QIODevice * QImageWriter::device();
  fn C_ZNK12QImageWriter6deviceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QByteArray QImageWriter::subType();
  fn C_ZNK12QImageWriter7subTypeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QList<QByteArray> QImageWriter::supportedMimeTypes();
  fn C_ZN12QImageWriter18supportedMimeTypesEv() -> *mut c_void;
  // proto:  int QImageWriter::quality();
  fn C_ZNK12QImageWriter7qualityEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QImageWriter::write(const QImage & image);
  fn C_ZN12QImageWriter5writeERK6QImage(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QImageWriter::setCompression(int compression);
  fn C_ZN12QImageWriter14setCompressionEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto: static QList<QByteArray> QImageWriter::supportedImageFormats();
  fn C_ZN12QImageWriter21supportedImageFormatsEv() -> *mut c_void;
  // proto:  QString QImageWriter::fileName();
  fn C_ZNK12QImageWriter8fileNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QImageWriter::setOptimizedWrite(bool optimize);
  fn C_ZN12QImageWriter17setOptimizedWriteEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QString QImageWriter::errorString();
  fn C_ZNK12QImageWriter11errorStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QImageWriter::setQuality(int quality);
  fn C_ZN12QImageWriter10setQualityEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  float QImageWriter::gamma();
  fn C_ZNK12QImageWriter5gammaEv(qthis: u64 /* *mut c_void*/) -> c_float;
  // proto:  QString QImageWriter::description();
  fn C_ZNK12QImageWriter11descriptionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QImageWriter::QImageWriter();
  fn C_ZN12QImageWriterC2Ev() -> u64;
  // proto:  void QImageWriter::setFormat(const QByteArray & format);
  fn C_ZN12QImageWriter9setFormatERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QImageWriter::QImageWriter(const QString & fileName, const QByteArray & format);
  fn C_ZN12QImageWriterC2ERK7QStringRK10QByteArray(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  void QImageWriter::setDevice(QIODevice * device);
  fn C_ZN12QImageWriter9setDeviceEP9QIODevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QImageWriter::setSubType(const QByteArray & type);
  fn C_ZN12QImageWriter10setSubTypeERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QImageWriter::progressiveScanWrite();
  fn C_ZNK12QImageWriter20progressiveScanWriteEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QByteArray QImageWriter::format();
  fn C_ZNK12QImageWriter6formatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<QByteArray> QImageWriter::supportedSubTypes();
  fn C_ZNK12QImageWriter17supportedSubTypesEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QImageWriter::canWrite();
  fn C_ZNK12QImageWriter8canWriteEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QImageWriter::QImageWriter(QIODevice * device, const QByteArray & format);
  fn C_ZN12QImageWriterC2EP9QIODeviceRK10QByteArray(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  int QImageWriter::compression();
  fn C_ZNK12QImageWriter11compressionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QImageWriter::setProgressiveScanWrite(bool progressive);
  fn C_ZN12QImageWriter23setProgressiveScanWriteEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QImageWriter::setDescription(const QString & description);
  fn C_ZN12QImageWriter14setDescriptionERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QImageWriter)=8
#[derive(Default)]
pub struct QImageWriter {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QImageWriter {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QImageWriter {
    return QImageWriter{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QImageWriter::setText(const QString & key, const QString & text);
impl /*struct*/ QImageWriter {
  pub fn setText<RetType, T: QImageWriter_setText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QImageWriter_setText<RetType> {
  fn setText(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  void QImageWriter::setText(const QString & key, const QString & text);
impl<'a> /*trait*/ QImageWriter_setText<()> for (&'a QString, &'a QString) {
  fn setText(self , rsthis: & QImageWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter7setTextERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN12QImageWriter7setTextERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QImageWriter::setGamma(float gamma);
impl /*struct*/ QImageWriter {
  pub fn setGamma<RetType, T: QImageWriter_setGamma<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGamma(self);
    // return 1;
  }
}

pub trait QImageWriter_setGamma<RetType> {
  fn setGamma(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  void QImageWriter::setGamma(float gamma);
impl<'a> /*trait*/ QImageWriter_setGamma<()> for (f32) {
  fn setGamma(self , rsthis: & QImageWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter8setGammaEf()};
    let arg0 = self  as c_float;
     unsafe {C_ZN12QImageWriter8setGammaEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QImageWriter::setFileName(const QString & fileName);
impl /*struct*/ QImageWriter {
  pub fn setFileName<RetType, T: QImageWriter_setFileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFileName(self);
    // return 1;
  }
}

pub trait QImageWriter_setFileName<RetType> {
  fn setFileName(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  void QImageWriter::setFileName(const QString & fileName);
impl<'a> /*trait*/ QImageWriter_setFileName<()> for (&'a QString) {
  fn setFileName(self , rsthis: & QImageWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QImageWriter11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QImageWriter::optimizedWrite();
impl /*struct*/ QImageWriter {
  pub fn optimizedWrite<RetType, T: QImageWriter_optimizedWrite<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.optimizedWrite(self);
    // return 1;
  }
}

pub trait QImageWriter_optimizedWrite<RetType> {
  fn optimizedWrite(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  bool QImageWriter::optimizedWrite();
impl<'a> /*trait*/ QImageWriter_optimizedWrite<i8> for () {
  fn optimizedWrite(self , rsthis: & QImageWriter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter14optimizedWriteEv()};
    let mut ret = unsafe {C_ZNK12QImageWriter14optimizedWriteEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QImageWriter::~QImageWriter();
impl /*struct*/ QImageWriter {
  pub fn free<RetType, T: QImageWriter_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QImageWriter_free<RetType> {
  fn free(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  void QImageWriter::~QImageWriter();
impl<'a> /*trait*/ QImageWriter_free<()> for () {
  fn free(self , rsthis: & QImageWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriterD2Ev()};
     unsafe {C_ZN12QImageWriterD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QIODevice * QImageWriter::device();
impl /*struct*/ QImageWriter {
  pub fn device<RetType, T: QImageWriter_device<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.device(self);
    // return 1;
  }
}

pub trait QImageWriter_device<RetType> {
  fn device(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  QIODevice * QImageWriter::device();
impl<'a> /*trait*/ QImageWriter_device<QIODevice> for () {
  fn device(self , rsthis: & QImageWriter) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter6deviceEv()};
    let mut ret = unsafe {C_ZNK12QImageWriter6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QImageWriter::subType();
impl /*struct*/ QImageWriter {
  pub fn subType<RetType, T: QImageWriter_subType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.subType(self);
    // return 1;
  }
}

pub trait QImageWriter_subType<RetType> {
  fn subType(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  QByteArray QImageWriter::subType();
impl<'a> /*trait*/ QImageWriter_subType<QByteArray> for () {
  fn subType(self , rsthis: & QImageWriter) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter7subTypeEv()};
    let mut ret = unsafe {C_ZNK12QImageWriter7subTypeEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QList<QByteArray> QImageWriter::supportedMimeTypes();
impl /*struct*/ QImageWriter {
  pub fn supportedMimeTypes_s<RetType, T: QImageWriter_supportedMimeTypes_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.supportedMimeTypes_s();
    // return 1;
  }
}

pub trait QImageWriter_supportedMimeTypes_s<RetType> {
  fn supportedMimeTypes_s(self ) -> RetType;
}

  // proto: static QList<QByteArray> QImageWriter::supportedMimeTypes();
impl<'a> /*trait*/ QImageWriter_supportedMimeTypes_s<u64> for () {
  fn supportedMimeTypes_s(self ) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter18supportedMimeTypesEv()};
    let mut ret = unsafe {C_ZN12QImageWriter18supportedMimeTypesEv()};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  int QImageWriter::quality();
impl /*struct*/ QImageWriter {
  pub fn quality<RetType, T: QImageWriter_quality<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.quality(self);
    // return 1;
  }
}

pub trait QImageWriter_quality<RetType> {
  fn quality(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  int QImageWriter::quality();
impl<'a> /*trait*/ QImageWriter_quality<i32> for () {
  fn quality(self , rsthis: & QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter7qualityEv()};
    let mut ret = unsafe {C_ZNK12QImageWriter7qualityEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QImageWriter::write(const QImage & image);
impl /*struct*/ QImageWriter {
  pub fn write<RetType, T: QImageWriter_write<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.write(self);
    // return 1;
  }
}

pub trait QImageWriter_write<RetType> {
  fn write(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  bool QImageWriter::write(const QImage & image);
impl<'a> /*trait*/ QImageWriter_write<i8> for (&'a QImage) {
  fn write(self , rsthis: & QImageWriter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter5writeERK6QImage()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN12QImageWriter5writeERK6QImage(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QImageWriter::setCompression(int compression);
impl /*struct*/ QImageWriter {
  pub fn setCompression<RetType, T: QImageWriter_setCompression<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCompression(self);
    // return 1;
  }
}

pub trait QImageWriter_setCompression<RetType> {
  fn setCompression(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  void QImageWriter::setCompression(int compression);
impl<'a> /*trait*/ QImageWriter_setCompression<()> for (i32) {
  fn setCompression(self , rsthis: & QImageWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter14setCompressionEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QImageWriter14setCompressionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QList<QByteArray> QImageWriter::supportedImageFormats();
impl /*struct*/ QImageWriter {
  pub fn supportedImageFormats_s<RetType, T: QImageWriter_supportedImageFormats_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.supportedImageFormats_s();
    // return 1;
  }
}

pub trait QImageWriter_supportedImageFormats_s<RetType> {
  fn supportedImageFormats_s(self ) -> RetType;
}

  // proto: static QList<QByteArray> QImageWriter::supportedImageFormats();
impl<'a> /*trait*/ QImageWriter_supportedImageFormats_s<u64> for () {
  fn supportedImageFormats_s(self ) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter21supportedImageFormatsEv()};
    let mut ret = unsafe {C_ZN12QImageWriter21supportedImageFormatsEv()};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  QString QImageWriter::fileName();
impl /*struct*/ QImageWriter {
  pub fn fileName<RetType, T: QImageWriter_fileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QImageWriter_fileName<RetType> {
  fn fileName(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  QString QImageWriter::fileName();
impl<'a> /*trait*/ QImageWriter_fileName<QString> for () {
  fn fileName(self , rsthis: & QImageWriter) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter8fileNameEv()};
    let mut ret = unsafe {C_ZNK12QImageWriter8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QImageWriter::setOptimizedWrite(bool optimize);
impl /*struct*/ QImageWriter {
  pub fn setOptimizedWrite<RetType, T: QImageWriter_setOptimizedWrite<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOptimizedWrite(self);
    // return 1;
  }
}

pub trait QImageWriter_setOptimizedWrite<RetType> {
  fn setOptimizedWrite(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  void QImageWriter::setOptimizedWrite(bool optimize);
impl<'a> /*trait*/ QImageWriter_setOptimizedWrite<()> for (i8) {
  fn setOptimizedWrite(self , rsthis: & QImageWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter17setOptimizedWriteEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN12QImageWriter17setOptimizedWriteEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QImageWriter::errorString();
impl /*struct*/ QImageWriter {
  pub fn errorString<RetType, T: QImageWriter_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QImageWriter_errorString<RetType> {
  fn errorString(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  QString QImageWriter::errorString();
impl<'a> /*trait*/ QImageWriter_errorString<QString> for () {
  fn errorString(self , rsthis: & QImageWriter) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter11errorStringEv()};
    let mut ret = unsafe {C_ZNK12QImageWriter11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QImageWriter::setQuality(int quality);
impl /*struct*/ QImageWriter {
  pub fn setQuality<RetType, T: QImageWriter_setQuality<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setQuality(self);
    // return 1;
  }
}

pub trait QImageWriter_setQuality<RetType> {
  fn setQuality(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  void QImageWriter::setQuality(int quality);
impl<'a> /*trait*/ QImageWriter_setQuality<()> for (i32) {
  fn setQuality(self , rsthis: & QImageWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter10setQualityEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QImageWriter10setQualityEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  float QImageWriter::gamma();
impl /*struct*/ QImageWriter {
  pub fn gamma<RetType, T: QImageWriter_gamma<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.gamma(self);
    // return 1;
  }
}

pub trait QImageWriter_gamma<RetType> {
  fn gamma(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  float QImageWriter::gamma();
impl<'a> /*trait*/ QImageWriter_gamma<f32> for () {
  fn gamma(self , rsthis: & QImageWriter) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter5gammaEv()};
    let mut ret = unsafe {C_ZNK12QImageWriter5gammaEv(rsthis.qclsinst)};
    return ret as f32; // 1
    // return 1;
  }
}

  // proto:  QString QImageWriter::description();
impl /*struct*/ QImageWriter {
  pub fn description<RetType, T: QImageWriter_description<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.description(self);
    // return 1;
  }
}

pub trait QImageWriter_description<RetType> {
  fn description(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  QString QImageWriter::description();
impl<'a> /*trait*/ QImageWriter_description<QString> for () {
  fn description(self , rsthis: & QImageWriter) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter11descriptionEv()};
    let mut ret = unsafe {C_ZNK12QImageWriter11descriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QImageWriter::QImageWriter();
impl /*struct*/ QImageWriter {
  pub fn new<T: QImageWriter_new>(value: T) -> QImageWriter {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QImageWriter_new {
  fn new(self) -> QImageWriter;
}

  // proto:  void QImageWriter::QImageWriter();
impl<'a> /*trait*/ QImageWriter_new for () {
  fn new(self) -> QImageWriter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriterC2Ev()};
    let ctysz: c_int = unsafe{QImageWriter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN12QImageWriterC2Ev()};
    let rsthis = QImageWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QImageWriter::setFormat(const QByteArray & format);
impl /*struct*/ QImageWriter {
  pub fn setFormat<RetType, T: QImageWriter_setFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QImageWriter_setFormat<RetType> {
  fn setFormat(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  void QImageWriter::setFormat(const QByteArray & format);
impl<'a> /*trait*/ QImageWriter_setFormat<()> for (&'a QByteArray) {
  fn setFormat(self , rsthis: & QImageWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter9setFormatERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QImageWriter9setFormatERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QImageWriter::QImageWriter(const QString & fileName, const QByteArray & format);
impl<'a> /*trait*/ QImageWriter_new for (&'a QString, &'a QByteArray) {
  fn new(self) -> QImageWriter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriterC2ERK7QStringRK10QByteArray()};
    let ctysz: c_int = unsafe{QImageWriter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QImageWriterC2ERK7QStringRK10QByteArray(arg0, arg1)};
    let rsthis = QImageWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QImageWriter::setDevice(QIODevice * device);
impl /*struct*/ QImageWriter {
  pub fn setDevice<RetType, T: QImageWriter_setDevice<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDevice(self);
    // return 1;
  }
}

pub trait QImageWriter_setDevice<RetType> {
  fn setDevice(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  void QImageWriter::setDevice(QIODevice * device);
impl<'a> /*trait*/ QImageWriter_setDevice<()> for (&'a QIODevice) {
  fn setDevice(self , rsthis: & QImageWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QImageWriter9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QImageWriter::setSubType(const QByteArray & type);
impl /*struct*/ QImageWriter {
  pub fn setSubType<RetType, T: QImageWriter_setSubType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSubType(self);
    // return 1;
  }
}

pub trait QImageWriter_setSubType<RetType> {
  fn setSubType(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  void QImageWriter::setSubType(const QByteArray & type);
impl<'a> /*trait*/ QImageWriter_setSubType<()> for (&'a QByteArray) {
  fn setSubType(self , rsthis: & QImageWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter10setSubTypeERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QImageWriter10setSubTypeERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QImageWriter::progressiveScanWrite();
impl /*struct*/ QImageWriter {
  pub fn progressiveScanWrite<RetType, T: QImageWriter_progressiveScanWrite<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.progressiveScanWrite(self);
    // return 1;
  }
}

pub trait QImageWriter_progressiveScanWrite<RetType> {
  fn progressiveScanWrite(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  bool QImageWriter::progressiveScanWrite();
impl<'a> /*trait*/ QImageWriter_progressiveScanWrite<i8> for () {
  fn progressiveScanWrite(self , rsthis: & QImageWriter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter20progressiveScanWriteEv()};
    let mut ret = unsafe {C_ZNK12QImageWriter20progressiveScanWriteEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QByteArray QImageWriter::format();
impl /*struct*/ QImageWriter {
  pub fn format<RetType, T: QImageWriter_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QImageWriter_format<RetType> {
  fn format(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  QByteArray QImageWriter::format();
impl<'a> /*trait*/ QImageWriter_format<QByteArray> for () {
  fn format(self , rsthis: & QImageWriter) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter6formatEv()};
    let mut ret = unsafe {C_ZNK12QImageWriter6formatEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QByteArray> QImageWriter::supportedSubTypes();
impl /*struct*/ QImageWriter {
  pub fn supportedSubTypes<RetType, T: QImageWriter_supportedSubTypes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.supportedSubTypes(self);
    // return 1;
  }
}

pub trait QImageWriter_supportedSubTypes<RetType> {
  fn supportedSubTypes(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  QList<QByteArray> QImageWriter::supportedSubTypes();
impl<'a> /*trait*/ QImageWriter_supportedSubTypes<u64> for () {
  fn supportedSubTypes(self , rsthis: & QImageWriter) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter17supportedSubTypesEv()};
    let mut ret = unsafe {C_ZNK12QImageWriter17supportedSubTypesEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  bool QImageWriter::canWrite();
impl /*struct*/ QImageWriter {
  pub fn canWrite<RetType, T: QImageWriter_canWrite<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canWrite(self);
    // return 1;
  }
}

pub trait QImageWriter_canWrite<RetType> {
  fn canWrite(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  bool QImageWriter::canWrite();
impl<'a> /*trait*/ QImageWriter_canWrite<i8> for () {
  fn canWrite(self , rsthis: & QImageWriter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter8canWriteEv()};
    let mut ret = unsafe {C_ZNK12QImageWriter8canWriteEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QImageWriter::QImageWriter(QIODevice * device, const QByteArray & format);
impl<'a> /*trait*/ QImageWriter_new for (&'a QIODevice, &'a QByteArray) {
  fn new(self) -> QImageWriter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriterC2EP9QIODeviceRK10QByteArray()};
    let ctysz: c_int = unsafe{QImageWriter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QImageWriterC2EP9QIODeviceRK10QByteArray(arg0, arg1)};
    let rsthis = QImageWriter{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QImageWriter::compression();
impl /*struct*/ QImageWriter {
  pub fn compression<RetType, T: QImageWriter_compression<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.compression(self);
    // return 1;
  }
}

pub trait QImageWriter_compression<RetType> {
  fn compression(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  int QImageWriter::compression();
impl<'a> /*trait*/ QImageWriter_compression<i32> for () {
  fn compression(self , rsthis: & QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter11compressionEv()};
    let mut ret = unsafe {C_ZNK12QImageWriter11compressionEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QImageWriter::setProgressiveScanWrite(bool progressive);
impl /*struct*/ QImageWriter {
  pub fn setProgressiveScanWrite<RetType, T: QImageWriter_setProgressiveScanWrite<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setProgressiveScanWrite(self);
    // return 1;
  }
}

pub trait QImageWriter_setProgressiveScanWrite<RetType> {
  fn setProgressiveScanWrite(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  void QImageWriter::setProgressiveScanWrite(bool progressive);
impl<'a> /*trait*/ QImageWriter_setProgressiveScanWrite<()> for (i8) {
  fn setProgressiveScanWrite(self , rsthis: & QImageWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter23setProgressiveScanWriteEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN12QImageWriter23setProgressiveScanWriteEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QImageWriter::setDescription(const QString & description);
impl /*struct*/ QImageWriter {
  pub fn setDescription<RetType, T: QImageWriter_setDescription<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDescription(self);
    // return 1;
  }
}

pub trait QImageWriter_setDescription<RetType> {
  fn setDescription(self , rsthis: & QImageWriter) -> RetType;
}

  // proto:  void QImageWriter::setDescription(const QString & description);
impl<'a> /*trait*/ QImageWriter_setDescription<()> for (&'a QString) {
  fn setDescription(self , rsthis: & QImageWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter14setDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QImageWriter14setDescriptionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

