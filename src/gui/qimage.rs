// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtGui/qimage.h
// dst-file: /src/gui/qimage.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::qpaintdevice::QPaintDevice; // 773
use std::ops::Deref;
use super::super::core::qrect::QRect; // 771
use super::qtransform::QTransform; // 773
use super::super::core::qsize::QSize; // 771
use super::super::core::qstring::QString; // 771
use super::super::core::qbytearray::QByteArray; // 771
use super::super::core::qpoint::QPoint; // 771
use super::qmatrix::QMatrix; // 773
use super::super::core::qiodevice::QIODevice; // 771
use super::qpixelformat::QPixelFormat; // 773
use super::qcolor::QColor; // 773
use super::qpaintengine::QPaintEngine; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QImage_Class_Size() -> c_int;
  // proto:  QImage QImage::copy(const QRect & rect);
  fn _ZNK6QImage4copyERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto: static QTransform QImage::trueMatrix(const QTransform & , int w, int h);
  fn _ZN6QImage10trueMatrixERK10QTransformii(arg0: *mut c_void, arg1: c_int, arg2: c_int) -> *mut c_void;
  // proto:  uchar * QImage::bits();
  fn _ZN6QImage4bitsEv(qthis: u64 /* *mut c_void*/) -> *mut c_uchar;
  // proto:  void QImage::setAlphaChannel(const QImage & alphaChannel);
  fn _ZN6QImage15setAlphaChannelERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QImage::text(const QString & key);
  fn _ZNK6QImage4textERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRect QImage::rect();
  fn _ZNK6QImage4rectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QImage::QImage(const char *const [] xpm);
  fn dector_ZN6QImageC1EPKPKc(arg0: *mut *mut c_char) -> *mut c_void;
  fn _ZN6QImageC1EPKPKc(qthis: u64 /* *mut c_void*/, arg0: *mut *mut c_char);
  // proto:  QImage QImage::createHeuristicMask(bool clipTight);
  fn _ZNK6QImage19createHeuristicMaskEb(qthis: u64 /* *mut c_void*/, arg0: c_char) -> *mut c_void;
  // proto:  const uchar * QImage::constBits();
  fn _ZNK6QImage9constBitsEv(qthis: u64 /* *mut c_void*/) -> *mut c_uchar;
  // proto: static QImage QImage::fromData(const QByteArray & data, const char * format);
  fn demth_ZN6QImage8fromDataERK10QByteArrayPKc(arg0: *mut c_void, arg1: *mut c_char) -> *mut c_void;
  // proto: static QImage QImage::fromData(const uchar * data, int size, const char * format);
  fn _ZN6QImage8fromDataEPKhiPKc(arg0: *mut c_uchar, arg1: c_int, arg2: *mut c_char) -> *mut c_void;
  // proto:  bool QImage::isDetached();
  fn _ZNK6QImage10isDetachedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QImage::setOffset(const QPoint & );
  fn _ZN6QImage9setOffsetERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QMatrix QImage::trueMatrix(const QMatrix & , int w, int h);
  fn _ZN6QImage10trueMatrixERK7QMatrixii(arg0: *mut c_void, arg1: c_int, arg2: c_int) -> *mut c_void;
  // proto:  bool QImage::isGrayscale();
  fn _ZNK6QImage11isGrayscaleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QImage::save(QIODevice * device, const char * format, int quality);
  fn _ZNK6QImage4saveEP9QIODevicePKci(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char, arg2: c_int) -> c_char;
  // proto:  int QImage::depth();
  fn _ZNK6QImage5depthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QImage QImage::alphaChannel();
  fn _ZNK6QImage12alphaChannelEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QImage::hasAlphaChannel();
  fn _ZNK6QImage15hasAlphaChannelEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QImage::loadFromData(const uchar * buf, int len, const char * format);
  fn _ZN6QImage12loadFromDataEPKhiPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_uchar, arg1: c_int, arg2: *mut c_char) -> c_char;
  // proto:  int QImage::colorCount();
  fn _ZNK6QImage10colorCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QImage::allGray();
  fn _ZNK6QImage7allGrayEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QImage::setColorCount(int );
  fn _ZN6QImage13setColorCountEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QRgb QImage::pixel(const QPoint & pt);
  fn _ZNK6QImage5pixelERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_uint;
  // proto:  void QImage::setDevicePixelRatio(qreal scaleFactor);
  fn _ZN6QImage19setDevicePixelRatioEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  QImage QImage::copy(int x, int y, int w, int h);
  fn demth_ZNK6QImage4copyEiiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> *mut c_void;
  // proto:  void QImage::setText(const QString & key, const QString & value);
  fn _ZN6QImage7setTextERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QRgb QImage::color(int i);
  fn _ZNK6QImage5colorEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_uint;
  // proto:  void QImage::setPixel(const QPoint & pt, uint index_or_rgb);
  fn _ZN6QImage8setPixelERK6QPointj(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_uint);
  // proto:  QPoint QImage::offset();
  fn _ZNK6QImage6offsetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const uchar * QImage::constScanLine(int );
  fn _ZNK6QImage13constScanLineEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_uchar;
  // proto:  QStringList QImage::textKeys();
  fn _ZNK6QImage8textKeysEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QImage::dotsPerMeterY();
  fn _ZNK6QImage13dotsPerMeterYEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QImage::fill(uint pixel);
  fn _ZN6QImage4fillEj(qthis: u64 /* *mut c_void*/, arg0: c_uint);
  // proto:  QPixelFormat QImage::pixelFormat();
  fn _ZNK6QImage11pixelFormatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QImage::dotsPerMeterX();
  fn _ZNK6QImage13dotsPerMeterXEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QImage::setDotsPerMeterY(int );
  fn _ZN6QImage16setDotsPerMeterYEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QImage::bitPlaneCount();
  fn _ZNK6QImage13bitPlaneCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QImage::fill(const QColor & color);
  fn _ZN6QImage4fillERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QImage::detach();
  fn _ZN6QImage6detachEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QImage::loadFromData(const QByteArray & data, const char * aformat);
  fn demth_ZN6QImage12loadFromDataERK10QByteArrayPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  void QImage::QImage(const QString & fileName, const char * format);
  fn dector_ZN6QImageC1ERK7QStringPKc(arg0: *mut c_void, arg1: *mut c_char) -> *mut c_void;
  fn _ZN6QImageC1ERK7QStringPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char);
  // proto:  QPaintEngine * QImage::paintEngine();
  fn _ZNK6QImage11paintEngineEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QImage::QImage(const QImage & );
  fn dector_ZN6QImageC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN6QImageC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QImage::swap(QImage & other);
  fn demth_ZN6QImage4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qreal QImage::devicePixelRatio();
  fn _ZNK6QImage16devicePixelRatioEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  int QImage::devType();
  fn _ZNK6QImage7devTypeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QImage::valid(const QPoint & pt);
  fn _ZNK6QImage5validERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  int QImage::pixelIndex(const QPoint & pt);
  fn _ZNK6QImage10pixelIndexERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  void QImage::setDotsPerMeterX(int );
  fn _ZN6QImage16setDotsPerMeterXEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QImage::setPixel(int x, int y, uint index_or_rgb);
  fn _ZN6QImage8setPixelEiij(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_uint);
  // proto:  bool QImage::load(const QString & fileName, const char * format);
  fn _ZN6QImage4loadERK7QStringPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  QVector<QRgb> QImage::colorTable();
  fn _ZNK6QImage10colorTableEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QImage::size();
  fn _ZNK6QImage4sizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QImage::height();
  fn _ZNK6QImage6heightEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QImage::pixelIndex(int x, int y);
  fn _ZNK6QImage10pixelIndexEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> c_int;
  // proto:  int QImage::width();
  fn _ZNK6QImage5widthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QImage::load(QIODevice * device, const char * format);
  fn _ZN6QImage4loadEP9QIODevicePKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> c_char;
  // proto:  void QImage::QImage();
  fn dector_ZN6QImageC1Ev() -> *mut c_void;
  fn _ZN6QImageC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  uchar * QImage::scanLine(int );
  fn _ZN6QImage8scanLineEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_uchar;
  // proto:  int QImage::bytesPerLine();
  fn _ZNK6QImage12bytesPerLineEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  qint64 QImage::cacheKey();
  fn _ZNK6QImage8cacheKeyEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  QRgb QImage::pixel(int x, int y);
  fn _ZNK6QImage5pixelEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> c_uint;
  // proto:  void QImage::~QImage();
  fn _ZN6QImageD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QImage::save(const QString & fileName, const char * format, int quality);
  fn _ZNK6QImage4saveERK7QStringPKci(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char, arg2: c_int) -> c_char;
  // proto:  void QImage::setColor(int i, QRgb c);
  fn _ZN6QImage8setColorEij(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_uint);
  // proto:  bool QImage::isNull();
  fn _ZNK6QImage6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QImage::byteCount();
  fn _ZNK6QImage9byteCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QImage::valid(int x, int y);
  fn _ZNK6QImage5validEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QImage)=32
