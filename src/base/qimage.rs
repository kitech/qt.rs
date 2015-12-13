// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrect::QRect;
use super::qtransform::QTransform;
use super::qstring::QString;
use super::qbytearray::QByteArray;
use super::qpoint::QPoint;
use super::qmatrix::QMatrix;
use super::qiodevice::QIODevice;
use super::qcolor::QColor;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QImage QImage::copy(const QRect & rect);
  fn _ZNK6QImage4copyERK5QRect(arg0: *const c_void) -> i32;
  // proto: QTransform QImage::trueMatrix(const QTransform & , int w, int h);
  fn _ZN6QImage10trueMatrixERK10QTransformii(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: uchar * QImage::bits();
  fn _ZN6QImage4bitsEv() -> i32;
  // proto: void QImage::setAlphaChannel(const QImage & alphaChannel);
  fn _ZN6QImage15setAlphaChannelERKS_(arg0: *const c_void) -> i32;
  // proto: QString QImage::text(const QString & key);
  fn _ZNK6QImage4textERK7QString(arg0: *const c_void) -> i32;
  // proto: QRect QImage::rect();
  fn _ZNK6QImage4rectEv() -> i32;
  // proto: void QImage::NewQImage(const char *const [] xpm);
  fn _ZN6QImageC1EPKPKc(qthis: *mut c_void, arg0: *const *const c_char) -> i32;
  // proto: QImage QImage::createHeuristicMask(bool clipTight);
  fn _ZNK6QImage19createHeuristicMaskEb(arg0: int8_t) -> i32;
  // proto: const uchar * QImage::constBits();
  fn _ZNK6QImage9constBitsEv() -> i32;
  // proto: QImage QImage::fromData(const QByteArray & data, const char * format);
  fn _ZN6QImage8fromDataERK10QByteArrayPKc(arg0: *const c_void, arg1: *const c_char) -> i32;
  // proto: QImage QImage::fromData(const uchar * data, int size, const char * format);
  fn _ZN6QImage8fromDataEPKhiPKc(arg0: *const c_uchar, arg1: c_int, arg2: *const c_char) -> i32;
  // proto: bool QImage::isDetached();
  fn _ZNK6QImage10isDetachedEv() -> i32;
  // proto: void QImage::setOffset(const QPoint & );
  fn _ZN6QImage9setOffsetERK6QPoint(arg0: *const c_void) -> i32;
  // proto: QMatrix QImage::trueMatrix(const QMatrix & , int w, int h);
  fn _ZN6QImage10trueMatrixERK7QMatrixii(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: bool QImage::isGrayscale();
  fn _ZNK6QImage11isGrayscaleEv() -> i32;
  // proto: bool QImage::save(QIODevice * device, const char * format, int quality);
  fn _ZNK6QImage4saveEP9QIODevicePKci(arg0: *mut c_void, arg1: *const c_char, arg2: c_int) -> i32;
  // proto: int QImage::depth();
  fn _ZNK6QImage5depthEv() -> i32;
  // proto: QImage QImage::alphaChannel();
  fn _ZNK6QImage12alphaChannelEv() -> i32;
  // proto: bool QImage::hasAlphaChannel();
  fn _ZNK6QImage15hasAlphaChannelEv() -> i32;
  // proto: bool QImage::loadFromData(const uchar * buf, int len, const char * format);
  fn _ZN6QImage12loadFromDataEPKhiPKc(arg0: *const c_uchar, arg1: c_int, arg2: *const c_char) -> i32;
  // proto: int QImage::colorCount();
  fn _ZNK6QImage10colorCountEv() -> i32;
  // proto: bool QImage::allGray();
  fn _ZNK6QImage7allGrayEv() -> i32;
  // proto: void QImage::setColorCount(int );
  fn _ZN6QImage13setColorCountEi(arg0: c_int) -> i32;
  // proto: unsigned int QImage::pixel(const QPoint & pt);
  fn _ZNK6QImage5pixelERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QImage::setDevicePixelRatio(qreal scaleFactor);
  fn _ZN6QImage19setDevicePixelRatioEd(arg0: c_double) -> i32;
  // proto: QImage QImage::copy(int x, int y, int w, int h);
  fn _ZNK6QImage4copyEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QImage::setText(const QString & key, const QString & value);
  fn _ZN6QImage7setTextERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: unsigned int QImage::color(int i);
  fn _ZNK6QImage5colorEi(arg0: c_int) -> i32;
  // proto: void QImage::setPixel(const QPoint & pt, uint index_or_rgb);
  fn _ZN6QImage8setPixelERK6QPointj(arg0: *const c_void, arg1: c_uint) -> i32;
  // proto: QPoint QImage::offset();
  fn _ZNK6QImage6offsetEv() -> i32;
  // proto: const uchar * QImage::constScanLine(int );
  fn _ZNK6QImage13constScanLineEi(arg0: c_int) -> i32;
  // proto: QStringList QImage::textKeys();
  fn _ZNK6QImage8textKeysEv() -> i32;
  // proto: int QImage::dotsPerMeterY();
  fn _ZNK6QImage13dotsPerMeterYEv() -> i32;
  // proto: void QImage::fill(uint pixel);
  fn _ZN6QImage4fillEj(arg0: c_uint) -> i32;
  // proto: QPixelFormat QImage::pixelFormat();
  fn _ZNK6QImage11pixelFormatEv() -> i32;
  // proto: int QImage::dotsPerMeterX();
  fn _ZNK6QImage13dotsPerMeterXEv() -> i32;
  // proto: void QImage::setDotsPerMeterY(int );
  fn _ZN6QImage16setDotsPerMeterYEi(arg0: c_int) -> i32;
  // proto: int QImage::bitPlaneCount();
  fn _ZNK6QImage13bitPlaneCountEv() -> i32;
  // proto: void QImage::fill(const QColor & color);
  fn _ZN6QImage4fillERK6QColor(arg0: *const c_void) -> i32;
  // proto: void QImage::detach();
  fn _ZN6QImage6detachEv() -> i32;
  // proto: bool QImage::loadFromData(const QByteArray & data, const char * aformat);
  fn _ZN6QImage12loadFromDataERK10QByteArrayPKc(arg0: *const c_void, arg1: *const c_char) -> i32;
  // proto: void QImage::NewQImage(const QString & fileName, const char * format);
  fn _ZN6QImageC1ERK7QStringPKc(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_char) -> i32;
  // proto: QPaintEngine * QImage::paintEngine();
  fn _ZNK6QImage11paintEngineEv() -> i32;
  // proto: void QImage::NewQImage(const QImage & );
  fn _ZN6QImageC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QImage::swap(QImage & other);
  fn _ZN6QImage4swapERS_(arg0: *mut c_void) -> i32;
  // proto: double QImage::devicePixelRatio();
  fn _ZNK6QImage16devicePixelRatioEv() -> i32;
  // proto: int QImage::devType();
  fn _ZNK6QImage7devTypeEv() -> i32;
  // proto: bool QImage::valid(const QPoint & pt);
  fn _ZNK6QImage5validERK6QPoint(arg0: *const c_void) -> i32;
  // proto: int QImage::pixelIndex(const QPoint & pt);
  fn _ZNK6QImage10pixelIndexERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QImage::setDotsPerMeterX(int );
  fn _ZN6QImage16setDotsPerMeterXEi(arg0: c_int) -> i32;
  // proto: void QImage::setPixel(int x, int y, uint index_or_rgb);
  fn _ZN6QImage8setPixelEiij(arg0: c_int, arg1: c_int, arg2: c_uint) -> i32;
  // proto: bool QImage::load(const QString & fileName, const char * format);
  fn _ZN6QImage4loadERK7QStringPKc(arg0: *const c_void, arg1: *const c_char) -> i32;
  // proto: QVector<QRgb> QImage::colorTable();
  fn _ZNK6QImage10colorTableEv() -> i32;
  // proto: QSize QImage::size();
  fn _ZNK6QImage4sizeEv() -> i32;
  // proto: int QImage::height();
  fn _ZNK6QImage6heightEv() -> i32;
  // proto: int QImage::pixelIndex(int x, int y);
  fn _ZNK6QImage10pixelIndexEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: int QImage::width();
  fn _ZNK6QImage5widthEv() -> i32;
  // proto: bool QImage::load(QIODevice * device, const char * format);
  fn _ZN6QImage4loadEP9QIODevicePKc(arg0: *mut c_void, arg1: *const c_char) -> i32;
  // proto: void QImage::NewQImage();
  fn _ZN6QImageC1Ev(qthis: *mut c_void) -> i32;
  // proto: uchar * QImage::scanLine(int );
  fn _ZN6QImage8scanLineEi(arg0: c_int) -> i32;
  // proto: int QImage::bytesPerLine();
  fn _ZNK6QImage12bytesPerLineEv() -> i32;
  // proto: long long QImage::cacheKey();
  fn _ZNK6QImage8cacheKeyEv() -> i32;
  // proto: unsigned int QImage::pixel(int x, int y);
  fn _ZNK6QImage5pixelEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QImage::FreeQImage();
  fn _ZN6QImageD0Ev() -> i32;
  // proto: bool QImage::save(const QString & fileName, const char * format, int quality);
  fn _ZNK6QImage4saveERK7QStringPKci(arg0: *const c_void, arg1: *const c_char, arg2: c_int) -> i32;
  // proto: void QImage::setColor(int i, QRgb c);
  fn _ZN6QImage8setColorEij(arg0: c_int, arg1: c_uint) -> i32;
  // proto: bool QImage::isNull();
  fn _ZNK6QImage6isNullEv() -> i32;
  // proto: int QImage::byteCount();
  fn _ZNK6QImage9byteCountEv() -> i32;
  // proto: bool QImage::valid(int x, int y);
  fn _ZNK6QImage5validEii(arg0: c_int, arg1: c_int) -> i32;
}

// body block begin
// class sizeof(QImage)=32
pub struct QImage {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QImage {
  pub fn copy<T: QImage_copy>(&mut self, value: T) -> i32 {
    value.copy(self);
    return 1;
  }
}

pub trait QImage_copy {
  fn copy(self, this: &mut QImage) -> i32;
}

// proto: QImage QImage::copy(const QRect & rect);
impl<'a> /*trait*/ QImage_copy for (&'a  QRect) {
  fn copy(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4copyERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK6QImage4copyERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn trueMatrix<T: QImage_trueMatrix>(&mut self, value: T) -> i32 {
    value.trueMatrix(self);
    return 1;
  }
}

pub trait QImage_trueMatrix {
  fn trueMatrix(self, this: &mut QImage) -> i32;
}

// proto: QTransform QImage::trueMatrix(const QTransform & , int w, int h);
impl<'a> /*trait*/ QImage_trueMatrix for (&'a  QTransform, i32, i32) {
  fn trueMatrix(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage10trueMatrixERK10QTransformii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN6QImage10trueMatrixERK10QTransformii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn bits<T: QImage_bits>(&mut self, value: T) -> i32 {
    value.bits(self);
    return 1;
  }
}

pub trait QImage_bits {
  fn bits(self, this: &mut QImage) -> i32;
}

// proto: uchar * QImage::bits();
impl<'a> /*trait*/ QImage_bits for () {
  fn bits(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4bitsEv()};
    unsafe {_ZN6QImage4bitsEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn setAlphaChannel<T: QImage_setAlphaChannel>(&mut self, value: T) -> i32 {
    value.setAlphaChannel(self);
    return 1;
  }
}

pub trait QImage_setAlphaChannel {
  fn setAlphaChannel(self, this: &mut QImage) -> i32;
}

// proto: void QImage::setAlphaChannel(const QImage & alphaChannel);
impl<'a> /*trait*/ QImage_setAlphaChannel for (&'a  QImage) {
  fn setAlphaChannel(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage15setAlphaChannelERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QImage15setAlphaChannelERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn text<T: QImage_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QImage_text {
  fn text(self, this: &mut QImage) -> i32;
}

// proto: QString QImage::text(const QString & key);
impl<'a> /*trait*/ QImage_text for (&'a  QString) {
  fn text(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4textERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK6QImage4textERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn rect<T: QImage_rect>(&mut self, value: T) -> i32 {
    value.rect(self);
    return 1;
  }
}

pub trait QImage_rect {
  fn rect(self, this: &mut QImage) -> i32;
}

// proto: QRect QImage::rect();
impl<'a> /*trait*/ QImage_rect for () {
  fn rect(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4rectEv()};
    unsafe {_ZNK6QImage4rectEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn NewQImage<T: QImage_NewQImage>(value: T) -> QImage {
    let rsthis = value.NewQImage();
    return rsthis;
    // return 1;
  }
}

pub trait QImage_NewQImage {
  fn NewQImage(self) -> QImage;
}

// proto: void QImage::NewQImage(const char *const [] xpm);
impl<'a> /*trait*/ QImage_NewQImage for (&'a  Vec<&'a  i8>) {
  fn NewQImage(self) -> QImage {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImageC1EPKPKc()};
    let arg0 = 0  as *const *const c_char;
    unsafe {_ZN6QImageC1EPKPKc(qthis, arg0)};
    let rsthis = QImage{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn createHeuristicMask<T: QImage_createHeuristicMask>(&mut self, value: T) -> i32 {
    value.createHeuristicMask(self);
    return 1;
  }
}

pub trait QImage_createHeuristicMask {
  fn createHeuristicMask(self, this: &mut QImage) -> i32;
}

// proto: QImage QImage::createHeuristicMask(bool clipTight);
impl<'a> /*trait*/ QImage_createHeuristicMask for (i8) {
  fn createHeuristicMask(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage19createHeuristicMaskEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZNK6QImage19createHeuristicMaskEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn constBits<T: QImage_constBits>(&mut self, value: T) -> i32 {
    value.constBits(self);
    return 1;
  }
}

pub trait QImage_constBits {
  fn constBits(self, this: &mut QImage) -> i32;
}

// proto: const uchar * QImage::constBits();
impl<'a> /*trait*/ QImage_constBits for () {
  fn constBits(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage9constBitsEv()};
    unsafe {_ZNK6QImage9constBitsEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn fromData<T: QImage_fromData>(&mut self, value: T) -> i32 {
    value.fromData(self);
    return 1;
  }
}

pub trait QImage_fromData {
  fn fromData(self, this: &mut QImage) -> i32;
}

// proto: QImage QImage::fromData(const QByteArray & data, const char * format);
impl<'a> /*trait*/ QImage_fromData for (&'a  QByteArray, &'a  String) {
  fn fromData(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage8fromDataERK10QByteArrayPKc()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN6QImage8fromDataERK10QByteArrayPKc(arg0, arg1)};
    return 1;
  }
}

// proto: QImage QImage::fromData(const uchar * data, int size, const char * format);
impl<'a> /*trait*/ QImage_fromData for (&'a  String, i32, &'a  String) {
  fn fromData(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage8fromDataEPKhiPKc()};
    let arg0 = self.0.as_ptr()  as *const c_uchar;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *const c_char;
    unsafe {_ZN6QImage8fromDataEPKhiPKc(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn isDetached<T: QImage_isDetached>(&mut self, value: T) -> i32 {
    value.isDetached(self);
    return 1;
  }
}

pub trait QImage_isDetached {
  fn isDetached(self, this: &mut QImage) -> i32;
}

// proto: bool QImage::isDetached();
impl<'a> /*trait*/ QImage_isDetached for () {
  fn isDetached(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage10isDetachedEv()};
    unsafe {_ZNK6QImage10isDetachedEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn setOffset<T: QImage_setOffset>(&mut self, value: T) -> i32 {
    value.setOffset(self);
    return 1;
  }
}

pub trait QImage_setOffset {
  fn setOffset(self, this: &mut QImage) -> i32;
}

// proto: void QImage::setOffset(const QPoint & );
impl<'a> /*trait*/ QImage_setOffset for (&'a  QPoint) {
  fn setOffset(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage9setOffsetERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QImage9setOffsetERK6QPoint(arg0)};
    return 1;
  }
}

