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
use super::qpicture::QPicture;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QPictureIO::NewQPictureIO(const QString & fileName, const char * format);
  fn _ZN10QPictureIOC1ERK7QStringPKc(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_char) -> i32;
  // proto: QString QPictureIO::description();
  fn _ZNK10QPictureIO11descriptionEv() -> i32;
  // proto: QList<QByteArray> QPictureIO::inputFormats();
  fn _ZN10QPictureIO12inputFormatsEv() -> i32;
  // proto: void QPictureIO::setGamma(float );
  fn _ZN10QPictureIO8setGammaEf(arg0: c_float) -> i32;
  // proto: int QPictureIO::status();
  fn _ZNK10QPictureIO6statusEv() -> i32;
  // proto: int QPictureIO::quality();
  fn _ZNK10QPictureIO7qualityEv() -> i32;
  // proto: bool QPictureIO::write();
  fn _ZN10QPictureIO5writeEv() -> i32;
  // proto: void QPictureIO::setFileName(const QString & );
  fn _ZN10QPictureIO11setFileNameERK7QString(arg0: *const c_void) -> i32;
  // proto: void QPictureIO::FreeQPictureIO();
  fn _ZN10QPictureIOD0Ev() -> i32;
  // proto: const char * QPictureIO::parameters();
  fn _ZNK10QPictureIO10parametersEv() -> i32;
  // proto: QByteArray QPictureIO::pictureFormat(QIODevice * );
  fn _ZN10QPictureIO13pictureFormatEP9QIODevice(arg0: *mut c_void) -> i32;
  // proto: void QPictureIO::NewQPictureIO(const QPictureIO & );
  fn _ZN10QPictureIOC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QPictureIO::read();
  fn _ZN10QPictureIO4readEv() -> i32;
  // proto: QString QPictureIO::fileName();
  fn _ZNK10QPictureIO8fileNameEv() -> i32;
  // proto: void QPictureIO::NewQPictureIO(QIODevice * ioDevice, const char * format);
  fn _ZN10QPictureIOC1EP9QIODevicePKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char) -> i32;
  // proto: const char * QPictureIO::format();
  fn _ZNK10QPictureIO6formatEv() -> i32;
  // proto: void QPictureIO::setQuality(int );
  fn _ZN10QPictureIO10setQualityEi(arg0: c_int) -> i32;
  // proto: const QPicture & QPictureIO::picture();
  fn _ZNK10QPictureIO7pictureEv() -> i32;
  // proto: void QPictureIO::setFormat(const char * );
  fn _ZN10QPictureIO9setFormatEPKc(arg0: *const c_char) -> i32;
  // proto: void QPictureIO::setDescription(const QString & );
  fn _ZN10QPictureIO14setDescriptionERK7QString(arg0: *const c_void) -> i32;
  // proto: QByteArray QPictureIO::pictureFormat(const QString & fileName);
  fn _ZN10QPictureIO13pictureFormatERK7QString(arg0: *const c_void) -> i32;
  // proto: void QPictureIO::setIODevice(QIODevice * );
  fn _ZN10QPictureIO11setIODeviceEP9QIODevice(arg0: *mut c_void) -> i32;
  // proto: void QPictureIO::setStatus(int );
  fn _ZN10QPictureIO9setStatusEi(arg0: c_int) -> i32;
  // proto: QIODevice * QPictureIO::ioDevice();
  fn _ZNK10QPictureIO8ioDeviceEv() -> i32;
  // proto: float QPictureIO::gamma();
  fn _ZNK10QPictureIO5gammaEv() -> i32;
  // proto: QList<QByteArray> QPictureIO::outputFormats();
  fn _ZN10QPictureIO13outputFormatsEv() -> i32;
  // proto: void QPictureIO::setPicture(const QPicture & );
  fn _ZN10QPictureIO10setPictureERK8QPicture(arg0: *const c_void) -> i32;
  // proto: void QPictureIO::setParameters(const char * );
  fn _ZN10QPictureIO13setParametersEPKc(arg0: *const c_char) -> i32;
  // proto: void QPictureIO::NewQPictureIO();
  fn _ZN10QPictureIOC1Ev(qthis: *mut c_void) -> i32;
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
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN10QPictureIOC1ERK7QStringPKc(qthis, arg0, arg1)};
    let rsthis = QPictureIO{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn description<T: QPictureIO_description>(&mut self, value: T) -> i32 {
    value.description(self);
    return 1;
  }
}

