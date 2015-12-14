// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrect::QRect;
use super::qstring::QString;
use super::qbytearray::QByteArray;
use super::qpoint::QPoint;
use super::qiodevice::QIODevice;
use super::qstringlist::QStringList;
use super::qpixelformat::QPixelFormat;
use super::qcolor::QColor;
use super::qpaintengine::QPaintEngine;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QImage QImage::copy(const QRect & rect);
  fn _ZNK6QImage4copyERK5QRect(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  uchar * QImage::bits();
  fn _ZN6QImage4bitsEv(qthis: *mut c_void) -> *mut c_uchar;
  // proto:  void QImage::setAlphaChannel(const QImage & alphaChannel);
  fn _ZN6QImage15setAlphaChannelERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QImage::text(const QString & key);
  fn _ZNK6QImage4textERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRect QImage::rect();
  fn _ZNK6QImage4rectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QImage::NewQImage(const char *const [] xpm);
  fn _ZN6QImageC1EPKPKc(qthis: *mut c_void, arg0: *mut *mut c_char) ;
  // proto:  QImage QImage::createHeuristicMask(bool clipTight);
  fn _ZNK6QImage19createHeuristicMaskEb(qthis: *mut c_void, arg0: int8_t) -> *mut c_void;
  // proto:  const uchar * QImage::constBits();
  fn _ZNK6QImage9constBitsEv(qthis: *mut c_void) -> *const c_uchar;
  // proto: static QImage QImage::fromData(const QByteArray & data, const char * format);
  fn _ZN6QImage8fromDataERK10QByteArrayPKc(arg0: *mut c_void, arg1: *const c_char) -> *mut c_void;
  // proto: static QImage QImage::fromData(const uchar * data, int size, const char * format);
  fn _ZN6QImage8fromDataEPKhiPKc(arg0: *const c_uchar, arg1: c_int, arg2: *const c_char) -> *mut c_void;
  // proto:  bool QImage::isDetached();
  fn _ZNK6QImage10isDetachedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QImage::setOffset(const QPoint & );
  fn _ZN6QImage9setOffsetERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QImage::isGrayscale();
  fn _ZNK6QImage11isGrayscaleEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QImage::save(QIODevice * device, const char * format, int quality);
  fn _ZNK6QImage4saveEP9QIODevicePKci(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char, arg2: c_int) -> int8_t;
  // proto:  int QImage::depth();
  fn _ZNK6QImage5depthEv(qthis: *mut c_void) -> c_int;
  // proto:  QImage QImage::alphaChannel();
  fn _ZNK6QImage12alphaChannelEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QImage::hasAlphaChannel();
  fn _ZNK6QImage15hasAlphaChannelEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QImage::loadFromData(const uchar * buf, int len, const char * format);
  fn _ZN6QImage12loadFromDataEPKhiPKc(qthis: *mut c_void, arg0: *const c_uchar, arg1: c_int, arg2: *const c_char) -> int8_t;
  // proto:  int QImage::colorCount();
  fn _ZNK6QImage10colorCountEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QImage::allGray();
  fn _ZNK6QImage7allGrayEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QImage::setColorCount(int );
  fn _ZN6QImage13setColorCountEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  unsigned int QImage::pixel(const QPoint & pt);
  fn _ZNK6QImage5pixelERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> c_uint;
  // proto:  void QImage::setDevicePixelRatio(qreal scaleFactor);
  fn _ZN6QImage19setDevicePixelRatioEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QImage QImage::copy(int x, int y, int w, int h);
  fn _ZNK6QImage4copyEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> *mut c_void;
  // proto:  void QImage::setText(const QString & key, const QString & value);
  fn _ZN6QImage7setTextERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  unsigned int QImage::color(int i);
  fn _ZNK6QImage5colorEi(qthis: *mut c_void, arg0: c_int) -> c_uint;
  // proto:  void QImage::setPixel(const QPoint & pt, uint index_or_rgb);
  fn _ZN6QImage8setPixelERK6QPointj(qthis: *mut c_void, arg0: *mut c_void, arg1: c_uint) ;
  // proto:  QPoint QImage::offset();
  fn _ZNK6QImage6offsetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const uchar * QImage::constScanLine(int );
  fn _ZNK6QImage13constScanLineEi(qthis: *mut c_void, arg0: c_int) -> *const c_uchar;
  // proto:  QStringList QImage::textKeys();
  fn _ZNK6QImage8textKeysEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QImage::dotsPerMeterY();
  fn _ZNK6QImage13dotsPerMeterYEv(qthis: *mut c_void) -> c_int;
  // proto:  void QImage::fill(uint pixel);
  fn _ZN6QImage4fillEj(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  QPixelFormat QImage::pixelFormat();
  fn _ZNK6QImage11pixelFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QImage::dotsPerMeterX();
  fn _ZNK6QImage13dotsPerMeterXEv(qthis: *mut c_void) -> c_int;
  // proto:  void QImage::setDotsPerMeterY(int );
  fn _ZN6QImage16setDotsPerMeterYEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QImage::bitPlaneCount();
  fn _ZNK6QImage13bitPlaneCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QImage::fill(const QColor & color);
  fn _ZN6QImage4fillERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QImage::detach();
  fn _ZN6QImage6detachEv(qthis: *mut c_void) ;
  // proto:  bool QImage::loadFromData(const QByteArray & data, const char * aformat);
  fn _ZN6QImage12loadFromDataERK10QByteArrayPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char) -> int8_t;
  // proto:  void QImage::NewQImage(const QString & fileName, const char * format);
  fn _ZN6QImageC1ERK7QStringPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char) ;
  // proto:  QPaintEngine * QImage::paintEngine();
  fn _ZNK6QImage11paintEngineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QImage::NewQImage(const QImage & );
  fn _ZN6QImageC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QImage::swap(QImage & other);
  fn _ZN6QImage4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QImage::devicePixelRatio();
  fn _ZNK6QImage16devicePixelRatioEv(qthis: *mut c_void) -> c_double;
  // proto:  int QImage::devType();
  fn _ZNK6QImage7devTypeEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QImage::valid(const QPoint & pt);
  fn _ZNK6QImage5validERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  int QImage::pixelIndex(const QPoint & pt);
  fn _ZNK6QImage10pixelIndexERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QImage::setDotsPerMeterX(int );
  fn _ZN6QImage16setDotsPerMeterXEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QImage::setPixel(int x, int y, uint index_or_rgb);
  fn _ZN6QImage8setPixelEiij(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_uint) ;
  // proto:  bool QImage::load(const QString & fileName, const char * format);
  fn _ZN6QImage4loadERK7QStringPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char) -> int8_t;
  // proto:  QVector<QRgb> QImage::colorTable();
  fn _ZNK6QImage10colorTableEv(qthis: *mut c_void) ;
  // proto:  QSize QImage::size();
  fn _ZNK6QImage4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QImage::height();
  fn _ZNK6QImage6heightEv(qthis: *mut c_void) -> c_int;
  // proto:  int QImage::pixelIndex(int x, int y);
  fn _ZNK6QImage10pixelIndexEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> c_int;
  // proto:  int QImage::width();
  fn _ZNK6QImage5widthEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QImage::load(QIODevice * device, const char * format);
  fn _ZN6QImage4loadEP9QIODevicePKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char) -> int8_t;
  // proto:  void QImage::NewQImage();
  fn _ZN6QImageC1Ev(qthis: *mut c_void) ;
  // proto:  uchar * QImage::scanLine(int );
  fn _ZN6QImage8scanLineEi(qthis: *mut c_void, arg0: c_int) -> *mut c_uchar;
  // proto:  int QImage::bytesPerLine();
  fn _ZNK6QImage12bytesPerLineEv(qthis: *mut c_void) -> c_int;
  // proto:  long long QImage::cacheKey();
  fn _ZNK6QImage8cacheKeyEv(qthis: *mut c_void) -> c_longlong;
  // proto:  unsigned int QImage::pixel(int x, int y);
  fn _ZNK6QImage5pixelEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> c_uint;
  // proto:  void QImage::FreeQImage();
  fn _ZN6QImageD0Ev(qthis: *mut c_void) ;
  // proto:  bool QImage::save(const QString & fileName, const char * format, int quality);
  fn _ZNK6QImage4saveERK7QStringPKci(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char, arg2: c_int) -> int8_t;
  // proto:  void QImage::setColor(int i, QRgb c);
  fn _ZN6QImage8setColorEij(qthis: *mut c_void, arg0: c_int, arg1: c_uint) ;
  // proto:  bool QImage::isNull();
  fn _ZNK6QImage6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QImage::byteCount();
  fn _ZNK6QImage9byteCountEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QImage::valid(int x, int y);
  fn _ZNK6QImage5validEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> int8_t;
}

