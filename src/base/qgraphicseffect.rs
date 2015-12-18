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
  // proto:  QRectF QGraphicsEffect::boundingRectFor(const QRectF & sourceRect);
  fn _ZNK15QGraphicsEffect15boundingRectForERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QGraphicsEffectSource * QGraphicsEffect::source();
  fn _ZNK15QGraphicsEffect6sourceEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsEffect::update();
  fn _ZN15QGraphicsEffect6updateEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsEffect::setEnabled(bool enable);
  fn _ZN15QGraphicsEffect10setEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QGraphicsEffect::enabledChanged(bool enabled);
  fn _ZN15QGraphicsEffect14enabledChangedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  const QMetaObject * QGraphicsEffect::metaObject();
  fn _ZNK15QGraphicsEffect10metaObjectEv(qthis: *mut c_void) ;
  // proto:  bool QGraphicsEffect::isEnabled();
  fn _ZNK15QGraphicsEffect9isEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QGraphicsEffect::FreeQGraphicsEffect();
  fn _ZN15QGraphicsEffectD0Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsEffect::NewQGraphicsEffect(const QGraphicsEffect & );
  fn _ZN15QGraphicsEffectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsEffect::NewQGraphicsEffect(QObject * parent);
  fn _ZN15QGraphicsEffectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QGraphicsEffect)=1
pub struct QGraphicsEffect {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsEffect {
  pub fn boundingRectFor<RetType, T: QGraphicsEffect_boundingRectFor<RetType>>(&mut self, value: T) -> RetType {
    return value.boundingRectFor(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_boundingRectFor<RetType> {
  fn boundingRectFor(self, rsthis: &mut QGraphicsEffect) -> RetType;
}

// proto:  QRectF QGraphicsEffect::boundingRectFor(const QRectF & sourceRect);
impl<'a> /*trait*/ QGraphicsEffect_boundingRectFor<QRectF> for (&'a  QRectF) {
  fn boundingRectFor(self, rsthis: &mut QGraphicsEffect) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsEffect15boundingRectForERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK15QGraphicsEffect15boundingRectForERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsEffect {
  pub fn source<RetType, T: QGraphicsEffect_source<RetType>>(&mut self, value: T) -> RetType {
    return value.source(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_source<RetType> {
  fn source(self, rsthis: &mut QGraphicsEffect) -> RetType;
}

// proto:  QGraphicsEffectSource * QGraphicsEffect::source();
impl<'a> /*trait*/ QGraphicsEffect_source<()> for () {
  fn source(self, rsthis: &mut QGraphicsEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsEffect6sourceEv()};
     unsafe {_ZNK15QGraphicsEffect6sourceEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsEffect {
  pub fn update<RetType, T: QGraphicsEffect_update<RetType>>(&mut self, value: T) -> RetType {
    return value.update(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_update<RetType> {
  fn update(self, rsthis: &mut QGraphicsEffect) -> RetType;
}

// proto:  void QGraphicsEffect::update();
impl<'a> /*trait*/ QGraphicsEffect_update<()> for () {
  fn update(self, rsthis: &mut QGraphicsEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffect6updateEv()};
     unsafe {_ZN15QGraphicsEffect6updateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsEffect {
  pub fn setEnabled<RetType, T: QGraphicsEffect_setEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setEnabled(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_setEnabled<RetType> {
  fn setEnabled(self, rsthis: &mut QGraphicsEffect) -> RetType;
}

// proto:  void QGraphicsEffect::setEnabled(bool enable);
impl<'a> /*trait*/ QGraphicsEffect_setEnabled<()> for (i8) {
  fn setEnabled(self, rsthis: &mut QGraphicsEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffect10setEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QGraphicsEffect10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsEffect {
  pub fn enabledChanged<RetType, T: QGraphicsEffect_enabledChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.enabledChanged(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_enabledChanged<RetType> {
  fn enabledChanged(self, rsthis: &mut QGraphicsEffect) -> RetType;
}

// proto:  void QGraphicsEffect::enabledChanged(bool enabled);
impl<'a> /*trait*/ QGraphicsEffect_enabledChanged<()> for (i8) {
  fn enabledChanged(self, rsthis: &mut QGraphicsEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffect14enabledChangedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QGraphicsEffect14enabledChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsEffect {
  pub fn metaObject<RetType, T: QGraphicsEffect_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QGraphicsEffect) -> RetType;
}

// proto:  const QMetaObject * QGraphicsEffect::metaObject();
impl<'a> /*trait*/ QGraphicsEffect_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QGraphicsEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsEffect10metaObjectEv()};
     unsafe {_ZNK15QGraphicsEffect10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsEffect {
  pub fn isEnabled<RetType, T: QGraphicsEffect_isEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.isEnabled(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_isEnabled<RetType> {
  fn isEnabled(self, rsthis: &mut QGraphicsEffect) -> RetType;
}

// proto:  bool QGraphicsEffect::isEnabled();
impl<'a> /*trait*/ QGraphicsEffect_isEnabled<i8> for () {
  fn isEnabled(self, rsthis: &mut QGraphicsEffect) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsEffect9isEnabledEv()};
    let mut ret = unsafe {_ZNK15QGraphicsEffect9isEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsEffect {
  pub fn FreeQGraphicsEffect<RetType, T: QGraphicsEffect_FreeQGraphicsEffect<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQGraphicsEffect(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_FreeQGraphicsEffect<RetType> {
  fn FreeQGraphicsEffect(self, rsthis: &mut QGraphicsEffect) -> RetType;
}

// proto:  void QGraphicsEffect::FreeQGraphicsEffect();
impl<'a> /*trait*/ QGraphicsEffect_FreeQGraphicsEffect<()> for () {
  fn FreeQGraphicsEffect(self, rsthis: &mut QGraphicsEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffectD0Ev()};
     unsafe {_ZN15QGraphicsEffectD0Ev(rsthis.qclsinst)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
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