#[derive(Default)]
pub struct QImage {
  qbase: QPaintDevice,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QImage {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QImage {
    return QImage{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QImage {
  type Target = QPaintDevice;

  fn deref(&self) -> &QPaintDevice {
    return & self.qbase;
  }
}
impl AsRef<QPaintDevice> for QImage {
  fn as_ref(& self) -> & QPaintDevice {
    return & self.qbase;
  }
}
  // proto:  QImage QImage::copy(const QRect & rect);
impl /*struct*/ QImage {
  pub fn copy<RetType, T: QImage_copy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.copy(self);
    // return 1;
  }
}

pub trait QImage_copy<RetType> {
  fn copy(self , rsthis: & QImage) -> RetType;
}

  // proto:  QImage QImage::copy(const QRect & rect);
impl<'a> /*trait*/ QImage_copy<QImage> for (&'a QRect) {
  fn copy(self , rsthis: & QImage) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4copyERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QImage4copyERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QImage::inheritFrom(ret as u64);
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
impl<'a> /*trait*/ QImage_trueMatrix_s<QTransform> for (&'a QTransform, i32, i32) {
  fn trueMatrix_s(self ) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage10trueMatrixERK10QTransformii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN6QImage10trueMatrixERK10QTransformii(arg0, arg1, arg2)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  uchar * QImage::bits();
impl /*struct*/ QImage {
  pub fn bits<RetType, T: QImage_bits<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bits(self);
    // return 1;
  }
}

pub trait QImage_bits<RetType> {
  fn bits(self , rsthis: & QImage) -> RetType;
}

  // proto:  uchar * QImage::bits();
impl<'a> /*trait*/ QImage_bits<String> for () {
  fn bits(self , rsthis: & QImage) -> String {
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
  pub fn setAlphaChannel<RetType, T: QImage_setAlphaChannel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAlphaChannel(self);
    // return 1;
  }
}

pub trait QImage_setAlphaChannel<RetType> {
  fn setAlphaChannel(self , rsthis: & QImage) -> RetType;
}

