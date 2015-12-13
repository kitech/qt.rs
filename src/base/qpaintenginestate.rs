// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: double QPaintEngineState::opacity();
  fn _ZNK17QPaintEngineState7opacityEv() -> i32;
  // proto: QMatrix QPaintEngineState::matrix();
  fn _ZNK17QPaintEngineState6matrixEv() -> i32;
  // proto: QPainter * QPaintEngineState::painter();
  fn _ZNK17QPaintEngineState7painterEv() -> i32;
  // proto: QTransform QPaintEngineState::transform();
  fn _ZNK17QPaintEngineState9transformEv() -> i32;
  // proto: QPointF QPaintEngineState::brushOrigin();
  fn _ZNK17QPaintEngineState11brushOriginEv() -> i32;
  // proto: bool QPaintEngineState::penNeedsResolving();
  fn _ZNK17QPaintEngineState17penNeedsResolvingEv() -> i32;
  // proto: bool QPaintEngineState::isClipEnabled();
  fn _ZNK17QPaintEngineState13isClipEnabledEv() -> i32;
  // proto: QFont QPaintEngineState::font();
  fn _ZNK17QPaintEngineState4fontEv() -> i32;
  // proto: bool QPaintEngineState::brushNeedsResolving();
  fn _ZNK17QPaintEngineState19brushNeedsResolvingEv() -> i32;
  // proto: QRegion QPaintEngineState::clipRegion();
  fn _ZNK17QPaintEngineState10clipRegionEv() -> i32;
  // proto: QPainterPath QPaintEngineState::clipPath();
  fn _ZNK17QPaintEngineState8clipPathEv() -> i32;
  // proto: QBrush QPaintEngineState::brush();
  fn _ZNK17QPaintEngineState5brushEv() -> i32;
  // proto: QPen QPaintEngineState::pen();
  fn _ZNK17QPaintEngineState3penEv() -> i32;
  // proto: QBrush QPaintEngineState::backgroundBrush();
  fn _ZNK17QPaintEngineState15backgroundBrushEv() -> i32;
}

