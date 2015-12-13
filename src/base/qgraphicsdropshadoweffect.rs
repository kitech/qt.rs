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
use super::qcolor::QColor;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGraphicsDropShadowEffect::blurRadiusChanged(qreal blurRadius);
  fn _ZN25QGraphicsDropShadowEffect17blurRadiusChangedEd(arg0: c_double) -> i32;
  // proto: const QMetaObject * QGraphicsDropShadowEffect::metaObject();
  fn _ZNK25QGraphicsDropShadowEffect10metaObjectEv() -> i32;
  // proto: QRectF QGraphicsDropShadowEffect::boundingRectFor(const QRectF & rect);
  fn _ZNK25QGraphicsDropShadowEffect15boundingRectForERK6QRectF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsDropShadowEffect::NewQGraphicsDropShadowEffect(QObject * parent);
  fn _ZN25QGraphicsDropShadowEffectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QPointF QGraphicsDropShadowEffect::offset();
  fn _ZNK25QGraphicsDropShadowEffect6offsetEv() -> i32;
  // proto: void QGraphicsDropShadowEffect::setYOffset(qreal dy);
  fn _ZN25QGraphicsDropShadowEffect10setYOffsetEd(arg0: c_double) -> i32;
  // proto: double QGraphicsDropShadowEffect::xOffset();
  fn _ZNK25QGraphicsDropShadowEffect7xOffsetEv() -> i32;
  // proto: double QGraphicsDropShadowEffect::blurRadius();
  fn _ZNK25QGraphicsDropShadowEffect10blurRadiusEv() -> i32;
  // proto: QColor QGraphicsDropShadowEffect::color();
  fn _ZNK25QGraphicsDropShadowEffect5colorEv() -> i32;
  // proto: void QGraphicsDropShadowEffect::setColor(const QColor & color);
  fn _ZN25QGraphicsDropShadowEffect8setColorERK6QColor(arg0: *const c_void) -> i32;
  // proto: void QGraphicsDropShadowEffect::setOffset(const QPointF & ofs);
  fn _ZN25QGraphicsDropShadowEffect9setOffsetERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsDropShadowEffect::offsetChanged(const QPointF & offset);
  fn _ZN25QGraphicsDropShadowEffect13offsetChangedERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsDropShadowEffect::setOffset(qreal dx, qreal dy);
  fn _ZN25QGraphicsDropShadowEffect9setOffsetEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: void QGraphicsDropShadowEffect::setOffset(qreal d);
  fn _ZN25QGraphicsDropShadowEffect9setOffsetEd(arg0: c_double) -> i32;
  // proto: void QGraphicsDropShadowEffect::NewQGraphicsDropShadowEffect(const QGraphicsDropShadowEffect & );
  fn _ZN25QGraphicsDropShadowEffectC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QGraphicsDropShadowEffect::colorChanged(const QColor & color);
  fn _ZN25QGraphicsDropShadowEffect12colorChangedERK6QColor(arg0: *const c_void) -> i32;
  // proto: double QGraphicsDropShadowEffect::yOffset();
  fn _ZNK25QGraphicsDropShadowEffect7yOffsetEv() -> i32;
  // proto: void QGraphicsDropShadowEffect::setXOffset(qreal dx);
  fn _ZN25QGraphicsDropShadowEffect10setXOffsetEd(arg0: c_double) -> i32;
  // proto: void QGraphicsDropShadowEffect::setBlurRadius(qreal blurRadius);
  fn _ZN25QGraphicsDropShadowEffect13setBlurRadiusEd(arg0: c_double) -> i32;
  // proto: void QGraphicsDropShadowEffect::FreeQGraphicsDropShadowEffect();
  fn _ZN25QGraphicsDropShadowEffectD0Ev() -> i32;
}

