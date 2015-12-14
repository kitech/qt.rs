// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrectf::QRectF;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  double QTextLine::ascent();
  fn _ZNK9QTextLine6ascentEv(qthis: *mut c_void) -> c_double;
  // proto:  double QTextLine::leading();
  fn _ZNK9QTextLine7leadingEv(qthis: *mut c_void) -> c_double;
  // proto:  int QTextLine::textStart();
  fn _ZNK9QTextLine9textStartEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTextLine::leadingIncluded();
  fn _ZNK9QTextLine15leadingIncludedEv(qthis: *mut c_void) -> int8_t;
  // proto:  double QTextLine::x();
  fn _ZNK9QTextLine1xEv(qthis: *mut c_void) -> c_double;
  // proto:  double QTextLine::height();
  fn _ZNK9QTextLine6heightEv(qthis: *mut c_void) -> c_double;
  // proto:  double QTextLine::y();
  fn _ZNK9QTextLine1yEv(qthis: *mut c_void) -> c_double;
  // proto:  double QTextLine::horizontalAdvance();
  fn _ZNK9QTextLine17horizontalAdvanceEv(qthis: *mut c_void) -> c_double;
  // proto:  QRectF QTextLine::naturalTextRect();
  fn _ZNK9QTextLine15naturalTextRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextLine::setNumColumns(int columns, qreal alignmentWidth);
  fn _ZN9QTextLine13setNumColumnsEid(qthis: *mut c_void, arg0: c_int, arg1: c_double) ;
  // proto:  double QTextLine::width();
  fn _ZNK9QTextLine5widthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextLine::setLeadingIncluded(bool included);
  fn _ZN9QTextLine18setLeadingIncludedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextLine::setPosition(const QPointF & pos);
  fn _ZN9QTextLine11setPositionERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTextLine::lineNumber();
  fn _ZNK9QTextLine10lineNumberEv(qthis: *mut c_void) -> c_int;
  // proto:  QRectF QTextLine::rect();
  fn _ZNK9QTextLine4rectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextLine::NewQTextLine();
  fn _ZN9QTextLineC1Ev(qthis: *mut c_void) ;
  // proto:  void QTextLine::setNumColumns(int columns);
  fn _ZN9QTextLine13setNumColumnsEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QTextLine::textLength();
  fn _ZNK9QTextLine10textLengthEv(qthis: *mut c_void) -> c_int;
  // proto:  QPointF QTextLine::position();
  fn _ZNK9QTextLine8positionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QList<QGlyphRun> QTextLine::glyphRuns(int from, int length);
  fn _ZNK9QTextLine9glyphRunsEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  double QTextLine::descent();
  fn _ZNK9QTextLine7descentEv(qthis: *mut c_void) -> c_double;
  // proto:  double QTextLine::naturalTextWidth();
  fn _ZNK9QTextLine16naturalTextWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextLine::setLineWidth(qreal width);
  fn _ZN9QTextLine12setLineWidthEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  bool QTextLine::isValid();
  fn _ZNK9QTextLine7isValidEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QTextLine)=16
