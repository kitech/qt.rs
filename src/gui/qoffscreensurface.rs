// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtGui/qoffscreensurface.h
// dst-file: /src/gui/qoffscreensurface.rs
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
use super::qscreen::QScreen; // 773
use super::qsurfaceformat::QSurfaceFormat; // 773
use super::super::core::qsize::QSize; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QOffscreenSurface_Class_Size() -> c_int;
  // proto:  void QOffscreenSurface::~QOffscreenSurface();
  fn _ZN17QOffscreenSurfaceD0Ev(qthis: *mut c_void);
  // proto:  void QOffscreenSurface::screenChanged(QScreen * screen);
  fn _ZN17QOffscreenSurface13screenChangedEP7QScreen(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOffscreenSurface::QOffscreenSurface(const QOffscreenSurface & );
  fn dector_ZN17QOffscreenSurfaceC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN17QOffscreenSurfaceC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOffscreenSurface::QOffscreenSurface(QScreen * screen);
  fn dector_ZN17QOffscreenSurfaceC1EP7QScreen(arg0: *mut c_void) -> *mut c_void;
  fn _ZN17QOffscreenSurfaceC1EP7QScreen(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QScreen * QOffscreenSurface::screen();
  fn _ZNK17QOffscreenSurface6screenEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QOffscreenSurface::setFormat(const QSurfaceFormat & format);
  fn _ZN17QOffscreenSurface9setFormatERK14QSurfaceFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOffscreenSurface::setScreen(QScreen * screen);
  fn _ZN17QOffscreenSurface9setScreenEP7QScreen(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSurfaceFormat QOffscreenSurface::requestedFormat();
  fn _ZNK17QOffscreenSurface15requestedFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSurfaceFormat QOffscreenSurface::format();
  fn _ZNK17QOffscreenSurface6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPlatformOffscreenSurface * QOffscreenSurface::handle();
  fn _ZNK17QOffscreenSurface6handleEv(qthis: *mut c_void);
  // proto:  const QMetaObject * QOffscreenSurface::metaObject();
  fn _ZNK17QOffscreenSurface10metaObjectEv(qthis: *mut c_void);
  // proto:  void QOffscreenSurface::destroy();
  fn _ZN17QOffscreenSurface7destroyEv(qthis: *mut c_void);
  // proto:  bool QOffscreenSurface::isValid();
  fn _ZNK17QOffscreenSurface7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  QSize QOffscreenSurface::size();
  fn _ZNK17QOffscreenSurface4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QOffscreenSurface::create();
  fn _ZN17QOffscreenSurface6createEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QOffscreenSurface)=1
pub struct QOffscreenSurface {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOffscreenSurface {
  pub fn inheritFrom(qthis: *mut c_void) -> QOffscreenSurface {
    return QOffscreenSurface{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOffscreenSurface {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QOffscreenSurface {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QOffscreenSurface::~QOffscreenSurface();
impl /*struct*/ QOffscreenSurface {
  pub fn Free<RetType, T: QOffscreenSurface_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_Free<RetType> {
  fn Free(self , rsthis: & QOffscreenSurface) -> RetType;
}

  // proto:  void QOffscreenSurface::~QOffscreenSurface();
impl<'a> /*trait*/ QOffscreenSurface_Free<()> for () {
  fn Free(self , rsthis: & QOffscreenSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurfaceD0Ev()};
     unsafe {_ZN17QOffscreenSurfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOffscreenSurface::screenChanged(QScreen * screen);
impl /*struct*/ QOffscreenSurface {
  pub fn screenChanged<RetType, T: QOffscreenSurface_screenChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenChanged(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_screenChanged<RetType> {
  fn screenChanged(self , rsthis: & QOffscreenSurface) -> RetType;
}

  // proto:  void QOffscreenSurface::screenChanged(QScreen * screen);
impl<'a> /*trait*/ QOffscreenSurface_screenChanged<()> for (&'a QScreen) {
  fn screenChanged(self , rsthis: & QOffscreenSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurface13screenChangedEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QOffscreenSurface13screenChangedEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOffscreenSurface::QOffscreenSurface(const QOffscreenSurface & );
impl /*struct*/ QOffscreenSurface {
  pub fn New<T: QOffscreenSurface_New>(value: T) -> QOffscreenSurface {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOffscreenSurface_New {
  fn New(self) -> QOffscreenSurface;
}

  // proto:  void QOffscreenSurface::QOffscreenSurface(const QOffscreenSurface & );
impl<'a> /*trait*/ QOffscreenSurface_New for (&'a QOffscreenSurface) {
  fn New(self) -> QOffscreenSurface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurfaceC1ERKS_()};
    let ctysz: c_int = unsafe{QOffscreenSurface_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN17QOffscreenSurfaceC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN17QOffscreenSurfaceC1ERKS_(arg0)};
    let rsthis = QOffscreenSurface{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOffscreenSurface::QOffscreenSurface(QScreen * screen);
impl<'a> /*trait*/ QOffscreenSurface_New for (&'a QScreen) {
  fn New(self) -> QOffscreenSurface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurfaceC1EP7QScreen()};
    let ctysz: c_int = unsafe{QOffscreenSurface_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN17QOffscreenSurfaceC1EP7QScreen(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN17QOffscreenSurfaceC1EP7QScreen(arg0)};
    let rsthis = QOffscreenSurface{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QScreen * QOffscreenSurface::screen();
impl /*struct*/ QOffscreenSurface {
  pub fn screen<RetType, T: QOffscreenSurface_screen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screen(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_screen<RetType> {
  fn screen(self , rsthis: & QOffscreenSurface) -> RetType;
}

  // proto:  QScreen * QOffscreenSurface::screen();
impl<'a> /*trait*/ QOffscreenSurface_screen<QScreen> for () {
  fn screen(self , rsthis: & QOffscreenSurface) -> QScreen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface6screenEv()};
    let mut ret = unsafe {_ZNK17QOffscreenSurface6screenEv(rsthis.qclsinst)};
    let mut ret1 = QScreen::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QOffscreenSurface::setFormat(const QSurfaceFormat & format);
impl /*struct*/ QOffscreenSurface {
  pub fn setFormat<RetType, T: QOffscreenSurface_setFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_setFormat<RetType> {
  fn setFormat(self , rsthis: & QOffscreenSurface) -> RetType;
}

  // proto:  void QOffscreenSurface::setFormat(const QSurfaceFormat & format);
impl<'a> /*trait*/ QOffscreenSurface_setFormat<()> for (&'a QSurfaceFormat) {
  fn setFormat(self , rsthis: & QOffscreenSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurface9setFormatERK14QSurfaceFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QOffscreenSurface9setFormatERK14QSurfaceFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOffscreenSurface::setScreen(QScreen * screen);
impl /*struct*/ QOffscreenSurface {
  pub fn setScreen<RetType, T: QOffscreenSurface_setScreen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScreen(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_setScreen<RetType> {
  fn setScreen(self , rsthis: & QOffscreenSurface) -> RetType;
}

  // proto:  void QOffscreenSurface::setScreen(QScreen * screen);
impl<'a> /*trait*/ QOffscreenSurface_setScreen<()> for (&'a QScreen) {
  fn setScreen(self , rsthis: & QOffscreenSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurface9setScreenEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QOffscreenSurface9setScreenEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSurfaceFormat QOffscreenSurface::requestedFormat();
impl /*struct*/ QOffscreenSurface {
  pub fn requestedFormat<RetType, T: QOffscreenSurface_requestedFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.requestedFormat(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_requestedFormat<RetType> {
  fn requestedFormat(self , rsthis: & QOffscreenSurface) -> RetType;
}

  // proto:  QSurfaceFormat QOffscreenSurface::requestedFormat();
impl<'a> /*trait*/ QOffscreenSurface_requestedFormat<QSurfaceFormat> for () {
  fn requestedFormat(self , rsthis: & QOffscreenSurface) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface15requestedFormatEv()};
    let mut ret = unsafe {_ZNK17QOffscreenSurface15requestedFormatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QSurfaceFormat QOffscreenSurface::format();
impl /*struct*/ QOffscreenSurface {
  pub fn format<RetType, T: QOffscreenSurface_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_format<RetType> {
  fn format(self , rsthis: & QOffscreenSurface) -> RetType;
}

  // proto:  QSurfaceFormat QOffscreenSurface::format();
impl<'a> /*trait*/ QOffscreenSurface_format<QSurfaceFormat> for () {
  fn format(self , rsthis: & QOffscreenSurface) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface6formatEv()};
    let mut ret = unsafe {_ZNK17QOffscreenSurface6formatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPlatformOffscreenSurface * QOffscreenSurface::handle();
impl /*struct*/ QOffscreenSurface {
  pub fn handle<RetType, T: QOffscreenSurface_handle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.handle(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_handle<RetType> {
  fn handle(self , rsthis: & QOffscreenSurface) -> RetType;
}

  // proto:  QPlatformOffscreenSurface * QOffscreenSurface::handle();
impl<'a> /*trait*/ QOffscreenSurface_handle<()> for () {
  fn handle(self , rsthis: & QOffscreenSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface6handleEv()};
     unsafe {_ZNK17QOffscreenSurface6handleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QOffscreenSurface::metaObject();
impl /*struct*/ QOffscreenSurface {
  pub fn metaObject<RetType, T: QOffscreenSurface_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_metaObject<RetType> {
  fn metaObject(self , rsthis: & QOffscreenSurface) -> RetType;
}

  // proto:  const QMetaObject * QOffscreenSurface::metaObject();
impl<'a> /*trait*/ QOffscreenSurface_metaObject<()> for () {
  fn metaObject(self , rsthis: & QOffscreenSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface10metaObjectEv()};
     unsafe {_ZNK17QOffscreenSurface10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOffscreenSurface::destroy();
impl /*struct*/ QOffscreenSurface {
  pub fn destroy<RetType, T: QOffscreenSurface_destroy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.destroy(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_destroy<RetType> {
  fn destroy(self , rsthis: & QOffscreenSurface) -> RetType;
}

  // proto:  void QOffscreenSurface::destroy();
impl<'a> /*trait*/ QOffscreenSurface_destroy<()> for () {
  fn destroy(self , rsthis: & QOffscreenSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurface7destroyEv()};
     unsafe {_ZN17QOffscreenSurface7destroyEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOffscreenSurface::isValid();
impl /*struct*/ QOffscreenSurface {
  pub fn isValid<RetType, T: QOffscreenSurface_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_isValid<RetType> {
  fn isValid(self , rsthis: & QOffscreenSurface) -> RetType;
}

  // proto:  bool QOffscreenSurface::isValid();
impl<'a> /*trait*/ QOffscreenSurface_isValid<i8> for () {
  fn isValid(self , rsthis: & QOffscreenSurface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface7isValidEv()};
    let mut ret = unsafe {_ZNK17QOffscreenSurface7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSize QOffscreenSurface::size();
impl /*struct*/ QOffscreenSurface {
  pub fn size<RetType, T: QOffscreenSurface_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_size<RetType> {
  fn size(self , rsthis: & QOffscreenSurface) -> RetType;
}

  // proto:  QSize QOffscreenSurface::size();
impl<'a> /*trait*/ QOffscreenSurface_size<QSize> for () {
  fn size(self , rsthis: & QOffscreenSurface) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface4sizeEv()};
    let mut ret = unsafe {_ZNK17QOffscreenSurface4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QOffscreenSurface::create();
impl /*struct*/ QOffscreenSurface {
  pub fn create<RetType, T: QOffscreenSurface_create<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_create<RetType> {
  fn create(self , rsthis: & QOffscreenSurface) -> RetType;
}

  // proto:  void QOffscreenSurface::create();
impl<'a> /*trait*/ QOffscreenSurface_create<()> for () {
  fn create(self , rsthis: & QOffscreenSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurface6createEv()};
     unsafe {_ZN17QOffscreenSurface6createEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

