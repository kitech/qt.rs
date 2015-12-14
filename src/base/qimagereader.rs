// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qsize::QSize;
use super::qrect::QRect;
use super::qstringlist::QStringList;
use super::qiodevice::QIODevice;
use super::qbytearray::QByteArray;
use super::qcolor::QColor;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QString QImageReader::errorString();
  fn _ZNK12QImageReader11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QImageReader::canRead();
  fn _ZNK12QImageReader7canReadEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QImageReader::FreeQImageReader();
  fn _ZN12QImageReaderD0Ev(qthis: *mut c_void) ;
  // proto:  void QImageReader::setScaledSize(const QSize & size);
  fn _ZN12QImageReader13setScaledSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QImageReader::setScaledClipRect(const QRect & rect);
  fn _ZN12QImageReader17setScaledClipRectERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QImageReader::imageCount();
  fn _ZNK12QImageReader10imageCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QStringList QImageReader::textKeys();
  fn _ZNK12QImageReader8textKeysEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QImageReader::decideFormatFromContent();
  fn _ZNK12QImageReader23decideFormatFromContentEv(qthis: *mut c_void) -> int8_t;
  // proto:  QIODevice * QImageReader::device();
  fn _ZNK12QImageReader6deviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QImageReader::autoTransform();
  fn _ZNK12QImageReader13autoTransformEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QImageReader::jumpToNextImage();
  fn _ZN12QImageReader15jumpToNextImageEv(qthis: *mut c_void) -> int8_t;
  // proto: static QByteArray QImageReader::imageFormat(const QString & fileName);
  fn _ZN12QImageReader11imageFormatERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  QList<QByteArray> QImageReader::supportedSubTypes();
  fn _ZNK12QImageReader17supportedSubTypesEv(qthis: *mut c_void) ;
  // proto:  QSize QImageReader::size();
  fn _ZNK12QImageReader4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QColor QImageReader::backgroundColor();
  fn _ZNK12QImageReader15backgroundColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QByteArray QImageReader::subType();
  fn _ZNK12QImageReader7subTypeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QImageReader::currentImageNumber();
  fn _ZNK12QImageReader18currentImageNumberEv(qthis: *mut c_void) -> c_int;
  // proto: static QList<QByteArray> QImageReader::supportedImageFormats();
  fn _ZN12QImageReader21supportedImageFormatsEv() ;
  // proto:  int QImageReader::loopCount();
  fn _ZNK12QImageReader9loopCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QImageReader::setDecideFormatFromContent(bool ignored);
  fn _ZN12QImageReader26setDecideFormatFromContentEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QRect QImageReader::scaledClipRect();
  fn _ZNK12QImageReader14scaledClipRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QList<QByteArray> QImageReader::supportedMimeTypes();
  fn _ZN12QImageReader18supportedMimeTypesEv() ;
  // proto:  QString QImageReader::text(const QString & key);
  fn _ZNK12QImageReader4textERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QImageReader::nextImageDelay();
  fn _ZNK12QImageReader14nextImageDelayEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QImageReader::supportsAnimation();
  fn _ZNK12QImageReader17supportsAnimationEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QImageReader::jumpToImage(int imageNumber);
  fn _ZN12QImageReader11jumpToImageEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QImageReader::setFileName(const QString & fileName);
  fn _ZN12QImageReader11setFileNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QImageReader::NewQImageReader(const QImageReader & );
  fn _ZN12QImageReaderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSize QImageReader::scaledSize();
  fn _ZNK12QImageReader10scaledSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QImageReader::setAutoTransform(bool enabled);
  fn _ZN12QImageReader16setAutoTransformEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QImageReader::setClipRect(const QRect & rect);
  fn _ZN12QImageReader11setClipRectERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QImageReader::autoDetectImageFormat();
  fn _ZNK12QImageReader21autoDetectImageFormatEv(qthis: *mut c_void) -> int8_t;
  // proto:  QRect QImageReader::currentImageRect();
  fn _ZNK12QImageReader16currentImageRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QImageReader::NewQImageReader(const QString & fileName, const QByteArray & format);
  fn _ZN12QImageReaderC1ERK7QStringRK10QByteArray(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto: static QByteArray QImageReader::imageFormat(QIODevice * device);
  fn _ZN12QImageReader11imageFormatEP9QIODevice(arg0: *mut c_void) -> *mut c_void;
  // proto:  int QImageReader::quality();
  fn _ZNK12QImageReader7qualityEv(qthis: *mut c_void) -> c_int;
  // proto:  void QImageReader::setDevice(QIODevice * device);
  fn _ZN12QImageReader9setDeviceEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QImageReader::setBackgroundColor(const QColor & color);
  fn _ZN12QImageReader18setBackgroundColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QImageReader::setQuality(int quality);
  fn _ZN12QImageReader10setQualityEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QImageReader::NewQImageReader(QIODevice * device, const QByteArray & format);
  fn _ZN12QImageReaderC1EP9QIODeviceRK10QByteArray(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QImageReader::setAutoDetectImageFormat(bool enabled);
  fn _ZN12QImageReader24setAutoDetectImageFormatEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QImageReader::NewQImageReader();
  fn _ZN12QImageReaderC1Ev(qthis: *mut c_void) ;
  // proto:  void QImageReader::setFormat(const QByteArray & format);
  fn _ZN12QImageReader9setFormatERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QImageReader::fileName();
  fn _ZNK12QImageReader8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRect QImageReader::clipRect();
  fn _ZNK12QImageReader8clipRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QByteArray QImageReader::format();
  fn _ZNK12QImageReader6formatEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QImageReader)=8
pub struct QImageReader {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QImageReader {
  pub fn errorString<T: QImageReader_errorString>(&mut self, value: T) -> QString {
    return value.errorString(self);
    // return 1;
  }
}

pub trait QImageReader_errorString {
  fn errorString(self, rsthis: &mut QImageReader) -> QString;
}

// proto:  QString QImageReader::errorString();
impl<'a> /*trait*/ QImageReader_errorString for () {
  fn errorString(self, rsthis: &mut QImageReader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader11errorStringEv()};
    let mut ret = unsafe {_ZNK12QImageReader11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn canRead<T: QImageReader_canRead>(&mut self, value: T) -> i8 {
    return value.canRead(self);
    // return 1;
  }
}

pub trait QImageReader_canRead {
  fn canRead(self, rsthis: &mut QImageReader) -> i8;
}

// proto:  bool QImageReader::canRead();
impl<'a> /*trait*/ QImageReader_canRead for () {
  fn canRead(self, rsthis: &mut QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader7canReadEv()};
    let mut ret = unsafe {_ZNK12QImageReader7canReadEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn FreeQImageReader<T: QImageReader_FreeQImageReader>(&mut self, value: T)  {
     value.FreeQImageReader(self);
    // return 1;
  }
}

pub trait QImageReader_FreeQImageReader {
  fn FreeQImageReader(self, rsthis: &mut QImageReader) ;
}

// proto:  void QImageReader::FreeQImageReader();
impl<'a> /*trait*/ QImageReader_FreeQImageReader for () {
  fn FreeQImageReader(self, rsthis: &mut QImageReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReaderD0Ev()};
     unsafe {_ZN12QImageReaderD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setScaledSize<T: QImageReader_setScaledSize>(&mut self, value: T)  {
     value.setScaledSize(self);
    // return 1;
  }
}

pub trait QImageReader_setScaledSize {
  fn setScaledSize(self, rsthis: &mut QImageReader) ;
}

// proto:  void QImageReader::setScaledSize(const QSize & size);
impl<'a> /*trait*/ QImageReader_setScaledSize for (&'a  QSize) {
  fn setScaledSize(self, rsthis: &mut QImageReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader13setScaledSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageReader13setScaledSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setScaledClipRect<T: QImageReader_setScaledClipRect>(&mut self, value: T)  {
     value.setScaledClipRect(self);
    // return 1;
  }
}

pub trait QImageReader_setScaledClipRect {
  fn setScaledClipRect(self, rsthis: &mut QImageReader) ;
}

// proto:  void QImageReader::setScaledClipRect(const QRect & rect);
impl<'a> /*trait*/ QImageReader_setScaledClipRect for (&'a  QRect) {
  fn setScaledClipRect(self, rsthis: &mut QImageReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader17setScaledClipRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageReader17setScaledClipRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn imageCount<T: QImageReader_imageCount>(&mut self, value: T) -> i32 {
    return value.imageCount(self);
    // return 1;
  }
}

pub trait QImageReader_imageCount {
  fn imageCount(self, rsthis: &mut QImageReader) -> i32;
}

// proto:  int QImageReader::imageCount();
impl<'a> /*trait*/ QImageReader_imageCount for () {
  fn imageCount(self, rsthis: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader10imageCountEv()};
    let mut ret = unsafe {_ZNK12QImageReader10imageCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn textKeys<T: QImageReader_textKeys>(&mut self, value: T) -> QStringList {
    return value.textKeys(self);
    // return 1;
  }
}

pub trait QImageReader_textKeys {
  fn textKeys(self, rsthis: &mut QImageReader) -> QStringList;
}

// proto:  QStringList QImageReader::textKeys();
impl<'a> /*trait*/ QImageReader_textKeys for () {
  fn textKeys(self, rsthis: &mut QImageReader) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader8textKeysEv()};
    let mut ret = unsafe {_ZNK12QImageReader8textKeysEv(rsthis.qclsinst)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn decideFormatFromContent<T: QImageReader_decideFormatFromContent>(&mut self, value: T) -> i8 {
    return value.decideFormatFromContent(self);
    // return 1;
  }
}

pub trait QImageReader_decideFormatFromContent {
  fn decideFormatFromContent(self, rsthis: &mut QImageReader) -> i8;
}

// proto:  bool QImageReader::decideFormatFromContent();
impl<'a> /*trait*/ QImageReader_decideFormatFromContent for () {
  fn decideFormatFromContent(self, rsthis: &mut QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader23decideFormatFromContentEv()};
    let mut ret = unsafe {_ZNK12QImageReader23decideFormatFromContentEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn device<T: QImageReader_device>(&mut self, value: T) -> QIODevice {
    return value.device(self);
    // return 1;
  }
}

pub trait QImageReader_device {
  fn device(self, rsthis: &mut QImageReader) -> QIODevice;
}

// proto:  QIODevice * QImageReader::device();
impl<'a> /*trait*/ QImageReader_device for () {
  fn device(self, rsthis: &mut QImageReader) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader6deviceEv()};
    let mut ret = unsafe {_ZNK12QImageReader6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn autoTransform<T: QImageReader_autoTransform>(&mut self, value: T) -> i8 {
    return value.autoTransform(self);
    // return 1;
  }
}

