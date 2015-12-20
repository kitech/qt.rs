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
  fn _ZNK14QOpenGLContext7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  void QOpenGLContext::setScreen(QScreen * screen);
  fn _ZN14QOpenGLContext9setScreenEP7QScreen(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLContext::QOpenGLContext(QObject * parent);
  fn _ZN14QOpenGLContextC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QOpenGLFunctions * QOpenGLContext::functions();
  fn _ZNK14QOpenGLContext9functionsEv(qthis: *mut c_void);
  // proto:  void QOpenGLContext::~QOpenGLContext();
  fn _ZN14QOpenGLContextD0Ev(qthis: *mut c_void);
  // proto:  void QOpenGLContext::setFormat(const QSurfaceFormat & format);
  fn _ZN14QOpenGLContext9setFormatERK14QSurfaceFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QOpenGLContext::metaObject();
  fn _ZNK14QOpenGLContext10metaObjectEv(qthis: *mut c_void);
  // proto:  bool QOpenGLContext::hasExtension(const QByteArray & extension);
  fn _ZNK14QOpenGLContext12hasExtensionERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QSet<QByteArray> QOpenGLContext::extensions();
  fn _ZNK14QOpenGLContext10extensionsEv(qthis: *mut c_void);
  // proto:  QSurface * QOpenGLContext::surface();
  fn _ZNK14QOpenGLContext7surfaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAbstractOpenGLFunctions * QOpenGLContext::versionFunctions(const QOpenGLVersionProfile & versionProfile);
  fn _ZNK14QOpenGLContext16versionFunctionsERK21QOpenGLVersionProfile(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLContext::setShareContext(QOpenGLContext * shareContext);
  fn _ZN14QOpenGLContext15setShareContextEPS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static bool QOpenGLContext::areSharing(QOpenGLContext * first, QOpenGLContext * second);
  fn _ZN14QOpenGLContext10areSharingEPS_S0_(arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  QScreen * QOpenGLContext::screen();
  fn _ZNK14QOpenGLContext6screenEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QVariant QOpenGLContext::nativeHandle();
  fn _ZNK14QOpenGLContext12nativeHandleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QOpenGLContext::aboutToBeDestroyed();
  fn _ZN14QOpenGLContext18aboutToBeDestroyedEv(qthis: *mut c_void);
  // proto:  bool QOpenGLContext::isOpenGLES();
  fn _ZNK14QOpenGLContext10isOpenGLESEv(qthis: *mut c_void) -> c_char;
  // proto:  QPlatformOpenGLContext * QOpenGLContext::handle();
  fn _ZNK14QOpenGLContext6handleEv(qthis: *mut c_void);
  // proto: static QOpenGLContext * QOpenGLContext::globalShareContext();
  fn _ZN14QOpenGLContext18globalShareContextEv();
  // proto:  bool QOpenGLContext::makeCurrent(QSurface * surface);
  fn _ZN14QOpenGLContext11makeCurrentEP8QSurface(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QPlatformOpenGLContext * QOpenGLContext::shareHandle();
  fn _ZNK14QOpenGLContext11shareHandleEv(qthis: *mut c_void);
  // proto:  bool QOpenGLContext::create();
  fn _ZN14QOpenGLContext6createEv(qthis: *mut c_void) -> c_char;
  // proto:  QOpenGLContext * QOpenGLContext::shareContext();
  fn _ZNK14QOpenGLContext12shareContextEv(qthis: *mut c_void);
  // proto: static QOpenGLContext * QOpenGLContext::currentContext();
  fn _ZN14QOpenGLContext14currentContextEv();
  // proto:  GLuint QOpenGLContext::defaultFramebufferObject();
  fn _ZNK14QOpenGLContext24defaultFramebufferObjectEv(qthis: *mut c_void);
  // proto: static bool QOpenGLContext::supportsThreadedOpenGL();
  fn _ZN14QOpenGLContext22supportsThreadedOpenGLEv() -> c_char;
  // proto:  void QOpenGLContext::doneCurrent();
  fn _ZN14QOpenGLContext11doneCurrentEv(qthis: *mut c_void);
  // proto:  QOpenGLContextGroup * QOpenGLContext::shareGroup();
  fn _ZNK14QOpenGLContext10shareGroupEv(qthis: *mut c_void);
  // proto:  QSurfaceFormat QOpenGLContext::format();
  fn _ZNK14QOpenGLContext6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static void * QOpenGLContext::openGLModuleHandle();
  fn _ZN14QOpenGLContext18openGLModuleHandleEv() -> *mut c_void;
  // proto:  void QOpenGLContext::setNativeHandle(const QVariant & handle);
  fn _ZN14QOpenGLContext15setNativeHandleERK8QVariant(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QFunctionPointer QOpenGLContext::getProcAddress(const QByteArray & procName);
  fn _ZNK14QOpenGLContext14getProcAddressERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLContext::swapBuffers(QSurface * surface);
  fn _ZN14QOpenGLContext11swapBuffersEP8QSurface(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QOpenGLContext)=1
pub struct QOpenGLContext {
  pub qclsinst: *mut c_void,
}

  // proto:  bool QOpenGLContext::isValid();
impl /*struct*/ QOpenGLContext {
  pub fn isValid<RetType, T: QOpenGLContext_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QOpenGLContext_isValid<RetType> {
  fn isValid(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  bool QOpenGLContext::isValid();
impl<'a> /*trait*/ QOpenGLContext_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QOpenGLContext) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext7isValidEv()};
    let mut ret = unsafe {_ZNK14QOpenGLContext7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLContext::setScreen(QScreen * screen);
impl /*struct*/ QOpenGLContext {
  pub fn setScreen<RetType, T: QOpenGLContext_setScreen<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setScreen(self);
    // return 1;
  }
}

pub trait QOpenGLContext_setScreen<RetType> {
  fn setScreen(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  void QOpenGLContext::setScreen(QScreen * screen);
impl<'a> /*trait*/ QOpenGLContext_setScreen<()> for (QScreen) {
  fn setScreen(self , rsthis: &mut QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext9setScreenEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLContext9setScreenEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLContext::QOpenGLContext(QObject * parent);
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

  // proto:  void QOpenGLContext::QOpenGLContext(QObject * parent);
impl<'a> /*trait*/ QOpenGLContext_NewQOpenGLContext for (QObject) {
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

  // proto:  QOpenGLFunctions * QOpenGLContext::functions();
impl /*struct*/ QOpenGLContext {
  pub fn functions<RetType, T: QOpenGLContext_functions<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.functions(self);
    // return 1;
  }
}

pub trait QOpenGLContext_functions<RetType> {
  fn functions(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  QOpenGLFunctions * QOpenGLContext::functions();
impl<'a> /*trait*/ QOpenGLContext_functions<()> for () {
  fn functions(self , rsthis: &mut QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext9functionsEv()};
     unsafe {_ZNK14QOpenGLContext9functionsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLContext::~QOpenGLContext();
impl /*struct*/ QOpenGLContext {
  pub fn FreeQOpenGLContext<RetType, T: QOpenGLContext_FreeQOpenGLContext<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQOpenGLContext(self);
    // return 1;
  }
}

pub trait QOpenGLContext_FreeQOpenGLContext<RetType> {
  fn FreeQOpenGLContext(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  void QOpenGLContext::~QOpenGLContext();
impl<'a> /*trait*/ QOpenGLContext_FreeQOpenGLContext<()> for () {
  fn FreeQOpenGLContext(self , rsthis: &mut QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContextD0Ev()};
     unsafe {_ZN14QOpenGLContextD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLContext::setFormat(const QSurfaceFormat & format);
impl /*struct*/ QOpenGLContext {
  pub fn setFormat<RetType, T: QOpenGLContext_setFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QOpenGLContext_setFormat<RetType> {
  fn setFormat(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  void QOpenGLContext::setFormat(const QSurfaceFormat & format);
impl<'a> /*trait*/ QOpenGLContext_setFormat<()> for (QSurfaceFormat) {
  fn setFormat(self , rsthis: &mut QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext9setFormatERK14QSurfaceFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLContext9setFormatERK14QSurfaceFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QOpenGLContext::metaObject();
impl /*struct*/ QOpenGLContext {
  pub fn metaObject<RetType, T: QOpenGLContext_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLContext_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  const QMetaObject * QOpenGLContext::metaObject();
impl<'a> /*trait*/ QOpenGLContext_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext10metaObjectEv()};
     unsafe {_ZNK14QOpenGLContext10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLContext::hasExtension(const QByteArray & extension);
impl /*struct*/ QOpenGLContext {
  pub fn hasExtension<RetType, T: QOpenGLContext_hasExtension<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasExtension(self);
    // return 1;
  }
}

pub trait QOpenGLContext_hasExtension<RetType> {
  fn hasExtension(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  bool QOpenGLContext::hasExtension(const QByteArray & extension);
impl<'a> /*trait*/ QOpenGLContext_hasExtension<i8> for (QByteArray) {
  fn hasExtension(self , rsthis: &mut QOpenGLContext) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext12hasExtensionERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QOpenGLContext12hasExtensionERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSet<QByteArray> QOpenGLContext::extensions();
impl /*struct*/ QOpenGLContext {
  pub fn extensions<RetType, T: QOpenGLContext_extensions<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.extensions(self);
    // return 1;
  }
}

pub trait QOpenGLContext_extensions<RetType> {
  fn extensions(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  QSet<QByteArray> QOpenGLContext::extensions();
impl<'a> /*trait*/ QOpenGLContext_extensions<()> for () {
  fn extensions(self , rsthis: &mut QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext10extensionsEv()};
     unsafe {_ZNK14QOpenGLContext10extensionsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSurface * QOpenGLContext::surface();
impl /*struct*/ QOpenGLContext {
  pub fn surface<RetType, T: QOpenGLContext_surface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.surface(self);
    // return 1;
  }
}

pub trait QOpenGLContext_surface<RetType> {
  fn surface(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  QSurface * QOpenGLContext::surface();
impl<'a> /*trait*/ QOpenGLContext_surface<QSurface> for () {
  fn surface(self , rsthis: &mut QOpenGLContext) -> QSurface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext7surfaceEv()};
    let mut ret = unsafe {_ZNK14QOpenGLContext7surfaceEv(rsthis.qclsinst)};
    let mut ret1 = QSurface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QAbstractOpenGLFunctions * QOpenGLContext::versionFunctions(const QOpenGLVersionProfile & versionProfile);
impl /*struct*/ QOpenGLContext {
  pub fn versionFunctions<RetType, T: QOpenGLContext_versionFunctions<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.versionFunctions(self);
    // return 1;
  }
}

pub trait QOpenGLContext_versionFunctions<RetType> {
  fn versionFunctions(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  QAbstractOpenGLFunctions * QOpenGLContext::versionFunctions(const QOpenGLVersionProfile & versionProfile);
impl<'a> /*trait*/ QOpenGLContext_versionFunctions<()> for (QOpenGLVersionProfile) {
  fn versionFunctions(self , rsthis: &mut QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext16versionFunctionsERK21QOpenGLVersionProfile()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK14QOpenGLContext16versionFunctionsERK21QOpenGLVersionProfile(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLContext::setShareContext(QOpenGLContext * shareContext);
impl /*struct*/ QOpenGLContext {
  pub fn setShareContext<RetType, T: QOpenGLContext_setShareContext<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setShareContext(self);
    // return 1;
  }
}

pub trait QOpenGLContext_setShareContext<RetType> {
  fn setShareContext(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  void QOpenGLContext::setShareContext(QOpenGLContext * shareContext);
impl<'a> /*trait*/ QOpenGLContext_setShareContext<()> for (QOpenGLContext) {
  fn setShareContext(self , rsthis: &mut QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext15setShareContextEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLContext15setShareContextEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static bool QOpenGLContext::areSharing(QOpenGLContext * first, QOpenGLContext * second);
impl /*struct*/ QOpenGLContext {
  pub fn areSharing_s<RetType, T: QOpenGLContext_areSharing_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.areSharing_s();
    // return 1;
  }
}

pub trait QOpenGLContext_areSharing_s<RetType> {
  fn areSharing_s(self ) -> RetType;
}

  // proto: static bool QOpenGLContext::areSharing(QOpenGLContext * first, QOpenGLContext * second);
impl<'a> /*trait*/ QOpenGLContext_areSharing_s<i8> for (QOpenGLContext, QOpenGLContext) {
  fn areSharing_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext10areSharingEPS_S0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QOpenGLContext10areSharingEPS_S0_(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QScreen * QOpenGLContext::screen();
impl /*struct*/ QOpenGLContext {
  pub fn screen<RetType, T: QOpenGLContext_screen<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.screen(self);
    // return 1;
  }
}

pub trait QOpenGLContext_screen<RetType> {
  fn screen(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  QScreen * QOpenGLContext::screen();
impl<'a> /*trait*/ QOpenGLContext_screen<QScreen> for () {
  fn screen(self , rsthis: &mut QOpenGLContext) -> QScreen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext6screenEv()};
    let mut ret = unsafe {_ZNK14QOpenGLContext6screenEv(rsthis.qclsinst)};
    let mut ret1 = QScreen{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QOpenGLContext::nativeHandle();
impl /*struct*/ QOpenGLContext {
  pub fn nativeHandle<RetType, T: QOpenGLContext_nativeHandle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.nativeHandle(self);
    // return 1;
  }
}

pub trait QOpenGLContext_nativeHandle<RetType> {
  fn nativeHandle(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  QVariant QOpenGLContext::nativeHandle();
impl<'a> /*trait*/ QOpenGLContext_nativeHandle<QVariant> for () {
  fn nativeHandle(self , rsthis: &mut QOpenGLContext) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext12nativeHandleEv()};
    let mut ret = unsafe {_ZNK14QOpenGLContext12nativeHandleEv(rsthis.qclsinst)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QOpenGLContext::aboutToBeDestroyed();
impl /*struct*/ QOpenGLContext {
  pub fn aboutToBeDestroyed<RetType, T: QOpenGLContext_aboutToBeDestroyed<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.aboutToBeDestroyed(self);
    // return 1;
  }
}

pub trait QOpenGLContext_aboutToBeDestroyed<RetType> {
  fn aboutToBeDestroyed(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  void QOpenGLContext::aboutToBeDestroyed();
impl<'a> /*trait*/ QOpenGLContext_aboutToBeDestroyed<()> for () {
  fn aboutToBeDestroyed(self , rsthis: &mut QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext18aboutToBeDestroyedEv()};
     unsafe {_ZN14QOpenGLContext18aboutToBeDestroyedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLContext::isOpenGLES();
impl /*struct*/ QOpenGLContext {
  pub fn isOpenGLES<RetType, T: QOpenGLContext_isOpenGLES<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isOpenGLES(self);
    // return 1;
  }
}

pub trait QOpenGLContext_isOpenGLES<RetType> {
  fn isOpenGLES(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  bool QOpenGLContext::isOpenGLES();
impl<'a> /*trait*/ QOpenGLContext_isOpenGLES<i8> for () {
  fn isOpenGLES(self , rsthis: &mut QOpenGLContext) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext10isOpenGLESEv()};
    let mut ret = unsafe {_ZNK14QOpenGLContext10isOpenGLESEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPlatformOpenGLContext * QOpenGLContext::handle();
impl /*struct*/ QOpenGLContext {
  pub fn handle<RetType, T: QOpenGLContext_handle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.handle(self);
    // return 1;
  }
}

pub trait QOpenGLContext_handle<RetType> {
  fn handle(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  QPlatformOpenGLContext * QOpenGLContext::handle();
impl<'a> /*trait*/ QOpenGLContext_handle<()> for () {
  fn handle(self , rsthis: &mut QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext6handleEv()};
     unsafe {_ZNK14QOpenGLContext6handleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QOpenGLContext * QOpenGLContext::globalShareContext();
impl /*struct*/ QOpenGLContext {
  pub fn globalShareContext_s<RetType, T: QOpenGLContext_globalShareContext_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.globalShareContext_s();
    // return 1;
  }
}

pub trait QOpenGLContext_globalShareContext_s<RetType> {
  fn globalShareContext_s(self ) -> RetType;
}

  // proto: static QOpenGLContext * QOpenGLContext::globalShareContext();
impl<'a> /*trait*/ QOpenGLContext_globalShareContext_s<()> for () {
  fn globalShareContext_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext18globalShareContextEv()};
     unsafe {_ZN14QOpenGLContext18globalShareContextEv()};
    // return 1;
  }
}

  // proto:  bool QOpenGLContext::makeCurrent(QSurface * surface);
impl /*struct*/ QOpenGLContext {
  pub fn makeCurrent<RetType, T: QOpenGLContext_makeCurrent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.makeCurrent(self);
    // return 1;
  }
}

pub trait QOpenGLContext_makeCurrent<RetType> {
  fn makeCurrent(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  bool QOpenGLContext::makeCurrent(QSurface * surface);
impl<'a> /*trait*/ QOpenGLContext_makeCurrent<i8> for (QSurface) {
  fn makeCurrent(self , rsthis: &mut QOpenGLContext) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext11makeCurrentEP8QSurface()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QOpenGLContext11makeCurrentEP8QSurface(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPlatformOpenGLContext * QOpenGLContext::shareHandle();
impl /*struct*/ QOpenGLContext {
  pub fn shareHandle<RetType, T: QOpenGLContext_shareHandle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.shareHandle(self);
    // return 1;
  }
}

pub trait QOpenGLContext_shareHandle<RetType> {
  fn shareHandle(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  QPlatformOpenGLContext * QOpenGLContext::shareHandle();
impl<'a> /*trait*/ QOpenGLContext_shareHandle<()> for () {
  fn shareHandle(self , rsthis: &mut QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext11shareHandleEv()};
     unsafe {_ZNK14QOpenGLContext11shareHandleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLContext::create();
impl /*struct*/ QOpenGLContext {
  pub fn create<RetType, T: QOpenGLContext_create<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QOpenGLContext_create<RetType> {
  fn create(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  bool QOpenGLContext::create();
impl<'a> /*trait*/ QOpenGLContext_create<i8> for () {
  fn create(self , rsthis: &mut QOpenGLContext) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext6createEv()};
    let mut ret = unsafe {_ZN14QOpenGLContext6createEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QOpenGLContext * QOpenGLContext::shareContext();
impl /*struct*/ QOpenGLContext {
  pub fn shareContext<RetType, T: QOpenGLContext_shareContext<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.shareContext(self);
    // return 1;
  }
}

pub trait QOpenGLContext_shareContext<RetType> {
  fn shareContext(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  QOpenGLContext * QOpenGLContext::shareContext();
impl<'a> /*trait*/ QOpenGLContext_shareContext<()> for () {
  fn shareContext(self , rsthis: &mut QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext12shareContextEv()};
     unsafe {_ZNK14QOpenGLContext12shareContextEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QOpenGLContext * QOpenGLContext::currentContext();
impl /*struct*/ QOpenGLContext {
  pub fn currentContext_s<RetType, T: QOpenGLContext_currentContext_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentContext_s();
    // return 1;
  }
}

pub trait QOpenGLContext_currentContext_s<RetType> {
  fn currentContext_s(self ) -> RetType;
}

  // proto: static QOpenGLContext * QOpenGLContext::currentContext();
impl<'a> /*trait*/ QOpenGLContext_currentContext_s<()> for () {
  fn currentContext_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext14currentContextEv()};
     unsafe {_ZN14QOpenGLContext14currentContextEv()};
    // return 1;
  }
}

  // proto:  GLuint QOpenGLContext::defaultFramebufferObject();
impl /*struct*/ QOpenGLContext {
  pub fn defaultFramebufferObject<RetType, T: QOpenGLContext_defaultFramebufferObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.defaultFramebufferObject(self);
    // return 1;
  }
}

pub trait QOpenGLContext_defaultFramebufferObject<RetType> {
  fn defaultFramebufferObject(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  GLuint QOpenGLContext::defaultFramebufferObject();
impl<'a> /*trait*/ QOpenGLContext_defaultFramebufferObject<()> for () {
  fn defaultFramebufferObject(self , rsthis: &mut QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext24defaultFramebufferObjectEv()};
     unsafe {_ZNK14QOpenGLContext24defaultFramebufferObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static bool QOpenGLContext::supportsThreadedOpenGL();
impl /*struct*/ QOpenGLContext {
  pub fn supportsThreadedOpenGL_s<RetType, T: QOpenGLContext_supportsThreadedOpenGL_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.supportsThreadedOpenGL_s();
    // return 1;
  }
}

pub trait QOpenGLContext_supportsThreadedOpenGL_s<RetType> {
  fn supportsThreadedOpenGL_s(self ) -> RetType;
}

  // proto: static bool QOpenGLContext::supportsThreadedOpenGL();
impl<'a> /*trait*/ QOpenGLContext_supportsThreadedOpenGL_s<i8> for () {
  fn supportsThreadedOpenGL_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext22supportsThreadedOpenGLEv()};
    let mut ret = unsafe {_ZN14QOpenGLContext22supportsThreadedOpenGLEv()};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLContext::doneCurrent();
impl /*struct*/ QOpenGLContext {
  pub fn doneCurrent<RetType, T: QOpenGLContext_doneCurrent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.doneCurrent(self);
    // return 1;
  }
}

pub trait QOpenGLContext_doneCurrent<RetType> {
  fn doneCurrent(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  void QOpenGLContext::doneCurrent();
impl<'a> /*trait*/ QOpenGLContext_doneCurrent<()> for () {
  fn doneCurrent(self , rsthis: &mut QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext11doneCurrentEv()};
     unsafe {_ZN14QOpenGLContext11doneCurrentEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QOpenGLContextGroup * QOpenGLContext::shareGroup();
impl /*struct*/ QOpenGLContext {
  pub fn shareGroup<RetType, T: QOpenGLContext_shareGroup<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.shareGroup(self);
    // return 1;
  }
}

pub trait QOpenGLContext_shareGroup<RetType> {
  fn shareGroup(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  QOpenGLContextGroup * QOpenGLContext::shareGroup();
impl<'a> /*trait*/ QOpenGLContext_shareGroup<()> for () {
  fn shareGroup(self , rsthis: &mut QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext10shareGroupEv()};
     unsafe {_ZNK14QOpenGLContext10shareGroupEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSurfaceFormat QOpenGLContext::format();
impl /*struct*/ QOpenGLContext {
  pub fn format<RetType, T: QOpenGLContext_format<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QOpenGLContext_format<RetType> {
  fn format(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  QSurfaceFormat QOpenGLContext::format();
impl<'a> /*trait*/ QOpenGLContext_format<QSurfaceFormat> for () {
  fn format(self , rsthis: &mut QOpenGLContext) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext6formatEv()};
    let mut ret = unsafe {_ZNK14QOpenGLContext6formatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static void * QOpenGLContext::openGLModuleHandle();
impl /*struct*/ QOpenGLContext {
  pub fn openGLModuleHandle_s<RetType, T: QOpenGLContext_openGLModuleHandle_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.openGLModuleHandle_s();
    // return 1;
  }
}

pub trait QOpenGLContext_openGLModuleHandle_s<RetType> {
  fn openGLModuleHandle_s(self ) -> RetType;
}

  // proto: static void * QOpenGLContext::openGLModuleHandle();
impl<'a> /*trait*/ QOpenGLContext_openGLModuleHandle_s<*mut c_void> for () {
  fn openGLModuleHandle_s(self ) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext18openGLModuleHandleEv()};
    let mut ret = unsafe {_ZN14QOpenGLContext18openGLModuleHandleEv()};
    return ret as *mut c_void;
    // return 1;
  }
}

  // proto:  void QOpenGLContext::setNativeHandle(const QVariant & handle);
impl /*struct*/ QOpenGLContext {
  pub fn setNativeHandle<RetType, T: QOpenGLContext_setNativeHandle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setNativeHandle(self);
    // return 1;
  }
}

pub trait QOpenGLContext_setNativeHandle<RetType> {
  fn setNativeHandle(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  void QOpenGLContext::setNativeHandle(const QVariant & handle);
impl<'a> /*trait*/ QOpenGLContext_setNativeHandle<()> for (QVariant) {
  fn setNativeHandle(self , rsthis: &mut QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext15setNativeHandleERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLContext15setNativeHandleERK8QVariant(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QFunctionPointer QOpenGLContext::getProcAddress(const QByteArray & procName);
impl /*struct*/ QOpenGLContext {
  pub fn getProcAddress<RetType, T: QOpenGLContext_getProcAddress<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.getProcAddress(self);
    // return 1;
  }
}

pub trait QOpenGLContext_getProcAddress<RetType> {
  fn getProcAddress(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  QFunctionPointer QOpenGLContext::getProcAddress(const QByteArray & procName);
impl<'a> /*trait*/ QOpenGLContext_getProcAddress<()> for (QByteArray) {
  fn getProcAddress(self , rsthis: &mut QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext14getProcAddressERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK14QOpenGLContext14getProcAddressERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLContext::swapBuffers(QSurface * surface);
impl /*struct*/ QOpenGLContext {
  pub fn swapBuffers<RetType, T: QOpenGLContext_swapBuffers<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swapBuffers(self);
    // return 1;
  }
}

pub trait QOpenGLContext_swapBuffers<RetType> {
  fn swapBuffers(self , rsthis: &mut QOpenGLContext) -> RetType;
}

  // proto:  void QOpenGLContext::swapBuffers(QSurface * surface);
impl<'a> /*trait*/ QOpenGLContext_swapBuffers<()> for (QSurface) {
  fn swapBuffers(self , rsthis: &mut QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext11swapBuffersEP8QSurface()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLContext11swapBuffersEP8QSurface(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

