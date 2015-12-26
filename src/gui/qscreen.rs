// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtGui/qscreen.h
// dst-file: /src/gui/qscreen.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::core::qrect::QRect; // 771
use super::qpixmap::QPixmap; // 773
use super::super::core::qsize::QSize; // 771
use super::super::core::qsize::QSizeF; // 771
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QScreen_Class_Size() -> c_int;
  // proto:  qreal QScreen::logicalDotsPerInchY();
  fn _ZNK7QScreen19logicalDotsPerInchYEv(qthis: *mut c_void) -> c_double;
  // proto:  QRect QScreen::geometry();
  fn _ZNK7QScreen8geometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPixmap QScreen::grabWindow(WId window, int x, int y, int w, int h);
  fn _ZN7QScreen10grabWindowEiiiii(qthis: *mut c_void, arg0: *mut c_uint, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) -> *mut c_void;
  // proto:  QSize QScreen::size();
  fn _ZNK7QScreen4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSizeF QScreen::physicalSize();
  fn _ZNK7QScreen12physicalSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPlatformScreen * QScreen::handle();
  fn _ZNK7QScreen6handleEv(qthis: *mut c_void);
  // proto:  QRect QScreen::availableVirtualGeometry();
  fn _ZNK7QScreen24availableVirtualGeometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QScreen::~QScreen();
  fn _ZN7QScreenD0Ev(qthis: *mut c_void);
  // proto:  QSize QScreen::virtualSize();
  fn _ZNK7QScreen11virtualSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QScreen::devicePixelRatio();
  fn _ZNK7QScreen16devicePixelRatioEv(qthis: *mut c_void) -> c_double;
  // proto:  QList<QScreen *> QScreen::virtualSiblings();
  fn _ZNK7QScreen15virtualSiblingsEv(qthis: *mut c_void);
  // proto:  void QScreen::QScreen(const QScreen & );
  fn dector_ZN7QScreenC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN7QScreenC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QScreen::logicalDotsPerInchChanged(qreal dpi);
  fn _ZN7QScreen25logicalDotsPerInchChangedEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QScreen::availableGeometryChanged(const QRect & geometry);
  fn _ZN7QScreen24availableGeometryChangedERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QScreen::geometryChanged(const QRect & geometry);
  fn _ZN7QScreen15geometryChangedERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRect QScreen::virtualGeometry();
  fn _ZNK7QScreen15virtualGeometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QScreen::logicalDotsPerInch();
  fn _ZNK7QScreen18logicalDotsPerInchEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QScreen::physicalDotsPerInch();
  fn _ZNK7QScreen19physicalDotsPerInchEv(qthis: *mut c_void) -> c_double;
  // proto:  void QScreen::physicalDotsPerInchChanged(qreal dpi);
  fn _ZN7QScreen26physicalDotsPerInchChangedEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QScreen::refreshRate();
  fn _ZNK7QScreen11refreshRateEv(qthis: *mut c_void) -> c_double;
  // proto:  const QMetaObject * QScreen::metaObject();
  fn _ZNK7QScreen10metaObjectEv(qthis: *mut c_void);
  // proto:  void QScreen::refreshRateChanged(qreal refreshRate);
  fn _ZN7QScreen18refreshRateChangedEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QSize QScreen::availableSize();
  fn _ZNK7QScreen13availableSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QScreen::name();
  fn _ZNK7QScreen4nameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QScreen::availableVirtualSize();
  fn _ZNK7QScreen20availableVirtualSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QScreen::logicalDotsPerInchX();
  fn _ZNK7QScreen19logicalDotsPerInchXEv(qthis: *mut c_void) -> c_double;
  // proto:  void QScreen::virtualGeometryChanged(const QRect & rect);
  fn _ZN7QScreen22virtualGeometryChangedERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRect QScreen::availableGeometry();
  fn _ZNK7QScreen17availableGeometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QScreen::physicalDotsPerInchX();
  fn _ZNK7QScreen20physicalDotsPerInchXEv(qthis: *mut c_void) -> c_double;
  // proto:  void QScreen::physicalSizeChanged(const QSizeF & size);
  fn _ZN7QScreen19physicalSizeChangedERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QScreen::physicalDotsPerInchY();
  fn _ZNK7QScreen20physicalDotsPerInchYEv(qthis: *mut c_void) -> c_double;
  // proto:  int QScreen::depth();
  fn _ZNK7QScreen5depthEv(qthis: *mut c_void) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QScreen)=1