pub trait QImageReader_autoTransform {
  fn autoTransform(self, rsthis: &mut QImageReader) -> i8;
}

// proto:  bool QImageReader::autoTransform();
impl<'a> /*trait*/ QImageReader_autoTransform for () {
  fn autoTransform(self, rsthis: &mut QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader13autoTransformEv()};
    let mut ret = unsafe {_ZNK12QImageReader13autoTransformEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn jumpToNextImage<T: QImageReader_jumpToNextImage>(&mut self, value: T) -> i8 {
    return value.jumpToNextImage(self);
    // return 1;
  }
}

pub trait QImageReader_jumpToNextImage {
  fn jumpToNextImage(self, rsthis: &mut QImageReader) -> i8;
}

// proto:  bool QImageReader::jumpToNextImage();
impl<'a> /*trait*/ QImageReader_jumpToNextImage for () {
  fn jumpToNextImage(self, rsthis: &mut QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader15jumpToNextImageEv()};
    let mut ret = unsafe {_ZN12QImageReader15jumpToNextImageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn imageFormat<T: QImageReader_imageFormat>(&mut self, value: T) -> QByteArray {
    return value.imageFormat(self);
    // return 1;
  }
}

pub trait QImageReader_imageFormat {
  fn imageFormat(self, rsthis: &mut QImageReader) -> QByteArray;
}

