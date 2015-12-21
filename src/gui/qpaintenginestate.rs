// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmatrix::QMatrix;
use super::qpainter::QPainter;
use super::qtransform::QTransform;
use super::qpointf::QPointF;
use super::qfont::QFont;
use super::qregion::QRegion;
use super::qpainterpath::QPainterPath;
use super::qbrush::QBrush;
use super::qpen::QPen;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
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
}

// body block begin
// class sizeof(QPaintEngineState)=1
pub struct QPaintEngineState {
  pub qclsinst: *mut c_void,
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

