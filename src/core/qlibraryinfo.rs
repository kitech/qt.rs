// auto generated, do not modify.
// created: Wed Dec 30 23:22:52 2015
// src-file: /QtCore/qlibraryinfo.h
// dst-file: /src/core/qlibraryinfo.rs
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
use super::qdatetime::QDate; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QLibraryInfo_Class_Size() -> c_int;
  // proto:  void QLibraryInfo::QLibraryInfo();
  fn dector_ZN12QLibraryInfoC1Ev() -> *mut c_void;
  fn _ZN12QLibraryInfoC1Ev(qthis: u64 /* *mut c_void*/);
  // proto: static QStringList QLibraryInfo::platformPluginArguments(const QString & platformName);
  fn _ZN12QLibraryInfo23platformPluginArgumentsERK7QString(arg0: *mut c_void);
  // proto: static QString QLibraryInfo::licensee();
  fn _ZN12QLibraryInfo8licenseeEv() -> *mut c_void;
  // proto: static QString QLibraryInfo::licensedProducts();
  fn _ZN12QLibraryInfo16licensedProductsEv() -> *mut c_void;
  // proto: static bool QLibraryInfo::isDebugBuild();
  fn _ZN12QLibraryInfo12isDebugBuildEv() -> c_char;
  // proto: static const char * QLibraryInfo::build();
  fn _ZN12QLibraryInfo5buildEv() -> *mut c_char;
  // proto: static QDate QLibraryInfo::buildDate();
  fn _ZN12QLibraryInfo9buildDateEv() -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QLibraryInfo)=1
#[derive(Default)]
pub struct QLibraryInfo {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QLibraryInfo {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QLibraryInfo {
    return QLibraryInfo{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QLibraryInfo::QLibraryInfo();
impl /*struct*/ QLibraryInfo {
  pub fn New<T: QLibraryInfo_New>(value: T) -> QLibraryInfo {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QLibraryInfo_New {
  fn New(self) -> QLibraryInfo;
}

  // proto:  void QLibraryInfo::QLibraryInfo();
impl<'a> /*trait*/ QLibraryInfo_New for () {
  fn New(self) -> QLibraryInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfoC1Ev()};
    let ctysz: c_int = unsafe{QLibraryInfo_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN12QLibraryInfoC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN12QLibraryInfoC1Ev()} as u64;
    let rsthis = QLibraryInfo{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static QStringList QLibraryInfo::platformPluginArguments(const QString & platformName);
impl /*struct*/ QLibraryInfo {
  pub fn platformPluginArguments_s<RetType, T: QLibraryInfo_platformPluginArguments_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.platformPluginArguments_s();
    // return 1;
  }
}

pub trait QLibraryInfo_platformPluginArguments_s<RetType> {
  fn platformPluginArguments_s(self ) -> RetType;
}

  // proto: static QStringList QLibraryInfo::platformPluginArguments(const QString & platformName);
impl<'a> /*trait*/ QLibraryInfo_platformPluginArguments_s<()> for (&'a QString) {
  fn platformPluginArguments_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfo23platformPluginArgumentsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QLibraryInfo23platformPluginArgumentsERK7QString(arg0)};
    // return 1;
  }
}

  // proto: static QString QLibraryInfo::licensee();
impl /*struct*/ QLibraryInfo {
  pub fn licensee_s<RetType, T: QLibraryInfo_licensee_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.licensee_s();
    // return 1;
  }
}

pub trait QLibraryInfo_licensee_s<RetType> {
  fn licensee_s(self ) -> RetType;
}

  // proto: static QString QLibraryInfo::licensee();
impl<'a> /*trait*/ QLibraryInfo_licensee_s<QString> for () {
  fn licensee_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfo8licenseeEv()};
    let mut ret = unsafe {_ZN12QLibraryInfo8licenseeEv()};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QLibraryInfo::licensedProducts();
impl /*struct*/ QLibraryInfo {
  pub fn licensedProducts_s<RetType, T: QLibraryInfo_licensedProducts_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.licensedProducts_s();
    // return 1;
  }
}

pub trait QLibraryInfo_licensedProducts_s<RetType> {
  fn licensedProducts_s(self ) -> RetType;
}

  // proto: static QString QLibraryInfo::licensedProducts();
impl<'a> /*trait*/ QLibraryInfo_licensedProducts_s<QString> for () {
  fn licensedProducts_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfo16licensedProductsEv()};
    let mut ret = unsafe {_ZN12QLibraryInfo16licensedProductsEv()};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QLibraryInfo::isDebugBuild();
impl /*struct*/ QLibraryInfo {
  pub fn isDebugBuild_s<RetType, T: QLibraryInfo_isDebugBuild_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isDebugBuild_s();
    // return 1;
  }
}

pub trait QLibraryInfo_isDebugBuild_s<RetType> {
  fn isDebugBuild_s(self ) -> RetType;
}

  // proto: static bool QLibraryInfo::isDebugBuild();
impl<'a> /*trait*/ QLibraryInfo_isDebugBuild_s<i8> for () {
  fn isDebugBuild_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfo12isDebugBuildEv()};
    let mut ret = unsafe {_ZN12QLibraryInfo12isDebugBuildEv()};
    return ret as i8;
    // return 1;
  }
}

  // proto: static const char * QLibraryInfo::build();
impl /*struct*/ QLibraryInfo {
  pub fn build_s<RetType, T: QLibraryInfo_build_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.build_s();
    // return 1;
  }
}

pub trait QLibraryInfo_build_s<RetType> {
  fn build_s(self ) -> RetType;
}

  // proto: static const char * QLibraryInfo::build();
impl<'a> /*trait*/ QLibraryInfo_build_s<String> for () {
  fn build_s(self ) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfo5buildEv()};
    let mut ret = unsafe {_ZN12QLibraryInfo5buildEv()};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto: static QDate QLibraryInfo::buildDate();
impl /*struct*/ QLibraryInfo {
  pub fn buildDate_s<RetType, T: QLibraryInfo_buildDate_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.buildDate_s();
    // return 1;
  }
}

pub trait QLibraryInfo_buildDate_s<RetType> {
  fn buildDate_s(self ) -> RetType;
}

  // proto: static QDate QLibraryInfo::buildDate();
impl<'a> /*trait*/ QLibraryInfo_buildDate_s<QDate> for () {
  fn buildDate_s(self ) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfo9buildDateEv()};
    let mut ret = unsafe {_ZN12QLibraryInfo9buildDateEv()};
    let mut ret1 = QDate::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

