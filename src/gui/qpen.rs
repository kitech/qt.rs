// auto generated, do not modify.
// created: Wed Dec 30 23:22:52 2015
// src-file: /QtGui/qpen.h
// dst-file: /src/gui/qpen.rs
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
use super::qcolor::QColor; // 773
use super::qbrush::QBrush; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPen_Class_Size() -> c_int;
  // proto:  void QPen::~QPen();
  fn _ZN4QPenD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  qreal QPen::dashOffset();
  fn _ZNK4QPen10dashOffsetEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QPen::QPen(const QColor & color);
  fn dector_ZN4QPenC1ERK6QColor(arg0: *mut c_void) -> *mut c_void;
  fn _ZN4QPenC1ERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qreal QPen::miterLimit();
  fn _ZNK4QPen10miterLimitEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QPen::setWidthF(qreal width);
  fn _ZN4QPen9setWidthFEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QPen::setBrush(const QBrush & brush);
  fn _ZN4QPen8setBrushERK6QBrush(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QColor QPen::color();
  fn _ZNK4QPen5colorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPen::setWidth(int width);
  fn _ZN4QPen8setWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  qreal QPen::widthF();
  fn _ZNK4QPen6widthFEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QPen::setCosmetic(bool cosmetic);
  fn _ZN4QPen11setCosmeticEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QPen::isSolid();
  fn _ZNK4QPen7isSolidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QPen::setColor(const QColor & color);
  fn _ZN4QPen8setColorERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QVector<qreal> QPen::dashPattern();
  fn _ZNK4QPen11dashPatternEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QPen::isDetached();
  fn _ZN4QPen10isDetachedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QPen::QPen(const QPen & pen);
  fn dector_ZN4QPenC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN4QPenC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPen::setMiterLimit(qreal limit);
  fn _ZN4QPen13setMiterLimitEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QPen::QPen();
  fn dector_ZN4QPenC1Ev() -> *mut c_void;
  fn _ZN4QPenC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QPen::width();
  fn _ZNK4QPen5widthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QPen::swap(QPen & other);
  fn _ZN4QPen4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QBrush QPen::brush();
  fn _ZNK4QPen5brushEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QPen::isCosmetic();
  fn _ZNK4QPen10isCosmeticEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QPen::setDashOffset(qreal doffset);
  fn _ZN4QPen13setDashOffsetEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
} // <= ext block end

