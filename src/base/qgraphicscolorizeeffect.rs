// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qcolor::QColor;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGraphicsColorizeEffect::setColor(const QColor & c);
  fn _ZN23QGraphicsColorizeEffect8setColorERK6QColor(arg0: *const c_void) -> i32;
  // proto: void QGraphicsColorizeEffect::strengthChanged(qreal strength);
  fn _ZN23QGraphicsColorizeEffect15strengthChangedEd(arg0: c_double) -> i32;
  // proto: void QGraphicsColorizeEffect::NewQGraphicsColorizeEffect(const QGraphicsColorizeEffect & );
  fn _ZN23QGraphicsColorizeEffectC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QGraphicsColorizeEffect::setStrength(qreal strength);
  fn _ZN23QGraphicsColorizeEffect11setStrengthEd(arg0: c_double) -> i32;
  // proto: void QGraphicsColorizeEffect::NewQGraphicsColorizeEffect(QObject * parent);
  fn _ZN23QGraphicsColorizeEffectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: double QGraphicsColorizeEffect::strength();
  fn _ZNK23QGraphicsColorizeEffect8strengthEv() -> i32;
  // proto: void QGraphicsColorizeEffect::FreeQGraphicsColorizeEffect();
  fn _ZN23QGraphicsColorizeEffectD0Ev() -> i32;
  // proto: void QGraphicsColorizeEffect::colorChanged(const QColor & color);
  fn _ZN23QGraphicsColorizeEffect12colorChangedERK6QColor(arg0: *const c_void) -> i32;
  // proto: QColor QGraphicsColorizeEffect::color();
  fn _ZNK23QGraphicsColorizeEffect5colorEv() -> i32;
  // proto: const QMetaObject * QGraphicsColorizeEffect::metaObject();
  fn _ZNK23QGraphicsColorizeEffect10metaObjectEv() -> i32;
}

