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
  pub fn kernelType<RetType, T: QSysInfo_kernelType<RetType>>(&mut self, value: T) -> RetType {
    return value.kernelType(self);
    // return 1;
  }
}

pub trait QSysInfo_kernelType<RetType> {
  fn kernelType(self, rsthis: &mut QSysInfo) -> RetType;
}

// proto: static QString QSysInfo::kernelType();
impl<'a> /*trait*/ QSysInfo_kernelType<QString> for () {
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
  pub fn productType<RetType, T: QSysInfo_productType<RetType>>(&mut self, value: T) -> RetType {
    return value.productType(self);
    // return 1;
  }
}

pub trait QSysInfo_productType<RetType> {
  fn productType(self, rsthis: &mut QSysInfo) -> RetType;
}

// proto: static QString QSysInfo::productType();
impl<'a> /*trait*/ QSysInfo_productType<QString> for () {
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
  pub fn prettyProductName<RetType, T: QSysInfo_prettyProductName<RetType>>(&mut self, value: T) -> RetType {
    return value.prettyProductName(self);
    // return 1;
  }
}

pub trait QSysInfo_prettyProductName<RetType> {
  fn prettyProductName(self, rsthis: &mut QSysInfo) -> RetType;
}

// proto: static QString QSysInfo::prettyProductName();
impl<'a> /*trait*/ QSysInfo_prettyProductName<QString> for () {
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
  pub fn currentCpuArchitecture<RetType, T: QSysInfo_currentCpuArchitecture<RetType>>(&mut self, value: T) -> RetType {
    return value.currentCpuArchitecture(self);
    // return 1;
  }
}

pub trait QSysInfo_currentCpuArchitecture<RetType> {
  fn currentCpuArchitecture(self, rsthis: &mut QSysInfo) -> RetType;
}

// proto: static QString QSysInfo::currentCpuArchitecture();
impl<'a> /*trait*/ QSysInfo_currentCpuArchitecture<QString> for () {
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
  pub fn buildCpuArchitecture<RetType, T: QSysInfo_buildCpuArchitecture<RetType>>(&mut self, value: T) -> RetType {
    return value.buildCpuArchitecture(self);
    // return 1;
  }
}

pub trait QSysInfo_buildCpuArchitecture<RetType> {
  fn buildCpuArchitecture(self, rsthis: &mut QSysInfo) -> RetType;
}

// proto: static QString QSysInfo::buildCpuArchitecture();
impl<'a> /*trait*/ QSysInfo_buildCpuArchitecture<QString> for () {
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
  pub fn kernelVersion<RetType, T: QSysInfo_kernelVersion<RetType>>(&mut self, value: T) -> RetType {
    return value.kernelVersion(self);
    // return 1;
  }
}

pub trait QSysInfo_kernelVersion<RetType> {
  fn kernelVersion(self, rsthis: &mut QSysInfo) -> RetType;
}

// proto: static QString QSysInfo::kernelVersion();
impl<'a> /*trait*/ QSysInfo_kernelVersion<QString> for () {
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
  pub fn productVersion<RetType, T: QSysInfo_productVersion<RetType>>(&mut self, value: T) -> RetType {
    return value.productVersion(self);
    // return 1;
  }
}

pub trait QSysInfo_productVersion<RetType> {
  fn productVersion(self, rsthis: &mut QSysInfo) -> RetType;
}

// proto: static QString QSysInfo::productVersion();
impl<'a> /*trait*/ QSysInfo_productVersion<QString> for () {
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
  pub fn buildAbi<RetType, T: QSysInfo_buildAbi<RetType>>(&mut self, value: T) -> RetType {
    return value.buildAbi(self);
    // return 1;
  }
}

pub trait QSysInfo_buildAbi<RetType> {
  fn buildAbi(self, rsthis: &mut QSysInfo) -> RetType;
}

// proto: static QString QSysInfo::buildAbi();
impl<'a> /*trait*/ QSysInfo_buildAbi<QString> for () {
  fn buildAbi(self, rsthis: &mut QSysInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo8buildAbiEv()};
    let mut ret = unsafe {_ZN8QSysInfo8buildAbiEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

