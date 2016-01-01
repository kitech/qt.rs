// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
// src-file: /QtCore/qlibrary.h
// dst-file: /src/core/qlibrary.rs
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
use super::qobject::QObject; // 773
use std::ops::Deref;
use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QLibrary_Class_Size() -> c_int;
  // proto: static QFunctionPointer QLibrary::resolve(const QString & fileName, const QString & version, const char * symbol);
  fn _ZN8QLibrary7resolveERK7QStringS2_PKc(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_char);
  // proto:  void QLibrary::QLibrary(const QString & fileName, const QString & version, QObject * parent);
  fn dector_ZN8QLibraryC1ERK7QStringS2_P7QObject(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  fn _ZN8QLibraryC1ERK7QStringS2_P7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  const QMetaObject * QLibrary::metaObject();
  fn _ZNK8QLibrary10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto: static QFunctionPointer QLibrary::resolve(const QString & fileName, const char * symbol);
  fn _ZN8QLibrary7resolveERK7QStringPKc(arg0: *mut c_void, arg1: *mut c_char);
  // proto:  void QLibrary::QLibrary(QObject * parent);
  fn dector_ZN8QLibraryC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN8QLibraryC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QLibrary::isLoaded();
  fn _ZNK8QLibrary8isLoadedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QLibrary::load();
  fn _ZN8QLibrary4loadEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QLibrary::QLibrary(const QString & fileName, QObject * parent);
  fn dector_ZN8QLibraryC1ERK7QStringP7QObject(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN8QLibraryC1ERK7QStringP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QString QLibrary::fileName();
  fn _ZNK8QLibrary8fileNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QLibrary::setFileName(const QString & fileName);
  fn _ZN8QLibrary11setFileNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QLibrary::QLibrary(const QLibrary & );
  fn dector_ZN8QLibraryC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN8QLibraryC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static bool QLibrary::isLibrary(const QString & fileName);
  fn _ZN8QLibrary9isLibraryERK7QString(arg0: *mut c_void) -> c_char;
  // proto:  bool QLibrary::unload();
  fn _ZN8QLibrary6unloadEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QFunctionPointer QLibrary::resolve(const char * symbol);
  fn _ZN8QLibrary7resolveEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char);
  // proto:  void QLibrary::setFileNameAndVersion(const QString & fileName, const QString & version);
  fn _ZN8QLibrary21setFileNameAndVersionERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QLibrary::setFileNameAndVersion(const QString & fileName, int verNum);
  fn _ZN8QLibrary21setFileNameAndVersionERK7QStringi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  void QLibrary::QLibrary(const QString & fileName, int verNum, QObject * parent);
  fn dector_ZN8QLibraryC1ERK7QStringiP7QObject(arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  fn _ZN8QLibraryC1ERK7QStringiP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void);
  // proto:  void QLibrary::~QLibrary();
  fn _ZN8QLibraryD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QString QLibrary::errorString();
  fn _ZNK8QLibrary11errorStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QFunctionPointer QLibrary::resolve(const QString & fileName, int verNum, const char * symbol);
  fn _ZN8QLibrary7resolveERK7QStringiPKc(arg0: *mut c_void, arg1: c_int, arg2: *mut c_char);
} // <= ext block end

