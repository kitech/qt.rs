// auto generated, do not modify.
// created: Fri Jan  1 15:54:32 2016
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
use super::qpauseanimation::QPauseAnimation; // 773
use super::qobject::QObject; // 773
use super::qabstractanimation::QAbstractAnimation; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSequentialAnimationGroup_Class_Size() -> c_int;
  // proto:  QPauseAnimation * QSequentialAnimationGroup::insertPause(int index, int msecs);
  fn _ZN25QSequentialAnimationGroup11insertPauseEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QSequentialAnimationGroup::QSequentialAnimationGroup(QObject * parent);
  fn dector_ZN25QSequentialAnimationGroupC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN25QSequentialAnimationGroupC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPauseAnimation * QSequentialAnimationGroup::addPause(int msecs);
  fn _ZN25QSequentialAnimationGroup8addPauseEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QSequentialAnimationGroup::~QSequentialAnimationGroup();
  fn _ZN25QSequentialAnimationGroupD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSequentialAnimationGroup::QSequentialAnimationGroup(const QSequentialAnimationGroup & );
  fn dector_ZN25QSequentialAnimationGroupC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN25QSequentialAnimationGroupC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QSequentialAnimationGroup::metaObject();
  fn _ZNK25QSequentialAnimationGroup10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QAbstractAnimation * QSequentialAnimationGroup::currentAnimation();
  fn _ZNK25QSequentialAnimationGroup16currentAnimationEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QSequentialAnimationGroup::duration();
  fn _ZNK25QSequentialAnimationGroup8durationEv(qthis: u64 /* *mut c_void*/) -> c_int;
  fn QSequentialAnimationGroup_SlotProxy_connect__ZN25QSequentialAnimationGroup23currentAnimationChangedEP18QAbstractAnimation(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QSequentialAnimationGroup)=1
#[derive(Default)]
pub struct QSequentialAnimationGroup {
  qbase: QAnimationGroup,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _currentAnimationChanged: QSequentialAnimationGroup_currentAnimationChanged_signal,
}

impl /*struct*/ QSequentialAnimationGroup {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSequentialAnimationGroup {
    return QSequentialAnimationGroup{qbase: QAnimationGroup::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret1 = QPauseAnimation::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSequentialAnimationGroup::QSequentialAnimationGroup(QObject * parent);
impl /*struct*/ QSequentialAnimationGroup {
  pub fn new<T: QSequentialAnimationGroup_new>(value: T) -> QSequentialAnimationGroup {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_new {
  fn new(self) -> QSequentialAnimationGroup;
}

  // proto:  void QSequentialAnimationGroup::QSequentialAnimationGroup(QObject * parent);
impl<'a> /*trait*/ QSequentialAnimationGroup_new for (&'a QObject) {
  fn new(self) -> QSequentialAnimationGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QSequentialAnimationGroupC1EP7QObject()};
    let ctysz: c_int = unsafe{QSequentialAnimationGroup_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN25QSequentialAnimationGroupC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN25QSequentialAnimationGroupC1EP7QObject(arg0)} as u64;
    let rsthis = QSequentialAnimationGroup{qbase: QAnimationGroup::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
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
    let mut ret1 = QPauseAnimation::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSequentialAnimationGroup::~QSequentialAnimationGroup();
impl /*struct*/ QSequentialAnimationGroup {
  pub fn free<RetType, T: QSequentialAnimationGroup_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_free<RetType> {
  fn free(self , rsthis: & QSequentialAnimationGroup) -> RetType;
}

  // proto:  void QSequentialAnimationGroup::~QSequentialAnimationGroup();
impl<'a> /*trait*/ QSequentialAnimationGroup_free<()> for () {
  fn free(self , rsthis: & QSequentialAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QSequentialAnimationGroupD0Ev()};
     unsafe {_ZN25QSequentialAnimationGroupD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSequentialAnimationGroup::QSequentialAnimationGroup(const QSequentialAnimationGroup & );
impl<'a> /*trait*/ QSequentialAnimationGroup_new for (&'a QSequentialAnimationGroup) {
  fn new(self) -> QSequentialAnimationGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QSequentialAnimationGroupC1ERKS_()};
    let ctysz: c_int = unsafe{QSequentialAnimationGroup_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN25QSequentialAnimationGroupC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN25QSequentialAnimationGroupC1ERKS_(arg0)} as u64;
    let rsthis = QSequentialAnimationGroup{qbase: QAnimationGroup::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
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

#[derive(Default)] // for QSequentialAnimationGroup_currentAnimationChanged
pub struct QSequentialAnimationGroup_currentAnimationChanged_signal{poi:u64}
impl /* struct */ QSequentialAnimationGroup {
  pub fn currentAnimationChanged(&self) -> QSequentialAnimationGroup_currentAnimationChanged_signal {
     return QSequentialAnimationGroup_currentAnimationChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QSequentialAnimationGroup_currentAnimationChanged_signal {
  pub fn connect<T: QSequentialAnimationGroup_currentAnimationChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QSequentialAnimationGroup_currentAnimationChanged_signal_connect {
  fn connect(self, sigthis: QSequentialAnimationGroup_currentAnimationChanged_signal);
}

// currentAnimationChanged(class QAbstractAnimation *)
extern fn QSequentialAnimationGroup_currentAnimationChanged_signal_connect_cb_0(rsfptr:fn(QAbstractAnimation), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QAbstractAnimation::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QSequentialAnimationGroup_currentAnimationChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QAbstractAnimation)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QAbstractAnimation::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QSequentialAnimationGroup_currentAnimationChanged_signal_connect for fn(QAbstractAnimation) {
  fn connect(self, sigthis: QSequentialAnimationGroup_currentAnimationChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSequentialAnimationGroup_currentAnimationChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QSequentialAnimationGroup_SlotProxy_connect__ZN25QSequentialAnimationGroup23currentAnimationChangedEP18QAbstractAnimation(arg0, arg1, arg2)};
  }
}
impl /* trait */ QSequentialAnimationGroup_currentAnimationChanged_signal_connect for Box<Fn(QAbstractAnimation)> {
  fn connect(self, sigthis: QSequentialAnimationGroup_currentAnimationChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSequentialAnimationGroup_currentAnimationChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QSequentialAnimationGroup_SlotProxy_connect__ZN25QSequentialAnimationGroup23currentAnimationChangedEP18QAbstractAnimation(arg0, arg1, arg2)};
  }
}
// <= body block end

