// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qiodevice::QIODevice;
use super::qbytearray::QByteArray;
use super::qpicture::QPicture;

// ext block begin
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
}

// body block begin
// class sizeof(QPictureIO)=8
pub struct QPictureIO {
  pub qclsinst: *mut c_void,
}

  // proto:  void QPictureIO::QPictureIO(const QString & fileName, const char * format);
impl /*struct*/ QPictureIO {
  pub fn NewQPictureIO<T: QPictureIO_NewQPictureIO>(value: T) -> QPictureIO {
    let rsthis = value.NewQPictureIO();
    return rsthis;
    // return 1;
  }
}

pub trait QPictureIO_NewQPictureIO {
  fn NewQPictureIO(self) -> QPictureIO;
}

  // proto:  void QPictureIO::QPictureIO(const QString & fileName, const char * format);
impl<'a> /*trait*/ QPictureIO_NewQPictureIO for (QString, &'a  String) {
  fn NewQPictureIO(self) -> QPictureIO {
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
  pub fn description<RetType, T: QPictureIO_description<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.description(self);
    // return 1;
  }
}

pub trait QPictureIO_description<RetType> {
  fn description(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  QString QPictureIO::description();
impl<'a> /*trait*/ QPictureIO_description<QString> for () {
  fn description(self , rsthis: &mut QPictureIO) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO11descriptionEv()};
    let mut ret = unsafe {_ZNK10QPictureIO11descriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
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
  pub fn setGamma<RetType, T: QPictureIO_setGamma<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setGamma(self);
    // return 1;
  }
}