// body block begin
// class sizeof(QImage)=32
pub struct QImage {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QImage {
  pub fn copy<T: QImage_copy>(&mut self, value: T) -> QImage {
    return value.copy(self);
    // return 1;
  }
}

pub trait QImage_copy {
  fn copy(self, rsthis: &mut QImage) -> QImage;
}

// proto:  QImage QImage::copy(const QRect & rect);
impl<'a> /*trait*/ QImage_copy for (&'a  QRect) {
  fn copy(self, rsthis: &mut QImage) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4copyERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QImage4copyERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QImage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn bits<T: QImage_bits>(&mut self, value: T) -> String {
    return value.bits(self);
    // return 1;
  }
}

pub trait QImage_bits {
  fn bits(self, rsthis: &mut QImage) -> String;
}

// proto:  uchar * QImage::bits();
impl<'a> /*trait*/ QImage_bits for () {
  fn bits(self, rsthis: &mut QImage) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4bitsEv()};
    let mut ret = unsafe {_ZN6QImage4bitsEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn setAlphaChannel<T: QImage_setAlphaChannel>(&mut self, value: T)  {
     value.setAlphaChannel(self);
    // return 1;
  }
}

pub trait QImage_setAlphaChannel {
  fn setAlphaChannel(self, rsthis: &mut QImage) ;
}

