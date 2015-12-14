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
  // proto:  void QPictureIO::NewQPictureIO(const QString & fileName, const char * format);
  fn _ZN10QPictureIOC1ERK7QStringPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char) ;
  // proto:  QString QPictureIO::description();
  fn _ZNK10QPictureIO11descriptionEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QList<QByteArray> QPictureIO::inputFormats();
  fn _ZN10QPictureIO12inputFormatsEv() ;
  // proto:  void QPictureIO::setGamma(float );
  fn _ZN10QPictureIO8setGammaEf(qthis: *mut c_void, arg0: c_float) ;
  // proto:  int QPictureIO::status();
  fn _ZNK10QPictureIO6statusEv(qthis: *mut c_void) -> c_int;
  // proto:  int QPictureIO::quality();
  fn _ZNK10QPictureIO7qualityEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QPictureIO::write();
  fn _ZN10QPictureIO5writeEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QPictureIO::setFileName(const QString & );
  fn _ZN10QPictureIO11setFileNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPictureIO::FreeQPictureIO();
  fn _ZN10QPictureIOD0Ev(qthis: *mut c_void) ;
  // proto:  const char * QPictureIO::parameters();
  fn _ZNK10QPictureIO10parametersEv(qthis: *mut c_void) -> *const c_char;
  // proto: static QByteArray QPictureIO::pictureFormat(QIODevice * );
  fn _ZN10QPictureIO13pictureFormatEP9QIODevice(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPictureIO::NewQPictureIO(const QPictureIO & );
  fn _ZN10QPictureIOC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QPictureIO::read();
  fn _ZN10QPictureIO4readEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QPictureIO::fileName();
  fn _ZNK10QPictureIO8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPictureIO::NewQPictureIO(QIODevice * ioDevice, const char * format);
  fn _ZN10QPictureIOC1EP9QIODevicePKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char) ;
  // proto:  const char * QPictureIO::format();
  fn _ZNK10QPictureIO6formatEv(qthis: *mut c_void) -> *const c_char;
  // proto:  void QPictureIO::setQuality(int );
  fn _ZN10QPictureIO10setQualityEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  const QPicture & QPictureIO::picture();
  fn _ZNK10QPictureIO7pictureEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPictureIO::setFormat(const char * );
  fn _ZN10QPictureIO9setFormatEPKc(qthis: *mut c_void, arg0: *const c_char) ;
  // proto:  void QPictureIO::setDescription(const QString & );
  fn _ZN10QPictureIO14setDescriptionERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QByteArray QPictureIO::pictureFormat(const QString & fileName);
  fn _ZN10QPictureIO13pictureFormatERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPictureIO::setIODevice(QIODevice * );
  fn _ZN10QPictureIO11setIODeviceEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPictureIO::setStatus(int );
  fn _ZN10QPictureIO9setStatusEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QIODevice * QPictureIO::ioDevice();
  fn _ZNK10QPictureIO8ioDeviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  float QPictureIO::gamma();
  fn _ZNK10QPictureIO5gammaEv(qthis: *mut c_void) -> c_float;
  // proto: static QList<QByteArray> QPictureIO::outputFormats();
  fn _ZN10QPictureIO13outputFormatsEv() ;
  // proto:  void QPictureIO::setPicture(const QPicture & );
  fn _ZN10QPictureIO10setPictureERK8QPicture(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPictureIO::setParameters(const char * );
  fn _ZN10QPictureIO13setParametersEPKc(qthis: *mut c_void, arg0: *const c_char) ;
  // proto:  void QPictureIO::NewQPictureIO();
  fn _ZN10QPictureIOC1Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QPictureIO)=8
pub struct QPictureIO {
  pub qclsinst: *mut c_void,
}

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

// proto: void QPictureIO::NewQPictureIO(const QString & fileName, const char * format);
impl<'a> /*trait*/ QPictureIO_NewQPictureIO for (&'a  QString, &'a  String) {
  fn NewQPictureIO(self) -> QPictureIO {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIOC1ERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN10QPictureIOC1ERK7QStringPKc(qthis, arg0, arg1)};
    let rsthis = QPictureIO{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn description<T: QPictureIO_description>(&mut self, value: T) -> QString {
    return value.description(self);
    // return 1;
  }
}

pub trait QPictureIO_description {
  fn description(self, rsthis: &mut QPictureIO) -> QString;
}

