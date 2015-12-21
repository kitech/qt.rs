// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtGui/qpaintengine.h
// dst-file: /src/gui/qpaintengine.rs
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
use super::qfont::QFont; // 773
use super::super::core::qstring::QString; // 771
use super::qmatrix::QMatrix; // 773
use super::qpainter::QPainter; // 773
use super::qtransform::QTransform; // 773
use super::super::core::qpoint::QPointF; // 771
use super::qregion::QRegion; // 773
use super::qpainterpath::QPainterPath; // 773
use super::qbrush::QBrush; // 773
use super::qpen::QPen; // 773
use super::super::core::qrect::QRect; // 771
use super::super::core::qpoint::QPoint; // 771
use super::qpaintdevice::QPaintDevice; // 773
use super::super::core::qrect::QRectF; // 771
use super::qimage::QImage; // 773
use super::qpixmap::QPixmap; // 773
use super::super::core::qline::QLine; // 771
use super::super::core::qline::QLineF; // 771
// use super::qpaintengine::QPaintEngineState; // 773
// use super::qpaintengine::QTextItem; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  qreal QTextItem::descent();
  fn _ZNK9QTextItem7descentEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QTextItem::width();
  fn _ZNK9QTextItem5widthEv(qthis: *mut c_void) -> c_double;
  // proto:  QFont QTextItem::font();
  fn _ZNK9QTextItem4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QTextItem::ascent();
  fn _ZNK9QTextItem6ascentEv(qthis: *mut c_void) -> c_double;
  // proto:  QString QTextItem::text();
  fn _ZNK9QTextItem4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QPaintEngineState::opacity();
  fn _ZNK17QPaintEngineState7opacityEv(qthis: *mut c_void) -> c_double;
  // proto:  QMatrix QPaintEngineState::matrix();
  fn _ZNK17QPaintEngineState6matrixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPainter * QPaintEngineState::painter();
  fn _ZNK17QPaintEngineState7painterEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTransform QPaintEngineState::transform();
  fn _ZNK17QPaintEngineState9transformEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QPaintEngineState::brushOrigin();
  fn _ZNK17QPaintEngineState11brushOriginEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QPaintEngineState::penNeedsResolving();
  fn _ZNK17QPaintEngineState17penNeedsResolvingEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QPaintEngineState::isClipEnabled();
  fn _ZNK17QPaintEngineState13isClipEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  QFont QPaintEngineState::font();
  fn _ZNK17QPaintEngineState4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QPaintEngineState::brushNeedsResolving();
  fn _ZNK17QPaintEngineState19brushNeedsResolvingEv(qthis: *mut c_void) -> c_char;
  // proto:  QRegion QPaintEngineState::clipRegion();
  fn _ZNK17QPaintEngineState10clipRegionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPainterPath QPaintEngineState::clipPath();
  fn _ZNK17QPaintEngineState8clipPathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QBrush QPaintEngineState::brush();
  fn _ZNK17QPaintEngineState5brushEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPen QPaintEngineState::pen();
  fn _ZNK17QPaintEngineState3penEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QBrush QPaintEngineState::backgroundBrush();
  fn _ZNK17QPaintEngineState15backgroundBrushEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPaintEngine::drawEllipse(const QRect & r);
  fn _ZN12QPaintEngine11drawEllipseERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPaintEngine::QPaintEngine(const QPaintEngine & );
  fn _ZN12QPaintEngineC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QPaintEngine::isActive();
  fn _ZNK12QPaintEngine8isActiveEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPaintEngine::drawPoints(const QPointF * points, int pointCount);
  fn _ZN12QPaintEngine10drawPointsEPK7QPointFi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  QPoint QPaintEngine::coordinateOffset();
  fn _ZNK12QPaintEngine16coordinateOffsetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPaintEngine::setPaintDevice(QPaintDevice * device);
  fn _ZN12QPaintEngine14setPaintDeviceEP12QPaintDevice(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPaintEngine::setSystemRect(const QRect & rect);
  fn _ZN12QPaintEngine13setSystemRectERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPaintEngine::~QPaintEngine();
  fn _ZN12QPaintEngineD0Ev(qthis: *mut c_void);
  // proto:  bool QPaintEngine::end();
  fn _ZN12QPaintEngine3endEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPaintEngine::drawTiledPixmap(const QRectF & r, const QPixmap & pixmap, const QPointF & s);
  fn _ZN12QPaintEngine15drawTiledPixmapERK6QRectFRK7QPixmapRK7QPointF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QPaintEngine::setActive(bool newState);
  fn _ZN12QPaintEngine9setActiveEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QPaintEngine::drawPixmap(const QRectF & r, const QPixmap & pm, const QRectF & sr);
  fn _ZN12QPaintEngine10drawPixmapERK6QRectFRK7QPixmapS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QPaintEngine::drawLines(const QLine * lines, int lineCount);
  fn _ZN12QPaintEngine9drawLinesEPK5QLinei(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPaintEngine::drawPath(const QPainterPath & path);
  fn _ZN12QPaintEngine8drawPathERK12QPainterPath(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPaintEngine::drawLines(const QLineF * lines, int lineCount);
  fn _ZN12QPaintEngine9drawLinesEPK6QLineFi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPaintEngine::updateState(const QPaintEngineState & state);
  fn _ZN12QPaintEngine11updateStateERK17QPaintEngineState(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QPaintEngine::begin(QPaintDevice * pdev);
  fn _ZN12QPaintEngine5beginEP12QPaintDevice(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QRect QPaintEngine::systemRect();
  fn _ZNK12QPaintEngine10systemRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPaintEngine::drawRects(const QRectF * rects, int rectCount);
  fn _ZN12QPaintEngine9drawRectsEPK6QRectFi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPaintEngine::setSystemClip(const QRegion & baseClip);
  fn _ZN12QPaintEngine13setSystemClipERK7QRegion(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRegion QPaintEngine::systemClip();
  fn _ZNK12QPaintEngine10systemClipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPaintDevice * QPaintEngine::paintDevice();
  fn _ZNK12QPaintEngine11paintDeviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPaintEngine::syncState();
  fn _ZN12QPaintEngine9syncStateEv(qthis: *mut c_void);
  // proto:  QPainter * QPaintEngine::painter();
  fn _ZNK12QPaintEngine7painterEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPaintEngine::drawEllipse(const QRectF & r);
  fn _ZN12QPaintEngine11drawEllipseERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPaintEngine::drawTextItem(const QPointF & p, const QTextItem & textItem);
  fn _ZN12QPaintEngine12drawTextItemERK7QPointFRK9QTextItem(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPaintEngine::fix_neg_rect(int * x, int * y, int * w, int * h);
  fn _ZN12QPaintEngine12fix_neg_rectEPiS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int);
  // proto:  bool QPaintEngine::isExtended();
  fn _ZNK12QPaintEngine10isExtendedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPaintEngine::drawRects(const QRect * rects, int rectCount);
  fn _ZN12QPaintEngine9drawRectsEPK5QRecti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPaintEngine::drawPoints(const QPoint * points, int pointCount);
  fn _ZN12QPaintEngine10drawPointsEPK6QPointi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QTextItem)=1
pub struct QTextItem {
  pub qclsinst: *mut c_void,
}

// class sizeof(QPaintEngineState)=1
pub struct QPaintEngineState {
  pub qclsinst: *mut c_void,
}

// class sizeof(QPaintEngine)=1
pub struct QPaintEngine {
  pub qclsinst: *mut c_void,
}

  // proto:  qreal QTextItem::descent();
impl /*struct*/ QTextItem {
  pub fn descent<RetType, T: QTextItem_descent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.descent(self);
    // return 1;
  }
}

pub trait QTextItem_descent<RetType> {
  fn descent(self , rsthis: &mut QTextItem) -> RetType;
}

  // proto:  qreal QTextItem::descent();
impl<'a> /*trait*/ QTextItem_descent<f64> for () {
  fn descent(self , rsthis: &mut QTextItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem7descentEv()};
    let mut ret = unsafe {_ZNK9QTextItem7descentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QTextItem::width();
impl /*struct*/ QTextItem {
  pub fn width<RetType, T: QTextItem_width<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QTextItem_width<RetType> {
  fn width(self , rsthis: &mut QTextItem) -> RetType;
}

  // proto:  qreal QTextItem::width();
impl<'a> /*trait*/ QTextItem_width<f64> for () {
  fn width(self , rsthis: &mut QTextItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem5widthEv()};
    let mut ret = unsafe {_ZNK9QTextItem5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QFont QTextItem::font();
impl /*struct*/ QTextItem {
  pub fn font<RetType, T: QTextItem_font<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QTextItem_font<RetType> {
  fn font(self , rsthis: &mut QTextItem) -> RetType;
}

  // proto:  QFont QTextItem::font();
impl<'a> /*trait*/ QTextItem_font<QFont> for () {
  fn font(self , rsthis: &mut QTextItem) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem4fontEv()};
    let mut ret = unsafe {_ZNK9QTextItem4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QTextItem::ascent();
impl /*struct*/ QTextItem {
  pub fn ascent<RetType, T: QTextItem_ascent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.ascent(self);
    // return 1;
  }
}

pub trait QTextItem_ascent<RetType> {
  fn ascent(self , rsthis: &mut QTextItem) -> RetType;
}

  // proto:  qreal QTextItem::ascent();
impl<'a> /*trait*/ QTextItem_ascent<f64> for () {
  fn ascent(self , rsthis: &mut QTextItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem6ascentEv()};
    let mut ret = unsafe {_ZNK9QTextItem6ascentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QString QTextItem::text();
impl /*struct*/ QTextItem {
  pub fn text<RetType, T: QTextItem_text<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QTextItem_text<RetType> {
  fn text(self , rsthis: &mut QTextItem) -> RetType;
}

  // proto:  QString QTextItem::text();
impl<'a> /*trait*/ QTextItem_text<QString> for () {
  fn text(self , rsthis: &mut QTextItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem4textEv()};
    let mut ret = unsafe {_ZNK9QTextItem4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QPaintEngineState::opacity();
impl /*struct*/ QPaintEngineState {
  pub fn opacity<RetType, T: QPaintEngineState_opacity<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.opacity(self);
    // return 1;
  }
}

pub trait QPaintEngineState_opacity<RetType> {
  fn opacity(self , rsthis: &mut QPaintEngineState) -> RetType;
}

  // proto:  qreal QPaintEngineState::opacity();
impl<'a> /*trait*/ QPaintEngineState_opacity<f64> for () {
  fn opacity(self , rsthis: &mut QPaintEngineState) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState7opacityEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState7opacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QMatrix QPaintEngineState::matrix();
impl /*struct*/ QPaintEngineState {
  pub fn matrix<RetType, T: QPaintEngineState_matrix<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.matrix(self);
    // return 1;
  }
}

pub trait QPaintEngineState_matrix<RetType> {
  fn matrix(self , rsthis: &mut QPaintEngineState) -> RetType;
}

  // proto:  QMatrix QPaintEngineState::matrix();
impl<'a> /*trait*/ QPaintEngineState_matrix<QMatrix> for () {
  fn matrix(self , rsthis: &mut QPaintEngineState) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState6matrixEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState6matrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPainter * QPaintEngineState::painter();
impl /*struct*/ QPaintEngineState {
  pub fn painter<RetType, T: QPaintEngineState_painter<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.painter(self);
    // return 1;
  }
}

pub trait QPaintEngineState_painter<RetType> {
  fn painter(self , rsthis: &mut QPaintEngineState) -> RetType;
}

  // proto:  QPainter * QPaintEngineState::painter();
impl<'a> /*trait*/ QPaintEngineState_painter<QPainter> for () {
  fn painter(self , rsthis: &mut QPaintEngineState) -> QPainter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState7painterEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState7painterEv(rsthis.qclsinst)};
    let mut ret1 = QPainter{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QTransform QPaintEngineState::transform();
impl /*struct*/ QPaintEngineState {
  pub fn transform<RetType, T: QPaintEngineState_transform<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.transform(self);
    // return 1;
  }
}

pub trait QPaintEngineState_transform<RetType> {
  fn transform(self , rsthis: &mut QPaintEngineState) -> RetType;
}

  // proto:  QTransform QPaintEngineState::transform();
impl<'a> /*trait*/ QPaintEngineState_transform<QTransform> for () {
  fn transform(self , rsthis: &mut QPaintEngineState) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState9transformEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState9transformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QPaintEngineState::brushOrigin();
impl /*struct*/ QPaintEngineState {
  pub fn brushOrigin<RetType, T: QPaintEngineState_brushOrigin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.brushOrigin(self);
    // return 1;
  }
}

pub trait QPaintEngineState_brushOrigin<RetType> {
  fn brushOrigin(self , rsthis: &mut QPaintEngineState) -> RetType;
}

  // proto:  QPointF QPaintEngineState::brushOrigin();
impl<'a> /*trait*/ QPaintEngineState_brushOrigin<QPointF> for () {
  fn brushOrigin(self , rsthis: &mut QPaintEngineState) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState11brushOriginEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState11brushOriginEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPaintEngineState::penNeedsResolving();
impl /*struct*/ QPaintEngineState {
  pub fn penNeedsResolving<RetType, T: QPaintEngineState_penNeedsResolving<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.penNeedsResolving(self);
    // return 1;
  }
}

pub trait QPaintEngineState_penNeedsResolving<RetType> {
  fn penNeedsResolving(self , rsthis: &mut QPaintEngineState) -> RetType;
}

  // proto:  bool QPaintEngineState::penNeedsResolving();
impl<'a> /*trait*/ QPaintEngineState_penNeedsResolving<i8> for () {
  fn penNeedsResolving(self , rsthis: &mut QPaintEngineState) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState17penNeedsResolvingEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState17penNeedsResolvingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QPaintEngineState::isClipEnabled();
impl /*struct*/ QPaintEngineState {
  pub fn isClipEnabled<RetType, T: QPaintEngineState_isClipEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isClipEnabled(self);
    // return 1;
  }
}

pub trait QPaintEngineState_isClipEnabled<RetType> {
  fn isClipEnabled(self , rsthis: &mut QPaintEngineState) -> RetType;
}

  // proto:  bool QPaintEngineState::isClipEnabled();
impl<'a> /*trait*/ QPaintEngineState_isClipEnabled<i8> for () {
  fn isClipEnabled(self , rsthis: &mut QPaintEngineState) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState13isClipEnabledEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState13isClipEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QFont QPaintEngineState::font();
impl /*struct*/ QPaintEngineState {
  pub fn font<RetType, T: QPaintEngineState_font<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QPaintEngineState_font<RetType> {
  fn font(self , rsthis: &mut QPaintEngineState) -> RetType;
}

  // proto:  QFont QPaintEngineState::font();
impl<'a> /*trait*/ QPaintEngineState_font<QFont> for () {
  fn font(self , rsthis: &mut QPaintEngineState) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState4fontEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPaintEngineState::brushNeedsResolving();
impl /*struct*/ QPaintEngineState {
  pub fn brushNeedsResolving<RetType, T: QPaintEngineState_brushNeedsResolving<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.brushNeedsResolving(self);
    // return 1;
  }
}

pub trait QPaintEngineState_brushNeedsResolving<RetType> {
  fn brushNeedsResolving(self , rsthis: &mut QPaintEngineState) -> RetType;
}

  // proto:  bool QPaintEngineState::brushNeedsResolving();
impl<'a> /*trait*/ QPaintEngineState_brushNeedsResolving<i8> for () {
  fn brushNeedsResolving(self , rsthis: &mut QPaintEngineState) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState19brushNeedsResolvingEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState19brushNeedsResolvingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRegion QPaintEngineState::clipRegion();
impl /*struct*/ QPaintEngineState {
  pub fn clipRegion<RetType, T: QPaintEngineState_clipRegion<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clipRegion(self);
    // return 1;
  }
}

pub trait QPaintEngineState_clipRegion<RetType> {
  fn clipRegion(self , rsthis: &mut QPaintEngineState) -> RetType;
}

  // proto:  QRegion QPaintEngineState::clipRegion();
impl<'a> /*trait*/ QPaintEngineState_clipRegion<QRegion> for () {
  fn clipRegion(self , rsthis: &mut QPaintEngineState) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState10clipRegionEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState10clipRegionEv(rsthis.qclsinst)};
    let mut ret1 = QRegion{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPainterPath QPaintEngineState::clipPath();
impl /*struct*/ QPaintEngineState {
  pub fn clipPath<RetType, T: QPaintEngineState_clipPath<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clipPath(self);
    // return 1;
  }
}

pub trait QPaintEngineState_clipPath<RetType> {
  fn clipPath(self , rsthis: &mut QPaintEngineState) -> RetType;
}

  // proto:  QPainterPath QPaintEngineState::clipPath();
impl<'a> /*trait*/ QPaintEngineState_clipPath<QPainterPath> for () {
  fn clipPath(self , rsthis: &mut QPaintEngineState) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState8clipPathEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState8clipPathEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QBrush QPaintEngineState::brush();
impl /*struct*/ QPaintEngineState {
  pub fn brush<RetType, T: QPaintEngineState_brush<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.brush(self);
    // return 1;
  }
}

