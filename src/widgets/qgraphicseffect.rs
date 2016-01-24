// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtWidgets/qgraphicseffect.h
// dst-file: /src/widgets/qgraphicseffect.rs
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
// use super::qgraphicseffect::QGraphicsEffect; // 773
use std::ops::Deref;
use super::super::gui::qcolor::QColor; // 771
use super::super::core::qobject::QObject; // 771
use super::super::core::qobjectdefs::QMetaObject; // 771
use super::super::core::qrect::QRectF; // 771
use super::super::core::qpoint::QPointF; // 771
use super::super::gui::qbrush::QBrush; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGraphicsColorizeEffect_Class_Size() -> c_int;
  // proto:  void QGraphicsColorizeEffect::setColor(const QColor & c);
  fn C_ZN23QGraphicsColorizeEffect8setColorERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsColorizeEffect::setStrength(qreal strength);
  fn C_ZN23QGraphicsColorizeEffect11setStrengthEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QGraphicsColorizeEffect::QGraphicsColorizeEffect(QObject * parent);
  fn C_ZN23QGraphicsColorizeEffectC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  qreal QGraphicsColorizeEffect::strength();
  fn C_ZNK23QGraphicsColorizeEffect8strengthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QGraphicsColorizeEffect::~QGraphicsColorizeEffect();
  fn C_ZN23QGraphicsColorizeEffectD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QColor QGraphicsColorizeEffect::color();
  fn C_ZNK23QGraphicsColorizeEffect5colorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QGraphicsColorizeEffect::metaObject();
  fn C_ZNK23QGraphicsColorizeEffect10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QGraphicsEffect_Class_Size() -> c_int;
  // proto:  QRectF QGraphicsEffect::boundingRectFor(const QRectF & sourceRect);
  fn C_ZNK15QGraphicsEffect15boundingRectForERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QGraphicsEffectSource * QGraphicsEffect::source();
  fn C_ZNK15QGraphicsEffect6sourceEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsEffect::update();
  fn C_ZN15QGraphicsEffect6updateEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsEffect::setEnabled(bool enable);
  fn C_ZN15QGraphicsEffect10setEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  const QMetaObject * QGraphicsEffect::metaObject();
  fn C_ZNK15QGraphicsEffect10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QGraphicsEffect::isEnabled();
  fn C_ZNK15QGraphicsEffect9isEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QRectF QGraphicsEffect::boundingRect();
  fn C_ZNK15QGraphicsEffect12boundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsEffect::~QGraphicsEffect();
  fn C_ZN15QGraphicsEffectD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsEffect::QGraphicsEffect(QObject * parent);
  fn C_ZN15QGraphicsEffectC2EP7QObject(arg0: *mut c_void) -> u64;
  fn QGraphicsDropShadowEffect_Class_Size() -> c_int;
  // proto:  const QMetaObject * QGraphicsDropShadowEffect::metaObject();
  fn C_ZNK25QGraphicsDropShadowEffect10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRectF QGraphicsDropShadowEffect::boundingRectFor(const QRectF & rect);
  fn C_ZNK25QGraphicsDropShadowEffect15boundingRectForERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsDropShadowEffect::QGraphicsDropShadowEffect(QObject * parent);
  fn C_ZN25QGraphicsDropShadowEffectC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  QPointF QGraphicsDropShadowEffect::offset();
  fn C_ZNK25QGraphicsDropShadowEffect6offsetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsDropShadowEffect::setYOffset(qreal dy);
  fn C_ZN25QGraphicsDropShadowEffect10setYOffsetEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  qreal QGraphicsDropShadowEffect::xOffset();
  fn C_ZNK25QGraphicsDropShadowEffect7xOffsetEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QGraphicsDropShadowEffect::blurRadius();
  fn C_ZNK25QGraphicsDropShadowEffect10blurRadiusEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QColor QGraphicsDropShadowEffect::color();
  fn C_ZNK25QGraphicsDropShadowEffect5colorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsDropShadowEffect::setColor(const QColor & color);
  fn C_ZN25QGraphicsDropShadowEffect8setColorERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsDropShadowEffect::setOffset(const QPointF & ofs);
  fn C_ZN25QGraphicsDropShadowEffect9setOffsetERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsDropShadowEffect::setOffset(qreal dx, qreal dy);
  fn C_ZN25QGraphicsDropShadowEffect9setOffsetEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto:  void QGraphicsDropShadowEffect::setOffset(qreal d);
  fn C_ZN25QGraphicsDropShadowEffect9setOffsetEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  qreal QGraphicsDropShadowEffect::yOffset();
  fn C_ZNK25QGraphicsDropShadowEffect7yOffsetEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QGraphicsDropShadowEffect::setXOffset(qreal dx);
  fn C_ZN25QGraphicsDropShadowEffect10setXOffsetEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QGraphicsDropShadowEffect::setBlurRadius(qreal blurRadius);
  fn C_ZN25QGraphicsDropShadowEffect13setBlurRadiusEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QGraphicsDropShadowEffect::~QGraphicsDropShadowEffect();
  fn C_ZN25QGraphicsDropShadowEffectD2Ev(qthis: u64 /* *mut c_void*/);
  fn QGraphicsOpacityEffect_Class_Size() -> c_int;
  // proto:  void QGraphicsOpacityEffect::QGraphicsOpacityEffect(QObject * parent);
  fn C_ZN22QGraphicsOpacityEffectC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  void QGraphicsOpacityEffect::~QGraphicsOpacityEffect();
  fn C_ZN22QGraphicsOpacityEffectD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsOpacityEffect::setOpacityMask(const QBrush & mask);
  fn C_ZN22QGraphicsOpacityEffect14setOpacityMaskERK6QBrush(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QBrush QGraphicsOpacityEffect::opacityMask();
  fn C_ZNK22QGraphicsOpacityEffect11opacityMaskEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QGraphicsOpacityEffect::metaObject();
  fn C_ZNK22QGraphicsOpacityEffect10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QGraphicsOpacityEffect::opacity();
  fn C_ZNK22QGraphicsOpacityEffect7opacityEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QGraphicsOpacityEffect::setOpacity(qreal opacity);
  fn C_ZN22QGraphicsOpacityEffect10setOpacityEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  fn QGraphicsBlurEffect_Class_Size() -> c_int;
  // proto:  qreal QGraphicsBlurEffect::blurRadius();
  fn C_ZNK19QGraphicsBlurEffect10blurRadiusEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QGraphicsBlurEffect::setBlurRadius(qreal blurRadius);
  fn C_ZN19QGraphicsBlurEffect13setBlurRadiusEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QGraphicsBlurEffect::~QGraphicsBlurEffect();
  fn C_ZN19QGraphicsBlurEffectD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QGraphicsBlurEffect::metaObject();
  fn C_ZNK19QGraphicsBlurEffect10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRectF QGraphicsBlurEffect::boundingRectFor(const QRectF & rect);
  fn C_ZNK19QGraphicsBlurEffect15boundingRectForERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsBlurEffect::QGraphicsBlurEffect(QObject * parent);
  fn C_ZN19QGraphicsBlurEffectC2EP7QObject(arg0: *mut c_void) -> u64;
  fn QGraphicsColorizeEffect_SlotProxy_connect__ZN23QGraphicsColorizeEffect15strengthChangedEd(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsColorizeEffect_SlotProxy_connect__ZN23QGraphicsColorizeEffect12colorChangedERK6QColor(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsEffect_SlotProxy_connect__ZN15QGraphicsEffect14enabledChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsDropShadowEffect_SlotProxy_connect__ZN25QGraphicsDropShadowEffect17blurRadiusChangedEd(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsDropShadowEffect_SlotProxy_connect__ZN25QGraphicsDropShadowEffect12colorChangedERK6QColor(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsDropShadowEffect_SlotProxy_connect__ZN25QGraphicsDropShadowEffect13offsetChangedERK7QPointF(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsOpacityEffect_SlotProxy_connect__ZN22QGraphicsOpacityEffect14opacityChangedEd(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsOpacityEffect_SlotProxy_connect__ZN22QGraphicsOpacityEffect18opacityMaskChangedERK6QBrush(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsBlurEffect_SlotProxy_connect__ZN19QGraphicsBlurEffect17blurRadiusChangedEd(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsColorizeEffect)=1
#[derive(Default)]
pub struct QGraphicsColorizeEffect {
  qbase: QGraphicsEffect,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _strengthChanged: QGraphicsColorizeEffect_strengthChanged_signal,
  pub _colorChanged: QGraphicsColorizeEffect_colorChanged_signal,
}

// class sizeof(QGraphicsEffect)=1
#[derive(Default)]
pub struct QGraphicsEffect {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _enabledChanged: QGraphicsEffect_enabledChanged_signal,
}

// class sizeof(QGraphicsDropShadowEffect)=1
#[derive(Default)]
pub struct QGraphicsDropShadowEffect {
  qbase: QGraphicsEffect,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _colorChanged: QGraphicsDropShadowEffect_colorChanged_signal,
  pub _offsetChanged: QGraphicsDropShadowEffect_offsetChanged_signal,
  pub _blurRadiusChanged: QGraphicsDropShadowEffect_blurRadiusChanged_signal,
}

// class sizeof(QGraphicsOpacityEffect)=1
#[derive(Default)]
pub struct QGraphicsOpacityEffect {
  qbase: QGraphicsEffect,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _opacityMaskChanged: QGraphicsOpacityEffect_opacityMaskChanged_signal,
  pub _opacityChanged: QGraphicsOpacityEffect_opacityChanged_signal,
}

// class sizeof(QGraphicsBlurEffect)=1
#[derive(Default)]
pub struct QGraphicsBlurEffect {
  qbase: QGraphicsEffect,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _blurHintsChanged: QGraphicsBlurEffect_blurHintsChanged_signal,
  pub _blurRadiusChanged: QGraphicsBlurEffect_blurRadiusChanged_signal,
}

impl /*struct*/ QGraphicsColorizeEffect {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsColorizeEffect {
    return QGraphicsColorizeEffect{qbase: QGraphicsEffect::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsColorizeEffect {
  type Target = QGraphicsEffect;

  fn deref(&self) -> &QGraphicsEffect {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsEffect> for QGraphicsColorizeEffect {
  fn as_ref(& self) -> & QGraphicsEffect {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsColorizeEffect::setColor(const QColor & c);
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn setColor<RetType, T: QGraphicsColorizeEffect_setColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColor(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_setColor<RetType> {
  fn setColor(self , rsthis: & QGraphicsColorizeEffect) -> RetType;
}

  // proto:  void QGraphicsColorizeEffect::setColor(const QColor & c);
impl<'a> /*trait*/ QGraphicsColorizeEffect_setColor<()> for (&'a QColor) {
  fn setColor(self , rsthis: & QGraphicsColorizeEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffect8setColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN23QGraphicsColorizeEffect8setColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsColorizeEffect::setStrength(qreal strength);
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn setStrength<RetType, T: QGraphicsColorizeEffect_setStrength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStrength(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_setStrength<RetType> {
  fn setStrength(self , rsthis: & QGraphicsColorizeEffect) -> RetType;
}

  // proto:  void QGraphicsColorizeEffect::setStrength(qreal strength);
impl<'a> /*trait*/ QGraphicsColorizeEffect_setStrength<()> for (f64) {
  fn setStrength(self , rsthis: & QGraphicsColorizeEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffect11setStrengthEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN23QGraphicsColorizeEffect11setStrengthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsColorizeEffect::QGraphicsColorizeEffect(QObject * parent);
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn new<T: QGraphicsColorizeEffect_new>(value: T) -> QGraphicsColorizeEffect {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_new {
  fn new(self) -> QGraphicsColorizeEffect;
}

  // proto:  void QGraphicsColorizeEffect::QGraphicsColorizeEffect(QObject * parent);
impl<'a> /*trait*/ QGraphicsColorizeEffect_new for (&'a QObject) {
  fn new(self) -> QGraphicsColorizeEffect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffectC2EP7QObject()};
    let ctysz: c_int = unsafe{QGraphicsColorizeEffect_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN23QGraphicsColorizeEffectC2EP7QObject(arg0)};
    let rsthis = QGraphicsColorizeEffect{qbase: QGraphicsEffect::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QGraphicsColorizeEffect::strength();
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn strength<RetType, T: QGraphicsColorizeEffect_strength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.strength(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_strength<RetType> {
  fn strength(self , rsthis: & QGraphicsColorizeEffect) -> RetType;
}

  // proto:  qreal QGraphicsColorizeEffect::strength();
impl<'a> /*trait*/ QGraphicsColorizeEffect_strength<f64> for () {
  fn strength(self , rsthis: & QGraphicsColorizeEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsColorizeEffect8strengthEv()};
    let mut ret = unsafe {C_ZNK23QGraphicsColorizeEffect8strengthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsColorizeEffect::~QGraphicsColorizeEffect();
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn free<RetType, T: QGraphicsColorizeEffect_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_free<RetType> {
  fn free(self , rsthis: & QGraphicsColorizeEffect) -> RetType;
}

  // proto:  void QGraphicsColorizeEffect::~QGraphicsColorizeEffect();
impl<'a> /*trait*/ QGraphicsColorizeEffect_free<()> for () {
  fn free(self , rsthis: & QGraphicsColorizeEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffectD2Ev()};
     unsafe {C_ZN23QGraphicsColorizeEffectD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QColor QGraphicsColorizeEffect::color();
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn color<RetType, T: QGraphicsColorizeEffect_color<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.color(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_color<RetType> {
  fn color(self , rsthis: & QGraphicsColorizeEffect) -> RetType;
}

  // proto:  QColor QGraphicsColorizeEffect::color();
impl<'a> /*trait*/ QGraphicsColorizeEffect_color<QColor> for () {
  fn color(self , rsthis: & QGraphicsColorizeEffect) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsColorizeEffect5colorEv()};
    let mut ret = unsafe {C_ZNK23QGraphicsColorizeEffect5colorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsColorizeEffect::metaObject();
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn metaObject<RetType, T: QGraphicsColorizeEffect_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGraphicsColorizeEffect) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsColorizeEffect::metaObject();
impl<'a> /*trait*/ QGraphicsColorizeEffect_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QGraphicsColorizeEffect) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsColorizeEffect10metaObjectEv()};
    let mut ret = unsafe {C_ZNK23QGraphicsColorizeEffect10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsEffect {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsEffect {
    return QGraphicsEffect{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsEffect {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QGraphicsEffect {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  QRectF QGraphicsEffect::boundingRectFor(const QRectF & sourceRect);
impl /*struct*/ QGraphicsEffect {
  pub fn boundingRectFor<RetType, T: QGraphicsEffect_boundingRectFor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRectFor(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_boundingRectFor<RetType> {
  fn boundingRectFor(self , rsthis: & QGraphicsEffect) -> RetType;
}

  // proto:  QRectF QGraphicsEffect::boundingRectFor(const QRectF & sourceRect);
impl<'a> /*trait*/ QGraphicsEffect_boundingRectFor<QRectF> for (&'a QRectF) {
  fn boundingRectFor(self , rsthis: & QGraphicsEffect) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsEffect15boundingRectForERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK15QGraphicsEffect15boundingRectForERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsEffectSource * QGraphicsEffect::source();
impl /*struct*/ QGraphicsEffect {
  pub fn source<RetType, T: QGraphicsEffect_source<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.source(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_source<RetType> {
  fn source(self , rsthis: & QGraphicsEffect) -> RetType;
}

  // proto:  QGraphicsEffectSource * QGraphicsEffect::source();
impl<'a> /*trait*/ QGraphicsEffect_source<()> for () {
  fn source(self , rsthis: & QGraphicsEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsEffect6sourceEv()};
     unsafe {C_ZNK15QGraphicsEffect6sourceEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsEffect::update();
impl /*struct*/ QGraphicsEffect {
  pub fn update<RetType, T: QGraphicsEffect_update<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.update(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_update<RetType> {
  fn update(self , rsthis: & QGraphicsEffect) -> RetType;
}

  // proto:  void QGraphicsEffect::update();
impl<'a> /*trait*/ QGraphicsEffect_update<()> for () {
  fn update(self , rsthis: & QGraphicsEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffect6updateEv()};
     unsafe {C_ZN15QGraphicsEffect6updateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsEffect::setEnabled(bool enable);
impl /*struct*/ QGraphicsEffect {
  pub fn setEnabled<RetType, T: QGraphicsEffect_setEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEnabled(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_setEnabled<RetType> {
  fn setEnabled(self , rsthis: & QGraphicsEffect) -> RetType;
}

  // proto:  void QGraphicsEffect::setEnabled(bool enable);
impl<'a> /*trait*/ QGraphicsEffect_setEnabled<()> for (i8) {
  fn setEnabled(self , rsthis: & QGraphicsEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffect10setEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN15QGraphicsEffect10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsEffect::metaObject();
impl /*struct*/ QGraphicsEffect {
  pub fn metaObject<RetType, T: QGraphicsEffect_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGraphicsEffect) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsEffect::metaObject();
impl<'a> /*trait*/ QGraphicsEffect_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QGraphicsEffect) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsEffect10metaObjectEv()};
    let mut ret = unsafe {C_ZNK15QGraphicsEffect10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsEffect::isEnabled();
impl /*struct*/ QGraphicsEffect {
  pub fn isEnabled<RetType, T: QGraphicsEffect_isEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEnabled(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_isEnabled<RetType> {
  fn isEnabled(self , rsthis: & QGraphicsEffect) -> RetType;
}

  // proto:  bool QGraphicsEffect::isEnabled();
impl<'a> /*trait*/ QGraphicsEffect_isEnabled<i8> for () {
  fn isEnabled(self , rsthis: & QGraphicsEffect) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsEffect9isEnabledEv()};
    let mut ret = unsafe {C_ZNK15QGraphicsEffect9isEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsEffect::boundingRect();
impl /*struct*/ QGraphicsEffect {
  pub fn boundingRect<RetType, T: QGraphicsEffect_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QGraphicsEffect) -> RetType;
}

  // proto:  QRectF QGraphicsEffect::boundingRect();
impl<'a> /*trait*/ QGraphicsEffect_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: & QGraphicsEffect) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsEffect12boundingRectEv()};
    let mut ret = unsafe {C_ZNK15QGraphicsEffect12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsEffect::~QGraphicsEffect();
impl /*struct*/ QGraphicsEffect {
  pub fn free<RetType, T: QGraphicsEffect_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_free<RetType> {
  fn free(self , rsthis: & QGraphicsEffect) -> RetType;
}

  // proto:  void QGraphicsEffect::~QGraphicsEffect();
impl<'a> /*trait*/ QGraphicsEffect_free<()> for () {
  fn free(self , rsthis: & QGraphicsEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffectD2Ev()};
     unsafe {C_ZN15QGraphicsEffectD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsEffect::QGraphicsEffect(QObject * parent);
impl /*struct*/ QGraphicsEffect {
  pub fn new<T: QGraphicsEffect_new>(value: T) -> QGraphicsEffect {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsEffect_new {
  fn new(self) -> QGraphicsEffect;
}

  // proto:  void QGraphicsEffect::QGraphicsEffect(QObject * parent);
impl<'a> /*trait*/ QGraphicsEffect_new for (&'a QObject) {
  fn new(self) -> QGraphicsEffect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffectC2EP7QObject()};
    let ctysz: c_int = unsafe{QGraphicsEffect_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN15QGraphicsEffectC2EP7QObject(arg0)};
    let rsthis = QGraphicsEffect{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsDropShadowEffect {
    return QGraphicsDropShadowEffect{qbase: QGraphicsEffect::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsDropShadowEffect {
  type Target = QGraphicsEffect;

  fn deref(&self) -> &QGraphicsEffect {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsEffect> for QGraphicsDropShadowEffect {
  fn as_ref(& self) -> & QGraphicsEffect {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QGraphicsDropShadowEffect::metaObject();
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn metaObject<RetType, T: QGraphicsDropShadowEffect_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsDropShadowEffect::metaObject();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QGraphicsDropShadowEffect) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect10metaObjectEv()};
    let mut ret = unsafe {C_ZNK25QGraphicsDropShadowEffect10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsDropShadowEffect::boundingRectFor(const QRectF & rect);
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn boundingRectFor<RetType, T: QGraphicsDropShadowEffect_boundingRectFor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRectFor(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_boundingRectFor<RetType> {
  fn boundingRectFor(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  QRectF QGraphicsDropShadowEffect::boundingRectFor(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_boundingRectFor<QRectF> for (&'a QRectF) {
  fn boundingRectFor(self , rsthis: & QGraphicsDropShadowEffect) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect15boundingRectForERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK25QGraphicsDropShadowEffect15boundingRectForERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::QGraphicsDropShadowEffect(QObject * parent);
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn new<T: QGraphicsDropShadowEffect_new>(value: T) -> QGraphicsDropShadowEffect {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_new {
  fn new(self) -> QGraphicsDropShadowEffect;
}

  // proto:  void QGraphicsDropShadowEffect::QGraphicsDropShadowEffect(QObject * parent);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_new for (&'a QObject) {
  fn new(self) -> QGraphicsDropShadowEffect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffectC2EP7QObject()};
    let ctysz: c_int = unsafe{QGraphicsDropShadowEffect_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN25QGraphicsDropShadowEffectC2EP7QObject(arg0)};
    let rsthis = QGraphicsDropShadowEffect{qbase: QGraphicsEffect::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsDropShadowEffect::offset();
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn offset<RetType, T: QGraphicsDropShadowEffect_offset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.offset(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_offset<RetType> {
  fn offset(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  QPointF QGraphicsDropShadowEffect::offset();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_offset<QPointF> for () {
  fn offset(self , rsthis: & QGraphicsDropShadowEffect) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect6offsetEv()};
    let mut ret = unsafe {C_ZNK25QGraphicsDropShadowEffect6offsetEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::setYOffset(qreal dy);
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setYOffset<RetType, T: QGraphicsDropShadowEffect_setYOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setYOffset(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setYOffset<RetType> {
  fn setYOffset(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  void QGraphicsDropShadowEffect::setYOffset(qreal dy);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setYOffset<()> for (f64) {
  fn setYOffset(self , rsthis: & QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect10setYOffsetEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN25QGraphicsDropShadowEffect10setYOffsetEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsDropShadowEffect::xOffset();
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn xOffset<RetType, T: QGraphicsDropShadowEffect_xOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.xOffset(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_xOffset<RetType> {
  fn xOffset(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  qreal QGraphicsDropShadowEffect::xOffset();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_xOffset<f64> for () {
  fn xOffset(self , rsthis: & QGraphicsDropShadowEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect7xOffsetEv()};
    let mut ret = unsafe {C_ZNK25QGraphicsDropShadowEffect7xOffsetEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QGraphicsDropShadowEffect::blurRadius();
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn blurRadius<RetType, T: QGraphicsDropShadowEffect_blurRadius<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.blurRadius(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_blurRadius<RetType> {
  fn blurRadius(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  qreal QGraphicsDropShadowEffect::blurRadius();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_blurRadius<f64> for () {
  fn blurRadius(self , rsthis: & QGraphicsDropShadowEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect10blurRadiusEv()};
    let mut ret = unsafe {C_ZNK25QGraphicsDropShadowEffect10blurRadiusEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QColor QGraphicsDropShadowEffect::color();
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn color<RetType, T: QGraphicsDropShadowEffect_color<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.color(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_color<RetType> {
  fn color(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  QColor QGraphicsDropShadowEffect::color();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_color<QColor> for () {
  fn color(self , rsthis: & QGraphicsDropShadowEffect) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect5colorEv()};
    let mut ret = unsafe {C_ZNK25QGraphicsDropShadowEffect5colorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::setColor(const QColor & color);
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setColor<RetType, T: QGraphicsDropShadowEffect_setColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColor(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setColor<RetType> {
  fn setColor(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  void QGraphicsDropShadowEffect::setColor(const QColor & color);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setColor<()> for (&'a QColor) {
  fn setColor(self , rsthis: & QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect8setColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN25QGraphicsDropShadowEffect8setColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::setOffset(const QPointF & ofs);
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setOffset<RetType, T: QGraphicsDropShadowEffect_setOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOffset(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setOffset<RetType> {
  fn setOffset(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  void QGraphicsDropShadowEffect::setOffset(const QPointF & ofs);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setOffset<()> for (&'a QPointF) {
  fn setOffset(self , rsthis: & QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect9setOffsetERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN25QGraphicsDropShadowEffect9setOffsetERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::setOffset(qreal dx, qreal dy);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setOffset<()> for (f64, f64) {
  fn setOffset(self , rsthis: & QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect9setOffsetEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {C_ZN25QGraphicsDropShadowEffect9setOffsetEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::setOffset(qreal d);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setOffset<()> for (f64) {
  fn setOffset(self , rsthis: & QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect9setOffsetEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN25QGraphicsDropShadowEffect9setOffsetEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsDropShadowEffect::yOffset();
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn yOffset<RetType, T: QGraphicsDropShadowEffect_yOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.yOffset(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_yOffset<RetType> {
  fn yOffset(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  qreal QGraphicsDropShadowEffect::yOffset();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_yOffset<f64> for () {
  fn yOffset(self , rsthis: & QGraphicsDropShadowEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect7yOffsetEv()};
    let mut ret = unsafe {C_ZNK25QGraphicsDropShadowEffect7yOffsetEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::setXOffset(qreal dx);
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setXOffset<RetType, T: QGraphicsDropShadowEffect_setXOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setXOffset(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setXOffset<RetType> {
  fn setXOffset(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  void QGraphicsDropShadowEffect::setXOffset(qreal dx);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setXOffset<()> for (f64) {
  fn setXOffset(self , rsthis: & QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect10setXOffsetEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN25QGraphicsDropShadowEffect10setXOffsetEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::setBlurRadius(qreal blurRadius);
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setBlurRadius<RetType, T: QGraphicsDropShadowEffect_setBlurRadius<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBlurRadius(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setBlurRadius<RetType> {
  fn setBlurRadius(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  void QGraphicsDropShadowEffect::setBlurRadius(qreal blurRadius);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setBlurRadius<()> for (f64) {
  fn setBlurRadius(self , rsthis: & QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect13setBlurRadiusEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN25QGraphicsDropShadowEffect13setBlurRadiusEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::~QGraphicsDropShadowEffect();
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn free<RetType, T: QGraphicsDropShadowEffect_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_free<RetType> {
  fn free(self , rsthis: & QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  void QGraphicsDropShadowEffect::~QGraphicsDropShadowEffect();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_free<()> for () {
  fn free(self , rsthis: & QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffectD2Ev()};
     unsafe {C_ZN25QGraphicsDropShadowEffectD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsOpacityEffect {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsOpacityEffect {
    return QGraphicsOpacityEffect{qbase: QGraphicsEffect::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsOpacityEffect {
  type Target = QGraphicsEffect;

  fn deref(&self) -> &QGraphicsEffect {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsEffect> for QGraphicsOpacityEffect {
  fn as_ref(& self) -> & QGraphicsEffect {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsOpacityEffect::QGraphicsOpacityEffect(QObject * parent);
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn new<T: QGraphicsOpacityEffect_new>(value: T) -> QGraphicsOpacityEffect {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_new {
  fn new(self) -> QGraphicsOpacityEffect;
}

  // proto:  void QGraphicsOpacityEffect::QGraphicsOpacityEffect(QObject * parent);
impl<'a> /*trait*/ QGraphicsOpacityEffect_new for (&'a QObject) {
  fn new(self) -> QGraphicsOpacityEffect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffectC2EP7QObject()};
    let ctysz: c_int = unsafe{QGraphicsOpacityEffect_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN22QGraphicsOpacityEffectC2EP7QObject(arg0)};
    let rsthis = QGraphicsOpacityEffect{qbase: QGraphicsEffect::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsOpacityEffect::~QGraphicsOpacityEffect();
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn free<RetType, T: QGraphicsOpacityEffect_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_free<RetType> {
  fn free(self , rsthis: & QGraphicsOpacityEffect) -> RetType;
}

  // proto:  void QGraphicsOpacityEffect::~QGraphicsOpacityEffect();
impl<'a> /*trait*/ QGraphicsOpacityEffect_free<()> for () {
  fn free(self , rsthis: & QGraphicsOpacityEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffectD2Ev()};
     unsafe {C_ZN22QGraphicsOpacityEffectD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsOpacityEffect::setOpacityMask(const QBrush & mask);
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn setOpacityMask<RetType, T: QGraphicsOpacityEffect_setOpacityMask<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOpacityMask(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_setOpacityMask<RetType> {
  fn setOpacityMask(self , rsthis: & QGraphicsOpacityEffect) -> RetType;
}

  // proto:  void QGraphicsOpacityEffect::setOpacityMask(const QBrush & mask);
impl<'a> /*trait*/ QGraphicsOpacityEffect_setOpacityMask<()> for (&'a QBrush) {
  fn setOpacityMask(self , rsthis: & QGraphicsOpacityEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffect14setOpacityMaskERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN22QGraphicsOpacityEffect14setOpacityMaskERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QBrush QGraphicsOpacityEffect::opacityMask();
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacityMask<RetType, T: QGraphicsOpacityEffect_opacityMask<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opacityMask(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_opacityMask<RetType> {
  fn opacityMask(self , rsthis: & QGraphicsOpacityEffect) -> RetType;
}

  // proto:  QBrush QGraphicsOpacityEffect::opacityMask();
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacityMask<QBrush> for () {
  fn opacityMask(self , rsthis: & QGraphicsOpacityEffect) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsOpacityEffect11opacityMaskEv()};
    let mut ret = unsafe {C_ZNK22QGraphicsOpacityEffect11opacityMaskEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsOpacityEffect::metaObject();
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn metaObject<RetType, T: QGraphicsOpacityEffect_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGraphicsOpacityEffect) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsOpacityEffect::metaObject();
impl<'a> /*trait*/ QGraphicsOpacityEffect_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QGraphicsOpacityEffect) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsOpacityEffect10metaObjectEv()};
    let mut ret = unsafe {C_ZNK22QGraphicsOpacityEffect10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QGraphicsOpacityEffect::opacity();
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacity<RetType, T: QGraphicsOpacityEffect_opacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opacity(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_opacity<RetType> {
  fn opacity(self , rsthis: & QGraphicsOpacityEffect) -> RetType;
}

  // proto:  qreal QGraphicsOpacityEffect::opacity();
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacity<f64> for () {
  fn opacity(self , rsthis: & QGraphicsOpacityEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsOpacityEffect7opacityEv()};
    let mut ret = unsafe {C_ZNK22QGraphicsOpacityEffect7opacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsOpacityEffect::setOpacity(qreal opacity);
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn setOpacity<RetType, T: QGraphicsOpacityEffect_setOpacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOpacity(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_setOpacity<RetType> {
  fn setOpacity(self , rsthis: & QGraphicsOpacityEffect) -> RetType;
}

  // proto:  void QGraphicsOpacityEffect::setOpacity(qreal opacity);
impl<'a> /*trait*/ QGraphicsOpacityEffect_setOpacity<()> for (f64) {
  fn setOpacity(self , rsthis: & QGraphicsOpacityEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffect10setOpacityEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN22QGraphicsOpacityEffect10setOpacityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsBlurEffect {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsBlurEffect {
    return QGraphicsBlurEffect{qbase: QGraphicsEffect::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsBlurEffect {
  type Target = QGraphicsEffect;

  fn deref(&self) -> &QGraphicsEffect {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsEffect> for QGraphicsBlurEffect {
  fn as_ref(& self) -> & QGraphicsEffect {
    return & self.qbase;
  }
}
  // proto:  qreal QGraphicsBlurEffect::blurRadius();
impl /*struct*/ QGraphicsBlurEffect {
  pub fn blurRadius<RetType, T: QGraphicsBlurEffect_blurRadius<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.blurRadius(self);
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_blurRadius<RetType> {
  fn blurRadius(self , rsthis: & QGraphicsBlurEffect) -> RetType;
}

  // proto:  qreal QGraphicsBlurEffect::blurRadius();
impl<'a> /*trait*/ QGraphicsBlurEffect_blurRadius<f64> for () {
  fn blurRadius(self , rsthis: & QGraphicsBlurEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsBlurEffect10blurRadiusEv()};
    let mut ret = unsafe {C_ZNK19QGraphicsBlurEffect10blurRadiusEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsBlurEffect::setBlurRadius(qreal blurRadius);
impl /*struct*/ QGraphicsBlurEffect {
  pub fn setBlurRadius<RetType, T: QGraphicsBlurEffect_setBlurRadius<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBlurRadius(self);
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_setBlurRadius<RetType> {
  fn setBlurRadius(self , rsthis: & QGraphicsBlurEffect) -> RetType;
}

  // proto:  void QGraphicsBlurEffect::setBlurRadius(qreal blurRadius);
impl<'a> /*trait*/ QGraphicsBlurEffect_setBlurRadius<()> for (f64) {
  fn setBlurRadius(self , rsthis: & QGraphicsBlurEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsBlurEffect13setBlurRadiusEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN19QGraphicsBlurEffect13setBlurRadiusEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsBlurEffect::~QGraphicsBlurEffect();
impl /*struct*/ QGraphicsBlurEffect {
  pub fn free<RetType, T: QGraphicsBlurEffect_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_free<RetType> {
  fn free(self , rsthis: & QGraphicsBlurEffect) -> RetType;
}

  // proto:  void QGraphicsBlurEffect::~QGraphicsBlurEffect();
impl<'a> /*trait*/ QGraphicsBlurEffect_free<()> for () {
  fn free(self , rsthis: & QGraphicsBlurEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsBlurEffectD2Ev()};
     unsafe {C_ZN19QGraphicsBlurEffectD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsBlurEffect::metaObject();
impl /*struct*/ QGraphicsBlurEffect {
  pub fn metaObject<RetType, T: QGraphicsBlurEffect_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGraphicsBlurEffect) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsBlurEffect::metaObject();
impl<'a> /*trait*/ QGraphicsBlurEffect_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QGraphicsBlurEffect) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsBlurEffect10metaObjectEv()};
    let mut ret = unsafe {C_ZNK19QGraphicsBlurEffect10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsBlurEffect::boundingRectFor(const QRectF & rect);
impl /*struct*/ QGraphicsBlurEffect {
  pub fn boundingRectFor<RetType, T: QGraphicsBlurEffect_boundingRectFor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRectFor(self);
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_boundingRectFor<RetType> {
  fn boundingRectFor(self , rsthis: & QGraphicsBlurEffect) -> RetType;
}

  // proto:  QRectF QGraphicsBlurEffect::boundingRectFor(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsBlurEffect_boundingRectFor<QRectF> for (&'a QRectF) {
  fn boundingRectFor(self , rsthis: & QGraphicsBlurEffect) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsBlurEffect15boundingRectForERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK19QGraphicsBlurEffect15boundingRectForERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsBlurEffect::QGraphicsBlurEffect(QObject * parent);
impl /*struct*/ QGraphicsBlurEffect {
  pub fn new<T: QGraphicsBlurEffect_new>(value: T) -> QGraphicsBlurEffect {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_new {
  fn new(self) -> QGraphicsBlurEffect;
}

  // proto:  void QGraphicsBlurEffect::QGraphicsBlurEffect(QObject * parent);
impl<'a> /*trait*/ QGraphicsBlurEffect_new for (&'a QObject) {
  fn new(self) -> QGraphicsBlurEffect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsBlurEffectC2EP7QObject()};
    let ctysz: c_int = unsafe{QGraphicsBlurEffect_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN19QGraphicsBlurEffectC2EP7QObject(arg0)};
    let rsthis = QGraphicsBlurEffect{qbase: QGraphicsEffect::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

#[derive(Default)] // for QGraphicsColorizeEffect_strengthChanged
pub struct QGraphicsColorizeEffect_strengthChanged_signal{poi:u64}
impl /* struct */ QGraphicsColorizeEffect {
  pub fn strengthChanged(&self) -> QGraphicsColorizeEffect_strengthChanged_signal {
     return QGraphicsColorizeEffect_strengthChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsColorizeEffect_strengthChanged_signal {
  pub fn connect<T: QGraphicsColorizeEffect_strengthChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsColorizeEffect_strengthChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsColorizeEffect_strengthChanged_signal);
}

#[derive(Default)] // for QGraphicsColorizeEffect_colorChanged
pub struct QGraphicsColorizeEffect_colorChanged_signal{poi:u64}
impl /* struct */ QGraphicsColorizeEffect {
  pub fn colorChanged(&self) -> QGraphicsColorizeEffect_colorChanged_signal {
     return QGraphicsColorizeEffect_colorChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsColorizeEffect_colorChanged_signal {
  pub fn connect<T: QGraphicsColorizeEffect_colorChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsColorizeEffect_colorChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsColorizeEffect_colorChanged_signal);
}

// strengthChanged(qreal)
extern fn QGraphicsColorizeEffect_strengthChanged_signal_connect_cb_0(rsfptr:fn(f64), arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as f64;
  rsfptr(rsarg0);
}
extern fn QGraphicsColorizeEffect_strengthChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(f64)>, arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as f64;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QGraphicsColorizeEffect_strengthChanged_signal_connect for fn(f64) {
  fn connect(self, sigthis: QGraphicsColorizeEffect_strengthChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsColorizeEffect_strengthChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsColorizeEffect_SlotProxy_connect__ZN23QGraphicsColorizeEffect15strengthChangedEd(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsColorizeEffect_strengthChanged_signal_connect for Box<Fn(f64)> {
  fn connect(self, sigthis: QGraphicsColorizeEffect_strengthChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsColorizeEffect_strengthChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsColorizeEffect_SlotProxy_connect__ZN23QGraphicsColorizeEffect15strengthChangedEd(arg0, arg1, arg2)};
  }
}
// colorChanged(const class QColor &)
extern fn QGraphicsColorizeEffect_colorChanged_signal_connect_cb_1(rsfptr:fn(QColor), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QColor::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QGraphicsColorizeEffect_colorChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QColor)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QColor::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QGraphicsColorizeEffect_colorChanged_signal_connect for fn(QColor) {
  fn connect(self, sigthis: QGraphicsColorizeEffect_colorChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsColorizeEffect_colorChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsColorizeEffect_SlotProxy_connect__ZN23QGraphicsColorizeEffect12colorChangedERK6QColor(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsColorizeEffect_colorChanged_signal_connect for Box<Fn(QColor)> {
  fn connect(self, sigthis: QGraphicsColorizeEffect_colorChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsColorizeEffect_colorChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsColorizeEffect_SlotProxy_connect__ZN23QGraphicsColorizeEffect12colorChangedERK6QColor(arg0, arg1, arg2)};
  }
}
#[derive(Default)] // for QGraphicsEffect_enabledChanged
pub struct QGraphicsEffect_enabledChanged_signal{poi:u64}
impl /* struct */ QGraphicsEffect {
  pub fn enabledChanged(&self) -> QGraphicsEffect_enabledChanged_signal {
     return QGraphicsEffect_enabledChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsEffect_enabledChanged_signal {
  pub fn connect<T: QGraphicsEffect_enabledChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsEffect_enabledChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsEffect_enabledChanged_signal);
}

// enabledChanged(_Bool)
extern fn QGraphicsEffect_enabledChanged_signal_connect_cb_0(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QGraphicsEffect_enabledChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QGraphicsEffect_enabledChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QGraphicsEffect_enabledChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsEffect_enabledChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsEffect_SlotProxy_connect__ZN15QGraphicsEffect14enabledChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsEffect_enabledChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QGraphicsEffect_enabledChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsEffect_enabledChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsEffect_SlotProxy_connect__ZN15QGraphicsEffect14enabledChangedEb(arg0, arg1, arg2)};
  }
}
#[derive(Default)] // for QGraphicsDropShadowEffect_colorChanged
pub struct QGraphicsDropShadowEffect_colorChanged_signal{poi:u64}
impl /* struct */ QGraphicsDropShadowEffect {
  pub fn colorChanged(&self) -> QGraphicsDropShadowEffect_colorChanged_signal {
     return QGraphicsDropShadowEffect_colorChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsDropShadowEffect_colorChanged_signal {
  pub fn connect<T: QGraphicsDropShadowEffect_colorChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsDropShadowEffect_colorChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsDropShadowEffect_colorChanged_signal);
}

#[derive(Default)] // for QGraphicsDropShadowEffect_offsetChanged
pub struct QGraphicsDropShadowEffect_offsetChanged_signal{poi:u64}
impl /* struct */ QGraphicsDropShadowEffect {
  pub fn offsetChanged(&self) -> QGraphicsDropShadowEffect_offsetChanged_signal {
     return QGraphicsDropShadowEffect_offsetChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsDropShadowEffect_offsetChanged_signal {
  pub fn connect<T: QGraphicsDropShadowEffect_offsetChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsDropShadowEffect_offsetChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsDropShadowEffect_offsetChanged_signal);
}

#[derive(Default)] // for QGraphicsDropShadowEffect_blurRadiusChanged
pub struct QGraphicsDropShadowEffect_blurRadiusChanged_signal{poi:u64}
impl /* struct */ QGraphicsDropShadowEffect {
  pub fn blurRadiusChanged(&self) -> QGraphicsDropShadowEffect_blurRadiusChanged_signal {
     return QGraphicsDropShadowEffect_blurRadiusChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsDropShadowEffect_blurRadiusChanged_signal {
  pub fn connect<T: QGraphicsDropShadowEffect_blurRadiusChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsDropShadowEffect_blurRadiusChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsDropShadowEffect_blurRadiusChanged_signal);
}

// blurRadiusChanged(qreal)
extern fn QGraphicsDropShadowEffect_blurRadiusChanged_signal_connect_cb_0(rsfptr:fn(f64), arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as f64;
  rsfptr(rsarg0);
}
extern fn QGraphicsDropShadowEffect_blurRadiusChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(f64)>, arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as f64;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QGraphicsDropShadowEffect_blurRadiusChanged_signal_connect for fn(f64) {
  fn connect(self, sigthis: QGraphicsDropShadowEffect_blurRadiusChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsDropShadowEffect_blurRadiusChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsDropShadowEffect_SlotProxy_connect__ZN25QGraphicsDropShadowEffect17blurRadiusChangedEd(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsDropShadowEffect_blurRadiusChanged_signal_connect for Box<Fn(f64)> {
  fn connect(self, sigthis: QGraphicsDropShadowEffect_blurRadiusChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsDropShadowEffect_blurRadiusChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsDropShadowEffect_SlotProxy_connect__ZN25QGraphicsDropShadowEffect17blurRadiusChangedEd(arg0, arg1, arg2)};
  }
}
// colorChanged(const class QColor &)
extern fn QGraphicsDropShadowEffect_colorChanged_signal_connect_cb_1(rsfptr:fn(QColor), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QColor::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QGraphicsDropShadowEffect_colorChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QColor)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QColor::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QGraphicsDropShadowEffect_colorChanged_signal_connect for fn(QColor) {
  fn connect(self, sigthis: QGraphicsDropShadowEffect_colorChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsDropShadowEffect_colorChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsDropShadowEffect_SlotProxy_connect__ZN25QGraphicsDropShadowEffect12colorChangedERK6QColor(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsDropShadowEffect_colorChanged_signal_connect for Box<Fn(QColor)> {
  fn connect(self, sigthis: QGraphicsDropShadowEffect_colorChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsDropShadowEffect_colorChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsDropShadowEffect_SlotProxy_connect__ZN25QGraphicsDropShadowEffect12colorChangedERK6QColor(arg0, arg1, arg2)};
  }
}
// offsetChanged(const class QPointF &)
extern fn QGraphicsDropShadowEffect_offsetChanged_signal_connect_cb_2(rsfptr:fn(QPointF), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QPointF::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QGraphicsDropShadowEffect_offsetChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(QPointF)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QPointF::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QGraphicsDropShadowEffect_offsetChanged_signal_connect for fn(QPointF) {
  fn connect(self, sigthis: QGraphicsDropShadowEffect_offsetChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsDropShadowEffect_offsetChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsDropShadowEffect_SlotProxy_connect__ZN25QGraphicsDropShadowEffect13offsetChangedERK7QPointF(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsDropShadowEffect_offsetChanged_signal_connect for Box<Fn(QPointF)> {
  fn connect(self, sigthis: QGraphicsDropShadowEffect_offsetChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsDropShadowEffect_offsetChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsDropShadowEffect_SlotProxy_connect__ZN25QGraphicsDropShadowEffect13offsetChangedERK7QPointF(arg0, arg1, arg2)};
  }
}
#[derive(Default)] // for QGraphicsOpacityEffect_opacityMaskChanged
pub struct QGraphicsOpacityEffect_opacityMaskChanged_signal{poi:u64}
impl /* struct */ QGraphicsOpacityEffect {
  pub fn opacityMaskChanged(&self) -> QGraphicsOpacityEffect_opacityMaskChanged_signal {
     return QGraphicsOpacityEffect_opacityMaskChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsOpacityEffect_opacityMaskChanged_signal {
  pub fn connect<T: QGraphicsOpacityEffect_opacityMaskChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsOpacityEffect_opacityMaskChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsOpacityEffect_opacityMaskChanged_signal);
}

#[derive(Default)] // for QGraphicsOpacityEffect_opacityChanged
pub struct QGraphicsOpacityEffect_opacityChanged_signal{poi:u64}
impl /* struct */ QGraphicsOpacityEffect {
  pub fn opacityChanged(&self) -> QGraphicsOpacityEffect_opacityChanged_signal {
     return QGraphicsOpacityEffect_opacityChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsOpacityEffect_opacityChanged_signal {
  pub fn connect<T: QGraphicsOpacityEffect_opacityChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsOpacityEffect_opacityChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsOpacityEffect_opacityChanged_signal);
}

// opacityChanged(qreal)
extern fn QGraphicsOpacityEffect_opacityChanged_signal_connect_cb_0(rsfptr:fn(f64), arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as f64;
  rsfptr(rsarg0);
}
extern fn QGraphicsOpacityEffect_opacityChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(f64)>, arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as f64;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QGraphicsOpacityEffect_opacityChanged_signal_connect for fn(f64) {
  fn connect(self, sigthis: QGraphicsOpacityEffect_opacityChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsOpacityEffect_opacityChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsOpacityEffect_SlotProxy_connect__ZN22QGraphicsOpacityEffect14opacityChangedEd(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsOpacityEffect_opacityChanged_signal_connect for Box<Fn(f64)> {
  fn connect(self, sigthis: QGraphicsOpacityEffect_opacityChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsOpacityEffect_opacityChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsOpacityEffect_SlotProxy_connect__ZN22QGraphicsOpacityEffect14opacityChangedEd(arg0, arg1, arg2)};
  }
}
// opacityMaskChanged(const class QBrush &)
extern fn QGraphicsOpacityEffect_opacityMaskChanged_signal_connect_cb_1(rsfptr:fn(QBrush), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QBrush::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QGraphicsOpacityEffect_opacityMaskChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QBrush)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QBrush::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QGraphicsOpacityEffect_opacityMaskChanged_signal_connect for fn(QBrush) {
  fn connect(self, sigthis: QGraphicsOpacityEffect_opacityMaskChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsOpacityEffect_opacityMaskChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsOpacityEffect_SlotProxy_connect__ZN22QGraphicsOpacityEffect18opacityMaskChangedERK6QBrush(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsOpacityEffect_opacityMaskChanged_signal_connect for Box<Fn(QBrush)> {
  fn connect(self, sigthis: QGraphicsOpacityEffect_opacityMaskChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsOpacityEffect_opacityMaskChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsOpacityEffect_SlotProxy_connect__ZN22QGraphicsOpacityEffect18opacityMaskChangedERK6QBrush(arg0, arg1, arg2)};
  }
}
#[derive(Default)] // for QGraphicsBlurEffect_blurHintsChanged
pub struct QGraphicsBlurEffect_blurHintsChanged_signal{poi:u64}
impl /* struct */ QGraphicsBlurEffect {
  pub fn blurHintsChanged(&self) -> QGraphicsBlurEffect_blurHintsChanged_signal {
     return QGraphicsBlurEffect_blurHintsChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsBlurEffect_blurHintsChanged_signal {
  pub fn connect<T: QGraphicsBlurEffect_blurHintsChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsBlurEffect_blurHintsChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsBlurEffect_blurHintsChanged_signal);
}

#[derive(Default)] // for QGraphicsBlurEffect_blurRadiusChanged
pub struct QGraphicsBlurEffect_blurRadiusChanged_signal{poi:u64}
impl /* struct */ QGraphicsBlurEffect {
  pub fn blurRadiusChanged(&self) -> QGraphicsBlurEffect_blurRadiusChanged_signal {
     return QGraphicsBlurEffect_blurRadiusChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsBlurEffect_blurRadiusChanged_signal {
  pub fn connect<T: QGraphicsBlurEffect_blurRadiusChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsBlurEffect_blurRadiusChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsBlurEffect_blurRadiusChanged_signal);
}

// blurRadiusChanged(qreal)
extern fn QGraphicsBlurEffect_blurRadiusChanged_signal_connect_cb_0(rsfptr:fn(f64), arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as f64;
  rsfptr(rsarg0);
}
extern fn QGraphicsBlurEffect_blurRadiusChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(f64)>, arg0: c_double) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as f64;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QGraphicsBlurEffect_blurRadiusChanged_signal_connect for fn(f64) {
  fn connect(self, sigthis: QGraphicsBlurEffect_blurRadiusChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsBlurEffect_blurRadiusChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsBlurEffect_SlotProxy_connect__ZN19QGraphicsBlurEffect17blurRadiusChangedEd(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsBlurEffect_blurRadiusChanged_signal_connect for Box<Fn(f64)> {
  fn connect(self, sigthis: QGraphicsBlurEffect_blurRadiusChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsBlurEffect_blurRadiusChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsBlurEffect_SlotProxy_connect__ZN19QGraphicsBlurEffect17blurRadiusChangedEd(arg0, arg1, arg2)};
  }
}
// <= body block end