// proto: static QByteArray QImageReader::imageFormat(const QString & fileName);
impl<'a> /*trait*/ QImageReader_imageFormat for (&'a  QString) {
  fn imageFormat(self, rsthis: &mut QImageReader) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11imageFormatERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QImageReader11imageFormatERK7QString(arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn supportedSubTypes<T: QImageReader_supportedSubTypes>(&mut self, value: T)  {
     value.supportedSubTypes(self);
    // return 1;
  }
}

pub trait QImageReader_supportedSubTypes {
  fn supportedSubTypes(self, rsthis: &mut QImageReader) ;
}

// proto:  QList<QByteArray> QImageReader::supportedSubTypes();
impl<'a> /*trait*/ QImageReader_supportedSubTypes for () {
  fn supportedSubTypes(self, rsthis: &mut QImageReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader17supportedSubTypesEv()};
     unsafe {_ZNK12QImageReader17supportedSubTypesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn size<T: QImageReader_size>(&mut self, value: T) -> QSize {
    return value.size(self);
    // return 1;
  }
}

pub trait QImageReader_size {
  fn size(self, rsthis: &mut QImageReader) -> QSize;
}

// proto:  QSize QImageReader::size();
impl<'a> /*trait*/ QImageReader_size for () {
  fn size(self, rsthis: &mut QImageReader) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader4sizeEv()};
    let mut ret = unsafe {_ZNK12QImageReader4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn backgroundColor<T: QImageReader_backgroundColor>(&mut self, value: T) -> QColor {
    return value.backgroundColor(self);
    // return 1;
  }
}

