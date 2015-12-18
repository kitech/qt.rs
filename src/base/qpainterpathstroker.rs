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
  // proto:  double QPainterPathStroker::curveThreshold();
  fn _ZNK19QPainterPathStroker14curveThresholdEv(qthis: *mut c_void) -> c_double;
  // proto:  void QPainterPathStroker::setWidth(qreal width);
  fn _ZN19QPainterPathStroker8setWidthEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QPainterPathStroker::FreeQPainterPathStroker();
  fn _ZN19QPainterPathStrokerD0Ev(qthis: *mut c_void) ;
  // proto:  void QPainterPathStroker::setMiterLimit(qreal length);
  fn _ZN19QPainterPathStroker13setMiterLimitEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QPainterPathStroker::NewQPainterPathStroker(const QPen & pen);
  fn _ZN19QPainterPathStrokerC1ERK4QPen(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainterPathStroker::setCurveThreshold(qreal threshold);
  fn _ZN19QPainterPathStroker17setCurveThresholdEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QVector<qreal> QPainterPathStroker::dashPattern();
  fn _ZNK19QPainterPathStroker11dashPatternEv(qthis: *mut c_void) ;
  // proto:  double QPainterPathStroker::dashOffset();
  fn _ZNK19QPainterPathStroker10dashOffsetEv(qthis: *mut c_void) -> c_double;
  // proto:  void QPainterPathStroker::NewQPainterPathStroker();
  fn _ZN19QPainterPathStrokerC1Ev(qthis: *mut c_void) ;
  // proto:  QPainterPath QPainterPathStroker::createStroke(const QPainterPath & path);
  fn _ZNK19QPainterPathStroker12createStrokeERK12QPainterPath(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPathStroker::setDashOffset(qreal offset);
  fn _ZN19QPainterPathStroker13setDashOffsetEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QPainterPathStroker::width();
  fn _ZNK19QPainterPathStroker5widthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QPainterPathStroker::NewQPainterPathStroker(const QPainterPathStroker & );
  fn _ZN19QPainterPathStrokerC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QPainterPathStroker::miterLimit();
  fn _ZNK19QPainterPathStroker10miterLimitEv(qthis: *mut c_void) -> c_double;
}

// body block begin
// class sizeof(QPainterPathStroker)=1
pub struct QPainterPathStroker {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPainterPathStroker {
  pub fn curveThreshold<RetType, T: QPainterPathStroker_curveThreshold<RetType>>(&mut self, value: T) -> RetType {
    return value.curveThreshold(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_curveThreshold<RetType> {
  fn curveThreshold(self, rsthis: &mut QPainterPathStroker) -> RetType;
}

// proto:  double QPainterPathStroker::curveThreshold();
impl<'a> /*trait*/ QPainterPathStroker_curveThreshold<f64> for () {
  fn curveThreshold(self, rsthis: &mut QPainterPathStroker) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QPainterPathStroker14curveThresholdEv()};
    let mut ret = unsafe {_ZNK19QPainterPathStroker14curveThresholdEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn setWidth<RetType, T: QPainterPathStroker_setWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.setWidth(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_setWidth<RetType> {
  fn setWidth(self, rsthis: &mut QPainterPathStroker) -> RetType;
}

// proto:  void QPainterPathStroker::setWidth(qreal width);
impl<'a> /*trait*/ QPainterPathStroker_setWidth<()> for (f64) {
  fn setWidth(self, rsthis: &mut QPainterPathStroker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStroker8setWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QPainterPathStroker8setWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn FreeQPainterPathStroker<RetType, T: QPainterPathStroker_FreeQPainterPathStroker<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQPainterPathStroker(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_FreeQPainterPathStroker<RetType> {
  fn FreeQPainterPathStroker(self, rsthis: &mut QPainterPathStroker) -> RetType;
}

// proto:  void QPainterPathStroker::FreeQPainterPathStroker();
impl<'a> /*trait*/ QPainterPathStroker_FreeQPainterPathStroker<()> for () {
  fn FreeQPainterPathStroker(self, rsthis: &mut QPainterPathStroker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStrokerD0Ev()};
     unsafe {_ZN19QPainterPathStrokerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn setMiterLimit<RetType, T: QPainterPathStroker_setMiterLimit<RetType>>(&mut self, value: T) -> RetType {
    return value.setMiterLimit(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_setMiterLimit<RetType> {
  fn setMiterLimit(self, rsthis: &mut QPainterPathStroker) -> RetType;
}

// proto:  void QPainterPathStroker::setMiterLimit(qreal length);
impl<'a> /*trait*/ QPainterPathStroker_setMiterLimit<()> for (f64) {
  fn setMiterLimit(self, rsthis: &mut QPainterPathStroker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStroker13setMiterLimitEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QPainterPathStroker13setMiterLimitEd(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QPainterPathStrokerC1ERK4QPen(qthis, arg0)};
    let rsthis = QPainterPathStroker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn setCurveThreshold<RetType, T: QPainterPathStroker_setCurveThreshold<RetType>>(&mut self, value: T) -> RetType {
    return value.setCurveThreshold(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_setCurveThreshold<RetType> {
  fn setCurveThreshold(self, rsthis: &mut QPainterPathStroker) -> RetType;
}

// proto:  void QPainterPathStroker::setCurveThreshold(qreal threshold);
impl<'a> /*trait*/ QPainterPathStroker_setCurveThreshold<()> for (f64) {
  fn setCurveThreshold(self, rsthis: &mut QPainterPathStroker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStroker17setCurveThresholdEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QPainterPathStroker17setCurveThresholdEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn dashPattern<RetType, T: QPainterPathStroker_dashPattern<RetType>>(&mut self, value: T) -> RetType {
    return value.dashPattern(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_dashPattern<RetType> {
  fn dashPattern(self, rsthis: &mut QPainterPathStroker) -> RetType;
}

// proto:  QVector<qreal> QPainterPathStroker::dashPattern();
impl<'a> /*trait*/ QPainterPathStroker_dashPattern<()> for () {
  fn dashPattern(self, rsthis: &mut QPainterPathStroker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QPainterPathStroker11dashPatternEv()};
     unsafe {_ZNK19QPainterPathStroker11dashPatternEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn dashOffset<RetType, T: QPainterPathStroker_dashOffset<RetType>>(&mut self, value: T) -> RetType {
    return value.dashOffset(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_dashOffset<RetType> {
  fn dashOffset(self, rsthis: &mut QPainterPathStroker) -> RetType;
}

// proto:  double QPainterPathStroker::dashOffset();
impl<'a> /*trait*/ QPainterPathStroker_dashOffset<f64> for () {
  fn dashOffset(self, rsthis: &mut QPainterPathStroker) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QPainterPathStroker10dashOffsetEv()};
    let mut ret = unsafe {_ZNK19QPainterPathStroker10dashOffsetEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
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
  pub fn createStroke<RetType, T: QPainterPathStroker_createStroke<RetType>>(&mut self, value: T) -> RetType {
    return value.createStroke(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_createStroke<RetType> {
  fn createStroke(self, rsthis: &mut QPainterPathStroker) -> RetType;
}

// proto:  QPainterPath QPainterPathStroker::createStroke(const QPainterPath & path);
impl<'a> /*trait*/ QPainterPathStroker_createStroke<QPainterPath> for (&'a  QPainterPath) {
  fn createStroke(self, rsthis: &mut QPainterPathStroker) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QPainterPathStroker12createStrokeERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QPainterPathStroker12createStrokeERK12QPainterPath(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn setDashOffset<RetType, T: QPainterPathStroker_setDashOffset<RetType>>(&mut self, value: T) -> RetType {
    return value.setDashOffset(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_setDashOffset<RetType> {
  fn setDashOffset(self, rsthis: &mut QPainterPathStroker) -> RetType;
}

// proto:  void QPainterPathStroker::setDashOffset(qreal offset);
impl<'a> /*trait*/ QPainterPathStroker_setDashOffset<()> for (f64) {
  fn setDashOffset(self, rsthis: &mut QPainterPathStroker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStroker13setDashOffsetEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QPainterPathStroker13setDashOffsetEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn width<RetType, T: QPainterPathStroker_width<RetType>>(&mut self, value: T) -> RetType {
    return value.width(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_width<RetType> {
  fn width(self, rsthis: &mut QPainterPathStroker) -> RetType;
}

// proto:  double QPainterPathStroker::width();
impl<'a> /*trait*/ QPainterPathStroker_width<f64> for () {
  fn width(self, rsthis: &mut QPainterPathStroker) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QPainterPathStroker5widthEv()};
    let mut ret = unsafe {_ZNK19QPainterPathStroker5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto: void QPainterPathStroker::NewQPainterPathStroker(const QPainterPathStroker & );
impl<'a> /*trait*/ QPainterPathStroker_NewQPainterPathStroker for (&'a  QPainterPathStroker) {
  fn NewQPainterPathStroker(self) -> QPainterPathStroker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QPainterPathStrokerC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QPainterPathStrokerC1ERKS_(qthis, arg0)};
    let rsthis = QPainterPathStroker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPainterPathStroker {
  pub fn miterLimit<RetType, T: QPainterPathStroker_miterLimit<RetType>>(&mut self, value: T) -> RetType {
    return value.miterLimit(self);
    // return 1;
  }
}

pub trait QPainterPathStroker_miterLimit<RetType> {
  fn miterLimit(self, rsthis: &mut QPainterPathStroker) -> RetType;
}

// proto:  double QPainterPathStroker::miterLimit();
impl<'a> /*trait*/ QPainterPathStroker_miterLimit<f64> for () {
  fn miterLimit(self, rsthis: &mut QPainterPathStroker) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QPainterPathStroker10miterLimitEv()};
    let mut ret = unsafe {_ZNK19QPainterPathStroker10miterLimitEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

