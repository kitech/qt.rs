// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: double QTextLine::ascent();
  fn _ZNK9QTextLine6ascentEv() -> i32;
  // proto: double QTextLine::leading();
  fn _ZNK9QTextLine7leadingEv() -> i32;
  // proto: int QTextLine::textStart();
  fn _ZNK9QTextLine9textStartEv() -> i32;
  // proto: bool QTextLine::leadingIncluded();
  fn _ZNK9QTextLine15leadingIncludedEv() -> i32;
  // proto: double QTextLine::x();
  fn _ZNK9QTextLine1xEv() -> i32;
  // proto: double QTextLine::height();
  fn _ZNK9QTextLine6heightEv() -> i32;
  // proto: double QTextLine::y();
  fn _ZNK9QTextLine1yEv() -> i32;
  // proto: double QTextLine::horizontalAdvance();
  fn _ZNK9QTextLine17horizontalAdvanceEv() -> i32;
  // proto: QRectF QTextLine::naturalTextRect();
  fn _ZNK9QTextLine15naturalTextRectEv() -> i32;
  // proto: void QTextLine::setNumColumns(int columns, qreal alignmentWidth);
  fn _ZN9QTextLine13setNumColumnsEid(arg0: c_int, arg1: c_double) -> i32;
  // proto: double QTextLine::width();
  fn _ZNK9QTextLine5widthEv() -> i32;
  // proto: void QTextLine::setLeadingIncluded(bool included);
  fn _ZN9QTextLine18setLeadingIncludedEb(arg0: int8_t) -> i32;
  // proto: void QTextLine::setPosition(const QPointF & pos);
  fn _ZN9QTextLine11setPositionERK7QPointF(arg0: *const c_void) -> i32;
  // proto: int QTextLine::lineNumber();
  fn _ZNK9QTextLine10lineNumberEv() -> i32;
  // proto: QRectF QTextLine::rect();
  fn _ZNK9QTextLine4rectEv() -> i32;
  // proto: void QTextLine::NewQTextLine();
  fn _ZN9QTextLineC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QTextLine::setNumColumns(int columns);
  fn _ZN9QTextLine13setNumColumnsEi(arg0: c_int) -> i32;
  // proto: int QTextLine::textLength();
  fn _ZNK9QTextLine10textLengthEv() -> i32;
  // proto: QPointF QTextLine::position();
  fn _ZNK9QTextLine8positionEv() -> i32;
  // proto: QList<QGlyphRun> QTextLine::glyphRuns(int from, int length);
  fn _ZNK9QTextLine9glyphRunsEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: double QTextLine::descent();
  fn _ZNK9QTextLine7descentEv() -> i32;
  // proto: double QTextLine::naturalTextWidth();
  fn _ZNK9QTextLine16naturalTextWidthEv() -> i32;
  // proto: void QTextLine::setLineWidth(qreal width);
  fn _ZN9QTextLine12setLineWidthEd(arg0: c_double) -> i32;
  // proto: bool QTextLine::isValid();
  fn _ZNK9QTextLine7isValidEv() -> i32;
}

// body block begin
// class sizeof(QTextLine)=16
pub struct QTextLine {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextLine {
  pub fn ascent<T: QTextLine_ascent>(&mut self, value: T) -> i32 {
    value.ascent(self);
    return 1;
  }
}

pub trait QTextLine_ascent {
  fn ascent(self, this: &mut QTextLine) -> i32;
}

// proto: double QTextLine::ascent();
impl<'a> /*trait*/ QTextLine_ascent for () {
  fn ascent(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine6ascentEv()};
    unsafe {_ZNK9QTextLine6ascentEv()};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn leading<T: QTextLine_leading>(&mut self, value: T) -> i32 {
    value.leading(self);
    return 1;
  }
}

pub trait QTextLine_leading {
  fn leading(self, this: &mut QTextLine) -> i32;
}

// proto: double QTextLine::leading();
impl<'a> /*trait*/ QTextLine_leading for () {
  fn leading(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine7leadingEv()};
    unsafe {_ZNK9QTextLine7leadingEv()};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn textStart<T: QTextLine_textStart>(&mut self, value: T) -> i32 {
    value.textStart(self);
    return 1;
  }
}

pub trait QTextLine_textStart {
  fn textStart(self, this: &mut QTextLine) -> i32;
}

// proto: int QTextLine::textStart();
impl<'a> /*trait*/ QTextLine_textStart for () {
  fn textStart(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine9textStartEv()};
    unsafe {_ZNK9QTextLine9textStartEv()};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn leadingIncluded<T: QTextLine_leadingIncluded>(&mut self, value: T) -> i32 {
    value.leadingIncluded(self);
    return 1;
  }
}

