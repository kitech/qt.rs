// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qcolor::QColor;
use super::qbrush::QBrush;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QPen::FreeQPen();
  fn _ZN4QPenD0Ev() -> i32;
  // proto: double QPen::dashOffset();
  fn _ZNK4QPen10dashOffsetEv() -> i32;
  // proto: void QPen::NewQPen(const QColor & color);
  fn _ZN4QPenC1ERK6QColor(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: double QPen::miterLimit();
  fn _ZNK4QPen10miterLimitEv() -> i32;
  // proto: void QPen::setWidthF(qreal width);
  fn _ZN4QPen9setWidthFEd(arg0: c_double) -> i32;
  // proto: void QPen::setBrush(const QBrush & brush);
  fn _ZN4QPen8setBrushERK6QBrush(arg0: *const c_void) -> i32;
  // proto: QColor QPen::color();
  fn _ZNK4QPen5colorEv() -> i32;
  // proto: void QPen::setWidth(int width);
  fn _ZN4QPen8setWidthEi(arg0: c_int) -> i32;
  // proto: double QPen::widthF();
  fn _ZNK4QPen6widthFEv() -> i32;
  // proto: void QPen::setCosmetic(bool cosmetic);
  fn _ZN4QPen11setCosmeticEb(arg0: int8_t) -> i32;
  // proto: bool QPen::isSolid();
  fn _ZNK4QPen7isSolidEv() -> i32;
  // proto: void QPen::setColor(const QColor & color);
  fn _ZN4QPen8setColorERK6QColor(arg0: *const c_void) -> i32;
  // proto: QVector<qreal> QPen::dashPattern();
  fn _ZNK4QPen11dashPatternEv() -> i32;
  // proto: bool QPen::isDetached();
  fn _ZN4QPen10isDetachedEv() -> i32;
  // proto: void QPen::NewQPen(const QPen & pen);
  fn _ZN4QPenC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QPen::setMiterLimit(qreal limit);
  fn _ZN4QPen13setMiterLimitEd(arg0: c_double) -> i32;
  // proto: void QPen::NewQPen();
  fn _ZN4QPenC1Ev(qthis: *mut c_void) -> i32;
  // proto: int QPen::width();
  fn _ZNK4QPen5widthEv() -> i32;
  // proto: void QPen::swap(QPen & other);
  fn _ZN4QPen4swapERS_(arg0: *mut c_void) -> i32;
  // proto: QBrush QPen::brush();
  fn _ZNK4QPen5brushEv() -> i32;
  // proto: bool QPen::isCosmetic();
  fn _ZNK4QPen10isCosmeticEv() -> i32;
  // proto: void QPen::setDashOffset(qreal doffset);
  fn _ZN4QPen13setDashOffsetEd(arg0: c_double) -> i32;
}

// body block begin
// class sizeof(QPen)=8
pub struct QPen {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPen {
  pub fn FreeQPen<T: QPen_FreeQPen>(&mut self, value: T) -> i32 {
    value.FreeQPen(self);
    return 1;
  }
}

pub trait QPen_FreeQPen {
  fn FreeQPen(self, this: &mut QPen) -> i32;
}

// proto: void QPen::FreeQPen();
impl<'a> /*trait*/ QPen_FreeQPen for () {
  fn FreeQPen(self, this: &mut QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPenD0Ev()};
    unsafe {_ZN4QPenD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPen {
  pub fn dashOffset<T: QPen_dashOffset>(&mut self, value: T) -> i32 {
    value.dashOffset(self);
    return 1;
  }
}

pub trait QPen_dashOffset {
  fn dashOffset(self, this: &mut QPen) -> i32;
}

// proto: double QPen::dashOffset();
impl<'a> /*trait*/ QPen_dashOffset for () {
  fn dashOffset(self, this: &mut QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QPen10dashOffsetEv()};
    unsafe {_ZNK4QPen10dashOffsetEv()};
    return 1;
  }
}

impl /*struct*/ QPen {
  pub fn NewQPen<T: QPen_NewQPen>(value: T) -> QPen {
    let rsthis = value.NewQPen();
    return rsthis;
    // return 1;
  }
}

pub trait QPen_NewQPen {
  fn NewQPen(self) -> QPen;
}

// proto: void QPen::NewQPen(const QColor & color);
impl<'a> /*trait*/ QPen_NewQPen for (&'a  QColor) {
  fn NewQPen(self) -> QPen {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPenC1ERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QPenC1ERK6QColor(qthis, arg0)};
    let rsthis = QPen{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPen {
  pub fn miterLimit<T: QPen_miterLimit>(&mut self, value: T) -> i32 {
    value.miterLimit(self);
    return 1;
  }
}

pub trait QPen_miterLimit {
  fn miterLimit(self, this: &mut QPen) -> i32;
}

// proto: double QPen::miterLimit();
impl<'a> /*trait*/ QPen_miterLimit for () {
  fn miterLimit(self, this: &mut QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QPen10miterLimitEv()};
    unsafe {_ZNK4QPen10miterLimitEv()};
    return 1;
  }
}

