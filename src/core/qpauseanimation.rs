// auto generated, do not modify.
// created: Wed Dec 30 23:22:52 2015
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
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPauseAnimation_Class_Size() -> c_int;
  // proto:  void QPauseAnimation::QPauseAnimation(const QPauseAnimation & );
  fn dector_ZN15QPauseAnimationC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN15QPauseAnimationC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPauseAnimation::setDuration(int msecs);
  fn _ZN15QPauseAnimation11setDurationEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QPauseAnimation::QPauseAnimation(QObject * parent);
  fn dector_ZN15QPauseAnimationC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN15QPauseAnimationC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPauseAnimation::QPauseAnimation(int msecs, QObject * parent);
  fn dector_ZN15QPauseAnimationC1EiP7QObject(arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  fn _ZN15QPauseAnimationC1EiP7QObject(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  int QPauseAnimation::duration();
  fn _ZNK15QPauseAnimation8durationEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  const QMetaObject * QPauseAnimation::metaObject();
  fn _ZNK15QPauseAnimation10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QPauseAnimation::~QPauseAnimation();
  fn _ZN15QPauseAnimationD0Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QPauseAnimation)=1
#[derive(Default)]
pub struct QPauseAnimation {
  qbase: QAbstractAnimation,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QPauseAnimation {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPauseAnimation {
    return QPauseAnimation{qbase: QAbstractAnimation::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QPauseAnimation {
  type Target = QAbstractAnimation;

  fn deref(&self) -> &QAbstractAnimation {
    return & self.qbase;
  }
}
impl AsRef<QAbstractAnimation> for QPauseAnimation {
  fn as_ref(& self) -> & QAbstractAnimation {
    return & self.qbase;
  }
}
  // proto:  void QPauseAnimation::QPauseAnimation(const QPauseAnimation & );
impl /*struct*/ QPauseAnimation {
  pub fn New<T: QPauseAnimation_New>(value: T) -> QPauseAnimation {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QPauseAnimation_New {
  fn New(self) -> QPauseAnimation;
}

  // proto:  void QPauseAnimation::QPauseAnimation(const QPauseAnimation & );
impl<'a> /*trait*/ QPauseAnimation_New for (&'a QPauseAnimation) {
  fn New(self) -> QPauseAnimation {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimationC1ERKS_()};
    let ctysz: c_int = unsafe{QPauseAnimation_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN15QPauseAnimationC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN15QPauseAnimationC1ERKS_(arg0)} as u64;
    let rsthis = QPauseAnimation{qbase: QAbstractAnimation::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPauseAnimation::setDuration(int msecs);
impl /*struct*/ QPauseAnimation {
  pub fn setDuration<RetType, T: QPauseAnimation_setDuration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDuration(self);
    // return 1;
  }
}

pub trait QPauseAnimation_setDuration<RetType> {
  fn setDuration(self , rsthis: & QPauseAnimation) -> RetType;
}

  // proto:  void QPauseAnimation::setDuration(int msecs);
impl<'a> /*trait*/ QPauseAnimation_setDuration<()> for (i32) {
  fn setDuration(self , rsthis: & QPauseAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimation11setDurationEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QPauseAnimation11setDurationEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPauseAnimation::QPauseAnimation(QObject * parent);
impl<'a> /*trait*/ QPauseAnimation_New for (&'a QObject) {
  fn New(self) -> QPauseAnimation {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimationC1EP7QObject()};
    let ctysz: c_int = unsafe{QPauseAnimation_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN15QPauseAnimationC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN15QPauseAnimationC1EP7QObject(arg0)} as u64;
    let rsthis = QPauseAnimation{qbase: QAbstractAnimation::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPauseAnimation::QPauseAnimation(int msecs, QObject * parent);
impl<'a> /*trait*/ QPauseAnimation_New for (i32, &'a QObject) {
  fn New(self) -> QPauseAnimation {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimationC1EiP7QObject()};
    let ctysz: c_int = unsafe{QPauseAnimation_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN15QPauseAnimationC1EiP7QObject(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN15QPauseAnimationC1EiP7QObject(arg0, arg1)} as u64;
    let rsthis = QPauseAnimation{qbase: QAbstractAnimation::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QPauseAnimation::duration();
impl /*struct*/ QPauseAnimation {
  pub fn duration<RetType, T: QPauseAnimation_duration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.duration(self);
    // return 1;
  }
}

pub trait QPauseAnimation_duration<RetType> {
  fn duration(self , rsthis: & QPauseAnimation) -> RetType;
}

  // proto:  int QPauseAnimation::duration();
impl<'a> /*trait*/ QPauseAnimation_duration<i32> for () {
  fn duration(self , rsthis: & QPauseAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QPauseAnimation8durationEv()};
    let mut ret = unsafe {_ZNK15QPauseAnimation8durationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QMetaObject * QPauseAnimation::metaObject();
impl /*struct*/ QPauseAnimation {
  pub fn metaObject<RetType, T: QPauseAnimation_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPauseAnimation_metaObject<RetType> {
  fn metaObject(self , rsthis: & QPauseAnimation) -> RetType;
}

  // proto:  const QMetaObject * QPauseAnimation::metaObject();
impl<'a> /*trait*/ QPauseAnimation_metaObject<()> for () {
  fn metaObject(self , rsthis: & QPauseAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QPauseAnimation10metaObjectEv()};
     unsafe {_ZNK15QPauseAnimation10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPauseAnimation::~QPauseAnimation();
impl /*struct*/ QPauseAnimation {
  pub fn Free<RetType, T: QPauseAnimation_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QPauseAnimation_Free<RetType> {
  fn Free(self , rsthis: & QPauseAnimation) -> RetType;
}

  // proto:  void QPauseAnimation::~QPauseAnimation();
impl<'a> /*trait*/ QPauseAnimation_Free<()> for () {
  fn Free(self , rsthis: & QPauseAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimationD0Ev()};
     unsafe {_ZN15QPauseAnimationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