  // proto:  void QImage::setAlphaChannel(const QImage & alphaChannel);
impl<'a> /*trait*/ QImage_setAlphaChannel<()> for (&'a QImage) {
  fn setAlphaChannel(self , rsthis: & QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage15setAlphaChannelERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QImage15setAlphaChannelERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QImage::text(const QString & key);
impl /*struct*/ QImage {
  pub fn text<RetType, T: QImage_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QImage_text<RetType> {
  fn text(self , rsthis: & QImage) -> RetType;
}

  // proto:  QString QImage::text(const QString & key);
impl<'a> /*trait*/ QImage_text<QString> for (&'a QString) {
  fn text(self , rsthis: & QImage) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4textERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QImage4textERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QImage::rect();
impl /*struct*/ QImage {
  pub fn rect<RetType, T: QImage_rect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QImage_rect<RetType> {
  fn rect(self , rsthis: & QImage) -> RetType;
}

  // proto:  QRect QImage::rect();
impl<'a> /*trait*/ QImage_rect<QRect> for () {
  fn rect(self , rsthis: & QImage) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4rectEv()};
    let mut ret = unsafe {_ZNK6QImage4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QImage::QImage(const char *const [] xpm);
impl /*struct*/ QImage {
  pub fn New<T: QImage_New>(value: T) -> QImage {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QImage_New {
  fn New(self) -> QImage;
}

  // proto:  void QImage::QImage(const char *const [] xpm);
impl<'a> /*trait*/ QImage_New for (&'a  Vec<&'a  i8>) {
  fn New(self) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImageC1EPKPKc()};
    let ctysz: c_int = unsafe{QImage_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.as_ptr()  as *mut *mut c_char;
    // unsafe {_ZN6QImageC1EPKPKc(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN6QImageC1EPKPKc(arg0)} as u64;
    let rsthis = QImage{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QImage QImage::createHeuristicMask(bool clipTight);
impl /*struct*/ QImage {
  pub fn createHeuristicMask<RetType, T: QImage_createHeuristicMask<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createHeuristicMask(self);
    // return 1;
  }
}

pub trait QImage_createHeuristicMask<RetType> {
  fn createHeuristicMask(self , rsthis: & QImage) -> RetType;
}

  // proto:  QImage QImage::createHeuristicMask(bool clipTight);
impl<'a> /*trait*/ QImage_createHeuristicMask<QImage> for (i8) {
  fn createHeuristicMask(self , rsthis: & QImage) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage19createHeuristicMaskEb()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZNK6QImage19createHeuristicMaskEb(rsthis.qclsinst, arg0)};
    let mut ret1 = QImage::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const uchar * QImage::constBits();
impl /*struct*/ QImage {
  pub fn constBits<RetType, T: QImage_constBits<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.constBits(self);
    // return 1;
  }
}

pub trait QImage_constBits<RetType> {
  fn constBits(self , rsthis: & QImage) -> RetType;
}

  // proto:  const uchar * QImage::constBits();
impl<'a> /*trait*/ QImage_constBits<String> for () {
  fn constBits(self , rsthis: & QImage) -> String {
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
impl<'a> /*trait*/ QImage_fromData_s<QImage> for (&'a QByteArray, &'a  String) {
  fn fromData_s(self ) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage8fromDataERK10QByteArrayPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {demth_ZN6QImage8fromDataERK10QByteArrayPKc(arg0, arg1)};
    let mut ret1 = QImage::inheritFrom(ret as u64);
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
    let mut ret1 = QImage::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QImage::isDetached();
impl /*struct*/ QImage {
  pub fn isDetached<RetType, T: QImage_isDetached<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDetached(self);
    // return 1;
  }
}

pub trait QImage_isDetached<RetType> {
  fn isDetached(self , rsthis: & QImage) -> RetType;
}

  // proto:  bool QImage::isDetached();
impl<'a> /*trait*/ QImage_isDetached<i8> for () {
  fn isDetached(self , rsthis: & QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage10isDetachedEv()};
    let mut ret = unsafe {_ZNK6QImage10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QImage::setOffset(const QPoint & );
impl /*struct*/ QImage {
  pub fn setOffset<RetType, T: QImage_setOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOffset(self);
    // return 1;
  }
}

pub trait QImage_setOffset<RetType> {
  fn setOffset(self , rsthis: & QImage) -> RetType;
}

  // proto:  void QImage::setOffset(const QPoint & );
impl<'a> /*trait*/ QImage_setOffset<()> for (&'a QPoint) {
  fn setOffset(self , rsthis: & QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage9setOffsetERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QImage9setOffsetERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QMatrix QImage::trueMatrix(const QMatrix & , int w, int h);
impl<'a> /*trait*/ QImage_trueMatrix_s<QMatrix> for (&'a QMatrix, i32, i32) {
  fn trueMatrix_s(self ) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage10trueMatrixERK7QMatrixii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN6QImage10trueMatrixERK7QMatrixii(arg0, arg1, arg2)};
    let mut ret1 = QMatrix::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QImage::isGrayscale();
impl /*struct*/ QImage {
  pub fn isGrayscale<RetType, T: QImage_isGrayscale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isGrayscale(self);
    // return 1;
  }
}

pub trait QImage_isGrayscale<RetType> {
  fn isGrayscale(self , rsthis: & QImage) -> RetType;
}

  // proto:  bool QImage::isGrayscale();
impl<'a> /*trait*/ QImage_isGrayscale<i8> for () {
  fn isGrayscale(self , rsthis: & QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage11isGrayscaleEv()};
    let mut ret = unsafe {_ZNK6QImage11isGrayscaleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QImage::save(QIODevice * device, const char * format, int quality);
impl /*struct*/ QImage {
  pub fn save<RetType, T: QImage_save<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.save(self);
    // return 1;
  }
}

pub trait QImage_save<RetType> {
  fn save(self , rsthis: & QImage) -> RetType;
}

  // proto:  bool QImage::save(QIODevice * device, const char * format, int quality);
impl<'a> /*trait*/ QImage_save<i8> for (&'a QIODevice, &'a  String, i32) {
  fn save(self , rsthis: & QImage) -> i8 {
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
  pub fn depth<RetType, T: QImage_depth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.depth(self);
    // return 1;
  }
}

pub trait QImage_depth<RetType> {
  fn depth(self , rsthis: & QImage) -> RetType;
}

  // proto:  int QImage::depth();
impl<'a> /*trait*/ QImage_depth<i32> for () {
  fn depth(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5depthEv()};
    let mut ret = unsafe {_ZNK6QImage5depthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QImage QImage::alphaChannel();
impl /*struct*/ QImage {
  pub fn alphaChannel<RetType, T: QImage_alphaChannel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.alphaChannel(self);
    // return 1;
  }
}

pub trait QImage_alphaChannel<RetType> {
  fn alphaChannel(self , rsthis: & QImage) -> RetType;
}

  // proto:  QImage QImage::alphaChannel();
impl<'a> /*trait*/ QImage_alphaChannel<QImage> for () {
  fn alphaChannel(self , rsthis: & QImage) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage12alphaChannelEv()};
    let mut ret = unsafe {_ZNK6QImage12alphaChannelEv(rsthis.qclsinst)};
    let mut ret1 = QImage::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QImage::hasAlphaChannel();
impl /*struct*/ QImage {
  pub fn hasAlphaChannel<RetType, T: QImage_hasAlphaChannel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasAlphaChannel(self);
    // return 1;
  }
}

