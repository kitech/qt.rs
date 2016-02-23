// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtGui/qtextlayout.h
// dst-file: /src/gui/qtextlayout.rs
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
use super::super::core::qrect::*; // 771
use super::super::core::qpoint::*; // 771
use super::qpainter::*; // 773
// use super::qlist::*; // 775
use super::qfont::*; // 773
use super::super::core::qstring::*; // 771
use super::qrawfont::*; // 773
use super::qtextoption::*; // 773
use super::qpaintdevice::*; // 773
// use super::qtextlayout::QTextLine; // 773
// use super::qtextengine::*; // 775
use super::qtextobject::*; // 773
use super::qtextformat::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTextLine_Class_Size() -> c_int;
  // proto:  qreal QTextLine::ascent();
  fn C_ZNK9QTextLine6ascentEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QTextLine::leading();
  fn C_ZNK9QTextLine7leadingEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  int QTextLine::textStart();
  fn C_ZNK9QTextLine9textStartEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QTextLine::leadingIncluded();
  fn C_ZNK9QTextLine15leadingIncludedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qreal QTextLine::x();
  fn C_ZNK9QTextLine1xEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QTextLine::height();
  fn C_ZNK9QTextLine6heightEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QTextLine::y();
  fn C_ZNK9QTextLine1yEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QTextLine::horizontalAdvance();
  fn C_ZNK9QTextLine17horizontalAdvanceEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QRectF QTextLine::naturalTextRect();
  fn C_ZNK9QTextLine15naturalTextRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextLine::setNumColumns(int columns, qreal alignmentWidth);
  fn C_ZN9QTextLine13setNumColumnsEid(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_double);
  // proto:  qreal QTextLine::width();
  fn C_ZNK9QTextLine5widthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QTextLine::setLeadingIncluded(bool included);
  fn C_ZN9QTextLine18setLeadingIncludedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTextLine::setPosition(const QPointF & pos);
  fn C_ZN9QTextLine11setPositionERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QTextLine::lineNumber();
  fn C_ZNK9QTextLine10lineNumberEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QRectF QTextLine::rect();
  fn C_ZNK9QTextLine4rectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextLine::QTextLine();
  fn C_ZN9QTextLineC2Ev() -> u64;
  // proto:  void QTextLine::setNumColumns(int columns);
  fn C_ZN9QTextLine13setNumColumnsEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QTextLine::textLength();
  fn C_ZNK9QTextLine10textLengthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QPointF QTextLine::position();
  fn C_ZNK9QTextLine8positionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<QGlyphRun> QTextLine::glyphRuns(int from, int length);
  fn C_ZNK9QTextLine9glyphRunsEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  qreal QTextLine::descent();
  fn C_ZNK9QTextLine7descentEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QTextLine::naturalTextWidth();
  fn C_ZNK9QTextLine16naturalTextWidthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QTextLine::setLineWidth(qreal width);
  fn C_ZN9QTextLine12setLineWidthEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  bool QTextLine::isValid();
  fn C_ZNK9QTextLine7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QTextLayout_Class_Size() -> c_int;
  // proto:  void QTextLayout::setFont(const QFont & f);
  fn C_ZN11QTextLayout7setFontERK5QFont(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextLayout::setText(const QString & string);
  fn C_ZN11QTextLayout7setTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QTextLayout::isValidCursorPosition(int pos);
  fn C_ZNK11QTextLayout21isValidCursorPositionEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  QRectF QTextLayout::boundingRect();
  fn C_ZNK11QTextLayout12boundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextLayout::setRawFont(const QRawFont & rawFont);
  fn C_ZN11QTextLayout10setRawFontERK8QRawFont(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextLayout::setTextOption(const QTextOption & option);
  fn C_ZN11QTextLayout13setTextOptionERK11QTextOption(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextLayout::QTextLayout(const QString & text, const QFont & font, QPaintDevice * paintdevice);
  fn C_ZN11QTextLayoutC2ERK7QStringRK5QFontP12QPaintDevice(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> u64;
  // proto:  void QTextLayout::setPosition(const QPointF & p);
  fn C_ZN11QTextLayout11setPositionERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QTextLine QTextLayout::lineForTextPosition(int pos);
  fn C_ZNK11QTextLayout19lineForTextPositionEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  const QTextOption & QTextLayout::textOption();
  fn C_ZNK11QTextLayout10textOptionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTextEngine * QTextLayout::engine();
  fn C_ZNK11QTextLayout6engineEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTextLayout::preeditAreaPosition();
  fn C_ZNK11QTextLayout19preeditAreaPositionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextLayout::clearAdditionalFormats();
  fn C_ZN11QTextLayout22clearAdditionalFormatsEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QTextLayout::leftCursorPosition(int oldPos);
  fn C_ZNK11QTextLayout18leftCursorPositionEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  int QTextLayout::lineCount();
  fn C_ZNK11QTextLayout9lineCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextLayout::~QTextLayout();
  fn C_ZN11QTextLayoutD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextLayout::setCacheEnabled(bool enable);
  fn C_ZN11QTextLayout15setCacheEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QTextLine QTextLayout::lineAt(int i);
  fn C_ZNK11QTextLayout6lineAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  int QTextLayout::rightCursorPosition(int oldPos);
  fn C_ZNK11QTextLayout19rightCursorPositionEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  void QTextLayout::QTextLayout(const QTextBlock & b);
  fn C_ZN11QTextLayoutC2ERK10QTextBlock(arg0: *mut c_void) -> u64;
  // proto:  qreal QTextLayout::minimumWidth();
  fn C_ZNK11QTextLayout12minimumWidthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QTextLayout::drawCursor(QPainter * p, const QPointF & pos, int cursorPosition);
  fn C_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int);
  // proto:  QFont QTextLayout::font();
  fn C_ZNK11QTextLayout4fontEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextLayout::setPreeditArea(int position, const QString & text);
  fn C_ZN11QTextLayout14setPreeditAreaEiRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QTextLayout::beginLayout();
  fn C_ZN11QTextLayout11beginLayoutEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextLayout::QTextLayout(const QString & text);
  fn C_ZN11QTextLayoutC2ERK7QString(arg0: *mut c_void) -> u64;
  // proto:  void QTextLayout::setFlags(int flags);
  fn C_ZN11QTextLayout8setFlagsEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QPointF QTextLayout::position();
  fn C_ZNK11QTextLayout8positionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextLayout::clearLayout();
  fn C_ZN11QTextLayout11clearLayoutEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QTextLayout::cacheEnabled();
  fn C_ZNK11QTextLayout12cacheEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qreal QTextLayout::maximumWidth();
  fn C_ZNK11QTextLayout12maximumWidthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QString QTextLayout::text();
  fn C_ZNK11QTextLayout4textEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTextLine QTextLayout::createLine();
  fn C_ZN11QTextLayout10createLineEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QTextLayout::preeditAreaText();
  fn C_ZNK11QTextLayout15preeditAreaTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextLayout::drawCursor(QPainter * p, const QPointF & pos, int cursorPosition, int width);
  fn C_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFii(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: c_int);
  // proto:  void QTextLayout::endLayout();
  fn C_ZN11QTextLayout9endLayoutEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextLayout::QTextLayout();
  fn C_ZN11QTextLayoutC2Ev() -> u64;
  // proto:  QList<QGlyphRun> QTextLayout::glyphRuns(int from, int length);
  fn C_ZNK11QTextLayout9glyphRunsEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  fn QTextInlineObject_Class_Size() -> c_int;
  // proto:  void QTextInlineObject::setAscent(qreal a);
  fn C_ZN17QTextInlineObject9setAscentEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  qreal QTextInlineObject::width();
  fn C_ZNK17QTextInlineObject5widthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  int QTextInlineObject::formatIndex();
  fn C_ZNK17QTextInlineObject11formatIndexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QRectF QTextInlineObject::rect();
  fn C_ZNK17QTextInlineObject4rectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTextInlineObject::textPosition();
  fn C_ZNK17QTextInlineObject12textPositionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextInlineObject::setDescent(qreal d);
  fn C_ZN17QTextInlineObject10setDescentEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  qreal QTextInlineObject::height();
  fn C_ZNK17QTextInlineObject6heightEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  bool QTextInlineObject::isValid();
  fn C_ZNK17QTextInlineObject7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTextInlineObject::QTextInlineObject();
  fn C_ZN17QTextInlineObjectC2Ev() -> u64;
  // proto:  QTextFormat QTextInlineObject::format();
  fn C_ZNK17QTextInlineObject6formatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QTextInlineObject::descent();
  fn C_ZNK17QTextInlineObject7descentEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QTextInlineObject::ascent();
  fn C_ZNK17QTextInlineObject6ascentEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QTextInlineObject::setWidth(qreal w);
  fn C_ZN17QTextInlineObject8setWidthEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
} // <= ext block end

// body block begin =>
// class sizeof(QTextLine)=16
#[derive(Default)]
pub struct QTextLine {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QTextLayout)=8
#[derive(Default)]
pub struct QTextLayout {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QTextInlineObject)=16
#[derive(Default)]
pub struct QTextInlineObject {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTextLine {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextLine {
    return QTextLine{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  qreal QTextLine::ascent();
impl /*struct*/ QTextLine {
  pub fn ascent<RetType, T: QTextLine_ascent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ascent(self);
    // return 1;
  }
}

pub trait QTextLine_ascent<RetType> {
  fn ascent(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  qreal QTextLine::ascent();
impl<'a> /*trait*/ QTextLine_ascent<f64> for () {
  fn ascent(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine6ascentEv()};
    let mut ret = unsafe {C_ZNK9QTextLine6ascentEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  qreal QTextLine::leading();
impl /*struct*/ QTextLine {
  pub fn leading<RetType, T: QTextLine_leading<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.leading(self);
    // return 1;
  }
}

pub trait QTextLine_leading<RetType> {
  fn leading(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  qreal QTextLine::leading();
impl<'a> /*trait*/ QTextLine_leading<f64> for () {
  fn leading(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine7leadingEv()};
    let mut ret = unsafe {C_ZNK9QTextLine7leadingEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  int QTextLine::textStart();
impl /*struct*/ QTextLine {
  pub fn textStart<RetType, T: QTextLine_textStart<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textStart(self);
    // return 1;
  }
}

pub trait QTextLine_textStart<RetType> {
  fn textStart(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  int QTextLine::textStart();
impl<'a> /*trait*/ QTextLine_textStart<i32> for () {
  fn textStart(self , rsthis: & QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine9textStartEv()};
    let mut ret = unsafe {C_ZNK9QTextLine9textStartEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QTextLine::leadingIncluded();
impl /*struct*/ QTextLine {
  pub fn leadingIncluded<RetType, T: QTextLine_leadingIncluded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.leadingIncluded(self);
    // return 1;
  }
}

pub trait QTextLine_leadingIncluded<RetType> {
  fn leadingIncluded(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  bool QTextLine::leadingIncluded();
impl<'a> /*trait*/ QTextLine_leadingIncluded<i8> for () {
  fn leadingIncluded(self , rsthis: & QTextLine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine15leadingIncludedEv()};
    let mut ret = unsafe {C_ZNK9QTextLine15leadingIncludedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  qreal QTextLine::x();
impl /*struct*/ QTextLine {
  pub fn x<RetType, T: QTextLine_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QTextLine_x<RetType> {
  fn x(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  qreal QTextLine::x();
impl<'a> /*trait*/ QTextLine_x<f64> for () {
  fn x(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine1xEv()};
    let mut ret = unsafe {C_ZNK9QTextLine1xEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  qreal QTextLine::height();
impl /*struct*/ QTextLine {
  pub fn height<RetType, T: QTextLine_height<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QTextLine_height<RetType> {
  fn height(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  qreal QTextLine::height();
impl<'a> /*trait*/ QTextLine_height<f64> for () {
  fn height(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine6heightEv()};
    let mut ret = unsafe {C_ZNK9QTextLine6heightEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  qreal QTextLine::y();
impl /*struct*/ QTextLine {
  pub fn y<RetType, T: QTextLine_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QTextLine_y<RetType> {
  fn y(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  qreal QTextLine::y();
impl<'a> /*trait*/ QTextLine_y<f64> for () {
  fn y(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine1yEv()};
    let mut ret = unsafe {C_ZNK9QTextLine1yEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  qreal QTextLine::horizontalAdvance();
impl /*struct*/ QTextLine {
  pub fn horizontalAdvance<RetType, T: QTextLine_horizontalAdvance<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.horizontalAdvance(self);
    // return 1;
  }
}

pub trait QTextLine_horizontalAdvance<RetType> {
  fn horizontalAdvance(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  qreal QTextLine::horizontalAdvance();
impl<'a> /*trait*/ QTextLine_horizontalAdvance<f64> for () {
  fn horizontalAdvance(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine17horizontalAdvanceEv()};
    let mut ret = unsafe {C_ZNK9QTextLine17horizontalAdvanceEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  QRectF QTextLine::naturalTextRect();
impl /*struct*/ QTextLine {
  pub fn naturalTextRect<RetType, T: QTextLine_naturalTextRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.naturalTextRect(self);
    // return 1;
  }
}

pub trait QTextLine_naturalTextRect<RetType> {
  fn naturalTextRect(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  QRectF QTextLine::naturalTextRect();
impl<'a> /*trait*/ QTextLine_naturalTextRect<QRectF> for () {
  fn naturalTextRect(self , rsthis: & QTextLine) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine15naturalTextRectEv()};
    let mut ret = unsafe {C_ZNK9QTextLine15naturalTextRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextLine::setNumColumns(int columns, qreal alignmentWidth);
impl /*struct*/ QTextLine {
  pub fn setNumColumns<RetType, T: QTextLine_setNumColumns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNumColumns(self);
    // return 1;
  }
}

pub trait QTextLine_setNumColumns<RetType> {
  fn setNumColumns(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  void QTextLine::setNumColumns(int columns, qreal alignmentWidth);
impl<'a> /*trait*/ QTextLine_setNumColumns<()> for (i32, f64) {
  fn setNumColumns(self , rsthis: & QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine13setNumColumnsEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {C_ZN9QTextLine13setNumColumnsEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  qreal QTextLine::width();
impl /*struct*/ QTextLine {
  pub fn width<RetType, T: QTextLine_width<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QTextLine_width<RetType> {
  fn width(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  qreal QTextLine::width();
impl<'a> /*trait*/ QTextLine_width<f64> for () {
  fn width(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine5widthEv()};
    let mut ret = unsafe {C_ZNK9QTextLine5widthEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QTextLine::setLeadingIncluded(bool included);
impl /*struct*/ QTextLine {
  pub fn setLeadingIncluded<RetType, T: QTextLine_setLeadingIncluded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLeadingIncluded(self);
    // return 1;
  }
}

pub trait QTextLine_setLeadingIncluded<RetType> {
  fn setLeadingIncluded(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  void QTextLine::setLeadingIncluded(bool included);
impl<'a> /*trait*/ QTextLine_setLeadingIncluded<()> for (i8) {
  fn setLeadingIncluded(self , rsthis: & QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine18setLeadingIncludedEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN9QTextLine18setLeadingIncludedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextLine::setPosition(const QPointF & pos);
impl /*struct*/ QTextLine {
  pub fn setPosition<RetType, T: QTextLine_setPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPosition(self);
    // return 1;
  }
}

pub trait QTextLine_setPosition<RetType> {
  fn setPosition(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  void QTextLine::setPosition(const QPointF & pos);
impl<'a> /*trait*/ QTextLine_setPosition<()> for (&'a QPointF) {
  fn setPosition(self , rsthis: & QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine11setPositionERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QTextLine11setPositionERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextLine::lineNumber();
impl /*struct*/ QTextLine {
  pub fn lineNumber<RetType, T: QTextLine_lineNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineNumber(self);
    // return 1;
  }
}

pub trait QTextLine_lineNumber<RetType> {
  fn lineNumber(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  int QTextLine::lineNumber();
impl<'a> /*trait*/ QTextLine_lineNumber<i32> for () {
  fn lineNumber(self , rsthis: & QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine10lineNumberEv()};
    let mut ret = unsafe {C_ZNK9QTextLine10lineNumberEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QRectF QTextLine::rect();
impl /*struct*/ QTextLine {
  pub fn rect<RetType, T: QTextLine_rect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QTextLine_rect<RetType> {
  fn rect(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  QRectF QTextLine::rect();
impl<'a> /*trait*/ QTextLine_rect<QRectF> for () {
  fn rect(self , rsthis: & QTextLine) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine4rectEv()};
    let mut ret = unsafe {C_ZNK9QTextLine4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextLine::QTextLine();
impl /*struct*/ QTextLine {
  pub fn new<T: QTextLine_new>(value: T) -> QTextLine {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTextLine_new {
  fn new(self) -> QTextLine;
}

  // proto:  void QTextLine::QTextLine();
impl<'a> /*trait*/ QTextLine_new for () {
  fn new(self) -> QTextLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLineC2Ev()};
    let ctysz: c_int = unsafe{QTextLine_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN9QTextLineC2Ev()};
    let rsthis = QTextLine{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextLine::setNumColumns(int columns);
impl<'a> /*trait*/ QTextLine_setNumColumns<()> for (i32) {
  fn setNumColumns(self , rsthis: & QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine13setNumColumnsEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN9QTextLine13setNumColumnsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextLine::textLength();
impl /*struct*/ QTextLine {
  pub fn textLength<RetType, T: QTextLine_textLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textLength(self);
    // return 1;
  }
}

pub trait QTextLine_textLength<RetType> {
  fn textLength(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  int QTextLine::textLength();
impl<'a> /*trait*/ QTextLine_textLength<i32> for () {
  fn textLength(self , rsthis: & QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine10textLengthEv()};
    let mut ret = unsafe {C_ZNK9QTextLine10textLengthEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QPointF QTextLine::position();
impl /*struct*/ QTextLine {
  pub fn position<RetType, T: QTextLine_position<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QTextLine_position<RetType> {
  fn position(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  QPointF QTextLine::position();
impl<'a> /*trait*/ QTextLine_position<QPointF> for () {
  fn position(self , rsthis: & QTextLine) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine8positionEv()};
    let mut ret = unsafe {C_ZNK9QTextLine8positionEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QGlyphRun> QTextLine::glyphRuns(int from, int length);
impl /*struct*/ QTextLine {
  pub fn glyphRuns<RetType, T: QTextLine_glyphRuns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glyphRuns(self);
    // return 1;
  }
}

pub trait QTextLine_glyphRuns<RetType> {
  fn glyphRuns(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  QList<QGlyphRun> QTextLine::glyphRuns(int from, int length);
impl<'a> /*trait*/ QTextLine_glyphRuns<u64> for (Option<i32>, Option<i32>) {
  fn glyphRuns(self , rsthis: & QTextLine) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine9glyphRunsEii()};
    let arg0 = (if self.0.is_none() {-1} else {self.0.unwrap()})  as c_int;
    let arg1 = (if self.1.is_none() {-1} else {self.1.unwrap()})  as c_int;
    let mut ret = unsafe {C_ZNK9QTextLine9glyphRunsEii(rsthis.qclsinst, arg0, arg1)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  qreal QTextLine::descent();
impl /*struct*/ QTextLine {
  pub fn descent<RetType, T: QTextLine_descent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.descent(self);
    // return 1;
  }
}

pub trait QTextLine_descent<RetType> {
  fn descent(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  qreal QTextLine::descent();
impl<'a> /*trait*/ QTextLine_descent<f64> for () {
  fn descent(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine7descentEv()};
    let mut ret = unsafe {C_ZNK9QTextLine7descentEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  qreal QTextLine::naturalTextWidth();
impl /*struct*/ QTextLine {
  pub fn naturalTextWidth<RetType, T: QTextLine_naturalTextWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.naturalTextWidth(self);
    // return 1;
  }
}

pub trait QTextLine_naturalTextWidth<RetType> {
  fn naturalTextWidth(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  qreal QTextLine::naturalTextWidth();
impl<'a> /*trait*/ QTextLine_naturalTextWidth<f64> for () {
  fn naturalTextWidth(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine16naturalTextWidthEv()};
    let mut ret = unsafe {C_ZNK9QTextLine16naturalTextWidthEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QTextLine::setLineWidth(qreal width);
impl /*struct*/ QTextLine {
  pub fn setLineWidth<RetType, T: QTextLine_setLineWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLineWidth(self);
    // return 1;
  }
}

pub trait QTextLine_setLineWidth<RetType> {
  fn setLineWidth(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  void QTextLine::setLineWidth(qreal width);
impl<'a> /*trait*/ QTextLine_setLineWidth<()> for (f64) {
  fn setLineWidth(self , rsthis: & QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine12setLineWidthEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN9QTextLine12setLineWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextLine::isValid();
impl /*struct*/ QTextLine {
  pub fn isValid<RetType, T: QTextLine_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextLine_isValid<RetType> {
  fn isValid(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  bool QTextLine::isValid();
impl<'a> /*trait*/ QTextLine_isValid<i8> for () {
  fn isValid(self , rsthis: & QTextLine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine7isValidEv()};
    let mut ret = unsafe {C_ZNK9QTextLine7isValidEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextLayout {
    return QTextLayout{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QTextLayout::setFont(const QFont & f);
impl /*struct*/ QTextLayout {
  pub fn setFont<RetType, T: QTextLayout_setFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFont(self);
    // return 1;
  }
}

pub trait QTextLayout_setFont<RetType> {
  fn setFont(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::setFont(const QFont & f);
impl<'a> /*trait*/ QTextLayout_setFont<()> for (&'a QFont) {
  fn setFont(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QTextLayout7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextLayout::setText(const QString & string);
impl /*struct*/ QTextLayout {
  pub fn setText<RetType, T: QTextLayout_setText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QTextLayout_setText<RetType> {
  fn setText(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::setText(const QString & string);
impl<'a> /*trait*/ QTextLayout_setText<()> for (&'a QString) {
  fn setText(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QTextLayout7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextLayout::isValidCursorPosition(int pos);
impl /*struct*/ QTextLayout {
  pub fn isValidCursorPosition<RetType, T: QTextLayout_isValidCursorPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValidCursorPosition(self);
    // return 1;
  }
}

pub trait QTextLayout_isValidCursorPosition<RetType> {
  fn isValidCursorPosition(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  bool QTextLayout::isValidCursorPosition(int pos);
impl<'a> /*trait*/ QTextLayout_isValidCursorPosition<i8> for (i32) {
  fn isValidCursorPosition(self , rsthis: & QTextLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout21isValidCursorPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK11QTextLayout21isValidCursorPositionEi(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QRectF QTextLayout::boundingRect();
impl /*struct*/ QTextLayout {
  pub fn boundingRect<RetType, T: QTextLayout_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QTextLayout_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QRectF QTextLayout::boundingRect();
impl<'a> /*trait*/ QTextLayout_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: & QTextLayout) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout12boundingRectEv()};
    let mut ret = unsafe {C_ZNK11QTextLayout12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextLayout::setRawFont(const QRawFont & rawFont);
impl /*struct*/ QTextLayout {
  pub fn setRawFont<RetType, T: QTextLayout_setRawFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRawFont(self);
    // return 1;
  }
}

pub trait QTextLayout_setRawFont<RetType> {
  fn setRawFont(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::setRawFont(const QRawFont & rawFont);
impl<'a> /*trait*/ QTextLayout_setRawFont<()> for (&'a QRawFont) {
  fn setRawFont(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout10setRawFontERK8QRawFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QTextLayout10setRawFontERK8QRawFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextLayout::setTextOption(const QTextOption & option);
impl /*struct*/ QTextLayout {
  pub fn setTextOption<RetType, T: QTextLayout_setTextOption<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextOption(self);
    // return 1;
  }
}

pub trait QTextLayout_setTextOption<RetType> {
  fn setTextOption(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::setTextOption(const QTextOption & option);
impl<'a> /*trait*/ QTextLayout_setTextOption<()> for (&'a QTextOption) {
  fn setTextOption(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout13setTextOptionERK11QTextOption()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QTextLayout13setTextOptionERK11QTextOption(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextLayout::QTextLayout(const QString & text, const QFont & font, QPaintDevice * paintdevice);
impl /*struct*/ QTextLayout {
  pub fn new<T: QTextLayout_new>(value: T) -> QTextLayout {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTextLayout_new {
  fn new(self) -> QTextLayout;
}

  // proto:  void QTextLayout::QTextLayout(const QString & text, const QFont & font, QPaintDevice * paintdevice);
impl<'a> /*trait*/ QTextLayout_new for (&'a QString, &'a QFont, Option<&'a QPaintDevice>) {
  fn new(self) -> QTextLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutC2ERK7QStringRK5QFontP12QPaintDevice()};
    let ctysz: c_int = unsafe{QTextLayout_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = (if self.2.is_none() {0} else {self.2.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QTextLayoutC2ERK7QStringRK5QFontP12QPaintDevice(arg0, arg1, arg2)};
    let rsthis = QTextLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextLayout::setPosition(const QPointF & p);
impl /*struct*/ QTextLayout {
  pub fn setPosition<RetType, T: QTextLayout_setPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPosition(self);
    // return 1;
  }
}

pub trait QTextLayout_setPosition<RetType> {
  fn setPosition(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::setPosition(const QPointF & p);
impl<'a> /*trait*/ QTextLayout_setPosition<()> for (&'a QPointF) {
  fn setPosition(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout11setPositionERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QTextLayout11setPositionERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextLine QTextLayout::lineForTextPosition(int pos);
impl /*struct*/ QTextLayout {
  pub fn lineForTextPosition<RetType, T: QTextLayout_lineForTextPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineForTextPosition(self);
    // return 1;
  }
}

pub trait QTextLayout_lineForTextPosition<RetType> {
  fn lineForTextPosition(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QTextLine QTextLayout::lineForTextPosition(int pos);
impl<'a> /*trait*/ QTextLayout_lineForTextPosition<QTextLine> for (i32) {
  fn lineForTextPosition(self , rsthis: & QTextLayout) -> QTextLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout19lineForTextPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK11QTextLayout19lineForTextPositionEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextLine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QTextOption & QTextLayout::textOption();
impl /*struct*/ QTextLayout {
  pub fn textOption<RetType, T: QTextLayout_textOption<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textOption(self);
    // return 1;
  }
}

pub trait QTextLayout_textOption<RetType> {
  fn textOption(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  const QTextOption & QTextLayout::textOption();
impl<'a> /*trait*/ QTextLayout_textOption<QTextOption> for () {
  fn textOption(self , rsthis: & QTextLayout) -> QTextOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout10textOptionEv()};
    let mut ret = unsafe {C_ZNK11QTextLayout10textOptionEv(rsthis.qclsinst)};
    let mut ret1 = QTextOption::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextEngine * QTextLayout::engine();
impl /*struct*/ QTextLayout {
  pub fn engine<RetType, T: QTextLayout_engine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.engine(self);
    // return 1;
  }
}

pub trait QTextLayout_engine<RetType> {
  fn engine(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QTextEngine * QTextLayout::engine();
impl<'a> /*trait*/ QTextLayout_engine<u64> for () {
  fn engine(self , rsthis: & QTextLayout) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout6engineEv()};
    let mut ret = unsafe {C_ZNK11QTextLayout6engineEv(rsthis.qclsinst)};
    return ret as u64; // 4
    // return 1;
  }
}

  // proto:  int QTextLayout::preeditAreaPosition();
impl /*struct*/ QTextLayout {
  pub fn preeditAreaPosition<RetType, T: QTextLayout_preeditAreaPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.preeditAreaPosition(self);
    // return 1;
  }
}

pub trait QTextLayout_preeditAreaPosition<RetType> {
  fn preeditAreaPosition(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  int QTextLayout::preeditAreaPosition();
impl<'a> /*trait*/ QTextLayout_preeditAreaPosition<i32> for () {
  fn preeditAreaPosition(self , rsthis: & QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout19preeditAreaPositionEv()};
    let mut ret = unsafe {C_ZNK11QTextLayout19preeditAreaPositionEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTextLayout::clearAdditionalFormats();
impl /*struct*/ QTextLayout {
  pub fn clearAdditionalFormats<RetType, T: QTextLayout_clearAdditionalFormats<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearAdditionalFormats(self);
    // return 1;
  }
}

pub trait QTextLayout_clearAdditionalFormats<RetType> {
  fn clearAdditionalFormats(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::clearAdditionalFormats();
impl<'a> /*trait*/ QTextLayout_clearAdditionalFormats<()> for () {
  fn clearAdditionalFormats(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout22clearAdditionalFormatsEv()};
     unsafe {C_ZN11QTextLayout22clearAdditionalFormatsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTextLayout::leftCursorPosition(int oldPos);
impl /*struct*/ QTextLayout {
  pub fn leftCursorPosition<RetType, T: QTextLayout_leftCursorPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.leftCursorPosition(self);
    // return 1;
  }
}

pub trait QTextLayout_leftCursorPosition<RetType> {
  fn leftCursorPosition(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  int QTextLayout::leftCursorPosition(int oldPos);
impl<'a> /*trait*/ QTextLayout_leftCursorPosition<i32> for (i32) {
  fn leftCursorPosition(self , rsthis: & QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout18leftCursorPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK11QTextLayout18leftCursorPositionEi(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QTextLayout::lineCount();
impl /*struct*/ QTextLayout {
  pub fn lineCount<RetType, T: QTextLayout_lineCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineCount(self);
    // return 1;
  }
}

pub trait QTextLayout_lineCount<RetType> {
  fn lineCount(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  int QTextLayout::lineCount();
impl<'a> /*trait*/ QTextLayout_lineCount<i32> for () {
  fn lineCount(self , rsthis: & QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout9lineCountEv()};
    let mut ret = unsafe {C_ZNK11QTextLayout9lineCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTextLayout::~QTextLayout();
impl /*struct*/ QTextLayout {
  pub fn free<RetType, T: QTextLayout_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTextLayout_free<RetType> {
  fn free(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::~QTextLayout();
impl<'a> /*trait*/ QTextLayout_free<()> for () {
  fn free(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutD2Ev()};
     unsafe {C_ZN11QTextLayoutD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextLayout::setCacheEnabled(bool enable);
impl /*struct*/ QTextLayout {
  pub fn setCacheEnabled<RetType, T: QTextLayout_setCacheEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCacheEnabled(self);
    // return 1;
  }
}

pub trait QTextLayout_setCacheEnabled<RetType> {
  fn setCacheEnabled(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::setCacheEnabled(bool enable);
impl<'a> /*trait*/ QTextLayout_setCacheEnabled<()> for (i8) {
  fn setCacheEnabled(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout15setCacheEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN11QTextLayout15setCacheEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextLine QTextLayout::lineAt(int i);
impl /*struct*/ QTextLayout {
  pub fn lineAt<RetType, T: QTextLayout_lineAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineAt(self);
    // return 1;
  }
}

pub trait QTextLayout_lineAt<RetType> {
  fn lineAt(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QTextLine QTextLayout::lineAt(int i);
impl<'a> /*trait*/ QTextLayout_lineAt<QTextLine> for (i32) {
  fn lineAt(self , rsthis: & QTextLayout) -> QTextLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout6lineAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK11QTextLayout6lineAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextLine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextLayout::rightCursorPosition(int oldPos);
impl /*struct*/ QTextLayout {
  pub fn rightCursorPosition<RetType, T: QTextLayout_rightCursorPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rightCursorPosition(self);
    // return 1;
  }
}

pub trait QTextLayout_rightCursorPosition<RetType> {
  fn rightCursorPosition(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  int QTextLayout::rightCursorPosition(int oldPos);
impl<'a> /*trait*/ QTextLayout_rightCursorPosition<i32> for (i32) {
  fn rightCursorPosition(self , rsthis: & QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout19rightCursorPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK11QTextLayout19rightCursorPositionEi(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTextLayout::QTextLayout(const QTextBlock & b);
impl<'a> /*trait*/ QTextLayout_new for (&'a QTextBlock) {
  fn new(self) -> QTextLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutC2ERK10QTextBlock()};
    let ctysz: c_int = unsafe{QTextLayout_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QTextLayoutC2ERK10QTextBlock(arg0)};
    let rsthis = QTextLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QTextLayout::minimumWidth();
impl /*struct*/ QTextLayout {
  pub fn minimumWidth<RetType, T: QTextLayout_minimumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumWidth(self);
    // return 1;
  }
}

pub trait QTextLayout_minimumWidth<RetType> {
  fn minimumWidth(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  qreal QTextLayout::minimumWidth();
impl<'a> /*trait*/ QTextLayout_minimumWidth<f64> for () {
  fn minimumWidth(self , rsthis: & QTextLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout12minimumWidthEv()};
    let mut ret = unsafe {C_ZNK11QTextLayout12minimumWidthEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QTextLayout::drawCursor(QPainter * p, const QPointF & pos, int cursorPosition);
impl /*struct*/ QTextLayout {
  pub fn drawCursor<RetType, T: QTextLayout_drawCursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawCursor(self);
    // return 1;
  }
}

pub trait QTextLayout_drawCursor<RetType> {
  fn drawCursor(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::drawCursor(QPainter * p, const QPointF & pos, int cursorPosition);
impl<'a> /*trait*/ QTextLayout_drawCursor<()> for (&'a QPainter, &'a QPointF, i32) {
  fn drawCursor(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {C_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QFont QTextLayout::font();
impl /*struct*/ QTextLayout {
  pub fn font<RetType, T: QTextLayout_font<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QTextLayout_font<RetType> {
  fn font(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QFont QTextLayout::font();
impl<'a> /*trait*/ QTextLayout_font<QFont> for () {
  fn font(self , rsthis: & QTextLayout) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout4fontEv()};
    let mut ret = unsafe {C_ZNK11QTextLayout4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextLayout::setPreeditArea(int position, const QString & text);
impl /*struct*/ QTextLayout {
  pub fn setPreeditArea<RetType, T: QTextLayout_setPreeditArea<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPreeditArea(self);
    // return 1;
  }
}

pub trait QTextLayout_setPreeditArea<RetType> {
  fn setPreeditArea(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::setPreeditArea(int position, const QString & text);
impl<'a> /*trait*/ QTextLayout_setPreeditArea<()> for (i32, &'a QString) {
  fn setPreeditArea(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout14setPreeditAreaEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN11QTextLayout14setPreeditAreaEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTextLayout::beginLayout();
impl /*struct*/ QTextLayout {
  pub fn beginLayout<RetType, T: QTextLayout_beginLayout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.beginLayout(self);
    // return 1;
  }
}

pub trait QTextLayout_beginLayout<RetType> {
  fn beginLayout(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::beginLayout();
impl<'a> /*trait*/ QTextLayout_beginLayout<()> for () {
  fn beginLayout(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout11beginLayoutEv()};
     unsafe {C_ZN11QTextLayout11beginLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextLayout::QTextLayout(const QString & text);
impl<'a> /*trait*/ QTextLayout_new for (&'a QString) {
  fn new(self) -> QTextLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutC2ERK7QString()};
    let ctysz: c_int = unsafe{QTextLayout_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QTextLayoutC2ERK7QString(arg0)};
    let rsthis = QTextLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextLayout::setFlags(int flags);
impl /*struct*/ QTextLayout {
  pub fn setFlags<RetType, T: QTextLayout_setFlags<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFlags(self);
    // return 1;
  }
}

pub trait QTextLayout_setFlags<RetType> {
  fn setFlags(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::setFlags(int flags);
impl<'a> /*trait*/ QTextLayout_setFlags<()> for (i32) {
  fn setFlags(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout8setFlagsEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN11QTextLayout8setFlagsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPointF QTextLayout::position();
impl /*struct*/ QTextLayout {
  pub fn position<RetType, T: QTextLayout_position<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QTextLayout_position<RetType> {
  fn position(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QPointF QTextLayout::position();
impl<'a> /*trait*/ QTextLayout_position<QPointF> for () {
  fn position(self , rsthis: & QTextLayout) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout8positionEv()};
    let mut ret = unsafe {C_ZNK11QTextLayout8positionEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextLayout::clearLayout();
impl /*struct*/ QTextLayout {
  pub fn clearLayout<RetType, T: QTextLayout_clearLayout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearLayout(self);
    // return 1;
  }
}

pub trait QTextLayout_clearLayout<RetType> {
  fn clearLayout(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::clearLayout();
impl<'a> /*trait*/ QTextLayout_clearLayout<()> for () {
  fn clearLayout(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout11clearLayoutEv()};
     unsafe {C_ZN11QTextLayout11clearLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTextLayout::cacheEnabled();
impl /*struct*/ QTextLayout {
  pub fn cacheEnabled<RetType, T: QTextLayout_cacheEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cacheEnabled(self);
    // return 1;
  }
}

pub trait QTextLayout_cacheEnabled<RetType> {
  fn cacheEnabled(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  bool QTextLayout::cacheEnabled();
impl<'a> /*trait*/ QTextLayout_cacheEnabled<i8> for () {
  fn cacheEnabled(self , rsthis: & QTextLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout12cacheEnabledEv()};
    let mut ret = unsafe {C_ZNK11QTextLayout12cacheEnabledEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  qreal QTextLayout::maximumWidth();
impl /*struct*/ QTextLayout {
  pub fn maximumWidth<RetType, T: QTextLayout_maximumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumWidth(self);
    // return 1;
  }
}

pub trait QTextLayout_maximumWidth<RetType> {
  fn maximumWidth(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  qreal QTextLayout::maximumWidth();
impl<'a> /*trait*/ QTextLayout_maximumWidth<f64> for () {
  fn maximumWidth(self , rsthis: & QTextLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout12maximumWidthEv()};
    let mut ret = unsafe {C_ZNK11QTextLayout12maximumWidthEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  QString QTextLayout::text();
impl /*struct*/ QTextLayout {
  pub fn text<RetType, T: QTextLayout_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QTextLayout_text<RetType> {
  fn text(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QString QTextLayout::text();
impl<'a> /*trait*/ QTextLayout_text<QString> for () {
  fn text(self , rsthis: & QTextLayout) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout4textEv()};
    let mut ret = unsafe {C_ZNK11QTextLayout4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextLine QTextLayout::createLine();
impl /*struct*/ QTextLayout {
  pub fn createLine<RetType, T: QTextLayout_createLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createLine(self);
    // return 1;
  }
}

pub trait QTextLayout_createLine<RetType> {
  fn createLine(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QTextLine QTextLayout::createLine();
impl<'a> /*trait*/ QTextLayout_createLine<QTextLine> for () {
  fn createLine(self , rsthis: & QTextLayout) -> QTextLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout10createLineEv()};
    let mut ret = unsafe {C_ZN11QTextLayout10createLineEv(rsthis.qclsinst)};
    let mut ret1 = QTextLine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QTextLayout::preeditAreaText();
impl /*struct*/ QTextLayout {
  pub fn preeditAreaText<RetType, T: QTextLayout_preeditAreaText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.preeditAreaText(self);
    // return 1;
  }
}

pub trait QTextLayout_preeditAreaText<RetType> {
  fn preeditAreaText(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QString QTextLayout::preeditAreaText();
impl<'a> /*trait*/ QTextLayout_preeditAreaText<QString> for () {
  fn preeditAreaText(self , rsthis: & QTextLayout) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout15preeditAreaTextEv()};
    let mut ret = unsafe {C_ZNK11QTextLayout15preeditAreaTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextLayout::drawCursor(QPainter * p, const QPointF & pos, int cursorPosition, int width);
impl<'a> /*trait*/ QTextLayout_drawCursor<()> for (&'a QPainter, &'a QPointF, i32, i32) {
  fn drawCursor(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {C_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QTextLayout::endLayout();
impl /*struct*/ QTextLayout {
  pub fn endLayout<RetType, T: QTextLayout_endLayout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.endLayout(self);
    // return 1;
  }
}

pub trait QTextLayout_endLayout<RetType> {
  fn endLayout(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::endLayout();
impl<'a> /*trait*/ QTextLayout_endLayout<()> for () {
  fn endLayout(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout9endLayoutEv()};
     unsafe {C_ZN11QTextLayout9endLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextLayout::QTextLayout();
impl<'a> /*trait*/ QTextLayout_new for () {
  fn new(self) -> QTextLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutC2Ev()};
    let ctysz: c_int = unsafe{QTextLayout_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN11QTextLayoutC2Ev()};
    let rsthis = QTextLayout{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QList<QGlyphRun> QTextLayout::glyphRuns(int from, int length);
impl /*struct*/ QTextLayout {
  pub fn glyphRuns<RetType, T: QTextLayout_glyphRuns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glyphRuns(self);
    // return 1;
  }
}

pub trait QTextLayout_glyphRuns<RetType> {
  fn glyphRuns(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QList<QGlyphRun> QTextLayout::glyphRuns(int from, int length);
impl<'a> /*trait*/ QTextLayout_glyphRuns<u64> for (Option<i32>, Option<i32>) {
  fn glyphRuns(self , rsthis: & QTextLayout) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout9glyphRunsEii()};
    let arg0 = (if self.0.is_none() {-1} else {self.0.unwrap()})  as c_int;
    let arg1 = (if self.1.is_none() {-1} else {self.1.unwrap()})  as c_int;
    let mut ret = unsafe {C_ZNK11QTextLayout9glyphRunsEii(rsthis.qclsinst, arg0, arg1)};
    return ret as u64; // 5
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextInlineObject {
    return QTextInlineObject{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QTextInlineObject::setAscent(qreal a);
impl /*struct*/ QTextInlineObject {
  pub fn setAscent<RetType, T: QTextInlineObject_setAscent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAscent(self);
    // return 1;
  }
}

pub trait QTextInlineObject_setAscent<RetType> {
  fn setAscent(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  void QTextInlineObject::setAscent(qreal a);
impl<'a> /*trait*/ QTextInlineObject_setAscent<()> for (f64) {
  fn setAscent(self , rsthis: & QTextInlineObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QTextInlineObject9setAscentEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN17QTextInlineObject9setAscentEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextInlineObject::width();
impl /*struct*/ QTextInlineObject {
  pub fn width<RetType, T: QTextInlineObject_width<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QTextInlineObject_width<RetType> {
  fn width(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  qreal QTextInlineObject::width();
impl<'a> /*trait*/ QTextInlineObject_width<f64> for () {
  fn width(self , rsthis: & QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject5widthEv()};
    let mut ret = unsafe {C_ZNK17QTextInlineObject5widthEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  int QTextInlineObject::formatIndex();
impl /*struct*/ QTextInlineObject {
  pub fn formatIndex<RetType, T: QTextInlineObject_formatIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.formatIndex(self);
    // return 1;
  }
}

pub trait QTextInlineObject_formatIndex<RetType> {
  fn formatIndex(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  int QTextInlineObject::formatIndex();
impl<'a> /*trait*/ QTextInlineObject_formatIndex<i32> for () {
  fn formatIndex(self , rsthis: & QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject11formatIndexEv()};
    let mut ret = unsafe {C_ZNK17QTextInlineObject11formatIndexEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QRectF QTextInlineObject::rect();
impl /*struct*/ QTextInlineObject {
  pub fn rect<RetType, T: QTextInlineObject_rect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QTextInlineObject_rect<RetType> {
  fn rect(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  QRectF QTextInlineObject::rect();
impl<'a> /*trait*/ QTextInlineObject_rect<QRectF> for () {
  fn rect(self , rsthis: & QTextInlineObject) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject4rectEv()};
    let mut ret = unsafe {C_ZNK17QTextInlineObject4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextInlineObject::textPosition();
impl /*struct*/ QTextInlineObject {
  pub fn textPosition<RetType, T: QTextInlineObject_textPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textPosition(self);
    // return 1;
  }
}

pub trait QTextInlineObject_textPosition<RetType> {
  fn textPosition(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  int QTextInlineObject::textPosition();
impl<'a> /*trait*/ QTextInlineObject_textPosition<i32> for () {
  fn textPosition(self , rsthis: & QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject12textPositionEv()};
    let mut ret = unsafe {C_ZNK17QTextInlineObject12textPositionEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTextInlineObject::setDescent(qreal d);
impl /*struct*/ QTextInlineObject {
  pub fn setDescent<RetType, T: QTextInlineObject_setDescent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDescent(self);
    // return 1;
  }
}

pub trait QTextInlineObject_setDescent<RetType> {
  fn setDescent(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  void QTextInlineObject::setDescent(qreal d);
impl<'a> /*trait*/ QTextInlineObject_setDescent<()> for (f64) {
  fn setDescent(self , rsthis: & QTextInlineObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QTextInlineObject10setDescentEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN17QTextInlineObject10setDescentEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextInlineObject::height();
impl /*struct*/ QTextInlineObject {
  pub fn height<RetType, T: QTextInlineObject_height<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QTextInlineObject_height<RetType> {
  fn height(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  qreal QTextInlineObject::height();
impl<'a> /*trait*/ QTextInlineObject_height<f64> for () {
  fn height(self , rsthis: & QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject6heightEv()};
    let mut ret = unsafe {C_ZNK17QTextInlineObject6heightEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  bool QTextInlineObject::isValid();
impl /*struct*/ QTextInlineObject {
  pub fn isValid<RetType, T: QTextInlineObject_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextInlineObject_isValid<RetType> {
  fn isValid(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  bool QTextInlineObject::isValid();
impl<'a> /*trait*/ QTextInlineObject_isValid<i8> for () {
  fn isValid(self , rsthis: & QTextInlineObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject7isValidEv()};
    let mut ret = unsafe {C_ZNK17QTextInlineObject7isValidEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QTextInlineObject::QTextInlineObject();
impl /*struct*/ QTextInlineObject {
  pub fn new<T: QTextInlineObject_new>(value: T) -> QTextInlineObject {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTextInlineObject_new {
  fn new(self) -> QTextInlineObject;
}

  // proto:  void QTextInlineObject::QTextInlineObject();
impl<'a> /*trait*/ QTextInlineObject_new for () {
  fn new(self) -> QTextInlineObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QTextInlineObjectC2Ev()};
    let ctysz: c_int = unsafe{QTextInlineObject_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN17QTextInlineObjectC2Ev()};
    let rsthis = QTextInlineObject{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QTextFormat QTextInlineObject::format();
impl /*struct*/ QTextInlineObject {
  pub fn format<RetType, T: QTextInlineObject_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QTextInlineObject_format<RetType> {
  fn format(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  QTextFormat QTextInlineObject::format();
impl<'a> /*trait*/ QTextInlineObject_format<QTextFormat> for () {
  fn format(self , rsthis: & QTextInlineObject) -> QTextFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject6formatEv()};
    let mut ret = unsafe {C_ZNK17QTextInlineObject6formatEv(rsthis.qclsinst)};
    let mut ret1 = QTextFormat::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QTextInlineObject::descent();
impl /*struct*/ QTextInlineObject {
  pub fn descent<RetType, T: QTextInlineObject_descent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.descent(self);
    // return 1;
  }
}

pub trait QTextInlineObject_descent<RetType> {
  fn descent(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  qreal QTextInlineObject::descent();
impl<'a> /*trait*/ QTextInlineObject_descent<f64> for () {
  fn descent(self , rsthis: & QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject7descentEv()};
    let mut ret = unsafe {C_ZNK17QTextInlineObject7descentEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  qreal QTextInlineObject::ascent();
impl /*struct*/ QTextInlineObject {
  pub fn ascent<RetType, T: QTextInlineObject_ascent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ascent(self);
    // return 1;
  }
}

pub trait QTextInlineObject_ascent<RetType> {
  fn ascent(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  qreal QTextInlineObject::ascent();
impl<'a> /*trait*/ QTextInlineObject_ascent<f64> for () {
  fn ascent(self , rsthis: & QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject6ascentEv()};
    let mut ret = unsafe {C_ZNK17QTextInlineObject6ascentEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QTextInlineObject::setWidth(qreal w);
impl /*struct*/ QTextInlineObject {
  pub fn setWidth<RetType, T: QTextInlineObject_setWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWidth(self);
    // return 1;
  }
}

pub trait QTextInlineObject_setWidth<RetType> {
  fn setWidth(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  void QTextInlineObject::setWidth(qreal w);
impl<'a> /*trait*/ QTextInlineObject_setWidth<()> for (f64) {
  fn setWidth(self , rsthis: & QTextInlineObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QTextInlineObject8setWidthEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN17QTextInlineObject8setWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

