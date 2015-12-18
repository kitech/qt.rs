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
  fn _ZNK9QTextLine1xEv(qthis: *mut c_void) ;
  // proto:  double QTextLine::height();
  fn _ZNK9QTextLine6heightEv(qthis: *mut c_void) -> c_double;
  // proto:  double QTextLine::y();
  fn _ZNK9QTextLine1yEv(qthis: *mut c_void) ;
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
  pub fn ascent<RetType, T: QTextLine_ascent<RetType>>(&mut self, value: T) -> RetType {
    return value.ascent(self);
    // return 1;
  }
}

pub trait QTextLine_ascent<RetType> {
  fn ascent(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  double QTextLine::ascent();
impl<'a> /*trait*/ QTextLine_ascent<f64> for () {
  fn ascent(self, rsthis: &mut QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine6ascentEv()};
    let mut ret = unsafe {_ZNK9QTextLine6ascentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn leading<RetType, T: QTextLine_leading<RetType>>(&mut self, value: T) -> RetType {
    return value.leading(self);
    // return 1;
  }
}

pub trait QTextLine_leading<RetType> {
  fn leading(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  double QTextLine::leading();
impl<'a> /*trait*/ QTextLine_leading<f64> for () {
  fn leading(self, rsthis: &mut QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine7leadingEv()};
    let mut ret = unsafe {_ZNK9QTextLine7leadingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn textStart<RetType, T: QTextLine_textStart<RetType>>(&mut self, value: T) -> RetType {
    return value.textStart(self);
    // return 1;
  }
}

pub trait QTextLine_textStart<RetType> {
  fn textStart(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  int QTextLine::textStart();
impl<'a> /*trait*/ QTextLine_textStart<i32> for () {
  fn textStart(self, rsthis: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine9textStartEv()};
    let mut ret = unsafe {_ZNK9QTextLine9textStartEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn leadingIncluded<RetType, T: QTextLine_leadingIncluded<RetType>>(&mut self, value: T) -> RetType {
    return value.leadingIncluded(self);
    // return 1;
  }
}

pub trait QTextLine_leadingIncluded<RetType> {
  fn leadingIncluded(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  bool QTextLine::leadingIncluded();
impl<'a> /*trait*/ QTextLine_leadingIncluded<i8> for () {
  fn leadingIncluded(self, rsthis: &mut QTextLine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine15leadingIncludedEv()};
    let mut ret = unsafe {_ZNK9QTextLine15leadingIncludedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn x<RetType, T: QTextLine_x<RetType>>(&mut self, value: T) -> RetType {
    return value.x(self);
    // return 1;
  }
}

pub trait QTextLine_x<RetType> {
  fn x(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  double QTextLine::x();
impl<'a> /*trait*/ QTextLine_x<()> for () {
  fn x(self, rsthis: &mut QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine1xEv()};
     unsafe {_ZNK9QTextLine1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn height<RetType, T: QTextLine_height<RetType>>(&mut self, value: T) -> RetType {
    return value.height(self);
    // return 1;
  }
}

pub trait QTextLine_height<RetType> {
  fn height(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  double QTextLine::height();
impl<'a> /*trait*/ QTextLine_height<f64> for () {
  fn height(self, rsthis: &mut QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine6heightEv()};
    let mut ret = unsafe {_ZNK9QTextLine6heightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn y<RetType, T: QTextLine_y<RetType>>(&mut self, value: T) -> RetType {
    return value.y(self);
    // return 1;
  }
}

pub trait QTextLine_y<RetType> {
  fn y(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  double QTextLine::y();
impl<'a> /*trait*/ QTextLine_y<()> for () {
  fn y(self, rsthis: &mut QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine1yEv()};
     unsafe {_ZNK9QTextLine1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn horizontalAdvance<RetType, T: QTextLine_horizontalAdvance<RetType>>(&mut self, value: T) -> RetType {
    return value.horizontalAdvance(self);
    // return 1;
  }
}

pub trait QTextLine_horizontalAdvance<RetType> {
  fn horizontalAdvance(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  double QTextLine::horizontalAdvance();
impl<'a> /*trait*/ QTextLine_horizontalAdvance<f64> for () {
  fn horizontalAdvance(self, rsthis: &mut QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine17horizontalAdvanceEv()};
    let mut ret = unsafe {_ZNK9QTextLine17horizontalAdvanceEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn naturalTextRect<RetType, T: QTextLine_naturalTextRect<RetType>>(&mut self, value: T) -> RetType {
    return value.naturalTextRect(self);
    // return 1;
  }
}

pub trait QTextLine_naturalTextRect<RetType> {
  fn naturalTextRect(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  QRectF QTextLine::naturalTextRect();
impl<'a> /*trait*/ QTextLine_naturalTextRect<QRectF> for () {
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
  pub fn setNumColumns<RetType, T: QTextLine_setNumColumns<RetType>>(&mut self, value: T) -> RetType {
    return value.setNumColumns(self);
    // return 1;
  }
}

pub trait QTextLine_setNumColumns<RetType> {
  fn setNumColumns(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  void QTextLine::setNumColumns(int columns, qreal alignmentWidth);
impl<'a> /*trait*/ QTextLine_setNumColumns<()> for (i32, f64) {
  fn setNumColumns(self, rsthis: &mut QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine13setNumColumnsEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN9QTextLine13setNumColumnsEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn width<RetType, T: QTextLine_width<RetType>>(&mut self, value: T) -> RetType {
    return value.width(self);
    // return 1;
  }
}

pub trait QTextLine_width<RetType> {
  fn width(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  double QTextLine::width();
impl<'a> /*trait*/ QTextLine_width<f64> for () {
  fn width(self, rsthis: &mut QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine5widthEv()};
    let mut ret = unsafe {_ZNK9QTextLine5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn setLeadingIncluded<RetType, T: QTextLine_setLeadingIncluded<RetType>>(&mut self, value: T) -> RetType {
    return value.setLeadingIncluded(self);
    // return 1;
  }
}

pub trait QTextLine_setLeadingIncluded<RetType> {
  fn setLeadingIncluded(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  void QTextLine::setLeadingIncluded(bool included);
impl<'a> /*trait*/ QTextLine_setLeadingIncluded<()> for (i8) {
  fn setLeadingIncluded(self, rsthis: &mut QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine18setLeadingIncludedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QTextLine18setLeadingIncludedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn setPosition<RetType, T: QTextLine_setPosition<RetType>>(&mut self, value: T) -> RetType {
    return value.setPosition(self);
    // return 1;
  }
}

pub trait QTextLine_setPosition<RetType> {
  fn setPosition(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  void QTextLine::setPosition(const QPointF & pos);
impl<'a> /*trait*/ QTextLine_setPosition<()> for (&'a  QPointF) {
  fn setPosition(self, rsthis: &mut QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine11setPositionERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextLine11setPositionERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn lineNumber<RetType, T: QTextLine_lineNumber<RetType>>(&mut self, value: T) -> RetType {
    return value.lineNumber(self);
    // return 1;
  }
}

pub trait QTextLine_lineNumber<RetType> {
  fn lineNumber(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  int QTextLine::lineNumber();
impl<'a> /*trait*/ QTextLine_lineNumber<i32> for () {
  fn lineNumber(self, rsthis: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine10lineNumberEv()};
    let mut ret = unsafe {_ZNK9QTextLine10lineNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn rect<RetType, T: QTextLine_rect<RetType>>(&mut self, value: T) -> RetType {
    return value.rect(self);
    // return 1;
  }
}

pub trait QTextLine_rect<RetType> {
  fn rect(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  QRectF QTextLine::rect();
impl<'a> /*trait*/ QTextLine_rect<QRectF> for () {
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
impl<'a> /*trait*/ QTextLine_setNumColumns<()> for (i32) {
  fn setNumColumns(self, rsthis: &mut QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine13setNumColumnsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTextLine13setNumColumnsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn textLength<RetType, T: QTextLine_textLength<RetType>>(&mut self, value: T) -> RetType {
    return value.textLength(self);
    // return 1;
  }
}

pub trait QTextLine_textLength<RetType> {
  fn textLength(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  int QTextLine::textLength();
impl<'a> /*trait*/ QTextLine_textLength<i32> for () {
  fn textLength(self, rsthis: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine10textLengthEv()};
    let mut ret = unsafe {_ZNK9QTextLine10textLengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn position<RetType, T: QTextLine_position<RetType>>(&mut self, value: T) -> RetType {
    return value.position(self);
    // return 1;
  }
}

pub trait QTextLine_position<RetType> {
  fn position(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  QPointF QTextLine::position();
impl<'a> /*trait*/ QTextLine_position<QPointF> for () {
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
  pub fn glyphRuns<RetType, T: QTextLine_glyphRuns<RetType>>(&mut self, value: T) -> RetType {
    return value.glyphRuns(self);
    // return 1;
  }
}

pub trait QTextLine_glyphRuns<RetType> {
  fn glyphRuns(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  QList<QGlyphRun> QTextLine::glyphRuns(int from, int length);
impl<'a> /*trait*/ QTextLine_glyphRuns<()> for (i32, i32) {
  fn glyphRuns(self, rsthis: &mut QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine9glyphRunsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZNK9QTextLine9glyphRunsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn descent<RetType, T: QTextLine_descent<RetType>>(&mut self, value: T) -> RetType {
    return value.descent(self);
    // return 1;
  }
}

pub trait QTextLine_descent<RetType> {
  fn descent(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  double QTextLine::descent();
impl<'a> /*trait*/ QTextLine_descent<f64> for () {
  fn descent(self, rsthis: &mut QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine7descentEv()};
    let mut ret = unsafe {_ZNK9QTextLine7descentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn naturalTextWidth<RetType, T: QTextLine_naturalTextWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.naturalTextWidth(self);
    // return 1;
  }
}

pub trait QTextLine_naturalTextWidth<RetType> {
  fn naturalTextWidth(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  double QTextLine::naturalTextWidth();
impl<'a> /*trait*/ QTextLine_naturalTextWidth<f64> for () {
  fn naturalTextWidth(self, rsthis: &mut QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine16naturalTextWidthEv()};
    let mut ret = unsafe {_ZNK9QTextLine16naturalTextWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn setLineWidth<RetType, T: QTextLine_setLineWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.setLineWidth(self);
    // return 1;
  }
}

pub trait QTextLine_setLineWidth<RetType> {
  fn setLineWidth(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  void QTextLine::setLineWidth(qreal width);
impl<'a> /*trait*/ QTextLine_setLineWidth<()> for (f64) {
  fn setLineWidth(self, rsthis: &mut QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine12setLineWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN9QTextLine12setLineWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn isValid<RetType, T: QTextLine_isValid<RetType>>(&mut self, value: T) -> RetType {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QTextLine_isValid<RetType> {
  fn isValid(self, rsthis: &mut QTextLine) -> RetType;
}

// proto:  bool QTextLine::isValid();
impl<'a> /*trait*/ QTextLine_isValid<i8> for () {
  fn isValid(self, rsthis: &mut QTextLine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine7isValidEv()};
    let mut ret = unsafe {_ZNK9QTextLine7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

