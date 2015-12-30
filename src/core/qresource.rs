// auto generated, do not modify.
// created: Wed Dec 30 23:22:52 2015
// src-file: /QtCore/qresource.h
// dst-file: /src/core/qresource.rs
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
use super::qstring::QString; // 773
use super::qlocale::QLocale; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QResource_Class_Size() -> c_int;
  // proto:  void QResource::QResource(const QString & file, const QLocale & locale);
  fn dector_ZN9QResourceC1ERK7QStringRK7QLocale(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN9QResourceC1ERK7QStringRK7QLocale(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QLocale QResource::locale();
  fn _ZNK9QResource6localeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QResource::setLocale(const QLocale & locale);
  fn _ZN9QResource9setLocaleERK7QLocale(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static bool QResource::registerResource(const uchar * rccData, const QString & resourceRoot);
  fn _ZN9QResource16registerResourceEPKhRK7QString(arg0: *mut c_uchar, arg1: *mut c_void) -> c_char;
  // proto:  const uchar * QResource::data();
  fn _ZNK9QResource4dataEv(qthis: u64 /* *mut c_void*/) -> *mut c_uchar;
  // proto: static QStringList QResource::searchPaths();
  fn _ZN9QResource11searchPathsEv();
  // proto:  QString QResource::fileName();
  fn _ZNK9QResource8fileNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QResource::absoluteFilePath();
  fn _ZNK9QResource16absoluteFilePathEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static bool QResource::unregisterResource(const uchar * rccData, const QString & resourceRoot);
  fn _ZN9QResource18unregisterResourceEPKhRK7QString(arg0: *mut c_uchar, arg1: *mut c_void) -> c_char;
  // proto: static bool QResource::registerResource(const QString & rccFilename, const QString & resourceRoot);
  fn _ZN9QResource16registerResourceERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto: static void QResource::addSearchPath(const QString & path);
  fn _ZN9QResource13addSearchPathERK7QString(arg0: *mut c_void);
  // proto:  qint64 QResource::size();
  fn _ZNK9QResource4sizeEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  void QResource::~QResource();
  fn _ZN9QResourceD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QResource::isValid();
  fn _ZNK9QResource7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QResource::setFileName(const QString & file);
  fn _ZN9QResource11setFileNameERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static bool QResource::unregisterResource(const QString & rccFilename, const QString & resourceRoot);
  fn _ZN9QResource18unregisterResourceERK7QStringS2_(arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  bool QResource::isCompressed();
  fn _ZNK9QResource12isCompressedEv(qthis: u64 /* *mut c_void*/) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QResource)=1
#[derive(Default)]
pub struct QResource {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QResource {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QResource {
    return QResource{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QResource::QResource(const QString & file, const QLocale & locale);
impl /*struct*/ QResource {
  pub fn New<T: QResource_New>(value: T) -> QResource {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QResource_New {
  fn New(self) -> QResource;
}

  // proto:  void QResource::QResource(const QString & file, const QLocale & locale);
impl<'a> /*trait*/ QResource_New for (&'a QString, &'a QLocale) {
  fn New(self) -> QResource {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResourceC1ERK7QStringRK7QLocale()};
    let ctysz: c_int = unsafe{QResource_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN9QResourceC1ERK7QStringRK7QLocale(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN9QResourceC1ERK7QStringRK7QLocale(arg0, arg1)} as u64;
    let rsthis = QResource{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QLocale QResource::locale();
impl /*struct*/ QResource {
  pub fn locale<RetType, T: QResource_locale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.locale(self);
    // return 1;
  }
}

pub trait QResource_locale<RetType> {
  fn locale(self , rsthis: & QResource) -> RetType;
}

  // proto:  QLocale QResource::locale();
impl<'a> /*trait*/ QResource_locale<QLocale> for () {
  fn locale(self , rsthis: & QResource) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource6localeEv()};
    let mut ret = unsafe {_ZNK9QResource6localeEv(rsthis.qclsinst)};
    let mut ret1 = QLocale::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QResource::setLocale(const QLocale & locale);
impl /*struct*/ QResource {
  pub fn setLocale<RetType, T: QResource_setLocale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLocale(self);
    // return 1;
  }
}

pub trait QResource_setLocale<RetType> {
  fn setLocale(self , rsthis: & QResource) -> RetType;
}

  // proto:  void QResource::setLocale(const QLocale & locale);
impl<'a> /*trait*/ QResource_setLocale<()> for (&'a QLocale) {
  fn setLocale(self , rsthis: & QResource) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource9setLocaleERK7QLocale()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QResource9setLocaleERK7QLocale(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static bool QResource::registerResource(const uchar * rccData, const QString & resourceRoot);
impl /*struct*/ QResource {
  pub fn registerResource_s<RetType, T: QResource_registerResource_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.registerResource_s();
    // return 1;
  }
}

pub trait QResource_registerResource_s<RetType> {
  fn registerResource_s(self ) -> RetType;
}

  // proto: static bool QResource::registerResource(const uchar * rccData, const QString & resourceRoot);
impl<'a> /*trait*/ QResource_registerResource_s<i8> for (&'a  String, &'a QString) {
  fn registerResource_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource16registerResourceEPKhRK7QString()};
    let arg0 = self.0.as_ptr()  as *mut c_uchar;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QResource16registerResourceEPKhRK7QString(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const uchar * QResource::data();
impl /*struct*/ QResource {
  pub fn data<RetType, T: QResource_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QResource_data<RetType> {
  fn data(self , rsthis: & QResource) -> RetType;
}

  // proto:  const uchar * QResource::data();
impl<'a> /*trait*/ QResource_data<String> for () {
  fn data(self , rsthis: & QResource) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource4dataEv()};
    let mut ret = unsafe {_ZNK9QResource4dataEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto: static QStringList QResource::searchPaths();
impl /*struct*/ QResource {
  pub fn searchPaths_s<RetType, T: QResource_searchPaths_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.searchPaths_s();
    // return 1;
  }
}

pub trait QResource_searchPaths_s<RetType> {
  fn searchPaths_s(self ) -> RetType;
}

  // proto: static QStringList QResource::searchPaths();
impl<'a> /*trait*/ QResource_searchPaths_s<()> for () {
  fn searchPaths_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource11searchPathsEv()};
     unsafe {_ZN9QResource11searchPathsEv()};
    // return 1;
  }
}

  // proto:  QString QResource::fileName();
impl /*struct*/ QResource {
  pub fn fileName<RetType, T: QResource_fileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QResource_fileName<RetType> {
  fn fileName(self , rsthis: & QResource) -> RetType;
}

  // proto:  QString QResource::fileName();
impl<'a> /*trait*/ QResource_fileName<QString> for () {
  fn fileName(self , rsthis: & QResource) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource8fileNameEv()};
    let mut ret = unsafe {_ZNK9QResource8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QResource::absoluteFilePath();
impl /*struct*/ QResource {
  pub fn absoluteFilePath<RetType, T: QResource_absoluteFilePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.absoluteFilePath(self);
    // return 1;
  }
}

pub trait QResource_absoluteFilePath<RetType> {
  fn absoluteFilePath(self , rsthis: & QResource) -> RetType;
}

  // proto:  QString QResource::absoluteFilePath();
impl<'a> /*trait*/ QResource_absoluteFilePath<QString> for () {
  fn absoluteFilePath(self , rsthis: & QResource) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource16absoluteFilePathEv()};
    let mut ret = unsafe {_ZNK9QResource16absoluteFilePathEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QResource::unregisterResource(const uchar * rccData, const QString & resourceRoot);
impl /*struct*/ QResource {
  pub fn unregisterResource_s<RetType, T: QResource_unregisterResource_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.unregisterResource_s();
    // return 1;
  }
}

pub trait QResource_unregisterResource_s<RetType> {
  fn unregisterResource_s(self ) -> RetType;
}

  // proto: static bool QResource::unregisterResource(const uchar * rccData, const QString & resourceRoot);
impl<'a> /*trait*/ QResource_unregisterResource_s<i8> for (&'a  String, &'a QString) {
  fn unregisterResource_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource18unregisterResourceEPKhRK7QString()};
    let arg0 = self.0.as_ptr()  as *mut c_uchar;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QResource18unregisterResourceEPKhRK7QString(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static bool QResource::registerResource(const QString & rccFilename, const QString & resourceRoot);
impl<'a> /*trait*/ QResource_registerResource_s<i8> for (&'a QString, &'a QString) {
  fn registerResource_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource16registerResourceERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QResource16registerResourceERK7QStringS2_(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static void QResource::addSearchPath(const QString & path);
impl /*struct*/ QResource {
  pub fn addSearchPath_s<RetType, T: QResource_addSearchPath_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.addSearchPath_s();
    // return 1;
  }
}

pub trait QResource_addSearchPath_s<RetType> {
  fn addSearchPath_s(self ) -> RetType;
}

  // proto: static void QResource::addSearchPath(const QString & path);
impl<'a> /*trait*/ QResource_addSearchPath_s<()> for (&'a QString) {
  fn addSearchPath_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource13addSearchPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QResource13addSearchPathERK7QString(arg0)};
    // return 1;
  }
}

  // proto:  qint64 QResource::size();
impl /*struct*/ QResource {
  pub fn size<RetType, T: QResource_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QResource_size<RetType> {
  fn size(self , rsthis: & QResource) -> RetType;
}

  // proto:  qint64 QResource::size();
impl<'a> /*trait*/ QResource_size<i64> for () {
  fn size(self , rsthis: & QResource) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource4sizeEv()};
    let mut ret = unsafe {_ZNK9QResource4sizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QResource::~QResource();
impl /*struct*/ QResource {
  pub fn Free<RetType, T: QResource_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QResource_Free<RetType> {
  fn Free(self , rsthis: & QResource) -> RetType;
}

  // proto:  void QResource::~QResource();
impl<'a> /*trait*/ QResource_Free<()> for () {
  fn Free(self , rsthis: & QResource) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResourceD0Ev()};
     unsafe {_ZN9QResourceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QResource::isValid();
impl /*struct*/ QResource {
  pub fn isValid<RetType, T: QResource_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QResource_isValid<RetType> {
  fn isValid(self , rsthis: & QResource) -> RetType;
}

  // proto:  bool QResource::isValid();
impl<'a> /*trait*/ QResource_isValid<i8> for () {
  fn isValid(self , rsthis: & QResource) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource7isValidEv()};
    let mut ret = unsafe {_ZNK9QResource7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QResource::setFileName(const QString & file);
impl /*struct*/ QResource {
  pub fn setFileName<RetType, T: QResource_setFileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFileName(self);
    // return 1;
  }
}

pub trait QResource_setFileName<RetType> {
  fn setFileName(self , rsthis: & QResource) -> RetType;
}

  // proto:  void QResource::setFileName(const QString & file);
impl<'a> /*trait*/ QResource_setFileName<()> for (&'a QString) {
  fn setFileName(self , rsthis: & QResource) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QResource11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static bool QResource::unregisterResource(const QString & rccFilename, const QString & resourceRoot);
impl<'a> /*trait*/ QResource_unregisterResource_s<i8> for (&'a QString, &'a QString) {
  fn unregisterResource_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QResource18unregisterResourceERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QResource18unregisterResourceERK7QStringS2_(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QResource::isCompressed();
impl /*struct*/ QResource {
  pub fn isCompressed<RetType, T: QResource_isCompressed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCompressed(self);
    // return 1;
  }
}

pub trait QResource_isCompressed<RetType> {
  fn isCompressed(self , rsthis: & QResource) -> RetType;
}

  // proto:  bool QResource::isCompressed();
impl<'a> /*trait*/ QResource_isCompressed<i8> for () {
  fn isCompressed(self , rsthis: & QResource) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QResource12isCompressedEv()};
    let mut ret = unsafe {_ZNK9QResource12isCompressedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

