// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qimage::QImage;
use super::qbytearray::QByteArray;
use super::qiodevice::QIODevice;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QImageWriter::setText(const QString & key, const QString & text);
  fn _ZN12QImageWriter7setTextERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QImageWriter::setGamma(float gamma);
  fn _ZN12QImageWriter8setGammaEf(arg0: c_float) -> i32;
  // proto: void QImageWriter::setFileName(const QString & fileName);
  fn _ZN12QImageWriter11setFileNameERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QImageWriter::optimizedWrite();
  fn _ZNK12QImageWriter14optimizedWriteEv() -> i32;
  // proto: void QImageWriter::FreeQImageWriter();
  fn _ZN12QImageWriterD0Ev() -> i32;
  // proto: QIODevice * QImageWriter::device();
  fn _ZNK12QImageWriter6deviceEv() -> i32;
  // proto: QByteArray QImageWriter::subType();
  fn _ZNK12QImageWriter7subTypeEv() -> i32;
  // proto: QList<QByteArray> QImageWriter::supportedMimeTypes();
  fn _ZN12QImageWriter18supportedMimeTypesEv() -> i32;
  // proto: int QImageWriter::quality();
  fn _ZNK12QImageWriter7qualityEv() -> i32;
  // proto: bool QImageWriter::write(const QImage & image);
  fn _ZN12QImageWriter5writeERK6QImage(arg0: *const c_void) -> i32;
  // proto: void QImageWriter::setCompression(int compression);
  fn _ZN12QImageWriter14setCompressionEi(arg0: c_int) -> i32;
  // proto: QList<QByteArray> QImageWriter::supportedImageFormats();
  fn _ZN12QImageWriter21supportedImageFormatsEv() -> i32;
  // proto: QString QImageWriter::fileName();
  fn _ZNK12QImageWriter8fileNameEv() -> i32;
  // proto: void QImageWriter::setOptimizedWrite(bool optimize);
  fn _ZN12QImageWriter17setOptimizedWriteEb(arg0: int8_t) -> i32;
  // proto: QString QImageWriter::errorString();
  fn _ZNK12QImageWriter11errorStringEv() -> i32;
  // proto: void QImageWriter::setQuality(int quality);
  fn _ZN12QImageWriter10setQualityEi(arg0: c_int) -> i32;
  // proto: float QImageWriter::gamma();
  fn _ZNK12QImageWriter5gammaEv() -> i32;
  // proto: QString QImageWriter::description();
  fn _ZNK12QImageWriter11descriptionEv() -> i32;
  // proto: void QImageWriter::NewQImageWriter();
  fn _ZN12QImageWriterC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QImageWriter::setFormat(const QByteArray & format);
  fn _ZN12QImageWriter9setFormatERK10QByteArray(arg0: *const c_void) -> i32;
  // proto: void QImageWriter::NewQImageWriter(const QString & fileName, const QByteArray & format);
  fn _ZN12QImageWriterC1ERK7QStringRK10QByteArray(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QImageWriter::setDevice(QIODevice * device);
  fn _ZN12QImageWriter9setDeviceEP9QIODevice(arg0: *mut c_void) -> i32;
  // proto: void QImageWriter::setSubType(const QByteArray & type);
  fn _ZN12QImageWriter10setSubTypeERK10QByteArray(arg0: *const c_void) -> i32;
  // proto: bool QImageWriter::progressiveScanWrite();
  fn _ZNK12QImageWriter20progressiveScanWriteEv() -> i32;
  // proto: QByteArray QImageWriter::format();
  fn _ZNK12QImageWriter6formatEv() -> i32;
  // proto: QList<QByteArray> QImageWriter::supportedSubTypes();
  fn _ZNK12QImageWriter17supportedSubTypesEv() -> i32;
  // proto: bool QImageWriter::canWrite();
  fn _ZNK12QImageWriter8canWriteEv() -> i32;
  // proto: void QImageWriter::NewQImageWriter(QIODevice * device, const QByteArray & format);
  fn _ZN12QImageWriterC1EP9QIODeviceRK10QByteArray(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_void) -> i32;
  // proto: int QImageWriter::compression();
  fn _ZNK12QImageWriter11compressionEv() -> i32;
  // proto: void QImageWriter::setProgressiveScanWrite(bool progressive);
  fn _ZN12QImageWriter23setProgressiveScanWriteEb(arg0: int8_t) -> i32;
  // proto: void QImageWriter::setDescription(const QString & description);
  fn _ZN12QImageWriter14setDescriptionERK7QString(arg0: *const c_void) -> i32;
  // proto: void QImageWriter::NewQImageWriter(const QImageWriter & );
  fn _ZN12QImageWriterC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QImageWriter)=8
