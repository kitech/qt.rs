// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: const QMetaObject * QOpenGLContextGroup::metaObject();
  fn _ZNK19QOpenGLContextGroup10metaObjectEv() -> i32;
  // proto: QOpenGLContextGroup * QOpenGLContextGroup::currentContextGroup();
  fn _ZN19QOpenGLContextGroup19currentContextGroupEv() -> i32;
  // proto: void QOpenGLContextGroup::FreeQOpenGLContextGroup();
  fn _ZN19QOpenGLContextGroupD0Ev() -> i32;
  // proto: void QOpenGLContextGroup::NewQOpenGLContextGroup();
  fn _ZN19QOpenGLContextGroupC1Ev(qthis: *mut c_void) -> i32;
  // proto: QList<QOpenGLContext *> QOpenGLContextGroup::shares();
  fn _ZNK19QOpenGLContextGroup6sharesEv() -> i32;
}

// body block begin
// class sizeof(QOpenGLContextGroup)=1
pub struct QOpenGLContextGroup {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLContextGroup {
  pub fn metaObject<T: QOpenGLContextGroup_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QOpenGLContextGroup_metaObject {
  fn metaObject(self, this: &mut QOpenGLContextGroup) -> i32;
}

// proto: const QMetaObject * QOpenGLContextGroup::metaObject();
impl<'a> /*trait*/ QOpenGLContextGroup_metaObject for () {
  fn metaObject(self, this: &mut QOpenGLContextGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QOpenGLContextGroup10metaObjectEv()};
    unsafe {_ZNK19QOpenGLContextGroup10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContextGroup {
  pub fn currentContextGroup<T: QOpenGLContextGroup_currentContextGroup>(&mut self, value: T) -> i32 {
    value.currentContextGroup(self);
    return 1;
  }
}

pub trait QOpenGLContextGroup_currentContextGroup {
  fn currentContextGroup(self, this: &mut QOpenGLContextGroup) -> i32;
}

// proto: QOpenGLContextGroup * QOpenGLContextGroup::currentContextGroup();
impl<'a> /*trait*/ QOpenGLContextGroup_currentContextGroup for () {
  fn currentContextGroup(self, this: &mut QOpenGLContextGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLContextGroup19currentContextGroupEv()};
    unsafe {_ZN19QOpenGLContextGroup19currentContextGroupEv()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContextGroup {
  pub fn FreeQOpenGLContextGroup<T: QOpenGLContextGroup_FreeQOpenGLContextGroup>(&mut self, value: T) -> i32 {
    value.FreeQOpenGLContextGroup(self);
    return 1;
  }
}

pub trait QOpenGLContextGroup_FreeQOpenGLContextGroup {
  fn FreeQOpenGLContextGroup(self, this: &mut QOpenGLContextGroup) -> i32;
}

// proto: void QOpenGLContextGroup::FreeQOpenGLContextGroup();
impl<'a> /*trait*/ QOpenGLContextGroup_FreeQOpenGLContextGroup for () {
  fn FreeQOpenGLContextGroup(self, this: &mut QOpenGLContextGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLContextGroupD0Ev()};
    unsafe {_ZN19QOpenGLContextGroupD0Ev()};
    return 1;
  }
}

impl /*struct*/ QOpenGLContextGroup {
  pub fn NewQOpenGLContextGroup<T: QOpenGLContextGroup_NewQOpenGLContextGroup>(value: T) -> QOpenGLContextGroup {
    let rsthis = value.NewQOpenGLContextGroup();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLContextGroup_NewQOpenGLContextGroup {
  fn NewQOpenGLContextGroup(self) -> QOpenGLContextGroup;
}

// proto: void QOpenGLContextGroup::NewQOpenGLContextGroup();
impl<'a> /*trait*/ QOpenGLContextGroup_NewQOpenGLContextGroup for () {
  fn NewQOpenGLContextGroup(self) -> QOpenGLContextGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLContextGroupC1Ev()};
    unsafe {_ZN19QOpenGLContextGroupC1Ev(qthis)};
    let rsthis = QOpenGLContextGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLContextGroup {
  pub fn shares<T: QOpenGLContextGroup_shares>(&mut self, value: T) -> i32 {
    value.shares(self);
    return 1;
  }
}

pub trait QOpenGLContextGroup_shares {
  fn shares(self, this: &mut QOpenGLContextGroup) -> i32;
}

// proto: QList<QOpenGLContext *> QOpenGLContextGroup::shares();
impl<'a> /*trait*/ QOpenGLContextGroup_shares for () {
  fn shares(self, this: &mut QOpenGLContextGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QOpenGLContextGroup6sharesEv()};
    unsafe {_ZNK19QOpenGLContextGroup6sharesEv()};
    return 1;
  }
}