pub trait QImageReader_backgroundColor {
  fn backgroundColor(self, rsthis: &mut QImageReader) -> QColor;
}

// proto:  QColor QImageReader::backgroundColor();
impl<'a> /*trait*/ QImageReader_backgroundColor for () {
  fn backgroundColor(self, rsthis: &mut QImageReader) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader15backgroundColorEv()};
    let mut ret = unsafe {_ZNK12QImageReader15backgroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn subType<T: QImageReader_subType>(&mut self, value: T) -> QByteArray {
    return value.subType(self);
    // return 1;
  }
}

pub trait QImageReader_subType {
  fn subType(self, rsthis: &mut QImageReader) -> QByteArray;
}

// proto:  QByteArray QImageReader::subType();
impl<'a> /*trait*/ QImageReader_subType for () {
  fn subType(self, rsthis: &mut QImageReader) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader7subTypeEv()};
    let mut ret = unsafe {_ZNK12QImageReader7subTypeEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn currentImageNumber<T: QImageReader_currentImageNumber>(&mut self, value: T) -> i32 {
    return value.currentImageNumber(self);
    // return 1;
  }
}

pub trait QImageReader_currentImageNumber {
  fn currentImageNumber(self, rsthis: &mut QImageReader) -> i32;
}

// proto:  int QImageReader::currentImageNumber();
impl<'a> /*trait*/ QImageReader_currentImageNumber for () {
  fn currentImageNumber(self, rsthis: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader18currentImageNumberEv()};
    let mut ret = unsafe {_ZNK12QImageReader18currentImageNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn supportedImageFormats<T: QImageReader_supportedImageFormats>(&mut self, value: T)  {
     value.supportedImageFormats(self);
    // return 1;
  }
}

