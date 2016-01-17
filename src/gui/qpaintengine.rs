// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
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
use std::ops::Deref;
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
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTextItem_Class_Size() -> c_int;
  // proto:  qreal QTextItem::descent();
  fn _ZNK9QTextItem7descentEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QTextItem::width();
  fn _ZNK9QTextItem5widthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QFont QTextItem::font();
  fn _ZNK9QTextItem4fontEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QTextItem::ascent();
  fn _ZNK9QTextItem6ascentEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QString QTextItem::text();
  fn _ZNK9QTextItem4textEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QPaintEngineState_Class_Size() -> c_int;
  // proto:  qreal QPaintEngineState::opacity();
  fn _ZNK17QPaintEngineState7opacityEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QMatrix QPaintEngineState::matrix();
  fn _ZNK17QPaintEngineState6matrixEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPainter * QPaintEngineState::painter();
  fn _ZNK17QPaintEngineState7painterEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTransform QPaintEngineState::transform();
  fn _ZNK17QPaintEngineState9transformEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPointF QPaintEngineState::brushOrigin();
  fn _ZNK17QPaintEngineState11brushOriginEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QPaintEngineState::penNeedsResolving();
  fn _ZNK17QPaintEngineState17penNeedsResolvingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QPaintEngineState::isClipEnabled();
  fn _ZNK17QPaintEngineState13isClipEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QFont QPaintEngineState::font();
  fn _ZNK17QPaintEngineState4fontEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QPaintEngineState::brushNeedsResolving();
  fn _ZNK17QPaintEngineState19brushNeedsResolvingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QRegion QPaintEngineState::clipRegion();
  fn _ZNK17QPaintEngineState10clipRegionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPainterPath QPaintEngineState::clipPath();
  fn _ZNK17QPaintEngineState8clipPathEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QBrush QPaintEngineState::brush();
  fn _ZNK17QPaintEngineState5brushEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPen QPaintEngineState::pen();
  fn _ZNK17QPaintEngineState3penEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QBrush QPaintEngineState::backgroundBrush();
  fn _ZNK17QPaintEngineState15backgroundBrushEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QPaintEngine_Class_Size() -> c_int;
  // proto:  void QPaintEngine::drawEllipse(const QRect & r);
  fn _ZN12QPaintEngine11drawEllipseERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPaintEngine::QPaintEngine(const QPaintEngine & );
  fn _ZN12QPaintEngineC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QPaintEngine::isActive();
  fn _ZNK12QPaintEngine8isActiveEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QPaintEngine::drawPoints(const QPointF * points, int pointCount);
  fn _ZN12QPaintEngine10drawPointsEPK7QPointFi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  QPoint QPaintEngine::coordinateOffset();
  fn _ZNK12QPaintEngine16coordinateOffsetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPaintEngine::setPaintDevice(QPaintDevice * device);
  fn _ZN12QPaintEngine14setPaintDeviceEP12QPaintDevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPaintEngine::setSystemRect(const QRect & rect);
  fn _ZN12QPaintEngine13setSystemRectERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPaintEngine::~QPaintEngine();
  fn _ZN12QPaintEngineD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QPaintEngine::end();
  fn _ZN12QPaintEngine3endEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QPaintEngine::drawTiledPixmap(const QRectF & r, const QPixmap & pixmap, const QPointF & s);
  fn _ZN12QPaintEngine15drawTiledPixmapERK6QRectFRK7QPixmapRK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QPaintEngine::setActive(bool newState);
  fn _ZN12QPaintEngine9setActiveEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QPaintEngine::drawPixmap(const QRectF & r, const QPixmap & pm, const QRectF & sr);
  fn _ZN12QPaintEngine10drawPixmapERK6QRectFRK7QPixmapS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QPaintEngine::drawLines(const QLine * lines, int lineCount);
  fn _ZN12QPaintEngine9drawLinesEPK5QLinei(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPaintEngine::drawPath(const QPainterPath & path);
  fn _ZN12QPaintEngine8drawPathERK12QPainterPath(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPaintEngine::drawLines(const QLineF * lines, int lineCount);
  fn _ZN12QPaintEngine9drawLinesEPK6QLineFi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPaintEngine::updateState(const QPaintEngineState & state);
  fn _ZN12QPaintEngine11updateStateERK17QPaintEngineState(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QPaintEngine::begin(QPaintDevice * pdev);
  fn _ZN12QPaintEngine5beginEP12QPaintDevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QRect QPaintEngine::systemRect();
  fn _ZNK12QPaintEngine10systemRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPaintEngine::drawRects(const QRectF * rects, int rectCount);
  fn _ZN12QPaintEngine9drawRectsEPK6QRectFi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPaintEngine::setSystemClip(const QRegion & baseClip);
  fn _ZN12QPaintEngine13setSystemClipERK7QRegion(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QRegion QPaintEngine::systemClip();
  fn _ZNK12QPaintEngine10systemClipEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPaintDevice * QPaintEngine::paintDevice();
  fn _ZNK12QPaintEngine11paintDeviceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPaintEngine::syncState();
  fn _ZN12QPaintEngine9syncStateEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPainter * QPaintEngine::painter();
  fn _ZNK12QPaintEngine7painterEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPaintEngine::drawEllipse(const QRectF & r);
  fn _ZN12QPaintEngine11drawEllipseERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPaintEngine::drawTextItem(const QPointF & p, const QTextItem & textItem);
  fn _ZN12QPaintEngine12drawTextItemERK7QPointFRK9QTextItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPaintEngine::fix_neg_rect(int * x, int * y, int * w, int * h);
  fn _ZN12QPaintEngine12fix_neg_rectEPiS0_S0_S0_(qthis: u64 /* *mut c_void*/, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int);
  // proto:  bool QPaintEngine::isExtended();
  fn _ZNK12QPaintEngine10isExtendedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QPaintEngine::drawRects(const QRect * rects, int rectCount);
  fn _ZN12QPaintEngine9drawRectsEPK5QRecti(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPaintEngine::drawPoints(const QPoint * points, int pointCount);
  fn _ZN12QPaintEngine10drawPointsEPK6QPointi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QTextItem)=1
#[derive(Default)]
pub struct QTextItem {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QPaintEngineState)=1
#[derive(Default)]
pub struct QPaintEngineState {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QPaintEngine)=1
#[derive(Default)]
pub struct QPaintEngine {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTextItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextItem {
    return QTextItem{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  qreal QTextItem::descent();
impl /*struct*/ QTextItem {
  pub fn descent<RetType, T: QTextItem_descent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.descent(self);
    // return 1;
  }
}

pub trait QTextItem_descent<RetType> {
  fn descent(self , rsthis: & QTextItem) -> RetType;
}

  // proto:  qreal QTextItem::descent();
impl<'a> /*trait*/ QTextItem_descent<f64> for () {
  fn descent(self , rsthis: & QTextItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem7descentEv()};
    let mut ret = unsafe {_ZNK9QTextItem7descentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QTextItem::width();
impl /*struct*/ QTextItem {
  pub fn width<RetType, T: QTextItem_width<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QTextItem_width<RetType> {
  fn width(self , rsthis: & QTextItem) -> RetType;
}

  // proto:  qreal QTextItem::width();
impl<'a> /*trait*/ QTextItem_width<f64> for () {
  fn width(self , rsthis: & QTextItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem5widthEv()};
    let mut ret = unsafe {_ZNK9QTextItem5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QFont QTextItem::font();
impl /*struct*/ QTextItem {
  pub fn font<RetType, T: QTextItem_font<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QTextItem_font<RetType> {
  fn font(self , rsthis: & QTextItem) -> RetType;
}

  // proto:  QFont QTextItem::font();
impl<'a> /*trait*/ QTextItem_font<QFont> for () {
  fn font(self , rsthis: & QTextItem) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem4fontEv()};
    let mut ret = unsafe {_ZNK9QTextItem4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QTextItem::ascent();
impl /*struct*/ QTextItem {
  pub fn ascent<RetType, T: QTextItem_ascent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ascent(self);
    // return 1;
  }
}

pub trait QTextItem_ascent<RetType> {
  fn ascent(self , rsthis: & QTextItem) -> RetType;
}

  // proto:  qreal QTextItem::ascent();
impl<'a> /*trait*/ QTextItem_ascent<f64> for () {
  fn ascent(self , rsthis: & QTextItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem6ascentEv()};
    let mut ret = unsafe {_ZNK9QTextItem6ascentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QString QTextItem::text();
impl /*struct*/ QTextItem {
  pub fn text<RetType, T: QTextItem_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QTextItem_text<RetType> {
  fn text(self , rsthis: & QTextItem) -> RetType;
}

  // proto:  QString QTextItem::text();
impl<'a> /*trait*/ QTextItem_text<QString> for () {
  fn text(self , rsthis: & QTextItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextItem4textEv()};
    let mut ret = unsafe {_ZNK9QTextItem4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPaintEngineState {
    return QPaintEngineState{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  qreal QPaintEngineState::opacity();
impl /*struct*/ QPaintEngineState {
  pub fn opacity<RetType, T: QPaintEngineState_opacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opacity(self);
    // return 1;
  }
}

pub trait QPaintEngineState_opacity<RetType> {
  fn opacity(self , rsthis: & QPaintEngineState) -> RetType;
}

  // proto:  qreal QPaintEngineState::opacity();
impl<'a> /*trait*/ QPaintEngineState_opacity<f64> for () {
  fn opacity(self , rsthis: & QPaintEngineState) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState7opacityEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState7opacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QMatrix QPaintEngineState::matrix();
impl /*struct*/ QPaintEngineState {
  pub fn matrix<RetType, T: QPaintEngineState_matrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.matrix(self);
    // return 1;
  }
}

pub trait QPaintEngineState_matrix<RetType> {
  fn matrix(self , rsthis: & QPaintEngineState) -> RetType;
}

  // proto:  QMatrix QPaintEngineState::matrix();
impl<'a> /*trait*/ QPaintEngineState_matrix<QMatrix> for () {
  fn matrix(self , rsthis: & QPaintEngineState) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState6matrixEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState6matrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPainter * QPaintEngineState::painter();
impl /*struct*/ QPaintEngineState {
  pub fn painter<RetType, T: QPaintEngineState_painter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.painter(self);
    // return 1;
  }
}

pub trait QPaintEngineState_painter<RetType> {
  fn painter(self , rsthis: & QPaintEngineState) -> RetType;
}

  // proto:  QPainter * QPaintEngineState::painter();
impl<'a> /*trait*/ QPaintEngineState_painter<QPainter> for () {
  fn painter(self , rsthis: & QPaintEngineState) -> QPainter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState7painterEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState7painterEv(rsthis.qclsinst)};
    let mut ret1 = QPainter::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTransform QPaintEngineState::transform();
impl /*struct*/ QPaintEngineState {
  pub fn transform<RetType, T: QPaintEngineState_transform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.transform(self);
    // return 1;
  }
}

pub trait QPaintEngineState_transform<RetType> {
  fn transform(self , rsthis: & QPaintEngineState) -> RetType;
}

  // proto:  QTransform QPaintEngineState::transform();
impl<'a> /*trait*/ QPaintEngineState_transform<QTransform> for () {
  fn transform(self , rsthis: & QPaintEngineState) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState9transformEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState9transformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QPaintEngineState::brushOrigin();
impl /*struct*/ QPaintEngineState {
  pub fn brushOrigin<RetType, T: QPaintEngineState_brushOrigin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.brushOrigin(self);
    // return 1;
  }
}

pub trait QPaintEngineState_brushOrigin<RetType> {
  fn brushOrigin(self , rsthis: & QPaintEngineState) -> RetType;
}

  // proto:  QPointF QPaintEngineState::brushOrigin();
impl<'a> /*trait*/ QPaintEngineState_brushOrigin<QPointF> for () {
  fn brushOrigin(self , rsthis: & QPaintEngineState) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState11brushOriginEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState11brushOriginEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPaintEngineState::penNeedsResolving();
impl /*struct*/ QPaintEngineState {
  pub fn penNeedsResolving<RetType, T: QPaintEngineState_penNeedsResolving<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.penNeedsResolving(self);
    // return 1;
  }
}

pub trait QPaintEngineState_penNeedsResolving<RetType> {
  fn penNeedsResolving(self , rsthis: & QPaintEngineState) -> RetType;
}

  // proto:  bool QPaintEngineState::penNeedsResolving();
impl<'a> /*trait*/ QPaintEngineState_penNeedsResolving<i8> for () {
  fn penNeedsResolving(self , rsthis: & QPaintEngineState) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState17penNeedsResolvingEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState17penNeedsResolvingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QPaintEngineState::isClipEnabled();
impl /*struct*/ QPaintEngineState {
  pub fn isClipEnabled<RetType, T: QPaintEngineState_isClipEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isClipEnabled(self);
    // return 1;
  }
}

pub trait QPaintEngineState_isClipEnabled<RetType> {
  fn isClipEnabled(self , rsthis: & QPaintEngineState) -> RetType;
}

  // proto:  bool QPaintEngineState::isClipEnabled();
impl<'a> /*trait*/ QPaintEngineState_isClipEnabled<i8> for () {
  fn isClipEnabled(self , rsthis: & QPaintEngineState) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState13isClipEnabledEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState13isClipEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QFont QPaintEngineState::font();
impl /*struct*/ QPaintEngineState {
  pub fn font<RetType, T: QPaintEngineState_font<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QPaintEngineState_font<RetType> {
  fn font(self , rsthis: & QPaintEngineState) -> RetType;
}

  // proto:  QFont QPaintEngineState::font();
impl<'a> /*trait*/ QPaintEngineState_font<QFont> for () {
  fn font(self , rsthis: & QPaintEngineState) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState4fontEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPaintEngineState::brushNeedsResolving();
impl /*struct*/ QPaintEngineState {
  pub fn brushNeedsResolving<RetType, T: QPaintEngineState_brushNeedsResolving<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.brushNeedsResolving(self);
    // return 1;
  }
}

pub trait QPaintEngineState_brushNeedsResolving<RetType> {
  fn brushNeedsResolving(self , rsthis: & QPaintEngineState) -> RetType;
}

  // proto:  bool QPaintEngineState::brushNeedsResolving();
impl<'a> /*trait*/ QPaintEngineState_brushNeedsResolving<i8> for () {
  fn brushNeedsResolving(self , rsthis: & QPaintEngineState) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState19brushNeedsResolvingEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState19brushNeedsResolvingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRegion QPaintEngineState::clipRegion();
impl /*struct*/ QPaintEngineState {
  pub fn clipRegion<RetType, T: QPaintEngineState_clipRegion<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clipRegion(self);
    // return 1;
  }
}

pub trait QPaintEngineState_clipRegion<RetType> {
  fn clipRegion(self , rsthis: & QPaintEngineState) -> RetType;
}

  // proto:  QRegion QPaintEngineState::clipRegion();
impl<'a> /*trait*/ QPaintEngineState_clipRegion<QRegion> for () {
  fn clipRegion(self , rsthis: & QPaintEngineState) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState10clipRegionEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState10clipRegionEv(rsthis.qclsinst)};
    let mut ret1 = QRegion::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPainterPath QPaintEngineState::clipPath();
impl /*struct*/ QPaintEngineState {
  pub fn clipPath<RetType, T: QPaintEngineState_clipPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clipPath(self);
    // return 1;
  }
}

pub trait QPaintEngineState_clipPath<RetType> {
  fn clipPath(self , rsthis: & QPaintEngineState) -> RetType;
}

  // proto:  QPainterPath QPaintEngineState::clipPath();
impl<'a> /*trait*/ QPaintEngineState_clipPath<QPainterPath> for () {
  fn clipPath(self , rsthis: & QPaintEngineState) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState8clipPathEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState8clipPathEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QBrush QPaintEngineState::brush();
impl /*struct*/ QPaintEngineState {
  pub fn brush<RetType, T: QPaintEngineState_brush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.brush(self);
    // return 1;
  }
}

pub trait QPaintEngineState_brush<RetType> {
  fn brush(self , rsthis: & QPaintEngineState) -> RetType;
}

  // proto:  QBrush QPaintEngineState::brush();
impl<'a> /*trait*/ QPaintEngineState_brush<QBrush> for () {
  fn brush(self , rsthis: & QPaintEngineState) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState5brushEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState5brushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPen QPaintEngineState::pen();
impl /*struct*/ QPaintEngineState {
  pub fn pen<RetType, T: QPaintEngineState_pen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pen(self);
    // return 1;
  }
}

pub trait QPaintEngineState_pen<RetType> {
  fn pen(self , rsthis: & QPaintEngineState) -> RetType;
}

  // proto:  QPen QPaintEngineState::pen();
impl<'a> /*trait*/ QPaintEngineState_pen<QPen> for () {
  fn pen(self , rsthis: & QPaintEngineState) -> QPen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState3penEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState3penEv(rsthis.qclsinst)};
    let mut ret1 = QPen::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QBrush QPaintEngineState::backgroundBrush();
