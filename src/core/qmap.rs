// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
// src-file: /QtCore/qmap.h
// dst-file: /src/core/qmap.rs
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
// use super::qmap::QMapNodeBase; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QMapDataBase_Class_Size() -> c_int;
  // proto:  void QMapDataBase::rebalance(QMapNodeBase * x);
  fn _ZN12QMapDataBase9rebalanceEP12QMapNodeBase(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QMapDataBase * QMapDataBase::createData();
  fn _ZN12QMapDataBase10createDataEv() -> *mut c_void;
  // proto:  void QMapDataBase::rotateRight(QMapNodeBase * x);
  fn _ZN12QMapDataBase11rotateRightEP12QMapNodeBase(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QMapDataBase::freeTree(QMapNodeBase * root, int alignment);
  fn _ZN12QMapDataBase8freeTreeEP12QMapNodeBasei(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto: static void QMapDataBase::freeData(QMapDataBase * d);
  fn _ZN12QMapDataBase8freeDataEPS_(arg0: *mut c_void);
  // proto:  void QMapDataBase::rotateLeft(QMapNodeBase * x);
  fn _ZN12QMapDataBase10rotateLeftEP12QMapNodeBase(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QMapDataBase::recalcMostLeftNode();
  fn _ZN12QMapDataBase18recalcMostLeftNodeEv(qthis: u64 /* *mut c_void*/);
  // proto:  QMapNodeBase * QMapDataBase::createNode(int size, int alignment, QMapNodeBase * parent, bool left);
  fn _ZN12QMapDataBase10createNodeEiiP12QMapNodeBaseb(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void, arg3: c_char) -> *mut c_void;
  // proto:  void QMapDataBase::freeNodeAndRebalance(QMapNodeBase * z);
  fn _ZN12QMapDataBase20freeNodeAndRebalanceEP12QMapNodeBase(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QMapNodeBase_Class_Size() -> c_int;
  // proto:  void QMapNodeBase::setParent(QMapNodeBase * pp);
  fn _ZN12QMapNodeBase9setParentEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QMapNodeBase * QMapNodeBase::previousNode();
  fn _ZN12QMapNodeBase12previousNodeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QMapNodeBase * QMapNodeBase::nextNode();
  fn _ZN12QMapNodeBase8nextNodeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QMapNodeBase * QMapNodeBase::parent();
  fn _ZNK12QMapNodeBase6parentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QMapDataBase)=1
#[derive(Default)]
pub struct QMapDataBase {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QMapNodeBase)=24
#[derive(Default)]
pub struct QMapNodeBase {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QMapDataBase {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QMapDataBase {
    return QMapDataBase{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QMapDataBase::rebalance(QMapNodeBase * x);
impl /*struct*/ QMapDataBase {
  pub fn rebalance<RetType, T: QMapDataBase_rebalance<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rebalance(self);
    // return 1;
  }
}

pub trait QMapDataBase_rebalance<RetType> {
  fn rebalance(self , rsthis: & QMapDataBase) -> RetType;
}

  // proto:  void QMapDataBase::rebalance(QMapNodeBase * x);
impl<'a> /*trait*/ QMapDataBase_rebalance<()> for (&'a QMapNodeBase) {
  fn rebalance(self , rsthis: & QMapDataBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMapDataBase9rebalanceEP12QMapNodeBase()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QMapDataBase9rebalanceEP12QMapNodeBase(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QMapDataBase * QMapDataBase::createData();
impl /*struct*/ QMapDataBase {
  pub fn createData_s<RetType, T: QMapDataBase_createData_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.createData_s();
    // return 1;
  }
}

pub trait QMapDataBase_createData_s<RetType> {
  fn createData_s(self ) -> RetType;
}

  // proto: static QMapDataBase * QMapDataBase::createData();
impl<'a> /*trait*/ QMapDataBase_createData_s<QMapDataBase> for () {
  fn createData_s(self ) -> QMapDataBase {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMapDataBase10createDataEv()};
    let mut ret = unsafe {_ZN12QMapDataBase10createDataEv()};
    let mut ret1 = QMapDataBase::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMapDataBase::rotateRight(QMapNodeBase * x);
impl /*struct*/ QMapDataBase {
  pub fn rotateRight<RetType, T: QMapDataBase_rotateRight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rotateRight(self);
    // return 1;
  }
}

pub trait QMapDataBase_rotateRight<RetType> {
  fn rotateRight(self , rsthis: & QMapDataBase) -> RetType;
}

  // proto:  void QMapDataBase::rotateRight(QMapNodeBase * x);
impl<'a> /*trait*/ QMapDataBase_rotateRight<()> for (&'a QMapNodeBase) {
  fn rotateRight(self , rsthis: & QMapDataBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMapDataBase11rotateRightEP12QMapNodeBase()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QMapDataBase11rotateRightEP12QMapNodeBase(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMapDataBase::freeTree(QMapNodeBase * root, int alignment);
impl /*struct*/ QMapDataBase {
  pub fn freeTree<RetType, T: QMapDataBase_freeTree<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.freeTree(self);
    // return 1;
  }
}

pub trait QMapDataBase_freeTree<RetType> {
  fn freeTree(self , rsthis: & QMapDataBase) -> RetType;
}

  // proto:  void QMapDataBase::freeTree(QMapNodeBase * root, int alignment);
impl<'a> /*trait*/ QMapDataBase_freeTree<()> for (&'a QMapNodeBase, i32) {
  fn freeTree(self , rsthis: & QMapDataBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMapDataBase8freeTreeEP12QMapNodeBasei()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QMapDataBase8freeTreeEP12QMapNodeBasei(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto: static void QMapDataBase::freeData(QMapDataBase * d);
impl /*struct*/ QMapDataBase {
  pub fn freeData_s<RetType, T: QMapDataBase_freeData_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.freeData_s();
    // return 1;
  }
}

pub trait QMapDataBase_freeData_s<RetType> {
  fn freeData_s(self ) -> RetType;
}

  // proto: static void QMapDataBase::freeData(QMapDataBase * d);
impl<'a> /*trait*/ QMapDataBase_freeData_s<()> for (&'a QMapDataBase) {
  fn freeData_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMapDataBase8freeDataEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QMapDataBase8freeDataEPS_(arg0)};
    // return 1;
  }
}

  // proto:  void QMapDataBase::rotateLeft(QMapNodeBase * x);
impl /*struct*/ QMapDataBase {
  pub fn rotateLeft<RetType, T: QMapDataBase_rotateLeft<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rotateLeft(self);
    // return 1;
  }
}

pub trait QMapDataBase_rotateLeft<RetType> {
  fn rotateLeft(self , rsthis: & QMapDataBase) -> RetType;
}

  // proto:  void QMapDataBase::rotateLeft(QMapNodeBase * x);
impl<'a> /*trait*/ QMapDataBase_rotateLeft<()> for (&'a QMapNodeBase) {
  fn rotateLeft(self , rsthis: & QMapDataBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMapDataBase10rotateLeftEP12QMapNodeBase()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QMapDataBase10rotateLeftEP12QMapNodeBase(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMapDataBase::recalcMostLeftNode();
impl /*struct*/ QMapDataBase {
  pub fn recalcMostLeftNode<RetType, T: QMapDataBase_recalcMostLeftNode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.recalcMostLeftNode(self);
    // return 1;
  }
}

pub trait QMapDataBase_recalcMostLeftNode<RetType> {
  fn recalcMostLeftNode(self , rsthis: & QMapDataBase) -> RetType;
}

  // proto:  void QMapDataBase::recalcMostLeftNode();
impl<'a> /*trait*/ QMapDataBase_recalcMostLeftNode<()> for () {
  fn recalcMostLeftNode(self , rsthis: & QMapDataBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMapDataBase18recalcMostLeftNodeEv()};
     unsafe {_ZN12QMapDataBase18recalcMostLeftNodeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QMapNodeBase * QMapDataBase::createNode(int size, int alignment, QMapNodeBase * parent, bool left);
impl /*struct*/ QMapDataBase {
  pub fn createNode<RetType, T: QMapDataBase_createNode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createNode(self);
    // return 1;
  }
}

pub trait QMapDataBase_createNode<RetType> {
  fn createNode(self , rsthis: & QMapDataBase) -> RetType;
}

  // proto:  QMapNodeBase * QMapDataBase::createNode(int size, int alignment, QMapNodeBase * parent, bool left);
impl<'a> /*trait*/ QMapDataBase_createNode<QMapNodeBase> for (i32, i32, &'a QMapNodeBase, i8) {
  fn createNode(self , rsthis: & QMapDataBase) -> QMapNodeBase {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMapDataBase10createNodeEiiP12QMapNodeBaseb()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_char;
    let mut ret = unsafe {_ZN12QMapDataBase10createNodeEiiP12QMapNodeBaseb(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QMapNodeBase::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMapDataBase::freeNodeAndRebalance(QMapNodeBase * z);
impl /*struct*/ QMapDataBase {
  pub fn freeNodeAndRebalance<RetType, T: QMapDataBase_freeNodeAndRebalance<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.freeNodeAndRebalance(self);
    // return 1;
  }
}

pub trait QMapDataBase_freeNodeAndRebalance<RetType> {
  fn freeNodeAndRebalance(self , rsthis: & QMapDataBase) -> RetType;
}

  // proto:  void QMapDataBase::freeNodeAndRebalance(QMapNodeBase * z);
impl<'a> /*trait*/ QMapDataBase_freeNodeAndRebalance<()> for (&'a QMapNodeBase) {
  fn freeNodeAndRebalance(self , rsthis: & QMapDataBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMapDataBase20freeNodeAndRebalanceEP12QMapNodeBase()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QMapDataBase20freeNodeAndRebalanceEP12QMapNodeBase(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMapNodeBase {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QMapNodeBase {
    return QMapNodeBase{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QMapNodeBase::setParent(QMapNodeBase * pp);
impl /*struct*/ QMapNodeBase {
  pub fn setParent<RetType, T: QMapNodeBase_setParent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setParent(self);
    // return 1;
  }
}

pub trait QMapNodeBase_setParent<RetType> {
  fn setParent(self , rsthis: & QMapNodeBase) -> RetType;
}

  // proto:  void QMapNodeBase::setParent(QMapNodeBase * pp);
impl<'a> /*trait*/ QMapNodeBase_setParent<()> for (&'a QMapNodeBase) {
  fn setParent(self , rsthis: & QMapNodeBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMapNodeBase9setParentEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QMapNodeBase9setParentEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QMapNodeBase * QMapNodeBase::previousNode();
impl /*struct*/ QMapNodeBase {
  pub fn previousNode<RetType, T: QMapNodeBase_previousNode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.previousNode(self);
    // return 1;
  }
}

pub trait QMapNodeBase_previousNode<RetType> {
  fn previousNode(self , rsthis: & QMapNodeBase) -> RetType;
}

  // proto:  QMapNodeBase * QMapNodeBase::previousNode();
impl<'a> /*trait*/ QMapNodeBase_previousNode<QMapNodeBase> for () {
  fn previousNode(self , rsthis: & QMapNodeBase) -> QMapNodeBase {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMapNodeBase12previousNodeEv()};
    let mut ret = unsafe {_ZN12QMapNodeBase12previousNodeEv(rsthis.qclsinst)};
    let mut ret1 = QMapNodeBase::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QMapNodeBase * QMapNodeBase::nextNode();
impl /*struct*/ QMapNodeBase {
  pub fn nextNode<RetType, T: QMapNodeBase_nextNode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nextNode(self);
    // return 1;
  }
}

pub trait QMapNodeBase_nextNode<RetType> {
  fn nextNode(self , rsthis: & QMapNodeBase) -> RetType;
}

  // proto:  QMapNodeBase * QMapNodeBase::nextNode();
impl<'a> /*trait*/ QMapNodeBase_nextNode<QMapNodeBase> for () {
  fn nextNode(self , rsthis: & QMapNodeBase) -> QMapNodeBase {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QMapNodeBase8nextNodeEv()};
    let mut ret = unsafe {_ZN12QMapNodeBase8nextNodeEv(rsthis.qclsinst)};
    let mut ret1 = QMapNodeBase::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QMapNodeBase * QMapNodeBase::parent();
impl /*struct*/ QMapNodeBase {
  pub fn parent<RetType, T: QMapNodeBase_parent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QMapNodeBase_parent<RetType> {
  fn parent(self , rsthis: & QMapNodeBase) -> RetType;
}

  // proto:  QMapNodeBase * QMapNodeBase::parent();
impl<'a> /*trait*/ QMapNodeBase_parent<QMapNodeBase> for () {
  fn parent(self , rsthis: & QMapNodeBase) -> QMapNodeBase {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QMapNodeBase6parentEv()};
    let mut ret = unsafe {_ZNK12QMapNodeBase6parentEv(rsthis.qclsinst)};
    let mut ret1 = QMapNodeBase::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

