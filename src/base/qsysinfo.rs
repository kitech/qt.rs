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

// proto: static QString QSysInfo::kernelType();
impl /*struct*/ QSysInfo {
  pub fn kernelType_s<RetType, T: QSysInfo_kernelType_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.kernelType_s();
    // return 1;
  }
}

pub trait QSysInfo_kernelType_s<RetType> {
  fn kernelType_s(self ) -> RetType;
}

// proto: static QString QSysInfo::kernelType();
impl<'a> /*trait*/ QSysInfo_kernelType_s<QString> for () {
  fn kernelType_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo10kernelTypeEv()};
    let mut ret = unsafe {_ZN8QSysInfo10kernelTypeEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QString QSysInfo::productType();
impl /*struct*/ QSysInfo {
  pub fn productType_s<RetType, T: QSysInfo_productType_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.productType_s();
    // return 1;
  }
}

pub trait QSysInfo_productType_s<RetType> {
  fn productType_s(self ) -> RetType;
}

// proto: static QString QSysInfo::productType();
impl<'a> /*trait*/ QSysInfo_productType_s<QString> for () {
  fn productType_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo11productTypeEv()};
    let mut ret = unsafe {_ZN8QSysInfo11productTypeEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QString QSysInfo::prettyProductName();
impl /*struct*/ QSysInfo {
  pub fn prettyProductName_s<RetType, T: QSysInfo_prettyProductName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.prettyProductName_s();
    // return 1;
  }
}

pub trait QSysInfo_prettyProductName_s<RetType> {
  fn prettyProductName_s(self ) -> RetType;
}

// proto: static QString QSysInfo::prettyProductName();
impl<'a> /*trait*/ QSysInfo_prettyProductName_s<QString> for () {
  fn prettyProductName_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo17prettyProductNameEv()};
    let mut ret = unsafe {_ZN8QSysInfo17prettyProductNameEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QString QSysInfo::currentCpuArchitecture();
impl /*struct*/ QSysInfo {
  pub fn currentCpuArchitecture_s<RetType, T: QSysInfo_currentCpuArchitecture_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentCpuArchitecture_s();
    // return 1;
  }
}

pub trait QSysInfo_currentCpuArchitecture_s<RetType> {
  fn currentCpuArchitecture_s(self ) -> RetType;
}

// proto: static QString QSysInfo::currentCpuArchitecture();
impl<'a> /*trait*/ QSysInfo_currentCpuArchitecture_s<QString> for () {
  fn currentCpuArchitecture_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo22currentCpuArchitectureEv()};
    let mut ret = unsafe {_ZN8QSysInfo22currentCpuArchitectureEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QString QSysInfo::buildCpuArchitecture();
impl /*struct*/ QSysInfo {
  pub fn buildCpuArchitecture_s<RetType, T: QSysInfo_buildCpuArchitecture_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.buildCpuArchitecture_s();
    // return 1;
  }
}

pub trait QSysInfo_buildCpuArchitecture_s<RetType> {
  fn buildCpuArchitecture_s(self ) -> RetType;
}

// proto: static QString QSysInfo::buildCpuArchitecture();
impl<'a> /*trait*/ QSysInfo_buildCpuArchitecture_s<QString> for () {
  fn buildCpuArchitecture_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo20buildCpuArchitectureEv()};
    let mut ret = unsafe {_ZN8QSysInfo20buildCpuArchitectureEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QString QSysInfo::kernelVersion();
impl /*struct*/ QSysInfo {
  pub fn kernelVersion_s<RetType, T: QSysInfo_kernelVersion_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.kernelVersion_s();
    // return 1;
  }
}

pub trait QSysInfo_kernelVersion_s<RetType> {
  fn kernelVersion_s(self ) -> RetType;
}

// proto: static QString QSysInfo::kernelVersion();
impl<'a> /*trait*/ QSysInfo_kernelVersion_s<QString> for () {
  fn kernelVersion_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo13kernelVersionEv()};
    let mut ret = unsafe {_ZN8QSysInfo13kernelVersionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QString QSysInfo::productVersion();
impl /*struct*/ QSysInfo {
  pub fn productVersion_s<RetType, T: QSysInfo_productVersion_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.productVersion_s();
    // return 1;
  }
}

pub trait QSysInfo_productVersion_s<RetType> {
  fn productVersion_s(self ) -> RetType;
}

// proto: static QString QSysInfo::productVersion();
impl<'a> /*trait*/ QSysInfo_productVersion_s<QString> for () {
  fn productVersion_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo14productVersionEv()};
    let mut ret = unsafe {_ZN8QSysInfo14productVersionEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QString QSysInfo::buildAbi();
impl /*struct*/ QSysInfo {
  pub fn buildAbi_s<RetType, T: QSysInfo_buildAbi_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.buildAbi_s();
    // return 1;
  }
}

pub trait QSysInfo_buildAbi_s<RetType> {
  fn buildAbi_s(self ) -> RetType;
}

// proto: static QString QSysInfo::buildAbi();
impl<'a> /*trait*/ QSysInfo_buildAbi_s<QString> for () {
  fn buildAbi_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo8buildAbiEv()};
    let mut ret = unsafe {_ZN8QSysInfo8buildAbiEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

