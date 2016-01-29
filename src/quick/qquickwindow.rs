// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qquickwindow.h
// dst-file: /src/quick/qquickwindow.rs
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
use super::super::gui::qwindow::QWindow; // 771
use std::ops::Deref;
use super::super::core::qsize::QSize; // 771
use super::super::core::qrunnable::QRunnable; // 771
use super::super::core::qobject::QObject; // 771
use super::qquickitem::QQuickItem; // 773
use super::super::gui::qimage::QImage; // 771
use super::super::gui::qcolor::QColor; // 771
use super::qquickrendercontrol::QQuickRenderControl; // 773
use super::super::gui::qopenglframebufferobject::QOpenGLFramebufferObject; // 771
use super::super::gui::qaccessible::QAccessibleInterface; // 771
use super::super::core::qcoreevent::QEvent; // 771
use super::qsgtexture::QSGTexture; // 773
use super::super::qml::qqmlincubator::QQmlIncubationController; // 771
use super::super::gui::qopenglcontext::QOpenGLContext; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQuickWindow_Class_Size() -> c_int;
  // proto:  void QQuickWindow::QQuickWindow(QWindow * parent);
  fn _ZN12QQuickWindowC2EP7QWindow(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickWindow::setPersistentSceneGraph(bool persistent);
  fn _ZN12QQuickWindow23setPersistentSceneGraphEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QQuickWindow::isPersistentOpenGLContext();
  fn _ZNK12QQuickWindow25isPersistentOpenGLContextEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qreal QQuickWindow::effectiveDevicePixelRatio();
  fn _ZNK12QQuickWindow25effectiveDevicePixelRatioEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QQuickWindow::resetOpenGLState();
  fn _ZN12QQuickWindow16resetOpenGLStateEv(qthis: u64 /* *mut c_void*/);
  // proto:  uint QQuickWindow::renderTargetId();
  fn _ZNK12QQuickWindow14renderTargetIdEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  void QQuickWindow::setRenderTarget(uint fboId, const QSize & size);
  fn _ZN12QQuickWindow15setRenderTargetEjRK5QSize(qthis: u64 /* *mut c_void*/, arg0: c_uint, arg1: *mut c_void);
  // proto:  const QMetaObject * QQuickWindow::metaObject();
  fn _ZNK12QQuickWindow10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQuickWindow::QQuickWindow(const QQuickWindow & );
  fn _ZN12QQuickWindowC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QObject * QQuickWindow::focusObject();
  fn _ZNK12QQuickWindow11focusObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QQuickItem * QQuickWindow::activeFocusItem();
  fn _ZNK12QQuickWindow15activeFocusItemEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQuickWindow::update();
  fn _ZN12QQuickWindow6updateEv(qthis: u64 /* *mut c_void*/);
  // proto:  QOpenGLContext * QQuickWindow::openglContext();
  fn _ZNK12QQuickWindow13openglContextEv(qthis: u64 /* *mut c_void*/);
  // proto:  QColor QQuickWindow::color();
  fn _ZNK12QQuickWindow5colorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQuickWindow::releaseResources();
  fn _ZN12QQuickWindow16releaseResourcesEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQuickWindow::QQuickWindow(QQuickRenderControl * renderControl);
  fn _ZN12QQuickWindowC2EP19QQuickRenderControl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QQuickItem * QQuickWindow::mouseGrabberItem();
  fn _ZNK12QQuickWindow16mouseGrabberItemEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QQuickWindow::isPersistentSceneGraph();
  fn _ZNK12QQuickWindow22isPersistentSceneGraphEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQuickWindow::setRenderTarget(QOpenGLFramebufferObject * fbo);
  fn _ZN12QQuickWindow15setRenderTargetEP24QOpenGLFramebufferObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickWindow::~QQuickWindow();
  fn _ZN12QQuickWindowD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QQuickWindow::renderTargetSize();
  fn _ZNK12QQuickWindow16renderTargetSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQuickWindow::setColor(const QColor & color);
  fn _ZN12QQuickWindow8setColorERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static void QQuickWindow::setDefaultAlphaBuffer(bool useAlpha);
  fn _ZN12QQuickWindow21setDefaultAlphaBufferEb(arg0: c_char);
  // proto:  QAccessibleInterface * QQuickWindow::accessibleRoot();
  fn _ZNK12QQuickWindow14accessibleRootEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QQuickWindow::isSceneGraphInitialized();
  fn _ZNK12QQuickWindow23isSceneGraphInitializedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQuickWindow::setClearBeforeRendering(bool enabled);
  fn _ZN12QQuickWindow23setClearBeforeRenderingEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QImage QQuickWindow::grabWindow();
  fn _ZN12QQuickWindow10grabWindowEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQuickWindow::setPersistentOpenGLContext(bool persistent);
  fn _ZN12QQuickWindow26setPersistentOpenGLContextEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QQuickWindow::sendEvent(QQuickItem * , QEvent * );
  fn _ZN12QQuickWindow9sendEventEP10QQuickItemP6QEvent(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  QOpenGLFramebufferObject * QQuickWindow::renderTarget();
  fn _ZNK12QQuickWindow12renderTargetEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSGTexture * QQuickWindow::createTextureFromImage(const QImage & image);
  fn _ZNK12QQuickWindow22createTextureFromImageERK6QImage(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QQmlIncubationController * QQuickWindow::incubationController();
  fn _ZNK12QQuickWindow20incubationControllerEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QQuickWindow::clearBeforeRendering();
  fn _ZNK12QQuickWindow20clearBeforeRenderingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QQuickItem * QQuickWindow::contentItem();
  fn _ZNK12QQuickWindow11contentItemEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static bool QQuickWindow::hasDefaultAlphaBuffer();
  fn _ZN12QQuickWindow21hasDefaultAlphaBufferEv() -> c_char;
  fn QQuickWindow_SlotProxy_connect__ZN12QQuickWindow12frameSwappedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickWindow_SlotProxy_connect__ZN12QQuickWindow14afterRenderingEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickWindow_SlotProxy_connect__ZN12QQuickWindow15beforeRenderingEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickWindow_SlotProxy_connect__ZN12QQuickWindow21sceneGraphInvalidatedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickWindow_SlotProxy_connect__ZN12QQuickWindow22activeFocusItemChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickWindow_SlotProxy_connect__ZN12QQuickWindow18afterSynchronizingEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickWindow_SlotProxy_connect__ZN12QQuickWindow19beforeSynchronizingEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickWindow_SlotProxy_connect__ZN12QQuickWindow20openglContextCreatedEP14QOpenGLContext(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickWindow_SlotProxy_connect__ZN12QQuickWindow12colorChangedERK6QColor(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickWindow_SlotProxy_connect__ZN12QQuickWindow21sceneGraphInitializedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickWindow_SlotProxy_connect__ZN12QQuickWindow21sceneGraphAboutToStopEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickWindow_SlotProxy_connect__ZN12QQuickWindow14afterAnimatingEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QQuickWindow)=1
#[derive(Default)]
pub struct QQuickWindow {
  qbase: QWindow,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _colorChanged: QQuickWindow_colorChanged_signal,
  pub _sceneGraphInvalidated: QQuickWindow_sceneGraphInvalidated_signal,
  pub _beforeSynchronizing: QQuickWindow_beforeSynchronizing_signal,
  pub _sceneGraphAboutToStop: QQuickWindow_sceneGraphAboutToStop_signal,
  pub _afterAnimating: QQuickWindow_afterAnimating_signal,
  pub _sceneGraphError: QQuickWindow_sceneGraphError_signal,
  pub _sceneGraphInitialized: QQuickWindow_sceneGraphInitialized_signal,
  pub _activeFocusItemChanged: QQuickWindow_activeFocusItemChanged_signal,
  pub _afterRendering: QQuickWindow_afterRendering_signal,
  pub _closing: QQuickWindow_closing_signal,
  pub _afterSynchronizing: QQuickWindow_afterSynchronizing_signal,
  pub _beforeRendering: QQuickWindow_beforeRendering_signal,
  pub _frameSwapped: QQuickWindow_frameSwapped_signal,
  pub _openglContextCreated: QQuickWindow_openglContextCreated_signal,
}

impl /*struct*/ QQuickWindow {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQuickWindow {
    return QQuickWindow{qbase: QWindow::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQuickWindow {
  type Target = QWindow;

  fn deref(&self) -> &QWindow {
    return & self.qbase;
  }
}
impl AsRef<QWindow> for QQuickWindow {
  fn as_ref(& self) -> & QWindow {
    return & self.qbase;
  }
}
  // proto:  void QQuickWindow::QQuickWindow(QWindow * parent);
impl /*struct*/ QQuickWindow {
  pub fn new<T: QQuickWindow_new>(value: T) -> QQuickWindow {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQuickWindow_new {
  fn new(self) -> QQuickWindow;
}

  // proto:  void QQuickWindow::QQuickWindow(QWindow * parent);
impl<'a> /*trait*/ QQuickWindow_new for (&'a QWindow) {
  fn new(self) -> QQuickWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQuickWindowC2EP7QWindow()};
    let ctysz: c_int = unsafe{QQuickWindow_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QQuickWindowC2EP7QWindow(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQuickWindow{qbase: QWindow::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQuickWindow::setPersistentSceneGraph(bool persistent);
impl /*struct*/ QQuickWindow {
  pub fn setPersistentSceneGraph<RetType, T: QQuickWindow_setPersistentSceneGraph<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPersistentSceneGraph(self);
    // return 1;
  }
}

pub trait QQuickWindow_setPersistentSceneGraph<RetType> {
  fn setPersistentSceneGraph(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  void QQuickWindow::setPersistentSceneGraph(bool persistent);
impl<'a> /*trait*/ QQuickWindow_setPersistentSceneGraph<()> for (i8) {
  fn setPersistentSceneGraph(self , rsthis: & QQuickWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQuickWindow23setPersistentSceneGraphEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QQuickWindow23setPersistentSceneGraphEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QQuickWindow::isPersistentOpenGLContext();
impl /*struct*/ QQuickWindow {
  pub fn isPersistentOpenGLContext<RetType, T: QQuickWindow_isPersistentOpenGLContext<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isPersistentOpenGLContext(self);
    // return 1;
  }
}

pub trait QQuickWindow_isPersistentOpenGLContext<RetType> {
  fn isPersistentOpenGLContext(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  bool QQuickWindow::isPersistentOpenGLContext();
impl<'a> /*trait*/ QQuickWindow_isPersistentOpenGLContext<i8> for () {
  fn isPersistentOpenGLContext(self , rsthis: & QQuickWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQuickWindow25isPersistentOpenGLContextEv()};
    let mut ret = unsafe {_ZNK12QQuickWindow25isPersistentOpenGLContextEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QQuickWindow::effectiveDevicePixelRatio();
impl /*struct*/ QQuickWindow {
  pub fn effectiveDevicePixelRatio<RetType, T: QQuickWindow_effectiveDevicePixelRatio<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.effectiveDevicePixelRatio(self);
    // return 1;
  }
}

pub trait QQuickWindow_effectiveDevicePixelRatio<RetType> {
  fn effectiveDevicePixelRatio(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  qreal QQuickWindow::effectiveDevicePixelRatio();
impl<'a> /*trait*/ QQuickWindow_effectiveDevicePixelRatio<f64> for () {
  fn effectiveDevicePixelRatio(self , rsthis: & QQuickWindow) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQuickWindow25effectiveDevicePixelRatioEv()};
    let mut ret = unsafe {_ZNK12QQuickWindow25effectiveDevicePixelRatioEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QQuickWindow::resetOpenGLState();
impl /*struct*/ QQuickWindow {
  pub fn resetOpenGLState<RetType, T: QQuickWindow_resetOpenGLState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetOpenGLState(self);
    // return 1;
  }
}

pub trait QQuickWindow_resetOpenGLState<RetType> {
  fn resetOpenGLState(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  void QQuickWindow::resetOpenGLState();
impl<'a> /*trait*/ QQuickWindow_resetOpenGLState<()> for () {
  fn resetOpenGLState(self , rsthis: & QQuickWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQuickWindow16resetOpenGLStateEv()};
     unsafe {_ZN12QQuickWindow16resetOpenGLStateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  uint QQuickWindow::renderTargetId();
impl /*struct*/ QQuickWindow {
  pub fn renderTargetId<RetType, T: QQuickWindow_renderTargetId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.renderTargetId(self);
    // return 1;
  }
}

pub trait QQuickWindow_renderTargetId<RetType> {
  fn renderTargetId(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  uint QQuickWindow::renderTargetId();
impl<'a> /*trait*/ QQuickWindow_renderTargetId<u32> for () {
  fn renderTargetId(self , rsthis: & QQuickWindow) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQuickWindow14renderTargetIdEv()};
    let mut ret = unsafe {_ZNK12QQuickWindow14renderTargetIdEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  void QQuickWindow::setRenderTarget(uint fboId, const QSize & size);
impl /*struct*/ QQuickWindow {
  pub fn setRenderTarget<RetType, T: QQuickWindow_setRenderTarget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRenderTarget(self);
    // return 1;
  }
}

pub trait QQuickWindow_setRenderTarget<RetType> {
  fn setRenderTarget(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  void QQuickWindow::setRenderTarget(uint fboId, const QSize & size);
impl<'a> /*trait*/ QQuickWindow_setRenderTarget<()> for (u32, &'a QSize) {
  fn setRenderTarget(self , rsthis: & QQuickWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQuickWindow15setRenderTargetEjRK5QSize()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN12QQuickWindow15setRenderTargetEjRK5QSize(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QQuickWindow::metaObject();
impl /*struct*/ QQuickWindow {
  pub fn metaObject<RetType, T: QQuickWindow_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QQuickWindow_metaObject<RetType> {
  fn metaObject(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  const QMetaObject * QQuickWindow::metaObject();
impl<'a> /*trait*/ QQuickWindow_metaObject<()> for () {
  fn metaObject(self , rsthis: & QQuickWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQuickWindow10metaObjectEv()};
     unsafe {_ZNK12QQuickWindow10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQuickWindow::QQuickWindow(const QQuickWindow & );
impl<'a> /*trait*/ QQuickWindow_new for (&'a QQuickWindow) {
  fn new(self) -> QQuickWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQuickWindowC2ERKS_()};
    let ctysz: c_int = unsafe{QQuickWindow_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QQuickWindowC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQuickWindow{qbase: QWindow::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QObject * QQuickWindow::focusObject();
impl /*struct*/ QQuickWindow {
  pub fn focusObject<RetType, T: QQuickWindow_focusObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.focusObject(self);
    // return 1;
  }
}

pub trait QQuickWindow_focusObject<RetType> {
  fn focusObject(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  QObject * QQuickWindow::focusObject();
impl<'a> /*trait*/ QQuickWindow_focusObject<QObject> for () {
  fn focusObject(self , rsthis: & QQuickWindow) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQuickWindow11focusObjectEv()};
    let mut ret = unsafe {_ZNK12QQuickWindow11focusObjectEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QQuickItem * QQuickWindow::activeFocusItem();
impl /*struct*/ QQuickWindow {
  pub fn activeFocusItem<RetType, T: QQuickWindow_activeFocusItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activeFocusItem(self);
    // return 1;
  }
}

pub trait QQuickWindow_activeFocusItem<RetType> {
  fn activeFocusItem(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  QQuickItem * QQuickWindow::activeFocusItem();
impl<'a> /*trait*/ QQuickWindow_activeFocusItem<QQuickItem> for () {
  fn activeFocusItem(self , rsthis: & QQuickWindow) -> QQuickItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQuickWindow15activeFocusItemEv()};
    let mut ret = unsafe {_ZNK12QQuickWindow15activeFocusItemEv(rsthis.qclsinst)};
    let mut ret1 = QQuickItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickWindow::update();
impl /*struct*/ QQuickWindow {
  pub fn update<RetType, T: QQuickWindow_update<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.update(self);
    // return 1;
  }
}

pub trait QQuickWindow_update<RetType> {
  fn update(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  void QQuickWindow::update();
impl<'a> /*trait*/ QQuickWindow_update<()> for () {
  fn update(self , rsthis: & QQuickWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQuickWindow6updateEv()};
     unsafe {_ZN12QQuickWindow6updateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QOpenGLContext * QQuickWindow::openglContext();
impl /*struct*/ QQuickWindow {
  pub fn openglContext<RetType, T: QQuickWindow_openglContext<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.openglContext(self);
    // return 1;
  }
}

pub trait QQuickWindow_openglContext<RetType> {
  fn openglContext(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  QOpenGLContext * QQuickWindow::openglContext();
impl<'a> /*trait*/ QQuickWindow_openglContext<()> for () {
  fn openglContext(self , rsthis: & QQuickWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQuickWindow13openglContextEv()};
     unsafe {_ZNK12QQuickWindow13openglContextEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QColor QQuickWindow::color();
impl /*struct*/ QQuickWindow {
  pub fn color<RetType, T: QQuickWindow_color<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.color(self);
    // return 1;
  }
}

pub trait QQuickWindow_color<RetType> {
  fn color(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  QColor QQuickWindow::color();
impl<'a> /*trait*/ QQuickWindow_color<QColor> for () {
  fn color(self , rsthis: & QQuickWindow) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQuickWindow5colorEv()};
    let mut ret = unsafe {_ZNK12QQuickWindow5colorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickWindow::releaseResources();
impl /*struct*/ QQuickWindow {
  pub fn releaseResources<RetType, T: QQuickWindow_releaseResources<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.releaseResources(self);
    // return 1;
  }
}

pub trait QQuickWindow_releaseResources<RetType> {
  fn releaseResources(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  void QQuickWindow::releaseResources();
impl<'a> /*trait*/ QQuickWindow_releaseResources<()> for () {
  fn releaseResources(self , rsthis: & QQuickWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQuickWindow16releaseResourcesEv()};
     unsafe {_ZN12QQuickWindow16releaseResourcesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQuickWindow::QQuickWindow(QQuickRenderControl * renderControl);
impl<'a> /*trait*/ QQuickWindow_new for (&'a QQuickRenderControl) {
  fn new(self) -> QQuickWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQuickWindowC2EP19QQuickRenderControl()};
    let ctysz: c_int = unsafe{QQuickWindow_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QQuickWindowC2EP19QQuickRenderControl(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQuickWindow{qbase: QWindow::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QQuickItem * QQuickWindow::mouseGrabberItem();
impl /*struct*/ QQuickWindow {
  pub fn mouseGrabberItem<RetType, T: QQuickWindow_mouseGrabberItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mouseGrabberItem(self);
    // return 1;
  }
}

pub trait QQuickWindow_mouseGrabberItem<RetType> {
  fn mouseGrabberItem(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  QQuickItem * QQuickWindow::mouseGrabberItem();
impl<'a> /*trait*/ QQuickWindow_mouseGrabberItem<QQuickItem> for () {
  fn mouseGrabberItem(self , rsthis: & QQuickWindow) -> QQuickItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQuickWindow16mouseGrabberItemEv()};
    let mut ret = unsafe {_ZNK12QQuickWindow16mouseGrabberItemEv(rsthis.qclsinst)};
    let mut ret1 = QQuickItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QQuickWindow::isPersistentSceneGraph();
impl /*struct*/ QQuickWindow {
  pub fn isPersistentSceneGraph<RetType, T: QQuickWindow_isPersistentSceneGraph<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isPersistentSceneGraph(self);
    // return 1;
  }
}

pub trait QQuickWindow_isPersistentSceneGraph<RetType> {
  fn isPersistentSceneGraph(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  bool QQuickWindow::isPersistentSceneGraph();
impl<'a> /*trait*/ QQuickWindow_isPersistentSceneGraph<i8> for () {
  fn isPersistentSceneGraph(self , rsthis: & QQuickWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQuickWindow22isPersistentSceneGraphEv()};
    let mut ret = unsafe {_ZNK12QQuickWindow22isPersistentSceneGraphEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQuickWindow::setRenderTarget(QOpenGLFramebufferObject * fbo);
impl<'a> /*trait*/ QQuickWindow_setRenderTarget<()> for (&'a QOpenGLFramebufferObject) {
  fn setRenderTarget(self , rsthis: & QQuickWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQuickWindow15setRenderTargetEP24QOpenGLFramebufferObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QQuickWindow15setRenderTargetEP24QOpenGLFramebufferObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickWindow::~QQuickWindow();
impl /*struct*/ QQuickWindow {
  pub fn free<RetType, T: QQuickWindow_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQuickWindow_free<RetType> {
  fn free(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  void QQuickWindow::~QQuickWindow();
impl<'a> /*trait*/ QQuickWindow_free<()> for () {
  fn free(self , rsthis: & QQuickWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQuickWindowD2Ev()};
     unsafe {_ZN12QQuickWindowD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QQuickWindow::renderTargetSize();
impl /*struct*/ QQuickWindow {
  pub fn renderTargetSize<RetType, T: QQuickWindow_renderTargetSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.renderTargetSize(self);
    // return 1;
  }
}

pub trait QQuickWindow_renderTargetSize<RetType> {
  fn renderTargetSize(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  QSize QQuickWindow::renderTargetSize();
impl<'a> /*trait*/ QQuickWindow_renderTargetSize<QSize> for () {
  fn renderTargetSize(self , rsthis: & QQuickWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQuickWindow16renderTargetSizeEv()};
    let mut ret = unsafe {_ZNK12QQuickWindow16renderTargetSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickWindow::setColor(const QColor & color);
impl /*struct*/ QQuickWindow {
  pub fn setColor<RetType, T: QQuickWindow_setColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColor(self);
    // return 1;
  }
}

pub trait QQuickWindow_setColor<RetType> {
  fn setColor(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  void QQuickWindow::setColor(const QColor & color);
impl<'a> /*trait*/ QQuickWindow_setColor<()> for (&'a QColor) {
  fn setColor(self , rsthis: & QQuickWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQuickWindow8setColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QQuickWindow8setColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static void QQuickWindow::setDefaultAlphaBuffer(bool useAlpha);
impl /*struct*/ QQuickWindow {
  pub fn setDefaultAlphaBuffer_s<RetType, T: QQuickWindow_setDefaultAlphaBuffer_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDefaultAlphaBuffer_s();
    // return 1;
  }
}

pub trait QQuickWindow_setDefaultAlphaBuffer_s<RetType> {
  fn setDefaultAlphaBuffer_s(self ) -> RetType;
}

  // proto: static void QQuickWindow::setDefaultAlphaBuffer(bool useAlpha);
impl<'a> /*trait*/ QQuickWindow_setDefaultAlphaBuffer_s<()> for (i8) {
  fn setDefaultAlphaBuffer_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQuickWindow21setDefaultAlphaBufferEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QQuickWindow21setDefaultAlphaBufferEb(arg0)};
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QQuickWindow::accessibleRoot();
impl /*struct*/ QQuickWindow {
  pub fn accessibleRoot<RetType, T: QQuickWindow_accessibleRoot<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.accessibleRoot(self);
    // return 1;
  }
}

pub trait QQuickWindow_accessibleRoot<RetType> {
  fn accessibleRoot(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  QAccessibleInterface * QQuickWindow::accessibleRoot();
impl<'a> /*trait*/ QQuickWindow_accessibleRoot<QAccessibleInterface> for () {
  fn accessibleRoot(self , rsthis: & QQuickWindow) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQuickWindow14accessibleRootEv()};
    let mut ret = unsafe {_ZNK12QQuickWindow14accessibleRootEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QQuickWindow::isSceneGraphInitialized();
impl /*struct*/ QQuickWindow {
  pub fn isSceneGraphInitialized<RetType, T: QQuickWindow_isSceneGraphInitialized<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSceneGraphInitialized(self);
    // return 1;
  }
}

pub trait QQuickWindow_isSceneGraphInitialized<RetType> {
  fn isSceneGraphInitialized(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  bool QQuickWindow::isSceneGraphInitialized();
impl<'a> /*trait*/ QQuickWindow_isSceneGraphInitialized<i8> for () {
  fn isSceneGraphInitialized(self , rsthis: & QQuickWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQuickWindow23isSceneGraphInitializedEv()};
    let mut ret = unsafe {_ZNK12QQuickWindow23isSceneGraphInitializedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQuickWindow::setClearBeforeRendering(bool enabled);
impl /*struct*/ QQuickWindow {
  pub fn setClearBeforeRendering<RetType, T: QQuickWindow_setClearBeforeRendering<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setClearBeforeRendering(self);
    // return 1;
  }
}

pub trait QQuickWindow_setClearBeforeRendering<RetType> {
  fn setClearBeforeRendering(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  void QQuickWindow::setClearBeforeRendering(bool enabled);
impl<'a> /*trait*/ QQuickWindow_setClearBeforeRendering<()> for (i8) {
  fn setClearBeforeRendering(self , rsthis: & QQuickWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQuickWindow23setClearBeforeRenderingEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QQuickWindow23setClearBeforeRenderingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QImage QQuickWindow::grabWindow();
impl /*struct*/ QQuickWindow {
  pub fn grabWindow<RetType, T: QQuickWindow_grabWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.grabWindow(self);
    // return 1;
  }
}

pub trait QQuickWindow_grabWindow<RetType> {
  fn grabWindow(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  QImage QQuickWindow::grabWindow();
impl<'a> /*trait*/ QQuickWindow_grabWindow<QImage> for () {
  fn grabWindow(self , rsthis: & QQuickWindow) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQuickWindow10grabWindowEv()};
    let mut ret = unsafe {_ZN12QQuickWindow10grabWindowEv(rsthis.qclsinst)};
    let mut ret1 = QImage::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickWindow::setPersistentOpenGLContext(bool persistent);
impl /*struct*/ QQuickWindow {
  pub fn setPersistentOpenGLContext<RetType, T: QQuickWindow_setPersistentOpenGLContext<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPersistentOpenGLContext(self);
    // return 1;
  }
}

pub trait QQuickWindow_setPersistentOpenGLContext<RetType> {
  fn setPersistentOpenGLContext(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  void QQuickWindow::setPersistentOpenGLContext(bool persistent);
impl<'a> /*trait*/ QQuickWindow_setPersistentOpenGLContext<()> for (i8) {
  fn setPersistentOpenGLContext(self , rsthis: & QQuickWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQuickWindow26setPersistentOpenGLContextEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QQuickWindow26setPersistentOpenGLContextEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QQuickWindow::sendEvent(QQuickItem * , QEvent * );
impl /*struct*/ QQuickWindow {
  pub fn sendEvent<RetType, T: QQuickWindow_sendEvent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sendEvent(self);
    // return 1;
  }
}

pub trait QQuickWindow_sendEvent<RetType> {
  fn sendEvent(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  bool QQuickWindow::sendEvent(QQuickItem * , QEvent * );
impl<'a> /*trait*/ QQuickWindow_sendEvent<i8> for (&'a QQuickItem, &'a QEvent) {
  fn sendEvent(self , rsthis: & QQuickWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQuickWindow9sendEventEP10QQuickItemP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QQuickWindow9sendEventEP10QQuickItemP6QEvent(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QOpenGLFramebufferObject * QQuickWindow::renderTarget();
impl /*struct*/ QQuickWindow {
  pub fn renderTarget<RetType, T: QQuickWindow_renderTarget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.renderTarget(self);
    // return 1;
  }
}

pub trait QQuickWindow_renderTarget<RetType> {
  fn renderTarget(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  QOpenGLFramebufferObject * QQuickWindow::renderTarget();
impl<'a> /*trait*/ QQuickWindow_renderTarget<()> for () {
  fn renderTarget(self , rsthis: & QQuickWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQuickWindow12renderTargetEv()};
     unsafe {_ZNK12QQuickWindow12renderTargetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSGTexture * QQuickWindow::createTextureFromImage(const QImage & image);
impl /*struct*/ QQuickWindow {
  pub fn createTextureFromImage<RetType, T: QQuickWindow_createTextureFromImage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createTextureFromImage(self);
    // return 1;
  }
}

pub trait QQuickWindow_createTextureFromImage<RetType> {
  fn createTextureFromImage(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  QSGTexture * QQuickWindow::createTextureFromImage(const QImage & image);
impl<'a> /*trait*/ QQuickWindow_createTextureFromImage<QSGTexture> for (&'a QImage) {
  fn createTextureFromImage(self , rsthis: & QQuickWindow) -> QSGTexture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQuickWindow22createTextureFromImageERK6QImage()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QQuickWindow22createTextureFromImageERK6QImage(rsthis.qclsinst, arg0)};
    let mut ret1 = QSGTexture::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QQmlIncubationController * QQuickWindow::incubationController();
impl /*struct*/ QQuickWindow {
  pub fn incubationController<RetType, T: QQuickWindow_incubationController<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.incubationController(self);
    // return 1;
  }
}

pub trait QQuickWindow_incubationController<RetType> {
  fn incubationController(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  QQmlIncubationController * QQuickWindow::incubationController();
impl<'a> /*trait*/ QQuickWindow_incubationController<QQmlIncubationController> for () {
  fn incubationController(self , rsthis: & QQuickWindow) -> QQmlIncubationController {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQuickWindow20incubationControllerEv()};
    let mut ret = unsafe {_ZNK12QQuickWindow20incubationControllerEv(rsthis.qclsinst)};
    let mut ret1 = QQmlIncubationController::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QQuickWindow::clearBeforeRendering();
impl /*struct*/ QQuickWindow {
  pub fn clearBeforeRendering<RetType, T: QQuickWindow_clearBeforeRendering<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearBeforeRendering(self);
    // return 1;
  }
}

pub trait QQuickWindow_clearBeforeRendering<RetType> {
  fn clearBeforeRendering(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  bool QQuickWindow::clearBeforeRendering();
impl<'a> /*trait*/ QQuickWindow_clearBeforeRendering<i8> for () {
  fn clearBeforeRendering(self , rsthis: & QQuickWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQuickWindow20clearBeforeRenderingEv()};
    let mut ret = unsafe {_ZNK12QQuickWindow20clearBeforeRenderingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QQuickItem * QQuickWindow::contentItem();
impl /*struct*/ QQuickWindow {
  pub fn contentItem<RetType, T: QQuickWindow_contentItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contentItem(self);
    // return 1;
  }
}

pub trait QQuickWindow_contentItem<RetType> {
  fn contentItem(self , rsthis: & QQuickWindow) -> RetType;
}

  // proto:  QQuickItem * QQuickWindow::contentItem();
impl<'a> /*trait*/ QQuickWindow_contentItem<QQuickItem> for () {
  fn contentItem(self , rsthis: & QQuickWindow) -> QQuickItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QQuickWindow11contentItemEv()};
    let mut ret = unsafe {_ZNK12QQuickWindow11contentItemEv(rsthis.qclsinst)};
    let mut ret1 = QQuickItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QQuickWindow::hasDefaultAlphaBuffer();
impl /*struct*/ QQuickWindow {
  pub fn hasDefaultAlphaBuffer_s<RetType, T: QQuickWindow_hasDefaultAlphaBuffer_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.hasDefaultAlphaBuffer_s();
    // return 1;
  }
}

pub trait QQuickWindow_hasDefaultAlphaBuffer_s<RetType> {
  fn hasDefaultAlphaBuffer_s(self ) -> RetType;
}

  // proto: static bool QQuickWindow::hasDefaultAlphaBuffer();
impl<'a> /*trait*/ QQuickWindow_hasDefaultAlphaBuffer_s<i8> for () {
  fn hasDefaultAlphaBuffer_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QQuickWindow21hasDefaultAlphaBufferEv()};
    let mut ret = unsafe {_ZN12QQuickWindow21hasDefaultAlphaBufferEv()};
    return ret as i8;
    // return 1;
  }
}

#[derive(Default)] // for QQuickWindow_colorChanged
pub struct QQuickWindow_colorChanged_signal{poi:u64}
impl /* struct */ QQuickWindow {
  pub fn colorChanged(&self) -> QQuickWindow_colorChanged_signal {
     return QQuickWindow_colorChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickWindow_colorChanged_signal {
  pub fn connect<T: QQuickWindow_colorChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickWindow_colorChanged_signal_connect {
  fn connect(self, sigthis: QQuickWindow_colorChanged_signal);
}

#[derive(Default)] // for QQuickWindow_sceneGraphInvalidated
pub struct QQuickWindow_sceneGraphInvalidated_signal{poi:u64}
impl /* struct */ QQuickWindow {
  pub fn sceneGraphInvalidated(&self) -> QQuickWindow_sceneGraphInvalidated_signal {
     return QQuickWindow_sceneGraphInvalidated_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickWindow_sceneGraphInvalidated_signal {
  pub fn connect<T: QQuickWindow_sceneGraphInvalidated_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickWindow_sceneGraphInvalidated_signal_connect {
  fn connect(self, sigthis: QQuickWindow_sceneGraphInvalidated_signal);
}

#[derive(Default)] // for QQuickWindow_beforeSynchronizing
pub struct QQuickWindow_beforeSynchronizing_signal{poi:u64}
impl /* struct */ QQuickWindow {
  pub fn beforeSynchronizing(&self) -> QQuickWindow_beforeSynchronizing_signal {
     return QQuickWindow_beforeSynchronizing_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickWindow_beforeSynchronizing_signal {
  pub fn connect<T: QQuickWindow_beforeSynchronizing_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickWindow_beforeSynchronizing_signal_connect {
  fn connect(self, sigthis: QQuickWindow_beforeSynchronizing_signal);
}

#[derive(Default)] // for QQuickWindow_sceneGraphAboutToStop
pub struct QQuickWindow_sceneGraphAboutToStop_signal{poi:u64}
impl /* struct */ QQuickWindow {
  pub fn sceneGraphAboutToStop(&self) -> QQuickWindow_sceneGraphAboutToStop_signal {
     return QQuickWindow_sceneGraphAboutToStop_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickWindow_sceneGraphAboutToStop_signal {
  pub fn connect<T: QQuickWindow_sceneGraphAboutToStop_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickWindow_sceneGraphAboutToStop_signal_connect {
  fn connect(self, sigthis: QQuickWindow_sceneGraphAboutToStop_signal);
}

#[derive(Default)] // for QQuickWindow_afterAnimating
pub struct QQuickWindow_afterAnimating_signal{poi:u64}
impl /* struct */ QQuickWindow {
  pub fn afterAnimating(&self) -> QQuickWindow_afterAnimating_signal {
     return QQuickWindow_afterAnimating_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickWindow_afterAnimating_signal {
  pub fn connect<T: QQuickWindow_afterAnimating_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickWindow_afterAnimating_signal_connect {
  fn connect(self, sigthis: QQuickWindow_afterAnimating_signal);
}

#[derive(Default)] // for QQuickWindow_sceneGraphError
pub struct QQuickWindow_sceneGraphError_signal{poi:u64}
impl /* struct */ QQuickWindow {
  pub fn sceneGraphError(&self) -> QQuickWindow_sceneGraphError_signal {
     return QQuickWindow_sceneGraphError_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickWindow_sceneGraphError_signal {
  pub fn connect<T: QQuickWindow_sceneGraphError_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickWindow_sceneGraphError_signal_connect {
  fn connect(self, sigthis: QQuickWindow_sceneGraphError_signal);
}

#[derive(Default)] // for QQuickWindow_sceneGraphInitialized
pub struct QQuickWindow_sceneGraphInitialized_signal{poi:u64}
impl /* struct */ QQuickWindow {
  pub fn sceneGraphInitialized(&self) -> QQuickWindow_sceneGraphInitialized_signal {
     return QQuickWindow_sceneGraphInitialized_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickWindow_sceneGraphInitialized_signal {
  pub fn connect<T: QQuickWindow_sceneGraphInitialized_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickWindow_sceneGraphInitialized_signal_connect {
  fn connect(self, sigthis: QQuickWindow_sceneGraphInitialized_signal);
}

#[derive(Default)] // for QQuickWindow_activeFocusItemChanged
pub struct QQuickWindow_activeFocusItemChanged_signal{poi:u64}
impl /* struct */ QQuickWindow {
  pub fn activeFocusItemChanged(&self) -> QQuickWindow_activeFocusItemChanged_signal {
     return QQuickWindow_activeFocusItemChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickWindow_activeFocusItemChanged_signal {
  pub fn connect<T: QQuickWindow_activeFocusItemChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickWindow_activeFocusItemChanged_signal_connect {
  fn connect(self, sigthis: QQuickWindow_activeFocusItemChanged_signal);
}

#[derive(Default)] // for QQuickWindow_afterRendering
pub struct QQuickWindow_afterRendering_signal{poi:u64}
impl /* struct */ QQuickWindow {
  pub fn afterRendering(&self) -> QQuickWindow_afterRendering_signal {
     return QQuickWindow_afterRendering_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickWindow_afterRendering_signal {
  pub fn connect<T: QQuickWindow_afterRendering_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickWindow_afterRendering_signal_connect {
  fn connect(self, sigthis: QQuickWindow_afterRendering_signal);
}

#[derive(Default)] // for QQuickWindow_closing
pub struct QQuickWindow_closing_signal{poi:u64}
impl /* struct */ QQuickWindow {
  pub fn closing(&self) -> QQuickWindow_closing_signal {
     return QQuickWindow_closing_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickWindow_closing_signal {
  pub fn connect<T: QQuickWindow_closing_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickWindow_closing_signal_connect {
  fn connect(self, sigthis: QQuickWindow_closing_signal);
}

#[derive(Default)] // for QQuickWindow_afterSynchronizing
pub struct QQuickWindow_afterSynchronizing_signal{poi:u64}
impl /* struct */ QQuickWindow {
  pub fn afterSynchronizing(&self) -> QQuickWindow_afterSynchronizing_signal {
     return QQuickWindow_afterSynchronizing_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickWindow_afterSynchronizing_signal {
  pub fn connect<T: QQuickWindow_afterSynchronizing_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickWindow_afterSynchronizing_signal_connect {
  fn connect(self, sigthis: QQuickWindow_afterSynchronizing_signal);
}

#[derive(Default)] // for QQuickWindow_beforeRendering
pub struct QQuickWindow_beforeRendering_signal{poi:u64}
impl /* struct */ QQuickWindow {
  pub fn beforeRendering(&self) -> QQuickWindow_beforeRendering_signal {
     return QQuickWindow_beforeRendering_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickWindow_beforeRendering_signal {
  pub fn connect<T: QQuickWindow_beforeRendering_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickWindow_beforeRendering_signal_connect {
  fn connect(self, sigthis: QQuickWindow_beforeRendering_signal);
}

#[derive(Default)] // for QQuickWindow_frameSwapped
pub struct QQuickWindow_frameSwapped_signal{poi:u64}
impl /* struct */ QQuickWindow {
  pub fn frameSwapped(&self) -> QQuickWindow_frameSwapped_signal {
     return QQuickWindow_frameSwapped_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickWindow_frameSwapped_signal {
  pub fn connect<T: QQuickWindow_frameSwapped_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickWindow_frameSwapped_signal_connect {
  fn connect(self, sigthis: QQuickWindow_frameSwapped_signal);
}

#[derive(Default)] // for QQuickWindow_openglContextCreated
pub struct QQuickWindow_openglContextCreated_signal{poi:u64}
impl /* struct */ QQuickWindow {
  pub fn openglContextCreated(&self) -> QQuickWindow_openglContextCreated_signal {
     return QQuickWindow_openglContextCreated_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickWindow_openglContextCreated_signal {
  pub fn connect<T: QQuickWindow_openglContextCreated_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickWindow_openglContextCreated_signal_connect {
  fn connect(self, sigthis: QQuickWindow_openglContextCreated_signal);
}

// frameSwapped()
extern fn QQuickWindow_frameSwapped_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickWindow_frameSwapped_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickWindow_frameSwapped_signal_connect for fn() {
  fn connect(self, sigthis: QQuickWindow_frameSwapped_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_frameSwapped_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow12frameSwappedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickWindow_frameSwapped_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickWindow_frameSwapped_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_frameSwapped_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow12frameSwappedEv(arg0, arg1, arg2)};
  }
}
// afterRendering()
extern fn QQuickWindow_afterRendering_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickWindow_afterRendering_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickWindow_afterRendering_signal_connect for fn() {
  fn connect(self, sigthis: QQuickWindow_afterRendering_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_afterRendering_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow14afterRenderingEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickWindow_afterRendering_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickWindow_afterRendering_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_afterRendering_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow14afterRenderingEv(arg0, arg1, arg2)};
  }
}
// beforeRendering()
extern fn QQuickWindow_beforeRendering_signal_connect_cb_2(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickWindow_beforeRendering_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickWindow_beforeRendering_signal_connect for fn() {
  fn connect(self, sigthis: QQuickWindow_beforeRendering_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_beforeRendering_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow15beforeRenderingEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickWindow_beforeRendering_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickWindow_beforeRendering_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_beforeRendering_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow15beforeRenderingEv(arg0, arg1, arg2)};
  }
}
// sceneGraphInvalidated()
extern fn QQuickWindow_sceneGraphInvalidated_signal_connect_cb_3(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickWindow_sceneGraphInvalidated_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickWindow_sceneGraphInvalidated_signal_connect for fn() {
  fn connect(self, sigthis: QQuickWindow_sceneGraphInvalidated_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_sceneGraphInvalidated_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow21sceneGraphInvalidatedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickWindow_sceneGraphInvalidated_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickWindow_sceneGraphInvalidated_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_sceneGraphInvalidated_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow21sceneGraphInvalidatedEv(arg0, arg1, arg2)};
  }
}
// activeFocusItemChanged()
extern fn QQuickWindow_activeFocusItemChanged_signal_connect_cb_4(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickWindow_activeFocusItemChanged_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickWindow_activeFocusItemChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickWindow_activeFocusItemChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_activeFocusItemChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow22activeFocusItemChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickWindow_activeFocusItemChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickWindow_activeFocusItemChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_activeFocusItemChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow22activeFocusItemChangedEv(arg0, arg1, arg2)};
  }
}
// afterSynchronizing()
extern fn QQuickWindow_afterSynchronizing_signal_connect_cb_5(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickWindow_afterSynchronizing_signal_connect_cb_box_5(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickWindow_afterSynchronizing_signal_connect for fn() {
  fn connect(self, sigthis: QQuickWindow_afterSynchronizing_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_afterSynchronizing_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow18afterSynchronizingEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickWindow_afterSynchronizing_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickWindow_afterSynchronizing_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_afterSynchronizing_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow18afterSynchronizingEv(arg0, arg1, arg2)};
  }
}
// beforeSynchronizing()
extern fn QQuickWindow_beforeSynchronizing_signal_connect_cb_6(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickWindow_beforeSynchronizing_signal_connect_cb_box_6(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickWindow_beforeSynchronizing_signal_connect for fn() {
  fn connect(self, sigthis: QQuickWindow_beforeSynchronizing_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_beforeSynchronizing_signal_connect_cb_6 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow19beforeSynchronizingEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickWindow_beforeSynchronizing_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickWindow_beforeSynchronizing_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_beforeSynchronizing_signal_connect_cb_box_6 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow19beforeSynchronizingEv(arg0, arg1, arg2)};
  }
}
// openglContextCreated(class QOpenGLContext *)
extern fn QQuickWindow_openglContextCreated_signal_connect_cb_7(rsfptr:fn(QOpenGLContext), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QOpenGLContext::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QQuickWindow_openglContextCreated_signal_connect_cb_box_7(rsfptr_raw:*mut Box<Fn(QOpenGLContext)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QOpenGLContext::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QQuickWindow_openglContextCreated_signal_connect for fn(QOpenGLContext) {
  fn connect(self, sigthis: QQuickWindow_openglContextCreated_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_openglContextCreated_signal_connect_cb_7 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow20openglContextCreatedEP14QOpenGLContext(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickWindow_openglContextCreated_signal_connect for Box<Fn(QOpenGLContext)> {
  fn connect(self, sigthis: QQuickWindow_openglContextCreated_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_openglContextCreated_signal_connect_cb_box_7 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow20openglContextCreatedEP14QOpenGLContext(arg0, arg1, arg2)};
  }
}
// colorChanged(const class QColor &)
extern fn QQuickWindow_colorChanged_signal_connect_cb_8(rsfptr:fn(QColor), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QColor::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QQuickWindow_colorChanged_signal_connect_cb_box_8(rsfptr_raw:*mut Box<Fn(QColor)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QColor::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QQuickWindow_colorChanged_signal_connect for fn(QColor) {
  fn connect(self, sigthis: QQuickWindow_colorChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_colorChanged_signal_connect_cb_8 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow12colorChangedERK6QColor(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickWindow_colorChanged_signal_connect for Box<Fn(QColor)> {
  fn connect(self, sigthis: QQuickWindow_colorChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_colorChanged_signal_connect_cb_box_8 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow12colorChangedERK6QColor(arg0, arg1, arg2)};
  }
}
// sceneGraphInitialized()
extern fn QQuickWindow_sceneGraphInitialized_signal_connect_cb_9(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickWindow_sceneGraphInitialized_signal_connect_cb_box_9(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickWindow_sceneGraphInitialized_signal_connect for fn() {
  fn connect(self, sigthis: QQuickWindow_sceneGraphInitialized_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_sceneGraphInitialized_signal_connect_cb_9 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow21sceneGraphInitializedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickWindow_sceneGraphInitialized_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickWindow_sceneGraphInitialized_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_sceneGraphInitialized_signal_connect_cb_box_9 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow21sceneGraphInitializedEv(arg0, arg1, arg2)};
  }
}
// sceneGraphAboutToStop()
extern fn QQuickWindow_sceneGraphAboutToStop_signal_connect_cb_10(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickWindow_sceneGraphAboutToStop_signal_connect_cb_box_10(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickWindow_sceneGraphAboutToStop_signal_connect for fn() {
  fn connect(self, sigthis: QQuickWindow_sceneGraphAboutToStop_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_sceneGraphAboutToStop_signal_connect_cb_10 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow21sceneGraphAboutToStopEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickWindow_sceneGraphAboutToStop_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickWindow_sceneGraphAboutToStop_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_sceneGraphAboutToStop_signal_connect_cb_box_10 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow21sceneGraphAboutToStopEv(arg0, arg1, arg2)};
  }
}
// afterAnimating()
extern fn QQuickWindow_afterAnimating_signal_connect_cb_11(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickWindow_afterAnimating_signal_connect_cb_box_11(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickWindow_afterAnimating_signal_connect for fn() {
  fn connect(self, sigthis: QQuickWindow_afterAnimating_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_afterAnimating_signal_connect_cb_11 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow14afterAnimatingEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickWindow_afterAnimating_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickWindow_afterAnimating_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickWindow_afterAnimating_signal_connect_cb_box_11 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickWindow_SlotProxy_connect__ZN12QQuickWindow14afterAnimatingEv(arg0, arg1, arg2)};
  }
}
// <= body block end

