// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qfont::QFont;
use super::qstring::QString;
use super::qrawfont::QRawFont;
use super::qtextoption::QTextOption;
use super::qpaintdevice::QPaintDevice;
use super::qpointf::QPointF;
use super::qtextline::QTextLine;
use super::qtextblock::QTextBlock;
use super::qpainter::QPainter;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTextLayout::setFont(const QFont & f);
  fn _ZN11QTextLayout7setFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextLayout::setText(const QString & string);
  fn _ZN11QTextLayout7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTextLayout::isValidCursorPosition(int pos);
  fn _ZNK11QTextLayout21isValidCursorPositionEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QTextLayout::setRawFont(const QRawFont & rawFont);
  fn _ZN11QTextLayout10setRawFontERK8QRawFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextLayout::setTextOption(const QTextOption & option);
  fn _ZN11QTextLayout13setTextOptionERK11QTextOption(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextLayout::NewQTextLayout(const QString & text, const QFont & font, QPaintDevice * paintdevice);
  fn _ZN11QTextLayoutC1ERK7QStringRK5QFontP12QPaintDevice(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QTextLayout::setPosition(const QPointF & p);
  fn _ZN11QTextLayout11setPositionERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextLine QTextLayout::lineForTextPosition(int pos);
  fn _ZNK11QTextLayout19lineForTextPositionEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  const QTextOption & QTextLayout::textOption();
  fn _ZNK11QTextLayout10textOptionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextEngine * QTextLayout::engine();
  fn _ZNK11QTextLayout6engineEv(qthis: *mut c_void) ;
  // proto:  int QTextLayout::preeditAreaPosition();
  fn _ZNK11QTextLayout19preeditAreaPositionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextLayout::clearAdditionalFormats();
  fn _ZN11QTextLayout22clearAdditionalFormatsEv(qthis: *mut c_void) ;
  // proto:  int QTextLayout::leftCursorPosition(int oldPos);
  fn _ZNK11QTextLayout18leftCursorPositionEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  int QTextLayout::lineCount();
  fn _ZNK11QTextLayout9lineCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextLayout::FreeQTextLayout();
  fn _ZN11QTextLayoutD0Ev(qthis: *mut c_void) ;
  // proto:  void QTextLayout::setCacheEnabled(bool enable);
  fn _ZN11QTextLayout15setCacheEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QTextLine QTextLayout::lineAt(int i);
  fn _ZNK11QTextLayout6lineAtEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QTextLayout::rightCursorPosition(int oldPos);
  fn _ZNK11QTextLayout19rightCursorPositionEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QTextLayout::NewQTextLayout(const QTextBlock & b);
  fn _ZN11QTextLayoutC1ERK10QTextBlock(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QTextLayout::minimumWidth();
  fn _ZNK11QTextLayout12minimumWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextLayout::drawCursor(QPainter * p, const QPointF & pos, int cursorPosition);
  fn _ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFi(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) ;
  // proto:  QFont QTextLayout::font();
  fn _ZNK11QTextLayout4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextLayout::setPreeditArea(int position, const QString & text);
  fn _ZN11QTextLayout14setPreeditAreaEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QTextLayout::beginLayout();
  fn _ZN11QTextLayout11beginLayoutEv(qthis: *mut c_void) ;
  // proto:  void QTextLayout::NewQTextLayout(const QString & text);
  fn _ZN11QTextLayoutC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextLayout::setFlags(int flags);
  fn _ZN11QTextLayout8setFlagsEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QPointF QTextLayout::position();
  fn _ZNK11QTextLayout8positionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextLayout::clearLayout();
  fn _ZN11QTextLayout11clearLayoutEv(qthis: *mut c_void) ;
  // proto:  bool QTextLayout::cacheEnabled();
  fn _ZNK11QTextLayout12cacheEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  double QTextLayout::maximumWidth();
  fn _ZNK11QTextLayout12maximumWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  QString QTextLayout::text();
  fn _ZNK11QTextLayout4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextLayout::NewQTextLayout(const QTextLayout & );
  fn _ZN11QTextLayoutC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTextLine QTextLayout::createLine();
  fn _ZN11QTextLayout10createLineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QTextLayout::preeditAreaText();
  fn _ZNK11QTextLayout15preeditAreaTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextLayout::drawCursor(QPainter * p, const QPointF & pos, int cursorPosition, int width);
  fn _ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFii(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: c_int) ;
  // proto:  void QTextLayout::endLayout();
  fn _ZN11QTextLayout9endLayoutEv(qthis: *mut c_void) ;
  // proto:  void QTextLayout::NewQTextLayout();
  fn _ZN11QTextLayoutC1Ev(qthis: *mut c_void) ;
  // proto:  QList<QGlyphRun> QTextLayout::glyphRuns(int from, int length);
  fn _ZNK11QTextLayout9glyphRunsEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
}

// body block begin
// class sizeof(QTextLayout)=8
pub struct QTextLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextLayout {
  pub fn setFont<RetType, T: QTextLayout_setFont<RetType>>(&mut self, value: T) -> RetType {
    return value.setFont(self);
    // return 1;
  }
}

pub trait QTextLayout_setFont<RetType> {
  fn setFont(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  void QTextLayout::setFont(const QFont & f);
impl<'a> /*trait*/ QTextLayout_setFont<()> for (&'a  QFont) {
  fn setFont(self, rsthis: &mut QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextLayout7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn setText<RetType, T: QTextLayout_setText<RetType>>(&mut self, value: T) -> RetType {
    return value.setText(self);
    // return 1;
  }
}

pub trait QTextLayout_setText<RetType> {
  fn setText(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  void QTextLayout::setText(const QString & string);
impl<'a> /*trait*/ QTextLayout_setText<()> for (&'a  QString) {
  fn setText(self, rsthis: &mut QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextLayout7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn isValidCursorPosition<RetType, T: QTextLayout_isValidCursorPosition<RetType>>(&mut self, value: T) -> RetType {
    return value.isValidCursorPosition(self);
    // return 1;
  }
}

pub trait QTextLayout_isValidCursorPosition<RetType> {
  fn isValidCursorPosition(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  bool QTextLayout::isValidCursorPosition(int pos);
impl<'a> /*trait*/ QTextLayout_isValidCursorPosition<i8> for (i32) {
  fn isValidCursorPosition(self, rsthis: &mut QTextLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout21isValidCursorPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextLayout21isValidCursorPositionEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn setRawFont<RetType, T: QTextLayout_setRawFont<RetType>>(&mut self, value: T) -> RetType {
    return value.setRawFont(self);
    // return 1;
  }
}

pub trait QTextLayout_setRawFont<RetType> {
  fn setRawFont(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  void QTextLayout::setRawFont(const QRawFont & rawFont);
impl<'a> /*trait*/ QTextLayout_setRawFont<()> for (&'a  QRawFont) {
  fn setRawFont(self, rsthis: &mut QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout10setRawFontERK8QRawFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextLayout10setRawFontERK8QRawFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn setTextOption<RetType, T: QTextLayout_setTextOption<RetType>>(&mut self, value: T) -> RetType {
    return value.setTextOption(self);
    // return 1;
  }
}

pub trait QTextLayout_setTextOption<RetType> {
  fn setTextOption(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  void QTextLayout::setTextOption(const QTextOption & option);
impl<'a> /*trait*/ QTextLayout_setTextOption<()> for (&'a  QTextOption) {
  fn setTextOption(self, rsthis: &mut QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout13setTextOptionERK11QTextOption()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextLayout13setTextOptionERK11QTextOption(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn NewQTextLayout<T: QTextLayout_NewQTextLayout>(value: T) -> QTextLayout {
    let rsthis = value.NewQTextLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QTextLayout_NewQTextLayout {
  fn NewQTextLayout(self) -> QTextLayout;
}

// proto: void QTextLayout::NewQTextLayout(const QString & text, const QFont & font, QPaintDevice * paintdevice);
impl<'a> /*trait*/ QTextLayout_NewQTextLayout for (&'a  QString, &'a  QFont, &'a mut QPaintDevice) {
  fn NewQTextLayout(self) -> QTextLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutC1ERK7QStringRK5QFontP12QPaintDevice()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN11QTextLayoutC1ERK7QStringRK5QFontP12QPaintDevice(qthis, arg0, arg1, arg2)};
    let rsthis = QTextLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn setPosition<RetType, T: QTextLayout_setPosition<RetType>>(&mut self, value: T) -> RetType {
    return value.setPosition(self);
    // return 1;
  }
}

pub trait QTextLayout_setPosition<RetType> {
  fn setPosition(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  void QTextLayout::setPosition(const QPointF & p);
impl<'a> /*trait*/ QTextLayout_setPosition<()> for (&'a  QPointF) {
  fn setPosition(self, rsthis: &mut QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout11setPositionERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextLayout11setPositionERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn lineForTextPosition<RetType, T: QTextLayout_lineForTextPosition<RetType>>(&mut self, value: T) -> RetType {
    return value.lineForTextPosition(self);
    // return 1;
  }
}

pub trait QTextLayout_lineForTextPosition<RetType> {
  fn lineForTextPosition(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  QTextLine QTextLayout::lineForTextPosition(int pos);
impl<'a> /*trait*/ QTextLayout_lineForTextPosition<QTextLine> for (i32) {
  fn lineForTextPosition(self, rsthis: &mut QTextLayout) -> QTextLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout19lineForTextPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextLayout19lineForTextPositionEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextLine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn textOption<RetType, T: QTextLayout_textOption<RetType>>(&mut self, value: T) -> RetType {
    return value.textOption(self);
    // return 1;
  }
}

pub trait QTextLayout_textOption<RetType> {
  fn textOption(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  const QTextOption & QTextLayout::textOption();
impl<'a> /*trait*/ QTextLayout_textOption<QTextOption> for () {
  fn textOption(self, rsthis: &mut QTextLayout) -> QTextOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout10textOptionEv()};
    let mut ret = unsafe {_ZNK11QTextLayout10textOptionEv(rsthis.qclsinst)};
    let mut ret1 = QTextOption{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn engine<RetType, T: QTextLayout_engine<RetType>>(&mut self, value: T) -> RetType {
    return value.engine(self);
    // return 1;
  }
}

pub trait QTextLayout_engine<RetType> {
  fn engine(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  QTextEngine * QTextLayout::engine();
impl<'a> /*trait*/ QTextLayout_engine<()> for () {
  fn engine(self, rsthis: &mut QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout6engineEv()};
     unsafe {_ZNK11QTextLayout6engineEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn preeditAreaPosition<RetType, T: QTextLayout_preeditAreaPosition<RetType>>(&mut self, value: T) -> RetType {
    return value.preeditAreaPosition(self);
    // return 1;
  }
}

pub trait QTextLayout_preeditAreaPosition<RetType> {
  fn preeditAreaPosition(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  int QTextLayout::preeditAreaPosition();
impl<'a> /*trait*/ QTextLayout_preeditAreaPosition<i32> for () {
  fn preeditAreaPosition(self, rsthis: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout19preeditAreaPositionEv()};
    let mut ret = unsafe {_ZNK11QTextLayout19preeditAreaPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn clearAdditionalFormats<RetType, T: QTextLayout_clearAdditionalFormats<RetType>>(&mut self, value: T) -> RetType {
    return value.clearAdditionalFormats(self);
    // return 1;
  }
}

pub trait QTextLayout_clearAdditionalFormats<RetType> {
  fn clearAdditionalFormats(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  void QTextLayout::clearAdditionalFormats();
impl<'a> /*trait*/ QTextLayout_clearAdditionalFormats<()> for () {
  fn clearAdditionalFormats(self, rsthis: &mut QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout22clearAdditionalFormatsEv()};
     unsafe {_ZN11QTextLayout22clearAdditionalFormatsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn leftCursorPosition<RetType, T: QTextLayout_leftCursorPosition<RetType>>(&mut self, value: T) -> RetType {
    return value.leftCursorPosition(self);
    // return 1;
  }
}

pub trait QTextLayout_leftCursorPosition<RetType> {
  fn leftCursorPosition(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  int QTextLayout::leftCursorPosition(int oldPos);
impl<'a> /*trait*/ QTextLayout_leftCursorPosition<i32> for (i32) {
  fn leftCursorPosition(self, rsthis: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout18leftCursorPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextLayout18leftCursorPositionEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn lineCount<RetType, T: QTextLayout_lineCount<RetType>>(&mut self, value: T) -> RetType {
    return value.lineCount(self);
    // return 1;
  }
}

pub trait QTextLayout_lineCount<RetType> {
  fn lineCount(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  int QTextLayout::lineCount();
impl<'a> /*trait*/ QTextLayout_lineCount<i32> for () {
  fn lineCount(self, rsthis: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout9lineCountEv()};
    let mut ret = unsafe {_ZNK11QTextLayout9lineCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn FreeQTextLayout<RetType, T: QTextLayout_FreeQTextLayout<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQTextLayout(self);
    // return 1;
  }
}

pub trait QTextLayout_FreeQTextLayout<RetType> {
  fn FreeQTextLayout(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  void QTextLayout::FreeQTextLayout();
impl<'a> /*trait*/ QTextLayout_FreeQTextLayout<()> for () {
  fn FreeQTextLayout(self, rsthis: &mut QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutD0Ev()};
     unsafe {_ZN11QTextLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn setCacheEnabled<RetType, T: QTextLayout_setCacheEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setCacheEnabled(self);
    // return 1;
  }
}

pub trait QTextLayout_setCacheEnabled<RetType> {
  fn setCacheEnabled(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  void QTextLayout::setCacheEnabled(bool enable);
impl<'a> /*trait*/ QTextLayout_setCacheEnabled<()> for (i8) {
  fn setCacheEnabled(self, rsthis: &mut QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout15setCacheEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QTextLayout15setCacheEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn lineAt<RetType, T: QTextLayout_lineAt<RetType>>(&mut self, value: T) -> RetType {
    return value.lineAt(self);
    // return 1;
  }
}

pub trait QTextLayout_lineAt<RetType> {
  fn lineAt(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  QTextLine QTextLayout::lineAt(int i);
impl<'a> /*trait*/ QTextLayout_lineAt<QTextLine> for (i32) {
  fn lineAt(self, rsthis: &mut QTextLayout) -> QTextLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout6lineAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextLayout6lineAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextLine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn rightCursorPosition<RetType, T: QTextLayout_rightCursorPosition<RetType>>(&mut self, value: T) -> RetType {
    return value.rightCursorPosition(self);
    // return 1;
  }
}

pub trait QTextLayout_rightCursorPosition<RetType> {
  fn rightCursorPosition(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  int QTextLayout::rightCursorPosition(int oldPos);
impl<'a> /*trait*/ QTextLayout_rightCursorPosition<i32> for (i32) {
  fn rightCursorPosition(self, rsthis: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout19rightCursorPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextLayout19rightCursorPositionEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QTextLayout::NewQTextLayout(const QTextBlock & b);
impl<'a> /*trait*/ QTextLayout_NewQTextLayout for (&'a  QTextBlock) {
  fn NewQTextLayout(self) -> QTextLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutC1ERK10QTextBlock()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTextLayoutC1ERK10QTextBlock(qthis, arg0)};
    let rsthis = QTextLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn minimumWidth<RetType, T: QTextLayout_minimumWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.minimumWidth(self);
    // return 1;
  }
}

pub trait QTextLayout_minimumWidth<RetType> {
  fn minimumWidth(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  double QTextLayout::minimumWidth();
impl<'a> /*trait*/ QTextLayout_minimumWidth<f64> for () {
  fn minimumWidth(self, rsthis: &mut QTextLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout12minimumWidthEv()};
    let mut ret = unsafe {_ZNK11QTextLayout12minimumWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn drawCursor<RetType, T: QTextLayout_drawCursor<RetType>>(&mut self, value: T) -> RetType {
    return value.drawCursor(self);
    // return 1;
  }
}

pub trait QTextLayout_drawCursor<RetType> {
  fn drawCursor(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  void QTextLayout::drawCursor(QPainter * p, const QPointF & pos, int cursorPosition);
impl<'a> /*trait*/ QTextLayout_drawCursor<()> for (&'a mut QPainter, &'a  QPointF, i32) {
  fn drawCursor(self, rsthis: &mut QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn font<RetType, T: QTextLayout_font<RetType>>(&mut self, value: T) -> RetType {
    return value.font(self);
    // return 1;
  }
}

pub trait QTextLayout_font<RetType> {
  fn font(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  QFont QTextLayout::font();
impl<'a> /*trait*/ QTextLayout_font<QFont> for () {
  fn font(self, rsthis: &mut QTextLayout) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout4fontEv()};
    let mut ret = unsafe {_ZNK11QTextLayout4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn setPreeditArea<RetType, T: QTextLayout_setPreeditArea<RetType>>(&mut self, value: T) -> RetType {
    return value.setPreeditArea(self);
    // return 1;
  }
}

pub trait QTextLayout_setPreeditArea<RetType> {
  fn setPreeditArea(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  void QTextLayout::setPreeditArea(int position, const QString & text);
impl<'a> /*trait*/ QTextLayout_setPreeditArea<()> for (i32, &'a  QString) {
  fn setPreeditArea(self, rsthis: &mut QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout14setPreeditAreaEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextLayout14setPreeditAreaEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn beginLayout<RetType, T: QTextLayout_beginLayout<RetType>>(&mut self, value: T) -> RetType {
    return value.beginLayout(self);
    // return 1;
  }
}

pub trait QTextLayout_beginLayout<RetType> {
  fn beginLayout(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  void QTextLayout::beginLayout();
impl<'a> /*trait*/ QTextLayout_beginLayout<()> for () {
  fn beginLayout(self, rsthis: &mut QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout11beginLayoutEv()};
     unsafe {_ZN11QTextLayout11beginLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QTextLayout::NewQTextLayout(const QString & text);
impl<'a> /*trait*/ QTextLayout_NewQTextLayout for (&'a  QString) {
  fn NewQTextLayout(self) -> QTextLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTextLayoutC1ERK7QString(qthis, arg0)};
    let rsthis = QTextLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn setFlags<RetType, T: QTextLayout_setFlags<RetType>>(&mut self, value: T) -> RetType {
    return value.setFlags(self);
    // return 1;
  }
}

pub trait QTextLayout_setFlags<RetType> {
  fn setFlags(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  void QTextLayout::setFlags(int flags);
impl<'a> /*trait*/ QTextLayout_setFlags<()> for (i32) {
  fn setFlags(self, rsthis: &mut QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout8setFlagsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QTextLayout8setFlagsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn position<RetType, T: QTextLayout_position<RetType>>(&mut self, value: T) -> RetType {
    return value.position(self);
    // return 1;
  }
}

pub trait QTextLayout_position<RetType> {
  fn position(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  QPointF QTextLayout::position();
impl<'a> /*trait*/ QTextLayout_position<QPointF> for () {
  fn position(self, rsthis: &mut QTextLayout) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout8positionEv()};
    let mut ret = unsafe {_ZNK11QTextLayout8positionEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn clearLayout<RetType, T: QTextLayout_clearLayout<RetType>>(&mut self, value: T) -> RetType {
    return value.clearLayout(self);
    // return 1;
  }
}

pub trait QTextLayout_clearLayout<RetType> {
  fn clearLayout(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  void QTextLayout::clearLayout();
impl<'a> /*trait*/ QTextLayout_clearLayout<()> for () {
  fn clearLayout(self, rsthis: &mut QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout11clearLayoutEv()};
     unsafe {_ZN11QTextLayout11clearLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn cacheEnabled<RetType, T: QTextLayout_cacheEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.cacheEnabled(self);
    // return 1;
  }
}

pub trait QTextLayout_cacheEnabled<RetType> {
  fn cacheEnabled(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  bool QTextLayout::cacheEnabled();
impl<'a> /*trait*/ QTextLayout_cacheEnabled<i8> for () {
  fn cacheEnabled(self, rsthis: &mut QTextLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout12cacheEnabledEv()};
    let mut ret = unsafe {_ZNK11QTextLayout12cacheEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn maximumWidth<RetType, T: QTextLayout_maximumWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.maximumWidth(self);
    // return 1;
  }
}

pub trait QTextLayout_maximumWidth<RetType> {
  fn maximumWidth(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  double QTextLayout::maximumWidth();
impl<'a> /*trait*/ QTextLayout_maximumWidth<f64> for () {
  fn maximumWidth(self, rsthis: &mut QTextLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout12maximumWidthEv()};
    let mut ret = unsafe {_ZNK11QTextLayout12maximumWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn text<RetType, T: QTextLayout_text<RetType>>(&mut self, value: T) -> RetType {
    return value.text(self);
    // return 1;
  }
}

pub trait QTextLayout_text<RetType> {
  fn text(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  QString QTextLayout::text();
impl<'a> /*trait*/ QTextLayout_text<QString> for () {
  fn text(self, rsthis: &mut QTextLayout) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout4textEv()};
    let mut ret = unsafe {_ZNK11QTextLayout4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTextLayout::NewQTextLayout(const QTextLayout & );
impl<'a> /*trait*/ QTextLayout_NewQTextLayout for (&'a  QTextLayout) {
  fn NewQTextLayout(self) -> QTextLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTextLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QTextLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn createLine<RetType, T: QTextLayout_createLine<RetType>>(&mut self, value: T) -> RetType {
    return value.createLine(self);
    // return 1;
  }
}

pub trait QTextLayout_createLine<RetType> {
  fn createLine(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  QTextLine QTextLayout::createLine();
impl<'a> /*trait*/ QTextLayout_createLine<QTextLine> for () {
  fn createLine(self, rsthis: &mut QTextLayout) -> QTextLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout10createLineEv()};
    let mut ret = unsafe {_ZN11QTextLayout10createLineEv(rsthis.qclsinst)};
    let mut ret1 = QTextLine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn preeditAreaText<RetType, T: QTextLayout_preeditAreaText<RetType>>(&mut self, value: T) -> RetType {
    return value.preeditAreaText(self);
    // return 1;
  }
}

pub trait QTextLayout_preeditAreaText<RetType> {
  fn preeditAreaText(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  QString QTextLayout::preeditAreaText();
impl<'a> /*trait*/ QTextLayout_preeditAreaText<QString> for () {
  fn preeditAreaText(self, rsthis: &mut QTextLayout) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout15preeditAreaTextEv()};
    let mut ret = unsafe {_ZNK11QTextLayout15preeditAreaTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTextLayout::drawCursor(QPainter * p, const QPointF & pos, int cursorPosition, int width);
impl<'a> /*trait*/ QTextLayout_drawCursor<()> for (&'a mut QPainter, &'a  QPointF, i32, i32) {
  fn drawCursor(self, rsthis: &mut QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn endLayout<RetType, T: QTextLayout_endLayout<RetType>>(&mut self, value: T) -> RetType {
    return value.endLayout(self);
    // return 1;
  }
}

pub trait QTextLayout_endLayout<RetType> {
  fn endLayout(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  void QTextLayout::endLayout();
impl<'a> /*trait*/ QTextLayout_endLayout<()> for () {
  fn endLayout(self, rsthis: &mut QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout9endLayoutEv()};
     unsafe {_ZN11QTextLayout9endLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QTextLayout::NewQTextLayout();
impl<'a> /*trait*/ QTextLayout_NewQTextLayout for () {
  fn NewQTextLayout(self) -> QTextLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutC1Ev()};
    unsafe {_ZN11QTextLayoutC1Ev(qthis)};
    let rsthis = QTextLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn glyphRuns<RetType, T: QTextLayout_glyphRuns<RetType>>(&mut self, value: T) -> RetType {
    return value.glyphRuns(self);
    // return 1;
  }
}

pub trait QTextLayout_glyphRuns<RetType> {
  fn glyphRuns(self, rsthis: &mut QTextLayout) -> RetType;
}

// proto:  QList<QGlyphRun> QTextLayout::glyphRuns(int from, int length);
impl<'a> /*trait*/ QTextLayout_glyphRuns<()> for (i32, i32) {
  fn glyphRuns(self, rsthis: &mut QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout9glyphRunsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZNK11QTextLayout9glyphRunsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

