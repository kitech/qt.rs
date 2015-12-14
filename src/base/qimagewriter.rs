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
use super::qimage::QImage;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QImageWriter::setText(const QString & key, const QString & text);
  fn _ZN12QImageWriter7setTextERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QImageWriter::setGamma(float gamma);
  fn _ZN12QImageWriter8setGammaEf(qthis: *mut c_void, arg0: c_float) ;
  // proto:  void QImageWriter::setFileName(const QString & fileName);
  fn _ZN12QImageWriter11setFileNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QImageWriter::optimizedWrite();
  fn _ZNK12QImageWriter14optimizedWriteEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QImageWriter::FreeQImageWriter();
  fn _ZN12QImageWriterD0Ev(qthis: *mut c_void) ;
  // proto:  QIODevice * QImageWriter::device();
  fn _ZNK12QImageWriter6deviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QByteArray QImageWriter::subType();
  fn _ZNK12QImageWriter7subTypeEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QList<QByteArray> QImageWriter::supportedMimeTypes();
  fn _ZN12QImageWriter18supportedMimeTypesEv() ;
  // proto:  int QImageWriter::quality();
  fn _ZNK12QImageWriter7qualityEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QImageWriter::write(const QImage & image);
  fn _ZN12QImageWriter5writeERK6QImage(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QImageWriter::setCompression(int compression);
  fn _ZN12QImageWriter14setCompressionEi(qthis: *mut c_void, arg0: c_int) ;
  // proto: static QList<QByteArray> QImageWriter::supportedImageFormats();
  fn _ZN12QImageWriter21supportedImageFormatsEv() ;
  // proto:  QString QImageWriter::fileName();
  fn _ZNK12QImageWriter8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QImageWriter::setOptimizedWrite(bool optimize);
  fn _ZN12QImageWriter17setOptimizedWriteEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QString QImageWriter::errorString();
  fn _ZNK12QImageWriter11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QImageWriter::setQuality(int quality);
  fn _ZN12QImageWriter10setQualityEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  float QImageWriter::gamma();
  fn _ZNK12QImageWriter5gammaEv(qthis: *mut c_void) -> c_float;
  // proto:  QString QImageWriter::description();
  fn _ZNK12QImageWriter11descriptionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QImageWriter::NewQImageWriter();
  fn _ZN12QImageWriterC1Ev(qthis: *mut c_void) ;
  // proto:  void QImageWriter::setFormat(const QByteArray & format);
  fn _ZN12QImageWriter9setFormatERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QImageWriter::NewQImageWriter(const QString & fileName, const QByteArray & format);
  fn _ZN12QImageWriterC1ERK7QStringRK10QByteArray(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QImageWriter::setDevice(QIODevice * device);
  fn _ZN12QImageWriter9setDeviceEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QImageWriter::setSubType(const QByteArray & type);
  fn _ZN12QImageWriter10setSubTypeERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QImageWriter::progressiveScanWrite();
  fn _ZNK12QImageWriter20progressiveScanWriteEv(qthis: *mut c_void) -> int8_t;
  // proto:  QByteArray QImageWriter::format();
  fn _ZNK12QImageWriter6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QList<QByteArray> QImageWriter::supportedSubTypes();
  fn _ZNK12QImageWriter17supportedSubTypesEv(qthis: *mut c_void) ;
  // proto:  bool QImageWriter::canWrite();
  fn _ZNK12QImageWriter8canWriteEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QImageWriter::NewQImageWriter(QIODevice * device, const QByteArray & format);
  fn _ZN12QImageWriterC1EP9QIODeviceRK10QByteArray(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  int QImageWriter::compression();
  fn _ZNK12QImageWriter11compressionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QImageWriter::setProgressiveScanWrite(bool progressive);
  fn _ZN12QImageWriter23setProgressiveScanWriteEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QImageWriter::setDescription(const QString & description);
  fn _ZN12QImageWriter14setDescriptionERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QImageWriter::NewQImageWriter(const QImageWriter & );
  fn _ZN12QImageWriterC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QImageWriter)=8
pub struct QImageWriter {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QImageWriter {
  pub fn setText<T: QImageWriter_setText>(&mut self, value: T)  {
     value.setText(self);
    // return 1;
  }
}

pub trait QImageWriter_setText {
  fn setText(self, rsthis: &mut QImageWriter) ;
}

// proto:  void QImageWriter::setText(const QString & key, const QString & text);
impl<'a> /*trait*/ QImageWriter_setText for (&'a  QString, &'a  QString) {
  fn setText(self, rsthis: &mut QImageWriter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter7setTextERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageWriter7setTextERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn setGamma<T: QImageWriter_setGamma>(&mut self, value: T)  {
     value.setGamma(self);
    // return 1;
  }
}

pub trait QImageWriter_setGamma {
  fn setGamma(self, rsthis: &mut QImageWriter) ;
}

// proto:  void QImageWriter::setGamma(float gamma);
impl<'a> /*trait*/ QImageWriter_setGamma for (f32) {
  fn setGamma(self, rsthis: &mut QImageWriter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter8setGammaEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN12QImageWriter8setGammaEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn setFileName<T: QImageWriter_setFileName>(&mut self, value: T)  {
     value.setFileName(self);
    // return 1;
  }
}

pub trait QImageWriter_setFileName {
  fn setFileName(self, rsthis: &mut QImageWriter) ;
}

// proto:  void QImageWriter::setFileName(const QString & fileName);
impl<'a> /*trait*/ QImageWriter_setFileName for (&'a  QString) {
  fn setFileName(self, rsthis: &mut QImageWriter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageWriter11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn optimizedWrite<T: QImageWriter_optimizedWrite>(&mut self, value: T) -> i8 {
    return value.optimizedWrite(self);
    // return 1;
  }
}

pub trait QImageWriter_optimizedWrite {
  fn optimizedWrite(self, rsthis: &mut QImageWriter) -> i8;
}

// proto:  bool QImageWriter::optimizedWrite();
impl<'a> /*trait*/ QImageWriter_optimizedWrite for () {
  fn optimizedWrite(self, rsthis: &mut QImageWriter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter14optimizedWriteEv()};
    let mut ret = unsafe {_ZNK12QImageWriter14optimizedWriteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn FreeQImageWriter<T: QImageWriter_FreeQImageWriter>(&mut self, value: T)  {
     value.FreeQImageWriter(self);
    // return 1;
  }
}

pub trait QImageWriter_FreeQImageWriter {
  fn FreeQImageWriter(self, rsthis: &mut QImageWriter) ;
}

// proto:  void QImageWriter::FreeQImageWriter();
impl<'a> /*trait*/ QImageWriter_FreeQImageWriter for () {
  fn FreeQImageWriter(self, rsthis: &mut QImageWriter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriterD0Ev()};
     unsafe {_ZN12QImageWriterD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn device<T: QImageWriter_device>(&mut self, value: T) -> QIODevice {
    return value.device(self);
    // return 1;
  }
}

pub trait QImageWriter_device {
  fn device(self, rsthis: &mut QImageWriter) -> QIODevice;
}

// proto:  QIODevice * QImageWriter::device();
impl<'a> /*trait*/ QImageWriter_device for () {
  fn device(self, rsthis: &mut QImageWriter) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter6deviceEv()};
    let mut ret = unsafe {_ZNK12QImageWriter6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn subType<T: QImageWriter_subType>(&mut self, value: T) -> QByteArray {
    return value.subType(self);
    // return 1;
  }
}

pub trait QImageWriter_subType {
  fn subType(self, rsthis: &mut QImageWriter) -> QByteArray;
}

// proto:  QByteArray QImageWriter::subType();
impl<'a> /*trait*/ QImageWriter_subType for () {
  fn subType(self, rsthis: &mut QImageWriter) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter7subTypeEv()};
    let mut ret = unsafe {_ZNK12QImageWriter7subTypeEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn supportedMimeTypes<T: QImageWriter_supportedMimeTypes>(&mut self, value: T)  {
     value.supportedMimeTypes(self);
    // return 1;
  }
}

pub trait QImageWriter_supportedMimeTypes {
  fn supportedMimeTypes(self, rsthis: &mut QImageWriter) ;
}

// proto: static QList<QByteArray> QImageWriter::supportedMimeTypes();
impl<'a> /*trait*/ QImageWriter_supportedMimeTypes for () {
  fn supportedMimeTypes(self, rsthis: &mut QImageWriter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter18supportedMimeTypesEv()};
     unsafe {_ZN12QImageWriter18supportedMimeTypesEv()};
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn quality<T: QImageWriter_quality>(&mut self, value: T) -> i32 {
    return value.quality(self);
    // return 1;
  }
}

