// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
// src-file: /QtGui/qpicture.h
// dst-file: /src/gui/qpicture.rs
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
use super::super::core::qiodevice::QIODevice; // 771
use super::super::core::qbytearray::QByteArray; // 771
// use super::qpicture::QPicture; // 773
use super::qpaintdevice::QPaintDevice; // 773
use super::super::core::qrect::QRect; // 771
use super::qpainter::QPainter; // 773
use super::qpaintengine::QPaintEngine; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QPictureIO::QPictureIO(const QString & fileName, const char * format);
  fn _ZN10QPictureIOC1ERK7QStringPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char);
  // proto:  QString QPictureIO::description();
  fn _ZNK10QPictureIO11descriptionEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QList<QByteArray> QPictureIO::inputFormats();
  fn _ZN10QPictureIO12inputFormatsEv();
  // proto:  void QPictureIO::setGamma(float );
  fn _ZN10QPictureIO8setGammaEf(qthis: *mut c_void, arg0: c_float);
  // proto:  int QPictureIO::status();
  fn _ZNK10QPictureIO6statusEv(qthis: *mut c_void) -> c_int;
  // proto:  int QPictureIO::quality();
  fn _ZNK10QPictureIO7qualityEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QPictureIO::write();
  fn _ZN10QPictureIO5writeEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPictureIO::setFileName(const QString & );
  fn _ZN10QPictureIO11setFileNameERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPictureIO::~QPictureIO();
  fn _ZN10QPictureIOD0Ev(qthis: *mut c_void);
  // proto:  const char * QPictureIO::parameters();
  fn _ZNK10QPictureIO10parametersEv(qthis: *mut c_void) -> *mut c_char;
  // proto: static QByteArray QPictureIO::pictureFormat(QIODevice * );
  fn _ZN10QPictureIO13pictureFormatEP9QIODevice(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPictureIO::QPictureIO(const QPictureIO & );
  fn _ZN10QPictureIOC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QPictureIO::read();
  fn _ZN10QPictureIO4readEv(qthis: *mut c_void) -> c_char;
  // proto:  QString QPictureIO::fileName();
  fn _ZNK10QPictureIO8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPictureIO::QPictureIO(QIODevice * ioDevice, const char * format);
  fn _ZN10QPictureIOC1EP9QIODevicePKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char);
  // proto:  const char * QPictureIO::format();
  fn _ZNK10QPictureIO6formatEv(qthis: *mut c_void) -> *mut c_char;
  // proto:  void QPictureIO::setQuality(int );
  fn _ZN10QPictureIO10setQualityEi(qthis: *mut c_void, arg0: c_int);
  // proto:  const QPicture & QPictureIO::picture();
  fn _ZNK10QPictureIO7pictureEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPictureIO::setFormat(const char * );
  fn _ZN10QPictureIO9setFormatEPKc(qthis: *mut c_void, arg0: *mut c_char);
  // proto:  void QPictureIO::setDescription(const QString & );
  fn _ZN10QPictureIO14setDescriptionERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QByteArray QPictureIO::pictureFormat(const QString & fileName);
  fn _ZN10QPictureIO13pictureFormatERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPictureIO::setIODevice(QIODevice * );
  fn _ZN10QPictureIO11setIODeviceEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPictureIO::setStatus(int );
  fn _ZN10QPictureIO9setStatusEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QIODevice * QPictureIO::ioDevice();
  fn _ZNK10QPictureIO8ioDeviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  float QPictureIO::gamma();
  fn _ZNK10QPictureIO5gammaEv(qthis: *mut c_void) -> c_float;
  // proto: static QList<QByteArray> QPictureIO::outputFormats();
  fn _ZN10QPictureIO13outputFormatsEv();
  // proto:  void QPictureIO::setPicture(const QPicture & );
  fn _ZN10QPictureIO10setPictureERK8QPicture(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPictureIO::setParameters(const char * );
  fn _ZN10QPictureIO13setParametersEPKc(qthis: *mut c_void, arg0: *mut c_char);
  // proto:  void QPictureIO::QPictureIO();
  fn _ZN10QPictureIOC1Ev(qthis: *mut c_void);
  // proto:  const char * QPicture::data();
  fn _ZNK8QPicture4dataEv(qthis: *mut c_void) -> *mut c_char;
  // proto: static QStringList QPicture::inputFormatList();
  fn _ZN8QPicture15inputFormatListEv();
  // proto:  void QPicture::swap(QPicture & other);
  fn _ZN8QPicture4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  uint QPicture::size();
  fn _ZNK8QPicture4sizeEv(qthis: *mut c_void) -> c_uint;
  // proto:  bool QPicture::isNull();
  fn _ZNK8QPicture6isNullEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QPicture::save(QIODevice * dev, const char * format);
  fn _ZN8QPicture4saveEP9QIODevicePKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  void QPicture::detach();
  fn _ZN8QPicture6detachEv(qthis: *mut c_void);
  // proto: static QList<QByteArray> QPicture::inputFormats();
  fn _ZN8QPicture12inputFormatsEv();
  // proto:  void QPicture::QPicture(int formatVersion);
  fn _ZN8QPictureC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QPicture::QPicture(const QPicture & );
  fn _ZN8QPictureC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QPicture::isDetached();
  fn _ZNK8QPicture10isDetachedEv(qthis: *mut c_void) -> c_char;
  // proto: static QStringList QPicture::outputFormatList();
  fn _ZN8QPicture16outputFormatListEv();
  // proto:  void QPicture::setData(const char * data, uint size);
  fn _ZN8QPicture7setDataEPKcj(qthis: *mut c_void, arg0: *mut c_char, arg1: c_uint);
  // proto: static QList<QByteArray> QPicture::outputFormats();
  fn _ZN8QPicture13outputFormatsEv();
  // proto:  int QPicture::devType();
  fn _ZNK8QPicture7devTypeEv(qthis: *mut c_void) -> c_int;
  // proto: static const char * QPicture::pictureFormat(const QString & fileName);
  fn _ZN8QPicture13pictureFormatERK7QString(arg0: *mut c_void) -> *mut c_char;
  // proto:  bool QPicture::save(const QString & fileName, const char * format);
  fn _ZN8QPicture4saveERK7QStringPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  bool QPicture::load(const QString & fileName, const char * format);
  fn _ZN8QPicture4loadERK7QStringPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  void QPicture::~QPicture();
  fn _ZN8QPictureD0Ev(qthis: *mut c_void);
  // proto:  void QPicture::setBoundingRect(const QRect & r);
  fn _ZN8QPicture15setBoundingRectERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QPicture::load(QIODevice * dev, const char * format);
  fn _ZN8QPicture4loadEP9QIODevicePKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  QRect QPicture::boundingRect();
  fn _ZNK8QPicture12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QPicture::play(QPainter * p);
  fn _ZN8QPicture4playEP8QPainter(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QPaintEngine * QPicture::paintEngine();
  fn _ZNK8QPicture11paintEngineEv(qthis: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QPictureIO)=8
pub struct QPictureIO {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QPicture)=1
pub struct QPicture {
  qbase: QPaintDevice,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPictureIO {
  pub fn inheritFrom(qthis: *mut c_void) -> QPictureIO {
    return QPictureIO{qclsinst: qthis};
  }
}
  // proto:  void QPictureIO::QPictureIO(const QString & fileName, const char * format);
impl /*struct*/ QPictureIO {
  pub fn New<T: QPictureIO_New>(value: T) -> QPictureIO {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QPictureIO_New {
  fn New(self) -> QPictureIO;
}

  // proto:  void QPictureIO::QPictureIO(const QString & fileName, const char * format);
impl<'a> /*trait*/ QPictureIO_New for (&'a QString, &'a  String) {
  fn New(self) -> QPictureIO {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIOC1ERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    unsafe {_ZN10QPictureIOC1ERK7QStringPKc(qthis, arg0, arg1)};
    let rsthis = QPictureIO{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QPictureIO::description();
impl /*struct*/ QPictureIO {
  pub fn description<RetType, T: QPictureIO_description<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.description(self);
    // return 1;
  }
}

pub trait QPictureIO_description<RetType> {
  fn description(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  QString QPictureIO::description();
impl<'a> /*trait*/ QPictureIO_description<QString> for () {
  fn description(self , rsthis: & QPictureIO) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO11descriptionEv()};
    let mut ret = unsafe {_ZNK10QPictureIO11descriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QList<QByteArray> QPictureIO::inputFormats();
impl /*struct*/ QPictureIO {
  pub fn inputFormats_s<RetType, T: QPictureIO_inputFormats_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.inputFormats_s();
    // return 1;
  }
}

pub trait QPictureIO_inputFormats_s<RetType> {
  fn inputFormats_s(self ) -> RetType;
}

  // proto: static QList<QByteArray> QPictureIO::inputFormats();
impl<'a> /*trait*/ QPictureIO_inputFormats_s<()> for () {
  fn inputFormats_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO12inputFormatsEv()};
     unsafe {_ZN10QPictureIO12inputFormatsEv()};
    // return 1;
  }
}

  // proto:  void QPictureIO::setGamma(float );
impl /*struct*/ QPictureIO {
  pub fn setGamma<RetType, T: QPictureIO_setGamma<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGamma(self);
    // return 1;
  }
}

pub trait QPictureIO_setGamma<RetType> {
  fn setGamma(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::setGamma(float );
impl<'a> /*trait*/ QPictureIO_setGamma<()> for (f32) {
  fn setGamma(self , rsthis: & QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO8setGammaEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN10QPictureIO8setGammaEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QPictureIO::status();
impl /*struct*/ QPictureIO {
  pub fn status<RetType, T: QPictureIO_status<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.status(self);
    // return 1;
  }
}

pub trait QPictureIO_status<RetType> {
  fn status(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  int QPictureIO::status();
impl<'a> /*trait*/ QPictureIO_status<i32> for () {
  fn status(self , rsthis: & QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO6statusEv()};
    let mut ret = unsafe {_ZNK10QPictureIO6statusEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QPictureIO::quality();
impl /*struct*/ QPictureIO {
  pub fn quality<RetType, T: QPictureIO_quality<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.quality(self);
    // return 1;
  }
}

pub trait QPictureIO_quality<RetType> {
  fn quality(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  int QPictureIO::quality();
impl<'a> /*trait*/ QPictureIO_quality<i32> for () {
  fn quality(self , rsthis: & QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO7qualityEv()};
    let mut ret = unsafe {_ZNK10QPictureIO7qualityEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QPictureIO::write();
impl /*struct*/ QPictureIO {
  pub fn write<RetType, T: QPictureIO_write<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.write(self);
    // return 1;
  }
}

pub trait QPictureIO_write<RetType> {
  fn write(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  bool QPictureIO::write();
impl<'a> /*trait*/ QPictureIO_write<i8> for () {
  fn write(self , rsthis: & QPictureIO) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO5writeEv()};
    let mut ret = unsafe {_ZN10QPictureIO5writeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPictureIO::setFileName(const QString & );
impl /*struct*/ QPictureIO {
  pub fn setFileName<RetType, T: QPictureIO_setFileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFileName(self);
    // return 1;
  }
}

pub trait QPictureIO_setFileName<RetType> {
  fn setFileName(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::setFileName(const QString & );
impl<'a> /*trait*/ QPictureIO_setFileName<()> for (&'a QString) {
  fn setFileName(self , rsthis: & QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QPictureIO11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPictureIO::~QPictureIO();
impl /*struct*/ QPictureIO {
  pub fn Free<RetType, T: QPictureIO_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QPictureIO_Free<RetType> {
  fn Free(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::~QPictureIO();
impl<'a> /*trait*/ QPictureIO_Free<()> for () {
  fn Free(self , rsthis: & QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIOD0Ev()};
     unsafe {_ZN10QPictureIOD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const char * QPictureIO::parameters();
impl /*struct*/ QPictureIO {
  pub fn parameters<RetType, T: QPictureIO_parameters<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parameters(self);
    // return 1;
  }
}

pub trait QPictureIO_parameters<RetType> {
  fn parameters(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  const char * QPictureIO::parameters();
impl<'a> /*trait*/ QPictureIO_parameters<String> for () {
  fn parameters(self , rsthis: & QPictureIO) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO10parametersEv()};
    let mut ret = unsafe {_ZNK10QPictureIO10parametersEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto: static QByteArray QPictureIO::pictureFormat(QIODevice * );
impl /*struct*/ QPictureIO {
  pub fn pictureFormat_s<RetType, T: QPictureIO_pictureFormat_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.pictureFormat_s();
    // return 1;
  }
}

pub trait QPictureIO_pictureFormat_s<RetType> {
  fn pictureFormat_s(self ) -> RetType;
}

  // proto: static QByteArray QPictureIO::pictureFormat(QIODevice * );
impl<'a> /*trait*/ QPictureIO_pictureFormat_s<QByteArray> for (&'a QIODevice) {
  fn pictureFormat_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO13pictureFormatEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QPictureIO13pictureFormatEP9QIODevice(arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPictureIO::QPictureIO(const QPictureIO & );
impl<'a> /*trait*/ QPictureIO_New for (&'a QPictureIO) {
  fn New(self) -> QPictureIO {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIOC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QPictureIOC1ERKS_(qthis, arg0)};
    let rsthis = QPictureIO{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QPictureIO::read();
impl /*struct*/ QPictureIO {
  pub fn read<RetType, T: QPictureIO_read<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.read(self);
    // return 1;
  }
}

pub trait QPictureIO_read<RetType> {
  fn read(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  bool QPictureIO::read();
impl<'a> /*trait*/ QPictureIO_read<i8> for () {
  fn read(self , rsthis: & QPictureIO) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO4readEv()};
    let mut ret = unsafe {_ZN10QPictureIO4readEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QPictureIO::fileName();
impl /*struct*/ QPictureIO {
  pub fn fileName<RetType, T: QPictureIO_fileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QPictureIO_fileName<RetType> {
  fn fileName(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  QString QPictureIO::fileName();
impl<'a> /*trait*/ QPictureIO_fileName<QString> for () {
  fn fileName(self , rsthis: & QPictureIO) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO8fileNameEv()};
    let mut ret = unsafe {_ZNK10QPictureIO8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPictureIO::QPictureIO(QIODevice * ioDevice, const char * format);
impl<'a> /*trait*/ QPictureIO_New for (&'a QIODevice, &'a  String) {
  fn New(self) -> QPictureIO {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIOC1EP9QIODevicePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    unsafe {_ZN10QPictureIOC1EP9QIODevicePKc(qthis, arg0, arg1)};
    let rsthis = QPictureIO{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const char * QPictureIO::format();
impl /*struct*/ QPictureIO {
  pub fn format<RetType, T: QPictureIO_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QPictureIO_format<RetType> {
  fn format(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  const char * QPictureIO::format();
impl<'a> /*trait*/ QPictureIO_format<String> for () {
  fn format(self , rsthis: & QPictureIO) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO6formatEv()};
    let mut ret = unsafe {_ZNK10QPictureIO6formatEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  void QPictureIO::setQuality(int );
impl /*struct*/ QPictureIO {
  pub fn setQuality<RetType, T: QPictureIO_setQuality<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setQuality(self);
    // return 1;
  }
}

pub trait QPictureIO_setQuality<RetType> {
  fn setQuality(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::setQuality(int );
impl<'a> /*trait*/ QPictureIO_setQuality<()> for (i32) {
  fn setQuality(self , rsthis: & QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO10setQualityEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QPictureIO10setQualityEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QPicture & QPictureIO::picture();
impl /*struct*/ QPictureIO {
  pub fn picture<RetType, T: QPictureIO_picture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.picture(self);
    // return 1;
  }
}

pub trait QPictureIO_picture<RetType> {
  fn picture(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  const QPicture & QPictureIO::picture();
impl<'a> /*trait*/ QPictureIO_picture<QPicture> for () {
  fn picture(self , rsthis: & QPictureIO) -> QPicture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO7pictureEv()};
    let mut ret = unsafe {_ZNK10QPictureIO7pictureEv(rsthis.qclsinst)};
    let mut ret1 = QPicture::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPictureIO::setFormat(const char * );
impl /*struct*/ QPictureIO {
  pub fn setFormat<RetType, T: QPictureIO_setFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QPictureIO_setFormat<RetType> {
  fn setFormat(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::setFormat(const char * );
impl<'a> /*trait*/ QPictureIO_setFormat<()> for (&'a  String) {
  fn setFormat(self , rsthis: & QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO9setFormatEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZN10QPictureIO9setFormatEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPictureIO::setDescription(const QString & );
impl /*struct*/ QPictureIO {
  pub fn setDescription<RetType, T: QPictureIO_setDescription<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDescription(self);
    // return 1;
  }
}

pub trait QPictureIO_setDescription<RetType> {
  fn setDescription(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::setDescription(const QString & );
impl<'a> /*trait*/ QPictureIO_setDescription<()> for (&'a QString) {
  fn setDescription(self , rsthis: & QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO14setDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QPictureIO14setDescriptionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QByteArray QPictureIO::pictureFormat(const QString & fileName);
impl<'a> /*trait*/ QPictureIO_pictureFormat_s<QByteArray> for (&'a QString) {
  fn pictureFormat_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO13pictureFormatERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QPictureIO13pictureFormatERK7QString(arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPictureIO::setIODevice(QIODevice * );
impl /*struct*/ QPictureIO {
  pub fn setIODevice<RetType, T: QPictureIO_setIODevice<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIODevice(self);
    // return 1;
  }
}

pub trait QPictureIO_setIODevice<RetType> {
  fn setIODevice(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::setIODevice(QIODevice * );
impl<'a> /*trait*/ QPictureIO_setIODevice<()> for (&'a QIODevice) {
  fn setIODevice(self , rsthis: & QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO11setIODeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QPictureIO11setIODeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPictureIO::setStatus(int );
impl /*struct*/ QPictureIO {
  pub fn setStatus<RetType, T: QPictureIO_setStatus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStatus(self);
    // return 1;
  }
}

pub trait QPictureIO_setStatus<RetType> {
  fn setStatus(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::setStatus(int );
impl<'a> /*trait*/ QPictureIO_setStatus<()> for (i32) {
  fn setStatus(self , rsthis: & QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO9setStatusEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QPictureIO9setStatusEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QIODevice * QPictureIO::ioDevice();
impl /*struct*/ QPictureIO {
  pub fn ioDevice<RetType, T: QPictureIO_ioDevice<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ioDevice(self);
    // return 1;
  }
}

pub trait QPictureIO_ioDevice<RetType> {
  fn ioDevice(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  QIODevice * QPictureIO::ioDevice();
impl<'a> /*trait*/ QPictureIO_ioDevice<QIODevice> for () {
  fn ioDevice(self , rsthis: & QPictureIO) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO8ioDeviceEv()};
    let mut ret = unsafe {_ZNK10QPictureIO8ioDeviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  float QPictureIO::gamma();
impl /*struct*/ QPictureIO {
  pub fn gamma<RetType, T: QPictureIO_gamma<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.gamma(self);
    // return 1;
  }
}

pub trait QPictureIO_gamma<RetType> {
  fn gamma(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  float QPictureIO::gamma();
impl<'a> /*trait*/ QPictureIO_gamma<f32> for () {
  fn gamma(self , rsthis: & QPictureIO) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO5gammaEv()};
    let mut ret = unsafe {_ZNK10QPictureIO5gammaEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

  // proto: static QList<QByteArray> QPictureIO::outputFormats();
impl /*struct*/ QPictureIO {
  pub fn outputFormats_s<RetType, T: QPictureIO_outputFormats_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.outputFormats_s();
    // return 1;
  }
}

pub trait QPictureIO_outputFormats_s<RetType> {
  fn outputFormats_s(self ) -> RetType;
}

  // proto: static QList<QByteArray> QPictureIO::outputFormats();
impl<'a> /*trait*/ QPictureIO_outputFormats_s<()> for () {
  fn outputFormats_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO13outputFormatsEv()};
     unsafe {_ZN10QPictureIO13outputFormatsEv()};
    // return 1;
  }
}

  // proto:  void QPictureIO::setPicture(const QPicture & );
impl /*struct*/ QPictureIO {
  pub fn setPicture<RetType, T: QPictureIO_setPicture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPicture(self);
    // return 1;
  }
}

pub trait QPictureIO_setPicture<RetType> {
  fn setPicture(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::setPicture(const QPicture & );
impl<'a> /*trait*/ QPictureIO_setPicture<()> for (&'a QPicture) {
  fn setPicture(self , rsthis: & QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO10setPictureERK8QPicture()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QPictureIO10setPictureERK8QPicture(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPictureIO::setParameters(const char * );
impl /*struct*/ QPictureIO {
  pub fn setParameters<RetType, T: QPictureIO_setParameters<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setParameters(self);
    // return 1;
  }
}

pub trait QPictureIO_setParameters<RetType> {
  fn setParameters(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::setParameters(const char * );
impl<'a> /*trait*/ QPictureIO_setParameters<()> for (&'a  String) {
  fn setParameters(self , rsthis: & QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO13setParametersEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZN10QPictureIO13setParametersEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPictureIO::QPictureIO();
impl<'a> /*trait*/ QPictureIO_New for () {
  fn New(self) -> QPictureIO {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIOC1Ev()};
    unsafe {_ZN10QPictureIOC1Ev(qthis)};
    let rsthis = QPictureIO{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn inheritFrom(qthis: *mut c_void) -> QPicture {
    return QPicture{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QPicture {
  type Target = QPaintDevice;

  fn deref(&self) -> &QPaintDevice {
    return & self.qbase;
  }
}
impl AsRef<QPaintDevice> for QPicture {
  fn as_ref(& self) -> & QPaintDevice {
    return & self.qbase;
  }
}
  // proto:  const char * QPicture::data();
impl /*struct*/ QPicture {
  pub fn data<RetType, T: QPicture_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QPicture_data<RetType> {
  fn data(self , rsthis: & QPicture) -> RetType;
}

  // proto:  const char * QPicture::data();
impl<'a> /*trait*/ QPicture_data<String> for () {
  fn data(self , rsthis: & QPicture) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture4dataEv()};
    let mut ret = unsafe {_ZNK8QPicture4dataEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto: static QStringList QPicture::inputFormatList();
impl /*struct*/ QPicture {
  pub fn inputFormatList_s<RetType, T: QPicture_inputFormatList_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.inputFormatList_s();
    // return 1;
  }
}

pub trait QPicture_inputFormatList_s<RetType> {
  fn inputFormatList_s(self ) -> RetType;
}

  // proto: static QStringList QPicture::inputFormatList();
impl<'a> /*trait*/ QPicture_inputFormatList_s<()> for () {
  fn inputFormatList_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture15inputFormatListEv()};
     unsafe {_ZN8QPicture15inputFormatListEv()};
    // return 1;
  }
}

  // proto:  void QPicture::swap(QPicture & other);
impl /*struct*/ QPicture {
  pub fn swap<RetType, T: QPicture_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QPicture_swap<RetType> {
  fn swap(self , rsthis: & QPicture) -> RetType;
}

  // proto:  void QPicture::swap(QPicture & other);
impl<'a> /*trait*/ QPicture_swap<()> for (&'a QPicture) {
  fn swap(self , rsthis: & QPicture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPicture4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  uint QPicture::size();
impl /*struct*/ QPicture {
  pub fn size<RetType, T: QPicture_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QPicture_size<RetType> {
  fn size(self , rsthis: & QPicture) -> RetType;
}

  // proto:  uint QPicture::size();
impl<'a> /*trait*/ QPicture_size<u32> for () {
  fn size(self , rsthis: & QPicture) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture4sizeEv()};
    let mut ret = unsafe {_ZNK8QPicture4sizeEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  bool QPicture::isNull();
impl /*struct*/ QPicture {
  pub fn isNull<RetType, T: QPicture_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QPicture_isNull<RetType> {
  fn isNull(self , rsthis: & QPicture) -> RetType;
}

  // proto:  bool QPicture::isNull();
impl<'a> /*trait*/ QPicture_isNull<i8> for () {
  fn isNull(self , rsthis: & QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture6isNullEv()};
    let mut ret = unsafe {_ZNK8QPicture6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QPicture::save(QIODevice * dev, const char * format);
impl /*struct*/ QPicture {
  pub fn save<RetType, T: QPicture_save<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.save(self);
    // return 1;
  }
}

pub trait QPicture_save<RetType> {
  fn save(self , rsthis: & QPicture) -> RetType;
}

  // proto:  bool QPicture::save(QIODevice * dev, const char * format);
impl<'a> /*trait*/ QPicture_save<i8> for (&'a QIODevice, &'a  String) {
  fn save(self , rsthis: & QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4saveEP9QIODevicePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN8QPicture4saveEP9QIODevicePKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPicture::detach();
impl /*struct*/ QPicture {
  pub fn detach<RetType, T: QPicture_detach<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.detach(self);
    // return 1;
  }
}

pub trait QPicture_detach<RetType> {
  fn detach(self , rsthis: & QPicture) -> RetType;
}

  // proto:  void QPicture::detach();
impl<'a> /*trait*/ QPicture_detach<()> for () {
  fn detach(self , rsthis: & QPicture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture6detachEv()};
     unsafe {_ZN8QPicture6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QList<QByteArray> QPicture::inputFormats();
impl /*struct*/ QPicture {
  pub fn inputFormats_s<RetType, T: QPicture_inputFormats_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.inputFormats_s();
    // return 1;
  }
}

pub trait QPicture_inputFormats_s<RetType> {
  fn inputFormats_s(self ) -> RetType;
}

  // proto: static QList<QByteArray> QPicture::inputFormats();
impl<'a> /*trait*/ QPicture_inputFormats_s<()> for () {
  fn inputFormats_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture12inputFormatsEv()};
     unsafe {_ZN8QPicture12inputFormatsEv()};
    // return 1;
  }
}

  // proto:  void QPicture::QPicture(int formatVersion);
impl /*struct*/ QPicture {
  pub fn New<T: QPicture_New>(value: T) -> QPicture {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QPicture_New {
  fn New(self) -> QPicture;
}

  // proto:  void QPicture::QPicture(int formatVersion);
impl<'a> /*trait*/ QPicture_New for (i32) {
  fn New(self) -> QPicture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPictureC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QPictureC1Ei(qthis, arg0)};
    let rsthis = QPicture{/**/qbase: QPaintDevice::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPicture::QPicture(const QPicture & );
impl<'a> /*trait*/ QPicture_New for (&'a QPicture) {
  fn New(self) -> QPicture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPictureC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QPictureC1ERKS_(qthis, arg0)};
    let rsthis = QPicture{/**/qbase: QPaintDevice::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QPicture::isDetached();
impl /*struct*/ QPicture {
  pub fn isDetached<RetType, T: QPicture_isDetached<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDetached(self);
    // return 1;
  }
}

pub trait QPicture_isDetached<RetType> {
  fn isDetached(self , rsthis: & QPicture) -> RetType;
}

  // proto:  bool QPicture::isDetached();
impl<'a> /*trait*/ QPicture_isDetached<i8> for () {
  fn isDetached(self , rsthis: & QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture10isDetachedEv()};
    let mut ret = unsafe {_ZNK8QPicture10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QStringList QPicture::outputFormatList();
impl /*struct*/ QPicture {
  pub fn outputFormatList_s<RetType, T: QPicture_outputFormatList_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.outputFormatList_s();
    // return 1;
  }
}

pub trait QPicture_outputFormatList_s<RetType> {
  fn outputFormatList_s(self ) -> RetType;
}

  // proto: static QStringList QPicture::outputFormatList();
impl<'a> /*trait*/ QPicture_outputFormatList_s<()> for () {
  fn outputFormatList_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture16outputFormatListEv()};
     unsafe {_ZN8QPicture16outputFormatListEv()};
    // return 1;
  }
}

  // proto:  void QPicture::setData(const char * data, uint size);
impl /*struct*/ QPicture {
  pub fn setData<RetType, T: QPicture_setData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QPicture_setData<RetType> {
  fn setData(self , rsthis: & QPicture) -> RetType;
}

  // proto:  void QPicture::setData(const char * data, uint size);
impl<'a> /*trait*/ QPicture_setData<()> for (&'a  String, u32) {
  fn setData(self , rsthis: & QPicture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture7setDataEPKcj()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN8QPicture7setDataEPKcj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto: static QList<QByteArray> QPicture::outputFormats();
impl /*struct*/ QPicture {
  pub fn outputFormats_s<RetType, T: QPicture_outputFormats_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.outputFormats_s();
    // return 1;
  }
}

pub trait QPicture_outputFormats_s<RetType> {
  fn outputFormats_s(self ) -> RetType;
}

  // proto: static QList<QByteArray> QPicture::outputFormats();
impl<'a> /*trait*/ QPicture_outputFormats_s<()> for () {
  fn outputFormats_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture13outputFormatsEv()};
     unsafe {_ZN8QPicture13outputFormatsEv()};
    // return 1;
  }
}

  // proto:  int QPicture::devType();
impl /*struct*/ QPicture {
  pub fn devType<RetType, T: QPicture_devType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.devType(self);
    // return 1;
  }
}

pub trait QPicture_devType<RetType> {
  fn devType(self , rsthis: & QPicture) -> RetType;
}

  // proto:  int QPicture::devType();
impl<'a> /*trait*/ QPicture_devType<i32> for () {
  fn devType(self , rsthis: & QPicture) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture7devTypeEv()};
    let mut ret = unsafe {_ZNK8QPicture7devTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static const char * QPicture::pictureFormat(const QString & fileName);
impl /*struct*/ QPicture {
  pub fn pictureFormat_s<RetType, T: QPicture_pictureFormat_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.pictureFormat_s();
    // return 1;
  }
}

pub trait QPicture_pictureFormat_s<RetType> {
  fn pictureFormat_s(self ) -> RetType;
}

  // proto: static const char * QPicture::pictureFormat(const QString & fileName);
impl<'a> /*trait*/ QPicture_pictureFormat_s<String> for (&'a QString) {
  fn pictureFormat_s(self ) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture13pictureFormatERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QPicture13pictureFormatERK7QString(arg0)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  bool QPicture::save(const QString & fileName, const char * format);
impl<'a> /*trait*/ QPicture_save<i8> for (&'a QString, &'a  String) {
  fn save(self , rsthis: & QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4saveERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN8QPicture4saveERK7QStringPKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QPicture::load(const QString & fileName, const char * format);
impl /*struct*/ QPicture {
  pub fn load<RetType, T: QPicture_load<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.load(self);
    // return 1;
  }
}

pub trait QPicture_load<RetType> {
  fn load(self , rsthis: & QPicture) -> RetType;
}

  // proto:  bool QPicture::load(const QString & fileName, const char * format);
impl<'a> /*trait*/ QPicture_load<i8> for (&'a QString, &'a  String) {
  fn load(self , rsthis: & QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4loadERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN8QPicture4loadERK7QStringPKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPicture::~QPicture();
impl /*struct*/ QPicture {
  pub fn Free<RetType, T: QPicture_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QPicture_Free<RetType> {
  fn Free(self , rsthis: & QPicture) -> RetType;
}

  // proto:  void QPicture::~QPicture();
impl<'a> /*trait*/ QPicture_Free<()> for () {
  fn Free(self , rsthis: & QPicture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPictureD0Ev()};
     unsafe {_ZN8QPictureD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPicture::setBoundingRect(const QRect & r);
impl /*struct*/ QPicture {
  pub fn setBoundingRect<RetType, T: QPicture_setBoundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBoundingRect(self);
    // return 1;
  }
}

pub trait QPicture_setBoundingRect<RetType> {
  fn setBoundingRect(self , rsthis: & QPicture) -> RetType;
}

  // proto:  void QPicture::setBoundingRect(const QRect & r);
impl<'a> /*trait*/ QPicture_setBoundingRect<()> for (&'a QRect) {
  fn setBoundingRect(self , rsthis: & QPicture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture15setBoundingRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPicture15setBoundingRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPicture::load(QIODevice * dev, const char * format);
impl<'a> /*trait*/ QPicture_load<i8> for (&'a QIODevice, &'a  String) {
  fn load(self , rsthis: & QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4loadEP9QIODevicePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN8QPicture4loadEP9QIODevicePKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRect QPicture::boundingRect();
impl /*struct*/ QPicture {
  pub fn boundingRect<RetType, T: QPicture_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QPicture_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QPicture) -> RetType;
}

  // proto:  QRect QPicture::boundingRect();
impl<'a> /*trait*/ QPicture_boundingRect<QRect> for () {
  fn boundingRect(self , rsthis: & QPicture) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture12boundingRectEv()};
    let mut ret = unsafe {_ZNK8QPicture12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPicture::play(QPainter * p);
impl /*struct*/ QPicture {
  pub fn play<RetType, T: QPicture_play<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.play(self);
    // return 1;
  }
}

pub trait QPicture_play<RetType> {
  fn play(self , rsthis: & QPicture) -> RetType;
}

  // proto:  bool QPicture::play(QPainter * p);
impl<'a> /*trait*/ QPicture_play<i8> for (&'a QPainter) {
  fn play(self , rsthis: & QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4playEP8QPainter()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QPicture4playEP8QPainter(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPaintEngine * QPicture::paintEngine();
impl /*struct*/ QPicture {
  pub fn paintEngine<RetType, T: QPicture_paintEngine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paintEngine(self);
    // return 1;
  }
}

pub trait QPicture_paintEngine<RetType> {
  fn paintEngine(self , rsthis: & QPicture) -> RetType;
}

  // proto:  QPaintEngine * QPicture::paintEngine();
impl<'a> /*trait*/ QPicture_paintEngine<QPaintEngine> for () {
  fn paintEngine(self , rsthis: & QPicture) -> QPaintEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPicture11paintEngineEv()};
    let mut ret = unsafe {_ZNK8QPicture11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

