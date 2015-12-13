// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsize::QSize;
use super::qimage::QImage;
use super::qrect::QRect;
use super::qstring::QString;
use super::qbytearray::QByteArray;
use super::qiodevice::QIODevice;
use super::qcolor::QColor;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QString QImageReader::errorString();
  fn _ZNK12QImageReader11errorStringEv() -> i32;
  // proto: bool QImageReader::canRead();
  fn _ZNK12QImageReader7canReadEv() -> i32;
  // proto: void QImageReader::FreeQImageReader();
  fn _ZN12QImageReaderD0Ev() -> i32;
  // proto: void QImageReader::setScaledSize(const QSize & size);
  fn _ZN12QImageReader13setScaledSizeERK5QSize(arg0: *const c_void) -> i32;
  // proto: bool QImageReader::read(QImage * image);
  fn _ZN12QImageReader4readEP6QImage(arg0: *mut c_void) -> i32;
  // proto: void QImageReader::setScaledClipRect(const QRect & rect);
  fn _ZN12QImageReader17setScaledClipRectERK5QRect(arg0: *const c_void) -> i32;
  // proto: int QImageReader::imageCount();
  fn _ZNK12QImageReader10imageCountEv() -> i32;
  // proto: QStringList QImageReader::textKeys();
  fn _ZNK12QImageReader8textKeysEv() -> i32;
  // proto: bool QImageReader::decideFormatFromContent();
  fn _ZNK12QImageReader23decideFormatFromContentEv() -> i32;
  // proto: QIODevice * QImageReader::device();
  fn _ZNK12QImageReader6deviceEv() -> i32;
  // proto: bool QImageReader::autoTransform();
  fn _ZNK12QImageReader13autoTransformEv() -> i32;
  // proto: bool QImageReader::jumpToNextImage();
  fn _ZN12QImageReader15jumpToNextImageEv() -> i32;
  // proto: QByteArray QImageReader::imageFormat(const QString & fileName);
  fn _ZN12QImageReader11imageFormatERK7QString(arg0: *const c_void) -> i32;
  // proto: QList<QByteArray> QImageReader::supportedSubTypes();
  fn _ZNK12QImageReader17supportedSubTypesEv() -> i32;
  // proto: QSize QImageReader::size();
  fn _ZNK12QImageReader4sizeEv() -> i32;
  // proto: QColor QImageReader::backgroundColor();
  fn _ZNK12QImageReader15backgroundColorEv() -> i32;
  // proto: QByteArray QImageReader::subType();
  fn _ZNK12QImageReader7subTypeEv() -> i32;
  // proto: int QImageReader::currentImageNumber();
  fn _ZNK12QImageReader18currentImageNumberEv() -> i32;
  // proto: QList<QByteArray> QImageReader::supportedImageFormats();
  fn _ZN12QImageReader21supportedImageFormatsEv() -> i32;
  // proto: int QImageReader::loopCount();
  fn _ZNK12QImageReader9loopCountEv() -> i32;
  // proto: void QImageReader::setDecideFormatFromContent(bool ignored);
  fn _ZN12QImageReader26setDecideFormatFromContentEb(arg0: int8_t) -> i32;
  // proto: QRect QImageReader::scaledClipRect();
  fn _ZNK12QImageReader14scaledClipRectEv() -> i32;
  // proto: QList<QByteArray> QImageReader::supportedMimeTypes();
  fn _ZN12QImageReader18supportedMimeTypesEv() -> i32;
  // proto: QString QImageReader::text(const QString & key);
  fn _ZNK12QImageReader4textERK7QString(arg0: *const c_void) -> i32;
  // proto: int QImageReader::nextImageDelay();
  fn _ZNK12QImageReader14nextImageDelayEv() -> i32;
  // proto: QImage QImageReader::read();
  fn _ZN12QImageReader4readEv() -> i32;
  // proto: bool QImageReader::supportsAnimation();
  fn _ZNK12QImageReader17supportsAnimationEv() -> i32;
  // proto: bool QImageReader::jumpToImage(int imageNumber);
  fn _ZN12QImageReader11jumpToImageEi(arg0: c_int) -> i32;
  // proto: void QImageReader::setFileName(const QString & fileName);
  fn _ZN12QImageReader11setFileNameERK7QString(arg0: *const c_void) -> i32;
  // proto: void QImageReader::NewQImageReader(const QImageReader & );
  fn _ZN12QImageReaderC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QSize QImageReader::scaledSize();
  fn _ZNK12QImageReader10scaledSizeEv() -> i32;
  // proto: void QImageReader::setAutoTransform(bool enabled);
  fn _ZN12QImageReader16setAutoTransformEb(arg0: int8_t) -> i32;
  // proto: void QImageReader::setClipRect(const QRect & rect);
  fn _ZN12QImageReader11setClipRectERK5QRect(arg0: *const c_void) -> i32;
  // proto: bool QImageReader::autoDetectImageFormat();
  fn _ZNK12QImageReader21autoDetectImageFormatEv() -> i32;
  // proto: QRect QImageReader::currentImageRect();
  fn _ZNK12QImageReader16currentImageRectEv() -> i32;
  // proto: void QImageReader::NewQImageReader(const QString & fileName, const QByteArray & format);
  fn _ZN12QImageReaderC1ERK7QStringRK10QByteArray(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QByteArray QImageReader::imageFormat(QIODevice * device);
  fn _ZN12QImageReader11imageFormatEP9QIODevice(arg0: *mut c_void) -> i32;
  // proto: int QImageReader::quality();
  fn _ZNK12QImageReader7qualityEv() -> i32;
  // proto: void QImageReader::setDevice(QIODevice * device);
  fn _ZN12QImageReader9setDeviceEP9QIODevice(arg0: *mut c_void) -> i32;
  // proto: void QImageReader::setBackgroundColor(const QColor & color);
  fn _ZN12QImageReader18setBackgroundColorERK6QColor(arg0: *const c_void) -> i32;
  // proto: void QImageReader::setQuality(int quality);
  fn _ZN12QImageReader10setQualityEi(arg0: c_int) -> i32;
  // proto: void QImageReader::NewQImageReader(QIODevice * device, const QByteArray & format);
  fn _ZN12QImageReaderC1EP9QIODeviceRK10QByteArray(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_void) -> i32;
  // proto: void QImageReader::setAutoDetectImageFormat(bool enabled);
  fn _ZN12QImageReader24setAutoDetectImageFormatEb(arg0: int8_t) -> i32;
  // proto: void QImageReader::NewQImageReader();
  fn _ZN12QImageReaderC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QImageReader::setFormat(const QByteArray & format);
  fn _ZN12QImageReader9setFormatERK10QByteArray(arg0: *const c_void) -> i32;
  // proto: QString QImageReader::fileName();
  fn _ZNK12QImageReader8fileNameEv() -> i32;
  // proto: QRect QImageReader::clipRect();
  fn _ZNK12QImageReader8clipRectEv() -> i32;
  // proto: QByteArray QImageReader::format();
  fn _ZNK12QImageReader6formatEv() -> i32;
}

