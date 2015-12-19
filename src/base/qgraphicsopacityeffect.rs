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

// proto:  void QGraphicsOpacityEffect::FreeQGraphicsOpacityEffect();
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn FreeQGraphicsOpacityEffect<RetType, T: QGraphicsOpacityEffect_FreeQGraphicsOpacityEffect<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsOpacityEffect(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_FreeQGraphicsOpacityEffect<RetType> {
  fn FreeQGraphicsOpacityEffect(self , rsthis: &mut QGraphicsOpacityEffect) -> RetType;
}

// proto:  void QGraphicsOpacityEffect::FreeQGraphicsOpacityEffect();
impl<'a> /*trait*/ QGraphicsOpacityEffect_FreeQGraphicsOpacityEffect<()> for () {
  fn FreeQGraphicsOpacityEffect(self , rsthis: &mut QGraphicsOpacityEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffectD0Ev()};
     unsafe {_ZN22QGraphicsOpacityEffectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QGraphicsOpacityEffect::setOpacityMask(const QBrush & mask);
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn setOpacityMask<RetType, T: QGraphicsOpacityEffect_setOpacityMask<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setOpacityMask(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_setOpacityMask<RetType> {
  fn setOpacityMask(self , rsthis: &mut QGraphicsOpacityEffect) -> RetType;
}

// proto:  void QGraphicsOpacityEffect::setOpacityMask(const QBrush & mask);
impl<'a> /*trait*/ QGraphicsOpacityEffect_setOpacityMask<()> for (&'a  QBrush) {
  fn setOpacityMask(self , rsthis: &mut QGraphicsOpacityEffect) -> () {
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

// proto:  QBrush QGraphicsOpacityEffect::opacityMask();
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacityMask<RetType, T: QGraphicsOpacityEffect_opacityMask<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.opacityMask(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_opacityMask<RetType> {
  fn opacityMask(self , rsthis: &mut QGraphicsOpacityEffect) -> RetType;
}

// proto:  QBrush QGraphicsOpacityEffect::opacityMask();
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacityMask<QBrush> for () {
  fn opacityMask(self , rsthis: &mut QGraphicsOpacityEffect) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsOpacityEffect11opacityMaskEv()};
    let mut ret = unsafe {_ZNK22QGraphicsOpacityEffect11opacityMaskEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsOpacityEffect::opacityChanged(qreal opacity);
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacityChanged<RetType, T: QGraphicsOpacityEffect_opacityChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.opacityChanged(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_opacityChanged<RetType> {
  fn opacityChanged(self , rsthis: &mut QGraphicsOpacityEffect) -> RetType;
}

// proto:  void QGraphicsOpacityEffect::opacityChanged(qreal opacity);
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacityChanged<()> for (f64) {
  fn opacityChanged(self , rsthis: &mut QGraphicsOpacityEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffect14opacityChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN22QGraphicsOpacityEffect14opacityChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QGraphicsOpacityEffect::opacityMaskChanged(const QBrush & mask);
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacityMaskChanged<RetType, T: QGraphicsOpacityEffect_opacityMaskChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.opacityMaskChanged(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_opacityMaskChanged<RetType> {
  fn opacityMaskChanged(self , rsthis: &mut QGraphicsOpacityEffect) -> RetType;
}

// proto:  void QGraphicsOpacityEffect::opacityMaskChanged(const QBrush & mask);
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacityMaskChanged<()> for (&'a  QBrush) {
  fn opacityMaskChanged(self , rsthis: &mut QGraphicsOpacityEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffect18opacityMaskChangedERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN22QGraphicsOpacityEffect18opacityMaskChangedERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  const QMetaObject * QGraphicsOpacityEffect::metaObject();
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn metaObject<RetType, T: QGraphicsOpacityEffect_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QGraphicsOpacityEffect) -> RetType;
}

// proto:  const QMetaObject * QGraphicsOpacityEffect::metaObject();
impl<'a> /*trait*/ QGraphicsOpacityEffect_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QGraphicsOpacityEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsOpacityEffect10metaObjectEv()};
     unsafe {_ZNK22QGraphicsOpacityEffect10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  double QGraphicsOpacityEffect::opacity();
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacity<RetType, T: QGraphicsOpacityEffect_opacity<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.opacity(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_opacity<RetType> {
  fn opacity(self , rsthis: &mut QGraphicsOpacityEffect) -> RetType;
}

// proto:  double QGraphicsOpacityEffect::opacity();
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacity<f64> for () {
  fn opacity(self , rsthis: &mut QGraphicsOpacityEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsOpacityEffect7opacityEv()};
    let mut ret = unsafe {_ZNK22QGraphicsOpacityEffect7opacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  void QGraphicsOpacityEffect::setOpacity(qreal opacity);
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn setOpacity<RetType, T: QGraphicsOpacityEffect_setOpacity<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setOpacity(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_setOpacity<RetType> {
  fn setOpacity(self , rsthis: &mut QGraphicsOpacityEffect) -> RetType;
}

// proto:  void QGraphicsOpacityEffect::setOpacity(qreal opacity);
impl<'a> /*trait*/ QGraphicsOpacityEffect_setOpacity<()> for (f64) {
  fn setOpacity(self , rsthis: &mut QGraphicsOpacityEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffect10setOpacityEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN22QGraphicsOpacityEffect10setOpacityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