pub trait QImage_hasAlphaChannel<RetType> {
  fn hasAlphaChannel(self , rsthis: & QImage) -> RetType;
}

  // proto:  bool QImage::hasAlphaChannel();
impl<'a> /*trait*/ QImage_hasAlphaChannel<i8> for () {
  fn hasAlphaChannel(self , rsthis: & QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage15hasAlphaChannelEv()};
    let mut ret = unsafe {_ZNK6QImage15hasAlphaChannelEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QImage::loadFromData(const uchar * buf, int len, const char * format);
impl /*struct*/ QImage {
  pub fn loadFromData<RetType, T: QImage_loadFromData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.loadFromData(self);
    // return 1;
  }
}

pub trait QImage_loadFromData<RetType> {
  fn loadFromData(self , rsthis: & QImage) -> RetType;
}

  // proto:  bool QImage::loadFromData(const uchar * buf, int len, const char * format);
impl<'a> /*trait*/ QImage_loadFromData<i8> for (&'a  String, i32, &'a  String) {
  fn loadFromData(self , rsthis: & QImage) -> i8 {
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
  pub fn colorCount<RetType, T: QImage_colorCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.colorCount(self);
    // return 1;
  }
}

pub trait QImage_colorCount<RetType> {
  fn colorCount(self , rsthis: & QImage) -> RetType;
}

  // proto:  int QImage::colorCount();
impl<'a> /*trait*/ QImage_colorCount<i32> for () {
  fn colorCount(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage10colorCountEv()};
    let mut ret = unsafe {_ZNK6QImage10colorCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QImage::allGray();
impl /*struct*/ QImage {
  pub fn allGray<RetType, T: QImage_allGray<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.allGray(self);
    // return 1;
  }
}

pub trait QImage_allGray<RetType> {
  fn allGray(self , rsthis: & QImage) -> RetType;
}

  // proto:  bool QImage::allGray();
impl<'a> /*trait*/ QImage_allGray<i8> for () {
  fn allGray(self , rsthis: & QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage7allGrayEv()};
    let mut ret = unsafe {_ZNK6QImage7allGrayEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QImage::setColorCount(int );
impl /*struct*/ QImage {
  pub fn setColorCount<RetType, T: QImage_setColorCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColorCount(self);
    // return 1;
  }
}

pub trait QImage_setColorCount<RetType> {
  fn setColorCount(self , rsthis: & QImage) -> RetType;
}

  // proto:  void QImage::setColorCount(int );