// body block begin
// class sizeof(QGraphicsColorizeEffect)=1
pub struct QGraphicsColorizeEffect {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsColorizeEffect {
  pub fn setColor<T: QGraphicsColorizeEffect_setColor>(&mut self, value: T) -> i32 {
    value.setColor(self);
    return 1;
  }
}

pub trait QGraphicsColorizeEffect_setColor {
  fn setColor(self, this: &mut QGraphicsColorizeEffect) -> i32;
}

// proto: void QGraphicsColorizeEffect::setColor(const QColor & c);
impl<'a> /*trait*/ QGraphicsColorizeEffect_setColor for (&'a  QColor) {
  fn setColor(self, this: &mut QGraphicsColorizeEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffect8setColorERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN23QGraphicsColorizeEffect8setColorERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsColorizeEffect {
  pub fn strengthChanged<T: QGraphicsColorizeEffect_strengthChanged>(&mut self, value: T) -> i32 {
    value.strengthChanged(self);
    return 1;
  }
}

pub trait QGraphicsColorizeEffect_strengthChanged {
  fn strengthChanged(self, this: &mut QGraphicsColorizeEffect) -> i32;
}

// proto: void QGraphicsColorizeEffect::strengthChanged(qreal strength);
impl<'a> /*trait*/ QGraphicsColorizeEffect_strengthChanged for (f64) {
  fn strengthChanged(self, this: &mut QGraphicsColorizeEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffect15strengthChangedEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN23QGraphicsColorizeEffect15strengthChangedEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsColorizeEffect {
  pub fn NewQGraphicsColorizeEffect<T: QGraphicsColorizeEffect_NewQGraphicsColorizeEffect>(value: T) -> QGraphicsColorizeEffect {
    let rsthis = value.NewQGraphicsColorizeEffect();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_NewQGraphicsColorizeEffect {
  fn NewQGraphicsColorizeEffect(self) -> QGraphicsColorizeEffect;
}

// proto: void QGraphicsColorizeEffect::NewQGraphicsColorizeEffect(const QGraphicsColorizeEffect & );
impl<'a> /*trait*/ QGraphicsColorizeEffect_NewQGraphicsColorizeEffect for (&'a  QGraphicsColorizeEffect) {
  fn NewQGraphicsColorizeEffect(self) -> QGraphicsColorizeEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffectC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN23QGraphicsColorizeEffectC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsColorizeEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsColorizeEffect {
  pub fn setStrength<T: QGraphicsColorizeEffect_setStrength>(&mut self, value: T) -> i32 {
    value.setStrength(self);
    return 1;
  }
}

pub trait QGraphicsColorizeEffect_setStrength {
  fn setStrength(self, this: &mut QGraphicsColorizeEffect) -> i32;
}

// proto: void QGraphicsColorizeEffect::setStrength(qreal strength);
impl<'a> /*trait*/ QGraphicsColorizeEffect_setStrength for (f64) {
  fn setStrength(self, this: &mut QGraphicsColorizeEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffect11setStrengthEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN23QGraphicsColorizeEffect11setStrengthEd(arg0)};
    return 1;
  }
}

// proto: void QGraphicsColorizeEffect::NewQGraphicsColorizeEffect(QObject * parent);
impl<'a> /*trait*/ QGraphicsColorizeEffect_NewQGraphicsColorizeEffect for (&'a mut QObject) {
  fn NewQGraphicsColorizeEffect(self) -> QGraphicsColorizeEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffectC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QGraphicsColorizeEffectC1EP7QObject(qthis, arg0)};
    let rsthis = QGraphicsColorizeEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsColorizeEffect {
  pub fn strength<T: QGraphicsColorizeEffect_strength>(&mut self, value: T) -> i32 {
    value.strength(self);
    return 1;
  }
}

pub trait QGraphicsColorizeEffect_strength {
  fn strength(self, this: &mut QGraphicsColorizeEffect) -> i32;
}

// proto: double QGraphicsColorizeEffect::strength();
impl<'a> /*trait*/ QGraphicsColorizeEffect_strength for () {
  fn strength(self, this: &mut QGraphicsColorizeEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsColorizeEffect8strengthEv()};
    unsafe {_ZNK23QGraphicsColorizeEffect8strengthEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsColorizeEffect {
  pub fn FreeQGraphicsColorizeEffect<T: QGraphicsColorizeEffect_FreeQGraphicsColorizeEffect>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsColorizeEffect(self);
    return 1;
  }
}

pub trait QGraphicsColorizeEffect_FreeQGraphicsColorizeEffect {
  fn FreeQGraphicsColorizeEffect(self, this: &mut QGraphicsColorizeEffect) -> i32;
}

// proto: void QGraphicsColorizeEffect::FreeQGraphicsColorizeEffect();
impl<'a> /*trait*/ QGraphicsColorizeEffect_FreeQGraphicsColorizeEffect for () {
  fn FreeQGraphicsColorizeEffect(self, this: &mut QGraphicsColorizeEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffectD0Ev()};
    unsafe {_ZN23QGraphicsColorizeEffectD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsColorizeEffect {
  pub fn colorChanged<T: QGraphicsColorizeEffect_colorChanged>(&mut self, value: T) -> i32 {
    value.colorChanged(self);
    return 1;
  }
}

pub trait QGraphicsColorizeEffect_colorChanged {
  fn colorChanged(self, this: &mut QGraphicsColorizeEffect) -> i32;
}

// proto: void QGraphicsColorizeEffect::colorChanged(const QColor & color);
impl<'a> /*trait*/ QGraphicsColorizeEffect_colorChanged for (&'a  QColor) {
  fn colorChanged(self, this: &mut QGraphicsColorizeEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffect12colorChangedERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN23QGraphicsColorizeEffect12colorChangedERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsColorizeEffect {
  pub fn color<T: QGraphicsColorizeEffect_color>(&mut self, value: T) -> i32 {
    value.color(self);
    return 1;
  }
}

pub trait QGraphicsColorizeEffect_color {
  fn color(self, this: &mut QGraphicsColorizeEffect) -> i32;
}

// proto: QColor QGraphicsColorizeEffect::color();
impl<'a> /*trait*/ QGraphicsColorizeEffect_color for () {
  fn color(self, this: &mut QGraphicsColorizeEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsColorizeEffect5colorEv()};
    unsafe {_ZNK23QGraphicsColorizeEffect5colorEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsColorizeEffect {
  pub fn metaObject<T: QGraphicsColorizeEffect_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QGraphicsColorizeEffect_metaObject {
  fn metaObject(self, this: &mut QGraphicsColorizeEffect) -> i32;
}

// proto: const QMetaObject * QGraphicsColorizeEffect::metaObject();
impl<'a> /*trait*/ QGraphicsColorizeEffect_metaObject for () {
  fn metaObject(self, this: &mut QGraphicsColorizeEffect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsColorizeEffect10metaObjectEv()};
    unsafe {_ZNK23QGraphicsColorizeEffect10metaObjectEv()};
    return 1;
  }
}

