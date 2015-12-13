// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK5QLine6isNullEv() -> i32;
  fn _ZNK5QLine10translatedERK6QPoint(arg0: *const c_void) -> i32;
  fn _ZN5QLine5setP2ERK6QPoint(arg0: *const c_void) -> i32;
  fn _ZNK5QLine2x2Ev() -> i32;
  fn _ZN5QLineC1ERK6QPointS2_(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN5QLine5setP1ERK6QPoint(arg0: *const c_void) -> i32;
  fn _ZN5QLine9translateERK6QPoint(arg0: *const c_void) -> i32;
  fn _ZNK5QLine2dxEv() -> i32;
  fn _ZNK5QLine2y2Ev() -> i32;
  fn _ZNK5QLine2dyEv() -> i32;
  fn _ZNK5QLine2y1Ev() -> i32;
  fn _ZNK5QLine2p1Ev() -> i32;
  fn _ZNK5QLine2p2Ev() -> i32;
  fn _ZN5QLineC1Eiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  fn _ZN5QLine9translateEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZNK5QLine10translatedEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZN5QLine9setPointsERK6QPointS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN5QLine7setLineEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  fn _ZNK5QLine2x1Ev() -> i32;
  fn _ZN5QLineC1Ev(qthis: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QLine)=16
pub struct QLine {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLine {
  pub fn isNull<T: QLine_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QLine_isNull {
  fn isNull(self, this: &mut QLine) -> i32;
}

// proto: bool QLine::isNull();
impl<'a> /*trait*/ QLine_isNull for () {
  fn isNull(self, this: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine6isNullEv()};
    unsafe {_ZNK5QLine6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QLine {
  pub fn translated<T: QLine_translated>(&mut self, value: T) -> i32 {
    value.translated(self);
    return 1;
  }
}

pub trait QLine_translated {
  fn translated(self, this: &mut QLine) -> i32;
}

// proto: QLine QLine::translated(const QPoint & p);
impl<'a> /*trait*/ QLine_translated for (&'a  QPoint) {
  fn translated(self, this: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine10translatedERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK5QLine10translatedERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QLine {
  pub fn setP2<T: QLine_setP2>(&mut self, value: T) -> i32 {
    value.setP2(self);
    return 1;
  }
}

pub trait QLine_setP2 {
  fn setP2(self, this: &mut QLine) -> i32;
}

// proto: void QLine::setP2(const QPoint & p2);
impl<'a> /*trait*/ QLine_setP2 for (&'a  QPoint) {
  fn setP2(self, this: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine5setP2ERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QLine5setP2ERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QLine {
  pub fn x2<T: QLine_x2>(&mut self, value: T) -> i32 {
    value.x2(self);
    return 1;
  }
}

pub trait QLine_x2 {
  fn x2(self, this: &mut QLine) -> i32;
}

// proto: int QLine::x2();
impl<'a> /*trait*/ QLine_x2 for () {
  fn x2(self, this: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2x2Ev()};
    unsafe {_ZNK5QLine2x2Ev()};
    return 1;
  }
}

impl /*struct*/ QLine {
  pub fn NewQLine<T: QLine_NewQLine>(value: T) -> QLine {
    let rsthis = value.NewQLine();
    return rsthis;
    // return 1;
  }
}

pub trait QLine_NewQLine {
  fn NewQLine(self) -> QLine;
}

// proto: void QLine::NewQLine(const QPoint & pt1, const QPoint & pt2);
impl<'a> /*trait*/ QLine_NewQLine for (&'a  QPoint, &'a  QPoint) {
  fn NewQLine(self) -> QLine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLineC1ERK6QPointS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QLineC1ERK6QPointS2_(qthis, arg0, arg1)};
    let rsthis = QLine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLine {
  pub fn setP1<T: QLine_setP1>(&mut self, value: T) -> i32 {
    value.setP1(self);
    return 1;
  }
}

pub trait QLine_setP1 {
  fn setP1(self, this: &mut QLine) -> i32;
}

// proto: void QLine::setP1(const QPoint & p1);
impl<'a> /*trait*/ QLine_setP1 for (&'a  QPoint) {
  fn setP1(self, this: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine5setP1ERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QLine5setP1ERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QLine {
  pub fn translate<T: QLine_translate>(&mut self, value: T) -> i32 {
    value.translate(self);
    return 1;
  }
}

pub trait QLine_translate {
  fn translate(self, this: &mut QLine) -> i32;
}

// proto: void QLine::translate(const QPoint & p);
impl<'a> /*trait*/ QLine_translate for (&'a  QPoint) {
  fn translate(self, this: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine9translateERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QLine9translateERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QLine {
  pub fn dx<T: QLine_dx>(&mut self, value: T) -> i32 {
    value.dx(self);
    return 1;
  }
}

pub trait QLine_dx {
  fn dx(self, this: &mut QLine) -> i32;
}

// proto: int QLine::dx();
impl<'a> /*trait*/ QLine_dx for () {
  fn dx(self, this: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2dxEv()};
    unsafe {_ZNK5QLine2dxEv()};
    return 1;
  }
}

impl /*struct*/ QLine {
  pub fn y2<T: QLine_y2>(&mut self, value: T) -> i32 {
    value.y2(self);
    return 1;
  }
}

pub trait QLine_y2 {
  fn y2(self, this: &mut QLine) -> i32;
}

// proto: int QLine::y2();
impl<'a> /*trait*/ QLine_y2 for () {
  fn y2(self, this: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2y2Ev()};
    unsafe {_ZNK5QLine2y2Ev()};
    return 1;
  }
}

impl /*struct*/ QLine {
  pub fn dy<T: QLine_dy>(&mut self, value: T) -> i32 {
    value.dy(self);
    return 1;
  }
}

pub trait QLine_dy {
  fn dy(self, this: &mut QLine) -> i32;
}

// proto: int QLine::dy();
impl<'a> /*trait*/ QLine_dy for () {
  fn dy(self, this: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2dyEv()};
    unsafe {_ZNK5QLine2dyEv()};
    return 1;
  }
}