pub trait QPaintEngineState_brush<RetType> {
  fn brush(self , rsthis: &mut QPaintEngineState) -> RetType;
}

  // proto:  QBrush QPaintEngineState::brush();
impl<'a> /*trait*/ QPaintEngineState_brush<QBrush> for () {
  fn brush(self , rsthis: &mut QPaintEngineState) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState5brushEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState5brushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPen QPaintEngineState::pen();
impl /*struct*/ QPaintEngineState {
  pub fn pen<RetType, T: QPaintEngineState_pen<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pen(self);
    // return 1;
  }
}

pub trait QPaintEngineState_pen<RetType> {
  fn pen(self , rsthis: &mut QPaintEngineState) -> RetType;
}

  // proto:  QPen QPaintEngineState::pen();
impl<'a> /*trait*/ QPaintEngineState_pen<QPen> for () {
  fn pen(self , rsthis: &mut QPaintEngineState) -> QPen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState3penEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState3penEv(rsthis.qclsinst)};
    let mut ret1 = QPen{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QBrush QPaintEngineState::backgroundBrush();
impl /*struct*/ QPaintEngineState {
  pub fn backgroundBrush<RetType, T: QPaintEngineState_backgroundBrush<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.backgroundBrush(self);
    // return 1;
  }
}

