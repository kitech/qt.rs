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
  // proto:  double QPaintEngineState::opacity();
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
  fn _ZNK17QPaintEngineState17penNeedsResolvingEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QPaintEngineState::isClipEnabled();
  fn _ZNK17QPaintEngineState13isClipEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  QFont QPaintEngineState::font();
  fn _ZNK17QPaintEngineState4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QPaintEngineState::brushNeedsResolving();
  fn _ZNK17QPaintEngineState19brushNeedsResolvingEv(qthis: *mut c_void) -> int8_t;
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

impl /*struct*/ QPaintEngineState {
  pub fn opacity<T: QPaintEngineState_opacity>(&mut self, value: T) -> f64 {
    return value.opacity(self);
    // return 1;
  }
}

pub trait QPaintEngineState_opacity {
  fn opacity(self, rsthis: &mut QPaintEngineState) -> f64;
}

// proto:  double QPaintEngineState::opacity();
impl<'a> /*trait*/ QPaintEngineState_opacity for () {
  fn opacity(self, rsthis: &mut QPaintEngineState) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState7opacityEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState7opacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn matrix<T: QPaintEngineState_matrix>(&mut self, value: T) -> QMatrix {
    return value.matrix(self);
    // return 1;
  }
}

pub trait QPaintEngineState_matrix {
  fn matrix(self, rsthis: &mut QPaintEngineState) -> QMatrix;
}

// proto:  QMatrix QPaintEngineState::matrix();
impl<'a> /*trait*/ QPaintEngineState_matrix for () {
  fn matrix(self, rsthis: &mut QPaintEngineState) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState6matrixEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState6matrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn painter<T: QPaintEngineState_painter>(&mut self, value: T) -> QPainter {
    return value.painter(self);
    // return 1;
  }
}

pub trait QPaintEngineState_painter {
  fn painter(self, rsthis: &mut QPaintEngineState) -> QPainter;
}

// proto:  QPainter * QPaintEngineState::painter();
impl<'a> /*trait*/ QPaintEngineState_painter for () {
  fn painter(self, rsthis: &mut QPaintEngineState) -> QPainter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState7painterEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState7painterEv(rsthis.qclsinst)};
    let mut ret1 = QPainter{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn transform<T: QPaintEngineState_transform>(&mut self, value: T) -> QTransform {
    return value.transform(self);
    // return 1;
  }
}

pub trait QPaintEngineState_transform {
  fn transform(self, rsthis: &mut QPaintEngineState) -> QTransform;
}

// proto:  QTransform QPaintEngineState::transform();
impl<'a> /*trait*/ QPaintEngineState_transform for () {
  fn transform(self, rsthis: &mut QPaintEngineState) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState9transformEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState9transformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn brushOrigin<T: QPaintEngineState_brushOrigin>(&mut self, value: T) -> QPointF {
    return value.brushOrigin(self);
    // return 1;
  }
}

pub trait QPaintEngineState_brushOrigin {
  fn brushOrigin(self, rsthis: &mut QPaintEngineState) -> QPointF;
}

// proto:  QPointF QPaintEngineState::brushOrigin();
impl<'a> /*trait*/ QPaintEngineState_brushOrigin for () {
  fn brushOrigin(self, rsthis: &mut QPaintEngineState) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState11brushOriginEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState11brushOriginEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn penNeedsResolving<T: QPaintEngineState_penNeedsResolving>(&mut self, value: T) -> i8 {
    return value.penNeedsResolving(self);
    // return 1;
  }
}

pub trait QPaintEngineState_penNeedsResolving {
  fn penNeedsResolving(self, rsthis: &mut QPaintEngineState) -> i8;
}

// proto:  bool QPaintEngineState::penNeedsResolving();
impl<'a> /*trait*/ QPaintEngineState_penNeedsResolving for () {
  fn penNeedsResolving(self, rsthis: &mut QPaintEngineState) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState17penNeedsResolvingEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState17penNeedsResolvingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn isClipEnabled<T: QPaintEngineState_isClipEnabled>(&mut self, value: T) -> i8 {
    return value.isClipEnabled(self);
    // return 1;
  }
}

pub trait QPaintEngineState_isClipEnabled {
  fn isClipEnabled(self, rsthis: &mut QPaintEngineState) -> i8;
}