pub struct QTextLine {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextLine {
  pub fn ascent<T: QTextLine_ascent>(&mut self, value: T) -> f64 {
    return value.ascent(self);
    // return 1;
  }
}

pub trait QTextLine_ascent {
  fn ascent(self, rsthis: &mut QTextLine) -> f64;
}

// proto:  double QTextLine::ascent();
impl<'a> /*trait*/ QTextLine_ascent for () {
  fn ascent(self, rsthis: &mut QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine6ascentEv()};
    let mut ret = unsafe {_ZNK9QTextLine6ascentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn leading<T: QTextLine_leading>(&mut self, value: T) -> f64 {
    return value.leading(self);
    // return 1;
  }
}

pub trait QTextLine_leading {
  fn leading(self, rsthis: &mut QTextLine) -> f64;
}

// proto:  double QTextLine::leading();
impl<'a> /*trait*/ QTextLine_leading for () {
  fn leading(self, rsthis: &mut QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine7leadingEv()};
    let mut ret = unsafe {_ZNK9QTextLine7leadingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn textStart<T: QTextLine_textStart>(&mut self, value: T) -> i32 {
    return value.textStart(self);
    // return 1;
  }
}

pub trait QTextLine_textStart {
  fn textStart(self, rsthis: &mut QTextLine) -> i32;
}

// proto:  int QTextLine::textStart();
impl<'a> /*trait*/ QTextLine_textStart for () {
  fn textStart(self, rsthis: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine9textStartEv()};
    let mut ret = unsafe {_ZNK9QTextLine9textStartEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn leadingIncluded<T: QTextLine_leadingIncluded>(&mut self, value: T) -> i8 {
    return value.leadingIncluded(self);
    // return 1;
  }
}

pub trait QTextLine_leadingIncluded {
  fn leadingIncluded(self, rsthis: &mut QTextLine) -> i8;
}

// proto:  bool QTextLine::leadingIncluded();
impl<'a> /*trait*/ QTextLine_leadingIncluded for () {
  fn leadingIncluded(self, rsthis: &mut QTextLine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine15leadingIncludedEv()};
    let mut ret = unsafe {_ZNK9QTextLine15leadingIncludedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn x<T: QTextLine_x>(&mut self, value: T) -> f64 {
    return value.x(self);
    // return 1;
  }
}

pub trait QTextLine_x {
  fn x(self, rsthis: &mut QTextLine) -> f64;
}

// proto:  double QTextLine::x();
impl<'a> /*trait*/ QTextLine_x for () {
  fn x(self, rsthis: &mut QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine1xEv()};
    let mut ret = unsafe {_ZNK9QTextLine1xEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn height<T: QTextLine_height>(&mut self, value: T) -> f64 {
    return value.height(self);
    // return 1;
  }
}

pub trait QTextLine_height {
  fn height(self, rsthis: &mut QTextLine) -> f64;
}

// proto:  double QTextLine::height();
impl<'a> /*trait*/ QTextLine_height for () {
  fn height(self, rsthis: &mut QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine6heightEv()};
    let mut ret = unsafe {_ZNK9QTextLine6heightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn y<T: QTextLine_y>(&mut self, value: T) -> f64 {
    return value.y(self);
    // return 1;
  }
}

pub trait QTextLine_y {
  fn y(self, rsthis: &mut QTextLine) -> f64;
}

// proto:  double QTextLine::y();
impl<'a> /*trait*/ QTextLine_y for () {
  fn y(self, rsthis: &mut QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine1yEv()};
    let mut ret = unsafe {_ZNK9QTextLine1yEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn horizontalAdvance<T: QTextLine_horizontalAdvance>(&mut self, value: T) -> f64 {
    return value.horizontalAdvance(self);
    // return 1;
  }
}

pub trait QTextLine_horizontalAdvance {
  fn horizontalAdvance(self, rsthis: &mut QTextLine) -> f64;
}

// proto:  double QTextLine::horizontalAdvance();
impl<'a> /*trait*/ QTextLine_horizontalAdvance for () {
  fn horizontalAdvance(self, rsthis: &mut QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine17horizontalAdvanceEv()};
    let mut ret = unsafe {_ZNK9QTextLine17horizontalAdvanceEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn naturalTextRect<T: QTextLine_naturalTextRect>(&mut self, value: T) -> QRectF {
    return value.naturalTextRect(self);
    // return 1;
  }
}

pub trait QTextLine_naturalTextRect {
  fn naturalTextRect(self, rsthis: &mut QTextLine) -> QRectF;
}

// proto:  QRectF QTextLine::naturalTextRect();
impl<'a> /*trait*/ QTextLine_naturalTextRect for () {
  fn naturalTextRect(self, rsthis: &mut QTextLine) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine15naturalTextRectEv()};
    let mut ret = unsafe {_ZNK9QTextLine15naturalTextRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn setNumColumns<T: QTextLine_setNumColumns>(&mut self, value: T)  {
     value.setNumColumns(self);
    // return 1;
  }
}