// proto:  void QImage::setAlphaChannel(const QImage & alphaChannel);
impl<'a> /*trait*/ QImage_setAlphaChannel for (&'a  QImage) {
  fn setAlphaChannel(self, rsthis: &mut QImage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage15setAlphaChannelERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QImage15setAlphaChannelERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn text<T: QImage_text>(&mut self, value: T) -> QString {
    return value.text(self);
    // return 1;
  }
}

pub trait QImage_text {
  fn text(self, rsthis: &mut QImage) -> QString;
}

// proto:  QString QImage::text(const QString & key);
impl<'a> /*trait*/ QImage_text for (&'a  QString) {
  fn text(self, rsthis: &mut QImage) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4textERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QImage4textERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn rect<T: QImage_rect>(&mut self, value: T) -> QRect {
    return value.rect(self);
    // return 1;
  }
}

pub trait QImage_rect {
  fn rect(self, rsthis: &mut QImage) -> QRect;
}

// proto:  QRect QImage::rect();
impl<'a> /*trait*/ QImage_rect for () {
  fn rect(self, rsthis: &mut QImage) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4rectEv()};
    let mut ret = unsafe {_ZNK6QImage4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
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
    let arg0 = 0  as *mut *mut c_char;
    unsafe {_ZN6QImageC1EPKPKc(qthis, arg0)};
    let rsthis = QImage{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn createHeuristicMask<T: QImage_createHeuristicMask>(&mut self, value: T) -> QImage {
    return value.createHeuristicMask(self);
    // return 1;
  }
}

pub trait QImage_createHeuristicMask {
  fn createHeuristicMask(self, rsthis: &mut QImage) -> QImage;
}

// proto:  QImage QImage::createHeuristicMask(bool clipTight);
impl<'a> /*trait*/ QImage_createHeuristicMask for (i8) {
  fn createHeuristicMask(self, rsthis: &mut QImage) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage19createHeuristicMaskEb()};
    let arg0 = self  as int8_t;
    let mut ret = unsafe {_ZNK6QImage19createHeuristicMaskEb(rsthis.qclsinst, arg0)};
    let mut ret1 = QImage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn constBits<T: QImage_constBits>(&mut self, value: T) -> String {
    return value.constBits(self);
    // return 1;
  }
}

pub trait QImage_constBits {
  fn constBits(self, rsthis: &mut QImage) -> String;
}

// proto:  const uchar * QImage::constBits();
impl<'a> /*trait*/ QImage_constBits for () {
  fn constBits(self, rsthis: &mut QImage) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage9constBitsEv()};
    let mut ret = unsafe {_ZNK6QImage9constBitsEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn fromData<T: QImage_fromData>(&mut self, value: T) -> QImage {
    return value.fromData(self);
    // return 1;
  }
}

pub trait QImage_fromData {
  fn fromData(self, rsthis: &mut QImage) -> QImage;
}

// proto: static QImage QImage::fromData(const QByteArray & data, const char * format);
impl<'a> /*trait*/ QImage_fromData for (&'a  QByteArray, &'a  String) {
  fn fromData(self, rsthis: &mut QImage) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage8fromDataERK10QByteArrayPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN6QImage8fromDataERK10QByteArrayPKc(arg0, arg1)};
    let mut ret1 = QImage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QImage QImage::fromData(const uchar * data, int size, const char * format);
impl<'a> /*trait*/ QImage_fromData for (&'a  String, i32, &'a  String) {
  fn fromData(self, rsthis: &mut QImage) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage8fromDataEPKhiPKc()};
    let arg0 = self.0.as_ptr()  as *const c_uchar;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN6QImage8fromDataEPKhiPKc(arg0, arg1, arg2)};
    let mut ret1 = QImage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn isDetached<T: QImage_isDetached>(&mut self, value: T) -> i8 {
    return value.isDetached(self);
    // return 1;
  }
}

pub trait QImage_isDetached {
  fn isDetached(self, rsthis: &mut QImage) -> i8;
}

// proto:  bool QImage::isDetached();
impl<'a> /*trait*/ QImage_isDetached for () {
  fn isDetached(self, rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage10isDetachedEv()};
    let mut ret = unsafe {_ZNK6QImage10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn setOffset<T: QImage_setOffset>(&mut self, value: T)  {
     value.setOffset(self);
    // return 1;
  }
}

pub trait QImage_setOffset {
  fn setOffset(self, rsthis: &mut QImage) ;
}