pub trait QPictureIO_description {
  fn description(self, this: &mut QPictureIO) -> i32;
}

// proto: QString QPictureIO::description();
impl<'a> /*trait*/ QPictureIO_description for () {
  fn description(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO11descriptionEv()};
    unsafe {_ZNK10QPictureIO11descriptionEv()};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn inputFormats<T: QPictureIO_inputFormats>(&mut self, value: T) -> i32 {
    value.inputFormats(self);
    return 1;
  }
}

pub trait QPictureIO_inputFormats {
  fn inputFormats(self, this: &mut QPictureIO) -> i32;
}

// proto: QList<QByteArray> QPictureIO::inputFormats();
impl<'a> /*trait*/ QPictureIO_inputFormats for () {
  fn inputFormats(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO12inputFormatsEv()};
    unsafe {_ZN10QPictureIO12inputFormatsEv()};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn setGamma<T: QPictureIO_setGamma>(&mut self, value: T) -> i32 {
    value.setGamma(self);
    return 1;
  }
}

pub trait QPictureIO_setGamma {
  fn setGamma(self, this: &mut QPictureIO) -> i32;
}

// proto: void QPictureIO::setGamma(float );
impl<'a> /*trait*/ QPictureIO_setGamma for (f32) {
  fn setGamma(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO8setGammaEf()};
    let arg0 = self  as c_float;
    unsafe {_ZN10QPictureIO8setGammaEf(arg0)};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn status<T: QPictureIO_status>(&mut self, value: T) -> i32 {
    value.status(self);
    return 1;
  }
}

pub trait QPictureIO_status {
  fn status(self, this: &mut QPictureIO) -> i32;
}

// proto: int QPictureIO::status();
impl<'a> /*trait*/ QPictureIO_status for () {
  fn status(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO6statusEv()};
    unsafe {_ZNK10QPictureIO6statusEv()};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn quality<T: QPictureIO_quality>(&mut self, value: T) -> i32 {
    value.quality(self);
    return 1;
  }
}

pub trait QPictureIO_quality {
  fn quality(self, this: &mut QPictureIO) -> i32;
}

// proto: int QPictureIO::quality();
impl<'a> /*trait*/ QPictureIO_quality for () {
  fn quality(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO7qualityEv()};
    unsafe {_ZNK10QPictureIO7qualityEv()};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn write<T: QPictureIO_write>(&mut self, value: T) -> i32 {
    value.write(self);
    return 1;
  }
}

pub trait QPictureIO_write {
  fn write(self, this: &mut QPictureIO) -> i32;
}

// proto: bool QPictureIO::write();
impl<'a> /*trait*/ QPictureIO_write for () {
  fn write(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO5writeEv()};
    unsafe {_ZN10QPictureIO5writeEv()};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn setFileName<T: QPictureIO_setFileName>(&mut self, value: T) -> i32 {
    value.setFileName(self);
    return 1;
  }
}

pub trait QPictureIO_setFileName {
  fn setFileName(self, this: &mut QPictureIO) -> i32;
}

// proto: void QPictureIO::setFileName(const QString & );
impl<'a> /*trait*/ QPictureIO_setFileName for (&'a  QString) {
  fn setFileName(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QPictureIO11setFileNameERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn FreeQPictureIO<T: QPictureIO_FreeQPictureIO>(&mut self, value: T) -> i32 {
    value.FreeQPictureIO(self);
    return 1;
  }
}

pub trait QPictureIO_FreeQPictureIO {
  fn FreeQPictureIO(self, this: &mut QPictureIO) -> i32;
}

// proto: void QPictureIO::FreeQPictureIO();
impl<'a> /*trait*/ QPictureIO_FreeQPictureIO for () {
  fn FreeQPictureIO(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIOD0Ev()};
    unsafe {_ZN10QPictureIOD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn parameters<T: QPictureIO_parameters>(&mut self, value: T) -> i32 {
    value.parameters(self);
    return 1;
  }
}

pub trait QPictureIO_parameters {
  fn parameters(self, this: &mut QPictureIO) -> i32;
}

// proto: const char * QPictureIO::parameters();
impl<'a> /*trait*/ QPictureIO_parameters for () {
  fn parameters(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO10parametersEv()};
    unsafe {_ZNK10QPictureIO10parametersEv()};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn pictureFormat<T: QPictureIO_pictureFormat>(&mut self, value: T) -> i32 {
    value.pictureFormat(self);
    return 1;
  }
}

