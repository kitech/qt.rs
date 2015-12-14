// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qscreen::QScreen;
use super::qobject::QObject;
use super::qsurfaceformat::QSurfaceFormat;
use super::qbytearray::QByteArray;
use super::qsurface::QSurface;
use super::qopenglversionprofile::QOpenGLVersionProfile;
use super::qvariant::QVariant;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QOpenGLContext::isValid();
  fn _ZNK14QOpenGLContext7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLContext::setScreen(QScreen * screen);
  fn _ZN14QOpenGLContext9setScreenEP7QScreen(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOpenGLContext::NewQOpenGLContext(QObject * parent);
  fn _ZN14QOpenGLContextC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QOpenGLFunctions * QOpenGLContext::functions();
  fn _ZNK14QOpenGLContext9functionsEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLContext::FreeQOpenGLContext();
  fn _ZN14QOpenGLContextD0Ev(qthis: *mut c_void) ;
  // proto:  void QOpenGLContext::setFormat(const QSurfaceFormat & format);
  fn _ZN14QOpenGLContext9setFormatERK14QSurfaceFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QOpenGLContext::metaObject();
  fn _ZNK14QOpenGLContext10metaObjectEv(qthis: *mut c_void) ;
  // proto:  bool QOpenGLContext::hasExtension(const QByteArray & extension);
  fn _ZNK14QOpenGLContext12hasExtensionERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QSet<QByteArray> QOpenGLContext::extensions();
  fn _ZNK14QOpenGLContext10extensionsEv(qthis: *mut c_void) ;
  // proto:  QSurface * QOpenGLContext::surface();
  fn _ZNK14QOpenGLContext7surfaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAbstractOpenGLFunctions * QOpenGLContext::versionFunctions(const QOpenGLVersionProfile & versionProfile);
  fn _ZNK14QOpenGLContext16versionFunctionsERK21QOpenGLVersionProfile(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOpenGLContext::setShareContext(QOpenGLContext * shareContext);
  fn _ZN14QOpenGLContext15setShareContextEPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static bool QOpenGLContext::areSharing(QOpenGLContext * first, QOpenGLContext * second);
  fn _ZN14QOpenGLContext10areSharingEPS_S0_(arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto:  QScreen * QOpenGLContext::screen();
  fn _ZNK14QOpenGLContext6screenEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QVariant QOpenGLContext::nativeHandle();
  fn _ZNK14QOpenGLContext12nativeHandleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QOpenGLContext::aboutToBeDestroyed();
  fn _ZN14QOpenGLContext18aboutToBeDestroyedEv(qthis: *mut c_void) ;
  // proto:  bool QOpenGLContext::isOpenGLES();
  fn _ZNK14QOpenGLContext10isOpenGLESEv(qthis: *mut c_void) -> int8_t;
  // proto:  QPlatformOpenGLContext * QOpenGLContext::handle();
  fn _ZNK14QOpenGLContext6handleEv(qthis: *mut c_void) ;
  // proto: static QOpenGLContext * QOpenGLContext::globalShareContext();
  fn _ZN14QOpenGLContext18globalShareContextEv() ;
  // proto:  bool QOpenGLContext::makeCurrent(QSurface * surface);
  fn _ZN14QOpenGLContext11makeCurrentEP8QSurface(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QPlatformOpenGLContext * QOpenGLContext::shareHandle();
  fn _ZNK14QOpenGLContext11shareHandleEv(qthis: *mut c_void) ;
  // proto:  bool QOpenGLContext::create();
  fn _ZN14QOpenGLContext6createEv(qthis: *mut c_void) -> int8_t;
  // proto:  QOpenGLContext * QOpenGLContext::shareContext();
  fn _ZNK14QOpenGLContext12shareContextEv(qthis: *mut c_void) ;
  // proto: static QOpenGLContext * QOpenGLContext::currentContext();
  fn _ZN14QOpenGLContext14currentContextEv() ;
  // proto:  QOpenGLContext::GLuint QOpenGLContext::defaultFramebufferObject();
  fn _ZNK14QOpenGLContext24defaultFramebufferObjectEv(qthis: *mut c_void) ;
  // proto: static bool QOpenGLContext::supportsThreadedOpenGL();
  fn _ZN14QOpenGLContext22supportsThreadedOpenGLEv() -> int8_t;
  // proto:  void QOpenGLContext::doneCurrent();
  fn _ZN14QOpenGLContext11doneCurrentEv(qthis: *mut c_void) ;
  // proto:  QOpenGLContextGroup * QOpenGLContext::shareGroup();
  fn _ZNK14QOpenGLContext10shareGroupEv(qthis: *mut c_void) ;
  // proto:  QSurfaceFormat QOpenGLContext::format();
  fn _ZNK14QOpenGLContext6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static void * QOpenGLContext::openGLModuleHandle();
  fn _ZN14QOpenGLContext18openGLModuleHandleEv() -> *mut uint8_t;
  // proto:  void QOpenGLContext::setNativeHandle(const QVariant & handle);
  fn _ZN14QOpenGLContext15setNativeHandleERK8QVariant(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QFunctionPointer QOpenGLContext::getProcAddress(const QByteArray & procName);
  fn _ZNK14QOpenGLContext14getProcAddressERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOpenGLContext::swapBuffers(QSurface * surface);
  fn _ZN14QOpenGLContext11swapBuffersEP8QSurface(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QOpenGLContext)=1
pub struct QOpenGLContext {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLContext {
  pub fn isValid<T: QOpenGLContext_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QOpenGLContext_isValid {
  fn isValid(self, rsthis: &mut QOpenGLContext) -> i8;
}

// proto:  bool QOpenGLContext::isValid();
impl<'a> /*trait*/ QOpenGLContext_isValid for () {
  fn isValid(self, rsthis: &mut QOpenGLContext) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext7isValidEv()};
    let mut ret = unsafe {_ZNK14QOpenGLContext7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn setScreen<T: QOpenGLContext_setScreen>(&mut self, value: T)  {
     value.setScreen(self);
    // return 1;
  }
}

pub trait QOpenGLContext_setScreen {
  fn setScreen(self, rsthis: &mut QOpenGLContext) ;
}

// proto:  void QOpenGLContext::setScreen(QScreen * screen);
impl<'a> /*trait*/ QOpenGLContext_setScreen for (&'a mut QScreen) {
  fn setScreen(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext9setScreenEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLContext9setScreenEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn NewQOpenGLContext<T: QOpenGLContext_NewQOpenGLContext>(value: T) -> QOpenGLContext {
    let rsthis = value.NewQOpenGLContext();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLContext_NewQOpenGLContext {
  fn NewQOpenGLContext(self) -> QOpenGLContext;
}

// proto: void QOpenGLContext::NewQOpenGLContext(QObject * parent);
impl<'a> /*trait*/ QOpenGLContext_NewQOpenGLContext for (&'a mut QObject) {
  fn NewQOpenGLContext(self) -> QOpenGLContext {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContextC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QOpenGLContextC1EP7QObject(qthis, arg0)};
    let rsthis = QOpenGLContext{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn functions<T: QOpenGLContext_functions>(&mut self, value: T)  {
     value.functions(self);
    // return 1;
  }
}

pub trait QOpenGLContext_functions {
  fn functions(self, rsthis: &mut QOpenGLContext) ;
}

// proto:  QOpenGLFunctions * QOpenGLContext::functions();
impl<'a> /*trait*/ QOpenGLContext_functions for () {
  fn functions(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext9functionsEv()};
     unsafe {_ZNK14QOpenGLContext9functionsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn FreeQOpenGLContext<T: QOpenGLContext_FreeQOpenGLContext>(&mut self, value: T)  {
     value.FreeQOpenGLContext(self);
    // return 1;
  }
}

pub trait QOpenGLContext_FreeQOpenGLContext {
  fn FreeQOpenGLContext(self, rsthis: &mut QOpenGLContext) ;
}

// proto:  void QOpenGLContext::FreeQOpenGLContext();
impl<'a> /*trait*/ QOpenGLContext_FreeQOpenGLContext for () {
  fn FreeQOpenGLContext(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContextD0Ev()};
     unsafe {_ZN14QOpenGLContextD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn setFormat<T: QOpenGLContext_setFormat>(&mut self, value: T)  {
     value.setFormat(self);
    // return 1;
  }
}

pub trait QOpenGLContext_setFormat {
  fn setFormat(self, rsthis: &mut QOpenGLContext) ;
}

// proto:  void QOpenGLContext::setFormat(const QSurfaceFormat & format);
impl<'a> /*trait*/ QOpenGLContext_setFormat for (&'a  QSurfaceFormat) {
  fn setFormat(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext9setFormatERK14QSurfaceFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLContext9setFormatERK14QSurfaceFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn metaObject<T: QOpenGLContext_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLContext_metaObject {
  fn metaObject(self, rsthis: &mut QOpenGLContext) ;
}

// proto:  const QMetaObject * QOpenGLContext::metaObject();
impl<'a> /*trait*/ QOpenGLContext_metaObject for () {
  fn metaObject(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext10metaObjectEv()};
     unsafe {_ZNK14QOpenGLContext10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn hasExtension<T: QOpenGLContext_hasExtension>(&mut self, value: T) -> i8 {
    return value.hasExtension(self);
    // return 1;
  }
}

pub trait QOpenGLContext_hasExtension {
  fn hasExtension(self, rsthis: &mut QOpenGLContext) -> i8;
}

// proto:  bool QOpenGLContext::hasExtension(const QByteArray & extension);
impl<'a> /*trait*/ QOpenGLContext_hasExtension for (&'a  QByteArray) {
  fn hasExtension(self, rsthis: &mut QOpenGLContext) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext12hasExtensionERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QOpenGLContext12hasExtensionERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn extensions<T: QOpenGLContext_extensions>(&mut self, value: T)  {
     value.extensions(self);
    // return 1;
  }
}

pub trait QOpenGLContext_extensions {
  fn extensions(self, rsthis: &mut QOpenGLContext) ;
}

// proto:  QSet<QByteArray> QOpenGLContext::extensions();
impl<'a> /*trait*/ QOpenGLContext_extensions for () {
  fn extensions(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext10extensionsEv()};
     unsafe {_ZNK14QOpenGLContext10extensionsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn surface<T: QOpenGLContext_surface>(&mut self, value: T) -> QSurface {
    return value.surface(self);
    // return 1;
  }
}

pub trait QOpenGLContext_surface {
  fn surface(self, rsthis: &mut QOpenGLContext) -> QSurface;
}

// proto:  QSurface * QOpenGLContext::surface();
impl<'a> /*trait*/ QOpenGLContext_surface for () {
  fn surface(self, rsthis: &mut QOpenGLContext) -> QSurface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext7surfaceEv()};
    let mut ret = unsafe {_ZNK14QOpenGLContext7surfaceEv(rsthis.qclsinst)};
    let mut ret1 = QSurface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn versionFunctions<T: QOpenGLContext_versionFunctions>(&mut self, value: T)  {
     value.versionFunctions(self);
    // return 1;
  }
}

pub trait QOpenGLContext_versionFunctions {
  fn versionFunctions(self, rsthis: &mut QOpenGLContext) ;
}

// proto:  QAbstractOpenGLFunctions * QOpenGLContext::versionFunctions(const QOpenGLVersionProfile & versionProfile);
impl<'a> /*trait*/ QOpenGLContext_versionFunctions for (&'a  QOpenGLVersionProfile) {
  fn versionFunctions(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext16versionFunctionsERK21QOpenGLVersionProfile()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK14QOpenGLContext16versionFunctionsERK21QOpenGLVersionProfile(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn setShareContext<T: QOpenGLContext_setShareContext>(&mut self, value: T)  {
     value.setShareContext(self);
    // return 1;
  }
}

pub trait QOpenGLContext_setShareContext {
  fn setShareContext(self, rsthis: &mut QOpenGLContext) ;
}

// proto:  void QOpenGLContext::setShareContext(QOpenGLContext * shareContext);
impl<'a> /*trait*/ QOpenGLContext_setShareContext for (&'a mut QOpenGLContext) {
  fn setShareContext(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext15setShareContextEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLContext15setShareContextEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn areSharing<T: QOpenGLContext_areSharing>(&mut self, value: T) -> i8 {
    return value.areSharing(self);
    // return 1;
  }
}

pub trait QOpenGLContext_areSharing {
  fn areSharing(self, rsthis: &mut QOpenGLContext) -> i8;
}

// proto: static bool QOpenGLContext::areSharing(QOpenGLContext * first, QOpenGLContext * second);
impl<'a> /*trait*/ QOpenGLContext_areSharing for (&'a mut QOpenGLContext, &'a mut QOpenGLContext) {
  fn areSharing(self, rsthis: &mut QOpenGLContext) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext10areSharingEPS_S0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QOpenGLContext10areSharingEPS_S0_(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn screen<T: QOpenGLContext_screen>(&mut self, value: T) -> QScreen {
    return value.screen(self);
    // return 1;
  }
}

pub trait QOpenGLContext_screen {
  fn screen(self, rsthis: &mut QOpenGLContext) -> QScreen;
}

// proto:  QScreen * QOpenGLContext::screen();
impl<'a> /*trait*/ QOpenGLContext_screen for () {
  fn screen(self, rsthis: &mut QOpenGLContext) -> QScreen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext6screenEv()};
    let mut ret = unsafe {_ZNK14QOpenGLContext6screenEv(rsthis.qclsinst)};
    let mut ret1 = QScreen{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn nativeHandle<T: QOpenGLContext_nativeHandle>(&mut self, value: T) -> QVariant {
    return value.nativeHandle(self);
    // return 1;
  }
}

pub trait QOpenGLContext_nativeHandle {
  fn nativeHandle(self, rsthis: &mut QOpenGLContext) -> QVariant;
}

// proto:  QVariant QOpenGLContext::nativeHandle();
impl<'a> /*trait*/ QOpenGLContext_nativeHandle for () {
  fn nativeHandle(self, rsthis: &mut QOpenGLContext) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext12nativeHandleEv()};
    let mut ret = unsafe {_ZNK14QOpenGLContext12nativeHandleEv(rsthis.qclsinst)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn aboutToBeDestroyed<T: QOpenGLContext_aboutToBeDestroyed>(&mut self, value: T)  {
     value.aboutToBeDestroyed(self);
    // return 1;
  }
}

pub trait QOpenGLContext_aboutToBeDestroyed {
  fn aboutToBeDestroyed(self, rsthis: &mut QOpenGLContext) ;
}

// proto:  void QOpenGLContext::aboutToBeDestroyed();
impl<'a> /*trait*/ QOpenGLContext_aboutToBeDestroyed for () {
  fn aboutToBeDestroyed(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext18aboutToBeDestroyedEv()};
     unsafe {_ZN14QOpenGLContext18aboutToBeDestroyedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn isOpenGLES<T: QOpenGLContext_isOpenGLES>(&mut self, value: T) -> i8 {
    return value.isOpenGLES(self);
    // return 1;
  }
}

pub trait QOpenGLContext_isOpenGLES {
  fn isOpenGLES(self, rsthis: &mut QOpenGLContext) -> i8;
}

// proto:  bool QOpenGLContext::isOpenGLES();
impl<'a> /*trait*/ QOpenGLContext_isOpenGLES for () {
  fn isOpenGLES(self, rsthis: &mut QOpenGLContext) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext10isOpenGLESEv()};
    let mut ret = unsafe {_ZNK14QOpenGLContext10isOpenGLESEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn handle<T: QOpenGLContext_handle>(&mut self, value: T)  {
     value.handle(self);
    // return 1;
  }
}

pub trait QOpenGLContext_handle {
  fn handle(self, rsthis: &mut QOpenGLContext) ;
}

// proto:  QPlatformOpenGLContext * QOpenGLContext::handle();
impl<'a> /*trait*/ QOpenGLContext_handle for () {
  fn handle(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext6handleEv()};
     unsafe {_ZNK14QOpenGLContext6handleEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn globalShareContext<T: QOpenGLContext_globalShareContext>(&mut self, value: T)  {
     value.globalShareContext(self);
    // return 1;
  }
}

pub trait QOpenGLContext_globalShareContext {
  fn globalShareContext(self, rsthis: &mut QOpenGLContext) ;
}

// proto: static QOpenGLContext * QOpenGLContext::globalShareContext();
impl<'a> /*trait*/ QOpenGLContext_globalShareContext for () {
  fn globalShareContext(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext18globalShareContextEv()};
     unsafe {_ZN14QOpenGLContext18globalShareContextEv()};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn makeCurrent<T: QOpenGLContext_makeCurrent>(&mut self, value: T) -> i8 {
    return value.makeCurrent(self);
    // return 1;
  }
}

pub trait QOpenGLContext_makeCurrent {
  fn makeCurrent(self, rsthis: &mut QOpenGLContext) -> i8;
}

// proto:  bool QOpenGLContext::makeCurrent(QSurface * surface);
impl<'a> /*trait*/ QOpenGLContext_makeCurrent for (&'a mut QSurface) {
  fn makeCurrent(self, rsthis: &mut QOpenGLContext) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext11makeCurrentEP8QSurface()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QOpenGLContext11makeCurrentEP8QSurface(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn shareHandle<T: QOpenGLContext_shareHandle>(&mut self, value: T)  {
     value.shareHandle(self);
    // return 1;
  }
}

pub trait QOpenGLContext_shareHandle {
  fn shareHandle(self, rsthis: &mut QOpenGLContext) ;
}

// proto:  QPlatformOpenGLContext * QOpenGLContext::shareHandle();
impl<'a> /*trait*/ QOpenGLContext_shareHandle for () {
  fn shareHandle(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext11shareHandleEv()};
     unsafe {_ZNK14QOpenGLContext11shareHandleEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn create<T: QOpenGLContext_create>(&mut self, value: T) -> i8 {
    return value.create(self);
    // return 1;
  }
}

pub trait QOpenGLContext_create {
  fn create(self, rsthis: &mut QOpenGLContext) -> i8;
}

// proto:  bool QOpenGLContext::create();
impl<'a> /*trait*/ QOpenGLContext_create for () {
  fn create(self, rsthis: &mut QOpenGLContext) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext6createEv()};
    let mut ret = unsafe {_ZN14QOpenGLContext6createEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn shareContext<T: QOpenGLContext_shareContext>(&mut self, value: T)  {
     value.shareContext(self);
    // return 1;
  }
}

pub trait QOpenGLContext_shareContext {
  fn shareContext(self, rsthis: &mut QOpenGLContext) ;
}

// proto:  QOpenGLContext * QOpenGLContext::shareContext();
impl<'a> /*trait*/ QOpenGLContext_shareContext for () {
  fn shareContext(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext12shareContextEv()};
     unsafe {_ZNK14QOpenGLContext12shareContextEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn currentContext<T: QOpenGLContext_currentContext>(&mut self, value: T)  {
     value.currentContext(self);
    // return 1;
  }
}

pub trait QOpenGLContext_currentContext {
  fn currentContext(self, rsthis: &mut QOpenGLContext) ;
}

// proto: static QOpenGLContext * QOpenGLContext::currentContext();
impl<'a> /*trait*/ QOpenGLContext_currentContext for () {
  fn currentContext(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext14currentContextEv()};
     unsafe {_ZN14QOpenGLContext14currentContextEv()};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn defaultFramebufferObject<T: QOpenGLContext_defaultFramebufferObject>(&mut self, value: T)  {
     value.defaultFramebufferObject(self);
    // return 1;
  }
}

pub trait QOpenGLContext_defaultFramebufferObject {
  fn defaultFramebufferObject(self, rsthis: &mut QOpenGLContext) ;
}

// proto:  QOpenGLContext::GLuint QOpenGLContext::defaultFramebufferObject();
impl<'a> /*trait*/ QOpenGLContext_defaultFramebufferObject for () {
  fn defaultFramebufferObject(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext24defaultFramebufferObjectEv()};
     unsafe {_ZNK14QOpenGLContext24defaultFramebufferObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn supportsThreadedOpenGL<T: QOpenGLContext_supportsThreadedOpenGL>(&mut self, value: T) -> i8 {
    return value.supportsThreadedOpenGL(self);
    // return 1;
  }
}

pub trait QOpenGLContext_supportsThreadedOpenGL {
  fn supportsThreadedOpenGL(self, rsthis: &mut QOpenGLContext) -> i8;
}

// proto: static bool QOpenGLContext::supportsThreadedOpenGL();
impl<'a> /*trait*/ QOpenGLContext_supportsThreadedOpenGL for () {
  fn supportsThreadedOpenGL(self, rsthis: &mut QOpenGLContext) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext22supportsThreadedOpenGLEv()};
    let mut ret = unsafe {_ZN14QOpenGLContext22supportsThreadedOpenGLEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn doneCurrent<T: QOpenGLContext_doneCurrent>(&mut self, value: T)  {
     value.doneCurrent(self);
    // return 1;
  }
}

