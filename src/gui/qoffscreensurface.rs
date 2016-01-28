// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
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
use super::super::core::qobject::*; // 771
use std::ops::Deref;
use super::qscreen::*; // 773
use super::qsurfaceformat::*; // 773
// use super::qplatformoffscreensurface::*; // 775
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qsize::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QOffscreenSurface_Class_Size() -> c_int;
  // proto:  void QOffscreenSurface::~QOffscreenSurface();
  fn C_ZN17QOffscreenSurfaceD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QOffscreenSurface::QOffscreenSurface(QScreen * screen);
  fn C_ZN17QOffscreenSurfaceC2EP7QScreen(arg0: *mut c_void) -> u64;
  // proto:  QScreen * QOffscreenSurface::screen();
  fn C_ZNK17QOffscreenSurface6screenEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QOffscreenSurface::setFormat(const QSurfaceFormat & format);
  fn C_ZN17QOffscreenSurface9setFormatERK14QSurfaceFormat(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QOffscreenSurface::setScreen(QScreen * screen);
  fn C_ZN17QOffscreenSurface9setScreenEP7QScreen(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSurfaceFormat QOffscreenSurface::requestedFormat();
  fn C_ZNK17QOffscreenSurface15requestedFormatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSurfaceFormat QOffscreenSurface::format();
  fn C_ZNK17QOffscreenSurface6formatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPlatformOffscreenSurface * QOffscreenSurface::handle();
  fn C_ZNK17QOffscreenSurface6handleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QOffscreenSurface::metaObject();
  fn C_ZNK17QOffscreenSurface10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QOffscreenSurface::destroy();
  fn C_ZN17QOffscreenSurface7destroyEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QOffscreenSurface::isValid();
  fn C_ZNK17QOffscreenSurface7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QSize QOffscreenSurface::size();
  fn C_ZNK17QOffscreenSurface4sizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QOffscreenSurface::create();
  fn C_ZN17QOffscreenSurface6createEv(qthis: u64 /* *mut c_void*/);
  fn QOffscreenSurface_SlotProxy_connect__ZN17QOffscreenSurface13screenChangedEP7QScreen(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QOffscreenSurface)=1
#[derive(Default)]
pub struct QOffscreenSurface {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _screenChanged: QOffscreenSurface_screenChanged_signal,
}

impl /*struct*/ QOffscreenSurface {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOffscreenSurface {
    return QOffscreenSurface{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn free<RetType, T: QOffscreenSurface_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_free<RetType> {
  fn free(self , rsthis: & QOffscreenSurface) -> RetType;
}

  // proto:  void QOffscreenSurface::~QOffscreenSurface();
impl<'a> /*trait*/ QOffscreenSurface_free<()> for () {
  fn free(self , rsthis: & QOffscreenSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurfaceD2Ev()};
     unsafe {C_ZN17QOffscreenSurfaceD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOffscreenSurface::QOffscreenSurface(QScreen * screen);
impl /*struct*/ QOffscreenSurface {
  pub fn new<T: QOffscreenSurface_new>(value: T) -> QOffscreenSurface {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QOffscreenSurface_new {
  fn new(self) -> QOffscreenSurface;
}

  // proto:  void QOffscreenSurface::QOffscreenSurface(QScreen * screen);
impl<'a> /*trait*/ QOffscreenSurface_new for (&'a QScreen) {
  fn new(self) -> QOffscreenSurface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurfaceC2EP7QScreen()};
    let ctysz: c_int = unsafe{QOffscreenSurface_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN17QOffscreenSurfaceC2EP7QScreen(arg0)};
    let rsthis = QOffscreenSurface{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK17QOffscreenSurface6screenEv(rsthis.qclsinst)};
    let mut ret1 = QScreen::inheritFrom(ret as u64);
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
     unsafe {C_ZN17QOffscreenSurface9setFormatERK14QSurfaceFormat(rsthis.qclsinst, arg0)};
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
     unsafe {C_ZN17QOffscreenSurface9setScreenEP7QScreen(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZNK17QOffscreenSurface15requestedFormatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK17QOffscreenSurface6formatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat::inheritFrom(ret as u64);
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
impl<'a> /*trait*/ QOffscreenSurface_handle<u64> for () {
  fn handle(self , rsthis: & QOffscreenSurface) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface6handleEv()};
    let mut ret = unsafe {C_ZNK17QOffscreenSurface6handleEv(rsthis.qclsinst)};
    return ret as u64; // 4
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
impl<'a> /*trait*/ QOffscreenSurface_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QOffscreenSurface) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface10metaObjectEv()};
    let mut ret = unsafe {C_ZNK17QOffscreenSurface10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
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
     unsafe {C_ZN17QOffscreenSurface7destroyEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {C_ZNK17QOffscreenSurface7isValidEv(rsthis.qclsinst)};
    return ret as i8; // 1
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
    let mut ret = unsafe {C_ZNK17QOffscreenSurface4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
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
     unsafe {C_ZN17QOffscreenSurface6createEv(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QOffscreenSurface_screenChanged
pub struct QOffscreenSurface_screenChanged_signal{poi:u64}
impl /* struct */ QOffscreenSurface {
  pub fn screenChanged(&self) -> QOffscreenSurface_screenChanged_signal {
     return QOffscreenSurface_screenChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QOffscreenSurface_screenChanged_signal {
  pub fn connect<T: QOffscreenSurface_screenChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QOffscreenSurface_screenChanged_signal_connect {
  fn connect(self, sigthis: QOffscreenSurface_screenChanged_signal);
}

// screenChanged(class QScreen *)
extern fn QOffscreenSurface_screenChanged_signal_connect_cb_0(rsfptr:fn(QScreen), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QScreen::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QOffscreenSurface_screenChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QScreen)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QScreen::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QOffscreenSurface_screenChanged_signal_connect for fn(QScreen) {
  fn connect(self, sigthis: QOffscreenSurface_screenChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QOffscreenSurface_screenChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QOffscreenSurface_SlotProxy_connect__ZN17QOffscreenSurface13screenChangedEP7QScreen(arg0, arg1, arg2)};
  }
}
impl /* trait */ QOffscreenSurface_screenChanged_signal_connect for Box<Fn(QScreen)> {
  fn connect(self, sigthis: QOffscreenSurface_screenChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QOffscreenSurface_screenChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QOffscreenSurface_SlotProxy_connect__ZN17QOffscreenSurface13screenChangedEP7QScreen(arg0, arg1, arg2)};
  }
}
// <= body block end

