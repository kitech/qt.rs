// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmatrix::QMatrix;
use super::qpolygonf::QPolygonF;
use super::qtransform::QTransform;
use super::qrectf::QRectF;
use super::qpointf::QPointF;
use super::qfont::QFont;
use super::qstring::QString;
use super::qregion::QRegion;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QPainterPath::setElementPositionAt(int i, qreal x, qreal y);
  fn _ZN12QPainterPath20setElementPositionAtEidd(qthis: *mut c_void, arg0: c_int, arg1: c_double, arg2: c_double) ;
  // proto:  QPolygonF QPainterPath::toFillPolygon(const QMatrix & matrix);
  fn _ZNK12QPainterPath13toFillPolygonERK7QMatrix(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPainterPath QPainterPath::translated(qreal dx, qreal dy);
  fn _ZNK12QPainterPath10translatedEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  void QPainterPath::FreeQPainterPath();
  fn _ZN12QPainterPathD0Ev(qthis: *mut c_void) ;
  // proto:  QList<QPolygonF> QPainterPath::toSubpathPolygons(const QTransform & matrix);
  fn _ZNK12QPainterPath17toSubpathPolygonsERK10QTransform(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRectF QPainterPath::controlPointRect();
  fn _ZNK12QPainterPath16controlPointRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QList<QPolygonF> QPainterPath::toFillPolygons(const QMatrix & matrix);
  fn _ZNK12QPainterPath14toFillPolygonsERK7QMatrix(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPainterPath QPainterPath::translated(const QPointF & offset);
  fn _ZNK12QPainterPath10translatedERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::quadTo(const QPointF & ctrlPt, const QPointF & endPt);
  fn _ZN12QPainterPath6quadToERK7QPointFS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QList<QPolygonF> QPainterPath::toFillPolygons(const QTransform & matrix);
  fn _ZNK12QPainterPath14toFillPolygonsERK10QTransform(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainterPath::arcTo(qreal x, qreal y, qreal w, qreal h, qreal startAngle, qreal arcLength);
  fn _ZN12QPainterPath5arcToEdddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double) ;
  // proto:  void QPainterPath::addRect(const QRectF & rect);
  fn _ZN12QPainterPath7addRectERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainterPath::addRoundRect(const QRectF & rect, int xRnd, int yRnd);
  fn _ZN12QPainterPath12addRoundRectERK6QRectFii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) ;
  // proto:  void QPainterPath::addText(qreal x, qreal y, const QFont & f, const QString & text);
  fn _ZN12QPainterPath7addTextEddRK5QFontRK7QString(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: *mut c_void, arg3: *mut c_void) ;
  // proto:  bool QPainterPath::intersects(const QRectF & rect);
  fn _ZNK12QPainterPath10intersectsERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QPainterPath::contains(const QPointF & pt);
  fn _ZNK12QPainterPath8containsERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QPainterPath::arcTo(const QRectF & rect, qreal startAngle, qreal arcLength);
  fn _ZN12QPainterPath5arcToERK6QRectFdd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double) ;
  // proto:  void QPainterPath::addRoundRect(const QRectF & rect, int roundness);
  fn _ZN12QPainterPath12addRoundRectERK6QRectFi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QPainterPath::addEllipse(const QPointF & center, qreal rx, qreal ry);
  fn _ZN12QPainterPath10addEllipseERK7QPointFdd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double) ;
  // proto:  void QPainterPath::lineTo(qreal x, qreal y);
  fn _ZN12QPainterPath6lineToEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  void QPainterPath::cubicTo(const QPointF & ctrlPt1, const QPointF & ctrlPt2, const QPointF & endPt);
  fn _ZN12QPainterPath7cubicToERK7QPointFS2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  double QPainterPath::slopeAtPercent(qreal t);
  fn _ZNK12QPainterPath14slopeAtPercentEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  void QPainterPath::addEllipse(qreal x, qreal y, qreal w, qreal h);
  fn _ZN12QPainterPath10addEllipseEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  bool QPainterPath::intersects(const QPainterPath & p);
  fn _ZNK12QPainterPath10intersectsERKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QPainterPath::addRoundRect(qreal x, qreal y, qreal w, qreal h, int roundness);
  fn _ZN12QPainterPath12addRoundRectEddddi(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_int) ;
  // proto:  void QPainterPath::NewQPainterPath(const QPointF & startPoint);
  fn _ZN12QPainterPathC1ERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPainterPath QPainterPath::intersected(const QPainterPath & r);
  fn _ZNK12QPainterPath11intersectedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::translate(qreal dx, qreal dy);
  fn _ZN12QPainterPath9translateEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  void QPainterPath::addPolygon(const QPolygonF & polygon);
  fn _ZN12QPainterPath10addPolygonERK9QPolygonF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainterPath::translate(const QPointF & offset);
  fn _ZN12QPainterPath9translateERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPolygonF QPainterPath::toFillPolygon(const QTransform & matrix);
  fn _ZNK12QPainterPath13toFillPolygonERK10QTransform(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::addPath(const QPainterPath & path);
  fn _ZN12QPainterPath7addPathERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainterPath::quadTo(qreal ctrlPtx, qreal ctrlPty, qreal endPtx, qreal endPty);
  fn _ZN12QPainterPath6quadToEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  int QPainterPath::elementCount();
  fn _ZNK12QPainterPath12elementCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QPainterPath QPainterPath::simplified();
  fn _ZNK12QPainterPath10simplifiedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QPainterPath::contains(const QRectF & rect);
  fn _ZNK12QPainterPath8containsERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  double QPainterPath::length();
  fn _ZNK12QPainterPath6lengthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QPainterPath::connectPath(const QPainterPath & path);
  fn _ZN12QPainterPath11connectPathERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainterPath::addRegion(const QRegion & region);
  fn _ZN12QPainterPath9addRegionERK7QRegion(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPointF QPainterPath::currentPosition();
  fn _ZNK12QPainterPath15currentPositionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPainterPath QPainterPath::toReversed();
  fn _ZNK12QPainterPath10toReversedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::addRoundRect(qreal x, qreal y, qreal w, qreal h, int xRnd, int yRnd);
  fn _ZN12QPainterPath12addRoundRectEddddii(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_int, arg5: c_int) ;
  // proto:  void QPainterPath::swap(QPainterPath & other);
  fn _ZN12QPainterPath4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QPainterPath::contains(const QPainterPath & p);
  fn _ZNK12QPainterPath8containsERKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QPainterPath::moveTo(qreal x, qreal y);
  fn _ZN12QPainterPath6moveToEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  QPainterPath QPainterPath::subtracted(const QPainterPath & r);
  fn _ZNK12QPainterPath10subtractedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::NewQPainterPath();
  fn _ZN12QPainterPathC1Ev(qthis: *mut c_void) ;
  // proto:  void QPainterPath::addText(const QPointF & point, const QFont & f, const QString & text);
  fn _ZN12QPainterPath7addTextERK7QPointFRK5QFontRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QPainterPath::NewQPainterPath(const QPainterPath & other);
  fn _ZN12QPainterPathC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPointF QPainterPath::pointAtPercent(qreal t);
  fn _ZNK12QPainterPath14pointAtPercentEd(qthis: *mut c_void, arg0: c_double) -> *mut c_void;
  // proto:  double QPainterPath::percentAtLength(qreal t);
  fn _ZNK12QPainterPath15percentAtLengthEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  void QPainterPath::cubicTo(qreal ctrlPt1x, qreal ctrlPt1y, qreal ctrlPt2x, qreal ctrlPt2y, qreal endPtx, qreal endPty);
  fn _ZN12QPainterPath7cubicToEdddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double) ;
  // proto:  void QPainterPath::lineTo(const QPointF & p);
  fn _ZN12QPainterPath6lineToERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPainterPath QPainterPath::subtractedInverted(const QPainterPath & r);
  fn _ZNK12QPainterPath18subtractedInvertedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::arcMoveTo(qreal x, qreal y, qreal w, qreal h, qreal angle);
  fn _ZN12QPainterPath9arcMoveToEddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) ;
  // proto:  bool QPainterPath::isEmpty();
  fn _ZNK12QPainterPath7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QPainterPath::addRect(qreal x, qreal y, qreal w, qreal h);
  fn _ZN12QPainterPath7addRectEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  void QPainterPath::arcMoveTo(const QRectF & rect, qreal angle);
  fn _ZN12QPainterPath9arcMoveToERK6QRectFd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double) ;
  // proto:  QList<QPolygonF> QPainterPath::toSubpathPolygons(const QMatrix & matrix);
  fn _ZNK12QPainterPath17toSubpathPolygonsERK7QMatrix(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPainterPath QPainterPath::united(const QPainterPath & r);
  fn _ZNK12QPainterPath6unitedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::addEllipse(const QRectF & rect);
  fn _ZN12QPainterPath10addEllipseERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainterPath::moveTo(const QPointF & p);
  fn _ZN12QPainterPath6moveToERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QPainterPath::angleAtPercent(qreal t);
  fn _ZNK12QPainterPath14angleAtPercentEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  void QPainterPath::closeSubpath();
  fn _ZN12QPainterPath12closeSubpathEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QPainterPath)=1