pub struct QImageWriter {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QImageWriter {
  pub fn setText<T: QImageWriter_setText>(&mut self, value: T) -> i32 {
    value.setText(self);
    return 1;
  }
}

pub trait QImageWriter_setText {
  fn setText(self, this: &mut QImageWriter) -> i32;
}

// proto: void QImageWriter::setText(const QString & key, const QString & text);
impl<'a> /*trait*/ QImageWriter_setText for (&'a  QString, &'a  QString) {
  fn setText(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter7setTextERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN12QImageWriter7setTextERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn setGamma<T: QImageWriter_setGamma>(&mut self, value: T) -> i32 {
    value.setGamma(self);
    return 1;
  }
}

pub trait QImageWriter_setGamma {
  fn setGamma(self, this: &mut QImageWriter) -> i32;
}

// proto: void QImageWriter::setGamma(float gamma);
impl<'a> /*trait*/ QImageWriter_setGamma for (f32) {
  fn setGamma(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter8setGammaEf()};
    let arg0 = self  as c_float;
    unsafe {_ZN12QImageWriter8setGammaEf(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn setFileName<T: QImageWriter_setFileName>(&mut self, value: T) -> i32 {
    value.setFileName(self);
    return 1;
  }
}

pub trait QImageWriter_setFileName {
  fn setFileName(self, this: &mut QImageWriter) -> i32;
}

// proto: void QImageWriter::setFileName(const QString & fileName);
impl<'a> /*trait*/ QImageWriter_setFileName for (&'a  QString) {
  fn setFileName(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QImageWriter11setFileNameERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn optimizedWrite<T: QImageWriter_optimizedWrite>(&mut self, value: T) -> i32 {
    value.optimizedWrite(self);
    return 1;
  }
}

pub trait QImageWriter_optimizedWrite {
  fn optimizedWrite(self, this: &mut QImageWriter) -> i32;
}

// proto: bool QImageWriter::optimizedWrite();
impl<'a> /*trait*/ QImageWriter_optimizedWrite for () {
  fn optimizedWrite(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter14optimizedWriteEv()};
    unsafe {_ZNK12QImageWriter14optimizedWriteEv()};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn FreeQImageWriter<T: QImageWriter_FreeQImageWriter>(&mut self, value: T) -> i32 {
    value.FreeQImageWriter(self);
    return 1;
  }
}

pub trait QImageWriter_FreeQImageWriter {
  fn FreeQImageWriter(self, this: &mut QImageWriter) -> i32;
}

// proto: void QImageWriter::FreeQImageWriter();
impl<'a> /*trait*/ QImageWriter_FreeQImageWriter for () {
  fn FreeQImageWriter(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriterD0Ev()};
    unsafe {_ZN12QImageWriterD0Ev()};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn device<T: QImageWriter_device>(&mut self, value: T) -> i32 {
    value.device(self);
    return 1;
  }
}

pub trait QImageWriter_device {
  fn device(self, this: &mut QImageWriter) -> i32;
}

// proto: QIODevice * QImageWriter::device();
impl<'a> /*trait*/ QImageWriter_device for () {
  fn device(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter6deviceEv()};
    unsafe {_ZNK12QImageWriter6deviceEv()};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn subType<T: QImageWriter_subType>(&mut self, value: T) -> i32 {
    value.subType(self);
    return 1;
  }
}

pub trait QImageWriter_subType {
  fn subType(self, this: &mut QImageWriter) -> i32;
}

// proto: QByteArray QImageWriter::subType();
impl<'a> /*trait*/ QImageWriter_subType for () {
  fn subType(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter7subTypeEv()};
    unsafe {_ZNK12QImageWriter7subTypeEv()};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn supportedMimeTypes<T: QImageWriter_supportedMimeTypes>(&mut self, value: T) -> i32 {
    value.supportedMimeTypes(self);
    return 1;
  }
}

