// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpen::QPen;
use super::qpainterpath::QPainterPath;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: double QPainterPathStroker::curveThreshold();
  fn _ZNK19QPainterPathStroker14curveThresholdEv() -> i32;
  // proto: void QPainterPathStroker::setWidth(qreal width);
  fn _ZN19QPainterPathStroker8setWidthEd(arg0: c_double) -> i32;
  // proto: void QPainterPathStroker::FreeQPainterPathStroker();
  fn _ZN19QPainterPathStrokerD0Ev() -> i32;
  // proto: void QPainterPathStroker::setMiterLimit(qreal length);
  fn _ZN19QPainterPathStroker13setMiterLimitEd(arg0: c_double) -> i32;
  // proto: void QPainterPathStroker::NewQPainterPathStroker(const QPen & pen);
  fn _ZN19QPainterPathStrokerC1ERK4QPen(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QPainterPathStroker::setCurveThreshold(qreal threshold);
  fn _ZN19QPainterPathStroker17setCurveThresholdEd(arg0: c_double) -> i32;
  // proto: QVector<qreal> QPainterPathStroker::dashPattern();
  fn _ZNK19QPainterPathStroker11dashPatternEv() -> i32;
  // proto: double QPainterPathStroker::dashOffset();
  fn _ZNK19QPainterPathStroker10dashOffsetEv() -> i32;
  // proto: void QPainterPathStroker::NewQPainterPathStroker();
  fn _ZN19QPainterPathStrokerC1Ev(qthis: *mut c_void) -> i32;
  // proto: QPainterPath QPainterPathStroker::createStroke(const QPainterPath & path);
  fn _ZNK19QPainterPathStroker12createStrokeERK12QPainterPath(arg0: *const c_void) -> i32;
  // proto: void QPainterPathStroker::setDashOffset(qreal offset);
  fn _ZN19QPainterPathStroker13setDashOffsetEd(arg0: c_double) -> i32;
  // proto: double QPainterPathStroker::width();
  fn _ZNK19QPainterPathStroker5widthEv() -> i32;
  // proto: void QPainterPathStroker::NewQPainterPathStroker(const QPainterPathStroker & );
  fn _ZN19QPainterPathStrokerC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: double QPainterPathStroker::miterLimit();
  fn _ZNK19QPainterPathStroker10miterLimitEv() -> i32;
}

// body block begin
// class sizeof(QPainterPathStroker)=1
pub struct QPainterPathStroker {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPainterPathStroker {
  pub fn curveThreshold<T: QPainterPathStroker_curveThreshold>(&mut self, value: T) -> i32 {
    value.curveThreshold(self);
    return 1;
  }
}

pub trait QPainterPathStroker_curveThreshold {
  fn curveThreshold(self, this: &mut QPainterPathStroker) -> i32;
}

// proto: double QPainterPathStroker::curveThreshold();
impl<'a> /*trait*/ QPainterPathStroker_curveThreshold for () {
  fn curveThreshold(self, this: &mut QPainterPathStroker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QPainterPathStroker14curveThresholdEv()};
    unsafe {_ZNK19QPainterPathStroker14curveThresholdEv()};
    return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn setWidth<T: QPainterPathStroker_setWidth>(&mut self, value: T) -> i32 {
    value.setWidth(self);
    return 1;
  }
}

pub trait QPainterPathStroker_setWidth {
  fn setWidth(self, this: &mut QPainterPathStroker) -> i32;
}

// proto: void QPainterPathStroker::setWidth(qreal width);
impl<'a> /*trait*/ QPainterPathStroker_setWidth for (f64) {
  fn setWidth(self, this: &mut QPainterPathStroker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStroker8setWidthEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN19QPainterPathStroker8setWidthEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn FreeQPainterPathStroker<T: QPainterPathStroker_FreeQPainterPathStroker>(&mut self, value: T) -> i32 {
    value.FreeQPainterPathStroker(self);
    return 1;
  }
}

pub trait QPainterPathStroker_FreeQPainterPathStroker {
  fn FreeQPainterPathStroker(self, this: &mut QPainterPathStroker) -> i32;
}

// proto: void QPainterPathStroker::FreeQPainterPathStroker();
impl<'a> /*trait*/ QPainterPathStroker_FreeQPainterPathStroker for () {
  fn FreeQPainterPathStroker(self, this: &mut QPainterPathStroker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStrokerD0Ev()};
    unsafe {_ZN19QPainterPathStrokerD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn setMiterLimit<T: QPainterPathStroker_setMiterLimit>(&mut self, value: T) -> i32 {
    value.setMiterLimit(self);
    return 1;
  }
}

pub trait QPainterPathStroker_setMiterLimit {
  fn setMiterLimit(self, this: &mut QPainterPathStroker) -> i32;
}

// proto: void QPainterPathStroker::setMiterLimit(qreal length);
impl<'a> /*trait*/ QPainterPathStroker_setMiterLimit for (f64) {
  fn setMiterLimit(self, this: &mut QPainterPathStroker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStroker13setMiterLimitEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN19QPainterPathStroker13setMiterLimitEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn NewQPainterPathStroker<T: QPainterPathStroker_NewQPainterPathStroker>(value: T) -> QPainterPathStroker {
    let rsthis = value.NewQPainterPathStroker();
    return rsthis;
    // return 1;
  }
}

pub trait QPainterPathStroker_NewQPainterPathStroker {
  fn NewQPainterPathStroker(self) -> QPainterPathStroker;
}

