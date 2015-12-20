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
  fn _ZN23QGraphicsColorizeEffect8setColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsColorizeEffect::strengthChanged(qreal strength);
  fn _ZN23QGraphicsColorizeEffect15strengthChangedEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsColorizeEffect::QGraphicsColorizeEffect(const QGraphicsColorizeEffect & );
  fn _ZN23QGraphicsColorizeEffectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsColorizeEffect::setStrength(qreal strength);
  fn _ZN23QGraphicsColorizeEffect11setStrengthEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsColorizeEffect::QGraphicsColorizeEffect(QObject * parent);
  fn _ZN23QGraphicsColorizeEffectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QGraphicsColorizeEffect::strength();
  fn _ZNK23QGraphicsColorizeEffect8strengthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsColorizeEffect::~QGraphicsColorizeEffect();
  fn _ZN23QGraphicsColorizeEffectD0Ev(qthis: *mut c_void);
  // proto:  void QGraphicsColorizeEffect::colorChanged(const QColor & color);
  fn _ZN23QGraphicsColorizeEffect12colorChangedERK6QColor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QColor QGraphicsColorizeEffect::color();
  fn _ZNK23QGraphicsColorizeEffect5colorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QGraphicsColorizeEffect::metaObject();
  fn _ZNK23QGraphicsColorizeEffect10metaObjectEv(qthis: *mut c_void);
}

// body block begin
// class sizeof(QGraphicsColorizeEffect)=1
pub struct QGraphicsColorizeEffect {
  pub qclsinst: *mut c_void,
}

  // proto:  void QGraphicsColorizeEffect::setColor(const QColor & c);
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn setColor<RetType, T: QGraphicsColorizeEffect_setColor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setColor(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_setColor<RetType> {
  fn setColor(self , rsthis: &mut QGraphicsColorizeEffect) -> RetType;
}

  // proto:  void QGraphicsColorizeEffect::setColor(const QColor & c);
impl<'a> /*trait*/ QGraphicsColorizeEffect_setColor<()> for (QColor) {
  fn setColor(self , rsthis: &mut QGraphicsColorizeEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffect8setColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsColorizeEffect8setColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsColorizeEffect::strengthChanged(qreal strength);
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn strengthChanged<RetType, T: QGraphicsColorizeEffect_strengthChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.strengthChanged(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_strengthChanged<RetType> {
  fn strengthChanged(self , rsthis: &mut QGraphicsColorizeEffect) -> RetType;
}

  // proto:  void QGraphicsColorizeEffect::strengthChanged(qreal strength);
impl<'a> /*trait*/ QGraphicsColorizeEffect_strengthChanged<()> for (f64) {
  fn strengthChanged(self , rsthis: &mut QGraphicsColorizeEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffect15strengthChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN23QGraphicsColorizeEffect15strengthChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsColorizeEffect::QGraphicsColorizeEffect(const QGraphicsColorizeEffect & );
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

  // proto:  void QGraphicsColorizeEffect::QGraphicsColorizeEffect(const QGraphicsColorizeEffect & );
impl<'a> /*trait*/ QGraphicsColorizeEffect_NewQGraphicsColorizeEffect for (QGraphicsColorizeEffect) {
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

  // proto:  void QGraphicsColorizeEffect::setStrength(qreal strength);
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn setStrength<RetType, T: QGraphicsColorizeEffect_setStrength<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setStrength(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_setStrength<RetType> {
  fn setStrength(self , rsthis: &mut QGraphicsColorizeEffect) -> RetType;
}

  // proto:  void QGraphicsColorizeEffect::setStrength(qreal strength);
impl<'a> /*trait*/ QGraphicsColorizeEffect_setStrength<()> for (f64) {
  fn setStrength(self , rsthis: &mut QGraphicsColorizeEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffect11setStrengthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN23QGraphicsColorizeEffect11setStrengthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsColorizeEffect::QGraphicsColorizeEffect(QObject * parent);
impl<'a> /*trait*/ QGraphicsColorizeEffect_NewQGraphicsColorizeEffect for (QObject) {
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

  // proto:  qreal QGraphicsColorizeEffect::strength();
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn strength<RetType, T: QGraphicsColorizeEffect_strength<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.strength(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_strength<RetType> {
  fn strength(self , rsthis: &mut QGraphicsColorizeEffect) -> RetType;
}

  // proto:  qreal QGraphicsColorizeEffect::strength();
impl<'a> /*trait*/ QGraphicsColorizeEffect_strength<f64> for () {
  fn strength(self , rsthis: &mut QGraphicsColorizeEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsColorizeEffect8strengthEv()};
    let mut ret = unsafe {_ZNK23QGraphicsColorizeEffect8strengthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsColorizeEffect::~QGraphicsColorizeEffect();
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn FreeQGraphicsColorizeEffect<RetType, T: QGraphicsColorizeEffect_FreeQGraphicsColorizeEffect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsColorizeEffect(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_FreeQGraphicsColorizeEffect<RetType> {
  fn FreeQGraphicsColorizeEffect(self , rsthis: &mut QGraphicsColorizeEffect) -> RetType;
}

  // proto:  void QGraphicsColorizeEffect::~QGraphicsColorizeEffect();
impl<'a> /*trait*/ QGraphicsColorizeEffect_FreeQGraphicsColorizeEffect<()> for () {
  fn FreeQGraphicsColorizeEffect(self , rsthis: &mut QGraphicsColorizeEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffectD0Ev()};
     unsafe {_ZN23QGraphicsColorizeEffectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsColorizeEffect::colorChanged(const QColor & color);
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn colorChanged<RetType, T: QGraphicsColorizeEffect_colorChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.colorChanged(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_colorChanged<RetType> {
  fn colorChanged(self , rsthis: &mut QGraphicsColorizeEffect) -> RetType;
}

  // proto:  void QGraphicsColorizeEffect::colorChanged(const QColor & color);
impl<'a> /*trait*/ QGraphicsColorizeEffect_colorChanged<()> for (QColor) {
  fn colorChanged(self , rsthis: &mut QGraphicsColorizeEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffect12colorChangedERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsColorizeEffect12colorChangedERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QColor QGraphicsColorizeEffect::color();
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn color<RetType, T: QGraphicsColorizeEffect_color<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.color(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_color<RetType> {
  fn color(self , rsthis: &mut QGraphicsColorizeEffect) -> RetType;
}

  // proto:  QColor QGraphicsColorizeEffect::color();
impl<'a> /*trait*/ QGraphicsColorizeEffect_color<QColor> for () {
  fn color(self , rsthis: &mut QGraphicsColorizeEffect) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsColorizeEffect5colorEv()};
    let mut ret = unsafe {_ZNK23QGraphicsColorizeEffect5colorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsColorizeEffect::metaObject();
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn metaObject<RetType, T: QGraphicsColorizeEffect_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QGraphicsColorizeEffect) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsColorizeEffect::metaObject();
impl<'a> /*trait*/ QGraphicsColorizeEffect_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QGraphicsColorizeEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsColorizeEffect10metaObjectEv()};
     unsafe {_ZNK23QGraphicsColorizeEffect10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

