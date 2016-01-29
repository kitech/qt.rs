// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qquickview.h
// dst-file: /src/quick/qquickview.rs
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
use super::qquickwindow::QQuickWindow; // 773
use std::ops::Deref;
use super::super::core::qurl::QUrl; // 771
use super::super::gui::qwindow::QWindow; // 771
use super::super::qml::qqmlcomponent::QQmlComponent; // 771
use super::super::core::qobject::QObject; // 771
use super::super::core::qsize::QSize; // 771
use super::super::qml::qqmlengine::QQmlEngine; // 771
use super::qquickitem::QQuickItem; // 773
use super::super::qml::qqmlcontext::QQmlContext; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQuickView_Class_Size() -> c_int;
  // proto:  void QQuickView::setSource(const QUrl & );
  fn _ZN10QQuickView9setSourceERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickView::QQuickView(const QUrl & source, QWindow * parent);
  fn _ZN10QQuickViewC2ERK4QUrlP7QWindow(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QQuickView::setContent(const QUrl & url, QQmlComponent * component, QObject * item);
  fn _ZN10QQuickView10setContentERK4QUrlP13QQmlComponentP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QSize QQuickView::sizeHint();
  fn _ZNK10QQuickView8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQuickView::~QQuickView();
  fn _ZN10QQuickViewD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QQmlEngine * QQuickView::engine();
  fn _ZNK10QQuickView6engineEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<QQmlError> QQuickView::errors();
  fn _ZNK10QQuickView6errorsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQuickView::QQuickView(const QQuickView & );
  fn _ZN10QQuickViewC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QQuickView::metaObject();
  fn _ZNK10QQuickView10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQuickView::QQuickView(QWindow * parent);
  fn _ZN10QQuickViewC2EP7QWindow(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSize QQuickView::initialSize();
  fn _ZNK10QQuickView11initialSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QUrl QQuickView::source();
  fn _ZNK10QQuickView6sourceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QQuickItem * QQuickView::rootObject();
  fn _ZNK10QQuickView10rootObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQuickView::QQuickView(QQmlEngine * engine, QWindow * parent);
  fn _ZN10QQuickViewC2EP10QQmlEngineP7QWindow(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QQmlContext * QQuickView::rootContext();
  fn _ZNK10QQuickView11rootContextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QQuickView)=1
#[derive(Default)]
pub struct QQuickView {
  qbase: QQuickWindow,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _statusChanged: QQuickView_statusChanged_signal,
}

impl /*struct*/ QQuickView {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQuickView {
    return QQuickView{qbase: QQuickWindow::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQuickView {
  type Target = QQuickWindow;

  fn deref(&self) -> &QQuickWindow {
    return & self.qbase;
  }
}
impl AsRef<QQuickWindow> for QQuickView {
  fn as_ref(& self) -> & QQuickWindow {
    return & self.qbase;
  }
}
  // proto:  void QQuickView::setSource(const QUrl & );
impl /*struct*/ QQuickView {
  pub fn setSource<RetType, T: QQuickView_setSource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSource(self);
    // return 1;
  }
}

pub trait QQuickView_setSource<RetType> {
  fn setSource(self , rsthis: & QQuickView) -> RetType;
}

  // proto:  void QQuickView::setSource(const QUrl & );
impl<'a> /*trait*/ QQuickView_setSource<()> for (&'a QUrl) {
  fn setSource(self , rsthis: & QQuickView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickView9setSourceERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QQuickView9setSourceERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickView::QQuickView(const QUrl & source, QWindow * parent);
impl /*struct*/ QQuickView {
  pub fn new<T: QQuickView_new>(value: T) -> QQuickView {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQuickView_new {
  fn new(self) -> QQuickView;
}

  // proto:  void QQuickView::QQuickView(const QUrl & source, QWindow * parent);
impl<'a> /*trait*/ QQuickView_new for (&'a QUrl, &'a QWindow) {
  fn new(self) -> QQuickView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickViewC2ERK4QUrlP7QWindow()};
    let ctysz: c_int = unsafe{QQuickView_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN10QQuickViewC2ERK4QUrlP7QWindow(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQuickView{qbase: QQuickWindow::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQuickView::setContent(const QUrl & url, QQmlComponent * component, QObject * item);
impl /*struct*/ QQuickView {
  pub fn setContent<RetType, T: QQuickView_setContent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setContent(self);
    // return 1;
  }
}

pub trait QQuickView_setContent<RetType> {
  fn setContent(self , rsthis: & QQuickView) -> RetType;
}

  // proto:  void QQuickView::setContent(const QUrl & url, QQmlComponent * component, QObject * item);
impl<'a> /*trait*/ QQuickView_setContent<()> for (&'a QUrl, &'a QQmlComponent, &'a QObject) {
  fn setContent(self , rsthis: & QQuickView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickView10setContentERK4QUrlP13QQmlComponentP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN10QQuickView10setContentERK4QUrlP13QQmlComponentP7QObject(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QSize QQuickView::sizeHint();
impl /*struct*/ QQuickView {
  pub fn sizeHint<RetType, T: QQuickView_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QQuickView_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QQuickView) -> RetType;
}

  // proto:  QSize QQuickView::sizeHint();
impl<'a> /*trait*/ QQuickView_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QQuickView) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickView8sizeHintEv()};
    let mut ret = unsafe {_ZNK10QQuickView8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickView::~QQuickView();
impl /*struct*/ QQuickView {
  pub fn free<RetType, T: QQuickView_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQuickView_free<RetType> {
  fn free(self , rsthis: & QQuickView) -> RetType;
}

  // proto:  void QQuickView::~QQuickView();
impl<'a> /*trait*/ QQuickView_free<()> for () {
  fn free(self , rsthis: & QQuickView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickViewD2Ev()};
     unsafe {_ZN10QQuickViewD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QQmlEngine * QQuickView::engine();
impl /*struct*/ QQuickView {
  pub fn engine<RetType, T: QQuickView_engine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.engine(self);
    // return 1;
  }
}

pub trait QQuickView_engine<RetType> {
  fn engine(self , rsthis: & QQuickView) -> RetType;
}

  // proto:  QQmlEngine * QQuickView::engine();
impl<'a> /*trait*/ QQuickView_engine<QQmlEngine> for () {
  fn engine(self , rsthis: & QQuickView) -> QQmlEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickView6engineEv()};
    let mut ret = unsafe {_ZNK10QQuickView6engineEv(rsthis.qclsinst)};
    let mut ret1 = QQmlEngine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QQmlError> QQuickView::errors();
impl /*struct*/ QQuickView {
  pub fn errors<RetType, T: QQuickView_errors<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errors(self);
    // return 1;
  }
}

pub trait QQuickView_errors<RetType> {
  fn errors(self , rsthis: & QQuickView) -> RetType;
}

  // proto:  QList<QQmlError> QQuickView::errors();
impl<'a> /*trait*/ QQuickView_errors<()> for () {
  fn errors(self , rsthis: & QQuickView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickView6errorsEv()};
     unsafe {_ZNK10QQuickView6errorsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQuickView::QQuickView(const QQuickView & );
impl<'a> /*trait*/ QQuickView_new for (&'a QQuickView) {
  fn new(self) -> QQuickView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickViewC2ERKS_()};
    let ctysz: c_int = unsafe{QQuickView_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QQuickViewC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQuickView{qbase: QQuickWindow::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QQuickView::metaObject();
impl /*struct*/ QQuickView {
  pub fn metaObject<RetType, T: QQuickView_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QQuickView_metaObject<RetType> {
  fn metaObject(self , rsthis: & QQuickView) -> RetType;
}

  // proto:  const QMetaObject * QQuickView::metaObject();
impl<'a> /*trait*/ QQuickView_metaObject<()> for () {
  fn metaObject(self , rsthis: & QQuickView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickView10metaObjectEv()};
     unsafe {_ZNK10QQuickView10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQuickView::QQuickView(QWindow * parent);
impl<'a> /*trait*/ QQuickView_new for (&'a QWindow) {
  fn new(self) -> QQuickView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickViewC2EP7QWindow()};
    let ctysz: c_int = unsafe{QQuickView_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QQuickViewC2EP7QWindow(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQuickView{qbase: QQuickWindow::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSize QQuickView::initialSize();
impl /*struct*/ QQuickView {
  pub fn initialSize<RetType, T: QQuickView_initialSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.initialSize(self);
    // return 1;
  }
}

pub trait QQuickView_initialSize<RetType> {
  fn initialSize(self , rsthis: & QQuickView) -> RetType;
}

  // proto:  QSize QQuickView::initialSize();
impl<'a> /*trait*/ QQuickView_initialSize<QSize> for () {
  fn initialSize(self , rsthis: & QQuickView) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickView11initialSizeEv()};
    let mut ret = unsafe {_ZNK10QQuickView11initialSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QUrl QQuickView::source();
impl /*struct*/ QQuickView {
  pub fn source<RetType, T: QQuickView_source<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.source(self);
    // return 1;
  }
}

pub trait QQuickView_source<RetType> {
  fn source(self , rsthis: & QQuickView) -> RetType;
}

  // proto:  QUrl QQuickView::source();
impl<'a> /*trait*/ QQuickView_source<QUrl> for () {
  fn source(self , rsthis: & QQuickView) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickView6sourceEv()};
    let mut ret = unsafe {_ZNK10QQuickView6sourceEv(rsthis.qclsinst)};
    let mut ret1 = QUrl::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QQuickItem * QQuickView::rootObject();
impl /*struct*/ QQuickView {
  pub fn rootObject<RetType, T: QQuickView_rootObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rootObject(self);
    // return 1;
  }
}

pub trait QQuickView_rootObject<RetType> {
  fn rootObject(self , rsthis: & QQuickView) -> RetType;
}

  // proto:  QQuickItem * QQuickView::rootObject();
impl<'a> /*trait*/ QQuickView_rootObject<QQuickItem> for () {
  fn rootObject(self , rsthis: & QQuickView) -> QQuickItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickView10rootObjectEv()};
    let mut ret = unsafe {_ZNK10QQuickView10rootObjectEv(rsthis.qclsinst)};
    let mut ret1 = QQuickItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickView::QQuickView(QQmlEngine * engine, QWindow * parent);
impl<'a> /*trait*/ QQuickView_new for (&'a QQmlEngine, &'a QWindow) {
  fn new(self) -> QQuickView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QQuickViewC2EP10QQmlEngineP7QWindow()};
    let ctysz: c_int = unsafe{QQuickView_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN10QQuickViewC2EP10QQmlEngineP7QWindow(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQuickView{qbase: QQuickWindow::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QQmlContext * QQuickView::rootContext();
impl /*struct*/ QQuickView {
  pub fn rootContext<RetType, T: QQuickView_rootContext<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rootContext(self);
    // return 1;
  }
}

pub trait QQuickView_rootContext<RetType> {
  fn rootContext(self , rsthis: & QQuickView) -> RetType;
}

  // proto:  QQmlContext * QQuickView::rootContext();
impl<'a> /*trait*/ QQuickView_rootContext<QQmlContext> for () {
  fn rootContext(self , rsthis: & QQuickView) -> QQmlContext {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QQuickView11rootContextEv()};
    let mut ret = unsafe {_ZNK10QQuickView11rootContextEv(rsthis.qclsinst)};
    let mut ret1 = QQmlContext::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QQuickView_statusChanged
pub struct QQuickView_statusChanged_signal{poi:u64}
impl /* struct */ QQuickView {
  pub fn statusChanged(&self) -> QQuickView_statusChanged_signal {
     return QQuickView_statusChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickView_statusChanged_signal {
  pub fn connect<T: QQuickView_statusChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickView_statusChanged_signal_connect {
  fn connect(self, sigthis: QQuickView_statusChanged_signal);
}

// <= body block end