// body block begin
// class sizeof(QImageReader)=8
pub struct QImageReader {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QImageReader {
  pub fn errorString<T: QImageReader_errorString>(&mut self, value: T) -> i32 {
    value.errorString(self);
    return 1;
  }
}

pub trait QImageReader_errorString {
  fn errorString(self, this: &mut QImageReader) -> i32;
}

// proto: QString QImageReader::errorString();
impl<'a> /*trait*/ QImageReader_errorString for () {
  fn errorString(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader11errorStringEv()};
    unsafe {_ZNK12QImageReader11errorStringEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn canRead<T: QImageReader_canRead>(&mut self, value: T) -> i32 {
    value.canRead(self);
    return 1;
  }
}

pub trait QImageReader_canRead {
  fn canRead(self, this: &mut QImageReader) -> i32;
}

// proto: bool QImageReader::canRead();
impl<'a> /*trait*/ QImageReader_canRead for () {
  fn canRead(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader7canReadEv()};
    unsafe {_ZNK12QImageReader7canReadEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn FreeQImageReader<T: QImageReader_FreeQImageReader>(&mut self, value: T) -> i32 {
    value.FreeQImageReader(self);
    return 1;
  }
}

pub trait QImageReader_FreeQImageReader {
  fn FreeQImageReader(self, this: &mut QImageReader) -> i32;
}

// proto: void QImageReader::FreeQImageReader();
impl<'a> /*trait*/ QImageReader_FreeQImageReader for () {
  fn FreeQImageReader(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReaderD0Ev()};
    unsafe {_ZN12QImageReaderD0Ev()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setScaledSize<T: QImageReader_setScaledSize>(&mut self, value: T) -> i32 {
    value.setScaledSize(self);
    return 1;
  }
}

pub trait QImageReader_setScaledSize {
  fn setScaledSize(self, this: &mut QImageReader) -> i32;
}

// proto: void QImageReader::setScaledSize(const QSize & size);
impl<'a> /*trait*/ QImageReader_setScaledSize for (&'a  QSize) {
  fn setScaledSize(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader13setScaledSizeERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QImageReader13setScaledSizeERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn read<T: QImageReader_read>(&mut self, value: T) -> i32 {
    value.read(self);
    return 1;
  }
}

pub trait QImageReader_read {
  fn read(self, this: &mut QImageReader) -> i32;
}

// proto: bool QImageReader::read(QImage * image);
impl<'a> /*trait*/ QImageReader_read for (&'a mut QImage) {
  fn read(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader4readEP6QImage()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QImageReader4readEP6QImage(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setScaledClipRect<T: QImageReader_setScaledClipRect>(&mut self, value: T) -> i32 {
    value.setScaledClipRect(self);
    return 1;
  }
}

pub trait QImageReader_setScaledClipRect {
  fn setScaledClipRect(self, this: &mut QImageReader) -> i32;
}

// proto: void QImageReader::setScaledClipRect(const QRect & rect);
impl<'a> /*trait*/ QImageReader_setScaledClipRect for (&'a  QRect) {
  fn setScaledClipRect(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader17setScaledClipRectERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QImageReader17setScaledClipRectERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn imageCount<T: QImageReader_imageCount>(&mut self, value: T) -> i32 {
    value.imageCount(self);
    return 1;
  }
}

pub trait QImageReader_imageCount {
  fn imageCount(self, this: &mut QImageReader) -> i32;
}

// proto: int QImageReader::imageCount();
impl<'a> /*trait*/ QImageReader_imageCount for () {
  fn imageCount(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader10imageCountEv()};
    unsafe {_ZNK12QImageReader10imageCountEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn textKeys<T: QImageReader_textKeys>(&mut self, value: T) -> i32 {
    value.textKeys(self);
    return 1;
  }
}

pub trait QImageReader_textKeys {
  fn textKeys(self, this: &mut QImageReader) -> i32;
}

// proto: QStringList QImageReader::textKeys();
impl<'a> /*trait*/ QImageReader_textKeys for () {
  fn textKeys(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader8textKeysEv()};
    unsafe {_ZNK12QImageReader8textKeysEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn decideFormatFromContent<T: QImageReader_decideFormatFromContent>(&mut self, value: T) -> i32 {
    value.decideFormatFromContent(self);
    return 1;
  }
}

pub trait QImageReader_decideFormatFromContent {
  fn decideFormatFromContent(self, this: &mut QImageReader) -> i32;
}

// proto: bool QImageReader::decideFormatFromContent();
impl<'a> /*trait*/ QImageReader_decideFormatFromContent for () {
  fn decideFormatFromContent(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader23decideFormatFromContentEv()};
    unsafe {_ZNK12QImageReader23decideFormatFromContentEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn device<T: QImageReader_device>(&mut self, value: T) -> i32 {
    value.device(self);
    return 1;
  }
}

pub trait QImageReader_device {
  fn device(self, this: &mut QImageReader) -> i32;
}

// proto: QIODevice * QImageReader::device();
impl<'a> /*trait*/ QImageReader_device for () {
  fn device(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader6deviceEv()};
    unsafe {_ZNK12QImageReader6deviceEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn autoTransform<T: QImageReader_autoTransform>(&mut self, value: T) -> i32 {
    value.autoTransform(self);
    return 1;
  }
}

pub trait QImageReader_autoTransform {
  fn autoTransform(self, this: &mut QImageReader) -> i32;
}

// proto: bool QImageReader::autoTransform();
impl<'a> /*trait*/ QImageReader_autoTransform for () {
  fn autoTransform(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader13autoTransformEv()};
    unsafe {_ZNK12QImageReader13autoTransformEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn jumpToNextImage<T: QImageReader_jumpToNextImage>(&mut self, value: T) -> i32 {
    value.jumpToNextImage(self);
    return 1;
  }
}

pub trait QImageReader_jumpToNextImage {
  fn jumpToNextImage(self, this: &mut QImageReader) -> i32;
}

// proto: bool QImageReader::jumpToNextImage();
impl<'a> /*trait*/ QImageReader_jumpToNextImage for () {
  fn jumpToNextImage(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader15jumpToNextImageEv()};
    unsafe {_ZN12QImageReader15jumpToNextImageEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn imageFormat<T: QImageReader_imageFormat>(&mut self, value: T) -> i32 {
    value.imageFormat(self);
    return 1;
  }
}

pub trait QImageReader_imageFormat {
  fn imageFormat(self, this: &mut QImageReader) -> i32;
}

// proto: QByteArray QImageReader::imageFormat(const QString & fileName);
impl<'a> /*trait*/ QImageReader_imageFormat for (&'a  QString) {
  fn imageFormat(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11imageFormatERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QImageReader11imageFormatERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn supportedSubTypes<T: QImageReader_supportedSubTypes>(&mut self, value: T) -> i32 {
    value.supportedSubTypes(self);
    return 1;
  }
}

pub trait QImageReader_supportedSubTypes {
  fn supportedSubTypes(self, this: &mut QImageReader) -> i32;
}

// proto: QList<QByteArray> QImageReader::supportedSubTypes();
impl<'a> /*trait*/ QImageReader_supportedSubTypes for () {
  fn supportedSubTypes(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader17supportedSubTypesEv()};
    unsafe {_ZNK12QImageReader17supportedSubTypesEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn size<T: QImageReader_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QImageReader_size {
  fn size(self, this: &mut QImageReader) -> i32;
}

// proto: QSize QImageReader::size();
impl<'a> /*trait*/ QImageReader_size for () {
  fn size(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader4sizeEv()};
    unsafe {_ZNK12QImageReader4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn backgroundColor<T: QImageReader_backgroundColor>(&mut self, value: T) -> i32 {
    value.backgroundColor(self);
    return 1;
  }
}

pub trait QImageReader_backgroundColor {
  fn backgroundColor(self, this: &mut QImageReader) -> i32;
}

// proto: QColor QImageReader::backgroundColor();
impl<'a> /*trait*/ QImageReader_backgroundColor for () {
  fn backgroundColor(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader15backgroundColorEv()};
    unsafe {_ZNK12QImageReader15backgroundColorEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn subType<T: QImageReader_subType>(&mut self, value: T) -> i32 {
    value.subType(self);
    return 1;
  }
}

pub trait QImageReader_subType {
  fn subType(self, this: &mut QImageReader) -> i32;
}

// proto: QByteArray QImageReader::subType();
impl<'a> /*trait*/ QImageReader_subType for () {
  fn subType(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader7subTypeEv()};
    unsafe {_ZNK12QImageReader7subTypeEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn currentImageNumber<T: QImageReader_currentImageNumber>(&mut self, value: T) -> i32 {
    value.currentImageNumber(self);
    return 1;
  }
}

pub trait QImageReader_currentImageNumber {
  fn currentImageNumber(self, this: &mut QImageReader) -> i32;
}

// proto: int QImageReader::currentImageNumber();
impl<'a> /*trait*/ QImageReader_currentImageNumber for () {
  fn currentImageNumber(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader18currentImageNumberEv()};
    unsafe {_ZNK12QImageReader18currentImageNumberEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn supportedImageFormats<T: QImageReader_supportedImageFormats>(&mut self, value: T) -> i32 {
    value.supportedImageFormats(self);
    return 1;
  }
}

pub trait QImageReader_supportedImageFormats {
  fn supportedImageFormats(self, this: &mut QImageReader) -> i32;
}

// proto: QList<QByteArray> QImageReader::supportedImageFormats();
impl<'a> /*trait*/ QImageReader_supportedImageFormats for () {
  fn supportedImageFormats(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader21supportedImageFormatsEv()};
    unsafe {_ZN12QImageReader21supportedImageFormatsEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn loopCount<T: QImageReader_loopCount>(&mut self, value: T) -> i32 {
    value.loopCount(self);
    return 1;
  }
}

pub trait QImageReader_loopCount {
  fn loopCount(self, this: &mut QImageReader) -> i32;
}

// proto: int QImageReader::loopCount();
impl<'a> /*trait*/ QImageReader_loopCount for () {
  fn loopCount(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader9loopCountEv()};
    unsafe {_ZNK12QImageReader9loopCountEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setDecideFormatFromContent<T: QImageReader_setDecideFormatFromContent>(&mut self, value: T) -> i32 {
    value.setDecideFormatFromContent(self);
    return 1;
  }
}

pub trait QImageReader_setDecideFormatFromContent {
  fn setDecideFormatFromContent(self, this: &mut QImageReader) -> i32;
}

// proto: void QImageReader::setDecideFormatFromContent(bool ignored);
impl<'a> /*trait*/ QImageReader_setDecideFormatFromContent for (i8) {
  fn setDecideFormatFromContent(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader26setDecideFormatFromContentEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN12QImageReader26setDecideFormatFromContentEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn scaledClipRect<T: QImageReader_scaledClipRect>(&mut self, value: T) -> i32 {
    value.scaledClipRect(self);
    return 1;
  }
}

pub trait QImageReader_scaledClipRect {
  fn scaledClipRect(self, this: &mut QImageReader) -> i32;
}

// proto: QRect QImageReader::scaledClipRect();
impl<'a> /*trait*/ QImageReader_scaledClipRect for () {
  fn scaledClipRect(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader14scaledClipRectEv()};
    unsafe {_ZNK12QImageReader14scaledClipRectEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn supportedMimeTypes<T: QImageReader_supportedMimeTypes>(&mut self, value: T) -> i32 {
    value.supportedMimeTypes(self);
    return 1;
  }
}

pub trait QImageReader_supportedMimeTypes {
  fn supportedMimeTypes(self, this: &mut QImageReader) -> i32;
}

// proto: QList<QByteArray> QImageReader::supportedMimeTypes();
impl<'a> /*trait*/ QImageReader_supportedMimeTypes for () {
  fn supportedMimeTypes(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader18supportedMimeTypesEv()};
    unsafe {_ZN12QImageReader18supportedMimeTypesEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn text<T: QImageReader_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QImageReader_text {
  fn text(self, this: &mut QImageReader) -> i32;
}

// proto: QString QImageReader::text(const QString & key);
impl<'a> /*trait*/ QImageReader_text for (&'a  QString) {
  fn text(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader4textERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK12QImageReader4textERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn nextImageDelay<T: QImageReader_nextImageDelay>(&mut self, value: T) -> i32 {
    value.nextImageDelay(self);
    return 1;
  }
}

pub trait QImageReader_nextImageDelay {
  fn nextImageDelay(self, this: &mut QImageReader) -> i32;
}

// proto: int QImageReader::nextImageDelay();
impl<'a> /*trait*/ QImageReader_nextImageDelay for () {
  fn nextImageDelay(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader14nextImageDelayEv()};
    unsafe {_ZNK12QImageReader14nextImageDelayEv()};
    return 1;
  }
}

// proto: QImage QImageReader::read();
impl<'a> /*trait*/ QImageReader_read for () {
  fn read(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader4readEv()};
    unsafe {_ZN12QImageReader4readEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn supportsAnimation<T: QImageReader_supportsAnimation>(&mut self, value: T) -> i32 {
    value.supportsAnimation(self);
    return 1;
  }
}

pub trait QImageReader_supportsAnimation {
  fn supportsAnimation(self, this: &mut QImageReader) -> i32;
}

// proto: bool QImageReader::supportsAnimation();
impl<'a> /*trait*/ QImageReader_supportsAnimation for () {
  fn supportsAnimation(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader17supportsAnimationEv()};
    unsafe {_ZNK12QImageReader17supportsAnimationEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn jumpToImage<T: QImageReader_jumpToImage>(&mut self, value: T) -> i32 {
    value.jumpToImage(self);
    return 1;
  }
}

pub trait QImageReader_jumpToImage {
  fn jumpToImage(self, this: &mut QImageReader) -> i32;
}

// proto: bool QImageReader::jumpToImage(int imageNumber);
impl<'a> /*trait*/ QImageReader_jumpToImage for (i32) {
  fn jumpToImage(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11jumpToImageEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QImageReader11jumpToImageEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setFileName<T: QImageReader_setFileName>(&mut self, value: T) -> i32 {
    value.setFileName(self);
    return 1;
  }
}

pub trait QImageReader_setFileName {
  fn setFileName(self, this: &mut QImageReader) -> i32;
}

// proto: void QImageReader::setFileName(const QString & fileName);
impl<'a> /*trait*/ QImageReader_setFileName for (&'a  QString) {
  fn setFileName(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QImageReader11setFileNameERK7QString(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QImageReaderC1ERKS_(qthis, arg0)};
    let rsthis = QImageReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn scaledSize<T: QImageReader_scaledSize>(&mut self, value: T) -> i32 {
    value.scaledSize(self);
    return 1;
  }
}

pub trait QImageReader_scaledSize {
  fn scaledSize(self, this: &mut QImageReader) -> i32;
}

// proto: QSize QImageReader::scaledSize();
impl<'a> /*trait*/ QImageReader_scaledSize for () {
  fn scaledSize(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader10scaledSizeEv()};
    unsafe {_ZNK12QImageReader10scaledSizeEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setAutoTransform<T: QImageReader_setAutoTransform>(&mut self, value: T) -> i32 {
    value.setAutoTransform(self);
    return 1;
  }
}

pub trait QImageReader_setAutoTransform {
  fn setAutoTransform(self, this: &mut QImageReader) -> i32;
}

// proto: void QImageReader::setAutoTransform(bool enabled);
impl<'a> /*trait*/ QImageReader_setAutoTransform for (i8) {
  fn setAutoTransform(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader16setAutoTransformEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN12QImageReader16setAutoTransformEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setClipRect<T: QImageReader_setClipRect>(&mut self, value: T) -> i32 {
    value.setClipRect(self);
    return 1;
  }
}

pub trait QImageReader_setClipRect {
  fn setClipRect(self, this: &mut QImageReader) -> i32;
}

// proto: void QImageReader::setClipRect(const QRect & rect);
impl<'a> /*trait*/ QImageReader_setClipRect for (&'a  QRect) {
  fn setClipRect(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11setClipRectERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QImageReader11setClipRectERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn autoDetectImageFormat<T: QImageReader_autoDetectImageFormat>(&mut self, value: T) -> i32 {
    value.autoDetectImageFormat(self);
    return 1;
  }
}

pub trait QImageReader_autoDetectImageFormat {
  fn autoDetectImageFormat(self, this: &mut QImageReader) -> i32;
}

// proto: bool QImageReader::autoDetectImageFormat();
impl<'a> /*trait*/ QImageReader_autoDetectImageFormat for () {
  fn autoDetectImageFormat(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader21autoDetectImageFormatEv()};
    unsafe {_ZNK12QImageReader21autoDetectImageFormatEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn currentImageRect<T: QImageReader_currentImageRect>(&mut self, value: T) -> i32 {
    value.currentImageRect(self);
    return 1;
  }
}

pub trait QImageReader_currentImageRect {
  fn currentImageRect(self, this: &mut QImageReader) -> i32;
}

// proto: QRect QImageReader::currentImageRect();
impl<'a> /*trait*/ QImageReader_currentImageRect for () {
  fn currentImageRect(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader16currentImageRectEv()};
    unsafe {_ZNK12QImageReader16currentImageRectEv()};
    return 1;
  }
}