pub trait QPaintEngineState_backgroundBrush<RetType> {
  fn backgroundBrush(self , rsthis: &mut QPaintEngineState) -> RetType;
}

  // proto:  QBrush QPaintEngineState::backgroundBrush();
impl<'a> /*trait*/ QPaintEngineState_backgroundBrush<QBrush> for () {
  fn backgroundBrush(self , rsthis: &mut QPaintEngineState) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState15backgroundBrushEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState15backgroundBrushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawEllipse(const QRect & r);
impl /*struct*/ QPaintEngine {
  pub fn drawEllipse<RetType, T: QPaintEngine_drawEllipse<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.drawEllipse(self);
    // return 1;
  }
}

pub trait QPaintEngine_drawEllipse<RetType> {
  fn drawEllipse(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::drawEllipse(const QRect & r);
impl<'a> /*trait*/ QPaintEngine_drawEllipse<()> for (QRect) {
  fn drawEllipse(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine11drawEllipseERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPaintEngine11drawEllipseERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::QPaintEngine(const QPaintEngine & );
impl /*struct*/ QPaintEngine {
  pub fn NewQPaintEngine<T: QPaintEngine_NewQPaintEngine>(value: T) -> QPaintEngine {
    let rsthis = value.NewQPaintEngine();
    return rsthis;
    // return 1;
  }
}

pub trait QPaintEngine_NewQPaintEngine {
  fn NewQPaintEngine(self) -> QPaintEngine;
}

  // proto:  void QPaintEngine::QPaintEngine(const QPaintEngine & );
impl<'a> /*trait*/ QPaintEngine_NewQPaintEngine for (QPaintEngine) {
  fn NewQPaintEngine(self) -> QPaintEngine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngineC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QPaintEngineC1ERKS_(qthis, arg0)};
    let rsthis = QPaintEngine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QPaintEngine::isActive();
impl /*struct*/ QPaintEngine {
  pub fn isActive<RetType, T: QPaintEngine_isActive<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isActive(self);
    // return 1;
  }
}

pub trait QPaintEngine_isActive<RetType> {
  fn isActive(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  bool QPaintEngine::isActive();
impl<'a> /*trait*/ QPaintEngine_isActive<i8> for () {
  fn isActive(self , rsthis: &mut QPaintEngine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine8isActiveEv()};
    let mut ret = unsafe {_ZNK12QPaintEngine8isActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawPoints(const QPointF * points, int pointCount);
impl /*struct*/ QPaintEngine {
  pub fn drawPoints<RetType, T: QPaintEngine_drawPoints<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.drawPoints(self);
    // return 1;
  }
}

pub trait QPaintEngine_drawPoints<RetType> {
  fn drawPoints(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::drawPoints(const QPointF * points, int pointCount);
impl<'a> /*trait*/ QPaintEngine_drawPoints<()> for (QPointF, i32) {
  fn drawPoints(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine10drawPointsEPK7QPointFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QPaintEngine10drawPointsEPK7QPointFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QPoint QPaintEngine::coordinateOffset();
impl /*struct*/ QPaintEngine {
  pub fn coordinateOffset<RetType, T: QPaintEngine_coordinateOffset<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.coordinateOffset(self);
    // return 1;
  }
}

pub trait QPaintEngine_coordinateOffset<RetType> {
  fn coordinateOffset(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  QPoint QPaintEngine::coordinateOffset();
impl<'a> /*trait*/ QPaintEngine_coordinateOffset<QPoint> for () {
  fn coordinateOffset(self , rsthis: &mut QPaintEngine) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine16coordinateOffsetEv()};
    let mut ret = unsafe {_ZNK12QPaintEngine16coordinateOffsetEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPaintEngine::setPaintDevice(QPaintDevice * device);
impl /*struct*/ QPaintEngine {
  pub fn setPaintDevice<RetType, T: QPaintEngine_setPaintDevice<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPaintDevice(self);
    // return 1;
  }
}

pub trait QPaintEngine_setPaintDevice<RetType> {
  fn setPaintDevice(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::setPaintDevice(QPaintDevice * device);
impl<'a> /*trait*/ QPaintEngine_setPaintDevice<()> for (QPaintDevice) {
  fn setPaintDevice(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine14setPaintDeviceEP12QPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPaintEngine14setPaintDeviceEP12QPaintDevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::setSystemRect(const QRect & rect);
impl /*struct*/ QPaintEngine {
  pub fn setSystemRect<RetType, T: QPaintEngine_setSystemRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSystemRect(self);
    // return 1;
  }
}

pub trait QPaintEngine_setSystemRect<RetType> {
  fn setSystemRect(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::setSystemRect(const QRect & rect);
impl<'a> /*trait*/ QPaintEngine_setSystemRect<()> for (QRect) {
  fn setSystemRect(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine13setSystemRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPaintEngine13setSystemRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::~QPaintEngine();
impl /*struct*/ QPaintEngine {
  pub fn FreeQPaintEngine<RetType, T: QPaintEngine_FreeQPaintEngine<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQPaintEngine(self);
    // return 1;
  }
}

pub trait QPaintEngine_FreeQPaintEngine<RetType> {
  fn FreeQPaintEngine(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::~QPaintEngine();
impl<'a> /*trait*/ QPaintEngine_FreeQPaintEngine<()> for () {
  fn FreeQPaintEngine(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngineD0Ev()};
     unsafe {_ZN12QPaintEngineD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QPaintEngine::end();
impl /*struct*/ QPaintEngine {
  pub fn end<RetType, T: QPaintEngine_end<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.end(self);
    // return 1;
  }
}

pub trait QPaintEngine_end<RetType> {
  fn end(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  bool QPaintEngine::end();
impl<'a> /*trait*/ QPaintEngine_end<i8> for () {
  fn end(self , rsthis: &mut QPaintEngine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine3endEv()};
    let mut ret = unsafe {_ZN12QPaintEngine3endEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawTiledPixmap(const QRectF & r, const QPixmap & pixmap, const QPointF & s);
impl /*struct*/ QPaintEngine {
  pub fn drawTiledPixmap<RetType, T: QPaintEngine_drawTiledPixmap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.drawTiledPixmap(self);
    // return 1;
  }
}

pub trait QPaintEngine_drawTiledPixmap<RetType> {
  fn drawTiledPixmap(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::drawTiledPixmap(const QRectF & r, const QPixmap & pixmap, const QPointF & s);
impl<'a> /*trait*/ QPaintEngine_drawTiledPixmap<()> for (QRectF, QPixmap, QPointF) {
  fn drawTiledPixmap(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine15drawTiledPixmapERK6QRectFRK7QPixmapRK7QPointF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN12QPaintEngine15drawTiledPixmapERK6QRectFRK7QPixmapRK7QPointF(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::setActive(bool newState);
impl /*struct*/ QPaintEngine {
  pub fn setActive<RetType, T: QPaintEngine_setActive<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setActive(self);
    // return 1;
  }
}

pub trait QPaintEngine_setActive<RetType> {
  fn setActive(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::setActive(bool newState);
impl<'a> /*trait*/ QPaintEngine_setActive<()> for (i8) {
  fn setActive(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine9setActiveEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QPaintEngine9setActiveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawPixmap(const QRectF & r, const QPixmap & pm, const QRectF & sr);
impl /*struct*/ QPaintEngine {
  pub fn drawPixmap<RetType, T: QPaintEngine_drawPixmap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.drawPixmap(self);
    // return 1;
  }
}

pub trait QPaintEngine_drawPixmap<RetType> {
  fn drawPixmap(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::drawPixmap(const QRectF & r, const QPixmap & pm, const QRectF & sr);
impl<'a> /*trait*/ QPaintEngine_drawPixmap<()> for (QRectF, QPixmap, QRectF) {
  fn drawPixmap(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine10drawPixmapERK6QRectFRK7QPixmapS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN12QPaintEngine10drawPixmapERK6QRectFRK7QPixmapS2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawLines(const QLine * lines, int lineCount);
impl /*struct*/ QPaintEngine {
  pub fn drawLines<RetType, T: QPaintEngine_drawLines<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.drawLines(self);
    // return 1;
  }
}

pub trait QPaintEngine_drawLines<RetType> {
  fn drawLines(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::drawLines(const QLine * lines, int lineCount);
impl<'a> /*trait*/ QPaintEngine_drawLines<()> for (QLine, i32) {
  fn drawLines(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine9drawLinesEPK5QLinei()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QPaintEngine9drawLinesEPK5QLinei(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawPath(const QPainterPath & path);
impl /*struct*/ QPaintEngine {
  pub fn drawPath<RetType, T: QPaintEngine_drawPath<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.drawPath(self);
    // return 1;
  }
}

pub trait QPaintEngine_drawPath<RetType> {
  fn drawPath(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::drawPath(const QPainterPath & path);
impl<'a> /*trait*/ QPaintEngine_drawPath<()> for (QPainterPath) {
  fn drawPath(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine8drawPathERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPaintEngine8drawPathERK12QPainterPath(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawLines(const QLineF * lines, int lineCount);
impl<'a> /*trait*/ QPaintEngine_drawLines<()> for (QLineF, i32) {
  fn drawLines(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine9drawLinesEPK6QLineFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QPaintEngine9drawLinesEPK6QLineFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::updateState(const QPaintEngineState & state);
impl /*struct*/ QPaintEngine {
  pub fn updateState<RetType, T: QPaintEngine_updateState<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.updateState(self);
    // return 1;
  }
}

pub trait QPaintEngine_updateState<RetType> {
  fn updateState(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::updateState(const QPaintEngineState & state);
impl<'a> /*trait*/ QPaintEngine_updateState<()> for (QPaintEngineState) {
  fn updateState(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine11updateStateERK17QPaintEngineState()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPaintEngine11updateStateERK17QPaintEngineState(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPaintEngine::begin(QPaintDevice * pdev);
impl /*struct*/ QPaintEngine {
  pub fn begin<RetType, T: QPaintEngine_begin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.begin(self);
    // return 1;
  }
}

pub trait QPaintEngine_begin<RetType> {
  fn begin(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  bool QPaintEngine::begin(QPaintDevice * pdev);
impl<'a> /*trait*/ QPaintEngine_begin<i8> for (QPaintDevice) {
  fn begin(self , rsthis: &mut QPaintEngine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine5beginEP12QPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QPaintEngine5beginEP12QPaintDevice(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRect QPaintEngine::systemRect();
impl /*struct*/ QPaintEngine {
  pub fn systemRect<RetType, T: QPaintEngine_systemRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.systemRect(self);
    // return 1;
  }
}

pub trait QPaintEngine_systemRect<RetType> {
  fn systemRect(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  QRect QPaintEngine::systemRect();
impl<'a> /*trait*/ QPaintEngine_systemRect<QRect> for () {
  fn systemRect(self , rsthis: &mut QPaintEngine) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine10systemRectEv()};
    let mut ret = unsafe {_ZNK12QPaintEngine10systemRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawRects(const QRectF * rects, int rectCount);
impl /*struct*/ QPaintEngine {
  pub fn drawRects<RetType, T: QPaintEngine_drawRects<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.drawRects(self);
    // return 1;
  }
}

pub trait QPaintEngine_drawRects<RetType> {
  fn drawRects(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::drawRects(const QRectF * rects, int rectCount);
impl<'a> /*trait*/ QPaintEngine_drawRects<()> for (QRectF, i32) {
  fn drawRects(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine9drawRectsEPK6QRectFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QPaintEngine9drawRectsEPK6QRectFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::setSystemClip(const QRegion & baseClip);
impl /*struct*/ QPaintEngine {
  pub fn setSystemClip<RetType, T: QPaintEngine_setSystemClip<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSystemClip(self);
    // return 1;
  }
}

pub trait QPaintEngine_setSystemClip<RetType> {
  fn setSystemClip(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::setSystemClip(const QRegion & baseClip);
impl<'a> /*trait*/ QPaintEngine_setSystemClip<()> for (QRegion) {
  fn setSystemClip(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine13setSystemClipERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPaintEngine13setSystemClipERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRegion QPaintEngine::systemClip();
impl /*struct*/ QPaintEngine {
  pub fn systemClip<RetType, T: QPaintEngine_systemClip<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.systemClip(self);
    // return 1;
  }
}

pub trait QPaintEngine_systemClip<RetType> {
  fn systemClip(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  QRegion QPaintEngine::systemClip();
impl<'a> /*trait*/ QPaintEngine_systemClip<QRegion> for () {
  fn systemClip(self , rsthis: &mut QPaintEngine) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine10systemClipEv()};
    let mut ret = unsafe {_ZNK12QPaintEngine10systemClipEv(rsthis.qclsinst)};
    let mut ret1 = QRegion{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPaintDevice * QPaintEngine::paintDevice();
impl /*struct*/ QPaintEngine {
  pub fn paintDevice<RetType, T: QPaintEngine_paintDevice<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.paintDevice(self);
    // return 1;
  }
}

pub trait QPaintEngine_paintDevice<RetType> {
  fn paintDevice(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  QPaintDevice * QPaintEngine::paintDevice();
impl<'a> /*trait*/ QPaintEngine_paintDevice<QPaintDevice> for () {
  fn paintDevice(self , rsthis: &mut QPaintEngine) -> QPaintDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine11paintDeviceEv()};
    let mut ret = unsafe {_ZNK12QPaintEngine11paintDeviceEv(rsthis.qclsinst)};
    let mut ret1 = QPaintDevice{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPaintEngine::syncState();
impl /*struct*/ QPaintEngine {
  pub fn syncState<RetType, T: QPaintEngine_syncState<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.syncState(self);
    // return 1;
  }
}

pub trait QPaintEngine_syncState<RetType> {
  fn syncState(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::syncState();
impl<'a> /*trait*/ QPaintEngine_syncState<()> for () {
  fn syncState(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine9syncStateEv()};
     unsafe {_ZN12QPaintEngine9syncStateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPainter * QPaintEngine::painter();
impl /*struct*/ QPaintEngine {
  pub fn painter<RetType, T: QPaintEngine_painter<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.painter(self);
    // return 1;
  }
}

pub trait QPaintEngine_painter<RetType> {
  fn painter(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  QPainter * QPaintEngine::painter();
impl<'a> /*trait*/ QPaintEngine_painter<QPainter> for () {
  fn painter(self , rsthis: &mut QPaintEngine) -> QPainter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine7painterEv()};
    let mut ret = unsafe {_ZNK12QPaintEngine7painterEv(rsthis.qclsinst)};
    let mut ret1 = QPainter{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawEllipse(const QRectF & r);
impl<'a> /*trait*/ QPaintEngine_drawEllipse<()> for (QRectF) {
  fn drawEllipse(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine11drawEllipseERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPaintEngine11drawEllipseERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawTextItem(const QPointF & p, const QTextItem & textItem);
impl /*struct*/ QPaintEngine {
  pub fn drawTextItem<RetType, T: QPaintEngine_drawTextItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.drawTextItem(self);
    // return 1;
  }
}

pub trait QPaintEngine_drawTextItem<RetType> {
  fn drawTextItem(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::drawTextItem(const QPointF & p, const QTextItem & textItem);
impl<'a> /*trait*/ QPaintEngine_drawTextItem<()> for (QPointF, QTextItem) {
  fn drawTextItem(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine12drawTextItemERK7QPointFRK9QTextItem()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN12QPaintEngine12drawTextItemERK7QPointFRK9QTextItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::fix_neg_rect(int * x, int * y, int * w, int * h);
impl /*struct*/ QPaintEngine {
  pub fn fix_neg_rect<RetType, T: QPaintEngine_fix_neg_rect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fix_neg_rect(self);
    // return 1;
  }
}

pub trait QPaintEngine_fix_neg_rect<RetType> {
  fn fix_neg_rect(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::fix_neg_rect(int * x, int * y, int * w, int * h);
impl<'a> /*trait*/ QPaintEngine_fix_neg_rect<()> for (&'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<i32>) {
  fn fix_neg_rect(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine12fix_neg_rectEPiS0_S0_S0_()};
    let arg0 = self.0.as_ptr()  as *mut c_int;
    let arg1 = self.1.as_ptr()  as *mut c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_int;
     unsafe {_ZN12QPaintEngine12fix_neg_rectEPiS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  bool QPaintEngine::isExtended();
impl /*struct*/ QPaintEngine {
  pub fn isExtended<RetType, T: QPaintEngine_isExtended<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isExtended(self);
    // return 1;
  }
}

pub trait QPaintEngine_isExtended<RetType> {
  fn isExtended(self , rsthis: &mut QPaintEngine) -> RetType;
}

  // proto:  bool QPaintEngine::isExtended();
impl<'a> /*trait*/ QPaintEngine_isExtended<i8> for () {
  fn isExtended(self , rsthis: &mut QPaintEngine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine10isExtendedEv()};
    let mut ret = unsafe {_ZNK12QPaintEngine10isExtendedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawRects(const QRect * rects, int rectCount);
impl<'a> /*trait*/ QPaintEngine_drawRects<()> for (QRect, i32) {
  fn drawRects(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine9drawRectsEPK5QRecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QPaintEngine9drawRectsEPK5QRecti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawPoints(const QPoint * points, int pointCount);
impl<'a> /*trait*/ QPaintEngine_drawPoints<()> for (QPoint, i32) {
  fn drawPoints(self , rsthis: &mut QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine10drawPointsEPK6QPointi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QPaintEngine10drawPointsEPK6QPointi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// <= body block end