// body block begin =>
// class sizeof(QLibrary)=1
#[derive(Default)]
pub struct QLibrary {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QLibrary {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QLibrary {
    return QLibrary{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QLibrary {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QLibrary {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto: static QFunctionPointer QLibrary::resolve(const QString & fileName, const QString & version, const char * symbol);
impl /*struct*/ QLibrary {
  pub fn resolve_s<RetType, T: QLibrary_resolve_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.resolve_s();
    // return 1;
  }
}

pub trait QLibrary_resolve_s<RetType> {
  fn resolve_s(self ) -> RetType;
}

  // proto: static QFunctionPointer QLibrary::resolve(const QString & fileName, const QString & version, const char * symbol);
impl<'a> /*trait*/ QLibrary_resolve_s<()> for (&'a QString, &'a QString, &'a  String) {
  fn resolve_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary7resolveERK7QStringS2_PKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.as_ptr()  as *mut c_char;
     unsafe {_ZN8QLibrary7resolveERK7QStringS2_PKc(arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QLibrary::QLibrary(const QString & fileName, const QString & version, QObject * parent);
impl /*struct*/ QLibrary {
  pub fn new<T: QLibrary_new>(value: T) -> QLibrary {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QLibrary_new {
  fn new(self) -> QLibrary;
}

  // proto:  void QLibrary::QLibrary(const QString & fileName, const QString & version, QObject * parent);
impl<'a> /*trait*/ QLibrary_new for (&'a QString, &'a QString, &'a QObject) {
  fn new(self) -> QLibrary {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibraryC1ERK7QStringS2_P7QObject()};
    let ctysz: c_int = unsafe{QLibrary_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    // unsafe {_ZN8QLibraryC1ERK7QStringS2_P7QObject(qthis, arg0, arg1, arg2)};
    let qthis: u64 = unsafe {dector_ZN8QLibraryC1ERK7QStringS2_P7QObject(arg0, arg1, arg2)} as u64;
    let rsthis = QLibrary{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QLibrary::metaObject();
impl /*struct*/ QLibrary {
  pub fn metaObject<RetType, T: QLibrary_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QLibrary_metaObject<RetType> {
  fn metaObject(self , rsthis: & QLibrary) -> RetType;
}

  // proto:  const QMetaObject * QLibrary::metaObject();
impl<'a> /*trait*/ QLibrary_metaObject<()> for () {
  fn metaObject(self , rsthis: & QLibrary) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QLibrary10metaObjectEv()};
     unsafe {_ZNK8QLibrary10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static QFunctionPointer QLibrary::resolve(const QString & fileName, const char * symbol);
impl<'a> /*trait*/ QLibrary_resolve_s<()> for (&'a QString, &'a  String) {
  fn resolve_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary7resolveERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {_ZN8QLibrary7resolveERK7QStringPKc(arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QLibrary::QLibrary(QObject * parent);
impl<'a> /*trait*/ QLibrary_new for (&'a QObject) {
  fn new(self) -> QLibrary {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibraryC1EP7QObject()};
    let ctysz: c_int = unsafe{QLibrary_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN8QLibraryC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN8QLibraryC1EP7QObject(arg0)} as u64;
    let rsthis = QLibrary{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QLibrary::isLoaded();
impl /*struct*/ QLibrary {
  pub fn isLoaded<RetType, T: QLibrary_isLoaded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLoaded(self);
    // return 1;
  }
}

pub trait QLibrary_isLoaded<RetType> {
  fn isLoaded(self , rsthis: & QLibrary) -> RetType;
}

  // proto:  bool QLibrary::isLoaded();
impl<'a> /*trait*/ QLibrary_isLoaded<i8> for () {
  fn isLoaded(self , rsthis: & QLibrary) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QLibrary8isLoadedEv()};
    let mut ret = unsafe {_ZNK8QLibrary8isLoadedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QLibrary::load();
impl /*struct*/ QLibrary {
  pub fn load<RetType, T: QLibrary_load<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.load(self);
    // return 1;
  }
}

pub trait QLibrary_load<RetType> {
  fn load(self , rsthis: & QLibrary) -> RetType;
}

  // proto:  bool QLibrary::load();
impl<'a> /*trait*/ QLibrary_load<i8> for () {
  fn load(self , rsthis: & QLibrary) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary4loadEv()};
    let mut ret = unsafe {_ZN8QLibrary4loadEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLibrary::QLibrary(const QString & fileName, QObject * parent);
impl<'a> /*trait*/ QLibrary_new for (&'a QString, &'a QObject) {
  fn new(self) -> QLibrary {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibraryC1ERK7QStringP7QObject()};
    let ctysz: c_int = unsafe{QLibrary_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN8QLibraryC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN8QLibraryC1ERK7QStringP7QObject(arg0, arg1)} as u64;
    let rsthis = QLibrary{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QLibrary::fileName();
impl /*struct*/ QLibrary {
  pub fn fileName<RetType, T: QLibrary_fileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QLibrary_fileName<RetType> {
  fn fileName(self , rsthis: & QLibrary) -> RetType;
}

  // proto:  QString QLibrary::fileName();
impl<'a> /*trait*/ QLibrary_fileName<QString> for () {
  fn fileName(self , rsthis: & QLibrary) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QLibrary8fileNameEv()};
    let mut ret = unsafe {_ZNK8QLibrary8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLibrary::setFileName(const QString & fileName);
impl /*struct*/ QLibrary {
  pub fn setFileName<RetType, T: QLibrary_setFileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFileName(self);
    // return 1;
  }
}

pub trait QLibrary_setFileName<RetType> {
  fn setFileName(self , rsthis: & QLibrary) -> RetType;
}

  // proto:  void QLibrary::setFileName(const QString & fileName);
impl<'a> /*trait*/ QLibrary_setFileName<()> for (&'a QString) {
  fn setFileName(self , rsthis: & QLibrary) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QLibrary11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLibrary::QLibrary(const QLibrary & );
impl<'a> /*trait*/ QLibrary_new for (&'a QLibrary) {
  fn new(self) -> QLibrary {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibraryC1ERKS_()};
    let ctysz: c_int = unsafe{QLibrary_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN8QLibraryC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN8QLibraryC1ERKS_(arg0)} as u64;
    let rsthis = QLibrary{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static bool QLibrary::isLibrary(const QString & fileName);
impl /*struct*/ QLibrary {
  pub fn isLibrary_s<RetType, T: QLibrary_isLibrary_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isLibrary_s();
    // return 1;
  }
}

pub trait QLibrary_isLibrary_s<RetType> {
  fn isLibrary_s(self ) -> RetType;
}

  // proto: static bool QLibrary::isLibrary(const QString & fileName);
impl<'a> /*trait*/ QLibrary_isLibrary_s<i8> for (&'a QString) {
  fn isLibrary_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary9isLibraryERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QLibrary9isLibraryERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QLibrary::unload();
impl /*struct*/ QLibrary {
  pub fn unload<RetType, T: QLibrary_unload<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unload(self);
    // return 1;
  }
}

pub trait QLibrary_unload<RetType> {
  fn unload(self , rsthis: & QLibrary) -> RetType;
}

  // proto:  bool QLibrary::unload();
impl<'a> /*trait*/ QLibrary_unload<i8> for () {
  fn unload(self , rsthis: & QLibrary) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary6unloadEv()};
    let mut ret = unsafe {_ZN8QLibrary6unloadEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QFunctionPointer QLibrary::resolve(const char * symbol);
impl /*struct*/ QLibrary {
  pub fn resolve<RetType, T: QLibrary_resolve<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resolve(self);
    // return 1;
  }
}

pub trait QLibrary_resolve<RetType> {
  fn resolve(self , rsthis: & QLibrary) -> RetType;
}

  // proto:  QFunctionPointer QLibrary::resolve(const char * symbol);
impl<'a> /*trait*/ QLibrary_resolve<()> for (&'a  String) {
  fn resolve(self , rsthis: & QLibrary) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary7resolveEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
     unsafe {_ZN8QLibrary7resolveEPKc(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLibrary::setFileNameAndVersion(const QString & fileName, const QString & version);
impl /*struct*/ QLibrary {
  pub fn setFileNameAndVersion<RetType, T: QLibrary_setFileNameAndVersion<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFileNameAndVersion(self);
    // return 1;
  }
}

pub trait QLibrary_setFileNameAndVersion<RetType> {
  fn setFileNameAndVersion(self , rsthis: & QLibrary) -> RetType;
}

  // proto:  void QLibrary::setFileNameAndVersion(const QString & fileName, const QString & version);
impl<'a> /*trait*/ QLibrary_setFileNameAndVersion<()> for (&'a QString, &'a QString) {
  fn setFileNameAndVersion(self , rsthis: & QLibrary) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary21setFileNameAndVersionERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QLibrary21setFileNameAndVersionERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QLibrary::setFileNameAndVersion(const QString & fileName, int verNum);
impl<'a> /*trait*/ QLibrary_setFileNameAndVersion<()> for (&'a QString, i32) {
  fn setFileNameAndVersion(self , rsthis: & QLibrary) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary21setFileNameAndVersionERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QLibrary21setFileNameAndVersionERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QLibrary::QLibrary(const QString & fileName, int verNum, QObject * parent);
impl<'a> /*trait*/ QLibrary_new for (&'a QString, i32, &'a QObject) {
  fn new(self) -> QLibrary {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibraryC1ERK7QStringiP7QObject()};
    let ctysz: c_int = unsafe{QLibrary_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    // unsafe {_ZN8QLibraryC1ERK7QStringiP7QObject(qthis, arg0, arg1, arg2)};
    let qthis: u64 = unsafe {dector_ZN8QLibraryC1ERK7QStringiP7QObject(arg0, arg1, arg2)} as u64;
    let rsthis = QLibrary{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QLibrary::~QLibrary();
impl /*struct*/ QLibrary {
  pub fn free<RetType, T: QLibrary_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QLibrary_free<RetType> {
  fn free(self , rsthis: & QLibrary) -> RetType;
}

  // proto:  void QLibrary::~QLibrary();
impl<'a> /*trait*/ QLibrary_free<()> for () {
  fn free(self , rsthis: & QLibrary) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibraryD0Ev()};
     unsafe {_ZN8QLibraryD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QLibrary::errorString();
impl /*struct*/ QLibrary {
  pub fn errorString<RetType, T: QLibrary_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QLibrary_errorString<RetType> {
  fn errorString(self , rsthis: & QLibrary) -> RetType;
}

  // proto:  QString QLibrary::errorString();
impl<'a> /*trait*/ QLibrary_errorString<QString> for () {
  fn errorString(self , rsthis: & QLibrary) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QLibrary11errorStringEv()};
    let mut ret = unsafe {_ZNK8QLibrary11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QFunctionPointer QLibrary::resolve(const QString & fileName, int verNum, const char * symbol);
impl<'a> /*trait*/ QLibrary_resolve_s<()> for (&'a QString, i32, &'a  String) {
  fn resolve_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary7resolveERK7QStringiPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *mut c_char;
     unsafe {_ZN8QLibrary7resolveERK7QStringiPKc(arg0, arg1, arg2)};
    // return 1;
  }
}

// <= body block end