pub trait QTextLine_setNumColumns {
  fn setNumColumns(self, rsthis: &mut QTextLine) ;
}

// proto:  void QTextLine::setNumColumns(int columns, qreal alignmentWidth);
impl<'a> /*trait*/ QTextLine_setNumColumns for (i32, f64) {
  fn setNumColumns(self, rsthis: &mut QTextLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine13setNumColumnsEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN9QTextLine13setNumColumnsEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn width<T: QTextLine_width>(&mut self, value: T) -> f64 {
    return value.width(self);
    // return 1;
  }
}

pub trait QTextLine_width {
  fn width(self, rsthis: &mut QTextLine) -> f64;
}

// proto:  double QTextLine::width();
impl<'a> /*trait*/ QTextLine_width for () {
  fn width(self, rsthis: &mut QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine5widthEv()};
    let mut ret = unsafe {_ZNK9QTextLine5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn setLeadingIncluded<T: QTextLine_setLeadingIncluded>(&mut self, value: T)  {
     value.setLeadingIncluded(self);
    // return 1;
  }
}

pub trait QTextLine_setLeadingIncluded {
  fn setLeadingIncluded(self, rsthis: &mut QTextLine) ;
}

// proto:  void QTextLine::setLeadingIncluded(bool included);
impl<'a> /*trait*/ QTextLine_setLeadingIncluded for (i8) {
  fn setLeadingIncluded(self, rsthis: &mut QTextLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine18setLeadingIncludedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTextLine18setLeadingIncludedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn setPosition<T: QTextLine_setPosition>(&mut self, value: T)  {
     value.setPosition(self);
    // return 1;
  }
}

pub trait QTextLine_setPosition {
  fn setPosition(self, rsthis: &mut QTextLine) ;
}

// proto:  void QTextLine::setPosition(const QPointF & pos);
impl<'a> /*trait*/ QTextLine_setPosition for (&'a  QPointF) {
  fn setPosition(self, rsthis: &mut QTextLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine11setPositionERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextLine11setPositionERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn lineNumber<T: QTextLine_lineNumber>(&mut self, value: T) -> i32 {
    return value.lineNumber(self);
    // return 1;
  }
}

pub trait QTextLine_lineNumber {
  fn lineNumber(self, rsthis: &mut QTextLine) -> i32;
}

// proto:  int QTextLine::lineNumber();
impl<'a> /*trait*/ QTextLine_lineNumber for () {
  fn lineNumber(self, rsthis: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine10lineNumberEv()};
    let mut ret = unsafe {_ZNK9QTextLine10lineNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn rect<T: QTextLine_rect>(&mut self, value: T) -> QRectF {
    return value.rect(self);
    // return 1;
  }
}

pub trait QTextLine_rect {
  fn rect(self, rsthis: &mut QTextLine) -> QRectF;
}

// proto:  QRectF QTextLine::rect();
impl<'a> /*trait*/ QTextLine_rect for () {
  fn rect(self, rsthis: &mut QTextLine) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine4rectEv()};
    let mut ret = unsafe {_ZNK9QTextLine4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn NewQTextLine<T: QTextLine_NewQTextLine>(value: T) -> QTextLine {
    let rsthis = value.NewQTextLine();
    return rsthis;
    // return 1;
  }
}

pub trait QTextLine_NewQTextLine {
  fn NewQTextLine(self) -> QTextLine;
}

// proto: void QTextLine::NewQTextLine();
impl<'a> /*trait*/ QTextLine_NewQTextLine for () {
  fn NewQTextLine(self) -> QTextLine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLineC1Ev()};
    unsafe {_ZN9QTextLineC1Ev(qthis)};
    let rsthis = QTextLine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QTextLine::setNumColumns(int columns);
