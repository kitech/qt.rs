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
  // proto: void QGraphicsOpacityEffect::NewQGraphicsOpacityEffect(QObject * parent);
  fn _ZN22QGraphicsOpacityEffectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QGraphicsOpacityEffect::FreeQGraphicsOpacityEffect();
  fn _ZN22QGraphicsOpacityEffectD0Ev() -> i32;
  // proto: void QGraphicsOpacityEffect::setOpacityMask(const QBrush & mask);
  fn _ZN22QGraphicsOpacityEffect14setOpacityMaskERK6QBrush(arg0: *const c_void) -> i32;
  // proto: void QGraphicsOpacityEffect::NewQGraphicsOpacityEffect(const QGraphicsOpacityEffect & );
  fn _ZN22QGraphicsOpacityEffectC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QBrush QGraphicsOpacityEffect::opacityMask();
  fn _ZNK22QGraphicsOpacityEffect11opacityMaskEv() -> i32;
  // proto: void QGraphicsOpacityEffect::opacityChanged(qreal opacity);
  fn _ZN22QGraphicsOpacityEffect14opacityChangedEd(arg0: c_double) -> i32;
  // proto: void QGraphicsOpacityEffect::opacityMaskChanged(const QBrush & mask);
  fn _ZN22QGraphicsOpacityEffect18opacityMaskChangedERK6QBrush(arg0: *const c_void) -> i32;
  // proto: const QMetaObject * QGraphicsOpacityEffect::metaObject();
  fn _ZNK22QGraphicsOpacityEffect10metaObjectEv() -> i32;
  // proto: double QGraphicsOpacityEffect::opacity();
  fn _ZNK22QGraphicsOpacityEffect7opacityEv() -> i32;
  // proto: void QGraphicsOpacityEffect::setOpacity(qreal opacity);
  fn _ZN22QGraphicsOpacityEffect10setOpacityEd(arg0: c_double) -> i32;
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
  pub fn FreeQGraphicsOpacityEffect<T: QGraphicsOpacityEffect_FreeQGraphicsOpacityEffect>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsOpacityEffect(self);
    return 1;
  }
}

pub trait QGraphicsOpacityEffect_FreeQGraphicsOpacityEffect {
  fn FreeQGraphicsOpacityEffect(self, this: &mut QGraphicsOpacityEffect) -> i32;
}

// proto: void QGraphicsOpacityEffect::FreeQGraphicsOpacityEffect();
impl<'a> /*trait*/ QGraphicsOpacityEffect_FreeQGraphicsOpacityEffect for () {
  fn FreeQGraphicsOpacityEffect(self, this: &mut QGraphicsOpacityEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffectD0Ev()};
    unsafe {_ZN22QGraphicsOpacityEffectD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsOpacityEffect {
  pub fn setOpacityMask<T: QGraphicsOpacityEffect_setOpacityMask>(&mut self, value: T) -> i32 {
    value.setOpacityMask(self);
    return 1;
  }
}

pub trait QGraphicsOpacityEffect_setOpacityMask {
  fn setOpacityMask(self, this: &mut QGraphicsOpacityEffect) -> i32;
}

// proto: void QGraphicsOpacityEffect::setOpacityMask(const QBrush & mask);
impl<'a> /*trait*/ QGraphicsOpacityEffect_setOpacityMask for (&'a  QBrush) {
  fn setOpacityMask(self, this: &mut QGraphicsOpacityEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffect14setOpacityMaskERK6QBrush()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN22QGraphicsOpacityEffect14setOpacityMaskERK6QBrush(arg0)};
    return 1;
  }
}

// proto: void QGraphicsOpacityEffect::NewQGraphicsOpacityEffect(const QGraphicsOpacityEffect & );
impl<'a> /*trait*/ QGraphicsOpacityEffect_NewQGraphicsOpacityEffect for (&'a  QGraphicsOpacityEffect) {
  fn NewQGraphicsOpacityEffect(self) -> QGraphicsOpacityEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffectC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN22QGraphicsOpacityEffectC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsOpacityEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacityMask<T: QGraphicsOpacityEffect_opacityMask>(&mut self, value: T) -> i32 {
    value.opacityMask(self);
    return 1;
  }
}