pub trait QPictureIO_setGamma<RetType> {
  fn setGamma(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::setGamma(float );
impl<'a> /*trait*/ QPictureIO_setGamma<()> for (f32) {
  fn setGamma(self , rsthis: &mut QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO8setGammaEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN10QPictureIO8setGammaEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QPictureIO::status();
impl /*struct*/ QPictureIO {
  pub fn status<RetType, T: QPictureIO_status<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.status(self);
    // return 1;
  }
}

pub trait QPictureIO_status<RetType> {
  fn status(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  int QPictureIO::status();
impl<'a> /*trait*/ QPictureIO_status<i32> for () {
  fn status(self , rsthis: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO6statusEv()};
    let mut ret = unsafe {_ZNK10QPictureIO6statusEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QPictureIO::quality();
impl /*struct*/ QPictureIO {
  pub fn quality<RetType, T: QPictureIO_quality<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.quality(self);
    // return 1;
  }
}

pub trait QPictureIO_quality<RetType> {
  fn quality(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  int QPictureIO::quality();
impl<'a> /*trait*/ QPictureIO_quality<i32> for () {
  fn quality(self , rsthis: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO7qualityEv()};
    let mut ret = unsafe {_ZNK10QPictureIO7qualityEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QPictureIO::write();
impl /*struct*/ QPictureIO {
  pub fn write<RetType, T: QPictureIO_write<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.write(self);
    // return 1;
  }
}

pub trait QPictureIO_write<RetType> {
  fn write(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  bool QPictureIO::write();
impl<'a> /*trait*/ QPictureIO_write<i8> for () {
  fn write(self , rsthis: &mut QPictureIO) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO5writeEv()};
    let mut ret = unsafe {_ZN10QPictureIO5writeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPictureIO::setFileName(const QString & );
impl /*struct*/ QPictureIO {
  pub fn setFileName<RetType, T: QPictureIO_setFileName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFileName(self);
    // return 1;
  }
}

pub trait QPictureIO_setFileName<RetType> {
  fn setFileName(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::setFileName(const QString & );
impl<'a> /*trait*/ QPictureIO_setFileName<()> for (QString) {
  fn setFileName(self , rsthis: &mut QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QPictureIO11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPictureIO::~QPictureIO();
impl /*struct*/ QPictureIO {
  pub fn FreeQPictureIO<RetType, T: QPictureIO_FreeQPictureIO<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQPictureIO(self);
    // return 1;
  }
}

pub trait QPictureIO_FreeQPictureIO<RetType> {
  fn FreeQPictureIO(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::~QPictureIO();
impl<'a> /*trait*/ QPictureIO_FreeQPictureIO<()> for () {
  fn FreeQPictureIO(self , rsthis: &mut QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIOD0Ev()};
     unsafe {_ZN10QPictureIOD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const char * QPictureIO::parameters();
impl /*struct*/ QPictureIO {
  pub fn parameters<RetType, T: QPictureIO_parameters<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.parameters(self);
    // return 1;
  }
}

pub trait QPictureIO_parameters<RetType> {
  fn parameters(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  const char * QPictureIO::parameters();
impl<'a> /*trait*/ QPictureIO_parameters<String> for () {
  fn parameters(self , rsthis: &mut QPictureIO) -> String {
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
impl<'a> /*trait*/ QPictureIO_pictureFormat_s<QByteArray> for (QIODevice) {
  fn pictureFormat_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO13pictureFormatEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QPictureIO13pictureFormatEP9QIODevice(arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPictureIO::QPictureIO(const QPictureIO & );
impl<'a> /*trait*/ QPictureIO_NewQPictureIO for (QPictureIO) {
  fn NewQPictureIO(self) -> QPictureIO {
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
  pub fn read<RetType, T: QPictureIO_read<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.read(self);
    // return 1;
  }
}

pub trait QPictureIO_read<RetType> {
  fn read(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  bool QPictureIO::read();
impl<'a> /*trait*/ QPictureIO_read<i8> for () {
  fn read(self , rsthis: &mut QPictureIO) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO4readEv()};
    let mut ret = unsafe {_ZN10QPictureIO4readEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QPictureIO::fileName();
impl /*struct*/ QPictureIO {
  pub fn fileName<RetType, T: QPictureIO_fileName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QPictureIO_fileName<RetType> {
  fn fileName(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  QString QPictureIO::fileName();
impl<'a> /*trait*/ QPictureIO_fileName<QString> for () {
  fn fileName(self , rsthis: &mut QPictureIO) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO8fileNameEv()};
    let mut ret = unsafe {_ZNK10QPictureIO8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPictureIO::QPictureIO(QIODevice * ioDevice, const char * format);
impl<'a> /*trait*/ QPictureIO_NewQPictureIO for (QIODevice, &'a  String) {
  fn NewQPictureIO(self) -> QPictureIO {
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
  pub fn format<RetType, T: QPictureIO_format<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QPictureIO_format<RetType> {
  fn format(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  const char * QPictureIO::format();
impl<'a> /*trait*/ QPictureIO_format<String> for () {
  fn format(self , rsthis: &mut QPictureIO) -> String {
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
  pub fn setQuality<RetType, T: QPictureIO_setQuality<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setQuality(self);
    // return 1;
  }
}

pub trait QPictureIO_setQuality<RetType> {
  fn setQuality(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::setQuality(int );
impl<'a> /*trait*/ QPictureIO_setQuality<()> for (i32) {
  fn setQuality(self , rsthis: &mut QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO10setQualityEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QPictureIO10setQualityEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QPicture & QPictureIO::picture();
impl /*struct*/ QPictureIO {
  pub fn picture<RetType, T: QPictureIO_picture<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.picture(self);
    // return 1;
  }
}

pub trait QPictureIO_picture<RetType> {
  fn picture(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  const QPicture & QPictureIO::picture();
impl<'a> /*trait*/ QPictureIO_picture<QPicture> for () {
  fn picture(self , rsthis: &mut QPictureIO) -> QPicture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO7pictureEv()};
    let mut ret = unsafe {_ZNK10QPictureIO7pictureEv(rsthis.qclsinst)};
    let mut ret1 = QPicture{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPictureIO::setFormat(const char * );
impl /*struct*/ QPictureIO {
  pub fn setFormat<RetType, T: QPictureIO_setFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QPictureIO_setFormat<RetType> {
  fn setFormat(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::setFormat(const char * );
impl<'a> /*trait*/ QPictureIO_setFormat<()> for (&'a  String) {
  fn setFormat(self , rsthis: &mut QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO9setFormatEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZN10QPictureIO9setFormatEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPictureIO::setDescription(const QString & );
impl /*struct*/ QPictureIO {
  pub fn setDescription<RetType, T: QPictureIO_setDescription<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDescription(self);
    // return 1;
  }
}

pub trait QPictureIO_setDescription<RetType> {
  fn setDescription(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::setDescription(const QString & );
impl<'a> /*trait*/ QPictureIO_setDescription<()> for (QString) {
  fn setDescription(self , rsthis: &mut QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO14setDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QPictureIO14setDescriptionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QByteArray QPictureIO::pictureFormat(const QString & fileName);
impl<'a> /*trait*/ QPictureIO_pictureFormat_s<QByteArray> for (QString) {
  fn pictureFormat_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO13pictureFormatERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QPictureIO13pictureFormatERK7QString(arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPictureIO::setIODevice(QIODevice * );
impl /*struct*/ QPictureIO {
  pub fn setIODevice<RetType, T: QPictureIO_setIODevice<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setIODevice(self);
    // return 1;
  }
}

pub trait QPictureIO_setIODevice<RetType> {
  fn setIODevice(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::setIODevice(QIODevice * );
impl<'a> /*trait*/ QPictureIO_setIODevice<()> for (QIODevice) {
  fn setIODevice(self , rsthis: &mut QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO11setIODeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QPictureIO11setIODeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPictureIO::setStatus(int );
impl /*struct*/ QPictureIO {
  pub fn setStatus<RetType, T: QPictureIO_setStatus<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setStatus(self);
    // return 1;
  }
}

pub trait QPictureIO_setStatus<RetType> {
  fn setStatus(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::setStatus(int );
impl<'a> /*trait*/ QPictureIO_setStatus<()> for (i32) {
  fn setStatus(self , rsthis: &mut QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO9setStatusEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QPictureIO9setStatusEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QIODevice * QPictureIO::ioDevice();
impl /*struct*/ QPictureIO {
  pub fn ioDevice<RetType, T: QPictureIO_ioDevice<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.ioDevice(self);
    // return 1;
  }
}

pub trait QPictureIO_ioDevice<RetType> {
  fn ioDevice(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  QIODevice * QPictureIO::ioDevice();
impl<'a> /*trait*/ QPictureIO_ioDevice<QIODevice> for () {
  fn ioDevice(self , rsthis: &mut QPictureIO) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO8ioDeviceEv()};
    let mut ret = unsafe {_ZNK10QPictureIO8ioDeviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  float QPictureIO::gamma();
impl /*struct*/ QPictureIO {
  pub fn gamma<RetType, T: QPictureIO_gamma<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.gamma(self);
    // return 1;
  }
}

pub trait QPictureIO_gamma<RetType> {
  fn gamma(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  float QPictureIO::gamma();
impl<'a> /*trait*/ QPictureIO_gamma<f32> for () {
  fn gamma(self , rsthis: &mut QPictureIO) -> f32 {
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
  pub fn setPicture<RetType, T: QPictureIO_setPicture<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPicture(self);
    // return 1;
  }
}

pub trait QPictureIO_setPicture<RetType> {
  fn setPicture(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::setPicture(const QPicture & );
impl<'a> /*trait*/ QPictureIO_setPicture<()> for (QPicture) {
  fn setPicture(self , rsthis: &mut QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO10setPictureERK8QPicture()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QPictureIO10setPictureERK8QPicture(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPictureIO::setParameters(const char * );
impl /*struct*/ QPictureIO {
  pub fn setParameters<RetType, T: QPictureIO_setParameters<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setParameters(self);
    // return 1;
  }
}

pub trait QPictureIO_setParameters<RetType> {
  fn setParameters(self , rsthis: &mut QPictureIO) -> RetType;
}

  // proto:  void QPictureIO::setParameters(const char * );
impl<'a> /*trait*/ QPictureIO_setParameters<()> for (&'a  String) {
  fn setParameters(self , rsthis: &mut QPictureIO) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO13setParametersEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZN10QPictureIO13setParametersEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPictureIO::QPictureIO();
impl<'a> /*trait*/ QPictureIO_NewQPictureIO for () {
  fn NewQPictureIO(self) -> QPictureIO {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIOC1Ev()};
    unsafe {_ZN10QPictureIOC1Ev(qthis)};
    let rsthis = QPictureIO{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

