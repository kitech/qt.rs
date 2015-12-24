// auto generated, do not modify.
// created: Thu Dec 24 23:00:39 2015
// src-file: /QtGui/qopenglcontext.h
// dst-file: /src/gui/qopenglcontext.rs
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
use std::ops::Deref;
use super::qsurfaceformat::QSurfaceFormat; // 773
use super::super::core::qobject::QObject; // 771
use super::qscreen::QScreen; // 773
use super::super::core::qbytearray::QByteArray; // 771
use super::qsurface::QSurface; // 773
// use super::qopenglcontext::QOpenGLVersionProfile; // 773
use super::super::core::qvariant::QVariant; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]

// #[link(name = "QtInline")]

extern {
  // proto:  bool QOpenGLVersionProfile::isLegacyVersion();
  fn _ZNK21QOpenGLVersionProfile15isLegacyVersionEv(qthis: *mut c_void) -> c_char;
  // proto:  void QOpenGLVersionProfile::~QOpenGLVersionProfile();
  fn _ZN21QOpenGLVersionProfileD0Ev(qthis: *mut c_void);
  // proto:  bool QOpenGLVersionProfile::hasProfiles();
  fn _ZNK21QOpenGLVersionProfile11hasProfilesEv(qthis: *mut c_void) -> c_char;
  // proto:  void QOpenGLVersionProfile::QOpenGLVersionProfile(const QSurfaceFormat & format);
  fn _ZN21QOpenGLVersionProfileC1ERK14QSurfaceFormat(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLVersionProfile::QOpenGLVersionProfile(const QOpenGLVersionProfile & other);
  fn _ZN21QOpenGLVersionProfileC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLVersionProfile::QOpenGLVersionProfile();
  fn _ZN21QOpenGLVersionProfileC1Ev(qthis: *mut c_void);
  // proto:  QPair<int, int> QOpenGLVersionProfile::version();
  fn _ZNK21QOpenGLVersionProfile7versionEv(qthis: *mut c_void);
  // proto:  void QOpenGLVersionProfile::setVersion(int majorVersion, int minorVersion);
  fn _ZN21QOpenGLVersionProfile10setVersionEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  bool QOpenGLVersionProfile::isValid();
  fn _ZNK21QOpenGLVersionProfile7isValidEv(qthis: *mut c_void) -> c_char;
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
  // proto:  const QMetaObject * QOpenGLContextGroup::metaObject();
  fn _ZNK19QOpenGLContextGroup10metaObjectEv(qthis: *mut c_void);
  // proto: static QOpenGLContextGroup * QOpenGLContextGroup::currentContextGroup();
  fn _ZN19QOpenGLContextGroup19currentContextGroupEv();
  // proto:  void QOpenGLContextGroup::~QOpenGLContextGroup();
  fn _ZN19QOpenGLContextGroupD0Ev(qthis: *mut c_void);
  // proto:  void QOpenGLContextGroup::QOpenGLContextGroup();
  fn _ZN19QOpenGLContextGroupC1Ev(qthis: *mut c_void);
  // proto:  QList<QOpenGLContext *> QOpenGLContextGroup::shares();
  fn _ZNK19QOpenGLContextGroup6sharesEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QOpenGLVersionProfile)=8
pub struct QOpenGLVersionProfile {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLContext)=1
pub struct QOpenGLContext {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

// class sizeof(QOpenGLContextGroup)=1
pub struct QOpenGLContextGroup {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLVersionProfile {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLVersionProfile {
    return QOpenGLVersionProfile{qclsinst: qthis};
  }
}
  // proto:  bool QOpenGLVersionProfile::isLegacyVersion();
impl /*struct*/ QOpenGLVersionProfile {
  pub fn isLegacyVersion<RetType, T: QOpenGLVersionProfile_isLegacyVersion<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLegacyVersion(self);
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_isLegacyVersion<RetType> {
  fn isLegacyVersion(self , rsthis: & QOpenGLVersionProfile) -> RetType;
}

  // proto:  bool QOpenGLVersionProfile::isLegacyVersion();
impl<'a> /*trait*/ QOpenGLVersionProfile_isLegacyVersion<i8> for () {
  fn isLegacyVersion(self , rsthis: & QOpenGLVersionProfile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QOpenGLVersionProfile15isLegacyVersionEv()};
    let mut ret = unsafe {_ZNK21QOpenGLVersionProfile15isLegacyVersionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLVersionProfile::~QOpenGLVersionProfile();
impl /*struct*/ QOpenGLVersionProfile {
  pub fn Free<RetType, T: QOpenGLVersionProfile_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_Free<RetType> {
  fn Free(self , rsthis: & QOpenGLVersionProfile) -> RetType;
}

  // proto:  void QOpenGLVersionProfile::~QOpenGLVersionProfile();
impl<'a> /*trait*/ QOpenGLVersionProfile_Free<()> for () {
  fn Free(self , rsthis: & QOpenGLVersionProfile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QOpenGLVersionProfileD0Ev()};
     unsafe {_ZN21QOpenGLVersionProfileD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLVersionProfile::hasProfiles();
impl /*struct*/ QOpenGLVersionProfile {
  pub fn hasProfiles<RetType, T: QOpenGLVersionProfile_hasProfiles<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasProfiles(self);
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_hasProfiles<RetType> {
  fn hasProfiles(self , rsthis: & QOpenGLVersionProfile) -> RetType;
}

  // proto:  bool QOpenGLVersionProfile::hasProfiles();
impl<'a> /*trait*/ QOpenGLVersionProfile_hasProfiles<i8> for () {
  fn hasProfiles(self , rsthis: & QOpenGLVersionProfile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QOpenGLVersionProfile11hasProfilesEv()};
    let mut ret = unsafe {_ZNK21QOpenGLVersionProfile11hasProfilesEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLVersionProfile::QOpenGLVersionProfile(const QSurfaceFormat & format);
impl /*struct*/ QOpenGLVersionProfile {
  pub fn New<T: QOpenGLVersionProfile_New>(value: T) -> QOpenGLVersionProfile {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_New {
  fn New(self) -> QOpenGLVersionProfile;
}

  // proto:  void QOpenGLVersionProfile::QOpenGLVersionProfile(const QSurfaceFormat & format);
impl<'a> /*trait*/ QOpenGLVersionProfile_New for (&'a QSurfaceFormat) {
  fn New(self) -> QOpenGLVersionProfile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QOpenGLVersionProfileC1ERK14QSurfaceFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QOpenGLVersionProfileC1ERK14QSurfaceFormat(qthis, arg0)};
    let rsthis = QOpenGLVersionProfile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLVersionProfile::QOpenGLVersionProfile(const QOpenGLVersionProfile & other);
impl<'a> /*trait*/ QOpenGLVersionProfile_New for (&'a QOpenGLVersionProfile) {
  fn New(self) -> QOpenGLVersionProfile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QOpenGLVersionProfileC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QOpenGLVersionProfileC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLVersionProfile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLVersionProfile::QOpenGLVersionProfile();
impl<'a> /*trait*/ QOpenGLVersionProfile_New for () {
  fn New(self) -> QOpenGLVersionProfile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QOpenGLVersionProfileC1Ev()};
    unsafe {_ZN21QOpenGLVersionProfileC1Ev(qthis)};
    let rsthis = QOpenGLVersionProfile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPair<int, int> QOpenGLVersionProfile::version();
impl /*struct*/ QOpenGLVersionProfile {
  pub fn version<RetType, T: QOpenGLVersionProfile_version<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.version(self);
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_version<RetType> {
  fn version(self , rsthis: & QOpenGLVersionProfile) -> RetType;
}

  // proto:  QPair<int, int> QOpenGLVersionProfile::version();
impl<'a> /*trait*/ QOpenGLVersionProfile_version<()> for () {
  fn version(self , rsthis: & QOpenGLVersionProfile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QOpenGLVersionProfile7versionEv()};
     unsafe {_ZNK21QOpenGLVersionProfile7versionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLVersionProfile::setVersion(int majorVersion, int minorVersion);
impl /*struct*/ QOpenGLVersionProfile {
  pub fn setVersion<RetType, T: QOpenGLVersionProfile_setVersion<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVersion(self);
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_setVersion<RetType> {
  fn setVersion(self , rsthis: & QOpenGLVersionProfile) -> RetType;
}

  // proto:  void QOpenGLVersionProfile::setVersion(int majorVersion, int minorVersion);
impl<'a> /*trait*/ QOpenGLVersionProfile_setVersion<()> for (i32, i32) {
  fn setVersion(self , rsthis: & QOpenGLVersionProfile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QOpenGLVersionProfile10setVersionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN21QOpenGLVersionProfile10setVersionEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QOpenGLVersionProfile::isValid();
impl /*struct*/ QOpenGLVersionProfile {
  pub fn isValid<RetType, T: QOpenGLVersionProfile_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_isValid<RetType> {
  fn isValid(self , rsthis: & QOpenGLVersionProfile) -> RetType;
}

  // proto:  bool QOpenGLVersionProfile::isValid();
impl<'a> /*trait*/ QOpenGLVersionProfile_isValid<i8> for () {
  fn isValid(self , rsthis: & QOpenGLVersionProfile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QOpenGLVersionProfile7isValidEv()};
    let mut ret = unsafe {_ZNK21QOpenGLVersionProfile7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLContext {
    return QOpenGLContext{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLContext {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QOpenGLContext {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  bool QOpenGLContext::isValid();
impl /*struct*/ QOpenGLContext {
  pub fn isValid<RetType, T: QOpenGLContext_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QOpenGLContext_isValid<RetType> {
  fn isValid(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  bool QOpenGLContext::isValid();
impl<'a> /*trait*/ QOpenGLContext_isValid<i8> for () {
  fn isValid(self , rsthis: & QOpenGLContext) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext7isValidEv()};
    let mut ret = unsafe {_ZNK14QOpenGLContext7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLContext::setScreen(QScreen * screen);
impl /*struct*/ QOpenGLContext {
  pub fn setScreen<RetType, T: QOpenGLContext_setScreen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScreen(self);
    // return 1;
  }
}

pub trait QOpenGLContext_setScreen<RetType> {
  fn setScreen(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  void QOpenGLContext::setScreen(QScreen * screen);
impl<'a> /*trait*/ QOpenGLContext_setScreen<()> for (&'a QScreen) {
  fn setScreen(self , rsthis: & QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext9setScreenEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLContext9setScreenEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLContext::QOpenGLContext(QObject * parent);
impl /*struct*/ QOpenGLContext {
  pub fn New<T: QOpenGLContext_New>(value: T) -> QOpenGLContext {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLContext_New {
  fn New(self) -> QOpenGLContext;
}

  // proto:  void QOpenGLContext::QOpenGLContext(QObject * parent);
impl<'a> /*trait*/ QOpenGLContext_New for (&'a QObject) {
  fn New(self) -> QOpenGLContext {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContextC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QOpenGLContextC1EP7QObject(qthis, arg0)};
    let rsthis = QOpenGLContext{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QOpenGLFunctions * QOpenGLContext::functions();
impl /*struct*/ QOpenGLContext {
  pub fn functions<RetType, T: QOpenGLContext_functions<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.functions(self);
    // return 1;
  }
}

pub trait QOpenGLContext_functions<RetType> {
  fn functions(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  QOpenGLFunctions * QOpenGLContext::functions();
impl<'a> /*trait*/ QOpenGLContext_functions<()> for () {
  fn functions(self , rsthis: & QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext9functionsEv()};
     unsafe {_ZNK14QOpenGLContext9functionsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLContext::~QOpenGLContext();
impl /*struct*/ QOpenGLContext {
  pub fn Free<RetType, T: QOpenGLContext_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QOpenGLContext_Free<RetType> {
  fn Free(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  void QOpenGLContext::~QOpenGLContext();
impl<'a> /*trait*/ QOpenGLContext_Free<()> for () {
  fn Free(self , rsthis: & QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContextD0Ev()};
     unsafe {_ZN14QOpenGLContextD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLContext::setFormat(const QSurfaceFormat & format);
impl /*struct*/ QOpenGLContext {
  pub fn setFormat<RetType, T: QOpenGLContext_setFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QOpenGLContext_setFormat<RetType> {
  fn setFormat(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  void QOpenGLContext::setFormat(const QSurfaceFormat & format);
impl<'a> /*trait*/ QOpenGLContext_setFormat<()> for (&'a QSurfaceFormat) {
  fn setFormat(self , rsthis: & QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext9setFormatERK14QSurfaceFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLContext9setFormatERK14QSurfaceFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QOpenGLContext::metaObject();
impl /*struct*/ QOpenGLContext {
  pub fn metaObject<RetType, T: QOpenGLContext_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLContext_metaObject<RetType> {
  fn metaObject(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  const QMetaObject * QOpenGLContext::metaObject();
impl<'a> /*trait*/ QOpenGLContext_metaObject<()> for () {
  fn metaObject(self , rsthis: & QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext10metaObjectEv()};
     unsafe {_ZNK14QOpenGLContext10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLContext::hasExtension(const QByteArray & extension);
impl /*struct*/ QOpenGLContext {
  pub fn hasExtension<RetType, T: QOpenGLContext_hasExtension<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasExtension(self);
    // return 1;
  }
}

pub trait QOpenGLContext_hasExtension<RetType> {
  fn hasExtension(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  bool QOpenGLContext::hasExtension(const QByteArray & extension);
impl<'a> /*trait*/ QOpenGLContext_hasExtension<i8> for (&'a QByteArray) {
  fn hasExtension(self , rsthis: & QOpenGLContext) -> i8 {
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
  pub fn extensions<RetType, T: QOpenGLContext_extensions<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.extensions(self);
    // return 1;
  }
}

pub trait QOpenGLContext_extensions<RetType> {
  fn extensions(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  QSet<QByteArray> QOpenGLContext::extensions();
impl<'a> /*trait*/ QOpenGLContext_extensions<()> for () {
  fn extensions(self , rsthis: & QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext10extensionsEv()};
     unsafe {_ZNK14QOpenGLContext10extensionsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSurface * QOpenGLContext::surface();
impl /*struct*/ QOpenGLContext {
  pub fn surface<RetType, T: QOpenGLContext_surface<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.surface(self);
    // return 1;
  }
}

pub trait QOpenGLContext_surface<RetType> {
  fn surface(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  QSurface * QOpenGLContext::surface();
impl<'a> /*trait*/ QOpenGLContext_surface<QSurface> for () {
  fn surface(self , rsthis: & QOpenGLContext) -> QSurface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext7surfaceEv()};
    let mut ret = unsafe {_ZNK14QOpenGLContext7surfaceEv(rsthis.qclsinst)};
    let mut ret1 = QSurface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QAbstractOpenGLFunctions * QOpenGLContext::versionFunctions(const QOpenGLVersionProfile & versionProfile);
impl /*struct*/ QOpenGLContext {
  pub fn versionFunctions<RetType, T: QOpenGLContext_versionFunctions<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.versionFunctions(self);
    // return 1;
  }
}

pub trait QOpenGLContext_versionFunctions<RetType> {
  fn versionFunctions(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  QAbstractOpenGLFunctions * QOpenGLContext::versionFunctions(const QOpenGLVersionProfile & versionProfile);
impl<'a> /*trait*/ QOpenGLContext_versionFunctions<()> for (&'a QOpenGLVersionProfile) {
  fn versionFunctions(self , rsthis: & QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext16versionFunctionsERK21QOpenGLVersionProfile()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK14QOpenGLContext16versionFunctionsERK21QOpenGLVersionProfile(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLContext::setShareContext(QOpenGLContext * shareContext);
impl /*struct*/ QOpenGLContext {
  pub fn setShareContext<RetType, T: QOpenGLContext_setShareContext<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setShareContext(self);
    // return 1;
  }
}

pub trait QOpenGLContext_setShareContext<RetType> {
  fn setShareContext(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  void QOpenGLContext::setShareContext(QOpenGLContext * shareContext);
impl<'a> /*trait*/ QOpenGLContext_setShareContext<()> for (&'a QOpenGLContext) {
  fn setShareContext(self , rsthis: & QOpenGLContext) -> () {
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
impl<'a> /*trait*/ QOpenGLContext_areSharing_s<i8> for (&'a QOpenGLContext, &'a QOpenGLContext) {
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
  pub fn screen<RetType, T: QOpenGLContext_screen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screen(self);
    // return 1;
  }
}

pub trait QOpenGLContext_screen<RetType> {
  fn screen(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  QScreen * QOpenGLContext::screen();
impl<'a> /*trait*/ QOpenGLContext_screen<QScreen> for () {
  fn screen(self , rsthis: & QOpenGLContext) -> QScreen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext6screenEv()};
    let mut ret = unsafe {_ZNK14QOpenGLContext6screenEv(rsthis.qclsinst)};
    let mut ret1 = QScreen::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QOpenGLContext::nativeHandle();
impl /*struct*/ QOpenGLContext {
  pub fn nativeHandle<RetType, T: QOpenGLContext_nativeHandle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nativeHandle(self);
    // return 1;
  }
}

pub trait QOpenGLContext_nativeHandle<RetType> {
  fn nativeHandle(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  QVariant QOpenGLContext::nativeHandle();
impl<'a> /*trait*/ QOpenGLContext_nativeHandle<QVariant> for () {
  fn nativeHandle(self , rsthis: & QOpenGLContext) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext12nativeHandleEv()};
    let mut ret = unsafe {_ZNK14QOpenGLContext12nativeHandleEv(rsthis.qclsinst)};
    let mut ret1 = QVariant::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QOpenGLContext::aboutToBeDestroyed();
impl /*struct*/ QOpenGLContext {
  pub fn aboutToBeDestroyed<RetType, T: QOpenGLContext_aboutToBeDestroyed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.aboutToBeDestroyed(self);
    // return 1;
  }
}

pub trait QOpenGLContext_aboutToBeDestroyed<RetType> {
  fn aboutToBeDestroyed(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  void QOpenGLContext::aboutToBeDestroyed();
impl<'a> /*trait*/ QOpenGLContext_aboutToBeDestroyed<()> for () {
  fn aboutToBeDestroyed(self , rsthis: & QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext18aboutToBeDestroyedEv()};
     unsafe {_ZN14QOpenGLContext18aboutToBeDestroyedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLContext::isOpenGLES();
impl /*struct*/ QOpenGLContext {
  pub fn isOpenGLES<RetType, T: QOpenGLContext_isOpenGLES<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isOpenGLES(self);
    // return 1;
  }
}

pub trait QOpenGLContext_isOpenGLES<RetType> {
  fn isOpenGLES(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  bool QOpenGLContext::isOpenGLES();
impl<'a> /*trait*/ QOpenGLContext_isOpenGLES<i8> for () {
  fn isOpenGLES(self , rsthis: & QOpenGLContext) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext10isOpenGLESEv()};
    let mut ret = unsafe {_ZNK14QOpenGLContext10isOpenGLESEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPlatformOpenGLContext * QOpenGLContext::handle();
impl /*struct*/ QOpenGLContext {
  pub fn handle<RetType, T: QOpenGLContext_handle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.handle(self);
    // return 1;
  }
}

pub trait QOpenGLContext_handle<RetType> {
  fn handle(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  QPlatformOpenGLContext * QOpenGLContext::handle();
impl<'a> /*trait*/ QOpenGLContext_handle<()> for () {
  fn handle(self , rsthis: & QOpenGLContext) -> () {
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
  pub fn makeCurrent<RetType, T: QOpenGLContext_makeCurrent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.makeCurrent(self);
    // return 1;
  }
}

pub trait QOpenGLContext_makeCurrent<RetType> {
  fn makeCurrent(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  bool QOpenGLContext::makeCurrent(QSurface * surface);
impl<'a> /*trait*/ QOpenGLContext_makeCurrent<i8> for (&'a QSurface) {
  fn makeCurrent(self , rsthis: & QOpenGLContext) -> i8 {
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
  pub fn shareHandle<RetType, T: QOpenGLContext_shareHandle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shareHandle(self);
    // return 1;
  }
}

pub trait QOpenGLContext_shareHandle<RetType> {
  fn shareHandle(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  QPlatformOpenGLContext * QOpenGLContext::shareHandle();
impl<'a> /*trait*/ QOpenGLContext_shareHandle<()> for () {
  fn shareHandle(self , rsthis: & QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext11shareHandleEv()};
     unsafe {_ZNK14QOpenGLContext11shareHandleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLContext::create();
impl /*struct*/ QOpenGLContext {
  pub fn create<RetType, T: QOpenGLContext_create<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QOpenGLContext_create<RetType> {
  fn create(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  bool QOpenGLContext::create();
impl<'a> /*trait*/ QOpenGLContext_create<i8> for () {
  fn create(self , rsthis: & QOpenGLContext) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext6createEv()};
    let mut ret = unsafe {_ZN14QOpenGLContext6createEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QOpenGLContext * QOpenGLContext::shareContext();
impl /*struct*/ QOpenGLContext {
  pub fn shareContext<RetType, T: QOpenGLContext_shareContext<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shareContext(self);
    // return 1;
  }
}

pub trait QOpenGLContext_shareContext<RetType> {
  fn shareContext(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  QOpenGLContext * QOpenGLContext::shareContext();
impl<'a> /*trait*/ QOpenGLContext_shareContext<()> for () {
  fn shareContext(self , rsthis: & QOpenGLContext) -> () {
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
  pub fn defaultFramebufferObject<RetType, T: QOpenGLContext_defaultFramebufferObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultFramebufferObject(self);
    // return 1;
  }
}

pub trait QOpenGLContext_defaultFramebufferObject<RetType> {
  fn defaultFramebufferObject(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  GLuint QOpenGLContext::defaultFramebufferObject();
impl<'a> /*trait*/ QOpenGLContext_defaultFramebufferObject<()> for () {
  fn defaultFramebufferObject(self , rsthis: & QOpenGLContext) -> () {
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
  pub fn doneCurrent<RetType, T: QOpenGLContext_doneCurrent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.doneCurrent(self);
    // return 1;
  }
}

pub trait QOpenGLContext_doneCurrent<RetType> {
  fn doneCurrent(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  void QOpenGLContext::doneCurrent();
impl<'a> /*trait*/ QOpenGLContext_doneCurrent<()> for () {
  fn doneCurrent(self , rsthis: & QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext11doneCurrentEv()};
     unsafe {_ZN14QOpenGLContext11doneCurrentEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QOpenGLContextGroup * QOpenGLContext::shareGroup();
impl /*struct*/ QOpenGLContext {
  pub fn shareGroup<RetType, T: QOpenGLContext_shareGroup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shareGroup(self);
    // return 1;
  }
}

pub trait QOpenGLContext_shareGroup<RetType> {
  fn shareGroup(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  QOpenGLContextGroup * QOpenGLContext::shareGroup();
impl<'a> /*trait*/ QOpenGLContext_shareGroup<()> for () {
  fn shareGroup(self , rsthis: & QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext10shareGroupEv()};
     unsafe {_ZNK14QOpenGLContext10shareGroupEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSurfaceFormat QOpenGLContext::format();
impl /*struct*/ QOpenGLContext {
  pub fn format<RetType, T: QOpenGLContext_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QOpenGLContext_format<RetType> {
  fn format(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  QSurfaceFormat QOpenGLContext::format();
impl<'a> /*trait*/ QOpenGLContext_format<QSurfaceFormat> for () {
  fn format(self , rsthis: & QOpenGLContext) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext6formatEv()};
    let mut ret = unsafe {_ZNK14QOpenGLContext6formatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat::inheritFrom(ret);
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
  pub fn setNativeHandle<RetType, T: QOpenGLContext_setNativeHandle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNativeHandle(self);
    // return 1;
  }
}

pub trait QOpenGLContext_setNativeHandle<RetType> {
  fn setNativeHandle(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  void QOpenGLContext::setNativeHandle(const QVariant & handle);
impl<'a> /*trait*/ QOpenGLContext_setNativeHandle<()> for (&'a QVariant) {
  fn setNativeHandle(self , rsthis: & QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext15setNativeHandleERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLContext15setNativeHandleERK8QVariant(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QFunctionPointer QOpenGLContext::getProcAddress(const QByteArray & procName);
impl /*struct*/ QOpenGLContext {
  pub fn getProcAddress<RetType, T: QOpenGLContext_getProcAddress<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.getProcAddress(self);
    // return 1;
  }
}

pub trait QOpenGLContext_getProcAddress<RetType> {
  fn getProcAddress(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  QFunctionPointer QOpenGLContext::getProcAddress(const QByteArray & procName);
impl<'a> /*trait*/ QOpenGLContext_getProcAddress<()> for (&'a QByteArray) {
  fn getProcAddress(self , rsthis: & QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext14getProcAddressERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK14QOpenGLContext14getProcAddressERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLContext::swapBuffers(QSurface * surface);
impl /*struct*/ QOpenGLContext {
  pub fn swapBuffers<RetType, T: QOpenGLContext_swapBuffers<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swapBuffers(self);
    // return 1;
  }
}

pub trait QOpenGLContext_swapBuffers<RetType> {
  fn swapBuffers(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  void QOpenGLContext::swapBuffers(QSurface * surface);
impl<'a> /*trait*/ QOpenGLContext_swapBuffers<()> for (&'a QSurface) {
  fn swapBuffers(self , rsthis: & QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext11swapBuffersEP8QSurface()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QOpenGLContext11swapBuffersEP8QSurface(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContextGroup {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLContextGroup {
    return QOpenGLContextGroup{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLContextGroup {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QOpenGLContextGroup {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QOpenGLContextGroup::metaObject();
impl /*struct*/ QOpenGLContextGroup {
  pub fn metaObject<RetType, T: QOpenGLContextGroup_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLContextGroup_metaObject<RetType> {
  fn metaObject(self , rsthis: & QOpenGLContextGroup) -> RetType;
}

  // proto:  const QMetaObject * QOpenGLContextGroup::metaObject();
impl<'a> /*trait*/ QOpenGLContextGroup_metaObject<()> for () {
  fn metaObject(self , rsthis: & QOpenGLContextGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QOpenGLContextGroup10metaObjectEv()};
     unsafe {_ZNK19QOpenGLContextGroup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QOpenGLContextGroup * QOpenGLContextGroup::currentContextGroup();
impl /*struct*/ QOpenGLContextGroup {
  pub fn currentContextGroup_s<RetType, T: QOpenGLContextGroup_currentContextGroup_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentContextGroup_s();
    // return 1;
  }
}

pub trait QOpenGLContextGroup_currentContextGroup_s<RetType> {
  fn currentContextGroup_s(self ) -> RetType;
}

  // proto: static QOpenGLContextGroup * QOpenGLContextGroup::currentContextGroup();
impl<'a> /*trait*/ QOpenGLContextGroup_currentContextGroup_s<()> for () {
  fn currentContextGroup_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLContextGroup19currentContextGroupEv()};
     unsafe {_ZN19QOpenGLContextGroup19currentContextGroupEv()};
    // return 1;
  }
}

  // proto:  void QOpenGLContextGroup::~QOpenGLContextGroup();
impl /*struct*/ QOpenGLContextGroup {
  pub fn Free<RetType, T: QOpenGLContextGroup_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QOpenGLContextGroup_Free<RetType> {
  fn Free(self , rsthis: & QOpenGLContextGroup) -> RetType;
}

  // proto:  void QOpenGLContextGroup::~QOpenGLContextGroup();
impl<'a> /*trait*/ QOpenGLContextGroup_Free<()> for () {
  fn Free(self , rsthis: & QOpenGLContextGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLContextGroupD0Ev()};
     unsafe {_ZN19QOpenGLContextGroupD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLContextGroup::QOpenGLContextGroup();
impl /*struct*/ QOpenGLContextGroup {
  pub fn New<T: QOpenGLContextGroup_New>(value: T) -> QOpenGLContextGroup {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLContextGroup_New {
  fn New(self) -> QOpenGLContextGroup;
}

  // proto:  void QOpenGLContextGroup::QOpenGLContextGroup();
impl<'a> /*trait*/ QOpenGLContextGroup_New for () {
  fn New(self) -> QOpenGLContextGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLContextGroupC1Ev()};
    unsafe {_ZN19QOpenGLContextGroupC1Ev(qthis)};
    let rsthis = QOpenGLContextGroup{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QList<QOpenGLContext *> QOpenGLContextGroup::shares();
impl /*struct*/ QOpenGLContextGroup {
  pub fn shares<RetType, T: QOpenGLContextGroup_shares<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shares(self);
    // return 1;
  }
}

pub trait QOpenGLContextGroup_shares<RetType> {
  fn shares(self , rsthis: & QOpenGLContextGroup) -> RetType;
}

  // proto:  QList<QOpenGLContext *> QOpenGLContextGroup::shares();
impl<'a> /*trait*/ QOpenGLContextGroup_shares<()> for () {
  fn shares(self , rsthis: & QOpenGLContextGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QOpenGLContextGroup6sharesEv()};
     unsafe {_ZNK19QOpenGLContextGroup6sharesEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

