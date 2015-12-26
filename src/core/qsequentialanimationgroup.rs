// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtCore/qsequentialanimationgroup.h
// dst-file: /src/core/qsequentialanimationgroup.rs
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
use super::qpauseanimation::QPauseAnimation; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSequentialAnimationGroup_Class_Size() -> c_int;
  // proto:  void QSequentialAnimationGroup::QSequentialAnimationGroup(QObject * parent);
  fn dector_ZN25QSequentialAnimationGroupC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN25QSequentialAnimationGroupC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPauseAnimation * QSequentialAnimationGroup::insertPause(int index, int msecs);
  fn _ZN25QSequentialAnimationGroup11insertPauseEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  QPauseAnimation * QSequentialAnimationGroup::addPause(int msecs);
  fn _ZN25QSequentialAnimationGroup8addPauseEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  const QMetaObject * QSequentialAnimationGroup::metaObject();
  fn _ZNK25QSequentialAnimationGroup10metaObjectEv(qthis: *mut c_void);
  // proto:  void QSequentialAnimationGroup::QSequentialAnimationGroup(const QSequentialAnimationGroup & );
  fn dector_ZN25QSequentialAnimationGroupC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN25QSequentialAnimationGroupC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSequentialAnimationGroup::~QSequentialAnimationGroup();
  fn _ZN25QSequentialAnimationGroupD0Ev(qthis: *mut c_void);
  // proto:  QAbstractAnimation * QSequentialAnimationGroup::currentAnimation();
  fn _ZNK25QSequentialAnimationGroup16currentAnimationEv(qthis: *mut c_void);
  // proto:  int QSequentialAnimationGroup::duration();
  fn _ZNK25QSequentialAnimationGroup8durationEv(qthis: *mut c_void) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QSequentialAnimationGroup)=1
