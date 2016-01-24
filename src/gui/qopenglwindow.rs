// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtGui/qopenglwindow.h
// dst-file: /src/gui/qopenglwindow.rs
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
use super::qpaintdevicewindow::QPaintDeviceWindow; // 773
use std::ops::Deref;
use super::qwindow::QWindow; // 773
use super::qimage::QImage; // 773
use super::qopenglcontext::QOpenGLContext; // 773
use super::super::core::qobjectdefs::QMetaObject; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QOpenGLWindow_Class_Size() -> c_int;
  // proto:  bool QOpenGLWindow::isValid();
  fn C_ZNK13QOpenGLWindow7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QImage QOpenGLWindow::grabFramebuffer();
  fn C_ZN13QOpenGLWindow15grabFramebufferEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QOpenGLContext * QOpenGLWindow::shareContext();
  fn C_ZNK13QOpenGLWindow12shareContextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QOpenGLWindow::makeCurrent();
  fn C_ZN13QOpenGLWindow11makeCurrentEv(qthis: u64 /* *mut c_void*/);
  // proto:  QOpenGLContext * QOpenGLWindow::context();
  fn C_ZNK13QOpenGLWindow7contextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QOpenGLWindow::doneCurrent();
  fn C_ZN13QOpenGLWindow11doneCurrentEv(qthis: u64 /* *mut c_void*/);
  // proto:  GLuint QOpenGLWindow::defaultFramebufferObject();
  fn C_ZNK13QOpenGLWindow24defaultFramebufferObjectEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  void QOpenGLWindow::~QOpenGLWindow();
  fn C_ZN13QOpenGLWindowD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QOpenGLWindow::metaObject();
  fn C_ZNK13QOpenGLWindow10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QOpenGLWindow_SlotProxy_connect__ZN13QOpenGLWindow12frameSwappedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QOpenGLWindow)=1
#[derive(Default)]
pub struct QOpenGLWindow {
  qbase: QPaintDeviceWindow,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _frameSwapped: QOpenGLWindow_frameSwapped_signal,
}

impl /*struct*/ QOpenGLWindow {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLWindow {
    return QOpenGLWindow{qbase: QPaintDeviceWindow::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLWindow {
  type Target = QPaintDeviceWindow;

  fn deref(&self) -> &QPaintDeviceWindow {
    return & self.qbase;
  }
}
impl AsRef<QPaintDeviceWindow> for QOpenGLWindow {
  fn as_ref(& self) -> & QPaintDeviceWindow {
    return & self.qbase;
  }
}
  // proto:  bool QOpenGLWindow::isValid();
impl /*struct*/ QOpenGLWindow {
  pub fn isValid<RetType, T: QOpenGLWindow_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QOpenGLWindow_isValid<RetType> {
  fn isValid(self , rsthis: & QOpenGLWindow) -> RetType;
}

  // proto:  bool QOpenGLWindow::isValid();
impl<'a> /*trait*/ QOpenGLWindow_isValid<i8> for () {
  fn isValid(self , rsthis: & QOpenGLWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWindow7isValidEv()};
    let mut ret = unsafe {C_ZNK13QOpenGLWindow7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QImage QOpenGLWindow::grabFramebuffer();
impl /*struct*/ QOpenGLWindow {
  pub fn grabFramebuffer<RetType, T: QOpenGLWindow_grabFramebuffer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.grabFramebuffer(self);
    // return 1;
  }
}

pub trait QOpenGLWindow_grabFramebuffer<RetType> {
  fn grabFramebuffer(self , rsthis: & QOpenGLWindow) -> RetType;
}

  // proto:  QImage QOpenGLWindow::grabFramebuffer();
impl<'a> /*trait*/ QOpenGLWindow_grabFramebuffer<QImage> for () {
  fn grabFramebuffer(self , rsthis: & QOpenGLWindow) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWindow15grabFramebufferEv()};
    let mut ret = unsafe {C_ZN13QOpenGLWindow15grabFramebufferEv(rsthis.qclsinst)};
    let mut ret1 = QImage::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QOpenGLContext * QOpenGLWindow::shareContext();
impl /*struct*/ QOpenGLWindow {
  pub fn shareContext<RetType, T: QOpenGLWindow_shareContext<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shareContext(self);
    // return 1;
  }
}

pub trait QOpenGLWindow_shareContext<RetType> {
  fn shareContext(self , rsthis: & QOpenGLWindow) -> RetType;
}

  // proto:  QOpenGLContext * QOpenGLWindow::shareContext();
impl<'a> /*trait*/ QOpenGLWindow_shareContext<QOpenGLContext> for () {
  fn shareContext(self , rsthis: & QOpenGLWindow) -> QOpenGLContext {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWindow12shareContextEv()};
    let mut ret = unsafe {C_ZNK13QOpenGLWindow12shareContextEv(rsthis.qclsinst)};
    let mut ret1 = QOpenGLContext::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QOpenGLWindow::makeCurrent();
impl /*struct*/ QOpenGLWindow {
  pub fn makeCurrent<RetType, T: QOpenGLWindow_makeCurrent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.makeCurrent(self);
    // return 1;
  }
}

pub trait QOpenGLWindow_makeCurrent<RetType> {
  fn makeCurrent(self , rsthis: & QOpenGLWindow) -> RetType;
}