// proto: QMatrix QImage::trueMatrix(const QMatrix & , int w, int h);
impl<'a> /*trait*/ QImage_trueMatrix for (&'a  QMatrix, i32, i32) {
  fn trueMatrix(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage10trueMatrixERK7QMatrixii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN6QImage10trueMatrixERK7QMatrixii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn isGrayscale<T: QImage_isGrayscale>(&mut self, value: T) -> i32 {
    value.isGrayscale(self);
    return 1;
  }
}

pub trait QImage_isGrayscale {
  fn isGrayscale(self, this: &mut QImage) -> i32;
}

// proto: bool QImage::isGrayscale();
impl<'a> /*trait*/ QImage_isGrayscale for () {
  fn isGrayscale(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage11isGrayscaleEv()};
    unsafe {_ZNK6QImage11isGrayscaleEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn save<T: QImage_save>(&mut self, value: T) -> i32 {
    value.save(self);
    return 1;
  }
}

pub trait QImage_save {
  fn save(self, this: &mut QImage) -> i32;
}

// proto: bool QImage::save(QIODevice * device, const char * format, int quality);
impl<'a> /*trait*/ QImage_save for (&'a mut QIODevice, &'a  String, i32) {
  fn save(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4saveEP9QIODevicePKci()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2  as c_int;
    unsafe {_ZNK6QImage4saveEP9QIODevicePKci(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn depth<T: QImage_depth>(&mut self, value: T) -> i32 {
    value.depth(self);
    return 1;
  }
}

pub trait QImage_depth {
  fn depth(self, this: &mut QImage) -> i32;
}

// proto: int QImage::depth();
impl<'a> /*trait*/ QImage_depth for () {
  fn depth(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5depthEv()};
    unsafe {_ZNK6QImage5depthEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn alphaChannel<T: QImage_alphaChannel>(&mut self, value: T) -> i32 {
    value.alphaChannel(self);
    return 1;
  }
}

pub trait QImage_alphaChannel {
  fn alphaChannel(self, this: &mut QImage) -> i32;
}

// proto: QImage QImage::alphaChannel();
impl<'a> /*trait*/ QImage_alphaChannel for () {
  fn alphaChannel(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage12alphaChannelEv()};
    unsafe {_ZNK6QImage12alphaChannelEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn hasAlphaChannel<T: QImage_hasAlphaChannel>(&mut self, value: T) -> i32 {
    value.hasAlphaChannel(self);
    return 1;
  }
}

pub trait QImage_hasAlphaChannel {
  fn hasAlphaChannel(self, this: &mut QImage) -> i32;
}

// proto: bool QImage::hasAlphaChannel();
impl<'a> /*trait*/ QImage_hasAlphaChannel for () {
  fn hasAlphaChannel(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage15hasAlphaChannelEv()};
    unsafe {_ZNK6QImage15hasAlphaChannelEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn loadFromData<T: QImage_loadFromData>(&mut self, value: T) -> i32 {
    value.loadFromData(self);
    return 1;
  }
}

pub trait QImage_loadFromData {
  fn loadFromData(self, this: &mut QImage) -> i32;
}

// proto: bool QImage::loadFromData(const uchar * buf, int len, const char * format);
impl<'a> /*trait*/ QImage_loadFromData for (&'a  String, i32, &'a  String) {
  fn loadFromData(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage12loadFromDataEPKhiPKc()};
    let arg0 = self.0.as_ptr()  as *const c_uchar;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *const c_char;
    unsafe {_ZN6QImage12loadFromDataEPKhiPKc(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn colorCount<T: QImage_colorCount>(&mut self, value: T) -> i32 {
    value.colorCount(self);
    return 1;
  }
}

pub trait QImage_colorCount {
  fn colorCount(self, this: &mut QImage) -> i32;
}

// proto: int QImage::colorCount();
impl<'a> /*trait*/ QImage_colorCount for () {
  fn colorCount(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage10colorCountEv()};
    unsafe {_ZNK6QImage10colorCountEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn allGray<T: QImage_allGray>(&mut self, value: T) -> i32 {
    value.allGray(self);
    return 1;
  }
}

pub trait QImage_allGray {
  fn allGray(self, this: &mut QImage) -> i32;
}

// proto: bool QImage::allGray();
impl<'a> /*trait*/ QImage_allGray for () {
  fn allGray(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage7allGrayEv()};
    unsafe {_ZNK6QImage7allGrayEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn setColorCount<T: QImage_setColorCount>(&mut self, value: T) -> i32 {
    value.setColorCount(self);
    return 1;
  }
}

pub trait QImage_setColorCount {
  fn setColorCount(self, this: &mut QImage) -> i32;
}

// proto: void QImage::setColorCount(int );
impl<'a> /*trait*/ QImage_setColorCount for (i32) {
  fn setColorCount(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage13setColorCountEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QImage13setColorCountEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn pixel<T: QImage_pixel>(&mut self, value: T) -> i32 {
    value.pixel(self);
    return 1;
  }
}

pub trait QImage_pixel {
  fn pixel(self, this: &mut QImage) -> i32;
}

// proto: unsigned int QImage::pixel(const QPoint & pt);
impl<'a> /*trait*/ QImage_pixel for (&'a  QPoint) {
  fn pixel(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5pixelERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK6QImage5pixelERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn setDevicePixelRatio<T: QImage_setDevicePixelRatio>(&mut self, value: T) -> i32 {
    value.setDevicePixelRatio(self);
    return 1;
  }
}

pub trait QImage_setDevicePixelRatio {
  fn setDevicePixelRatio(self, this: &mut QImage) -> i32;
}

// proto: void QImage::setDevicePixelRatio(qreal scaleFactor);
impl<'a> /*trait*/ QImage_setDevicePixelRatio for (f64) {
  fn setDevicePixelRatio(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage19setDevicePixelRatioEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QImage19setDevicePixelRatioEd(arg0)};
    return 1;
  }
}

// proto: QImage QImage::copy(int x, int y, int w, int h);
impl<'a> /*trait*/ QImage_copy for (i32, i32, i32, i32) {
  fn copy(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4copyEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZNK6QImage4copyEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn setText<T: QImage_setText>(&mut self, value: T) -> i32 {
    value.setText(self);
    return 1;
  }
}

pub trait QImage_setText {
  fn setText(self, this: &mut QImage) -> i32;
}

// proto: void QImage::setText(const QString & key, const QString & value);
impl<'a> /*trait*/ QImage_setText for (&'a  QString, &'a  QString) {
  fn setText(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage7setTextERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN6QImage7setTextERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn color<T: QImage_color>(&mut self, value: T) -> i32 {
    value.color(self);
    return 1;
  }
}

pub trait QImage_color {
  fn color(self, this: &mut QImage) -> i32;
}

// proto: unsigned int QImage::color(int i);
impl<'a> /*trait*/ QImage_color for (i32) {
  fn color(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5colorEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK6QImage5colorEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn setPixel<T: QImage_setPixel>(&mut self, value: T) -> i32 {
    value.setPixel(self);
    return 1;
  }
}

pub trait QImage_setPixel {
  fn setPixel(self, this: &mut QImage) -> i32;
}

// proto: void QImage::setPixel(const QPoint & pt, uint index_or_rgb);
impl<'a> /*trait*/ QImage_setPixel for (&'a  QPoint, u32) {
  fn setPixel(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage8setPixelERK6QPointj()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_uint;
    unsafe {_ZN6QImage8setPixelERK6QPointj(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn offset<T: QImage_offset>(&mut self, value: T) -> i32 {
    value.offset(self);
    return 1;
  }
}

pub trait QImage_offset {
  fn offset(self, this: &mut QImage) -> i32;
}

// proto: QPoint QImage::offset();
impl<'a> /*trait*/ QImage_offset for () {
  fn offset(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage6offsetEv()};
    unsafe {_ZNK6QImage6offsetEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn constScanLine<T: QImage_constScanLine>(&mut self, value: T) -> i32 {
    value.constScanLine(self);
    return 1;
  }
}

pub trait QImage_constScanLine {
  fn constScanLine(self, this: &mut QImage) -> i32;
}

// proto: const uchar * QImage::constScanLine(int );
impl<'a> /*trait*/ QImage_constScanLine for (i32) {
  fn constScanLine(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage13constScanLineEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK6QImage13constScanLineEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn textKeys<T: QImage_textKeys>(&mut self, value: T) -> i32 {
    value.textKeys(self);
    return 1;
  }
}

pub trait QImage_textKeys {
  fn textKeys(self, this: &mut QImage) -> i32;
}

// proto: QStringList QImage::textKeys();
impl<'a> /*trait*/ QImage_textKeys for () {
  fn textKeys(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage8textKeysEv()};
    unsafe {_ZNK6QImage8textKeysEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn dotsPerMeterY<T: QImage_dotsPerMeterY>(&mut self, value: T) -> i32 {
    value.dotsPerMeterY(self);
    return 1;
  }
}

pub trait QImage_dotsPerMeterY {
  fn dotsPerMeterY(self, this: &mut QImage) -> i32;
}

// proto: int QImage::dotsPerMeterY();
impl<'a> /*trait*/ QImage_dotsPerMeterY for () {
  fn dotsPerMeterY(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage13dotsPerMeterYEv()};
    unsafe {_ZNK6QImage13dotsPerMeterYEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn fill<T: QImage_fill>(&mut self, value: T) -> i32 {
    value.fill(self);
    return 1;
  }
}

pub trait QImage_fill {
  fn fill(self, this: &mut QImage) -> i32;
}

// proto: void QImage::fill(uint pixel);
impl<'a> /*trait*/ QImage_fill for (u32) {
  fn fill(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4fillEj()};
    let arg0 = self  as c_uint;
    unsafe {_ZN6QImage4fillEj(arg0)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn pixelFormat<T: QImage_pixelFormat>(&mut self, value: T) -> i32 {
    value.pixelFormat(self);
    return 1;
  }
}

pub trait QImage_pixelFormat {
  fn pixelFormat(self, this: &mut QImage) -> i32;
}

// proto: QPixelFormat QImage::pixelFormat();
impl<'a> /*trait*/ QImage_pixelFormat for () {
  fn pixelFormat(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage11pixelFormatEv()};
    unsafe {_ZNK6QImage11pixelFormatEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn dotsPerMeterX<T: QImage_dotsPerMeterX>(&mut self, value: T) -> i32 {
    value.dotsPerMeterX(self);
    return 1;
  }
}

pub trait QImage_dotsPerMeterX {
  fn dotsPerMeterX(self, this: &mut QImage) -> i32;
}

// proto: int QImage::dotsPerMeterX();
impl<'a> /*trait*/ QImage_dotsPerMeterX for () {
  fn dotsPerMeterX(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage13dotsPerMeterXEv()};
    unsafe {_ZNK6QImage13dotsPerMeterXEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn setDotsPerMeterY<T: QImage_setDotsPerMeterY>(&mut self, value: T) -> i32 {
    value.setDotsPerMeterY(self);
    return 1;
  }
}

pub trait QImage_setDotsPerMeterY {
  fn setDotsPerMeterY(self, this: &mut QImage) -> i32;
}

// proto: void QImage::setDotsPerMeterY(int );
impl<'a> /*trait*/ QImage_setDotsPerMeterY for (i32) {
  fn setDotsPerMeterY(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage16setDotsPerMeterYEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QImage16setDotsPerMeterYEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn bitPlaneCount<T: QImage_bitPlaneCount>(&mut self, value: T) -> i32 {
    value.bitPlaneCount(self);
    return 1;
  }
}

pub trait QImage_bitPlaneCount {
  fn bitPlaneCount(self, this: &mut QImage) -> i32;
}

// proto: int QImage::bitPlaneCount();
impl<'a> /*trait*/ QImage_bitPlaneCount for () {
  fn bitPlaneCount(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage13bitPlaneCountEv()};
    unsafe {_ZNK6QImage13bitPlaneCountEv()};
    return 1;
  }
}

// proto: void QImage::fill(const QColor & color);
impl<'a> /*trait*/ QImage_fill for (&'a  QColor) {
  fn fill(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4fillERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QImage4fillERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn detach<T: QImage_detach>(&mut self, value: T) -> i32 {
    value.detach(self);
    return 1;
  }
}

pub trait QImage_detach {
  fn detach(self, this: &mut QImage) -> i32;
}

// proto: void QImage::detach();
impl<'a> /*trait*/ QImage_detach for () {
  fn detach(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage6detachEv()};
    unsafe {_ZN6QImage6detachEv()};
    return 1;
  }
}