pub trait QPictureIO_pictureFormat {
  fn pictureFormat(self, this: &mut QPictureIO) -> i32;
}

// proto: QByteArray QPictureIO::pictureFormat(QIODevice * );
impl<'a> /*trait*/ QPictureIO_pictureFormat for (&'a mut QIODevice) {
  fn pictureFormat(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO13pictureFormatEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QPictureIO13pictureFormatEP9QIODevice(arg0)};
    return 1;
  }
}

// proto: void QPictureIO::NewQPictureIO(const QPictureIO & );
impl<'a> /*trait*/ QPictureIO_NewQPictureIO for (&'a  QPictureIO) {
  fn NewQPictureIO(self) -> QPictureIO {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIOC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QPictureIOC1ERKS_(qthis, arg0)};
    let rsthis = QPictureIO{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn read<T: QPictureIO_read>(&mut self, value: T) -> i32 {
    value.read(self);
    return 1;
  }
}

pub trait QPictureIO_read {
  fn read(self, this: &mut QPictureIO) -> i32;
}

// proto: bool QPictureIO::read();
impl<'a> /*trait*/ QPictureIO_read for () {
  fn read(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO4readEv()};
    unsafe {_ZN10QPictureIO4readEv()};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn fileName<T: QPictureIO_fileName>(&mut self, value: T) -> i32 {
    value.fileName(self);
    return 1;
  }
}

pub trait QPictureIO_fileName {
  fn fileName(self, this: &mut QPictureIO) -> i32;
}

// proto: QString QPictureIO::fileName();
impl<'a> /*trait*/ QPictureIO_fileName for () {
  fn fileName(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO8fileNameEv()};
    unsafe {_ZNK10QPictureIO8fileNameEv()};
    return 1;
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
  pub fn format<T: QPictureIO_format>(&mut self, value: T) -> i32 {
    value.format(self);
    return 1;
  }
}

pub trait QPictureIO_format {
  fn format(self, this: &mut QPictureIO) -> i32;
}

// proto: const char * QPictureIO::format();
impl<'a> /*trait*/ QPictureIO_format for () {
  fn format(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO6formatEv()};
    unsafe {_ZNK10QPictureIO6formatEv()};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn setQuality<T: QPictureIO_setQuality>(&mut self, value: T) -> i32 {
    value.setQuality(self);
    return 1;
  }
}

pub trait QPictureIO_setQuality {
  fn setQuality(self, this: &mut QPictureIO) -> i32;
}

// proto: void QPictureIO::setQuality(int );
impl<'a> /*trait*/ QPictureIO_setQuality for (i32) {
  fn setQuality(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO10setQualityEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QPictureIO10setQualityEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn picture<T: QPictureIO_picture>(&mut self, value: T) -> i32 {
    value.picture(self);
    return 1;
  }
}

pub trait QPictureIO_picture {
  fn picture(self, this: &mut QPictureIO) -> i32;
}

// proto: const QPicture & QPictureIO::picture();
impl<'a> /*trait*/ QPictureIO_picture for () {
  fn picture(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO7pictureEv()};
    unsafe {_ZNK10QPictureIO7pictureEv()};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn setFormat<T: QPictureIO_setFormat>(&mut self, value: T) -> i32 {
    value.setFormat(self);
    return 1;
  }
}

pub trait QPictureIO_setFormat {
  fn setFormat(self, this: &mut QPictureIO) -> i32;
}

// proto: void QPictureIO::setFormat(const char * );
impl<'a> /*trait*/ QPictureIO_setFormat for (&'a  String) {
  fn setFormat(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO9setFormatEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN10QPictureIO9setFormatEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn setDescription<T: QPictureIO_setDescription>(&mut self, value: T) -> i32 {
    value.setDescription(self);
    return 1;
  }
}

pub trait QPictureIO_setDescription {
  fn setDescription(self, this: &mut QPictureIO) -> i32;
}

// proto: void QPictureIO::setDescription(const QString & );
impl<'a> /*trait*/ QPictureIO_setDescription for (&'a  QString) {
  fn setDescription(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO14setDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QPictureIO14setDescriptionERK7QString(arg0)};
    return 1;
  }
}