pub trait QOpenGLContext_doneCurrent {
  fn doneCurrent(self, rsthis: &mut QOpenGLContext) ;
}

// proto:  void QOpenGLContext::doneCurrent();
impl<'a> /*trait*/ QOpenGLContext_doneCurrent for () {
  fn doneCurrent(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext11doneCurrentEv()};
     unsafe {_ZN14QOpenGLContext11doneCurrentEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn shareGroup<T: QOpenGLContext_shareGroup>(&mut self, value: T)  {
     value.shareGroup(self);
    // return 1;
  }
}

pub trait QOpenGLContext_shareGroup {
  fn shareGroup(self, rsthis: &mut QOpenGLContext) ;
}

// proto:  QOpenGLContextGroup * QOpenGLContext::shareGroup();
impl<'a> /*trait*/ QOpenGLContext_shareGroup for () {
  fn shareGroup(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext10shareGroupEv()};
     unsafe {_ZNK14QOpenGLContext10shareGroupEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn format<T: QOpenGLContext_format>(&mut self, value: T) -> QSurfaceFormat {
    return value.format(self);
    // return 1;
  }
}

pub trait QOpenGLContext_format {
  fn format(self, rsthis: &mut QOpenGLContext) -> QSurfaceFormat;
}

// proto:  QSurfaceFormat QOpenGLContext::format();
impl<'a> /*trait*/ QOpenGLContext_format for () {
  fn format(self, rsthis: &mut QOpenGLContext) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext6formatEv()};
    let mut ret = unsafe {_ZNK14QOpenGLContext6formatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn openGLModuleHandle<T: QOpenGLContext_openGLModuleHandle>(&mut self, value: T) {
    return value.openGLModuleHandle(self);
    // return 1;
  }
}

