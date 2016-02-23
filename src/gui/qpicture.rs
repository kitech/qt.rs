// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
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
use super::super::core::qstring::*; // 771
// use super::qlist::*; // 775
use super::super::core::qiodevice::*; // 771
use super::super::core::qbytearray::*; // 771
// use super::qpicture::QPicture; // 773
use super::qpaintdevice::*; // 773
use super::super::core::qstringlist::*; // 771
use super::super::core::qrect::*; // 771
use super::qpainter::*; // 773
use super::qpaintengine::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPictureIO_Class_Size() -> c_int;
  // proto:  void QPictureIO::QPictureIO(const QString & fileName, const char * format);
  fn C_ZN10QPictureIOC2ERK7QStringPKc(arg0: *mut c_void, arg1: *mut c_char) -> u64;
  // proto:  QString QPictureIO::description();
  fn C_ZNK10QPictureIO11descriptionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QList<QByteArray> QPictureIO::inputFormats();
  fn C_ZN10QPictureIO12inputFormatsEv() -> *mut c_void;
  // proto:  void QPictureIO::setGamma(float );
  fn C_ZN10QPictureIO8setGammaEf(qthis: u64 /* *mut c_void*/, arg0: c_float);
  // proto:  int QPictureIO::status();
  fn C_ZNK10QPictureIO6statusEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QPictureIO::quality();
  fn C_ZNK10QPictureIO7qualityEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QPictureIO::write();
  fn C_ZN10QPictureIO5writeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QPictureIO::setFileName(const QString & );
  fn C_ZN10QPictureIO11setFileNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPictureIO::~QPictureIO();
  fn C_ZN10QPictureIOD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const char * QPictureIO::parameters();
  fn C_ZNK10QPictureIO10parametersEv(qthis: u64 /* *mut c_void*/) -> *mut c_char;
  // proto: static QByteArray QPictureIO::pictureFormat(QIODevice * );
  fn C_ZN10QPictureIO13pictureFormatEP9QIODevice(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QPictureIO::read();
  fn C_ZN10QPictureIO4readEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QPictureIO::fileName();
  fn C_ZNK10QPictureIO8fileNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPictureIO::QPictureIO(QIODevice * ioDevice, const char * format);
  fn C_ZN10QPictureIOC2EP9QIODevicePKc(arg0: *mut c_void, arg1: *mut c_char) -> u64;
  // proto:  const char * QPictureIO::format();
  fn C_ZNK10QPictureIO6formatEv(qthis: u64 /* *mut c_void*/) -> *mut c_char;
  // proto:  void QPictureIO::setQuality(int );
  fn C_ZN10QPictureIO10setQualityEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  const QPicture & QPictureIO::picture();
  fn C_ZNK10QPictureIO7pictureEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPictureIO::setFormat(const char * );
  fn C_ZN10QPictureIO9setFormatEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char);
  // proto:  void QPictureIO::setDescription(const QString & );
  fn C_ZN10QPictureIO14setDescriptionERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QByteArray QPictureIO::pictureFormat(const QString & fileName);
  fn C_ZN10QPictureIO13pictureFormatERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPictureIO::setIODevice(QIODevice * );
  fn C_ZN10QPictureIO11setIODeviceEP9QIODevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPictureIO::setStatus(int );
  fn C_ZN10QPictureIO9setStatusEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QIODevice * QPictureIO::ioDevice();
  fn C_ZNK10QPictureIO8ioDeviceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  float QPictureIO::gamma();
  fn C_ZNK10QPictureIO5gammaEv(qthis: u64 /* *mut c_void*/) -> c_float;
  // proto: static QList<QByteArray> QPictureIO::outputFormats();
  fn C_ZN10QPictureIO13outputFormatsEv() -> *mut c_void;
  // proto:  void QPictureIO::setPicture(const QPicture & );
  fn C_ZN10QPictureIO10setPictureERK8QPicture(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPictureIO::setParameters(const char * );
  fn C_ZN10QPictureIO13setParametersEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char);
  // proto:  void QPictureIO::QPictureIO();
  fn C_ZN10QPictureIOC2Ev() -> u64;
  fn QPicture_Class_Size() -> c_int;
  // proto:  const char * QPicture::data();
  fn C_ZNK8QPicture4dataEv(qthis: u64 /* *mut c_void*/) -> *mut c_char;
  // proto: static QStringList QPicture::inputFormatList();
  fn C_ZN8QPicture15inputFormatListEv() -> *mut c_void;
  // proto:  void QPicture::swap(QPicture & other);
  fn C_ZN8QPicture4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  uint QPicture::size();
  fn C_ZNK8QPicture4sizeEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  bool QPicture::isNull();
  fn C_ZNK8QPicture6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QPicture::save(QIODevice * dev, const char * format);
  fn C_ZN8QPicture4saveEP9QIODevicePKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  void QPicture::detach();
  fn C_ZN8QPicture6detachEv(qthis: u64 /* *mut c_void*/);
  // proto: static QList<QByteArray> QPicture::inputFormats();
  fn C_ZN8QPicture12inputFormatsEv() -> *mut c_void;
  // proto:  void QPicture::QPicture(int formatVersion);
  fn C_ZN8QPictureC2Ei(arg0: c_int) -> u64;
  // proto:  void QPicture::QPicture(const QPicture & );
  fn C_ZN8QPictureC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  bool QPicture::isDetached();
  fn C_ZNK8QPicture10isDetachedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static QStringList QPicture::outputFormatList();
  fn C_ZN8QPicture16outputFormatListEv() -> *mut c_void;
  // proto:  void QPicture::setData(const char * data, uint size);
  fn C_ZN8QPicture7setDataEPKcj(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_uint);
  // proto: static QList<QByteArray> QPicture::outputFormats();
  fn C_ZN8QPicture13outputFormatsEv() -> *mut c_void;
  // proto:  int QPicture::devType();
  fn C_ZNK8QPicture7devTypeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto: static const char * QPicture::pictureFormat(const QString & fileName);
  fn C_ZN8QPicture13pictureFormatERK7QString(arg0: *mut c_void) -> *mut c_char;
  // proto:  bool QPicture::save(const QString & fileName, const char * format);
  fn C_ZN8QPicture4saveERK7QStringPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  bool QPicture::load(const QString & fileName, const char * format);
  fn C_ZN8QPicture4loadERK7QStringPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  void QPicture::~QPicture();
  fn C_ZN8QPictureD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QPicture::setBoundingRect(const QRect & r);
  fn C_ZN8QPicture15setBoundingRectERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QPicture::load(QIODevice * dev, const char * format);
  fn C_ZN8QPicture4loadEP9QIODevicePKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  QRect QPicture::boundingRect();
  fn C_ZNK8QPicture12boundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QPicture::play(QPainter * p);
  fn C_ZN8QPicture4playEP8QPainter(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QPaintEngine * QPicture::paintEngine();
  fn C_ZNK8QPicture11paintEngineEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QPictureIO)=8