pub trait QImageWriter_quality {
  fn quality(self, rsthis: &mut QImageWriter) -> i32;
}

// proto:  int QImageWriter::quality();
impl<'a> /*trait*/ QImageWriter_quality for () {
  fn quality(self, rsthis: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter7qualityEv()};
    let mut ret = unsafe {_ZNK12QImageWriter7qualityEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn write<T: QImageWriter_write>(&mut self, value: T) -> i8 {
    return value.write(self);
    // return 1;
  }
}

pub trait QImageWriter_write {
  fn write(self, rsthis: &mut QImageWriter) -> i8;
}

// proto:  bool QImageWriter::write(const QImage & image);
impl<'a> /*trait*/ QImageWriter_write for (&'a  QImage) {
  fn write(self, rsthis: &mut QImageWriter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter5writeERK6QImage()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QImageWriter5writeERK6QImage(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn setCompression<T: QImageWriter_setCompression>(&mut self, value: T)  {
     value.setCompression(self);
    // return 1;
  }
}

pub trait QImageWriter_setCompression {
  fn setCompression(self, rsthis: &mut QImageWriter) ;
}

// proto:  void QImageWriter::setCompression(int compression);
impl<'a> /*trait*/ QImageWriter_setCompression for (i32) {
  fn setCompression(self, rsthis: &mut QImageWriter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter14setCompressionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QImageWriter14setCompressionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn supportedImageFormats<T: QImageWriter_supportedImageFormats>(&mut self, value: T)  {
     value.supportedImageFormats(self);
    // return 1;
  }
}

pub trait QImageWriter_supportedImageFormats {
  fn supportedImageFormats(self, rsthis: &mut QImageWriter) ;
}

// proto: static QList<QByteArray> QImageWriter::supportedImageFormats();
impl<'a> /*trait*/ QImageWriter_supportedImageFormats for () {
  fn supportedImageFormats(self, rsthis: &mut QImageWriter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter21supportedImageFormatsEv()};
     unsafe {_ZN12QImageWriter21supportedImageFormatsEv()};
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn fileName<T: QImageWriter_fileName>(&mut self, value: T) -> QString {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QImageWriter_fileName {
  fn fileName(self, rsthis: &mut QImageWriter) -> QString;
}

// proto:  QString QImageWriter::fileName();
impl<'a> /*trait*/ QImageWriter_fileName for () {
  fn fileName(self, rsthis: &mut QImageWriter) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter8fileNameEv()};
    let mut ret = unsafe {_ZNK12QImageWriter8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn setOptimizedWrite<T: QImageWriter_setOptimizedWrite>(&mut self, value: T)  {
     value.setOptimizedWrite(self);
    // return 1;
  }
}

pub trait QImageWriter_setOptimizedWrite {
  fn setOptimizedWrite(self, rsthis: &mut QImageWriter) ;
}

// proto:  void QImageWriter::setOptimizedWrite(bool optimize);
impl<'a> /*trait*/ QImageWriter_setOptimizedWrite for (i8) {
  fn setOptimizedWrite(self, rsthis: &mut QImageWriter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter17setOptimizedWriteEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QImageWriter17setOptimizedWriteEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn errorString<T: QImageWriter_errorString>(&mut self, value: T) -> QString {
    return value.errorString(self);
    // return 1;
  }
}

pub trait QImageWriter_errorString {
  fn errorString(self, rsthis: &mut QImageWriter) -> QString;
}

// proto:  QString QImageWriter::errorString();
impl<'a> /*trait*/ QImageWriter_errorString for () {
  fn errorString(self, rsthis: &mut QImageWriter) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter11errorStringEv()};
    let mut ret = unsafe {_ZNK12QImageWriter11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn setQuality<T: QImageWriter_setQuality>(&mut self, value: T)  {
     value.setQuality(self);
    // return 1;
  }
}

pub trait QImageWriter_setQuality {
  fn setQuality(self, rsthis: &mut QImageWriter) ;
}

// proto:  void QImageWriter::setQuality(int quality);
impl<'a> /*trait*/ QImageWriter_setQuality for (i32) {
  fn setQuality(self, rsthis: &mut QImageWriter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter10setQualityEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QImageWriter10setQualityEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn gamma<T: QImageWriter_gamma>(&mut self, value: T) -> f32 {
    return value.gamma(self);
    // return 1;
  }
}

pub trait QImageWriter_gamma {
  fn gamma(self, rsthis: &mut QImageWriter) -> f32;
}

// proto:  float QImageWriter::gamma();
impl<'a> /*trait*/ QImageWriter_gamma for () {
  fn gamma(self, rsthis: &mut QImageWriter) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter5gammaEv()};
    let mut ret = unsafe {_ZNK12QImageWriter5gammaEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn description<T: QImageWriter_description>(&mut self, value: T) -> QString {
    return value.description(self);
    // return 1;
  }
}

