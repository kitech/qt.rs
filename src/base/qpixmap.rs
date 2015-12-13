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
use super::qpaintdevice::QPaintDevice;
use super::qcolor::QColor;
use super::qregion::QRegion;
use super::qrect::QRect;
use super::qtransform::QTransform;
use super::qpoint::QPoint;
use super::qbitmap::QBitmap;
use super::qobject::QObject;
use super::qmatrix::QMatrix;
use super::qiodevice::QIODevice;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QPixmap::save(const QString & fileName, const char * format, int quality);
  fn _ZNK7QPixmap4saveERK7QStringPKci(arg0: *const c_void, arg1: *const c_char, arg2: c_int) -> i32;
  // proto: void QPixmap::swap(QPixmap & other);
  fn _ZN7QPixmap4swapERS_(arg0: *mut c_void) -> i32;
  // proto: bool QPixmap::isQBitmap();
  fn _ZNK7QPixmap9isQBitmapEv() -> i32;
  // proto: double QPixmap::devicePixelRatio();
  fn _ZNK7QPixmap16devicePixelRatioEv() -> i32;
  // proto: void QPixmap::NewQPixmap(const QSize & );
  fn _ZN7QPixmapC1ERK5QSize(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QPixmap::fill(const QPaintDevice * device, int xofs, int yofs);
  fn _ZN7QPixmap4fillEPK12QPaintDeviceii(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: void QPixmap::NewQPixmap(const QSize & s, int type);
  fn _ZN7QPixmapC1ERK5QSizei(qthis: *mut c_void, arg0: *const c_void, arg1: c_int) -> i32;
  // proto: void QPixmap::fill(const QColor & fillColor);
  fn _ZN7QPixmap4fillERK6QColor(arg0: *const c_void) -> i32;
  // proto: int QPixmap::devType();
  fn _ZNK7QPixmap7devTypeEv() -> i32;
  // proto: void QPixmap::scroll(int dx, int dy, int x, int y, int width, int height, QRegion * exposed);
  fn _ZN7QPixmap6scrollEiiiiiiP7QRegion(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: *mut c_void) -> i32;
  // proto: QPixmap QPixmap::copy(const QRect & rect);
  fn _ZNK7QPixmap4copyERK5QRect(arg0: *const c_void) -> i32;
  // proto: QTransform QPixmap::trueMatrix(const QTransform & m, int w, int h);
  fn _ZN7QPixmap10trueMatrixERK10QTransformii(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: void QPixmap::NewQPixmap(int w, int h);
  fn _ZN7QPixmapC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> i32;
  // proto: QPixmap QPixmap::grabWindow(WId , int x, int y, int w, int h);
  fn _ZN7QPixmap10grabWindowEiiiii(arg0: *mut c_uint, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) -> i32;
  // proto: void QPixmap::fill(const QPaintDevice * device, const QPoint & ofs);
  fn _ZN7QPixmap4fillEPK12QPaintDeviceRK6QPoint(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: bool QPixmap::isDetached();
  fn _ZNK7QPixmap10isDetachedEv() -> i32;
  // proto: bool QPixmap::isNull();
  fn _ZNK7QPixmap6isNullEv() -> i32;
  // proto: QPixmap QPixmap::copy(int x, int y, int width, int height);
  fn _ZNK7QPixmap4copyEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: int QPixmap::defaultDepth();
  fn _ZN7QPixmap12defaultDepthEv() -> i32;
  // proto: void QPixmap::detach();
  fn _ZN7QPixmap6detachEv() -> i32;
  // proto: void QPixmap::scroll(int dx, int dy, const QRect & rect, QRegion * exposed);
  fn _ZN7QPixmap6scrollEiiRK5QRectP7QRegion(arg0: c_int, arg1: c_int, arg2: *const c_void, arg3: *mut c_void) -> i32;
  // proto: void QPixmap::setMask(const QBitmap & );
  fn _ZN7QPixmap7setMaskERK7QBitmap(arg0: *const c_void) -> i32;
  // proto: void QPixmap::NewQPixmap();
  fn _ZN7QPixmapC1Ev(qthis: *mut c_void) -> i32;
  // proto: QPixmap QPixmap::grabWidget(QObject * widget, const QRect & rect);
  fn _ZN7QPixmap10grabWidgetEP7QObjectRK5QRect(arg0: *mut c_void, arg1: *const c_void) -> i32;
  // proto: void QPixmap::NewQPixmap(const QPixmap & );
  fn _ZN7QPixmapC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QPixmap::setDevicePixelRatio(qreal scaleFactor);
  fn _ZN7QPixmap19setDevicePixelRatioEd(arg0: c_double) -> i32;
  // proto: void QPixmap::NewQPixmap(const char *const [] xpm);
  fn _ZN7QPixmapC1EPKPKc(qthis: *mut c_void, arg0: *const *const c_char) -> i32;
  // proto: long long QPixmap::cacheKey();
  fn _ZNK7QPixmap8cacheKeyEv() -> i32;
  // proto: QBitmap QPixmap::createHeuristicMask(bool clipTight);
  fn _ZNK7QPixmap19createHeuristicMaskEb(arg0: int8_t) -> i32;
  // proto: int QPixmap::depth();
  fn _ZNK7QPixmap5depthEv() -> i32;
  // proto: QImage QPixmap::toImage();
  fn _ZNK7QPixmap7toImageEv() -> i32;
  // proto: QPixmap QPixmap::grabWidget(QObject * widget, int x, int y, int w, int h);
  fn _ZN7QPixmap10grabWidgetEP7QObjectiiii(arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) -> i32;
  // proto: QPlatformPixmap * QPixmap::handle();
  fn _ZNK7QPixmap6handleEv() -> i32;
  // proto: bool QPixmap::hasAlphaChannel();
  fn _ZNK7QPixmap15hasAlphaChannelEv() -> i32;
  // proto: QRect QPixmap::rect();
  fn _ZNK7QPixmap4rectEv() -> i32;
  // proto: QMatrix QPixmap::trueMatrix(const QMatrix & m, int w, int h);
  fn _ZN7QPixmap10trueMatrixERK7QMatrixii(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: QBitmap QPixmap::mask();
  fn _ZNK7QPixmap4maskEv() -> i32;
  // proto: int QPixmap::width();
  fn _ZNK7QPixmap5widthEv() -> i32;
  // proto: QPaintEngine * QPixmap::paintEngine();
  fn _ZNK7QPixmap11paintEngineEv() -> i32;
  // proto: void QPixmap::FreeQPixmap();
  fn _ZN7QPixmapD0Ev() -> i32;
  // proto: int QPixmap::height();
  fn _ZNK7QPixmap6heightEv() -> i32;
  // proto: bool QPixmap::save(QIODevice * device, const char * format, int quality);
  fn _ZNK7QPixmap4saveEP9QIODevicePKci(arg0: *mut c_void, arg1: *const c_char, arg2: c_int) -> i32;
  // proto: QSize QPixmap::size();
  fn _ZNK7QPixmap4sizeEv() -> i32;
  // proto: bool QPixmap::hasAlpha();
  fn _ZNK7QPixmap8hasAlphaEv() -> i32;
}

// body block begin
// class sizeof(QPixmap)=1
pub struct QPixmap {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPixmap {
  pub fn save<T: QPixmap_save>(&mut self, value: T) -> i32 {
    value.save(self);
    return 1;
  }
}

pub trait QPixmap_save {
  fn save(self, this: &mut QPixmap) -> i32;
}

// proto: bool QPixmap::save(const QString & fileName, const char * format, int quality);
impl<'a> /*trait*/ QPixmap_save for (&'a  QString, &'a  String, i32) {
  fn save(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4saveERK7QStringPKci()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2  as c_int;
    unsafe {_ZNK7QPixmap4saveERK7QStringPKci(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn swap<T: QPixmap_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QPixmap_swap {
  fn swap(self, this: &mut QPixmap) -> i32;
}

// proto: void QPixmap::swap(QPixmap & other);
impl<'a> /*trait*/ QPixmap_swap for (&'a mut QPixmap) {
  fn swap(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QPixmap4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn isQBitmap<T: QPixmap_isQBitmap>(&mut self, value: T) -> i32 {
    value.isQBitmap(self);
    return 1;
  }
}

pub trait QPixmap_isQBitmap {
  fn isQBitmap(self, this: &mut QPixmap) -> i32;
}

// proto: bool QPixmap::isQBitmap();
impl<'a> /*trait*/ QPixmap_isQBitmap for () {
  fn isQBitmap(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap9isQBitmapEv()};
    unsafe {_ZNK7QPixmap9isQBitmapEv()};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn devicePixelRatio<T: QPixmap_devicePixelRatio>(&mut self, value: T) -> i32 {
    value.devicePixelRatio(self);
    return 1;
  }
}

pub trait QPixmap_devicePixelRatio {
  fn devicePixelRatio(self, this: &mut QPixmap) -> i32;
}

// proto: double QPixmap::devicePixelRatio();
impl<'a> /*trait*/ QPixmap_devicePixelRatio for () {
  fn devicePixelRatio(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap16devicePixelRatioEv()};
    unsafe {_ZNK7QPixmap16devicePixelRatioEv()};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn NewQPixmap<T: QPixmap_NewQPixmap>(value: T) -> QPixmap {
    let rsthis = value.NewQPixmap();
    return rsthis;
    // return 1;
  }
}

pub trait QPixmap_NewQPixmap {
  fn NewQPixmap(self) -> QPixmap;
}

// proto: void QPixmap::NewQPixmap(const QSize & );
impl<'a> /*trait*/ QPixmap_NewQPixmap for (&'a  QSize) {
  fn NewQPixmap(self) -> QPixmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapC1ERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QPixmapC1ERK5QSize(qthis, arg0)};
    let rsthis = QPixmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn fill<T: QPixmap_fill>(&mut self, value: T) -> i32 {
    value.fill(self);
    return 1;
  }
}

pub trait QPixmap_fill {
  fn fill(self, this: &mut QPixmap) -> i32;
}

// proto: void QPixmap::fill(const QPaintDevice * device, int xofs, int yofs);
impl<'a> /*trait*/ QPixmap_fill for (&'a  QPaintDevice, i32, i32) {
  fn fill(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap4fillEPK12QPaintDeviceii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN7QPixmap4fillEPK12QPaintDeviceii(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QPixmap::NewQPixmap(const QSize & s, int type);
impl<'a> /*trait*/ QPixmap_NewQPixmap for (&'a  QSize, i32) {
  fn NewQPixmap(self) -> QPixmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapC1ERK5QSizei()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QPixmapC1ERK5QSizei(qthis, arg0, arg1)};
    let rsthis = QPixmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QPixmap::fill(const QColor & fillColor);
impl<'a> /*trait*/ QPixmap_fill for (&'a  QColor) {
  fn fill(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap4fillERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QPixmap4fillERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn devType<T: QPixmap_devType>(&mut self, value: T) -> i32 {
    value.devType(self);
    return 1;
  }
}

pub trait QPixmap_devType {
  fn devType(self, this: &mut QPixmap) -> i32;
}

// proto: int QPixmap::devType();
impl<'a> /*trait*/ QPixmap_devType for () {
  fn devType(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap7devTypeEv()};
    unsafe {_ZNK7QPixmap7devTypeEv()};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn scroll<T: QPixmap_scroll>(&mut self, value: T) -> i32 {
    value.scroll(self);
    return 1;
  }
}

pub trait QPixmap_scroll {
  fn scroll(self, this: &mut QPixmap) -> i32;
}

// proto: void QPixmap::scroll(int dx, int dy, int x, int y, int width, int height, QRegion * exposed);
impl<'a> /*trait*/ QPixmap_scroll for (i32, i32, i32, i32, i32, i32, &'a mut QRegion) {
  fn scroll(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap6scrollEiiiiiiP7QRegion()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    let arg6 = self.6.qclsinst  as *mut c_void;
    unsafe {_ZN7QPixmap6scrollEiiiiiiP7QRegion(arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn copy<T: QPixmap_copy>(&mut self, value: T) -> i32 {
    value.copy(self);
    return 1;
  }
}

pub trait QPixmap_copy {
  fn copy(self, this: &mut QPixmap) -> i32;
}

// proto: QPixmap QPixmap::copy(const QRect & rect);
impl<'a> /*trait*/ QPixmap_copy for (&'a  QRect) {
  fn copy(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4copyERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QPixmap4copyERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn trueMatrix<T: QPixmap_trueMatrix>(&mut self, value: T) -> i32 {
    value.trueMatrix(self);
    return 1;
  }
}

pub trait QPixmap_trueMatrix {
  fn trueMatrix(self, this: &mut QPixmap) -> i32;
}

// proto: QTransform QPixmap::trueMatrix(const QTransform & m, int w, int h);
impl<'a> /*trait*/ QPixmap_trueMatrix for (&'a  QTransform, i32, i32) {
  fn trueMatrix(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap10trueMatrixERK10QTransformii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN7QPixmap10trueMatrixERK10QTransformii(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QPixmap::NewQPixmap(int w, int h);
impl<'a> /*trait*/ QPixmap_NewQPixmap for (i32, i32) {
  fn NewQPixmap(self) -> QPixmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QPixmapC1Eii(qthis, arg0, arg1)};
    let rsthis = QPixmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn grabWindow<T: QPixmap_grabWindow>(&mut self, value: T) -> i32 {
    value.grabWindow(self);
    return 1;
  }
}

pub trait QPixmap_grabWindow {
  fn grabWindow(self, this: &mut QPixmap) -> i32;
}

// proto: QPixmap QPixmap::grabWindow(WId , int x, int y, int w, int h);
impl<'a> /*trait*/ QPixmap_grabWindow for (*mut i32, i32, i32, i32, i32) {
  fn grabWindow(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap10grabWindowEiiiii()};
    let arg0 = self.0  as *mut c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    unsafe {_ZN7QPixmap10grabWindowEiiiii(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

// proto: void QPixmap::fill(const QPaintDevice * device, const QPoint & ofs);
impl<'a> /*trait*/ QPixmap_fill for (&'a  QPaintDevice, &'a  QPoint) {
  fn fill(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap4fillEPK12QPaintDeviceRK6QPoint()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN7QPixmap4fillEPK12QPaintDeviceRK6QPoint(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn isDetached<T: QPixmap_isDetached>(&mut self, value: T) -> i32 {
    value.isDetached(self);
    return 1;
  }
}

pub trait QPixmap_isDetached {
  fn isDetached(self, this: &mut QPixmap) -> i32;
}

// proto: bool QPixmap::isDetached();
impl<'a> /*trait*/ QPixmap_isDetached for () {
  fn isDetached(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap10isDetachedEv()};
    unsafe {_ZNK7QPixmap10isDetachedEv()};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn isNull<T: QPixmap_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QPixmap_isNull {
  fn isNull(self, this: &mut QPixmap) -> i32;
}

// proto: bool QPixmap::isNull();
impl<'a> /*trait*/ QPixmap_isNull for () {
  fn isNull(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap6isNullEv()};
    unsafe {_ZNK7QPixmap6isNullEv()};
    return 1;
  }
}

// proto: QPixmap QPixmap::copy(int x, int y, int width, int height);
impl<'a> /*trait*/ QPixmap_copy for (i32, i32, i32, i32) {
  fn copy(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4copyEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZNK7QPixmap4copyEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn defaultDepth<T: QPixmap_defaultDepth>(&mut self, value: T) -> i32 {
    value.defaultDepth(self);
    return 1;
  }
}

pub trait QPixmap_defaultDepth {
  fn defaultDepth(self, this: &mut QPixmap) -> i32;
}

// proto: int QPixmap::defaultDepth();
impl<'a> /*trait*/ QPixmap_defaultDepth for () {
  fn defaultDepth(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap12defaultDepthEv()};
    unsafe {_ZN7QPixmap12defaultDepthEv()};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn detach<T: QPixmap_detach>(&mut self, value: T) -> i32 {
    value.detach(self);
    return 1;
  }
}

pub trait QPixmap_detach {
  fn detach(self, this: &mut QPixmap) -> i32;
}

// proto: void QPixmap::detach();
impl<'a> /*trait*/ QPixmap_detach for () {
  fn detach(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap6detachEv()};
    unsafe {_ZN7QPixmap6detachEv()};
    return 1;
  }
}

// proto: void QPixmap::scroll(int dx, int dy, const QRect & rect, QRegion * exposed);
impl<'a> /*trait*/ QPixmap_scroll for (i32, i32, &'a  QRect, &'a mut QRegion) {
  fn scroll(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap6scrollEiiRK5QRectP7QRegion()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZN7QPixmap6scrollEiiRK5QRectP7QRegion(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn setMask<T: QPixmap_setMask>(&mut self, value: T) -> i32 {
    value.setMask(self);
    return 1;
  }
}

pub trait QPixmap_setMask {
  fn setMask(self, this: &mut QPixmap) -> i32;
}

// proto: void QPixmap::setMask(const QBitmap & );
impl<'a> /*trait*/ QPixmap_setMask for (&'a  QBitmap) {
  fn setMask(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap7setMaskERK7QBitmap()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QPixmap7setMaskERK7QBitmap(arg0)};
    return 1;
  }
}

// proto: void QPixmap::NewQPixmap();
impl<'a> /*trait*/ QPixmap_NewQPixmap for () {
  fn NewQPixmap(self) -> QPixmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapC1Ev()};
    unsafe {_ZN7QPixmapC1Ev(qthis)};
    let rsthis = QPixmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn grabWidget<T: QPixmap_grabWidget>(&mut self, value: T) -> i32 {
    value.grabWidget(self);
    return 1;
  }
}

pub trait QPixmap_grabWidget {
  fn grabWidget(self, this: &mut QPixmap) -> i32;
}

// proto: QPixmap QPixmap::grabWidget(QObject * widget, const QRect & rect);
impl<'a> /*trait*/ QPixmap_grabWidget for (&'a mut QObject, &'a  QRect) {
  fn grabWidget(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap10grabWidgetEP7QObjectRK5QRect()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN7QPixmap10grabWidgetEP7QObjectRK5QRect(arg0, arg1)};
    return 1;
  }
}

