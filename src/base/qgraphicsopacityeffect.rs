// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qbrush::QBrush;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGraphicsOpacityEffect::NewQGraphicsOpacityEffect(QObject * parent);
  fn _ZN22QGraphicsOpacityEffectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsOpacityEffect::FreeQGraphicsOpacityEffect();
  fn _ZN22QGraphicsOpacityEffectD0Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsOpacityEffect::setOpacityMask(const QBrush & mask);
  fn _ZN22QGraphicsOpacityEffect14setOpacityMaskERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsOpacityEffect::NewQGraphicsOpacityEffect(const QGraphicsOpacityEffect & );
  fn _ZN22QGraphicsOpacityEffectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QBrush QGraphicsOpacityEffect::opacityMask();
  fn _ZNK22QGraphicsOpacityEffect11opacityMaskEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsOpacityEffect::opacityChanged(qreal opacity);
  fn _ZN22QGraphicsOpacityEffect14opacityChangedEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsOpacityEffect::opacityMaskChanged(const QBrush & mask);
  fn _ZN22QGraphicsOpacityEffect18opacityMaskChangedERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QGraphicsOpacityEffect::metaObject();
  fn _ZNK22QGraphicsOpacityEffect10metaObjectEv(qthis: *mut c_void) ;
  // proto:  double QGraphicsOpacityEffect::opacity();
  fn _ZNK22QGraphicsOpacityEffect7opacityEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsOpacityEffect::setOpacity(qreal opacity);
  fn _ZN22QGraphicsOpacityEffect10setOpacityEd(qthis: *mut c_void, arg0: c_double) ;
}

