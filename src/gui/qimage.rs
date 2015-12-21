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
use super::qsize::QSize;
use super::qstring::QString;
use super::qbytearray::QByteArray;
use super::qpoint::QPoint;
use super::qmatrix::QMatrix;
use super::qiodevice::QIODevice;
use super::qpixelformat::QPixelFormat;
use super::qcolor::QColor;
use super::qpaintengine::QPaintEngine;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QImage QImage::copy(const QRect & rect);
  fn _ZNK6QImage4copyERK5QRect(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto: static QTransform QImage::trueMatrix(const QTransform & , int w, int h);
  fn _ZN6QImage10trueMatrixERK10QTransformii(arg0: *mut c_void, arg1: c_int, arg2: c_int) -> *mut c_void;
  // proto:  uchar * QImage::bits();
  fn _ZN6QImage4bitsEv(qthis: *mut c_void) -> *mut c_uchar;
  // proto:  void QImage::setAlphaChannel(const QImage & alphaChannel);
  fn _ZN6QImage15setAlphaChannelERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QImage::text(const QString & key);
  fn _ZNK6QImage4textERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRect QImage::rect();
  fn _ZNK6QImage4rectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QImage::QImage(const char *const [] xpm);
  fn _ZN6QImageC1EPKPKc(qthis: *mut c_void, arg0: *mut *mut c_char);
  // proto:  QImage QImage::createHeuristicMask(bool clipTight);
  fn _ZNK6QImage19createHeuristicMaskEb(qthis: *mut c_void, arg0: c_char) -> *mut c_void;
  // proto:  const uchar * QImage::constBits();
  fn _ZNK6QImage9constBitsEv(qthis: *mut c_void) -> *mut c_uchar;
  // proto: static QImage QImage::fromData(const QByteArray & data, const char * format);
  fn _ZN6QImage8fromDataERK10QByteArrayPKc(arg0: *mut c_void, arg1: *mut c_char) -> *mut c_void;
  // proto: static QImage QImage::fromData(const uchar * data, int size, const char * format);
  fn _ZN6QImage8fromDataEPKhiPKc(arg0: *mut c_uchar, arg1: c_int, arg2: *mut c_char) -> *mut c_void;
  // proto:  bool QImage::isDetached();
  fn _ZNK6QImage10isDetachedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QImage::setOffset(const QPoint & );
  fn _ZN6QImage9setOffsetERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QMatrix QImage::trueMatrix(const QMatrix & , int w, int h);
  fn _ZN6QImage10trueMatrixERK7QMatrixii(arg0: *mut c_void, arg1: c_int, arg2: c_int) -> *mut c_void;
  // proto:  bool QImage::isGrayscale();
  fn _ZNK6QImage11isGrayscaleEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QImage::save(QIODevice * device, const char * format, int quality);
  fn _ZNK6QImage4saveEP9QIODevicePKci(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char, arg2: c_int) -> c_char;
  // proto:  int QImage::depth();
  fn _ZNK6QImage5depthEv(qthis: *mut c_void) -> c_int;
  // proto:  QImage QImage::alphaChannel();
  fn _ZNK6QImage12alphaChannelEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QImage::hasAlphaChannel();
  fn _ZNK6QImage15hasAlphaChannelEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QImage::loadFromData(const uchar * buf, int len, const char * format);
  fn _ZN6QImage12loadFromDataEPKhiPKc(qthis: *mut c_void, arg0: *mut c_uchar, arg1: c_int, arg2: *mut c_char) -> c_char;
  // proto:  int QImage::colorCount();
  fn _ZNK6QImage10colorCountEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QImage::allGray();
  fn _ZNK6QImage7allGrayEv(qthis: *mut c_void) -> c_char;
  // proto:  void QImage::setColorCount(int );
  fn _ZN6QImage13setColorCountEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QRgb QImage::pixel(const QPoint & pt);
  fn _ZNK6QImage5pixelERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> c_uint;
  // proto:  void QImage::setDevicePixelRatio(qreal scaleFactor);
  fn _ZN6QImage19setDevicePixelRatioEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QImage QImage::copy(int x, int y, int w, int h);
  fn _ZNK6QImage4copyEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> *mut c_void;
  // proto:  void QImage::setText(const QString & key, const QString & value);
  fn _ZN6QImage7setTextERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QRgb QImage::color(int i);
  fn _ZNK6QImage5colorEi(qthis: *mut c_void, arg0: c_int) -> c_uint;
  // proto:  void QImage::setPixel(const QPoint & pt, uint index_or_rgb);
  fn _ZN6QImage8setPixelERK6QPointj(qthis: *mut c_void, arg0: *mut c_void, arg1: c_uint);
  // proto:  QPoint QImage::offset();
  fn _ZNK6QImage6offsetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const uchar * QImage::constScanLine(int );
  fn _ZNK6QImage13constScanLineEi(qthis: *mut c_void, arg0: c_int) -> *mut c_uchar;
  // proto:  QStringList QImage::textKeys();
  fn _ZNK6QImage8textKeysEv(qthis: *mut c_void);
  // proto:  int QImage::dotsPerMeterY();
  fn _ZNK6QImage13dotsPerMeterYEv(qthis: *mut c_void) -> c_int;
  // proto:  void QImage::fill(uint pixel);
  fn _ZN6QImage4fillEj(qthis: *mut c_void, arg0: c_uint);
  // proto:  QPixelFormat QImage::pixelFormat();
  fn _ZNK6QImage11pixelFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QImage::dotsPerMeterX();
  fn _ZNK6QImage13dotsPerMeterXEv(qthis: *mut c_void) -> c_int;
  // proto:  void QImage::setDotsPerMeterY(int );
  fn _ZN6QImage16setDotsPerMeterYEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QImage::bitPlaneCount();
  fn _ZNK6QImage13bitPlaneCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QImage::fill(const QColor & color);
  fn _ZN6QImage4fillERK6QColor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QImage::detach();
  fn _ZN6QImage6detachEv(qthis: *mut c_void);
  // proto:  bool QImage::loadFromData(const QByteArray & data, const char * aformat);
  fn _ZN6QImage12loadFromDataERK10QByteArrayPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  void QImage::QImage(const QString & fileName, const char * format);
  fn _ZN6QImageC1ERK7QStringPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char);
  // proto:  QPaintEngine * QImage::paintEngine();
  fn _ZNK6QImage11paintEngineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QImage::QImage(const QImage & );
  fn _ZN6QImageC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QImage::swap(QImage & other);
  fn _ZN6QImage4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QImage::devicePixelRatio();
  fn _ZNK6QImage16devicePixelRatioEv(qthis: *mut c_void) -> c_double;
  // proto:  int QImage::devType();
  fn _ZNK6QImage7devTypeEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QImage::valid(const QPoint & pt);
  fn _ZNK6QImage5validERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  int QImage::pixelIndex(const QPoint & pt);
  fn _ZNK6QImage10pixelIndexERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QImage::setDotsPerMeterX(int );
  fn _ZN6QImage16setDotsPerMeterXEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QImage::setPixel(int x, int y, uint index_or_rgb);
  fn _ZN6QImage8setPixelEiij(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_uint);
  // proto:  bool QImage::load(const QString & fileName, const char * format);
  fn _ZN6QImage4loadERK7QStringPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  QVector<QRgb> QImage::colorTable();
  fn _ZNK6QImage10colorTableEv(qthis: *mut c_void);
  // proto:  QSize QImage::size();
  fn _ZNK6QImage4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QImage::height();
  fn _ZNK6QImage6heightEv(qthis: *mut c_void) -> c_int;
  // proto:  int QImage::pixelIndex(int x, int y);
  fn _ZNK6QImage10pixelIndexEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> c_int;
  // proto:  int QImage::width();
  fn _ZNK6QImage5widthEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QImage::load(QIODevice * device, const char * format);
  fn _ZN6QImage4loadEP9QIODevicePKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  void QImage::QImage();
  fn _ZN6QImageC1Ev(qthis: *mut c_void);
  // proto:  uchar * QImage::scanLine(int );
  fn _ZN6QImage8scanLineEi(qthis: *mut c_void, arg0: c_int) -> *mut c_uchar;
  // proto:  int QImage::bytesPerLine();
  fn _ZNK6QImage12bytesPerLineEv(qthis: *mut c_void) -> c_int;
  // proto:  qint64 QImage::cacheKey();
  fn _ZNK6QImage8cacheKeyEv(qthis: *mut c_void) -> c_longlong;
  // proto:  QRgb QImage::pixel(int x, int y);
  fn _ZNK6QImage5pixelEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> c_uint;
  // proto:  void QImage::~QImage();
  fn _ZN6QImageD0Ev(qthis: *mut c_void);
  // proto:  bool QImage::save(const QString & fileName, const char * format, int quality);
  fn _ZNK6QImage4saveERK7QStringPKci(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char, arg2: c_int) -> c_char;
  // proto:  void QImage::setColor(int i, QRgb c);
  fn _ZN6QImage8setColorEij(qthis: *mut c_void, arg0: c_int, arg1: c_uint);
  // proto:  bool QImage::isNull();
  fn _ZNK6QImage6isNullEv(qthis: *mut c_void) -> c_char;
  // proto:  int QImage::byteCount();
  fn _ZNK6QImage9byteCountEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QImage::valid(int x, int y);
  fn _ZNK6QImage5validEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> c_char;
}

