// auto generated, do not modify.
// created: Fri Jan  1 15:54:32 2016
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
use super::qabstractanimation::QAbstractAnimation; // 773
use std::ops::Deref;
use super::qobject::QObject; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QAnimationGroup_Class_Size() -> c_int;
  // proto:  QAbstractAnimation * QAnimationGroup::animationAt(int index);
  fn _ZNK15QAnimationGroup11animationAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QAnimationGroup::~QAnimationGroup();
  fn _ZN15QAnimationGroupD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QAnimationGroup::QAnimationGroup(const QAnimationGroup & );
  fn dector_ZN15QAnimationGroupC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN15QAnimationGroupC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAnimationGroup::removeAnimation(QAbstractAnimation * animation);
  fn _ZN15QAnimationGroup15removeAnimationEP18QAbstractAnimation(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAnimationGroup::QAnimationGroup(QObject * parent);
  fn dector_ZN15QAnimationGroupC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN15QAnimationGroupC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QAnimationGroup::animationCount();
  fn _ZNK15QAnimationGroup14animationCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QAnimationGroup::addAnimation(QAbstractAnimation * animation);
  fn _ZN15QAnimationGroup12addAnimationEP18QAbstractAnimation(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAnimationGroup::clear();
  fn _ZN15QAnimationGroup5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  QAbstractAnimation * QAnimationGroup::takeAnimation(int index);
  fn _ZN15QAnimationGroup13takeAnimationEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QAnimationGroup::insertAnimation(int index, QAbstractAnimation * animation);
  fn _ZN15QAnimationGroup15insertAnimationEiP18QAbstractAnimation(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  const QMetaObject * QAnimationGroup::metaObject();
  fn _ZNK15QAnimationGroup10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QAnimationGroup::indexOfAnimation(QAbstractAnimation * animation);
  fn _ZNK15QAnimationGroup16indexOfAnimationEP18QAbstractAnimation(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QAnimationGroup)=1
#[derive(Default)]
pub struct QAnimationGroup {
  qbase: QAbstractAnimation,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QAnimationGroup {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAnimationGroup {
    return QAnimationGroup{qbase: QAbstractAnimation::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QAnimationGroup {
  type Target = QAbstractAnimation;

  fn deref(&self) -> &QAbstractAnimation {
    return & self.qbase;
  }
}
impl AsRef<QAbstractAnimation> for QAnimationGroup {
  fn as_ref(& self) -> & QAbstractAnimation {
    return & self.qbase;
  }
}
  // proto:  QAbstractAnimation * QAnimationGroup::animationAt(int index);
impl /*struct*/ QAnimationGroup {
  pub fn animationAt<RetType, T: QAnimationGroup_animationAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.animationAt(self);
    // return 1;
  }
}

pub trait QAnimationGroup_animationAt<RetType> {
  fn animationAt(self , rsthis: & QAnimationGroup) -> RetType;
}

  // proto:  QAbstractAnimation * QAnimationGroup::animationAt(int index);
impl<'a> /*trait*/ QAnimationGroup_animationAt<()> for (i32) {
  fn animationAt(self , rsthis: & QAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAnimationGroup11animationAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK15QAnimationGroup11animationAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAnimationGroup::~QAnimationGroup();
impl /*struct*/ QAnimationGroup {
  pub fn free<RetType, T: QAnimationGroup_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QAnimationGroup_free<RetType> {
  fn free(self , rsthis: & QAnimationGroup) -> RetType;
}

  // proto:  void QAnimationGroup::~QAnimationGroup();
impl<'a> /*trait*/ QAnimationGroup_free<()> for () {
  fn free(self , rsthis: & QAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroupD0Ev()};
     unsafe {_ZN15QAnimationGroupD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAnimationGroup::QAnimationGroup(const QAnimationGroup & );
impl /*struct*/ QAnimationGroup {
  pub fn new<T: QAnimationGroup_new>(value: T) -> QAnimationGroup {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QAnimationGroup_new {
  fn new(self) -> QAnimationGroup;
}

  // proto:  void QAnimationGroup::QAnimationGroup(const QAnimationGroup & );
impl<'a> /*trait*/ QAnimationGroup_new for (&'a QAnimationGroup) {
  fn new(self) -> QAnimationGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroupC1ERKS_()};
    let ctysz: c_int = unsafe{QAnimationGroup_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN15QAnimationGroupC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN15QAnimationGroupC1ERKS_(arg0)} as u64;
    let rsthis = QAnimationGroup{qbase: QAbstractAnimation::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAnimationGroup::removeAnimation(QAbstractAnimation * animation);
impl /*struct*/ QAnimationGroup {
  pub fn removeAnimation<RetType, T: QAnimationGroup_removeAnimation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeAnimation(self);
    // return 1;
  }
}

pub trait QAnimationGroup_removeAnimation<RetType> {
  fn removeAnimation(self , rsthis: & QAnimationGroup) -> RetType;
}

  // proto:  void QAnimationGroup::removeAnimation(QAbstractAnimation * animation);
impl<'a> /*trait*/ QAnimationGroup_removeAnimation<()> for (&'a QAbstractAnimation) {
  fn removeAnimation(self , rsthis: & QAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroup15removeAnimationEP18QAbstractAnimation()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QAnimationGroup15removeAnimationEP18QAbstractAnimation(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAnimationGroup::QAnimationGroup(QObject * parent);
impl<'a> /*trait*/ QAnimationGroup_new for (&'a QObject) {
  fn new(self) -> QAnimationGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroupC1EP7QObject()};
    let ctysz: c_int = unsafe{QAnimationGroup_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN15QAnimationGroupC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN15QAnimationGroupC1EP7QObject(arg0)} as u64;
    let rsthis = QAnimationGroup{qbase: QAbstractAnimation::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QAnimationGroup::animationCount();
impl /*struct*/ QAnimationGroup {
  pub fn animationCount<RetType, T: QAnimationGroup_animationCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.animationCount(self);
    // return 1;
  }
}

pub trait QAnimationGroup_animationCount<RetType> {
  fn animationCount(self , rsthis: & QAnimationGroup) -> RetType;
}

  // proto:  int QAnimationGroup::animationCount();
impl<'a> /*trait*/ QAnimationGroup_animationCount<i32> for () {
  fn animationCount(self , rsthis: & QAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAnimationGroup14animationCountEv()};
    let mut ret = unsafe {_ZNK15QAnimationGroup14animationCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAnimationGroup::addAnimation(QAbstractAnimation * animation);
impl /*struct*/ QAnimationGroup {
  pub fn addAnimation<RetType, T: QAnimationGroup_addAnimation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addAnimation(self);
    // return 1;
  }
}

pub trait QAnimationGroup_addAnimation<RetType> {
  fn addAnimation(self , rsthis: & QAnimationGroup) -> RetType;
}

  // proto:  void QAnimationGroup::addAnimation(QAbstractAnimation * animation);
impl<'a> /*trait*/ QAnimationGroup_addAnimation<()> for (&'a QAbstractAnimation) {
  fn addAnimation(self , rsthis: & QAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroup12addAnimationEP18QAbstractAnimation()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QAnimationGroup12addAnimationEP18QAbstractAnimation(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAnimationGroup::clear();
impl /*struct*/ QAnimationGroup {
  pub fn clear<RetType, T: QAnimationGroup_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QAnimationGroup_clear<RetType> {
  fn clear(self , rsthis: & QAnimationGroup) -> RetType;
}

  // proto:  void QAnimationGroup::clear();
impl<'a> /*trait*/ QAnimationGroup_clear<()> for () {
  fn clear(self , rsthis: & QAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroup5clearEv()};
     unsafe {_ZN15QAnimationGroup5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAbstractAnimation * QAnimationGroup::takeAnimation(int index);
impl /*struct*/ QAnimationGroup {
  pub fn takeAnimation<RetType, T: QAnimationGroup_takeAnimation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeAnimation(self);
    // return 1;
  }
}

pub trait QAnimationGroup_takeAnimation<RetType> {
  fn takeAnimation(self , rsthis: & QAnimationGroup) -> RetType;
}

  // proto:  QAbstractAnimation * QAnimationGroup::takeAnimation(int index);
impl<'a> /*trait*/ QAnimationGroup_takeAnimation<()> for (i32) {
  fn takeAnimation(self , rsthis: & QAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroup13takeAnimationEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAnimationGroup13takeAnimationEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAnimationGroup::insertAnimation(int index, QAbstractAnimation * animation);
impl /*struct*/ QAnimationGroup {
  pub fn insertAnimation<RetType, T: QAnimationGroup_insertAnimation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertAnimation(self);
    // return 1;
  }
}

pub trait QAnimationGroup_insertAnimation<RetType> {
  fn insertAnimation(self , rsthis: & QAnimationGroup) -> RetType;
}

  // proto:  void QAnimationGroup::insertAnimation(int index, QAbstractAnimation * animation);
impl<'a> /*trait*/ QAnimationGroup_insertAnimation<()> for (i32, &'a QAbstractAnimation) {
  fn insertAnimation(self , rsthis: & QAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroup15insertAnimationEiP18QAbstractAnimation()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QAnimationGroup15insertAnimationEiP18QAbstractAnimation(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QAnimationGroup::metaObject();
impl /*struct*/ QAnimationGroup {
  pub fn metaObject<RetType, T: QAnimationGroup_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAnimationGroup_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAnimationGroup) -> RetType;
}

  // proto:  const QMetaObject * QAnimationGroup::metaObject();
impl<'a> /*trait*/ QAnimationGroup_metaObject<()> for () {
  fn metaObject(self , rsthis: & QAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAnimationGroup10metaObjectEv()};
     unsafe {_ZNK15QAnimationGroup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QAnimationGroup::indexOfAnimation(QAbstractAnimation * animation);
impl /*struct*/ QAnimationGroup {
  pub fn indexOfAnimation<RetType, T: QAnimationGroup_indexOfAnimation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexOfAnimation(self);
    // return 1;
  }
}

pub trait QAnimationGroup_indexOfAnimation<RetType> {
  fn indexOfAnimation(self , rsthis: & QAnimationGroup) -> RetType;
}

  // proto:  int QAnimationGroup::indexOfAnimation(QAbstractAnimation * animation);
impl<'a> /*trait*/ QAnimationGroup_indexOfAnimation<i32> for (&'a QAbstractAnimation) {
  fn indexOfAnimation(self , rsthis: & QAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAnimationGroup16indexOfAnimationEP18QAbstractAnimation()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK15QAnimationGroup16indexOfAnimationEP18QAbstractAnimation(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// <= body block end

