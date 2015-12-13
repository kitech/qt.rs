// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN12QLibraryInfoC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN12QLibraryInfo23platformPluginArgumentsERK7QString(arg0: *const c_void) -> i32;
  fn _ZN12QLibraryInfo8licenseeEv() -> i32;
  fn _ZN12QLibraryInfo16licensedProductsEv() -> i32;
  fn _ZN12QLibraryInfo12isDebugBuildEv() -> i32;
  fn _ZN12QLibraryInfo5buildEv() -> i32;
  fn _ZN12QLibraryInfo9buildDateEv() -> i32;
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
  pub fn platformPluginArguments<T: QLibraryInfo_platformPluginArguments>(&mut self, value: T) -> i32 {
    value.platformPluginArguments(self);
    return 1;
  }
}

pub trait QLibraryInfo_platformPluginArguments {
  fn platformPluginArguments(self, this: &mut QLibraryInfo) -> i32;
}

// proto: QStringList QLibraryInfo::platformPluginArguments(const QString & platformName);
impl<'a> /*trait*/ QLibraryInfo_platformPluginArguments for (&'a  QString) {
  fn platformPluginArguments(self, this: &mut QLibraryInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfo23platformPluginArgumentsERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QLibraryInfo23platformPluginArgumentsERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QLibraryInfo {
  pub fn licensee<T: QLibraryInfo_licensee>(&mut self, value: T) -> i32 {
    value.licensee(self);
    return 1;
  }
}

pub trait QLibraryInfo_licensee {
  fn licensee(self, this: &mut QLibraryInfo) -> i32;
}

// proto: QString QLibraryInfo::licensee();
impl<'a> /*trait*/ QLibraryInfo_licensee for () {
  fn licensee(self, this: &mut QLibraryInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfo8licenseeEv()};
    unsafe {_ZN12QLibraryInfo8licenseeEv()};
    return 1;
  }
}

impl /*struct*/ QLibraryInfo {
  pub fn licensedProducts<T: QLibraryInfo_licensedProducts>(&mut self, value: T) -> i32 {
    value.licensedProducts(self);
    return 1;
  }
}

pub trait QLibraryInfo_licensedProducts {
  fn licensedProducts(self, this: &mut QLibraryInfo) -> i32;
}

// proto: QString QLibraryInfo::licensedProducts();
impl<'a> /*trait*/ QLibraryInfo_licensedProducts for () {
  fn licensedProducts(self, this: &mut QLibraryInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfo16licensedProductsEv()};
    unsafe {_ZN12QLibraryInfo16licensedProductsEv()};
    return 1;
  }
}

impl /*struct*/ QLibraryInfo {
  pub fn isDebugBuild<T: QLibraryInfo_isDebugBuild>(&mut self, value: T) -> i32 {
    value.isDebugBuild(self);
    return 1;
  }
}

pub trait QLibraryInfo_isDebugBuild {
  fn isDebugBuild(self, this: &mut QLibraryInfo) -> i32;
}

// proto: bool QLibraryInfo::isDebugBuild();
impl<'a> /*trait*/ QLibraryInfo_isDebugBuild for () {
  fn isDebugBuild(self, this: &mut QLibraryInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfo12isDebugBuildEv()};
    unsafe {_ZN12QLibraryInfo12isDebugBuildEv()};
    return 1;
  }
}

impl /*struct*/ QLibraryInfo {
  pub fn build<T: QLibraryInfo_build>(&mut self, value: T) -> i32 {
    value.build(self);
    return 1;
  }
}

pub trait QLibraryInfo_build {
  fn build(self, this: &mut QLibraryInfo) -> i32;
}

// proto: const char * QLibraryInfo::build();
impl<'a> /*trait*/ QLibraryInfo_build for () {
  fn build(self, this: &mut QLibraryInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfo5buildEv()};
    unsafe {_ZN12QLibraryInfo5buildEv()};
    return 1;
  }
}

impl /*struct*/ QLibraryInfo {
  pub fn buildDate<T: QLibraryInfo_buildDate>(&mut self, value: T) -> i32 {
    value.buildDate(self);
    return 1;
  }
}

pub trait QLibraryInfo_buildDate {
  fn buildDate(self, this: &mut QLibraryInfo) -> i32;
}

// proto: QDate QLibraryInfo::buildDate();
impl<'a> /*trait*/ QLibraryInfo_buildDate for () {
  fn buildDate(self, this: &mut QLibraryInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QLibraryInfo9buildDateEv()};
    unsafe {_ZN12QLibraryInfo9buildDateEv()};
    return 1;
  }
}