// proto:  QString QPictureIO::description();
impl<'a> /*trait*/ QPictureIO_description for () {
  fn description(self, rsthis: &mut QPictureIO) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO11descriptionEv()};
    let mut ret = unsafe {_ZNK10QPictureIO11descriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn inputFormats<T: QPictureIO_inputFormats>(&mut self, value: T)  {
     value.inputFormats(self);
    // return 1;
  }
}

pub trait QPictureIO_inputFormats {
  fn inputFormats(self, rsthis: &mut QPictureIO) ;
}

// proto: static QList<QByteArray> QPictureIO::inputFormats();
impl<'a> /*trait*/ QPictureIO_inputFormats for () {
  fn inputFormats(self, rsthis: &mut QPictureIO)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO12inputFormatsEv()};
     unsafe {_ZN10QPictureIO12inputFormatsEv()};
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn setGamma<T: QPictureIO_setGamma>(&mut self, value: T)  {
     value.setGamma(self);
    // return 1;
  }
}

pub trait QPictureIO_setGamma {
  fn setGamma(self, rsthis: &mut QPictureIO) ;
}

// proto:  void QPictureIO::setGamma(float );
impl<'a> /*trait*/ QPictureIO_setGamma for (f32) {
  fn setGamma(self, rsthis: &mut QPictureIO)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO8setGammaEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN10QPictureIO8setGammaEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn status<T: QPictureIO_status>(&mut self, value: T) -> i32 {
    return value.status(self);
    // return 1;
  }
}

pub trait QPictureIO_status {
  fn status(self, rsthis: &mut QPictureIO) -> i32;
}

// proto:  int QPictureIO::status();
impl<'a> /*trait*/ QPictureIO_status for () {
  fn status(self, rsthis: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO6statusEv()};
    let mut ret = unsafe {_ZNK10QPictureIO6statusEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn quality<T: QPictureIO_quality>(&mut self, value: T) -> i32 {
    return value.quality(self);
    // return 1;
  }
}

pub trait QPictureIO_quality {
  fn quality(self, rsthis: &mut QPictureIO) -> i32;
}

// proto:  int QPictureIO::quality();
impl<'a> /*trait*/ QPictureIO_quality for () {
  fn quality(self, rsthis: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO7qualityEv()};
    let mut ret = unsafe {_ZNK10QPictureIO7qualityEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn write<T: QPictureIO_write>(&mut self, value: T) -> i8 {
    return value.write(self);
    // return 1;
  }
}

pub trait QPictureIO_write {
  fn write(self, rsthis: &mut QPictureIO) -> i8;
}

// proto:  bool QPictureIO::write();
impl<'a> /*trait*/ QPictureIO_write for () {
  fn write(self, rsthis: &mut QPictureIO) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO5writeEv()};
    let mut ret = unsafe {_ZN10QPictureIO5writeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn setFileName<T: QPictureIO_setFileName>(&mut self, value: T)  {
     value.setFileName(self);
    // return 1;
  }
}

pub trait QPictureIO_setFileName {
  fn setFileName(self, rsthis: &mut QPictureIO) ;
}

// proto:  void QPictureIO::setFileName(const QString & );
impl<'a> /*trait*/ QPictureIO_setFileName for (&'a  QString) {
  fn setFileName(self, rsthis: &mut QPictureIO)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QPictureIO11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn FreeQPictureIO<T: QPictureIO_FreeQPictureIO>(&mut self, value: T)  {
     value.FreeQPictureIO(self);
    // return 1;
  }
}

pub trait QPictureIO_FreeQPictureIO {
  fn FreeQPictureIO(self, rsthis: &mut QPictureIO) ;
}

// proto:  void QPictureIO::FreeQPictureIO();
impl<'a> /*trait*/ QPictureIO_FreeQPictureIO for () {
  fn FreeQPictureIO(self, rsthis: &mut QPictureIO)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIOD0Ev()};
     unsafe {_ZN10QPictureIOD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn parameters<T: QPictureIO_parameters>(&mut self, value: T) -> String {
    return value.parameters(self);
    // return 1;
  }
}

pub trait QPictureIO_parameters {
  fn parameters(self, rsthis: &mut QPictureIO) -> String;
}

// proto:  const char * QPictureIO::parameters();
impl<'a> /*trait*/ QPictureIO_parameters for () {
  fn parameters(self, rsthis: &mut QPictureIO) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO10parametersEv()};
    let mut ret = unsafe {_ZNK10QPictureIO10parametersEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn pictureFormat<T: QPictureIO_pictureFormat>(&mut self, value: T) -> QByteArray {
    return value.pictureFormat(self);
    // return 1;
  }
}