  // proto:  void QOpenGLWindow::makeCurrent();
impl<'a> /*trait*/ QOpenGLWindow_makeCurrent<()> for () {
  fn makeCurrent(self , rsthis: & QOpenGLWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWindow11makeCurrentEv()};
     unsafe {C_ZN13QOpenGLWindow11makeCurrentEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QOpenGLContext * QOpenGLWindow::context();
impl /*struct*/ QOpenGLWindow {
  pub fn context<RetType, T: QOpenGLWindow_context<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.context(self);
    // return 1;
  }
}

pub trait QOpenGLWindow_context<RetType> {
  fn context(self , rsthis: & QOpenGLWindow) -> RetType;
}

  // proto:  QOpenGLContext * QOpenGLWindow::context();
impl<'a> /*trait*/ QOpenGLWindow_context<QOpenGLContext> for () {
  fn context(self , rsthis: & QOpenGLWindow) -> QOpenGLContext {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWindow7contextEv()};
    let mut ret = unsafe {C_ZNK13QOpenGLWindow7contextEv(rsthis.qclsinst)};
    let mut ret1 = QOpenGLContext::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QOpenGLWindow::doneCurrent();
impl /*struct*/ QOpenGLWindow {
  pub fn doneCurrent<RetType, T: QOpenGLWindow_doneCurrent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.doneCurrent(self);
    // return 1;
  }
}

pub trait QOpenGLWindow_doneCurrent<RetType> {
  fn doneCurrent(self , rsthis: & QOpenGLWindow) -> RetType;
}

  // proto:  void QOpenGLWindow::doneCurrent();
impl<'a> /*trait*/ QOpenGLWindow_doneCurrent<()> for () {
  fn doneCurrent(self , rsthis: & QOpenGLWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWindow11doneCurrentEv()};
     unsafe {C_ZN13QOpenGLWindow11doneCurrentEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  GLuint QOpenGLWindow::defaultFramebufferObject();
impl /*struct*/ QOpenGLWindow {
  pub fn defaultFramebufferObject<RetType, T: QOpenGLWindow_defaultFramebufferObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultFramebufferObject(self);
    // return 1;
  }
}

pub trait QOpenGLWindow_defaultFramebufferObject<RetType> {
  fn defaultFramebufferObject(self , rsthis: & QOpenGLWindow) -> RetType;
}

  // proto:  GLuint QOpenGLWindow::defaultFramebufferObject();
impl<'a> /*trait*/ QOpenGLWindow_defaultFramebufferObject<u32> for () {
  fn defaultFramebufferObject(self , rsthis: & QOpenGLWindow) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWindow24defaultFramebufferObjectEv()};
    let mut ret = unsafe {C_ZNK13QOpenGLWindow24defaultFramebufferObjectEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  void QOpenGLWindow::~QOpenGLWindow();
impl /*struct*/ QOpenGLWindow {
  pub fn free<RetType, T: QOpenGLWindow_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QOpenGLWindow_free<RetType> {
  fn free(self , rsthis: & QOpenGLWindow) -> RetType;
}

  // proto:  void QOpenGLWindow::~QOpenGLWindow();
impl<'a> /*trait*/ QOpenGLWindow_free<()> for () {
  fn free(self , rsthis: & QOpenGLWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWindowD2Ev()};
     unsafe {C_ZN13QOpenGLWindowD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QOpenGLWindow::metaObject();
impl /*struct*/ QOpenGLWindow {
  pub fn metaObject<RetType, T: QOpenGLWindow_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLWindow_metaObject<RetType> {
  fn metaObject(self , rsthis: & QOpenGLWindow) -> RetType;
}

  // proto:  const QMetaObject * QOpenGLWindow::metaObject();
impl<'a> /*trait*/ QOpenGLWindow_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QOpenGLWindow) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWindow10metaObjectEv()};
    let mut ret = unsafe {C_ZNK13QOpenGLWindow10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QOpenGLWindow_frameSwapped
pub struct QOpenGLWindow_frameSwapped_signal{poi:u64}
impl /* struct */ QOpenGLWindow {
  pub fn frameSwapped(&self) -> QOpenGLWindow_frameSwapped_signal {
     return QOpenGLWindow_frameSwapped_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QOpenGLWindow_frameSwapped_signal {
  pub fn connect<T: QOpenGLWindow_frameSwapped_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QOpenGLWindow_frameSwapped_signal_connect {
  fn connect(self, sigthis: QOpenGLWindow_frameSwapped_signal);
}

// frameSwapped()
extern fn QOpenGLWindow_frameSwapped_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QOpenGLWindow_frameSwapped_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QOpenGLWindow_frameSwapped_signal_connect for fn() {
  fn connect(self, sigthis: QOpenGLWindow_frameSwapped_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QOpenGLWindow_frameSwapped_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QOpenGLWindow_SlotProxy_connect__ZN13QOpenGLWindow12frameSwappedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QOpenGLWindow_frameSwapped_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QOpenGLWindow_frameSwapped_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QOpenGLWindow_frameSwapped_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QOpenGLWindow_SlotProxy_connect__ZN13QOpenGLWindow12frameSwappedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

