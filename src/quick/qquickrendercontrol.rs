// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qquickrendercontrol.h
// dst-file: /src/quick/qquickrendercontrol.rs
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
use super::super::core::qpoint::QPoint; // 771
use super::super::gui::qwindow::QWindow; // 771
use super::super::core::qthread::QThread; // 771
use super::super::gui::qimage::QImage; // 771
use super::qquickwindow::QQuickWindow; // 773
use super::super::gui::qopenglcontext::QOpenGLContext; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQuickRenderControl_Class_Size() -> c_int;
  // proto:  void QQuickRenderControl::invalidate();
  fn _ZN19QQuickRenderControl10invalidateEv(qthis: u64 /* *mut c_void*/);
  // proto:  QWindow * QQuickRenderControl::renderWindow(QPoint * offset);
  fn _ZN19QQuickRenderControl12renderWindowEP6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QQuickRenderControl::render();
  fn _ZN19QQuickRenderControl6renderEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QQuickRenderControl::sync();
  fn _ZN19QQuickRenderControl4syncEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQuickRenderControl::polishItems();
  fn _ZN19QQuickRenderControl11polishItemsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQuickRenderControl::QQuickRenderControl(QObject * parent);
  fn _ZN19QQuickRenderControlC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickRenderControl::prepareThread(QThread * targetThread);
  fn _ZN19QQuickRenderControl13prepareThreadEP7QThread(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickRenderControl::~QQuickRenderControl();
  fn _ZN19QQuickRenderControlD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QImage QQuickRenderControl::grab();
  fn _ZN19QQuickRenderControl4grabEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QQuickRenderControl::metaObject();
  fn _ZNK19QQuickRenderControl10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto: static QWindow * QQuickRenderControl::renderWindowFor(QQuickWindow * win, QPoint * offset);
  fn _ZN19QQuickRenderControl15renderWindowForEP12QQuickWindowP6QPoint(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QQuickRenderControl::initialize(QOpenGLContext * gl);
  fn _ZN19QQuickRenderControl10initializeEP14QOpenGLContext(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QQuickRenderControl_SlotProxy_connect__ZN19QQuickRenderControl12sceneChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickRenderControl_SlotProxy_connect__ZN19QQuickRenderControl15renderRequestedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QQuickRenderControl)=1
#[derive(Default)]
pub struct QQuickRenderControl {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _sceneChanged: QQuickRenderControl_sceneChanged_signal,
  pub _renderRequested: QQuickRenderControl_renderRequested_signal,
}

impl /*struct*/ QQuickRenderControl {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQuickRenderControl {
    return QQuickRenderControl{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQuickRenderControl {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QQuickRenderControl {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QQuickRenderControl::invalidate();
impl /*struct*/ QQuickRenderControl {
  pub fn invalidate<RetType, T: QQuickRenderControl_invalidate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.invalidate(self);
    // return 1;
  }
}

pub trait QQuickRenderControl_invalidate<RetType> {
  fn invalidate(self , rsthis: & QQuickRenderControl) -> RetType;
}

  // proto:  void QQuickRenderControl::invalidate();
impl<'a> /*trait*/ QQuickRenderControl_invalidate<()> for () {
  fn invalidate(self , rsthis: & QQuickRenderControl) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQuickRenderControl10invalidateEv()};
     unsafe {_ZN19QQuickRenderControl10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QWindow * QQuickRenderControl::renderWindow(QPoint * offset);
impl /*struct*/ QQuickRenderControl {
  pub fn renderWindow<RetType, T: QQuickRenderControl_renderWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.renderWindow(self);
    // return 1;
  }
}

pub trait QQuickRenderControl_renderWindow<RetType> {
  fn renderWindow(self , rsthis: & QQuickRenderControl) -> RetType;
}

  // proto:  QWindow * QQuickRenderControl::renderWindow(QPoint * offset);
impl<'a> /*trait*/ QQuickRenderControl_renderWindow<QWindow> for (&'a QPoint) {
  fn renderWindow(self , rsthis: & QQuickRenderControl) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQuickRenderControl12renderWindowEP6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN19QQuickRenderControl12renderWindowEP6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QWindow::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickRenderControl::render();
impl /*struct*/ QQuickRenderControl {
  pub fn render<RetType, T: QQuickRenderControl_render<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.render(self);
    // return 1;
  }
}

pub trait QQuickRenderControl_render<RetType> {
  fn render(self , rsthis: & QQuickRenderControl) -> RetType;
}

  // proto:  void QQuickRenderControl::render();
impl<'a> /*trait*/ QQuickRenderControl_render<()> for () {
  fn render(self , rsthis: & QQuickRenderControl) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQuickRenderControl6renderEv()};
     unsafe {_ZN19QQuickRenderControl6renderEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QQuickRenderControl::sync();
impl /*struct*/ QQuickRenderControl {
  pub fn sync<RetType, T: QQuickRenderControl_sync<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sync(self);
    // return 1;
  }
}

pub trait QQuickRenderControl_sync<RetType> {
  fn sync(self , rsthis: & QQuickRenderControl) -> RetType;
}

  // proto:  bool QQuickRenderControl::sync();
impl<'a> /*trait*/ QQuickRenderControl_sync<i8> for () {
  fn sync(self , rsthis: & QQuickRenderControl) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQuickRenderControl4syncEv()};
    let mut ret = unsafe {_ZN19QQuickRenderControl4syncEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQuickRenderControl::polishItems();
impl /*struct*/ QQuickRenderControl {
  pub fn polishItems<RetType, T: QQuickRenderControl_polishItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.polishItems(self);
    // return 1;
  }
}

pub trait QQuickRenderControl_polishItems<RetType> {
  fn polishItems(self , rsthis: & QQuickRenderControl) -> RetType;
}

  // proto:  void QQuickRenderControl::polishItems();
impl<'a> /*trait*/ QQuickRenderControl_polishItems<()> for () {
  fn polishItems(self , rsthis: & QQuickRenderControl) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQuickRenderControl11polishItemsEv()};
     unsafe {_ZN19QQuickRenderControl11polishItemsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQuickRenderControl::QQuickRenderControl(QObject * parent);
impl /*struct*/ QQuickRenderControl {
  pub fn new<T: QQuickRenderControl_new>(value: T) -> QQuickRenderControl {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQuickRenderControl_new {
  fn new(self) -> QQuickRenderControl;
}

  // proto:  void QQuickRenderControl::QQuickRenderControl(QObject * parent);
impl<'a> /*trait*/ QQuickRenderControl_new for (&'a QObject) {
  fn new(self) -> QQuickRenderControl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQuickRenderControlC2EP7QObject()};
    let ctysz: c_int = unsafe{QQuickRenderControl_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QQuickRenderControlC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQuickRenderControl{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQuickRenderControl::prepareThread(QThread * targetThread);
impl /*struct*/ QQuickRenderControl {
  pub fn prepareThread<RetType, T: QQuickRenderControl_prepareThread<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.prepareThread(self);
    // return 1;
  }
}

pub trait QQuickRenderControl_prepareThread<RetType> {
  fn prepareThread(self , rsthis: & QQuickRenderControl) -> RetType;
}

  // proto:  void QQuickRenderControl::prepareThread(QThread * targetThread);
impl<'a> /*trait*/ QQuickRenderControl_prepareThread<()> for (&'a QThread) {
  fn prepareThread(self , rsthis: & QQuickRenderControl) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQuickRenderControl13prepareThreadEP7QThread()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QQuickRenderControl13prepareThreadEP7QThread(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickRenderControl::~QQuickRenderControl();
impl /*struct*/ QQuickRenderControl {
  pub fn free<RetType, T: QQuickRenderControl_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQuickRenderControl_free<RetType> {
  fn free(self , rsthis: & QQuickRenderControl) -> RetType;
}

  // proto:  void QQuickRenderControl::~QQuickRenderControl();
impl<'a> /*trait*/ QQuickRenderControl_free<()> for () {
  fn free(self , rsthis: & QQuickRenderControl) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQuickRenderControlD2Ev()};
     unsafe {_ZN19QQuickRenderControlD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QImage QQuickRenderControl::grab();
impl /*struct*/ QQuickRenderControl {
  pub fn grab<RetType, T: QQuickRenderControl_grab<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.grab(self);
    // return 1;
  }
}

pub trait QQuickRenderControl_grab<RetType> {
  fn grab(self , rsthis: & QQuickRenderControl) -> RetType;
}

  // proto:  QImage QQuickRenderControl::grab();
impl<'a> /*trait*/ QQuickRenderControl_grab<QImage> for () {
  fn grab(self , rsthis: & QQuickRenderControl) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQuickRenderControl4grabEv()};
    let mut ret = unsafe {_ZN19QQuickRenderControl4grabEv(rsthis.qclsinst)};
    let mut ret1 = QImage::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QQuickRenderControl::metaObject();
impl /*struct*/ QQuickRenderControl {
  pub fn metaObject<RetType, T: QQuickRenderControl_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QQuickRenderControl_metaObject<RetType> {
  fn metaObject(self , rsthis: & QQuickRenderControl) -> RetType;
}

  // proto:  const QMetaObject * QQuickRenderControl::metaObject();
impl<'a> /*trait*/ QQuickRenderControl_metaObject<()> for () {
  fn metaObject(self , rsthis: & QQuickRenderControl) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QQuickRenderControl10metaObjectEv()};
     unsafe {_ZNK19QQuickRenderControl10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QWindow * QQuickRenderControl::renderWindowFor(QQuickWindow * win, QPoint * offset);
impl /*struct*/ QQuickRenderControl {
  pub fn renderWindowFor_s<RetType, T: QQuickRenderControl_renderWindowFor_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.renderWindowFor_s();
    // return 1;
  }
}

pub trait QQuickRenderControl_renderWindowFor_s<RetType> {
  fn renderWindowFor_s(self ) -> RetType;
}

  // proto: static QWindow * QQuickRenderControl::renderWindowFor(QQuickWindow * win, QPoint * offset);
impl<'a> /*trait*/ QQuickRenderControl_renderWindowFor_s<QWindow> for (&'a QQuickWindow, &'a QPoint) {
  fn renderWindowFor_s(self ) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQuickRenderControl15renderWindowForEP12QQuickWindowP6QPoint()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN19QQuickRenderControl15renderWindowForEP12QQuickWindowP6QPoint(arg0, arg1)};
    let mut ret1 = QWindow::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickRenderControl::initialize(QOpenGLContext * gl);
impl /*struct*/ QQuickRenderControl {
  pub fn initialize<RetType, T: QQuickRenderControl_initialize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.initialize(self);
    // return 1;
  }
}

pub trait QQuickRenderControl_initialize<RetType> {
  fn initialize(self , rsthis: & QQuickRenderControl) -> RetType;
}

  // proto:  void QQuickRenderControl::initialize(QOpenGLContext * gl);
impl<'a> /*trait*/ QQuickRenderControl_initialize<()> for (&'a QOpenGLContext) {
  fn initialize(self , rsthis: & QQuickRenderControl) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QQuickRenderControl10initializeEP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QQuickRenderControl10initializeEP14QOpenGLContext(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QQuickRenderControl_sceneChanged
pub struct QQuickRenderControl_sceneChanged_signal{poi:u64}
impl /* struct */ QQuickRenderControl {
  pub fn sceneChanged(&self) -> QQuickRenderControl_sceneChanged_signal {
     return QQuickRenderControl_sceneChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickRenderControl_sceneChanged_signal {
  pub fn connect<T: QQuickRenderControl_sceneChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickRenderControl_sceneChanged_signal_connect {
  fn connect(self, sigthis: QQuickRenderControl_sceneChanged_signal);
}

#[derive(Default)] // for QQuickRenderControl_renderRequested
pub struct QQuickRenderControl_renderRequested_signal{poi:u64}
impl /* struct */ QQuickRenderControl {
  pub fn renderRequested(&self) -> QQuickRenderControl_renderRequested_signal {
     return QQuickRenderControl_renderRequested_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickRenderControl_renderRequested_signal {
  pub fn connect<T: QQuickRenderControl_renderRequested_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickRenderControl_renderRequested_signal_connect {
  fn connect(self, sigthis: QQuickRenderControl_renderRequested_signal);
}

// sceneChanged()
extern fn QQuickRenderControl_sceneChanged_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickRenderControl_sceneChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickRenderControl_sceneChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickRenderControl_sceneChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickRenderControl_sceneChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickRenderControl_SlotProxy_connect__ZN19QQuickRenderControl12sceneChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickRenderControl_sceneChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickRenderControl_sceneChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickRenderControl_sceneChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickRenderControl_SlotProxy_connect__ZN19QQuickRenderControl12sceneChangedEv(arg0, arg1, arg2)};
  }
}
// renderRequested()
extern fn QQuickRenderControl_renderRequested_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickRenderControl_renderRequested_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickRenderControl_renderRequested_signal_connect for fn() {
  fn connect(self, sigthis: QQuickRenderControl_renderRequested_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickRenderControl_renderRequested_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickRenderControl_SlotProxy_connect__ZN19QQuickRenderControl15renderRequestedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickRenderControl_renderRequested_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickRenderControl_renderRequested_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickRenderControl_renderRequested_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickRenderControl_SlotProxy_connect__ZN19QQuickRenderControl15renderRequestedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

