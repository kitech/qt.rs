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
  fn _ZN23QParallelAnimationGroupD0Ev() -> i32;
  fn _ZN23QParallelAnimationGroupC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK23QParallelAnimationGroup8durationEv() -> i32;
  fn _ZN23QParallelAnimationGroupC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZNK23QParallelAnimationGroup10metaObjectEv() -> i32;
}

// body block begin
// class sizeof(QParallelAnimationGroup)=1
pub struct QParallelAnimationGroup {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QParallelAnimationGroup {
  pub fn FreeQParallelAnimationGroup<T: QParallelAnimationGroup_FreeQParallelAnimationGroup>(&mut self, value: T) -> i32 {
    value.FreeQParallelAnimationGroup(self);
    return 1;
  }
}

pub trait QParallelAnimationGroup_FreeQParallelAnimationGroup {
  fn FreeQParallelAnimationGroup(self, this: &mut QParallelAnimationGroup) -> i32;
}

// proto: void QParallelAnimationGroup::FreeQParallelAnimationGroup();
impl<'a> /*trait*/ QParallelAnimationGroup_FreeQParallelAnimationGroup for () {
  fn FreeQParallelAnimationGroup(self, this: &mut QParallelAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QParallelAnimationGroupD0Ev()};
    unsafe {_ZN23QParallelAnimationGroupD0Ev()};
    return 1;
  }
}

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

// proto: void QParallelAnimationGroup::NewQParallelAnimationGroup(const QParallelAnimationGroup & );
impl<'a> /*trait*/ QParallelAnimationGroup_NewQParallelAnimationGroup for (&'a  QParallelAnimationGroup) {
  fn NewQParallelAnimationGroup(self) -> QParallelAnimationGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QParallelAnimationGroupC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN23QParallelAnimationGroupC1ERKS_(qthis, arg0)};
    let rsthis = QParallelAnimationGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QParallelAnimationGroup {
  pub fn duration<T: QParallelAnimationGroup_duration>(&mut self, value: T) -> i32 {
    value.duration(self);
    return 1;
  }
}

pub trait QParallelAnimationGroup_duration {
  fn duration(self, this: &mut QParallelAnimationGroup) -> i32;
}

// proto: int QParallelAnimationGroup::duration();
impl<'a> /*trait*/ QParallelAnimationGroup_duration for () {
  fn duration(self, this: &mut QParallelAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QParallelAnimationGroup8durationEv()};
    unsafe {_ZNK23QParallelAnimationGroup8durationEv()};
    return 1;
  }
}

// proto: void QParallelAnimationGroup::NewQParallelAnimationGroup(QObject * parent);
impl<'a> /*trait*/ QParallelAnimationGroup_NewQParallelAnimationGroup for (&'a mut QObject) {
  fn NewQParallelAnimationGroup(self) -> QParallelAnimationGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QParallelAnimationGroupC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QParallelAnimationGroupC1EP7QObject(qthis, arg0)};
    let rsthis = QParallelAnimationGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QParallelAnimationGroup {
  pub fn metaObject<T: QParallelAnimationGroup_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QParallelAnimationGroup_metaObject {
  fn metaObject(self, this: &mut QParallelAnimationGroup) -> i32;
}

// proto: const QMetaObject * QParallelAnimationGroup::metaObject();
impl<'a> /*trait*/ QParallelAnimationGroup_metaObject for () {
  fn metaObject(self, this: &mut QParallelAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QParallelAnimationGroup10metaObjectEv()};
    unsafe {_ZNK23QParallelAnimationGroup10metaObjectEv()};
    return 1;
  }
}

