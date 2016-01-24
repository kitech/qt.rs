// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
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
use super::super::core::qobjectdefs::QMetaObject; // 771
use super::super::gui::qmatrix4x4::QMatrix4x4; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGraphicsRotation_Class_Size() -> c_int;
  // proto:  QVector3D QGraphicsRotation::origin();
  fn C_ZNK17QGraphicsRotation6originEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsRotation::setAngle(qreal );
  fn C_ZN17QGraphicsRotation8setAngleEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QGraphicsRotation::QGraphicsRotation(QObject * parent);
  fn C_ZN17QGraphicsRotationC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  const QMetaObject * QGraphicsRotation::metaObject();
  fn C_ZNK17QGraphicsRotation10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsRotation::~QGraphicsRotation();
  fn C_ZN17QGraphicsRotationD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsRotation::setOrigin(const QVector3D & point);
  fn C_ZN17QGraphicsRotation9setOriginERK9QVector3D(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QVector3D QGraphicsRotation::axis();
  fn C_ZNK17QGraphicsRotation4axisEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsRotation::applyTo(QMatrix4x4 * matrix);
  fn C_ZNK17QGraphicsRotation7applyToEP10QMatrix4x4(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsRotation::setAxis(const QVector3D & axis);
  fn C_ZN17QGraphicsRotation7setAxisERK9QVector3D(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qreal QGraphicsRotation::angle();
  fn C_ZNK17QGraphicsRotation5angleEv(qthis: u64 /* *mut c_void*/) -> c_double;
  fn QGraphicsScale_Class_Size() -> c_int;
  // proto:  void QGraphicsScale::applyTo(QMatrix4x4 * matrix);
  fn C_ZNK14QGraphicsScale7applyToEP10QMatrix4x4(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qreal QGraphicsScale::zScale();
  fn C_ZNK14QGraphicsScale6zScaleEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QGraphicsScale::xScale();
  fn C_ZNK14QGraphicsScale6xScaleEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QGraphicsScale::yScale();
  fn C_ZNK14QGraphicsScale6yScaleEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QGraphicsScale::setOrigin(const QVector3D & point);
  fn C_ZN14QGraphicsScale9setOriginERK9QVector3D(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsScale::setYScale(qreal );
  fn C_ZN14QGraphicsScale9setYScaleEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  QVector3D QGraphicsScale::origin();
  fn C_ZNK14QGraphicsScale6originEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsScale::setZScale(qreal );
  fn C_ZN14QGraphicsScale9setZScaleEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QGraphicsScale::setXScale(qreal );
  fn C_ZN14QGraphicsScale9setXScaleEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  const QMetaObject * QGraphicsScale::metaObject();
  fn C_ZNK14QGraphicsScale10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsScale::QGraphicsScale(QObject * parent);
  fn C_ZN14QGraphicsScaleC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  void QGraphicsScale::~QGraphicsScale();
  fn C_ZN14QGraphicsScaleD2Ev(qthis: u64 /* *mut c_void*/);
  fn QGraphicsTransform_Class_Size() -> c_int;
  // proto:  void QGraphicsTransform::applyTo(QMatrix4x4 * matrix);
  fn C_ZNK18QGraphicsTransform7applyToEP10QMatrix4x4(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsTransform::~QGraphicsTransform();
  fn C_ZN18QGraphicsTransformD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsTransform::QGraphicsTransform(QObject * parent);
  fn C_ZN18QGraphicsTransformC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  const QMetaObject * QGraphicsTransform::metaObject();
  fn C_ZNK18QGraphicsTransform10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QGraphicsRotation_SlotProxy_connect__ZN17QGraphicsRotation12angleChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsRotation_SlotProxy_connect__ZN17QGraphicsRotation11axisChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsRotation_SlotProxy_connect__ZN17QGraphicsRotation13originChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsScale_SlotProxy_connect__ZN14QGraphicsScale12scaleChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsScale_SlotProxy_connect__ZN14QGraphicsScale13originChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsScale_SlotProxy_connect__ZN14QGraphicsScale13zScaleChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsScale_SlotProxy_connect__ZN14QGraphicsScale13yScaleChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsScale_SlotProxy_connect__ZN14QGraphicsScale13xScaleChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsRotation)=1
#[derive(Default)]
pub struct QGraphicsRotation {
  qbase: QGraphicsTransform,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _originChanged: QGraphicsRotation_originChanged_signal,
  pub _axisChanged: QGraphicsRotation_axisChanged_signal,
  pub _angleChanged: QGraphicsRotation_angleChanged_signal,
}

// class sizeof(QGraphicsScale)=1
#[derive(Default)]
pub struct QGraphicsScale {
  qbase: QGraphicsTransform,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _yScaleChanged: QGraphicsScale_yScaleChanged_signal,
  pub _xScaleChanged: QGraphicsScale_xScaleChanged_signal,
  pub _zScaleChanged: QGraphicsScale_zScaleChanged_signal,
  pub _scaleChanged: QGraphicsScale_scaleChanged_signal,
  pub _originChanged: QGraphicsScale_originChanged_signal,
}

// class sizeof(QGraphicsTransform)=1
#[derive(Default)]
pub struct QGraphicsTransform {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QGraphicsRotation {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsRotation {
    return QGraphicsRotation{qbase: QGraphicsTransform::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK17QGraphicsRotation6originEv(rsthis.qclsinst)};
    let mut ret1 = QVector3D::inheritFrom(ret as u64);
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
     unsafe {C_ZN17QGraphicsRotation8setAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsRotation::QGraphicsRotation(QObject * parent);
impl /*struct*/ QGraphicsRotation {
  pub fn new<T: QGraphicsRotation_new>(value: T) -> QGraphicsRotation {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsRotation_new {
  fn new(self) -> QGraphicsRotation;
}

  // proto:  void QGraphicsRotation::QGraphicsRotation(QObject * parent);
impl<'a> /*trait*/ QGraphicsRotation_new for (&'a QObject) {
  fn new(self) -> QGraphicsRotation {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotationC2EP7QObject()};
    let ctysz: c_int = unsafe{QGraphicsRotation_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN17QGraphicsRotationC2EP7QObject(arg0)};
    let rsthis = QGraphicsRotation{qbase: QGraphicsTransform::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
impl<'a> /*trait*/ QGraphicsRotation_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QGraphicsRotation) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRotation10metaObjectEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsRotation10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsRotation::~QGraphicsRotation();
impl /*struct*/ QGraphicsRotation {
  pub fn free<RetType, T: QGraphicsRotation_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_free<RetType> {
  fn free(self , rsthis: & QGraphicsRotation) -> RetType;
}

  // proto:  void QGraphicsRotation::~QGraphicsRotation();
impl<'a> /*trait*/ QGraphicsRotation_free<()> for () {
  fn free(self , rsthis: & QGraphicsRotation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotationD2Ev()};
     unsafe {C_ZN17QGraphicsRotationD2Ev(rsthis.qclsinst)};
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
     unsafe {C_ZN17QGraphicsRotation9setOriginERK9QVector3D(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZNK17QGraphicsRotation4axisEv(rsthis.qclsinst)};
    let mut ret1 = QVector3D::inheritFrom(ret as u64);
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
     unsafe {C_ZNK17QGraphicsRotation7applyToEP10QMatrix4x4(rsthis.qclsinst, arg0)};
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
     unsafe {C_ZN17QGraphicsRotation7setAxisERK9QVector3D(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZNK17QGraphicsRotation5angleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsScale {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsScale {
    return QGraphicsScale{qbase: QGraphicsTransform::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
     unsafe {C_ZNK14QGraphicsScale7applyToEP10QMatrix4x4(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZNK14QGraphicsScale6zScaleEv(rsthis.qclsinst)};
    return ret as f64;
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
    let mut ret = unsafe {C_ZNK14QGraphicsScale6xScaleEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {C_ZNK14QGraphicsScale6yScaleEv(rsthis.qclsinst)};
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
     unsafe {C_ZN14QGraphicsScale9setOriginERK9QVector3D(rsthis.qclsinst, arg0)};
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
     unsafe {C_ZN14QGraphicsScale9setYScaleEd(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZNK14QGraphicsScale6originEv(rsthis.qclsinst)};
    let mut ret1 = QVector3D::inheritFrom(ret as u64);
    return ret1;
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
     unsafe {C_ZN14QGraphicsScale9setZScaleEd(rsthis.qclsinst, arg0)};
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
     unsafe {C_ZN14QGraphicsScale9setXScaleEd(rsthis.qclsinst, arg0)};
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
impl<'a> /*trait*/ QGraphicsScale_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QGraphicsScale) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScale10metaObjectEv()};
    let mut ret = unsafe {C_ZNK14QGraphicsScale10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsScale::QGraphicsScale(QObject * parent);
impl /*struct*/ QGraphicsScale {
  pub fn new<T: QGraphicsScale_new>(value: T) -> QGraphicsScale {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsScale_new {
  fn new(self) -> QGraphicsScale;
}

  // proto:  void QGraphicsScale::QGraphicsScale(QObject * parent);
impl<'a> /*trait*/ QGraphicsScale_new for (&'a QObject) {
  fn new(self) -> QGraphicsScale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScaleC2EP7QObject()};
    let ctysz: c_int = unsafe{QGraphicsScale_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN14QGraphicsScaleC2EP7QObject(arg0)};
    let rsthis = QGraphicsScale{qbase: QGraphicsTransform::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsScale::~QGraphicsScale();
impl /*struct*/ QGraphicsScale {
  pub fn free<RetType, T: QGraphicsScale_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsScale_free<RetType> {
  fn free(self , rsthis: & QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::~QGraphicsScale();
impl<'a> /*trait*/ QGraphicsScale_free<()> for () {
  fn free(self , rsthis: & QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScaleD2Ev()};
     unsafe {C_ZN14QGraphicsScaleD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsTransform {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsTransform {
    return QGraphicsTransform{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
     unsafe {C_ZNK18QGraphicsTransform7applyToEP10QMatrix4x4(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsTransform::~QGraphicsTransform();
impl /*struct*/ QGraphicsTransform {
  pub fn free<RetType, T: QGraphicsTransform_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsTransform_free<RetType> {
  fn free(self , rsthis: & QGraphicsTransform) -> RetType;
}

  // proto:  void QGraphicsTransform::~QGraphicsTransform();
impl<'a> /*trait*/ QGraphicsTransform_free<()> for () {
  fn free(self , rsthis: & QGraphicsTransform) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsTransformD2Ev()};
     unsafe {C_ZN18QGraphicsTransformD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsTransform::QGraphicsTransform(QObject * parent);
impl /*struct*/ QGraphicsTransform {
  pub fn new<T: QGraphicsTransform_new>(value: T) -> QGraphicsTransform {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsTransform_new {
  fn new(self) -> QGraphicsTransform;
}

  // proto:  void QGraphicsTransform::QGraphicsTransform(QObject * parent);
impl<'a> /*trait*/ QGraphicsTransform_new for (&'a QObject) {
  fn new(self) -> QGraphicsTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsTransformC2EP7QObject()};
    let ctysz: c_int = unsafe{QGraphicsTransform_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN18QGraphicsTransformC2EP7QObject(arg0)};
    let rsthis = QGraphicsTransform{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
impl<'a> /*trait*/ QGraphicsTransform_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QGraphicsTransform) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QGraphicsTransform10metaObjectEv()};
    let mut ret = unsafe {C_ZNK18QGraphicsTransform10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QGraphicsRotation_originChanged
pub struct QGraphicsRotation_originChanged_signal{poi:u64}
impl /* struct */ QGraphicsRotation {
  pub fn originChanged(&self) -> QGraphicsRotation_originChanged_signal {
     return QGraphicsRotation_originChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsRotation_originChanged_signal {
  pub fn connect<T: QGraphicsRotation_originChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsRotation_originChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsRotation_originChanged_signal);
}

#[derive(Default)] // for QGraphicsRotation_axisChanged
pub struct QGraphicsRotation_axisChanged_signal{poi:u64}
impl /* struct */ QGraphicsRotation {
  pub fn axisChanged(&self) -> QGraphicsRotation_axisChanged_signal {
     return QGraphicsRotation_axisChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsRotation_axisChanged_signal {
  pub fn connect<T: QGraphicsRotation_axisChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsRotation_axisChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsRotation_axisChanged_signal);
}

#[derive(Default)] // for QGraphicsRotation_angleChanged
pub struct QGraphicsRotation_angleChanged_signal{poi:u64}
impl /* struct */ QGraphicsRotation {
  pub fn angleChanged(&self) -> QGraphicsRotation_angleChanged_signal {
     return QGraphicsRotation_angleChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsRotation_angleChanged_signal {
  pub fn connect<T: QGraphicsRotation_angleChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsRotation_angleChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsRotation_angleChanged_signal);
}

// angleChanged()
extern fn QGraphicsRotation_angleChanged_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsRotation_angleChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsRotation_angleChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsRotation_angleChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsRotation_angleChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsRotation_SlotProxy_connect__ZN17QGraphicsRotation12angleChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsRotation_angleChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsRotation_angleChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsRotation_angleChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsRotation_SlotProxy_connect__ZN17QGraphicsRotation12angleChangedEv(arg0, arg1, arg2)};
  }
}
// axisChanged()
extern fn QGraphicsRotation_axisChanged_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsRotation_axisChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsRotation_axisChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsRotation_axisChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsRotation_axisChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsRotation_SlotProxy_connect__ZN17QGraphicsRotation11axisChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsRotation_axisChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsRotation_axisChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsRotation_axisChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsRotation_SlotProxy_connect__ZN17QGraphicsRotation11axisChangedEv(arg0, arg1, arg2)};
  }
}
// originChanged()
extern fn QGraphicsRotation_originChanged_signal_connect_cb_2(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsRotation_originChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsRotation_originChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsRotation_originChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsRotation_originChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsRotation_SlotProxy_connect__ZN17QGraphicsRotation13originChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsRotation_originChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsRotation_originChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsRotation_originChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsRotation_SlotProxy_connect__ZN17QGraphicsRotation13originChangedEv(arg0, arg1, arg2)};
  }
}
#[derive(Default)] // for QGraphicsScale_yScaleChanged
pub struct QGraphicsScale_yScaleChanged_signal{poi:u64}
impl /* struct */ QGraphicsScale {
  pub fn yScaleChanged(&self) -> QGraphicsScale_yScaleChanged_signal {
     return QGraphicsScale_yScaleChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsScale_yScaleChanged_signal {
  pub fn connect<T: QGraphicsScale_yScaleChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsScale_yScaleChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsScale_yScaleChanged_signal);
}

#[derive(Default)] // for QGraphicsScale_xScaleChanged
pub struct QGraphicsScale_xScaleChanged_signal{poi:u64}
impl /* struct */ QGraphicsScale {
  pub fn xScaleChanged(&self) -> QGraphicsScale_xScaleChanged_signal {
     return QGraphicsScale_xScaleChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsScale_xScaleChanged_signal {
  pub fn connect<T: QGraphicsScale_xScaleChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsScale_xScaleChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsScale_xScaleChanged_signal);
}

#[derive(Default)] // for QGraphicsScale_zScaleChanged
pub struct QGraphicsScale_zScaleChanged_signal{poi:u64}
impl /* struct */ QGraphicsScale {
  pub fn zScaleChanged(&self) -> QGraphicsScale_zScaleChanged_signal {
     return QGraphicsScale_zScaleChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsScale_zScaleChanged_signal {
  pub fn connect<T: QGraphicsScale_zScaleChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsScale_zScaleChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsScale_zScaleChanged_signal);
}

#[derive(Default)] // for QGraphicsScale_scaleChanged
pub struct QGraphicsScale_scaleChanged_signal{poi:u64}
impl /* struct */ QGraphicsScale {
  pub fn scaleChanged(&self) -> QGraphicsScale_scaleChanged_signal {
     return QGraphicsScale_scaleChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsScale_scaleChanged_signal {
  pub fn connect<T: QGraphicsScale_scaleChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsScale_scaleChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsScale_scaleChanged_signal);
}

#[derive(Default)] // for QGraphicsScale_originChanged
pub struct QGraphicsScale_originChanged_signal{poi:u64}
impl /* struct */ QGraphicsScale {
  pub fn originChanged(&self) -> QGraphicsScale_originChanged_signal {
     return QGraphicsScale_originChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsScale_originChanged_signal {
  pub fn connect<T: QGraphicsScale_originChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsScale_originChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsScale_originChanged_signal);
}

// scaleChanged()
extern fn QGraphicsScale_scaleChanged_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsScale_scaleChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsScale_scaleChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsScale_scaleChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsScale_scaleChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsScale_SlotProxy_connect__ZN14QGraphicsScale12scaleChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsScale_scaleChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsScale_scaleChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsScale_scaleChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsScale_SlotProxy_connect__ZN14QGraphicsScale12scaleChangedEv(arg0, arg1, arg2)};
  }
}
// originChanged()
extern fn QGraphicsScale_originChanged_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsScale_originChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsScale_originChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsScale_originChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsScale_originChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsScale_SlotProxy_connect__ZN14QGraphicsScale13originChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsScale_originChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsScale_originChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsScale_originChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsScale_SlotProxy_connect__ZN14QGraphicsScale13originChangedEv(arg0, arg1, arg2)};
  }
}
// zScaleChanged()
extern fn QGraphicsScale_zScaleChanged_signal_connect_cb_2(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsScale_zScaleChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsScale_zScaleChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsScale_zScaleChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsScale_zScaleChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsScale_SlotProxy_connect__ZN14QGraphicsScale13zScaleChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsScale_zScaleChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsScale_zScaleChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsScale_zScaleChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsScale_SlotProxy_connect__ZN14QGraphicsScale13zScaleChangedEv(arg0, arg1, arg2)};
  }
}
// yScaleChanged()
extern fn QGraphicsScale_yScaleChanged_signal_connect_cb_3(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsScale_yScaleChanged_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsScale_yScaleChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsScale_yScaleChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsScale_yScaleChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsScale_SlotProxy_connect__ZN14QGraphicsScale13yScaleChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsScale_yScaleChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsScale_yScaleChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsScale_yScaleChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsScale_SlotProxy_connect__ZN14QGraphicsScale13yScaleChangedEv(arg0, arg1, arg2)};
  }
}
// xScaleChanged()
extern fn QGraphicsScale_xScaleChanged_signal_connect_cb_4(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsScale_xScaleChanged_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsScale_xScaleChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsScale_xScaleChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsScale_xScaleChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsScale_SlotProxy_connect__ZN14QGraphicsScale13xScaleChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsScale_xScaleChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsScale_xScaleChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsScale_xScaleChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsScale_SlotProxy_connect__ZN14QGraphicsScale13xScaleChangedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