// proto:  void QImage::setOffset(const QPoint & );
impl<'a> /*trait*/ QImage_setOffset for (&'a  QPoint) {
  fn setOffset(self, rsthis: &mut QImage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage9setOffsetERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QImage9setOffsetERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn isGrayscale<T: QImage_isGrayscale>(&mut self, value: T) -> i8 {
    return value.isGrayscale(self);
    // return 1;
  }
}

pub trait QImage_isGrayscale {
  fn isGrayscale(self, rsthis: &mut QImage) -> i8;
}

// proto:  bool QImage::isGrayscale();
impl<'a> /*trait*/ QImage_isGrayscale for () {
  fn isGrayscale(self, rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage11isGrayscaleEv()};
    let mut ret = unsafe {_ZNK6QImage11isGrayscaleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn save<T: QImage_save>(&mut self, value: T) -> i8 {
    return value.save(self);
    // return 1;
  }
}

pub trait QImage_save {
  fn save(self, rsthis: &mut QImage) -> i8;
}

// proto:  bool QImage::save(QIODevice * device, const char * format, int quality);
impl<'a> /*trait*/ QImage_save for (&'a mut QIODevice, &'a  String, i32) {
  fn save(self, rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4saveEP9QIODevicePKci()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZNK6QImage4saveEP9QIODevicePKci(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn depth<T: QImage_depth>(&mut self, value: T) -> i32 {
    return value.depth(self);
    // return 1;
  }
}

pub trait QImage_depth {
  fn depth(self, rsthis: &mut QImage) -> i32;
}

// proto:  int QImage::depth();
impl<'a> /*trait*/ QImage_depth for () {
  fn depth(self, rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5depthEv()};
    let mut ret = unsafe {_ZNK6QImage5depthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn alphaChannel<T: QImage_alphaChannel>(&mut self, value: T) -> QImage {
    return value.alphaChannel(self);
    // return 1;
  }
}

pub trait QImage_alphaChannel {
  fn alphaChannel(self, rsthis: &mut QImage) -> QImage;
}

// proto:  QImage QImage::alphaChannel();
impl<'a> /*trait*/ QImage_alphaChannel for () {
  fn alphaChannel(self, rsthis: &mut QImage) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage12alphaChannelEv()};
    let mut ret = unsafe {_ZNK6QImage12alphaChannelEv(rsthis.qclsinst)};
    let mut ret1 = QImage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn hasAlphaChannel<T: QImage_hasAlphaChannel>(&mut self, value: T) -> i8 {
    return value.hasAlphaChannel(self);
    // return 1;
  }
}

pub trait QImage_hasAlphaChannel {
  fn hasAlphaChannel(self, rsthis: &mut QImage) -> i8;
}

// proto:  bool QImage::hasAlphaChannel();
impl<'a> /*trait*/ QImage_hasAlphaChannel for () {
  fn hasAlphaChannel(self, rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage15hasAlphaChannelEv()};
    let mut ret = unsafe {_ZNK6QImage15hasAlphaChannelEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn loadFromData<T: QImage_loadFromData>(&mut self, value: T) -> i8 {
    return value.loadFromData(self);
    // return 1;
  }
}

pub trait QImage_loadFromData {
  fn loadFromData(self, rsthis: &mut QImage) -> i8;
}

// proto:  bool QImage::loadFromData(const uchar * buf, int len, const char * format);
impl<'a> /*trait*/ QImage_loadFromData for (&'a  String, i32, &'a  String) {
  fn loadFromData(self, rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage12loadFromDataEPKhiPKc()};
    let arg0 = self.0.as_ptr()  as *const c_uchar;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN6QImage12loadFromDataEPKhiPKc(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn colorCount<T: QImage_colorCount>(&mut self, value: T) -> i32 {
    return value.colorCount(self);
    // return 1;
  }
}

pub trait QImage_colorCount {
  fn colorCount(self, rsthis: &mut QImage) -> i32;
}

// proto:  int QImage::colorCount();
impl<'a> /*trait*/ QImage_colorCount for () {
  fn colorCount(self, rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage10colorCountEv()};
    let mut ret = unsafe {_ZNK6QImage10colorCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn allGray<T: QImage_allGray>(&mut self, value: T) -> i8 {
    return value.allGray(self);
    // return 1;
  }
}

pub trait QImage_allGray {
  fn allGray(self, rsthis: &mut QImage) -> i8;
}

// proto:  bool QImage::allGray();
impl<'a> /*trait*/ QImage_allGray for () {
  fn allGray(self, rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage7allGrayEv()};
    let mut ret = unsafe {_ZNK6QImage7allGrayEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn setColorCount<T: QImage_setColorCount>(&mut self, value: T)  {
     value.setColorCount(self);
    // return 1;
  }
}

pub trait QImage_setColorCount {
  fn setColorCount(self, rsthis: &mut QImage) ;
}

// proto:  void QImage::setColorCount(int );
impl<'a> /*trait*/ QImage_setColorCount for (i32) {
  fn setColorCount(self, rsthis: &mut QImage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage13setColorCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QImage13setColorCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn pixel<T: QImage_pixel>(&mut self, value: T) -> u32 {
    return value.pixel(self);
    // return 1;
  }
}

pub trait QImage_pixel {
  fn pixel(self, rsthis: &mut QImage) -> u32;
}