pub struct QScreen {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QScreen {
  pub fn inheritFrom(qthis: *mut c_void) -> QScreen {
    return QScreen{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QScreen {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QScreen {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  qreal QScreen::logicalDotsPerInchY();
impl /*struct*/ QScreen {
  pub fn logicalDotsPerInchY<RetType, T: QScreen_logicalDotsPerInchY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.logicalDotsPerInchY(self);
    // return 1;
  }
}

pub trait QScreen_logicalDotsPerInchY<RetType> {
  fn logicalDotsPerInchY(self , rsthis: & QScreen) -> RetType;
}

  // proto:  qreal QScreen::logicalDotsPerInchY();
impl<'a> /*trait*/ QScreen_logicalDotsPerInchY<f64> for () {
  fn logicalDotsPerInchY(self , rsthis: & QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen19logicalDotsPerInchYEv()};
    let mut ret = unsafe {_ZNK7QScreen19logicalDotsPerInchYEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QRect QScreen::geometry();
impl /*struct*/ QScreen {
  pub fn geometry<RetType, T: QScreen_geometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.geometry(self);
    // return 1;
  }
}

pub trait QScreen_geometry<RetType> {
  fn geometry(self , rsthis: & QScreen) -> RetType;
}

  // proto:  QRect QScreen::geometry();
impl<'a> /*trait*/ QScreen_geometry<QRect> for () {
  fn geometry(self , rsthis: & QScreen) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen8geometryEv()};
    let mut ret = unsafe {_ZNK7QScreen8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPixmap QScreen::grabWindow(WId window, int x, int y, int w, int h);
impl /*struct*/ QScreen {
  pub fn grabWindow<RetType, T: QScreen_grabWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.grabWindow(self);
    // return 1;
  }
}

pub trait QScreen_grabWindow<RetType> {
  fn grabWindow(self , rsthis: & QScreen) -> RetType;
}

  // proto:  QPixmap QScreen::grabWindow(WId window, int x, int y, int w, int h);
impl<'a> /*trait*/ QScreen_grabWindow<QPixmap> for (*mut i32, i32, i32, i32, i32) {
  fn grabWindow(self , rsthis: & QScreen) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen10grabWindowEiiiii()};
    let arg0 = self.0  as *mut c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let mut ret = unsafe {_ZN7QScreen10grabWindowEiiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QPixmap::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QScreen::size();
impl /*struct*/ QScreen {
  pub fn size<RetType, T: QScreen_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QScreen_size<RetType> {
  fn size(self , rsthis: & QScreen) -> RetType;
}

  // proto:  QSize QScreen::size();
impl<'a> /*trait*/ QScreen_size<QSize> for () {
  fn size(self , rsthis: & QScreen) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen4sizeEv()};
    let mut ret = unsafe {_ZNK7QScreen4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QSizeF QScreen::physicalSize();
impl /*struct*/ QScreen {
  pub fn physicalSize<RetType, T: QScreen_physicalSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.physicalSize(self);
    // return 1;
  }
}

pub trait QScreen_physicalSize<RetType> {
  fn physicalSize(self , rsthis: & QScreen) -> RetType;
}

  // proto:  QSizeF QScreen::physicalSize();
impl<'a> /*trait*/ QScreen_physicalSize<QSizeF> for () {
  fn physicalSize(self , rsthis: & QScreen) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen12physicalSizeEv()};
    let mut ret = unsafe {_ZNK7QScreen12physicalSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPlatformScreen * QScreen::handle();
impl /*struct*/ QScreen {
  pub fn handle<RetType, T: QScreen_handle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.handle(self);
    // return 1;
  }
}

pub trait QScreen_handle<RetType> {
  fn handle(self , rsthis: & QScreen) -> RetType;
}

  // proto:  QPlatformScreen * QScreen::handle();
impl<'a> /*trait*/ QScreen_handle<()> for () {
  fn handle(self , rsthis: & QScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen6handleEv()};
     unsafe {_ZNK7QScreen6handleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRect QScreen::availableVirtualGeometry();
impl /*struct*/ QScreen {
  pub fn availableVirtualGeometry<RetType, T: QScreen_availableVirtualGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.availableVirtualGeometry(self);
    // return 1;
  }
}

pub trait QScreen_availableVirtualGeometry<RetType> {
  fn availableVirtualGeometry(self , rsthis: & QScreen) -> RetType;
}

  // proto:  QRect QScreen::availableVirtualGeometry();
impl<'a> /*trait*/ QScreen_availableVirtualGeometry<QRect> for () {
  fn availableVirtualGeometry(self , rsthis: & QScreen) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen24availableVirtualGeometryEv()};
    let mut ret = unsafe {_ZNK7QScreen24availableVirtualGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QScreen::~QScreen();
impl /*struct*/ QScreen {
  pub fn Free<RetType, T: QScreen_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QScreen_Free<RetType> {
  fn Free(self , rsthis: & QScreen) -> RetType;
}

  // proto:  void QScreen::~QScreen();
impl<'a> /*trait*/ QScreen_Free<()> for () {
  fn Free(self , rsthis: & QScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreenD0Ev()};
     unsafe {_ZN7QScreenD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QScreen::virtualSize();
impl /*struct*/ QScreen {
  pub fn virtualSize<RetType, T: QScreen_virtualSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.virtualSize(self);
    // return 1;
  }
}

pub trait QScreen_virtualSize<RetType> {
  fn virtualSize(self , rsthis: & QScreen) -> RetType;
}

  // proto:  QSize QScreen::virtualSize();
impl<'a> /*trait*/ QScreen_virtualSize<QSize> for () {
  fn virtualSize(self , rsthis: & QScreen) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen11virtualSizeEv()};
    let mut ret = unsafe {_ZNK7QScreen11virtualSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QScreen::devicePixelRatio();
impl /*struct*/ QScreen {
  pub fn devicePixelRatio<RetType, T: QScreen_devicePixelRatio<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.devicePixelRatio(self);
    // return 1;
  }
}

pub trait QScreen_devicePixelRatio<RetType> {
  fn devicePixelRatio(self , rsthis: & QScreen) -> RetType;
}

  // proto:  qreal QScreen::devicePixelRatio();
impl<'a> /*trait*/ QScreen_devicePixelRatio<f64> for () {
  fn devicePixelRatio(self , rsthis: & QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen16devicePixelRatioEv()};
    let mut ret = unsafe {_ZNK7QScreen16devicePixelRatioEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QList<QScreen *> QScreen::virtualSiblings();
impl /*struct*/ QScreen {
  pub fn virtualSiblings<RetType, T: QScreen_virtualSiblings<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.virtualSiblings(self);
    // return 1;
  }
}

pub trait QScreen_virtualSiblings<RetType> {
  fn virtualSiblings(self , rsthis: & QScreen) -> RetType;
}

  // proto:  QList<QScreen *> QScreen::virtualSiblings();
impl<'a> /*trait*/ QScreen_virtualSiblings<()> for () {
  fn virtualSiblings(self , rsthis: & QScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen15virtualSiblingsEv()};
     unsafe {_ZNK7QScreen15virtualSiblingsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QScreen::QScreen(const QScreen & );
impl /*struct*/ QScreen {
  pub fn New<T: QScreen_New>(value: T) -> QScreen {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QScreen_New {
  fn New(self) -> QScreen;
}

  // proto:  void QScreen::QScreen(const QScreen & );
impl<'a> /*trait*/ QScreen_New for (&'a QScreen) {
  fn New(self) -> QScreen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreenC1ERKS_()};
    let ctysz: c_int = unsafe{QScreen_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN7QScreenC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN7QScreenC1ERKS_(arg0)};
    let rsthis = QScreen{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QScreen::logicalDotsPerInchChanged(qreal dpi);
impl /*struct*/ QScreen {
  pub fn logicalDotsPerInchChanged<RetType, T: QScreen_logicalDotsPerInchChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.logicalDotsPerInchChanged(self);
    // return 1;
  }
}

pub trait QScreen_logicalDotsPerInchChanged<RetType> {
  fn logicalDotsPerInchChanged(self , rsthis: & QScreen) -> RetType;
}

  // proto:  void QScreen::logicalDotsPerInchChanged(qreal dpi);
impl<'a> /*trait*/ QScreen_logicalDotsPerInchChanged<()> for (f64) {
  fn logicalDotsPerInchChanged(self , rsthis: & QScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen25logicalDotsPerInchChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN7QScreen25logicalDotsPerInchChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QScreen::availableGeometryChanged(const QRect & geometry);
impl /*struct*/ QScreen {
  pub fn availableGeometryChanged<RetType, T: QScreen_availableGeometryChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.availableGeometryChanged(self);
    // return 1;
  }
}

pub trait QScreen_availableGeometryChanged<RetType> {
  fn availableGeometryChanged(self , rsthis: & QScreen) -> RetType;
}

  // proto:  void QScreen::availableGeometryChanged(const QRect & geometry);
impl<'a> /*trait*/ QScreen_availableGeometryChanged<()> for (&'a QRect) {
  fn availableGeometryChanged(self , rsthis: & QScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen24availableGeometryChangedERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QScreen24availableGeometryChangedERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QScreen::geometryChanged(const QRect & geometry);
impl /*struct*/ QScreen {
  pub fn geometryChanged<RetType, T: QScreen_geometryChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.geometryChanged(self);
    // return 1;
  }
}

pub trait QScreen_geometryChanged<RetType> {
  fn geometryChanged(self , rsthis: & QScreen) -> RetType;
}

  // proto:  void QScreen::geometryChanged(const QRect & geometry);
impl<'a> /*trait*/ QScreen_geometryChanged<()> for (&'a QRect) {
  fn geometryChanged(self , rsthis: & QScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen15geometryChangedERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QScreen15geometryChangedERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QScreen::virtualGeometry();
impl /*struct*/ QScreen {
  pub fn virtualGeometry<RetType, T: QScreen_virtualGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.virtualGeometry(self);
    // return 1;
  }
}

pub trait QScreen_virtualGeometry<RetType> {
  fn virtualGeometry(self , rsthis: & QScreen) -> RetType;
}

  // proto:  QRect QScreen::virtualGeometry();
impl<'a> /*trait*/ QScreen_virtualGeometry<QRect> for () {
  fn virtualGeometry(self , rsthis: & QScreen) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen15virtualGeometryEv()};
    let mut ret = unsafe {_ZNK7QScreen15virtualGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QScreen::logicalDotsPerInch();
impl /*struct*/ QScreen {
  pub fn logicalDotsPerInch<RetType, T: QScreen_logicalDotsPerInch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.logicalDotsPerInch(self);
    // return 1;
  }
}

pub trait QScreen_logicalDotsPerInch<RetType> {
  fn logicalDotsPerInch(self , rsthis: & QScreen) -> RetType;
}

  // proto:  qreal QScreen::logicalDotsPerInch();
impl<'a> /*trait*/ QScreen_logicalDotsPerInch<f64> for () {
  fn logicalDotsPerInch(self , rsthis: & QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen18logicalDotsPerInchEv()};
    let mut ret = unsafe {_ZNK7QScreen18logicalDotsPerInchEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QScreen::physicalDotsPerInch();
impl /*struct*/ QScreen {
  pub fn physicalDotsPerInch<RetType, T: QScreen_physicalDotsPerInch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.physicalDotsPerInch(self);
    // return 1;
  }
}

pub trait QScreen_physicalDotsPerInch<RetType> {
  fn physicalDotsPerInch(self , rsthis: & QScreen) -> RetType;
}

  // proto:  qreal QScreen::physicalDotsPerInch();
impl<'a> /*trait*/ QScreen_physicalDotsPerInch<f64> for () {
  fn physicalDotsPerInch(self , rsthis: & QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen19physicalDotsPerInchEv()};
    let mut ret = unsafe {_ZNK7QScreen19physicalDotsPerInchEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QScreen::physicalDotsPerInchChanged(qreal dpi);
impl /*struct*/ QScreen {
  pub fn physicalDotsPerInchChanged<RetType, T: QScreen_physicalDotsPerInchChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.physicalDotsPerInchChanged(self);
    // return 1;
  }
}

pub trait QScreen_physicalDotsPerInchChanged<RetType> {
  fn physicalDotsPerInchChanged(self , rsthis: & QScreen) -> RetType;
}

  // proto:  void QScreen::physicalDotsPerInchChanged(qreal dpi);
impl<'a> /*trait*/ QScreen_physicalDotsPerInchChanged<()> for (f64) {
  fn physicalDotsPerInchChanged(self , rsthis: & QScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen26physicalDotsPerInchChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN7QScreen26physicalDotsPerInchChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QScreen::refreshRate();
impl /*struct*/ QScreen {
  pub fn refreshRate<RetType, T: QScreen_refreshRate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.refreshRate(self);
    // return 1;
  }
}

pub trait QScreen_refreshRate<RetType> {
  fn refreshRate(self , rsthis: & QScreen) -> RetType;
}

  // proto:  qreal QScreen::refreshRate();
impl<'a> /*trait*/ QScreen_refreshRate<f64> for () {
  fn refreshRate(self , rsthis: & QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen11refreshRateEv()};
    let mut ret = unsafe {_ZNK7QScreen11refreshRateEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  const QMetaObject * QScreen::metaObject();
impl /*struct*/ QScreen {
  pub fn metaObject<RetType, T: QScreen_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QScreen_metaObject<RetType> {
  fn metaObject(self , rsthis: & QScreen) -> RetType;
}

  // proto:  const QMetaObject * QScreen::metaObject();
impl<'a> /*trait*/ QScreen_metaObject<()> for () {
  fn metaObject(self , rsthis: & QScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen10metaObjectEv()};
     unsafe {_ZNK7QScreen10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QScreen::refreshRateChanged(qreal refreshRate);
impl /*struct*/ QScreen {
  pub fn refreshRateChanged<RetType, T: QScreen_refreshRateChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.refreshRateChanged(self);
    // return 1;
  }
}

pub trait QScreen_refreshRateChanged<RetType> {
  fn refreshRateChanged(self , rsthis: & QScreen) -> RetType;
}

  // proto:  void QScreen::refreshRateChanged(qreal refreshRate);
impl<'a> /*trait*/ QScreen_refreshRateChanged<()> for (f64) {
  fn refreshRateChanged(self , rsthis: & QScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen18refreshRateChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN7QScreen18refreshRateChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QScreen::availableSize();
impl /*struct*/ QScreen {
  pub fn availableSize<RetType, T: QScreen_availableSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.availableSize(self);
    // return 1;
  }
}

pub trait QScreen_availableSize<RetType> {
  fn availableSize(self , rsthis: & QScreen) -> RetType;
}

  // proto:  QSize QScreen::availableSize();
impl<'a> /*trait*/ QScreen_availableSize<QSize> for () {
  fn availableSize(self , rsthis: & QScreen) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen13availableSizeEv()};
    let mut ret = unsafe {_ZNK7QScreen13availableSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QScreen::name();
impl /*struct*/ QScreen {
  pub fn name<RetType, T: QScreen_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QScreen_name<RetType> {
  fn name(self , rsthis: & QScreen) -> RetType;
}

  // proto:  QString QScreen::name();
impl<'a> /*trait*/ QScreen_name<QString> for () {
  fn name(self , rsthis: & QScreen) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen4nameEv()};
    let mut ret = unsafe {_ZNK7QScreen4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QScreen::availableVirtualSize();
impl /*struct*/ QScreen {
  pub fn availableVirtualSize<RetType, T: QScreen_availableVirtualSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.availableVirtualSize(self);
    // return 1;
  }
}

pub trait QScreen_availableVirtualSize<RetType> {
  fn availableVirtualSize(self , rsthis: & QScreen) -> RetType;
}

  // proto:  QSize QScreen::availableVirtualSize();
impl<'a> /*trait*/ QScreen_availableVirtualSize<QSize> for () {
  fn availableVirtualSize(self , rsthis: & QScreen) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen20availableVirtualSizeEv()};
    let mut ret = unsafe {_ZNK7QScreen20availableVirtualSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QScreen::logicalDotsPerInchX();
impl /*struct*/ QScreen {
  pub fn logicalDotsPerInchX<RetType, T: QScreen_logicalDotsPerInchX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.logicalDotsPerInchX(self);
    // return 1;
  }
}

pub trait QScreen_logicalDotsPerInchX<RetType> {
  fn logicalDotsPerInchX(self , rsthis: & QScreen) -> RetType;
}

  // proto:  qreal QScreen::logicalDotsPerInchX();
impl<'a> /*trait*/ QScreen_logicalDotsPerInchX<f64> for () {
  fn logicalDotsPerInchX(self , rsthis: & QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen19logicalDotsPerInchXEv()};
    let mut ret = unsafe {_ZNK7QScreen19logicalDotsPerInchXEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QScreen::virtualGeometryChanged(const QRect & rect);
impl /*struct*/ QScreen {
  pub fn virtualGeometryChanged<RetType, T: QScreen_virtualGeometryChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.virtualGeometryChanged(self);
    // return 1;
  }
}

pub trait QScreen_virtualGeometryChanged<RetType> {
  fn virtualGeometryChanged(self , rsthis: & QScreen) -> RetType;
}

  // proto:  void QScreen::virtualGeometryChanged(const QRect & rect);
impl<'a> /*trait*/ QScreen_virtualGeometryChanged<()> for (&'a QRect) {
  fn virtualGeometryChanged(self , rsthis: & QScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen22virtualGeometryChangedERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QScreen22virtualGeometryChangedERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QScreen::availableGeometry();
impl /*struct*/ QScreen {
  pub fn availableGeometry<RetType, T: QScreen_availableGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.availableGeometry(self);
    // return 1;
  }
}

pub trait QScreen_availableGeometry<RetType> {
  fn availableGeometry(self , rsthis: & QScreen) -> RetType;
}

  // proto:  QRect QScreen::availableGeometry();
impl<'a> /*trait*/ QScreen_availableGeometry<QRect> for () {
  fn availableGeometry(self , rsthis: & QScreen) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen17availableGeometryEv()};
    let mut ret = unsafe {_ZNK7QScreen17availableGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QScreen::physicalDotsPerInchX();
impl /*struct*/ QScreen {
  pub fn physicalDotsPerInchX<RetType, T: QScreen_physicalDotsPerInchX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.physicalDotsPerInchX(self);
    // return 1;
  }
}

pub trait QScreen_physicalDotsPerInchX<RetType> {
  fn physicalDotsPerInchX(self , rsthis: & QScreen) -> RetType;
}

  // proto:  qreal QScreen::physicalDotsPerInchX();
impl<'a> /*trait*/ QScreen_physicalDotsPerInchX<f64> for () {
  fn physicalDotsPerInchX(self , rsthis: & QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen20physicalDotsPerInchXEv()};
    let mut ret = unsafe {_ZNK7QScreen20physicalDotsPerInchXEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QScreen::physicalSizeChanged(const QSizeF & size);
impl /*struct*/ QScreen {
  pub fn physicalSizeChanged<RetType, T: QScreen_physicalSizeChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.physicalSizeChanged(self);
    // return 1;
  }
}

pub trait QScreen_physicalSizeChanged<RetType> {
  fn physicalSizeChanged(self , rsthis: & QScreen) -> RetType;
}

  // proto:  void QScreen::physicalSizeChanged(const QSizeF & size);
impl<'a> /*trait*/ QScreen_physicalSizeChanged<()> for (&'a QSizeF) {
  fn physicalSizeChanged(self , rsthis: & QScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen19physicalSizeChangedERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QScreen19physicalSizeChangedERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QScreen::physicalDotsPerInchY();
impl /*struct*/ QScreen {
  pub fn physicalDotsPerInchY<RetType, T: QScreen_physicalDotsPerInchY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.physicalDotsPerInchY(self);
    // return 1;
  }
}

pub trait QScreen_physicalDotsPerInchY<RetType> {
  fn physicalDotsPerInchY(self , rsthis: & QScreen) -> RetType;
}

  // proto:  qreal QScreen::physicalDotsPerInchY();
impl<'a> /*trait*/ QScreen_physicalDotsPerInchY<f64> for () {
  fn physicalDotsPerInchY(self , rsthis: & QScreen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen20physicalDotsPerInchYEv()};
    let mut ret = unsafe {_ZNK7QScreen20physicalDotsPerInchYEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  int QScreen::depth();
impl /*struct*/ QScreen {
  pub fn depth<RetType, T: QScreen_depth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.depth(self);
    // return 1;
  }
}

pub trait QScreen_depth<RetType> {
  fn depth(self , rsthis: & QScreen) -> RetType;
}

  // proto:  int QScreen::depth();
impl<'a> /*trait*/ QScreen_depth<i32> for () {
  fn depth(self , rsthis: & QScreen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen5depthEv()};
    let mut ret = unsafe {_ZNK7QScreen5depthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// <= body block end

