// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qsgabstractrenderer.h
// dst-file: /src/quick/qsgabstractrenderer.rs
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
use super::super::gui::qmatrix4x4::QMatrix4x4; // 771
use super::qsgnode::QSGRootNode; // 773
use super::super::core::qrect::QRect; // 771
use super::super::gui::qcolor::QColor; // 771
use super::super::core::qsize::QSize; // 771
use super::super::core::qrect::QRectF; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSGAbstractRenderer_Class_Size() -> c_int;
  // proto:  void QSGAbstractRenderer::setProjectionMatrix(const QMatrix4x4 & matrix);
  fn _ZN19QSGAbstractRenderer19setProjectionMatrixERK10QMatrix4x4(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSGAbstractRenderer::setRootNode(QSGRootNode * node);
  fn _ZN19QSGAbstractRenderer11setRootNodeEP11QSGRootNode(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSGAbstractRenderer::~QSGAbstractRenderer();
  fn _ZN19QSGAbstractRendererD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QSGRootNode * QSGAbstractRenderer::rootNode();
  fn _ZNK19QSGAbstractRenderer8rootNodeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGAbstractRenderer::setDeviceRect(const QRect & rect);
  fn _ZN19QSGAbstractRenderer13setDeviceRectERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QColor QSGAbstractRenderer::clearColor();
  fn _ZNK19QSGAbstractRenderer10clearColorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRect QSGAbstractRenderer::viewportRect();
  fn _ZNK19QSGAbstractRenderer12viewportRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGAbstractRenderer::setDeviceRect(const QSize & size);
  fn _ZN19QSGAbstractRenderer13setDeviceRectERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSGAbstractRenderer::setProjectionMatrixToRect(const QRectF & rect);
  fn _ZN19QSGAbstractRenderer25setProjectionMatrixToRectERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSGAbstractRenderer::setViewportRect(const QSize & size);
  fn _ZN19QSGAbstractRenderer15setViewportRectERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QRect QSGAbstractRenderer::deviceRect();
  fn _ZNK19QSGAbstractRenderer10deviceRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGAbstractRenderer::renderScene(GLuint fboId);
  fn _ZN19QSGAbstractRenderer11renderSceneEj(qthis: u64 /* *mut c_void*/, arg0: c_uint);
  // proto:  const QMetaObject * QSGAbstractRenderer::metaObject();
  fn _ZNK19QSGAbstractRenderer10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSGAbstractRenderer::QSGAbstractRenderer(QObject * parent);
  fn _ZN19QSGAbstractRendererC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSGAbstractRenderer::setClearColor(const QColor & color);
  fn _ZN19QSGAbstractRenderer13setClearColorERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QMatrix4x4 QSGAbstractRenderer::projectionMatrix();
  fn _ZNK19QSGAbstractRenderer16projectionMatrixEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGAbstractRenderer::setViewportRect(const QRect & rect);
  fn _ZN19QSGAbstractRenderer15setViewportRectERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QSGAbstractRenderer_SlotProxy_connect__ZN19QSGAbstractRenderer17sceneGraphChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QSGAbstractRenderer)=1
#[derive(Default)]
pub struct QSGAbstractRenderer {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _sceneGraphChanged: QSGAbstractRenderer_sceneGraphChanged_signal,
}

impl /*struct*/ QSGAbstractRenderer {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGAbstractRenderer {
    return QSGAbstractRenderer{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSGAbstractRenderer {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QSGAbstractRenderer {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QSGAbstractRenderer::setProjectionMatrix(const QMatrix4x4 & matrix);
impl /*struct*/ QSGAbstractRenderer {
  pub fn setProjectionMatrix<RetType, T: QSGAbstractRenderer_setProjectionMatrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setProjectionMatrix(self);
    // return 1;
  }
}

pub trait QSGAbstractRenderer_setProjectionMatrix<RetType> {
  fn setProjectionMatrix(self , rsthis: & QSGAbstractRenderer) -> RetType;
}

  // proto:  void QSGAbstractRenderer::setProjectionMatrix(const QMatrix4x4 & matrix);
impl<'a> /*trait*/ QSGAbstractRenderer_setProjectionMatrix<()> for (&'a QMatrix4x4) {
  fn setProjectionMatrix(self , rsthis: & QSGAbstractRenderer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QSGAbstractRenderer19setProjectionMatrixERK10QMatrix4x4()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QSGAbstractRenderer19setProjectionMatrixERK10QMatrix4x4(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSGAbstractRenderer::setRootNode(QSGRootNode * node);
impl /*struct*/ QSGAbstractRenderer {
  pub fn setRootNode<RetType, T: QSGAbstractRenderer_setRootNode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRootNode(self);
    // return 1;
  }
}

pub trait QSGAbstractRenderer_setRootNode<RetType> {
  fn setRootNode(self , rsthis: & QSGAbstractRenderer) -> RetType;
}

  // proto:  void QSGAbstractRenderer::setRootNode(QSGRootNode * node);
impl<'a> /*trait*/ QSGAbstractRenderer_setRootNode<()> for (&'a QSGRootNode) {
  fn setRootNode(self , rsthis: & QSGAbstractRenderer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QSGAbstractRenderer11setRootNodeEP11QSGRootNode()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QSGAbstractRenderer11setRootNodeEP11QSGRootNode(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSGAbstractRenderer::~QSGAbstractRenderer();
impl /*struct*/ QSGAbstractRenderer {
  pub fn free<RetType, T: QSGAbstractRenderer_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSGAbstractRenderer_free<RetType> {
  fn free(self , rsthis: & QSGAbstractRenderer) -> RetType;
}

  // proto:  void QSGAbstractRenderer::~QSGAbstractRenderer();
impl<'a> /*trait*/ QSGAbstractRenderer_free<()> for () {
  fn free(self , rsthis: & QSGAbstractRenderer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QSGAbstractRendererD2Ev()};
     unsafe {_ZN19QSGAbstractRendererD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSGRootNode * QSGAbstractRenderer::rootNode();
impl /*struct*/ QSGAbstractRenderer {
  pub fn rootNode<RetType, T: QSGAbstractRenderer_rootNode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rootNode(self);
    // return 1;
  }
}

pub trait QSGAbstractRenderer_rootNode<RetType> {
  fn rootNode(self , rsthis: & QSGAbstractRenderer) -> RetType;
}

  // proto:  QSGRootNode * QSGAbstractRenderer::rootNode();
impl<'a> /*trait*/ QSGAbstractRenderer_rootNode<QSGRootNode> for () {
  fn rootNode(self , rsthis: & QSGAbstractRenderer) -> QSGRootNode {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QSGAbstractRenderer8rootNodeEv()};
    let mut ret = unsafe {_ZNK19QSGAbstractRenderer8rootNodeEv(rsthis.qclsinst)};
    let mut ret1 = QSGRootNode::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGAbstractRenderer::setDeviceRect(const QRect & rect);
impl /*struct*/ QSGAbstractRenderer {
  pub fn setDeviceRect<RetType, T: QSGAbstractRenderer_setDeviceRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDeviceRect(self);
    // return 1;
  }
}

pub trait QSGAbstractRenderer_setDeviceRect<RetType> {
  fn setDeviceRect(self , rsthis: & QSGAbstractRenderer) -> RetType;
}

  // proto:  void QSGAbstractRenderer::setDeviceRect(const QRect & rect);
impl<'a> /*trait*/ QSGAbstractRenderer_setDeviceRect<()> for (&'a QRect) {
  fn setDeviceRect(self , rsthis: & QSGAbstractRenderer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QSGAbstractRenderer13setDeviceRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QSGAbstractRenderer13setDeviceRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QColor QSGAbstractRenderer::clearColor();
impl /*struct*/ QSGAbstractRenderer {
  pub fn clearColor<RetType, T: QSGAbstractRenderer_clearColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearColor(self);
    // return 1;
  }
}

pub trait QSGAbstractRenderer_clearColor<RetType> {
  fn clearColor(self , rsthis: & QSGAbstractRenderer) -> RetType;
}

  // proto:  QColor QSGAbstractRenderer::clearColor();
impl<'a> /*trait*/ QSGAbstractRenderer_clearColor<QColor> for () {
  fn clearColor(self , rsthis: & QSGAbstractRenderer) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QSGAbstractRenderer10clearColorEv()};
    let mut ret = unsafe {_ZNK19QSGAbstractRenderer10clearColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QSGAbstractRenderer::viewportRect();
impl /*struct*/ QSGAbstractRenderer {
  pub fn viewportRect<RetType, T: QSGAbstractRenderer_viewportRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.viewportRect(self);
    // return 1;
  }
}

pub trait QSGAbstractRenderer_viewportRect<RetType> {
  fn viewportRect(self , rsthis: & QSGAbstractRenderer) -> RetType;
}

  // proto:  QRect QSGAbstractRenderer::viewportRect();
impl<'a> /*trait*/ QSGAbstractRenderer_viewportRect<QRect> for () {
  fn viewportRect(self , rsthis: & QSGAbstractRenderer) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QSGAbstractRenderer12viewportRectEv()};
    let mut ret = unsafe {_ZNK19QSGAbstractRenderer12viewportRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGAbstractRenderer::setDeviceRect(const QSize & size);
impl<'a> /*trait*/ QSGAbstractRenderer_setDeviceRect<()> for (&'a QSize) {
  fn setDeviceRect(self , rsthis: & QSGAbstractRenderer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QSGAbstractRenderer13setDeviceRectERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QSGAbstractRenderer13setDeviceRectERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSGAbstractRenderer::setProjectionMatrixToRect(const QRectF & rect);
impl /*struct*/ QSGAbstractRenderer {
  pub fn setProjectionMatrixToRect<RetType, T: QSGAbstractRenderer_setProjectionMatrixToRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setProjectionMatrixToRect(self);
    // return 1;
  }
}

pub trait QSGAbstractRenderer_setProjectionMatrixToRect<RetType> {
  fn setProjectionMatrixToRect(self , rsthis: & QSGAbstractRenderer) -> RetType;
}

  // proto:  void QSGAbstractRenderer::setProjectionMatrixToRect(const QRectF & rect);
impl<'a> /*trait*/ QSGAbstractRenderer_setProjectionMatrixToRect<()> for (&'a QRectF) {
  fn setProjectionMatrixToRect(self , rsthis: & QSGAbstractRenderer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QSGAbstractRenderer25setProjectionMatrixToRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QSGAbstractRenderer25setProjectionMatrixToRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSGAbstractRenderer::setViewportRect(const QSize & size);
impl /*struct*/ QSGAbstractRenderer {
  pub fn setViewportRect<RetType, T: QSGAbstractRenderer_setViewportRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setViewportRect(self);
    // return 1;
  }
}

pub trait QSGAbstractRenderer_setViewportRect<RetType> {
  fn setViewportRect(self , rsthis: & QSGAbstractRenderer) -> RetType;
}

  // proto:  void QSGAbstractRenderer::setViewportRect(const QSize & size);
impl<'a> /*trait*/ QSGAbstractRenderer_setViewportRect<()> for (&'a QSize) {
  fn setViewportRect(self , rsthis: & QSGAbstractRenderer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QSGAbstractRenderer15setViewportRectERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QSGAbstractRenderer15setViewportRectERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QSGAbstractRenderer::deviceRect();
impl /*struct*/ QSGAbstractRenderer {
  pub fn deviceRect<RetType, T: QSGAbstractRenderer_deviceRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.deviceRect(self);
    // return 1;
  }
}

pub trait QSGAbstractRenderer_deviceRect<RetType> {
  fn deviceRect(self , rsthis: & QSGAbstractRenderer) -> RetType;
}

  // proto:  QRect QSGAbstractRenderer::deviceRect();
impl<'a> /*trait*/ QSGAbstractRenderer_deviceRect<QRect> for () {
  fn deviceRect(self , rsthis: & QSGAbstractRenderer) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QSGAbstractRenderer10deviceRectEv()};
    let mut ret = unsafe {_ZNK19QSGAbstractRenderer10deviceRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGAbstractRenderer::renderScene(GLuint fboId);
impl /*struct*/ QSGAbstractRenderer {
  pub fn renderScene<RetType, T: QSGAbstractRenderer_renderScene<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.renderScene(self);
    // return 1;
  }
}

pub trait QSGAbstractRenderer_renderScene<RetType> {
  fn renderScene(self , rsthis: & QSGAbstractRenderer) -> RetType;
}

  // proto:  void QSGAbstractRenderer::renderScene(GLuint fboId);
impl<'a> /*trait*/ QSGAbstractRenderer_renderScene<()> for (u32) {
  fn renderScene(self , rsthis: & QSGAbstractRenderer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QSGAbstractRenderer11renderSceneEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN19QSGAbstractRenderer11renderSceneEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QSGAbstractRenderer::metaObject();
impl /*struct*/ QSGAbstractRenderer {
  pub fn metaObject<RetType, T: QSGAbstractRenderer_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSGAbstractRenderer_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSGAbstractRenderer) -> RetType;
}

  // proto:  const QMetaObject * QSGAbstractRenderer::metaObject();
impl<'a> /*trait*/ QSGAbstractRenderer_metaObject<()> for () {
  fn metaObject(self , rsthis: & QSGAbstractRenderer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QSGAbstractRenderer10metaObjectEv()};
     unsafe {_ZNK19QSGAbstractRenderer10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSGAbstractRenderer::QSGAbstractRenderer(QObject * parent);
impl /*struct*/ QSGAbstractRenderer {
  pub fn new<T: QSGAbstractRenderer_new>(value: T) -> QSGAbstractRenderer {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSGAbstractRenderer_new {
  fn new(self) -> QSGAbstractRenderer;
}

  // proto:  void QSGAbstractRenderer::QSGAbstractRenderer(QObject * parent);
impl<'a> /*trait*/ QSGAbstractRenderer_new for (&'a QObject) {
  fn new(self) -> QSGAbstractRenderer {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QSGAbstractRendererC2EP7QObject()};
    let ctysz: c_int = unsafe{QSGAbstractRenderer_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QSGAbstractRendererC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSGAbstractRenderer{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSGAbstractRenderer::setClearColor(const QColor & color);
impl /*struct*/ QSGAbstractRenderer {
  pub fn setClearColor<RetType, T: QSGAbstractRenderer_setClearColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setClearColor(self);
    // return 1;
  }
}

pub trait QSGAbstractRenderer_setClearColor<RetType> {
  fn setClearColor(self , rsthis: & QSGAbstractRenderer) -> RetType;
}

  // proto:  void QSGAbstractRenderer::setClearColor(const QColor & color);
impl<'a> /*trait*/ QSGAbstractRenderer_setClearColor<()> for (&'a QColor) {
  fn setClearColor(self , rsthis: & QSGAbstractRenderer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QSGAbstractRenderer13setClearColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QSGAbstractRenderer13setClearColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QMatrix4x4 QSGAbstractRenderer::projectionMatrix();
impl /*struct*/ QSGAbstractRenderer {
  pub fn projectionMatrix<RetType, T: QSGAbstractRenderer_projectionMatrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.projectionMatrix(self);
    // return 1;
  }
}

pub trait QSGAbstractRenderer_projectionMatrix<RetType> {
  fn projectionMatrix(self , rsthis: & QSGAbstractRenderer) -> RetType;
}

  // proto:  QMatrix4x4 QSGAbstractRenderer::projectionMatrix();
impl<'a> /*trait*/ QSGAbstractRenderer_projectionMatrix<QMatrix4x4> for () {
  fn projectionMatrix(self , rsthis: & QSGAbstractRenderer) -> QMatrix4x4 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QSGAbstractRenderer16projectionMatrixEv()};
    let mut ret = unsafe {_ZNK19QSGAbstractRenderer16projectionMatrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix4x4::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGAbstractRenderer::setViewportRect(const QRect & rect);
impl<'a> /*trait*/ QSGAbstractRenderer_setViewportRect<()> for (&'a QRect) {
  fn setViewportRect(self , rsthis: & QSGAbstractRenderer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QSGAbstractRenderer15setViewportRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QSGAbstractRenderer15setViewportRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QSGAbstractRenderer_sceneGraphChanged
pub struct QSGAbstractRenderer_sceneGraphChanged_signal{poi:u64}
impl /* struct */ QSGAbstractRenderer {
  pub fn sceneGraphChanged(&self) -> QSGAbstractRenderer_sceneGraphChanged_signal {
     return QSGAbstractRenderer_sceneGraphChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QSGAbstractRenderer_sceneGraphChanged_signal {
  pub fn connect<T: QSGAbstractRenderer_sceneGraphChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QSGAbstractRenderer_sceneGraphChanged_signal_connect {
  fn connect(self, sigthis: QSGAbstractRenderer_sceneGraphChanged_signal);
}

// sceneGraphChanged()
extern fn QSGAbstractRenderer_sceneGraphChanged_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QSGAbstractRenderer_sceneGraphChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QSGAbstractRenderer_sceneGraphChanged_signal_connect for fn() {
  fn connect(self, sigthis: QSGAbstractRenderer_sceneGraphChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSGAbstractRenderer_sceneGraphChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QSGAbstractRenderer_SlotProxy_connect__ZN19QSGAbstractRenderer17sceneGraphChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QSGAbstractRenderer_sceneGraphChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QSGAbstractRenderer_sceneGraphChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSGAbstractRenderer_sceneGraphChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QSGAbstractRenderer_SlotProxy_connect__ZN19QSGAbstractRenderer17sceneGraphChangedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

