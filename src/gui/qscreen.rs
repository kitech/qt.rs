// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
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
use super::super::core::qobject::*; // 771
use std::ops::Deref;
use super::super::core::qrect::*; // 771
use super::qpixmap::*; // 773
use super::super::core::qsize::*; // 771
// use super::qplatformscreen::*; // 775
// use super::qlist::*; // 775
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qstring::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QScreen_Class_Size() -> c_int;
  // proto:  qreal QScreen::logicalDotsPerInchY();
  fn C_ZNK7QScreen19logicalDotsPerInchYEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QRect QScreen::geometry();
  fn C_ZNK7QScreen8geometryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPixmap QScreen::grabWindow(WId window, int x, int y, int w, int h);
  fn C_ZN7QScreen10grabWindowEiiiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) -> *mut c_void;
  // proto:  QSize QScreen::size();
  fn C_ZNK7QScreen4sizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSizeF QScreen::physicalSize();
  fn C_ZNK7QScreen12physicalSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPlatformScreen * QScreen::handle();
  fn C_ZNK7QScreen6handleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRect QScreen::availableVirtualGeometry();
  fn C_ZNK7QScreen24availableVirtualGeometryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QScreen::~QScreen();
  fn C_ZN7QScreenD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QScreen::virtualSize();
  fn C_ZNK7QScreen11virtualSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QScreen::devicePixelRatio();
  fn C_ZNK7QScreen16devicePixelRatioEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QList<QScreen *> QScreen::virtualSiblings();
  fn C_ZNK7QScreen15virtualSiblingsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRect QScreen::virtualGeometry();
  fn C_ZNK7QScreen15virtualGeometryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QScreen::logicalDotsPerInch();
  fn C_ZNK7QScreen18logicalDotsPerInchEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QScreen::physicalDotsPerInch();
  fn C_ZNK7QScreen19physicalDotsPerInchEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QScreen::refreshRate();
  fn C_ZNK7QScreen11refreshRateEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  const QMetaObject * QScreen::metaObject();
  fn C_ZNK7QScreen10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QScreen::availableSize();
  fn C_ZNK7QScreen13availableSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QScreen::name();
  fn C_ZNK7QScreen4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QScreen::availableVirtualSize();
  fn C_ZNK7QScreen20availableVirtualSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QScreen::logicalDotsPerInchX();
  fn C_ZNK7QScreen19logicalDotsPerInchXEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QRect QScreen::availableGeometry();
  fn C_ZNK7QScreen17availableGeometryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QScreen::physicalDotsPerInchX();
  fn C_ZNK7QScreen20physicalDotsPerInchXEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QScreen::physicalDotsPerInchY();
  fn C_ZNK7QScreen20physicalDotsPerInchYEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  int QScreen::depth();
  fn C_ZNK7QScreen5depthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  fn QScreen_SlotProxy_connect__ZN7QScreen25logicalDotsPerInchChangedEd(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QScreen_SlotProxy_connect__ZN7QScreen24availableGeometryChangedERK5QRect(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QScreen_SlotProxy_connect__ZN7QScreen15geometryChangedERK5QRect(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QScreen_SlotProxy_connect__ZN7QScreen26physicalDotsPerInchChangedEd(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QScreen_SlotProxy_connect__ZN7QScreen22virtualGeometryChangedERK5QRect(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QScreen_SlotProxy_connect__ZN7QScreen18refreshRateChangedEd(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QScreen_SlotProxy_connect__ZN7QScreen19physicalSizeChangedERK6QSizeF(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QScreen)=1
#[derive(Default)]
pub struct QScreen {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _geometryChanged: QScreen_geometryChanged_signal,
  pub _virtualGeometryChanged: QScreen_virtualGeometryChanged_signal,
  pub _refreshRateChanged: QScreen_refreshRateChanged_signal,
  pub _availableGeometryChanged: QScreen_availableGeometryChanged_signal,
  pub _physicalSizeChanged: QScreen_physicalSizeChanged_signal,
  pub _physicalDotsPerInchChanged: QScreen_physicalDotsPerInchChanged_signal,
  pub _logicalDotsPerInchChanged: QScreen_logicalDotsPerInchChanged_signal,
  pub _primaryOrientationChanged: QScreen_primaryOrientationChanged_signal,
  pub _orientationChanged: QScreen_orientationChanged_signal,
}

impl /*struct*/ QScreen {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QScreen {
    return QScreen{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK7QScreen19logicalDotsPerInchYEv(rsthis.qclsinst)};
    return ret as f64; // 1
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
    let mut ret = unsafe {C_ZNK7QScreen8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
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
impl<'a> /*trait*/ QScreen_grabWindow<QPixmap> for (*mut i32, Option<i32>, Option<i32>, Option<i32>, Option<i32>) {
  fn grabWindow(self , rsthis: & QScreen) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreen10grabWindowEiiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap()})  as c_int;
    let arg2 = (if self.2.is_none() {0} else {self.2.unwrap()})  as c_int;
    let arg3 = (if self.3.is_none() {-1} else {self.3.unwrap()})  as c_int;
    let arg4 = (if self.4.is_none() {-1} else {self.4.unwrap()})  as c_int;
    let mut ret = unsafe {C_ZN7QScreen10grabWindowEiiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QPixmap::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK7QScreen4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK7QScreen12physicalSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret as u64);
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
impl<'a> /*trait*/ QScreen_handle<u64> for () {
  fn handle(self , rsthis: & QScreen) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen6handleEv()};
    let mut ret = unsafe {C_ZNK7QScreen6handleEv(rsthis.qclsinst)};
    return ret as u64; // 4
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
    let mut ret = unsafe {C_ZNK7QScreen24availableVirtualGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QScreen::~QScreen();
impl /*struct*/ QScreen {
  pub fn free<RetType, T: QScreen_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QScreen_free<RetType> {
  fn free(self , rsthis: & QScreen) -> RetType;
}

  // proto:  void QScreen::~QScreen();
impl<'a> /*trait*/ QScreen_free<()> for () {
  fn free(self , rsthis: & QScreen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QScreenD2Ev()};
     unsafe {C_ZN7QScreenD2Ev(rsthis.qclsinst)};
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
    let mut ret = unsafe {C_ZNK7QScreen11virtualSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK7QScreen16devicePixelRatioEv(rsthis.qclsinst)};
    return ret as f64; // 1
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
impl<'a> /*trait*/ QScreen_virtualSiblings<u64> for () {
  fn virtualSiblings(self , rsthis: & QScreen) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen15virtualSiblingsEv()};
    let mut ret = unsafe {C_ZNK7QScreen15virtualSiblingsEv(rsthis.qclsinst)};
    return ret as u64; // 5
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
    let mut ret = unsafe {C_ZNK7QScreen15virtualGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK7QScreen18logicalDotsPerInchEv(rsthis.qclsinst)};
    return ret as f64; // 1
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
    let mut ret = unsafe {C_ZNK7QScreen19physicalDotsPerInchEv(rsthis.qclsinst)};
    return ret as f64; // 1
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
    let mut ret = unsafe {C_ZNK7QScreen11refreshRateEv(rsthis.qclsinst)};
    return ret as f64; // 1
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
impl<'a> /*trait*/ QScreen_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QScreen) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QScreen10metaObjectEv()};
    let mut ret = unsafe {C_ZNK7QScreen10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
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
    let mut ret = unsafe {C_ZNK7QScreen13availableSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK7QScreen4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK7QScreen20availableVirtualSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK7QScreen19logicalDotsPerInchXEv(rsthis.qclsinst)};
    return ret as f64; // 1
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
    let mut ret = unsafe {C_ZNK7QScreen17availableGeometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK7QScreen20physicalDotsPerInchXEv(rsthis.qclsinst)};
    return ret as f64; // 1
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
    let mut ret = unsafe {C_ZNK7QScreen20physicalDotsPerInchYEv(rsthis.qclsinst)};
    return ret as f64; // 1
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
    let mut ret = unsafe {C_ZNK7QScreen5depthEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

#[derive(Default)] // for QScreen_geometryChanged
pub struct QScreen_geometryChanged_signal{poi:u64}
impl /* struct */ QScreen {
  pub fn geometryChanged(&self) -> QScreen_geometryChanged_signal {
     return QScreen_geometryChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QScreen_geometryChanged_signal {
  pub fn connect<T: QScreen_geometryChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QScreen_geometryChanged_signal_connect {
  fn connect(self, sigthis: QScreen_geometryChanged_signal);
}

#[derive(Default)] // for QScreen_virtualGeometryChanged
pub struct QScreen_virtualGeometryChanged_signal{poi:u64}
impl /* struct */ QScreen {
  pub fn virtualGeometryChanged(&self) -> QScreen_virtualGeometryChanged_signal {
     return QScreen_virtualGeometryChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QScreen_virtualGeometryChanged_signal {
  pub fn connect<T: QScreen_virtualGeometryChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QScreen_virtualGeometryChanged_signal_connect {
  fn connect(self, sigthis: QScreen_virtualGeometryChanged_signal);
}

#[derive(Default)] // for QScreen_refreshRateChanged
pub struct QScreen_refreshRateChanged_signal{poi:u64}
impl /* struct */ QScreen {
  pub fn refreshRateChanged(&self) -> QScreen_refreshRateChanged_signal {
     return QScreen_refreshRateChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QScreen_refreshRateChanged_signal {
  pub fn connect<T: QScreen_refreshRateChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QScreen_refreshRateChanged_signal_connect {
  fn connect(self, sigthis: QScreen_refreshRateChanged_signal);
}

#[derive(Default)] // for QScreen_availableGeometryChanged
pub struct QScreen_availableGeometryChanged_signal{poi:u64}
impl /* struct */ QScreen {
  pub fn availableGeometryChanged(&self) -> QScreen_availableGeometryChanged_signal {
     return QScreen_availableGeometryChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QScreen_availableGeometryChanged_signal {
  pub fn connect<T: QScreen_availableGeometryChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QScreen_availableGeometryChanged_signal_connect {
  fn connect(self, sigthis: QScreen_availableGeometryChanged_signal);
}

#[derive(Default)] // for QScreen_physicalSizeChanged
pub struct QScreen_physicalSizeChanged_signal{poi:u64}
impl /* struct */ QScreen {
  pub fn physicalSizeChanged(&self) -> QScreen_physicalSizeChanged_signal {
     return QScreen_physicalSizeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QScreen_physicalSizeChanged_signal {
  pub fn connect<T: QScreen_physicalSizeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QScreen_physicalSizeChanged_signal_connect {
  fn connect(self, sigthis: QScreen_physicalSizeChanged_signal);
}

#[derive(Default)] // for QScreen_physicalDotsPerInchChanged
pub struct QScreen_physicalDotsPerInchChanged_signal{poi:u64}
impl /* struct */ QScreen {
  pub fn physicalDotsPerInchChanged(&self) -> QScreen_physicalDotsPerInchChanged_signal {
     return QScreen_physicalDotsPerInchChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QScreen_physicalDotsPerInchChanged_signal {
  pub fn connect<T: QScreen_physicalDotsPerInchChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QScreen_physicalDotsPerInchChanged_signal_connect {
  fn connect(self, sigthis: QScreen_physicalDotsPerInchChanged_signal);
}

#[derive(Default)] // for QScreen_logicalDotsPerInchChanged
pub struct QScreen_logicalDotsPerInchChanged_signal{poi:u64}
impl /* struct */ QScreen {
  pub fn logicalDotsPerInchChanged(&self) -> QScreen_logicalDotsPerInchChanged_signal {
     return QScreen_logicalDotsPerInchChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QScreen_logicalDotsPerInchChanged_signal {
  pub fn connect<T: QScreen_logicalDotsPerInchChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QScreen_logicalDotsPerInchChanged_signal_connect {
  fn connect(self, sigthis: QScreen_logicalDotsPerInchChanged_signal);
}

#[derive(Default)] // for QScreen_primaryOrientationChanged
pub struct QScreen_primaryOrientationChanged_signal{poi:u64}
impl /* struct */ QScreen {
  pub fn primaryOrientationChanged(&self) -> QScreen_primaryOrientationChanged_signal {
     return QScreen_primaryOrientationChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QScreen_primaryOrientationChanged_signal {
  pub fn connect<T: QScreen_primaryOrientationChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QScreen_primaryOrientationChanged_signal_connect {
  fn connect(self, sigthis: QScreen_primaryOrientationChanged_signal);
}

#[derive(Default)] // for QScreen_orientationChanged
pub struct QScreen_orientationChanged_signal{poi:u64}
impl /* struct */ QScreen {
  pub fn orientationChanged(&self) -> QScreen_orientationChanged_signal {
     return QScreen_orientationChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QScreen_orientationChanged_signal {
  pub fn connect<T: QScreen_orientationChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QScreen_orientationChanged_signal_connect {
  fn connect(self, sigthis: QScreen_orientationChanged_signal);
}

// logicalDotsPerInchChanged(qreal)
extern fn QScreen_logicalDotsPerInchChanged_signal_connect_cb_0(rsfptr:fn(f64), arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as f64;
  rsfptr(rsarg0);
}
extern fn QScreen_logicalDotsPerInchChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(f64)>, arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as f64;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QScreen_logicalDotsPerInchChanged_signal_connect for fn(f64) {
  fn connect(self, sigthis: QScreen_logicalDotsPerInchChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QScreen_logicalDotsPerInchChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QScreen_SlotProxy_connect__ZN7QScreen25logicalDotsPerInchChangedEd(arg0, arg1, arg2)};
  }
}
impl /* trait */ QScreen_logicalDotsPerInchChanged_signal_connect for Box<Fn(f64)> {
  fn connect(self, sigthis: QScreen_logicalDotsPerInchChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QScreen_logicalDotsPerInchChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QScreen_SlotProxy_connect__ZN7QScreen25logicalDotsPerInchChangedEd(arg0, arg1, arg2)};
  }
}
// availableGeometryChanged(const class QRect &)
extern fn QScreen_availableGeometryChanged_signal_connect_cb_1(rsfptr:fn(QRect), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QRect::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QScreen_availableGeometryChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QRect)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QRect::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QScreen_availableGeometryChanged_signal_connect for fn(QRect) {
  fn connect(self, sigthis: QScreen_availableGeometryChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QScreen_availableGeometryChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QScreen_SlotProxy_connect__ZN7QScreen24availableGeometryChangedERK5QRect(arg0, arg1, arg2)};
  }
}
impl /* trait */ QScreen_availableGeometryChanged_signal_connect for Box<Fn(QRect)> {
  fn connect(self, sigthis: QScreen_availableGeometryChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QScreen_availableGeometryChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QScreen_SlotProxy_connect__ZN7QScreen24availableGeometryChangedERK5QRect(arg0, arg1, arg2)};
  }
}
// geometryChanged(const class QRect &)
extern fn QScreen_geometryChanged_signal_connect_cb_2(rsfptr:fn(QRect), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QRect::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QScreen_geometryChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(QRect)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QRect::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QScreen_geometryChanged_signal_connect for fn(QRect) {
  fn connect(self, sigthis: QScreen_geometryChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QScreen_geometryChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QScreen_SlotProxy_connect__ZN7QScreen15geometryChangedERK5QRect(arg0, arg1, arg2)};
  }
}
impl /* trait */ QScreen_geometryChanged_signal_connect for Box<Fn(QRect)> {
  fn connect(self, sigthis: QScreen_geometryChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QScreen_geometryChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QScreen_SlotProxy_connect__ZN7QScreen15geometryChangedERK5QRect(arg0, arg1, arg2)};
  }
}
// physicalDotsPerInchChanged(qreal)
extern fn QScreen_physicalDotsPerInchChanged_signal_connect_cb_3(rsfptr:fn(f64), arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as f64;
  rsfptr(rsarg0);
}
extern fn QScreen_physicalDotsPerInchChanged_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(f64)>, arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as f64;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QScreen_physicalDotsPerInchChanged_signal_connect for fn(f64) {
  fn connect(self, sigthis: QScreen_physicalDotsPerInchChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QScreen_physicalDotsPerInchChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QScreen_SlotProxy_connect__ZN7QScreen26physicalDotsPerInchChangedEd(arg0, arg1, arg2)};
  }
}
impl /* trait */ QScreen_physicalDotsPerInchChanged_signal_connect for Box<Fn(f64)> {
  fn connect(self, sigthis: QScreen_physicalDotsPerInchChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QScreen_physicalDotsPerInchChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QScreen_SlotProxy_connect__ZN7QScreen26physicalDotsPerInchChangedEd(arg0, arg1, arg2)};
  }
}
// virtualGeometryChanged(const class QRect &)
extern fn QScreen_virtualGeometryChanged_signal_connect_cb_4(rsfptr:fn(QRect), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QRect::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QScreen_virtualGeometryChanged_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn(QRect)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QRect::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QScreen_virtualGeometryChanged_signal_connect for fn(QRect) {
  fn connect(self, sigthis: QScreen_virtualGeometryChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QScreen_virtualGeometryChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QScreen_SlotProxy_connect__ZN7QScreen22virtualGeometryChangedERK5QRect(arg0, arg1, arg2)};
  }
}
impl /* trait */ QScreen_virtualGeometryChanged_signal_connect for Box<Fn(QRect)> {
  fn connect(self, sigthis: QScreen_virtualGeometryChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QScreen_virtualGeometryChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QScreen_SlotProxy_connect__ZN7QScreen22virtualGeometryChangedERK5QRect(arg0, arg1, arg2)};
  }
}
// refreshRateChanged(qreal)
extern fn QScreen_refreshRateChanged_signal_connect_cb_5(rsfptr:fn(f64), arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as f64;
  rsfptr(rsarg0);
}
extern fn QScreen_refreshRateChanged_signal_connect_cb_box_5(rsfptr_raw:*mut Box<Fn(f64)>, arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as f64;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QScreen_refreshRateChanged_signal_connect for fn(f64) {
  fn connect(self, sigthis: QScreen_refreshRateChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QScreen_refreshRateChanged_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QScreen_SlotProxy_connect__ZN7QScreen18refreshRateChangedEd(arg0, arg1, arg2)};
  }
}
impl /* trait */ QScreen_refreshRateChanged_signal_connect for Box<Fn(f64)> {
  fn connect(self, sigthis: QScreen_refreshRateChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QScreen_refreshRateChanged_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QScreen_SlotProxy_connect__ZN7QScreen18refreshRateChangedEd(arg0, arg1, arg2)};
  }
}
// physicalSizeChanged(const class QSizeF &)
extern fn QScreen_physicalSizeChanged_signal_connect_cb_6(rsfptr:fn(QSizeF), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QSizeF::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QScreen_physicalSizeChanged_signal_connect_cb_box_6(rsfptr_raw:*mut Box<Fn(QSizeF)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QSizeF::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QScreen_physicalSizeChanged_signal_connect for fn(QSizeF) {
  fn connect(self, sigthis: QScreen_physicalSizeChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QScreen_physicalSizeChanged_signal_connect_cb_6 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QScreen_SlotProxy_connect__ZN7QScreen19physicalSizeChangedERK6QSizeF(arg0, arg1, arg2)};
  }
}
impl /* trait */ QScreen_physicalSizeChanged_signal_connect for Box<Fn(QSizeF)> {
  fn connect(self, sigthis: QScreen_physicalSizeChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QScreen_physicalSizeChanged_signal_connect_cb_box_6 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QScreen_SlotProxy_connect__ZN7QScreen19physicalSizeChangedERK6QSizeF(arg0, arg1, arg2)};
  }
}
// <= body block end