// body block begin
// class sizeof(QImage)=32
pub struct QImage {
  pub qclsinst: *mut c_void,
}

  // proto:  QImage QImage::copy(const QRect & rect);
impl /*struct*/ QImage {
  pub fn copy<RetType, T: QImage_copy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.copy(self);
    // return 1;
  }
}

pub trait QImage_copy<RetType> {
  fn copy(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  QImage QImage::copy(const QRect & rect);
impl<'a> /*trait*/ QImage_copy<QImage> for (QRect) {
  fn copy(self , rsthis: &mut QImage) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4copyERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QImage4copyERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QImage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static QTransform QImage::trueMatrix(const QTransform & , int w, int h);
impl /*struct*/ QImage {
  pub fn trueMatrix_s<RetType, T: QImage_trueMatrix_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.trueMatrix_s();
    // return 1;
  }
}

pub trait QImage_trueMatrix_s<RetType> {
  fn trueMatrix_s(self ) -> RetType;
}

  // proto: static QTransform QImage::trueMatrix(const QTransform & , int w, int h);
impl<'a> /*trait*/ QImage_trueMatrix_s<QTransform> for (QTransform, i32, i32) {
  fn trueMatrix_s(self ) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage10trueMatrixERK10QTransformii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN6QImage10trueMatrixERK10QTransformii(arg0, arg1, arg2)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  uchar * QImage::bits();
impl /*struct*/ QImage {
  pub fn bits<RetType, T: QImage_bits<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bits(self);
    // return 1;
  }
}