impl<'a> /*trait*/ QImage_setColorCount<()> for (i32) {
  fn setColorCount(self , rsthis: & QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage13setColorCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QImage13setColorCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRgb QImage::pixel(const QPoint & pt);
impl /*struct*/ QImage {
  pub fn pixel<RetType, T: QImage_pixel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pixel(self);
    // return 1;
  }
}

pub trait QImage_pixel<RetType> {
  fn pixel(self , rsthis: & QImage) -> RetType;
}

  // proto:  QRgb QImage::pixel(const QPoint & pt);
impl<'a> /*trait*/ QImage_pixel<u32> for (&'a QPoint) {
  fn pixel(self , rsthis: & QImage) -> u32 {
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
  pub fn setDevicePixelRatio<RetType, T: QImage_setDevicePixelRatio<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDevicePixelRatio(self);
    // return 1;
  }
}

pub trait QImage_setDevicePixelRatio<RetType> {
  fn setDevicePixelRatio(self , rsthis: & QImage) -> RetType;
}

  // proto:  void QImage::setDevicePixelRatio(qreal scaleFactor);
impl<'a> /*trait*/ QImage_setDevicePixelRatio<()> for (f64) {
  fn setDevicePixelRatio(self , rsthis: & QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage19setDevicePixelRatioEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QImage19setDevicePixelRatioEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QImage QImage::copy(int x, int y, int w, int h);
impl<'a> /*trait*/ QImage_copy<QImage> for (i32, i32, i32, i32) {
  fn copy(self , rsthis: & QImage) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4copyEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {demth_ZNK6QImage4copyEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QImage::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QImage::setText(const QString & key, const QString & value);
impl /*struct*/ QImage {
  pub fn setText<RetType, T: QImage_setText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QImage_setText<RetType> {
  fn setText(self , rsthis: & QImage) -> RetType;
}

  // proto:  void QImage::setText(const QString & key, const QString & value);
impl<'a> /*trait*/ QImage_setText<()> for (&'a QString, &'a QString) {
  fn setText(self , rsthis: & QImage) -> () {
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
  pub fn color<RetType, T: QImage_color<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.color(self);
    // return 1;
  }
}

pub trait QImage_color<RetType> {
  fn color(self , rsthis: & QImage) -> RetType;
}

  // proto:  QRgb QImage::color(int i);
impl<'a> /*trait*/ QImage_color<u32> for (i32) {
  fn color(self , rsthis: & QImage) -> u32 {
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
  pub fn setPixel<RetType, T: QImage_setPixel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPixel(self);
    // return 1;
  }
}

pub trait QImage_setPixel<RetType> {
  fn setPixel(self , rsthis: & QImage) -> RetType;
}

  // proto:  void QImage::setPixel(const QPoint & pt, uint index_or_rgb);
impl<'a> /*trait*/ QImage_setPixel<()> for (&'a QPoint, u32) {
  fn setPixel(self , rsthis: & QImage) -> () {
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
  pub fn offset<RetType, T: QImage_offset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.offset(self);
    // return 1;
  }
}

pub trait QImage_offset<RetType> {
  fn offset(self , rsthis: & QImage) -> RetType;
}

  // proto:  QPoint QImage::offset();
impl<'a> /*trait*/ QImage_offset<QPoint> for () {
  fn offset(self , rsthis: & QImage) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage6offsetEv()};
    let mut ret = unsafe {_ZNK6QImage6offsetEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const uchar * QImage::constScanLine(int );
impl /*struct*/ QImage {
  pub fn constScanLine<RetType, T: QImage_constScanLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.constScanLine(self);
    // return 1;
  }
}

pub trait QImage_constScanLine<RetType> {
  fn constScanLine(self , rsthis: & QImage) -> RetType;
}

  // proto:  const uchar * QImage::constScanLine(int );
impl<'a> /*trait*/ QImage_constScanLine<String> for (i32) {
  fn constScanLine(self , rsthis: & QImage) -> String {
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
  pub fn textKeys<RetType, T: QImage_textKeys<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textKeys(self);
    // return 1;
  }
}

pub trait QImage_textKeys<RetType> {
  fn textKeys(self , rsthis: & QImage) -> RetType;
}

  // proto:  QStringList QImage::textKeys();
impl<'a> /*trait*/ QImage_textKeys<()> for () {
  fn textKeys(self , rsthis: & QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage8textKeysEv()};
     unsafe {_ZNK6QImage8textKeysEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QImage::dotsPerMeterY();
impl /*struct*/ QImage {
  pub fn dotsPerMeterY<RetType, T: QImage_dotsPerMeterY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dotsPerMeterY(self);
    // return 1;
  }
}

pub trait QImage_dotsPerMeterY<RetType> {
  fn dotsPerMeterY(self , rsthis: & QImage) -> RetType;
}

  // proto:  int QImage::dotsPerMeterY();
impl<'a> /*trait*/ QImage_dotsPerMeterY<i32> for () {
  fn dotsPerMeterY(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage13dotsPerMeterYEv()};
    let mut ret = unsafe {_ZNK6QImage13dotsPerMeterYEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QImage::fill(uint pixel);
impl /*struct*/ QImage {
  pub fn fill<RetType, T: QImage_fill<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fill(self);
    // return 1;
  }
}