impl /*struct*/ QLine {
  pub fn y1<T: QLine_y1>(&mut self, value: T) -> i32 {
    value.y1(self);
    return 1;
  }
}

pub trait QLine_y1 {
  fn y1(self, this: &mut QLine) -> i32;
}

// proto: int QLine::y1();
impl<'a> /*trait*/ QLine_y1 for () {
  fn y1(self, this: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2y1Ev()};
    unsafe {_ZNK5QLine2y1Ev()};
    return 1;
  }
}

impl /*struct*/ QLine {
  pub fn p1<T: QLine_p1>(&mut self, value: T) -> i32 {
    value.p1(self);
    return 1;
  }
}

pub trait QLine_p1 {
  fn p1(self, this: &mut QLine) -> i32;
}

// proto: QPoint QLine::p1();
impl<'a> /*trait*/ QLine_p1 for () {
  fn p1(self, this: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2p1Ev()};
    unsafe {_ZNK5QLine2p1Ev()};
    return 1;
  }
}

impl /*struct*/ QLine {
  pub fn p2<T: QLine_p2>(&mut self, value: T) -> i32 {
    value.p2(self);
    return 1;
  }
}

pub trait QLine_p2 {
  fn p2(self, this: &mut QLine) -> i32;
}

// proto: QPoint QLine::p2();
impl<'a> /*trait*/ QLine_p2 for () {
  fn p2(self, this: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2p2Ev()};
    unsafe {_ZNK5QLine2p2Ev()};
    return 1;
  }
}

// proto: void QLine::NewQLine(int x1, int y1, int x2, int y2);
impl<'a> /*trait*/ QLine_NewQLine for (i32, i32, i32, i32) {
  fn NewQLine(self) -> QLine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLineC1Eiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN5QLineC1Eiiii(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QLine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QLine::translate(int dx, int dy);
impl<'a> /*trait*/ QLine_translate for (i32, i32) {
  fn translate(self, this: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine9translateEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN5QLine9translateEii(arg0, arg1)};
    return 1;
  }
}

// proto: QLine QLine::translated(int dx, int dy);
impl<'a> /*trait*/ QLine_translated for (i32, i32) {
  fn translated(self, this: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine10translatedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK5QLine10translatedEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLine {
  pub fn setPoints<T: QLine_setPoints>(&mut self, value: T) -> i32 {
    value.setPoints(self);
    return 1;
  }
}

pub trait QLine_setPoints {
  fn setPoints(self, this: &mut QLine) -> i32;
}

// proto: void QLine::setPoints(const QPoint & p1, const QPoint & p2);
impl<'a> /*trait*/ QLine_setPoints for (&'a  QPoint, &'a  QPoint) {
  fn setPoints(self, this: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine9setPointsERK6QPointS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QLine9setPointsERK6QPointS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLine {
  pub fn setLine<T: QLine_setLine>(&mut self, value: T) -> i32 {
    value.setLine(self);
    return 1;
  }
}

pub trait QLine_setLine {
  fn setLine(self, this: &mut QLine) -> i32;
}

// proto: void QLine::setLine(int x1, int y1, int x2, int y2);
impl<'a> /*trait*/ QLine_setLine for (i32, i32, i32, i32) {
  fn setLine(self, this: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine7setLineEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN5QLine7setLineEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QLine {
  pub fn x1<T: QLine_x1>(&mut self, value: T) -> i32 {
    value.x1(self);
    return 1;
  }
}

pub trait QLine_x1 {
  fn x1(self, this: &mut QLine) -> i32;
}

// proto: int QLine::x1();
impl<'a> /*trait*/ QLine_x1 for () {
  fn x1(self, this: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2x1Ev()};
    unsafe {_ZNK5QLine2x1Ev()};
    return 1;
  }
}

// proto: void QLine::NewQLine();
impl<'a> /*trait*/ QLine_NewQLine for () {
  fn NewQLine(self) -> QLine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLineC1Ev()};
    unsafe {_ZN5QLineC1Ev(qthis)};
    let rsthis = QLine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

