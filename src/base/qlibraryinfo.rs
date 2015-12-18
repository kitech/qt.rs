// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qdate::QDate;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QLibraryInfo::NewQLibraryInfo();
  fn _ZN12QLibraryInfoC1Ev(qthis: *mut c_void) ;
  // proto: static QStringList QLibraryInfo::platformPluginArguments(const QString & platformName);
  fn _ZN12QLibraryInfo23platformPluginArgumentsERK7QString(arg0: *mut c_void) ;
  // proto: static QString QLibraryInfo::licensee();
  fn _ZN12QLibraryInfo8licenseeEv() -> *mut c_void;
  // proto: static QString QLibraryInfo::licensedProducts();
  fn _ZN12QLibraryInfo16licensedProductsEv() -> *mut c_void;
  // proto: static bool QLibraryInfo::isDebugBuild();
  fn _ZN12QLibraryInfo12isDebugBuildEv() -> int8_t;
  // proto: static const char * QLibraryInfo::build();
  fn _ZN12QLibraryInfo5buildEv() -> *const c_char;
  // proto: static QDate QLibraryInfo::buildDate();
  fn _ZN12QLibraryInfo9buildDateEv() -> *mut c_void;
}

// body block begin
// class sizeof(QLibraryInfo)=1
pub struct QLibraryInfo {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLibraryInfo {
  pub fn NewQLibraryInfo<T: QLibraryInfo_NewQLibraryInfo>(value: T) -> QLibraryInfo {
    let rsthis = value.NewQLibraryInfo();
    return rsthis;
    // return 1;
  }
}

pub trait QLibraryInfo_NewQLibraryInfo {
  fn NewQLibraryInfo(self) -> QLibraryInfo;
}

// proto: void QLibraryInfo::NewQLibraryInfo();
impl<'a> /*trait*/ QLibraryInfo_NewQLibraryInfo for () {
  fn NewQLibraryInfo(self) -> QLibraryInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfoC1Ev()};
    unsafe {_ZN12QLibraryInfoC1Ev(qthis)};
    let rsthis = QLibraryInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLibraryInfo {
  pub fn platformPluginArguments<RetType, T: QLibraryInfo_platformPluginArguments<RetType>>(&mut self, value: T) -> RetType {
    return value.platformPluginArguments(self);
    // return 1;
  }
}

pub trait QLibraryInfo_platformPluginArguments<RetType> {
  fn platformPluginArguments(self, rsthis: &mut QLibraryInfo) -> RetType;
}

// proto: static QStringList QLibraryInfo::platformPluginArguments(const QString & platformName);
impl<'a> /*trait*/ QLibraryInfo_platformPluginArguments<()> for (&'a  QString) {
  fn platformPluginArguments(self, rsthis: &mut QLibraryInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfo23platformPluginArgumentsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QLibraryInfo23platformPluginArgumentsERK7QString(arg0)};
    // return 1;
  }
}

impl /*struct*/ QLibraryInfo {
  pub fn licensee<RetType, T: QLibraryInfo_licensee<RetType>>(&mut self, value: T) -> RetType {
    return value.licensee(self);
    // return 1;
  }
}

pub trait QLibraryInfo_licensee<RetType> {
  fn licensee(self, rsthis: &mut QLibraryInfo) -> RetType;
}

// proto: static QString QLibraryInfo::licensee();
impl<'a> /*trait*/ QLibraryInfo_licensee<QString> for () {
  fn licensee(self, rsthis: &mut QLibraryInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfo8licenseeEv()};
    let mut ret = unsafe {_ZN12QLibraryInfo8licenseeEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLibraryInfo {
  pub fn licensedProducts<RetType, T: QLibraryInfo_licensedProducts<RetType>>(&mut self, value: T) -> RetType {
    return value.licensedProducts(self);
    // return 1;
  }
}

pub trait QLibraryInfo_licensedProducts<RetType> {
  fn licensedProducts(self, rsthis: &mut QLibraryInfo) -> RetType;
}

// proto: static QString QLibraryInfo::licensedProducts();
impl<'a> /*trait*/ QLibraryInfo_licensedProducts<QString> for () {
  fn licensedProducts(self, rsthis: &mut QLibraryInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfo16licensedProductsEv()};
    let mut ret = unsafe {_ZN12QLibraryInfo16licensedProductsEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLibraryInfo {
  pub fn isDebugBuild<RetType, T: QLibraryInfo_isDebugBuild<RetType>>(&mut self, value: T) -> RetType {
    return value.isDebugBuild(self);
    // return 1;
  }
}

pub trait QLibraryInfo_isDebugBuild<RetType> {
  fn isDebugBuild(self, rsthis: &mut QLibraryInfo) -> RetType;
}

// proto: static bool QLibraryInfo::isDebugBuild();
impl<'a> /*trait*/ QLibraryInfo_isDebugBuild<i8> for () {
  fn isDebugBuild(self, rsthis: &mut QLibraryInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfo12isDebugBuildEv()};
    let mut ret = unsafe {_ZN12QLibraryInfo12isDebugBuildEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLibraryInfo {
  pub fn build<RetType, T: QLibraryInfo_build<RetType>>(&mut self, value: T) -> RetType {
    return value.build(self);
    // return 1;
  }
}

pub trait QLibraryInfo_build<RetType> {
  fn build(self, rsthis: &mut QLibraryInfo) -> RetType;
}

// proto: static const char * QLibraryInfo::build();
impl<'a> /*trait*/ QLibraryInfo_build<String> for () {
  fn build(self, rsthis: &mut QLibraryInfo) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfo5buildEv()};
    let mut ret = unsafe {_ZN12QLibraryInfo5buildEv()};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QLibraryInfo {
  pub fn buildDate<RetType, T: QLibraryInfo_buildDate<RetType>>(&mut self, value: T) -> RetType {
    return value.buildDate(self);
    // return 1;
  }
}

pub trait QLibraryInfo_buildDate<RetType> {
  fn buildDate(self, rsthis: &mut QLibraryInfo) -> RetType;
}

// proto: static QDate QLibraryInfo::buildDate();
impl<'a> /*trait*/ QLibraryInfo_buildDate<QDate> for () {
  fn buildDate(self, rsthis: &mut QLibraryInfo) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfo9buildDateEv()};
    let mut ret = unsafe {_ZN12QLibraryInfo9buildDateEv()};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