pub trait QTextLine_leadingIncluded {
  fn leadingIncluded(self, this: &mut QTextLine) -> i32;
}

// proto: bool QTextLine::leadingIncluded();
impl<'a> /*trait*/ QTextLine_leadingIncluded for () {
  fn leadingIncluded(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine15leadingIncludedEv()};
    unsafe {_ZNK9QTextLine15leadingIncludedEv()};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn x<T: QTextLine_x>(&mut self, value: T) -> i32 {
    value.x(self);
    return 1;
  }
}

pub trait QTextLine_x {
  fn x(self, this: &mut QTextLine) -> i32;
}

// proto: double QTextLine::x();
impl<'a> /*trait*/ QTextLine_x for () {
  fn x(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine1xEv()};
    unsafe {_ZNK9QTextLine1xEv()};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn height<T: QTextLine_height>(&mut self, value: T) -> i32 {
    value.height(self);
    return 1;
  }
}

pub trait QTextLine_height {
  fn height(self, this: &mut QTextLine) -> i32;
}

// proto: double QTextLine::height();
impl<'a> /*trait*/ QTextLine_height for () {
  fn height(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine6heightEv()};
    unsafe {_ZNK9QTextLine6heightEv()};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn y<T: QTextLine_y>(&mut self, value: T) -> i32 {
    value.y(self);
    return 1;
  }
}

pub trait QTextLine_y {
  fn y(self, this: &mut QTextLine) -> i32;
}

// proto: double QTextLine::y();
impl<'a> /*trait*/ QTextLine_y for () {
  fn y(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine1yEv()};
    unsafe {_ZNK9QTextLine1yEv()};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn horizontalAdvance<T: QTextLine_horizontalAdvance>(&mut self, value: T) -> i32 {
    value.horizontalAdvance(self);
    return 1;
  }
}

pub trait QTextLine_horizontalAdvance {
  fn horizontalAdvance(self, this: &mut QTextLine) -> i32;
}

// proto: double QTextLine::horizontalAdvance();
impl<'a> /*trait*/ QTextLine_horizontalAdvance for () {
  fn horizontalAdvance(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine17horizontalAdvanceEv()};
    unsafe {_ZNK9QTextLine17horizontalAdvanceEv()};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn naturalTextRect<T: QTextLine_naturalTextRect>(&mut self, value: T) -> i32 {
    value.naturalTextRect(self);
    return 1;
  }
}

pub trait QTextLine_naturalTextRect {
  fn naturalTextRect(self, this: &mut QTextLine) -> i32;
}

// proto: QRectF QTextLine::naturalTextRect();
impl<'a> /*trait*/ QTextLine_naturalTextRect for () {
  fn naturalTextRect(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine15naturalTextRectEv()};
    unsafe {_ZNK9QTextLine15naturalTextRectEv()};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn setNumColumns<T: QTextLine_setNumColumns>(&mut self, value: T) -> i32 {
    value.setNumColumns(self);
    return 1;
  }
}

pub trait QTextLine_setNumColumns {
  fn setNumColumns(self, this: &mut QTextLine) -> i32;
}

