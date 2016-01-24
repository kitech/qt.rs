// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtCore/qhash.h
// dst-file: /src/core/qhash.rs
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
  fn QHashDummyValue_Class_Size() -> c_int;
  fn QHashData_Class_Size() -> c_int;
  // proto:  void QHashData::hasShrunk();
  fn C_ZN9QHashData9hasShrunkEv(qthis: u64 /* *mut c_void*/);
  // proto:  void * QHashData::allocateNode(int nodeAlign);
  fn C_ZN9QHashData12allocateNodeEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  bool QHashData::willGrow();
  fn C_ZN9QHashData8willGrowEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QHashData::rehash(int hint);
  fn C_ZN9QHashData6rehashEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QHashData::freeNode(void * node);
  fn C_ZN9QHashData8freeNodeEPv(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QHashDummyValue)=1
#[derive(Default)]
pub struct QHashDummyValue {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QHashData)=1
#[derive(Default)]
pub struct QHashData {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QHashDummyValue {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QHashDummyValue {
    return QHashDummyValue{qclsinst: qthis, ..Default::default()};
  }
}
impl /*struct*/ QHashData {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QHashData {
    return QHashData{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QHashData::hasShrunk();
impl /*struct*/ QHashData {
  pub fn hasShrunk<RetType, T: QHashData_hasShrunk<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasShrunk(self);
    // return 1;
  }
}

pub trait QHashData_hasShrunk<RetType> {
  fn hasShrunk(self , rsthis: & QHashData) -> RetType;
}

  // proto:  void QHashData::hasShrunk();
impl<'a> /*trait*/ QHashData_hasShrunk<()> for () {
  fn hasShrunk(self , rsthis: & QHashData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHashData9hasShrunkEv()};
     unsafe {C_ZN9QHashData9hasShrunkEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void * QHashData::allocateNode(int nodeAlign);
impl /*struct*/ QHashData {
  pub fn allocateNode<RetType, T: QHashData_allocateNode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.allocateNode(self);
    // return 1;
  }
}

pub trait QHashData_allocateNode<RetType> {
  fn allocateNode(self , rsthis: & QHashData) -> RetType;
}

  // proto:  void * QHashData::allocateNode(int nodeAlign);
impl<'a> /*trait*/ QHashData_allocateNode<*mut c_void> for (i32) {
  fn allocateNode(self , rsthis: & QHashData) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHashData12allocateNodeEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN9QHashData12allocateNodeEi(rsthis.qclsinst, arg0)};
    return ret as *mut c_void;
    // return 1;
  }
}

  // proto:  bool QHashData::willGrow();
impl /*struct*/ QHashData {
  pub fn willGrow<RetType, T: QHashData_willGrow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.willGrow(self);
    // return 1;
  }
}

pub trait QHashData_willGrow<RetType> {
  fn willGrow(self , rsthis: & QHashData) -> RetType;
}

  // proto:  bool QHashData::willGrow();
impl<'a> /*trait*/ QHashData_willGrow<i8> for () {
  fn willGrow(self , rsthis: & QHashData) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHashData8willGrowEv()};
    let mut ret = unsafe {C_ZN9QHashData8willGrowEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QHashData::rehash(int hint);
impl /*struct*/ QHashData {
  pub fn rehash<RetType, T: QHashData_rehash<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rehash(self);
    // return 1;
  }
}

pub trait QHashData_rehash<RetType> {
  fn rehash(self , rsthis: & QHashData) -> RetType;
}

  // proto:  void QHashData::rehash(int hint);
impl<'a> /*trait*/ QHashData_rehash<()> for (i32) {
  fn rehash(self , rsthis: & QHashData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHashData6rehashEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN9QHashData6rehashEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QHashData::freeNode(void * node);
impl /*struct*/ QHashData {
  pub fn freeNode<RetType, T: QHashData_freeNode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.freeNode(self);
    // return 1;
  }
}

pub trait QHashData_freeNode<RetType> {
  fn freeNode(self , rsthis: & QHashData) -> RetType;
}

  // proto:  void QHashData::freeNode(void * node);
impl<'a> /*trait*/ QHashData_freeNode<()> for (*mut c_void) {
  fn freeNode(self , rsthis: & QHashData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QHashData8freeNodeEPv()};
    let arg0 = self  as *mut c_void;
     unsafe {C_ZN9QHashData8freeNodeEPv(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