pub trait QImageReader_supportedImageFormats {
  fn supportedImageFormats(self, rsthis: &mut QImageReader) ;
}

// proto: static QList<QByteArray> QImageReader::supportedImageFormats();
impl<'a> /*trait*/ QImageReader_supportedImageFormats for () {
  fn supportedImageFormats(self, rsthis: &mut QImageReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader21supportedImageFormatsEv()};
     unsafe {_ZN12QImageReader21supportedImageFormatsEv()};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn loopCount<T: QImageReader_loopCount>(&mut self, value: T) -> i32 {
    return value.loopCount(self);
    // return 1;
  }
}

pub trait QImageReader_loopCount {
  fn loopCount(self, rsthis: &mut QImageReader) -> i32;
}

// proto:  int QImageReader::loopCount();
impl<'a> /*trait*/ QImageReader_loopCount for () {
  fn loopCount(self, rsthis: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader9loopCountEv()};
    let mut ret = unsafe {_ZNK12QImageReader9loopCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setDecideFormatFromContent<T: QImageReader_setDecideFormatFromContent>(&mut self, value: T)  {
     value.setDecideFormatFromContent(self);
    // return 1;
  }
}

pub trait QImageReader_setDecideFormatFromContent {
  fn setDecideFormatFromContent(self, rsthis: &mut QImageReader) ;
}

// proto:  void QImageReader::setDecideFormatFromContent(bool ignored);
impl<'a> /*trait*/ QImageReader_setDecideFormatFromContent for (i8) {
  fn setDecideFormatFromContent(self, rsthis: &mut QImageReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader26setDecideFormatFromContentEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QImageReader26setDecideFormatFromContentEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn scaledClipRect<T: QImageReader_scaledClipRect>(&mut self, value: T) -> QRect {
    return value.scaledClipRect(self);
    // return 1;
  }
}

pub trait QImageReader_scaledClipRect {
  fn scaledClipRect(self, rsthis: &mut QImageReader) -> QRect;
}

// proto:  QRect QImageReader::scaledClipRect();
impl<'a> /*trait*/ QImageReader_scaledClipRect for () {
  fn scaledClipRect(self, rsthis: &mut QImageReader) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader14scaledClipRectEv()};
    let mut ret = unsafe {_ZNK12QImageReader14scaledClipRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn supportedMimeTypes<T: QImageReader_supportedMimeTypes>(&mut self, value: T)  {
     value.supportedMimeTypes(self);
    // return 1;
  }
}

pub trait QImageReader_supportedMimeTypes {
  fn supportedMimeTypes(self, rsthis: &mut QImageReader) ;
}

// proto: static QList<QByteArray> QImageReader::supportedMimeTypes();
impl<'a> /*trait*/ QImageReader_supportedMimeTypes for () {
  fn supportedMimeTypes(self, rsthis: &mut QImageReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader18supportedMimeTypesEv()};
     unsafe {_ZN12QImageReader18supportedMimeTypesEv()};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn text<T: QImageReader_text>(&mut self, value: T) -> QString {
    return value.text(self);
    // return 1;
  }
}

pub trait QImageReader_text {
  fn text(self, rsthis: &mut QImageReader) -> QString;
}

