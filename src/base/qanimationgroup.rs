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
  // proto:  QAbstractAnimation * QAnimationGroup::animationAt(int index);
  fn _ZNK15QAnimationGroup11animationAtEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QAnimationGroup::FreeQAnimationGroup();
  fn _ZN15QAnimationGroupD0Ev(qthis: *mut c_void) ;
  // proto:  void QAnimationGroup::NewQAnimationGroup(const QAnimationGroup & );
  fn _ZN15QAnimationGroupC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QAnimationGroup::NewQAnimationGroup(QObject * parent);
  fn _ZN15QAnimationGroupC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QAnimationGroup::animationCount();
  fn _ZNK15QAnimationGroup14animationCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAnimationGroup::clear();
  fn _ZN15QAnimationGroup5clearEv(qthis: *mut c_void) ;
  // proto:  QAbstractAnimation * QAnimationGroup::takeAnimation(int index);
  fn _ZN15QAnimationGroup13takeAnimationEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  const QMetaObject * QAnimationGroup::metaObject();
  fn _ZNK15QAnimationGroup10metaObjectEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QAnimationGroup)=1
pub struct QAnimationGroup {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAnimationGroup {
  pub fn animationAt<RetType, T: QAnimationGroup_animationAt<RetType>>(&mut self, value: T) -> RetType {
    return value.animationAt(self);
    // return 1;
  }
}

pub trait QAnimationGroup_animationAt<RetType> {
  fn animationAt(self, rsthis: &mut QAnimationGroup) -> RetType;
}

// proto:  QAbstractAnimation * QAnimationGroup::animationAt(int index);
impl<'a> /*trait*/ QAnimationGroup_animationAt<()> for (i32) {
  fn animationAt(self, rsthis: &mut QAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAnimationGroup11animationAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK15QAnimationGroup11animationAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAnimationGroup {
  pub fn FreeQAnimationGroup<RetType, T: QAnimationGroup_FreeQAnimationGroup<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQAnimationGroup(self);
    // return 1;
  }
}

pub trait QAnimationGroup_FreeQAnimationGroup<RetType> {
  fn FreeQAnimationGroup(self, rsthis: &mut QAnimationGroup) -> RetType;
}

// proto:  void QAnimationGroup::FreeQAnimationGroup();
impl<'a> /*trait*/ QAnimationGroup_FreeQAnimationGroup<()> for () {
  fn FreeQAnimationGroup(self, rsthis: &mut QAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroupD0Ev()};
     unsafe {_ZN15QAnimationGroupD0Ev(rsthis.qclsinst)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
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
  pub fn animationCount<RetType, T: QAnimationGroup_animationCount<RetType>>(&mut self, value: T) -> RetType {
    return value.animationCount(self);
    // return 1;
  }
}

pub trait QAnimationGroup_animationCount<RetType> {
  fn animationCount(self, rsthis: &mut QAnimationGroup) -> RetType;
}

// proto:  int QAnimationGroup::animationCount();
impl<'a> /*trait*/ QAnimationGroup_animationCount<i32> for () {
  fn animationCount(self, rsthis: &mut QAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAnimationGroup14animationCountEv()};
    let mut ret = unsafe {_ZNK15QAnimationGroup14animationCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAnimationGroup {
  pub fn clear<RetType, T: QAnimationGroup_clear<RetType>>(&mut self, value: T) -> RetType {
    return value.clear(self);
    // return 1;
  }
}

pub trait QAnimationGroup_clear<RetType> {
  fn clear(self, rsthis: &mut QAnimationGroup) -> RetType;
}

// proto:  void QAnimationGroup::clear();
impl<'a> /*trait*/ QAnimationGroup_clear<()> for () {
  fn clear(self, rsthis: &mut QAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroup5clearEv()};
     unsafe {_ZN15QAnimationGroup5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAnimationGroup {
  pub fn takeAnimation<RetType, T: QAnimationGroup_takeAnimation<RetType>>(&mut self, value: T) -> RetType {
    return value.takeAnimation(self);
    // return 1;
  }
}

pub trait QAnimationGroup_takeAnimation<RetType> {
  fn takeAnimation(self, rsthis: &mut QAnimationGroup) -> RetType;
}

// proto:  QAbstractAnimation * QAnimationGroup::takeAnimation(int index);
impl<'a> /*trait*/ QAnimationGroup_takeAnimation<()> for (i32) {
  fn takeAnimation(self, rsthis: &mut QAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAnimationGroup13takeAnimationEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAnimationGroup13takeAnimationEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAnimationGroup {
  pub fn metaObject<RetType, T: QAnimationGroup_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QAnimationGroup_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QAnimationGroup) -> RetType;
}

// proto:  const QMetaObject * QAnimationGroup::metaObject();
impl<'a> /*trait*/ QAnimationGroup_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAnimationGroup10metaObjectEv()};
     unsafe {_ZNK15QAnimationGroup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