pub trait QImageWriter_description {
  fn description(self, rsthis: &mut QImageWriter) -> QString;
}

// proto:  QString QImageWriter::description();
impl<'a> /*trait*/ QImageWriter_description for () {
  fn description(self, rsthis: &mut QImageWriter) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter11descriptionEv()};
    let mut ret = unsafe {_ZNK12QImageWriter11descriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn setFormat<T: QImageWriter_setFormat>(&mut self, value: T)  {
     value.setFormat(self);
    // return 1;
  }
}

pub trait QImageWriter_setFormat {
  fn setFormat(self, rsthis: &mut QImageWriter) ;
}

// proto:  void QImageWriter::setFormat(const QByteArray & format);
impl<'a> /*trait*/ QImageWriter_setFormat for (&'a  QByteArray) {
  fn setFormat(self, rsthis: &mut QImageWriter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter9setFormatERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageWriter9setFormatERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QImageWriter::NewQImageWriter(const QString & fileName, const QByteArray & format);
impl<'a> /*trait*/ QImageWriter_NewQImageWriter for (&'a  QString, &'a  QByteArray) {
  fn NewQImageWriter(self) -> QImageWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriterC1ERK7QStringRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QImageWriterC1ERK7QStringRK10QByteArray(qthis, arg0, arg1)};
    let rsthis = QImageWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn setDevice<T: QImageWriter_setDevice>(&mut self, value: T)  {
     value.setDevice(self);
    // return 1;
  }
}

