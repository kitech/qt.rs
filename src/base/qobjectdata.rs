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
  fn _ZNK11QObjectData17dynamicMetaObjectEv() -> i32;
  fn _ZN11QObjectDataD0Ev() -> i32;
}

// body block begin
// class sizeof(QObjectData)=1
pub struct QObjectData {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QObjectData {
  pub fn dynamicMetaObject<T: QObjectData_dynamicMetaObject>(&mut self, value: T) -> i32 {
    value.dynamicMetaObject(self);
    return 1;
  }
}

pub trait QObjectData_dynamicMetaObject {
  fn dynamicMetaObject(self, this: &mut QObjectData) -> i32;
}

// proto: QMetaObject * QObjectData::dynamicMetaObject();
impl<'a> /*trait*/ QObjectData_dynamicMetaObject for () {
  fn dynamicMetaObject(self, this: &mut QObjectData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QObjectData17dynamicMetaObjectEv()};
    unsafe {_ZNK11QObjectData17dynamicMetaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QObjectData {
  pub fn FreeQObjectData<T: QObjectData_FreeQObjectData>(&mut self, value: T) -> i32 {
    value.FreeQObjectData(self);
    return 1;
  }
}

pub trait QObjectData_FreeQObjectData {
  fn FreeQObjectData(self, this: &mut QObjectData) -> i32;
}

// proto: void QObjectData::FreeQObjectData();
impl<'a> /*trait*/ QObjectData_FreeQObjectData for () {
  fn FreeQObjectData(self, this: &mut QObjectData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QObjectDataD0Ev()};
    unsafe {_ZN11QObjectDataD0Ev()};
    return 1;
  }
}