// body block begin
// class sizeof(QGraphicsOpacityEffect)=1
pub struct QGraphicsOpacityEffect {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsOpacityEffect {
  pub fn NewQGraphicsOpacityEffect<T: QGraphicsOpacityEffect_NewQGraphicsOpacityEffect>(value: T) -> QGraphicsOpacityEffect {
    let rsthis = value.NewQGraphicsOpacityEffect();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_NewQGraphicsOpacityEffect {
  fn NewQGraphicsOpacityEffect(self) -> QGraphicsOpacityEffect;
}

// proto: void QGraphicsOpacityEffect::NewQGraphicsOpacityEffect(QObject * parent);
impl<'a> /*trait*/ QGraphicsOpacityEffect_NewQGraphicsOpacityEffect for (&'a mut QObject) {
  fn NewQGraphicsOpacityEffect(self) -> QGraphicsOpacityEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffectC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QGraphicsOpacityEffectC1EP7QObject(qthis, arg0)};
    let rsthis = QGraphicsOpacityEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsOpacityEffect {
  pub fn FreeQGraphicsOpacityEffect<T: QGraphicsOpacityEffect_FreeQGraphicsOpacityEffect>(&mut self, value: T)  {
     value.FreeQGraphicsOpacityEffect(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_FreeQGraphicsOpacityEffect {
  fn FreeQGraphicsOpacityEffect(self, rsthis: &mut QGraphicsOpacityEffect) ;
}

// proto:  void QGraphicsOpacityEffect::FreeQGraphicsOpacityEffect();
impl<'a> /*trait*/ QGraphicsOpacityEffect_FreeQGraphicsOpacityEffect for () {
  fn FreeQGraphicsOpacityEffect(self, rsthis: &mut QGraphicsOpacityEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffectD0Ev()};
     unsafe {_ZN22QGraphicsOpacityEffectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsOpacityEffect {
  pub fn setOpacityMask<T: QGraphicsOpacityEffect_setOpacityMask>(&mut self, value: T)  {
     value.setOpacityMask(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_setOpacityMask {
  fn setOpacityMask(self, rsthis: &mut QGraphicsOpacityEffect) ;
}

// proto:  void QGraphicsOpacityEffect::setOpacityMask(const QBrush & mask);
impl<'a> /*trait*/ QGraphicsOpacityEffect_setOpacityMask for (&'a  QBrush) {
  fn setOpacityMask(self, rsthis: &mut QGraphicsOpacityEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffect14setOpacityMaskERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN22QGraphicsOpacityEffect14setOpacityMaskERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QGraphicsOpacityEffect::NewQGraphicsOpacityEffect(const QGraphicsOpacityEffect & );
impl<'a> /*trait*/ QGraphicsOpacityEffect_NewQGraphicsOpacityEffect for (&'a  QGraphicsOpacityEffect) {
  fn NewQGraphicsOpacityEffect(self) -> QGraphicsOpacityEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffectC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QGraphicsOpacityEffectC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsOpacityEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacityMask<T: QGraphicsOpacityEffect_opacityMask>(&mut self, value: T) -> QBrush {
    return value.opacityMask(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_opacityMask {
  fn opacityMask(self, rsthis: &mut QGraphicsOpacityEffect) -> QBrush;
}

// proto:  QBrush QGraphicsOpacityEffect::opacityMask();
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacityMask for () {
  fn opacityMask(self, rsthis: &mut QGraphicsOpacityEffect) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsOpacityEffect11opacityMaskEv()};
    let mut ret = unsafe {_ZNK22QGraphicsOpacityEffect11opacityMaskEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacityChanged<T: QGraphicsOpacityEffect_opacityChanged>(&mut self, value: T)  {
     value.opacityChanged(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_opacityChanged {
  fn opacityChanged(self, rsthis: &mut QGraphicsOpacityEffect) ;
}

// proto:  void QGraphicsOpacityEffect::opacityChanged(qreal opacity);
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacityChanged for (f64) {
  fn opacityChanged(self, rsthis: &mut QGraphicsOpacityEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffect14opacityChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN22QGraphicsOpacityEffect14opacityChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacityMaskChanged<T: QGraphicsOpacityEffect_opacityMaskChanged>(&mut self, value: T)  {
     value.opacityMaskChanged(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_opacityMaskChanged {
  fn opacityMaskChanged(self, rsthis: &mut QGraphicsOpacityEffect) ;
}

// proto:  void QGraphicsOpacityEffect::opacityMaskChanged(const QBrush & mask);
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacityMaskChanged for (&'a  QBrush) {
  fn opacityMaskChanged(self, rsthis: &mut QGraphicsOpacityEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffect18opacityMaskChangedERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN22QGraphicsOpacityEffect18opacityMaskChangedERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsOpacityEffect {
  pub fn metaObject<T: QGraphicsOpacityEffect_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_metaObject {
  fn metaObject(self, rsthis: &mut QGraphicsOpacityEffect) ;
}

// proto:  const QMetaObject * QGraphicsOpacityEffect::metaObject();
impl<'a> /*trait*/ QGraphicsOpacityEffect_metaObject for () {
  fn metaObject(self, rsthis: &mut QGraphicsOpacityEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsOpacityEffect10metaObjectEv()};
     unsafe {_ZNK22QGraphicsOpacityEffect10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacity<T: QGraphicsOpacityEffect_opacity>(&mut self, value: T) -> f64 {
    return value.opacity(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_opacity {
  fn opacity(self, rsthis: &mut QGraphicsOpacityEffect) -> f64;
}

// proto:  double QGraphicsOpacityEffect::opacity();
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacity for () {
  fn opacity(self, rsthis: &mut QGraphicsOpacityEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsOpacityEffect7opacityEv()};
    let mut ret = unsafe {_ZNK22QGraphicsOpacityEffect7opacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsOpacityEffect {
  pub fn setOpacity<T: QGraphicsOpacityEffect_setOpacity>(&mut self, value: T)  {
     value.setOpacity(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_setOpacity {
  fn setOpacity(self, rsthis: &mut QGraphicsOpacityEffect) ;
}

// proto:  void QGraphicsOpacityEffect::setOpacity(qreal opacity);
impl<'a> /*trait*/ QGraphicsOpacityEffect_setOpacity for (f64) {
  fn setOpacity(self, rsthis: &mut QGraphicsOpacityEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffect10setOpacityEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN22QGraphicsOpacityEffect10setOpacityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

