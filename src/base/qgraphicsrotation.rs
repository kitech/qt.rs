// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qvector3d::QVector3D;
use super::qobject::QObject;
use super::qmatrix4x4::QMatrix4x4;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGraphicsRotation::angleChanged();
  fn _ZN17QGraphicsRotation12angleChangedEv(qthis: *mut c_void) ;
  // proto:  QVector3D QGraphicsRotation::origin();
  fn _ZNK17QGraphicsRotation6originEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsRotation::setAngle(qreal );
  fn _ZN17QGraphicsRotation8setAngleEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsRotation::NewQGraphicsRotation(QObject * parent);
  fn _ZN17QGraphicsRotationC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QGraphicsRotation::metaObject();
  fn _ZNK17QGraphicsRotation10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsRotation::FreeQGraphicsRotation();
  fn _ZN17QGraphicsRotationD0Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsRotation::setOrigin(const QVector3D & point);
  fn _ZN17QGraphicsRotation9setOriginERK9QVector3D(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QVector3D QGraphicsRotation::axis();
  fn _ZNK17QGraphicsRotation4axisEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsRotation::applyTo(QMatrix4x4 * matrix);
  fn _ZNK17QGraphicsRotation7applyToEP10QMatrix4x4(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsRotation::setAxis(const QVector3D & axis);
  fn _ZN17QGraphicsRotation7setAxisERK9QVector3D(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QGraphicsRotation::angle();
  fn _ZNK17QGraphicsRotation5angleEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsRotation::originChanged();
  fn _ZN17QGraphicsRotation13originChangedEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsRotation::axisChanged();
  fn _ZN17QGraphicsRotation11axisChangedEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QGraphicsRotation)=1
pub struct QGraphicsRotation {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsRotation {
  pub fn angleChanged<T: QGraphicsRotation_angleChanged>(&mut self, value: T)  {
     value.angleChanged(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_angleChanged {
  fn angleChanged(self, rsthis: &mut QGraphicsRotation) ;
}

// proto:  void QGraphicsRotation::angleChanged();
impl<'a> /*trait*/ QGraphicsRotation_angleChanged for () {
  fn angleChanged(self, rsthis: &mut QGraphicsRotation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotation12angleChangedEv()};
     unsafe {_ZN17QGraphicsRotation12angleChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsRotation {
  pub fn origin<T: QGraphicsRotation_origin>(&mut self, value: T) -> QVector3D {
    return value.origin(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_origin {
  fn origin(self, rsthis: &mut QGraphicsRotation) -> QVector3D;
}

// proto:  QVector3D QGraphicsRotation::origin();
impl<'a> /*trait*/ QGraphicsRotation_origin for () {
  fn origin(self, rsthis: &mut QGraphicsRotation) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRotation6originEv()};
    let mut ret = unsafe {_ZNK17QGraphicsRotation6originEv(rsthis.qclsinst)};
    let mut ret1 = QVector3D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsRotation {
  pub fn setAngle<T: QGraphicsRotation_setAngle>(&mut self, value: T)  {
     value.setAngle(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_setAngle {
  fn setAngle(self, rsthis: &mut QGraphicsRotation) ;
}

// proto:  void QGraphicsRotation::setAngle(qreal );
impl<'a> /*trait*/ QGraphicsRotation_setAngle for (f64) {
  fn setAngle(self, rsthis: &mut QGraphicsRotation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotation8setAngleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN17QGraphicsRotation8setAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsRotation {
  pub fn NewQGraphicsRotation<T: QGraphicsRotation_NewQGraphicsRotation>(value: T) -> QGraphicsRotation {
    let rsthis = value.NewQGraphicsRotation();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsRotation_NewQGraphicsRotation {
  fn NewQGraphicsRotation(self) -> QGraphicsRotation;
}

// proto: void QGraphicsRotation::NewQGraphicsRotation(QObject * parent);
impl<'a> /*trait*/ QGraphicsRotation_NewQGraphicsRotation for (&'a mut QObject) {
  fn NewQGraphicsRotation(self) -> QGraphicsRotation {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotationC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QGraphicsRotationC1EP7QObject(qthis, arg0)};
    let rsthis = QGraphicsRotation{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsRotation {
  pub fn metaObject<T: QGraphicsRotation_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_metaObject {
  fn metaObject(self, rsthis: &mut QGraphicsRotation) ;
}

// proto:  const QMetaObject * QGraphicsRotation::metaObject();
impl<'a> /*trait*/ QGraphicsRotation_metaObject for () {
  fn metaObject(self, rsthis: &mut QGraphicsRotation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRotation10metaObjectEv()};
     unsafe {_ZNK17QGraphicsRotation10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsRotation {
  pub fn FreeQGraphicsRotation<T: QGraphicsRotation_FreeQGraphicsRotation>(&mut self, value: T)  {
     value.FreeQGraphicsRotation(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_FreeQGraphicsRotation {
  fn FreeQGraphicsRotation(self, rsthis: &mut QGraphicsRotation) ;
}

// proto:  void QGraphicsRotation::FreeQGraphicsRotation();
impl<'a> /*trait*/ QGraphicsRotation_FreeQGraphicsRotation for () {
  fn FreeQGraphicsRotation(self, rsthis: &mut QGraphicsRotation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotationD0Ev()};
     unsafe {_ZN17QGraphicsRotationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsRotation {
  pub fn setOrigin<T: QGraphicsRotation_setOrigin>(&mut self, value: T)  {
     value.setOrigin(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_setOrigin {
  fn setOrigin(self, rsthis: &mut QGraphicsRotation) ;
}

// proto:  void QGraphicsRotation::setOrigin(const QVector3D & point);
impl<'a> /*trait*/ QGraphicsRotation_setOrigin for (&'a  QVector3D) {
  fn setOrigin(self, rsthis: &mut QGraphicsRotation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotation9setOriginERK9QVector3D()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsRotation9setOriginERK9QVector3D(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsRotation {
  pub fn axis<T: QGraphicsRotation_axis>(&mut self, value: T) -> QVector3D {
    return value.axis(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_axis {
  fn axis(self, rsthis: &mut QGraphicsRotation) -> QVector3D;
}

// proto:  QVector3D QGraphicsRotation::axis();
impl<'a> /*trait*/ QGraphicsRotation_axis for () {
  fn axis(self, rsthis: &mut QGraphicsRotation) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRotation4axisEv()};
    let mut ret = unsafe {_ZNK17QGraphicsRotation4axisEv(rsthis.qclsinst)};
    let mut ret1 = QVector3D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsRotation {
  pub fn applyTo<T: QGraphicsRotation_applyTo>(&mut self, value: T)  {
     value.applyTo(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_applyTo {
  fn applyTo(self, rsthis: &mut QGraphicsRotation) ;
}

// proto:  void QGraphicsRotation::applyTo(QMatrix4x4 * matrix);
impl<'a> /*trait*/ QGraphicsRotation_applyTo for (&'a mut QMatrix4x4) {
  fn applyTo(self, rsthis: &mut QGraphicsRotation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRotation7applyToEP10QMatrix4x4()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK17QGraphicsRotation7applyToEP10QMatrix4x4(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsRotation {
  pub fn setAxis<T: QGraphicsRotation_setAxis>(&mut self, value: T)  {
     value.setAxis(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_setAxis {
  fn setAxis(self, rsthis: &mut QGraphicsRotation) ;
}

// proto:  void QGraphicsRotation::setAxis(const QVector3D & axis);
impl<'a> /*trait*/ QGraphicsRotation_setAxis for (&'a  QVector3D) {
  fn setAxis(self, rsthis: &mut QGraphicsRotation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotation7setAxisERK9QVector3D()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsRotation7setAxisERK9QVector3D(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsRotation {
  pub fn angle<T: QGraphicsRotation_angle>(&mut self, value: T) -> f64 {
    return value.angle(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_angle {
  fn angle(self, rsthis: &mut QGraphicsRotation) -> f64;
}

// proto:  double QGraphicsRotation::angle();
impl<'a> /*trait*/ QGraphicsRotation_angle for () {
  fn angle(self, rsthis: &mut QGraphicsRotation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRotation5angleEv()};
    let mut ret = unsafe {_ZNK17QGraphicsRotation5angleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsRotation {
  pub fn originChanged<T: QGraphicsRotation_originChanged>(&mut self, value: T)  {
     value.originChanged(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_originChanged {
  fn originChanged(self, rsthis: &mut QGraphicsRotation) ;
}

// proto:  void QGraphicsRotation::originChanged();
impl<'a> /*trait*/ QGraphicsRotation_originChanged for () {
  fn originChanged(self, rsthis: &mut QGraphicsRotation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotation13originChangedEv()};
     unsafe {_ZN17QGraphicsRotation13originChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsRotation {
  pub fn axisChanged<T: QGraphicsRotation_axisChanged>(&mut self, value: T)  {
     value.axisChanged(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_axisChanged {
  fn axisChanged(self, rsthis: &mut QGraphicsRotation) ;
}

// proto:  void QGraphicsRotation::axisChanged();
impl<'a> /*trait*/ QGraphicsRotation_axisChanged for () {
  fn axisChanged(self, rsthis: &mut QGraphicsRotation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotation11axisChangedEv()};
     unsafe {_ZN17QGraphicsRotation11axisChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

