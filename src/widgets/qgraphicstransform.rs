// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
// src-file: /QtWidgets/qgraphicstransform.h
// dst-file: /src/widgets/qgraphicstransform.rs
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
// use super::qgraphicstransform::QGraphicsTransform; // 773
use std::ops::Deref;
use super::super::gui::qvector3d::QVector3D; // 771
use super::super::core::qobject::QObject; // 771
use super::super::gui::qmatrix4x4::QMatrix4x4; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QGraphicsRotation::angleChanged();
  fn _ZN17QGraphicsRotation12angleChangedEv(qthis: *mut c_void);
  // proto:  QVector3D QGraphicsRotation::origin();
  fn _ZNK17QGraphicsRotation6originEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsRotation::setAngle(qreal );
  fn _ZN17QGraphicsRotation8setAngleEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsRotation::QGraphicsRotation(QObject * parent);
  fn _ZN17QGraphicsRotationC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QGraphicsRotation::metaObject();
  fn _ZNK17QGraphicsRotation10metaObjectEv(qthis: *mut c_void);
  // proto:  void QGraphicsRotation::~QGraphicsRotation();
  fn _ZN17QGraphicsRotationD0Ev(qthis: *mut c_void);
  // proto:  void QGraphicsRotation::setOrigin(const QVector3D & point);
  fn _ZN17QGraphicsRotation9setOriginERK9QVector3D(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QVector3D QGraphicsRotation::axis();
  fn _ZNK17QGraphicsRotation4axisEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsRotation::applyTo(QMatrix4x4 * matrix);
  fn _ZNK17QGraphicsRotation7applyToEP10QMatrix4x4(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsRotation::setAxis(const QVector3D & axis);
  fn _ZN17QGraphicsRotation7setAxisERK9QVector3D(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QGraphicsRotation::angle();
  fn _ZNK17QGraphicsRotation5angleEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsRotation::originChanged();
  fn _ZN17QGraphicsRotation13originChangedEv(qthis: *mut c_void);
  // proto:  void QGraphicsRotation::axisChanged();
  fn _ZN17QGraphicsRotation11axisChangedEv(qthis: *mut c_void);
  // proto:  void QGraphicsScale::applyTo(QMatrix4x4 * matrix);
  fn _ZNK14QGraphicsScale7applyToEP10QMatrix4x4(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QGraphicsScale::zScale();
  fn _ZNK14QGraphicsScale6zScaleEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsScale::yScaleChanged();
  fn _ZN14QGraphicsScale13yScaleChangedEv(qthis: *mut c_void);
  // proto:  void QGraphicsScale::originChanged();
  fn _ZN14QGraphicsScale13originChangedEv(qthis: *mut c_void);
  // proto:  qreal QGraphicsScale::xScale();
  fn _ZNK14QGraphicsScale6xScaleEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QGraphicsScale::yScale();
  fn _ZNK14QGraphicsScale6yScaleEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsScale::setOrigin(const QVector3D & point);
  fn _ZN14QGraphicsScale9setOriginERK9QVector3D(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsScale::setYScale(qreal );
  fn _ZN14QGraphicsScale9setYScaleEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QVector3D QGraphicsScale::origin();
  fn _ZNK14QGraphicsScale6originEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsScale::scaleChanged();
  fn _ZN14QGraphicsScale12scaleChangedEv(qthis: *mut c_void);
  // proto:  void QGraphicsScale::setZScale(qreal );
  fn _ZN14QGraphicsScale9setZScaleEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsScale::setXScale(qreal );
  fn _ZN14QGraphicsScale9setXScaleEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsScale::xScaleChanged();
  fn _ZN14QGraphicsScale13xScaleChangedEv(qthis: *mut c_void);
  // proto:  void QGraphicsScale::zScaleChanged();
  fn _ZN14QGraphicsScale13zScaleChangedEv(qthis: *mut c_void);
  // proto:  const QMetaObject * QGraphicsScale::metaObject();
  fn _ZNK14QGraphicsScale10metaObjectEv(qthis: *mut c_void);
  // proto:  void QGraphicsScale::QGraphicsScale(QObject * parent);
  fn _ZN14QGraphicsScaleC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsScale::~QGraphicsScale();
  fn _ZN14QGraphicsScaleD0Ev(qthis: *mut c_void);
  // proto:  void QGraphicsTransform::applyTo(QMatrix4x4 * matrix);
  fn _ZNK18QGraphicsTransform7applyToEP10QMatrix4x4(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsTransform::~QGraphicsTransform();
  fn _ZN18QGraphicsTransformD0Ev(qthis: *mut c_void);
  // proto:  void QGraphicsTransform::QGraphicsTransform(QObject * parent);
  fn _ZN18QGraphicsTransformC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QGraphicsTransform::metaObject();
  fn _ZNK18QGraphicsTransform10metaObjectEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsRotation)=1
pub struct QGraphicsRotation {
  qbase: QGraphicsTransform,
  pub qclsinst: *mut c_void,
}

// class sizeof(QGraphicsScale)=1
pub struct QGraphicsScale {
  qbase: QGraphicsTransform,
  pub qclsinst: *mut c_void,
}

// class sizeof(QGraphicsTransform)=1
pub struct QGraphicsTransform {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsRotation {
  pub fn inheritFrom(qthis: *mut c_void) -> QGraphicsRotation {
    return QGraphicsRotation{qbase: QGraphicsTransform::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QGraphicsRotation {
  type Target = QGraphicsTransform;

  fn deref(&self) -> &QGraphicsTransform {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsTransform> for QGraphicsRotation {
  fn as_ref(& self) -> & QGraphicsTransform {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsRotation::angleChanged();
impl /*struct*/ QGraphicsRotation {
  pub fn angleChanged<RetType, T: QGraphicsRotation_angleChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.angleChanged(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_angleChanged<RetType> {
  fn angleChanged(self , rsthis: & QGraphicsRotation) -> RetType;
}

  // proto:  void QGraphicsRotation::angleChanged();
impl<'a> /*trait*/ QGraphicsRotation_angleChanged<()> for () {
  fn angleChanged(self , rsthis: & QGraphicsRotation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotation12angleChangedEv()};
     unsafe {_ZN17QGraphicsRotation12angleChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QVector3D QGraphicsRotation::origin();
impl /*struct*/ QGraphicsRotation {
  pub fn origin<RetType, T: QGraphicsRotation_origin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.origin(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_origin<RetType> {
  fn origin(self , rsthis: & QGraphicsRotation) -> RetType;
}

  // proto:  QVector3D QGraphicsRotation::origin();
impl<'a> /*trait*/ QGraphicsRotation_origin<QVector3D> for () {
  fn origin(self , rsthis: & QGraphicsRotation) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRotation6originEv()};
    let mut ret = unsafe {_ZNK17QGraphicsRotation6originEv(rsthis.qclsinst)};
    let mut ret1 = QVector3D::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsRotation::setAngle(qreal );
impl /*struct*/ QGraphicsRotation {
  pub fn setAngle<RetType, T: QGraphicsRotation_setAngle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAngle(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_setAngle<RetType> {
  fn setAngle(self , rsthis: & QGraphicsRotation) -> RetType;
}

  // proto:  void QGraphicsRotation::setAngle(qreal );
impl<'a> /*trait*/ QGraphicsRotation_setAngle<()> for (f64) {
  fn setAngle(self , rsthis: & QGraphicsRotation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotation8setAngleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN17QGraphicsRotation8setAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsRotation::QGraphicsRotation(QObject * parent);
impl /*struct*/ QGraphicsRotation {
  pub fn New<T: QGraphicsRotation_New>(value: T) -> QGraphicsRotation {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsRotation_New {
  fn New(self) -> QGraphicsRotation;
}

  // proto:  void QGraphicsRotation::QGraphicsRotation(QObject * parent);
impl<'a> /*trait*/ QGraphicsRotation_New for (&'a QObject) {
  fn New(self) -> QGraphicsRotation {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotationC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QGraphicsRotationC1EP7QObject(qthis, arg0)};
    let rsthis = QGraphicsRotation{/**/qbase: QGraphicsTransform::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsRotation::metaObject();
impl /*struct*/ QGraphicsRotation {
  pub fn metaObject<RetType, T: QGraphicsRotation_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGraphicsRotation) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsRotation::metaObject();
impl<'a> /*trait*/ QGraphicsRotation_metaObject<()> for () {
  fn metaObject(self , rsthis: & QGraphicsRotation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRotation10metaObjectEv()};
     unsafe {_ZNK17QGraphicsRotation10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsRotation::~QGraphicsRotation();
impl /*struct*/ QGraphicsRotation {
  pub fn Free<RetType, T: QGraphicsRotation_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_Free<RetType> {
  fn Free(self , rsthis: & QGraphicsRotation) -> RetType;
}

  // proto:  void QGraphicsRotation::~QGraphicsRotation();
impl<'a> /*trait*/ QGraphicsRotation_Free<()> for () {
  fn Free(self , rsthis: & QGraphicsRotation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotationD0Ev()};
     unsafe {_ZN17QGraphicsRotationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsRotation::setOrigin(const QVector3D & point);
impl /*struct*/ QGraphicsRotation {
  pub fn setOrigin<RetType, T: QGraphicsRotation_setOrigin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOrigin(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_setOrigin<RetType> {
  fn setOrigin(self , rsthis: & QGraphicsRotation) -> RetType;
}

  // proto:  void QGraphicsRotation::setOrigin(const QVector3D & point);
impl<'a> /*trait*/ QGraphicsRotation_setOrigin<()> for (&'a QVector3D) {
  fn setOrigin(self , rsthis: & QGraphicsRotation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotation9setOriginERK9QVector3D()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsRotation9setOriginERK9QVector3D(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVector3D QGraphicsRotation::axis();
impl /*struct*/ QGraphicsRotation {
  pub fn axis<RetType, T: QGraphicsRotation_axis<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.axis(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_axis<RetType> {
  fn axis(self , rsthis: & QGraphicsRotation) -> RetType;
}

  // proto:  QVector3D QGraphicsRotation::axis();
impl<'a> /*trait*/ QGraphicsRotation_axis<QVector3D> for () {
  fn axis(self , rsthis: & QGraphicsRotation) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRotation4axisEv()};
    let mut ret = unsafe {_ZNK17QGraphicsRotation4axisEv(rsthis.qclsinst)};
    let mut ret1 = QVector3D::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsRotation::applyTo(QMatrix4x4 * matrix);
impl /*struct*/ QGraphicsRotation {
  pub fn applyTo<RetType, T: QGraphicsRotation_applyTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.applyTo(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_applyTo<RetType> {
  fn applyTo(self , rsthis: & QGraphicsRotation) -> RetType;
}

  // proto:  void QGraphicsRotation::applyTo(QMatrix4x4 * matrix);
impl<'a> /*trait*/ QGraphicsRotation_applyTo<()> for (&'a QMatrix4x4) {
  fn applyTo(self , rsthis: & QGraphicsRotation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRotation7applyToEP10QMatrix4x4()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK17QGraphicsRotation7applyToEP10QMatrix4x4(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsRotation::setAxis(const QVector3D & axis);
impl /*struct*/ QGraphicsRotation {
  pub fn setAxis<RetType, T: QGraphicsRotation_setAxis<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAxis(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_setAxis<RetType> {
  fn setAxis(self , rsthis: & QGraphicsRotation) -> RetType;
}

  // proto:  void QGraphicsRotation::setAxis(const QVector3D & axis);
impl<'a> /*trait*/ QGraphicsRotation_setAxis<()> for (&'a QVector3D) {
  fn setAxis(self , rsthis: & QGraphicsRotation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotation7setAxisERK9QVector3D()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsRotation7setAxisERK9QVector3D(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsRotation::angle();
impl /*struct*/ QGraphicsRotation {
  pub fn angle<RetType, T: QGraphicsRotation_angle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.angle(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_angle<RetType> {
  fn angle(self , rsthis: & QGraphicsRotation) -> RetType;
}

  // proto:  qreal QGraphicsRotation::angle();
impl<'a> /*trait*/ QGraphicsRotation_angle<f64> for () {
  fn angle(self , rsthis: & QGraphicsRotation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRotation5angleEv()};
    let mut ret = unsafe {_ZNK17QGraphicsRotation5angleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsRotation::originChanged();
impl /*struct*/ QGraphicsRotation {
  pub fn originChanged<RetType, T: QGraphicsRotation_originChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.originChanged(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_originChanged<RetType> {
  fn originChanged(self , rsthis: & QGraphicsRotation) -> RetType;
}

  // proto:  void QGraphicsRotation::originChanged();
impl<'a> /*trait*/ QGraphicsRotation_originChanged<()> for () {
  fn originChanged(self , rsthis: & QGraphicsRotation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotation13originChangedEv()};
     unsafe {_ZN17QGraphicsRotation13originChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsRotation::axisChanged();
impl /*struct*/ QGraphicsRotation {
  pub fn axisChanged<RetType, T: QGraphicsRotation_axisChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.axisChanged(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_axisChanged<RetType> {
  fn axisChanged(self , rsthis: & QGraphicsRotation) -> RetType;
}

  // proto:  void QGraphicsRotation::axisChanged();
impl<'a> /*trait*/ QGraphicsRotation_axisChanged<()> for () {
  fn axisChanged(self , rsthis: & QGraphicsRotation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotation11axisChangedEv()};
     unsafe {_ZN17QGraphicsRotation11axisChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsScale {
  pub fn inheritFrom(qthis: *mut c_void) -> QGraphicsScale {
    return QGraphicsScale{qbase: QGraphicsTransform::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QGraphicsScale {
  type Target = QGraphicsTransform;

  fn deref(&self) -> &QGraphicsTransform {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsTransform> for QGraphicsScale {
  fn as_ref(& self) -> & QGraphicsTransform {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsScale::applyTo(QMatrix4x4 * matrix);
impl /*struct*/ QGraphicsScale {
  pub fn applyTo<RetType, T: QGraphicsScale_applyTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.applyTo(self);
    // return 1;
  }
}

pub trait QGraphicsScale_applyTo<RetType> {
  fn applyTo(self , rsthis: & QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::applyTo(QMatrix4x4 * matrix);
impl<'a> /*trait*/ QGraphicsScale_applyTo<()> for (&'a QMatrix4x4) {
  fn applyTo(self , rsthis: & QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScale7applyToEP10QMatrix4x4()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK14QGraphicsScale7applyToEP10QMatrix4x4(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsScale::zScale();
impl /*struct*/ QGraphicsScale {
  pub fn zScale<RetType, T: QGraphicsScale_zScale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.zScale(self);
    // return 1;
  }
}

pub trait QGraphicsScale_zScale<RetType> {
  fn zScale(self , rsthis: & QGraphicsScale) -> RetType;
}

  // proto:  qreal QGraphicsScale::zScale();
impl<'a> /*trait*/ QGraphicsScale_zScale<f64> for () {
  fn zScale(self , rsthis: & QGraphicsScale) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScale6zScaleEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScale6zScaleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsScale::yScaleChanged();
impl /*struct*/ QGraphicsScale {
  pub fn yScaleChanged<RetType, T: QGraphicsScale_yScaleChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.yScaleChanged(self);
    // return 1;
  }
}

pub trait QGraphicsScale_yScaleChanged<RetType> {
  fn yScaleChanged(self , rsthis: & QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::yScaleChanged();
impl<'a> /*trait*/ QGraphicsScale_yScaleChanged<()> for () {
  fn yScaleChanged(self , rsthis: & QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale13yScaleChangedEv()};
     unsafe {_ZN14QGraphicsScale13yScaleChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsScale::originChanged();
impl /*struct*/ QGraphicsScale {
  pub fn originChanged<RetType, T: QGraphicsScale_originChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.originChanged(self);
    // return 1;
  }
}

pub trait QGraphicsScale_originChanged<RetType> {
  fn originChanged(self , rsthis: & QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::originChanged();
impl<'a> /*trait*/ QGraphicsScale_originChanged<()> for () {
  fn originChanged(self , rsthis: & QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale13originChangedEv()};
     unsafe {_ZN14QGraphicsScale13originChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsScale::xScale();
impl /*struct*/ QGraphicsScale {
  pub fn xScale<RetType, T: QGraphicsScale_xScale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.xScale(self);
    // return 1;
  }
}

pub trait QGraphicsScale_xScale<RetType> {
  fn xScale(self , rsthis: & QGraphicsScale) -> RetType;
}

  // proto:  qreal QGraphicsScale::xScale();
impl<'a> /*trait*/ QGraphicsScale_xScale<f64> for () {
  fn xScale(self , rsthis: & QGraphicsScale) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScale6xScaleEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScale6xScaleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QGraphicsScale::yScale();
impl /*struct*/ QGraphicsScale {
  pub fn yScale<RetType, T: QGraphicsScale_yScale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.yScale(self);
    // return 1;
  }
}

pub trait QGraphicsScale_yScale<RetType> {
  fn yScale(self , rsthis: & QGraphicsScale) -> RetType;
}

  // proto:  qreal QGraphicsScale::yScale();
impl<'a> /*trait*/ QGraphicsScale_yScale<f64> for () {
  fn yScale(self , rsthis: & QGraphicsScale) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScale6yScaleEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScale6yScaleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsScale::setOrigin(const QVector3D & point);
impl /*struct*/ QGraphicsScale {
  pub fn setOrigin<RetType, T: QGraphicsScale_setOrigin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOrigin(self);
    // return 1;
  }
}

pub trait QGraphicsScale_setOrigin<RetType> {
  fn setOrigin(self , rsthis: & QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::setOrigin(const QVector3D & point);
impl<'a> /*trait*/ QGraphicsScale_setOrigin<()> for (&'a QVector3D) {
  fn setOrigin(self , rsthis: & QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale9setOriginERK9QVector3D()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScale9setOriginERK9QVector3D(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsScale::setYScale(qreal );
impl /*struct*/ QGraphicsScale {
  pub fn setYScale<RetType, T: QGraphicsScale_setYScale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setYScale(self);
    // return 1;
  }
}

pub trait QGraphicsScale_setYScale<RetType> {
  fn setYScale(self , rsthis: & QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::setYScale(qreal );
impl<'a> /*trait*/ QGraphicsScale_setYScale<()> for (f64) {
  fn setYScale(self , rsthis: & QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale9setYScaleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QGraphicsScale9setYScaleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVector3D QGraphicsScale::origin();
impl /*struct*/ QGraphicsScale {
  pub fn origin<RetType, T: QGraphicsScale_origin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.origin(self);
    // return 1;
  }
}

pub trait QGraphicsScale_origin<RetType> {
  fn origin(self , rsthis: & QGraphicsScale) -> RetType;
}

  // proto:  QVector3D QGraphicsScale::origin();
impl<'a> /*trait*/ QGraphicsScale_origin<QVector3D> for () {
  fn origin(self , rsthis: & QGraphicsScale) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScale6originEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScale6originEv(rsthis.qclsinst)};
    let mut ret1 = QVector3D::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsScale::scaleChanged();
impl /*struct*/ QGraphicsScale {
  pub fn scaleChanged<RetType, T: QGraphicsScale_scaleChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scaleChanged(self);
    // return 1;
  }
}

pub trait QGraphicsScale_scaleChanged<RetType> {
  fn scaleChanged(self , rsthis: & QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::scaleChanged();
impl<'a> /*trait*/ QGraphicsScale_scaleChanged<()> for () {
  fn scaleChanged(self , rsthis: & QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale12scaleChangedEv()};
     unsafe {_ZN14QGraphicsScale12scaleChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsScale::setZScale(qreal );
impl /*struct*/ QGraphicsScale {
  pub fn setZScale<RetType, T: QGraphicsScale_setZScale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setZScale(self);
    // return 1;
  }
}

pub trait QGraphicsScale_setZScale<RetType> {
  fn setZScale(self , rsthis: & QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::setZScale(qreal );
impl<'a> /*trait*/ QGraphicsScale_setZScale<()> for (f64) {
  fn setZScale(self , rsthis: & QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale9setZScaleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QGraphicsScale9setZScaleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsScale::setXScale(qreal );
impl /*struct*/ QGraphicsScale {
  pub fn setXScale<RetType, T: QGraphicsScale_setXScale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setXScale(self);
    // return 1;
  }
}

pub trait QGraphicsScale_setXScale<RetType> {
  fn setXScale(self , rsthis: & QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::setXScale(qreal );
impl<'a> /*trait*/ QGraphicsScale_setXScale<()> for (f64) {
  fn setXScale(self , rsthis: & QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale9setXScaleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QGraphicsScale9setXScaleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsScale::xScaleChanged();
impl /*struct*/ QGraphicsScale {
  pub fn xScaleChanged<RetType, T: QGraphicsScale_xScaleChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.xScaleChanged(self);
    // return 1;
  }
}

pub trait QGraphicsScale_xScaleChanged<RetType> {
  fn xScaleChanged(self , rsthis: & QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::xScaleChanged();
impl<'a> /*trait*/ QGraphicsScale_xScaleChanged<()> for () {
  fn xScaleChanged(self , rsthis: & QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale13xScaleChangedEv()};
     unsafe {_ZN14QGraphicsScale13xScaleChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsScale::zScaleChanged();
impl /*struct*/ QGraphicsScale {
  pub fn zScaleChanged<RetType, T: QGraphicsScale_zScaleChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.zScaleChanged(self);
    // return 1;
  }
}

pub trait QGraphicsScale_zScaleChanged<RetType> {
  fn zScaleChanged(self , rsthis: & QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::zScaleChanged();
impl<'a> /*trait*/ QGraphicsScale_zScaleChanged<()> for () {
  fn zScaleChanged(self , rsthis: & QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale13zScaleChangedEv()};
     unsafe {_ZN14QGraphicsScale13zScaleChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsScale::metaObject();
impl /*struct*/ QGraphicsScale {
  pub fn metaObject<RetType, T: QGraphicsScale_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsScale_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGraphicsScale) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsScale::metaObject();
impl<'a> /*trait*/ QGraphicsScale_metaObject<()> for () {
  fn metaObject(self , rsthis: & QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScale10metaObjectEv()};
     unsafe {_ZNK14QGraphicsScale10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsScale::QGraphicsScale(QObject * parent);
impl /*struct*/ QGraphicsScale {
  pub fn New<T: QGraphicsScale_New>(value: T) -> QGraphicsScale {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsScale_New {
  fn New(self) -> QGraphicsScale;
}

  // proto:  void QGraphicsScale::QGraphicsScale(QObject * parent);
impl<'a> /*trait*/ QGraphicsScale_New for (&'a QObject) {
  fn New(self) -> QGraphicsScale {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScaleC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QGraphicsScaleC1EP7QObject(qthis, arg0)};
    let rsthis = QGraphicsScale{/**/qbase: QGraphicsTransform::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsScale::~QGraphicsScale();
impl /*struct*/ QGraphicsScale {
  pub fn Free<RetType, T: QGraphicsScale_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QGraphicsScale_Free<RetType> {
  fn Free(self , rsthis: & QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::~QGraphicsScale();
impl<'a> /*trait*/ QGraphicsScale_Free<()> for () {
  fn Free(self , rsthis: & QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScaleD0Ev()};
     unsafe {_ZN14QGraphicsScaleD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsTransform {
  pub fn inheritFrom(qthis: *mut c_void) -> QGraphicsTransform {
    return QGraphicsTransform{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QGraphicsTransform {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QGraphicsTransform {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsTransform::applyTo(QMatrix4x4 * matrix);
impl /*struct*/ QGraphicsTransform {
  pub fn applyTo<RetType, T: QGraphicsTransform_applyTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.applyTo(self);
    // return 1;
  }
}

pub trait QGraphicsTransform_applyTo<RetType> {
  fn applyTo(self , rsthis: & QGraphicsTransform) -> RetType;
}

  // proto:  void QGraphicsTransform::applyTo(QMatrix4x4 * matrix);
impl<'a> /*trait*/ QGraphicsTransform_applyTo<()> for (&'a QMatrix4x4) {
  fn applyTo(self , rsthis: & QGraphicsTransform) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QGraphicsTransform7applyToEP10QMatrix4x4()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK18QGraphicsTransform7applyToEP10QMatrix4x4(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsTransform::~QGraphicsTransform();
impl /*struct*/ QGraphicsTransform {
  pub fn Free<RetType, T: QGraphicsTransform_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QGraphicsTransform_Free<RetType> {
  fn Free(self , rsthis: & QGraphicsTransform) -> RetType;
}

  // proto:  void QGraphicsTransform::~QGraphicsTransform();
impl<'a> /*trait*/ QGraphicsTransform_Free<()> for () {
  fn Free(self , rsthis: & QGraphicsTransform) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsTransformD0Ev()};
     unsafe {_ZN18QGraphicsTransformD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsTransform::QGraphicsTransform(QObject * parent);
impl /*struct*/ QGraphicsTransform {
  pub fn New<T: QGraphicsTransform_New>(value: T) -> QGraphicsTransform {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsTransform_New {
  fn New(self) -> QGraphicsTransform;
}

  // proto:  void QGraphicsTransform::QGraphicsTransform(QObject * parent);
impl<'a> /*trait*/ QGraphicsTransform_New for (&'a QObject) {
  fn New(self) -> QGraphicsTransform {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsTransformC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QGraphicsTransformC1EP7QObject(qthis, arg0)};
    let rsthis = QGraphicsTransform{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsTransform::metaObject();
impl /*struct*/ QGraphicsTransform {
  pub fn metaObject<RetType, T: QGraphicsTransform_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsTransform_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGraphicsTransform) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsTransform::metaObject();
impl<'a> /*trait*/ QGraphicsTransform_metaObject<()> for () {
  fn metaObject(self , rsthis: & QGraphicsTransform) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QGraphicsTransform10metaObjectEv()};
     unsafe {_ZNK18QGraphicsTransform10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