// proto: bool QImage::loadFromData(const QByteArray & data, const char * aformat);
impl<'a> /*trait*/ QImage_loadFromData for (&'a  QByteArray, &'a  String) {
  fn loadFromData(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage12loadFromDataERK10QByteArrayPKc()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN6QImage12loadFromDataERK10QByteArrayPKc(arg0, arg1)};
    return 1;
  }
}

// proto: void QImage::NewQImage(const QString & fileName, const char * format);
impl<'a> /*trait*/ QImage_NewQImage for (&'a  QString, &'a  String) {
  fn NewQImage(self) -> QImage {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImageC1ERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN6QImageC1ERK7QStringPKc(qthis, arg0, arg1)};
    let rsthis = QImage{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn paintEngine<T: QImage_paintEngine>(&mut self, value: T) -> i32 {
    value.paintEngine(self);
    return 1;
  }
}

pub trait QImage_paintEngine {
  fn paintEngine(self, this: &mut QImage) -> i32;
}

// proto: QPaintEngine * QImage::paintEngine();
impl<'a> /*trait*/ QImage_paintEngine for () {
  fn paintEngine(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage11paintEngineEv()};
    unsafe {_ZNK6QImage11paintEngineEv()};
    return 1;
  }
}

// proto: void QImage::NewQImage(const QImage & );
impl<'a> /*trait*/ QImage_NewQImage for (&'a  QImage) {
  fn NewQImage(self) -> QImage {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImageC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QImageC1ERKS_(qthis, arg0)};
    let rsthis = QImage{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn swap<T: QImage_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QImage_swap {
  fn swap(self, this: &mut QImage) -> i32;
}

// proto: void QImage::swap(QImage & other);
impl<'a> /*trait*/ QImage_swap for (&'a mut QImage) {
  fn swap(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QImage4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn devicePixelRatio<T: QImage_devicePixelRatio>(&mut self, value: T) -> i32 {
    value.devicePixelRatio(self);
    return 1;
  }
}

pub trait QImage_devicePixelRatio {
  fn devicePixelRatio(self, this: &mut QImage) -> i32;
}

// proto: double QImage::devicePixelRatio();
impl<'a> /*trait*/ QImage_devicePixelRatio for () {
  fn devicePixelRatio(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage16devicePixelRatioEv()};
    unsafe {_ZNK6QImage16devicePixelRatioEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn devType<T: QImage_devType>(&mut self, value: T) -> i32 {
    value.devType(self);
    return 1;
  }
}

pub trait QImage_devType {
  fn devType(self, this: &mut QImage) -> i32;
}

// proto: int QImage::devType();
impl<'a> /*trait*/ QImage_devType for () {
  fn devType(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage7devTypeEv()};
    unsafe {_ZNK6QImage7devTypeEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn valid<T: QImage_valid>(&mut self, value: T) -> i32 {
    value.valid(self);
    return 1;
  }
}

pub trait QImage_valid {
  fn valid(self, this: &mut QImage) -> i32;
}

// proto: bool QImage::valid(const QPoint & pt);
impl<'a> /*trait*/ QImage_valid for (&'a  QPoint) {
  fn valid(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5validERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK6QImage5validERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn pixelIndex<T: QImage_pixelIndex>(&mut self, value: T) -> i32 {
    value.pixelIndex(self);
    return 1;
  }
}

pub trait QImage_pixelIndex {
  fn pixelIndex(self, this: &mut QImage) -> i32;
}

// proto: int QImage::pixelIndex(const QPoint & pt);
impl<'a> /*trait*/ QImage_pixelIndex for (&'a  QPoint) {
  fn pixelIndex(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage10pixelIndexERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK6QImage10pixelIndexERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn setDotsPerMeterX<T: QImage_setDotsPerMeterX>(&mut self, value: T) -> i32 {
    value.setDotsPerMeterX(self);
    return 1;
  }
}

pub trait QImage_setDotsPerMeterX {
  fn setDotsPerMeterX(self, this: &mut QImage) -> i32;
}

// proto: void QImage::setDotsPerMeterX(int );
impl<'a> /*trait*/ QImage_setDotsPerMeterX for (i32) {
  fn setDotsPerMeterX(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage16setDotsPerMeterXEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QImage16setDotsPerMeterXEi(arg0)};
    return 1;
  }
}

