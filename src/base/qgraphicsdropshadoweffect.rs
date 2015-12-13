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
use super::qpointf::QPointF;
use super::qcolor::QColor;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGraphicsDropShadowEffect::blurRadiusChanged(qreal blurRadius);
  fn _ZN25QGraphicsDropShadowEffect17blurRadiusChangedEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  const QMetaObject * QGraphicsDropShadowEffect::metaObject();
  fn _ZNK25QGraphicsDropShadowEffect10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QRectF QGraphicsDropShadowEffect::boundingRectFor(const QRectF & rect);
  fn _ZNK25QGraphicsDropShadowEffect15boundingRectForERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsDropShadowEffect::NewQGraphicsDropShadowEffect(QObject * parent);
  fn _ZN25QGraphicsDropShadowEffectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPointF QGraphicsDropShadowEffect::offset();
  fn _ZNK25QGraphicsDropShadowEffect6offsetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsDropShadowEffect::setYOffset(qreal dy);
  fn _ZN25QGraphicsDropShadowEffect10setYOffsetEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QGraphicsDropShadowEffect::xOffset();
  fn _ZNK25QGraphicsDropShadowEffect7xOffsetEv(qthis: *mut c_void) -> c_double;
  // proto:  double QGraphicsDropShadowEffect::blurRadius();
  fn _ZNK25QGraphicsDropShadowEffect10blurRadiusEv(qthis: *mut c_void) -> c_double;
  // proto:  QColor QGraphicsDropShadowEffect::color();
  fn _ZNK25QGraphicsDropShadowEffect5colorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsDropShadowEffect::setColor(const QColor & color);
  fn _ZN25QGraphicsDropShadowEffect8setColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsDropShadowEffect::setOffset(const QPointF & ofs);
  fn _ZN25QGraphicsDropShadowEffect9setOffsetERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsDropShadowEffect::offsetChanged(const QPointF & offset);
  fn _ZN25QGraphicsDropShadowEffect13offsetChangedERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsDropShadowEffect::setOffset(qreal dx, qreal dy);
  fn _ZN25QGraphicsDropShadowEffect9setOffsetEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  void QGraphicsDropShadowEffect::setOffset(qreal d);
  fn _ZN25QGraphicsDropShadowEffect9setOffsetEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsDropShadowEffect::NewQGraphicsDropShadowEffect(const QGraphicsDropShadowEffect & );
  fn _ZN25QGraphicsDropShadowEffectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsDropShadowEffect::colorChanged(const QColor & color);
  fn _ZN25QGraphicsDropShadowEffect12colorChangedERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QGraphicsDropShadowEffect::yOffset();
  fn _ZNK25QGraphicsDropShadowEffect7yOffsetEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsDropShadowEffect::setXOffset(qreal dx);
  fn _ZN25QGraphicsDropShadowEffect10setXOffsetEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsDropShadowEffect::setBlurRadius(qreal blurRadius);
  fn _ZN25QGraphicsDropShadowEffect13setBlurRadiusEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsDropShadowEffect::FreeQGraphicsDropShadowEffect();
  fn _ZN25QGraphicsDropShadowEffectD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QGraphicsDropShadowEffect)=1
