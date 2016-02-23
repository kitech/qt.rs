// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qopenglwidget.h
// dst-file: /src/widgets/qopenglwidget.rs
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
use super::qwidget::*; // 773
use std::ops::Deref;
use super::super::gui::qopenglcontext::*; // 771
use super::super::gui::qimage::*; // 771
use super::super::core::qobjectdefs::*; // 771
use super::super::gui::qsurfaceformat::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QOpenGLWidget_Class_Size() -> c_int;
  // proto:  void QOpenGLWidget::~QOpenGLWidget();
  fn C_ZN13QOpenGLWidgetD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  GLuint QOpenGLWidget::defaultFramebufferObject();
  fn C_ZNK13QOpenGLWidget24defaultFramebufferObjectEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  bool QOpenGLWidget::isValid();
  fn C_ZNK13QOpenGLWidget7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QOpenGLContext * QOpenGLWidget::context();
  fn C_ZNK13QOpenGLWidget7contextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QOpenGLWidget::doneCurrent();
  fn C_ZN13QOpenGLWidget11doneCurrentEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QOpenGLWidget::makeCurrent();
  fn C_ZN13QOpenGLWidget11makeCurrentEv(qthis: u64 /* *mut c_void*/);
  // proto:  QImage QOpenGLWidget::grabFramebuffer();
  fn C_ZN13QOpenGLWidget15grabFramebufferEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QOpenGLWidget::metaObject();
  fn C_ZNK13QOpenGLWidget10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QOpenGLWidget::setFormat(const QSurfaceFormat & format);
  fn C_ZN13QOpenGLWidget9setFormatERK14QSurfaceFormat(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSurfaceFormat QOpenGLWidget::format();
  fn C_ZNK13QOpenGLWidget6formatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QOpenGLWidget_SlotProxy_connect__ZN13QOpenGLWidget13aboutToResizeEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QOpenGLWidget_SlotProxy_connect__ZN13QOpenGLWidget12frameSwappedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QOpenGLWidget_SlotProxy_connect__ZN13QOpenGLWidget7resizedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QOpenGLWidget_SlotProxy_connect__ZN13QOpenGLWidget14aboutToComposeEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QOpenGLWidget)=1
#[derive(Default)]
pub struct QOpenGLWidget {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _aboutToResize: QOpenGLWidget_aboutToResize_signal,
  pub _resized: QOpenGLWidget_resized_signal,
  pub _frameSwapped: QOpenGLWidget_frameSwapped_signal,
  pub _aboutToCompose: QOpenGLWidget_aboutToCompose_signal,
}