pub trait QOpenGLContext_openGLModuleHandle {
  fn openGLModuleHandle(self, rsthis: &mut QOpenGLContext);
}

// proto: static void * QOpenGLContext::openGLModuleHandle();
impl<'a> /*trait*/ QOpenGLContext_openGLModuleHandle for () {
  fn openGLModuleHandle(self, rsthis: &mut QOpenGLContext) {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext18openGLModuleHandleEv()};
    let mut ret = unsafe {_ZN14QOpenGLContext18openGLModuleHandleEv()};
      // return ret as &mut u8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn setNativeHandle<T: QOpenGLContext_setNativeHandle>(&mut self, value: T)  {
     value.setNativeHandle(self);
    // return 1;
  }
}

pub trait QOpenGLContext_setNativeHandle {
  fn setNativeHandle(self, rsthis: &mut QOpenGLContext) ;
}

// proto:  void QOpenGLContext::setNativeHandle(const QVariant & handle);
impl<'a> /*trait*/ QOpenGLContext_setNativeHandle for (&'a  QVariant) {
  fn setNativeHandle(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext15setNativeHandleERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLContext15setNativeHandleERK8QVariant(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn getProcAddress<T: QOpenGLContext_getProcAddress>(&mut self, value: T)  {
     value.getProcAddress(self);
    // return 1;
  }
}

pub trait QOpenGLContext_getProcAddress {
  fn getProcAddress(self, rsthis: &mut QOpenGLContext) ;
}

// proto:  QFunctionPointer QOpenGLContext::getProcAddress(const QByteArray & procName);
impl<'a> /*trait*/ QOpenGLContext_getProcAddress for (&'a  QByteArray) {
  fn getProcAddress(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext14getProcAddressERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK14QOpenGLContext14getProcAddressERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn swapBuffers<T: QOpenGLContext_swapBuffers>(&mut self, value: T)  {
     value.swapBuffers(self);
    // return 1;
  }
}

pub trait QOpenGLContext_swapBuffers {
  fn swapBuffers(self, rsthis: &mut QOpenGLContext) ;
}

// proto:  void QOpenGLContext::swapBuffers(QSurface * surface);
impl<'a> /*trait*/ QOpenGLContext_swapBuffers for (&'a mut QSurface) {
  fn swapBuffers(self, rsthis: &mut QOpenGLContext)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext11swapBuffersEP8QSurface()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLContext11swapBuffersEP8QSurface(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