// proto:  unsigned int QImage::pixel(const QPoint & pt);
impl<'a> /*trait*/ QImage_pixel for (&'a  QPoint) {
  fn pixel(self, rsthis: &mut QImage) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5pixelERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QImage5pixelERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn setDevicePixelRatio<T: QImage_setDevicePixelRatio>(&mut self, value: T)  {
     value.setDevicePixelRatio(self);
    // return 1;
  }
}

pub trait QImage_setDevicePixelRatio {
  fn setDevicePixelRatio(self, rsthis: &mut QImage) ;
}

// proto:  void QImage::setDevicePixelRatio(qreal scaleFactor);
impl<'a> /*trait*/ QImage_setDevicePixelRatio for (f64) {
  fn setDevicePixelRatio(self, rsthis: &mut QImage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage19setDevicePixelRatioEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QImage19setDevicePixelRatioEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QImage QImage::copy(int x, int y, int w, int h);
impl<'a> /*trait*/ QImage_copy for (i32, i32, i32, i32) {
  fn copy(self, rsthis: &mut QImage) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4copyEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZNK6QImage4copyEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QImage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn setText<T: QImage_setText>(&mut self, value: T)  {
     value.setText(self);
    // return 1;
  }
}

pub trait QImage_setText {
  fn setText(self, rsthis: &mut QImage) ;
}

// proto:  void QImage::setText(const QString & key, const QString & value);
impl<'a> /*trait*/ QImage_setText for (&'a  QString, &'a  QString) {
  fn setText(self, rsthis: &mut QImage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage7setTextERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN6QImage7setTextERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn color<T: QImage_color>(&mut self, value: T) -> u32 {
    return value.color(self);
    // return 1;
  }
}

pub trait QImage_color {
  fn color(self, rsthis: &mut QImage) -> u32;
}

// proto:  unsigned int QImage::color(int i);
impl<'a> /*trait*/ QImage_color for (i32) {
  fn color(self, rsthis: &mut QImage) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5colorEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK6QImage5colorEi(rsthis.qclsinst, arg0)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn setPixel<T: QImage_setPixel>(&mut self, value: T)  {
     value.setPixel(self);
    // return 1;
  }
}

pub trait QImage_setPixel {
  fn setPixel(self, rsthis: &mut QImage) ;
}

// proto:  void QImage::setPixel(const QPoint & pt, uint index_or_rgb);
impl<'a> /*trait*/ QImage_setPixel for (&'a  QPoint, u32) {
  fn setPixel(self, rsthis: &mut QImage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage8setPixelERK6QPointj()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN6QImage8setPixelERK6QPointj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn offset<T: QImage_offset>(&mut self, value: T) -> QPoint {
    return value.offset(self);
    // return 1;
  }
}

pub trait QImage_offset {
  fn offset(self, rsthis: &mut QImage) -> QPoint;
}

// proto:  QPoint QImage::offset();
impl<'a> /*trait*/ QImage_offset for () {
  fn offset(self, rsthis: &mut QImage) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage6offsetEv()};
    let mut ret = unsafe {_ZNK6QImage6offsetEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn constScanLine<T: QImage_constScanLine>(&mut self, value: T) -> String {
    return value.constScanLine(self);
    // return 1;
  }
}

pub trait QImage_constScanLine {
  fn constScanLine(self, rsthis: &mut QImage) -> String;
}

// proto:  const uchar * QImage::constScanLine(int );
impl<'a> /*trait*/ QImage_constScanLine for (i32) {
  fn constScanLine(self, rsthis: &mut QImage) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage13constScanLineEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK6QImage13constScanLineEi(rsthis.qclsinst, arg0)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn textKeys<T: QImage_textKeys>(&mut self, value: T) -> QStringList {
    return value.textKeys(self);
    // return 1;
  }
}

pub trait QImage_textKeys {
  fn textKeys(self, rsthis: &mut QImage) -> QStringList;
}

// proto:  QStringList QImage::textKeys();
impl<'a> /*trait*/ QImage_textKeys for () {
  fn textKeys(self, rsthis: &mut QImage) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage8textKeysEv()};
    let mut ret = unsafe {_ZNK6QImage8textKeysEv(rsthis.qclsinst)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn dotsPerMeterY<T: QImage_dotsPerMeterY>(&mut self, value: T) -> i32 {
    return value.dotsPerMeterY(self);
    // return 1;
  }
}

pub trait QImage_dotsPerMeterY {
  fn dotsPerMeterY(self, rsthis: &mut QImage) -> i32;
}

// proto:  int QImage::dotsPerMeterY();
impl<'a> /*trait*/ QImage_dotsPerMeterY for () {
  fn dotsPerMeterY(self, rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage13dotsPerMeterYEv()};
    let mut ret = unsafe {_ZNK6QImage13dotsPerMeterYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn fill<T: QImage_fill>(&mut self, value: T)  {
     value.fill(self);
    // return 1;
  }
}

pub trait QImage_fill {
  fn fill(self, rsthis: &mut QImage) ;
}

