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
  pub fn insertPause<T: QSequentialAnimationGroup_insertPause>(&mut self, value: T) -> QPauseAnimation {
    return value.insertPause(self);
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_insertPause {
  fn insertPause(self, rsthis: &mut QSequentialAnimationGroup) -> QPauseAnimation;
}

// proto:  QPauseAnimation * QSequentialAnimationGroup::insertPause(int index, int msecs);
impl<'a> /*trait*/ QSequentialAnimationGroup_insertPause for (i32, i32) {
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
  pub fn addPause<T: QSequentialAnimationGroup_addPause>(&mut self, value: T) -> QPauseAnimation {
    return value.addPause(self);
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_addPause {
  fn addPause(self, rsthis: &mut QSequentialAnimationGroup) -> QPauseAnimation;
}

// proto:  QPauseAnimation * QSequentialAnimationGroup::addPause(int msecs);
impl<'a> /*trait*/ QSequentialAnimationGroup_addPause for (i32) {
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
  pub fn FreeQSequentialAnimationGroup<T: QSequentialAnimationGroup_FreeQSequentialAnimationGroup>(&mut self, value: T)  {
     value.FreeQSequentialAnimationGroup(self);
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_FreeQSequentialAnimationGroup {
  fn FreeQSequentialAnimationGroup(self, rsthis: &mut QSequentialAnimationGroup) ;
}

// proto:  void QSequentialAnimationGroup::FreeQSequentialAnimationGroup();
impl<'a> /*trait*/ QSequentialAnimationGroup_FreeQSequentialAnimationGroup for () {
  fn FreeQSequentialAnimationGroup(self, rsthis: &mut QSequentialAnimationGroup)  {
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
  pub fn metaObject<T: QSequentialAnimationGroup_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_metaObject {
  fn metaObject(self, rsthis: &mut QSequentialAnimationGroup) ;
}

// proto:  const QMetaObject * QSequentialAnimationGroup::metaObject();
impl<'a> /*trait*/ QSequentialAnimationGroup_metaObject for () {
  fn metaObject(self, rsthis: &mut QSequentialAnimationGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QSequentialAnimationGroup10metaObjectEv()};
     unsafe {_ZNK25QSequentialAnimationGroup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSequentialAnimationGroup {
  pub fn currentAnimation<T: QSequentialAnimationGroup_currentAnimation>(&mut self, value: T)  {
     value.currentAnimation(self);
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_currentAnimation {
  fn currentAnimation(self, rsthis: &mut QSequentialAnimationGroup) ;
}

// proto:  QAbstractAnimation * QSequentialAnimationGroup::currentAnimation();
impl<'a> /*trait*/ QSequentialAnimationGroup_currentAnimation for () {
  fn currentAnimation(self, rsthis: &mut QSequentialAnimationGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QSequentialAnimationGroup16currentAnimationEv()};
     unsafe {_ZNK25QSequentialAnimationGroup16currentAnimationEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSequentialAnimationGroup {
  pub fn duration<T: QSequentialAnimationGroup_duration>(&mut self, value: T) -> i32 {
    return value.duration(self);
    // return 1;
  }
}

pub trait QSequentialAnimationGroup_duration {
  fn duration(self, rsthis: &mut QSequentialAnimationGroup) -> i32;
}

// proto:  int QSequentialAnimationGroup::duration();
impl<'a> /*trait*/ QSequentialAnimationGroup_duration for () {
  fn duration(self, rsthis: &mut QSequentialAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QSequentialAnimationGroup8durationEv()};
    let mut ret = unsafe {_ZNK25QSequentialAnimationGroup8durationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