impl /*struct*/ QPaintEngineState {
  pub fn backgroundBrush<RetType, T: QPaintEngineState_backgroundBrush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.backgroundBrush(self);
    // return 1;
  }
}

pub trait QPaintEngineState_backgroundBrush<RetType> {
  fn backgroundBrush(self , rsthis: & QPaintEngineState) -> RetType;
}

  // proto:  QBrush QPaintEngineState::backgroundBrush();
impl<'a> /*trait*/ QPaintEngineState_backgroundBrush<QBrush> for () {
  fn backgroundBrush(self , rsthis: & QPaintEngineState) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState15backgroundBrushEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState15backgroundBrushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPaintEngine {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPaintEngine {
    return QPaintEngine{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QPaintEngine::drawEllipse(const QRect & r);
impl /*struct*/ QPaintEngine {
  pub fn drawEllipse<RetType, T: QPaintEngine_drawEllipse<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawEllipse(self);
    // return 1;
  }
}

pub trait QPaintEngine_drawEllipse<RetType> {
  fn drawEllipse(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::drawEllipse(const QRect & r);
impl<'a> /*trait*/ QPaintEngine_drawEllipse<()> for (&'a QRect) {
  fn drawEllipse(self , rsthis: & QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine11drawEllipseERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPaintEngine11drawEllipseERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::QPaintEngine(const QPaintEngine & );
impl /*struct*/ QPaintEngine {
  pub fn new<T: QPaintEngine_new>(value: T) -> QPaintEngine {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPaintEngine_new {
  fn new(self) -> QPaintEngine;
}

  // proto:  void QPaintEngine::QPaintEngine(const QPaintEngine & );
impl<'a> /*trait*/ QPaintEngine_new for (&'a QPaintEngine) {
  fn new(self) -> QPaintEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngineC2ERKS_()};
    let ctysz: c_int = unsafe{QPaintEngine_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QPaintEngineC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QPaintEngine{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QPaintEngine::isActive();
impl /*struct*/ QPaintEngine {
  pub fn isActive<RetType, T: QPaintEngine_isActive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isActive(self);
    // return 1;
  }
}

pub trait QPaintEngine_isActive<RetType> {
  fn isActive(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  bool QPaintEngine::isActive();
impl<'a> /*trait*/ QPaintEngine_isActive<i8> for () {
  fn isActive(self , rsthis: & QPaintEngine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine8isActiveEv()};
    let mut ret = unsafe {_ZNK12QPaintEngine8isActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawPoints(const QPointF * points, int pointCount);
impl /*struct*/ QPaintEngine {
  pub fn drawPoints<RetType, T: QPaintEngine_drawPoints<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawPoints(self);
    // return 1;
  }
}

pub trait QPaintEngine_drawPoints<RetType> {
  fn drawPoints(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::drawPoints(const QPointF * points, int pointCount);
impl<'a> /*trait*/ QPaintEngine_drawPoints<()> for (&'a QPointF, i32) {
  fn drawPoints(self , rsthis: & QPaintEngine) -> () {
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
  pub fn coordinateOffset<RetType, T: QPaintEngine_coordinateOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.coordinateOffset(self);
    // return 1;
  }
}

pub trait QPaintEngine_coordinateOffset<RetType> {
  fn coordinateOffset(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  QPoint QPaintEngine::coordinateOffset();
impl<'a> /*trait*/ QPaintEngine_coordinateOffset<QPoint> for () {
  fn coordinateOffset(self , rsthis: & QPaintEngine) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine16coordinateOffsetEv()};
    let mut ret = unsafe {_ZNK12QPaintEngine16coordinateOffsetEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPaintEngine::setPaintDevice(QPaintDevice * device);
impl /*struct*/ QPaintEngine {
  pub fn setPaintDevice<RetType, T: QPaintEngine_setPaintDevice<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPaintDevice(self);
    // return 1;
  }
}

pub trait QPaintEngine_setPaintDevice<RetType> {
  fn setPaintDevice(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::setPaintDevice(QPaintDevice * device);
impl<'a> /*trait*/ QPaintEngine_setPaintDevice<()> for (&'a QPaintDevice) {
  fn setPaintDevice(self , rsthis: & QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine14setPaintDeviceEP12QPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPaintEngine14setPaintDeviceEP12QPaintDevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::setSystemRect(const QRect & rect);
impl /*struct*/ QPaintEngine {
  pub fn setSystemRect<RetType, T: QPaintEngine_setSystemRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSystemRect(self);
    // return 1;
  }
}

pub trait QPaintEngine_setSystemRect<RetType> {
  fn setSystemRect(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::setSystemRect(const QRect & rect);
impl<'a> /*trait*/ QPaintEngine_setSystemRect<()> for (&'a QRect) {
  fn setSystemRect(self , rsthis: & QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine13setSystemRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPaintEngine13setSystemRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::~QPaintEngine();
impl /*struct*/ QPaintEngine {
  pub fn free<RetType, T: QPaintEngine_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QPaintEngine_free<RetType> {
  fn free(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::~QPaintEngine();
impl<'a> /*trait*/ QPaintEngine_free<()> for () {
  fn free(self , rsthis: & QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngineD2Ev()};
     unsafe {_ZN12QPaintEngineD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QPaintEngine::end();
impl /*struct*/ QPaintEngine {
  pub fn end<RetType, T: QPaintEngine_end<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.end(self);
    // return 1;
  }
}

pub trait QPaintEngine_end<RetType> {
  fn end(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  bool QPaintEngine::end();
impl<'a> /*trait*/ QPaintEngine_end<i8> for () {
  fn end(self , rsthis: & QPaintEngine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine3endEv()};
    let mut ret = unsafe {_ZN12QPaintEngine3endEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawTiledPixmap(const QRectF & r, const QPixmap & pixmap, const QPointF & s);
impl /*struct*/ QPaintEngine {
  pub fn drawTiledPixmap<RetType, T: QPaintEngine_drawTiledPixmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawTiledPixmap(self);
    // return 1;
  }
}

pub trait QPaintEngine_drawTiledPixmap<RetType> {
  fn drawTiledPixmap(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::drawTiledPixmap(const QRectF & r, const QPixmap & pixmap, const QPointF & s);
impl<'a> /*trait*/ QPaintEngine_drawTiledPixmap<()> for (&'a QRectF, &'a QPixmap, &'a QPointF) {
  fn drawTiledPixmap(self , rsthis: & QPaintEngine) -> () {
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
  pub fn setActive<RetType, T: QPaintEngine_setActive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setActive(self);
    // return 1;
  }
}

pub trait QPaintEngine_setActive<RetType> {
  fn setActive(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::setActive(bool newState);
impl<'a> /*trait*/ QPaintEngine_setActive<()> for (i8) {
  fn setActive(self , rsthis: & QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine9setActiveEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QPaintEngine9setActiveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawPixmap(const QRectF & r, const QPixmap & pm, const QRectF & sr);
impl /*struct*/ QPaintEngine {
  pub fn drawPixmap<RetType, T: QPaintEngine_drawPixmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawPixmap(self);
    // return 1;
  }
}

pub trait QPaintEngine_drawPixmap<RetType> {
  fn drawPixmap(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::drawPixmap(const QRectF & r, const QPixmap & pm, const QRectF & sr);
impl<'a> /*trait*/ QPaintEngine_drawPixmap<()> for (&'a QRectF, &'a QPixmap, &'a QRectF) {
  fn drawPixmap(self , rsthis: & QPaintEngine) -> () {
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
  pub fn drawLines<RetType, T: QPaintEngine_drawLines<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawLines(self);
    // return 1;
  }
}

pub trait QPaintEngine_drawLines<RetType> {
  fn drawLines(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::drawLines(const QLine * lines, int lineCount);
impl<'a> /*trait*/ QPaintEngine_drawLines<()> for (&'a QLine, i32) {
  fn drawLines(self , rsthis: & QPaintEngine) -> () {
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
  pub fn drawPath<RetType, T: QPaintEngine_drawPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawPath(self);
    // return 1;
  }
}

pub trait QPaintEngine_drawPath<RetType> {
  fn drawPath(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::drawPath(const QPainterPath & path);
impl<'a> /*trait*/ QPaintEngine_drawPath<()> for (&'a QPainterPath) {
  fn drawPath(self , rsthis: & QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine8drawPathERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPaintEngine8drawPathERK12QPainterPath(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawLines(const QLineF * lines, int lineCount);
impl<'a> /*trait*/ QPaintEngine_drawLines<()> for (&'a QLineF, i32) {
  fn drawLines(self , rsthis: & QPaintEngine) -> () {
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
  pub fn updateState<RetType, T: QPaintEngine_updateState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.updateState(self);
    // return 1;
  }
}

pub trait QPaintEngine_updateState<RetType> {
  fn updateState(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::updateState(const QPaintEngineState & state);
impl<'a> /*trait*/ QPaintEngine_updateState<()> for (&'a QPaintEngineState) {
  fn updateState(self , rsthis: & QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine11updateStateERK17QPaintEngineState()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPaintEngine11updateStateERK17QPaintEngineState(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPaintEngine::begin(QPaintDevice * pdev);
impl /*struct*/ QPaintEngine {
  pub fn begin<RetType, T: QPaintEngine_begin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.begin(self);
    // return 1;
  }
}

pub trait QPaintEngine_begin<RetType> {
  fn begin(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  bool QPaintEngine::begin(QPaintDevice * pdev);
impl<'a> /*trait*/ QPaintEngine_begin<i8> for (&'a QPaintDevice) {
  fn begin(self , rsthis: & QPaintEngine) -> i8 {
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
  pub fn systemRect<RetType, T: QPaintEngine_systemRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.systemRect(self);
    // return 1;
  }
}

pub trait QPaintEngine_systemRect<RetType> {
  fn systemRect(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  QRect QPaintEngine::systemRect();
impl<'a> /*trait*/ QPaintEngine_systemRect<QRect> for () {
  fn systemRect(self , rsthis: & QPaintEngine) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine10systemRectEv()};
    let mut ret = unsafe {_ZNK12QPaintEngine10systemRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawRects(const QRectF * rects, int rectCount);
impl /*struct*/ QPaintEngine {
  pub fn drawRects<RetType, T: QPaintEngine_drawRects<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawRects(self);
    // return 1;
  }
}

pub trait QPaintEngine_drawRects<RetType> {
  fn drawRects(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::drawRects(const QRectF * rects, int rectCount);
impl<'a> /*trait*/ QPaintEngine_drawRects<()> for (&'a QRectF, i32) {
  fn drawRects(self , rsthis: & QPaintEngine) -> () {
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
  pub fn setSystemClip<RetType, T: QPaintEngine_setSystemClip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSystemClip(self);
    // return 1;
  }
}

pub trait QPaintEngine_setSystemClip<RetType> {
  fn setSystemClip(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::setSystemClip(const QRegion & baseClip);
impl<'a> /*trait*/ QPaintEngine_setSystemClip<()> for (&'a QRegion) {
  fn setSystemClip(self , rsthis: & QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine13setSystemClipERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPaintEngine13setSystemClipERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRegion QPaintEngine::systemClip();
impl /*struct*/ QPaintEngine {
  pub fn systemClip<RetType, T: QPaintEngine_systemClip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.systemClip(self);
    // return 1;
  }
}

pub trait QPaintEngine_systemClip<RetType> {
  fn systemClip(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  QRegion QPaintEngine::systemClip();
impl<'a> /*trait*/ QPaintEngine_systemClip<QRegion> for () {
  fn systemClip(self , rsthis: & QPaintEngine) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine10systemClipEv()};
    let mut ret = unsafe {_ZNK12QPaintEngine10systemClipEv(rsthis.qclsinst)};
    let mut ret1 = QRegion::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPaintDevice * QPaintEngine::paintDevice();
impl /*struct*/ QPaintEngine {
  pub fn paintDevice<RetType, T: QPaintEngine_paintDevice<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paintDevice(self);
    // return 1;
  }
}

pub trait QPaintEngine_paintDevice<RetType> {
  fn paintDevice(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  QPaintDevice * QPaintEngine::paintDevice();
impl<'a> /*trait*/ QPaintEngine_paintDevice<QPaintDevice> for () {
  fn paintDevice(self , rsthis: & QPaintEngine) -> QPaintDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine11paintDeviceEv()};
    let mut ret = unsafe {_ZNK12QPaintEngine11paintDeviceEv(rsthis.qclsinst)};
    let mut ret1 = QPaintDevice::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPaintEngine::syncState();
impl /*struct*/ QPaintEngine {
  pub fn syncState<RetType, T: QPaintEngine_syncState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.syncState(self);
    // return 1;
  }
}

pub trait QPaintEngine_syncState<RetType> {
  fn syncState(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::syncState();
impl<'a> /*trait*/ QPaintEngine_syncState<()> for () {
  fn syncState(self , rsthis: & QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine9syncStateEv()};
     unsafe {_ZN12QPaintEngine9syncStateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPainter * QPaintEngine::painter();
impl /*struct*/ QPaintEngine {
  pub fn painter<RetType, T: QPaintEngine_painter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.painter(self);
    // return 1;
  }
}

pub trait QPaintEngine_painter<RetType> {
  fn painter(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  QPainter * QPaintEngine::painter();
impl<'a> /*trait*/ QPaintEngine_painter<QPainter> for () {
  fn painter(self , rsthis: & QPaintEngine) -> QPainter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine7painterEv()};
    let mut ret = unsafe {_ZNK12QPaintEngine7painterEv(rsthis.qclsinst)};
    let mut ret1 = QPainter::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawEllipse(const QRectF & r);
impl<'a> /*trait*/ QPaintEngine_drawEllipse<()> for (&'a QRectF) {
  fn drawEllipse(self , rsthis: & QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine11drawEllipseERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPaintEngine11drawEllipseERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawTextItem(const QPointF & p, const QTextItem & textItem);
impl /*struct*/ QPaintEngine {
  pub fn drawTextItem<RetType, T: QPaintEngine_drawTextItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawTextItem(self);
    // return 1;
  }
}

pub trait QPaintEngine_drawTextItem<RetType> {
  fn drawTextItem(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::drawTextItem(const QPointF & p, const QTextItem & textItem);
impl<'a> /*trait*/ QPaintEngine_drawTextItem<()> for (&'a QPointF, &'a QTextItem) {
  fn drawTextItem(self , rsthis: & QPaintEngine) -> () {
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
  pub fn fix_neg_rect<RetType, T: QPaintEngine_fix_neg_rect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fix_neg_rect(self);
    // return 1;
  }
}

pub trait QPaintEngine_fix_neg_rect<RetType> {
  fn fix_neg_rect(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  void QPaintEngine::fix_neg_rect(int * x, int * y, int * w, int * h);
impl<'a> /*trait*/ QPaintEngine_fix_neg_rect<()> for (&'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<i32>) {
  fn fix_neg_rect(self , rsthis: & QPaintEngine) -> () {
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
  pub fn isExtended<RetType, T: QPaintEngine_isExtended<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isExtended(self);
    // return 1;
  }
}

pub trait QPaintEngine_isExtended<RetType> {
  fn isExtended(self , rsthis: & QPaintEngine) -> RetType;
}

  // proto:  bool QPaintEngine::isExtended();
impl<'a> /*trait*/ QPaintEngine_isExtended<i8> for () {
  fn isExtended(self , rsthis: & QPaintEngine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPaintEngine10isExtendedEv()};
    let mut ret = unsafe {_ZNK12QPaintEngine10isExtendedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawRects(const QRect * rects, int rectCount);
impl<'a> /*trait*/ QPaintEngine_drawRects<()> for (&'a QRect, i32) {
  fn drawRects(self , rsthis: & QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine9drawRectsEPK5QRecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QPaintEngine9drawRectsEPK5QRecti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPaintEngine::drawPoints(const QPoint * points, int pointCount);
impl<'a> /*trait*/ QPaintEngine_drawPoints<()> for (&'a QPoint, i32) {
  fn drawPoints(self , rsthis: & QPaintEngine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPaintEngine10drawPointsEPK6QPointi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QPaintEngine10drawPointsEPK6QPointi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// <= body block end