// body block begin
// class sizeof(QGraphicsDropShadowEffect)=1
pub struct QGraphicsDropShadowEffect {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn blurRadiusChanged<T: QGraphicsDropShadowEffect_blurRadiusChanged>(&mut self, value: T) -> i32 {
    value.blurRadiusChanged(self);
    return 1;
  }
}

pub trait QGraphicsDropShadowEffect_blurRadiusChanged {
  fn blurRadiusChanged(self, this: &mut QGraphicsDropShadowEffect) -> i32;
}

// proto: void QGraphicsDropShadowEffect::blurRadiusChanged(qreal blurRadius);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_blurRadiusChanged for (f64) {
  fn blurRadiusChanged(self, this: &mut QGraphicsDropShadowEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect17blurRadiusChangedEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN25QGraphicsDropShadowEffect17blurRadiusChangedEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn metaObject<T: QGraphicsDropShadowEffect_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QGraphicsDropShadowEffect_metaObject {
  fn metaObject(self, this: &mut QGraphicsDropShadowEffect) -> i32;
}

// proto: const QMetaObject * QGraphicsDropShadowEffect::metaObject();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_metaObject for () {
  fn metaObject(self, this: &mut QGraphicsDropShadowEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect10metaObjectEv()};
    unsafe {_ZNK25QGraphicsDropShadowEffect10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn boundingRectFor<T: QGraphicsDropShadowEffect_boundingRectFor>(&mut self, value: T) -> i32 {
    value.boundingRectFor(self);
    return 1;
  }
}

pub trait QGraphicsDropShadowEffect_boundingRectFor {
  fn boundingRectFor(self, this: &mut QGraphicsDropShadowEffect) -> i32;
}

// proto: QRectF QGraphicsDropShadowEffect::boundingRectFor(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_boundingRectFor for (&'a  QRectF) {
  fn boundingRectFor(self, this: &mut QGraphicsDropShadowEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect15boundingRectForERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK25QGraphicsDropShadowEffect15boundingRectForERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn NewQGraphicsDropShadowEffect<T: QGraphicsDropShadowEffect_NewQGraphicsDropShadowEffect>(value: T) -> QGraphicsDropShadowEffect {
    let rsthis = value.NewQGraphicsDropShadowEffect();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_NewQGraphicsDropShadowEffect {
  fn NewQGraphicsDropShadowEffect(self) -> QGraphicsDropShadowEffect;
}

// proto: void QGraphicsDropShadowEffect::NewQGraphicsDropShadowEffect(QObject * parent);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_NewQGraphicsDropShadowEffect for (&'a mut QObject) {
  fn NewQGraphicsDropShadowEffect(self) -> QGraphicsDropShadowEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffectC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN25QGraphicsDropShadowEffectC1EP7QObject(qthis, arg0)};
    let rsthis = QGraphicsDropShadowEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn offset<T: QGraphicsDropShadowEffect_offset>(&mut self, value: T) -> i32 {
    value.offset(self);
    return 1;
  }
}

pub trait QGraphicsDropShadowEffect_offset {
  fn offset(self, this: &mut QGraphicsDropShadowEffect) -> i32;
}

// proto: QPointF QGraphicsDropShadowEffect::offset();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_offset for () {
  fn offset(self, this: &mut QGraphicsDropShadowEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect6offsetEv()};
    unsafe {_ZNK25QGraphicsDropShadowEffect6offsetEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setYOffset<T: QGraphicsDropShadowEffect_setYOffset>(&mut self, value: T) -> i32 {
    value.setYOffset(self);
    return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setYOffset {
  fn setYOffset(self, this: &mut QGraphicsDropShadowEffect) -> i32;
}

// proto: void QGraphicsDropShadowEffect::setYOffset(qreal dy);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setYOffset for (f64) {
  fn setYOffset(self, this: &mut QGraphicsDropShadowEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect10setYOffsetEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN25QGraphicsDropShadowEffect10setYOffsetEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn xOffset<T: QGraphicsDropShadowEffect_xOffset>(&mut self, value: T) -> i32 {
    value.xOffset(self);
    return 1;
  }
}

pub trait QGraphicsDropShadowEffect_xOffset {
  fn xOffset(self, this: &mut QGraphicsDropShadowEffect) -> i32;
}

// proto: double QGraphicsDropShadowEffect::xOffset();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_xOffset for () {
  fn xOffset(self, this: &mut QGraphicsDropShadowEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect7xOffsetEv()};
    unsafe {_ZNK25QGraphicsDropShadowEffect7xOffsetEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn blurRadius<T: QGraphicsDropShadowEffect_blurRadius>(&mut self, value: T) -> i32 {
    value.blurRadius(self);
    return 1;
  }
}

pub trait QGraphicsDropShadowEffect_blurRadius {
  fn blurRadius(self, this: &mut QGraphicsDropShadowEffect) -> i32;
}

// proto: double QGraphicsDropShadowEffect::blurRadius();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_blurRadius for () {
  fn blurRadius(self, this: &mut QGraphicsDropShadowEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect10blurRadiusEv()};
    unsafe {_ZNK25QGraphicsDropShadowEffect10blurRadiusEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn color<T: QGraphicsDropShadowEffect_color>(&mut self, value: T) -> i32 {
    value.color(self);
    return 1;
  }
}

pub trait QGraphicsDropShadowEffect_color {
  fn color(self, this: &mut QGraphicsDropShadowEffect) -> i32;
}

// proto: QColor QGraphicsDropShadowEffect::color();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_color for () {
  fn color(self, this: &mut QGraphicsDropShadowEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect5colorEv()};
    unsafe {_ZNK25QGraphicsDropShadowEffect5colorEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setColor<T: QGraphicsDropShadowEffect_setColor>(&mut self, value: T) -> i32 {
    value.setColor(self);
    return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setColor {
  fn setColor(self, this: &mut QGraphicsDropShadowEffect) -> i32;
}

// proto: void QGraphicsDropShadowEffect::setColor(const QColor & color);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setColor for (&'a  QColor) {
  fn setColor(self, this: &mut QGraphicsDropShadowEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect8setColorERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN25QGraphicsDropShadowEffect8setColorERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setOffset<T: QGraphicsDropShadowEffect_setOffset>(&mut self, value: T) -> i32 {
    value.setOffset(self);
    return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setOffset {
  fn setOffset(self, this: &mut QGraphicsDropShadowEffect) -> i32;
}

// proto: void QGraphicsDropShadowEffect::setOffset(const QPointF & ofs);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setOffset for (&'a  QPointF) {
  fn setOffset(self, this: &mut QGraphicsDropShadowEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect9setOffsetERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN25QGraphicsDropShadowEffect9setOffsetERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn offsetChanged<T: QGraphicsDropShadowEffect_offsetChanged>(&mut self, value: T) -> i32 {
    value.offsetChanged(self);
    return 1;
  }
}

pub trait QGraphicsDropShadowEffect_offsetChanged {
  fn offsetChanged(self, this: &mut QGraphicsDropShadowEffect) -> i32;
}

// proto: void QGraphicsDropShadowEffect::offsetChanged(const QPointF & offset);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_offsetChanged for (&'a  QPointF) {
  fn offsetChanged(self, this: &mut QGraphicsDropShadowEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect13offsetChangedERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN25QGraphicsDropShadowEffect13offsetChangedERK7QPointF(arg0)};
    return 1;
  }
}

// proto: void QGraphicsDropShadowEffect::setOffset(qreal dx, qreal dy);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setOffset for (f64, f64) {
  fn setOffset(self, this: &mut QGraphicsDropShadowEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect9setOffsetEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN25QGraphicsDropShadowEffect9setOffsetEdd(arg0, arg1)};
    return 1;
  }
}

// proto: void QGraphicsDropShadowEffect::setOffset(qreal d);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setOffset for (f64) {
  fn setOffset(self, this: &mut QGraphicsDropShadowEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect9setOffsetEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN25QGraphicsDropShadowEffect9setOffsetEd(arg0)};
    return 1;
  }
}

// proto: void QGraphicsDropShadowEffect::NewQGraphicsDropShadowEffect(const QGraphicsDropShadowEffect & );
impl<'a> /*trait*/ QGraphicsDropShadowEffect_NewQGraphicsDropShadowEffect for (&'a  QGraphicsDropShadowEffect) {
  fn NewQGraphicsDropShadowEffect(self) -> QGraphicsDropShadowEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffectC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN25QGraphicsDropShadowEffectC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsDropShadowEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn colorChanged<T: QGraphicsDropShadowEffect_colorChanged>(&mut self, value: T) -> i32 {
    value.colorChanged(self);
    return 1;
  }
}