// proto: void QImageReader::NewQImageReader(const QString & fileName, const QByteArray & format);
impl<'a> /*trait*/ QImageReader_NewQImageReader for (&'a  QString, &'a  QByteArray) {
  fn NewQImageReader(self) -> QImageReader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReaderC1ERK7QStringRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN12QImageReaderC1ERK7QStringRK10QByteArray(qthis, arg0, arg1)};
    let rsthis = QImageReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: QByteArray QImageReader::imageFormat(QIODevice * device);
impl<'a> /*trait*/ QImageReader_imageFormat for (&'a mut QIODevice) {
  fn imageFormat(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11imageFormatEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QImageReader11imageFormatEP9QIODevice(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn quality<T: QImageReader_quality>(&mut self, value: T) -> i32 {
    value.quality(self);
    return 1;
  }
}

pub trait QImageReader_quality {
  fn quality(self, this: &mut QImageReader) -> i32;
}

// proto: int QImageReader::quality();
impl<'a> /*trait*/ QImageReader_quality for () {
  fn quality(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader7qualityEv()};
    unsafe {_ZNK12QImageReader7qualityEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setDevice<T: QImageReader_setDevice>(&mut self, value: T) -> i32 {
    value.setDevice(self);
    return 1;
  }
}

pub trait QImageReader_setDevice {
  fn setDevice(self, this: &mut QImageReader) -> i32;
}

// proto: void QImageReader::setDevice(QIODevice * device);
impl<'a> /*trait*/ QImageReader_setDevice for (&'a mut QIODevice) {
  fn setDevice(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QImageReader9setDeviceEP9QIODevice(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setBackgroundColor<T: QImageReader_setBackgroundColor>(&mut self, value: T) -> i32 {
    value.setBackgroundColor(self);
    return 1;
  }
}

pub trait QImageReader_setBackgroundColor {
  fn setBackgroundColor(self, this: &mut QImageReader) -> i32;
}

// proto: void QImageReader::setBackgroundColor(const QColor & color);
impl<'a> /*trait*/ QImageReader_setBackgroundColor for (&'a  QColor) {
  fn setBackgroundColor(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader18setBackgroundColorERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QImageReader18setBackgroundColorERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setQuality<T: QImageReader_setQuality>(&mut self, value: T) -> i32 {
    value.setQuality(self);
    return 1;
  }
}

pub trait QImageReader_setQuality {
  fn setQuality(self, this: &mut QImageReader) -> i32;
}

// proto: void QImageReader::setQuality(int quality);
impl<'a> /*trait*/ QImageReader_setQuality for (i32) {
  fn setQuality(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader10setQualityEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QImageReader10setQualityEi(arg0)};
    return 1;
  }
}

// proto: void QImageReader::NewQImageReader(QIODevice * device, const QByteArray & format);
impl<'a> /*trait*/ QImageReader_NewQImageReader for (&'a mut QIODevice, &'a  QByteArray) {
  fn NewQImageReader(self) -> QImageReader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReaderC1EP9QIODeviceRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN12QImageReaderC1EP9QIODeviceRK10QByteArray(qthis, arg0, arg1)};
    let rsthis = QImageReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setAutoDetectImageFormat<T: QImageReader_setAutoDetectImageFormat>(&mut self, value: T) -> i32 {
    value.setAutoDetectImageFormat(self);
    return 1;
  }
}