pub trait QImageWriter_supportedMimeTypes {
  fn supportedMimeTypes(self, this: &mut QImageWriter) -> i32;
}

// proto: QList<QByteArray> QImageWriter::supportedMimeTypes();
impl<'a> /*trait*/ QImageWriter_supportedMimeTypes for () {
  fn supportedMimeTypes(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter18supportedMimeTypesEv()};
    unsafe {_ZN12QImageWriter18supportedMimeTypesEv()};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn quality<T: QImageWriter_quality>(&mut self, value: T) -> i32 {
    value.quality(self);
    return 1;
  }
}

pub trait QImageWriter_quality {
  fn quality(self, this: &mut QImageWriter) -> i32;
}

// proto: int QImageWriter::quality();
impl<'a> /*trait*/ QImageWriter_quality for () {
  fn quality(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter7qualityEv()};
    unsafe {_ZNK12QImageWriter7qualityEv()};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn write<T: QImageWriter_write>(&mut self, value: T) -> i32 {
    value.write(self);
    return 1;
  }
}

pub trait QImageWriter_write {
  fn write(self, this: &mut QImageWriter) -> i32;
}

// proto: bool QImageWriter::write(const QImage & image);
impl<'a> /*trait*/ QImageWriter_write for (&'a  QImage) {
  fn write(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter5writeERK6QImage()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QImageWriter5writeERK6QImage(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn setCompression<T: QImageWriter_setCompression>(&mut self, value: T) -> i32 {
    value.setCompression(self);
    return 1;
  }
}

pub trait QImageWriter_setCompression {
  fn setCompression(self, this: &mut QImageWriter) -> i32;
}

// proto: void QImageWriter::setCompression(int compression);
impl<'a> /*trait*/ QImageWriter_setCompression for (i32) {
  fn setCompression(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter14setCompressionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QImageWriter14setCompressionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn supportedImageFormats<T: QImageWriter_supportedImageFormats>(&mut self, value: T) -> i32 {
    value.supportedImageFormats(self);
    return 1;
  }
}

pub trait QImageWriter_supportedImageFormats {
  fn supportedImageFormats(self, this: &mut QImageWriter) -> i32;
}

// proto: QList<QByteArray> QImageWriter::supportedImageFormats();
impl<'a> /*trait*/ QImageWriter_supportedImageFormats for () {
  fn supportedImageFormats(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter21supportedImageFormatsEv()};
    unsafe {_ZN12QImageWriter21supportedImageFormatsEv()};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn fileName<T: QImageWriter_fileName>(&mut self, value: T) -> i32 {
    value.fileName(self);
    return 1;
  }
}

pub trait QImageWriter_fileName {
  fn fileName(self, this: &mut QImageWriter) -> i32;
}

// proto: QString QImageWriter::fileName();
impl<'a> /*trait*/ QImageWriter_fileName for () {
  fn fileName(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter8fileNameEv()};
    unsafe {_ZNK12QImageWriter8fileNameEv()};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn setOptimizedWrite<T: QImageWriter_setOptimizedWrite>(&mut self, value: T) -> i32 {
    value.setOptimizedWrite(self);
    return 1;
  }
}

pub trait QImageWriter_setOptimizedWrite {
  fn setOptimizedWrite(self, this: &mut QImageWriter) -> i32;
}

// proto: void QImageWriter::setOptimizedWrite(bool optimize);
impl<'a> /*trait*/ QImageWriter_setOptimizedWrite for (i8) {
  fn setOptimizedWrite(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter17setOptimizedWriteEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN12QImageWriter17setOptimizedWriteEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn errorString<T: QImageWriter_errorString>(&mut self, value: T) -> i32 {
    value.errorString(self);
    return 1;
  }
}

pub trait QImageWriter_errorString {
  fn errorString(self, this: &mut QImageWriter) -> i32;
}

// proto: QString QImageWriter::errorString();
impl<'a> /*trait*/ QImageWriter_errorString for () {
  fn errorString(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter11errorStringEv()};
    unsafe {_ZNK12QImageWriter11errorStringEv()};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn setQuality<T: QImageWriter_setQuality>(&mut self, value: T) -> i32 {
    value.setQuality(self);
    return 1;
  }
}

pub trait QImageWriter_setQuality {
  fn setQuality(self, this: &mut QImageWriter) -> i32;
}