impl /*struct*/ QPen {
  pub fn setWidthF<T: QPen_setWidthF>(&mut self, value: T) -> i32 {
    value.setWidthF(self);
    return 1;
  }
}

pub trait QPen_setWidthF {
  fn setWidthF(self, this: &mut QPen) -> i32;
}

// proto: void QPen::setWidthF(qreal width);
impl<'a> /*trait*/ QPen_setWidthF for (f64) {
  fn setWidthF(self, this: &mut QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPen9setWidthFEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN4QPen9setWidthFEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QPen {
  pub fn setBrush<T: QPen_setBrush>(&mut self, value: T) -> i32 {
    value.setBrush(self);
    return 1;
  }
}

pub trait QPen_setBrush {
  fn setBrush(self, this: &mut QPen) -> i32;
}

// proto: void QPen::setBrush(const QBrush & brush);
impl<'a> /*trait*/ QPen_setBrush for (&'a  QBrush) {
  fn setBrush(self, this: &mut QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPen8setBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QPen8setBrushERK6QBrush(arg0)};
    return 1;
  }
}

impl /*struct*/ QPen {
  pub fn color<T: QPen_color>(&mut self, value: T) -> i32 {
    value.color(self);
    return 1;
  }
}

pub trait QPen_color {
  fn color(self, this: &mut QPen) -> i32;
}

// proto: QColor QPen::color();
impl<'a> /*trait*/ QPen_color for () {
  fn color(self, this: &mut QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QPen5colorEv()};
    unsafe {_ZNK4QPen5colorEv()};
    return 1;
  }
}

impl /*struct*/ QPen {
  pub fn setWidth<T: QPen_setWidth>(&mut self, value: T) -> i32 {
    value.setWidth(self);
    return 1;
  }
}

pub trait QPen_setWidth {
  fn setWidth(self, this: &mut QPen) -> i32;
}

// proto: void QPen::setWidth(int width);
impl<'a> /*trait*/ QPen_setWidth for (i32) {
  fn setWidth(self, this: &mut QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPen8setWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN4QPen8setWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QPen {
  pub fn widthF<T: QPen_widthF>(&mut self, value: T) -> i32 {
    value.widthF(self);
    return 1;
  }
}

pub trait QPen_widthF {
  fn widthF(self, this: &mut QPen) -> i32;
}

// proto: double QPen::widthF();
impl<'a> /*trait*/ QPen_widthF for () {
  fn widthF(self, this: &mut QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QPen6widthFEv()};
    unsafe {_ZNK4QPen6widthFEv()};
    return 1;
  }
}

impl /*struct*/ QPen {
  pub fn setCosmetic<T: QPen_setCosmetic>(&mut self, value: T) -> i32 {
    value.setCosmetic(self);
    return 1;
  }
}

pub trait QPen_setCosmetic {
  fn setCosmetic(self, this: &mut QPen) -> i32;
}

// proto: void QPen::setCosmetic(bool cosmetic);
impl<'a> /*trait*/ QPen_setCosmetic for (i8) {
  fn setCosmetic(self, this: &mut QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPen11setCosmeticEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN4QPen11setCosmeticEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QPen {
  pub fn isSolid<T: QPen_isSolid>(&mut self, value: T) -> i32 {
    value.isSolid(self);
    return 1;
  }
}

pub trait QPen_isSolid {
  fn isSolid(self, this: &mut QPen) -> i32;
}

// proto: bool QPen::isSolid();
impl<'a> /*trait*/ QPen_isSolid for () {
  fn isSolid(self, this: &mut QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QPen7isSolidEv()};
    unsafe {_ZNK4QPen7isSolidEv()};
    return 1;
  }
}

impl /*struct*/ QPen {
  pub fn setColor<T: QPen_setColor>(&mut self, value: T) -> i32 {
    value.setColor(self);
    return 1;
  }
}

pub trait QPen_setColor {
  fn setColor(self, this: &mut QPen) -> i32;
}