// body block begin =>
// class sizeof(QPen)=8
#[derive(Default)]
pub struct QPen {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QPen {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPen {
    return QPen{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QPen::~QPen();
impl /*struct*/ QPen {
  pub fn Free<RetType, T: QPen_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QPen_Free<RetType> {
  fn Free(self , rsthis: & QPen) -> RetType;
}

  // proto:  void QPen::~QPen();
impl<'a> /*trait*/ QPen_Free<()> for () {
  fn Free(self , rsthis: & QPen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPenD0Ev()};
     unsafe {_ZN4QPenD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QPen::dashOffset();
impl /*struct*/ QPen {
  pub fn dashOffset<RetType, T: QPen_dashOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dashOffset(self);
    // return 1;
  }
}

pub trait QPen_dashOffset<RetType> {
  fn dashOffset(self , rsthis: & QPen) -> RetType;
}

  // proto:  qreal QPen::dashOffset();
impl<'a> /*trait*/ QPen_dashOffset<f64> for () {
  fn dashOffset(self , rsthis: & QPen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QPen10dashOffsetEv()};
    let mut ret = unsafe {_ZNK4QPen10dashOffsetEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPen::QPen(const QColor & color);
impl /*struct*/ QPen {
  pub fn New<T: QPen_New>(value: T) -> QPen {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QPen_New {
  fn New(self) -> QPen;
}

  // proto:  void QPen::QPen(const QColor & color);
impl<'a> /*trait*/ QPen_New for (&'a QColor) {
  fn New(self) -> QPen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPenC1ERK6QColor()};
    let ctysz: c_int = unsafe{QPen_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN4QPenC1ERK6QColor(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN4QPenC1ERK6QColor(arg0)} as u64;
    let rsthis = QPen{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QPen::miterLimit();
impl /*struct*/ QPen {
  pub fn miterLimit<RetType, T: QPen_miterLimit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.miterLimit(self);
    // return 1;
  }
}

pub trait QPen_miterLimit<RetType> {
  fn miterLimit(self , rsthis: & QPen) -> RetType;
}

  // proto:  qreal QPen::miterLimit();
impl<'a> /*trait*/ QPen_miterLimit<f64> for () {
  fn miterLimit(self , rsthis: & QPen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QPen10miterLimitEv()};
    let mut ret = unsafe {_ZNK4QPen10miterLimitEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPen::setWidthF(qreal width);
impl /*struct*/ QPen {
  pub fn setWidthF<RetType, T: QPen_setWidthF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWidthF(self);
    // return 1;
  }
}

pub trait QPen_setWidthF<RetType> {
  fn setWidthF(self , rsthis: & QPen) -> RetType;
}

  // proto:  void QPen::setWidthF(qreal width);
impl<'a> /*trait*/ QPen_setWidthF<()> for (f64) {
  fn setWidthF(self , rsthis: & QPen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPen9setWidthFEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN4QPen9setWidthFEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPen::setBrush(const QBrush & brush);
impl /*struct*/ QPen {
  pub fn setBrush<RetType, T: QPen_setBrush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBrush(self);
    // return 1;
  }
}

pub trait QPen_setBrush<RetType> {
  fn setBrush(self , rsthis: & QPen) -> RetType;
}

  // proto:  void QPen::setBrush(const QBrush & brush);
impl<'a> /*trait*/ QPen_setBrush<()> for (&'a QBrush) {
  fn setBrush(self , rsthis: & QPen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPen8setBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QPen8setBrushERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QColor QPen::color();
impl /*struct*/ QPen {
  pub fn color<RetType, T: QPen_color<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.color(self);
    // return 1;
  }
}

pub trait QPen_color<RetType> {
  fn color(self , rsthis: & QPen) -> RetType;
}

  // proto:  QColor QPen::color();
impl<'a> /*trait*/ QPen_color<QColor> for () {
  fn color(self , rsthis: & QPen) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QPen5colorEv()};
    let mut ret = unsafe {_ZNK4QPen5colorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPen::setWidth(int width);
impl /*struct*/ QPen {
  pub fn setWidth<RetType, T: QPen_setWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWidth(self);
    // return 1;
  }
}

pub trait QPen_setWidth<RetType> {
  fn setWidth(self , rsthis: & QPen) -> RetType;
}

  // proto:  void QPen::setWidth(int width);
impl<'a> /*trait*/ QPen_setWidth<()> for (i32) {
  fn setWidth(self , rsthis: & QPen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPen8setWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN4QPen8setWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QPen::widthF();
impl /*struct*/ QPen {
  pub fn widthF<RetType, T: QPen_widthF<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widthF(self);
    // return 1;
  }
}

pub trait QPen_widthF<RetType> {
  fn widthF(self , rsthis: & QPen) -> RetType;
}

  // proto:  qreal QPen::widthF();
impl<'a> /*trait*/ QPen_widthF<f64> for () {
  fn widthF(self , rsthis: & QPen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QPen6widthFEv()};
    let mut ret = unsafe {_ZNK4QPen6widthFEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPen::setCosmetic(bool cosmetic);
impl /*struct*/ QPen {
  pub fn setCosmetic<RetType, T: QPen_setCosmetic<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCosmetic(self);
    // return 1;
  }
}

pub trait QPen_setCosmetic<RetType> {
  fn setCosmetic(self , rsthis: & QPen) -> RetType;
}

  // proto:  void QPen::setCosmetic(bool cosmetic);
impl<'a> /*trait*/ QPen_setCosmetic<()> for (i8) {
  fn setCosmetic(self , rsthis: & QPen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPen11setCosmeticEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN4QPen11setCosmeticEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPen::isSolid();
impl /*struct*/ QPen {
  pub fn isSolid<RetType, T: QPen_isSolid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSolid(self);
    // return 1;
  }
}

pub trait QPen_isSolid<RetType> {
  fn isSolid(self , rsthis: & QPen) -> RetType;
}

  // proto:  bool QPen::isSolid();
impl<'a> /*trait*/ QPen_isSolid<i8> for () {
  fn isSolid(self , rsthis: & QPen) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QPen7isSolidEv()};
    let mut ret = unsafe {_ZNK4QPen7isSolidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPen::setColor(const QColor & color);
impl /*struct*/ QPen {
  pub fn setColor<RetType, T: QPen_setColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColor(self);
    // return 1;
  }
}

pub trait QPen_setColor<RetType> {
  fn setColor(self , rsthis: & QPen) -> RetType;
}

  // proto:  void QPen::setColor(const QColor & color);
impl<'a> /*trait*/ QPen_setColor<()> for (&'a QColor) {
  fn setColor(self , rsthis: & QPen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPen8setColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QPen8setColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVector<qreal> QPen::dashPattern();
impl /*struct*/ QPen {
  pub fn dashPattern<RetType, T: QPen_dashPattern<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dashPattern(self);
    // return 1;
  }
}

pub trait QPen_dashPattern<RetType> {
  fn dashPattern(self , rsthis: & QPen) -> RetType;
}

  // proto:  QVector<qreal> QPen::dashPattern();
impl<'a> /*trait*/ QPen_dashPattern<()> for () {
  fn dashPattern(self , rsthis: & QPen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QPen11dashPatternEv()};
     unsafe {_ZNK4QPen11dashPatternEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QPen::isDetached();
impl /*struct*/ QPen {
  pub fn isDetached<RetType, T: QPen_isDetached<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDetached(self);
    // return 1;
  }
}

pub trait QPen_isDetached<RetType> {
  fn isDetached(self , rsthis: & QPen) -> RetType;
}

  // proto:  bool QPen::isDetached();
impl<'a> /*trait*/ QPen_isDetached<i8> for () {
  fn isDetached(self , rsthis: & QPen) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPen10isDetachedEv()};
    let mut ret = unsafe {_ZN4QPen10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPen::QPen(const QPen & pen);
impl<'a> /*trait*/ QPen_New for (&'a QPen) {
  fn New(self) -> QPen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPenC1ERKS_()};
    let ctysz: c_int = unsafe{QPen_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN4QPenC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN4QPenC1ERKS_(arg0)} as u64;
    let rsthis = QPen{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPen::setMiterLimit(qreal limit);
impl /*struct*/ QPen {
  pub fn setMiterLimit<RetType, T: QPen_setMiterLimit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMiterLimit(self);
    // return 1;
  }
}

pub trait QPen_setMiterLimit<RetType> {
  fn setMiterLimit(self , rsthis: & QPen) -> RetType;
}

  // proto:  void QPen::setMiterLimit(qreal limit);
impl<'a> /*trait*/ QPen_setMiterLimit<()> for (f64) {
  fn setMiterLimit(self , rsthis: & QPen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPen13setMiterLimitEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN4QPen13setMiterLimitEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPen::QPen();
impl<'a> /*trait*/ QPen_New for () {
  fn New(self) -> QPen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPenC1Ev()};
    let ctysz: c_int = unsafe{QPen_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN4QPenC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN4QPenC1Ev()} as u64;
    let rsthis = QPen{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QPen::width();
impl /*struct*/ QPen {
  pub fn width<RetType, T: QPen_width<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QPen_width<RetType> {
  fn width(self , rsthis: & QPen) -> RetType;
}

  // proto:  int QPen::width();
impl<'a> /*trait*/ QPen_width<i32> for () {
  fn width(self , rsthis: & QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QPen5widthEv()};
    let mut ret = unsafe {_ZNK4QPen5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QPen::swap(QPen & other);
impl /*struct*/ QPen {
  pub fn swap<RetType, T: QPen_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QPen_swap<RetType> {
  fn swap(self , rsthis: & QPen) -> RetType;
}

  // proto:  void QPen::swap(QPen & other);
impl<'a> /*trait*/ QPen_swap<()> for (&'a QPen) {
  fn swap(self , rsthis: & QPen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPen4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QPen4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QBrush QPen::brush();
impl /*struct*/ QPen {
  pub fn brush<RetType, T: QPen_brush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.brush(self);
    // return 1;
  }
}

pub trait QPen_brush<RetType> {
  fn brush(self , rsthis: & QPen) -> RetType;
}

  // proto:  QBrush QPen::brush();
impl<'a> /*trait*/ QPen_brush<QBrush> for () {
  fn brush(self , rsthis: & QPen) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QPen5brushEv()};
    let mut ret = unsafe {_ZNK4QPen5brushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPen::isCosmetic();
impl /*struct*/ QPen {
  pub fn isCosmetic<RetType, T: QPen_isCosmetic<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCosmetic(self);
    // return 1;
  }
}

pub trait QPen_isCosmetic<RetType> {
  fn isCosmetic(self , rsthis: & QPen) -> RetType;
}

  // proto:  bool QPen::isCosmetic();
impl<'a> /*trait*/ QPen_isCosmetic<i8> for () {
  fn isCosmetic(self , rsthis: & QPen) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QPen10isCosmeticEv()};
    let mut ret = unsafe {_ZNK4QPen10isCosmeticEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPen::setDashOffset(qreal doffset);
impl /*struct*/ QPen {
  pub fn setDashOffset<RetType, T: QPen_setDashOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDashOffset(self);
    // return 1;
  }
}

pub trait QPen_setDashOffset<RetType> {
  fn setDashOffset(self , rsthis: & QPen) -> RetType;
}

  // proto:  void QPen::setDashOffset(qreal doffset);
impl<'a> /*trait*/ QPen_setDashOffset<()> for (f64) {
  fn setDashOffset(self , rsthis: & QPen) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPen13setDashOffsetEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN4QPen13setDashOffsetEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