// proto:  void QImage::fill(uint pixel);
impl<'a> /*trait*/ QImage_fill for (u32) {
  fn fill(self, rsthis: &mut QImage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4fillEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN6QImage4fillEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn pixelFormat<T: QImage_pixelFormat>(&mut self, value: T) -> QPixelFormat {
    return value.pixelFormat(self);
    // return 1;
  }
}

pub trait QImage_pixelFormat {
  fn pixelFormat(self, rsthis: &mut QImage) -> QPixelFormat;
}

// proto:  QPixelFormat QImage::pixelFormat();
impl<'a> /*trait*/ QImage_pixelFormat for () {
  fn pixelFormat(self, rsthis: &mut QImage) -> QPixelFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage11pixelFormatEv()};
    let mut ret = unsafe {_ZNK6QImage11pixelFormatEv(rsthis.qclsinst)};
    let mut ret1 = QPixelFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn dotsPerMeterX<T: QImage_dotsPerMeterX>(&mut self, value: T) -> i32 {
    return value.dotsPerMeterX(self);
    // return 1;
  }
}

pub trait QImage_dotsPerMeterX {
  fn dotsPerMeterX(self, rsthis: &mut QImage) -> i32;
}

// proto:  int QImage::dotsPerMeterX();
impl<'a> /*trait*/ QImage_dotsPerMeterX for () {
  fn dotsPerMeterX(self, rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage13dotsPerMeterXEv()};
    let mut ret = unsafe {_ZNK6QImage13dotsPerMeterXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn setDotsPerMeterY<T: QImage_setDotsPerMeterY>(&mut self, value: T)  {
     value.setDotsPerMeterY(self);
    // return 1;
  }
}

pub trait QImage_setDotsPerMeterY {
  fn setDotsPerMeterY(self, rsthis: &mut QImage) ;
}

// proto:  void QImage::setDotsPerMeterY(int );
impl<'a> /*trait*/ QImage_setDotsPerMeterY for (i32) {
  fn setDotsPerMeterY(self, rsthis: &mut QImage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage16setDotsPerMeterYEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QImage16setDotsPerMeterYEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn bitPlaneCount<T: QImage_bitPlaneCount>(&mut self, value: T) -> i32 {
    return value.bitPlaneCount(self);
    // return 1;
  }
}

pub trait QImage_bitPlaneCount {
  fn bitPlaneCount(self, rsthis: &mut QImage) -> i32;
}