// proto: void QPixmap::NewQPixmap(const QPixmap & );
impl<'a> /*trait*/ QPixmap_NewQPixmap for (&'a  QPixmap) {
  fn NewQPixmap(self) -> QPixmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QPixmapC1ERKS_(qthis, arg0)};
    let rsthis = QPixmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn setDevicePixelRatio<T: QPixmap_setDevicePixelRatio>(&mut self, value: T) -> i32 {
    value.setDevicePixelRatio(self);
    return 1;
  }
}

pub trait QPixmap_setDevicePixelRatio {
  fn setDevicePixelRatio(self, this: &mut QPixmap) -> i32;
}

// proto: void QPixmap::setDevicePixelRatio(qreal scaleFactor);
impl<'a> /*trait*/ QPixmap_setDevicePixelRatio for (f64) {
  fn setDevicePixelRatio(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap19setDevicePixelRatioEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN7QPixmap19setDevicePixelRatioEd(arg0)};
    return 1;
  }
}

// proto: void QPixmap::NewQPixmap(const char *const [] xpm);
impl<'a> /*trait*/ QPixmap_NewQPixmap for (&'a  Vec<&'a  i8>) {
  fn NewQPixmap(self) -> QPixmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapC1EPKPKc()};
    let arg0 = 0  as *const *const c_char;
    unsafe {_ZN7QPixmapC1EPKPKc(qthis, arg0)};
    let rsthis = QPixmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn cacheKey<T: QPixmap_cacheKey>(&mut self, value: T) -> i32 {
    value.cacheKey(self);
    return 1;
  }
}

