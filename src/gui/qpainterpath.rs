// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtGui/qpainterpath.h
// dst-file: /src/gui/qpainterpath.rs
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
use std::ops::Deref;
use super::qmatrix::QMatrix; // 773
use super::qpolygon::QPolygonF; // 773
use super::qtransform::QTransform; // 773
use super::super::core::qrect::QRectF; // 771
use super::super::core::qpoint::QPointF; // 771
use super::qfont::QFont; // 773
use super::super::core::qstring::QString; // 771
use super::qregion::QRegion; // 773
use super::qpen::QPen; // 773
// use super::qpainterpath::QPainterPath; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPainterPath_Class_Size() -> c_int;
  // proto:  void QPainterPath::setElementPositionAt(int i, qreal x, qreal y);
  fn _ZN12QPainterPath20setElementPositionAtEidd(qthis: *mut c_void, arg0: c_int, arg1: c_double, arg2: c_double);
  // proto:  QPolygonF QPainterPath::toFillPolygon(const QMatrix & matrix);
  fn _ZNK12QPainterPath13toFillPolygonERK7QMatrix(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPainterPath QPainterPath::translated(qreal dx, qreal dy);
  fn _ZNK12QPainterPath10translatedEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  void QPainterPath::~QPainterPath();
  fn _ZN12QPainterPathD0Ev(qthis: *mut c_void);
  // proto:  QList<QPolygonF> QPainterPath::toSubpathPolygons(const QTransform & matrix);
  fn _ZNK12QPainterPath17toSubpathPolygonsERK10QTransform(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRectF QPainterPath::controlPointRect();
  fn _ZNK12QPainterPath16controlPointRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QList<QPolygonF> QPainterPath::toFillPolygons(const QMatrix & matrix);
  fn _ZNK12QPainterPath14toFillPolygonsERK7QMatrix(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPainterPath QPainterPath::translated(const QPointF & offset);
  fn _ZNK12QPainterPath10translatedERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::quadTo(const QPointF & ctrlPt, const QPointF & endPt);
  fn _ZN12QPainterPath6quadToERK7QPointFS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QList<QPolygonF> QPainterPath::toFillPolygons(const QTransform & matrix);
  fn _ZNK12QPainterPath14toFillPolygonsERK10QTransform(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainterPath::arcTo(qreal x, qreal y, qreal w, qreal h, qreal startAngle, qreal arcLength);
  fn _ZN12QPainterPath5arcToEdddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double);
  // proto:  void QPainterPath::addRect(const QRectF & rect);
  fn _ZN12QPainterPath7addRectERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainterPath::addRoundRect(const QRectF & rect, int xRnd, int yRnd);
  fn _ZN12QPainterPath12addRoundRectERK6QRectFii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  void QPainterPath::addText(qreal x, qreal y, const QFont & f, const QString & text);
  fn _ZN12QPainterPath7addTextEddRK5QFontRK7QString(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: *mut c_void, arg3: *mut c_void);
  // proto:  bool QPainterPath::intersects(const QRectF & rect);
  fn _ZNK12QPainterPath10intersectsERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  bool QPainterPath::contains(const QPointF & pt);
  fn _ZNK12QPainterPath8containsERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QPainterPath::arcTo(const QRectF & rect, qreal startAngle, qreal arcLength);
  fn _ZN12QPainterPath5arcToERK6QRectFdd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double);
  // proto:  void QPainterPath::addRoundRect(const QRectF & rect, int roundness);
  fn _ZN12QPainterPath12addRoundRectERK6QRectFi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPainterPath::addEllipse(const QPointF & center, qreal rx, qreal ry);
  fn _ZN12QPainterPath10addEllipseERK7QPointFdd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double);
  // proto:  void QPainterPath::lineTo(qreal x, qreal y);
  fn _ZN12QPainterPath6lineToEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QPainterPath::cubicTo(const QPointF & ctrlPt1, const QPointF & ctrlPt2, const QPointF & endPt);
  fn _ZN12QPainterPath7cubicToERK7QPointFS2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  qreal QPainterPath::slopeAtPercent(qreal t);
  fn _ZNK12QPainterPath14slopeAtPercentEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  void QPainterPath::addEllipse(qreal x, qreal y, qreal w, qreal h);
  fn _ZN12QPainterPath10addEllipseEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  void QPainterPath::addRoundRect(qreal x, qreal y, qreal w, qreal h, int roundness);
  fn _ZN12QPainterPath12addRoundRectEddddi(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_int);
  // proto:  void QPainterPath::QPainterPath(const QPointF & startPoint);
  fn dector_ZN12QPainterPathC1ERK7QPointF(arg0: *mut c_void) -> *mut c_void;
  fn _ZN12QPainterPathC1ERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPainterPath QPainterPath::intersected(const QPainterPath & r);
  fn _ZNK12QPainterPath11intersectedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::translate(qreal dx, qreal dy);
  fn _ZN12QPainterPath9translateEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QPainterPath::addPolygon(const QPolygonF & polygon);
  fn _ZN12QPainterPath10addPolygonERK9QPolygonF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainterPath::translate(const QPointF & offset);
  fn _ZN12QPainterPath9translateERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPolygonF QPainterPath::toFillPolygon(const QTransform & matrix);
  fn _ZNK12QPainterPath13toFillPolygonERK10QTransform(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::addPath(const QPainterPath & path);
  fn _ZN12QPainterPath7addPathERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainterPath::quadTo(qreal ctrlPtx, qreal ctrlPty, qreal endPtx, qreal endPty);
  fn _ZN12QPainterPath6quadToEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  int QPainterPath::elementCount();
  fn _ZNK12QPainterPath12elementCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QPainterPath QPainterPath::simplified();
  fn _ZNK12QPainterPath10simplifiedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QPainterPath::contains(const QRectF & rect);
  fn _ZNK12QPainterPath8containsERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  qreal QPainterPath::length();
  fn _ZNK12QPainterPath6lengthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QPainterPath::connectPath(const QPainterPath & path);
  fn _ZN12QPainterPath11connectPathERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainterPath::addRegion(const QRegion & region);
  fn _ZN12QPainterPath9addRegionERK7QRegion(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPointF QPainterPath::currentPosition();
  fn _ZNK12QPainterPath15currentPositionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPainterPath QPainterPath::toReversed();
  fn _ZNK12QPainterPath10toReversedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::addRoundRect(qreal x, qreal y, qreal w, qreal h, int xRnd, int yRnd);
  fn _ZN12QPainterPath12addRoundRectEddddii(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_int, arg5: c_int);
  // proto:  QRectF QPainterPath::boundingRect();
  fn _ZNK12QPainterPath12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QPainterPath::contains(const QPainterPath & p);
  fn _ZNK12QPainterPath8containsERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QPainterPath::moveTo(qreal x, qreal y);
  fn _ZN12QPainterPath6moveToEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  QPainterPath QPainterPath::subtracted(const QPainterPath & r);
  fn _ZNK12QPainterPath10subtractedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::QPainterPath();
  fn dector_ZN12QPainterPathC1Ev() -> *mut c_void;
  fn _ZN12QPainterPathC1Ev(qthis: *mut c_void);
  // proto:  void QPainterPath::addText(const QPointF & point, const QFont & f, const QString & text);
  fn _ZN12QPainterPath7addTextERK7QPointFRK5QFontRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QPainterPath::QPainterPath(const QPainterPath & other);
  fn dector_ZN12QPainterPathC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN12QPainterPathC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPointF QPainterPath::pointAtPercent(qreal t);
  fn _ZNK12QPainterPath14pointAtPercentEd(qthis: *mut c_void, arg0: c_double) -> *mut c_void;
  // proto:  qreal QPainterPath::percentAtLength(qreal t);
  fn _ZNK12QPainterPath15percentAtLengthEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  void QPainterPath::cubicTo(qreal ctrlPt1x, qreal ctrlPt1y, qreal ctrlPt2x, qreal ctrlPt2y, qreal endPtx, qreal endPty);
  fn _ZN12QPainterPath7cubicToEdddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double);
  // proto:  void QPainterPath::lineTo(const QPointF & p);
  fn _ZN12QPainterPath6lineToERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPainterPath QPainterPath::subtractedInverted(const QPainterPath & r);
  fn _ZNK12QPainterPath18subtractedInvertedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QPainterPath::intersects(const QPainterPath & p);
  fn _ZNK12QPainterPath10intersectsERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QPainterPath::arcMoveTo(qreal x, qreal y, qreal w, qreal h, qreal angle);
  fn _ZN12QPainterPath9arcMoveToEddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double);
  // proto:  bool QPainterPath::isEmpty();
  fn _ZNK12QPainterPath7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPainterPath::addRect(qreal x, qreal y, qreal w, qreal h);
  fn _ZN12QPainterPath7addRectEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  void QPainterPath::arcMoveTo(const QRectF & rect, qreal angle);
  fn _ZN12QPainterPath9arcMoveToERK6QRectFd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double);
  // proto:  QList<QPolygonF> QPainterPath::toSubpathPolygons(const QMatrix & matrix);
  fn _ZNK12QPainterPath17toSubpathPolygonsERK7QMatrix(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPainterPath QPainterPath::united(const QPainterPath & r);
  fn _ZNK12QPainterPath6unitedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::addEllipse(const QRectF & rect);
  fn _ZN12QPainterPath10addEllipseERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainterPath::moveTo(const QPointF & p);
  fn _ZN12QPainterPath6moveToERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QPainterPath::angleAtPercent(qreal t);
  fn _ZNK12QPainterPath14angleAtPercentEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  void QPainterPath::closeSubpath();
  fn _ZN12QPainterPath12closeSubpathEv(qthis: *mut c_void);
  fn QPainterPathStroker_Class_Size() -> c_int;
  // proto:  qreal QPainterPathStroker::curveThreshold();
  fn _ZNK19QPainterPathStroker14curveThresholdEv(qthis: *mut c_void) -> c_double;
  // proto:  void QPainterPathStroker::QPainterPathStroker();
  fn dector_ZN19QPainterPathStrokerC1Ev() -> *mut c_void;
  fn _ZN19QPainterPathStrokerC1Ev(qthis: *mut c_void);
  // proto:  void QPainterPathStroker::setDashOffset(qreal offset);
  fn _ZN19QPainterPathStroker13setDashOffsetEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QPainterPathStroker::QPainterPathStroker(const QPen & pen);
  fn dector_ZN19QPainterPathStrokerC1ERK4QPen(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QPainterPathStrokerC1ERK4QPen(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainterPathStroker::setCurveThreshold(qreal threshold);
  fn _ZN19QPainterPathStroker17setCurveThresholdEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QPainterPathStroker::dashOffset();
  fn _ZNK19QPainterPathStroker10dashOffsetEv(qthis: *mut c_void) -> c_double;
  // proto:  void QPainterPathStroker::setWidth(qreal width);
  fn _ZN19QPainterPathStroker8setWidthEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QPainterPathStroker::~QPainterPathStroker();
  fn _ZN19QPainterPathStrokerD0Ev(qthis: *mut c_void);
  // proto:  qreal QPainterPathStroker::width();
  fn _ZNK19QPainterPathStroker5widthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QPainterPathStroker::QPainterPathStroker(const QPainterPathStroker & );
  fn dector_ZN19QPainterPathStrokerC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QPainterPathStrokerC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QPainterPathStroker::miterLimit();
  fn _ZNK19QPainterPathStroker10miterLimitEv(qthis: *mut c_void) -> c_double;
  // proto:  QVector<qreal> QPainterPathStroker::dashPattern();
  fn _ZNK19QPainterPathStroker11dashPatternEv(qthis: *mut c_void);
  // proto:  QPainterPath QPainterPathStroker::createStroke(const QPainterPath & path);
  fn _ZNK19QPainterPathStroker12createStrokeERK12QPainterPath(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPathStroker::setMiterLimit(qreal length);
  fn _ZN19QPainterPathStroker13setMiterLimitEd(qthis: *mut c_void, arg0: c_double);
} // <= ext block end

// body block begin =>
// class sizeof(QPainterPath)=1
pub struct QPainterPath {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QPainterPathStroker)=1
pub struct QPainterPathStroker {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPainterPath {
  pub fn inheritFrom(qthis: *mut c_void) -> QPainterPath {
    return QPainterPath{qclsinst: qthis};
  }
}
  // proto:  void QPainterPath::setElementPositionAt(int i, qreal x, qreal y);
impl /*struct*/ QPainterPath {
  pub fn setElementPositionAt<RetType, T: QPainterPath_setElementPositionAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setElementPositionAt(self);
    // return 1;
  }
}

pub trait QPainterPath_setElementPositionAt<RetType> {
  fn setElementPositionAt(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::setElementPositionAt(int i, qreal x, qreal y);
impl<'a> /*trait*/ QPainterPath_setElementPositionAt<()> for (i32, f64, f64) {
  fn setElementPositionAt(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath20setElementPositionAtEidd()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
     unsafe {_ZN12QPainterPath20setElementPositionAtEidd(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QPolygonF QPainterPath::toFillPolygon(const QMatrix & matrix);
impl /*struct*/ QPainterPath {
  pub fn toFillPolygon<RetType, T: QPainterPath_toFillPolygon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toFillPolygon(self);
    // return 1;
  }
}

pub trait QPainterPath_toFillPolygon<RetType> {
  fn toFillPolygon(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  QPolygonF QPainterPath::toFillPolygon(const QMatrix & matrix);
impl<'a> /*trait*/ QPainterPath_toFillPolygon<QPolygonF> for (&'a QMatrix) {
  fn toFillPolygon(self , rsthis: & QPainterPath) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath13toFillPolygonERK7QMatrix()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath13toFillPolygonERK7QMatrix(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPainterPath QPainterPath::translated(qreal dx, qreal dy);
impl /*struct*/ QPainterPath {
  pub fn translated<RetType, T: QPainterPath_translated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.translated(self);
    // return 1;
  }
}

pub trait QPainterPath_translated<RetType> {
  fn translated(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  QPainterPath QPainterPath::translated(qreal dx, qreal dy);
impl<'a> /*trait*/ QPainterPath_translated<QPainterPath> for (f64, f64) {
  fn translated(self , rsthis: & QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10translatedEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK12QPainterPath10translatedEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPainterPath::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainterPath::~QPainterPath();
impl /*struct*/ QPainterPath {
  pub fn Free<RetType, T: QPainterPath_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QPainterPath_Free<RetType> {
  fn Free(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::~QPainterPath();
impl<'a> /*trait*/ QPainterPath_Free<()> for () {
  fn Free(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPathD0Ev()};
     unsafe {_ZN12QPainterPathD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QPolygonF> QPainterPath::toSubpathPolygons(const QTransform & matrix);
impl /*struct*/ QPainterPath {
  pub fn toSubpathPolygons<RetType, T: QPainterPath_toSubpathPolygons<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toSubpathPolygons(self);
    // return 1;
  }
}

pub trait QPainterPath_toSubpathPolygons<RetType> {
  fn toSubpathPolygons(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  QList<QPolygonF> QPainterPath::toSubpathPolygons(const QTransform & matrix);
impl<'a> /*trait*/ QPainterPath_toSubpathPolygons<()> for (&'a QTransform) {
  fn toSubpathPolygons(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath17toSubpathPolygonsERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK12QPainterPath17toSubpathPolygonsERK10QTransform(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QPainterPath::controlPointRect();
impl /*struct*/ QPainterPath {
  pub fn controlPointRect<RetType, T: QPainterPath_controlPointRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.controlPointRect(self);
    // return 1;
  }
}

pub trait QPainterPath_controlPointRect<RetType> {
  fn controlPointRect(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  QRectF QPainterPath::controlPointRect();
impl<'a> /*trait*/ QPainterPath_controlPointRect<QRectF> for () {
  fn controlPointRect(self , rsthis: & QPainterPath) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath16controlPointRectEv()};
    let mut ret = unsafe {_ZNK12QPainterPath16controlPointRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QPolygonF> QPainterPath::toFillPolygons(const QMatrix & matrix);
impl /*struct*/ QPainterPath {
  pub fn toFillPolygons<RetType, T: QPainterPath_toFillPolygons<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toFillPolygons(self);
    // return 1;
  }
}

pub trait QPainterPath_toFillPolygons<RetType> {
  fn toFillPolygons(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  QList<QPolygonF> QPainterPath::toFillPolygons(const QMatrix & matrix);
impl<'a> /*trait*/ QPainterPath_toFillPolygons<()> for (&'a QMatrix) {
  fn toFillPolygons(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath14toFillPolygonsERK7QMatrix()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK12QPainterPath14toFillPolygonsERK7QMatrix(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPainterPath QPainterPath::translated(const QPointF & offset);
impl<'a> /*trait*/ QPainterPath_translated<QPainterPath> for (&'a QPointF) {
  fn translated(self , rsthis: & QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10translatedERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath10translatedERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainterPath::quadTo(const QPointF & ctrlPt, const QPointF & endPt);
impl /*struct*/ QPainterPath {
  pub fn quadTo<RetType, T: QPainterPath_quadTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.quadTo(self);
    // return 1;
  }
}

pub trait QPainterPath_quadTo<RetType> {
  fn quadTo(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::quadTo(const QPointF & ctrlPt, const QPointF & endPt);
impl<'a> /*trait*/ QPainterPath_quadTo<()> for (&'a QPointF, &'a QPointF) {
  fn quadTo(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath6quadToERK7QPointFS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath6quadToERK7QPointFS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QList<QPolygonF> QPainterPath::toFillPolygons(const QTransform & matrix);
impl<'a> /*trait*/ QPainterPath_toFillPolygons<()> for (&'a QTransform) {
  fn toFillPolygons(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath14toFillPolygonsERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK12QPainterPath14toFillPolygonsERK10QTransform(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainterPath::arcTo(qreal x, qreal y, qreal w, qreal h, qreal startAngle, qreal arcLength);
impl /*struct*/ QPainterPath {
  pub fn arcTo<RetType, T: QPainterPath_arcTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.arcTo(self);
    // return 1;
  }
}

pub trait QPainterPath_arcTo<RetType> {
  fn arcTo(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::arcTo(qreal x, qreal y, qreal w, qreal h, qreal startAngle, qreal arcLength);
impl<'a> /*trait*/ QPainterPath_arcTo<()> for (f64, f64, f64, f64, f64, f64) {
  fn arcTo(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath5arcToEdddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
     unsafe {_ZN12QPainterPath5arcToEdddddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  void QPainterPath::addRect(const QRectF & rect);
impl /*struct*/ QPainterPath {
  pub fn addRect<RetType, T: QPainterPath_addRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addRect(self);
    // return 1;
  }
}

pub trait QPainterPath_addRect<RetType> {
  fn addRect(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::addRect(const QRectF & rect);
impl<'a> /*trait*/ QPainterPath_addRect<()> for (&'a QRectF) {
  fn addRect(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath7addRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath7addRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainterPath::addRoundRect(const QRectF & rect, int xRnd, int yRnd);
impl /*struct*/ QPainterPath {
  pub fn addRoundRect<RetType, T: QPainterPath_addRoundRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addRoundRect(self);
    // return 1;
  }
}

pub trait QPainterPath_addRoundRect<RetType> {
  fn addRoundRect(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::addRoundRect(const QRectF & rect, int xRnd, int yRnd);
impl<'a> /*trait*/ QPainterPath_addRoundRect<()> for (&'a QRectF, i32, i32) {
  fn addRoundRect(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath12addRoundRectERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN12QPainterPath12addRoundRectERK6QRectFii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainterPath::addText(qreal x, qreal y, const QFont & f, const QString & text);
impl /*struct*/ QPainterPath {
  pub fn addText<RetType, T: QPainterPath_addText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addText(self);
    // return 1;
  }
}

pub trait QPainterPath_addText<RetType> {
  fn addText(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::addText(qreal x, qreal y, const QFont & f, const QString & text);
impl<'a> /*trait*/ QPainterPath_addText<()> for (f64, f64, &'a QFont, &'a QString) {
  fn addText(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath7addTextEddRK5QFontRK7QString()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath7addTextEddRK5QFontRK7QString(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  bool QPainterPath::intersects(const QRectF & rect);
impl /*struct*/ QPainterPath {
  pub fn intersects<RetType, T: QPainterPath_intersects<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.intersects(self);
    // return 1;
  }
}

pub trait QPainterPath_intersects<RetType> {
  fn intersects(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  bool QPainterPath::intersects(const QRectF & rect);
impl<'a> /*trait*/ QPainterPath_intersects<i8> for (&'a QRectF) {
  fn intersects(self , rsthis: & QPainterPath) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10intersectsERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath10intersectsERK6QRectF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QPainterPath::contains(const QPointF & pt);
impl /*struct*/ QPainterPath {
  pub fn contains<RetType, T: QPainterPath_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QPainterPath_contains<RetType> {
  fn contains(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  bool QPainterPath::contains(const QPointF & pt);
impl<'a> /*trait*/ QPainterPath_contains<i8> for (&'a QPointF) {
  fn contains(self , rsthis: & QPainterPath) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPainterPath::arcTo(const QRectF & rect, qreal startAngle, qreal arcLength);
impl<'a> /*trait*/ QPainterPath_arcTo<()> for (&'a QRectF, f64, f64) {
  fn arcTo(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath5arcToERK6QRectFdd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
     unsafe {_ZN12QPainterPath5arcToERK6QRectFdd(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainterPath::addRoundRect(const QRectF & rect, int roundness);
impl<'a> /*trait*/ QPainterPath_addRoundRect<()> for (&'a QRectF, i32) {
  fn addRoundRect(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath12addRoundRectERK6QRectFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QPainterPath12addRoundRectERK6QRectFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainterPath::addEllipse(const QPointF & center, qreal rx, qreal ry);
impl /*struct*/ QPainterPath {
  pub fn addEllipse<RetType, T: QPainterPath_addEllipse<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addEllipse(self);
    // return 1;
  }
}

pub trait QPainterPath_addEllipse<RetType> {
  fn addEllipse(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::addEllipse(const QPointF & center, qreal rx, qreal ry);
impl<'a> /*trait*/ QPainterPath_addEllipse<()> for (&'a QPointF, f64, f64) {
  fn addEllipse(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath10addEllipseERK7QPointFdd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
     unsafe {_ZN12QPainterPath10addEllipseERK7QPointFdd(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainterPath::lineTo(qreal x, qreal y);
impl /*struct*/ QPainterPath {
  pub fn lineTo<RetType, T: QPainterPath_lineTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineTo(self);
    // return 1;
  }
}

pub trait QPainterPath_lineTo<RetType> {
  fn lineTo(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::lineTo(qreal x, qreal y);
impl<'a> /*trait*/ QPainterPath_lineTo<()> for (f64, f64) {
  fn lineTo(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath6lineToEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN12QPainterPath6lineToEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainterPath::cubicTo(const QPointF & ctrlPt1, const QPointF & ctrlPt2, const QPointF & endPt);
impl /*struct*/ QPainterPath {
  pub fn cubicTo<RetType, T: QPainterPath_cubicTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cubicTo(self);
    // return 1;
  }
}

pub trait QPainterPath_cubicTo<RetType> {
  fn cubicTo(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::cubicTo(const QPointF & ctrlPt1, const QPointF & ctrlPt2, const QPointF & endPt);
impl<'a> /*trait*/ QPainterPath_cubicTo<()> for (&'a QPointF, &'a QPointF, &'a QPointF) {
  fn cubicTo(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath7cubicToERK7QPointFS2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath7cubicToERK7QPointFS2_S2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  qreal QPainterPath::slopeAtPercent(qreal t);
impl /*struct*/ QPainterPath {
  pub fn slopeAtPercent<RetType, T: QPainterPath_slopeAtPercent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.slopeAtPercent(self);
    // return 1;
  }
}

pub trait QPainterPath_slopeAtPercent<RetType> {
  fn slopeAtPercent(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  qreal QPainterPath::slopeAtPercent(qreal t);
impl<'a> /*trait*/ QPainterPath_slopeAtPercent<f64> for (f64) {
  fn slopeAtPercent(self , rsthis: & QPainterPath) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath14slopeAtPercentEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK12QPainterPath14slopeAtPercentEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPainterPath::addEllipse(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QPainterPath_addEllipse<()> for (f64, f64, f64, f64) {
  fn addEllipse(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath10addEllipseEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN12QPainterPath10addEllipseEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QPainterPath::addRoundRect(qreal x, qreal y, qreal w, qreal h, int roundness);
impl<'a> /*trait*/ QPainterPath_addRoundRect<()> for (f64, f64, f64, f64, i32) {
  fn addRoundRect(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath12addRoundRectEddddi()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_int;
     unsafe {_ZN12QPainterPath12addRoundRectEddddi(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

  // proto:  void QPainterPath::QPainterPath(const QPointF & startPoint);
impl /*struct*/ QPainterPath {
  pub fn New<T: QPainterPath_New>(value: T) -> QPainterPath {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QPainterPath_New {
  fn New(self) -> QPainterPath;
}

  // proto:  void QPainterPath::QPainterPath(const QPointF & startPoint);
impl<'a> /*trait*/ QPainterPath_New for (&'a QPointF) {
  fn New(self) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPathC1ERK7QPointF()};
    let ctysz: c_int = unsafe{QPainterPath_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN12QPainterPathC1ERK7QPointF(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN12QPainterPathC1ERK7QPointF(arg0)};
    let rsthis = QPainterPath{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPainterPath QPainterPath::intersected(const QPainterPath & r);
impl /*struct*/ QPainterPath {
  pub fn intersected<RetType, T: QPainterPath_intersected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.intersected(self);
    // return 1;
  }
}

pub trait QPainterPath_intersected<RetType> {
  fn intersected(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  QPainterPath QPainterPath::intersected(const QPainterPath & r);
impl<'a> /*trait*/ QPainterPath_intersected<QPainterPath> for (&'a QPainterPath) {
  fn intersected(self , rsthis: & QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath11intersectedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath11intersectedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainterPath::translate(qreal dx, qreal dy);
impl /*struct*/ QPainterPath {
  pub fn translate<RetType, T: QPainterPath_translate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.translate(self);
    // return 1;
  }
}

pub trait QPainterPath_translate<RetType> {
  fn translate(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QPainterPath_translate<()> for (f64, f64) {
  fn translate(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN12QPainterPath9translateEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainterPath::addPolygon(const QPolygonF & polygon);
impl /*struct*/ QPainterPath {
  pub fn addPolygon<RetType, T: QPainterPath_addPolygon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addPolygon(self);
    // return 1;
  }
}

pub trait QPainterPath_addPolygon<RetType> {
  fn addPolygon(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::addPolygon(const QPolygonF & polygon);
impl<'a> /*trait*/ QPainterPath_addPolygon<()> for (&'a QPolygonF) {
  fn addPolygon(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath10addPolygonERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath10addPolygonERK9QPolygonF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainterPath::translate(const QPointF & offset);
impl<'a> /*trait*/ QPainterPath_translate<()> for (&'a QPointF) {
  fn translate(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath9translateERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath9translateERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPolygonF QPainterPath::toFillPolygon(const QTransform & matrix);
impl<'a> /*trait*/ QPainterPath_toFillPolygon<QPolygonF> for (&'a QTransform) {
  fn toFillPolygon(self , rsthis: & QPainterPath) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath13toFillPolygonERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath13toFillPolygonERK10QTransform(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainterPath::addPath(const QPainterPath & path);
impl /*struct*/ QPainterPath {
  pub fn addPath<RetType, T: QPainterPath_addPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addPath(self);
    // return 1;
  }
}

pub trait QPainterPath_addPath<RetType> {
  fn addPath(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::addPath(const QPainterPath & path);
impl<'a> /*trait*/ QPainterPath_addPath<()> for (&'a QPainterPath) {
  fn addPath(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath7addPathERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath7addPathERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainterPath::quadTo(qreal ctrlPtx, qreal ctrlPty, qreal endPtx, qreal endPty);
impl<'a> /*trait*/ QPainterPath_quadTo<()> for (f64, f64, f64, f64) {
  fn quadTo(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath6quadToEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN12QPainterPath6quadToEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  int QPainterPath::elementCount();
impl /*struct*/ QPainterPath {
  pub fn elementCount<RetType, T: QPainterPath_elementCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.elementCount(self);
    // return 1;
  }
}

pub trait QPainterPath_elementCount<RetType> {
  fn elementCount(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  int QPainterPath::elementCount();
impl<'a> /*trait*/ QPainterPath_elementCount<i32> for () {
  fn elementCount(self , rsthis: & QPainterPath) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath12elementCountEv()};
    let mut ret = unsafe {_ZNK12QPainterPath12elementCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QPainterPath QPainterPath::simplified();
impl /*struct*/ QPainterPath {
  pub fn simplified<RetType, T: QPainterPath_simplified<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.simplified(self);
    // return 1;
  }
}

pub trait QPainterPath_simplified<RetType> {
  fn simplified(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  QPainterPath QPainterPath::simplified();
impl<'a> /*trait*/ QPainterPath_simplified<QPainterPath> for () {
  fn simplified(self , rsthis: & QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10simplifiedEv()};
    let mut ret = unsafe {_ZNK12QPainterPath10simplifiedEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPainterPath::contains(const QRectF & rect);
impl<'a> /*trait*/ QPainterPath_contains<i8> for (&'a QRectF) {
  fn contains(self , rsthis: & QPainterPath) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath8containsERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath8containsERK6QRectF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QPainterPath::length();
impl /*struct*/ QPainterPath {
  pub fn length<RetType, T: QPainterPath_length<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.length(self);
    // return 1;
  }
}

pub trait QPainterPath_length<RetType> {
  fn length(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  qreal QPainterPath::length();
impl<'a> /*trait*/ QPainterPath_length<f64> for () {
  fn length(self , rsthis: & QPainterPath) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath6lengthEv()};
    let mut ret = unsafe {_ZNK12QPainterPath6lengthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPainterPath::connectPath(const QPainterPath & path);
impl /*struct*/ QPainterPath {
  pub fn connectPath<RetType, T: QPainterPath_connectPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.connectPath(self);
    // return 1;
  }
}

pub trait QPainterPath_connectPath<RetType> {
  fn connectPath(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::connectPath(const QPainterPath & path);
impl<'a> /*trait*/ QPainterPath_connectPath<()> for (&'a QPainterPath) {
  fn connectPath(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath11connectPathERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath11connectPathERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainterPath::addRegion(const QRegion & region);
impl /*struct*/ QPainterPath {
  pub fn addRegion<RetType, T: QPainterPath_addRegion<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addRegion(self);
    // return 1;
  }
}

pub trait QPainterPath_addRegion<RetType> {
  fn addRegion(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::addRegion(const QRegion & region);
impl<'a> /*trait*/ QPainterPath_addRegion<()> for (&'a QRegion) {
  fn addRegion(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath9addRegionERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath9addRegionERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPointF QPainterPath::currentPosition();
impl /*struct*/ QPainterPath {
  pub fn currentPosition<RetType, T: QPainterPath_currentPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentPosition(self);
    // return 1;
  }
}

pub trait QPainterPath_currentPosition<RetType> {
  fn currentPosition(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  QPointF QPainterPath::currentPosition();
impl<'a> /*trait*/ QPainterPath_currentPosition<QPointF> for () {
  fn currentPosition(self , rsthis: & QPainterPath) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath15currentPositionEv()};
    let mut ret = unsafe {_ZNK12QPainterPath15currentPositionEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPainterPath QPainterPath::toReversed();
impl /*struct*/ QPainterPath {
  pub fn toReversed<RetType, T: QPainterPath_toReversed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toReversed(self);
    // return 1;
  }
}

pub trait QPainterPath_toReversed<RetType> {
  fn toReversed(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  QPainterPath QPainterPath::toReversed();
impl<'a> /*trait*/ QPainterPath_toReversed<QPainterPath> for () {
  fn toReversed(self , rsthis: & QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10toReversedEv()};
    let mut ret = unsafe {_ZNK12QPainterPath10toReversedEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainterPath::addRoundRect(qreal x, qreal y, qreal w, qreal h, int xRnd, int yRnd);
impl<'a> /*trait*/ QPainterPath_addRoundRect<()> for (f64, f64, f64, f64, i32, i32) {
  fn addRoundRect(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath12addRoundRectEddddii()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
     unsafe {_ZN12QPainterPath12addRoundRectEddddii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  QRectF QPainterPath::boundingRect();
impl /*struct*/ QPainterPath {
  pub fn boundingRect<RetType, T: QPainterPath_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QPainterPath_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  QRectF QPainterPath::boundingRect();
impl<'a> /*trait*/ QPainterPath_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: & QPainterPath) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath12boundingRectEv()};
    let mut ret = unsafe {_ZNK12QPainterPath12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPainterPath::contains(const QPainterPath & p);
impl<'a> /*trait*/ QPainterPath_contains<i8> for (&'a QPainterPath) {
  fn contains(self , rsthis: & QPainterPath) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath8containsERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath8containsERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPainterPath::moveTo(qreal x, qreal y);
impl /*struct*/ QPainterPath {
  pub fn moveTo<RetType, T: QPainterPath_moveTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.moveTo(self);
    // return 1;
  }
}

pub trait QPainterPath_moveTo<RetType> {
  fn moveTo(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::moveTo(qreal x, qreal y);
impl<'a> /*trait*/ QPainterPath_moveTo<()> for (f64, f64) {
  fn moveTo(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath6moveToEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN12QPainterPath6moveToEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QPainterPath QPainterPath::subtracted(const QPainterPath & r);
impl /*struct*/ QPainterPath {
  pub fn subtracted<RetType, T: QPainterPath_subtracted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.subtracted(self);
    // return 1;
  }
}

pub trait QPainterPath_subtracted<RetType> {
  fn subtracted(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  QPainterPath QPainterPath::subtracted(const QPainterPath & r);
impl<'a> /*trait*/ QPainterPath_subtracted<QPainterPath> for (&'a QPainterPath) {
  fn subtracted(self , rsthis: & QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10subtractedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath10subtractedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainterPath::QPainterPath();
impl<'a> /*trait*/ QPainterPath_New for () {
  fn New(self) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPathC1Ev()};
    let ctysz: c_int = unsafe{QPainterPath_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN12QPainterPathC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN12QPainterPathC1Ev()};
    let rsthis = QPainterPath{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPainterPath::addText(const QPointF & point, const QFont & f, const QString & text);
impl<'a> /*trait*/ QPainterPath_addText<()> for (&'a QPointF, &'a QFont, &'a QString) {
  fn addText(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath7addTextERK7QPointFRK5QFontRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath7addTextERK7QPointFRK5QFontRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainterPath::QPainterPath(const QPainterPath & other);
impl<'a> /*trait*/ QPainterPath_New for (&'a QPainterPath) {
  fn New(self) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPathC1ERKS_()};
    let ctysz: c_int = unsafe{QPainterPath_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN12QPainterPathC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN12QPainterPathC1ERKS_(arg0)};
    let rsthis = QPainterPath{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPointF QPainterPath::pointAtPercent(qreal t);
impl /*struct*/ QPainterPath {
  pub fn pointAtPercent<RetType, T: QPainterPath_pointAtPercent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pointAtPercent(self);
    // return 1;
  }
}

pub trait QPainterPath_pointAtPercent<RetType> {
  fn pointAtPercent(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  QPointF QPainterPath::pointAtPercent(qreal t);
impl<'a> /*trait*/ QPainterPath_pointAtPercent<QPointF> for (f64) {
  fn pointAtPercent(self , rsthis: & QPainterPath) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath14pointAtPercentEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK12QPainterPath14pointAtPercentEd(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QPainterPath::percentAtLength(qreal t);
impl /*struct*/ QPainterPath {
  pub fn percentAtLength<RetType, T: QPainterPath_percentAtLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.percentAtLength(self);
    // return 1;
  }
}

pub trait QPainterPath_percentAtLength<RetType> {
  fn percentAtLength(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  qreal QPainterPath::percentAtLength(qreal t);
impl<'a> /*trait*/ QPainterPath_percentAtLength<f64> for (f64) {
  fn percentAtLength(self , rsthis: & QPainterPath) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath15percentAtLengthEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK12QPainterPath15percentAtLengthEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPainterPath::cubicTo(qreal ctrlPt1x, qreal ctrlPt1y, qreal ctrlPt2x, qreal ctrlPt2y, qreal endPtx, qreal endPty);
impl<'a> /*trait*/ QPainterPath_cubicTo<()> for (f64, f64, f64, f64, f64, f64) {
  fn cubicTo(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath7cubicToEdddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
     unsafe {_ZN12QPainterPath7cubicToEdddddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  void QPainterPath::lineTo(const QPointF & p);
impl<'a> /*trait*/ QPainterPath_lineTo<()> for (&'a QPointF) {
  fn lineTo(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath6lineToERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath6lineToERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPainterPath QPainterPath::subtractedInverted(const QPainterPath & r);
impl /*struct*/ QPainterPath {
  pub fn subtractedInverted<RetType, T: QPainterPath_subtractedInverted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.subtractedInverted(self);
    // return 1;
  }
}

pub trait QPainterPath_subtractedInverted<RetType> {
  fn subtractedInverted(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  QPainterPath QPainterPath::subtractedInverted(const QPainterPath & r);
impl<'a> /*trait*/ QPainterPath_subtractedInverted<QPainterPath> for (&'a QPainterPath) {
  fn subtractedInverted(self , rsthis: & QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath18subtractedInvertedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath18subtractedInvertedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPainterPath::intersects(const QPainterPath & p);
impl<'a> /*trait*/ QPainterPath_intersects<i8> for (&'a QPainterPath) {
  fn intersects(self , rsthis: & QPainterPath) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10intersectsERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath10intersectsERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPainterPath::arcMoveTo(qreal x, qreal y, qreal w, qreal h, qreal angle);
impl /*struct*/ QPainterPath {
  pub fn arcMoveTo<RetType, T: QPainterPath_arcMoveTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.arcMoveTo(self);
    // return 1;
  }
}

pub trait QPainterPath_arcMoveTo<RetType> {
  fn arcMoveTo(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::arcMoveTo(qreal x, qreal y, qreal w, qreal h, qreal angle);
impl<'a> /*trait*/ QPainterPath_arcMoveTo<()> for (f64, f64, f64, f64, f64) {
  fn arcMoveTo(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath9arcMoveToEddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
     unsafe {_ZN12QPainterPath9arcMoveToEddddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

  // proto:  bool QPainterPath::isEmpty();
impl /*struct*/ QPainterPath {
  pub fn isEmpty<RetType, T: QPainterPath_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QPainterPath_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  bool QPainterPath::isEmpty();
impl<'a> /*trait*/ QPainterPath_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QPainterPath) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath7isEmptyEv()};
    let mut ret = unsafe {_ZNK12QPainterPath7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPainterPath::addRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QPainterPath_addRect<()> for (f64, f64, f64, f64) {
  fn addRect(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath7addRectEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN12QPainterPath7addRectEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QPainterPath::arcMoveTo(const QRectF & rect, qreal angle);
impl<'a> /*trait*/ QPainterPath_arcMoveTo<()> for (&'a QRectF, f64) {
  fn arcMoveTo(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath9arcMoveToERK6QRectFd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
     unsafe {_ZN12QPainterPath9arcMoveToERK6QRectFd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QList<QPolygonF> QPainterPath::toSubpathPolygons(const QMatrix & matrix);
impl<'a> /*trait*/ QPainterPath_toSubpathPolygons<()> for (&'a QMatrix) {
  fn toSubpathPolygons(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath17toSubpathPolygonsERK7QMatrix()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK12QPainterPath17toSubpathPolygonsERK7QMatrix(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPainterPath QPainterPath::united(const QPainterPath & r);
impl /*struct*/ QPainterPath {
  pub fn united<RetType, T: QPainterPath_united<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.united(self);
    // return 1;
  }
}

pub trait QPainterPath_united<RetType> {
  fn united(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  QPainterPath QPainterPath::united(const QPainterPath & r);
impl<'a> /*trait*/ QPainterPath_united<QPainterPath> for (&'a QPainterPath) {
  fn united(self , rsthis: & QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath6unitedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath6unitedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainterPath::addEllipse(const QRectF & rect);
impl<'a> /*trait*/ QPainterPath_addEllipse<()> for (&'a QRectF) {
  fn addEllipse(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath10addEllipseERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath10addEllipseERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainterPath::moveTo(const QPointF & p);
impl<'a> /*trait*/ QPainterPath_moveTo<()> for (&'a QPointF) {
  fn moveTo(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath6moveToERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath6moveToERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QPainterPath::angleAtPercent(qreal t);
impl /*struct*/ QPainterPath {
  pub fn angleAtPercent<RetType, T: QPainterPath_angleAtPercent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.angleAtPercent(self);
    // return 1;
  }
}

pub trait QPainterPath_angleAtPercent<RetType> {
  fn angleAtPercent(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  qreal QPainterPath::angleAtPercent(qreal t);
impl<'a> /*trait*/ QPainterPath_angleAtPercent<f64> for (f64) {
  fn angleAtPercent(self , rsthis: & QPainterPath) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath14angleAtPercentEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK12QPainterPath14angleAtPercentEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPainterPath::closeSubpath();
impl /*struct*/ QPainterPath {
  pub fn closeSubpath<RetType, T: QPainterPath_closeSubpath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.closeSubpath(self);
    // return 1;
  }
}

pub trait QPainterPath_closeSubpath<RetType> {
  fn closeSubpath(self , rsthis: & QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::closeSubpath();
impl<'a> /*trait*/ QPainterPath_closeSubpath<()> for () {
  fn closeSubpath(self , rsthis: & QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath12closeSubpathEv()};
     unsafe {_ZN12QPainterPath12closeSubpathEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn inheritFrom(qthis: *mut c_void) -> QPainterPathStroker {
    return QPainterPathStroker{qclsinst: qthis};
  }
}
  // proto:  qreal QPainterPathStroker::curveThreshold();
impl /*struct*/ QPainterPathStroker {
  pub fn curveThreshold<RetType, T: QPainterPathStroker_curveThreshold<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.curveThreshold(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_curveThreshold<RetType> {
  fn curveThreshold(self , rsthis: & QPainterPathStroker) -> RetType;
}

  // proto:  qreal QPainterPathStroker::curveThreshold();
impl<'a> /*trait*/ QPainterPathStroker_curveThreshold<f64> for () {
  fn curveThreshold(self , rsthis: & QPainterPathStroker) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QPainterPathStroker14curveThresholdEv()};
    let mut ret = unsafe {_ZNK19QPainterPathStroker14curveThresholdEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPainterPathStroker::QPainterPathStroker();
impl /*struct*/ QPainterPathStroker {
  pub fn New<T: QPainterPathStroker_New>(value: T) -> QPainterPathStroker {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QPainterPathStroker_New {
  fn New(self) -> QPainterPathStroker;
}

  // proto:  void QPainterPathStroker::QPainterPathStroker();
impl<'a> /*trait*/ QPainterPathStroker_New for () {
  fn New(self) -> QPainterPathStroker {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStrokerC1Ev()};
    let ctysz: c_int = unsafe{QPainterPathStroker_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN19QPainterPathStrokerC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN19QPainterPathStrokerC1Ev()};
    let rsthis = QPainterPathStroker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPainterPathStroker::setDashOffset(qreal offset);
impl /*struct*/ QPainterPathStroker {
  pub fn setDashOffset<RetType, T: QPainterPathStroker_setDashOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDashOffset(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_setDashOffset<RetType> {
  fn setDashOffset(self , rsthis: & QPainterPathStroker) -> RetType;
}

  // proto:  void QPainterPathStroker::setDashOffset(qreal offset);
impl<'a> /*trait*/ QPainterPathStroker_setDashOffset<()> for (f64) {
  fn setDashOffset(self , rsthis: & QPainterPathStroker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStroker13setDashOffsetEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QPainterPathStroker13setDashOffsetEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainterPathStroker::QPainterPathStroker(const QPen & pen);
impl<'a> /*trait*/ QPainterPathStroker_New for (&'a QPen) {
  fn New(self) -> QPainterPathStroker {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStrokerC1ERK4QPen()};
    let ctysz: c_int = unsafe{QPainterPathStroker_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QPainterPathStrokerC1ERK4QPen(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN19QPainterPathStrokerC1ERK4QPen(arg0)};
    let rsthis = QPainterPathStroker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPainterPathStroker::setCurveThreshold(qreal threshold);
impl /*struct*/ QPainterPathStroker {
  pub fn setCurveThreshold<RetType, T: QPainterPathStroker_setCurveThreshold<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurveThreshold(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_setCurveThreshold<RetType> {
  fn setCurveThreshold(self , rsthis: & QPainterPathStroker) -> RetType;
}

  // proto:  void QPainterPathStroker::setCurveThreshold(qreal threshold);
impl<'a> /*trait*/ QPainterPathStroker_setCurveThreshold<()> for (f64) {
  fn setCurveThreshold(self , rsthis: & QPainterPathStroker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStroker17setCurveThresholdEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QPainterPathStroker17setCurveThresholdEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QPainterPathStroker::dashOffset();
impl /*struct*/ QPainterPathStroker {
  pub fn dashOffset<RetType, T: QPainterPathStroker_dashOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dashOffset(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_dashOffset<RetType> {
  fn dashOffset(self , rsthis: & QPainterPathStroker) -> RetType;
}

  // proto:  qreal QPainterPathStroker::dashOffset();
impl<'a> /*trait*/ QPainterPathStroker_dashOffset<f64> for () {
  fn dashOffset(self , rsthis: & QPainterPathStroker) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QPainterPathStroker10dashOffsetEv()};
    let mut ret = unsafe {_ZNK19QPainterPathStroker10dashOffsetEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPainterPathStroker::setWidth(qreal width);
impl /*struct*/ QPainterPathStroker {
  pub fn setWidth<RetType, T: QPainterPathStroker_setWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWidth(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_setWidth<RetType> {
  fn setWidth(self , rsthis: & QPainterPathStroker) -> RetType;
}

  // proto:  void QPainterPathStroker::setWidth(qreal width);
impl<'a> /*trait*/ QPainterPathStroker_setWidth<()> for (f64) {
  fn setWidth(self , rsthis: & QPainterPathStroker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStroker8setWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QPainterPathStroker8setWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainterPathStroker::~QPainterPathStroker();
impl /*struct*/ QPainterPathStroker {
  pub fn Free<RetType, T: QPainterPathStroker_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_Free<RetType> {
  fn Free(self , rsthis: & QPainterPathStroker) -> RetType;
}

  // proto:  void QPainterPathStroker::~QPainterPathStroker();
impl<'a> /*trait*/ QPainterPathStroker_Free<()> for () {
  fn Free(self , rsthis: & QPainterPathStroker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStrokerD0Ev()};
     unsafe {_ZN19QPainterPathStrokerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QPainterPathStroker::width();
impl /*struct*/ QPainterPathStroker {
  pub fn width<RetType, T: QPainterPathStroker_width<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_width<RetType> {
  fn width(self , rsthis: & QPainterPathStroker) -> RetType;
}

  // proto:  qreal QPainterPathStroker::width();
impl<'a> /*trait*/ QPainterPathStroker_width<f64> for () {
  fn width(self , rsthis: & QPainterPathStroker) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QPainterPathStroker5widthEv()};
    let mut ret = unsafe {_ZNK19QPainterPathStroker5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPainterPathStroker::QPainterPathStroker(const QPainterPathStroker & );
impl<'a> /*trait*/ QPainterPathStroker_New for (&'a QPainterPathStroker) {
  fn New(self) -> QPainterPathStroker {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStrokerC1ERKS_()};
    let ctysz: c_int = unsafe{QPainterPathStroker_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QPainterPathStrokerC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN19QPainterPathStrokerC1ERKS_(arg0)};
    let rsthis = QPainterPathStroker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QPainterPathStroker::miterLimit();
impl /*struct*/ QPainterPathStroker {
  pub fn miterLimit<RetType, T: QPainterPathStroker_miterLimit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.miterLimit(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_miterLimit<RetType> {
  fn miterLimit(self , rsthis: & QPainterPathStroker) -> RetType;
}

  // proto:  qreal QPainterPathStroker::miterLimit();
impl<'a> /*trait*/ QPainterPathStroker_miterLimit<f64> for () {
  fn miterLimit(self , rsthis: & QPainterPathStroker) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QPainterPathStroker10miterLimitEv()};
    let mut ret = unsafe {_ZNK19QPainterPathStroker10miterLimitEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QVector<qreal> QPainterPathStroker::dashPattern();
impl /*struct*/ QPainterPathStroker {
  pub fn dashPattern<RetType, T: QPainterPathStroker_dashPattern<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dashPattern(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_dashPattern<RetType> {
  fn dashPattern(self , rsthis: & QPainterPathStroker) -> RetType;
}

  // proto:  QVector<qreal> QPainterPathStroker::dashPattern();
impl<'a> /*trait*/ QPainterPathStroker_dashPattern<()> for () {
  fn dashPattern(self , rsthis: & QPainterPathStroker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QPainterPathStroker11dashPatternEv()};
     unsafe {_ZNK19QPainterPathStroker11dashPatternEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPainterPath QPainterPathStroker::createStroke(const QPainterPath & path);
impl /*struct*/ QPainterPathStroker {
  pub fn createStroke<RetType, T: QPainterPathStroker_createStroke<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createStroke(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_createStroke<RetType> {
  fn createStroke(self , rsthis: & QPainterPathStroker) -> RetType;
}

  // proto:  QPainterPath QPainterPathStroker::createStroke(const QPainterPath & path);
impl<'a> /*trait*/ QPainterPathStroker_createStroke<QPainterPath> for (&'a QPainterPath) {
  fn createStroke(self , rsthis: & QPainterPathStroker) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QPainterPathStroker12createStrokeERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QPainterPathStroker12createStrokeERK12QPainterPath(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainterPathStroker::setMiterLimit(qreal length);
impl /*struct*/ QPainterPathStroker {
  pub fn setMiterLimit<RetType, T: QPainterPathStroker_setMiterLimit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMiterLimit(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_setMiterLimit<RetType> {
  fn setMiterLimit(self , rsthis: & QPainterPathStroker) -> RetType;
}

  // proto:  void QPainterPathStroker::setMiterLimit(qreal length);
impl<'a> /*trait*/ QPainterPathStroker_setMiterLimit<()> for (f64) {
  fn setMiterLimit(self , rsthis: & QPainterPathStroker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStroker13setMiterLimitEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QPainterPathStroker13setMiterLimitEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

