// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpauseanimation::QPauseAnimation;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QPauseAnimation * QSequentialAnimationGroup::insertPause(int index, int msecs);
  fn _ZN25QSequentialAnimationGroup11insertPauseEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QSequentialAnimationGroup::NewQSequentialAnimationGroup(QObject * parent);
  fn _ZN25QSequentialAnimationGroupC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPauseAnimation * QSequentialAnimationGroup::addPause(int msecs);
  fn _ZN25QSequentialAnimationGroup8addPauseEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QSequentialAnimationGroup::FreeQSequentialAnimationGroup();
  fn _ZN25QSequentialAnimationGroupD0Ev(qthis: *mut c_void) ;
  // proto:  void QSequentialAnimationGroup::NewQSequentialAnimationGroup(const QSequentialAnimationGroup & );
  fn _ZN25QSequentialAnimationGroupC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QSequentialAnimationGroup::metaObject();
  fn _ZNK25QSequentialAnimationGroup10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QAbstractAnimation * QSequentialAnimationGroup::currentAnimation();
  fn _ZNK25QSequentialAnimationGroup16currentAnimationEv(qthis: *mut c_void) ;
  // proto:  int QSequentialAnimationGroup::duration();
  fn _ZNK25QSequentialAnimationGroup8durationEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QSequentialAnimationGroup)=1
pub struct QSequentialAnimationGroup {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSequentialAnimationGroup {
  pub fn insertPause<RetType, T: QSequentialAnimationGroup_insertPause<RetType>>(&mut self, value: T) -> RetType {
    return value.insertPause(self);
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_insertPause<RetType> {
  fn insertPause(self, rsthis: &mut QSequentialAnimationGroup) -> RetType;
}

// proto:  QPauseAnimation * QSequentialAnimationGroup::insertPause(int index, int msecs);
impl<'a> /*trait*/ QSequentialAnimationGroup_insertPause<QPauseAnimation> for (i32, i32) {
  fn insertPause(self, rsthis: &mut QSequentialAnimationGroup) -> QPauseAnimation {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QSequentialAnimationGroup11insertPauseEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN25QSequentialAnimationGroup11insertPauseEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPauseAnimation{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSequentialAnimationGroup {
  pub fn NewQSequentialAnimationGroup<T: QSequentialAnimationGroup_NewQSequentialAnimationGroup>(value: T) -> QSequentialAnimationGroup {
    let rsthis = value.NewQSequentialAnimationGroup();
    return rsthis;
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_NewQSequentialAnimationGroup {
  fn NewQSequentialAnimationGroup(self) -> QSequentialAnimationGroup;
}

// proto: void QSequentialAnimationGroup::NewQSequentialAnimationGroup(QObject * parent);
impl<'a> /*trait*/ QSequentialAnimationGroup_NewQSequentialAnimationGroup for (&'a mut QObject) {
  fn NewQSequentialAnimationGroup(self) -> QSequentialAnimationGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QSequentialAnimationGroupC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN25QSequentialAnimationGroupC1EP7QObject(qthis, arg0)};
    let rsthis = QSequentialAnimationGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSequentialAnimationGroup {
  pub fn addPause<RetType, T: QSequentialAnimationGroup_addPause<RetType>>(&mut self, value: T) -> RetType {
    return value.addPause(self);
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_addPause<RetType> {
  fn addPause(self, rsthis: &mut QSequentialAnimationGroup) -> RetType;
}

// proto:  QPauseAnimation * QSequentialAnimationGroup::addPause(int msecs);
impl<'a> /*trait*/ QSequentialAnimationGroup_addPause<QPauseAnimation> for (i32) {
  fn addPause(self, rsthis: &mut QSequentialAnimationGroup) -> QPauseAnimation {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QSequentialAnimationGroup8addPauseEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN25QSequentialAnimationGroup8addPauseEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QPauseAnimation{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSequentialAnimationGroup {
  pub fn FreeQSequentialAnimationGroup<RetType, T: QSequentialAnimationGroup_FreeQSequentialAnimationGroup<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQSequentialAnimationGroup(self);
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_FreeQSequentialAnimationGroup<RetType> {
  fn FreeQSequentialAnimationGroup(self, rsthis: &mut QSequentialAnimationGroup) -> RetType;
}

// proto:  void QSequentialAnimationGroup::FreeQSequentialAnimationGroup();
impl<'a> /*trait*/ QSequentialAnimationGroup_FreeQSequentialAnimationGroup<()> for () {
  fn FreeQSequentialAnimationGroup(self, rsthis: &mut QSequentialAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QSequentialAnimationGroupD0Ev()};
     unsafe {_ZN25QSequentialAnimationGroupD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QSequentialAnimationGroup::NewQSequentialAnimationGroup(const QSequentialAnimationGroup & );
impl<'a> /*trait*/ QSequentialAnimationGroup_NewQSequentialAnimationGroup for (&'a  QSequentialAnimationGroup) {
  fn NewQSequentialAnimationGroup(self) -> QSequentialAnimationGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QSequentialAnimationGroupC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN25QSequentialAnimationGroupC1ERKS_(qthis, arg0)};
    let rsthis = QSequentialAnimationGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSequentialAnimationGroup {
  pub fn metaObject<RetType, T: QSequentialAnimationGroup_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QSequentialAnimationGroup) -> RetType;
}

// proto:  const QMetaObject * QSequentialAnimationGroup::metaObject();
impl<'a> /*trait*/ QSequentialAnimationGroup_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QSequentialAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QSequentialAnimationGroup10metaObjectEv()};
     unsafe {_ZNK25QSequentialAnimationGroup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSequentialAnimationGroup {
  pub fn currentAnimation<RetType, T: QSequentialAnimationGroup_currentAnimation<RetType>>(&mut self, value: T) -> RetType {
    return value.currentAnimation(self);
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_currentAnimation<RetType> {
  fn currentAnimation(self, rsthis: &mut QSequentialAnimationGroup) -> RetType;
}

// proto:  QAbstractAnimation * QSequentialAnimationGroup::currentAnimation();
impl<'a> /*trait*/ QSequentialAnimationGroup_currentAnimation<()> for () {
  fn currentAnimation(self, rsthis: &mut QSequentialAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QSequentialAnimationGroup16currentAnimationEv()};
     unsafe {_ZNK25QSequentialAnimationGroup16currentAnimationEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSequentialAnimationGroup {
  pub fn duration<RetType, T: QSequentialAnimationGroup_duration<RetType>>(&mut self, value: T) -> RetType {
    return value.duration(self);
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_duration<RetType> {
  fn duration(self, rsthis: &mut QSequentialAnimationGroup) -> RetType;
}

// proto:  int QSequentialAnimationGroup::duration();
impl<'a> /*trait*/ QSequentialAnimationGroup_duration<i32> for () {
  fn duration(self, rsthis: &mut QSequentialAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QSequentialAnimationGroup8durationEv()};
    let mut ret = unsafe {_ZNK25QSequentialAnimationGroup8durationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

