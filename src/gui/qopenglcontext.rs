// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
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
use super::qsurfaceformat::*; // 773
// use super::qpair::*; // 775
use super::super::core::qobject::*; // 771
use super::qscreen::*; // 773
use super::qopenglfunctions::*; // 773
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qbytearray::*; // 771
// use super::qset::*; // 775
use super::qsurface::*; // 773
// use super::qopenglcontext::QOpenGLVersionProfile; // 773
use super::qopenglversionfunctions::*; // 773
use super::super::core::qvariant::*; // 771
// use super::qplatformopenglcontext::*; // 775
// use super::qopenglcontext::QOpenGLContextGroup; // 773
// use super::qfunctionpointer::*; // 775
// use super::qlist::*; // 775
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QOpenGLVersionProfile_Class_Size() -> c_int;
  // proto:  bool QOpenGLVersionProfile::isLegacyVersion();
  fn C_ZNK21QOpenGLVersionProfile15isLegacyVersionEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QOpenGLVersionProfile::~QOpenGLVersionProfile();
  fn C_ZN21QOpenGLVersionProfileD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QOpenGLVersionProfile::hasProfiles();
  fn C_ZNK21QOpenGLVersionProfile11hasProfilesEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QOpenGLVersionProfile::QOpenGLVersionProfile(const QSurfaceFormat & format);
  fn C_ZN21QOpenGLVersionProfileC2ERK14QSurfaceFormat(arg0: *mut c_void) -> u64;
  // proto:  void QOpenGLVersionProfile::QOpenGLVersionProfile(const QOpenGLVersionProfile & other);
  fn C_ZN21QOpenGLVersionProfileC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QOpenGLVersionProfile::QOpenGLVersionProfile();
  fn C_ZN21QOpenGLVersionProfileC2Ev() -> u64;
  // proto:  QPair<int, int> QOpenGLVersionProfile::version();
  fn C_ZNK21QOpenGLVersionProfile7versionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QOpenGLVersionProfile::setVersion(int majorVersion, int minorVersion);
  fn C_ZN21QOpenGLVersionProfile10setVersionEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  bool QOpenGLVersionProfile::isValid();
  fn C_ZNK21QOpenGLVersionProfile7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QOpenGLContext_Class_Size() -> c_int;
  // proto:  bool QOpenGLContext::isValid();
  fn C_ZNK14QOpenGLContext7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QOpenGLContext::setScreen(QScreen * screen);
  fn C_ZN14QOpenGLContext9setScreenEP7QScreen(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QOpenGLContext::QOpenGLContext(QObject * parent);
  fn C_ZN14QOpenGLContextC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  QOpenGLFunctions * QOpenGLContext::functions();
  fn C_ZNK14QOpenGLContext9functionsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QOpenGLContext::~QOpenGLContext();
  fn C_ZN14QOpenGLContextD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QOpenGLContext::setFormat(const QSurfaceFormat & format);
  fn C_ZN14QOpenGLContext9setFormatERK14QSurfaceFormat(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QOpenGLContext::metaObject();
  fn C_ZNK14QOpenGLContext10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QOpenGLContext::hasExtension(const QByteArray & extension);
  fn C_ZNK14QOpenGLContext12hasExtensionERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QSet<QByteArray> QOpenGLContext::extensions();
  fn C_ZNK14QOpenGLContext10extensionsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSurface * QOpenGLContext::surface();
  fn C_ZNK14QOpenGLContext7surfaceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QAbstractOpenGLFunctions * QOpenGLContext::versionFunctions(const QOpenGLVersionProfile & versionProfile);
  fn C_ZNK14QOpenGLContext16versionFunctionsERK21QOpenGLVersionProfile(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QOpenGLContext::setShareContext(QOpenGLContext * shareContext);
  fn C_ZN14QOpenGLContext15setShareContextEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static bool QOpenGLContext::areSharing(QOpenGLContext * first, QOpenGLContext * second);
  fn C_ZN14QOpenGLContext10areSharingEPS_S0_(arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  QScreen * QOpenGLContext::screen();
  fn C_ZNK14QOpenGLContext6screenEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QVariant QOpenGLContext::nativeHandle();
  fn C_ZNK14QOpenGLContext12nativeHandleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QOpenGLContext::isOpenGLES();
  fn C_ZNK14QOpenGLContext10isOpenGLESEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QPlatformOpenGLContext * QOpenGLContext::handle();
  fn C_ZNK14QOpenGLContext6handleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QOpenGLContext * QOpenGLContext::globalShareContext();
  fn C_ZN14QOpenGLContext18globalShareContextEv() -> *mut c_void;
  // proto:  bool QOpenGLContext::makeCurrent(QSurface * surface);
  fn C_ZN14QOpenGLContext11makeCurrentEP8QSurface(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QPlatformOpenGLContext * QOpenGLContext::shareHandle();
  fn C_ZNK14QOpenGLContext11shareHandleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QOpenGLContext::create();
  fn C_ZN14QOpenGLContext6createEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QOpenGLContext * QOpenGLContext::shareContext();
  fn C_ZNK14QOpenGLContext12shareContextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QOpenGLContext * QOpenGLContext::currentContext();
  fn C_ZN14QOpenGLContext14currentContextEv() -> *mut c_void;
  // proto:  GLuint QOpenGLContext::defaultFramebufferObject();
  fn C_ZNK14QOpenGLContext24defaultFramebufferObjectEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto: static bool QOpenGLContext::supportsThreadedOpenGL();
  fn C_ZN14QOpenGLContext22supportsThreadedOpenGLEv() -> c_char;
  // proto:  void QOpenGLContext::doneCurrent();
  fn C_ZN14QOpenGLContext11doneCurrentEv(qthis: u64 /* *mut c_void*/);
  // proto:  QOpenGLContextGroup * QOpenGLContext::shareGroup();
  fn C_ZNK14QOpenGLContext10shareGroupEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSurfaceFormat QOpenGLContext::format();
  fn C_ZNK14QOpenGLContext6formatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static void * QOpenGLContext::openGLModuleHandle();
  fn C_ZN14QOpenGLContext18openGLModuleHandleEv() -> *mut c_void;
  // proto:  void QOpenGLContext::setNativeHandle(const QVariant & handle);
  fn C_ZN14QOpenGLContext15setNativeHandleERK8QVariant(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QFunctionPointer QOpenGLContext::getProcAddress(const QByteArray & procName);
  fn C_ZNK14QOpenGLContext14getProcAddressERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QOpenGLContext::swapBuffers(QSurface * surface);
  fn C_ZN14QOpenGLContext11swapBuffersEP8QSurface(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QOpenGLContextGroup_Class_Size() -> c_int;
  // proto:  const QMetaObject * QOpenGLContextGroup::metaObject();
  fn C_ZNK19QOpenGLContextGroup10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QOpenGLContextGroup * QOpenGLContextGroup::currentContextGroup();
  fn C_ZN19QOpenGLContextGroup19currentContextGroupEv() -> *mut c_void;
  // proto:  void QOpenGLContextGroup::~QOpenGLContextGroup();
  fn C_ZN19QOpenGLContextGroupD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QList<QOpenGLContext *> QOpenGLContextGroup::shares();
  fn C_ZNK19QOpenGLContextGroup6sharesEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QOpenGLContext_SlotProxy_connect__ZN14QOpenGLContext18aboutToBeDestroyedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QOpenGLVersionProfile)=8
#[derive(Default)]
pub struct QOpenGLVersionProfile {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QOpenGLContext)=1
#[derive(Default)]
pub struct QOpenGLContext {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _aboutToBeDestroyed: QOpenGLContext_aboutToBeDestroyed_signal,
}

// class sizeof(QOpenGLContextGroup)=1
#[derive(Default)]
pub struct QOpenGLContextGroup {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QOpenGLVersionProfile {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLVersionProfile {
    return QOpenGLVersionProfile{qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK21QOpenGLVersionProfile15isLegacyVersionEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QOpenGLVersionProfile::~QOpenGLVersionProfile();
impl /*struct*/ QOpenGLVersionProfile {
  pub fn free<RetType, T: QOpenGLVersionProfile_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_free<RetType> {
  fn free(self , rsthis: & QOpenGLVersionProfile) -> RetType;
}

  // proto:  void QOpenGLVersionProfile::~QOpenGLVersionProfile();
impl<'a> /*trait*/ QOpenGLVersionProfile_free<()> for () {
  fn free(self , rsthis: & QOpenGLVersionProfile) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QOpenGLVersionProfileD2Ev()};
     unsafe {C_ZN21QOpenGLVersionProfileD2Ev(rsthis.qclsinst)};
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
    let mut ret = unsafe {C_ZNK21QOpenGLVersionProfile11hasProfilesEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QOpenGLVersionProfile::QOpenGLVersionProfile(const QSurfaceFormat & format);
impl /*struct*/ QOpenGLVersionProfile {
  pub fn new<T: QOpenGLVersionProfile_new>(value: T) -> QOpenGLVersionProfile {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLVersionProfile_new {
  fn new(self) -> QOpenGLVersionProfile;
}

  // proto:  void QOpenGLVersionProfile::QOpenGLVersionProfile(const QSurfaceFormat & format);
impl<'a> /*trait*/ QOpenGLVersionProfile_new for (&'a QSurfaceFormat) {
  fn new(self) -> QOpenGLVersionProfile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QOpenGLVersionProfileC2ERK14QSurfaceFormat()};
    let ctysz: c_int = unsafe{QOpenGLVersionProfile_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN21QOpenGLVersionProfileC2ERK14QSurfaceFormat(arg0)};
    let rsthis = QOpenGLVersionProfile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLVersionProfile::QOpenGLVersionProfile(const QOpenGLVersionProfile & other);
impl<'a> /*trait*/ QOpenGLVersionProfile_new for (&'a QOpenGLVersionProfile) {
  fn new(self) -> QOpenGLVersionProfile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QOpenGLVersionProfileC2ERKS_()};
    let ctysz: c_int = unsafe{QOpenGLVersionProfile_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN21QOpenGLVersionProfileC2ERKS_(arg0)};
    let rsthis = QOpenGLVersionProfile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLVersionProfile::QOpenGLVersionProfile();
impl<'a> /*trait*/ QOpenGLVersionProfile_new for () {
  fn new(self) -> QOpenGLVersionProfile {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QOpenGLVersionProfileC2Ev()};
    let ctysz: c_int = unsafe{QOpenGLVersionProfile_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN21QOpenGLVersionProfileC2Ev()};
    let rsthis = QOpenGLVersionProfile{qclsinst: qthis, ..Default::default()};
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
impl<'a> /*trait*/ QOpenGLVersionProfile_version<u64> for () {
  fn version(self , rsthis: & QOpenGLVersionProfile) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QOpenGLVersionProfile7versionEv()};
    let mut ret = unsafe {C_ZNK21QOpenGLVersionProfile7versionEv(rsthis.qclsinst)};
    return ret as u64; // 5
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
     unsafe {C_ZN21QOpenGLVersionProfile10setVersionEii(rsthis.qclsinst, arg0, arg1)};
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
    let mut ret = unsafe {C_ZNK21QOpenGLVersionProfile7isValidEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

impl /*struct*/ QOpenGLContext {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLContext {
    return QOpenGLContext{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK14QOpenGLContext7isValidEv(rsthis.qclsinst)};
    return ret as i8; // 1
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
     unsafe {C_ZN14QOpenGLContext9setScreenEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QOpenGLContext::QOpenGLContext(QObject * parent);
impl /*struct*/ QOpenGLContext {
  pub fn new<T: QOpenGLContext_new>(value: T) -> QOpenGLContext {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLContext_new {
  fn new(self) -> QOpenGLContext;
}

  // proto:  void QOpenGLContext::QOpenGLContext(QObject * parent);
impl<'a> /*trait*/ QOpenGLContext_new for (&'a QObject) {
  fn new(self) -> QOpenGLContext {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContextC2EP7QObject()};
    let ctysz: c_int = unsafe{QOpenGLContext_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN14QOpenGLContextC2EP7QObject(arg0)};
    let rsthis = QOpenGLContext{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
impl<'a> /*trait*/ QOpenGLContext_functions<QOpenGLFunctions> for () {
  fn functions(self , rsthis: & QOpenGLContext) -> QOpenGLFunctions {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext9functionsEv()};
    let mut ret = unsafe {C_ZNK14QOpenGLContext9functionsEv(rsthis.qclsinst)};
    let mut ret1 = QOpenGLFunctions::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QOpenGLContext::~QOpenGLContext();
impl /*struct*/ QOpenGLContext {
  pub fn free<RetType, T: QOpenGLContext_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QOpenGLContext_free<RetType> {
  fn free(self , rsthis: & QOpenGLContext) -> RetType;
}

  // proto:  void QOpenGLContext::~QOpenGLContext();
impl<'a> /*trait*/ QOpenGLContext_free<()> for () {
  fn free(self , rsthis: & QOpenGLContext) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContextD2Ev()};
     unsafe {C_ZN14QOpenGLContextD2Ev(rsthis.qclsinst)};
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
     unsafe {C_ZN14QOpenGLContext9setFormatERK14QSurfaceFormat(rsthis.qclsinst, arg0)};
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
impl<'a> /*trait*/ QOpenGLContext_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QOpenGLContext) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext10metaObjectEv()};
    let mut ret = unsafe {C_ZNK14QOpenGLContext10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
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
    let mut ret = unsafe {C_ZNK14QOpenGLContext12hasExtensionERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
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
impl<'a> /*trait*/ QOpenGLContext_extensions<u64> for () {
  fn extensions(self , rsthis: & QOpenGLContext) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext10extensionsEv()};
    let mut ret = unsafe {C_ZNK14QOpenGLContext10extensionsEv(rsthis.qclsinst)};
    return ret as u64; // 5
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
    let mut ret = unsafe {C_ZNK14QOpenGLContext7surfaceEv(rsthis.qclsinst)};
    let mut ret1 = QSurface::inheritFrom(ret as u64);
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
impl<'a> /*trait*/ QOpenGLContext_versionFunctions<QAbstractOpenGLFunctions> for (&'a QOpenGLVersionProfile) {
  fn versionFunctions(self , rsthis: & QOpenGLContext) -> QAbstractOpenGLFunctions {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext16versionFunctionsERK21QOpenGLVersionProfile()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK14QOpenGLContext16versionFunctionsERK21QOpenGLVersionProfile(rsthis.qclsinst, arg0)};
    let mut ret1 = QAbstractOpenGLFunctions::inheritFrom(ret as u64);
    return ret1;
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
     unsafe {C_ZN14QOpenGLContext15setShareContextEPS_(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZN14QOpenGLContext10areSharingEPS_S0_(arg0, arg1)};
    return ret as i8; // 1
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
    let mut ret = unsafe {C_ZNK14QOpenGLContext6screenEv(rsthis.qclsinst)};
    let mut ret1 = QScreen::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK14QOpenGLContext12nativeHandleEv(rsthis.qclsinst)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
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
    let mut ret = unsafe {C_ZNK14QOpenGLContext10isOpenGLESEv(rsthis.qclsinst)};
    return ret as i8; // 1
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
impl<'a> /*trait*/ QOpenGLContext_handle<u64> for () {
  fn handle(self , rsthis: & QOpenGLContext) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext6handleEv()};
    let mut ret = unsafe {C_ZNK14QOpenGLContext6handleEv(rsthis.qclsinst)};
    return ret as u64; // 4
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
impl<'a> /*trait*/ QOpenGLContext_globalShareContext_s<QOpenGLContext> for () {
  fn globalShareContext_s(self ) -> QOpenGLContext {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext18globalShareContextEv()};
    let mut ret = unsafe {C_ZN14QOpenGLContext18globalShareContextEv()};
    let mut ret1 = QOpenGLContext::inheritFrom(ret as u64);
    return ret1;
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
    let mut ret = unsafe {C_ZN14QOpenGLContext11makeCurrentEP8QSurface(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
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
impl<'a> /*trait*/ QOpenGLContext_shareHandle<u64> for () {
  fn shareHandle(self , rsthis: & QOpenGLContext) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext11shareHandleEv()};
    let mut ret = unsafe {C_ZNK14QOpenGLContext11shareHandleEv(rsthis.qclsinst)};
    return ret as u64; // 4
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
    let mut ret = unsafe {C_ZN14QOpenGLContext6createEv(rsthis.qclsinst)};
    return ret as i8; // 1
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
impl<'a> /*trait*/ QOpenGLContext_shareContext<QOpenGLContext> for () {
  fn shareContext(self , rsthis: & QOpenGLContext) -> QOpenGLContext {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext12shareContextEv()};
    let mut ret = unsafe {C_ZNK14QOpenGLContext12shareContextEv(rsthis.qclsinst)};
    let mut ret1 = QOpenGLContext::inheritFrom(ret as u64);
    return ret1;
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
impl<'a> /*trait*/ QOpenGLContext_currentContext_s<QOpenGLContext> for () {
  fn currentContext_s(self ) -> QOpenGLContext {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QOpenGLContext14currentContextEv()};
    let mut ret = unsafe {C_ZN14QOpenGLContext14currentContextEv()};
    let mut ret1 = QOpenGLContext::inheritFrom(ret as u64);
    return ret1;
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
impl<'a> /*trait*/ QOpenGLContext_defaultFramebufferObject<u32> for () {
  fn defaultFramebufferObject(self , rsthis: & QOpenGLContext) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext24defaultFramebufferObjectEv()};
    let mut ret = unsafe {C_ZNK14QOpenGLContext24defaultFramebufferObjectEv(rsthis.qclsinst)};
    return ret as u32; // 1
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
    let mut ret = unsafe {C_ZN14QOpenGLContext22supportsThreadedOpenGLEv()};
    return ret as i8; // 1
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
     unsafe {C_ZN14QOpenGLContext11doneCurrentEv(rsthis.qclsinst)};
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
impl<'a> /*trait*/ QOpenGLContext_shareGroup<QOpenGLContextGroup> for () {
  fn shareGroup(self , rsthis: & QOpenGLContext) -> QOpenGLContextGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext10shareGroupEv()};
    let mut ret = unsafe {C_ZNK14QOpenGLContext10shareGroupEv(rsthis.qclsinst)};
    let mut ret1 = QOpenGLContextGroup::inheritFrom(ret as u64);
    return ret1;
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
    let mut ret = unsafe {C_ZNK14QOpenGLContext6formatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZN14QOpenGLContext18openGLModuleHandleEv()};
    return ret as *mut c_void; // 1
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
     unsafe {C_ZN14QOpenGLContext15setNativeHandleERK8QVariant(rsthis.qclsinst, arg0)};
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
impl<'a> /*trait*/ QOpenGLContext_getProcAddress<u64> for (&'a QByteArray) {
  fn getProcAddress(self , rsthis: & QOpenGLContext) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QOpenGLContext14getProcAddressERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK14QOpenGLContext14getProcAddressERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as u64; // 3
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
     unsafe {C_ZN14QOpenGLContext11swapBuffersEP8QSurface(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContextGroup {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLContextGroup {
    return QOpenGLContextGroup{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
impl<'a> /*trait*/ QOpenGLContextGroup_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QOpenGLContextGroup) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QOpenGLContextGroup10metaObjectEv()};
    let mut ret = unsafe {C_ZNK19QOpenGLContextGroup10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
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
impl<'a> /*trait*/ QOpenGLContextGroup_currentContextGroup_s<QOpenGLContextGroup> for () {
  fn currentContextGroup_s(self ) -> QOpenGLContextGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLContextGroup19currentContextGroupEv()};
    let mut ret = unsafe {C_ZN19QOpenGLContextGroup19currentContextGroupEv()};
    let mut ret1 = QOpenGLContextGroup::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QOpenGLContextGroup::~QOpenGLContextGroup();
impl /*struct*/ QOpenGLContextGroup {
  pub fn free<RetType, T: QOpenGLContextGroup_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QOpenGLContextGroup_free<RetType> {
  fn free(self , rsthis: & QOpenGLContextGroup) -> RetType;
}

  // proto:  void QOpenGLContextGroup::~QOpenGLContextGroup();
impl<'a> /*trait*/ QOpenGLContextGroup_free<()> for () {
  fn free(self , rsthis: & QOpenGLContextGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLContextGroupD2Ev()};
     unsafe {C_ZN19QOpenGLContextGroupD2Ev(rsthis.qclsinst)};
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
impl<'a> /*trait*/ QOpenGLContextGroup_shares<u64> for () {
  fn shares(self , rsthis: & QOpenGLContextGroup) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QOpenGLContextGroup6sharesEv()};
    let mut ret = unsafe {C_ZNK19QOpenGLContextGroup6sharesEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

#[derive(Default)] // for QOpenGLContext_aboutToBeDestroyed
pub struct QOpenGLContext_aboutToBeDestroyed_signal{poi:u64}
impl /* struct */ QOpenGLContext {
  pub fn aboutToBeDestroyed(&self) -> QOpenGLContext_aboutToBeDestroyed_signal {
     return QOpenGLContext_aboutToBeDestroyed_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QOpenGLContext_aboutToBeDestroyed_signal {
  pub fn connect<T: QOpenGLContext_aboutToBeDestroyed_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QOpenGLContext_aboutToBeDestroyed_signal_connect {
  fn connect(self, sigthis: QOpenGLContext_aboutToBeDestroyed_signal);
}

// aboutToBeDestroyed()
extern fn QOpenGLContext_aboutToBeDestroyed_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QOpenGLContext_aboutToBeDestroyed_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QOpenGLContext_aboutToBeDestroyed_signal_connect for fn() {
  fn connect(self, sigthis: QOpenGLContext_aboutToBeDestroyed_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QOpenGLContext_aboutToBeDestroyed_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QOpenGLContext_SlotProxy_connect__ZN14QOpenGLContext18aboutToBeDestroyedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QOpenGLContext_aboutToBeDestroyed_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QOpenGLContext_aboutToBeDestroyed_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QOpenGLContext_aboutToBeDestroyed_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QOpenGLContext_SlotProxy_connect__ZN14QOpenGLContext18aboutToBeDestroyedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