pub struct QPainterPath {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPainterPath {
  pub fn setElementPositionAt<T: QPainterPath_setElementPositionAt>(&mut self, value: T)  {
     value.setElementPositionAt(self);
    // return 1;
  }
}

pub trait QPainterPath_setElementPositionAt {
  fn setElementPositionAt(self, rsthis: &mut QPainterPath) ;
}

// proto:  void QPainterPath::setElementPositionAt(int i, qreal x, qreal y);
impl<'a> /*trait*/ QPainterPath_setElementPositionAt for (i32, f64, f64) {
  fn setElementPositionAt(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath20setElementPositionAtEidd()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
     unsafe {_ZN12QPainterPath20setElementPositionAtEidd(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn toFillPolygon<T: QPainterPath_toFillPolygon>(&mut self, value: T) -> QPolygonF {
    return value.toFillPolygon(self);
    // return 1;
  }
}

pub trait QPainterPath_toFillPolygon {
  fn toFillPolygon(self, rsthis: &mut QPainterPath) -> QPolygonF;
}

// proto:  QPolygonF QPainterPath::toFillPolygon(const QMatrix & matrix);
impl<'a> /*trait*/ QPainterPath_toFillPolygon for (&'a  QMatrix) {
  fn toFillPolygon(self, rsthis: &mut QPainterPath) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath13toFillPolygonERK7QMatrix()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath13toFillPolygonERK7QMatrix(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn translated<T: QPainterPath_translated>(&mut self, value: T) -> QPainterPath {
    return value.translated(self);
    // return 1;
  }
}

pub trait QPainterPath_translated {
  fn translated(self, rsthis: &mut QPainterPath) -> QPainterPath;
}

// proto:  QPainterPath QPainterPath::translated(qreal dx, qreal dy);
impl<'a> /*trait*/ QPainterPath_translated for (f64, f64) {
  fn translated(self, rsthis: &mut QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10translatedEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK12QPainterPath10translatedEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn FreeQPainterPath<T: QPainterPath_FreeQPainterPath>(&mut self, value: T)  {
     value.FreeQPainterPath(self);
    // return 1;
  }
}

pub trait QPainterPath_FreeQPainterPath {
  fn FreeQPainterPath(self, rsthis: &mut QPainterPath) ;
}

// proto:  void QPainterPath::FreeQPainterPath();
impl<'a> /*trait*/ QPainterPath_FreeQPainterPath for () {
  fn FreeQPainterPath(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPathD0Ev()};
     unsafe {_ZN12QPainterPathD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn toSubpathPolygons<T: QPainterPath_toSubpathPolygons>(&mut self, value: T)  {
     value.toSubpathPolygons(self);
    // return 1;
  }
}

pub trait QPainterPath_toSubpathPolygons {
  fn toSubpathPolygons(self, rsthis: &mut QPainterPath) ;
}

// proto:  QList<QPolygonF> QPainterPath::toSubpathPolygons(const QTransform & matrix);
impl<'a> /*trait*/ QPainterPath_toSubpathPolygons for (&'a  QTransform) {
  fn toSubpathPolygons(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath17toSubpathPolygonsERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK12QPainterPath17toSubpathPolygonsERK10QTransform(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn controlPointRect<T: QPainterPath_controlPointRect>(&mut self, value: T) -> QRectF {
    return value.controlPointRect(self);
    // return 1;
  }
}

pub trait QPainterPath_controlPointRect {
  fn controlPointRect(self, rsthis: &mut QPainterPath) -> QRectF;
}

// proto:  QRectF QPainterPath::controlPointRect();
impl<'a> /*trait*/ QPainterPath_controlPointRect for () {
  fn controlPointRect(self, rsthis: &mut QPainterPath) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath16controlPointRectEv()};
    let mut ret = unsafe {_ZNK12QPainterPath16controlPointRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn toFillPolygons<T: QPainterPath_toFillPolygons>(&mut self, value: T)  {
     value.toFillPolygons(self);
    // return 1;
  }
}

pub trait QPainterPath_toFillPolygons {
  fn toFillPolygons(self, rsthis: &mut QPainterPath) ;
}

// proto:  QList<QPolygonF> QPainterPath::toFillPolygons(const QMatrix & matrix);
impl<'a> /*trait*/ QPainterPath_toFillPolygons for (&'a  QMatrix) {
  fn toFillPolygons(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath14toFillPolygonsERK7QMatrix()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK12QPainterPath14toFillPolygonsERK7QMatrix(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QPainterPath QPainterPath::translated(const QPointF & offset);
impl<'a> /*trait*/ QPainterPath_translated for (&'a  QPointF) {
  fn translated(self, rsthis: &mut QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10translatedERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath10translatedERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn quadTo<T: QPainterPath_quadTo>(&mut self, value: T)  {
     value.quadTo(self);
    // return 1;
  }
}

pub trait QPainterPath_quadTo {
  fn quadTo(self, rsthis: &mut QPainterPath) ;
}

// proto:  void QPainterPath::quadTo(const QPointF & ctrlPt, const QPointF & endPt);
impl<'a> /*trait*/ QPainterPath_quadTo for (&'a  QPointF, &'a  QPointF) {
  fn quadTo(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath6quadToERK7QPointFS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath6quadToERK7QPointFS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QList<QPolygonF> QPainterPath::toFillPolygons(const QTransform & matrix);
impl<'a> /*trait*/ QPainterPath_toFillPolygons for (&'a  QTransform) {
  fn toFillPolygons(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath14toFillPolygonsERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK12QPainterPath14toFillPolygonsERK10QTransform(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn arcTo<T: QPainterPath_arcTo>(&mut self, value: T)  {
     value.arcTo(self);
    // return 1;
  }
}

pub trait QPainterPath_arcTo {
  fn arcTo(self, rsthis: &mut QPainterPath) ;
}

// proto:  void QPainterPath::arcTo(qreal x, qreal y, qreal w, qreal h, qreal startAngle, qreal arcLength);
impl<'a> /*trait*/ QPainterPath_arcTo for (f64, f64, f64, f64, f64, f64) {
  fn arcTo(self, rsthis: &mut QPainterPath)  {
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

impl /*struct*/ QPainterPath {
  pub fn addRect<T: QPainterPath_addRect>(&mut self, value: T)  {
     value.addRect(self);
    // return 1;
  }
}

pub trait QPainterPath_addRect {
  fn addRect(self, rsthis: &mut QPainterPath) ;
}

// proto:  void QPainterPath::addRect(const QRectF & rect);
impl<'a> /*trait*/ QPainterPath_addRect for (&'a  QRectF) {
  fn addRect(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath7addRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath7addRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn addRoundRect<T: QPainterPath_addRoundRect>(&mut self, value: T)  {
     value.addRoundRect(self);
    // return 1;
  }
}

pub trait QPainterPath_addRoundRect {
  fn addRoundRect(self, rsthis: &mut QPainterPath) ;
}

// proto:  void QPainterPath::addRoundRect(const QRectF & rect, int xRnd, int yRnd);
impl<'a> /*trait*/ QPainterPath_addRoundRect for (&'a  QRectF, i32, i32) {
  fn addRoundRect(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath12addRoundRectERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN12QPainterPath12addRoundRectERK6QRectFii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn addText<T: QPainterPath_addText>(&mut self, value: T)  {
     value.addText(self);
    // return 1;
  }
}

pub trait QPainterPath_addText {
  fn addText(self, rsthis: &mut QPainterPath) ;
}

// proto:  void QPainterPath::addText(qreal x, qreal y, const QFont & f, const QString & text);
impl<'a> /*trait*/ QPainterPath_addText for (f64, f64, &'a  QFont, &'a  QString) {
  fn addText(self, rsthis: &mut QPainterPath)  {
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

impl /*struct*/ QPainterPath {
  pub fn intersects<T: QPainterPath_intersects>(&mut self, value: T) -> i8 {
    return value.intersects(self);
    // return 1;
  }
}

pub trait QPainterPath_intersects {
  fn intersects(self, rsthis: &mut QPainterPath) -> i8;
}

// proto:  bool QPainterPath::intersects(const QRectF & rect);
impl<'a> /*trait*/ QPainterPath_intersects for (&'a  QRectF) {
  fn intersects(self, rsthis: &mut QPainterPath) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10intersectsERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath10intersectsERK6QRectF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn contains<T: QPainterPath_contains>(&mut self, value: T) -> i8 {
    return value.contains(self);
    // return 1;
  }
}

pub trait QPainterPath_contains {
  fn contains(self, rsthis: &mut QPainterPath) -> i8;
}

// proto:  bool QPainterPath::contains(const QPointF & pt);
impl<'a> /*trait*/ QPainterPath_contains for (&'a  QPointF) {
  fn contains(self, rsthis: &mut QPainterPath) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QPainterPath::arcTo(const QRectF & rect, qreal startAngle, qreal arcLength);
impl<'a> /*trait*/ QPainterPath_arcTo for (&'a  QRectF, f64, f64) {
  fn arcTo(self, rsthis: &mut QPainterPath)  {
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
impl<'a> /*trait*/ QPainterPath_addRoundRect for (&'a  QRectF, i32) {
  fn addRoundRect(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath12addRoundRectERK6QRectFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QPainterPath12addRoundRectERK6QRectFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn addEllipse<T: QPainterPath_addEllipse>(&mut self, value: T)  {
     value.addEllipse(self);
    // return 1;
  }
}

pub trait QPainterPath_addEllipse {
  fn addEllipse(self, rsthis: &mut QPainterPath) ;
}

// proto:  void QPainterPath::addEllipse(const QPointF & center, qreal rx, qreal ry);
impl<'a> /*trait*/ QPainterPath_addEllipse for (&'a  QPointF, f64, f64) {
  fn addEllipse(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath10addEllipseERK7QPointFdd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
     unsafe {_ZN12QPainterPath10addEllipseERK7QPointFdd(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn lineTo<T: QPainterPath_lineTo>(&mut self, value: T)  {
     value.lineTo(self);
    // return 1;
  }
}

pub trait QPainterPath_lineTo {
  fn lineTo(self, rsthis: &mut QPainterPath) ;
}

// proto:  void QPainterPath::lineTo(qreal x, qreal y);
impl<'a> /*trait*/ QPainterPath_lineTo for (f64, f64) {
  fn lineTo(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath6lineToEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN12QPainterPath6lineToEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn cubicTo<T: QPainterPath_cubicTo>(&mut self, value: T)  {
     value.cubicTo(self);
    // return 1;
  }
}

pub trait QPainterPath_cubicTo {
  fn cubicTo(self, rsthis: &mut QPainterPath) ;
}

// proto:  void QPainterPath::cubicTo(const QPointF & ctrlPt1, const QPointF & ctrlPt2, const QPointF & endPt);
impl<'a> /*trait*/ QPainterPath_cubicTo for (&'a  QPointF, &'a  QPointF, &'a  QPointF) {
  fn cubicTo(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath7cubicToERK7QPointFS2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath7cubicToERK7QPointFS2_S2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn slopeAtPercent<T: QPainterPath_slopeAtPercent>(&mut self, value: T) -> f64 {
    return value.slopeAtPercent(self);
    // return 1;
  }
}

pub trait QPainterPath_slopeAtPercent {
  fn slopeAtPercent(self, rsthis: &mut QPainterPath) -> f64;
}

// proto:  double QPainterPath::slopeAtPercent(qreal t);
impl<'a> /*trait*/ QPainterPath_slopeAtPercent for (f64) {
  fn slopeAtPercent(self, rsthis: &mut QPainterPath) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath14slopeAtPercentEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK12QPainterPath14slopeAtPercentEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

// proto:  void QPainterPath::addEllipse(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QPainterPath_addEllipse for (f64, f64, f64, f64) {
  fn addEllipse(self, rsthis: &mut QPainterPath)  {
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

// proto:  bool QPainterPath::intersects(const QPainterPath & p);
impl<'a> /*trait*/ QPainterPath_intersects for (&'a  QPainterPath) {
  fn intersects(self, rsthis: &mut QPainterPath) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10intersectsERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath10intersectsERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QPainterPath::addRoundRect(qreal x, qreal y, qreal w, qreal h, int roundness);
impl<'a> /*trait*/ QPainterPath_addRoundRect for (f64, f64, f64, f64, i32) {
  fn addRoundRect(self, rsthis: &mut QPainterPath)  {
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

impl /*struct*/ QPainterPath {
  pub fn NewQPainterPath<T: QPainterPath_NewQPainterPath>(value: T) -> QPainterPath {
    let rsthis = value.NewQPainterPath();
    return rsthis;
    // return 1;
  }
}

pub trait QPainterPath_NewQPainterPath {
  fn NewQPainterPath(self) -> QPainterPath;
}

// proto: void QPainterPath::NewQPainterPath(const QPointF & startPoint);
impl<'a> /*trait*/ QPainterPath_NewQPainterPath for (&'a  QPointF) {
  fn NewQPainterPath(self) -> QPainterPath {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPathC1ERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QPainterPathC1ERK7QPointF(qthis, arg0)};
    let rsthis = QPainterPath{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn intersected<T: QPainterPath_intersected>(&mut self, value: T) -> QPainterPath {
    return value.intersected(self);
    // return 1;
  }
}

pub trait QPainterPath_intersected {
  fn intersected(self, rsthis: &mut QPainterPath) -> QPainterPath;
}

// proto:  QPainterPath QPainterPath::intersected(const QPainterPath & r);
impl<'a> /*trait*/ QPainterPath_intersected for (&'a  QPainterPath) {
  fn intersected(self, rsthis: &mut QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath11intersectedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath11intersectedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn translate<T: QPainterPath_translate>(&mut self, value: T)  {
     value.translate(self);
    // return 1;
  }
}

pub trait QPainterPath_translate {
  fn translate(self, rsthis: &mut QPainterPath) ;
}

// proto:  void QPainterPath::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QPainterPath_translate for (f64, f64) {
  fn translate(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN12QPainterPath9translateEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn addPolygon<T: QPainterPath_addPolygon>(&mut self, value: T)  {
     value.addPolygon(self);
    // return 1;
  }
}

pub trait QPainterPath_addPolygon {
  fn addPolygon(self, rsthis: &mut QPainterPath) ;
}

// proto:  void QPainterPath::addPolygon(const QPolygonF & polygon);
impl<'a> /*trait*/ QPainterPath_addPolygon for (&'a  QPolygonF) {
  fn addPolygon(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath10addPolygonERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath10addPolygonERK9QPolygonF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainterPath::translate(const QPointF & offset);
impl<'a> /*trait*/ QPainterPath_translate for (&'a  QPointF) {
  fn translate(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath9translateERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath9translateERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QPolygonF QPainterPath::toFillPolygon(const QTransform & matrix);
impl<'a> /*trait*/ QPainterPath_toFillPolygon for (&'a  QTransform) {
  fn toFillPolygon(self, rsthis: &mut QPainterPath) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath13toFillPolygonERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath13toFillPolygonERK10QTransform(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn addPath<T: QPainterPath_addPath>(&mut self, value: T)  {
     value.addPath(self);
    // return 1;
  }
}

pub trait QPainterPath_addPath {
  fn addPath(self, rsthis: &mut QPainterPath) ;
}

// proto:  void QPainterPath::addPath(const QPainterPath & path);
impl<'a> /*trait*/ QPainterPath_addPath for (&'a  QPainterPath) {
  fn addPath(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath7addPathERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath7addPathERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainterPath::quadTo(qreal ctrlPtx, qreal ctrlPty, qreal endPtx, qreal endPty);
impl<'a> /*trait*/ QPainterPath_quadTo for (f64, f64, f64, f64) {
  fn quadTo(self, rsthis: &mut QPainterPath)  {
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

impl /*struct*/ QPainterPath {
  pub fn elementCount<T: QPainterPath_elementCount>(&mut self, value: T) -> i32 {
    return value.elementCount(self);
    // return 1;
  }
}

pub trait QPainterPath_elementCount {
  fn elementCount(self, rsthis: &mut QPainterPath) -> i32;
}

// proto:  int QPainterPath::elementCount();
impl<'a> /*trait*/ QPainterPath_elementCount for () {
  fn elementCount(self, rsthis: &mut QPainterPath) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath12elementCountEv()};
    let mut ret = unsafe {_ZNK12QPainterPath12elementCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn simplified<T: QPainterPath_simplified>(&mut self, value: T) -> QPainterPath {
    return value.simplified(self);
    // return 1;
  }
}

pub trait QPainterPath_simplified {
  fn simplified(self, rsthis: &mut QPainterPath) -> QPainterPath;
}

// proto:  QPainterPath QPainterPath::simplified();
impl<'a> /*trait*/ QPainterPath_simplified for () {
  fn simplified(self, rsthis: &mut QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10simplifiedEv()};
    let mut ret = unsafe {_ZNK12QPainterPath10simplifiedEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QPainterPath::contains(const QRectF & rect);
impl<'a> /*trait*/ QPainterPath_contains for (&'a  QRectF) {
  fn contains(self, rsthis: &mut QPainterPath) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath8containsERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath8containsERK6QRectF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn length<T: QPainterPath_length>(&mut self, value: T) -> f64 {
    return value.length(self);
    // return 1;
  }
}

pub trait QPainterPath_length {
  fn length(self, rsthis: &mut QPainterPath) -> f64;
}

// proto:  double QPainterPath::length();
impl<'a> /*trait*/ QPainterPath_length for () {
  fn length(self, rsthis: &mut QPainterPath) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath6lengthEv()};
    let mut ret = unsafe {_ZNK12QPainterPath6lengthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn connectPath<T: QPainterPath_connectPath>(&mut self, value: T)  {
     value.connectPath(self);
    // return 1;
  }
}

pub trait QPainterPath_connectPath {
  fn connectPath(self, rsthis: &mut QPainterPath) ;
}

// proto:  void QPainterPath::connectPath(const QPainterPath & path);
impl<'a> /*trait*/ QPainterPath_connectPath for (&'a  QPainterPath) {
  fn connectPath(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath11connectPathERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath11connectPathERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn addRegion<T: QPainterPath_addRegion>(&mut self, value: T)  {
     value.addRegion(self);
    // return 1;
  }
}

pub trait QPainterPath_addRegion {
  fn addRegion(self, rsthis: &mut QPainterPath) ;
}

// proto:  void QPainterPath::addRegion(const QRegion & region);
impl<'a> /*trait*/ QPainterPath_addRegion for (&'a  QRegion) {
  fn addRegion(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath9addRegionERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath9addRegionERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn currentPosition<T: QPainterPath_currentPosition>(&mut self, value: T) -> QPointF {
    return value.currentPosition(self);
    // return 1;
  }
}

pub trait QPainterPath_currentPosition {
  fn currentPosition(self, rsthis: &mut QPainterPath) -> QPointF;
}

// proto:  QPointF QPainterPath::currentPosition();
impl<'a> /*trait*/ QPainterPath_currentPosition for () {
  fn currentPosition(self, rsthis: &mut QPainterPath) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath15currentPositionEv()};
    let mut ret = unsafe {_ZNK12QPainterPath15currentPositionEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn toReversed<T: QPainterPath_toReversed>(&mut self, value: T) -> QPainterPath {
    return value.toReversed(self);
    // return 1;
  }
}

pub trait QPainterPath_toReversed {
  fn toReversed(self, rsthis: &mut QPainterPath) -> QPainterPath;
}

// proto:  QPainterPath QPainterPath::toReversed();
impl<'a> /*trait*/ QPainterPath_toReversed for () {
  fn toReversed(self, rsthis: &mut QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10toReversedEv()};
    let mut ret = unsafe {_ZNK12QPainterPath10toReversedEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QPainterPath::addRoundRect(qreal x, qreal y, qreal w, qreal h, int xRnd, int yRnd);
impl<'a> /*trait*/ QPainterPath_addRoundRect for (f64, f64, f64, f64, i32, i32) {
  fn addRoundRect(self, rsthis: &mut QPainterPath)  {
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

impl /*struct*/ QPainterPath {
  pub fn swap<T: QPainterPath_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QPainterPath_swap {
  fn swap(self, rsthis: &mut QPainterPath) ;
}

// proto:  void QPainterPath::swap(QPainterPath & other);
impl<'a> /*trait*/ QPainterPath_swap for (&'a mut QPainterPath) {
  fn swap(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QPainterPath::contains(const QPainterPath & p);
impl<'a> /*trait*/ QPainterPath_contains for (&'a  QPainterPath) {
  fn contains(self, rsthis: &mut QPainterPath) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath8containsERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath8containsERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn moveTo<T: QPainterPath_moveTo>(&mut self, value: T)  {
     value.moveTo(self);
    // return 1;
  }
}

pub trait QPainterPath_moveTo {
  fn moveTo(self, rsthis: &mut QPainterPath) ;
}

// proto:  void QPainterPath::moveTo(qreal x, qreal y);
impl<'a> /*trait*/ QPainterPath_moveTo for (f64, f64) {
  fn moveTo(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath6moveToEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN12QPainterPath6moveToEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn subtracted<T: QPainterPath_subtracted>(&mut self, value: T) -> QPainterPath {
    return value.subtracted(self);
    // return 1;
  }
}

pub trait QPainterPath_subtracted {
  fn subtracted(self, rsthis: &mut QPainterPath) -> QPainterPath;
}

// proto:  QPainterPath QPainterPath::subtracted(const QPainterPath & r);
impl<'a> /*trait*/ QPainterPath_subtracted for (&'a  QPainterPath) {
  fn subtracted(self, rsthis: &mut QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10subtractedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath10subtractedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QPainterPath::NewQPainterPath();
impl<'a> /*trait*/ QPainterPath_NewQPainterPath for () {
  fn NewQPainterPath(self) -> QPainterPath {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPathC1Ev()};
    unsafe {_ZN12QPainterPathC1Ev(qthis)};
    let rsthis = QPainterPath{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QPainterPath::addText(const QPointF & point, const QFont & f, const QString & text);
impl<'a> /*trait*/ QPainterPath_addText for (&'a  QPointF, &'a  QFont, &'a  QString) {
  fn addText(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath7addTextERK7QPointFRK5QFontRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath7addTextERK7QPointFRK5QFontRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto: void QPainterPath::NewQPainterPath(const QPainterPath & other);
impl<'a> /*trait*/ QPainterPath_NewQPainterPath for (&'a  QPainterPath) {
  fn NewQPainterPath(self) -> QPainterPath {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPathC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QPainterPathC1ERKS_(qthis, arg0)};
    let rsthis = QPainterPath{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn pointAtPercent<T: QPainterPath_pointAtPercent>(&mut self, value: T) -> QPointF {
    return value.pointAtPercent(self);
    // return 1;
  }
}

pub trait QPainterPath_pointAtPercent {
  fn pointAtPercent(self, rsthis: &mut QPainterPath) -> QPointF;
}

// proto:  QPointF QPainterPath::pointAtPercent(qreal t);
impl<'a> /*trait*/ QPainterPath_pointAtPercent for (f64) {
  fn pointAtPercent(self, rsthis: &mut QPainterPath) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath14pointAtPercentEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK12QPainterPath14pointAtPercentEd(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn percentAtLength<T: QPainterPath_percentAtLength>(&mut self, value: T) -> f64 {
    return value.percentAtLength(self);
    // return 1;
  }
}

pub trait QPainterPath_percentAtLength {
  fn percentAtLength(self, rsthis: &mut QPainterPath) -> f64;
}

// proto:  double QPainterPath::percentAtLength(qreal t);
impl<'a> /*trait*/ QPainterPath_percentAtLength for (f64) {
  fn percentAtLength(self, rsthis: &mut QPainterPath) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath15percentAtLengthEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK12QPainterPath15percentAtLengthEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

// proto:  void QPainterPath::cubicTo(qreal ctrlPt1x, qreal ctrlPt1y, qreal ctrlPt2x, qreal ctrlPt2y, qreal endPtx, qreal endPty);
impl<'a> /*trait*/ QPainterPath_cubicTo for (f64, f64, f64, f64, f64, f64) {
  fn cubicTo(self, rsthis: &mut QPainterPath)  {
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
impl<'a> /*trait*/ QPainterPath_lineTo for (&'a  QPointF) {
  fn lineTo(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath6lineToERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath6lineToERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn subtractedInverted<T: QPainterPath_subtractedInverted>(&mut self, value: T) -> QPainterPath {
    return value.subtractedInverted(self);
    // return 1;
  }
}

pub trait QPainterPath_subtractedInverted {
  fn subtractedInverted(self, rsthis: &mut QPainterPath) -> QPainterPath;
}

// proto:  QPainterPath QPainterPath::subtractedInverted(const QPainterPath & r);
impl<'a> /*trait*/ QPainterPath_subtractedInverted for (&'a  QPainterPath) {
  fn subtractedInverted(self, rsthis: &mut QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath18subtractedInvertedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath18subtractedInvertedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn arcMoveTo<T: QPainterPath_arcMoveTo>(&mut self, value: T)  {
     value.arcMoveTo(self);
    // return 1;
  }
}

pub trait QPainterPath_arcMoveTo {
  fn arcMoveTo(self, rsthis: &mut QPainterPath) ;
}

// proto:  void QPainterPath::arcMoveTo(qreal x, qreal y, qreal w, qreal h, qreal angle);
impl<'a> /*trait*/ QPainterPath_arcMoveTo for (f64, f64, f64, f64, f64) {
  fn arcMoveTo(self, rsthis: &mut QPainterPath)  {
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

impl /*struct*/ QPainterPath {
  pub fn isEmpty<T: QPainterPath_isEmpty>(&mut self, value: T) -> i8 {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QPainterPath_isEmpty {
  fn isEmpty(self, rsthis: &mut QPainterPath) -> i8;
}

// proto:  bool QPainterPath::isEmpty();
impl<'a> /*trait*/ QPainterPath_isEmpty for () {
  fn isEmpty(self, rsthis: &mut QPainterPath) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath7isEmptyEv()};
    let mut ret = unsafe {_ZNK12QPainterPath7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QPainterPath::addRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QPainterPath_addRect for (f64, f64, f64, f64) {
  fn addRect(self, rsthis: &mut QPainterPath)  {
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
impl<'a> /*trait*/ QPainterPath_arcMoveTo for (&'a  QRectF, f64) {
  fn arcMoveTo(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath9arcMoveToERK6QRectFd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
     unsafe {_ZN12QPainterPath9arcMoveToERK6QRectFd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QList<QPolygonF> QPainterPath::toSubpathPolygons(const QMatrix & matrix);
impl<'a> /*trait*/ QPainterPath_toSubpathPolygons for (&'a  QMatrix) {
  fn toSubpathPolygons(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath17toSubpathPolygonsERK7QMatrix()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK12QPainterPath17toSubpathPolygonsERK7QMatrix(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn united<T: QPainterPath_united>(&mut self, value: T) -> QPainterPath {
    return value.united(self);
    // return 1;
  }
}

pub trait QPainterPath_united {
  fn united(self, rsthis: &mut QPainterPath) -> QPainterPath;
}

// proto:  QPainterPath QPainterPath::united(const QPainterPath & r);
impl<'a> /*trait*/ QPainterPath_united for (&'a  QPainterPath) {
  fn united(self, rsthis: &mut QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath6unitedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath6unitedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QPainterPath::addEllipse(const QRectF & rect);
impl<'a> /*trait*/ QPainterPath_addEllipse for (&'a  QRectF) {
  fn addEllipse(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath10addEllipseERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath10addEllipseERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainterPath::moveTo(const QPointF & p);
impl<'a> /*trait*/ QPainterPath_moveTo for (&'a  QPointF) {
  fn moveTo(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath6moveToERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath6moveToERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn angleAtPercent<T: QPainterPath_angleAtPercent>(&mut self, value: T) -> f64 {
    return value.angleAtPercent(self);
    // return 1;
  }
}

pub trait QPainterPath_angleAtPercent {
  fn angleAtPercent(self, rsthis: &mut QPainterPath) -> f64;
}

// proto:  double QPainterPath::angleAtPercent(qreal t);
impl<'a> /*trait*/ QPainterPath_angleAtPercent for (f64) {
  fn angleAtPercent(self, rsthis: &mut QPainterPath) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath14angleAtPercentEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK12QPainterPath14angleAtPercentEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QPainterPath {
  pub fn closeSubpath<T: QPainterPath_closeSubpath>(&mut self, value: T)  {
     value.closeSubpath(self);
    // return 1;
  }
}

pub trait QPainterPath_closeSubpath {
  fn closeSubpath(self, rsthis: &mut QPainterPath) ;
}

// proto:  void QPainterPath::closeSubpath();
impl<'a> /*trait*/ QPainterPath_closeSubpath for () {
  fn closeSubpath(self, rsthis: &mut QPainterPath)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath12closeSubpathEv()};
     unsafe {_ZN12QPainterPath12closeSubpathEv(rsthis.qclsinst)};
    // return 1;
  }
}

