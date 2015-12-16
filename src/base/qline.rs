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
  // proto:  bool QLine::isNull();
  fn _ZNK5QLine6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  QLine QLine::translated(const QPoint & p);
  fn _ZNK5QLine10translatedERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QLine::setP2(const QPoint & p2);
  fn _ZN5QLine5setP2ERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QLine::x2();
  fn _ZNK5QLine2x2Ev(qthis: *mut c_void) -> c_int;
  // proto:  void QLine::NewQLine(const QPoint & pt1, const QPoint & pt2);
  fn _ZN5QLineC1ERK6QPointS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QLine::setP1(const QPoint & p1);
  fn _ZN5QLine5setP1ERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QLine::translate(const QPoint & p);
  fn _ZN5QLine9translateERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QLine::dx();
  fn _ZNK5QLine2dxEv(qthis: *mut c_void) -> c_int;
  // proto:  int QLine::y2();
  fn _ZNK5QLine2y2Ev(qthis: *mut c_void) -> c_int;
  // proto:  int QLine::dy();
  fn _ZNK5QLine2dyEv(qthis: *mut c_void) -> c_int;
  // proto:  int QLine::y1();
  fn _ZNK5QLine2y1Ev(qthis: *mut c_void) -> c_int;
  // proto:  QPoint QLine::p1();
  fn _ZNK5QLine2p1Ev(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QLine::p2();
  fn _ZNK5QLine2p2Ev(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLine::NewQLine(int x1, int y1, int x2, int y2);
  fn _ZN5QLineC1Eiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QLine::translate(int dx, int dy);
  fn _ZN5QLine9translateEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QLine QLine::translated(int dx, int dy);
  fn _ZNK5QLine10translatedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QLine::setPoints(const QPoint & p1, const QPoint & p2);
  fn _ZN5QLine9setPointsERK6QPointS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QLine::setLine(int x1, int y1, int x2, int y2);
  fn _ZN5QLine7setLineEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  int QLine::x1();
  fn _ZNK5QLine2x1Ev(qthis: *mut c_void) -> c_int;
  // proto:  void QLine::NewQLine();
  fn _ZN5QLineC1Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QLine)=16
pub struct QLine {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLine {
  pub fn isNull<T: QLine_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QLine_isNull {
  fn isNull(self, rsthis: &mut QLine) -> i8;
}

// proto:  bool QLine::isNull();
impl<'a> /*trait*/ QLine_isNull for () {
  fn isNull(self, rsthis: &mut QLine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine6isNullEv()};
    let mut ret = unsafe {_ZNK5QLine6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLine {
  pub fn translated<T: QLine_translated>(&mut self, value: T) -> QLine {
    return value.translated(self);
    // return 1;
  }
}

pub trait QLine_translated {
  fn translated(self, rsthis: &mut QLine) -> QLine;
}

// proto:  QLine QLine::translated(const QPoint & p);
impl<'a> /*trait*/ QLine_translated for (&'a  QPoint) {
  fn translated(self, rsthis: &mut QLine) -> QLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine10translatedERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QLine10translatedERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QLine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLine {
  pub fn setP2<T: QLine_setP2>(&mut self, value: T)  {
     value.setP2(self);
    // return 1;
  }
}

pub trait QLine_setP2 {
  fn setP2(self, rsthis: &mut QLine) ;
}

// proto:  void QLine::setP2(const QPoint & p2);
impl<'a> /*trait*/ QLine_setP2 for (&'a  QPoint) {
  fn setP2(self, rsthis: &mut QLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine5setP2ERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QLine5setP2ERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLine {
  pub fn x2<T: QLine_x2>(&mut self, value: T) -> i32 {
    return value.x2(self);
    // return 1;
  }
}

pub trait QLine_x2 {
  fn x2(self, rsthis: &mut QLine) -> i32;
}

// proto:  int QLine::x2();
impl<'a> /*trait*/ QLine_x2 for () {
  fn x2(self, rsthis: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2x2Ev()};
    let mut ret = unsafe {_ZNK5QLine2x2Ev(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
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
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN5QLineC1ERK6QPointS2_(qthis, arg0, arg1)};
    let rsthis = QLine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLine {
  pub fn setP1<T: QLine_setP1>(&mut self, value: T)  {
     value.setP1(self);
    // return 1;
  }
}

pub trait QLine_setP1 {
  fn setP1(self, rsthis: &mut QLine) ;
}

// proto:  void QLine::setP1(const QPoint & p1);
impl<'a> /*trait*/ QLine_setP1 for (&'a  QPoint) {
  fn setP1(self, rsthis: &mut QLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine5setP1ERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QLine5setP1ERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLine {
  pub fn translate<T: QLine_translate>(&mut self, value: T)  {
     value.translate(self);
    // return 1;
  }
}

pub trait QLine_translate {
  fn translate(self, rsthis: &mut QLine) ;
}

// proto:  void QLine::translate(const QPoint & p);
impl<'a> /*trait*/ QLine_translate for (&'a  QPoint) {
  fn translate(self, rsthis: &mut QLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine9translateERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QLine9translateERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLine {
  pub fn dx<T: QLine_dx>(&mut self, value: T) -> i32 {
    return value.dx(self);
    // return 1;
  }
}

pub trait QLine_dx {
  fn dx(self, rsthis: &mut QLine) -> i32;
}

// proto:  int QLine::dx();
impl<'a> /*trait*/ QLine_dx for () {
  fn dx(self, rsthis: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2dxEv()};
    let mut ret = unsafe {_ZNK5QLine2dxEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QLine {
  pub fn y2<T: QLine_y2>(&mut self, value: T) -> i32 {
    return value.y2(self);
    // return 1;
  }
}

pub trait QLine_y2 {
  fn y2(self, rsthis: &mut QLine) -> i32;
}

// proto:  int QLine::y2();
impl<'a> /*trait*/ QLine_y2 for () {
  fn y2(self, rsthis: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2y2Ev()};
    let mut ret = unsafe {_ZNK5QLine2y2Ev(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QLine {
  pub fn dy<T: QLine_dy>(&mut self, value: T) -> i32 {
    return value.dy(self);
    // return 1;
  }
}

pub trait QLine_dy {
  fn dy(self, rsthis: &mut QLine) -> i32;
}

// proto:  int QLine::dy();
impl<'a> /*trait*/ QLine_dy for () {
  fn dy(self, rsthis: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2dyEv()};
    let mut ret = unsafe {_ZNK5QLine2dyEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QLine {
  pub fn y1<T: QLine_y1>(&mut self, value: T) -> i32 {
    return value.y1(self);
    // return 1;
  }
}

pub trait QLine_y1 {
  fn y1(self, rsthis: &mut QLine) -> i32;
}

// proto:  int QLine::y1();
impl<'a> /*trait*/ QLine_y1 for () {
  fn y1(self, rsthis: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2y1Ev()};
    let mut ret = unsafe {_ZNK5QLine2y1Ev(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QLine {
  pub fn p1<T: QLine_p1>(&mut self, value: T) -> QPoint {
    return value.p1(self);
    // return 1;
  }
}

pub trait QLine_p1 {
  fn p1(self, rsthis: &mut QLine) -> QPoint;
}

// proto:  QPoint QLine::p1();
impl<'a> /*trait*/ QLine_p1 for () {
  fn p1(self, rsthis: &mut QLine) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2p1Ev()};
    let mut ret = unsafe {_ZNK5QLine2p1Ev(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLine {
  pub fn p2<T: QLine_p2>(&mut self, value: T) -> QPoint {
    return value.p2(self);
    // return 1;
  }
}

pub trait QLine_p2 {
  fn p2(self, rsthis: &mut QLine) -> QPoint;
}

// proto:  QPoint QLine::p2();
impl<'a> /*trait*/ QLine_p2 for () {
  fn p2(self, rsthis: &mut QLine) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2p2Ev()};
    let mut ret = unsafe {_ZNK5QLine2p2Ev(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
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

// proto:  void QLine::translate(int dx, int dy);
impl<'a> /*trait*/ QLine_translate for (i32, i32) {
  fn translate(self, rsthis: &mut QLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine9translateEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN5QLine9translateEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QLine QLine::translated(int dx, int dy);
impl<'a> /*trait*/ QLine_translated for (i32, i32) {
  fn translated(self, rsthis: &mut QLine) -> QLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine10translatedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK5QLine10translatedEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QLine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLine {
  pub fn setPoints<T: QLine_setPoints>(&mut self, value: T)  {
     value.setPoints(self);
    // return 1;
  }
}

pub trait QLine_setPoints {
  fn setPoints(self, rsthis: &mut QLine) ;
}

// proto:  void QLine::setPoints(const QPoint & p1, const QPoint & p2);
impl<'a> /*trait*/ QLine_setPoints for (&'a  QPoint, &'a  QPoint) {
  fn setPoints(self, rsthis: &mut QLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine9setPointsERK6QPointS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN5QLine9setPointsERK6QPointS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QLine {
  pub fn setLine<T: QLine_setLine>(&mut self, value: T)  {
     value.setLine(self);
    // return 1;
  }
}

pub trait QLine_setLine {
  fn setLine(self, rsthis: &mut QLine) ;
}

// proto:  void QLine::setLine(int x1, int y1, int x2, int y2);
impl<'a> /*trait*/ QLine_setLine for (i32, i32, i32, i32) {
  fn setLine(self, rsthis: &mut QLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine7setLineEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN5QLine7setLineEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QLine {
  pub fn x1<T: QLine_x1>(&mut self, value: T) -> i32 {
    return value.x1(self);
    // return 1;
  }
}

pub trait QLine_x1 {
  fn x1(self, rsthis: &mut QLine) -> i32;
}

// proto:  int QLine::x1();
impl<'a> /*trait*/ QLine_x1 for () {
  fn x1(self, rsthis: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2x1Ev()};
    let mut ret = unsafe {_ZNK5QLine2x1Ev(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
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

