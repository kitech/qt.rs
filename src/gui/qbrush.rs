// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
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
// use super::qbrush::QGradient; // 773
use std::ops::Deref;
use super::super::core::qpoint::QPointF; // 771
use super::qpixmap::QPixmap; // 773
use super::qimage::QImage; // 773
use super::qcolor::QColor; // 773
use super::qtransform::QTransform; // 773
use super::qmatrix::QMatrix; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QRadialGradient_Class_Size() -> c_int;
  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal radius, qreal fx, qreal fy);
  fn dector_ZN15QRadialGradientC1Eddddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> *mut c_void;
  fn _ZN15QRadialGradientC1Eddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double);
  // proto:  void QRadialGradient::setFocalPoint(qreal x, qreal y);
  fn _ZN15QRadialGradient13setFocalPointEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QRadialGradient::QRadialGradient();
  fn dector_ZN15QRadialGradientC1Ev() -> *mut c_void;
  fn _ZN15QRadialGradientC1Ev(qthis: *mut c_void);
  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal radius, const QPointF & focalPoint);
  fn dector_ZN15QRadialGradientC1ERK7QPointFdS2_(arg0: *mut c_void, arg1: c_double, arg2: *mut c_void) -> *mut c_void;
  fn _ZN15QRadialGradientC1ERK7QPointFdS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: *mut c_void);
  // proto:  qreal QRadialGradient::radius();
  fn _ZNK15QRadialGradient6radiusEv(qthis: *mut c_void) -> c_double;
  // proto:  void QRadialGradient::setFocalPoint(const QPointF & focalPoint);
  fn _ZN15QRadialGradient13setFocalPointERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal centerRadius, const QPointF & focalPoint, qreal focalRadius);
  fn dector_ZN15QRadialGradientC1ERK7QPointFdS2_d(arg0: *mut c_void, arg1: c_double, arg2: *mut c_void, arg3: c_double) -> *mut c_void;
  fn _ZN15QRadialGradientC1ERK7QPointFdS2_d(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: *mut c_void, arg3: c_double);
  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal centerRadius, qreal fx, qreal fy, qreal focalRadius);
  fn dector_ZN15QRadialGradientC1Edddddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double) -> *mut c_void;
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
  fn dector_ZN15QRadialGradientC1ERK7QPointFd(arg0: *mut c_void, arg1: c_double) -> *mut c_void;
  fn _ZN15QRadialGradientC1ERK7QPointFd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double);
  // proto:  void QRadialGradient::setCenterRadius(qreal radius);
  fn _ZN15QRadialGradient15setCenterRadiusEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QRadialGradient::setFocalRadius(qreal radius);
  fn _ZN15QRadialGradient14setFocalRadiusEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QRadialGradient::setRadius(qreal radius);
  fn _ZN15QRadialGradient9setRadiusEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal radius);
  fn dector_ZN15QRadialGradientC1Eddd(arg0: c_double, arg1: c_double, arg2: c_double) -> *mut c_void;
  fn _ZN15QRadialGradientC1Eddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double);
  // proto:  void QRadialGradient::setCenter(qreal x, qreal y);
  fn _ZN15QRadialGradient9setCenterEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  fn QConicalGradient_Class_Size() -> c_int;
  // proto:  qreal QConicalGradient::angle();
  fn _ZNK16QConicalGradient5angleEv(qthis: *mut c_void) -> c_double;
  // proto:  QPointF QConicalGradient::center();
  fn _ZNK16QConicalGradient6centerEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QConicalGradient::QConicalGradient(const QPointF & center, qreal startAngle);
  fn dector_ZN16QConicalGradientC1ERK7QPointFd(arg0: *mut c_void, arg1: c_double) -> *mut c_void;
  fn _ZN16QConicalGradientC1ERK7QPointFd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double);
  // proto:  void QConicalGradient::QConicalGradient();
  fn dector_ZN16QConicalGradientC1Ev() -> *mut c_void;
  fn _ZN16QConicalGradientC1Ev(qthis: *mut c_void);
  // proto:  void QConicalGradient::setAngle(qreal angle);
  fn _ZN16QConicalGradient8setAngleEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QConicalGradient::setCenter(qreal x, qreal y);
  fn _ZN16QConicalGradient9setCenterEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QConicalGradient::setCenter(const QPointF & center);
  fn _ZN16QConicalGradient9setCenterERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QConicalGradient::QConicalGradient(qreal cx, qreal cy, qreal startAngle);
  fn dector_ZN16QConicalGradientC1Eddd(arg0: c_double, arg1: c_double, arg2: c_double) -> *mut c_void;
  fn _ZN16QConicalGradientC1Eddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double);
  fn QBrush_Class_Size() -> c_int;
  // proto:  void QBrush::QBrush();
  fn dector_ZN6QBrushC1Ev() -> *mut c_void;
  fn _ZN6QBrushC1Ev(qthis: *mut c_void);
  // proto:  void QBrush::QBrush(const QPixmap & pixmap);
  fn dector_ZN6QBrushC1ERK7QPixmap(arg0: *mut c_void) -> *mut c_void;
  fn _ZN6QBrushC1ERK7QPixmap(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QBrush::setTexture(const QPixmap & pixmap);
  fn _ZN6QBrush10setTextureERK7QPixmap(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QBrush::setTextureImage(const QImage & image);
  fn _ZN6QBrush15setTextureImageERK6QImage(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QBrush::QBrush(const QColor & color, const QPixmap & pixmap);
  fn dector_ZN6QBrushC1ERK6QColorRK7QPixmap(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN6QBrushC1ERK6QColorRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QPixmap QBrush::texture();
  fn _ZNK6QBrush7textureEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QBrush::QBrush(const QGradient & gradient);
  fn dector_ZN6QBrushC1ERK9QGradient(arg0: *mut c_void) -> *mut c_void;
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
  fn dector_ZN6QBrushC1ERKS_(arg0: *mut c_void) -> *mut c_void;
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
  fn dector_ZN6QBrushC1ERK6QImage(arg0: *mut c_void) -> *mut c_void;
  fn _ZN6QBrushC1ERK6QImage(qthis: *mut c_void, arg0: *mut c_void);
  fn QGradient_Class_Size() -> c_int;
  // proto:  void QGradient::setColorAt(qreal pos, const QColor & color);
  fn _ZN9QGradient10setColorAtEdRK6QColor(qthis: *mut c_void, arg0: c_double, arg1: *mut c_void);
  // proto:  QGradientStops QGradient::stops();
  fn _ZNK9QGradient5stopsEv(qthis: *mut c_void);
  // proto:  void QGradient::QGradient();
  fn dector_ZN9QGradientC1Ev() -> *mut c_void;
  fn _ZN9QGradientC1Ev(qthis: *mut c_void);
  fn QBrushData_Class_Size() -> c_int;
  fn QLinearGradient_Class_Size() -> c_int;
  // proto:  void QLinearGradient::setFinalStop(qreal x, qreal y);
  fn _ZN15QLinearGradient12setFinalStopEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  QPointF QLinearGradient::start();
  fn _ZNK15QLinearGradient5startEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLinearGradient::QLinearGradient(qreal xStart, qreal yStart, qreal xFinalStop, qreal yFinalStop);
  fn dector_ZN15QLinearGradientC1Edddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  fn _ZN15QLinearGradientC1Edddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  void QLinearGradient::QLinearGradient(const QPointF & start, const QPointF & finalStop);
  fn dector_ZN15QLinearGradientC1ERK7QPointFS2_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN15QLinearGradientC1ERK7QPointFS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QLinearGradient::setStart(qreal x, qreal y);
  fn _ZN15QLinearGradient8setStartEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QLinearGradient::setStart(const QPointF & start);
  fn _ZN15QLinearGradient8setStartERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QLinearGradient::QLinearGradient();
  fn dector_ZN15QLinearGradientC1Ev() -> *mut c_void;
  fn _ZN15QLinearGradientC1Ev(qthis: *mut c_void);
  // proto:  QPointF QLinearGradient::finalStop();
  fn _ZNK15QLinearGradient9finalStopEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLinearGradient::setFinalStop(const QPointF & stop);
  fn _ZN15QLinearGradient12setFinalStopERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QRadialGradient)=1
pub struct QRadialGradient {
  qbase: QGradient,
  pub qclsinst: *mut c_void,
}

// class sizeof(QConicalGradient)=1
pub struct QConicalGradient {
  qbase: QGradient,
  pub qclsinst: *mut c_void,
}

// class sizeof(QBrush)=1
pub struct QBrush {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QGradient)=1
pub struct QGradient {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QBrushData)=1
pub struct QBrushData {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QLinearGradient)=1
pub struct QLinearGradient {
  qbase: QGradient,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRadialGradient {
  pub fn inheritFrom(qthis: *mut c_void) -> QRadialGradient {
    return QRadialGradient{qbase: QGradient::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QRadialGradient {
  type Target = QGradient;

  fn deref(&self) -> &QGradient {
    return & self.qbase;
  }
}
impl AsRef<QGradient> for QRadialGradient {
  fn as_ref(& self) -> & QGradient {
    return & self.qbase;
  }
}
  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal radius, qreal fx, qreal fy);
impl /*struct*/ QRadialGradient {
  pub fn New<T: QRadialGradient_New>(value: T) -> QRadialGradient {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QRadialGradient_New {
  fn New(self) -> QRadialGradient;
}

  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal radius, qreal fx, qreal fy);
impl<'a> /*trait*/ QRadialGradient_New for (f64, f64, f64, f64, f64) {
  fn New(self) -> QRadialGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1Eddddd()};
    let ctysz: c_int = unsafe{QRadialGradient_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    // unsafe {_ZN15QRadialGradientC1Eddddd(qthis, arg0, arg1, arg2, arg3, arg4)};
    let qthis: *mut c_void = unsafe {dector_ZN15QRadialGradientC1Eddddd(arg0, arg1, arg2, arg3, arg4)};
    let rsthis = QRadialGradient{/**/qbase: QGradient::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRadialGradient::setFocalPoint(qreal x, qreal y);
impl /*struct*/ QRadialGradient {
  pub fn setFocalPoint<RetType, T: QRadialGradient_setFocalPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFocalPoint(self);
    // return 1;
  }
}

pub trait QRadialGradient_setFocalPoint<RetType> {
  fn setFocalPoint(self , rsthis: & QRadialGradient) -> RetType;
}

  // proto:  void QRadialGradient::setFocalPoint(qreal x, qreal y);
impl<'a> /*trait*/ QRadialGradient_setFocalPoint<()> for (f64, f64) {
  fn setFocalPoint(self , rsthis: & QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient13setFocalPointEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN15QRadialGradient13setFocalPointEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient();
impl<'a> /*trait*/ QRadialGradient_New for () {
  fn New(self) -> QRadialGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1Ev()};
    let ctysz: c_int = unsafe{QRadialGradient_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN15QRadialGradientC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN15QRadialGradientC1Ev()};
    let rsthis = QRadialGradient{/**/qbase: QGradient::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal radius, const QPointF & focalPoint);
impl<'a> /*trait*/ QRadialGradient_New for (&'a QPointF, f64, &'a QPointF) {
  fn New(self) -> QRadialGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1ERK7QPointFdS2_()};
    let ctysz: c_int = unsafe{QRadialGradient_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.qclsinst  as *mut c_void;
    // unsafe {_ZN15QRadialGradientC1ERK7QPointFdS2_(qthis, arg0, arg1, arg2)};
    let qthis: *mut c_void = unsafe {dector_ZN15QRadialGradientC1ERK7QPointFdS2_(arg0, arg1, arg2)};
    let rsthis = QRadialGradient{/**/qbase: QGradient::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QRadialGradient::radius();
impl /*struct*/ QRadialGradient {
  pub fn radius<RetType, T: QRadialGradient_radius<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.radius(self);
    // return 1;
  }
}

pub trait QRadialGradient_radius<RetType> {
  fn radius(self , rsthis: & QRadialGradient) -> RetType;
}

  // proto:  qreal QRadialGradient::radius();
impl<'a> /*trait*/ QRadialGradient_radius<f64> for () {
  fn radius(self , rsthis: & QRadialGradient) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient6radiusEv()};
    let mut ret = unsafe {_ZNK15QRadialGradient6radiusEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QRadialGradient::setFocalPoint(const QPointF & focalPoint);
impl<'a> /*trait*/ QRadialGradient_setFocalPoint<()> for (&'a QPointF) {
  fn setFocalPoint(self , rsthis: & QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient13setFocalPointERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QRadialGradient13setFocalPointERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal centerRadius, const QPointF & focalPoint, qreal focalRadius);
impl<'a> /*trait*/ QRadialGradient_New for (&'a QPointF, f64, &'a QPointF, f64) {
  fn New(self) -> QRadialGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1ERK7QPointFdS2_d()};
    let ctysz: c_int = unsafe{QRadialGradient_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_double;
    // unsafe {_ZN15QRadialGradientC1ERK7QPointFdS2_d(qthis, arg0, arg1, arg2, arg3)};
    let qthis: *mut c_void = unsafe {dector_ZN15QRadialGradientC1ERK7QPointFdS2_d(arg0, arg1, arg2, arg3)};
    let rsthis = QRadialGradient{/**/qbase: QGradient::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal centerRadius, qreal fx, qreal fy, qreal focalRadius);
impl<'a> /*trait*/ QRadialGradient_New for (f64, f64, f64, f64, f64, f64) {
  fn New(self) -> QRadialGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1Edddddd()};
    let ctysz: c_int = unsafe{QRadialGradient_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
    // unsafe {_ZN15QRadialGradientC1Edddddd(qthis, arg0, arg1, arg2, arg3, arg4, arg5)};
    let qthis: *mut c_void = unsafe {dector_ZN15QRadialGradientC1Edddddd(arg0, arg1, arg2, arg3, arg4, arg5)};
    let rsthis = QRadialGradient{/**/qbase: QGradient::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QRadialGradient::centerRadius();
impl /*struct*/ QRadialGradient {
  pub fn centerRadius<RetType, T: QRadialGradient_centerRadius<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.centerRadius(self);
    // return 1;
  }
}

pub trait QRadialGradient_centerRadius<RetType> {
  fn centerRadius(self , rsthis: & QRadialGradient) -> RetType;
}

  // proto:  qreal QRadialGradient::centerRadius();
impl<'a> /*trait*/ QRadialGradient_centerRadius<f64> for () {
  fn centerRadius(self , rsthis: & QRadialGradient) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient12centerRadiusEv()};
    let mut ret = unsafe {_ZNK15QRadialGradient12centerRadiusEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QPointF QRadialGradient::focalPoint();
impl /*struct*/ QRadialGradient {
  pub fn focalPoint<RetType, T: QRadialGradient_focalPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.focalPoint(self);
    // return 1;
  }
}

pub trait QRadialGradient_focalPoint<RetType> {
  fn focalPoint(self , rsthis: & QRadialGradient) -> RetType;
}

  // proto:  QPointF QRadialGradient::focalPoint();
impl<'a> /*trait*/ QRadialGradient_focalPoint<QPointF> for () {
  fn focalPoint(self , rsthis: & QRadialGradient) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient10focalPointEv()};
    let mut ret = unsafe {_ZNK15QRadialGradient10focalPointEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QRadialGradient::focalRadius();
impl /*struct*/ QRadialGradient {
  pub fn focalRadius<RetType, T: QRadialGradient_focalRadius<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.focalRadius(self);
    // return 1;
  }
}

pub trait QRadialGradient_focalRadius<RetType> {
  fn focalRadius(self , rsthis: & QRadialGradient) -> RetType;
}

  // proto:  qreal QRadialGradient::focalRadius();
impl<'a> /*trait*/ QRadialGradient_focalRadius<f64> for () {
  fn focalRadius(self , rsthis: & QRadialGradient) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient11focalRadiusEv()};
    let mut ret = unsafe {_ZNK15QRadialGradient11focalRadiusEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QPointF QRadialGradient::center();
impl /*struct*/ QRadialGradient {
  pub fn center<RetType, T: QRadialGradient_center<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.center(self);
    // return 1;
  }
}

pub trait QRadialGradient_center<RetType> {
  fn center(self , rsthis: & QRadialGradient) -> RetType;
}

  // proto:  QPointF QRadialGradient::center();
impl<'a> /*trait*/ QRadialGradient_center<QPointF> for () {
  fn center(self , rsthis: & QRadialGradient) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QRadialGradient6centerEv()};
    let mut ret = unsafe {_ZNK15QRadialGradient6centerEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRadialGradient::setCenter(const QPointF & center);
impl /*struct*/ QRadialGradient {
  pub fn setCenter<RetType, T: QRadialGradient_setCenter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCenter(self);
    // return 1;
  }
}

pub trait QRadialGradient_setCenter<RetType> {
  fn setCenter(self , rsthis: & QRadialGradient) -> RetType;
}

  // proto:  void QRadialGradient::setCenter(const QPointF & center);
impl<'a> /*trait*/ QRadialGradient_setCenter<()> for (&'a QPointF) {
  fn setCenter(self , rsthis: & QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient9setCenterERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QRadialGradient9setCenterERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal radius);
impl<'a> /*trait*/ QRadialGradient_New for (&'a QPointF, f64) {
  fn New(self) -> QRadialGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1ERK7QPointFd()};
    let ctysz: c_int = unsafe{QRadialGradient_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    // unsafe {_ZN15QRadialGradientC1ERK7QPointFd(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN15QRadialGradientC1ERK7QPointFd(arg0, arg1)};
    let rsthis = QRadialGradient{/**/qbase: QGradient::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRadialGradient::setCenterRadius(qreal radius);
impl /*struct*/ QRadialGradient {
  pub fn setCenterRadius<RetType, T: QRadialGradient_setCenterRadius<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCenterRadius(self);
    // return 1;
  }
}

pub trait QRadialGradient_setCenterRadius<RetType> {
  fn setCenterRadius(self , rsthis: & QRadialGradient) -> RetType;
}

  // proto:  void QRadialGradient::setCenterRadius(qreal radius);
impl<'a> /*trait*/ QRadialGradient_setCenterRadius<()> for (f64) {
  fn setCenterRadius(self , rsthis: & QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient15setCenterRadiusEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QRadialGradient15setCenterRadiusEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::setFocalRadius(qreal radius);
impl /*struct*/ QRadialGradient {
  pub fn setFocalRadius<RetType, T: QRadialGradient_setFocalRadius<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFocalRadius(self);
    // return 1;
  }
}

pub trait QRadialGradient_setFocalRadius<RetType> {
  fn setFocalRadius(self , rsthis: & QRadialGradient) -> RetType;
}

  // proto:  void QRadialGradient::setFocalRadius(qreal radius);
impl<'a> /*trait*/ QRadialGradient_setFocalRadius<()> for (f64) {
  fn setFocalRadius(self , rsthis: & QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient14setFocalRadiusEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QRadialGradient14setFocalRadiusEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::setRadius(qreal radius);
impl /*struct*/ QRadialGradient {
  pub fn setRadius<RetType, T: QRadialGradient_setRadius<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRadius(self);
    // return 1;
  }
}

pub trait QRadialGradient_setRadius<RetType> {
  fn setRadius(self , rsthis: & QRadialGradient) -> RetType;
}

  // proto:  void QRadialGradient::setRadius(qreal radius);
impl<'a> /*trait*/ QRadialGradient_setRadius<()> for (f64) {
  fn setRadius(self , rsthis: & QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient9setRadiusEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QRadialGradient9setRadiusEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal radius);
impl<'a> /*trait*/ QRadialGradient_New for (f64, f64, f64) {
  fn New(self) -> QRadialGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC1Eddd()};
    let ctysz: c_int = unsafe{QRadialGradient_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    // unsafe {_ZN15QRadialGradientC1Eddd(qthis, arg0, arg1, arg2)};
    let qthis: *mut c_void = unsafe {dector_ZN15QRadialGradientC1Eddd(arg0, arg1, arg2)};
    let rsthis = QRadialGradient{/**/qbase: QGradient::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRadialGradient::setCenter(qreal x, qreal y);
impl<'a> /*trait*/ QRadialGradient_setCenter<()> for (f64, f64) {
  fn setCenter(self , rsthis: & QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient9setCenterEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN15QRadialGradient9setCenterEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QConicalGradient {
  pub fn inheritFrom(qthis: *mut c_void) -> QConicalGradient {
    return QConicalGradient{qbase: QGradient::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QConicalGradient {
  type Target = QGradient;

  fn deref(&self) -> &QGradient {
    return & self.qbase;
  }
}
impl AsRef<QGradient> for QConicalGradient {
  fn as_ref(& self) -> & QGradient {
    return & self.qbase;
  }
}
  // proto:  qreal QConicalGradient::angle();
impl /*struct*/ QConicalGradient {
  pub fn angle<RetType, T: QConicalGradient_angle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.angle(self);
    // return 1;
  }
}

pub trait QConicalGradient_angle<RetType> {
  fn angle(self , rsthis: & QConicalGradient) -> RetType;
}

  // proto:  qreal QConicalGradient::angle();
impl<'a> /*trait*/ QConicalGradient_angle<f64> for () {
  fn angle(self , rsthis: & QConicalGradient) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QConicalGradient5angleEv()};
    let mut ret = unsafe {_ZNK16QConicalGradient5angleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QPointF QConicalGradient::center();
impl /*struct*/ QConicalGradient {
  pub fn center<RetType, T: QConicalGradient_center<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.center(self);
    // return 1;
  }
}

pub trait QConicalGradient_center<RetType> {
  fn center(self , rsthis: & QConicalGradient) -> RetType;
}

  // proto:  QPointF QConicalGradient::center();
impl<'a> /*trait*/ QConicalGradient_center<QPointF> for () {
  fn center(self , rsthis: & QConicalGradient) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QConicalGradient6centerEv()};
    let mut ret = unsafe {_ZNK16QConicalGradient6centerEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QConicalGradient::QConicalGradient(const QPointF & center, qreal startAngle);
impl /*struct*/ QConicalGradient {
  pub fn New<T: QConicalGradient_New>(value: T) -> QConicalGradient {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QConicalGradient_New {
  fn New(self) -> QConicalGradient;
}

  // proto:  void QConicalGradient::QConicalGradient(const QPointF & center, qreal startAngle);
impl<'a> /*trait*/ QConicalGradient_New for (&'a QPointF, f64) {
  fn New(self) -> QConicalGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradientC1ERK7QPointFd()};
    let ctysz: c_int = unsafe{QConicalGradient_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    // unsafe {_ZN16QConicalGradientC1ERK7QPointFd(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN16QConicalGradientC1ERK7QPointFd(arg0, arg1)};
    let rsthis = QConicalGradient{/**/qbase: QGradient::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QConicalGradient::QConicalGradient();
impl<'a> /*trait*/ QConicalGradient_New for () {
  fn New(self) -> QConicalGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradientC1Ev()};
    let ctysz: c_int = unsafe{QConicalGradient_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN16QConicalGradientC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN16QConicalGradientC1Ev()};
    let rsthis = QConicalGradient{/**/qbase: QGradient::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QConicalGradient::setAngle(qreal angle);
impl /*struct*/ QConicalGradient {
  pub fn setAngle<RetType, T: QConicalGradient_setAngle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAngle(self);
    // return 1;
  }
}

pub trait QConicalGradient_setAngle<RetType> {
  fn setAngle(self , rsthis: & QConicalGradient) -> RetType;
}

  // proto:  void QConicalGradient::setAngle(qreal angle);
impl<'a> /*trait*/ QConicalGradient_setAngle<()> for (f64) {
  fn setAngle(self , rsthis: & QConicalGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradient8setAngleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QConicalGradient8setAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QConicalGradient::setCenter(qreal x, qreal y);
impl /*struct*/ QConicalGradient {
  pub fn setCenter<RetType, T: QConicalGradient_setCenter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCenter(self);
    // return 1;
  }
}

pub trait QConicalGradient_setCenter<RetType> {
  fn setCenter(self , rsthis: & QConicalGradient) -> RetType;
}

  // proto:  void QConicalGradient::setCenter(qreal x, qreal y);
impl<'a> /*trait*/ QConicalGradient_setCenter<()> for (f64, f64) {
  fn setCenter(self , rsthis: & QConicalGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradient9setCenterEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN16QConicalGradient9setCenterEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QConicalGradient::setCenter(const QPointF & center);
impl<'a> /*trait*/ QConicalGradient_setCenter<()> for (&'a QPointF) {
  fn setCenter(self , rsthis: & QConicalGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradient9setCenterERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QConicalGradient9setCenterERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QConicalGradient::QConicalGradient(qreal cx, qreal cy, qreal startAngle);
impl<'a> /*trait*/ QConicalGradient_New for (f64, f64, f64) {
  fn New(self) -> QConicalGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradientC1Eddd()};
    let ctysz: c_int = unsafe{QConicalGradient_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    // unsafe {_ZN16QConicalGradientC1Eddd(qthis, arg0, arg1, arg2)};
    let qthis: *mut c_void = unsafe {dector_ZN16QConicalGradientC1Eddd(arg0, arg1, arg2)};
    let rsthis = QConicalGradient{/**/qbase: QGradient::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBrush {
  pub fn inheritFrom(qthis: *mut c_void) -> QBrush {
    return QBrush{qclsinst: qthis};
  }
}
  // proto:  void QBrush::QBrush();
impl /*struct*/ QBrush {
  pub fn New<T: QBrush_New>(value: T) -> QBrush {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QBrush_New {
  fn New(self) -> QBrush;
}

  // proto:  void QBrush::QBrush();
impl<'a> /*trait*/ QBrush_New for () {
  fn New(self) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC1Ev()};
    let ctysz: c_int = unsafe{QBrush_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN6QBrushC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN6QBrushC1Ev()};
    let rsthis = QBrush{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QBrush::QBrush(const QPixmap & pixmap);
impl<'a> /*trait*/ QBrush_New for (&'a QPixmap) {
  fn New(self) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC1ERK7QPixmap()};
    let ctysz: c_int = unsafe{QBrush_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN6QBrushC1ERK7QPixmap(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN6QBrushC1ERK7QPixmap(arg0)};
    let rsthis = QBrush{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QBrush::setTexture(const QPixmap & pixmap);
impl /*struct*/ QBrush {
  pub fn setTexture<RetType, T: QBrush_setTexture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTexture(self);
    // return 1;
  }
}

pub trait QBrush_setTexture<RetType> {
  fn setTexture(self , rsthis: & QBrush) -> RetType;
}

  // proto:  void QBrush::setTexture(const QPixmap & pixmap);
impl<'a> /*trait*/ QBrush_setTexture<()> for (&'a QPixmap) {
  fn setTexture(self , rsthis: & QBrush) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrush10setTextureERK7QPixmap()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QBrush10setTextureERK7QPixmap(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QBrush::setTextureImage(const QImage & image);
impl /*struct*/ QBrush {
  pub fn setTextureImage<RetType, T: QBrush_setTextureImage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextureImage(self);
    // return 1;
  }
}

pub trait QBrush_setTextureImage<RetType> {
  fn setTextureImage(self , rsthis: & QBrush) -> RetType;
}

  // proto:  void QBrush::setTextureImage(const QImage & image);
impl<'a> /*trait*/ QBrush_setTextureImage<()> for (&'a QImage) {
  fn setTextureImage(self , rsthis: & QBrush) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrush15setTextureImageERK6QImage()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QBrush15setTextureImageERK6QImage(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QBrush::QBrush(const QColor & color, const QPixmap & pixmap);
impl<'a> /*trait*/ QBrush_New for (&'a QColor, &'a QPixmap) {
  fn New(self) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC1ERK6QColorRK7QPixmap()};
    let ctysz: c_int = unsafe{QBrush_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN6QBrushC1ERK6QColorRK7QPixmap(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN6QBrushC1ERK6QColorRK7QPixmap(arg0, arg1)};
    let rsthis = QBrush{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPixmap QBrush::texture();
impl /*struct*/ QBrush {
  pub fn texture<RetType, T: QBrush_texture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.texture(self);
    // return 1;
  }
}

pub trait QBrush_texture<RetType> {
  fn texture(self , rsthis: & QBrush) -> RetType;
}

  // proto:  QPixmap QBrush::texture();
impl<'a> /*trait*/ QBrush_texture<QPixmap> for () {
  fn texture(self , rsthis: & QBrush) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush7textureEv()};
    let mut ret = unsafe {_ZNK6QBrush7textureEv(rsthis.qclsinst)};
    let mut ret1 = QPixmap::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QBrush::QBrush(const QGradient & gradient);
impl<'a> /*trait*/ QBrush_New for (&'a QGradient) {
  fn New(self) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC1ERK9QGradient()};
    let ctysz: c_int = unsafe{QBrush_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN6QBrushC1ERK9QGradient(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN6QBrushC1ERK9QGradient(arg0)};
    let rsthis = QBrush{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QTransform QBrush::transform();
impl /*struct*/ QBrush {
  pub fn transform<RetType, T: QBrush_transform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.transform(self);
    // return 1;
  }
}

pub trait QBrush_transform<RetType> {
  fn transform(self , rsthis: & QBrush) -> RetType;
}

  // proto:  QTransform QBrush::transform();
impl<'a> /*trait*/ QBrush_transform<QTransform> for () {
  fn transform(self , rsthis: & QBrush) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush9transformEv()};
    let mut ret = unsafe {_ZNK6QBrush9transformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QBrush::setTransform(const QTransform & );
impl /*struct*/ QBrush {
  pub fn setTransform<RetType, T: QBrush_setTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTransform(self);
    // return 1;
  }
}

pub trait QBrush_setTransform<RetType> {
  fn setTransform(self , rsthis: & QBrush) -> RetType;
}

  // proto:  void QBrush::setTransform(const QTransform & );
impl<'a> /*trait*/ QBrush_setTransform<()> for (&'a QTransform) {
  fn setTransform(self , rsthis: & QBrush) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrush12setTransformERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QBrush12setTransformERK10QTransform(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QBrush::isOpaque();
impl /*struct*/ QBrush {
  pub fn isOpaque<RetType, T: QBrush_isOpaque<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isOpaque(self);
    // return 1;
  }
}

pub trait QBrush_isOpaque<RetType> {
  fn isOpaque(self , rsthis: & QBrush) -> RetType;
}

  // proto:  bool QBrush::isOpaque();
impl<'a> /*trait*/ QBrush_isOpaque<i8> for () {
  fn isOpaque(self , rsthis: & QBrush) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush8isOpaqueEv()};
    let mut ret = unsafe {_ZNK6QBrush8isOpaqueEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QGradient * QBrush::gradient();
impl /*struct*/ QBrush {
  pub fn gradient<RetType, T: QBrush_gradient<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.gradient(self);
    // return 1;
  }
}

pub trait QBrush_gradient<RetType> {
  fn gradient(self , rsthis: & QBrush) -> RetType;
}

  // proto:  const QGradient * QBrush::gradient();
impl<'a> /*trait*/ QBrush_gradient<QGradient> for () {
  fn gradient(self , rsthis: & QBrush) -> QGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush8gradientEv()};
    let mut ret = unsafe {_ZNK6QBrush8gradientEv(rsthis.qclsinst)};
    let mut ret1 = QGradient::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QBrush::~QBrush();
impl /*struct*/ QBrush {
  pub fn Free<RetType, T: QBrush_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QBrush_Free<RetType> {
  fn Free(self , rsthis: & QBrush) -> RetType;
}

  // proto:  void QBrush::~QBrush();
impl<'a> /*trait*/ QBrush_Free<()> for () {
  fn Free(self , rsthis: & QBrush) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushD0Ev()};
     unsafe {_ZN6QBrushD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QBrush::setMatrix(const QMatrix & mat);
impl /*struct*/ QBrush {
  pub fn setMatrix<RetType, T: QBrush_setMatrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMatrix(self);
    // return 1;
  }
}

pub trait QBrush_setMatrix<RetType> {
  fn setMatrix(self , rsthis: & QBrush) -> RetType;
}

  // proto:  void QBrush::setMatrix(const QMatrix & mat);
impl<'a> /*trait*/ QBrush_setMatrix<()> for (&'a QMatrix) {
  fn setMatrix(self , rsthis: & QBrush) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrush9setMatrixERK7QMatrix()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QBrush9setMatrixERK7QMatrix(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QBrush::setColor(const QColor & color);
impl /*struct*/ QBrush {
  pub fn setColor<RetType, T: QBrush_setColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColor(self);
    // return 1;
  }
}

pub trait QBrush_setColor<RetType> {
  fn setColor(self , rsthis: & QBrush) -> RetType;
}

  // proto:  void QBrush::setColor(const QColor & color);
impl<'a> /*trait*/ QBrush_setColor<()> for (&'a QColor) {
  fn setColor(self , rsthis: & QBrush) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrush8setColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QBrush8setColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QBrush::QBrush(const QBrush & brush);
impl<'a> /*trait*/ QBrush_New for (&'a QBrush) {
  fn New(self) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC1ERKS_()};
    let ctysz: c_int = unsafe{QBrush_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN6QBrushC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN6QBrushC1ERKS_(arg0)};
    let rsthis = QBrush{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMatrix & QBrush::matrix();
impl /*struct*/ QBrush {
  pub fn matrix<RetType, T: QBrush_matrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.matrix(self);
    // return 1;
  }
}

pub trait QBrush_matrix<RetType> {
  fn matrix(self , rsthis: & QBrush) -> RetType;
}

  // proto:  const QMatrix & QBrush::matrix();
impl<'a> /*trait*/ QBrush_matrix<QMatrix> for () {
  fn matrix(self , rsthis: & QBrush) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush6matrixEv()};
    let mut ret = unsafe {_ZNK6QBrush6matrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QImage QBrush::textureImage();
impl /*struct*/ QBrush {
  pub fn textureImage<RetType, T: QBrush_textureImage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textureImage(self);
    // return 1;
  }
}

pub trait QBrush_textureImage<RetType> {
  fn textureImage(self , rsthis: & QBrush) -> RetType;
}

  // proto:  QImage QBrush::textureImage();
impl<'a> /*trait*/ QBrush_textureImage<QImage> for () {
  fn textureImage(self , rsthis: & QBrush) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush12textureImageEv()};
    let mut ret = unsafe {_ZNK6QBrush12textureImageEv(rsthis.qclsinst)};
    let mut ret1 = QImage::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QBrush::isDetached();
impl /*struct*/ QBrush {
  pub fn isDetached<RetType, T: QBrush_isDetached<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDetached(self);
    // return 1;
  }
}

pub trait QBrush_isDetached<RetType> {
  fn isDetached(self , rsthis: & QBrush) -> RetType;
}

  // proto:  bool QBrush::isDetached();
impl<'a> /*trait*/ QBrush_isDetached<i8> for () {
  fn isDetached(self , rsthis: & QBrush) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush10isDetachedEv()};
    let mut ret = unsafe {_ZNK6QBrush10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QBrush::swap(QBrush & other);
impl /*struct*/ QBrush {
  pub fn swap<RetType, T: QBrush_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QBrush_swap<RetType> {
  fn swap(self , rsthis: & QBrush) -> RetType;
}

  // proto:  void QBrush::swap(QBrush & other);
impl<'a> /*trait*/ QBrush_swap<()> for (&'a QBrush) {
  fn swap(self , rsthis: & QBrush) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrush4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QBrush4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QColor & QBrush::color();
impl /*struct*/ QBrush {
  pub fn color<RetType, T: QBrush_color<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.color(self);
    // return 1;
  }
}

pub trait QBrush_color<RetType> {
  fn color(self , rsthis: & QBrush) -> RetType;
}

  // proto:  const QColor & QBrush::color();
impl<'a> /*trait*/ QBrush_color<QColor> for () {
  fn color(self , rsthis: & QBrush) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QBrush5colorEv()};
    let mut ret = unsafe {_ZNK6QBrush5colorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QBrush::QBrush(const QImage & image);
impl<'a> /*trait*/ QBrush_New for (&'a QImage) {
  fn New(self) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC1ERK6QImage()};
    let ctysz: c_int = unsafe{QBrush_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN6QBrushC1ERK6QImage(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN6QBrushC1ERK6QImage(arg0)};
    let rsthis = QBrush{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGradient {
  pub fn inheritFrom(qthis: *mut c_void) -> QGradient {
    return QGradient{qclsinst: qthis};
  }
}
  // proto:  void QGradient::setColorAt(qreal pos, const QColor & color);
impl /*struct*/ QGradient {
  pub fn setColorAt<RetType, T: QGradient_setColorAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColorAt(self);
    // return 1;
  }
}

pub trait QGradient_setColorAt<RetType> {
  fn setColorAt(self , rsthis: & QGradient) -> RetType;
}

  // proto:  void QGradient::setColorAt(qreal pos, const QColor & color);
impl<'a> /*trait*/ QGradient_setColorAt<()> for (f64, &'a QColor) {
  fn setColorAt(self , rsthis: & QGradient) -> () {
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
  pub fn stops<RetType, T: QGradient_stops<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stops(self);
    // return 1;
  }
}

pub trait QGradient_stops<RetType> {
  fn stops(self , rsthis: & QGradient) -> RetType;
}

  // proto:  QGradientStops QGradient::stops();
impl<'a> /*trait*/ QGradient_stops<()> for () {
  fn stops(self , rsthis: & QGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGradient5stopsEv()};
     unsafe {_ZNK9QGradient5stopsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGradient::QGradient();
impl /*struct*/ QGradient {
  pub fn New<T: QGradient_New>(value: T) -> QGradient {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGradient_New {
  fn New(self) -> QGradient;
}

  // proto:  void QGradient::QGradient();
impl<'a> /*trait*/ QGradient_New for () {
  fn New(self) -> QGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGradientC1Ev()};
    let ctysz: c_int = unsafe{QGradient_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN9QGradientC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN9QGradientC1Ev()};
    let rsthis = QGradient{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBrushData {
  pub fn inheritFrom(qthis: *mut c_void) -> QBrushData {
    return QBrushData{qclsinst: qthis};
  }
}
impl /*struct*/ QLinearGradient {
  pub fn inheritFrom(qthis: *mut c_void) -> QLinearGradient {
    return QLinearGradient{qbase: QGradient::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QLinearGradient {
  type Target = QGradient;

  fn deref(&self) -> &QGradient {
    return & self.qbase;
  }
}
impl AsRef<QGradient> for QLinearGradient {
  fn as_ref(& self) -> & QGradient {
    return & self.qbase;
  }
}
  // proto:  void QLinearGradient::setFinalStop(qreal x, qreal y);
impl /*struct*/ QLinearGradient {
  pub fn setFinalStop<RetType, T: QLinearGradient_setFinalStop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFinalStop(self);
    // return 1;
  }
}

pub trait QLinearGradient_setFinalStop<RetType> {
  fn setFinalStop(self , rsthis: & QLinearGradient) -> RetType;
}

  // proto:  void QLinearGradient::setFinalStop(qreal x, qreal y);
impl<'a> /*trait*/ QLinearGradient_setFinalStop<()> for (f64, f64) {
  fn setFinalStop(self , rsthis: & QLinearGradient) -> () {
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
  pub fn start<RetType, T: QLinearGradient_start<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.start(self);
    // return 1;
  }
}

pub trait QLinearGradient_start<RetType> {
  fn start(self , rsthis: & QLinearGradient) -> RetType;
}

  // proto:  QPointF QLinearGradient::start();
impl<'a> /*trait*/ QLinearGradient_start<QPointF> for () {
  fn start(self , rsthis: & QLinearGradient) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QLinearGradient5startEv()};
    let mut ret = unsafe {_ZNK15QLinearGradient5startEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLinearGradient::QLinearGradient(qreal xStart, qreal yStart, qreal xFinalStop, qreal yFinalStop);
impl /*struct*/ QLinearGradient {
  pub fn New<T: QLinearGradient_New>(value: T) -> QLinearGradient {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QLinearGradient_New {
  fn New(self) -> QLinearGradient;
}

  // proto:  void QLinearGradient::QLinearGradient(qreal xStart, qreal yStart, qreal xFinalStop, qreal yFinalStop);
impl<'a> /*trait*/ QLinearGradient_New for (f64, f64, f64, f64) {
  fn New(self) -> QLinearGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradientC1Edddd()};
    let ctysz: c_int = unsafe{QLinearGradient_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    // unsafe {_ZN15QLinearGradientC1Edddd(qthis, arg0, arg1, arg2, arg3)};
    let qthis: *mut c_void = unsafe {dector_ZN15QLinearGradientC1Edddd(arg0, arg1, arg2, arg3)};
    let rsthis = QLinearGradient{/**/qbase: QGradient::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QLinearGradient::QLinearGradient(const QPointF & start, const QPointF & finalStop);
impl<'a> /*trait*/ QLinearGradient_New for (&'a QPointF, &'a QPointF) {
  fn New(self) -> QLinearGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradientC1ERK7QPointFS2_()};
    let ctysz: c_int = unsafe{QLinearGradient_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN15QLinearGradientC1ERK7QPointFS2_(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN15QLinearGradientC1ERK7QPointFS2_(arg0, arg1)};
    let rsthis = QLinearGradient{/**/qbase: QGradient::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QLinearGradient::setStart(qreal x, qreal y);
impl /*struct*/ QLinearGradient {
  pub fn setStart<RetType, T: QLinearGradient_setStart<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStart(self);
    // return 1;
  }
}

pub trait QLinearGradient_setStart<RetType> {
  fn setStart(self , rsthis: & QLinearGradient) -> RetType;
}

  // proto:  void QLinearGradient::setStart(qreal x, qreal y);
impl<'a> /*trait*/ QLinearGradient_setStart<()> for (f64, f64) {
  fn setStart(self , rsthis: & QLinearGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradient8setStartEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN15QLinearGradient8setStartEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QLinearGradient::setStart(const QPointF & start);
impl<'a> /*trait*/ QLinearGradient_setStart<()> for (&'a QPointF) {
  fn setStart(self , rsthis: & QLinearGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradient8setStartERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QLinearGradient8setStartERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLinearGradient::QLinearGradient();
impl<'a> /*trait*/ QLinearGradient_New for () {
  fn New(self) -> QLinearGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradientC1Ev()};
    let ctysz: c_int = unsafe{QLinearGradient_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN15QLinearGradientC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN15QLinearGradientC1Ev()};
    let rsthis = QLinearGradient{/**/qbase: QGradient::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPointF QLinearGradient::finalStop();
impl /*struct*/ QLinearGradient {
  pub fn finalStop<RetType, T: QLinearGradient_finalStop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.finalStop(self);
    // return 1;
  }
}

pub trait QLinearGradient_finalStop<RetType> {
  fn finalStop(self , rsthis: & QLinearGradient) -> RetType;
}

  // proto:  QPointF QLinearGradient::finalStop();
impl<'a> /*trait*/ QLinearGradient_finalStop<QPointF> for () {
  fn finalStop(self , rsthis: & QLinearGradient) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QLinearGradient9finalStopEv()};
    let mut ret = unsafe {_ZNK15QLinearGradient9finalStopEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLinearGradient::setFinalStop(const QPointF & stop);
impl<'a> /*trait*/ QLinearGradient_setFinalStop<()> for (&'a QPointF) {
  fn setFinalStop(self , rsthis: & QLinearGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradient12setFinalStopERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QLinearGradient12setFinalStopERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