// proto: void QPainterPathStroker::NewQPainterPathStroker(const QPen & pen);
impl<'a> /*trait*/ QPainterPathStroker_NewQPainterPathStroker for (&'a  QPen) {
  fn NewQPainterPathStroker(self) -> QPainterPathStroker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStrokerC1ERK4QPen()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QPainterPathStrokerC1ERK4QPen(qthis, arg0)};
    let rsthis = QPainterPathStroker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn setCurveThreshold<T: QPainterPathStroker_setCurveThreshold>(&mut self, value: T) -> i32 {
    value.setCurveThreshold(self);
    return 1;
  }
}

pub trait QPainterPathStroker_setCurveThreshold {
  fn setCurveThreshold(self, this: &mut QPainterPathStroker) -> i32;
}

// proto: void QPainterPathStroker::setCurveThreshold(qreal threshold);
impl<'a> /*trait*/ QPainterPathStroker_setCurveThreshold for (f64) {
  fn setCurveThreshold(self, this: &mut QPainterPathStroker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStroker17setCurveThresholdEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN19QPainterPathStroker17setCurveThresholdEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn dashPattern<T: QPainterPathStroker_dashPattern>(&mut self, value: T) -> i32 {
    value.dashPattern(self);
    return 1;
  }
}

pub trait QPainterPathStroker_dashPattern {
  fn dashPattern(self, this: &mut QPainterPathStroker) -> i32;
}

// proto: QVector<qreal> QPainterPathStroker::dashPattern();
impl<'a> /*trait*/ QPainterPathStroker_dashPattern for () {
  fn dashPattern(self, this: &mut QPainterPathStroker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QPainterPathStroker11dashPatternEv()};
    unsafe {_ZNK19QPainterPathStroker11dashPatternEv()};
    return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn dashOffset<T: QPainterPathStroker_dashOffset>(&mut self, value: T) -> i32 {
    value.dashOffset(self);
    return 1;
  }
}

pub trait QPainterPathStroker_dashOffset {
  fn dashOffset(self, this: &mut QPainterPathStroker) -> i32;
}

// proto: double QPainterPathStroker::dashOffset();
impl<'a> /*trait*/ QPainterPathStroker_dashOffset for () {
  fn dashOffset(self, this: &mut QPainterPathStroker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QPainterPathStroker10dashOffsetEv()};
    unsafe {_ZNK19QPainterPathStroker10dashOffsetEv()};
    return 1;
  }
}

// proto: void QPainterPathStroker::NewQPainterPathStroker();
impl<'a> /*trait*/ QPainterPathStroker_NewQPainterPathStroker for () {
  fn NewQPainterPathStroker(self) -> QPainterPathStroker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStrokerC1Ev()};
    unsafe {_ZN19QPainterPathStrokerC1Ev(qthis)};
    let rsthis = QPainterPathStroker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn createStroke<T: QPainterPathStroker_createStroke>(&mut self, value: T) -> i32 {
    value.createStroke(self);
    return 1;
  }
}

pub trait QPainterPathStroker_createStroke {
  fn createStroke(self, this: &mut QPainterPathStroker) -> i32;
}

// proto: QPainterPath QPainterPathStroker::createStroke(const QPainterPath & path);
impl<'a> /*trait*/ QPainterPathStroker_createStroke for (&'a  QPainterPath) {
  fn createStroke(self, this: &mut QPainterPathStroker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QPainterPathStroker12createStrokeERK12QPainterPath()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK19QPainterPathStroker12createStrokeERK12QPainterPath(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn setDashOffset<T: QPainterPathStroker_setDashOffset>(&mut self, value: T) -> i32 {
    value.setDashOffset(self);
    return 1;
  }
}

pub trait QPainterPathStroker_setDashOffset {
  fn setDashOffset(self, this: &mut QPainterPathStroker) -> i32;
}

// proto: void QPainterPathStroker::setDashOffset(qreal offset);
impl<'a> /*trait*/ QPainterPathStroker_setDashOffset for (f64) {
  fn setDashOffset(self, this: &mut QPainterPathStroker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStroker13setDashOffsetEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN19QPainterPathStroker13setDashOffsetEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn width<T: QPainterPathStroker_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QPainterPathStroker_width {
  fn width(self, this: &mut QPainterPathStroker) -> i32;
}

// proto: double QPainterPathStroker::width();
impl<'a> /*trait*/ QPainterPathStroker_width for () {
  fn width(self, this: &mut QPainterPathStroker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QPainterPathStroker5widthEv()};
    unsafe {_ZNK19QPainterPathStroker5widthEv()};
    return 1;
  }
}

// proto: void QPainterPathStroker::NewQPainterPathStroker(const QPainterPathStroker & );
impl<'a> /*trait*/ QPainterPathStroker_NewQPainterPathStroker for (&'a  QPainterPathStroker) {
  fn NewQPainterPathStroker(self) -> QPainterPathStroker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStrokerC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QPainterPathStrokerC1ERKS_(qthis, arg0)};
    let rsthis = QPainterPathStroker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn miterLimit<T: QPainterPathStroker_miterLimit>(&mut self, value: T) -> i32 {
    value.miterLimit(self);
    return 1;
  }
}

pub trait QPainterPathStroker_miterLimit {
  fn miterLimit(self, this: &mut QPainterPathStroker) -> i32;
}

// proto: double QPainterPathStroker::miterLimit();
impl<'a> /*trait*/ QPainterPathStroker_miterLimit for () {
  fn miterLimit(self, this: &mut QPainterPathStroker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QPainterPathStroker10miterLimitEv()};
    unsafe {_ZNK19QPainterPathStroker10miterLimitEv()};
    return 1;
  }
}

