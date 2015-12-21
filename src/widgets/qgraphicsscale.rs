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
}

// body block begin
// class sizeof(QGraphicsScale)=1
pub struct QGraphicsScale {
  pub qclsinst: *mut c_void,
}

  // proto:  void QGraphicsScale::applyTo(QMatrix4x4 * matrix);
impl /*struct*/ QGraphicsScale {
  pub fn applyTo<RetType, T: QGraphicsScale_applyTo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.applyTo(self);
    // return 1;
  }
}

pub trait QGraphicsScale_applyTo<RetType> {
  fn applyTo(self , rsthis: &mut QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::applyTo(QMatrix4x4 * matrix);
impl<'a> /*trait*/ QGraphicsScale_applyTo<()> for (QMatrix4x4) {
  fn applyTo(self , rsthis: &mut QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScale7applyToEP10QMatrix4x4()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK14QGraphicsScale7applyToEP10QMatrix4x4(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsScale::zScale();
impl /*struct*/ QGraphicsScale {
  pub fn zScale<RetType, T: QGraphicsScale_zScale<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.zScale(self);
    // return 1;
  }
}

pub trait QGraphicsScale_zScale<RetType> {
  fn zScale(self , rsthis: &mut QGraphicsScale) -> RetType;
}

  // proto:  qreal QGraphicsScale::zScale();
impl<'a> /*trait*/ QGraphicsScale_zScale<f64> for () {
  fn zScale(self , rsthis: &mut QGraphicsScale) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScale6zScaleEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScale6zScaleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsScale::yScaleChanged();
impl /*struct*/ QGraphicsScale {
  pub fn yScaleChanged<RetType, T: QGraphicsScale_yScaleChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.yScaleChanged(self);
    // return 1;
  }
}

pub trait QGraphicsScale_yScaleChanged<RetType> {
  fn yScaleChanged(self , rsthis: &mut QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::yScaleChanged();
impl<'a> /*trait*/ QGraphicsScale_yScaleChanged<()> for () {
  fn yScaleChanged(self , rsthis: &mut QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale13yScaleChangedEv()};
     unsafe {_ZN14QGraphicsScale13yScaleChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsScale::originChanged();
impl /*struct*/ QGraphicsScale {
  pub fn originChanged<RetType, T: QGraphicsScale_originChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.originChanged(self);
    // return 1;
  }
}

pub trait QGraphicsScale_originChanged<RetType> {
  fn originChanged(self , rsthis: &mut QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::originChanged();
impl<'a> /*trait*/ QGraphicsScale_originChanged<()> for () {
  fn originChanged(self , rsthis: &mut QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale13originChangedEv()};
     unsafe {_ZN14QGraphicsScale13originChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsScale::xScale();
impl /*struct*/ QGraphicsScale {
  pub fn xScale<RetType, T: QGraphicsScale_xScale<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.xScale(self);
    // return 1;
  }
}

pub trait QGraphicsScale_xScale<RetType> {
  fn xScale(self , rsthis: &mut QGraphicsScale) -> RetType;
}

  // proto:  qreal QGraphicsScale::xScale();
impl<'a> /*trait*/ QGraphicsScale_xScale<f64> for () {
  fn xScale(self , rsthis: &mut QGraphicsScale) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScale6xScaleEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScale6xScaleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QGraphicsScale::yScale();
impl /*struct*/ QGraphicsScale {
  pub fn yScale<RetType, T: QGraphicsScale_yScale<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.yScale(self);
    // return 1;
  }
}

pub trait QGraphicsScale_yScale<RetType> {
  fn yScale(self , rsthis: &mut QGraphicsScale) -> RetType;
}

  // proto:  qreal QGraphicsScale::yScale();
impl<'a> /*trait*/ QGraphicsScale_yScale<f64> for () {
  fn yScale(self , rsthis: &mut QGraphicsScale) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScale6yScaleEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScale6yScaleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsScale::setOrigin(const QVector3D & point);
impl /*struct*/ QGraphicsScale {
  pub fn setOrigin<RetType, T: QGraphicsScale_setOrigin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setOrigin(self);
    // return 1;
  }
}

pub trait QGraphicsScale_setOrigin<RetType> {
  fn setOrigin(self , rsthis: &mut QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::setOrigin(const QVector3D & point);
impl<'a> /*trait*/ QGraphicsScale_setOrigin<()> for (QVector3D) {
  fn setOrigin(self , rsthis: &mut QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale9setOriginERK9QVector3D()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QGraphicsScale9setOriginERK9QVector3D(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsScale::setYScale(qreal );
impl /*struct*/ QGraphicsScale {
  pub fn setYScale<RetType, T: QGraphicsScale_setYScale<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setYScale(self);
    // return 1;
  }
}

pub trait QGraphicsScale_setYScale<RetType> {
  fn setYScale(self , rsthis: &mut QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::setYScale(qreal );
impl<'a> /*trait*/ QGraphicsScale_setYScale<()> for (f64) {
  fn setYScale(self , rsthis: &mut QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale9setYScaleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QGraphicsScale9setYScaleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVector3D QGraphicsScale::origin();
impl /*struct*/ QGraphicsScale {
  pub fn origin<RetType, T: QGraphicsScale_origin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.origin(self);
    // return 1;
  }
}

pub trait QGraphicsScale_origin<RetType> {
  fn origin(self , rsthis: &mut QGraphicsScale) -> RetType;
}

  // proto:  QVector3D QGraphicsScale::origin();
impl<'a> /*trait*/ QGraphicsScale_origin<QVector3D> for () {
  fn origin(self , rsthis: &mut QGraphicsScale) -> QVector3D {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScale6originEv()};
    let mut ret = unsafe {_ZNK14QGraphicsScale6originEv(rsthis.qclsinst)};
    let mut ret1 = QVector3D{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsScale::scaleChanged();
impl /*struct*/ QGraphicsScale {
  pub fn scaleChanged<RetType, T: QGraphicsScale_scaleChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.scaleChanged(self);
    // return 1;
  }
}

pub trait QGraphicsScale_scaleChanged<RetType> {
  fn scaleChanged(self , rsthis: &mut QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::scaleChanged();
impl<'a> /*trait*/ QGraphicsScale_scaleChanged<()> for () {
  fn scaleChanged(self , rsthis: &mut QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale12scaleChangedEv()};
     unsafe {_ZN14QGraphicsScale12scaleChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsScale::setZScale(qreal );
impl /*struct*/ QGraphicsScale {
  pub fn setZScale<RetType, T: QGraphicsScale_setZScale<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setZScale(self);
    // return 1;
  }
}

pub trait QGraphicsScale_setZScale<RetType> {
  fn setZScale(self , rsthis: &mut QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::setZScale(qreal );
impl<'a> /*trait*/ QGraphicsScale_setZScale<()> for (f64) {
  fn setZScale(self , rsthis: &mut QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale9setZScaleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QGraphicsScale9setZScaleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsScale::setXScale(qreal );
impl /*struct*/ QGraphicsScale {
  pub fn setXScale<RetType, T: QGraphicsScale_setXScale<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setXScale(self);
    // return 1;
  }
}

pub trait QGraphicsScale_setXScale<RetType> {
  fn setXScale(self , rsthis: &mut QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::setXScale(qreal );
impl<'a> /*trait*/ QGraphicsScale_setXScale<()> for (f64) {
  fn setXScale(self , rsthis: &mut QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale9setXScaleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN14QGraphicsScale9setXScaleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsScale::xScaleChanged();
impl /*struct*/ QGraphicsScale {
  pub fn xScaleChanged<RetType, T: QGraphicsScale_xScaleChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.xScaleChanged(self);
    // return 1;
  }
}

pub trait QGraphicsScale_xScaleChanged<RetType> {
  fn xScaleChanged(self , rsthis: &mut QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::xScaleChanged();
impl<'a> /*trait*/ QGraphicsScale_xScaleChanged<()> for () {
  fn xScaleChanged(self , rsthis: &mut QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale13xScaleChangedEv()};
     unsafe {_ZN14QGraphicsScale13xScaleChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsScale::zScaleChanged();
impl /*struct*/ QGraphicsScale {
  pub fn zScaleChanged<RetType, T: QGraphicsScale_zScaleChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.zScaleChanged(self);
    // return 1;
  }
}

pub trait QGraphicsScale_zScaleChanged<RetType> {
  fn zScaleChanged(self , rsthis: &mut QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::zScaleChanged();
impl<'a> /*trait*/ QGraphicsScale_zScaleChanged<()> for () {
  fn zScaleChanged(self , rsthis: &mut QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScale13zScaleChangedEv()};
     unsafe {_ZN14QGraphicsScale13zScaleChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsScale::metaObject();
impl /*struct*/ QGraphicsScale {
  pub fn metaObject<RetType, T: QGraphicsScale_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsScale_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QGraphicsScale) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsScale::metaObject();
impl<'a> /*trait*/ QGraphicsScale_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGraphicsScale10metaObjectEv()};
     unsafe {_ZNK14QGraphicsScale10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsScale::QGraphicsScale(QObject * parent);
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

  // proto:  void QGraphicsScale::QGraphicsScale(QObject * parent);
impl<'a> /*trait*/ QGraphicsScale_NewQGraphicsScale for (QObject) {
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

  // proto:  void QGraphicsScale::~QGraphicsScale();
impl /*struct*/ QGraphicsScale {
  pub fn FreeQGraphicsScale<RetType, T: QGraphicsScale_FreeQGraphicsScale<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsScale(self);
    // return 1;
  }
}

pub trait QGraphicsScale_FreeQGraphicsScale<RetType> {
  fn FreeQGraphicsScale(self , rsthis: &mut QGraphicsScale) -> RetType;
}

  // proto:  void QGraphicsScale::~QGraphicsScale();
impl<'a> /*trait*/ QGraphicsScale_FreeQGraphicsScale<()> for () {
  fn FreeQGraphicsScale(self , rsthis: &mut QGraphicsScale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGraphicsScaleD0Ev()};
     unsafe {_ZN14QGraphicsScaleD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

