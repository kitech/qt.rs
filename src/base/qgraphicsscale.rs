// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmatrix4x4::QMatrix4x4;
use super::qvector3d::QVector3D;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGraphicsScale::applyTo(QMatrix4x4 * matrix);
  fn _ZNK14QGraphicsScale7applyToEP10QMatrix4x4(arg0: *mut c_void) -> i32;
  // proto: double QGraphicsScale::zScale();
  fn _ZNK14QGraphicsScale6zScaleEv() -> i32;
  // proto: void QGraphicsScale::yScaleChanged();
  fn _ZN14QGraphicsScale13yScaleChangedEv() -> i32;
  // proto: void QGraphicsScale::originChanged();
  fn _ZN14QGraphicsScale13originChangedEv() -> i32;
  // proto: double QGraphicsScale::xScale();
  fn _ZNK14QGraphicsScale6xScaleEv() -> i32;
  // proto: double QGraphicsScale::yScale();
  fn _ZNK14QGraphicsScale6yScaleEv() -> i32;
  // proto: void QGraphicsScale::setOrigin(const QVector3D & point);
  fn _ZN14QGraphicsScale9setOriginERK9QVector3D(arg0: *const c_void) -> i32;
  // proto: void QGraphicsScale::setYScale(qreal );
  fn _ZN14QGraphicsScale9setYScaleEd(arg0: c_double) -> i32;
  // proto: QVector3D QGraphicsScale::origin();
  fn _ZNK14QGraphicsScale6originEv() -> i32;
  // proto: void QGraphicsScale::scaleChanged();
  fn _ZN14QGraphicsScale12scaleChangedEv() -> i32;
  // proto: void QGraphicsScale::setZScale(qreal );
  fn _ZN14QGraphicsScale9setZScaleEd(arg0: c_double) -> i32;
  // proto: void QGraphicsScale::setXScale(qreal );
  fn _ZN14QGraphicsScale9setXScaleEd(arg0: c_double) -> i32;
  // proto: void QGraphicsScale::xScaleChanged();
  fn _ZN14QGraphicsScale13xScaleChangedEv() -> i32;
  // proto: void QGraphicsScale::zScaleChanged();
  fn _ZN14QGraphicsScale13zScaleChangedEv() -> i32;
  // proto: const QMetaObject * QGraphicsScale::metaObject();
  fn _ZNK14QGraphicsScale10metaObjectEv() -> i32;
  // proto: void QGraphicsScale::NewQGraphicsScale(QObject * parent);
  fn _ZN14QGraphicsScaleC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QGraphicsScale::FreeQGraphicsScale();
  fn _ZN14QGraphicsScaleD0Ev() -> i32;
}