// proto: void QPen::setColor(const QColor & color);
impl<'a> /*trait*/ QPen_setColor for (&'a  QColor) {
  fn setColor(self, this: &mut QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPen8setColorERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QPen8setColorERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QPen {
  pub fn dashPattern<T: QPen_dashPattern>(&mut self, value: T) -> i32 {
    value.dashPattern(self);
    return 1;
  }
}

pub trait QPen_dashPattern {
  fn dashPattern(self, this: &mut QPen) -> i32;
}

// proto: QVector<qreal> QPen::dashPattern();
impl<'a> /*trait*/ QPen_dashPattern for () {
  fn dashPattern(self, this: &mut QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QPen11dashPatternEv()};
    unsafe {_ZNK4QPen11dashPatternEv()};
    return 1;
  }
}

impl /*struct*/ QPen {
  pub fn isDetached<T: QPen_isDetached>(&mut self, value: T) -> i32 {
    value.isDetached(self);
    return 1;
  }
}

pub trait QPen_isDetached {
  fn isDetached(self, this: &mut QPen) -> i32;
}

// proto: bool QPen::isDetached();
impl<'a> /*trait*/ QPen_isDetached for () {
  fn isDetached(self, this: &mut QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPen10isDetachedEv()};
    unsafe {_ZN4QPen10isDetachedEv()};
    return 1;
  }
}

// proto: void QPen::NewQPen(const QPen & pen);
impl<'a> /*trait*/ QPen_NewQPen for (&'a  QPen) {
  fn NewQPen(self) -> QPen {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPenC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QPenC1ERKS_(qthis, arg0)};
    let rsthis = QPen{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPen {
  pub fn setMiterLimit<T: QPen_setMiterLimit>(&mut self, value: T) -> i32 {
    value.setMiterLimit(self);
    return 1;
  }
}

pub trait QPen_setMiterLimit {
  fn setMiterLimit(self, this: &mut QPen) -> i32;
}

// proto: void QPen::setMiterLimit(qreal limit);
impl<'a> /*trait*/ QPen_setMiterLimit for (f64) {
  fn setMiterLimit(self, this: &mut QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPen13setMiterLimitEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN4QPen13setMiterLimitEd(arg0)};
    return 1;
  }
}

// proto: void QPen::NewQPen();
impl<'a> /*trait*/ QPen_NewQPen for () {
  fn NewQPen(self) -> QPen {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPenC1Ev()};
    unsafe {_ZN4QPenC1Ev(qthis)};
    let rsthis = QPen{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPen {
  pub fn width<T: QPen_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QPen_width {
  fn width(self, this: &mut QPen) -> i32;
}

// proto: int QPen::width();
impl<'a> /*trait*/ QPen_width for () {
  fn width(self, this: &mut QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QPen5widthEv()};
    unsafe {_ZNK4QPen5widthEv()};
    return 1;
  }
}

impl /*struct*/ QPen {
  pub fn swap<T: QPen_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QPen_swap {
  fn swap(self, this: &mut QPen) -> i32;
}

// proto: void QPen::swap(QPen & other);
impl<'a> /*trait*/ QPen_swap for (&'a mut QPen) {
  fn swap(self, this: &mut QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPen4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN4QPen4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QPen {
  pub fn brush<T: QPen_brush>(&mut self, value: T) -> i32 {
    value.brush(self);
    return 1;
  }
}

pub trait QPen_brush {
  fn brush(self, this: &mut QPen) -> i32;
}

// proto: QBrush QPen::brush();
impl<'a> /*trait*/ QPen_brush for () {
  fn brush(self, this: &mut QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QPen5brushEv()};
    unsafe {_ZNK4QPen5brushEv()};
    return 1;
  }
}

impl /*struct*/ QPen {
  pub fn isCosmetic<T: QPen_isCosmetic>(&mut self, value: T) -> i32 {
    value.isCosmetic(self);
    return 1;
  }
}

pub trait QPen_isCosmetic {
  fn isCosmetic(self, this: &mut QPen) -> i32;
}

// proto: bool QPen::isCosmetic();
impl<'a> /*trait*/ QPen_isCosmetic for () {
  fn isCosmetic(self, this: &mut QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QPen10isCosmeticEv()};
    unsafe {_ZNK4QPen10isCosmeticEv()};
    return 1;
  }
}

impl /*struct*/ QPen {
  pub fn setDashOffset<T: QPen_setDashOffset>(&mut self, value: T) -> i32 {
    value.setDashOffset(self);
    return 1;
  }
}

pub trait QPen_setDashOffset {
  fn setDashOffset(self, this: &mut QPen) -> i32;
}

// proto: void QPen::setDashOffset(qreal doffset);
impl<'a> /*trait*/ QPen_setDashOffset for (f64) {
  fn setDashOffset(self, this: &mut QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QPen13setDashOffsetEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN4QPen13setDashOffsetEd(arg0)};
    return 1;
  }
}