// proto:  int QImage::bitPlaneCount();
impl<'a> /*trait*/ QImage_bitPlaneCount for () {
  fn bitPlaneCount(self, rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage13bitPlaneCountEv()};
    let mut ret = unsafe {_ZNK6QImage13bitPlaneCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QImage::fill(const QColor & color);
impl<'a> /*trait*/ QImage_fill for (&'a  QColor) {
  fn fill(self, rsthis: &mut QImage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4fillERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QImage4fillERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn detach<T: QImage_detach>(&mut self, value: T)  {
     value.detach(self);
    // return 1;
  }
}

pub trait QImage_detach {
  fn detach(self, rsthis: &mut QImage) ;
}

// proto:  void QImage::detach();
impl<'a> /*trait*/ QImage_detach for () {
  fn detach(self, rsthis: &mut QImage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage6detachEv()};
     unsafe {_ZN6QImage6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  bool QImage::loadFromData(const QByteArray & data, const char * aformat);
impl<'a> /*trait*/ QImage_loadFromData for (&'a  QByteArray, &'a  String) {
  fn loadFromData(self, rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage12loadFromDataERK10QByteArrayPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN6QImage12loadFromDataERK10QByteArrayPKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QImage::NewQImage(const QString & fileName, const char * format);
impl<'a> /*trait*/ QImage_NewQImage for (&'a  QString, &'a  String) {
  fn NewQImage(self) -> QImage {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImageC1ERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN6QImageC1ERK7QStringPKc(qthis, arg0, arg1)};
    let rsthis = QImage{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn paintEngine<T: QImage_paintEngine>(&mut self, value: T) -> QPaintEngine {
    return value.paintEngine(self);
    // return 1;
  }
}

pub trait QImage_paintEngine {
  fn paintEngine(self, rsthis: &mut QImage) -> QPaintEngine;
}

// proto:  QPaintEngine * QImage::paintEngine();
impl<'a> /*trait*/ QImage_paintEngine for () {
  fn paintEngine(self, rsthis: &mut QImage) -> QPaintEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage11paintEngineEv()};
    let mut ret = unsafe {_ZNK6QImage11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QImage::NewQImage(const QImage & );
impl<'a> /*trait*/ QImage_NewQImage for (&'a  QImage) {
  fn NewQImage(self) -> QImage {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImageC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QImageC1ERKS_(qthis, arg0)};
    let rsthis = QImage{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn swap<T: QImage_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QImage_swap {
  fn swap(self, rsthis: &mut QImage) ;
}

// proto:  void QImage::swap(QImage & other);
impl<'a> /*trait*/ QImage_swap for (&'a mut QImage) {
  fn swap(self, rsthis: &mut QImage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QImage4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn devicePixelRatio<T: QImage_devicePixelRatio>(&mut self, value: T) -> f64 {
    return value.devicePixelRatio(self);
    // return 1;
  }
}

pub trait QImage_devicePixelRatio {
  fn devicePixelRatio(self, rsthis: &mut QImage) -> f64;
}

// proto:  double QImage::devicePixelRatio();
impl<'a> /*trait*/ QImage_devicePixelRatio for () {
  fn devicePixelRatio(self, rsthis: &mut QImage) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage16devicePixelRatioEv()};
    let mut ret = unsafe {_ZNK6QImage16devicePixelRatioEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn devType<T: QImage_devType>(&mut self, value: T) -> i32 {
    return value.devType(self);
    // return 1;
  }
}

pub trait QImage_devType {
  fn devType(self, rsthis: &mut QImage) -> i32;
}

// proto:  int QImage::devType();
impl<'a> /*trait*/ QImage_devType for () {
  fn devType(self, rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage7devTypeEv()};
    let mut ret = unsafe {_ZNK6QImage7devTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn valid<T: QImage_valid>(&mut self, value: T) -> i8 {
    return value.valid(self);
    // return 1;
  }
}

pub trait QImage_valid {
  fn valid(self, rsthis: &mut QImage) -> i8;
}

// proto:  bool QImage::valid(const QPoint & pt);
impl<'a> /*trait*/ QImage_valid for (&'a  QPoint) {
  fn valid(self, rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5validERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QImage5validERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn pixelIndex<T: QImage_pixelIndex>(&mut self, value: T) -> i32 {
    return value.pixelIndex(self);
    // return 1;
  }
}

pub trait QImage_pixelIndex {
  fn pixelIndex(self, rsthis: &mut QImage) -> i32;
}

// proto:  int QImage::pixelIndex(const QPoint & pt);
impl<'a> /*trait*/ QImage_pixelIndex for (&'a  QPoint) {
  fn pixelIndex(self, rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage10pixelIndexERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QImage10pixelIndexERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn setDotsPerMeterX<T: QImage_setDotsPerMeterX>(&mut self, value: T)  {
     value.setDotsPerMeterX(self);
    // return 1;
  }
}

pub trait QImage_setDotsPerMeterX {
  fn setDotsPerMeterX(self, rsthis: &mut QImage) ;
}

// proto:  void QImage::setDotsPerMeterX(int );
impl<'a> /*trait*/ QImage_setDotsPerMeterX for (i32) {
  fn setDotsPerMeterX(self, rsthis: &mut QImage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage16setDotsPerMeterXEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QImage16setDotsPerMeterXEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QImage::setPixel(int x, int y, uint index_or_rgb);
impl<'a> /*trait*/ QImage_setPixel for (i32, i32, u32) {
  fn setPixel(self, rsthis: &mut QImage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage8setPixelEiij()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uint;
     unsafe {_ZN6QImage8setPixelEiij(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn load<T: QImage_load>(&mut self, value: T) -> i8 {
    return value.load(self);
    // return 1;
  }
}

pub trait QImage_load {
  fn load(self, rsthis: &mut QImage) -> i8;
}

// proto:  bool QImage::load(const QString & fileName, const char * format);
impl<'a> /*trait*/ QImage_load for (&'a  QString, &'a  String) {
  fn load(self, rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4loadERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN6QImage4loadERK7QStringPKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn colorTable<T: QImage_colorTable>(&mut self, value: T)  {
     value.colorTable(self);
    // return 1;
  }
}

pub trait QImage_colorTable {
  fn colorTable(self, rsthis: &mut QImage) ;
}

// proto:  QVector<QRgb> QImage::colorTable();
impl<'a> /*trait*/ QImage_colorTable for () {
  fn colorTable(self, rsthis: &mut QImage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage10colorTableEv()};
     unsafe {_ZNK6QImage10colorTableEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn size<T: QImage_size>(&mut self, value: T) -> QSize {
    return value.size(self);
    // return 1;
  }
}

pub trait QImage_size {
  fn size(self, rsthis: &mut QImage) -> QSize;
}

// proto:  QSize QImage::size();
impl<'a> /*trait*/ QImage_size for () {
  fn size(self, rsthis: &mut QImage) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4sizeEv()};
    let mut ret = unsafe {_ZNK6QImage4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn height<T: QImage_height>(&mut self, value: T) -> i32 {
    return value.height(self);
    // return 1;
  }
}

pub trait QImage_height {
  fn height(self, rsthis: &mut QImage) -> i32;
}

// proto:  int QImage::height();
impl<'a> /*trait*/ QImage_height for () {
  fn height(self, rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage6heightEv()};
    let mut ret = unsafe {_ZNK6QImage6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QImage::pixelIndex(int x, int y);
impl<'a> /*trait*/ QImage_pixelIndex for (i32, i32) {
  fn pixelIndex(self, rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage10pixelIndexEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK6QImage10pixelIndexEii(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn width<T: QImage_width>(&mut self, value: T) -> i32 {
    return value.width(self);
    // return 1;
  }
}

pub trait QImage_width {
  fn width(self, rsthis: &mut QImage) -> i32;
}

// proto:  int QImage::width();
impl<'a> /*trait*/ QImage_width for () {
  fn width(self, rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5widthEv()};
    let mut ret = unsafe {_ZNK6QImage5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  bool QImage::load(QIODevice * device, const char * format);
impl<'a> /*trait*/ QImage_load for (&'a mut QIODevice, &'a  String) {
  fn load(self, rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4loadEP9QIODevicePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN6QImage4loadEP9QIODevicePKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
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
  pub fn scanLine<T: QImage_scanLine>(&mut self, value: T) -> String {
    return value.scanLine(self);
    // return 1;
  }
}

pub trait QImage_scanLine {
  fn scanLine(self, rsthis: &mut QImage) -> String;
}

// proto:  uchar * QImage::scanLine(int );
impl<'a> /*trait*/ QImage_scanLine for (i32) {
  fn scanLine(self, rsthis: &mut QImage) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage8scanLineEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN6QImage8scanLineEi(rsthis.qclsinst, arg0)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn bytesPerLine<T: QImage_bytesPerLine>(&mut self, value: T) -> i32 {
    return value.bytesPerLine(self);
    // return 1;
  }
}

pub trait QImage_bytesPerLine {
  fn bytesPerLine(self, rsthis: &mut QImage) -> i32;
}

// proto:  int QImage::bytesPerLine();
impl<'a> /*trait*/ QImage_bytesPerLine for () {
  fn bytesPerLine(self, rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage12bytesPerLineEv()};
    let mut ret = unsafe {_ZNK6QImage12bytesPerLineEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn cacheKey<T: QImage_cacheKey>(&mut self, value: T) -> i64 {
    return value.cacheKey(self);
    // return 1;
  }
}

pub trait QImage_cacheKey {
  fn cacheKey(self, rsthis: &mut QImage) -> i64;
}

// proto:  long long QImage::cacheKey();
impl<'a> /*trait*/ QImage_cacheKey for () {
  fn cacheKey(self, rsthis: &mut QImage) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage8cacheKeyEv()};
    let mut ret = unsafe {_ZNK6QImage8cacheKeyEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

// proto:  unsigned int QImage::pixel(int x, int y);
impl<'a> /*trait*/ QImage_pixel for (i32, i32) {
  fn pixel(self, rsthis: &mut QImage) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5pixelEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK6QImage5pixelEii(rsthis.qclsinst, arg0, arg1)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn FreeQImage<T: QImage_FreeQImage>(&mut self, value: T)  {
     value.FreeQImage(self);
    // return 1;
  }
}

pub trait QImage_FreeQImage {
  fn FreeQImage(self, rsthis: &mut QImage) ;
}

// proto:  void QImage::FreeQImage();
impl<'a> /*trait*/ QImage_FreeQImage for () {
  fn FreeQImage(self, rsthis: &mut QImage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImageD0Ev()};
     unsafe {_ZN6QImageD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  bool QImage::save(const QString & fileName, const char * format, int quality);
impl<'a> /*trait*/ QImage_save for (&'a  QString, &'a  String, i32) {
  fn save(self, rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4saveERK7QStringPKci()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZNK6QImage4saveERK7QStringPKci(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn setColor<T: QImage_setColor>(&mut self, value: T)  {
     value.setColor(self);
    // return 1;
  }
}

pub trait QImage_setColor {
  fn setColor(self, rsthis: &mut QImage) ;
}

// proto:  void QImage::setColor(int i, QRgb c);
impl<'a> /*trait*/ QImage_setColor for (i32, u32) {
  fn setColor(self, rsthis: &mut QImage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage8setColorEij()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN6QImage8setColorEij(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn isNull<T: QImage_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QImage_isNull {
  fn isNull(self, rsthis: &mut QImage) -> i8;
}

// proto:  bool QImage::isNull();
impl<'a> /*trait*/ QImage_isNull for () {
  fn isNull(self, rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage6isNullEv()};
    let mut ret = unsafe {_ZNK6QImage6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImage {
  pub fn byteCount<T: QImage_byteCount>(&mut self, value: T) -> i32 {
    return value.byteCount(self);
    // return 1;
  }
}

pub trait QImage_byteCount {
  fn byteCount(self, rsthis: &mut QImage) -> i32;
}

// proto:  int QImage::byteCount();
impl<'a> /*trait*/ QImage_byteCount for () {
  fn byteCount(self, rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage9byteCountEv()};
    let mut ret = unsafe {_ZNK6QImage9byteCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  bool QImage::valid(int x, int y);
impl<'a> /*trait*/ QImage_valid for (i32, i32) {
  fn valid(self, rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5validEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK6QImage5validEii(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