// proto: void QImage::setPixel(int x, int y, uint index_or_rgb);
impl<'a> /*trait*/ QImage_setPixel for (i32, i32, u32) {
  fn setPixel(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage8setPixelEiij()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uint;
    unsafe {_ZN6QImage8setPixelEiij(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn load<T: QImage_load>(&mut self, value: T) -> i32 {
    value.load(self);
    return 1;
  }
}

pub trait QImage_load {
  fn load(self, this: &mut QImage) -> i32;
}

// proto: bool QImage::load(const QString & fileName, const char * format);
impl<'a> /*trait*/ QImage_load for (&'a  QString, &'a  String) {
  fn load(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4loadERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN6QImage4loadERK7QStringPKc(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn colorTable<T: QImage_colorTable>(&mut self, value: T) -> i32 {
    value.colorTable(self);
    return 1;
  }
}

pub trait QImage_colorTable {
  fn colorTable(self, this: &mut QImage) -> i32;
}

// proto: QVector<QRgb> QImage::colorTable();
impl<'a> /*trait*/ QImage_colorTable for () {
  fn colorTable(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage10colorTableEv()};
    unsafe {_ZNK6QImage10colorTableEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn size<T: QImage_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QImage_size {
  fn size(self, this: &mut QImage) -> i32;
}

// proto: QSize QImage::size();
impl<'a> /*trait*/ QImage_size for () {
  fn size(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4sizeEv()};
    unsafe {_ZNK6QImage4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn height<T: QImage_height>(&mut self, value: T) -> i32 {
    value.height(self);
    return 1;
  }
}

pub trait QImage_height {
  fn height(self, this: &mut QImage) -> i32;
}

// proto: int QImage::height();
impl<'a> /*trait*/ QImage_height for () {
  fn height(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage6heightEv()};
    unsafe {_ZNK6QImage6heightEv()};
    return 1;
  }
}

// proto: int QImage::pixelIndex(int x, int y);
impl<'a> /*trait*/ QImage_pixelIndex for (i32, i32) {
  fn pixelIndex(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage10pixelIndexEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK6QImage10pixelIndexEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn width<T: QImage_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QImage_width {
  fn width(self, this: &mut QImage) -> i32;
}

// proto: int QImage::width();
impl<'a> /*trait*/ QImage_width for () {
  fn width(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5widthEv()};
    unsafe {_ZNK6QImage5widthEv()};
    return 1;
  }
}

// proto: bool QImage::load(QIODevice * device, const char * format);
impl<'a> /*trait*/ QImage_load for (&'a mut QIODevice, &'a  String) {
  fn load(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4loadEP9QIODevicePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN6QImage4loadEP9QIODevicePKc(arg0, arg1)};
    return 1;
  }
}

