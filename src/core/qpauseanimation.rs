// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
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
use super::qabstractanimation::*; // 773
use std::ops::Deref;
use super::qobject::*; // 773
use super::qobjectdefs::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPauseAnimation_Class_Size() -> c_int;
  // proto:  void QPauseAnimation::setDuration(int msecs);
  fn C_ZN15QPauseAnimation11setDurationEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QPauseAnimation::QPauseAnimation(QObject * parent);
  fn C_ZN15QPauseAnimationC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  void QPauseAnimation::QPauseAnimation(int msecs, QObject * parent);
  fn C_ZN15QPauseAnimationC2EiP7QObject(arg0: c_int, arg1: *mut c_void) -> u64;
  // proto:  int QPauseAnimation::duration();
  fn C_ZNK15QPauseAnimation8durationEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  const QMetaObject * QPauseAnimation::metaObject();
  fn C_ZNK15QPauseAnimation10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPauseAnimation::~QPauseAnimation();
  fn C_ZN15QPauseAnimationD2Ev(qthis: u64 /* *mut c_void*/);
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
     unsafe {C_ZN15QPauseAnimation11setDurationEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPauseAnimation::QPauseAnimation(QObject * parent);
impl /*struct*/ QPauseAnimation {
  pub fn new<T: QPauseAnimation_new>(value: T) -> QPauseAnimation {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPauseAnimation_new {
  fn new(self) -> QPauseAnimation;
}

  // proto:  void QPauseAnimation::QPauseAnimation(QObject * parent);
impl<'a> /*trait*/ QPauseAnimation_new for (Option<&'a QObject>) {
  fn new(self) -> QPauseAnimation {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimationC2EP7QObject()};
    let ctysz: c_int = unsafe{QPauseAnimation_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN15QPauseAnimationC2EP7QObject(arg0)};
    let rsthis = QPauseAnimation{qbase: QAbstractAnimation::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPauseAnimation::QPauseAnimation(int msecs, QObject * parent);
impl<'a> /*trait*/ QPauseAnimation_new for (i32, Option<&'a QObject>) {
  fn new(self) -> QPauseAnimation {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimationC2EiP7QObject()};
    let ctysz: c_int = unsafe{QPauseAnimation_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN15QPauseAnimationC2EiP7QObject(arg0, arg1)};
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
    let mut ret = unsafe {C_ZNK15QPauseAnimation8durationEv(rsthis.qclsinst)};
    return ret as i32; // 1
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
impl<'a> /*trait*/ QPauseAnimation_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QPauseAnimation) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QPauseAnimation10metaObjectEv()};
    let mut ret = unsafe {C_ZNK15QPauseAnimation10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPauseAnimation::~QPauseAnimation();
impl /*struct*/ QPauseAnimation {
  pub fn free<RetType, T: QPauseAnimation_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QPauseAnimation_free<RetType> {
  fn free(self , rsthis: & QPauseAnimation) -> RetType;
}

  // proto:  void QPauseAnimation::~QPauseAnimation();
impl<'a> /*trait*/ QPauseAnimation_free<()> for () {
  fn free(self , rsthis: & QPauseAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimationD2Ev()};
     unsafe {C_ZN15QPauseAnimationD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

