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
  fn _ZNK15QAnimationGroup11animationAtEi(arg0: c_int) -> i32;
  fn _ZN15QAnimationGroupD0Ev() -> i32;
  fn _ZN15QAnimationGroupC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN15QAnimationGroupC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZNK15QAnimationGroup14animationCountEv() -> i32;
  fn _ZN15QAnimationGroup5clearEv() -> i32;
  fn _ZN15QAnimationGroup13takeAnimationEi(arg0: c_int) -> i32;
  fn _ZNK15QAnimationGroup10metaObjectEv() -> i32;
}

// body block begin
// class sizeof(QAnimationGroup)=1
pub struct QAnimationGroup {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAnimationGroup {
  pub fn animationAt<T: QAnimationGroup_animationAt>(&mut self, value: T) -> i32 {
    value.animationAt(self);
    return 1;
  }
}

pub trait QAnimationGroup_animationAt {
  fn animationAt(self, this: &mut QAnimationGroup) -> i32;
}

// proto: QAbstractAnimation * QAnimationGroup::animationAt(int index);
impl<'a> /*trait*/ QAnimationGroup_animationAt for (i32) {
  fn animationAt(self, this: &mut QAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAnimationGroup11animationAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK15QAnimationGroup11animationAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAnimationGroup {
  pub fn FreeQAnimationGroup<T: QAnimationGroup_FreeQAnimationGroup>(&mut self, value: T) -> i32 {
    value.FreeQAnimationGroup(self);
    return 1;
  }
}

pub trait QAnimationGroup_FreeQAnimationGroup {
  fn FreeQAnimationGroup(self, this: &mut QAnimationGroup) -> i32;
}

// proto: void QAnimationGroup::FreeQAnimationGroup();
impl<'a> /*trait*/ QAnimationGroup_FreeQAnimationGroup for () {
  fn FreeQAnimationGroup(self, this: &mut QAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroupD0Ev()};
    unsafe {_ZN15QAnimationGroupD0Ev()};
    return 1;
  }
}

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

// proto: void QAnimationGroup::NewQAnimationGroup(const QAnimationGroup & );
impl<'a> /*trait*/ QAnimationGroup_NewQAnimationGroup for (&'a  QAnimationGroup) {
  fn NewQAnimationGroup(self) -> QAnimationGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroupC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QAnimationGroupC1ERKS_(qthis, arg0)};
    let rsthis = QAnimationGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QAnimationGroup::NewQAnimationGroup(QObject * parent);
impl<'a> /*trait*/ QAnimationGroup_NewQAnimationGroup for (&'a mut QObject) {
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

impl /*struct*/ QAnimationGroup {
  pub fn animationCount<T: QAnimationGroup_animationCount>(&mut self, value: T) -> i32 {
    value.animationCount(self);
    return 1;
  }
}

pub trait QAnimationGroup_animationCount {
  fn animationCount(self, this: &mut QAnimationGroup) -> i32;
}

// proto: int QAnimationGroup::animationCount();
impl<'a> /*trait*/ QAnimationGroup_animationCount for () {
  fn animationCount(self, this: &mut QAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAnimationGroup14animationCountEv()};
    unsafe {_ZNK15QAnimationGroup14animationCountEv()};
    return 1;
  }
}

impl /*struct*/ QAnimationGroup {
  pub fn clear<T: QAnimationGroup_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QAnimationGroup_clear {
  fn clear(self, this: &mut QAnimationGroup) -> i32;
}

// proto: void QAnimationGroup::clear();
impl<'a> /*trait*/ QAnimationGroup_clear for () {
  fn clear(self, this: &mut QAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroup5clearEv()};
    unsafe {_ZN15QAnimationGroup5clearEv()};
    return 1;
  }
}

impl /*struct*/ QAnimationGroup {
  pub fn takeAnimation<T: QAnimationGroup_takeAnimation>(&mut self, value: T) -> i32 {
    value.takeAnimation(self);
    return 1;
  }
}

pub trait QAnimationGroup_takeAnimation {
  fn takeAnimation(self, this: &mut QAnimationGroup) -> i32;
}

// proto: QAbstractAnimation * QAnimationGroup::takeAnimation(int index);
impl<'a> /*trait*/ QAnimationGroup_takeAnimation for (i32) {
  fn takeAnimation(self, this: &mut QAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroup13takeAnimationEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QAnimationGroup13takeAnimationEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QAnimationGroup {
  pub fn metaObject<T: QAnimationGroup_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QAnimationGroup_metaObject {
  fn metaObject(self, this: &mut QAnimationGroup) -> i32;
}

// proto: const QMetaObject * QAnimationGroup::metaObject();
impl<'a> /*trait*/ QAnimationGroup_metaObject for () {
  fn metaObject(self, this: &mut QAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAnimationGroup10metaObjectEv()};
    unsafe {_ZNK15QAnimationGroup10metaObjectEv()};
    return 1;
  }
}

