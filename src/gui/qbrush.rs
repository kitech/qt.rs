// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtGui/qbrush.h
// dst-file: /src/gui/qbrush.rs
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
use super::super::core::qpoint::QPointF; // 771
use super::qpixmap::QPixmap; // 773
use super::qimage::QImage; // 773
use super::qcolor::QColor; // 773
// use super::qbrush::QGradient; // 773
use super::qtransform::QTransform; // 773
use super::qmatrix::QMatrix; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal radius, qreal fx, qreal fy);
  fn _ZN15QRadialGradientC1Eddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double);
  // proto:  void QRadialGradient::setFocalPoint(qreal x, qreal y);
  fn _ZN15QRadialGradient13setFocalPointEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QRadialGradient::QRadialGradient();
  fn _ZN15QRadialGradientC1Ev(qthis: *mut c_void);
  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal radius, const QPointF & focalPoint);
  fn _ZN15QRadialGradientC1ERK7QPointFdS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: *mut c_void);
  // proto:  qreal QRadialGradient::radius();
  fn _ZNK15QRadialGradient6radiusEv(qthis: *mut c_void) -> c_double;
  // proto:  void QRadialGradient::setFocalPoint(const QPointF & focalPoint);
  fn _ZN15QRadialGradient13setFocalPointERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal centerRadius, const QPointF & focalPoint, qreal focalRadius);
  fn _ZN15QRadialGradientC1ERK7QPointFdS2_d(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: *mut c_void, arg3: c_double);
  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal centerRadius, qreal fx, qreal fy, qreal focalRadius);
  fn _ZN15QRadialGradientC1Edddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double);
  // proto:  qreal QRadialGradient::centerRadius();
  fn _ZNK15QRadialGradient12centerRadiusEv(qthis: *mut c_void) -> c_double;
  // proto:  QPointF QRadialGradient::focalPoint();
  fn _ZNK15QRadialGradient10focalPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QRadialGradient::focalRadius();
  fn _ZNK15QRadialGradient11focalRadiusEv(qthis: *mut c_void) -> c_double;
  // proto:  QPointF QRadialGradient::center();
  fn _ZNK15QRadialGradient6centerEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRadialGradient::setCenter(const QPointF & center);
  fn _ZN15QRadialGradient9setCenterERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal radius);
  fn _ZN15QRadialGradientC1ERK7QPointFd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double);
  // proto:  void QRadialGradient::setCenterRadius(qreal radius);
  fn _ZN15QRadialGradient15setCenterRadiusEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QRadialGradient::setFocalRadius(qreal radius);
  fn _ZN15QRadialGradient14setFocalRadiusEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QRadialGradient::setRadius(qreal radius);
  fn _ZN15QRadialGradient9setRadiusEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal radius);
  fn _ZN15QRadialGradientC1Eddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double);
  // proto:  void QRadialGradient::setCenter(qreal x, qreal y);
  fn _ZN15QRadialGradient9setCenterEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  qreal QConicalGradient::angle();
  fn _ZNK16QConicalGradient5angleEv(qthis: *mut c_void) -> c_double;
  // proto:  QPointF QConicalGradient::center();
  fn _ZNK16QConicalGradient6centerEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QConicalGradient::QConicalGradient(const QPointF & center, qreal startAngle);
  fn _ZN16QConicalGradientC1ERK7QPointFd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double);
  // proto:  void QConicalGradient::QConicalGradient();
  fn _ZN16QConicalGradientC1Ev(qthis: *mut c_void);
  // proto:  void QConicalGradient::setAngle(qreal angle);
  fn _ZN16QConicalGradient8setAngleEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QConicalGradient::setCenter(qreal x, qreal y);
  fn _ZN16QConicalGradient9setCenterEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QConicalGradient::setCenter(const QPointF & center);
  fn _ZN16QConicalGradient9setCenterERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QConicalGradient::QConicalGradient(qreal cx, qreal cy, qreal startAngle);
  fn _ZN16QConicalGradientC1Eddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double);
  // proto:  void QBrush::QBrush();
  fn _ZN6QBrushC1Ev(qthis: *mut c_void);
  // proto:  void QBrush::QBrush(const QPixmap & pixmap);
  fn _ZN6QBrushC1ERK7QPixmap(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QBrush::setTexture(const QPixmap & pixmap);
  fn _ZN6QBrush10setTextureERK7QPixmap(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QBrush::setTextureImage(const QImage & image);
  fn _ZN6QBrush15setTextureImageERK6QImage(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QBrush::QBrush(const QColor & color, const QPixmap & pixmap);
  fn _ZN6QBrushC1ERK6QColorRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QPixmap QBrush::texture();
  fn _ZNK6QBrush7textureEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QBrush::QBrush(const QGradient & gradient);
  fn _ZN6QBrushC1ERK9QGradient(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTransform QBrush::transform();
  fn _ZNK6QBrush9transformEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QBrush::setTransform(const QTransform & );
  fn _ZN6QBrush12setTransformERK10QTransform(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QBrush::isOpaque();
  fn _ZNK6QBrush8isOpaqueEv(qthis: *mut c_void) -> c_char;
  // proto:  const QGradient * QBrush::gradient();
  fn _ZNK6QBrush8gradientEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QBrush::~QBrush();
  fn _ZN6QBrushD0Ev(qthis: *mut c_void);
  // proto:  void QBrush::setMatrix(const QMatrix & mat);
  fn _ZN6QBrush9setMatrixERK7QMatrix(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QBrush::setColor(const QColor & color);
  fn _ZN6QBrush8setColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QBrush::QBrush(const QBrush & brush);
  fn _ZN6QBrushC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMatrix & QBrush::matrix();
  fn _ZNK6QBrush6matrixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QImage QBrush::textureImage();
  fn _ZNK6QBrush12textureImageEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QBrush::isDetached();
  fn _ZNK6QBrush10isDetachedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QBrush::swap(QBrush & other);
  fn _ZN6QBrush4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QColor & QBrush::color();
  fn _ZNK6QBrush5colorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QBrush::QBrush(const QImage & image);
  fn _ZN6QBrushC1ERK6QImage(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGradient::setColorAt(qreal pos, const QColor & color);
  fn _ZN9QGradient10setColorAtEdRK6QColor(qthis: *mut c_void, arg0: c_double, arg1: *mut c_void);
  // proto:  QGradientStops QGradient::stops();
  fn _ZNK9QGradient5stopsEv(qthis: *mut c_void);
  // proto:  void QGradient::QGradient();
  fn _ZN9QGradientC1Ev(qthis: *mut c_void);
  // proto:  void QLinearGradient::setFinalStop(qreal x, qreal y);
  fn _ZN15QLinearGradient12setFinalStopEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  QPointF QLinearGradient::start();
  fn _ZNK15QLinearGradient5startEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLinearGradient::QLinearGradient(qreal xStart, qreal yStart, qreal xFinalStop, qreal yFinalStop);
  fn _ZN15QLinearGradientC1Edddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  void QLinearGradient::QLinearGradient(const QPointF & start, const QPointF & finalStop);
  fn _ZN15QLinearGradientC1ERK7QPointFS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QLinearGradient::setStart(qreal x, qreal y);
  fn _ZN15QLinearGradient8setStartEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QLinearGradient::setStart(const QPointF & start);
  fn _ZN15QLinearGradient8setStartERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QLinearGradient::QLinearGradient();
  fn _ZN15QLinearGradientC1Ev(qthis: *mut c_void);
  // proto:  QPointF QLinearGradient::finalStop();
  fn _ZNK15QLinearGradient9finalStopEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLinearGradient::setFinalStop(const QPointF & stop);
  fn _ZN15QLinearGradient12setFinalStopERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QRadialGradient)=1
pub struct QRadialGradient {
  pub qclsinst: *mut c_void,
}

// class sizeof(QConicalGradient)=1
pub struct QConicalGradient {
  pub qclsinst: *mut c_void,
}

// class sizeof(QBrush)=1
pub struct QBrush {
  pub qclsinst: *mut c_void,
}

// class sizeof(QGradient)=1
pub struct QGradient {
  pub qclsinst: *mut c_void,
}

// class sizeof(QBrushData)=1
pub struct QBrushData {
  pub qclsinst: *mut c_void,
}

// class sizeof(QLinearGradient)=1
pub struct QLinearGradient {
  pub qclsinst: *mut c_void,
}

  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal radius, qreal fx, qreal fy);
impl /*struct*/ QRadialGradient {
  pub fn NewQRadialGradient<T: QRadialGradient_NewQRadialGradient>(value: T) -> QRadialGradient {
    let rsthis = value.NewQRadialGradient();
    return rsthis;
    // return 1;
  }
}

pub trait QRadialGradient_NewQRadialGradient {
  fn NewQRadialGradient(self) -> QRadialGradient;
}

  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal radius, qreal fx, qreal fy);
impl<'a> /*trait*/ QRadialGradient_NewQRadialGradient for (f64, f64, f64, f64, f64) {
  fn NewQRadialGradient(self) -> QRadialGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1Eddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    unsafe {_ZN15QRadialGradientC1Eddddd(qthis, arg0, arg1, arg2, arg3, arg4)};
    let rsthis = QRadialGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRadialGradient::setFocalPoint(qreal x, qreal y);
impl /*struct*/ QRadialGradient {
  pub fn setFocalPoint<RetType, T: QRadialGradient_setFocalPoint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFocalPoint(self);
    // return 1;
  }
}

pub trait QRadialGradient_setFocalPoint<RetType> {
  fn setFocalPoint(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  void QRadialGradient::setFocalPoint(qreal x, qreal y);
impl<'a> /*trait*/ QRadialGradient_setFocalPoint<()> for (f64, f64) {
  fn setFocalPoint(self , rsthis: &mut QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient13setFocalPointEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN15QRadialGradient13setFocalPointEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient();
impl<'a> /*trait*/ QRadialGradient_NewQRadialGradient for () {
  fn NewQRadialGradient(self) -> QRadialGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1Ev()};
    unsafe {_ZN15QRadialGradientC1Ev(qthis)};
    let rsthis = QRadialGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal radius, const QPointF & focalPoint);
impl<'a> /*trait*/ QRadialGradient_NewQRadialGradient for (QPointF, f64, QPointF) {
  fn NewQRadialGradient(self) -> QRadialGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1ERK7QPointFdS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN15QRadialGradientC1ERK7QPointFdS2_(qthis, arg0, arg1, arg2)};
    let rsthis = QRadialGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QRadialGradient::radius();
impl /*struct*/ QRadialGradient {
  pub fn radius<RetType, T: QRadialGradient_radius<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.radius(self);
    // return 1;
  }
}

pub trait QRadialGradient_radius<RetType> {
  fn radius(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  qreal QRadialGradient::radius();
impl<'a> /*trait*/ QRadialGradient_radius<f64> for () {
  fn radius(self , rsthis: &mut QRadialGradient) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient6radiusEv()};
    let mut ret = unsafe {_ZNK15QRadialGradient6radiusEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QRadialGradient::setFocalPoint(const QPointF & focalPoint);
impl<'a> /*trait*/ QRadialGradient_setFocalPoint<()> for (QPointF) {
  fn setFocalPoint(self , rsthis: &mut QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient13setFocalPointERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QRadialGradient13setFocalPointERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal centerRadius, const QPointF & focalPoint, qreal focalRadius);
impl<'a> /*trait*/ QRadialGradient_NewQRadialGradient for (QPointF, f64, QPointF, f64) {
  fn NewQRadialGradient(self) -> QRadialGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1ERK7QPointFdS2_d()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_double;
    unsafe {_ZN15QRadialGradientC1ERK7QPointFdS2_d(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QRadialGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal centerRadius, qreal fx, qreal fy, qreal focalRadius);
impl<'a> /*trait*/ QRadialGradient_NewQRadialGradient for (f64, f64, f64, f64, f64, f64) {
  fn NewQRadialGradient(self) -> QRadialGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1Edddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
    unsafe {_ZN15QRadialGradientC1Edddddd(qthis, arg0, arg1, arg2, arg3, arg4, arg5)};
    let rsthis = QRadialGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QRadialGradient::centerRadius();
impl /*struct*/ QRadialGradient {
  pub fn centerRadius<RetType, T: QRadialGradient_centerRadius<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.centerRadius(self);
    // return 1;
  }
}

pub trait QRadialGradient_centerRadius<RetType> {
  fn centerRadius(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  qreal QRadialGradient::centerRadius();
impl<'a> /*trait*/ QRadialGradient_centerRadius<f64> for () {
  fn centerRadius(self , rsthis: &mut QRadialGradient) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient12centerRadiusEv()};
    let mut ret = unsafe {_ZNK15QRadialGradient12centerRadiusEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QPointF QRadialGradient::focalPoint();
impl /*struct*/ QRadialGradient {
  pub fn focalPoint<RetType, T: QRadialGradient_focalPoint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.focalPoint(self);
    // return 1;
  }
}

pub trait QRadialGradient_focalPoint<RetType> {
  fn focalPoint(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  QPointF QRadialGradient::focalPoint();
impl<'a> /*trait*/ QRadialGradient_focalPoint<QPointF> for () {
  fn focalPoint(self , rsthis: &mut QRadialGradient) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient10focalPointEv()};
    let mut ret = unsafe {_ZNK15QRadialGradient10focalPointEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QRadialGradient::focalRadius();
impl /*struct*/ QRadialGradient {
  pub fn focalRadius<RetType, T: QRadialGradient_focalRadius<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.focalRadius(self);
    // return 1;
  }
}

pub trait QRadialGradient_focalRadius<RetType> {
  fn focalRadius(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  qreal QRadialGradient::focalRadius();
impl<'a> /*trait*/ QRadialGradient_focalRadius<f64> for () {
  fn focalRadius(self , rsthis: &mut QRadialGradient) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient11focalRadiusEv()};
    let mut ret = unsafe {_ZNK15QRadialGradient11focalRadiusEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QPointF QRadialGradient::center();
impl /*struct*/ QRadialGradient {
  pub fn center<RetType, T: QRadialGradient_center<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.center(self);
    // return 1;
  }
}

pub trait QRadialGradient_center<RetType> {
  fn center(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  QPointF QRadialGradient::center();
impl<'a> /*trait*/ QRadialGradient_center<QPointF> for () {
  fn center(self , rsthis: &mut QRadialGradient) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient6centerEv()};
    let mut ret = unsafe {_ZNK15QRadialGradient6centerEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QRadialGradient::setCenter(const QPointF & center);
impl /*struct*/ QRadialGradient {
  pub fn setCenter<RetType, T: QRadialGradient_setCenter<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCenter(self);
    // return 1;
  }
}

pub trait QRadialGradient_setCenter<RetType> {
  fn setCenter(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  void QRadialGradient::setCenter(const QPointF & center);
impl<'a> /*trait*/ QRadialGradient_setCenter<()> for (QPointF) {
  fn setCenter(self , rsthis: &mut QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient9setCenterERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QRadialGradient9setCenterERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal radius);
impl<'a> /*trait*/ QRadialGradient_NewQRadialGradient for (QPointF, f64) {
  fn NewQRadialGradient(self) -> QRadialGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1ERK7QPointFd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    unsafe {_ZN15QRadialGradientC1ERK7QPointFd(qthis, arg0, arg1)};
    let rsthis = QRadialGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRadialGradient::setCenterRadius(qreal radius);
impl /*struct*/ QRadialGradient {
  pub fn setCenterRadius<RetType, T: QRadialGradient_setCenterRadius<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCenterRadius(self);
    // return 1;
  }
}

pub trait QRadialGradient_setCenterRadius<RetType> {
  fn setCenterRadius(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  void QRadialGradient::setCenterRadius(qreal radius);
impl<'a> /*trait*/ QRadialGradient_setCenterRadius<()> for (f64) {
  fn setCenterRadius(self , rsthis: &mut QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient15setCenterRadiusEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QRadialGradient15setCenterRadiusEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::setFocalRadius(qreal radius);
impl /*struct*/ QRadialGradient {
  pub fn setFocalRadius<RetType, T: QRadialGradient_setFocalRadius<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFocalRadius(self);
    // return 1;
  }
}

pub trait QRadialGradient_setFocalRadius<RetType> {
  fn setFocalRadius(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  void QRadialGradient::setFocalRadius(qreal radius);
impl<'a> /*trait*/ QRadialGradient_setFocalRadius<()> for (f64) {
  fn setFocalRadius(self , rsthis: &mut QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient14setFocalRadiusEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QRadialGradient14setFocalRadiusEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::setRadius(qreal radius);
impl /*struct*/ QRadialGradient {
  pub fn setRadius<RetType, T: QRadialGradient_setRadius<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRadius(self);
    // return 1;
  }
}

pub trait QRadialGradient_setRadius<RetType> {
  fn setRadius(self , rsthis: &mut QRadialGradient) -> RetType;
}

  // proto:  void QRadialGradient::setRadius(qreal radius);
impl<'a> /*trait*/ QRadialGradient_setRadius<()> for (f64) {
  fn setRadius(self , rsthis: &mut QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient9setRadiusEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QRadialGradient9setRadiusEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal radius);
impl<'a> /*trait*/ QRadialGradient_NewQRadialGradient for (f64, f64, f64) {
  fn NewQRadialGradient(self) -> QRadialGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1Eddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    unsafe {_ZN15QRadialGradientC1Eddd(qthis, arg0, arg1, arg2)};
    let rsthis = QRadialGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRadialGradient::setCenter(qreal x, qreal y);
impl<'a> /*trait*/ QRadialGradient_setCenter<()> for (f64, f64) {
  fn setCenter(self , rsthis: &mut QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient9setCenterEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN15QRadialGradient9setCenterEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  qreal QConicalGradient::angle();
impl /*struct*/ QConicalGradient {
  pub fn angle<RetType, T: QConicalGradient_angle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.angle(self);
    // return 1;
  }
}

pub trait QConicalGradient_angle<RetType> {
  fn angle(self , rsthis: &mut QConicalGradient) -> RetType;
}

  // proto:  qreal QConicalGradient::angle();
impl<'a> /*trait*/ QConicalGradient_angle<f64> for () {
  fn angle(self , rsthis: &mut QConicalGradient) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QConicalGradient5angleEv()};
    let mut ret = unsafe {_ZNK16QConicalGradient5angleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QPointF QConicalGradient::center();
impl /*struct*/ QConicalGradient {
  pub fn center<RetType, T: QConicalGradient_center<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.center(self);
    // return 1;
  }
}

pub trait QConicalGradient_center<RetType> {
  fn center(self , rsthis: &mut QConicalGradient) -> RetType;
}

  // proto:  QPointF QConicalGradient::center();
impl<'a> /*trait*/ QConicalGradient_center<QPointF> for () {
  fn center(self , rsthis: &mut QConicalGradient) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QConicalGradient6centerEv()};
    let mut ret = unsafe {_ZNK16QConicalGradient6centerEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QConicalGradient::QConicalGradient(const QPointF & center, qreal startAngle);
impl /*struct*/ QConicalGradient {
  pub fn NewQConicalGradient<T: QConicalGradient_NewQConicalGradient>(value: T) -> QConicalGradient {
    let rsthis = value.NewQConicalGradient();
    return rsthis;
    // return 1;
  }
}

pub trait QConicalGradient_NewQConicalGradient {
  fn NewQConicalGradient(self) -> QConicalGradient;
}

  // proto:  void QConicalGradient::QConicalGradient(const QPointF & center, qreal startAngle);
impl<'a> /*trait*/ QConicalGradient_NewQConicalGradient for (QPointF, f64) {
  fn NewQConicalGradient(self) -> QConicalGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradientC1ERK7QPointFd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    unsafe {_ZN16QConicalGradientC1ERK7QPointFd(qthis, arg0, arg1)};
    let rsthis = QConicalGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QConicalGradient::QConicalGradient();
impl<'a> /*trait*/ QConicalGradient_NewQConicalGradient for () {
  fn NewQConicalGradient(self) -> QConicalGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradientC1Ev()};
    unsafe {_ZN16QConicalGradientC1Ev(qthis)};
    let rsthis = QConicalGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QConicalGradient::setAngle(qreal angle);
impl /*struct*/ QConicalGradient {
  pub fn setAngle<RetType, T: QConicalGradient_setAngle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAngle(self);
    // return 1;
  }
}

pub trait QConicalGradient_setAngle<RetType> {
  fn setAngle(self , rsthis: &mut QConicalGradient) -> RetType;
}

  // proto:  void QConicalGradient::setAngle(qreal angle);
impl<'a> /*trait*/ QConicalGradient_setAngle<()> for (f64) {
  fn setAngle(self , rsthis: &mut QConicalGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradient8setAngleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QConicalGradient8setAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QConicalGradient::setCenter(qreal x, qreal y);
impl /*struct*/ QConicalGradient {
  pub fn setCenter<RetType, T: QConicalGradient_setCenter<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCenter(self);
    // return 1;
  }
}

pub trait QConicalGradient_setCenter<RetType> {
  fn setCenter(self , rsthis: &mut QConicalGradient) -> RetType;
}

  // proto:  void QConicalGradient::setCenter(qreal x, qreal y);
impl<'a> /*trait*/ QConicalGradient_setCenter<()> for (f64, f64) {
  fn setCenter(self , rsthis: &mut QConicalGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradient9setCenterEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN16QConicalGradient9setCenterEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QConicalGradient::setCenter(const QPointF & center);
impl<'a> /*trait*/ QConicalGradient_setCenter<()> for (QPointF) {
  fn setCenter(self , rsthis: &mut QConicalGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradient9setCenterERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QConicalGradient9setCenterERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QConicalGradient::QConicalGradient(qreal cx, qreal cy, qreal startAngle);
impl<'a> /*trait*/ QConicalGradient_NewQConicalGradient for (f64, f64, f64) {
  fn NewQConicalGradient(self) -> QConicalGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradientC1Eddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    unsafe {_ZN16QConicalGradientC1Eddd(qthis, arg0, arg1, arg2)};
    let rsthis = QConicalGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QBrush::QBrush();
impl /*struct*/ QBrush {
  pub fn NewQBrush<T: QBrush_NewQBrush>(value: T) -> QBrush {
    let rsthis = value.NewQBrush();
    return rsthis;
    // return 1;
  }
}

pub trait QBrush_NewQBrush {
  fn NewQBrush(self) -> QBrush;
}

  // proto:  void QBrush::QBrush();
impl<'a> /*trait*/ QBrush_NewQBrush for () {
  fn NewQBrush(self) -> QBrush {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC1Ev()};
    unsafe {_ZN6QBrushC1Ev(qthis)};
    let rsthis = QBrush{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QBrush::QBrush(const QPixmap & pixmap);
impl<'a> /*trait*/ QBrush_NewQBrush for (QPixmap) {
  fn NewQBrush(self) -> QBrush {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC1ERK7QPixmap()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QBrushC1ERK7QPixmap(qthis, arg0)};
    let rsthis = QBrush{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QBrush::setTexture(const QPixmap & pixmap);
impl /*struct*/ QBrush {
  pub fn setTexture<RetType, T: QBrush_setTexture<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTexture(self);
    // return 1;
  }
}

pub trait QBrush_setTexture<RetType> {
  fn setTexture(self , rsthis: &mut QBrush) -> RetType;
}

  // proto:  void QBrush::setTexture(const QPixmap & pixmap);
impl<'a> /*trait*/ QBrush_setTexture<()> for (QPixmap) {
  fn setTexture(self , rsthis: &mut QBrush) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrush10setTextureERK7QPixmap()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QBrush10setTextureERK7QPixmap(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QBrush::setTextureImage(const QImage & image);
impl /*struct*/ QBrush {
  pub fn setTextureImage<RetType, T: QBrush_setTextureImage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTextureImage(self);
    // return 1;
  }
}

pub trait QBrush_setTextureImage<RetType> {
  fn setTextureImage(self , rsthis: &mut QBrush) -> RetType;
}

  // proto:  void QBrush::setTextureImage(const QImage & image);
impl<'a> /*trait*/ QBrush_setTextureImage<()> for (QImage) {
  fn setTextureImage(self , rsthis: &mut QBrush) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrush15setTextureImageERK6QImage()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QBrush15setTextureImageERK6QImage(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QBrush::QBrush(const QColor & color, const QPixmap & pixmap);
impl<'a> /*trait*/ QBrush_NewQBrush for (QColor, QPixmap) {
  fn NewQBrush(self) -> QBrush {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC1ERK6QColorRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN6QBrushC1ERK6QColorRK7QPixmap(qthis, arg0, arg1)};
    let rsthis = QBrush{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPixmap QBrush::texture();
impl /*struct*/ QBrush {
  pub fn texture<RetType, T: QBrush_texture<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.texture(self);
    // return 1;
  }
}

pub trait QBrush_texture<RetType> {
  fn texture(self , rsthis: &mut QBrush) -> RetType;
}

  // proto:  QPixmap QBrush::texture();
impl<'a> /*trait*/ QBrush_texture<QPixmap> for () {
  fn texture(self , rsthis: &mut QBrush) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush7textureEv()};
    let mut ret = unsafe {_ZNK6QBrush7textureEv(rsthis.qclsinst)};
    let mut ret1 = QPixmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QBrush::QBrush(const QGradient & gradient);
impl<'a> /*trait*/ QBrush_NewQBrush for (QGradient) {
  fn NewQBrush(self) -> QBrush {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC1ERK9QGradient()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QBrushC1ERK9QGradient(qthis, arg0)};
    let rsthis = QBrush{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QTransform QBrush::transform();
impl /*struct*/ QBrush {
  pub fn transform<RetType, T: QBrush_transform<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.transform(self);
    // return 1;
  }
}

pub trait QBrush_transform<RetType> {
  fn transform(self , rsthis: &mut QBrush) -> RetType;
}

  // proto:  QTransform QBrush::transform();
impl<'a> /*trait*/ QBrush_transform<QTransform> for () {
  fn transform(self , rsthis: &mut QBrush) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush9transformEv()};
    let mut ret = unsafe {_ZNK6QBrush9transformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QBrush::setTransform(const QTransform & );
impl /*struct*/ QBrush {
  pub fn setTransform<RetType, T: QBrush_setTransform<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTransform(self);
    // return 1;
  }
}

pub trait QBrush_setTransform<RetType> {
  fn setTransform(self , rsthis: &mut QBrush) -> RetType;
}

  // proto:  void QBrush::setTransform(const QTransform & );
impl<'a> /*trait*/ QBrush_setTransform<()> for (QTransform) {
  fn setTransform(self , rsthis: &mut QBrush) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrush12setTransformERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QBrush12setTransformERK10QTransform(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QBrush::isOpaque();
impl /*struct*/ QBrush {
  pub fn isOpaque<RetType, T: QBrush_isOpaque<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isOpaque(self);
    // return 1;
  }
}

pub trait QBrush_isOpaque<RetType> {
  fn isOpaque(self , rsthis: &mut QBrush) -> RetType;
}

  // proto:  bool QBrush::isOpaque();
impl<'a> /*trait*/ QBrush_isOpaque<i8> for () {
  fn isOpaque(self , rsthis: &mut QBrush) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush8isOpaqueEv()};
    let mut ret = unsafe {_ZNK6QBrush8isOpaqueEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QGradient * QBrush::gradient();
impl /*struct*/ QBrush {
  pub fn gradient<RetType, T: QBrush_gradient<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.gradient(self);
    // return 1;
  }
}

pub trait QBrush_gradient<RetType> {
  fn gradient(self , rsthis: &mut QBrush) -> RetType;
}

  // proto:  const QGradient * QBrush::gradient();
impl<'a> /*trait*/ QBrush_gradient<QGradient> for () {
  fn gradient(self , rsthis: &mut QBrush) -> QGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush8gradientEv()};
    let mut ret = unsafe {_ZNK6QBrush8gradientEv(rsthis.qclsinst)};
    let mut ret1 = QGradient{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QBrush::~QBrush();
impl /*struct*/ QBrush {
  pub fn FreeQBrush<RetType, T: QBrush_FreeQBrush<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQBrush(self);
    // return 1;
  }
}

pub trait QBrush_FreeQBrush<RetType> {
  fn FreeQBrush(self , rsthis: &mut QBrush) -> RetType;
}

  // proto:  void QBrush::~QBrush();
impl<'a> /*trait*/ QBrush_FreeQBrush<()> for () {
  fn FreeQBrush(self , rsthis: &mut QBrush) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushD0Ev()};
     unsafe {_ZN6QBrushD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QBrush::setMatrix(const QMatrix & mat);
impl /*struct*/ QBrush {
  pub fn setMatrix<RetType, T: QBrush_setMatrix<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMatrix(self);
    // return 1;
  }
}

pub trait QBrush_setMatrix<RetType> {
  fn setMatrix(self , rsthis: &mut QBrush) -> RetType;
}

  // proto:  void QBrush::setMatrix(const QMatrix & mat);
impl<'a> /*trait*/ QBrush_setMatrix<()> for (QMatrix) {
  fn setMatrix(self , rsthis: &mut QBrush) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrush9setMatrixERK7QMatrix()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QBrush9setMatrixERK7QMatrix(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QBrush::setColor(const QColor & color);
impl /*struct*/ QBrush {
  pub fn setColor<RetType, T: QBrush_setColor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setColor(self);
    // return 1;
  }
}

pub trait QBrush_setColor<RetType> {
  fn setColor(self , rsthis: &mut QBrush) -> RetType;
}

  // proto:  void QBrush::setColor(const QColor & color);
impl<'a> /*trait*/ QBrush_setColor<()> for (QColor) {
  fn setColor(self , rsthis: &mut QBrush) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrush8setColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QBrush8setColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QBrush::QBrush(const QBrush & brush);
impl<'a> /*trait*/ QBrush_NewQBrush for (QBrush) {
  fn NewQBrush(self) -> QBrush {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QBrushC1ERKS_(qthis, arg0)};
    let rsthis = QBrush{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMatrix & QBrush::matrix();
impl /*struct*/ QBrush {
  pub fn matrix<RetType, T: QBrush_matrix<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.matrix(self);
    // return 1;
  }
}

pub trait QBrush_matrix<RetType> {
  fn matrix(self , rsthis: &mut QBrush) -> RetType;
}

  // proto:  const QMatrix & QBrush::matrix();
impl<'a> /*trait*/ QBrush_matrix<QMatrix> for () {
  fn matrix(self , rsthis: &mut QBrush) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush6matrixEv()};
    let mut ret = unsafe {_ZNK6QBrush6matrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QImage QBrush::textureImage();
impl /*struct*/ QBrush {
  pub fn textureImage<RetType, T: QBrush_textureImage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.textureImage(self);
    // return 1;
  }
}

pub trait QBrush_textureImage<RetType> {
  fn textureImage(self , rsthis: &mut QBrush) -> RetType;
}

  // proto:  QImage QBrush::textureImage();
impl<'a> /*trait*/ QBrush_textureImage<QImage> for () {
  fn textureImage(self , rsthis: &mut QBrush) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush12textureImageEv()};
    let mut ret = unsafe {_ZNK6QBrush12textureImageEv(rsthis.qclsinst)};
    let mut ret1 = QImage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QBrush::isDetached();
impl /*struct*/ QBrush {
  pub fn isDetached<RetType, T: QBrush_isDetached<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isDetached(self);
    // return 1;
  }
}

pub trait QBrush_isDetached<RetType> {
  fn isDetached(self , rsthis: &mut QBrush) -> RetType;
}

  // proto:  bool QBrush::isDetached();
impl<'a> /*trait*/ QBrush_isDetached<i8> for () {
  fn isDetached(self , rsthis: &mut QBrush) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush10isDetachedEv()};
    let mut ret = unsafe {_ZNK6QBrush10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QBrush::swap(QBrush & other);
impl /*struct*/ QBrush {
  pub fn swap<RetType, T: QBrush_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QBrush_swap<RetType> {
  fn swap(self , rsthis: &mut QBrush) -> RetType;
}

  // proto:  void QBrush::swap(QBrush & other);
impl<'a> /*trait*/ QBrush_swap<()> for (QBrush) {
  fn swap(self , rsthis: &mut QBrush) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrush4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QBrush4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QColor & QBrush::color();
impl /*struct*/ QBrush {
  pub fn color<RetType, T: QBrush_color<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.color(self);
    // return 1;
  }
}

pub trait QBrush_color<RetType> {
  fn color(self , rsthis: &mut QBrush) -> RetType;
}

  // proto:  const QColor & QBrush::color();
impl<'a> /*trait*/ QBrush_color<QColor> for () {
  fn color(self , rsthis: &mut QBrush) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush5colorEv()};
    let mut ret = unsafe {_ZNK6QBrush5colorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QBrush::QBrush(const QImage & image);
impl<'a> /*trait*/ QBrush_NewQBrush for (QImage) {
  fn NewQBrush(self) -> QBrush {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC1ERK6QImage()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QBrushC1ERK6QImage(qthis, arg0)};
    let rsthis = QBrush{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGradient::setColorAt(qreal pos, const QColor & color);
impl /*struct*/ QGradient {
  pub fn setColorAt<RetType, T: QGradient_setColorAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setColorAt(self);
    // return 1;
  }
}

pub trait QGradient_setColorAt<RetType> {
  fn setColorAt(self , rsthis: &mut QGradient) -> RetType;
}

  // proto:  void QGradient::setColorAt(qreal pos, const QColor & color);
impl<'a> /*trait*/ QGradient_setColorAt<()> for (f64, QColor) {
  fn setColorAt(self , rsthis: &mut QGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGradient10setColorAtEdRK6QColor()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QGradient10setColorAtEdRK6QColor(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QGradientStops QGradient::stops();
impl /*struct*/ QGradient {
  pub fn stops<RetType, T: QGradient_stops<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.stops(self);
    // return 1;
  }
}

pub trait QGradient_stops<RetType> {
  fn stops(self , rsthis: &mut QGradient) -> RetType;
}

  // proto:  QGradientStops QGradient::stops();
impl<'a> /*trait*/ QGradient_stops<()> for () {
  fn stops(self , rsthis: &mut QGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGradient5stopsEv()};
     unsafe {_ZNK9QGradient5stopsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGradient::QGradient();
impl /*struct*/ QGradient {
  pub fn NewQGradient<T: QGradient_NewQGradient>(value: T) -> QGradient {
    let rsthis = value.NewQGradient();
    return rsthis;
    // return 1;
  }
}

pub trait QGradient_NewQGradient {
  fn NewQGradient(self) -> QGradient;
}

  // proto:  void QGradient::QGradient();
impl<'a> /*trait*/ QGradient_NewQGradient for () {
  fn NewQGradient(self) -> QGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGradientC1Ev()};
    unsafe {_ZN9QGradientC1Ev(qthis)};
    let rsthis = QGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QLinearGradient::setFinalStop(qreal x, qreal y);
impl /*struct*/ QLinearGradient {
  pub fn setFinalStop<RetType, T: QLinearGradient_setFinalStop<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFinalStop(self);
    // return 1;
  }
}

pub trait QLinearGradient_setFinalStop<RetType> {
  fn setFinalStop(self , rsthis: &mut QLinearGradient) -> RetType;
}

  // proto:  void QLinearGradient::setFinalStop(qreal x, qreal y);
impl<'a> /*trait*/ QLinearGradient_setFinalStop<()> for (f64, f64) {
  fn setFinalStop(self , rsthis: &mut QLinearGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradient12setFinalStopEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN15QLinearGradient12setFinalStopEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QPointF QLinearGradient::start();
impl /*struct*/ QLinearGradient {
  pub fn start<RetType, T: QLinearGradient_start<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.start(self);
    // return 1;
  }
}

pub trait QLinearGradient_start<RetType> {
  fn start(self , rsthis: &mut QLinearGradient) -> RetType;
}

  // proto:  QPointF QLinearGradient::start();
impl<'a> /*trait*/ QLinearGradient_start<QPointF> for () {
  fn start(self , rsthis: &mut QLinearGradient) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QLinearGradient5startEv()};
    let mut ret = unsafe {_ZNK15QLinearGradient5startEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QLinearGradient::QLinearGradient(qreal xStart, qreal yStart, qreal xFinalStop, qreal yFinalStop);
impl /*struct*/ QLinearGradient {
  pub fn NewQLinearGradient<T: QLinearGradient_NewQLinearGradient>(value: T) -> QLinearGradient {
    let rsthis = value.NewQLinearGradient();
    return rsthis;
    // return 1;
  }
}

pub trait QLinearGradient_NewQLinearGradient {
  fn NewQLinearGradient(self) -> QLinearGradient;
}

  // proto:  void QLinearGradient::QLinearGradient(qreal xStart, qreal yStart, qreal xFinalStop, qreal yFinalStop);
impl<'a> /*trait*/ QLinearGradient_NewQLinearGradient for (f64, f64, f64, f64) {
  fn NewQLinearGradient(self) -> QLinearGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradientC1Edddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN15QLinearGradientC1Edddd(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QLinearGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QLinearGradient::QLinearGradient(const QPointF & start, const QPointF & finalStop);
impl<'a> /*trait*/ QLinearGradient_NewQLinearGradient for (QPointF, QPointF) {
  fn NewQLinearGradient(self) -> QLinearGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradientC1ERK7QPointFS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN15QLinearGradientC1ERK7QPointFS2_(qthis, arg0, arg1)};
    let rsthis = QLinearGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QLinearGradient::setStart(qreal x, qreal y);
impl /*struct*/ QLinearGradient {
  pub fn setStart<RetType, T: QLinearGradient_setStart<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setStart(self);
    // return 1;
  }
}

pub trait QLinearGradient_setStart<RetType> {
  fn setStart(self , rsthis: &mut QLinearGradient) -> RetType;
}

  // proto:  void QLinearGradient::setStart(qreal x, qreal y);
impl<'a> /*trait*/ QLinearGradient_setStart<()> for (f64, f64) {
  fn setStart(self , rsthis: &mut QLinearGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradient8setStartEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN15QLinearGradient8setStartEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QLinearGradient::setStart(const QPointF & start);
impl<'a> /*trait*/ QLinearGradient_setStart<()> for (QPointF) {
  fn setStart(self , rsthis: &mut QLinearGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradient8setStartERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QLinearGradient8setStartERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLinearGradient::QLinearGradient();
impl<'a> /*trait*/ QLinearGradient_NewQLinearGradient for () {
  fn NewQLinearGradient(self) -> QLinearGradient {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradientC1Ev()};
    unsafe {_ZN15QLinearGradientC1Ev(qthis)};
    let rsthis = QLinearGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPointF QLinearGradient::finalStop();
impl /*struct*/ QLinearGradient {
  pub fn finalStop<RetType, T: QLinearGradient_finalStop<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.finalStop(self);
    // return 1;
  }
}

pub trait QLinearGradient_finalStop<RetType> {
  fn finalStop(self , rsthis: &mut QLinearGradient) -> RetType;
}

  // proto:  QPointF QLinearGradient::finalStop();
impl<'a> /*trait*/ QLinearGradient_finalStop<QPointF> for () {
  fn finalStop(self , rsthis: &mut QLinearGradient) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QLinearGradient9finalStopEv()};
    let mut ret = unsafe {_ZNK15QLinearGradient9finalStopEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QLinearGradient::setFinalStop(const QPointF & stop);
impl<'a> /*trait*/ QLinearGradient_setFinalStop<()> for (QPointF) {
  fn setFinalStop(self , rsthis: &mut QLinearGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradient12setFinalStopERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QLinearGradient12setFinalStopERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