impl /*struct*/ QOpenGLWidget {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLWidget {
    return QOpenGLWidget{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLWidget {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QOpenGLWidget {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLWidget::~QOpenGLWidget();
impl /*struct*/ QOpenGLWidget {
  pub fn free<RetType, T: QOpenGLWidget_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_free<RetType> {
  fn free(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  void QOpenGLWidget::~QOpenGLWidget();
impl<'a> /*trait*/ QOpenGLWidget_free<()> for () {
  fn free(self , rsthis: & QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidgetD2Ev()};
     unsafe {C_ZN13QOpenGLWidgetD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  GLuint QOpenGLWidget::defaultFramebufferObject();
impl /*struct*/ QOpenGLWidget {
  pub fn defaultFramebufferObject<RetType, T: QOpenGLWidget_defaultFramebufferObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultFramebufferObject(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_defaultFramebufferObject<RetType> {
  fn defaultFramebufferObject(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  GLuint QOpenGLWidget::defaultFramebufferObject();
impl<'a> /*trait*/ QOpenGLWidget_defaultFramebufferObject<u32> for () {
  fn defaultFramebufferObject(self , rsthis: & QOpenGLWidget) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget24defaultFramebufferObjectEv()};
    let mut ret = unsafe {C_ZNK13QOpenGLWidget24defaultFramebufferObjectEv(rsthis.qclsinst)};
    return ret as u32; // 1
    // return 1;
  }
}

  // proto:  bool QOpenGLWidget::isValid();
impl /*struct*/ QOpenGLWidget {
  pub fn isValid<RetType, T: QOpenGLWidget_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_isValid<RetType> {
  fn isValid(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  bool QOpenGLWidget::isValid();
impl<'a> /*trait*/ QOpenGLWidget_isValid<i8> for () {
  fn isValid(self , rsthis: & QOpenGLWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget7isValidEv()};
    let mut ret = unsafe {C_ZNK13QOpenGLWidget7isValidEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QOpenGLContext * QOpenGLWidget::context();
impl /*struct*/ QOpenGLWidget {
  pub fn context<RetType, T: QOpenGLWidget_context<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.context(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_context<RetType> {
  fn context(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  QOpenGLContext * QOpenGLWidget::context();
impl<'a> /*trait*/ QOpenGLWidget_context<QOpenGLContext> for () {
  fn context(self , rsthis: & QOpenGLWidget) -> QOpenGLContext {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget7contextEv()};
    let mut ret = unsafe {C_ZNK13QOpenGLWidget7contextEv(rsthis.qclsinst)};
    let mut ret1 = QOpenGLContext::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QOpenGLWidget::doneCurrent();
impl /*struct*/ QOpenGLWidget {
  pub fn doneCurrent<RetType, T: QOpenGLWidget_doneCurrent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.doneCurrent(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_doneCurrent<RetType> {
  fn doneCurrent(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  void QOpenGLWidget::doneCurrent();
impl<'a> /*trait*/ QOpenGLWidget_doneCurrent<()> for () {
  fn doneCurrent(self , rsthis: & QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget11doneCurrentEv()};
     unsafe {C_ZN13QOpenGLWidget11doneCurrentEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLWidget::makeCurrent();
impl /*struct*/ QOpenGLWidget {
  pub fn makeCurrent<RetType, T: QOpenGLWidget_makeCurrent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.makeCurrent(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_makeCurrent<RetType> {
  fn makeCurrent(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  void QOpenGLWidget::makeCurrent();
impl<'a> /*trait*/ QOpenGLWidget_makeCurrent<()> for () {
  fn makeCurrent(self , rsthis: & QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget11makeCurrentEv()};
     unsafe {C_ZN13QOpenGLWidget11makeCurrentEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QImage QOpenGLWidget::grabFramebuffer();
impl /*struct*/ QOpenGLWidget {
  pub fn grabFramebuffer<RetType, T: QOpenGLWidget_grabFramebuffer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.grabFramebuffer(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_grabFramebuffer<RetType> {
  fn grabFramebuffer(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  QImage QOpenGLWidget::grabFramebuffer();
impl<'a> /*trait*/ QOpenGLWidget_grabFramebuffer<QImage> for () {
  fn grabFramebuffer(self , rsthis: & QOpenGLWidget) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget15grabFramebufferEv()};
    let mut ret = unsafe {C_ZN13QOpenGLWidget15grabFramebufferEv(rsthis.qclsinst)};
    let mut ret1 = QImage::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QOpenGLWidget::metaObject();
impl /*struct*/ QOpenGLWidget {
  pub fn metaObject<RetType, T: QOpenGLWidget_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_metaObject<RetType> {
  fn metaObject(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  const QMetaObject * QOpenGLWidget::metaObject();
impl<'a> /*trait*/ QOpenGLWidget_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QOpenGLWidget) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget10metaObjectEv()};
    let mut ret = unsafe {C_ZNK13QOpenGLWidget10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QOpenGLWidget::setFormat(const QSurfaceFormat & format);
impl /*struct*/ QOpenGLWidget {
  pub fn setFormat<RetType, T: QOpenGLWidget_setFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_setFormat<RetType> {
  fn setFormat(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  void QOpenGLWidget::setFormat(const QSurfaceFormat & format);
impl<'a> /*trait*/ QOpenGLWidget_setFormat<()> for (&'a QSurfaceFormat) {
  fn setFormat(self , rsthis: & QOpenGLWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWidget9setFormatERK14QSurfaceFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QOpenGLWidget9setFormatERK14QSurfaceFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSurfaceFormat QOpenGLWidget::format();
impl /*struct*/ QOpenGLWidget {
  pub fn format<RetType, T: QOpenGLWidget_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QOpenGLWidget_format<RetType> {
  fn format(self , rsthis: & QOpenGLWidget) -> RetType;
}

  // proto:  QSurfaceFormat QOpenGLWidget::format();
impl<'a> /*trait*/ QOpenGLWidget_format<QSurfaceFormat> for () {
  fn format(self , rsthis: & QOpenGLWidget) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWidget6formatEv()};
    let mut ret = unsafe {C_ZNK13QOpenGLWidget6formatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QOpenGLWidget_aboutToResize
pub struct QOpenGLWidget_aboutToResize_signal{poi:u64}
impl /* struct */ QOpenGLWidget {
  pub fn aboutToResize(&self) -> QOpenGLWidget_aboutToResize_signal {
     return QOpenGLWidget_aboutToResize_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QOpenGLWidget_aboutToResize_signal {
  pub fn connect<T: QOpenGLWidget_aboutToResize_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QOpenGLWidget_aboutToResize_signal_connect {
  fn connect(self, sigthis: QOpenGLWidget_aboutToResize_signal);
}

#[derive(Default)] // for QOpenGLWidget_resized
pub struct QOpenGLWidget_resized_signal{poi:u64}
impl /* struct */ QOpenGLWidget {
  pub fn resized(&self) -> QOpenGLWidget_resized_signal {
     return QOpenGLWidget_resized_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QOpenGLWidget_resized_signal {
  pub fn connect<T: QOpenGLWidget_resized_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QOpenGLWidget_resized_signal_connect {
  fn connect(self, sigthis: QOpenGLWidget_resized_signal);
}

#[derive(Default)] // for QOpenGLWidget_frameSwapped
pub struct QOpenGLWidget_frameSwapped_signal{poi:u64}
impl /* struct */ QOpenGLWidget {
  pub fn frameSwapped(&self) -> QOpenGLWidget_frameSwapped_signal {
     return QOpenGLWidget_frameSwapped_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QOpenGLWidget_frameSwapped_signal {
  pub fn connect<T: QOpenGLWidget_frameSwapped_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QOpenGLWidget_frameSwapped_signal_connect {
  fn connect(self, sigthis: QOpenGLWidget_frameSwapped_signal);
}

#[derive(Default)] // for QOpenGLWidget_aboutToCompose
pub struct QOpenGLWidget_aboutToCompose_signal{poi:u64}
impl /* struct */ QOpenGLWidget {
  pub fn aboutToCompose(&self) -> QOpenGLWidget_aboutToCompose_signal {
     return QOpenGLWidget_aboutToCompose_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QOpenGLWidget_aboutToCompose_signal {
  pub fn connect<T: QOpenGLWidget_aboutToCompose_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QOpenGLWidget_aboutToCompose_signal_connect {
  fn connect(self, sigthis: QOpenGLWidget_aboutToCompose_signal);
}

// aboutToResize()
extern fn QOpenGLWidget_aboutToResize_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QOpenGLWidget_aboutToResize_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QOpenGLWidget_aboutToResize_signal_connect for fn() {
  fn connect(self, sigthis: QOpenGLWidget_aboutToResize_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QOpenGLWidget_aboutToResize_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QOpenGLWidget_SlotProxy_connect__ZN13QOpenGLWidget13aboutToResizeEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QOpenGLWidget_aboutToResize_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QOpenGLWidget_aboutToResize_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QOpenGLWidget_aboutToResize_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QOpenGLWidget_SlotProxy_connect__ZN13QOpenGLWidget13aboutToResizeEv(arg0, arg1, arg2)};
  }
}
// frameSwapped()
extern fn QOpenGLWidget_frameSwapped_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QOpenGLWidget_frameSwapped_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QOpenGLWidget_frameSwapped_signal_connect for fn() {
  fn connect(self, sigthis: QOpenGLWidget_frameSwapped_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QOpenGLWidget_frameSwapped_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QOpenGLWidget_SlotProxy_connect__ZN13QOpenGLWidget12frameSwappedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QOpenGLWidget_frameSwapped_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QOpenGLWidget_frameSwapped_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QOpenGLWidget_frameSwapped_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QOpenGLWidget_SlotProxy_connect__ZN13QOpenGLWidget12frameSwappedEv(arg0, arg1, arg2)};
  }
}
// resized()
extern fn QOpenGLWidget_resized_signal_connect_cb_2(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QOpenGLWidget_resized_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QOpenGLWidget_resized_signal_connect for fn() {
  fn connect(self, sigthis: QOpenGLWidget_resized_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QOpenGLWidget_resized_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QOpenGLWidget_SlotProxy_connect__ZN13QOpenGLWidget7resizedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QOpenGLWidget_resized_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QOpenGLWidget_resized_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QOpenGLWidget_resized_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QOpenGLWidget_SlotProxy_connect__ZN13QOpenGLWidget7resizedEv(arg0, arg1, arg2)};
  }
}
// aboutToCompose()
extern fn QOpenGLWidget_aboutToCompose_signal_connect_cb_3(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QOpenGLWidget_aboutToCompose_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QOpenGLWidget_aboutToCompose_signal_connect for fn() {
  fn connect(self, sigthis: QOpenGLWidget_aboutToCompose_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QOpenGLWidget_aboutToCompose_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QOpenGLWidget_SlotProxy_connect__ZN13QOpenGLWidget14aboutToComposeEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QOpenGLWidget_aboutToCompose_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QOpenGLWidget_aboutToCompose_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QOpenGLWidget_aboutToCompose_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QOpenGLWidget_SlotProxy_connect__ZN13QOpenGLWidget14aboutToComposeEv(arg0, arg1, arg2)};
  }
}
// <= body block end