pub trait QImage_fill<RetType> {
  fn fill(self , rsthis: & QImage) -> RetType;
}

  // proto:  void QImage::fill(uint pixel);
impl<'a> /*trait*/ QImage_fill<()> for (u32) {
  fn fill(self , rsthis: & QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4fillEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN6QImage4fillEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPixelFormat QImage::pixelFormat();
impl /*struct*/ QImage {
  pub fn pixelFormat<RetType, T: QImage_pixelFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pixelFormat(self);
    // return 1;
  }
}

pub trait QImage_pixelFormat<RetType> {
  fn pixelFormat(self , rsthis: & QImage) -> RetType;
}

  // proto:  QPixelFormat QImage::pixelFormat();
impl<'a> /*trait*/ QImage_pixelFormat<QPixelFormat> for () {
  fn pixelFormat(self , rsthis: & QImage) -> QPixelFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage11pixelFormatEv()};
    let mut ret = unsafe {_ZNK6QImage11pixelFormatEv(rsthis.qclsinst)};
    let mut ret1 = QPixelFormat::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QImage::dotsPerMeterX();
impl /*struct*/ QImage {
  pub fn dotsPerMeterX<RetType, T: QImage_dotsPerMeterX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dotsPerMeterX(self);
    // return 1;
  }
}

pub trait QImage_dotsPerMeterX<RetType> {
  fn dotsPerMeterX(self , rsthis: & QImage) -> RetType;
}

  // proto:  int QImage::dotsPerMeterX();
impl<'a> /*trait*/ QImage_dotsPerMeterX<i32> for () {
  fn dotsPerMeterX(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage13dotsPerMeterXEv()};
    let mut ret = unsafe {_ZNK6QImage13dotsPerMeterXEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QImage::setDotsPerMeterY(int );
impl /*struct*/ QImage {
  pub fn setDotsPerMeterY<RetType, T: QImage_setDotsPerMeterY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDotsPerMeterY(self);
    // return 1;
  }
}

pub trait QImage_setDotsPerMeterY<RetType> {
  fn setDotsPerMeterY(self , rsthis: & QImage) -> RetType;
}

  // proto:  void QImage::setDotsPerMeterY(int );
impl<'a> /*trait*/ QImage_setDotsPerMeterY<()> for (i32) {
  fn setDotsPerMeterY(self , rsthis: & QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage16setDotsPerMeterYEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QImage16setDotsPerMeterYEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QImage::bitPlaneCount();
impl /*struct*/ QImage {
  pub fn bitPlaneCount<RetType, T: QImage_bitPlaneCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bitPlaneCount(self);
    // return 1;
  }
}

pub trait QImage_bitPlaneCount<RetType> {
  fn bitPlaneCount(self , rsthis: & QImage) -> RetType;
}

  // proto:  int QImage::bitPlaneCount();
impl<'a> /*trait*/ QImage_bitPlaneCount<i32> for () {
  fn bitPlaneCount(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage13bitPlaneCountEv()};
    let mut ret = unsafe {_ZNK6QImage13bitPlaneCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QImage::fill(const QColor & color);
impl<'a> /*trait*/ QImage_fill<()> for (&'a QColor) {
  fn fill(self , rsthis: & QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4fillERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QImage4fillERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QImage::detach();
impl /*struct*/ QImage {
  pub fn detach<RetType, T: QImage_detach<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.detach(self);
    // return 1;
  }
}

pub trait QImage_detach<RetType> {
  fn detach(self , rsthis: & QImage) -> RetType;
}

