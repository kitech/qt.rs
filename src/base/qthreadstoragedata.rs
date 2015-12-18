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
  // proto:  void ** QThreadStorageData::set(void * p);
  fn _ZN18QThreadStorageData3setEPv(qthis: *mut c_void, arg0: *mut uint8_t) ;
  // proto:  void ** QThreadStorageData::get();
  fn _ZNK18QThreadStorageData3getEv(qthis: *mut c_void) ;
  // proto: static void QThreadStorageData::finish(void ** );
  fn _ZN18QThreadStorageData6finishEPPv(arg0: *mut uint8_t) ;
  // proto:  void QThreadStorageData::FreeQThreadStorageData();
  fn _ZN18QThreadStorageDataD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QThreadStorageData)=4
pub struct QThreadStorageData {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QThreadStorageData {
  pub fn set<RetType, T: QThreadStorageData_set<RetType>>(&mut self, value: T) -> RetType {
    return value.set(self);
    // return 1;
  }
}

pub trait QThreadStorageData_set<RetType> {
  fn set(self, rsthis: &mut QThreadStorageData) -> RetType;
}

// proto:  void ** QThreadStorageData::set(void * p);
impl<'a> /*trait*/ QThreadStorageData_set<()> for (&'a mut u8) {
  fn set(self, rsthis: &mut QThreadStorageData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QThreadStorageData3setEPv()};
    let arg0 = self  as *mut uint8_t;
     unsafe {_ZN18QThreadStorageData3setEPv(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QThreadStorageData {
  pub fn get<RetType, T: QThreadStorageData_get<RetType>>(&mut self, value: T) -> RetType {
    return value.get(self);
    // return 1;
  }
}

pub trait QThreadStorageData_get<RetType> {
  fn get(self, rsthis: &mut QThreadStorageData) -> RetType;
}

// proto:  void ** QThreadStorageData::get();
impl<'a> /*trait*/ QThreadStorageData_get<()> for () {
  fn get(self, rsthis: &mut QThreadStorageData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QThreadStorageData3getEv()};
     unsafe {_ZNK18QThreadStorageData3getEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QThreadStorageData {
  pub fn finish<RetType, T: QThreadStorageData_finish<RetType>>(&mut self, value: T) -> RetType {
    return value.finish(self);
    // return 1;
  }
}

pub trait QThreadStorageData_finish<RetType> {
  fn finish(self, rsthis: &mut QThreadStorageData) -> RetType;
}

// proto: static void QThreadStorageData::finish(void ** );
impl<'a> /*trait*/ QThreadStorageData_finish<()> for (&'a mut u8) {
  fn finish(self, rsthis: &mut QThreadStorageData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QThreadStorageData6finishEPPv()};
    let arg0 = self  as *mut uint8_t;
     unsafe {_ZN18QThreadStorageData6finishEPPv(arg0)};
    // return 1;
  }
}

impl /*struct*/ QThreadStorageData {
  pub fn FreeQThreadStorageData<RetType, T: QThreadStorageData_FreeQThreadStorageData<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQThreadStorageData(self);
    // return 1;
  }
}

pub trait QThreadStorageData_FreeQThreadStorageData<RetType> {
  fn FreeQThreadStorageData(self, rsthis: &mut QThreadStorageData) -> RetType;
}

// proto:  void QThreadStorageData::FreeQThreadStorageData();
impl<'a> /*trait*/ QThreadStorageData_FreeQThreadStorageData<()> for () {
  fn FreeQThreadStorageData(self, rsthis: &mut QThreadStorageData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QThreadStorageDataD0Ev()};
     unsafe {_ZN18QThreadStorageDataD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

