// auto generated, do not modify.
// created: Sat Dec 26 12:15:38 2015
// src-file: /QtCore/qarraydata.h
// dst-file: /src/core/qarraydata.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use std::ops::Deref;
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QArrayData_Class_Size() -> c_int;
  // proto: static QArrayData * QArrayData::sharedNull();
  fn _ZN10QArrayData10sharedNullEv() -> *mut c_void;
  // proto:  void * QArrayData::data();
  fn _ZN10QArrayData4dataEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static void QArrayData::deallocate(QArrayData * data, int objectSize, int alignment);
  fn _ZN10QArrayData10deallocateEPS_ii(arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  bool QArrayData::isMutable();
  fn _ZNK10QArrayData9isMutableEv(qthis: *mut c_void) -> c_char;
  // proto:  int QArrayData::detachCapacity(int newSize);
  fn _ZNK10QArrayData14detachCapacityEi(qthis: *mut c_void, arg0: c_int) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QArrayData)=1
pub struct QArrayData {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QArrayData {
  pub fn inheritFrom(qthis: *mut c_void) -> QArrayData {
    return QArrayData{qclsinst: qthis};
  }
}
  // proto: static QArrayData * QArrayData::sharedNull();
impl /*struct*/ QArrayData {
  pub fn sharedNull_s<RetType, T: QArrayData_sharedNull_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.sharedNull_s();
    // return 1;
  }
}

pub trait QArrayData_sharedNull_s<RetType> {
  fn sharedNull_s(self ) -> RetType;
}

  // proto: static QArrayData * QArrayData::sharedNull();
impl<'a> /*trait*/ QArrayData_sharedNull_s<QArrayData> for () {
  fn sharedNull_s(self ) -> QArrayData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QArrayData10sharedNullEv()};
    let mut ret = unsafe {_ZN10QArrayData10sharedNullEv()};
    let mut ret1 = QArrayData::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void * QArrayData::data();
impl /*struct*/ QArrayData {
  pub fn data<RetType, T: QArrayData_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QArrayData_data<RetType> {
  fn data(self , rsthis: & QArrayData) -> RetType;
}

  // proto:  void * QArrayData::data();
impl<'a> /*trait*/ QArrayData_data<*mut c_void> for () {
  fn data(self , rsthis: & QArrayData) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QArrayData4dataEv()};
    let mut ret = unsafe {_ZN10QArrayData4dataEv(rsthis.qclsinst)};
    return ret as *mut c_void;
    // return 1;
  }
}

  // proto: static void QArrayData::deallocate(QArrayData * data, int objectSize, int alignment);
impl /*struct*/ QArrayData {
  pub fn deallocate_s<RetType, T: QArrayData_deallocate_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.deallocate_s();
    // return 1;
  }
}

pub trait QArrayData_deallocate_s<RetType> {
  fn deallocate_s(self ) -> RetType;
}

  // proto: static void QArrayData::deallocate(QArrayData * data, int objectSize, int alignment);
impl<'a> /*trait*/ QArrayData_deallocate_s<()> for (&'a QArrayData, i32, i32) {
  fn deallocate_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QArrayData10deallocateEPS_ii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN10QArrayData10deallocateEPS_ii(arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  bool QArrayData::isMutable();
impl /*struct*/ QArrayData {
  pub fn isMutable<RetType, T: QArrayData_isMutable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isMutable(self);
    // return 1;
  }
}

pub trait QArrayData_isMutable<RetType> {
  fn isMutable(self , rsthis: & QArrayData) -> RetType;
}

  // proto:  bool QArrayData::isMutable();
impl<'a> /*trait*/ QArrayData_isMutable<i8> for () {
  fn isMutable(self , rsthis: & QArrayData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QArrayData9isMutableEv()};
    let mut ret = unsafe {_ZNK10QArrayData9isMutableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QArrayData::detachCapacity(int newSize);
impl /*struct*/ QArrayData {
  pub fn detachCapacity<RetType, T: QArrayData_detachCapacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.detachCapacity(self);
    // return 1;
  }
}

pub trait QArrayData_detachCapacity<RetType> {
  fn detachCapacity(self , rsthis: & QArrayData) -> RetType;
}

  // proto:  int QArrayData::detachCapacity(int newSize);
impl<'a> /*trait*/ QArrayData_detachCapacity<i32> for (i32) {
  fn detachCapacity(self , rsthis: & QArrayData) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QArrayData14detachCapacityEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QArrayData14detachCapacityEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// <= body block end