  // proto:  void QImage::detach();
impl<'a> /*trait*/ QImage_detach<()> for () {
  fn detach(self , rsthis: & QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage6detachEv()};
     unsafe {_ZN6QImage6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QImage::loadFromData(const QByteArray & data, const char * aformat);
impl<'a> /*trait*/ QImage_loadFromData<i8> for (&'a QByteArray, &'a  String) {
  fn loadFromData(self , rsthis: & QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage12loadFromDataERK10QByteArrayPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {demth_ZN6QImage12loadFromDataERK10QByteArrayPKc(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QImage::QImage(const QString & fileName, const char * format);
impl<'a> /*trait*/ QImage_New for (&'a QString, &'a  String) {
  fn New(self) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImageC1ERK7QStringPKc()};
    let ctysz: c_int = unsafe{QImage_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    // unsafe {_ZN6QImageC1ERK7QStringPKc(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN6QImageC1ERK7QStringPKc(arg0, arg1)} as u64;
    let rsthis = QImage{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPaintEngine * QImage::paintEngine();
impl /*struct*/ QImage {
  pub fn paintEngine<RetType, T: QImage_paintEngine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paintEngine(self);
    // return 1;
  }
}

pub trait QImage_paintEngine<RetType> {
  fn paintEngine(self , rsthis: & QImage) -> RetType;
}

  // proto:  QPaintEngine * QImage::paintEngine();
impl<'a> /*trait*/ QImage_paintEngine<QPaintEngine> for () {
  fn paintEngine(self , rsthis: & QImage) -> QPaintEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage11paintEngineEv()};
    let mut ret = unsafe {_ZNK6QImage11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QImage::QImage(const QImage & );
impl<'a> /*trait*/ QImage_New for (&'a QImage) {
  fn New(self) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImageC1ERKS_()};
    let ctysz: c_int = unsafe{QImage_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN6QImageC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN6QImageC1ERKS_(arg0)} as u64;
    let rsthis = QImage{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QImage::swap(QImage & other);
impl /*struct*/ QImage {
  pub fn swap<RetType, T: QImage_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QImage_swap<RetType> {
  fn swap(self , rsthis: & QImage) -> RetType;
}

  // proto:  void QImage::swap(QImage & other);
impl<'a> /*trait*/ QImage_swap<()> for (&'a QImage) {
  fn swap(self , rsthis: & QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {demth_ZN6QImage4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QImage::devicePixelRatio();
impl /*struct*/ QImage {
  pub fn devicePixelRatio<RetType, T: QImage_devicePixelRatio<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.devicePixelRatio(self);
    // return 1;
  }
}

pub trait QImage_devicePixelRatio<RetType> {
  fn devicePixelRatio(self , rsthis: & QImage) -> RetType;
}

  // proto:  qreal QImage::devicePixelRatio();
impl<'a> /*trait*/ QImage_devicePixelRatio<f64> for () {
  fn devicePixelRatio(self , rsthis: & QImage) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage16devicePixelRatioEv()};
    let mut ret = unsafe {_ZNK6QImage16devicePixelRatioEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  int QImage::devType();
impl /*struct*/ QImage {
  pub fn devType<RetType, T: QImage_devType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.devType(self);
    // return 1;
  }
}

pub trait QImage_devType<RetType> {
  fn devType(self , rsthis: & QImage) -> RetType;
}

  // proto:  int QImage::devType();
impl<'a> /*trait*/ QImage_devType<i32> for () {
  fn devType(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage7devTypeEv()};
    let mut ret = unsafe {_ZNK6QImage7devTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QImage::valid(const QPoint & pt);
impl /*struct*/ QImage {
  pub fn valid<RetType, T: QImage_valid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.valid(self);
    // return 1;
  }
}

pub trait QImage_valid<RetType> {
  fn valid(self , rsthis: & QImage) -> RetType;
}

  // proto:  bool QImage::valid(const QPoint & pt);
impl<'a> /*trait*/ QImage_valid<i8> for (&'a QPoint) {
  fn valid(self , rsthis: & QImage) -> i8 {
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
  pub fn pixelIndex<RetType, T: QImage_pixelIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pixelIndex(self);
    // return 1;
  }
}

pub trait QImage_pixelIndex<RetType> {
  fn pixelIndex(self , rsthis: & QImage) -> RetType;
}

  // proto:  int QImage::pixelIndex(const QPoint & pt);
impl<'a> /*trait*/ QImage_pixelIndex<i32> for (&'a QPoint) {
  fn pixelIndex(self , rsthis: & QImage) -> i32 {
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
  pub fn setDotsPerMeterX<RetType, T: QImage_setDotsPerMeterX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDotsPerMeterX(self);
    // return 1;
  }
}

pub trait QImage_setDotsPerMeterX<RetType> {
  fn setDotsPerMeterX(self , rsthis: & QImage) -> RetType;
}

  // proto:  void QImage::setDotsPerMeterX(int );
impl<'a> /*trait*/ QImage_setDotsPerMeterX<()> for (i32) {
  fn setDotsPerMeterX(self , rsthis: & QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImage16setDotsPerMeterXEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QImage16setDotsPerMeterXEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QImage::setPixel(int x, int y, uint index_or_rgb);
impl<'a> /*trait*/ QImage_setPixel<()> for (i32, i32, u32) {
  fn setPixel(self , rsthis: & QImage) -> () {
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
  pub fn load<RetType, T: QImage_load<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.load(self);
    // return 1;
  }
}

pub trait QImage_load<RetType> {
  fn load(self , rsthis: & QImage) -> RetType;
}

  // proto:  bool QImage::load(const QString & fileName, const char * format);
impl<'a> /*trait*/ QImage_load<i8> for (&'a QString, &'a  String) {
  fn load(self , rsthis: & QImage) -> i8 {
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
  pub fn colorTable<RetType, T: QImage_colorTable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.colorTable(self);
    // return 1;
  }
}

pub trait QImage_colorTable<RetType> {
  fn colorTable(self , rsthis: & QImage) -> RetType;
}

  // proto:  QVector<QRgb> QImage::colorTable();
impl<'a> /*trait*/ QImage_colorTable<()> for () {
  fn colorTable(self , rsthis: & QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage10colorTableEv()};
     unsafe {_ZNK6QImage10colorTableEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QImage::size();
impl /*struct*/ QImage {
  pub fn size<RetType, T: QImage_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QImage_size<RetType> {
  fn size(self , rsthis: & QImage) -> RetType;
}

  // proto:  QSize QImage::size();
impl<'a> /*trait*/ QImage_size<QSize> for () {
  fn size(self , rsthis: & QImage) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage4sizeEv()};
    let mut ret = unsafe {_ZNK6QImage4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QImage::height();
impl /*struct*/ QImage {
  pub fn height<RetType, T: QImage_height<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QImage_height<RetType> {
  fn height(self , rsthis: & QImage) -> RetType;
}

  // proto:  int QImage::height();
impl<'a> /*trait*/ QImage_height<i32> for () {
  fn height(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage6heightEv()};
    let mut ret = unsafe {_ZNK6QImage6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QImage::pixelIndex(int x, int y);
impl<'a> /*trait*/ QImage_pixelIndex<i32> for (i32, i32) {
  fn pixelIndex(self , rsthis: & QImage) -> i32 {
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
  pub fn width<RetType, T: QImage_width<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QImage_width<RetType> {
  fn width(self , rsthis: & QImage) -> RetType;
}

  // proto:  int QImage::width();
impl<'a> /*trait*/ QImage_width<i32> for () {
  fn width(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5widthEv()};
    let mut ret = unsafe {_ZNK6QImage5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QImage::load(QIODevice * device, const char * format);
impl<'a> /*trait*/ QImage_load<i8> for (&'a QIODevice, &'a  String) {
  fn load(self , rsthis: & QImage) -> i8 {
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
impl<'a> /*trait*/ QImage_New for () {
  fn New(self) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImageC1Ev()};
    let ctysz: c_int = unsafe{QImage_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN6QImageC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN6QImageC1Ev()} as u64;
    let rsthis = QImage{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  uchar * QImage::scanLine(int );
impl /*struct*/ QImage {
  pub fn scanLine<RetType, T: QImage_scanLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scanLine(self);
    // return 1;
  }
}

pub trait QImage_scanLine<RetType> {
  fn scanLine(self , rsthis: & QImage) -> RetType;
}

  // proto:  uchar * QImage::scanLine(int );
impl<'a> /*trait*/ QImage_scanLine<String> for (i32) {
  fn scanLine(self , rsthis: & QImage) -> String {
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
  pub fn bytesPerLine<RetType, T: QImage_bytesPerLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bytesPerLine(self);
    // return 1;
  }
}

pub trait QImage_bytesPerLine<RetType> {
  fn bytesPerLine(self , rsthis: & QImage) -> RetType;
}

  // proto:  int QImage::bytesPerLine();
impl<'a> /*trait*/ QImage_bytesPerLine<i32> for () {
  fn bytesPerLine(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage12bytesPerLineEv()};
    let mut ret = unsafe {_ZNK6QImage12bytesPerLineEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  qint64 QImage::cacheKey();
impl /*struct*/ QImage {
  pub fn cacheKey<RetType, T: QImage_cacheKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cacheKey(self);
    // return 1;
  }
}

pub trait QImage_cacheKey<RetType> {
  fn cacheKey(self , rsthis: & QImage) -> RetType;
}

  // proto:  qint64 QImage::cacheKey();
impl<'a> /*trait*/ QImage_cacheKey<i64> for () {
  fn cacheKey(self , rsthis: & QImage) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage8cacheKeyEv()};
    let mut ret = unsafe {_ZNK6QImage8cacheKeyEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  QRgb QImage::pixel(int x, int y);
impl<'a> /*trait*/ QImage_pixel<u32> for (i32, i32) {
  fn pixel(self , rsthis: & QImage) -> u32 {
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
  pub fn Free<RetType, T: QImage_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QImage_Free<RetType> {
  fn Free(self , rsthis: & QImage) -> RetType;
}

  // proto:  void QImage::~QImage();
impl<'a> /*trait*/ QImage_Free<()> for () {
  fn Free(self , rsthis: & QImage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QImageD0Ev()};
     unsafe {_ZN6QImageD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QImage::save(const QString & fileName, const char * format, int quality);
impl<'a> /*trait*/ QImage_save<i8> for (&'a QString, &'a  String, i32) {
  fn save(self , rsthis: & QImage) -> i8 {
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
  pub fn setColor<RetType, T: QImage_setColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColor(self);
    // return 1;
  }
}

pub trait QImage_setColor<RetType> {
  fn setColor(self , rsthis: & QImage) -> RetType;
}

  // proto:  void QImage::setColor(int i, QRgb c);
impl<'a> /*trait*/ QImage_setColor<()> for (i32, u32) {
  fn setColor(self , rsthis: & QImage) -> () {
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
  pub fn isNull<RetType, T: QImage_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QImage_isNull<RetType> {
  fn isNull(self , rsthis: & QImage) -> RetType;
}

  // proto:  bool QImage::isNull();
impl<'a> /*trait*/ QImage_isNull<i8> for () {
  fn isNull(self , rsthis: & QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage6isNullEv()};
    let mut ret = unsafe {_ZNK6QImage6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QImage::byteCount();
impl /*struct*/ QImage {
  pub fn byteCount<RetType, T: QImage_byteCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.byteCount(self);
    // return 1;
  }
}

pub trait QImage_byteCount<RetType> {
  fn byteCount(self , rsthis: & QImage) -> RetType;
}

  // proto:  int QImage::byteCount();
impl<'a> /*trait*/ QImage_byteCount<i32> for () {
  fn byteCount(self , rsthis: & QImage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage9byteCountEv()};
    let mut ret = unsafe {_ZNK6QImage9byteCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QImage::valid(int x, int y);
impl<'a> /*trait*/ QImage_valid<i8> for (i32, i32) {
  fn valid(self , rsthis: & QImage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QImage5validEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK6QImage5validEii(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

