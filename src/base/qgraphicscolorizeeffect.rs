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
  // proto:  void QGraphicsColorizeEffect::setColor(const QColor & c);
  fn _ZN23QGraphicsColorizeEffect8setColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsColorizeEffect::strengthChanged(qreal strength);
  fn _ZN23QGraphicsColorizeEffect15strengthChangedEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsColorizeEffect::NewQGraphicsColorizeEffect(const QGraphicsColorizeEffect & );
  fn _ZN23QGraphicsColorizeEffectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsColorizeEffect::setStrength(qreal strength);
  fn _ZN23QGraphicsColorizeEffect11setStrengthEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsColorizeEffect::NewQGraphicsColorizeEffect(QObject * parent);
  fn _ZN23QGraphicsColorizeEffectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QGraphicsColorizeEffect::strength();
  fn _ZNK23QGraphicsColorizeEffect8strengthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsColorizeEffect::FreeQGraphicsColorizeEffect();
  fn _ZN23QGraphicsColorizeEffectD0Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsColorizeEffect::colorChanged(const QColor & color);
  fn _ZN23QGraphicsColorizeEffect12colorChangedERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QColor QGraphicsColorizeEffect::color();
  fn _ZNK23QGraphicsColorizeEffect5colorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QGraphicsColorizeEffect::metaObject();
  fn _ZNK23QGraphicsColorizeEffect10metaObjectEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QGraphicsColorizeEffect)=1
pub struct QGraphicsColorizeEffect {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsColorizeEffect {
  pub fn setColor<T: QGraphicsColorizeEffect_setColor>(&mut self, value: T)  {
     value.setColor(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_setColor {
  fn setColor(self, rsthis: &mut QGraphicsColorizeEffect) ;
}

// proto:  void QGraphicsColorizeEffect::setColor(const QColor & c);
impl<'a> /*trait*/ QGraphicsColorizeEffect_setColor for (&'a  QColor) {
  fn setColor(self, rsthis: &mut QGraphicsColorizeEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffect8setColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsColorizeEffect8setColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsColorizeEffect {
  pub fn strengthChanged<T: QGraphicsColorizeEffect_strengthChanged>(&mut self, value: T)  {
     value.strengthChanged(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_strengthChanged {
  fn strengthChanged(self, rsthis: &mut QGraphicsColorizeEffect) ;
}

// proto:  void QGraphicsColorizeEffect::strengthChanged(qreal strength);
impl<'a> /*trait*/ QGraphicsColorizeEffect_strengthChanged for (f64) {
  fn strengthChanged(self, rsthis: &mut QGraphicsColorizeEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffect15strengthChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN23QGraphicsColorizeEffect15strengthChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QGraphicsColorizeEffectC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsColorizeEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsColorizeEffect {
  pub fn setStrength<T: QGraphicsColorizeEffect_setStrength>(&mut self, value: T)  {
     value.setStrength(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_setStrength {
  fn setStrength(self, rsthis: &mut QGraphicsColorizeEffect) ;
}

// proto:  void QGraphicsColorizeEffect::setStrength(qreal strength);
impl<'a> /*trait*/ QGraphicsColorizeEffect_setStrength for (f64) {
  fn setStrength(self, rsthis: &mut QGraphicsColorizeEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffect11setStrengthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN23QGraphicsColorizeEffect11setStrengthEd(rsthis.qclsinst, arg0)};
    // return 1;
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
  pub fn strength<T: QGraphicsColorizeEffect_strength>(&mut self, value: T) -> f64 {
    return value.strength(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_strength {
  fn strength(self, rsthis: &mut QGraphicsColorizeEffect) -> f64;
}

// proto:  double QGraphicsColorizeEffect::strength();
impl<'a> /*trait*/ QGraphicsColorizeEffect_strength for () {
  fn strength(self, rsthis: &mut QGraphicsColorizeEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsColorizeEffect8strengthEv()};
    let mut ret = unsafe {_ZNK23QGraphicsColorizeEffect8strengthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsColorizeEffect {
  pub fn FreeQGraphicsColorizeEffect<T: QGraphicsColorizeEffect_FreeQGraphicsColorizeEffect>(&mut self, value: T)  {
     value.FreeQGraphicsColorizeEffect(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_FreeQGraphicsColorizeEffect {
  fn FreeQGraphicsColorizeEffect(self, rsthis: &mut QGraphicsColorizeEffect) ;
}

// proto:  void QGraphicsColorizeEffect::FreeQGraphicsColorizeEffect();
impl<'a> /*trait*/ QGraphicsColorizeEffect_FreeQGraphicsColorizeEffect for () {
  fn FreeQGraphicsColorizeEffect(self, rsthis: &mut QGraphicsColorizeEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffectD0Ev()};
     unsafe {_ZN23QGraphicsColorizeEffectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsColorizeEffect {
  pub fn colorChanged<T: QGraphicsColorizeEffect_colorChanged>(&mut self, value: T)  {
     value.colorChanged(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_colorChanged {
  fn colorChanged(self, rsthis: &mut QGraphicsColorizeEffect) ;
}

// proto:  void QGraphicsColorizeEffect::colorChanged(const QColor & color);
impl<'a> /*trait*/ QGraphicsColorizeEffect_colorChanged for (&'a  QColor) {
  fn colorChanged(self, rsthis: &mut QGraphicsColorizeEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffect12colorChangedERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsColorizeEffect12colorChangedERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsColorizeEffect {
  pub fn color<T: QGraphicsColorizeEffect_color>(&mut self, value: T) -> QColor {
    return value.color(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_color {
  fn color(self, rsthis: &mut QGraphicsColorizeEffect) -> QColor;
}

// proto:  QColor QGraphicsColorizeEffect::color();
impl<'a> /*trait*/ QGraphicsColorizeEffect_color for () {
  fn color(self, rsthis: &mut QGraphicsColorizeEffect) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsColorizeEffect5colorEv()};
    let mut ret = unsafe {_ZNK23QGraphicsColorizeEffect5colorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsColorizeEffect {
  pub fn metaObject<T: QGraphicsColorizeEffect_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_metaObject {
  fn metaObject(self, rsthis: &mut QGraphicsColorizeEffect) ;
}

// proto:  const QMetaObject * QGraphicsColorizeEffect::metaObject();
impl<'a> /*trait*/ QGraphicsColorizeEffect_metaObject for () {
  fn metaObject(self, rsthis: &mut QGraphicsColorizeEffect)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsColorizeEffect10metaObjectEv()};
     unsafe {_ZNK23QGraphicsColorizeEffect10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

