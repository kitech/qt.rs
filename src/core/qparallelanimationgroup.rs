// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtCore/qparallelanimationgroup.h
// dst-file: /src/core/qparallelanimationgroup.rs
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
use super::qanimationgroup::*; // 773
use std::ops::Deref;
use super::qobject::*; // 773
use super::qobjectdefs::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QParallelAnimationGroup_Class_Size() -> c_int;
  // proto:  void QParallelAnimationGroup::~QParallelAnimationGroup();
  fn C_ZN23QParallelAnimationGroupD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QParallelAnimationGroup::duration();
  fn C_ZNK23QParallelAnimationGroup8durationEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QParallelAnimationGroup::QParallelAnimationGroup(QObject * parent);
  fn C_ZN23QParallelAnimationGroupC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  const QMetaObject * QParallelAnimationGroup::metaObject();
  fn C_ZNK23QParallelAnimationGroup10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QParallelAnimationGroup)=1
#[derive(Default)]
pub struct QParallelAnimationGroup {
  qbase: QAnimationGroup,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QParallelAnimationGroup {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QParallelAnimationGroup {
    return QParallelAnimationGroup{qbase: QAnimationGroup::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QParallelAnimationGroup {
  type Target = QAnimationGroup;

  fn deref(&self) -> &QAnimationGroup {
    return & self.qbase;
  }
}
impl AsRef<QAnimationGroup> for QParallelAnimationGroup {
  fn as_ref(& self) -> & QAnimationGroup {
    return & self.qbase;
  }
}
  // proto:  void QParallelAnimationGroup::~QParallelAnimationGroup();
impl /*struct*/ QParallelAnimationGroup {
  pub fn free<RetType, T: QParallelAnimationGroup_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QParallelAnimationGroup_free<RetType> {
  fn free(self , rsthis: & QParallelAnimationGroup) -> RetType;
}

  // proto:  void QParallelAnimationGroup::~QParallelAnimationGroup();
impl<'a> /*trait*/ QParallelAnimationGroup_free<()> for () {
  fn free(self , rsthis: & QParallelAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QParallelAnimationGroupD2Ev()};
     unsafe {C_ZN23QParallelAnimationGroupD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QParallelAnimationGroup::duration();
impl /*struct*/ QParallelAnimationGroup {
  pub fn duration<RetType, T: QParallelAnimationGroup_duration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.duration(self);
    // return 1;
  }
}

pub trait QParallelAnimationGroup_duration<RetType> {
  fn duration(self , rsthis: & QParallelAnimationGroup) -> RetType;
}

  // proto:  int QParallelAnimationGroup::duration();
impl<'a> /*trait*/ QParallelAnimationGroup_duration<i32> for () {
  fn duration(self , rsthis: & QParallelAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QParallelAnimationGroup8durationEv()};
    let mut ret = unsafe {C_ZNK23QParallelAnimationGroup8durationEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QParallelAnimationGroup::QParallelAnimationGroup(QObject * parent);
impl /*struct*/ QParallelAnimationGroup {
  pub fn new<T: QParallelAnimationGroup_new>(value: T) -> QParallelAnimationGroup {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QParallelAnimationGroup_new {
  fn new(self) -> QParallelAnimationGroup;
}

  // proto:  void QParallelAnimationGroup::QParallelAnimationGroup(QObject * parent);
impl<'a> /*trait*/ QParallelAnimationGroup_new for (&'a QObject) {
  fn new(self) -> QParallelAnimationGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QParallelAnimationGroupC2EP7QObject()};
    let ctysz: c_int = unsafe{QParallelAnimationGroup_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN23QParallelAnimationGroupC2EP7QObject(arg0)};
    let rsthis = QParallelAnimationGroup{qbase: QAnimationGroup::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QParallelAnimationGroup::metaObject();
impl /*struct*/ QParallelAnimationGroup {
  pub fn metaObject<RetType, T: QParallelAnimationGroup_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QParallelAnimationGroup_metaObject<RetType> {
  fn metaObject(self , rsthis: & QParallelAnimationGroup) -> RetType;
}

  // proto:  const QMetaObject * QParallelAnimationGroup::metaObject();
impl<'a> /*trait*/ QParallelAnimationGroup_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QParallelAnimationGroup) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QParallelAnimationGroup10metaObjectEv()};
    let mut ret = unsafe {C_ZNK23QParallelAnimationGroup10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