// proto:  bool QPaintEngineState::isClipEnabled();
impl<'a> /*trait*/ QPaintEngineState_isClipEnabled for () {
  fn isClipEnabled(self, rsthis: &mut QPaintEngineState) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState13isClipEnabledEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState13isClipEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn font<T: QPaintEngineState_font>(&mut self, value: T) -> QFont {
    return value.font(self);
    // return 1;
  }
}

pub trait QPaintEngineState_font {
  fn font(self, rsthis: &mut QPaintEngineState) -> QFont;
}

// proto:  QFont QPaintEngineState::font();
impl<'a> /*trait*/ QPaintEngineState_font for () {
  fn font(self, rsthis: &mut QPaintEngineState) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState4fontEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn brushNeedsResolving<T: QPaintEngineState_brushNeedsResolving>(&mut self, value: T) -> i8 {
    return value.brushNeedsResolving(self);
    // return 1;
  }
}

pub trait QPaintEngineState_brushNeedsResolving {
  fn brushNeedsResolving(self, rsthis: &mut QPaintEngineState) -> i8;
}

// proto:  bool QPaintEngineState::brushNeedsResolving();
impl<'a> /*trait*/ QPaintEngineState_brushNeedsResolving for () {
  fn brushNeedsResolving(self, rsthis: &mut QPaintEngineState) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState19brushNeedsResolvingEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState19brushNeedsResolvingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn clipRegion<T: QPaintEngineState_clipRegion>(&mut self, value: T) -> QRegion {
    return value.clipRegion(self);
    // return 1;
  }
}

pub trait QPaintEngineState_clipRegion {
  fn clipRegion(self, rsthis: &mut QPaintEngineState) -> QRegion;
}

// proto:  QRegion QPaintEngineState::clipRegion();
impl<'a> /*trait*/ QPaintEngineState_clipRegion for () {
  fn clipRegion(self, rsthis: &mut QPaintEngineState) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState10clipRegionEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState10clipRegionEv(rsthis.qclsinst)};
    let mut ret1 = QRegion{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn clipPath<T: QPaintEngineState_clipPath>(&mut self, value: T) -> QPainterPath {
    return value.clipPath(self);
    // return 1;
  }
}

pub trait QPaintEngineState_clipPath {
  fn clipPath(self, rsthis: &mut QPaintEngineState) -> QPainterPath;
}

// proto:  QPainterPath QPaintEngineState::clipPath();
impl<'a> /*trait*/ QPaintEngineState_clipPath for () {
  fn clipPath(self, rsthis: &mut QPaintEngineState) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState8clipPathEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState8clipPathEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn brush<T: QPaintEngineState_brush>(&mut self, value: T) -> QBrush {
    return value.brush(self);
    // return 1;
  }
}

pub trait QPaintEngineState_brush {
  fn brush(self, rsthis: &mut QPaintEngineState) -> QBrush;
}

// proto:  QBrush QPaintEngineState::brush();
impl<'a> /*trait*/ QPaintEngineState_brush for () {
  fn brush(self, rsthis: &mut QPaintEngineState) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState5brushEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState5brushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn pen<T: QPaintEngineState_pen>(&mut self, value: T) -> QPen {
    return value.pen(self);
    // return 1;
  }
}

pub trait QPaintEngineState_pen {
  fn pen(self, rsthis: &mut QPaintEngineState) -> QPen;
}

// proto:  QPen QPaintEngineState::pen();
impl<'a> /*trait*/ QPaintEngineState_pen for () {
  fn pen(self, rsthis: &mut QPaintEngineState) -> QPen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState3penEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState3penEv(rsthis.qclsinst)};
    let mut ret1 = QPen{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn backgroundBrush<T: QPaintEngineState_backgroundBrush>(&mut self, value: T) -> QBrush {
    return value.backgroundBrush(self);
    // return 1;
  }
}

pub trait QPaintEngineState_backgroundBrush {
  fn backgroundBrush(self, rsthis: &mut QPaintEngineState) -> QBrush;
}

// proto:  QBrush QPaintEngineState::backgroundBrush();
impl<'a> /*trait*/ QPaintEngineState_backgroundBrush for () {
  fn backgroundBrush(self, rsthis: &mut QPaintEngineState) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState15backgroundBrushEv()};
    let mut ret = unsafe {_ZNK17QPaintEngineState15backgroundBrushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