// proto: void QImage::NewQImage();
impl<'a> /*trait*/ QImage_NewQImage for () {
  fn NewQImage(self) -> QImage {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImageC1Ev()};
    unsafe {_ZN6QImageC1Ev(qthis)};
    let rsthis = QImage{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn scanLine<T: QImage_scanLine>(&mut self, value: T) -> i32 {
    value.scanLine(self);
    return 1;
  }
}

pub trait QImage_scanLine {
  fn scanLine(self, this: &mut QImage) -> i32;
}

// proto: uchar * QImage::scanLine(int );
impl<'a> /*trait*/ QImage_scanLine for (i32) {
  fn scanLine(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage8scanLineEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN6QImage8scanLineEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn bytesPerLine<T: QImage_bytesPerLine>(&mut self, value: T) -> i32 {
    value.bytesPerLine(self);
    return 1;
  }
}

pub trait QImage_bytesPerLine {
  fn bytesPerLine(self, this: &mut QImage) -> i32;
}

// proto: int QImage::bytesPerLine();
impl<'a> /*trait*/ QImage_bytesPerLine for () {
  fn bytesPerLine(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage12bytesPerLineEv()};
    unsafe {_ZNK6QImage12bytesPerLineEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn cacheKey<T: QImage_cacheKey>(&mut self, value: T) -> i32 {
    value.cacheKey(self);
    return 1;
  }
}