pub trait QGraphicsOpacityEffect_opacityMask {
  fn opacityMask(self, this: &mut QGraphicsOpacityEffect) -> i32;
}

// proto: QBrush QGraphicsOpacityEffect::opacityMask();
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacityMask for () {
  fn opacityMask(self, this: &mut QGraphicsOpacityEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsOpacityEffect11opacityMaskEv()};
    unsafe {_ZNK22QGraphicsOpacityEffect11opacityMaskEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacityChanged<T: QGraphicsOpacityEffect_opacityChanged>(&mut self, value: T) -> i32 {
    value.opacityChanged(self);
    return 1;
  }
}

pub trait QGraphicsOpacityEffect_opacityChanged {
  fn opacityChanged(self, this: &mut QGraphicsOpacityEffect) -> i32;
}

// proto: void QGraphicsOpacityEffect::opacityChanged(qreal opacity);
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacityChanged for (f64) {
  fn opacityChanged(self, this: &mut QGraphicsOpacityEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffect14opacityChangedEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN22QGraphicsOpacityEffect14opacityChangedEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacityMaskChanged<T: QGraphicsOpacityEffect_opacityMaskChanged>(&mut self, value: T) -> i32 {
    value.opacityMaskChanged(self);
    return 1;
  }
}

pub trait QGraphicsOpacityEffect_opacityMaskChanged {
  fn opacityMaskChanged(self, this: &mut QGraphicsOpacityEffect) -> i32;
}

// proto: void QGraphicsOpacityEffect::opacityMaskChanged(const QBrush & mask);
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacityMaskChanged for (&'a  QBrush) {
  fn opacityMaskChanged(self, this: &mut QGraphicsOpacityEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffect18opacityMaskChangedERK6QBrush()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN22QGraphicsOpacityEffect18opacityMaskChangedERK6QBrush(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsOpacityEffect {
  pub fn metaObject<T: QGraphicsOpacityEffect_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QGraphicsOpacityEffect_metaObject {
  fn metaObject(self, this: &mut QGraphicsOpacityEffect) -> i32;
}

// proto: const QMetaObject * QGraphicsOpacityEffect::metaObject();
impl<'a> /*trait*/ QGraphicsOpacityEffect_metaObject for () {
  fn metaObject(self, this: &mut QGraphicsOpacityEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsOpacityEffect10metaObjectEv()};
    unsafe {_ZNK22QGraphicsOpacityEffect10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacity<T: QGraphicsOpacityEffect_opacity>(&mut self, value: T) -> i32 {
    value.opacity(self);
    return 1;
  }
}

pub trait QGraphicsOpacityEffect_opacity {
  fn opacity(self, this: &mut QGraphicsOpacityEffect) -> i32;
}

// proto: double QGraphicsOpacityEffect::opacity();
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacity for () {
  fn opacity(self, this: &mut QGraphicsOpacityEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsOpacityEffect7opacityEv()};
    unsafe {_ZNK22QGraphicsOpacityEffect7opacityEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsOpacityEffect {
  pub fn setOpacity<T: QGraphicsOpacityEffect_setOpacity>(&mut self, value: T) -> i32 {
    value.setOpacity(self);
    return 1;
  }
}

pub trait QGraphicsOpacityEffect_setOpacity {
  fn setOpacity(self, this: &mut QGraphicsOpacityEffect) -> i32;
}

// proto: void QGraphicsOpacityEffect::setOpacity(qreal opacity);
impl<'a> /*trait*/ QGraphicsOpacityEffect_setOpacity for (f64) {
  fn setOpacity(self, this: &mut QGraphicsOpacityEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffect10setOpacityEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN22QGraphicsOpacityEffect10setOpacityEd(arg0)};
    return 1;
  }
}