pub struct QSequentialAnimationGroup {
  qbase: QAnimationGroup,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSequentialAnimationGroup {
  pub fn inheritFrom(qthis: *mut c_void) -> QSequentialAnimationGroup {
    return QSequentialAnimationGroup{qbase: QAnimationGroup::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QSequentialAnimationGroup {
  type Target = QAnimationGroup;

  fn deref(&self) -> &QAnimationGroup {
    return & self.qbase;
  }
}
impl AsRef<QAnimationGroup> for QSequentialAnimationGroup {
  fn as_ref(& self) -> & QAnimationGroup {
    return & self.qbase;
  }
}
  // proto:  void QSequentialAnimationGroup::QSequentialAnimationGroup(QObject * parent);
impl /*struct*/ QSequentialAnimationGroup {
  pub fn New<T: QSequentialAnimationGroup_New>(value: T) -> QSequentialAnimationGroup {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_New {
  fn New(self) -> QSequentialAnimationGroup;
}

  // proto:  void QSequentialAnimationGroup::QSequentialAnimationGroup(QObject * parent);
impl<'a> /*trait*/ QSequentialAnimationGroup_New for (&'a QObject) {
  fn New(self) -> QSequentialAnimationGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QSequentialAnimationGroupC1EP7QObject()};
    let ctysz: c_int = unsafe{QSequentialAnimationGroup_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN25QSequentialAnimationGroupC1EP7QObject(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN25QSequentialAnimationGroupC1EP7QObject(arg0)};
    let rsthis = QSequentialAnimationGroup{/**/qbase: QAnimationGroup::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPauseAnimation * QSequentialAnimationGroup::insertPause(int index, int msecs);
impl /*struct*/ QSequentialAnimationGroup {
  pub fn insertPause<RetType, T: QSequentialAnimationGroup_insertPause<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertPause(self);
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_insertPause<RetType> {
  fn insertPause(self , rsthis: & QSequentialAnimationGroup) -> RetType;
}

  // proto:  QPauseAnimation * QSequentialAnimationGroup::insertPause(int index, int msecs);
impl<'a> /*trait*/ QSequentialAnimationGroup_insertPause<QPauseAnimation> for (i32, i32) {
  fn insertPause(self , rsthis: & QSequentialAnimationGroup) -> QPauseAnimation {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QSequentialAnimationGroup11insertPauseEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN25QSequentialAnimationGroup11insertPauseEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPauseAnimation::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPauseAnimation * QSequentialAnimationGroup::addPause(int msecs);
impl /*struct*/ QSequentialAnimationGroup {
  pub fn addPause<RetType, T: QSequentialAnimationGroup_addPause<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addPause(self);
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_addPause<RetType> {
  fn addPause(self , rsthis: & QSequentialAnimationGroup) -> RetType;
}

  // proto:  QPauseAnimation * QSequentialAnimationGroup::addPause(int msecs);
impl<'a> /*trait*/ QSequentialAnimationGroup_addPause<QPauseAnimation> for (i32) {
  fn addPause(self , rsthis: & QSequentialAnimationGroup) -> QPauseAnimation {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QSequentialAnimationGroup8addPauseEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN25QSequentialAnimationGroup8addPauseEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QPauseAnimation::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QSequentialAnimationGroup::metaObject();
impl /*struct*/ QSequentialAnimationGroup {
  pub fn metaObject<RetType, T: QSequentialAnimationGroup_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSequentialAnimationGroup) -> RetType;
}

  // proto:  const QMetaObject * QSequentialAnimationGroup::metaObject();
impl<'a> /*trait*/ QSequentialAnimationGroup_metaObject<()> for () {
  fn metaObject(self , rsthis: & QSequentialAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QSequentialAnimationGroup10metaObjectEv()};
     unsafe {_ZNK25QSequentialAnimationGroup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSequentialAnimationGroup::QSequentialAnimationGroup(const QSequentialAnimationGroup & );
impl<'a> /*trait*/ QSequentialAnimationGroup_New for (&'a QSequentialAnimationGroup) {
  fn New(self) -> QSequentialAnimationGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QSequentialAnimationGroupC1ERKS_()};
    let ctysz: c_int = unsafe{QSequentialAnimationGroup_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN25QSequentialAnimationGroupC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN25QSequentialAnimationGroupC1ERKS_(arg0)};
    let rsthis = QSequentialAnimationGroup{/**/qbase: QAnimationGroup::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSequentialAnimationGroup::~QSequentialAnimationGroup();
impl /*struct*/ QSequentialAnimationGroup {
  pub fn Free<RetType, T: QSequentialAnimationGroup_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_Free<RetType> {
  fn Free(self , rsthis: & QSequentialAnimationGroup) -> RetType;
}

  // proto:  void QSequentialAnimationGroup::~QSequentialAnimationGroup();
impl<'a> /*trait*/ QSequentialAnimationGroup_Free<()> for () {
  fn Free(self , rsthis: & QSequentialAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QSequentialAnimationGroupD0Ev()};
     unsafe {_ZN25QSequentialAnimationGroupD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAbstractAnimation * QSequentialAnimationGroup::currentAnimation();
impl /*struct*/ QSequentialAnimationGroup {
  pub fn currentAnimation<RetType, T: QSequentialAnimationGroup_currentAnimation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentAnimation(self);
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_currentAnimation<RetType> {
  fn currentAnimation(self , rsthis: & QSequentialAnimationGroup) -> RetType;
}

  // proto:  QAbstractAnimation * QSequentialAnimationGroup::currentAnimation();
impl<'a> /*trait*/ QSequentialAnimationGroup_currentAnimation<()> for () {
  fn currentAnimation(self , rsthis: & QSequentialAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QSequentialAnimationGroup16currentAnimationEv()};
     unsafe {_ZNK25QSequentialAnimationGroup16currentAnimationEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QSequentialAnimationGroup::duration();
impl /*struct*/ QSequentialAnimationGroup {
  pub fn duration<RetType, T: QSequentialAnimationGroup_duration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.duration(self);
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_duration<RetType> {
  fn duration(self , rsthis: & QSequentialAnimationGroup) -> RetType;
}

  // proto:  int QSequentialAnimationGroup::duration();
impl<'a> /*trait*/ QSequentialAnimationGroup_duration<i32> for () {
  fn duration(self , rsthis: & QSequentialAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QSequentialAnimationGroup8durationEv()};
    let mut ret = unsafe {_ZNK25QSequentialAnimationGroup8durationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// <= body block end

