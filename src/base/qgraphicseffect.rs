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
  // proto: QRectF QGraphicsEffect::boundingRectFor(const QRectF & sourceRect);
  fn _ZNK15QGraphicsEffect15boundingRectForERK6QRectF(arg0: *const c_void) -> i32;
  // proto: QGraphicsEffectSource * QGraphicsEffect::source();
  fn _ZNK15QGraphicsEffect6sourceEv() -> i32;
  // proto: void QGraphicsEffect::update();
  fn _ZN15QGraphicsEffect6updateEv() -> i32;
  // proto: void QGraphicsEffect::setEnabled(bool enable);
  fn _ZN15QGraphicsEffect10setEnabledEb(arg0: int8_t) -> i32;
  // proto: void QGraphicsEffect::enabledChanged(bool enabled);
  fn _ZN15QGraphicsEffect14enabledChangedEb(arg0: int8_t) -> i32;
  // proto: const QMetaObject * QGraphicsEffect::metaObject();
  fn _ZNK15QGraphicsEffect10metaObjectEv() -> i32;
  // proto: bool QGraphicsEffect::isEnabled();
  fn _ZNK15QGraphicsEffect9isEnabledEv() -> i32;
  // proto: QRectF QGraphicsEffect::boundingRect();
  fn _ZNK15QGraphicsEffect12boundingRectEv() -> i32;
  // proto: void QGraphicsEffect::FreeQGraphicsEffect();
  fn _ZN15QGraphicsEffectD0Ev() -> i32;
  // proto: void QGraphicsEffect::NewQGraphicsEffect(const QGraphicsEffect & );
  fn _ZN15QGraphicsEffectC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QGraphicsEffect::NewQGraphicsEffect(QObject * parent);
  fn _ZN15QGraphicsEffectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QGraphicsEffect)=1
