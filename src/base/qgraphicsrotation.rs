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

// proto:  void QGraphicsRotation::angleChanged();
impl /*struct*/ QGraphicsRotation {
  pub fn angleChanged<RetType, T: QGraphicsRotation_angleChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.angleChanged(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_angleChanged<RetType> {
  fn angleChanged(self , rsthis: &mut QGraphicsRotation) -> RetType;
}

// proto:  void QGraphicsRotation::angleChanged();
impl<'a> /*trait*/ QGraphicsRotation_angleChanged<()> for () {
  fn angleChanged(self , rsthis: &mut QGraphicsRotation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotation12angleChangedEv()};
     unsafe {_ZN17QGraphicsRotation12angleChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QVector3D QGraphicsRotation::origin();
impl /*struct*/ QGraphicsRotation {
  pub fn origin<RetType, T: QGraphicsRotation_origin<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.origin(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_origin<RetType> {
  fn origin(self , rsthis: &mut QGraphicsRotation) -> RetType;
}

// proto:  QVector3D QGraphicsRotation::origin();
impl<'a> /*trait*/ QGraphicsRotation_origin<QVector3D> for () {
  fn origin(self , rsthis: &mut QGraphicsRotation) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRotation6originEv()};
    let mut ret = unsafe {_ZNK17QGraphicsRotation6originEv(rsthis.qclsinst)};
    let mut ret1 = QVector3D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsRotation::setAngle(qreal );
impl /*struct*/ QGraphicsRotation {
  pub fn setAngle<RetType, T: QGraphicsRotation_setAngle<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setAngle(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_setAngle<RetType> {
  fn setAngle(self , rsthis: &mut QGraphicsRotation) -> RetType;
}

// proto:  void QGraphicsRotation::setAngle(qreal );
impl<'a> /*trait*/ QGraphicsRotation_setAngle<()> for (f64) {
  fn setAngle(self , rsthis: &mut QGraphicsRotation) -> () {
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

// proto:  const QMetaObject * QGraphicsRotation::metaObject();
impl /*struct*/ QGraphicsRotation {
  pub fn metaObject<RetType, T: QGraphicsRotation_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QGraphicsRotation) -> RetType;
}

// proto:  const QMetaObject * QGraphicsRotation::metaObject();
impl<'a> /*trait*/ QGraphicsRotation_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QGraphicsRotation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRotation10metaObjectEv()};
     unsafe {_ZNK17QGraphicsRotation10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QGraphicsRotation::FreeQGraphicsRotation();
impl /*struct*/ QGraphicsRotation {
  pub fn FreeQGraphicsRotation<RetType, T: QGraphicsRotation_FreeQGraphicsRotation<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsRotation(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_FreeQGraphicsRotation<RetType> {
  fn FreeQGraphicsRotation(self , rsthis: &mut QGraphicsRotation) -> RetType;
}

// proto:  void QGraphicsRotation::FreeQGraphicsRotation();
impl<'a> /*trait*/ QGraphicsRotation_FreeQGraphicsRotation<()> for () {
  fn FreeQGraphicsRotation(self , rsthis: &mut QGraphicsRotation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotationD0Ev()};
     unsafe {_ZN17QGraphicsRotationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QGraphicsRotation::setOrigin(const QVector3D & point);
impl /*struct*/ QGraphicsRotation {
  pub fn setOrigin<RetType, T: QGraphicsRotation_setOrigin<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setOrigin(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_setOrigin<RetType> {
  fn setOrigin(self , rsthis: &mut QGraphicsRotation) -> RetType;
}

// proto:  void QGraphicsRotation::setOrigin(const QVector3D & point);
impl<'a> /*trait*/ QGraphicsRotation_setOrigin<()> for (&'a  QVector3D) {
  fn setOrigin(self , rsthis: &mut QGraphicsRotation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotation9setOriginERK9QVector3D()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsRotation9setOriginERK9QVector3D(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QVector3D QGraphicsRotation::axis();
impl /*struct*/ QGraphicsRotation {
  pub fn axis<RetType, T: QGraphicsRotation_axis<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.axis(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_axis<RetType> {
  fn axis(self , rsthis: &mut QGraphicsRotation) -> RetType;
}

// proto:  QVector3D QGraphicsRotation::axis();
impl<'a> /*trait*/ QGraphicsRotation_axis<QVector3D> for () {
  fn axis(self , rsthis: &mut QGraphicsRotation) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRotation4axisEv()};
    let mut ret = unsafe {_ZNK17QGraphicsRotation4axisEv(rsthis.qclsinst)};
    let mut ret1 = QVector3D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsRotation::applyTo(QMatrix4x4 * matrix);
impl /*struct*/ QGraphicsRotation {
  pub fn applyTo<RetType, T: QGraphicsRotation_applyTo<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.applyTo(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_applyTo<RetType> {
  fn applyTo(self , rsthis: &mut QGraphicsRotation) -> RetType;
}

// proto:  void QGraphicsRotation::applyTo(QMatrix4x4 * matrix);
impl<'a> /*trait*/ QGraphicsRotation_applyTo<()> for (&'a mut QMatrix4x4) {
  fn applyTo(self , rsthis: &mut QGraphicsRotation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRotation7applyToEP10QMatrix4x4()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK17QGraphicsRotation7applyToEP10QMatrix4x4(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QGraphicsRotation::setAxis(const QVector3D & axis);
impl /*struct*/ QGraphicsRotation {
  pub fn setAxis<RetType, T: QGraphicsRotation_setAxis<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setAxis(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_setAxis<RetType> {
  fn setAxis(self , rsthis: &mut QGraphicsRotation) -> RetType;
}

// proto:  void QGraphicsRotation::setAxis(const QVector3D & axis);
impl<'a> /*trait*/ QGraphicsRotation_setAxis<()> for (&'a  QVector3D) {
  fn setAxis(self , rsthis: &mut QGraphicsRotation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotation7setAxisERK9QVector3D()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsRotation7setAxisERK9QVector3D(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  double QGraphicsRotation::angle();
impl /*struct*/ QGraphicsRotation {
  pub fn angle<RetType, T: QGraphicsRotation_angle<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.angle(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_angle<RetType> {
  fn angle(self , rsthis: &mut QGraphicsRotation) -> RetType;
}

// proto:  double QGraphicsRotation::angle();
impl<'a> /*trait*/ QGraphicsRotation_angle<f64> for () {
  fn angle(self , rsthis: &mut QGraphicsRotation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRotation5angleEv()};
    let mut ret = unsafe {_ZNK17QGraphicsRotation5angleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  void QGraphicsRotation::originChanged();
impl /*struct*/ QGraphicsRotation {
  pub fn originChanged<RetType, T: QGraphicsRotation_originChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.originChanged(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_originChanged<RetType> {
  fn originChanged(self , rsthis: &mut QGraphicsRotation) -> RetType;
}

// proto:  void QGraphicsRotation::originChanged();
impl<'a> /*trait*/ QGraphicsRotation_originChanged<()> for () {
  fn originChanged(self , rsthis: &mut QGraphicsRotation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotation13originChangedEv()};
     unsafe {_ZN17QGraphicsRotation13originChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QGraphicsRotation::axisChanged();
impl /*struct*/ QGraphicsRotation {
  pub fn axisChanged<RetType, T: QGraphicsRotation_axisChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.axisChanged(self);
    // return 1;
  }
}

pub trait QGraphicsRotation_axisChanged<RetType> {
  fn axisChanged(self , rsthis: &mut QGraphicsRotation) -> RetType;
}

// proto:  void QGraphicsRotation::axisChanged();
impl<'a> /*trait*/ QGraphicsRotation_axisChanged<()> for () {
  fn axisChanged(self , rsthis: &mut QGraphicsRotation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRotation11axisChangedEv()};
     unsafe {_ZN17QGraphicsRotation11axisChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