// body block begin
// class sizeof(QGraphicsScale)=1
pub struct QGraphicsScale {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsScale {
  pub fn applyTo<T: QGraphicsScale_applyTo>(&mut self, value: T) -> i32 {
    value.applyTo(self);
    return 1;
  }
}

pub trait QGraphicsScale_applyTo {
  fn applyTo(self, this: &mut QGraphicsScale) -> i32;
}

// proto: void QGraphicsScale::applyTo(QMatrix4x4 * matrix);
impl<'a> /*trait*/ QGraphicsScale_applyTo for (&'a mut QMatrix4x4) {
  fn applyTo(self, this: &mut QGraphicsScale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScale7applyToEP10QMatrix4x4()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK14QGraphicsScale7applyToEP10QMatrix4x4(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScale {
  pub fn zScale<T: QGraphicsScale_zScale>(&mut self, value: T) -> i32 {
    value.zScale(self);
    return 1;
  }
}

pub trait QGraphicsScale_zScale {
  fn zScale(self, this: &mut QGraphicsScale) -> i32;
}

// proto: double QGraphicsScale::zScale();
impl<'a> /*trait*/ QGraphicsScale_zScale for () {
  fn zScale(self, this: &mut QGraphicsScale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScale6zScaleEv()};
    unsafe {_ZNK14QGraphicsScale6zScaleEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScale {
  pub fn yScaleChanged<T: QGraphicsScale_yScaleChanged>(&mut self, value: T) -> i32 {
    value.yScaleChanged(self);
    return 1;
  }
}

pub trait QGraphicsScale_yScaleChanged {
  fn yScaleChanged(self, this: &mut QGraphicsScale) -> i32;
}

// proto: void QGraphicsScale::yScaleChanged();
impl<'a> /*trait*/ QGraphicsScale_yScaleChanged for () {
  fn yScaleChanged(self, this: &mut QGraphicsScale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale13yScaleChangedEv()};
    unsafe {_ZN14QGraphicsScale13yScaleChangedEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScale {
  pub fn originChanged<T: QGraphicsScale_originChanged>(&mut self, value: T) -> i32 {
    value.originChanged(self);
    return 1;
  }
}

pub trait QGraphicsScale_originChanged {
  fn originChanged(self, this: &mut QGraphicsScale) -> i32;
}

// proto: void QGraphicsScale::originChanged();
impl<'a> /*trait*/ QGraphicsScale_originChanged for () {
  fn originChanged(self, this: &mut QGraphicsScale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale13originChangedEv()};
    unsafe {_ZN14QGraphicsScale13originChangedEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScale {
  pub fn xScale<T: QGraphicsScale_xScale>(&mut self, value: T) -> i32 {
    value.xScale(self);
    return 1;
  }
}

pub trait QGraphicsScale_xScale {
  fn xScale(self, this: &mut QGraphicsScale) -> i32;
}

// proto: double QGraphicsScale::xScale();
impl<'a> /*trait*/ QGraphicsScale_xScale for () {
  fn xScale(self, this: &mut QGraphicsScale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScale6xScaleEv()};
    unsafe {_ZNK14QGraphicsScale6xScaleEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScale {
  pub fn yScale<T: QGraphicsScale_yScale>(&mut self, value: T) -> i32 {
    value.yScale(self);
    return 1;
  }
}

pub trait QGraphicsScale_yScale {
  fn yScale(self, this: &mut QGraphicsScale) -> i32;
}

// proto: double QGraphicsScale::yScale();
impl<'a> /*trait*/ QGraphicsScale_yScale for () {
  fn yScale(self, this: &mut QGraphicsScale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScale6yScaleEv()};
    unsafe {_ZNK14QGraphicsScale6yScaleEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScale {
  pub fn setOrigin<T: QGraphicsScale_setOrigin>(&mut self, value: T) -> i32 {
    value.setOrigin(self);
    return 1;
  }
}

pub trait QGraphicsScale_setOrigin {
  fn setOrigin(self, this: &mut QGraphicsScale) -> i32;
}

// proto: void QGraphicsScale::setOrigin(const QVector3D & point);
impl<'a> /*trait*/ QGraphicsScale_setOrigin for (&'a  QVector3D) {
  fn setOrigin(self, this: &mut QGraphicsScale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale9setOriginERK9QVector3D()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QGraphicsScale9setOriginERK9QVector3D(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScale {
  pub fn setYScale<T: QGraphicsScale_setYScale>(&mut self, value: T) -> i32 {
    value.setYScale(self);
    return 1;
  }
}

pub trait QGraphicsScale_setYScale {
  fn setYScale(self, this: &mut QGraphicsScale) -> i32;
}

// proto: void QGraphicsScale::setYScale(qreal );
impl<'a> /*trait*/ QGraphicsScale_setYScale for (f64) {
  fn setYScale(self, this: &mut QGraphicsScale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale9setYScaleEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN14QGraphicsScale9setYScaleEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScale {
  pub fn origin<T: QGraphicsScale_origin>(&mut self, value: T) -> i32 {
    value.origin(self);
    return 1;
  }
}

pub trait QGraphicsScale_origin {
  fn origin(self, this: &mut QGraphicsScale) -> i32;
}

// proto: QVector3D QGraphicsScale::origin();
impl<'a> /*trait*/ QGraphicsScale_origin for () {
  fn origin(self, this: &mut QGraphicsScale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScale6originEv()};
    unsafe {_ZNK14QGraphicsScale6originEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScale {
  pub fn scaleChanged<T: QGraphicsScale_scaleChanged>(&mut self, value: T) -> i32 {
    value.scaleChanged(self);
    return 1;
  }
}

pub trait QGraphicsScale_scaleChanged {
  fn scaleChanged(self, this: &mut QGraphicsScale) -> i32;
}

// proto: void QGraphicsScale::scaleChanged();
impl<'a> /*trait*/ QGraphicsScale_scaleChanged for () {
  fn scaleChanged(self, this: &mut QGraphicsScale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale12scaleChangedEv()};
    unsafe {_ZN14QGraphicsScale12scaleChangedEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScale {
  pub fn setZScale<T: QGraphicsScale_setZScale>(&mut self, value: T) -> i32 {
    value.setZScale(self);
    return 1;
  }
}

pub trait QGraphicsScale_setZScale {
  fn setZScale(self, this: &mut QGraphicsScale) -> i32;
}

// proto: void QGraphicsScale::setZScale(qreal );
impl<'a> /*trait*/ QGraphicsScale_setZScale for (f64) {
  fn setZScale(self, this: &mut QGraphicsScale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale9setZScaleEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN14QGraphicsScale9setZScaleEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScale {
  pub fn setXScale<T: QGraphicsScale_setXScale>(&mut self, value: T) -> i32 {
    value.setXScale(self);
    return 1;
  }
}

pub trait QGraphicsScale_setXScale {
  fn setXScale(self, this: &mut QGraphicsScale) -> i32;
}

// proto: void QGraphicsScale::setXScale(qreal );
impl<'a> /*trait*/ QGraphicsScale_setXScale for (f64) {
  fn setXScale(self, this: &mut QGraphicsScale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale9setXScaleEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN14QGraphicsScale9setXScaleEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsScale {
  pub fn xScaleChanged<T: QGraphicsScale_xScaleChanged>(&mut self, value: T) -> i32 {
    value.xScaleChanged(self);
    return 1;
  }
}

pub trait QGraphicsScale_xScaleChanged {
  fn xScaleChanged(self, this: &mut QGraphicsScale) -> i32;
}

// proto: void QGraphicsScale::xScaleChanged();
impl<'a> /*trait*/ QGraphicsScale_xScaleChanged for () {
  fn xScaleChanged(self, this: &mut QGraphicsScale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale13xScaleChangedEv()};
    unsafe {_ZN14QGraphicsScale13xScaleChangedEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScale {
  pub fn zScaleChanged<T: QGraphicsScale_zScaleChanged>(&mut self, value: T) -> i32 {
    value.zScaleChanged(self);
    return 1;
  }
}

pub trait QGraphicsScale_zScaleChanged {
  fn zScaleChanged(self, this: &mut QGraphicsScale) -> i32;
}

// proto: void QGraphicsScale::zScaleChanged();
impl<'a> /*trait*/ QGraphicsScale_zScaleChanged for () {
  fn zScaleChanged(self, this: &mut QGraphicsScale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale13zScaleChangedEv()};
    unsafe {_ZN14QGraphicsScale13zScaleChangedEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScale {
  pub fn metaObject<T: QGraphicsScale_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QGraphicsScale_metaObject {
  fn metaObject(self, this: &mut QGraphicsScale) -> i32;
}

// proto: const QMetaObject * QGraphicsScale::metaObject();
impl<'a> /*trait*/ QGraphicsScale_metaObject for () {
  fn metaObject(self, this: &mut QGraphicsScale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScale10metaObjectEv()};
    unsafe {_ZNK14QGraphicsScale10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsScale {
  pub fn NewQGraphicsScale<T: QGraphicsScale_NewQGraphicsScale>(value: T) -> QGraphicsScale {
    let rsthis = value.NewQGraphicsScale();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsScale_NewQGraphicsScale {
  fn NewQGraphicsScale(self) -> QGraphicsScale;
}

// proto: void QGraphicsScale::NewQGraphicsScale(QObject * parent);
impl<'a> /*trait*/ QGraphicsScale_NewQGraphicsScale for (&'a mut QObject) {
  fn NewQGraphicsScale(self) -> QGraphicsScale {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScaleC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QGraphicsScaleC1EP7QObject(qthis, arg0)};
    let rsthis = QGraphicsScale{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsScale {
  pub fn FreeQGraphicsScale<T: QGraphicsScale_FreeQGraphicsScale>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsScale(self);
    return 1;
  }
}

pub trait QGraphicsScale_FreeQGraphicsScale {
  fn FreeQGraphicsScale(self, this: &mut QGraphicsScale) -> i32;
}

// proto: void QGraphicsScale::FreeQGraphicsScale();
impl<'a> /*trait*/ QGraphicsScale_FreeQGraphicsScale for () {
  fn FreeQGraphicsScale(self, this: &mut QGraphicsScale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScaleD0Ev()};
    unsafe {_ZN14QGraphicsScaleD0Ev()};
    return 1;
  }
}