// proto:  QString QImageReader::text(const QString & key);
impl<'a> /*trait*/ QImageReader_text for (&'a  QString) {
  fn text(self, rsthis: &mut QImageReader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader4textERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QImageReader4textERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn nextImageDelay<T: QImageReader_nextImageDelay>(&mut self, value: T) -> i32 {
    return value.nextImageDelay(self);
    // return 1;
  }
}

pub trait QImageReader_nextImageDelay {
  fn nextImageDelay(self, rsthis: &mut QImageReader) -> i32;
}

// proto:  int QImageReader::nextImageDelay();
impl<'a> /*trait*/ QImageReader_nextImageDelay for () {
  fn nextImageDelay(self, rsthis: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader14nextImageDelayEv()};
    let mut ret = unsafe {_ZNK12QImageReader14nextImageDelayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn supportsAnimation<T: QImageReader_supportsAnimation>(&mut self, value: T) -> i8 {
    return value.supportsAnimation(self);
    // return 1;
  }
}

pub trait QImageReader_supportsAnimation {
  fn supportsAnimation(self, rsthis: &mut QImageReader) -> i8;
}

// proto:  bool QImageReader::supportsAnimation();
impl<'a> /*trait*/ QImageReader_supportsAnimation for () {
  fn supportsAnimation(self, rsthis: &mut QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader17supportsAnimationEv()};
    let mut ret = unsafe {_ZNK12QImageReader17supportsAnimationEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn jumpToImage<T: QImageReader_jumpToImage>(&mut self, value: T) -> i8 {
    return value.jumpToImage(self);
    // return 1;
  }
}

pub trait QImageReader_jumpToImage {
  fn jumpToImage(self, rsthis: &mut QImageReader) -> i8;
}

// proto:  bool QImageReader::jumpToImage(int imageNumber);
impl<'a> /*trait*/ QImageReader_jumpToImage for (i32) {
  fn jumpToImage(self, rsthis: &mut QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11jumpToImageEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN12QImageReader11jumpToImageEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setFileName<T: QImageReader_setFileName>(&mut self, value: T)  {
     value.setFileName(self);
    // return 1;
  }
}

pub trait QImageReader_setFileName {
  fn setFileName(self, rsthis: &mut QImageReader) ;
}

// proto:  void QImageReader::setFileName(const QString & fileName);
impl<'a> /*trait*/ QImageReader_setFileName for (&'a  QString) {
  fn setFileName(self, rsthis: &mut QImageReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageReader11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn NewQImageReader<T: QImageReader_NewQImageReader>(value: T) -> QImageReader {
    let rsthis = value.NewQImageReader();
    return rsthis;
    // return 1;
  }
}

pub trait QImageReader_NewQImageReader {
  fn NewQImageReader(self) -> QImageReader;
}

// proto: void QImageReader::NewQImageReader(const QImageReader & );
impl<'a> /*trait*/ QImageReader_NewQImageReader for (&'a  QImageReader) {
  fn NewQImageReader(self) -> QImageReader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReaderC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QImageReaderC1ERKS_(qthis, arg0)};
    let rsthis = QImageReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn scaledSize<T: QImageReader_scaledSize>(&mut self, value: T) -> QSize {
    return value.scaledSize(self);
    // return 1;
  }
}

pub trait QImageReader_scaledSize {
  fn scaledSize(self, rsthis: &mut QImageReader) -> QSize;
}

// proto:  QSize QImageReader::scaledSize();
impl<'a> /*trait*/ QImageReader_scaledSize for () {
  fn scaledSize(self, rsthis: &mut QImageReader) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader10scaledSizeEv()};
    let mut ret = unsafe {_ZNK12QImageReader10scaledSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setAutoTransform<T: QImageReader_setAutoTransform>(&mut self, value: T)  {
     value.setAutoTransform(self);
    // return 1;
  }
}

pub trait QImageReader_setAutoTransform {
  fn setAutoTransform(self, rsthis: &mut QImageReader) ;
}

// proto:  void QImageReader::setAutoTransform(bool enabled);
impl<'a> /*trait*/ QImageReader_setAutoTransform for (i8) {
  fn setAutoTransform(self, rsthis: &mut QImageReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader16setAutoTransformEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QImageReader16setAutoTransformEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setClipRect<T: QImageReader_setClipRect>(&mut self, value: T)  {
     value.setClipRect(self);
    // return 1;
  }
}

pub trait QImageReader_setClipRect {
  fn setClipRect(self, rsthis: &mut QImageReader) ;
}

