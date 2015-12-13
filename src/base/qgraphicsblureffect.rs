// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrectf::QRectF;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: double QGraphicsBlurEffect::blurRadius();
  fn _ZNK19QGraphicsBlurEffect10blurRadiusEv() -> i32;
  // proto: void QGraphicsBlurEffect::setBlurRadius(qreal blurRadius);
  fn _ZN19QGraphicsBlurEffect13setBlurRadiusEd(arg0: c_double) -> i32;
  // proto: void QGraphicsBlurEffect::FreeQGraphicsBlurEffect();
  fn _ZN19QGraphicsBlurEffectD0Ev() -> i32;
  // proto: void QGraphicsBlurEffect::NewQGraphicsBlurEffect(const QGraphicsBlurEffect & );
  fn _ZN19QGraphicsBlurEffectC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: const QMetaObject * QGraphicsBlurEffect::metaObject();
  fn _ZNK19QGraphicsBlurEffect10metaObjectEv() -> i32;
  // proto: QRectF QGraphicsBlurEffect::boundingRectFor(const QRectF & rect);
  fn _ZNK19QGraphicsBlurEffect15boundingRectForERK6QRectF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsBlurEffect::NewQGraphicsBlurEffect(QObject * parent);
  fn _ZN19QGraphicsBlurEffectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QGraphicsBlurEffect::blurRadiusChanged(qreal blurRadius);
  fn _ZN19QGraphicsBlurEffect17blurRadiusChangedEd(arg0: c_double) -> i32;
}

// body block begin
// class sizeof(QGraphicsBlurEffect)=1
pub struct QGraphicsBlurEffect {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsBlurEffect {
  pub fn blurRadius<T: QGraphicsBlurEffect_blurRadius>(&mut self, value: T) -> i32 {
    value.blurRadius(self);
    return 1;
  }
}

pub trait QGraphicsBlurEffect_blurRadius {
  fn blurRadius(self, this: &mut QGraphicsBlurEffect) -> i32;
}

// proto: double QGraphicsBlurEffect::blurRadius();
impl<'a> /*trait*/ QGraphicsBlurEffect_blurRadius for () {
  fn blurRadius(self, this: &mut QGraphicsBlurEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsBlurEffect10blurRadiusEv()};
    unsafe {_ZNK19QGraphicsBlurEffect10blurRadiusEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsBlurEffect {
  pub fn setBlurRadius<T: QGraphicsBlurEffect_setBlurRadius>(&mut self, value: T) -> i32 {
    value.setBlurRadius(self);
    return 1;
  }
}

pub trait QGraphicsBlurEffect_setBlurRadius {
  fn setBlurRadius(self, this: &mut QGraphicsBlurEffect) -> i32;
}

// proto: void QGraphicsBlurEffect::setBlurRadius(qreal blurRadius);
impl<'a> /*trait*/ QGraphicsBlurEffect_setBlurRadius for (f64) {
  fn setBlurRadius(self, this: &mut QGraphicsBlurEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsBlurEffect13setBlurRadiusEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN19QGraphicsBlurEffect13setBlurRadiusEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsBlurEffect {
  pub fn FreeQGraphicsBlurEffect<T: QGraphicsBlurEffect_FreeQGraphicsBlurEffect>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsBlurEffect(self);
    return 1;
  }
}

pub trait QGraphicsBlurEffect_FreeQGraphicsBlurEffect {
  fn FreeQGraphicsBlurEffect(self, this: &mut QGraphicsBlurEffect) -> i32;
}

// proto: void QGraphicsBlurEffect::FreeQGraphicsBlurEffect();
impl<'a> /*trait*/ QGraphicsBlurEffect_FreeQGraphicsBlurEffect for () {
  fn FreeQGraphicsBlurEffect(self, this: &mut QGraphicsBlurEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsBlurEffectD0Ev()};
    unsafe {_ZN19QGraphicsBlurEffectD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsBlurEffect {
  pub fn NewQGraphicsBlurEffect<T: QGraphicsBlurEffect_NewQGraphicsBlurEffect>(value: T) -> QGraphicsBlurEffect {
    let rsthis = value.NewQGraphicsBlurEffect();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_NewQGraphicsBlurEffect {
  fn NewQGraphicsBlurEffect(self) -> QGraphicsBlurEffect;
}

// proto: void QGraphicsBlurEffect::NewQGraphicsBlurEffect(const QGraphicsBlurEffect & );
impl<'a> /*trait*/ QGraphicsBlurEffect_NewQGraphicsBlurEffect for (&'a  QGraphicsBlurEffect) {
  fn NewQGraphicsBlurEffect(self) -> QGraphicsBlurEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsBlurEffectC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QGraphicsBlurEffectC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsBlurEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsBlurEffect {
  pub fn metaObject<T: QGraphicsBlurEffect_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QGraphicsBlurEffect_metaObject {
  fn metaObject(self, this: &mut QGraphicsBlurEffect) -> i32;
}

// proto: const QMetaObject * QGraphicsBlurEffect::metaObject();
impl<'a> /*trait*/ QGraphicsBlurEffect_metaObject for () {
  fn metaObject(self, this: &mut QGraphicsBlurEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsBlurEffect10metaObjectEv()};
    unsafe {_ZNK19QGraphicsBlurEffect10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsBlurEffect {
  pub fn boundingRectFor<T: QGraphicsBlurEffect_boundingRectFor>(&mut self, value: T) -> i32 {
    value.boundingRectFor(self);
    return 1;
  }
}

pub trait QGraphicsBlurEffect_boundingRectFor {
  fn boundingRectFor(self, this: &mut QGraphicsBlurEffect) -> i32;
}

// proto: QRectF QGraphicsBlurEffect::boundingRectFor(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsBlurEffect_boundingRectFor for (&'a  QRectF) {
  fn boundingRectFor(self, this: &mut QGraphicsBlurEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsBlurEffect15boundingRectForERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK19QGraphicsBlurEffect15boundingRectForERK6QRectF(arg0)};
    return 1;
  }
}

// proto: void QGraphicsBlurEffect::NewQGraphicsBlurEffect(QObject * parent);
impl<'a> /*trait*/ QGraphicsBlurEffect_NewQGraphicsBlurEffect for (&'a mut QObject) {
  fn NewQGraphicsBlurEffect(self) -> QGraphicsBlurEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsBlurEffectC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QGraphicsBlurEffectC1EP7QObject(qthis, arg0)};
    let rsthis = QGraphicsBlurEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsBlurEffect {
  pub fn blurRadiusChanged<T: QGraphicsBlurEffect_blurRadiusChanged>(&mut self, value: T) -> i32 {
    value.blurRadiusChanged(self);
    return 1;
  }
}

pub trait QGraphicsBlurEffect_blurRadiusChanged {
  fn blurRadiusChanged(self, this: &mut QGraphicsBlurEffect) -> i32;
}

// proto: void QGraphicsBlurEffect::blurRadiusChanged(qreal blurRadius);
impl<'a> /*trait*/ QGraphicsBlurEffect_blurRadiusChanged for (f64) {
  fn blurRadiusChanged(self, this: &mut QGraphicsBlurEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsBlurEffect17blurRadiusChangedEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN19QGraphicsBlurEffect17blurRadiusChangedEd(arg0)};
    return 1;
  }
}

