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
  // proto:  qreal QGraphicsBlurEffect::blurRadius();
  fn _ZNK19QGraphicsBlurEffect10blurRadiusEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsBlurEffect::setBlurRadius(qreal blurRadius);
  fn _ZN19QGraphicsBlurEffect13setBlurRadiusEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsBlurEffect::~QGraphicsBlurEffect();
  fn _ZN19QGraphicsBlurEffectD0Ev(qthis: *mut c_void);
  // proto:  void QGraphicsBlurEffect::QGraphicsBlurEffect(const QGraphicsBlurEffect & );
  fn _ZN19QGraphicsBlurEffectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QGraphicsBlurEffect::metaObject();
  fn _ZNK19QGraphicsBlurEffect10metaObjectEv(qthis: *mut c_void);
  // proto:  QRectF QGraphicsBlurEffect::boundingRectFor(const QRectF & rect);
  fn _ZNK19QGraphicsBlurEffect15boundingRectForERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsBlurEffect::QGraphicsBlurEffect(QObject * parent);
  fn _ZN19QGraphicsBlurEffectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsBlurEffect::blurRadiusChanged(qreal blurRadius);
  fn _ZN19QGraphicsBlurEffect17blurRadiusChangedEd(qthis: *mut c_void, arg0: c_double);
}

// body block begin
// class sizeof(QGraphicsBlurEffect)=1
pub struct QGraphicsBlurEffect {
  pub qclsinst: *mut c_void,
}

  // proto:  qreal QGraphicsBlurEffect::blurRadius();
impl /*struct*/ QGraphicsBlurEffect {
  pub fn blurRadius<RetType, T: QGraphicsBlurEffect_blurRadius<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.blurRadius(self);
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_blurRadius<RetType> {
  fn blurRadius(self , rsthis: &mut QGraphicsBlurEffect) -> RetType;
}

  // proto:  qreal QGraphicsBlurEffect::blurRadius();
impl<'a> /*trait*/ QGraphicsBlurEffect_blurRadius<f64> for () {
  fn blurRadius(self , rsthis: &mut QGraphicsBlurEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsBlurEffect10blurRadiusEv()};
    let mut ret = unsafe {_ZNK19QGraphicsBlurEffect10blurRadiusEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsBlurEffect::setBlurRadius(qreal blurRadius);
impl /*struct*/ QGraphicsBlurEffect {
  pub fn setBlurRadius<RetType, T: QGraphicsBlurEffect_setBlurRadius<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBlurRadius(self);
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_setBlurRadius<RetType> {
  fn setBlurRadius(self , rsthis: &mut QGraphicsBlurEffect) -> RetType;
}

  // proto:  void QGraphicsBlurEffect::setBlurRadius(qreal blurRadius);
impl<'a> /*trait*/ QGraphicsBlurEffect_setBlurRadius<()> for (f64) {
  fn setBlurRadius(self , rsthis: &mut QGraphicsBlurEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsBlurEffect13setBlurRadiusEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsBlurEffect13setBlurRadiusEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsBlurEffect::~QGraphicsBlurEffect();
impl /*struct*/ QGraphicsBlurEffect {
  pub fn FreeQGraphicsBlurEffect<RetType, T: QGraphicsBlurEffect_FreeQGraphicsBlurEffect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsBlurEffect(self);
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_FreeQGraphicsBlurEffect<RetType> {
  fn FreeQGraphicsBlurEffect(self , rsthis: &mut QGraphicsBlurEffect) -> RetType;
}

  // proto:  void QGraphicsBlurEffect::~QGraphicsBlurEffect();
impl<'a> /*trait*/ QGraphicsBlurEffect_FreeQGraphicsBlurEffect<()> for () {
  fn FreeQGraphicsBlurEffect(self , rsthis: &mut QGraphicsBlurEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsBlurEffectD0Ev()};
     unsafe {_ZN19QGraphicsBlurEffectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsBlurEffect::QGraphicsBlurEffect(const QGraphicsBlurEffect & );
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

  // proto:  void QGraphicsBlurEffect::QGraphicsBlurEffect(const QGraphicsBlurEffect & );
impl<'a> /*trait*/ QGraphicsBlurEffect_NewQGraphicsBlurEffect for (QGraphicsBlurEffect) {
  fn NewQGraphicsBlurEffect(self) -> QGraphicsBlurEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsBlurEffectC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QGraphicsBlurEffectC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsBlurEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsBlurEffect::metaObject();
impl /*struct*/ QGraphicsBlurEffect {
  pub fn metaObject<RetType, T: QGraphicsBlurEffect_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QGraphicsBlurEffect) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsBlurEffect::metaObject();
impl<'a> /*trait*/ QGraphicsBlurEffect_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QGraphicsBlurEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsBlurEffect10metaObjectEv()};
     unsafe {_ZNK19QGraphicsBlurEffect10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRectF QGraphicsBlurEffect::boundingRectFor(const QRectF & rect);
impl /*struct*/ QGraphicsBlurEffect {
  pub fn boundingRectFor<RetType, T: QGraphicsBlurEffect_boundingRectFor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.boundingRectFor(self);
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_boundingRectFor<RetType> {
  fn boundingRectFor(self , rsthis: &mut QGraphicsBlurEffect) -> RetType;
}

  // proto:  QRectF QGraphicsBlurEffect::boundingRectFor(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsBlurEffect_boundingRectFor<QRectF> for (QRectF) {
  fn boundingRectFor(self , rsthis: &mut QGraphicsBlurEffect) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsBlurEffect15boundingRectForERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QGraphicsBlurEffect15boundingRectForERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsBlurEffect::QGraphicsBlurEffect(QObject * parent);
impl<'a> /*trait*/ QGraphicsBlurEffect_NewQGraphicsBlurEffect for (QObject) {
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

  // proto:  void QGraphicsBlurEffect::blurRadiusChanged(qreal blurRadius);
impl /*struct*/ QGraphicsBlurEffect {
  pub fn blurRadiusChanged<RetType, T: QGraphicsBlurEffect_blurRadiusChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.blurRadiusChanged(self);
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_blurRadiusChanged<RetType> {
  fn blurRadiusChanged(self , rsthis: &mut QGraphicsBlurEffect) -> RetType;
}

  // proto:  void QGraphicsBlurEffect::blurRadiusChanged(qreal blurRadius);
impl<'a> /*trait*/ QGraphicsBlurEffect_blurRadiusChanged<()> for (f64) {
  fn blurRadiusChanged(self , rsthis: &mut QGraphicsBlurEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsBlurEffect17blurRadiusChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsBlurEffect17blurRadiusChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