// proto: void QTextLine::setNumColumns(int columns, qreal alignmentWidth);
impl<'a> /*trait*/ QTextLine_setNumColumns for (i32, f64) {
  fn setNumColumns(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine13setNumColumnsEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
    unsafe {_ZN9QTextLine13setNumColumnsEid(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn width<T: QTextLine_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QTextLine_width {
  fn width(self, this: &mut QTextLine) -> i32;
}

// proto: double QTextLine::width();
impl<'a> /*trait*/ QTextLine_width for () {
  fn width(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine5widthEv()};
    unsafe {_ZNK9QTextLine5widthEv()};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn setLeadingIncluded<T: QTextLine_setLeadingIncluded>(&mut self, value: T) -> i32 {
    value.setLeadingIncluded(self);
    return 1;
  }
}

pub trait QTextLine_setLeadingIncluded {
  fn setLeadingIncluded(self, this: &mut QTextLine) -> i32;
}

// proto: void QTextLine::setLeadingIncluded(bool included);
impl<'a> /*trait*/ QTextLine_setLeadingIncluded for (i8) {
  fn setLeadingIncluded(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine18setLeadingIncludedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QTextLine18setLeadingIncludedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn setPosition<T: QTextLine_setPosition>(&mut self, value: T) -> i32 {
    value.setPosition(self);
    return 1;
  }
}

pub trait QTextLine_setPosition {
  fn setPosition(self, this: &mut QTextLine) -> i32;
}

// proto: void QTextLine::setPosition(const QPointF & pos);
impl<'a> /*trait*/ QTextLine_setPosition for (&'a  QPointF) {
  fn setPosition(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine11setPositionERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QTextLine11setPositionERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn lineNumber<T: QTextLine_lineNumber>(&mut self, value: T) -> i32 {
    value.lineNumber(self);
    return 1;
  }
}

pub trait QTextLine_lineNumber {
  fn lineNumber(self, this: &mut QTextLine) -> i32;
}

// proto: int QTextLine::lineNumber();
impl<'a> /*trait*/ QTextLine_lineNumber for () {
  fn lineNumber(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine10lineNumberEv()};
    unsafe {_ZNK9QTextLine10lineNumberEv()};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn rect<T: QTextLine_rect>(&mut self, value: T) -> i32 {
    value.rect(self);
    return 1;
  }
}

pub trait QTextLine_rect {
  fn rect(self, this: &mut QTextLine) -> i32;
}

// proto: QRectF QTextLine::rect();
impl<'a> /*trait*/ QTextLine_rect for () {
  fn rect(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine4rectEv()};
    unsafe {_ZNK9QTextLine4rectEv()};
    return 1;
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

// proto: void QTextLine::setNumColumns(int columns);
impl<'a> /*trait*/ QTextLine_setNumColumns for (i32) {
  fn setNumColumns(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine13setNumColumnsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QTextLine13setNumColumnsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn textLength<T: QTextLine_textLength>(&mut self, value: T) -> i32 {
    value.textLength(self);
    return 1;
  }
}

pub trait QTextLine_textLength {
  fn textLength(self, this: &mut QTextLine) -> i32;
}

// proto: int QTextLine::textLength();
impl<'a> /*trait*/ QTextLine_textLength for () {
  fn textLength(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine10textLengthEv()};
    unsafe {_ZNK9QTextLine10textLengthEv()};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn position<T: QTextLine_position>(&mut self, value: T) -> i32 {
    value.position(self);
    return 1;
  }
}

pub trait QTextLine_position {
  fn position(self, this: &mut QTextLine) -> i32;
}

// proto: QPointF QTextLine::position();
impl<'a> /*trait*/ QTextLine_position for () {
  fn position(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine8positionEv()};
    unsafe {_ZNK9QTextLine8positionEv()};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn glyphRuns<T: QTextLine_glyphRuns>(&mut self, value: T) -> i32 {
    value.glyphRuns(self);
    return 1;
  }
}

pub trait QTextLine_glyphRuns {
  fn glyphRuns(self, this: &mut QTextLine) -> i32;
}

// proto: QList<QGlyphRun> QTextLine::glyphRuns(int from, int length);
impl<'a> /*trait*/ QTextLine_glyphRuns for (i32, i32) {
  fn glyphRuns(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine9glyphRunsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK9QTextLine9glyphRunsEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn descent<T: QTextLine_descent>(&mut self, value: T) -> i32 {
    value.descent(self);
    return 1;
  }
}

pub trait QTextLine_descent {
  fn descent(self, this: &mut QTextLine) -> i32;
}

// proto: double QTextLine::descent();
impl<'a> /*trait*/ QTextLine_descent for () {
  fn descent(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine7descentEv()};
    unsafe {_ZNK9QTextLine7descentEv()};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn naturalTextWidth<T: QTextLine_naturalTextWidth>(&mut self, value: T) -> i32 {
    value.naturalTextWidth(self);
    return 1;
  }
}

pub trait QTextLine_naturalTextWidth {
  fn naturalTextWidth(self, this: &mut QTextLine) -> i32;
}

// proto: double QTextLine::naturalTextWidth();
impl<'a> /*trait*/ QTextLine_naturalTextWidth for () {
  fn naturalTextWidth(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine16naturalTextWidthEv()};
    unsafe {_ZNK9QTextLine16naturalTextWidthEv()};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn setLineWidth<T: QTextLine_setLineWidth>(&mut self, value: T) -> i32 {
    value.setLineWidth(self);
    return 1;
  }
}

pub trait QTextLine_setLineWidth {
  fn setLineWidth(self, this: &mut QTextLine) -> i32;
}

// proto: void QTextLine::setLineWidth(qreal width);
impl<'a> /*trait*/ QTextLine_setLineWidth for (f64) {
  fn setLineWidth(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine12setLineWidthEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN9QTextLine12setLineWidthEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextLine {
  pub fn isValid<T: QTextLine_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QTextLine_isValid {
  fn isValid(self, this: &mut QTextLine) -> i32;
}

// proto: bool QTextLine::isValid();
impl<'a> /*trait*/ QTextLine_isValid for () {
  fn isValid(self, this: &mut QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine7isValidEv()};
    unsafe {_ZNK9QTextLine7isValidEv()};
    return 1;
  }
}

