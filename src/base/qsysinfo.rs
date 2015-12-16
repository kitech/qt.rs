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
  // proto: static QString QSysInfo::kernelType();
  fn _ZN8QSysInfo10kernelTypeEv() -> *mut c_void;
  // proto: static QString QSysInfo::productType();
  fn _ZN8QSysInfo11productTypeEv() -> *mut c_void;
  // proto: static QString QSysInfo::prettyProductName();
  fn _ZN8QSysInfo17prettyProductNameEv() -> *mut c_void;
  // proto: static QString QSysInfo::currentCpuArchitecture();
  fn _ZN8QSysInfo22currentCpuArchitectureEv() -> *mut c_void;
  // proto: static QString QSysInfo::buildCpuArchitecture();
  fn _ZN8QSysInfo20buildCpuArchitectureEv() -> *mut c_void;
  // proto: static QString QSysInfo::kernelVersion();
  fn _ZN8QSysInfo13kernelVersionEv() -> *mut c_void;
  // proto: static QString QSysInfo::productVersion();
  fn _ZN8QSysInfo14productVersionEv() -> *mut c_void;
  // proto: static QString QSysInfo::buildAbi();
  fn _ZN8QSysInfo8buildAbiEv() -> *mut c_void;
}

// body block begin
// class sizeof(QSysInfo)=1
pub struct QSysInfo {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSysInfo {
  pub fn kernelType<T: QSysInfo_kernelType>(&mut self, value: T) -> QString {
    return value.kernelType(self);
    // return 1;
  }
}

pub trait QSysInfo_kernelType {
  fn kernelType(self, rsthis: &mut QSysInfo) -> QString;
}

// proto: static QString QSysInfo::kernelType();
impl<'a> /*trait*/ QSysInfo_kernelType for () {
  fn kernelType(self, rsthis: &mut QSysInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo10kernelTypeEv()};
    let mut ret = unsafe {_ZN8QSysInfo10kernelTypeEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSysInfo {
  pub fn productType<T: QSysInfo_productType>(&mut self, value: T) -> QString {
    return value.productType(self);
    // return 1;
  }
}

pub trait QSysInfo_productType {
  fn productType(self, rsthis: &mut QSysInfo) -> QString;
}

// proto: static QString QSysInfo::productType();
impl<'a> /*trait*/ QSysInfo_productType for () {
  fn productType(self, rsthis: &mut QSysInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo11productTypeEv()};
    let mut ret = unsafe {_ZN8QSysInfo11productTypeEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSysInfo {
  pub fn prettyProductName<T: QSysInfo_prettyProductName>(&mut self, value: T) -> QString {
    return value.prettyProductName(self);
    // return 1;
  }
}

pub trait QSysInfo_prettyProductName {
  fn prettyProductName(self, rsthis: &mut QSysInfo) -> QString;
}

// proto: static QString QSysInfo::prettyProductName();
impl<'a> /*trait*/ QSysInfo_prettyProductName for () {
  fn prettyProductName(self, rsthis: &mut QSysInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo17prettyProductNameEv()};
    let mut ret = unsafe {_ZN8QSysInfo17prettyProductNameEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSysInfo {
  pub fn currentCpuArchitecture<T: QSysInfo_currentCpuArchitecture>(&mut self, value: T) -> QString {
    return value.currentCpuArchitecture(self);
    // return 1;
  }
}

pub trait QSysInfo_currentCpuArchitecture {
  fn currentCpuArchitecture(self, rsthis: &mut QSysInfo) -> QString;
}

// proto: static QString QSysInfo::currentCpuArchitecture();
impl<'a> /*trait*/ QSysInfo_currentCpuArchitecture for () {
  fn currentCpuArchitecture(self, rsthis: &mut QSysInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo22currentCpuArchitectureEv()};
    let mut ret = unsafe {_ZN8QSysInfo22currentCpuArchitectureEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSysInfo {
  pub fn buildCpuArchitecture<T: QSysInfo_buildCpuArchitecture>(&mut self, value: T) -> QString {
    return value.buildCpuArchitecture(self);
    // return 1;
  }
}

pub trait QSysInfo_buildCpuArchitecture {
  fn buildCpuArchitecture(self, rsthis: &mut QSysInfo) -> QString;
}

// proto: static QString QSysInfo::buildCpuArchitecture();
impl<'a> /*trait*/ QSysInfo_buildCpuArchitecture for () {
  fn buildCpuArchitecture(self, rsthis: &mut QSysInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo20buildCpuArchitectureEv()};
    let mut ret = unsafe {_ZN8QSysInfo20buildCpuArchitectureEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSysInfo {
  pub fn kernelVersion<T: QSysInfo_kernelVersion>(&mut self, value: T) -> QString {
    return value.kernelVersion(self);
    // return 1;
  }
}

pub trait QSysInfo_kernelVersion {
  fn kernelVersion(self, rsthis: &mut QSysInfo) -> QString;
}

// proto: static QString QSysInfo::kernelVersion();
impl<'a> /*trait*/ QSysInfo_kernelVersion for () {
  fn kernelVersion(self, rsthis: &mut QSysInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo13kernelVersionEv()};
    let mut ret = unsafe {_ZN8QSysInfo13kernelVersionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSysInfo {
  pub fn productVersion<T: QSysInfo_productVersion>(&mut self, value: T) -> QString {
    return value.productVersion(self);
    // return 1;
  }
}

pub trait QSysInfo_productVersion {
  fn productVersion(self, rsthis: &mut QSysInfo) -> QString;
}

// proto: static QString QSysInfo::productVersion();
impl<'a> /*trait*/ QSysInfo_productVersion for () {
  fn productVersion(self, rsthis: &mut QSysInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo14productVersionEv()};
    let mut ret = unsafe {_ZN8QSysInfo14productVersionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSysInfo {
  pub fn buildAbi<T: QSysInfo_buildAbi>(&mut self, value: T) -> QString {
    return value.buildAbi(self);
    // return 1;
  }
}

pub trait QSysInfo_buildAbi {
  fn buildAbi(self, rsthis: &mut QSysInfo) -> QString;
}

// proto: static QString QSysInfo::buildAbi();
impl<'a> /*trait*/ QSysInfo_buildAbi for () {
  fn buildAbi(self, rsthis: &mut QSysInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo8buildAbiEv()};
    let mut ret = unsafe {_ZN8QSysInfo8buildAbiEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