pub trait QImageReader_setAutoDetectImageFormat {
  fn setAutoDetectImageFormat(self, this: &mut QImageReader) -> i32;
}

// proto: void QImageReader::setAutoDetectImageFormat(bool enabled);
impl<'a> /*trait*/ QImageReader_setAutoDetectImageFormat for (i8) {
  fn setAutoDetectImageFormat(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader24setAutoDetectImageFormatEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN12QImageReader24setAutoDetectImageFormatEb(arg0)};
    return 1;
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
  pub fn setFormat<T: QImageReader_setFormat>(&mut self, value: T) -> i32 {
    value.setFormat(self);
    return 1;
  }
}

pub trait QImageReader_setFormat {
  fn setFormat(self, this: &mut QImageReader) -> i32;
}

// proto: void QImageReader::setFormat(const QByteArray & format);
impl<'a> /*trait*/ QImageReader_setFormat for (&'a  QByteArray) {
  fn setFormat(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader9setFormatERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QImageReader9setFormatERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn fileName<T: QImageReader_fileName>(&mut self, value: T) -> i32 {
    value.fileName(self);
    return 1;
  }
}

pub trait QImageReader_fileName {
  fn fileName(self, this: &mut QImageReader) -> i32;
}

// proto: QString QImageReader::fileName();
impl<'a> /*trait*/ QImageReader_fileName for () {
  fn fileName(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader8fileNameEv()};
    unsafe {_ZNK12QImageReader8fileNameEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn clipRect<T: QImageReader_clipRect>(&mut self, value: T) -> i32 {
    value.clipRect(self);
    return 1;
  }
}

pub trait QImageReader_clipRect {
  fn clipRect(self, this: &mut QImageReader) -> i32;
}

// proto: QRect QImageReader::clipRect();
impl<'a> /*trait*/ QImageReader_clipRect for () {
  fn clipRect(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader8clipRectEv()};
    unsafe {_ZNK12QImageReader8clipRectEv()};
    return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn format<T: QImageReader_format>(&mut self, value: T) -> i32 {
    value.format(self);
    return 1;
  }
}

pub trait QImageReader_format {
  fn format(self, this: &mut QImageReader) -> i32;
}

// proto: QByteArray QImageReader::format();
impl<'a> /*trait*/ QImageReader_format for () {
  fn format(self, this: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader6formatEv()};
    unsafe {_ZNK12QImageReader6formatEv()};
    return 1;
  }
}