pub trait QImage_cacheKey {
  fn cacheKey(self, this: &mut QImage) -> i32;
}

// proto: long long QImage::cacheKey();
impl<'a> /*trait*/ QImage_cacheKey for () {
  fn cacheKey(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage8cacheKeyEv()};
    unsafe {_ZNK6QImage8cacheKeyEv()};
    return 1;
  }
}

// proto: unsigned int QImage::pixel(int x, int y);
impl<'a> /*trait*/ QImage_pixel for (i32, i32) {
  fn pixel(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5pixelEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK6QImage5pixelEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn FreeQImage<T: QImage_FreeQImage>(&mut self, value: T) -> i32 {
    value.FreeQImage(self);
    return 1;
  }
}

pub trait QImage_FreeQImage {
  fn FreeQImage(self, this: &mut QImage) -> i32;
}

// proto: void QImage::FreeQImage();
impl<'a> /*trait*/ QImage_FreeQImage for () {
  fn FreeQImage(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImageD0Ev()};
    unsafe {_ZN6QImageD0Ev()};
    return 1;
  }
}

// proto: bool QImage::save(const QString & fileName, const char * format, int quality);
impl<'a> /*trait*/ QImage_save for (&'a  QString, &'a  String, i32) {
  fn save(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4saveERK7QStringPKci()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2  as c_int;
    unsafe {_ZNK6QImage4saveERK7QStringPKci(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn setColor<T: QImage_setColor>(&mut self, value: T) -> i32 {
    value.setColor(self);
    return 1;
  }
}

pub trait QImage_setColor {
  fn setColor(self, this: &mut QImage) -> i32;
}

// proto: void QImage::setColor(int i, QRgb c);
impl<'a> /*trait*/ QImage_setColor for (i32, u32) {
  fn setColor(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage8setColorEij()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_uint;
    unsafe {_ZN6QImage8setColorEij(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn isNull<T: QImage_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QImage_isNull {
  fn isNull(self, this: &mut QImage) -> i32;
}

// proto: bool QImage::isNull();
impl<'a> /*trait*/ QImage_isNull for () {
  fn isNull(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage6isNullEv()};
    unsafe {_ZNK6QImage6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QImage {
  pub fn byteCount<T: QImage_byteCount>(&mut self, value: T) -> i32 {
    value.byteCount(self);
    return 1;
  }
}

pub trait QImage_byteCount {
  fn byteCount(self, this: &mut QImage) -> i32;
}

// proto: int QImage::byteCount();
impl<'a> /*trait*/ QImage_byteCount for () {
  fn byteCount(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage9byteCountEv()};
    unsafe {_ZNK6QImage9byteCountEv()};
    return 1;
  }
}

// proto: bool QImage::valid(int x, int y);
impl<'a> /*trait*/ QImage_valid for (i32, i32) {
  fn valid(self, this: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5validEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK6QImage5validEii(arg0, arg1)};
    return 1;
  }
}

