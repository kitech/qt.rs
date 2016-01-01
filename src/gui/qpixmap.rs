// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
// src-file: /QtGui/qpixmap.h
// dst-file: /src/gui/qpixmap.rs
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
use super::super::core::qstring::QString; // 771
use super::super::core::qsize::QSize; // 771
use super::qtransform::QTransform; // 773
use super::qcolor::QColor; // 773
use super::qmatrix::QMatrix; // 773
use super::qregion::QRegion; // 773
use super::super::core::qrect::QRect; // 771
use super::super::core::qpoint::QPoint; // 771
use super::qbitmap::QBitmap; // 773
use super::super::core::qobject::QObject; // 771
use super::super::core::qbytearray::QByteArray; // 771
use super::qimage::QImage; // 773
use super::qimagereader::QImageReader; // 773
use super::qpaintengine::QPaintEngine; // 773
use super::super::core::qiodevice::QIODevice; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPixmap_Class_Size() -> c_int;
  // proto:  bool QPixmap::save(const QString & fileName, const char * format, int quality);
  fn _ZNK7QPixmap4saveERK7QStringPKci(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char, arg2: c_int) -> c_char;
  // proto:  void QPixmap::swap(QPixmap & other);
  fn demth_ZN7QPixmap4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QPixmap::isQBitmap();
  fn _ZNK7QPixmap9isQBitmapEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qreal QPixmap::devicePixelRatio();
  fn _ZNK7QPixmap16devicePixelRatioEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QPixmap::QPixmap(const QSize & );
  fn dector_ZN7QPixmapC1ERK5QSize(arg0: *mut c_void) -> *mut c_void;
  fn _ZN7QPixmapC1ERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPixmap::fill(const QPaintDevice * device, int xofs, int yofs);
  fn demth_ZN7QPixmap4fillEPK12QPaintDeviceii(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  void QPixmap::QPixmap(const QSize & s, int type);
  fn dector_ZN7QPixmapC1ERK5QSizei(arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  fn _ZN7QPixmapC1ERK5QSizei(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPixmap::fill(const QColor & fillColor);
  fn _ZN7QPixmap4fillERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QPixmap::devType();
  fn _ZNK7QPixmap7devTypeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QPixmap::scroll(int dx, int dy, int x, int y, int width, int height, QRegion * exposed);
  fn demth_ZN7QPixmap6scrollEiiiiiiP7QRegion(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: *mut c_void);
  // proto:  QPixmap QPixmap::copy(const QRect & rect);
  fn _ZNK7QPixmap4copyERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto: static QTransform QPixmap::trueMatrix(const QTransform & m, int w, int h);
  fn _ZN7QPixmap10trueMatrixERK10QTransformii(arg0: *mut c_void, arg1: c_int, arg2: c_int) -> *mut c_void;
  // proto:  void QPixmap::QPixmap(int w, int h);
  fn dector_ZN7QPixmapC1Eii(arg0: c_int, arg1: c_int) -> *mut c_void;
  fn _ZN7QPixmapC1Eii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto: static QPixmap QPixmap::grabWindow(WId , int x, int y, int w, int h);
  fn _ZN7QPixmap10grabWindowEiiiii(arg0: *mut c_uint, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) -> *mut c_void;
  // proto:  void QPixmap::fill(const QPaintDevice * device, const QPoint & ofs);
  fn _ZN7QPixmap4fillEPK12QPaintDeviceRK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QPixmap::isDetached();
  fn _ZNK7QPixmap10isDetachedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QPixmap::isNull();
  fn _ZNK7QPixmap6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QPixmap QPixmap::copy(int x, int y, int width, int height);
  fn demth_ZNK7QPixmap4copyEiiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> *mut c_void;
  // proto: static int QPixmap::defaultDepth();
  fn _ZN7QPixmap12defaultDepthEv() -> c_int;
  // proto:  void QPixmap::detach();
  fn _ZN7QPixmap6detachEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QPixmap::scroll(int dx, int dy, const QRect & rect, QRegion * exposed);
  fn _ZN7QPixmap6scrollEiiRK5QRectP7QRegion(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void, arg3: *mut c_void);
  // proto:  void QPixmap::setMask(const QBitmap & );
  fn _ZN7QPixmap7setMaskERK7QBitmap(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPixmap::QPixmap();
  fn dector_ZN7QPixmapC1Ev() -> *mut c_void;
  fn _ZN7QPixmapC1Ev(qthis: u64 /* *mut c_void*/);
  // proto: static QPixmap QPixmap::grabWidget(QObject * widget, const QRect & rect);
  fn _ZN7QPixmap10grabWidgetEP7QObjectRK5QRect(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QPixmap::QPixmap(const QPixmap & );
  fn dector_ZN7QPixmapC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN7QPixmapC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPixmap::setDevicePixelRatio(qreal scaleFactor);
  fn _ZN7QPixmap19setDevicePixelRatioEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QPixmap::QPixmap(const char *const [] xpm);
  fn dector_ZN7QPixmapC1EPKPKc(arg0: *mut *mut c_char) -> *mut c_void;
  fn _ZN7QPixmapC1EPKPKc(qthis: u64 /* *mut c_void*/, arg0: *mut *mut c_char);
  // proto:  qint64 QPixmap::cacheKey();
  fn _ZNK7QPixmap8cacheKeyEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  QBitmap QPixmap::createHeuristicMask(bool clipTight);
  fn _ZNK7QPixmap19createHeuristicMaskEb(qthis: u64 /* *mut c_void*/, arg0: c_char) -> *mut c_void;
  // proto:  int QPixmap::depth();
  fn _ZNK7QPixmap5depthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QImage QPixmap::toImage();
  fn _ZNK7QPixmap7toImageEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QPixmap QPixmap::grabWidget(QObject * widget, int x, int y, int w, int h);
  fn demth_ZN7QPixmap10grabWidgetEP7QObjectiiii(arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) -> *mut c_void;
  // proto:  QPlatformPixmap * QPixmap::handle();
  fn _ZNK7QPixmap6handleEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QPixmap::hasAlphaChannel();
  fn _ZNK7QPixmap15hasAlphaChannelEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QRect QPixmap::rect();
  fn _ZNK7QPixmap4rectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QMatrix QPixmap::trueMatrix(const QMatrix & m, int w, int h);
  fn _ZN7QPixmap10trueMatrixERK7QMatrixii(arg0: *mut c_void, arg1: c_int, arg2: c_int) -> *mut c_void;
  // proto:  QBitmap QPixmap::mask();
  fn _ZNK7QPixmap4maskEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QPixmap::width();
  fn _ZNK7QPixmap5widthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QPaintEngine * QPixmap::paintEngine();
  fn _ZNK7QPixmap11paintEngineEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPixmap::~QPixmap();
  fn _ZN7QPixmapD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QPixmap::height();
  fn _ZNK7QPixmap6heightEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QPixmap::save(QIODevice * device, const char * format, int quality);
  fn _ZNK7QPixmap4saveEP9QIODevicePKci(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char, arg2: c_int) -> c_char;
  // proto:  QSize QPixmap::size();
  fn _ZNK7QPixmap4sizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QPixmap::hasAlpha();
  fn _ZNK7QPixmap8hasAlphaEv(qthis: u64 /* *mut c_void*/) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QPixmap)=1
#[derive(Default)]
pub struct QPixmap {
  qbase: QPaintDevice,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QPixmap {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPixmap {
    return QPixmap{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QPixmap {
  type Target = QPaintDevice;

  fn deref(&self) -> &QPaintDevice {
    return & self.qbase;
  }
}
impl AsRef<QPaintDevice> for QPixmap {
  fn as_ref(& self) -> & QPaintDevice {
    return & self.qbase;
  }
}
  // proto:  bool QPixmap::save(const QString & fileName, const char * format, int quality);
impl /*struct*/ QPixmap {
  pub fn save<RetType, T: QPixmap_save<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.save(self);
    // return 1;
  }
}

pub trait QPixmap_save<RetType> {
  fn save(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  bool QPixmap::save(const QString & fileName, const char * format, int quality);
impl<'a> /*trait*/ QPixmap_save<i8> for (&'a QString, &'a  String, i32) {
  fn save(self , rsthis: & QPixmap) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4saveERK7QStringPKci()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZNK7QPixmap4saveERK7QStringPKci(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPixmap::swap(QPixmap & other);
impl /*struct*/ QPixmap {
  pub fn swap<RetType, T: QPixmap_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QPixmap_swap<RetType> {
  fn swap(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  void QPixmap::swap(QPixmap & other);
impl<'a> /*trait*/ QPixmap_swap<()> for (&'a QPixmap) {
  fn swap(self , rsthis: & QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {demth_ZN7QPixmap4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPixmap::isQBitmap();
impl /*struct*/ QPixmap {
  pub fn isQBitmap<RetType, T: QPixmap_isQBitmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isQBitmap(self);
    // return 1;
  }
}

pub trait QPixmap_isQBitmap<RetType> {
  fn isQBitmap(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  bool QPixmap::isQBitmap();
impl<'a> /*trait*/ QPixmap_isQBitmap<i8> for () {
  fn isQBitmap(self , rsthis: & QPixmap) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap9isQBitmapEv()};
    let mut ret = unsafe {_ZNK7QPixmap9isQBitmapEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QPixmap::devicePixelRatio();
impl /*struct*/ QPixmap {
  pub fn devicePixelRatio<RetType, T: QPixmap_devicePixelRatio<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.devicePixelRatio(self);
    // return 1;
  }
}

pub trait QPixmap_devicePixelRatio<RetType> {
  fn devicePixelRatio(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  qreal QPixmap::devicePixelRatio();
impl<'a> /*trait*/ QPixmap_devicePixelRatio<f64> for () {
  fn devicePixelRatio(self , rsthis: & QPixmap) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap16devicePixelRatioEv()};
    let mut ret = unsafe {_ZNK7QPixmap16devicePixelRatioEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPixmap::QPixmap(const QSize & );
impl /*struct*/ QPixmap {
  pub fn new<T: QPixmap_new>(value: T) -> QPixmap {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPixmap_new {
  fn new(self) -> QPixmap;
}

  // proto:  void QPixmap::QPixmap(const QSize & );
impl<'a> /*trait*/ QPixmap_new for (&'a QSize) {
  fn new(self) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapC1ERK5QSize()};
    let ctysz: c_int = unsafe{QPixmap_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN7QPixmapC1ERK5QSize(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN7QPixmapC1ERK5QSize(arg0)} as u64;
    let rsthis = QPixmap{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPixmap::fill(const QPaintDevice * device, int xofs, int yofs);
impl /*struct*/ QPixmap {
  pub fn fill<RetType, T: QPixmap_fill<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fill(self);
    // return 1;
  }
}

pub trait QPixmap_fill<RetType> {
  fn fill(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  void QPixmap::fill(const QPaintDevice * device, int xofs, int yofs);
impl<'a> /*trait*/ QPixmap_fill<()> for (&'a QPaintDevice, i32, i32) {
  fn fill(self , rsthis: & QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap4fillEPK12QPaintDeviceii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {demth_ZN7QPixmap4fillEPK12QPaintDeviceii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPixmap::QPixmap(const QSize & s, int type);
impl<'a> /*trait*/ QPixmap_new for (&'a QSize, i32) {
  fn new(self) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapC1ERK5QSizei()};
    let ctysz: c_int = unsafe{QPixmap_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    // unsafe {_ZN7QPixmapC1ERK5QSizei(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN7QPixmapC1ERK5QSizei(arg0, arg1)} as u64;
    let rsthis = QPixmap{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPixmap::fill(const QColor & fillColor);
impl<'a> /*trait*/ QPixmap_fill<()> for (&'a QColor) {
  fn fill(self , rsthis: & QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap4fillERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QPixmap4fillERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QPixmap::devType();
impl /*struct*/ QPixmap {
  pub fn devType<RetType, T: QPixmap_devType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.devType(self);
    // return 1;
  }
}

pub trait QPixmap_devType<RetType> {
  fn devType(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  int QPixmap::devType();
impl<'a> /*trait*/ QPixmap_devType<i32> for () {
  fn devType(self , rsthis: & QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap7devTypeEv()};
    let mut ret = unsafe {_ZNK7QPixmap7devTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QPixmap::scroll(int dx, int dy, int x, int y, int width, int height, QRegion * exposed);
impl /*struct*/ QPixmap {
  pub fn scroll<RetType, T: QPixmap_scroll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scroll(self);
    // return 1;
  }
}

pub trait QPixmap_scroll<RetType> {
  fn scroll(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  void QPixmap::scroll(int dx, int dy, int x, int y, int width, int height, QRegion * exposed);
impl<'a> /*trait*/ QPixmap_scroll<()> for (i32, i32, i32, i32, i32, i32, &'a QRegion) {
  fn scroll(self , rsthis: & QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap6scrollEiiiiiiP7QRegion()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    let arg6 = self.6.qclsinst  as *mut c_void;
     unsafe {demth_ZN7QPixmap6scrollEiiiiiiP7QRegion(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    // return 1;
  }
}

  // proto:  QPixmap QPixmap::copy(const QRect & rect);
impl /*struct*/ QPixmap {
  pub fn copy<RetType, T: QPixmap_copy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.copy(self);
    // return 1;
  }
}

pub trait QPixmap_copy<RetType> {
  fn copy(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  QPixmap QPixmap::copy(const QRect & rect);
impl<'a> /*trait*/ QPixmap_copy<QPixmap> for (&'a QRect) {
  fn copy(self , rsthis: & QPixmap) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4copyERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QPixmap4copyERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QPixmap::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QTransform QPixmap::trueMatrix(const QTransform & m, int w, int h);
impl /*struct*/ QPixmap {
  pub fn trueMatrix_s<RetType, T: QPixmap_trueMatrix_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.trueMatrix_s();
    // return 1;
  }
}

pub trait QPixmap_trueMatrix_s<RetType> {
  fn trueMatrix_s(self ) -> RetType;
}

  // proto: static QTransform QPixmap::trueMatrix(const QTransform & m, int w, int h);
impl<'a> /*trait*/ QPixmap_trueMatrix_s<QTransform> for (&'a QTransform, i32, i32) {
  fn trueMatrix_s(self ) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap10trueMatrixERK10QTransformii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN7QPixmap10trueMatrixERK10QTransformii(arg0, arg1, arg2)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPixmap::QPixmap(int w, int h);
impl<'a> /*trait*/ QPixmap_new for (i32, i32) {
  fn new(self) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapC1Eii()};
    let ctysz: c_int = unsafe{QPixmap_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    // unsafe {_ZN7QPixmapC1Eii(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN7QPixmapC1Eii(arg0, arg1)} as u64;
    let rsthis = QPixmap{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static QPixmap QPixmap::grabWindow(WId , int x, int y, int w, int h);
impl /*struct*/ QPixmap {
  pub fn grabWindow_s<RetType, T: QPixmap_grabWindow_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.grabWindow_s();
    // return 1;
  }
}

pub trait QPixmap_grabWindow_s<RetType> {
  fn grabWindow_s(self ) -> RetType;
}

  // proto: static QPixmap QPixmap::grabWindow(WId , int x, int y, int w, int h);
impl<'a> /*trait*/ QPixmap_grabWindow_s<QPixmap> for (*mut i32, i32, i32, i32, i32) {
  fn grabWindow_s(self ) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap10grabWindowEiiiii()};
    let arg0 = self.0  as *mut c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let mut ret = unsafe {_ZN7QPixmap10grabWindowEiiiii(arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QPixmap::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPixmap::fill(const QPaintDevice * device, const QPoint & ofs);
impl<'a> /*trait*/ QPixmap_fill<()> for (&'a QPaintDevice, &'a QPoint) {
  fn fill(self , rsthis: & QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap4fillEPK12QPaintDeviceRK6QPoint()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QPixmap4fillEPK12QPaintDeviceRK6QPoint(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QPixmap::isDetached();
impl /*struct*/ QPixmap {
  pub fn isDetached<RetType, T: QPixmap_isDetached<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDetached(self);
    // return 1;
  }
}

pub trait QPixmap_isDetached<RetType> {
  fn isDetached(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  bool QPixmap::isDetached();
impl<'a> /*trait*/ QPixmap_isDetached<i8> for () {
  fn isDetached(self , rsthis: & QPixmap) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap10isDetachedEv()};
    let mut ret = unsafe {_ZNK7QPixmap10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QPixmap::isNull();
impl /*struct*/ QPixmap {
  pub fn isNull<RetType, T: QPixmap_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QPixmap_isNull<RetType> {
  fn isNull(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  bool QPixmap::isNull();
impl<'a> /*trait*/ QPixmap_isNull<i8> for () {
  fn isNull(self , rsthis: & QPixmap) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap6isNullEv()};
    let mut ret = unsafe {_ZNK7QPixmap6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPixmap QPixmap::copy(int x, int y, int width, int height);
impl<'a> /*trait*/ QPixmap_copy<QPixmap> for (i32, i32, i32, i32) {
  fn copy(self , rsthis: & QPixmap) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4copyEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {demth_ZNK7QPixmap4copyEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QPixmap::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static int QPixmap::defaultDepth();
impl /*struct*/ QPixmap {
  pub fn defaultDepth_s<RetType, T: QPixmap_defaultDepth_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.defaultDepth_s();
    // return 1;
  }
}

pub trait QPixmap_defaultDepth_s<RetType> {
  fn defaultDepth_s(self ) -> RetType;
}

  // proto: static int QPixmap::defaultDepth();
impl<'a> /*trait*/ QPixmap_defaultDepth_s<i32> for () {
  fn defaultDepth_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap12defaultDepthEv()};
    let mut ret = unsafe {_ZN7QPixmap12defaultDepthEv()};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QPixmap::detach();
impl /*struct*/ QPixmap {
  pub fn detach<RetType, T: QPixmap_detach<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.detach(self);
    // return 1;
  }
}

pub trait QPixmap_detach<RetType> {
  fn detach(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  void QPixmap::detach();
impl<'a> /*trait*/ QPixmap_detach<()> for () {
  fn detach(self , rsthis: & QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap6detachEv()};
     unsafe {_ZN7QPixmap6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPixmap::scroll(int dx, int dy, const QRect & rect, QRegion * exposed);
impl<'a> /*trait*/ QPixmap_scroll<()> for (i32, i32, &'a QRect, &'a QRegion) {
  fn scroll(self , rsthis: & QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap6scrollEiiRK5QRectP7QRegion()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
     unsafe {_ZN7QPixmap6scrollEiiRK5QRectP7QRegion(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QPixmap::setMask(const QBitmap & );
impl /*struct*/ QPixmap {
  pub fn setMask<RetType, T: QPixmap_setMask<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMask(self);
    // return 1;
  }
}

pub trait QPixmap_setMask<RetType> {
  fn setMask(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  void QPixmap::setMask(const QBitmap & );
impl<'a> /*trait*/ QPixmap_setMask<()> for (&'a QBitmap) {
  fn setMask(self , rsthis: & QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap7setMaskERK7QBitmap()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QPixmap7setMaskERK7QBitmap(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPixmap::QPixmap();
impl<'a> /*trait*/ QPixmap_new for () {
  fn new(self) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapC1Ev()};
    let ctysz: c_int = unsafe{QPixmap_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN7QPixmapC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN7QPixmapC1Ev()} as u64;
    let rsthis = QPixmap{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static QPixmap QPixmap::grabWidget(QObject * widget, const QRect & rect);
impl /*struct*/ QPixmap {
  pub fn grabWidget_s<RetType, T: QPixmap_grabWidget_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.grabWidget_s();
    // return 1;
  }
}

pub trait QPixmap_grabWidget_s<RetType> {
  fn grabWidget_s(self ) -> RetType;
}

  // proto: static QPixmap QPixmap::grabWidget(QObject * widget, const QRect & rect);
impl<'a> /*trait*/ QPixmap_grabWidget_s<QPixmap> for (&'a QObject, &'a QRect) {
  fn grabWidget_s(self ) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap10grabWidgetEP7QObjectRK5QRect()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QPixmap10grabWidgetEP7QObjectRK5QRect(arg0, arg1)};
    let mut ret1 = QPixmap::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPixmap::QPixmap(const QPixmap & );
impl<'a> /*trait*/ QPixmap_new for (&'a QPixmap) {
  fn new(self) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapC1ERKS_()};
    let ctysz: c_int = unsafe{QPixmap_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN7QPixmapC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN7QPixmapC1ERKS_(arg0)} as u64;
    let rsthis = QPixmap{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPixmap::setDevicePixelRatio(qreal scaleFactor);
impl /*struct*/ QPixmap {
  pub fn setDevicePixelRatio<RetType, T: QPixmap_setDevicePixelRatio<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDevicePixelRatio(self);
    // return 1;
  }
}

pub trait QPixmap_setDevicePixelRatio<RetType> {
  fn setDevicePixelRatio(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  void QPixmap::setDevicePixelRatio(qreal scaleFactor);
impl<'a> /*trait*/ QPixmap_setDevicePixelRatio<()> for (f64) {
  fn setDevicePixelRatio(self , rsthis: & QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap19setDevicePixelRatioEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN7QPixmap19setDevicePixelRatioEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPixmap::QPixmap(const char *const [] xpm);
impl<'a> /*trait*/ QPixmap_new for (&'a  Vec<&'a  i8>) {
  fn new(self) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapC1EPKPKc()};
    let ctysz: c_int = unsafe{QPixmap_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.as_ptr()  as *mut *mut c_char;
    // unsafe {_ZN7QPixmapC1EPKPKc(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN7QPixmapC1EPKPKc(arg0)} as u64;
    let rsthis = QPixmap{qbase: QPaintDevice::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qint64 QPixmap::cacheKey();
impl /*struct*/ QPixmap {
  pub fn cacheKey<RetType, T: QPixmap_cacheKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cacheKey(self);
    // return 1;
  }
}

pub trait QPixmap_cacheKey<RetType> {
  fn cacheKey(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  qint64 QPixmap::cacheKey();
impl<'a> /*trait*/ QPixmap_cacheKey<i64> for () {
  fn cacheKey(self , rsthis: & QPixmap) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap8cacheKeyEv()};
    let mut ret = unsafe {_ZNK7QPixmap8cacheKeyEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  QBitmap QPixmap::createHeuristicMask(bool clipTight);
impl /*struct*/ QPixmap {
  pub fn createHeuristicMask<RetType, T: QPixmap_createHeuristicMask<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createHeuristicMask(self);
    // return 1;
  }
}

pub trait QPixmap_createHeuristicMask<RetType> {
  fn createHeuristicMask(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  QBitmap QPixmap::createHeuristicMask(bool clipTight);
impl<'a> /*trait*/ QPixmap_createHeuristicMask<QBitmap> for (i8) {
  fn createHeuristicMask(self , rsthis: & QPixmap) -> QBitmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap19createHeuristicMaskEb()};
    let arg0 = self  as c_char;
    let mut ret = unsafe {_ZNK7QPixmap19createHeuristicMaskEb(rsthis.qclsinst, arg0)};
    let mut ret1 = QBitmap::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QPixmap::depth();
impl /*struct*/ QPixmap {
  pub fn depth<RetType, T: QPixmap_depth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.depth(self);
    // return 1;
  }
}

pub trait QPixmap_depth<RetType> {
  fn depth(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  int QPixmap::depth();
impl<'a> /*trait*/ QPixmap_depth<i32> for () {
  fn depth(self , rsthis: & QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap5depthEv()};
    let mut ret = unsafe {_ZNK7QPixmap5depthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QImage QPixmap::toImage();
impl /*struct*/ QPixmap {
  pub fn toImage<RetType, T: QPixmap_toImage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toImage(self);
    // return 1;
  }
}

pub trait QPixmap_toImage<RetType> {
  fn toImage(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  QImage QPixmap::toImage();
impl<'a> /*trait*/ QPixmap_toImage<QImage> for () {
  fn toImage(self , rsthis: & QPixmap) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap7toImageEv()};
    let mut ret = unsafe {_ZNK7QPixmap7toImageEv(rsthis.qclsinst)};
    let mut ret1 = QImage::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QPixmap QPixmap::grabWidget(QObject * widget, int x, int y, int w, int h);
impl<'a> /*trait*/ QPixmap_grabWidget_s<QPixmap> for (&'a QObject, i32, i32, i32, i32) {
  fn grabWidget_s(self ) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap10grabWidgetEP7QObjectiiii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let mut ret = unsafe {demth_ZN7QPixmap10grabWidgetEP7QObjectiiii(arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QPixmap::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPlatformPixmap * QPixmap::handle();
impl /*struct*/ QPixmap {
  pub fn handle<RetType, T: QPixmap_handle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.handle(self);
    // return 1;
  }
}

pub trait QPixmap_handle<RetType> {
  fn handle(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  QPlatformPixmap * QPixmap::handle();
impl<'a> /*trait*/ QPixmap_handle<()> for () {
  fn handle(self , rsthis: & QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap6handleEv()};
     unsafe {_ZNK7QPixmap6handleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QPixmap::hasAlphaChannel();
impl /*struct*/ QPixmap {
  pub fn hasAlphaChannel<RetType, T: QPixmap_hasAlphaChannel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasAlphaChannel(self);
    // return 1;
  }
}

pub trait QPixmap_hasAlphaChannel<RetType> {
  fn hasAlphaChannel(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  bool QPixmap::hasAlphaChannel();
impl<'a> /*trait*/ QPixmap_hasAlphaChannel<i8> for () {
  fn hasAlphaChannel(self , rsthis: & QPixmap) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap15hasAlphaChannelEv()};
    let mut ret = unsafe {_ZNK7QPixmap15hasAlphaChannelEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRect QPixmap::rect();
impl /*struct*/ QPixmap {
  pub fn rect<RetType, T: QPixmap_rect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QPixmap_rect<RetType> {
  fn rect(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  QRect QPixmap::rect();
impl<'a> /*trait*/ QPixmap_rect<QRect> for () {
  fn rect(self , rsthis: & QPixmap) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4rectEv()};
    let mut ret = unsafe {_ZNK7QPixmap4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QMatrix QPixmap::trueMatrix(const QMatrix & m, int w, int h);
impl<'a> /*trait*/ QPixmap_trueMatrix_s<QMatrix> for (&'a QMatrix, i32, i32) {
  fn trueMatrix_s(self ) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap10trueMatrixERK7QMatrixii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN7QPixmap10trueMatrixERK7QMatrixii(arg0, arg1, arg2)};
    let mut ret1 = QMatrix::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QBitmap QPixmap::mask();
impl /*struct*/ QPixmap {
  pub fn mask<RetType, T: QPixmap_mask<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mask(self);
    // return 1;
  }
}

pub trait QPixmap_mask<RetType> {
  fn mask(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  QBitmap QPixmap::mask();
impl<'a> /*trait*/ QPixmap_mask<QBitmap> for () {
  fn mask(self , rsthis: & QPixmap) -> QBitmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4maskEv()};
    let mut ret = unsafe {_ZNK7QPixmap4maskEv(rsthis.qclsinst)};
    let mut ret1 = QBitmap::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QPixmap::width();
impl /*struct*/ QPixmap {
  pub fn width<RetType, T: QPixmap_width<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QPixmap_width<RetType> {
  fn width(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  int QPixmap::width();
impl<'a> /*trait*/ QPixmap_width<i32> for () {
  fn width(self , rsthis: & QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap5widthEv()};
    let mut ret = unsafe {_ZNK7QPixmap5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QPaintEngine * QPixmap::paintEngine();
impl /*struct*/ QPixmap {
  pub fn paintEngine<RetType, T: QPixmap_paintEngine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paintEngine(self);
    // return 1;
  }
}

pub trait QPixmap_paintEngine<RetType> {
  fn paintEngine(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  QPaintEngine * QPixmap::paintEngine();
impl<'a> /*trait*/ QPixmap_paintEngine<QPaintEngine> for () {
  fn paintEngine(self , rsthis: & QPixmap) -> QPaintEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap11paintEngineEv()};
    let mut ret = unsafe {_ZNK7QPixmap11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPixmap::~QPixmap();
impl /*struct*/ QPixmap {
  pub fn free<RetType, T: QPixmap_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QPixmap_free<RetType> {
  fn free(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  void QPixmap::~QPixmap();
impl<'a> /*trait*/ QPixmap_free<()> for () {
  fn free(self , rsthis: & QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapD0Ev()};
     unsafe {_ZN7QPixmapD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QPixmap::height();
impl /*struct*/ QPixmap {
  pub fn height<RetType, T: QPixmap_height<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QPixmap_height<RetType> {
  fn height(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  int QPixmap::height();
impl<'a> /*trait*/ QPixmap_height<i32> for () {
  fn height(self , rsthis: & QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap6heightEv()};
    let mut ret = unsafe {_ZNK7QPixmap6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QPixmap::save(QIODevice * device, const char * format, int quality);
impl<'a> /*trait*/ QPixmap_save<i8> for (&'a QIODevice, &'a  String, i32) {
  fn save(self , rsthis: & QPixmap) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4saveEP9QIODevicePKci()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZNK7QPixmap4saveEP9QIODevicePKci(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSize QPixmap::size();
impl /*struct*/ QPixmap {
  pub fn size<RetType, T: QPixmap_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QPixmap_size<RetType> {
  fn size(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  QSize QPixmap::size();
impl<'a> /*trait*/ QPixmap_size<QSize> for () {
  fn size(self , rsthis: & QPixmap) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4sizeEv()};
    let mut ret = unsafe {_ZNK7QPixmap4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPixmap::hasAlpha();
impl /*struct*/ QPixmap {
  pub fn hasAlpha<RetType, T: QPixmap_hasAlpha<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasAlpha(self);
    // return 1;
  }
}

pub trait QPixmap_hasAlpha<RetType> {
  fn hasAlpha(self , rsthis: & QPixmap) -> RetType;
}

  // proto:  bool QPixmap::hasAlpha();
impl<'a> /*trait*/ QPixmap_hasAlpha<i8> for () {
  fn hasAlpha(self , rsthis: & QPixmap) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap8hasAlphaEv()};
    let mut ret = unsafe {_ZNK7QPixmap8hasAlphaEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

