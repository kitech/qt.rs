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
use super::qopenglversionprofile::QOpenGLVersionProfile;
use super::qsurface::QSurface;
use super::qvariant::QVariant;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QOpenGLContext::isValid();
  fn _ZNK14QOpenGLContext7isValidEv() -> i32;
  // proto: void QOpenGLContext::setScreen(QScreen * screen);
  fn _ZN14QOpenGLContext9setScreenEP7QScreen(arg0: *mut c_void) -> i32;
  // proto: void QOpenGLContext::NewQOpenGLContext(QObject * parent);
  fn _ZN14QOpenGLContextC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QOpenGLFunctions * QOpenGLContext::functions();
  fn _ZNK14QOpenGLContext9functionsEv() -> i32;
  // proto: void QOpenGLContext::FreeQOpenGLContext();
  fn _ZN14QOpenGLContextD0Ev() -> i32;
  // proto: void QOpenGLContext::setFormat(const QSurfaceFormat & format);
  fn _ZN14QOpenGLContext9setFormatERK14QSurfaceFormat(arg0: *const c_void) -> i32;
  // proto: const QMetaObject * QOpenGLContext::metaObject();
  fn _ZNK14QOpenGLContext10metaObjectEv() -> i32;
  // proto: bool QOpenGLContext::hasExtension(const QByteArray & extension);
  fn _ZNK14QOpenGLContext12hasExtensionERK10QByteArray(arg0: *const c_void) -> i32;
  // proto: QSet<QByteArray> QOpenGLContext::extensions();
  fn _ZNK14QOpenGLContext10extensionsEv() -> i32;
  // proto: QSurface * QOpenGLContext::surface();
  fn _ZNK14QOpenGLContext7surfaceEv() -> i32;
  // proto: QAbstractOpenGLFunctions * QOpenGLContext::versionFunctions(const QOpenGLVersionProfile & versionProfile);
  fn _ZNK14QOpenGLContext16versionFunctionsERK21QOpenGLVersionProfile(arg0: *const c_void) -> i32;
  // proto: void QOpenGLContext::setShareContext(QOpenGLContext * shareContext);
  fn _ZN14QOpenGLContext15setShareContextEPS_(arg0: *mut c_void) -> i32;
  // proto: bool QOpenGLContext::areSharing(QOpenGLContext * first, QOpenGLContext * second);
  fn _ZN14QOpenGLContext10areSharingEPS_S0_(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  // proto: QScreen * QOpenGLContext::screen();
  fn _ZNK14QOpenGLContext6screenEv() -> i32;
  // proto: QVariant QOpenGLContext::nativeHandle();
  fn _ZNK14QOpenGLContext12nativeHandleEv() -> i32;
  // proto: void QOpenGLContext::aboutToBeDestroyed();
  fn _ZN14QOpenGLContext18aboutToBeDestroyedEv() -> i32;
  // proto: bool QOpenGLContext::isOpenGLES();
  fn _ZNK14QOpenGLContext10isOpenGLESEv() -> i32;
  // proto: QPlatformOpenGLContext * QOpenGLContext::handle();
  fn _ZNK14QOpenGLContext6handleEv() -> i32;
  // proto: QOpenGLContext * QOpenGLContext::globalShareContext();
  fn _ZN14QOpenGLContext18globalShareContextEv() -> i32;
  // proto: bool QOpenGLContext::makeCurrent(QSurface * surface);
  fn _ZN14QOpenGLContext11makeCurrentEP8QSurface(arg0: *mut c_void) -> i32;
  // proto: QPlatformOpenGLContext * QOpenGLContext::shareHandle();
  fn _ZNK14QOpenGLContext11shareHandleEv() -> i32;
  // proto: bool QOpenGLContext::create();
  fn _ZN14QOpenGLContext6createEv() -> i32;
  // proto: QOpenGLContext * QOpenGLContext::shareContext();
  fn _ZNK14QOpenGLContext12shareContextEv() -> i32;
  // proto: QOpenGLContext * QOpenGLContext::currentContext();
  fn _ZN14QOpenGLContext14currentContextEv() -> i32;
  // proto: QOpenGLContext::GLuint QOpenGLContext::defaultFramebufferObject();
  fn _ZNK14QOpenGLContext24defaultFramebufferObjectEv() -> i32;
  // proto: bool QOpenGLContext::supportsThreadedOpenGL();
  fn _ZN14QOpenGLContext22supportsThreadedOpenGLEv() -> i32;
  // proto: void QOpenGLContext::doneCurrent();
  fn _ZN14QOpenGLContext11doneCurrentEv() -> i32;
  // proto: QOpenGLContextGroup * QOpenGLContext::shareGroup();
  fn _ZNK14QOpenGLContext10shareGroupEv() -> i32;
  // proto: QSurfaceFormat QOpenGLContext::format();
  fn _ZNK14QOpenGLContext6formatEv() -> i32;
  // proto: void * QOpenGLContext::openGLModuleHandle();
  fn _ZN14QOpenGLContext18openGLModuleHandleEv() -> i32;
  // proto: void QOpenGLContext::setNativeHandle(const QVariant & handle);
  fn _ZN14QOpenGLContext15setNativeHandleERK8QVariant(arg0: *const c_void) -> i32;
  // proto: QFunctionPointer QOpenGLContext::getProcAddress(const QByteArray & procName);
  fn _ZNK14QOpenGLContext14getProcAddressERK10QByteArray(arg0: *const c_void) -> i32;
  // proto: void QOpenGLContext::swapBuffers(QSurface * surface);
  fn _ZN14QOpenGLContext11swapBuffersEP8QSurface(arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QOpenGLContext)=1
pub struct QOpenGLContext {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLContext {
  pub fn isValid<T: QOpenGLContext_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QOpenGLContext_isValid {
  fn isValid(self, this: &mut QOpenGLContext) -> i32;
}

// proto: bool QOpenGLContext::isValid();
impl<'a> /*trait*/ QOpenGLContext_isValid for () {
  fn isValid(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext7isValidEv()};
    unsafe {_ZNK14QOpenGLContext7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn setScreen<T: QOpenGLContext_setScreen>(&mut self, value: T) -> i32 {
    value.setScreen(self);
    return 1;
  }
}

pub trait QOpenGLContext_setScreen {
  fn setScreen(self, this: &mut QOpenGLContext) -> i32;
}

// proto: void QOpenGLContext::setScreen(QScreen * screen);
impl<'a> /*trait*/ QOpenGLContext_setScreen for (&'a mut QScreen) {
  fn setScreen(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext9setScreenEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QOpenGLContext9setScreenEP7QScreen(arg0)};
    return 1;
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
  pub fn functions<T: QOpenGLContext_functions>(&mut self, value: T) -> i32 {
    value.functions(self);
    return 1;
  }
}

pub trait QOpenGLContext_functions {
  fn functions(self, this: &mut QOpenGLContext) -> i32;
}

// proto: QOpenGLFunctions * QOpenGLContext::functions();
impl<'a> /*trait*/ QOpenGLContext_functions for () {
  fn functions(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext9functionsEv()};
    unsafe {_ZNK14QOpenGLContext9functionsEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn FreeQOpenGLContext<T: QOpenGLContext_FreeQOpenGLContext>(&mut self, value: T) -> i32 {
    value.FreeQOpenGLContext(self);
    return 1;
  }
}

pub trait QOpenGLContext_FreeQOpenGLContext {
  fn FreeQOpenGLContext(self, this: &mut QOpenGLContext) -> i32;
}

// proto: void QOpenGLContext::FreeQOpenGLContext();
impl<'a> /*trait*/ QOpenGLContext_FreeQOpenGLContext for () {
  fn FreeQOpenGLContext(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContextD0Ev()};
    unsafe {_ZN14QOpenGLContextD0Ev()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn setFormat<T: QOpenGLContext_setFormat>(&mut self, value: T) -> i32 {
    value.setFormat(self);
    return 1;
  }
}

pub trait QOpenGLContext_setFormat {
  fn setFormat(self, this: &mut QOpenGLContext) -> i32;
}

// proto: void QOpenGLContext::setFormat(const QSurfaceFormat & format);
impl<'a> /*trait*/ QOpenGLContext_setFormat for (&'a  QSurfaceFormat) {
  fn setFormat(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext9setFormatERK14QSurfaceFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QOpenGLContext9setFormatERK14QSurfaceFormat(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn metaObject<T: QOpenGLContext_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QOpenGLContext_metaObject {
  fn metaObject(self, this: &mut QOpenGLContext) -> i32;
}

// proto: const QMetaObject * QOpenGLContext::metaObject();
impl<'a> /*trait*/ QOpenGLContext_metaObject for () {
  fn metaObject(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext10metaObjectEv()};
    unsafe {_ZNK14QOpenGLContext10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn hasExtension<T: QOpenGLContext_hasExtension>(&mut self, value: T) -> i32 {
    value.hasExtension(self);
    return 1;
  }
}

pub trait QOpenGLContext_hasExtension {
  fn hasExtension(self, this: &mut QOpenGLContext) -> i32;
}

// proto: bool QOpenGLContext::hasExtension(const QByteArray & extension);
impl<'a> /*trait*/ QOpenGLContext_hasExtension for (&'a  QByteArray) {
  fn hasExtension(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext12hasExtensionERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK14QOpenGLContext12hasExtensionERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn extensions<T: QOpenGLContext_extensions>(&mut self, value: T) -> i32 {
    value.extensions(self);
    return 1;
  }
}

pub trait QOpenGLContext_extensions {
  fn extensions(self, this: &mut QOpenGLContext) -> i32;
}

// proto: QSet<QByteArray> QOpenGLContext::extensions();
impl<'a> /*trait*/ QOpenGLContext_extensions for () {
  fn extensions(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext10extensionsEv()};
    unsafe {_ZNK14QOpenGLContext10extensionsEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn surface<T: QOpenGLContext_surface>(&mut self, value: T) -> i32 {
    value.surface(self);
    return 1;
  }
}

pub trait QOpenGLContext_surface {
  fn surface(self, this: &mut QOpenGLContext) -> i32;
}

// proto: QSurface * QOpenGLContext::surface();
impl<'a> /*trait*/ QOpenGLContext_surface for () {
  fn surface(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext7surfaceEv()};
    unsafe {_ZNK14QOpenGLContext7surfaceEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn versionFunctions<T: QOpenGLContext_versionFunctions>(&mut self, value: T) -> i32 {
    value.versionFunctions(self);
    return 1;
  }
}

pub trait QOpenGLContext_versionFunctions {
  fn versionFunctions(self, this: &mut QOpenGLContext) -> i32;
}

// proto: QAbstractOpenGLFunctions * QOpenGLContext::versionFunctions(const QOpenGLVersionProfile & versionProfile);
impl<'a> /*trait*/ QOpenGLContext_versionFunctions for (&'a  QOpenGLVersionProfile) {
  fn versionFunctions(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext16versionFunctionsERK21QOpenGLVersionProfile()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK14QOpenGLContext16versionFunctionsERK21QOpenGLVersionProfile(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn setShareContext<T: QOpenGLContext_setShareContext>(&mut self, value: T) -> i32 {
    value.setShareContext(self);
    return 1;
  }
}

pub trait QOpenGLContext_setShareContext {
  fn setShareContext(self, this: &mut QOpenGLContext) -> i32;
}

// proto: void QOpenGLContext::setShareContext(QOpenGLContext * shareContext);
impl<'a> /*trait*/ QOpenGLContext_setShareContext for (&'a mut QOpenGLContext) {
  fn setShareContext(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext15setShareContextEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QOpenGLContext15setShareContextEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn areSharing<T: QOpenGLContext_areSharing>(&mut self, value: T) -> i32 {
    value.areSharing(self);
    return 1;
  }
}

pub trait QOpenGLContext_areSharing {
  fn areSharing(self, this: &mut QOpenGLContext) -> i32;
}

// proto: bool QOpenGLContext::areSharing(QOpenGLContext * first, QOpenGLContext * second);
impl<'a> /*trait*/ QOpenGLContext_areSharing for (&'a mut QOpenGLContext, &'a mut QOpenGLContext) {
  fn areSharing(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext10areSharingEPS_S0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN14QOpenGLContext10areSharingEPS_S0_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn screen<T: QOpenGLContext_screen>(&mut self, value: T) -> i32 {
    value.screen(self);
    return 1;
  }
}

pub trait QOpenGLContext_screen {
  fn screen(self, this: &mut QOpenGLContext) -> i32;
}

// proto: QScreen * QOpenGLContext::screen();
impl<'a> /*trait*/ QOpenGLContext_screen for () {
  fn screen(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext6screenEv()};
    unsafe {_ZNK14QOpenGLContext6screenEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn nativeHandle<T: QOpenGLContext_nativeHandle>(&mut self, value: T) -> i32 {
    value.nativeHandle(self);
    return 1;
  }
}

pub trait QOpenGLContext_nativeHandle {
  fn nativeHandle(self, this: &mut QOpenGLContext) -> i32;
}

// proto: QVariant QOpenGLContext::nativeHandle();
impl<'a> /*trait*/ QOpenGLContext_nativeHandle for () {
  fn nativeHandle(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext12nativeHandleEv()};
    unsafe {_ZNK14QOpenGLContext12nativeHandleEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn aboutToBeDestroyed<T: QOpenGLContext_aboutToBeDestroyed>(&mut self, value: T) -> i32 {
    value.aboutToBeDestroyed(self);
    return 1;
  }
}

pub trait QOpenGLContext_aboutToBeDestroyed {
  fn aboutToBeDestroyed(self, this: &mut QOpenGLContext) -> i32;
}

// proto: void QOpenGLContext::aboutToBeDestroyed();
impl<'a> /*trait*/ QOpenGLContext_aboutToBeDestroyed for () {
  fn aboutToBeDestroyed(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext18aboutToBeDestroyedEv()};
    unsafe {_ZN14QOpenGLContext18aboutToBeDestroyedEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn isOpenGLES<T: QOpenGLContext_isOpenGLES>(&mut self, value: T) -> i32 {
    value.isOpenGLES(self);
    return 1;
  }
}

pub trait QOpenGLContext_isOpenGLES {
  fn isOpenGLES(self, this: &mut QOpenGLContext) -> i32;
}

// proto: bool QOpenGLContext::isOpenGLES();
impl<'a> /*trait*/ QOpenGLContext_isOpenGLES for () {
  fn isOpenGLES(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext10isOpenGLESEv()};
    unsafe {_ZNK14QOpenGLContext10isOpenGLESEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn handle<T: QOpenGLContext_handle>(&mut self, value: T) -> i32 {
    value.handle(self);
    return 1;
  }
}

pub trait QOpenGLContext_handle {
  fn handle(self, this: &mut QOpenGLContext) -> i32;
}

// proto: QPlatformOpenGLContext * QOpenGLContext::handle();
impl<'a> /*trait*/ QOpenGLContext_handle for () {
  fn handle(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext6handleEv()};
    unsafe {_ZNK14QOpenGLContext6handleEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn globalShareContext<T: QOpenGLContext_globalShareContext>(&mut self, value: T) -> i32 {
    value.globalShareContext(self);
    return 1;
  }
}

pub trait QOpenGLContext_globalShareContext {
  fn globalShareContext(self, this: &mut QOpenGLContext) -> i32;
}

// proto: QOpenGLContext * QOpenGLContext::globalShareContext();
impl<'a> /*trait*/ QOpenGLContext_globalShareContext for () {
  fn globalShareContext(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext18globalShareContextEv()};
    unsafe {_ZN14QOpenGLContext18globalShareContextEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn makeCurrent<T: QOpenGLContext_makeCurrent>(&mut self, value: T) -> i32 {
    value.makeCurrent(self);
    return 1;
  }
}

pub trait QOpenGLContext_makeCurrent {
  fn makeCurrent(self, this: &mut QOpenGLContext) -> i32;
}

// proto: bool QOpenGLContext::makeCurrent(QSurface * surface);
impl<'a> /*trait*/ QOpenGLContext_makeCurrent for (&'a mut QSurface) {
  fn makeCurrent(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext11makeCurrentEP8QSurface()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QOpenGLContext11makeCurrentEP8QSurface(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn shareHandle<T: QOpenGLContext_shareHandle>(&mut self, value: T) -> i32 {
    value.shareHandle(self);
    return 1;
  }
}

pub trait QOpenGLContext_shareHandle {
  fn shareHandle(self, this: &mut QOpenGLContext) -> i32;
}

// proto: QPlatformOpenGLContext * QOpenGLContext::shareHandle();
impl<'a> /*trait*/ QOpenGLContext_shareHandle for () {
  fn shareHandle(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext11shareHandleEv()};
    unsafe {_ZNK14QOpenGLContext11shareHandleEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn create<T: QOpenGLContext_create>(&mut self, value: T) -> i32 {
    value.create(self);
    return 1;
  }
}

pub trait QOpenGLContext_create {
  fn create(self, this: &mut QOpenGLContext) -> i32;
}

// proto: bool QOpenGLContext::create();
impl<'a> /*trait*/ QOpenGLContext_create for () {
  fn create(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext6createEv()};
    unsafe {_ZN14QOpenGLContext6createEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn shareContext<T: QOpenGLContext_shareContext>(&mut self, value: T) -> i32 {
    value.shareContext(self);
    return 1;
  }
}

pub trait QOpenGLContext_shareContext {
  fn shareContext(self, this: &mut QOpenGLContext) -> i32;
}

// proto: QOpenGLContext * QOpenGLContext::shareContext();
impl<'a> /*trait*/ QOpenGLContext_shareContext for () {
  fn shareContext(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext12shareContextEv()};
    unsafe {_ZNK14QOpenGLContext12shareContextEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn currentContext<T: QOpenGLContext_currentContext>(&mut self, value: T) -> i32 {
    value.currentContext(self);
    return 1;
  }
}

pub trait QOpenGLContext_currentContext {
  fn currentContext(self, this: &mut QOpenGLContext) -> i32;
}

// proto: QOpenGLContext * QOpenGLContext::currentContext();
impl<'a> /*trait*/ QOpenGLContext_currentContext for () {
  fn currentContext(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext14currentContextEv()};
    unsafe {_ZN14QOpenGLContext14currentContextEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn defaultFramebufferObject<T: QOpenGLContext_defaultFramebufferObject>(&mut self, value: T) -> i32 {
    value.defaultFramebufferObject(self);
    return 1;
  }
}

pub trait QOpenGLContext_defaultFramebufferObject {
  fn defaultFramebufferObject(self, this: &mut QOpenGLContext) -> i32;
}

// proto: QOpenGLContext::GLuint QOpenGLContext::defaultFramebufferObject();
impl<'a> /*trait*/ QOpenGLContext_defaultFramebufferObject for () {
  fn defaultFramebufferObject(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext24defaultFramebufferObjectEv()};
    unsafe {_ZNK14QOpenGLContext24defaultFramebufferObjectEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn supportsThreadedOpenGL<T: QOpenGLContext_supportsThreadedOpenGL>(&mut self, value: T) -> i32 {
    value.supportsThreadedOpenGL(self);
    return 1;
  }
}

pub trait QOpenGLContext_supportsThreadedOpenGL {
  fn supportsThreadedOpenGL(self, this: &mut QOpenGLContext) -> i32;
}

// proto: bool QOpenGLContext::supportsThreadedOpenGL();
impl<'a> /*trait*/ QOpenGLContext_supportsThreadedOpenGL for () {
  fn supportsThreadedOpenGL(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext22supportsThreadedOpenGLEv()};
    unsafe {_ZN14QOpenGLContext22supportsThreadedOpenGLEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn doneCurrent<T: QOpenGLContext_doneCurrent>(&mut self, value: T) -> i32 {
    value.doneCurrent(self);
    return 1;
  }
}

pub trait QOpenGLContext_doneCurrent {
  fn doneCurrent(self, this: &mut QOpenGLContext) -> i32;
}

// proto: void QOpenGLContext::doneCurrent();
impl<'a> /*trait*/ QOpenGLContext_doneCurrent for () {
  fn doneCurrent(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext11doneCurrentEv()};
    unsafe {_ZN14QOpenGLContext11doneCurrentEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn shareGroup<T: QOpenGLContext_shareGroup>(&mut self, value: T) -> i32 {
    value.shareGroup(self);
    return 1;
  }
}

pub trait QOpenGLContext_shareGroup {
  fn shareGroup(self, this: &mut QOpenGLContext) -> i32;
}

// proto: QOpenGLContextGroup * QOpenGLContext::shareGroup();
impl<'a> /*trait*/ QOpenGLContext_shareGroup for () {
  fn shareGroup(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext10shareGroupEv()};
    unsafe {_ZNK14QOpenGLContext10shareGroupEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn format<T: QOpenGLContext_format>(&mut self, value: T) -> i32 {
    value.format(self);
    return 1;
  }
}

pub trait QOpenGLContext_format {
  fn format(self, this: &mut QOpenGLContext) -> i32;
}

// proto: QSurfaceFormat QOpenGLContext::format();
impl<'a> /*trait*/ QOpenGLContext_format for () {
  fn format(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext6formatEv()};
    unsafe {_ZNK14QOpenGLContext6formatEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn openGLModuleHandle<T: QOpenGLContext_openGLModuleHandle>(&mut self, value: T) -> i32 {
    value.openGLModuleHandle(self);
    return 1;
  }
}

pub trait QOpenGLContext_openGLModuleHandle {
  fn openGLModuleHandle(self, this: &mut QOpenGLContext) -> i32;
}

// proto: void * QOpenGLContext::openGLModuleHandle();
impl<'a> /*trait*/ QOpenGLContext_openGLModuleHandle for () {
  fn openGLModuleHandle(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext18openGLModuleHandleEv()};
    unsafe {_ZN14QOpenGLContext18openGLModuleHandleEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn setNativeHandle<T: QOpenGLContext_setNativeHandle>(&mut self, value: T) -> i32 {
    value.setNativeHandle(self);
    return 1;
  }
}

pub trait QOpenGLContext_setNativeHandle {
  fn setNativeHandle(self, this: &mut QOpenGLContext) -> i32;
}

// proto: void QOpenGLContext::setNativeHandle(const QVariant & handle);
impl<'a> /*trait*/ QOpenGLContext_setNativeHandle for (&'a  QVariant) {
  fn setNativeHandle(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext15setNativeHandleERK8QVariant()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QOpenGLContext15setNativeHandleERK8QVariant(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn getProcAddress<T: QOpenGLContext_getProcAddress>(&mut self, value: T) -> i32 {
    value.getProcAddress(self);
    return 1;
  }
}

pub trait QOpenGLContext_getProcAddress {
  fn getProcAddress(self, this: &mut QOpenGLContext) -> i32;
}

// proto: QFunctionPointer QOpenGLContext::getProcAddress(const QByteArray & procName);
impl<'a> /*trait*/ QOpenGLContext_getProcAddress for (&'a  QByteArray) {
  fn getProcAddress(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext14getProcAddressERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK14QOpenGLContext14getProcAddressERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn swapBuffers<T: QOpenGLContext_swapBuffers>(&mut self, value: T) -> i32 {
    value.swapBuffers(self);
    return 1;
  }
}

pub trait QOpenGLContext_swapBuffers {
  fn swapBuffers(self, this: &mut QOpenGLContext) -> i32;
}

// proto: void QOpenGLContext::swapBuffers(QSurface * surface);
impl<'a> /*trait*/ QOpenGLContext_swapBuffers for (&'a mut QSurface) {
  fn swapBuffers(self, this: &mut QOpenGLContext) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext11swapBuffersEP8QSurface()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QOpenGLContext11swapBuffersEP8QSurface(arg0)};
    return 1;
  }
}

