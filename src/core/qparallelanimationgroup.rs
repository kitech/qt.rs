// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
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
use super::qanimationgroup::QAnimationGroup; // 773
use std::ops::Deref;
use super::qobject::QObject; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QParallelAnimationGroup::~QParallelAnimationGroup();
  fn _ZN23QParallelAnimationGroupD0Ev(qthis: *mut c_void);
  // proto:  void QParallelAnimationGroup::QParallelAnimationGroup(const QParallelAnimationGroup & );
  fn _ZN23QParallelAnimationGroupC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QParallelAnimationGroup::duration();
  fn _ZNK23QParallelAnimationGroup8durationEv(qthis: *mut c_void) -> c_int;
  // proto:  void QParallelAnimationGroup::QParallelAnimationGroup(QObject * parent);
  fn _ZN23QParallelAnimationGroupC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QParallelAnimationGroup::metaObject();
  fn _ZNK23QParallelAnimationGroup10metaObjectEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QParallelAnimationGroup)=1
pub struct QParallelAnimationGroup {
  qbase: QAnimationGroup,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QParallelAnimationGroup {
  pub fn inheritFrom(qthis: *mut c_void) -> QParallelAnimationGroup {
    return QParallelAnimationGroup{qbase: QAnimationGroup::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QParallelAnimationGroup {
  type Target = QAnimationGroup;

  fn deref(&self) -> &QAnimationGroup {
    return &self.qbase;
  }
}
impl AsRef<QAnimationGroup> for QParallelAnimationGroup {
  fn as_ref(&self) -> &QAnimationGroup {
    return &self.qbase;
  }
}
  // proto:  void QParallelAnimationGroup::~QParallelAnimationGroup();
impl /*struct*/ QParallelAnimationGroup {
  pub fn FreeQParallelAnimationGroup<RetType, T: QParallelAnimationGroup_FreeQParallelAnimationGroup<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQParallelAnimationGroup(self);
    // return 1;
  }
}

pub trait QParallelAnimationGroup_FreeQParallelAnimationGroup<RetType> {
  fn FreeQParallelAnimationGroup(self , rsthis: &mut QParallelAnimationGroup) -> RetType;
}

  // proto:  void QParallelAnimationGroup::~QParallelAnimationGroup();
impl<'a> /*trait*/ QParallelAnimationGroup_FreeQParallelAnimationGroup<()> for () {
  fn FreeQParallelAnimationGroup(self , rsthis: &mut QParallelAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QParallelAnimationGroupD0Ev()};
     unsafe {_ZN23QParallelAnimationGroupD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QParallelAnimationGroup::QParallelAnimationGroup(const QParallelAnimationGroup & );
impl /*struct*/ QParallelAnimationGroup {
  pub fn NewQParallelAnimationGroup<T: QParallelAnimationGroup_NewQParallelAnimationGroup>(value: T) -> QParallelAnimationGroup {
    let rsthis = value.NewQParallelAnimationGroup();
    return rsthis;
    // return 1;
  }
}

pub trait QParallelAnimationGroup_NewQParallelAnimationGroup {
  fn NewQParallelAnimationGroup(self) -> QParallelAnimationGroup;
}

  // proto:  void QParallelAnimationGroup::QParallelAnimationGroup(const QParallelAnimationGroup & );
impl<'a> /*trait*/ QParallelAnimationGroup_NewQParallelAnimationGroup for (QParallelAnimationGroup) {
  fn NewQParallelAnimationGroup(self) -> QParallelAnimationGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QParallelAnimationGroupC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QParallelAnimationGroupC1ERKS_(qthis, arg0)};
    let rsthis = QParallelAnimationGroup{/**/qbase: QAnimationGroup::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QParallelAnimationGroup::duration();
impl /*struct*/ QParallelAnimationGroup {
  pub fn duration<RetType, T: QParallelAnimationGroup_duration<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.duration(self);
    // return 1;
  }
}

pub trait QParallelAnimationGroup_duration<RetType> {
  fn duration(self , rsthis: &mut QParallelAnimationGroup) -> RetType;
}

  // proto:  int QParallelAnimationGroup::duration();
impl<'a> /*trait*/ QParallelAnimationGroup_duration<i32> for () {
  fn duration(self , rsthis: &mut QParallelAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QParallelAnimationGroup8durationEv()};
    let mut ret = unsafe {_ZNK23QParallelAnimationGroup8durationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QParallelAnimationGroup::QParallelAnimationGroup(QObject * parent);
impl<'a> /*trait*/ QParallelAnimationGroup_NewQParallelAnimationGroup for (QObject) {
  fn NewQParallelAnimationGroup(self) -> QParallelAnimationGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QParallelAnimationGroupC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QParallelAnimationGroupC1EP7QObject(qthis, arg0)};
    let rsthis = QParallelAnimationGroup{/**/qbase: QAnimationGroup::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QParallelAnimationGroup::metaObject();
impl /*struct*/ QParallelAnimationGroup {
  pub fn metaObject<RetType, T: QParallelAnimationGroup_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QParallelAnimationGroup_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QParallelAnimationGroup) -> RetType;
}

  // proto:  const QMetaObject * QParallelAnimationGroup::metaObject();
impl<'a> /*trait*/ QParallelAnimationGroup_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QParallelAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QParallelAnimationGroup10metaObjectEv()};
     unsafe {_ZNK23QParallelAnimationGroup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