pub trait QImageWriter_setDevice {
  fn setDevice(self, rsthis: &mut QImageWriter) ;
}

// proto:  void QImageWriter::setDevice(QIODevice * device);
impl<'a> /*trait*/ QImageWriter_setDevice for (&'a mut QIODevice) {
  fn setDevice(self, rsthis: &mut QImageWriter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageWriter9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn setSubType<T: QImageWriter_setSubType>(&mut self, value: T)  {
     value.setSubType(self);
    // return 1;
  }
}

pub trait QImageWriter_setSubType {
  fn setSubType(self, rsthis: &mut QImageWriter) ;
}

// proto:  void QImageWriter::setSubType(const QByteArray & type);
impl<'a> /*trait*/ QImageWriter_setSubType for (&'a  QByteArray) {
  fn setSubType(self, rsthis: &mut QImageWriter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter10setSubTypeERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageWriter10setSubTypeERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn progressiveScanWrite<T: QImageWriter_progressiveScanWrite>(&mut self, value: T) -> i8 {
    return value.progressiveScanWrite(self);
    // return 1;
  }
}

pub trait QImageWriter_progressiveScanWrite {
  fn progressiveScanWrite(self, rsthis: &mut QImageWriter) -> i8;
}

// proto:  bool QImageWriter::progressiveScanWrite();
impl<'a> /*trait*/ QImageWriter_progressiveScanWrite for () {
  fn progressiveScanWrite(self, rsthis: &mut QImageWriter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter20progressiveScanWriteEv()};
    let mut ret = unsafe {_ZNK12QImageWriter20progressiveScanWriteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn format<T: QImageWriter_format>(&mut self, value: T) -> QByteArray {
    return value.format(self);
    // return 1;
  }
}

pub trait QImageWriter_format {
  fn format(self, rsthis: &mut QImageWriter) -> QByteArray;
}

// proto:  QByteArray QImageWriter::format();
impl<'a> /*trait*/ QImageWriter_format for () {
  fn format(self, rsthis: &mut QImageWriter) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter6formatEv()};
    let mut ret = unsafe {_ZNK12QImageWriter6formatEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn supportedSubTypes<T: QImageWriter_supportedSubTypes>(&mut self, value: T)  {
     value.supportedSubTypes(self);
    // return 1;
  }
}