// body block begin
// class sizeof(QPaintEngineState)=1
pub struct QPaintEngineState {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPaintEngineState {
  pub fn opacity<T: QPaintEngineState_opacity>(&mut self, value: T) -> i32 {
    value.opacity(self);
    return 1;
  }
}

pub trait QPaintEngineState_opacity {
  fn opacity(self, this: &mut QPaintEngineState) -> i32;
}

// proto: double QPaintEngineState::opacity();
impl<'a> /*trait*/ QPaintEngineState_opacity for () {
  fn opacity(self, this: &mut QPaintEngineState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState7opacityEv()};
    unsafe {_ZNK17QPaintEngineState7opacityEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn matrix<T: QPaintEngineState_matrix>(&mut self, value: T) -> i32 {
    value.matrix(self);
    return 1;
  }
}

pub trait QPaintEngineState_matrix {
  fn matrix(self, this: &mut QPaintEngineState) -> i32;
}

// proto: QMatrix QPaintEngineState::matrix();
impl<'a> /*trait*/ QPaintEngineState_matrix for () {
  fn matrix(self, this: &mut QPaintEngineState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState6matrixEv()};
    unsafe {_ZNK17QPaintEngineState6matrixEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn painter<T: QPaintEngineState_painter>(&mut self, value: T) -> i32 {
    value.painter(self);
    return 1;
  }
}

pub trait QPaintEngineState_painter {
  fn painter(self, this: &mut QPaintEngineState) -> i32;
}

// proto: QPainter * QPaintEngineState::painter();
impl<'a> /*trait*/ QPaintEngineState_painter for () {
  fn painter(self, this: &mut QPaintEngineState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState7painterEv()};
    unsafe {_ZNK17QPaintEngineState7painterEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn transform<T: QPaintEngineState_transform>(&mut self, value: T) -> i32 {
    value.transform(self);
    return 1;
  }
}

pub trait QPaintEngineState_transform {
  fn transform(self, this: &mut QPaintEngineState) -> i32;
}

// proto: QTransform QPaintEngineState::transform();
impl<'a> /*trait*/ QPaintEngineState_transform for () {
  fn transform(self, this: &mut QPaintEngineState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState9transformEv()};
    unsafe {_ZNK17QPaintEngineState9transformEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn brushOrigin<T: QPaintEngineState_brushOrigin>(&mut self, value: T) -> i32 {
    value.brushOrigin(self);
    return 1;
  }
}

pub trait QPaintEngineState_brushOrigin {
  fn brushOrigin(self, this: &mut QPaintEngineState) -> i32;
}

// proto: QPointF QPaintEngineState::brushOrigin();
impl<'a> /*trait*/ QPaintEngineState_brushOrigin for () {
  fn brushOrigin(self, this: &mut QPaintEngineState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState11brushOriginEv()};
    unsafe {_ZNK17QPaintEngineState11brushOriginEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn penNeedsResolving<T: QPaintEngineState_penNeedsResolving>(&mut self, value: T) -> i32 {
    value.penNeedsResolving(self);
    return 1;
  }
}

pub trait QPaintEngineState_penNeedsResolving {
  fn penNeedsResolving(self, this: &mut QPaintEngineState) -> i32;
}

// proto: bool QPaintEngineState::penNeedsResolving();
impl<'a> /*trait*/ QPaintEngineState_penNeedsResolving for () {
  fn penNeedsResolving(self, this: &mut QPaintEngineState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState17penNeedsResolvingEv()};
    unsafe {_ZNK17QPaintEngineState17penNeedsResolvingEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn isClipEnabled<T: QPaintEngineState_isClipEnabled>(&mut self, value: T) -> i32 {
    value.isClipEnabled(self);
    return 1;
  }
}

pub trait QPaintEngineState_isClipEnabled {
  fn isClipEnabled(self, this: &mut QPaintEngineState) -> i32;
}

// proto: bool QPaintEngineState::isClipEnabled();
impl<'a> /*trait*/ QPaintEngineState_isClipEnabled for () {
  fn isClipEnabled(self, this: &mut QPaintEngineState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState13isClipEnabledEv()};
    unsafe {_ZNK17QPaintEngineState13isClipEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn font<T: QPaintEngineState_font>(&mut self, value: T) -> i32 {
    value.font(self);
    return 1;
  }
}

pub trait QPaintEngineState_font {
  fn font(self, this: &mut QPaintEngineState) -> i32;
}

// proto: QFont QPaintEngineState::font();
impl<'a> /*trait*/ QPaintEngineState_font for () {
  fn font(self, this: &mut QPaintEngineState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState4fontEv()};
    unsafe {_ZNK17QPaintEngineState4fontEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn brushNeedsResolving<T: QPaintEngineState_brushNeedsResolving>(&mut self, value: T) -> i32 {
    value.brushNeedsResolving(self);
    return 1;
  }
}

pub trait QPaintEngineState_brushNeedsResolving {
  fn brushNeedsResolving(self, this: &mut QPaintEngineState) -> i32;
}

// proto: bool QPaintEngineState::brushNeedsResolving();
impl<'a> /*trait*/ QPaintEngineState_brushNeedsResolving for () {
  fn brushNeedsResolving(self, this: &mut QPaintEngineState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState19brushNeedsResolvingEv()};
    unsafe {_ZNK17QPaintEngineState19brushNeedsResolvingEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn clipRegion<T: QPaintEngineState_clipRegion>(&mut self, value: T) -> i32 {
    value.clipRegion(self);
    return 1;
  }
}

pub trait QPaintEngineState_clipRegion {
  fn clipRegion(self, this: &mut QPaintEngineState) -> i32;
}

// proto: QRegion QPaintEngineState::clipRegion();
impl<'a> /*trait*/ QPaintEngineState_clipRegion for () {
  fn clipRegion(self, this: &mut QPaintEngineState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState10clipRegionEv()};
    unsafe {_ZNK17QPaintEngineState10clipRegionEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn clipPath<T: QPaintEngineState_clipPath>(&mut self, value: T) -> i32 {
    value.clipPath(self);
    return 1;
  }
}

pub trait QPaintEngineState_clipPath {
  fn clipPath(self, this: &mut QPaintEngineState) -> i32;
}

// proto: QPainterPath QPaintEngineState::clipPath();
impl<'a> /*trait*/ QPaintEngineState_clipPath for () {
  fn clipPath(self, this: &mut QPaintEngineState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState8clipPathEv()};
    unsafe {_ZNK17QPaintEngineState8clipPathEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn brush<T: QPaintEngineState_brush>(&mut self, value: T) -> i32 {
    value.brush(self);
    return 1;
  }
}

pub trait QPaintEngineState_brush {
  fn brush(self, this: &mut QPaintEngineState) -> i32;
}

// proto: QBrush QPaintEngineState::brush();
impl<'a> /*trait*/ QPaintEngineState_brush for () {
  fn brush(self, this: &mut QPaintEngineState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState5brushEv()};
    unsafe {_ZNK17QPaintEngineState5brushEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn pen<T: QPaintEngineState_pen>(&mut self, value: T) -> i32 {
    value.pen(self);
    return 1;
  }
}

pub trait QPaintEngineState_pen {
  fn pen(self, this: &mut QPaintEngineState) -> i32;
}

// proto: QPen QPaintEngineState::pen();
impl<'a> /*trait*/ QPaintEngineState_pen for () {
  fn pen(self, this: &mut QPaintEngineState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState3penEv()};
    unsafe {_ZNK17QPaintEngineState3penEv()};
    return 1;
  }
}

impl /*struct*/ QPaintEngineState {
  pub fn backgroundBrush<T: QPaintEngineState_backgroundBrush>(&mut self, value: T) -> i32 {
    value.backgroundBrush(self);
    return 1;
  }
}

pub trait QPaintEngineState_backgroundBrush {
  fn backgroundBrush(self, this: &mut QPaintEngineState) -> i32;
}

// proto: QBrush QPaintEngineState::backgroundBrush();
impl<'a> /*trait*/ QPaintEngineState_backgroundBrush for () {
  fn backgroundBrush(self, this: &mut QPaintEngineState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QPaintEngineState15backgroundBrushEv()};
    unsafe {_ZNK17QPaintEngineState15backgroundBrushEv()};
    return 1;
  }
}