pub trait QImage_bits<RetType> {
  fn bits(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  uchar * QImage::bits();
impl<'a> /*trait*/ QImage_bits<String> for () {
  fn bits(self , rsthis: &mut QImage) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4bitsEv()};
    let mut ret = unsafe {_ZN6QImage4bitsEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  void QImage::setAlphaChannel(const QImage & alphaChannel);
impl /*struct*/ QImage {
  pub fn setAlphaChannel<RetType, T: QImage_setAlphaChannel<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAlphaChannel(self);
    // return 1;
  }
}

pub trait QImage_setAlphaChannel<RetType> {
  fn setAlphaChannel(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  void QImage::setAlphaChannel(const QImage & alphaChannel);
impl<'a> /*trait*/ QImage_setAlphaChannel<()> for (QImage) {
  fn setAlphaChannel(self , rsthis: &mut QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage15setAlphaChannelERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QImage15setAlphaChannelERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QImage::text(const QString & key);
impl /*struct*/ QImage {
  pub fn text<RetType, T: QImage_text<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QImage_text<RetType> {
  fn text(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  QString QImage::text(const QString & key);
impl<'a> /*trait*/ QImage_text<QString> for (QString) {
  fn text(self , rsthis: &mut QImage) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4textERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QImage4textERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QImage::rect();
impl /*struct*/ QImage {
  pub fn rect<RetType, T: QImage_rect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QImage_rect<RetType> {
  fn rect(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  QRect QImage::rect();
impl<'a> /*trait*/ QImage_rect<QRect> for () {
  fn rect(self , rsthis: &mut QImage) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4rectEv()};
    let mut ret = unsafe {_ZNK6QImage4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QImage::QImage(const char *const [] xpm);
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

  // proto:  void QImage::QImage(const char *const [] xpm);
impl<'a> /*trait*/ QImage_NewQImage for (&'a  Vec<&'a  i8>) {
  fn NewQImage(self) -> QImage {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImageC1EPKPKc()};
    let arg0 = self.as_ptr()  as *mut *mut c_char;
    unsafe {_ZN6QImageC1EPKPKc(qthis, arg0)};
    let rsthis = QImage{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QImage QImage::createHeuristicMask(bool clipTight);
impl /*struct*/ QImage {
  pub fn createHeuristicMask<RetType, T: QImage_createHeuristicMask<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.createHeuristicMask(self);
    // return 1;
  }
}

pub trait QImage_createHeuristicMask<RetType> {
  fn createHeuristicMask(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  QImage QImage::createHeuristicMask(bool clipTight);
impl<'a> /*trait*/ QImage_createHeuristicMask<QImage> for (i8) {
  fn createHeuristicMask(self , rsthis: &mut QImage) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage19createHeuristicMaskEb()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZNK6QImage19createHeuristicMaskEb(rsthis.qclsinst, arg0)};
    let mut ret1 = QImage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const uchar * QImage::constBits();
impl /*struct*/ QImage {
  pub fn constBits<RetType, T: QImage_constBits<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.constBits(self);
    // return 1;
  }
}

pub trait QImage_constBits<RetType> {
  fn constBits(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  const uchar * QImage::constBits();
impl<'a> /*trait*/ QImage_constBits<String> for () {
  fn constBits(self , rsthis: &mut QImage) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage9constBitsEv()};
    let mut ret = unsafe {_ZNK6QImage9constBitsEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto: static QImage QImage::fromData(const QByteArray & data, const char * format);
impl /*struct*/ QImage {
  pub fn fromData_s<RetType, T: QImage_fromData_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromData_s();
    // return 1;
  }
}

pub trait QImage_fromData_s<RetType> {
  fn fromData_s(self ) -> RetType;
}

  // proto: static QImage QImage::fromData(const QByteArray & data, const char * format);
impl<'a> /*trait*/ QImage_fromData_s<QImage> for (QByteArray, &'a  String) {
  fn fromData_s(self ) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage8fromDataERK10QByteArrayPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN6QImage8fromDataERK10QByteArrayPKc(arg0, arg1)};
    let mut ret1 = QImage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static QImage QImage::fromData(const uchar * data, int size, const char * format);
impl<'a> /*trait*/ QImage_fromData_s<QImage> for (&'a  String, i32, &'a  String) {
  fn fromData_s(self ) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage8fromDataEPKhiPKc()};
    let arg0 = self.0.as_ptr()  as *mut c_uchar;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN6QImage8fromDataEPKhiPKc(arg0, arg1, arg2)};
    let mut ret1 = QImage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QImage::isDetached();
impl /*struct*/ QImage {
  pub fn isDetached<RetType, T: QImage_isDetached<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isDetached(self);
    // return 1;
  }
}

pub trait QImage_isDetached<RetType> {
  fn isDetached(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  bool QImage::isDetached();
impl<'a> /*trait*/ QImage_isDetached<i8> for () {
  fn isDetached(self , rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage10isDetachedEv()};
    let mut ret = unsafe {_ZNK6QImage10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QImage::setOffset(const QPoint & );
impl /*struct*/ QImage {
  pub fn setOffset<RetType, T: QImage_setOffset<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setOffset(self);
    // return 1;
  }
}

pub trait QImage_setOffset<RetType> {
  fn setOffset(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  void QImage::setOffset(const QPoint & );
impl<'a> /*trait*/ QImage_setOffset<()> for (QPoint) {
  fn setOffset(self , rsthis: &mut QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage9setOffsetERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QImage9setOffsetERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QMatrix QImage::trueMatrix(const QMatrix & , int w, int h);
impl<'a> /*trait*/ QImage_trueMatrix_s<QMatrix> for (QMatrix, i32, i32) {
  fn trueMatrix_s(self ) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage10trueMatrixERK7QMatrixii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN6QImage10trueMatrixERK7QMatrixii(arg0, arg1, arg2)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QImage::isGrayscale();
impl /*struct*/ QImage {
  pub fn isGrayscale<RetType, T: QImage_isGrayscale<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isGrayscale(self);
    // return 1;
  }
}

pub trait QImage_isGrayscale<RetType> {
  fn isGrayscale(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  bool QImage::isGrayscale();
impl<'a> /*trait*/ QImage_isGrayscale<i8> for () {
  fn isGrayscale(self , rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage11isGrayscaleEv()};
    let mut ret = unsafe {_ZNK6QImage11isGrayscaleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QImage::save(QIODevice * device, const char * format, int quality);
impl /*struct*/ QImage {
  pub fn save<RetType, T: QImage_save<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.save(self);
    // return 1;
  }
}

pub trait QImage_save<RetType> {
  fn save(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  bool QImage::save(QIODevice * device, const char * format, int quality);
impl<'a> /*trait*/ QImage_save<i8> for (QIODevice, &'a  String, i32) {
  fn save(self , rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4saveEP9QIODevicePKci()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZNK6QImage4saveEP9QIODevicePKci(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QImage::depth();
impl /*struct*/ QImage {
  pub fn depth<RetType, T: QImage_depth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.depth(self);
    // return 1;
  }
}

pub trait QImage_depth<RetType> {
  fn depth(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  int QImage::depth();
impl<'a> /*trait*/ QImage_depth<i32> for () {
  fn depth(self , rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5depthEv()};
    let mut ret = unsafe {_ZNK6QImage5depthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QImage QImage::alphaChannel();
impl /*struct*/ QImage {
  pub fn alphaChannel<RetType, T: QImage_alphaChannel<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.alphaChannel(self);
    // return 1;
  }
}

pub trait QImage_alphaChannel<RetType> {
  fn alphaChannel(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  QImage QImage::alphaChannel();
impl<'a> /*trait*/ QImage_alphaChannel<QImage> for () {
  fn alphaChannel(self , rsthis: &mut QImage) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage12alphaChannelEv()};
    let mut ret = unsafe {_ZNK6QImage12alphaChannelEv(rsthis.qclsinst)};
    let mut ret1 = QImage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QImage::hasAlphaChannel();
impl /*struct*/ QImage {
  pub fn hasAlphaChannel<RetType, T: QImage_hasAlphaChannel<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasAlphaChannel(self);
    // return 1;
  }
}

pub trait QImage_hasAlphaChannel<RetType> {
  fn hasAlphaChannel(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  bool QImage::hasAlphaChannel();
impl<'a> /*trait*/ QImage_hasAlphaChannel<i8> for () {
  fn hasAlphaChannel(self , rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage15hasAlphaChannelEv()};
    let mut ret = unsafe {_ZNK6QImage15hasAlphaChannelEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QImage::loadFromData(const uchar * buf, int len, const char * format);
impl /*struct*/ QImage {
  pub fn loadFromData<RetType, T: QImage_loadFromData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.loadFromData(self);
    // return 1;
  }
}

pub trait QImage_loadFromData<RetType> {
  fn loadFromData(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  bool QImage::loadFromData(const uchar * buf, int len, const char * format);
impl<'a> /*trait*/ QImage_loadFromData<i8> for (&'a  String, i32, &'a  String) {
  fn loadFromData(self , rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage12loadFromDataEPKhiPKc()};
    let arg0 = self.0.as_ptr()  as *mut c_uchar;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN6QImage12loadFromDataEPKhiPKc(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QImage::colorCount();
impl /*struct*/ QImage {
  pub fn colorCount<RetType, T: QImage_colorCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.colorCount(self);
    // return 1;
  }
}

pub trait QImage_colorCount<RetType> {
  fn colorCount(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  int QImage::colorCount();
impl<'a> /*trait*/ QImage_colorCount<i32> for () {
  fn colorCount(self , rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage10colorCountEv()};
    let mut ret = unsafe {_ZNK6QImage10colorCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QImage::allGray();
impl /*struct*/ QImage {
  pub fn allGray<RetType, T: QImage_allGray<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.allGray(self);
    // return 1;
  }
}

pub trait QImage_allGray<RetType> {
  fn allGray(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  bool QImage::allGray();
impl<'a> /*trait*/ QImage_allGray<i8> for () {
  fn allGray(self , rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage7allGrayEv()};
    let mut ret = unsafe {_ZNK6QImage7allGrayEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QImage::setColorCount(int );
impl /*struct*/ QImage {
  pub fn setColorCount<RetType, T: QImage_setColorCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setColorCount(self);
    // return 1;
  }
}

pub trait QImage_setColorCount<RetType> {
  fn setColorCount(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  void QImage::setColorCount(int );
impl<'a> /*trait*/ QImage_setColorCount<()> for (i32) {
  fn setColorCount(self , rsthis: &mut QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage13setColorCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QImage13setColorCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRgb QImage::pixel(const QPoint & pt);
impl /*struct*/ QImage {
  pub fn pixel<RetType, T: QImage_pixel<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pixel(self);
    // return 1;
  }
}

pub trait QImage_pixel<RetType> {
  fn pixel(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  QRgb QImage::pixel(const QPoint & pt);
impl<'a> /*trait*/ QImage_pixel<u32> for (QPoint) {
  fn pixel(self , rsthis: &mut QImage) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5pixelERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QImage5pixelERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  void QImage::setDevicePixelRatio(qreal scaleFactor);
impl /*struct*/ QImage {
  pub fn setDevicePixelRatio<RetType, T: QImage_setDevicePixelRatio<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDevicePixelRatio(self);
    // return 1;
  }
}

pub trait QImage_setDevicePixelRatio<RetType> {
  fn setDevicePixelRatio(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  void QImage::setDevicePixelRatio(qreal scaleFactor);
impl<'a> /*trait*/ QImage_setDevicePixelRatio<()> for (f64) {
  fn setDevicePixelRatio(self , rsthis: &mut QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage19setDevicePixelRatioEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QImage19setDevicePixelRatioEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QImage QImage::copy(int x, int y, int w, int h);
impl<'a> /*trait*/ QImage_copy<QImage> for (i32, i32, i32, i32) {
  fn copy(self , rsthis: &mut QImage) -> QImage {
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

  // proto:  void QImage::setText(const QString & key, const QString & value);
impl /*struct*/ QImage {
  pub fn setText<RetType, T: QImage_setText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QImage_setText<RetType> {
  fn setText(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  void QImage::setText(const QString & key, const QString & value);
impl<'a> /*trait*/ QImage_setText<()> for (QString, QString) {
  fn setText(self , rsthis: &mut QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage7setTextERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN6QImage7setTextERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QRgb QImage::color(int i);
impl /*struct*/ QImage {
  pub fn color<RetType, T: QImage_color<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.color(self);
    // return 1;
  }
}

pub trait QImage_color<RetType> {
  fn color(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  QRgb QImage::color(int i);
impl<'a> /*trait*/ QImage_color<u32> for (i32) {
  fn color(self , rsthis: &mut QImage) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5colorEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK6QImage5colorEi(rsthis.qclsinst, arg0)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  void QImage::setPixel(const QPoint & pt, uint index_or_rgb);
impl /*struct*/ QImage {
  pub fn setPixel<RetType, T: QImage_setPixel<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPixel(self);
    // return 1;
  }
}

pub trait QImage_setPixel<RetType> {
  fn setPixel(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  void QImage::setPixel(const QPoint & pt, uint index_or_rgb);
impl<'a> /*trait*/ QImage_setPixel<()> for (QPoint, u32) {
  fn setPixel(self , rsthis: &mut QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage8setPixelERK6QPointj()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN6QImage8setPixelERK6QPointj(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QPoint QImage::offset();
impl /*struct*/ QImage {
  pub fn offset<RetType, T: QImage_offset<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.offset(self);
    // return 1;
  }
}

pub trait QImage_offset<RetType> {
  fn offset(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  QPoint QImage::offset();
impl<'a> /*trait*/ QImage_offset<QPoint> for () {
  fn offset(self , rsthis: &mut QImage) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage6offsetEv()};
    let mut ret = unsafe {_ZNK6QImage6offsetEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const uchar * QImage::constScanLine(int );
impl /*struct*/ QImage {
  pub fn constScanLine<RetType, T: QImage_constScanLine<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.constScanLine(self);
    // return 1;
  }
}

pub trait QImage_constScanLine<RetType> {
  fn constScanLine(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  const uchar * QImage::constScanLine(int );
impl<'a> /*trait*/ QImage_constScanLine<String> for (i32) {
  fn constScanLine(self , rsthis: &mut QImage) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage13constScanLineEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK6QImage13constScanLineEi(rsthis.qclsinst, arg0)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  QStringList QImage::textKeys();
impl /*struct*/ QImage {
  pub fn textKeys<RetType, T: QImage_textKeys<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.textKeys(self);
    // return 1;
  }
}

pub trait QImage_textKeys<RetType> {
  fn textKeys(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  QStringList QImage::textKeys();
impl<'a> /*trait*/ QImage_textKeys<()> for () {
  fn textKeys(self , rsthis: &mut QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage8textKeysEv()};
     unsafe {_ZNK6QImage8textKeysEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QImage::dotsPerMeterY();
impl /*struct*/ QImage {
  pub fn dotsPerMeterY<RetType, T: QImage_dotsPerMeterY<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.dotsPerMeterY(self);
    // return 1;
  }
}

pub trait QImage_dotsPerMeterY<RetType> {
  fn dotsPerMeterY(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  int QImage::dotsPerMeterY();
impl<'a> /*trait*/ QImage_dotsPerMeterY<i32> for () {
  fn dotsPerMeterY(self , rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage13dotsPerMeterYEv()};
    let mut ret = unsafe {_ZNK6QImage13dotsPerMeterYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QImage::fill(uint pixel);
impl /*struct*/ QImage {
  pub fn fill<RetType, T: QImage_fill<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fill(self);
    // return 1;
  }
}

pub trait QImage_fill<RetType> {
  fn fill(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  void QImage::fill(uint pixel);
impl<'a> /*trait*/ QImage_fill<()> for (u32) {
  fn fill(self , rsthis: &mut QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4fillEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN6QImage4fillEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPixelFormat QImage::pixelFormat();
impl /*struct*/ QImage {
  pub fn pixelFormat<RetType, T: QImage_pixelFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pixelFormat(self);
    // return 1;
  }
}

pub trait QImage_pixelFormat<RetType> {
  fn pixelFormat(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  QPixelFormat QImage::pixelFormat();
impl<'a> /*trait*/ QImage_pixelFormat<QPixelFormat> for () {
  fn pixelFormat(self , rsthis: &mut QImage) -> QPixelFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage11pixelFormatEv()};
    let mut ret = unsafe {_ZNK6QImage11pixelFormatEv(rsthis.qclsinst)};
    let mut ret1 = QPixelFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QImage::dotsPerMeterX();
impl /*struct*/ QImage {
  pub fn dotsPerMeterX<RetType, T: QImage_dotsPerMeterX<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.dotsPerMeterX(self);
    // return 1;
  }
}

pub trait QImage_dotsPerMeterX<RetType> {
  fn dotsPerMeterX(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  int QImage::dotsPerMeterX();
impl<'a> /*trait*/ QImage_dotsPerMeterX<i32> for () {
  fn dotsPerMeterX(self , rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage13dotsPerMeterXEv()};
    let mut ret = unsafe {_ZNK6QImage13dotsPerMeterXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QImage::setDotsPerMeterY(int );
impl /*struct*/ QImage {
  pub fn setDotsPerMeterY<RetType, T: QImage_setDotsPerMeterY<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDotsPerMeterY(self);
    // return 1;
  }
}

pub trait QImage_setDotsPerMeterY<RetType> {
  fn setDotsPerMeterY(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  void QImage::setDotsPerMeterY(int );
impl<'a> /*trait*/ QImage_setDotsPerMeterY<()> for (i32) {
  fn setDotsPerMeterY(self , rsthis: &mut QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage16setDotsPerMeterYEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QImage16setDotsPerMeterYEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QImage::bitPlaneCount();
impl /*struct*/ QImage {
  pub fn bitPlaneCount<RetType, T: QImage_bitPlaneCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bitPlaneCount(self);
    // return 1;
  }
}

pub trait QImage_bitPlaneCount<RetType> {
  fn bitPlaneCount(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  int QImage::bitPlaneCount();
impl<'a> /*trait*/ QImage_bitPlaneCount<i32> for () {
  fn bitPlaneCount(self , rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage13bitPlaneCountEv()};
    let mut ret = unsafe {_ZNK6QImage13bitPlaneCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QImage::fill(const QColor & color);
impl<'a> /*trait*/ QImage_fill<()> for (QColor) {
  fn fill(self , rsthis: &mut QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4fillERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QImage4fillERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QImage::detach();
impl /*struct*/ QImage {
  pub fn detach<RetType, T: QImage_detach<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.detach(self);
    // return 1;
  }
}

pub trait QImage_detach<RetType> {
  fn detach(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  void QImage::detach();
impl<'a> /*trait*/ QImage_detach<()> for () {
  fn detach(self , rsthis: &mut QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage6detachEv()};
     unsafe {_ZN6QImage6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QImage::loadFromData(const QByteArray & data, const char * aformat);
impl<'a> /*trait*/ QImage_loadFromData<i8> for (QByteArray, &'a  String) {
  fn loadFromData(self , rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage12loadFromDataERK10QByteArrayPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN6QImage12loadFromDataERK10QByteArrayPKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QImage::QImage(const QString & fileName, const char * format);
impl<'a> /*trait*/ QImage_NewQImage for (QString, &'a  String) {
  fn NewQImage(self) -> QImage {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImageC1ERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    unsafe {_ZN6QImageC1ERK7QStringPKc(qthis, arg0, arg1)};
    let rsthis = QImage{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPaintEngine * QImage::paintEngine();
impl /*struct*/ QImage {
  pub fn paintEngine<RetType, T: QImage_paintEngine<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.paintEngine(self);
    // return 1;
  }
}

pub trait QImage_paintEngine<RetType> {
  fn paintEngine(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  QPaintEngine * QImage::paintEngine();
impl<'a> /*trait*/ QImage_paintEngine<QPaintEngine> for () {
  fn paintEngine(self , rsthis: &mut QImage) -> QPaintEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage11paintEngineEv()};
    let mut ret = unsafe {_ZNK6QImage11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QImage::QImage(const QImage & );
impl<'a> /*trait*/ QImage_NewQImage for (QImage) {
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

  // proto:  void QImage::swap(QImage & other);
impl /*struct*/ QImage {
  pub fn swap<RetType, T: QImage_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QImage_swap<RetType> {
  fn swap(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  void QImage::swap(QImage & other);
impl<'a> /*trait*/ QImage_swap<()> for (QImage) {
  fn swap(self , rsthis: &mut QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QImage4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QImage::devicePixelRatio();
impl /*struct*/ QImage {
  pub fn devicePixelRatio<RetType, T: QImage_devicePixelRatio<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.devicePixelRatio(self);
    // return 1;
  }
}

pub trait QImage_devicePixelRatio<RetType> {
  fn devicePixelRatio(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  qreal QImage::devicePixelRatio();
impl<'a> /*trait*/ QImage_devicePixelRatio<f64> for () {
  fn devicePixelRatio(self , rsthis: &mut QImage) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage16devicePixelRatioEv()};
    let mut ret = unsafe {_ZNK6QImage16devicePixelRatioEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  int QImage::devType();
impl /*struct*/ QImage {
  pub fn devType<RetType, T: QImage_devType<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.devType(self);
    // return 1;
  }
}

pub trait QImage_devType<RetType> {
  fn devType(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  int QImage::devType();
impl<'a> /*trait*/ QImage_devType<i32> for () {
  fn devType(self , rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage7devTypeEv()};
    let mut ret = unsafe {_ZNK6QImage7devTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QImage::valid(const QPoint & pt);
impl /*struct*/ QImage {
  pub fn valid<RetType, T: QImage_valid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.valid(self);
    // return 1;
  }
}

pub trait QImage_valid<RetType> {
  fn valid(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  bool QImage::valid(const QPoint & pt);
impl<'a> /*trait*/ QImage_valid<i8> for (QPoint) {
  fn valid(self , rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5validERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QImage5validERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QImage::pixelIndex(const QPoint & pt);
impl /*struct*/ QImage {
  pub fn pixelIndex<RetType, T: QImage_pixelIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pixelIndex(self);
    // return 1;
  }
}

pub trait QImage_pixelIndex<RetType> {
  fn pixelIndex(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  int QImage::pixelIndex(const QPoint & pt);
impl<'a> /*trait*/ QImage_pixelIndex<i32> for (QPoint) {
  fn pixelIndex(self , rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage10pixelIndexERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QImage10pixelIndexERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QImage::setDotsPerMeterX(int );
impl /*struct*/ QImage {
  pub fn setDotsPerMeterX<RetType, T: QImage_setDotsPerMeterX<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDotsPerMeterX(self);
    // return 1;
  }
}

pub trait QImage_setDotsPerMeterX<RetType> {
  fn setDotsPerMeterX(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  void QImage::setDotsPerMeterX(int );
impl<'a> /*trait*/ QImage_setDotsPerMeterX<()> for (i32) {
  fn setDotsPerMeterX(self , rsthis: &mut QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage16setDotsPerMeterXEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QImage16setDotsPerMeterXEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QImage::setPixel(int x, int y, uint index_or_rgb);
impl<'a> /*trait*/ QImage_setPixel<()> for (i32, i32, u32) {
  fn setPixel(self , rsthis: &mut QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage8setPixelEiij()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_uint;
     unsafe {_ZN6QImage8setPixelEiij(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  bool QImage::load(const QString & fileName, const char * format);
impl /*struct*/ QImage {
  pub fn load<RetType, T: QImage_load<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.load(self);
    // return 1;
  }
}

pub trait QImage_load<RetType> {
  fn load(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  bool QImage::load(const QString & fileName, const char * format);
impl<'a> /*trait*/ QImage_load<i8> for (QString, &'a  String) {
  fn load(self , rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4loadERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN6QImage4loadERK7QStringPKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QVector<QRgb> QImage::colorTable();
impl /*struct*/ QImage {
  pub fn colorTable<RetType, T: QImage_colorTable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.colorTable(self);
    // return 1;
  }
}

pub trait QImage_colorTable<RetType> {
  fn colorTable(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  QVector<QRgb> QImage::colorTable();
impl<'a> /*trait*/ QImage_colorTable<()> for () {
  fn colorTable(self , rsthis: &mut QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage10colorTableEv()};
     unsafe {_ZNK6QImage10colorTableEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QImage::size();
impl /*struct*/ QImage {
  pub fn size<RetType, T: QImage_size<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QImage_size<RetType> {
  fn size(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  QSize QImage::size();
impl<'a> /*trait*/ QImage_size<QSize> for () {
  fn size(self , rsthis: &mut QImage) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4sizeEv()};
    let mut ret = unsafe {_ZNK6QImage4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QImage::height();
impl /*struct*/ QImage {
  pub fn height<RetType, T: QImage_height<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QImage_height<RetType> {
  fn height(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  int QImage::height();
impl<'a> /*trait*/ QImage_height<i32> for () {
  fn height(self , rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage6heightEv()};
    let mut ret = unsafe {_ZNK6QImage6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QImage::pixelIndex(int x, int y);
impl<'a> /*trait*/ QImage_pixelIndex<i32> for (i32, i32) {
  fn pixelIndex(self , rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage10pixelIndexEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK6QImage10pixelIndexEii(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QImage::width();
impl /*struct*/ QImage {
  pub fn width<RetType, T: QImage_width<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QImage_width<RetType> {
  fn width(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  int QImage::width();
impl<'a> /*trait*/ QImage_width<i32> for () {
  fn width(self , rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5widthEv()};
    let mut ret = unsafe {_ZNK6QImage5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QImage::load(QIODevice * device, const char * format);
impl<'a> /*trait*/ QImage_load<i8> for (QIODevice, &'a  String) {
  fn load(self , rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4loadEP9QIODevicePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN6QImage4loadEP9QIODevicePKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QImage::QImage();
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

  // proto:  uchar * QImage::scanLine(int );
impl /*struct*/ QImage {
  pub fn scanLine<RetType, T: QImage_scanLine<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.scanLine(self);
    // return 1;
  }
}

pub trait QImage_scanLine<RetType> {
  fn scanLine(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  uchar * QImage::scanLine(int );
impl<'a> /*trait*/ QImage_scanLine<String> for (i32) {
  fn scanLine(self , rsthis: &mut QImage) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage8scanLineEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN6QImage8scanLineEi(rsthis.qclsinst, arg0)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  int QImage::bytesPerLine();
impl /*struct*/ QImage {
  pub fn bytesPerLine<RetType, T: QImage_bytesPerLine<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bytesPerLine(self);
    // return 1;
  }
}

pub trait QImage_bytesPerLine<RetType> {
  fn bytesPerLine(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  int QImage::bytesPerLine();
impl<'a> /*trait*/ QImage_bytesPerLine<i32> for () {
  fn bytesPerLine(self , rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage12bytesPerLineEv()};
    let mut ret = unsafe {_ZNK6QImage12bytesPerLineEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  qint64 QImage::cacheKey();
impl /*struct*/ QImage {
  pub fn cacheKey<RetType, T: QImage_cacheKey<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cacheKey(self);
    // return 1;
  }
}

pub trait QImage_cacheKey<RetType> {
  fn cacheKey(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  qint64 QImage::cacheKey();
impl<'a> /*trait*/ QImage_cacheKey<i64> for () {
  fn cacheKey(self , rsthis: &mut QImage) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage8cacheKeyEv()};
    let mut ret = unsafe {_ZNK6QImage8cacheKeyEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  QRgb QImage::pixel(int x, int y);
impl<'a> /*trait*/ QImage_pixel<u32> for (i32, i32) {
  fn pixel(self , rsthis: &mut QImage) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5pixelEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK6QImage5pixelEii(rsthis.qclsinst, arg0, arg1)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  void QImage::~QImage();
impl /*struct*/ QImage {
  pub fn FreeQImage<RetType, T: QImage_FreeQImage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQImage(self);
    // return 1;
  }
}

pub trait QImage_FreeQImage<RetType> {
  fn FreeQImage(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  void QImage::~QImage();
impl<'a> /*trait*/ QImage_FreeQImage<()> for () {
  fn FreeQImage(self , rsthis: &mut QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImageD0Ev()};
     unsafe {_ZN6QImageD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QImage::save(const QString & fileName, const char * format, int quality);
impl<'a> /*trait*/ QImage_save<i8> for (QString, &'a  String, i32) {
  fn save(self , rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4saveERK7QStringPKci()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZNK6QImage4saveERK7QStringPKci(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QImage::setColor(int i, QRgb c);
impl /*struct*/ QImage {
  pub fn setColor<RetType, T: QImage_setColor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setColor(self);
    // return 1;
  }
}

pub trait QImage_setColor<RetType> {
  fn setColor(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  void QImage::setColor(int i, QRgb c);
impl<'a> /*trait*/ QImage_setColor<()> for (i32, u32) {
  fn setColor(self , rsthis: &mut QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage8setColorEij()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_uint;
     unsafe {_ZN6QImage8setColorEij(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QImage::isNull();
impl /*struct*/ QImage {
  pub fn isNull<RetType, T: QImage_isNull<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QImage_isNull<RetType> {
  fn isNull(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  bool QImage::isNull();
impl<'a> /*trait*/ QImage_isNull<i8> for () {
  fn isNull(self , rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage6isNullEv()};
    let mut ret = unsafe {_ZNK6QImage6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QImage::byteCount();
impl /*struct*/ QImage {
  pub fn byteCount<RetType, T: QImage_byteCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.byteCount(self);
    // return 1;
  }
}

pub trait QImage_byteCount<RetType> {
  fn byteCount(self , rsthis: &mut QImage) -> RetType;
}

  // proto:  int QImage::byteCount();
impl<'a> /*trait*/ QImage_byteCount<i32> for () {
  fn byteCount(self , rsthis: &mut QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage9byteCountEv()};
    let mut ret = unsafe {_ZNK6QImage9byteCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QImage::valid(int x, int y);
impl<'a> /*trait*/ QImage_valid<i8> for (i32, i32) {
  fn valid(self , rsthis: &mut QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5validEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK6QImage5validEii(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