pub trait QPixmap_cacheKey {
  fn cacheKey(self, this: &mut QPixmap) -> i32;
}

// proto: long long QPixmap::cacheKey();
impl<'a> /*trait*/ QPixmap_cacheKey for () {
  fn cacheKey(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap8cacheKeyEv()};
    unsafe {_ZNK7QPixmap8cacheKeyEv()};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn createHeuristicMask<T: QPixmap_createHeuristicMask>(&mut self, value: T) -> i32 {
    value.createHeuristicMask(self);
    return 1;
  }
}

pub trait QPixmap_createHeuristicMask {
  fn createHeuristicMask(self, this: &mut QPixmap) -> i32;
}

// proto: QBitmap QPixmap::createHeuristicMask(bool clipTight);
impl<'a> /*trait*/ QPixmap_createHeuristicMask for (i8) {
  fn createHeuristicMask(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap19createHeuristicMaskEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZNK7QPixmap19createHeuristicMaskEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn depth<T: QPixmap_depth>(&mut self, value: T) -> i32 {
    value.depth(self);
    return 1;
  }
}

pub trait QPixmap_depth {
  fn depth(self, this: &mut QPixmap) -> i32;
}

// proto: int QPixmap::depth();
impl<'a> /*trait*/ QPixmap_depth for () {
  fn depth(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap5depthEv()};
    unsafe {_ZNK7QPixmap5depthEv()};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn toImage<T: QPixmap_toImage>(&mut self, value: T) -> i32 {
    value.toImage(self);
    return 1;
  }
}

