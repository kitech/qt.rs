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
  fn _ZN14QMetaClassInfoC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK14QMetaClassInfo19enclosingMetaObjectEv() -> i32;
  fn _ZNK14QMetaClassInfo4nameEv() -> i32;
  fn _ZNK14QMetaClassInfo5valueEv() -> i32;
}

// body block begin
// class sizeof(QMetaClassInfo)=16
pub struct QMetaClassInfo {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMetaClassInfo {
  pub fn NewQMetaClassInfo<T: QMetaClassInfo_NewQMetaClassInfo>(value: T) -> QMetaClassInfo {
    let rsthis = value.NewQMetaClassInfo();
    return rsthis;
    // return 1;
  }
}

pub trait QMetaClassInfo_NewQMetaClassInfo {
  fn NewQMetaClassInfo(self) -> QMetaClassInfo;
}

// proto: void QMetaClassInfo::NewQMetaClassInfo();
impl<'a> /*trait*/ QMetaClassInfo_NewQMetaClassInfo for () {
  fn NewQMetaClassInfo(self) -> QMetaClassInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QMetaClassInfoC1Ev()};
    unsafe {_ZN14QMetaClassInfoC1Ev(qthis)};
    let rsthis = QMetaClassInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMetaClassInfo {
  pub fn enclosingMetaObject<T: QMetaClassInfo_enclosingMetaObject>(&mut self, value: T) -> i32 {
    value.enclosingMetaObject(self);
    return 1;
  }
}

pub trait QMetaClassInfo_enclosingMetaObject {
  fn enclosingMetaObject(self, this: &mut QMetaClassInfo) -> i32;
}

// proto: const QMetaObject * QMetaClassInfo::enclosingMetaObject();
impl<'a> /*trait*/ QMetaClassInfo_enclosingMetaObject for () {
  fn enclosingMetaObject(self, this: &mut QMetaClassInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMetaClassInfo19enclosingMetaObjectEv()};
    unsafe {_ZNK14QMetaClassInfo19enclosingMetaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QMetaClassInfo {
  pub fn name<T: QMetaClassInfo_name>(&mut self, value: T) -> i32 {
    value.name(self);
    return 1;
  }
}

pub trait QMetaClassInfo_name {
  fn name(self, this: &mut QMetaClassInfo) -> i32;
}

// proto: const char * QMetaClassInfo::name();
impl<'a> /*trait*/ QMetaClassInfo_name for () {
  fn name(self, this: &mut QMetaClassInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMetaClassInfo4nameEv()};
    unsafe {_ZNK14QMetaClassInfo4nameEv()};
    return 1;
  }
}

impl /*struct*/ QMetaClassInfo {
  pub fn value<T: QMetaClassInfo_value>(&mut self, value: T) -> i32 {
    value.value(self);
    return 1;
  }
}

pub trait QMetaClassInfo_value {
  fn value(self, this: &mut QMetaClassInfo) -> i32;
}

// proto: const char * QMetaClassInfo::value();
impl<'a> /*trait*/ QMetaClassInfo_value for () {
  fn value(self, this: &mut QMetaClassInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QMetaClassInfo5valueEv()};
    unsafe {_ZNK14QMetaClassInfo5valueEv()};
    return 1;
  }
}