// proto: void QImageWriter::setQuality(int quality);
impl<'a> /*trait*/ QImageWriter_setQuality for (i32) {
  fn setQuality(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter10setQualityEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QImageWriter10setQualityEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn gamma<T: QImageWriter_gamma>(&mut self, value: T) -> i32 {
    value.gamma(self);
    return 1;
  }
}

pub trait QImageWriter_gamma {
  fn gamma(self, this: &mut QImageWriter) -> i32;
}

// proto: float QImageWriter::gamma();
impl<'a> /*trait*/ QImageWriter_gamma for () {
  fn gamma(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter5gammaEv()};
    unsafe {_ZNK12QImageWriter5gammaEv()};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn description<T: QImageWriter_description>(&mut self, value: T) -> i32 {
    value.description(self);
    return 1;
  }
}

pub trait QImageWriter_description {
  fn description(self, this: &mut QImageWriter) -> i32;
}

// proto: QString QImageWriter::description();
impl<'a> /*trait*/ QImageWriter_description for () {
  fn description(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter11descriptionEv()};
    unsafe {_ZNK12QImageWriter11descriptionEv()};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn NewQImageWriter<T: QImageWriter_NewQImageWriter>(value: T) -> QImageWriter {
    let rsthis = value.NewQImageWriter();
    return rsthis;
    // return 1;
  }
}

pub trait QImageWriter_NewQImageWriter {
  fn NewQImageWriter(self) -> QImageWriter;
}

// proto: void QImageWriter::NewQImageWriter();
impl<'a> /*trait*/ QImageWriter_NewQImageWriter for () {
  fn NewQImageWriter(self) -> QImageWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriterC1Ev()};
    unsafe {_ZN12QImageWriterC1Ev(qthis)};
    let rsthis = QImageWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn setFormat<T: QImageWriter_setFormat>(&mut self, value: T) -> i32 {
    value.setFormat(self);
    return 1;
  }
}

pub trait QImageWriter_setFormat {
  fn setFormat(self, this: &mut QImageWriter) -> i32;
}

// proto: void QImageWriter::setFormat(const QByteArray & format);
impl<'a> /*trait*/ QImageWriter_setFormat for (&'a  QByteArray) {
  fn setFormat(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter9setFormatERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QImageWriter9setFormatERK10QByteArray(arg0)};
    return 1;
  }
}

// proto: void QImageWriter::NewQImageWriter(const QString & fileName, const QByteArray & format);
impl<'a> /*trait*/ QImageWriter_NewQImageWriter for (&'a  QString, &'a  QByteArray) {
  fn NewQImageWriter(self) -> QImageWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriterC1ERK7QStringRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN12QImageWriterC1ERK7QStringRK10QByteArray(qthis, arg0, arg1)};
    let rsthis = QImageWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn setDevice<T: QImageWriter_setDevice>(&mut self, value: T) -> i32 {
    value.setDevice(self);
    return 1;
  }
}

pub trait QImageWriter_setDevice {
  fn setDevice(self, this: &mut QImageWriter) -> i32;
}

// proto: void QImageWriter::setDevice(QIODevice * device);
impl<'a> /*trait*/ QImageWriter_setDevice for (&'a mut QIODevice) {
  fn setDevice(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QImageWriter9setDeviceEP9QIODevice(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn setSubType<T: QImageWriter_setSubType>(&mut self, value: T) -> i32 {
    value.setSubType(self);
    return 1;
  }
}

pub trait QImageWriter_setSubType {
  fn setSubType(self, this: &mut QImageWriter) -> i32;
}

// proto: void QImageWriter::setSubType(const QByteArray & type);
impl<'a> /*trait*/ QImageWriter_setSubType for (&'a  QByteArray) {
  fn setSubType(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter10setSubTypeERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QImageWriter10setSubTypeERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn progressiveScanWrite<T: QImageWriter_progressiveScanWrite>(&mut self, value: T) -> i32 {
    value.progressiveScanWrite(self);
    return 1;
  }
}

pub trait QImageWriter_progressiveScanWrite {
  fn progressiveScanWrite(self, this: &mut QImageWriter) -> i32;
}

// proto: bool QImageWriter::progressiveScanWrite();
impl<'a> /*trait*/ QImageWriter_progressiveScanWrite for () {
  fn progressiveScanWrite(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter20progressiveScanWriteEv()};
    unsafe {_ZNK12QImageWriter20progressiveScanWriteEv()};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn format<T: QImageWriter_format>(&mut self, value: T) -> i32 {
    value.format(self);
    return 1;
  }
}

pub trait QImageWriter_format {
  fn format(self, this: &mut QImageWriter) -> i32;
}

// proto: QByteArray QImageWriter::format();
impl<'a> /*trait*/ QImageWriter_format for () {
  fn format(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter6formatEv()};
    unsafe {_ZNK12QImageWriter6formatEv()};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn supportedSubTypes<T: QImageWriter_supportedSubTypes>(&mut self, value: T) -> i32 {
    value.supportedSubTypes(self);
    return 1;
  }
}