pub trait QPixmap_toImage {
  fn toImage(self, this: &mut QPixmap) -> i32;
}

// proto: QImage QPixmap::toImage();
impl<'a> /*trait*/ QPixmap_toImage for () {
  fn toImage(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap7toImageEv()};
    unsafe {_ZNK7QPixmap7toImageEv()};
    return 1;
  }
}

// proto: QPixmap QPixmap::grabWidget(QObject * widget, int x, int y, int w, int h);
impl<'a> /*trait*/ QPixmap_grabWidget for (&'a mut QObject, i32, i32, i32, i32) {
  fn grabWidget(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap10grabWidgetEP7QObjectiiii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    unsafe {_ZN7QPixmap10grabWidgetEP7QObjectiiii(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn handle<T: QPixmap_handle>(&mut self, value: T) -> i32 {
    value.handle(self);
    return 1;
  }
}

pub trait QPixmap_handle {
  fn handle(self, this: &mut QPixmap) -> i32;
}

// proto: QPlatformPixmap * QPixmap::handle();
impl<'a> /*trait*/ QPixmap_handle for () {
  fn handle(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap6handleEv()};
    unsafe {_ZNK7QPixmap6handleEv()};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn hasAlphaChannel<T: QPixmap_hasAlphaChannel>(&mut self, value: T) -> i32 {
    value.hasAlphaChannel(self);
    return 1;
  }
}

pub trait QPixmap_hasAlphaChannel {
  fn hasAlphaChannel(self, this: &mut QPixmap) -> i32;
}

// proto: bool QPixmap::hasAlphaChannel();
impl<'a> /*trait*/ QPixmap_hasAlphaChannel for () {
  fn hasAlphaChannel(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap15hasAlphaChannelEv()};
    unsafe {_ZNK7QPixmap15hasAlphaChannelEv()};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn rect<T: QPixmap_rect>(&mut self, value: T) -> i32 {
    value.rect(self);
    return 1;
  }
}

pub trait QPixmap_rect {
  fn rect(self, this: &mut QPixmap) -> i32;
}

// proto: QRect QPixmap::rect();
impl<'a> /*trait*/ QPixmap_rect for () {
  fn rect(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4rectEv()};
    unsafe {_ZNK7QPixmap4rectEv()};
    return 1;
  }
}