#[derive(Default)]
pub struct QPictureIO {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QPicture)=1
#[derive(Default)]
pub struct QPicture {
  qbase: QPaintDevice,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QPictureIO {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPictureIO {
    return QPictureIO{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QPictureIO::QPictureIO(const QString & fileName, const char * format);
impl /*struct*/ QPictureIO {
  pub fn new<T: QPictureIO_new>(value: T) -> QPictureIO {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPictureIO_new {
  fn new(self) -> QPictureIO;
}

  // proto:  void QPictureIO::QPictureIO(const QString & fileName, const char * format);
impl<'a> /*trait*/ QPictureIO_new for (&'a QString, &'a  String) {
  fn new(self) -> QPictureIO {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIOC2ERK7QStringPKc()};
    let ctysz: c_int = unsafe{QPictureIO_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let qthis: u64 = unsafe {C_ZN10QPictureIOC2ERK7QStringPKc(arg0, arg1)};
    let rsthis = QPictureIO{qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK10QPictureIO11descriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
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
impl<'a> /*trait*/ QPictureIO_inputFormats_s<u64> for () {
  fn inputFormats_s(self ) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO12inputFormatsEv()};
    let mut ret = unsafe {C_ZN10QPictureIO12inputFormatsEv()};
    return ret as u64; // 5
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
     unsafe {C_ZN10QPictureIO8setGammaEf(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZNK10QPictureIO6statusEv(rsthis.qclsinst)};
    return ret as i32; // 1
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
    let mut ret = unsafe {C_ZNK10QPictureIO7qualityEv(rsthis.qclsinst)};
    return ret as i32; // 1
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
    let mut ret = unsafe {C_ZN10QPictureIO5writeEv(rsthis.qclsinst)};
    return ret as i8; // 1
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
     unsafe {C_ZN10QPictureIO11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPictureIO::~QPictureIO();
impl /*struct*/ QPictureIO {
  pub fn free<RetType, T: QPictureIO_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QPictureIO_free<RetType> {
  fn free(self , rsthis: & QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::~QPictureIO();
impl<'a> /*trait*/ QPictureIO_free<()> for () {
  fn free(self , rsthis: & QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIOD2Ev()};
     unsafe {C_ZN10QPictureIOD2Ev(rsthis.qclsinst)};
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
    let mut ret = unsafe {C_ZNK10QPictureIO10parametersEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {C_ZN10QPictureIO13pictureFormatEP9QIODevice(arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
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
    let mut ret = unsafe {C_ZN10QPictureIO4readEv(rsthis.qclsinst)};
    return ret as i8; // 1
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
    let mut ret = unsafe {C_ZNK10QPictureIO8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPictureIO::QPictureIO(QIODevice * ioDevice, const char * format);
impl<'a> /*trait*/ QPictureIO_new for (&'a QIODevice, &'a  String) {
  fn new(self) -> QPictureIO {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIOC2EP9QIODevicePKc()};
    let ctysz: c_int = unsafe{QPictureIO_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let qthis: u64 = unsafe {C_ZN10QPictureIOC2EP9QIODevicePKc(arg0, arg1)};
    let rsthis = QPictureIO{qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK10QPictureIO6formatEv(rsthis.qclsinst)};
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
     unsafe {C_ZN10QPictureIO10setQualityEi(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZNK10QPictureIO7pictureEv(rsthis.qclsinst)};
    let mut ret1 = QPicture::inheritFrom(ret as u64);
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
     unsafe {C_ZN10QPictureIO9setFormatEPKc(rsthis.qclsinst, arg0)};
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
     unsafe {C_ZN10QPictureIO14setDescriptionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QByteArray QPictureIO::pictureFormat(const QString & fileName);
impl<'a> /*trait*/ QPictureIO_pictureFormat_s<QByteArray> for (&'a QString) {
  fn pictureFormat_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO13pictureFormatERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN10QPictureIO13pictureFormatERK7QString(arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
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
     unsafe {C_ZN10QPictureIO11setIODeviceEP9QIODevice(rsthis.qclsinst, arg0)};
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
     unsafe {C_ZN10QPictureIO9setStatusEi(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZNK10QPictureIO8ioDeviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK10QPictureIO5gammaEv(rsthis.qclsinst)};
    return ret as f32; // 1
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
impl<'a> /*trait*/ QPictureIO_outputFormats_s<u64> for () {
  fn outputFormats_s(self ) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO13outputFormatsEv()};
    let mut ret = unsafe {C_ZN10QPictureIO13outputFormatsEv()};
    return ret as u64; // 5
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
     unsafe {C_ZN10QPictureIO10setPictureERK8QPicture(rsthis.qclsinst, arg0)};
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
     unsafe {C_ZN10QPictureIO13setParametersEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPictureIO::QPictureIO();
impl<'a> /*trait*/ QPictureIO_new for () {
  fn new(self) -> QPictureIO {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIOC2Ev()};
    let ctysz: c_int = unsafe{QPictureIO_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN10QPictureIOC2Ev()};
    let rsthis = QPictureIO{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPicture {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPicture {
    return QPicture{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK8QPicture4dataEv(rsthis.qclsinst)};
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
impl<'a> /*trait*/ QPicture_inputFormatList_s<QStringList> for () {
  fn inputFormatList_s(self ) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture15inputFormatListEv()};
    let mut ret = unsafe {C_ZN8QPicture15inputFormatListEv()};
    let mut ret1 = QStringList::inheritFrom(ret as u64);
    return ret1;
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
     unsafe {C_ZN8QPicture4swapERS_(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZNK8QPicture4sizeEv(rsthis.qclsinst)};
    return ret as u32; // 1
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
    let mut ret = unsafe {C_ZNK8QPicture6isNullEv(rsthis.qclsinst)};
    return ret as i8; // 1
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
impl<'a> /*trait*/ QPicture_save<i8> for (&'a QIODevice, Option<&'a  String>) {
  fn save(self , rsthis: & QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4saveEP9QIODevicePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {0 as *const u8} else {self.1.unwrap().as_ptr()})  as *mut c_char;
    let mut ret = unsafe {C_ZN8QPicture4saveEP9QIODevicePKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8; // 1
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
     unsafe {C_ZN8QPicture6detachEv(rsthis.qclsinst)};
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
impl<'a> /*trait*/ QPicture_inputFormats_s<u64> for () {
  fn inputFormats_s(self ) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture12inputFormatsEv()};
    let mut ret = unsafe {C_ZN8QPicture12inputFormatsEv()};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  void QPicture::QPicture(int formatVersion);
impl /*struct*/ QPicture {
  pub fn new<T: QPicture_new>(value: T) -> QPicture {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPicture_new {
  fn new(self) -> QPicture;
}

  // proto:  void QPicture::QPicture(int formatVersion);
impl<'a> /*trait*/ QPicture_new for (Option<i32>) {
  fn new(self) -> QPicture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPictureC2Ei()};
    let ctysz: c_int = unsafe{QPicture_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {-1} else {self.unwrap()})  as c_int;
    let qthis: u64 = unsafe {C_ZN8QPictureC2Ei(arg0)};
    let rsthis = QPicture{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPicture::QPicture(const QPicture & );
impl<'a> /*trait*/ QPicture_new for (&'a QPicture) {
  fn new(self) -> QPicture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPictureC2ERKS_()};
    let ctysz: c_int = unsafe{QPicture_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN8QPictureC2ERKS_(arg0)};
    let rsthis = QPicture{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK8QPicture10isDetachedEv(rsthis.qclsinst)};
    return ret as i8; // 1
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
impl<'a> /*trait*/ QPicture_outputFormatList_s<QStringList> for () {
  fn outputFormatList_s(self ) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture16outputFormatListEv()};
    let mut ret = unsafe {C_ZN8QPicture16outputFormatListEv()};
    let mut ret1 = QStringList::inheritFrom(ret as u64);
    return ret1;
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
     unsafe {C_ZN8QPicture7setDataEPKcj(rsthis.qclsinst, arg0, arg1)};
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
impl<'a> /*trait*/ QPicture_outputFormats_s<u64> for () {
  fn outputFormats_s(self ) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture13outputFormatsEv()};
    let mut ret = unsafe {C_ZN8QPicture13outputFormatsEv()};
    return ret as u64; // 5
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
    let mut ret = unsafe {C_ZNK8QPicture7devTypeEv(rsthis.qclsinst)};
    return ret as i32; // 1
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
    let mut ret = unsafe {C_ZN8QPicture13pictureFormatERK7QString(arg0)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  bool QPicture::save(const QString & fileName, const char * format);
impl<'a> /*trait*/ QPicture_save<i8> for (&'a QString, Option<&'a  String>) {
  fn save(self , rsthis: & QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4saveERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {0 as *const u8} else {self.1.unwrap().as_ptr()})  as *mut c_char;
    let mut ret = unsafe {C_ZN8QPicture4saveERK7QStringPKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8; // 1
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
impl<'a> /*trait*/ QPicture_load<i8> for (&'a QString, Option<&'a  String>) {
  fn load(self , rsthis: & QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4loadERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {0 as *const u8} else {self.1.unwrap().as_ptr()})  as *mut c_char;
    let mut ret = unsafe {C_ZN8QPicture4loadERK7QStringPKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QPicture::~QPicture();
impl /*struct*/ QPicture {
  pub fn free<RetType, T: QPicture_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QPicture_free<RetType> {
  fn free(self , rsthis: & QPicture) -> RetType;
}

  // proto:  void QPicture::~QPicture();
impl<'a> /*trait*/ QPicture_free<()> for () {
  fn free(self , rsthis: & QPicture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPictureD2Ev()};
     unsafe {C_ZN8QPictureD2Ev(rsthis.qclsinst)};
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
     unsafe {C_ZN8QPicture15setBoundingRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPicture::load(QIODevice * dev, const char * format);
impl<'a> /*trait*/ QPicture_load<i8> for (&'a QIODevice, Option<&'a  String>) {
  fn load(self , rsthis: & QPicture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPicture4loadEP9QIODevicePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {0 as *const u8} else {self.1.unwrap().as_ptr()})  as *mut c_char;
    let mut ret = unsafe {C_ZN8QPicture4loadEP9QIODevicePKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8; // 1
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
    let mut ret = unsafe {C_ZNK8QPicture12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZN8QPicture4playEP8QPainter(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
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
    let mut ret = unsafe {C_ZNK8QPicture11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

