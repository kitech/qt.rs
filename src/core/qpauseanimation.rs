// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtCore/qpauseanimation.h
// dst-file: /src/core/qpauseanimation.rs
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
use super::qabstractanimation::QAbstractAnimation; // 773
use std::ops::Deref;
use super::qobject::QObject; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QPauseAnimation::QPauseAnimation(const QPauseAnimation & );
  fn _ZN15QPauseAnimationC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPauseAnimation::setDuration(int msecs);
  fn _ZN15QPauseAnimation11setDurationEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QPauseAnimation::QPauseAnimation(QObject * parent);
  fn _ZN15QPauseAnimationC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPauseAnimation::QPauseAnimation(int msecs, QObject * parent);
  fn _ZN15QPauseAnimationC1EiP7QObject(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  int QPauseAnimation::duration();
  fn _ZNK15QPauseAnimation8durationEv(qthis: *mut c_void) -> c_int;
  // proto:  const QMetaObject * QPauseAnimation::metaObject();
  fn _ZNK15QPauseAnimation10metaObjectEv(qthis: *mut c_void);
  // proto:  void QPauseAnimation::~QPauseAnimation();
  fn _ZN15QPauseAnimationD0Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QPauseAnimation)=1
pub struct QPauseAnimation {
  qbase: QAbstractAnimation,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPauseAnimation {
  pub fn inheritFrom(qthis: *mut c_void) -> QPauseAnimation {
    return QPauseAnimation{qbase: QAbstractAnimation::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QPauseAnimation {
  type Target = QAbstractAnimation;

  fn deref(&self) -> &QAbstractAnimation {
    return &self.qbase;
  }
}
impl AsRef<QAbstractAnimation> for QPauseAnimation {
  fn as_ref(&self) -> &QAbstractAnimation {
    return &self.qbase;
  }
}
  // proto:  void QPauseAnimation::QPauseAnimation(const QPauseAnimation & );
impl /*struct*/ QPauseAnimation {
  pub fn NewQPauseAnimation<T: QPauseAnimation_NewQPauseAnimation>(value: T) -> QPauseAnimation {
    let rsthis = value.NewQPauseAnimation();
    return rsthis;
    // return 1;
  }
}

pub trait QPauseAnimation_NewQPauseAnimation {
  fn NewQPauseAnimation(self) -> QPauseAnimation;
}

  // proto:  void QPauseAnimation::QPauseAnimation(const QPauseAnimation & );
impl<'a> /*trait*/ QPauseAnimation_NewQPauseAnimation for (QPauseAnimation) {
  fn NewQPauseAnimation(self) -> QPauseAnimation {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimationC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QPauseAnimationC1ERKS_(qthis, arg0)};
    let rsthis = QPauseAnimation{/**/qbase: QAbstractAnimation::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPauseAnimation::setDuration(int msecs);
impl /*struct*/ QPauseAnimation {
  pub fn setDuration<RetType, T: QPauseAnimation_setDuration<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDuration(self);
    // return 1;
  }
}

pub trait QPauseAnimation_setDuration<RetType> {
  fn setDuration(self , rsthis: &mut QPauseAnimation) -> RetType;
}

  // proto:  void QPauseAnimation::setDuration(int msecs);
impl<'a> /*trait*/ QPauseAnimation_setDuration<()> for (i32) {
  fn setDuration(self , rsthis: &mut QPauseAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimation11setDurationEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QPauseAnimation11setDurationEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPauseAnimation::QPauseAnimation(QObject * parent);
impl<'a> /*trait*/ QPauseAnimation_NewQPauseAnimation for (QObject) {
  fn NewQPauseAnimation(self) -> QPauseAnimation {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimationC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QPauseAnimationC1EP7QObject(qthis, arg0)};
    let rsthis = QPauseAnimation{/**/qbase: QAbstractAnimation::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPauseAnimation::QPauseAnimation(int msecs, QObject * parent);
impl<'a> /*trait*/ QPauseAnimation_NewQPauseAnimation for (i32, QObject) {
  fn NewQPauseAnimation(self) -> QPauseAnimation {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimationC1EiP7QObject()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN15QPauseAnimationC1EiP7QObject(qthis, arg0, arg1)};
    let rsthis = QPauseAnimation{/**/qbase: QAbstractAnimation::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QPauseAnimation::duration();
impl /*struct*/ QPauseAnimation {
  pub fn duration<RetType, T: QPauseAnimation_duration<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.duration(self);
    // return 1;
  }
}

pub trait QPauseAnimation_duration<RetType> {
  fn duration(self , rsthis: &mut QPauseAnimation) -> RetType;
}

  // proto:  int QPauseAnimation::duration();
impl<'a> /*trait*/ QPauseAnimation_duration<i32> for () {
  fn duration(self , rsthis: &mut QPauseAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QPauseAnimation8durationEv()};
    let mut ret = unsafe {_ZNK15QPauseAnimation8durationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QMetaObject * QPauseAnimation::metaObject();
impl /*struct*/ QPauseAnimation {
  pub fn metaObject<RetType, T: QPauseAnimation_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPauseAnimation_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QPauseAnimation) -> RetType;
}

  // proto:  const QMetaObject * QPauseAnimation::metaObject();
impl<'a> /*trait*/ QPauseAnimation_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QPauseAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QPauseAnimation10metaObjectEv()};
     unsafe {_ZNK15QPauseAnimation10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPauseAnimation::~QPauseAnimation();
impl /*struct*/ QPauseAnimation {
  pub fn FreeQPauseAnimation<RetType, T: QPauseAnimation_FreeQPauseAnimation<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQPauseAnimation(self);
    // return 1;
  }
}

pub trait QPauseAnimation_FreeQPauseAnimation<RetType> {
  fn FreeQPauseAnimation(self , rsthis: &mut QPauseAnimation) -> RetType;
}

  // proto:  void QPauseAnimation::~QPauseAnimation();
impl<'a> /*trait*/ QPauseAnimation_FreeQPauseAnimation<()> for () {
  fn FreeQPauseAnimation(self , rsthis: &mut QPauseAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimationD0Ev()};
     unsafe {_ZN15QPauseAnimationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