// proto: QMatrix QPixmap::trueMatrix(const QMatrix & m, int w, int h);
impl<'a> /*trait*/ QPixmap_trueMatrix for (&'a  QMatrix, i32, i32) {
  fn trueMatrix(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap10trueMatrixERK7QMatrixii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN7QPixmap10trueMatrixERK7QMatrixii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn mask<T: QPixmap_mask>(&mut self, value: T) -> i32 {
    value.mask(self);
    return 1;
  }
}

pub trait QPixmap_mask {
  fn mask(self, this: &mut QPixmap) -> i32;
}

// proto: QBitmap QPixmap::mask();
impl<'a> /*trait*/ QPixmap_mask for () {
  fn mask(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4maskEv()};
    unsafe {_ZNK7QPixmap4maskEv()};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn width<T: QPixmap_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QPixmap_width {
  fn width(self, this: &mut QPixmap) -> i32;
}

// proto: int QPixmap::width();
impl<'a> /*trait*/ QPixmap_width for () {
  fn width(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap5widthEv()};
    unsafe {_ZNK7QPixmap5widthEv()};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn paintEngine<T: QPixmap_paintEngine>(&mut self, value: T) -> i32 {
    value.paintEngine(self);
    return 1;
  }
}

pub trait QPixmap_paintEngine {
  fn paintEngine(self, this: &mut QPixmap) -> i32;
}