pub trait QPictureIO_pictureFormat {
  fn pictureFormat(self, rsthis: &mut QPictureIO) -> QByteArray;
}

// proto: static QByteArray QPictureIO::pictureFormat(QIODevice * );
impl<'a> /*trait*/ QPictureIO_pictureFormat for (&'a mut QIODevice) {
  fn pictureFormat(self, rsthis: &mut QPictureIO) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO13pictureFormatEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QPictureIO13pictureFormatEP9QIODevice(arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QPictureIO::NewQPictureIO(const QPictureIO & );
impl<'a> /*trait*/ QPictureIO_NewQPictureIO for (&'a  QPictureIO) {
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

impl /*struct*/ QPictureIO {
  pub fn read<T: QPictureIO_read>(&mut self, value: T) -> i8 {
    return value.read(self);
    // return 1;
  }
}

pub trait QPictureIO_read {
  fn read(self, rsthis: &mut QPictureIO) -> i8;
}

// proto:  bool QPictureIO::read();
impl<'a> /*trait*/ QPictureIO_read for () {
  fn read(self, rsthis: &mut QPictureIO) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO4readEv()};
    let mut ret = unsafe {_ZN10QPictureIO4readEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn fileName<T: QPictureIO_fileName>(&mut self, value: T) -> QString {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QPictureIO_fileName {
  fn fileName(self, rsthis: &mut QPictureIO) -> QString;
}

// proto:  QString QPictureIO::fileName();
impl<'a> /*trait*/ QPictureIO_fileName for () {
  fn fileName(self, rsthis: &mut QPictureIO) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO8fileNameEv()};
    let mut ret = unsafe {_ZNK10QPictureIO8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QPictureIO::NewQPictureIO(QIODevice * ioDevice, const char * format);
impl<'a> /*trait*/ QPictureIO_NewQPictureIO for (&'a mut QIODevice, &'a  String) {
  fn NewQPictureIO(self) -> QPictureIO {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIOC1EP9QIODevicePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN10QPictureIOC1EP9QIODevicePKc(qthis, arg0, arg1)};
    let rsthis = QPictureIO{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn format<T: QPictureIO_format>(&mut self, value: T) -> String {
    return value.format(self);
    // return 1;
  }
}

pub trait QPictureIO_format {
  fn format(self, rsthis: &mut QPictureIO) -> String;
}

// proto:  const char * QPictureIO::format();
impl<'a> /*trait*/ QPictureIO_format for () {
  fn format(self, rsthis: &mut QPictureIO) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO6formatEv()};
    let mut ret = unsafe {_ZNK10QPictureIO6formatEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn setQuality<T: QPictureIO_setQuality>(&mut self, value: T)  {
     value.setQuality(self);
    // return 1;
  }
}

pub trait QPictureIO_setQuality {
  fn setQuality(self, rsthis: &mut QPictureIO) ;
}

// proto:  void QPictureIO::setQuality(int );
impl<'a> /*trait*/ QPictureIO_setQuality for (i32) {
  fn setQuality(self, rsthis: &mut QPictureIO)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO10setQualityEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QPictureIO10setQualityEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn picture<T: QPictureIO_picture>(&mut self, value: T) -> QPicture {
    return value.picture(self);
    // return 1;
  }
}

pub trait QPictureIO_picture {
  fn picture(self, rsthis: &mut QPictureIO) -> QPicture;
}

// proto:  const QPicture & QPictureIO::picture();
impl<'a> /*trait*/ QPictureIO_picture for () {
  fn picture(self, rsthis: &mut QPictureIO) -> QPicture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO7pictureEv()};
    let mut ret = unsafe {_ZNK10QPictureIO7pictureEv(rsthis.qclsinst)};
    let mut ret1 = QPicture{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn setFormat<T: QPictureIO_setFormat>(&mut self, value: T)  {
     value.setFormat(self);
    // return 1;
  }
}

pub trait QPictureIO_setFormat {
  fn setFormat(self, rsthis: &mut QPictureIO) ;
}