pub trait QImageWriter_supportedSubTypes {
  fn supportedSubTypes(self, rsthis: &mut QImageWriter) ;
}

// proto:  QList<QByteArray> QImageWriter::supportedSubTypes();
impl<'a> /*trait*/ QImageWriter_supportedSubTypes for () {
  fn supportedSubTypes(self, rsthis: &mut QImageWriter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter17supportedSubTypesEv()};
     unsafe {_ZNK12QImageWriter17supportedSubTypesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn canWrite<T: QImageWriter_canWrite>(&mut self, value: T) -> i8 {
    return value.canWrite(self);
    // return 1;
  }
}

pub trait QImageWriter_canWrite {
  fn canWrite(self, rsthis: &mut QImageWriter) -> i8;
}

// proto:  bool QImageWriter::canWrite();
impl<'a> /*trait*/ QImageWriter_canWrite for () {
  fn canWrite(self, rsthis: &mut QImageWriter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter8canWriteEv()};
    let mut ret = unsafe {_ZNK12QImageWriter8canWriteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QImageWriter::NewQImageWriter(QIODevice * device, const QByteArray & format);
impl<'a> /*trait*/ QImageWriter_NewQImageWriter for (&'a mut QIODevice, &'a  QByteArray) {
  fn NewQImageWriter(self) -> QImageWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriterC1EP9QIODeviceRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QImageWriterC1EP9QIODeviceRK10QByteArray(qthis, arg0, arg1)};
    let rsthis = QImageWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn compression<T: QImageWriter_compression>(&mut self, value: T) -> i32 {
    return value.compression(self);
    // return 1;
  }
}

pub trait QImageWriter_compression {
  fn compression(self, rsthis: &mut QImageWriter) -> i32;
}

// proto:  int QImageWriter::compression();
impl<'a> /*trait*/ QImageWriter_compression for () {
  fn compression(self, rsthis: &mut QImageWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageWriter11compressionEv()};
    let mut ret = unsafe {_ZNK12QImageWriter11compressionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn setProgressiveScanWrite<T: QImageWriter_setProgressiveScanWrite>(&mut self, value: T)  {
     value.setProgressiveScanWrite(self);
    // return 1;
  }
}

pub trait QImageWriter_setProgressiveScanWrite {
  fn setProgressiveScanWrite(self, rsthis: &mut QImageWriter) ;
}

// proto:  void QImageWriter::setProgressiveScanWrite(bool progressive);
impl<'a> /*trait*/ QImageWriter_setProgressiveScanWrite for (i8) {
  fn setProgressiveScanWrite(self, rsthis: &mut QImageWriter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter23setProgressiveScanWriteEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QImageWriter23setProgressiveScanWriteEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageWriter {
  pub fn setDescription<T: QImageWriter_setDescription>(&mut self, value: T)  {
     value.setDescription(self);
    // return 1;
  }
}

pub trait QImageWriter_setDescription {
  fn setDescription(self, rsthis: &mut QImageWriter) ;
}

// proto:  void QImageWriter::setDescription(const QString & description);
impl<'a> /*trait*/ QImageWriter_setDescription for (&'a  QString) {
  fn setDescription(self, rsthis: &mut QImageWriter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriter14setDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageWriter14setDescriptionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QImageWriter::NewQImageWriter(const QImageWriter & );
impl<'a> /*trait*/ QImageWriter_NewQImageWriter for (&'a  QImageWriter) {
  fn NewQImageWriter(self) -> QImageWriter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageWriterC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QImageWriterC1ERKS_(qthis, arg0)};
    let rsthis = QImageWriter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