// proto: QPaintEngine * QPixmap::paintEngine();
impl<'a> /*trait*/ QPixmap_paintEngine for () {
  fn paintEngine(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap11paintEngineEv()};
    unsafe {_ZNK7QPixmap11paintEngineEv()};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn FreeQPixmap<T: QPixmap_FreeQPixmap>(&mut self, value: T) -> i32 {
    value.FreeQPixmap(self);
    return 1;
  }
}

pub trait QPixmap_FreeQPixmap {
  fn FreeQPixmap(self, this: &mut QPixmap) -> i32;
}

// proto: void QPixmap::FreeQPixmap();
impl<'a> /*trait*/ QPixmap_FreeQPixmap for () {
  fn FreeQPixmap(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapD0Ev()};
    unsafe {_ZN7QPixmapD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn height<T: QPixmap_height>(&mut self, value: T) -> i32 {
    value.height(self);
    return 1;
  }
}

pub trait QPixmap_height {
  fn height(self, this: &mut QPixmap) -> i32;
}

// proto: int QPixmap::height();
impl<'a> /*trait*/ QPixmap_height for () {
  fn height(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap6heightEv()};
    unsafe {_ZNK7QPixmap6heightEv()};
    return 1;
  }
}

// proto: bool QPixmap::save(QIODevice * device, const char * format, int quality);
impl<'a> /*trait*/ QPixmap_save for (&'a mut QIODevice, &'a  String, i32) {
  fn save(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4saveEP9QIODevicePKci()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2  as c_int;
    unsafe {_ZNK7QPixmap4saveEP9QIODevicePKci(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn size<T: QPixmap_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QPixmap_size {
  fn size(self, this: &mut QPixmap) -> i32;
}

// proto: QSize QPixmap::size();
impl<'a> /*trait*/ QPixmap_size for () {
  fn size(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4sizeEv()};
    unsafe {_ZNK7QPixmap4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn hasAlpha<T: QPixmap_hasAlpha>(&mut self, value: T) -> i32 {
    value.hasAlpha(self);
    return 1;
  }
}

pub trait QPixmap_hasAlpha {
  fn hasAlpha(self, this: &mut QPixmap) -> i32;
}

// proto: bool QPixmap::hasAlpha();
impl<'a> /*trait*/ QPixmap_hasAlpha for () {
  fn hasAlpha(self, this: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap8hasAlphaEv()};
    unsafe {_ZNK7QPixmap8hasAlphaEv()};
    return 1;
  }
}