impl<'a> /*trait*/ QTextLine_setNumColumns for (i32) {
  fn setNumColumns(self, rsthis: &mut QTextLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine13setNumColumnsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTextLine13setNumColumnsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn textLength<T: QTextLine_textLength>(&mut self, value: T) -> i32 {
    return value.textLength(self);
    // return 1;
  }
}

pub trait QTextLine_textLength {
  fn textLength(self, rsthis: &mut QTextLine) -> i32;
}

// proto:  int QTextLine::textLength();
impl<'a> /*trait*/ QTextLine_textLength for () {
  fn textLength(self, rsthis: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine10textLengthEv()};
    let mut ret = unsafe {_ZNK9QTextLine10textLengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn position<T: QTextLine_position>(&mut self, value: T) -> QPointF {
    return value.position(self);
    // return 1;
  }
}

pub trait QTextLine_position {
  fn position(self, rsthis: &mut QTextLine) -> QPointF;
}

// proto:  QPointF QTextLine::position();
impl<'a> /*trait*/ QTextLine_position for () {
  fn position(self, rsthis: &mut QTextLine) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine8positionEv()};
    let mut ret = unsafe {_ZNK9QTextLine8positionEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn glyphRuns<T: QTextLine_glyphRuns>(&mut self, value: T)  {
     value.glyphRuns(self);
    // return 1;
  }
}

pub trait QTextLine_glyphRuns {
  fn glyphRuns(self, rsthis: &mut QTextLine) ;
}

// proto:  QList<QGlyphRun> QTextLine::glyphRuns(int from, int length);
impl<'a> /*trait*/ QTextLine_glyphRuns for (i32, i32) {
  fn glyphRuns(self, rsthis: &mut QTextLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine9glyphRunsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZNK9QTextLine9glyphRunsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn descent<T: QTextLine_descent>(&mut self, value: T) -> f64 {
    return value.descent(self);
    // return 1;
  }
}

pub trait QTextLine_descent {
  fn descent(self, rsthis: &mut QTextLine) -> f64;
}

// proto:  double QTextLine::descent();
impl<'a> /*trait*/ QTextLine_descent for () {
  fn descent(self, rsthis: &mut QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine7descentEv()};
    let mut ret = unsafe {_ZNK9QTextLine7descentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn naturalTextWidth<T: QTextLine_naturalTextWidth>(&mut self, value: T) -> f64 {
    return value.naturalTextWidth(self);
    // return 1;
  }
}

pub trait QTextLine_naturalTextWidth {
  fn naturalTextWidth(self, rsthis: &mut QTextLine) -> f64;
}

// proto:  double QTextLine::naturalTextWidth();
impl<'a> /*trait*/ QTextLine_naturalTextWidth for () {
  fn naturalTextWidth(self, rsthis: &mut QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine16naturalTextWidthEv()};
    let mut ret = unsafe {_ZNK9QTextLine16naturalTextWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn setLineWidth<T: QTextLine_setLineWidth>(&mut self, value: T)  {
     value.setLineWidth(self);
    // return 1;
  }
}

pub trait QTextLine_setLineWidth {
  fn setLineWidth(self, rsthis: &mut QTextLine) ;
}

// proto:  void QTextLine::setLineWidth(qreal width);
impl<'a> /*trait*/ QTextLine_setLineWidth for (f64) {
  fn setLineWidth(self, rsthis: &mut QTextLine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine12setLineWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN9QTextLine12setLineWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn isValid<T: QTextLine_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QTextLine_isValid {
  fn isValid(self, rsthis: &mut QTextLine) -> i8;
}

// proto:  bool QTextLine::isValid();
impl<'a> /*trait*/ QTextLine_isValid for () {
  fn isValid(self, rsthis: &mut QTextLine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine7isValidEv()};
    let mut ret = unsafe {_ZNK9QTextLine7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

