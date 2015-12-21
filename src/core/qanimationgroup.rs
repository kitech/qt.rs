// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qanimationgroup.h
// dst-file: /src/core/qanimationgroup.rs
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
use super::qobject::QObject; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QAbstractAnimation * QAnimationGroup::animationAt(int index);
  fn _ZNK15QAnimationGroup11animationAtEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QAnimationGroup::~QAnimationGroup();
  fn _ZN15QAnimationGroupD0Ev(qthis: *mut c_void);
  // proto:  void QAnimationGroup::QAnimationGroup(const QAnimationGroup & );
  fn _ZN15QAnimationGroupC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAnimationGroup::QAnimationGroup(QObject * parent);
  fn _ZN15QAnimationGroupC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QAnimationGroup::animationCount();
  fn _ZNK15QAnimationGroup14animationCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAnimationGroup::clear();
  fn _ZN15QAnimationGroup5clearEv(qthis: *mut c_void);
  // proto:  QAbstractAnimation * QAnimationGroup::takeAnimation(int index);
  fn _ZN15QAnimationGroup13takeAnimationEi(qthis: *mut c_void, arg0: c_int);
  // proto:  const QMetaObject * QAnimationGroup::metaObject();
  fn _ZNK15QAnimationGroup10metaObjectEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAnimationGroup)=1
pub struct QAnimationGroup {
  pub qclsinst: *mut c_void,
}

  // proto:  QAbstractAnimation * QAnimationGroup::animationAt(int index);
impl /*struct*/ QAnimationGroup {
  pub fn animationAt<RetType, T: QAnimationGroup_animationAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.animationAt(self);
    // return 1;
  }
}

pub trait QAnimationGroup_animationAt<RetType> {
  fn animationAt(self , rsthis: &mut QAnimationGroup) -> RetType;
}

  // proto:  QAbstractAnimation * QAnimationGroup::animationAt(int index);
impl<'a> /*trait*/ QAnimationGroup_animationAt<()> for (i32) {
  fn animationAt(self , rsthis: &mut QAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAnimationGroup11animationAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK15QAnimationGroup11animationAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAnimationGroup::~QAnimationGroup();
impl /*struct*/ QAnimationGroup {
  pub fn FreeQAnimationGroup<RetType, T: QAnimationGroup_FreeQAnimationGroup<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAnimationGroup(self);
    // return 1;
  }
}

pub trait QAnimationGroup_FreeQAnimationGroup<RetType> {
  fn FreeQAnimationGroup(self , rsthis: &mut QAnimationGroup) -> RetType;
}

  // proto:  void QAnimationGroup::~QAnimationGroup();
impl<'a> /*trait*/ QAnimationGroup_FreeQAnimationGroup<()> for () {
  fn FreeQAnimationGroup(self , rsthis: &mut QAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroupD0Ev()};
     unsafe {_ZN15QAnimationGroupD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAnimationGroup::QAnimationGroup(const QAnimationGroup & );
impl /*struct*/ QAnimationGroup {
  pub fn NewQAnimationGroup<T: QAnimationGroup_NewQAnimationGroup>(value: T) -> QAnimationGroup {
    let rsthis = value.NewQAnimationGroup();
    return rsthis;
    // return 1;
  }
}

pub trait QAnimationGroup_NewQAnimationGroup {
  fn NewQAnimationGroup(self) -> QAnimationGroup;
}

  // proto:  void QAnimationGroup::QAnimationGroup(const QAnimationGroup & );
impl<'a> /*trait*/ QAnimationGroup_NewQAnimationGroup for (QAnimationGroup) {
  fn NewQAnimationGroup(self) -> QAnimationGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroupC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QAnimationGroupC1ERKS_(qthis, arg0)};
    let rsthis = QAnimationGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAnimationGroup::QAnimationGroup(QObject * parent);
impl<'a> /*trait*/ QAnimationGroup_NewQAnimationGroup for (QObject) {
  fn NewQAnimationGroup(self) -> QAnimationGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroupC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QAnimationGroupC1EP7QObject(qthis, arg0)};
    let rsthis = QAnimationGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QAnimationGroup::animationCount();
impl /*struct*/ QAnimationGroup {
  pub fn animationCount<RetType, T: QAnimationGroup_animationCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.animationCount(self);
    // return 1;
  }
}

pub trait QAnimationGroup_animationCount<RetType> {
  fn animationCount(self , rsthis: &mut QAnimationGroup) -> RetType;
}

  // proto:  int QAnimationGroup::animationCount();
impl<'a> /*trait*/ QAnimationGroup_animationCount<i32> for () {
  fn animationCount(self , rsthis: &mut QAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAnimationGroup14animationCountEv()};
    let mut ret = unsafe {_ZNK15QAnimationGroup14animationCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAnimationGroup::clear();
impl /*struct*/ QAnimationGroup {
  pub fn clear<RetType, T: QAnimationGroup_clear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QAnimationGroup_clear<RetType> {
  fn clear(self , rsthis: &mut QAnimationGroup) -> RetType;
}

  // proto:  void QAnimationGroup::clear();
impl<'a> /*trait*/ QAnimationGroup_clear<()> for () {
  fn clear(self , rsthis: &mut QAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroup5clearEv()};
     unsafe {_ZN15QAnimationGroup5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAbstractAnimation * QAnimationGroup::takeAnimation(int index);
impl /*struct*/ QAnimationGroup {
  pub fn takeAnimation<RetType, T: QAnimationGroup_takeAnimation<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.takeAnimation(self);
    // return 1;
  }
}

pub trait QAnimationGroup_takeAnimation<RetType> {
  fn takeAnimation(self , rsthis: &mut QAnimationGroup) -> RetType;
}

  // proto:  QAbstractAnimation * QAnimationGroup::takeAnimation(int index);
impl<'a> /*trait*/ QAnimationGroup_takeAnimation<()> for (i32) {
  fn takeAnimation(self , rsthis: &mut QAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroup13takeAnimationEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAnimationGroup13takeAnimationEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QAnimationGroup::metaObject();
impl /*struct*/ QAnimationGroup {
  pub fn metaObject<RetType, T: QAnimationGroup_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAnimationGroup_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QAnimationGroup) -> RetType;
}

  // proto:  const QMetaObject * QAnimationGroup::metaObject();
impl<'a> /*trait*/ QAnimationGroup_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAnimationGroup10metaObjectEv()};
     unsafe {_ZNK15QAnimationGroup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

