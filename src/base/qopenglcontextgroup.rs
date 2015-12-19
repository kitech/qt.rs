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

// proto:  const QMetaObject * QOpenGLContextGroup::metaObject();
impl /*struct*/ QOpenGLContextGroup {
  pub fn metaObject<RetType, T: QOpenGLContextGroup_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLContextGroup_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QOpenGLContextGroup) -> RetType;
}

// proto:  const QMetaObject * QOpenGLContextGroup::metaObject();
impl<'a> /*trait*/ QOpenGLContextGroup_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QOpenGLContextGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QOpenGLContextGroup10metaObjectEv()};
     unsafe {_ZNK19QOpenGLContextGroup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: static QOpenGLContextGroup * QOpenGLContextGroup::currentContextGroup();
impl /*struct*/ QOpenGLContextGroup {
  pub fn currentContextGroup_s<RetType, T: QOpenGLContextGroup_currentContextGroup_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentContextGroup_s();
    // return 1;
  }
}

pub trait QOpenGLContextGroup_currentContextGroup_s<RetType> {
  fn currentContextGroup_s(self ) -> RetType;
}

// proto: static QOpenGLContextGroup * QOpenGLContextGroup::currentContextGroup();
impl<'a> /*trait*/ QOpenGLContextGroup_currentContextGroup_s<()> for () {
  fn currentContextGroup_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLContextGroup19currentContextGroupEv()};
     unsafe {_ZN19QOpenGLContextGroup19currentContextGroupEv()};
    // return 1;
  }
}

// proto:  void QOpenGLContextGroup::FreeQOpenGLContextGroup();
impl /*struct*/ QOpenGLContextGroup {
  pub fn FreeQOpenGLContextGroup<RetType, T: QOpenGLContextGroup_FreeQOpenGLContextGroup<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQOpenGLContextGroup(self);
    // return 1;
  }
}

pub trait QOpenGLContextGroup_FreeQOpenGLContextGroup<RetType> {
  fn FreeQOpenGLContextGroup(self , rsthis: &mut QOpenGLContextGroup) -> RetType;
}

// proto:  void QOpenGLContextGroup::FreeQOpenGLContextGroup();
impl<'a> /*trait*/ QOpenGLContextGroup_FreeQOpenGLContextGroup<()> for () {
  fn FreeQOpenGLContextGroup(self , rsthis: &mut QOpenGLContextGroup) -> () {
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

// proto:  QList<QOpenGLContext *> QOpenGLContextGroup::shares();
impl /*struct*/ QOpenGLContextGroup {
  pub fn shares<RetType, T: QOpenGLContextGroup_shares<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.shares(self);
    // return 1;
  }
}

pub trait QOpenGLContextGroup_shares<RetType> {
  fn shares(self , rsthis: &mut QOpenGLContextGroup) -> RetType;
}

// proto:  QList<QOpenGLContext *> QOpenGLContextGroup::shares();
impl<'a> /*trait*/ QOpenGLContextGroup_shares<()> for () {
  fn shares(self , rsthis: &mut QOpenGLContextGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QOpenGLContextGroup6sharesEv()};
     unsafe {_ZNK19QOpenGLContextGroup6sharesEv(rsthis.qclsinst)};
    // return 1;
  }
}

