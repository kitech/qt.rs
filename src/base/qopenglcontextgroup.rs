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
  // proto:  const QMetaObject * QOpenGLContextGroup::metaObject();
  fn _ZNK19QOpenGLContextGroup10metaObjectEv(qthis: *mut c_void) ;
  // proto: static QOpenGLContextGroup * QOpenGLContextGroup::currentContextGroup();
  fn _ZN19QOpenGLContextGroup19currentContextGroupEv() ;
  // proto:  void QOpenGLContextGroup::FreeQOpenGLContextGroup();
  fn _ZN19QOpenGLContextGroupD0Ev(qthis: *mut c_void) ;
  // proto:  void QOpenGLContextGroup::NewQOpenGLContextGroup();
  fn _ZN19QOpenGLContextGroupC1Ev(qthis: *mut c_void) ;
  // proto:  QList<QOpenGLContext *> QOpenGLContextGroup::shares();
  fn _ZNK19QOpenGLContextGroup6sharesEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QOpenGLContextGroup)=1
pub struct QOpenGLContextGroup {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLContextGroup {
  pub fn metaObject<T: QOpenGLContextGroup_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLContextGroup_metaObject {
  fn metaObject(self, rsthis: &mut QOpenGLContextGroup) ;
}

// proto:  const QMetaObject * QOpenGLContextGroup::metaObject();
impl<'a> /*trait*/ QOpenGLContextGroup_metaObject for () {
  fn metaObject(self, rsthis: &mut QOpenGLContextGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QOpenGLContextGroup10metaObjectEv()};
     unsafe {_ZNK19QOpenGLContextGroup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContextGroup {
  pub fn currentContextGroup<T: QOpenGLContextGroup_currentContextGroup>(&mut self, value: T)  {
     value.currentContextGroup(self);
    // return 1;
  }
}

pub trait QOpenGLContextGroup_currentContextGroup {
  fn currentContextGroup(self, rsthis: &mut QOpenGLContextGroup) ;
}

// proto: static QOpenGLContextGroup * QOpenGLContextGroup::currentContextGroup();
impl<'a> /*trait*/ QOpenGLContextGroup_currentContextGroup for () {
  fn currentContextGroup(self, rsthis: &mut QOpenGLContextGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLContextGroup19currentContextGroupEv()};
     unsafe {_ZN19QOpenGLContextGroup19currentContextGroupEv()};
    // return 1;
  }
}

impl /*struct*/ QOpenGLContextGroup {
  pub fn FreeQOpenGLContextGroup<T: QOpenGLContextGroup_FreeQOpenGLContextGroup>(&mut self, value: T)  {
     value.FreeQOpenGLContextGroup(self);
    // return 1;
  }
}

pub trait QOpenGLContextGroup_FreeQOpenGLContextGroup {
  fn FreeQOpenGLContextGroup(self, rsthis: &mut QOpenGLContextGroup) ;
}

// proto:  void QOpenGLContextGroup::FreeQOpenGLContextGroup();
impl<'a> /*trait*/ QOpenGLContextGroup_FreeQOpenGLContextGroup for () {
  fn FreeQOpenGLContextGroup(self, rsthis: &mut QOpenGLContextGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLContextGroupD0Ev()};
     unsafe {_ZN19QOpenGLContextGroupD0Ev(rsthis.qclsinst)};
    // return 1;
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
  pub fn shares<T: QOpenGLContextGroup_shares>(&mut self, value: T)  {
     value.shares(self);
    // return 1;
  }
}

pub trait QOpenGLContextGroup_shares {
  fn shares(self, rsthis: &mut QOpenGLContextGroup) ;
}

// proto:  QList<QOpenGLContext *> QOpenGLContextGroup::shares();
impl<'a> /*trait*/ QOpenGLContextGroup_shares for () {
  fn shares(self, rsthis: &mut QOpenGLContextGroup)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QOpenGLContextGroup6sharesEv()};
     unsafe {_ZNK19QOpenGLContextGroup6sharesEv(rsthis.qclsinst)};
    // return 1;
  }
}