pub trait QImageWriter_supportedSubTypes {
  fn supportedSubTypes(self, this: &mut QImageWriter) -> i32;
}

// proto: QList<QByteArray> QImageWriter::supportedSubTypes();
impl<'a> /*trait*/ QImageWriter_supportedSubTypes for () {
  fn supportedSubTypes(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter17supportedSubTypesEv()};
    unsafe {_ZNK12QImageWriter17supportedSubTypesEv()};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn canWrite<T: QImageWriter_canWrite>(&mut self, value: T) -> i32 {
    value.canWrite(self);
    return 1;
  }
}

pub trait QImageWriter_canWrite {
  fn canWrite(self, this: &mut QImageWriter) -> i32;
}

// proto: bool QImageWriter::canWrite();
impl<'a> /*trait*/ QImageWriter_canWrite for () {
  fn canWrite(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter8canWriteEv()};
    unsafe {_ZNK12QImageWriter8canWriteEv()};
    return 1;
  }
}

// proto: void QImageWriter::NewQImageWriter(QIODevice * device, const QByteArray & format);
impl<'a> /*trait*/ QImageWriter_NewQImageWriter for (&'a mut QIODevice, &'a  QByteArray) {
  fn NewQImageWriter(self) -> QImageWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriterC1EP9QIODeviceRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN12QImageWriterC1EP9QIODeviceRK10QByteArray(qthis, arg0, arg1)};
    let rsthis = QImageWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn compression<T: QImageWriter_compression>(&mut self, value: T) -> i32 {
    value.compression(self);
    return 1;
  }
}

pub trait QImageWriter_compression {
  fn compression(self, this: &mut QImageWriter) -> i32;
}

// proto: int QImageWriter::compression();
impl<'a> /*trait*/ QImageWriter_compression for () {
  fn compression(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter11compressionEv()};
    unsafe {_ZNK12QImageWriter11compressionEv()};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn setProgressiveScanWrite<T: QImageWriter_setProgressiveScanWrite>(&mut self, value: T) -> i32 {
    value.setProgressiveScanWrite(self);
    return 1;
  }
}

pub trait QImageWriter_setProgressiveScanWrite {
  fn setProgressiveScanWrite(self, this: &mut QImageWriter) -> i32;
}

// proto: void QImageWriter::setProgressiveScanWrite(bool progressive);
impl<'a> /*trait*/ QImageWriter_setProgressiveScanWrite for (i8) {
  fn setProgressiveScanWrite(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter23setProgressiveScanWriteEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN12QImageWriter23setProgressiveScanWriteEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn setDescription<T: QImageWriter_setDescription>(&mut self, value: T) -> i32 {
    value.setDescription(self);
    return 1;
  }
}

pub trait QImageWriter_setDescription {
  fn setDescription(self, this: &mut QImageWriter) -> i32;
}

// proto: void QImageWriter::setDescription(const QString & description);
impl<'a> /*trait*/ QImageWriter_setDescription for (&'a  QString) {
  fn setDescription(self, this: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter14setDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QImageWriter14setDescriptionERK7QString(arg0)};
    return 1;
  }
}

// proto: void QImageWriter::NewQImageWriter(const QImageWriter & );
impl<'a> /*trait*/ QImageWriter_NewQImageWriter for (&'a  QImageWriter) {
  fn NewQImageWriter(self) -> QImageWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriterC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QImageWriterC1ERKS_(qthis, arg0)};
    let rsthis = QImageWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