pub struct QGraphicsEffect {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsEffect {
  pub fn boundingRectFor<T: QGraphicsEffect_boundingRectFor>(&mut self, value: T) -> i32 {
    value.boundingRectFor(self);
    return 1;
  }
}

pub trait QGraphicsEffect_boundingRectFor {
  fn boundingRectFor(self, this: &mut QGraphicsEffect) -> i32;
}

// proto: QRectF QGraphicsEffect::boundingRectFor(const QRectF & sourceRect);
impl<'a> /*trait*/ QGraphicsEffect_boundingRectFor for (&'a  QRectF) {
  fn boundingRectFor(self, this: &mut QGraphicsEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsEffect15boundingRectForERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK15QGraphicsEffect15boundingRectForERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsEffect {
  pub fn source<T: QGraphicsEffect_source>(&mut self, value: T) -> i32 {
    value.source(self);
    return 1;
  }
}

pub trait QGraphicsEffect_source {
  fn source(self, this: &mut QGraphicsEffect) -> i32;
}

// proto: QGraphicsEffectSource * QGraphicsEffect::source();
impl<'a> /*trait*/ QGraphicsEffect_source for () {
  fn source(self, this: &mut QGraphicsEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsEffect6sourceEv()};
    unsafe {_ZNK15QGraphicsEffect6sourceEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsEffect {
  pub fn update<T: QGraphicsEffect_update>(&mut self, value: T) -> i32 {
    value.update(self);
    return 1;
  }
}

pub trait QGraphicsEffect_update {
  fn update(self, this: &mut QGraphicsEffect) -> i32;
}

// proto: void QGraphicsEffect::update();
impl<'a> /*trait*/ QGraphicsEffect_update for () {
  fn update(self, this: &mut QGraphicsEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffect6updateEv()};
    unsafe {_ZN15QGraphicsEffect6updateEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsEffect {
  pub fn setEnabled<T: QGraphicsEffect_setEnabled>(&mut self, value: T) -> i32 {
    value.setEnabled(self);
    return 1;
  }
}

pub trait QGraphicsEffect_setEnabled {
  fn setEnabled(self, this: &mut QGraphicsEffect) -> i32;
}

// proto: void QGraphicsEffect::setEnabled(bool enable);
impl<'a> /*trait*/ QGraphicsEffect_setEnabled for (i8) {
  fn setEnabled(self, this: &mut QGraphicsEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffect10setEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QGraphicsEffect10setEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsEffect {
  pub fn enabledChanged<T: QGraphicsEffect_enabledChanged>(&mut self, value: T) -> i32 {
    value.enabledChanged(self);
    return 1;
  }
}

pub trait QGraphicsEffect_enabledChanged {
  fn enabledChanged(self, this: &mut QGraphicsEffect) -> i32;
}

// proto: void QGraphicsEffect::enabledChanged(bool enabled);
impl<'a> /*trait*/ QGraphicsEffect_enabledChanged for (i8) {
  fn enabledChanged(self, this: &mut QGraphicsEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffect14enabledChangedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QGraphicsEffect14enabledChangedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsEffect {
  pub fn metaObject<T: QGraphicsEffect_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QGraphicsEffect_metaObject {
  fn metaObject(self, this: &mut QGraphicsEffect) -> i32;
}

// proto: const QMetaObject * QGraphicsEffect::metaObject();
impl<'a> /*trait*/ QGraphicsEffect_metaObject for () {
  fn metaObject(self, this: &mut QGraphicsEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsEffect10metaObjectEv()};
    unsafe {_ZNK15QGraphicsEffect10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsEffect {
  pub fn isEnabled<T: QGraphicsEffect_isEnabled>(&mut self, value: T) -> i32 {
    value.isEnabled(self);
    return 1;
  }
}

pub trait QGraphicsEffect_isEnabled {
  fn isEnabled(self, this: &mut QGraphicsEffect) -> i32;
}

// proto: bool QGraphicsEffect::isEnabled();
impl<'a> /*trait*/ QGraphicsEffect_isEnabled for () {
  fn isEnabled(self, this: &mut QGraphicsEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsEffect9isEnabledEv()};
    unsafe {_ZNK15QGraphicsEffect9isEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsEffect {
  pub fn boundingRect<T: QGraphicsEffect_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QGraphicsEffect_boundingRect {
  fn boundingRect(self, this: &mut QGraphicsEffect) -> i32;
}

// proto: QRectF QGraphicsEffect::boundingRect();
impl<'a> /*trait*/ QGraphicsEffect_boundingRect for () {
  fn boundingRect(self, this: &mut QGraphicsEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsEffect12boundingRectEv()};
    unsafe {_ZNK15QGraphicsEffect12boundingRectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsEffect {
  pub fn FreeQGraphicsEffect<T: QGraphicsEffect_FreeQGraphicsEffect>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsEffect(self);
    return 1;
  }
}

pub trait QGraphicsEffect_FreeQGraphicsEffect {
  fn FreeQGraphicsEffect(self, this: &mut QGraphicsEffect) -> i32;
}

// proto: void QGraphicsEffect::FreeQGraphicsEffect();
impl<'a> /*trait*/ QGraphicsEffect_FreeQGraphicsEffect for () {
  fn FreeQGraphicsEffect(self, this: &mut QGraphicsEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffectD0Ev()};
    unsafe {_ZN15QGraphicsEffectD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsEffect {
  pub fn NewQGraphicsEffect<T: QGraphicsEffect_NewQGraphicsEffect>(value: T) -> QGraphicsEffect {
    let rsthis = value.NewQGraphicsEffect();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsEffect_NewQGraphicsEffect {
  fn NewQGraphicsEffect(self) -> QGraphicsEffect;
}

// proto: void QGraphicsEffect::NewQGraphicsEffect(const QGraphicsEffect & );
impl<'a> /*trait*/ QGraphicsEffect_NewQGraphicsEffect for (&'a  QGraphicsEffect) {
  fn NewQGraphicsEffect(self) -> QGraphicsEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffectC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QGraphicsEffectC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QGraphicsEffect::NewQGraphicsEffect(QObject * parent);
impl<'a> /*trait*/ QGraphicsEffect_NewQGraphicsEffect for (&'a mut QObject) {
  fn NewQGraphicsEffect(self) -> QGraphicsEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffectC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsEffectC1EP7QObject(qthis, arg0)};
    let rsthis = QGraphicsEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