// proto:  void QPictureIO::setFormat(const char * );
impl<'a> /*trait*/ QPictureIO_setFormat for (&'a  String) {
  fn setFormat(self, rsthis: &mut QPictureIO)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO9setFormatEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
     unsafe {_ZN10QPictureIO9setFormatEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn setDescription<T: QPictureIO_setDescription>(&mut self, value: T)  {
     value.setDescription(self);
    // return 1;
  }
}

pub trait QPictureIO_setDescription {
  fn setDescription(self, rsthis: &mut QPictureIO) ;
}

// proto:  void QPictureIO::setDescription(const QString & );
impl<'a> /*trait*/ QPictureIO_setDescription for (&'a  QString) {
  fn setDescription(self, rsthis: &mut QPictureIO)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO14setDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QPictureIO14setDescriptionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: static QByteArray QPictureIO::pictureFormat(const QString & fileName);
impl<'a> /*trait*/ QPictureIO_pictureFormat for (&'a  QString) {
  fn pictureFormat(self, rsthis: &mut QPictureIO) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO13pictureFormatERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QPictureIO13pictureFormatERK7QString(arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn setIODevice<T: QPictureIO_setIODevice>(&mut self, value: T)  {
     value.setIODevice(self);
    // return 1;
  }
}

pub trait QPictureIO_setIODevice {
  fn setIODevice(self, rsthis: &mut QPictureIO) ;
}

// proto:  void QPictureIO::setIODevice(QIODevice * );
impl<'a> /*trait*/ QPictureIO_setIODevice for (&'a mut QIODevice) {
  fn setIODevice(self, rsthis: &mut QPictureIO)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO11setIODeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QPictureIO11setIODeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn setStatus<T: QPictureIO_setStatus>(&mut self, value: T)  {
     value.setStatus(self);
    // return 1;
  }
}

pub trait QPictureIO_setStatus {
  fn setStatus(self, rsthis: &mut QPictureIO) ;
}

// proto:  void QPictureIO::setStatus(int );
impl<'a> /*trait*/ QPictureIO_setStatus for (i32) {
  fn setStatus(self, rsthis: &mut QPictureIO)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO9setStatusEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QPictureIO9setStatusEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn ioDevice<T: QPictureIO_ioDevice>(&mut self, value: T) -> QIODevice {
    return value.ioDevice(self);
    // return 1;
  }
}

pub trait QPictureIO_ioDevice {
  fn ioDevice(self, rsthis: &mut QPictureIO) -> QIODevice;
}

// proto:  QIODevice * QPictureIO::ioDevice();
impl<'a> /*trait*/ QPictureIO_ioDevice for () {
  fn ioDevice(self, rsthis: &mut QPictureIO) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO8ioDeviceEv()};
    let mut ret = unsafe {_ZNK10QPictureIO8ioDeviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn gamma<T: QPictureIO_gamma>(&mut self, value: T) -> f32 {
    return value.gamma(self);
    // return 1;
  }
}

pub trait QPictureIO_gamma {
  fn gamma(self, rsthis: &mut QPictureIO) -> f32;
}

// proto:  float QPictureIO::gamma();
impl<'a> /*trait*/ QPictureIO_gamma for () {
  fn gamma(self, rsthis: &mut QPictureIO) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO5gammaEv()};
    let mut ret = unsafe {_ZNK10QPictureIO5gammaEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn outputFormats<T: QPictureIO_outputFormats>(&mut self, value: T)  {
     value.outputFormats(self);
    // return 1;
  }
}

pub trait QPictureIO_outputFormats {
  fn outputFormats(self, rsthis: &mut QPictureIO) ;
}

// proto: static QList<QByteArray> QPictureIO::outputFormats();
impl<'a> /*trait*/ QPictureIO_outputFormats for () {
  fn outputFormats(self, rsthis: &mut QPictureIO)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO13outputFormatsEv()};
     unsafe {_ZN10QPictureIO13outputFormatsEv()};
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn setPicture<T: QPictureIO_setPicture>(&mut self, value: T)  {
     value.setPicture(self);
    // return 1;
  }
}

pub trait QPictureIO_setPicture {
  fn setPicture(self, rsthis: &mut QPictureIO) ;
}

// proto:  void QPictureIO::setPicture(const QPicture & );
impl<'a> /*trait*/ QPictureIO_setPicture for (&'a  QPicture) {
  fn setPicture(self, rsthis: &mut QPictureIO)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO10setPictureERK8QPicture()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QPictureIO10setPictureERK8QPicture(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn setParameters<T: QPictureIO_setParameters>(&mut self, value: T)  {
     value.setParameters(self);
    // return 1;
  }
}

pub trait QPictureIO_setParameters {
  fn setParameters(self, rsthis: &mut QPictureIO) ;
}

// proto:  void QPictureIO::setParameters(const char * );
impl<'a> /*trait*/ QPictureIO_setParameters for (&'a  String) {
  fn setParameters(self, rsthis: &mut QPictureIO)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO13setParametersEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
     unsafe {_ZN10QPictureIO13setParametersEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QPictureIO::NewQPictureIO();
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