// proto:  void QImageReader::setClipRect(const QRect & rect);
impl<'a> /*trait*/ QImageReader_setClipRect for (&'a  QRect) {
  fn setClipRect(self, rsthis: &mut QImageReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11setClipRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageReader11setClipRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn autoDetectImageFormat<T: QImageReader_autoDetectImageFormat>(&mut self, value: T) -> i8 {
    return value.autoDetectImageFormat(self);
    // return 1;
  }
}

pub trait QImageReader_autoDetectImageFormat {
  fn autoDetectImageFormat(self, rsthis: &mut QImageReader) -> i8;
}

// proto:  bool QImageReader::autoDetectImageFormat();
impl<'a> /*trait*/ QImageReader_autoDetectImageFormat for () {
  fn autoDetectImageFormat(self, rsthis: &mut QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader21autoDetectImageFormatEv()};
    let mut ret = unsafe {_ZNK12QImageReader21autoDetectImageFormatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn currentImageRect<T: QImageReader_currentImageRect>(&mut self, value: T) -> QRect {
    return value.currentImageRect(self);
    // return 1;
  }
}

pub trait QImageReader_currentImageRect {
  fn currentImageRect(self, rsthis: &mut QImageReader) -> QRect;
}

// proto:  QRect QImageReader::currentImageRect();
impl<'a> /*trait*/ QImageReader_currentImageRect for () {
  fn currentImageRect(self, rsthis: &mut QImageReader) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader16currentImageRectEv()};
    let mut ret = unsafe {_ZNK12QImageReader16currentImageRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QImageReader::NewQImageReader(const QString & fileName, const QByteArray & format);
impl<'a> /*trait*/ QImageReader_NewQImageReader for (&'a  QString, &'a  QByteArray) {
  fn NewQImageReader(self) -> QImageReader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReaderC1ERK7QStringRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QImageReaderC1ERK7QStringRK10QByteArray(qthis, arg0, arg1)};
    let rsthis = QImageReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: static QByteArray QImageReader::imageFormat(QIODevice * device);
impl<'a> /*trait*/ QImageReader_imageFormat for (&'a mut QIODevice) {
  fn imageFormat(self, rsthis: &mut QImageReader) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11imageFormatEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QImageReader11imageFormatEP9QIODevice(arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn quality<T: QImageReader_quality>(&mut self, value: T) -> i32 {
    return value.quality(self);
    // return 1;
  }
}

pub trait QImageReader_quality {
  fn quality(self, rsthis: &mut QImageReader) -> i32;
}

// proto:  int QImageReader::quality();
impl<'a> /*trait*/ QImageReader_quality for () {
  fn quality(self, rsthis: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader7qualityEv()};
    let mut ret = unsafe {_ZNK12QImageReader7qualityEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setDevice<T: QImageReader_setDevice>(&mut self, value: T)  {
     value.setDevice(self);
    // return 1;
  }
}

pub trait QImageReader_setDevice {
  fn setDevice(self, rsthis: &mut QImageReader) ;
}

// proto:  void QImageReader::setDevice(QIODevice * device);
impl<'a> /*trait*/ QImageReader_setDevice for (&'a mut QIODevice) {
  fn setDevice(self, rsthis: &mut QImageReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageReader9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setBackgroundColor<T: QImageReader_setBackgroundColor>(&mut self, value: T)  {
     value.setBackgroundColor(self);
    // return 1;
  }
}

pub trait QImageReader_setBackgroundColor {
  fn setBackgroundColor(self, rsthis: &mut QImageReader) ;
}

// proto:  void QImageReader::setBackgroundColor(const QColor & color);
impl<'a> /*trait*/ QImageReader_setBackgroundColor for (&'a  QColor) {
  fn setBackgroundColor(self, rsthis: &mut QImageReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader18setBackgroundColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageReader18setBackgroundColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setQuality<T: QImageReader_setQuality>(&mut self, value: T)  {
     value.setQuality(self);
    // return 1;
  }
}

pub trait QImageReader_setQuality {
  fn setQuality(self, rsthis: &mut QImageReader) ;
}

// proto:  void QImageReader::setQuality(int quality);
impl<'a> /*trait*/ QImageReader_setQuality for (i32) {
  fn setQuality(self, rsthis: &mut QImageReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader10setQualityEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QImageReader10setQualityEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QImageReader::NewQImageReader(QIODevice * device, const QByteArray & format);
impl<'a> /*trait*/ QImageReader_NewQImageReader for (&'a mut QIODevice, &'a  QByteArray) {
  fn NewQImageReader(self) -> QImageReader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReaderC1EP9QIODeviceRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QImageReaderC1EP9QIODeviceRK10QByteArray(qthis, arg0, arg1)};
    let rsthis = QImageReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setAutoDetectImageFormat<T: QImageReader_setAutoDetectImageFormat>(&mut self, value: T)  {
     value.setAutoDetectImageFormat(self);
    // return 1;
  }
}

