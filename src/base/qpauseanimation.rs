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
  fn _ZN15QPauseAnimationC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN15QPauseAnimation11setDurationEi(arg0: c_int) -> i32;
  fn _ZN15QPauseAnimationC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN15QPauseAnimationC1EiP7QObject(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> i32;
  fn _ZNK15QPauseAnimation8durationEv() -> i32;
  fn _ZNK15QPauseAnimation10metaObjectEv() -> i32;
  fn _ZN15QPauseAnimationD0Ev() -> i32;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QPauseAnimationC1ERKS_(qthis, arg0)};
    let rsthis = QPauseAnimation{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPauseAnimation {
  pub fn setDuration<T: QPauseAnimation_setDuration>(&mut self, value: T) -> i32 {
    value.setDuration(self);
    return 1;
  }
}

pub trait QPauseAnimation_setDuration {
  fn setDuration(self, this: &mut QPauseAnimation) -> i32;
}

// proto: void QPauseAnimation::setDuration(int msecs);
impl<'a> /*trait*/ QPauseAnimation_setDuration for (i32) {
  fn setDuration(self, this: &mut QPauseAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimation11setDurationEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QPauseAnimation11setDurationEi(arg0)};
    return 1;
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
  pub fn duration<T: QPauseAnimation_duration>(&mut self, value: T) -> i32 {
    value.duration(self);
    return 1;
  }
}

pub trait QPauseAnimation_duration {
  fn duration(self, this: &mut QPauseAnimation) -> i32;
}

// proto: int QPauseAnimation::duration();
impl<'a> /*trait*/ QPauseAnimation_duration for () {
  fn duration(self, this: &mut QPauseAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QPauseAnimation8durationEv()};
    unsafe {_ZNK15QPauseAnimation8durationEv()};
    return 1;
  }
}

impl /*struct*/ QPauseAnimation {
  pub fn metaObject<T: QPauseAnimation_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QPauseAnimation_metaObject {
  fn metaObject(self, this: &mut QPauseAnimation) -> i32;
}

// proto: const QMetaObject * QPauseAnimation::metaObject();
impl<'a> /*trait*/ QPauseAnimation_metaObject for () {
  fn metaObject(self, this: &mut QPauseAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QPauseAnimation10metaObjectEv()};
    unsafe {_ZNK15QPauseAnimation10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QPauseAnimation {
  pub fn FreeQPauseAnimation<T: QPauseAnimation_FreeQPauseAnimation>(&mut self, value: T) -> i32 {
    value.FreeQPauseAnimation(self);
    return 1;
  }
}

pub trait QPauseAnimation_FreeQPauseAnimation {
  fn FreeQPauseAnimation(self, this: &mut QPauseAnimation) -> i32;
}

// proto: void QPauseAnimation::FreeQPauseAnimation();
impl<'a> /*trait*/ QPauseAnimation_FreeQPauseAnimation for () {
  fn FreeQPauseAnimation(self, this: &mut QPauseAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QPauseAnimationD0Ev()};
    unsafe {_ZN15QPauseAnimationD0Ev()};
    return 1;
  }
}

