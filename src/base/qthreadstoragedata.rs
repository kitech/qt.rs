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
  fn _ZN18QThreadStorageData3setEPv(arg0: *mut uint8_t) -> i32;
  fn _ZNK18QThreadStorageData3getEv() -> i32;
  fn _ZN18QThreadStorageData6finishEPPv(arg0: *mut uint8_t) -> i32;
  fn _ZN18QThreadStorageDataD0Ev() -> i32;
}

// body block begin
// class sizeof(QThreadStorageData)=4
pub struct QThreadStorageData {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QThreadStorageData {
  pub fn set<T: QThreadStorageData_set>(&mut self, value: T) -> i32 {
    value.set(self);
    return 1;
  }
}

pub trait QThreadStorageData_set {
  fn set(self, this: &mut QThreadStorageData) -> i32;
}

// proto: void ** QThreadStorageData::set(void * p);
impl<'a> /*trait*/ QThreadStorageData_set for (&'a mut u8) {
  fn set(self, this: &mut QThreadStorageData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QThreadStorageData3setEPv()};
    let arg0 = self  as *mut uint8_t;
    unsafe {_ZN18QThreadStorageData3setEPv(arg0)};
    return 1;
  }
}

impl /*struct*/ QThreadStorageData {
  pub fn get<T: QThreadStorageData_get>(&mut self, value: T) -> i32 {
    value.get(self);
    return 1;
  }
}

pub trait QThreadStorageData_get {
  fn get(self, this: &mut QThreadStorageData) -> i32;
}

// proto: void ** QThreadStorageData::get();
impl<'a> /*trait*/ QThreadStorageData_get for () {
  fn get(self, this: &mut QThreadStorageData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QThreadStorageData3getEv()};
    unsafe {_ZNK18QThreadStorageData3getEv()};
    return 1;
  }
}

impl /*struct*/ QThreadStorageData {
  pub fn finish<T: QThreadStorageData_finish>(&mut self, value: T) -> i32 {
    value.finish(self);
    return 1;
  }
}

pub trait QThreadStorageData_finish {
  fn finish(self, this: &mut QThreadStorageData) -> i32;
}

// proto: void QThreadStorageData::finish(void ** );
impl<'a> /*trait*/ QThreadStorageData_finish for (&'a mut u8) {
  fn finish(self, this: &mut QThreadStorageData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QThreadStorageData6finishEPPv()};
    let arg0 = self  as *mut uint8_t;
    unsafe {_ZN18QThreadStorageData6finishEPPv(arg0)};
    return 1;
  }
}

impl /*struct*/ QThreadStorageData {
  pub fn FreeQThreadStorageData<T: QThreadStorageData_FreeQThreadStorageData>(&mut self, value: T) -> i32 {
    value.FreeQThreadStorageData(self);
    return 1;
  }
}

pub trait QThreadStorageData_FreeQThreadStorageData {
  fn FreeQThreadStorageData(self, this: &mut QThreadStorageData) -> i32;
}

// proto: void QThreadStorageData::FreeQThreadStorageData();
impl<'a> /*trait*/ QThreadStorageData_FreeQThreadStorageData for () {
  fn FreeQThreadStorageData(self, this: &mut QThreadStorageData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QThreadStorageDataD0Ev()};
    unsafe {_ZN18QThreadStorageDataD0Ev()};
    return 1;
  }
}