pub trait QImageReader_setAutoDetectImageFormat {
  fn setAutoDetectImageFormat(self, rsthis: &mut QImageReader) ;
}

// proto:  void QImageReader::setAutoDetectImageFormat(bool enabled);
impl<'a> /*trait*/ QImageReader_setAutoDetectImageFormat for (i8) {
  fn setAutoDetectImageFormat(self, rsthis: &mut QImageReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader24setAutoDetectImageFormatEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QImageReader24setAutoDetectImageFormatEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QImageReader::NewQImageReader();
impl<'a> /*trait*/ QImageReader_NewQImageReader for () {
  fn NewQImageReader(self) -> QImageReader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReaderC1Ev()};
    unsafe {_ZN12QImageReaderC1Ev(qthis)};
    let rsthis = QImageReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setFormat<T: QImageReader_setFormat>(&mut self, value: T)  {
     value.setFormat(self);
    // return 1;
  }
}

pub trait QImageReader_setFormat {
  fn setFormat(self, rsthis: &mut QImageReader) ;
}

// proto:  void QImageReader::setFormat(const QByteArray & format);
impl<'a> /*trait*/ QImageReader_setFormat for (&'a  QByteArray) {
  fn setFormat(self, rsthis: &mut QImageReader)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader9setFormatERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageReader9setFormatERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn fileName<T: QImageReader_fileName>(&mut self, value: T) -> QString {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QImageReader_fileName {
  fn fileName(self, rsthis: &mut QImageReader) -> QString;
}

// proto:  QString QImageReader::fileName();
impl<'a> /*trait*/ QImageReader_fileName for () {
  fn fileName(self, rsthis: &mut QImageReader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader8fileNameEv()};
    let mut ret = unsafe {_ZNK12QImageReader8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn clipRect<T: QImageReader_clipRect>(&mut self, value: T) -> QRect {
    return value.clipRect(self);
    // return 1;
  }
}

pub trait QImageReader_clipRect {
  fn clipRect(self, rsthis: &mut QImageReader) -> QRect;
}

// proto:  QRect QImageReader::clipRect();
impl<'a> /*trait*/ QImageReader_clipRect for () {
  fn clipRect(self, rsthis: &mut QImageReader) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader8clipRectEv()};
    let mut ret = unsafe {_ZNK12QImageReader8clipRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn format<T: QImageReader_format>(&mut self, value: T) -> QByteArray {
    return value.format(self);
    // return 1;
  }
}

pub trait QImageReader_format {
  fn format(self, rsthis: &mut QImageReader) -> QByteArray;
}

// proto:  QByteArray QImageReader::format();
impl<'a> /*trait*/ QImageReader_format for () {
  fn format(self, rsthis: &mut QImageReader) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader6formatEv()};
    let mut ret = unsafe {_ZNK12QImageReader6formatEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

