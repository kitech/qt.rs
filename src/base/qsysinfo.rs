// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN8QSysInfo10kernelTypeEv() -> i32;
  fn _ZN8QSysInfo11productTypeEv() -> i32;
  fn _ZN8QSysInfo17prettyProductNameEv() -> i32;
  fn _ZN8QSysInfo22currentCpuArchitectureEv() -> i32;
  fn _ZN8QSysInfo20buildCpuArchitectureEv() -> i32;
  fn _ZN8QSysInfo13kernelVersionEv() -> i32;
  fn _ZN8QSysInfo14productVersionEv() -> i32;
  fn _ZN8QSysInfo8buildAbiEv() -> i32;
}

// body block begin
// class sizeof(QSysInfo)=1
pub struct QSysInfo {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSysInfo {
  pub fn kernelType<T: QSysInfo_kernelType>(&mut self, value: T) -> i32 {
    value.kernelType(self);
    return 1;
  }
}

pub trait QSysInfo_kernelType {
  fn kernelType(self, this: &mut QSysInfo) -> i32;
}

// proto: QString QSysInfo::kernelType();
impl<'a> /*trait*/ QSysInfo_kernelType for () {
  fn kernelType(self, this: &mut QSysInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo10kernelTypeEv()};
    unsafe {_ZN8QSysInfo10kernelTypeEv()};
    return 1;
  }
}

impl /*struct*/ QSysInfo {
  pub fn productType<T: QSysInfo_productType>(&mut self, value: T) -> i32 {
    value.productType(self);
    return 1;
  }
}

pub trait QSysInfo_productType {
  fn productType(self, this: &mut QSysInfo) -> i32;
}

// proto: QString QSysInfo::productType();
impl<'a> /*trait*/ QSysInfo_productType for () {
  fn productType(self, this: &mut QSysInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo11productTypeEv()};
    unsafe {_ZN8QSysInfo11productTypeEv()};
    return 1;
  }
}

impl /*struct*/ QSysInfo {
  pub fn prettyProductName<T: QSysInfo_prettyProductName>(&mut self, value: T) -> i32 {
    value.prettyProductName(self);
    return 1;
  }
}

pub trait QSysInfo_prettyProductName {
  fn prettyProductName(self, this: &mut QSysInfo) -> i32;
}

// proto: QString QSysInfo::prettyProductName();
impl<'a> /*trait*/ QSysInfo_prettyProductName for () {
  fn prettyProductName(self, this: &mut QSysInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo17prettyProductNameEv()};
    unsafe {_ZN8QSysInfo17prettyProductNameEv()};
    return 1;
  }
}

impl /*struct*/ QSysInfo {
  pub fn currentCpuArchitecture<T: QSysInfo_currentCpuArchitecture>(&mut self, value: T) -> i32 {
    value.currentCpuArchitecture(self);
    return 1;
  }
}

pub trait QSysInfo_currentCpuArchitecture {
  fn currentCpuArchitecture(self, this: &mut QSysInfo) -> i32;
}

// proto: QString QSysInfo::currentCpuArchitecture();
impl<'a> /*trait*/ QSysInfo_currentCpuArchitecture for () {
  fn currentCpuArchitecture(self, this: &mut QSysInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo22currentCpuArchitectureEv()};
    unsafe {_ZN8QSysInfo22currentCpuArchitectureEv()};
    return 1;
  }
}

impl /*struct*/ QSysInfo {
  pub fn buildCpuArchitecture<T: QSysInfo_buildCpuArchitecture>(&mut self, value: T) -> i32 {
    value.buildCpuArchitecture(self);
    return 1;
  }
}

pub trait QSysInfo_buildCpuArchitecture {
  fn buildCpuArchitecture(self, this: &mut QSysInfo) -> i32;
}

// proto: QString QSysInfo::buildCpuArchitecture();
impl<'a> /*trait*/ QSysInfo_buildCpuArchitecture for () {
  fn buildCpuArchitecture(self, this: &mut QSysInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo20buildCpuArchitectureEv()};
    unsafe {_ZN8QSysInfo20buildCpuArchitectureEv()};
    return 1;
  }
}

impl /*struct*/ QSysInfo {
  pub fn kernelVersion<T: QSysInfo_kernelVersion>(&mut self, value: T) -> i32 {
    value.kernelVersion(self);
    return 1;
  }
}

pub trait QSysInfo_kernelVersion {
  fn kernelVersion(self, this: &mut QSysInfo) -> i32;
}

// proto: QString QSysInfo::kernelVersion();
impl<'a> /*trait*/ QSysInfo_kernelVersion for () {
  fn kernelVersion(self, this: &mut QSysInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo13kernelVersionEv()};
    unsafe {_ZN8QSysInfo13kernelVersionEv()};
    return 1;
  }
}

impl /*struct*/ QSysInfo {
  pub fn productVersion<T: QSysInfo_productVersion>(&mut self, value: T) -> i32 {
    value.productVersion(self);
    return 1;
  }
}

pub trait QSysInfo_productVersion {
  fn productVersion(self, this: &mut QSysInfo) -> i32;
}

// proto: QString QSysInfo::productVersion();
impl<'a> /*trait*/ QSysInfo_productVersion for () {
  fn productVersion(self, this: &mut QSysInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo14productVersionEv()};
    unsafe {_ZN8QSysInfo14productVersionEv()};
    return 1;
  }
}

impl /*struct*/ QSysInfo {
  pub fn buildAbi<T: QSysInfo_buildAbi>(&mut self, value: T) -> i32 {
    value.buildAbi(self);
    return 1;
  }
}

pub trait QSysInfo_buildAbi {
  fn buildAbi(self, this: &mut QSysInfo) -> i32;
}

// proto: QString QSysInfo::buildAbi();
impl<'a> /*trait*/ QSysInfo_buildAbi for () {
  fn buildAbi(self, this: &mut QSysInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QSysInfo8buildAbiEv()};
    unsafe {_ZN8QSysInfo8buildAbiEv()};
    return 1;
  }
}