pub struct QGraphicsDropShadowEffect {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn blurRadiusChanged<T: QGraphicsDropShadowEffect_blurRadiusChanged>(&mut self, value: T)  {
     value.blurRadiusChanged(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_blurRadiusChanged {
  fn blurRadiusChanged(self, rsthis: &mut QGraphicsDropShadowEffect) ;
}

// proto:  void QGraphicsDropShadowEffect::blurRadiusChanged(qreal blurRadius);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_blurRadiusChanged for (f64) {
  fn blurRadiusChanged(self, rsthis: &mut QGraphicsDropShadowEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect17blurRadiusChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN25QGraphicsDropShadowEffect17blurRadiusChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn metaObject<T: QGraphicsDropShadowEffect_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_metaObject {
  fn metaObject(self, rsthis: &mut QGraphicsDropShadowEffect) ;
}

// proto:  const QMetaObject * QGraphicsDropShadowEffect::metaObject();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_metaObject for () {
  fn metaObject(self, rsthis: &mut QGraphicsDropShadowEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect10metaObjectEv()};
     unsafe {_ZNK25QGraphicsDropShadowEffect10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn boundingRectFor<T: QGraphicsDropShadowEffect_boundingRectFor>(&mut self, value: T) -> QRectF {
    return value.boundingRectFor(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_boundingRectFor {
  fn boundingRectFor(self, rsthis: &mut QGraphicsDropShadowEffect) -> QRectF;
}

// proto:  QRectF QGraphicsDropShadowEffect::boundingRectFor(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_boundingRectFor for (&'a  QRectF) {
  fn boundingRectFor(self, rsthis: &mut QGraphicsDropShadowEffect) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect15boundingRectForERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK25QGraphicsDropShadowEffect15boundingRectForERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn offset<T: QGraphicsDropShadowEffect_offset>(&mut self, value: T) -> QPointF {
    return value.offset(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_offset {
  fn offset(self, rsthis: &mut QGraphicsDropShadowEffect) -> QPointF;
}

// proto:  QPointF QGraphicsDropShadowEffect::offset();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_offset for () {
  fn offset(self, rsthis: &mut QGraphicsDropShadowEffect) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect6offsetEv()};
    let mut ret = unsafe {_ZNK25QGraphicsDropShadowEffect6offsetEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setYOffset<T: QGraphicsDropShadowEffect_setYOffset>(&mut self, value: T)  {
     value.setYOffset(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setYOffset {
  fn setYOffset(self, rsthis: &mut QGraphicsDropShadowEffect) ;
}

// proto:  void QGraphicsDropShadowEffect::setYOffset(qreal dy);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setYOffset for (f64) {
  fn setYOffset(self, rsthis: &mut QGraphicsDropShadowEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect10setYOffsetEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN25QGraphicsDropShadowEffect10setYOffsetEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn xOffset<T: QGraphicsDropShadowEffect_xOffset>(&mut self, value: T) -> f64 {
    return value.xOffset(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_xOffset {
  fn xOffset(self, rsthis: &mut QGraphicsDropShadowEffect) -> f64;
}

// proto:  double QGraphicsDropShadowEffect::xOffset();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_xOffset for () {
  fn xOffset(self, rsthis: &mut QGraphicsDropShadowEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect7xOffsetEv()};
    let mut ret = unsafe {_ZNK25QGraphicsDropShadowEffect7xOffsetEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn blurRadius<T: QGraphicsDropShadowEffect_blurRadius>(&mut self, value: T) -> f64 {
    return value.blurRadius(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_blurRadius {
  fn blurRadius(self, rsthis: &mut QGraphicsDropShadowEffect) -> f64;
}

// proto:  double QGraphicsDropShadowEffect::blurRadius();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_blurRadius for () {
  fn blurRadius(self, rsthis: &mut QGraphicsDropShadowEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect10blurRadiusEv()};
    let mut ret = unsafe {_ZNK25QGraphicsDropShadowEffect10blurRadiusEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn color<T: QGraphicsDropShadowEffect_color>(&mut self, value: T) -> QColor {
    return value.color(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_color {
  fn color(self, rsthis: &mut QGraphicsDropShadowEffect) -> QColor;
}

// proto:  QColor QGraphicsDropShadowEffect::color();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_color for () {
  fn color(self, rsthis: &mut QGraphicsDropShadowEffect) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect5colorEv()};
    let mut ret = unsafe {_ZNK25QGraphicsDropShadowEffect5colorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setColor<T: QGraphicsDropShadowEffect_setColor>(&mut self, value: T)  {
     value.setColor(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setColor {
  fn setColor(self, rsthis: &mut QGraphicsDropShadowEffect) ;
}

// proto:  void QGraphicsDropShadowEffect::setColor(const QColor & color);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setColor for (&'a  QColor) {
  fn setColor(self, rsthis: &mut QGraphicsDropShadowEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect8setColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN25QGraphicsDropShadowEffect8setColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setOffset<T: QGraphicsDropShadowEffect_setOffset>(&mut self, value: T)  {
     value.setOffset(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setOffset {
  fn setOffset(self, rsthis: &mut QGraphicsDropShadowEffect) ;
}

// proto:  void QGraphicsDropShadowEffect::setOffset(const QPointF & ofs);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setOffset for (&'a  QPointF) {
  fn setOffset(self, rsthis: &mut QGraphicsDropShadowEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect9setOffsetERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN25QGraphicsDropShadowEffect9setOffsetERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn offsetChanged<T: QGraphicsDropShadowEffect_offsetChanged>(&mut self, value: T)  {
     value.offsetChanged(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_offsetChanged {
  fn offsetChanged(self, rsthis: &mut QGraphicsDropShadowEffect) ;
}

// proto:  void QGraphicsDropShadowEffect::offsetChanged(const QPointF & offset);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_offsetChanged for (&'a  QPointF) {
  fn offsetChanged(self, rsthis: &mut QGraphicsDropShadowEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect13offsetChangedERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN25QGraphicsDropShadowEffect13offsetChangedERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QGraphicsDropShadowEffect::setOffset(qreal dx, qreal dy);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setOffset for (f64, f64) {
  fn setOffset(self, rsthis: &mut QGraphicsDropShadowEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect9setOffsetEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN25QGraphicsDropShadowEffect9setOffsetEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QGraphicsDropShadowEffect::setOffset(qreal d);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setOffset for (f64) {
  fn setOffset(self, rsthis: &mut QGraphicsDropShadowEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect9setOffsetEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN25QGraphicsDropShadowEffect9setOffsetEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QGraphicsDropShadowEffect::NewQGraphicsDropShadowEffect(const QGraphicsDropShadowEffect & );
impl<'a> /*trait*/ QGraphicsDropShadowEffect_NewQGraphicsDropShadowEffect for (&'a  QGraphicsDropShadowEffect) {
  fn NewQGraphicsDropShadowEffect(self) -> QGraphicsDropShadowEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffectC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN25QGraphicsDropShadowEffectC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsDropShadowEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn colorChanged<T: QGraphicsDropShadowEffect_colorChanged>(&mut self, value: T)  {
     value.colorChanged(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_colorChanged {
  fn colorChanged(self, rsthis: &mut QGraphicsDropShadowEffect) ;
}

// proto:  void QGraphicsDropShadowEffect::colorChanged(const QColor & color);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_colorChanged for (&'a  QColor) {
  fn colorChanged(self, rsthis: &mut QGraphicsDropShadowEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect12colorChangedERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN25QGraphicsDropShadowEffect12colorChangedERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn yOffset<T: QGraphicsDropShadowEffect_yOffset>(&mut self, value: T) -> f64 {
    return value.yOffset(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_yOffset {
  fn yOffset(self, rsthis: &mut QGraphicsDropShadowEffect) -> f64;
}

// proto:  double QGraphicsDropShadowEffect::yOffset();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_yOffset for () {
  fn yOffset(self, rsthis: &mut QGraphicsDropShadowEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect7yOffsetEv()};
    let mut ret = unsafe {_ZNK25QGraphicsDropShadowEffect7yOffsetEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setXOffset<T: QGraphicsDropShadowEffect_setXOffset>(&mut self, value: T)  {
     value.setXOffset(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setXOffset {
  fn setXOffset(self, rsthis: &mut QGraphicsDropShadowEffect) ;
}

// proto:  void QGraphicsDropShadowEffect::setXOffset(qreal dx);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setXOffset for (f64) {
  fn setXOffset(self, rsthis: &mut QGraphicsDropShadowEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect10setXOffsetEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN25QGraphicsDropShadowEffect10setXOffsetEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setBlurRadius<T: QGraphicsDropShadowEffect_setBlurRadius>(&mut self, value: T)  {
     value.setBlurRadius(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setBlurRadius {
  fn setBlurRadius(self, rsthis: &mut QGraphicsDropShadowEffect) ;
}

// proto:  void QGraphicsDropShadowEffect::setBlurRadius(qreal blurRadius);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setBlurRadius for (f64) {
  fn setBlurRadius(self, rsthis: &mut QGraphicsDropShadowEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect13setBlurRadiusEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN25QGraphicsDropShadowEffect13setBlurRadiusEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn FreeQGraphicsDropShadowEffect<T: QGraphicsDropShadowEffect_FreeQGraphicsDropShadowEffect>(&mut self, value: T)  {
     value.FreeQGraphicsDropShadowEffect(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_FreeQGraphicsDropShadowEffect {
  fn FreeQGraphicsDropShadowEffect(self, rsthis: &mut QGraphicsDropShadowEffect) ;
}

// proto:  void QGraphicsDropShadowEffect::FreeQGraphicsDropShadowEffect();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_FreeQGraphicsDropShadowEffect for () {
  fn FreeQGraphicsDropShadowEffect(self, rsthis: &mut QGraphicsDropShadowEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffectD0Ev()};
     unsafe {_ZN25QGraphicsDropShadowEffectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

