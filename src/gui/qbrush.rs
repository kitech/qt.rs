// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
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
use super::super::core::qpoint::*; // 771
use super::qpixmap::*; // 773
use super::qimage::*; // 773
use super::qcolor::*; // 773
use super::qtransform::*; // 773
use super::qmatrix::*; // 773
// use super::qvector::*; // 775
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QRadialGradient_Class_Size() -> c_int;
  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal radius, qreal fx, qreal fy);
  fn C_ZN15QRadialGradientC2Eddddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> u64;
  // proto:  void QRadialGradient::setFocalPoint(qreal x, qreal y);
  fn C_ZN15QRadialGradient13setFocalPointEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto:  void QRadialGradient::QRadialGradient();
  fn C_ZN15QRadialGradientC2Ev() -> u64;
  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal radius, const QPointF & focalPoint);
  fn C_ZN15QRadialGradientC2ERK7QPointFdS2_(arg0: *mut c_void, arg1: c_double, arg2: *mut c_void) -> u64;
  // proto:  qreal QRadialGradient::radius();
  fn C_ZNK15QRadialGradient6radiusEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QRadialGradient::setFocalPoint(const QPointF & focalPoint);
  fn C_ZN15QRadialGradient13setFocalPointERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal centerRadius, const QPointF & focalPoint, qreal focalRadius);
  fn C_ZN15QRadialGradientC2ERK7QPointFdS2_d(arg0: *mut c_void, arg1: c_double, arg2: *mut c_void, arg3: c_double) -> u64;
  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal centerRadius, qreal fx, qreal fy, qreal focalRadius);
  fn C_ZN15QRadialGradientC2Edddddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double) -> u64;
  // proto:  qreal QRadialGradient::centerRadius();
  fn C_ZNK15QRadialGradient12centerRadiusEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QPointF QRadialGradient::focalPoint();
  fn C_ZNK15QRadialGradient10focalPointEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QRadialGradient::focalRadius();
  fn C_ZNK15QRadialGradient11focalRadiusEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QPointF QRadialGradient::center();
  fn C_ZNK15QRadialGradient6centerEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QRadialGradient::setCenter(const QPointF & center);
  fn C_ZN15QRadialGradient9setCenterERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal radius);
  fn C_ZN15QRadialGradientC2ERK7QPointFd(arg0: *mut c_void, arg1: c_double) -> u64;
  // proto:  void QRadialGradient::setCenterRadius(qreal radius);
  fn C_ZN15QRadialGradient15setCenterRadiusEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QRadialGradient::setFocalRadius(qreal radius);
  fn C_ZN15QRadialGradient14setFocalRadiusEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QRadialGradient::setRadius(qreal radius);
  fn C_ZN15QRadialGradient9setRadiusEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal radius);
  fn C_ZN15QRadialGradientC2Eddd(arg0: c_double, arg1: c_double, arg2: c_double) -> u64;
  // proto:  void QRadialGradient::setCenter(qreal x, qreal y);
  fn C_ZN15QRadialGradient9setCenterEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  fn QConicalGradient_Class_Size() -> c_int;
  // proto:  qreal QConicalGradient::angle();
  fn C_ZNK16QConicalGradient5angleEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QPointF QConicalGradient::center();
  fn C_ZNK16QConicalGradient6centerEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QConicalGradient::QConicalGradient(const QPointF & center, qreal startAngle);
  fn C_ZN16QConicalGradientC2ERK7QPointFd(arg0: *mut c_void, arg1: c_double) -> u64;
  // proto:  void QConicalGradient::QConicalGradient();
  fn C_ZN16QConicalGradientC2Ev() -> u64;
  // proto:  void QConicalGradient::setAngle(qreal angle);
  fn C_ZN16QConicalGradient8setAngleEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QConicalGradient::setCenter(qreal x, qreal y);
  fn C_ZN16QConicalGradient9setCenterEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto:  void QConicalGradient::setCenter(const QPointF & center);
  fn C_ZN16QConicalGradient9setCenterERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QConicalGradient::QConicalGradient(qreal cx, qreal cy, qreal startAngle);
  fn C_ZN16QConicalGradientC2Eddd(arg0: c_double, arg1: c_double, arg2: c_double) -> u64;
  fn QBrush_Class_Size() -> c_int;
  // proto:  void QBrush::QBrush();
  fn C_ZN6QBrushC2Ev() -> u64;
  // proto:  void QBrush::QBrush(const QPixmap & pixmap);
  fn C_ZN6QBrushC2ERK7QPixmap(arg0: *mut c_void) -> u64;
  // proto:  void QBrush::setTexture(const QPixmap & pixmap);
  fn C_ZN6QBrush10setTextureERK7QPixmap(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QBrush::setTextureImage(const QImage & image);
  fn C_ZN6QBrush15setTextureImageERK6QImage(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QBrush::QBrush(const QColor & color, const QPixmap & pixmap);
  fn C_ZN6QBrushC2ERK6QColorRK7QPixmap(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  QPixmap QBrush::texture();
  fn C_ZNK6QBrush7textureEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QBrush::QBrush(const QGradient & gradient);
  fn C_ZN6QBrushC2ERK9QGradient(arg0: *mut c_void) -> u64;
  // proto:  QTransform QBrush::transform();
  fn C_ZNK6QBrush9transformEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QBrush::setTransform(const QTransform & );
  fn C_ZN6QBrush12setTransformERK10QTransform(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QBrush::isOpaque();
  fn C_ZNK6QBrush8isOpaqueEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QGradient * QBrush::gradient();
  fn C_ZNK6QBrush8gradientEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QBrush::~QBrush();
  fn C_ZN6QBrushD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QBrush::setMatrix(const QMatrix & mat);
  fn C_ZN6QBrush9setMatrixERK7QMatrix(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QBrush::setColor(const QColor & color);
  fn C_ZN6QBrush8setColorERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QBrush::QBrush(const QBrush & brush);
  fn C_ZN6QBrushC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  const QMatrix & QBrush::matrix();
  fn C_ZNK6QBrush6matrixEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QImage QBrush::textureImage();
  fn C_ZNK6QBrush12textureImageEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QBrush::isDetached();
  fn C_ZNK6QBrush10isDetachedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QBrush::swap(QBrush & other);
  fn C_ZN6QBrush4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QColor & QBrush::color();
  fn C_ZNK6QBrush5colorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QBrush::QBrush(const QImage & image);
  fn C_ZN6QBrushC2ERK6QImage(arg0: *mut c_void) -> u64;
  fn QGradient_Class_Size() -> c_int;
  // proto:  void QGradient::setColorAt(qreal pos, const QColor & color);
  fn C_ZN9QGradient10setColorAtEdRK6QColor(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: *mut c_void);
  // proto:  QGradientStops QGradient::stops();
  fn C_ZNK9QGradient5stopsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGradient::QGradient();
  fn C_ZN9QGradientC2Ev() -> u64;
  fn QBrushData_Class_Size() -> c_int;
  fn QLinearGradient_Class_Size() -> c_int;
  // proto:  void QLinearGradient::setFinalStop(qreal x, qreal y);
  fn C_ZN15QLinearGradient12setFinalStopEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto:  QPointF QLinearGradient::start();
  fn C_ZNK15QLinearGradient5startEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QLinearGradient::QLinearGradient(qreal xStart, qreal yStart, qreal xFinalStop, qreal yFinalStop);
  fn C_ZN15QLinearGradientC2Edddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> u64;
  // proto:  void QLinearGradient::QLinearGradient(const QPointF & start, const QPointF & finalStop);
  fn C_ZN15QLinearGradientC2ERK7QPointFS2_(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  void QLinearGradient::setStart(qreal x, qreal y);
  fn C_ZN15QLinearGradient8setStartEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto:  void QLinearGradient::setStart(const QPointF & start);
  fn C_ZN15QLinearGradient8setStartERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QLinearGradient::QLinearGradient();
  fn C_ZN15QLinearGradientC2Ev() -> u64;
  // proto:  QPointF QLinearGradient::finalStop();
  fn C_ZNK15QLinearGradient9finalStopEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QLinearGradient::setFinalStop(const QPointF & stop);
  fn C_ZN15QLinearGradient12setFinalStopERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QRadialGradient)=1
#[derive(Default)]
pub struct QRadialGradient {
  qbase: QGradient,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QConicalGradient)=1
#[derive(Default)]
pub struct QConicalGradient {
  qbase: QGradient,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QBrush)=1
#[derive(Default)]
pub struct QBrush {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGradient)=1
#[derive(Default)]
pub struct QGradient {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QBrushData)=1
#[derive(Default)]
pub struct QBrushData {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QLinearGradient)=1
#[derive(Default)]
pub struct QLinearGradient {
  qbase: QGradient,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QRadialGradient {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QRadialGradient {
    return QRadialGradient{qbase: QGradient::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QRadialGradient_new>(value: T) -> QRadialGradient {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QRadialGradient_new {
  fn new(self) -> QRadialGradient;
}

  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal radius, qreal fx, qreal fy);
impl<'a> /*trait*/ QRadialGradient_new for (f64, f64, f64, f64, f64) {
  fn new(self) -> QRadialGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC2Eddddd()};
    let ctysz: c_int = unsafe{QRadialGradient_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let qthis: u64 = unsafe {C_ZN15QRadialGradientC2Eddddd(arg0, arg1, arg2, arg3, arg4)};
    let rsthis = QRadialGradient{qbase: QGradient::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
     unsafe {C_ZN15QRadialGradient13setFocalPointEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient();
impl<'a> /*trait*/ QRadialGradient_new for () {
  fn new(self) -> QRadialGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC2Ev()};
    let ctysz: c_int = unsafe{QRadialGradient_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN15QRadialGradientC2Ev()};
    let rsthis = QRadialGradient{qbase: QGradient::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal radius, const QPointF & focalPoint);
impl<'a> /*trait*/ QRadialGradient_new for (&'a QPointF, f64, &'a QPointF) {
  fn new(self) -> QRadialGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC2ERK7QPointFdS2_()};
    let ctysz: c_int = unsafe{QRadialGradient_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN15QRadialGradientC2ERK7QPointFdS2_(arg0, arg1, arg2)};
    let rsthis = QRadialGradient{qbase: QGradient::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK15QRadialGradient6radiusEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QRadialGradient::setFocalPoint(const QPointF & focalPoint);
impl<'a> /*trait*/ QRadialGradient_setFocalPoint<()> for (&'a QPointF) {
  fn setFocalPoint(self , rsthis: & QRadialGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradient13setFocalPointERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QRadialGradient13setFocalPointERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal centerRadius, const QPointF & focalPoint, qreal focalRadius);
impl<'a> /*trait*/ QRadialGradient_new for (&'a QPointF, f64, &'a QPointF, f64) {
  fn new(self) -> QRadialGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC2ERK7QPointFdS2_d()};
    let ctysz: c_int = unsafe{QRadialGradient_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_double;
    let qthis: u64 = unsafe {C_ZN15QRadialGradientC2ERK7QPointFdS2_d(arg0, arg1, arg2, arg3)};
    let rsthis = QRadialGradient{qbase: QGradient::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal centerRadius, qreal fx, qreal fy, qreal focalRadius);
impl<'a> /*trait*/ QRadialGradient_new for (f64, f64, f64, f64, f64, f64) {
  fn new(self) -> QRadialGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC2Edddddd()};
    let ctysz: c_int = unsafe{QRadialGradient_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
    let qthis: u64 = unsafe {C_ZN15QRadialGradientC2Edddddd(arg0, arg1, arg2, arg3, arg4, arg5)};
    let rsthis = QRadialGradient{qbase: QGradient::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK15QRadialGradient12centerRadiusEv(rsthis.qclsinst)};
    return ret as f64; // 1
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
    let mut ret = unsafe {C_ZNK15QRadialGradient10focalPointEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK15QRadialGradient11focalRadiusEv(rsthis.qclsinst)};
    return ret as f64; // 1
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
    let mut ret = unsafe {C_ZNK15QRadialGradient6centerEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
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
     unsafe {C_ZN15QRadialGradient9setCenterERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient(const QPointF & center, qreal radius);
impl<'a> /*trait*/ QRadialGradient_new for (&'a QPointF, f64) {
  fn new(self) -> QRadialGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC2ERK7QPointFd()};
    let ctysz: c_int = unsafe{QRadialGradient_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let qthis: u64 = unsafe {C_ZN15QRadialGradientC2ERK7QPointFd(arg0, arg1)};
    let rsthis = QRadialGradient{qbase: QGradient::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
     unsafe {C_ZN15QRadialGradient15setCenterRadiusEd(rsthis.qclsinst, arg0)};
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
     unsafe {C_ZN15QRadialGradient14setFocalRadiusEd(rsthis.qclsinst, arg0)};
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
     unsafe {C_ZN15QRadialGradient9setRadiusEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRadialGradient::QRadialGradient(qreal cx, qreal cy, qreal radius);
impl<'a> /*trait*/ QRadialGradient_new for (f64, f64, f64) {
  fn new(self) -> QRadialGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QRadialGradientC2Eddd()};
    let ctysz: c_int = unsafe{QRadialGradient_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let qthis: u64 = unsafe {C_ZN15QRadialGradientC2Eddd(arg0, arg1, arg2)};
    let rsthis = QRadialGradient{qbase: QGradient::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
     unsafe {C_ZN15QRadialGradient9setCenterEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QConicalGradient {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QConicalGradient {
    return QConicalGradient{qbase: QGradient::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK16QConicalGradient5angleEv(rsthis.qclsinst)};
    return ret as f64; // 1
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
    let mut ret = unsafe {C_ZNK16QConicalGradient6centerEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QConicalGradient::QConicalGradient(const QPointF & center, qreal startAngle);
impl /*struct*/ QConicalGradient {
  pub fn new<T: QConicalGradient_new>(value: T) -> QConicalGradient {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QConicalGradient_new {
  fn new(self) -> QConicalGradient;
}

  // proto:  void QConicalGradient::QConicalGradient(const QPointF & center, qreal startAngle);
impl<'a> /*trait*/ QConicalGradient_new for (&'a QPointF, f64) {
  fn new(self) -> QConicalGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradientC2ERK7QPointFd()};
    let ctysz: c_int = unsafe{QConicalGradient_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let qthis: u64 = unsafe {C_ZN16QConicalGradientC2ERK7QPointFd(arg0, arg1)};
    let rsthis = QConicalGradient{qbase: QGradient::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QConicalGradient::QConicalGradient();
impl<'a> /*trait*/ QConicalGradient_new for () {
  fn new(self) -> QConicalGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradientC2Ev()};
    let ctysz: c_int = unsafe{QConicalGradient_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN16QConicalGradientC2Ev()};
    let rsthis = QConicalGradient{qbase: QGradient::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
     unsafe {C_ZN16QConicalGradient8setAngleEd(rsthis.qclsinst, arg0)};
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
     unsafe {C_ZN16QConicalGradient9setCenterEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QConicalGradient::setCenter(const QPointF & center);
impl<'a> /*trait*/ QConicalGradient_setCenter<()> for (&'a QPointF) {
  fn setCenter(self , rsthis: & QConicalGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradient9setCenterERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QConicalGradient9setCenterERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QConicalGradient::QConicalGradient(qreal cx, qreal cy, qreal startAngle);
impl<'a> /*trait*/ QConicalGradient_new for (f64, f64, f64) {
  fn new(self) -> QConicalGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QConicalGradientC2Eddd()};
    let ctysz: c_int = unsafe{QConicalGradient_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let qthis: u64 = unsafe {C_ZN16QConicalGradientC2Eddd(arg0, arg1, arg2)};
    let rsthis = QConicalGradient{qbase: QGradient::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBrush {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QBrush {
    return QBrush{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QBrush::QBrush();
impl /*struct*/ QBrush {
  pub fn new<T: QBrush_new>(value: T) -> QBrush {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QBrush_new {
  fn new(self) -> QBrush;
}

  // proto:  void QBrush::QBrush();
impl<'a> /*trait*/ QBrush_new for () {
  fn new(self) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC2Ev()};
    let ctysz: c_int = unsafe{QBrush_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN6QBrushC2Ev()};
    let rsthis = QBrush{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QBrush::QBrush(const QPixmap & pixmap);
impl<'a> /*trait*/ QBrush_new for (&'a QPixmap) {
  fn new(self) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC2ERK7QPixmap()};
    let ctysz: c_int = unsafe{QBrush_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN6QBrushC2ERK7QPixmap(arg0)};
    let rsthis = QBrush{qclsinst: qthis, ..Default::default()};
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
     unsafe {C_ZN6QBrush10setTextureERK7QPixmap(rsthis.qclsinst, arg0)};
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
     unsafe {C_ZN6QBrush15setTextureImageERK6QImage(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QBrush::QBrush(const QColor & color, const QPixmap & pixmap);
impl<'a> /*trait*/ QBrush_new for (&'a QColor, &'a QPixmap) {
  fn new(self) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC2ERK6QColorRK7QPixmap()};
    let ctysz: c_int = unsafe{QBrush_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN6QBrushC2ERK6QColorRK7QPixmap(arg0, arg1)};
    let rsthis = QBrush{qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK6QBrush7textureEv(rsthis.qclsinst)};
    let mut ret1 = QPixmap::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QBrush::QBrush(const QGradient & gradient);
impl<'a> /*trait*/ QBrush_new for (&'a QGradient) {
  fn new(self) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC2ERK9QGradient()};
    let ctysz: c_int = unsafe{QBrush_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN6QBrushC2ERK9QGradient(arg0)};
    let rsthis = QBrush{qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK6QBrush9transformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
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
     unsafe {C_ZN6QBrush12setTransformERK10QTransform(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZNK6QBrush8isOpaqueEv(rsthis.qclsinst)};
    return ret as i8; // 1
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
    let mut ret = unsafe {C_ZNK6QBrush8gradientEv(rsthis.qclsinst)};
    let mut ret1 = QGradient::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QBrush::~QBrush();
impl /*struct*/ QBrush {
  pub fn free<RetType, T: QBrush_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QBrush_free<RetType> {
  fn free(self , rsthis: & QBrush) -> RetType;
}

  // proto:  void QBrush::~QBrush();
impl<'a> /*trait*/ QBrush_free<()> for () {
  fn free(self , rsthis: & QBrush) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushD2Ev()};
     unsafe {C_ZN6QBrushD2Ev(rsthis.qclsinst)};
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
     unsafe {C_ZN6QBrush9setMatrixERK7QMatrix(rsthis.qclsinst, arg0)};
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
     unsafe {C_ZN6QBrush8setColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QBrush::QBrush(const QBrush & brush);
impl<'a> /*trait*/ QBrush_new for (&'a QBrush) {
  fn new(self) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC2ERKS_()};
    let ctysz: c_int = unsafe{QBrush_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN6QBrushC2ERKS_(arg0)};
    let rsthis = QBrush{qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK6QBrush6matrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK6QBrush12textureImageEv(rsthis.qclsinst)};
    let mut ret1 = QImage::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZNK6QBrush10isDetachedEv(rsthis.qclsinst)};
    return ret as i8; // 1
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
     unsafe {C_ZN6QBrush4swapERS_(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZNK6QBrush5colorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QBrush::QBrush(const QImage & image);
impl<'a> /*trait*/ QBrush_new for (&'a QImage) {
  fn new(self) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QBrushC2ERK6QImage()};
    let ctysz: c_int = unsafe{QBrush_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN6QBrushC2ERK6QImage(arg0)};
    let rsthis = QBrush{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGradient {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGradient {
    return QGradient{qclsinst: qthis, ..Default::default()};
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
     unsafe {C_ZN9QGradient10setColorAtEdRK6QColor(rsthis.qclsinst, arg0, arg1)};
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
impl<'a> /*trait*/ QGradient_stops<u64> for () {
  fn stops(self , rsthis: & QGradient) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGradient5stopsEv()};
    let mut ret = unsafe {C_ZNK9QGradient5stopsEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  void QGradient::QGradient();
impl /*struct*/ QGradient {
  pub fn new<T: QGradient_new>(value: T) -> QGradient {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGradient_new {
  fn new(self) -> QGradient;
}

  // proto:  void QGradient::QGradient();
impl<'a> /*trait*/ QGradient_new for () {
  fn new(self) -> QGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGradientC2Ev()};
    let ctysz: c_int = unsafe{QGradient_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN9QGradientC2Ev()};
    let rsthis = QGradient{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBrushData {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QBrushData {
    return QBrushData{qclsinst: qthis, ..Default::default()};
  }
}
impl /*struct*/ QLinearGradient {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QLinearGradient {
    return QLinearGradient{qbase: QGradient::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
     unsafe {C_ZN15QLinearGradient12setFinalStopEdd(rsthis.qclsinst, arg0, arg1)};
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
    let mut ret = unsafe {C_ZNK15QLinearGradient5startEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLinearGradient::QLinearGradient(qreal xStart, qreal yStart, qreal xFinalStop, qreal yFinalStop);
impl /*struct*/ QLinearGradient {
  pub fn new<T: QLinearGradient_new>(value: T) -> QLinearGradient {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QLinearGradient_new {
  fn new(self) -> QLinearGradient;
}

  // proto:  void QLinearGradient::QLinearGradient(qreal xStart, qreal yStart, qreal xFinalStop, qreal yFinalStop);
impl<'a> /*trait*/ QLinearGradient_new for (f64, f64, f64, f64) {
  fn new(self) -> QLinearGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradientC2Edddd()};
    let ctysz: c_int = unsafe{QLinearGradient_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let qthis: u64 = unsafe {C_ZN15QLinearGradientC2Edddd(arg0, arg1, arg2, arg3)};
    let rsthis = QLinearGradient{qbase: QGradient::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QLinearGradient::QLinearGradient(const QPointF & start, const QPointF & finalStop);
impl<'a> /*trait*/ QLinearGradient_new for (&'a QPointF, &'a QPointF) {
  fn new(self) -> QLinearGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradientC2ERK7QPointFS2_()};
    let ctysz: c_int = unsafe{QLinearGradient_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN15QLinearGradientC2ERK7QPointFS2_(arg0, arg1)};
    let rsthis = QLinearGradient{qbase: QGradient::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
     unsafe {C_ZN15QLinearGradient8setStartEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QLinearGradient::setStart(const QPointF & start);
impl<'a> /*trait*/ QLinearGradient_setStart<()> for (&'a QPointF) {
  fn setStart(self , rsthis: & QLinearGradient) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradient8setStartERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QLinearGradient8setStartERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLinearGradient::QLinearGradient();
impl<'a> /*trait*/ QLinearGradient_new for () {
  fn new(self) -> QLinearGradient {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QLinearGradientC2Ev()};
    let ctysz: c_int = unsafe{QLinearGradient_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN15QLinearGradientC2Ev()};
    let rsthis = QLinearGradient{qbase: QGradient::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {C_ZNK15QLinearGradient9finalStopEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
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
     unsafe {C_ZN15QLinearGradient12setFinalStopERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

