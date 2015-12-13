// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN25QSequentialAnimationGroup11insertPauseEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZN25QSequentialAnimationGroupC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN25QSequentialAnimationGroup8addPauseEi(arg0: c_int) -> i32;
  fn _ZN25QSequentialAnimationGroupD0Ev() -> i32;
  fn _ZN25QSequentialAnimationGroupC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK25QSequentialAnimationGroup10metaObjectEv() -> i32;
  fn _ZNK25QSequentialAnimationGroup16currentAnimationEv() -> i32;
  fn _ZNK25QSequentialAnimationGroup8durationEv() -> i32;
}

// body block begin
// class sizeof(QSequentialAnimationGroup)=1
pub struct QSequentialAnimationGroup {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSequentialAnimationGroup {
  pub fn insertPause<T: QSequentialAnimationGroup_insertPause>(&mut self, value: T) -> i32 {
    value.insertPause(self);
    return 1;
  }
}

pub trait QSequentialAnimationGroup_insertPause {
  fn insertPause(self, this: &mut QSequentialAnimationGroup) -> i32;
}

// proto: QPauseAnimation * QSequentialAnimationGroup::insertPause(int index, int msecs);
impl<'a> /*trait*/ QSequentialAnimationGroup_insertPause for (i32, i32) {
  fn insertPause(self, this: &mut QSequentialAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QSequentialAnimationGroup11insertPauseEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN25QSequentialAnimationGroup11insertPauseEii(arg0, arg1)};
    return 1;
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
  pub fn addPause<T: QSequentialAnimationGroup_addPause>(&mut self, value: T) -> i32 {
    value.addPause(self);
    return 1;
  }
}

pub trait QSequentialAnimationGroup_addPause {
  fn addPause(self, this: &mut QSequentialAnimationGroup) -> i32;
}

// proto: QPauseAnimation * QSequentialAnimationGroup::addPause(int msecs);
impl<'a> /*trait*/ QSequentialAnimationGroup_addPause for (i32) {
  fn addPause(self, this: &mut QSequentialAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QSequentialAnimationGroup8addPauseEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN25QSequentialAnimationGroup8addPauseEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSequentialAnimationGroup {
  pub fn FreeQSequentialAnimationGroup<T: QSequentialAnimationGroup_FreeQSequentialAnimationGroup>(&mut self, value: T) -> i32 {
    value.FreeQSequentialAnimationGroup(self);
    return 1;
  }
}

pub trait QSequentialAnimationGroup_FreeQSequentialAnimationGroup {
  fn FreeQSequentialAnimationGroup(self, this: &mut QSequentialAnimationGroup) -> i32;
}

// proto: void QSequentialAnimationGroup::FreeQSequentialAnimationGroup();
impl<'a> /*trait*/ QSequentialAnimationGroup_FreeQSequentialAnimationGroup for () {
  fn FreeQSequentialAnimationGroup(self, this: &mut QSequentialAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QSequentialAnimationGroupD0Ev()};
    unsafe {_ZN25QSequentialAnimationGroupD0Ev()};
    return 1;
  }
}

// proto: void QSequentialAnimationGroup::NewQSequentialAnimationGroup(const QSequentialAnimationGroup & );
impl<'a> /*trait*/ QSequentialAnimationGroup_NewQSequentialAnimationGroup for (&'a  QSequentialAnimationGroup) {
  fn NewQSequentialAnimationGroup(self) -> QSequentialAnimationGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QSequentialAnimationGroupC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN25QSequentialAnimationGroupC1ERKS_(qthis, arg0)};
    let rsthis = QSequentialAnimationGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSequentialAnimationGroup {
  pub fn metaObject<T: QSequentialAnimationGroup_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QSequentialAnimationGroup_metaObject {
  fn metaObject(self, this: &mut QSequentialAnimationGroup) -> i32;
}

// proto: const QMetaObject * QSequentialAnimationGroup::metaObject();
impl<'a> /*trait*/ QSequentialAnimationGroup_metaObject for () {
  fn metaObject(self, this: &mut QSequentialAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QSequentialAnimationGroup10metaObjectEv()};
    unsafe {_ZNK25QSequentialAnimationGroup10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QSequentialAnimationGroup {
  pub fn currentAnimation<T: QSequentialAnimationGroup_currentAnimation>(&mut self, value: T) -> i32 {
    value.currentAnimation(self);
    return 1;
  }
}

pub trait QSequentialAnimationGroup_currentAnimation {
  fn currentAnimation(self, this: &mut QSequentialAnimationGroup) -> i32;
}

// proto: QAbstractAnimation * QSequentialAnimationGroup::currentAnimation();
impl<'a> /*trait*/ QSequentialAnimationGroup_currentAnimation for () {
  fn currentAnimation(self, this: &mut QSequentialAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QSequentialAnimationGroup16currentAnimationEv()};
    unsafe {_ZNK25QSequentialAnimationGroup16currentAnimationEv()};
    return 1;
  }
}

impl /*struct*/ QSequentialAnimationGroup {
  pub fn duration<T: QSequentialAnimationGroup_duration>(&mut self, value: T) -> i32 {
    value.duration(self);
    return 1;
  }
}

pub trait QSequentialAnimationGroup_duration {
  fn duration(self, this: &mut QSequentialAnimationGroup) -> i32;
}

// proto: int QSequentialAnimationGroup::duration();
impl<'a> /*trait*/ QSequentialAnimationGroup_duration for () {
  fn duration(self, this: &mut QSequentialAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QSequentialAnimationGroup8durationEv()};
    unsafe {_ZNK25QSequentialAnimationGroup8durationEv()};
    return 1;
  }
}