// proto: QByteArray QPictureIO::pictureFormat(const QString & fileName);
impl<'a> /*trait*/ QPictureIO_pictureFormat for (&'a  QString) {
  fn pictureFormat(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO13pictureFormatERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QPictureIO13pictureFormatERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn setIODevice<T: QPictureIO_setIODevice>(&mut self, value: T) -> i32 {
    value.setIODevice(self);
    return 1;
  }
}

pub trait QPictureIO_setIODevice {
  fn setIODevice(self, this: &mut QPictureIO) -> i32;
}

// proto: void QPictureIO::setIODevice(QIODevice * );
impl<'a> /*trait*/ QPictureIO_setIODevice for (&'a mut QIODevice) {
  fn setIODevice(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO11setIODeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QPictureIO11setIODeviceEP9QIODevice(arg0)};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn setStatus<T: QPictureIO_setStatus>(&mut self, value: T) -> i32 {
    value.setStatus(self);
    return 1;
  }
}

pub trait QPictureIO_setStatus {
  fn setStatus(self, this: &mut QPictureIO) -> i32;
}

// proto: void QPictureIO::setStatus(int );
impl<'a> /*trait*/ QPictureIO_setStatus for (i32) {
  fn setStatus(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO9setStatusEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QPictureIO9setStatusEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn ioDevice<T: QPictureIO_ioDevice>(&mut self, value: T) -> i32 {
    value.ioDevice(self);
    return 1;
  }
}

pub trait QPictureIO_ioDevice {
  fn ioDevice(self, this: &mut QPictureIO) -> i32;
}

// proto: QIODevice * QPictureIO::ioDevice();
impl<'a> /*trait*/ QPictureIO_ioDevice for () {
  fn ioDevice(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO8ioDeviceEv()};
    unsafe {_ZNK10QPictureIO8ioDeviceEv()};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn gamma<T: QPictureIO_gamma>(&mut self, value: T) -> i32 {
    value.gamma(self);
    return 1;
  }
}

pub trait QPictureIO_gamma {
  fn gamma(self, this: &mut QPictureIO) -> i32;
}

// proto: float QPictureIO::gamma();
impl<'a> /*trait*/ QPictureIO_gamma for () {
  fn gamma(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPictureIO5gammaEv()};
    unsafe {_ZNK10QPictureIO5gammaEv()};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn outputFormats<T: QPictureIO_outputFormats>(&mut self, value: T) -> i32 {
    value.outputFormats(self);
    return 1;
  }
}

pub trait QPictureIO_outputFormats {
  fn outputFormats(self, this: &mut QPictureIO) -> i32;
}

// proto: QList<QByteArray> QPictureIO::outputFormats();
impl<'a> /*trait*/ QPictureIO_outputFormats for () {
  fn outputFormats(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO13outputFormatsEv()};
    unsafe {_ZN10QPictureIO13outputFormatsEv()};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn setPicture<T: QPictureIO_setPicture>(&mut self, value: T) -> i32 {
    value.setPicture(self);
    return 1;
  }
}

pub trait QPictureIO_setPicture {
  fn setPicture(self, this: &mut QPictureIO) -> i32;
}

// proto: void QPictureIO::setPicture(const QPicture & );
impl<'a> /*trait*/ QPictureIO_setPicture for (&'a  QPicture) {
  fn setPicture(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO10setPictureERK8QPicture()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QPictureIO10setPictureERK8QPicture(arg0)};
    return 1;
  }
}

impl /*struct*/ QPictureIO {
  pub fn setParameters<T: QPictureIO_setParameters>(&mut self, value: T) -> i32 {
    value.setParameters(self);
    return 1;
  }
}

pub trait QPictureIO_setParameters {
  fn setParameters(self, this: &mut QPictureIO) -> i32;
}

// proto: void QPictureIO::setParameters(const char * );
impl<'a> /*trait*/ QPictureIO_setParameters for (&'a  String) {
  fn setParameters(self, this: &mut QPictureIO) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPictureIO13setParametersEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN10QPictureIO13setParametersEPKc(arg0)};
    return 1;
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