pub trait QGraphicsDropShadowEffect_colorChanged {
  fn colorChanged(self, this: &mut QGraphicsDropShadowEffect) -> i32;
}

// proto: void QGraphicsDropShadowEffect::colorChanged(const QColor & color);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_colorChanged for (&'a  QColor) {
  fn colorChanged(self, this: &mut QGraphicsDropShadowEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect12colorChangedERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN25QGraphicsDropShadowEffect12colorChangedERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn yOffset<T: QGraphicsDropShadowEffect_yOffset>(&mut self, value: T) -> i32 {
    value.yOffset(self);
    return 1;
  }
}

pub trait QGraphicsDropShadowEffect_yOffset {
  fn yOffset(self, this: &mut QGraphicsDropShadowEffect) -> i32;
}

// proto: double QGraphicsDropShadowEffect::yOffset();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_yOffset for () {
  fn yOffset(self, this: &mut QGraphicsDropShadowEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect7yOffsetEv()};
    unsafe {_ZNK25QGraphicsDropShadowEffect7yOffsetEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setXOffset<T: QGraphicsDropShadowEffect_setXOffset>(&mut self, value: T) -> i32 {
    value.setXOffset(self);
    return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setXOffset {
  fn setXOffset(self, this: &mut QGraphicsDropShadowEffect) -> i32;
}

// proto: void QGraphicsDropShadowEffect::setXOffset(qreal dx);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setXOffset for (f64) {
  fn setXOffset(self, this: &mut QGraphicsDropShadowEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect10setXOffsetEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN25QGraphicsDropShadowEffect10setXOffsetEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setBlurRadius<T: QGraphicsDropShadowEffect_setBlurRadius>(&mut self, value: T) -> i32 {
    value.setBlurRadius(self);
    return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setBlurRadius {
  fn setBlurRadius(self, this: &mut QGraphicsDropShadowEffect) -> i32;
}

// proto: void QGraphicsDropShadowEffect::setBlurRadius(qreal blurRadius);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setBlurRadius for (f64) {
  fn setBlurRadius(self, this: &mut QGraphicsDropShadowEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect13setBlurRadiusEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN25QGraphicsDropShadowEffect13setBlurRadiusEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn FreeQGraphicsDropShadowEffect<T: QGraphicsDropShadowEffect_FreeQGraphicsDropShadowEffect>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsDropShadowEffect(self);
    return 1;
  }
}

pub trait QGraphicsDropShadowEffect_FreeQGraphicsDropShadowEffect {
  fn FreeQGraphicsDropShadowEffect(self, this: &mut QGraphicsDropShadowEffect) -> i32;
}

// proto: void QGraphicsDropShadowEffect::FreeQGraphicsDropShadowEffect();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_FreeQGraphicsDropShadowEffect for () {
  fn FreeQGraphicsDropShadowEffect(self, this: &mut QGraphicsDropShadowEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffectD0Ev()};
    unsafe {_ZN25QGraphicsDropShadowEffectD0Ev()};
    return 1;
  }
}

