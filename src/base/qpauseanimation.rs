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
  // proto:  void QPauseAnimation::NewQPauseAnimation(const QPauseAnimation & );
  fn _ZN15QPauseAnimationC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPauseAnimation::setDuration(int msecs);
  fn _ZN15QPauseAnimation11setDurationEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QPauseAnimation::NewQPauseAnimation(QObject * parent);
  fn _ZN15QPauseAnimationC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPauseAnimation::NewQPauseAnimation(int msecs, QObject * parent);
  fn _ZN15QPauseAnimationC1EiP7QObject(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  int QPauseAnimation::duration();
  fn _ZNK15QPauseAnimation8durationEv(qthis: *mut c_void) -> c_int;
  // proto:  const QMetaObject * QPauseAnimation::metaObject();
  fn _ZNK15QPauseAnimation10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QPauseAnimation::FreeQPauseAnimation();
  fn _ZN15QPauseAnimationD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QPauseAnimation)=1
pub struct QPauseAnimation {
  pub qclsinst: *mut c_void,
}

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

// proto: void QPauseAnimation::NewQPauseAnimation(const QPauseAnimation & );
impl<'a> /*trait*/ QPauseAnimation_NewQPauseAnimation for (&'a  QPauseAnimation) {
  fn NewQPauseAnimation(self) -> QPauseAnimation {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimationC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QPauseAnimationC1ERKS_(qthis, arg0)};
    let rsthis = QPauseAnimation{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPauseAnimation {
  pub fn setDuration<RetType, T: QPauseAnimation_setDuration<RetType>>(&mut self, value: T) -> RetType {
    return value.setDuration(self);
    // return 1;
  }
}

pub trait QPauseAnimation_setDuration<RetType> {
  fn setDuration(self, rsthis: &mut QPauseAnimation) -> RetType;
}

// proto:  void QPauseAnimation::setDuration(int msecs);
impl<'a> /*trait*/ QPauseAnimation_setDuration<()> for (i32) {
  fn setDuration(self, rsthis: &mut QPauseAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimation11setDurationEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QPauseAnimation11setDurationEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QPauseAnimation::NewQPauseAnimation(QObject * parent);
impl<'a> /*trait*/ QPauseAnimation_NewQPauseAnimation for (&'a mut QObject) {
  fn NewQPauseAnimation(self) -> QPauseAnimation {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimationC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QPauseAnimationC1EP7QObject(qthis, arg0)};
    let rsthis = QPauseAnimation{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QPauseAnimation::NewQPauseAnimation(int msecs, QObject * parent);
impl<'a> /*trait*/ QPauseAnimation_NewQPauseAnimation for (i32, &'a mut QObject) {
  fn NewQPauseAnimation(self) -> QPauseAnimation {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimationC1EiP7QObject()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN15QPauseAnimationC1EiP7QObject(qthis, arg0, arg1)};
    let rsthis = QPauseAnimation{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPauseAnimation {
  pub fn duration<RetType, T: QPauseAnimation_duration<RetType>>(&mut self, value: T) -> RetType {
    return value.duration(self);
    // return 1;
  }
}

pub trait QPauseAnimation_duration<RetType> {
  fn duration(self, rsthis: &mut QPauseAnimation) -> RetType;
}

// proto:  int QPauseAnimation::duration();
impl<'a> /*trait*/ QPauseAnimation_duration<i32> for () {
  fn duration(self, rsthis: &mut QPauseAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QPauseAnimation8durationEv()};
    let mut ret = unsafe {_ZNK15QPauseAnimation8durationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPauseAnimation {
  pub fn metaObject<RetType, T: QPauseAnimation_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QPauseAnimation_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QPauseAnimation) -> RetType;
}

// proto:  const QMetaObject * QPauseAnimation::metaObject();
impl<'a> /*trait*/ QPauseAnimation_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QPauseAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QPauseAnimation10metaObjectEv()};
     unsafe {_ZNK15QPauseAnimation10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPauseAnimation {
  pub fn FreeQPauseAnimation<RetType, T: QPauseAnimation_FreeQPauseAnimation<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQPauseAnimation(self);
    // return 1;
  }
}

pub trait QPauseAnimation_FreeQPauseAnimation<RetType> {
  fn FreeQPauseAnimation(self, rsthis: &mut QPauseAnimation) -> RetType;
}

// proto:  void QPauseAnimation::FreeQPauseAnimation();
impl<'a> /*trait*/ QPauseAnimation_FreeQPauseAnimation<()> for () {
  fn FreeQPauseAnimation(self, rsthis: &mut QPauseAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimationD0Ev()};
     unsafe {_ZN15QPauseAnimationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

