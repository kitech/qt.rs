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
use super::qtextblock::QTextBlock;
use super::qpainter::QPainter;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QTextLayout::setFont(const QFont & f);
  fn _ZN11QTextLayout7setFontERK5QFont(arg0: *const c_void) -> i32;
  // proto: void QTextLayout::setText(const QString & string);
  fn _ZN11QTextLayout7setTextERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QTextLayout::isValidCursorPosition(int pos);
  fn _ZNK11QTextLayout21isValidCursorPositionEi(arg0: c_int) -> i32;
  // proto: QRectF QTextLayout::boundingRect();
  fn _ZNK11QTextLayout12boundingRectEv() -> i32;
  // proto: void QTextLayout::setRawFont(const QRawFont & rawFont);
  fn _ZN11QTextLayout10setRawFontERK8QRawFont(arg0: *const c_void) -> i32;
  // proto: void QTextLayout::setTextOption(const QTextOption & option);
  fn _ZN11QTextLayout13setTextOptionERK11QTextOption(arg0: *const c_void) -> i32;
  // proto: void QTextLayout::NewQTextLayout(const QString & text, const QFont & font, QPaintDevice * paintdevice);
  fn _ZN11QTextLayoutC1ERK7QStringRK5QFontP12QPaintDevice(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: void QTextLayout::setPosition(const QPointF & p);
  fn _ZN11QTextLayout11setPositionERK7QPointF(arg0: *const c_void) -> i32;
  // proto: QTextLine QTextLayout::lineForTextPosition(int pos);
  fn _ZNK11QTextLayout19lineForTextPositionEi(arg0: c_int) -> i32;
  // proto: const QTextOption & QTextLayout::textOption();
  fn _ZNK11QTextLayout10textOptionEv() -> i32;
  // proto: QTextEngine * QTextLayout::engine();
  fn _ZNK11QTextLayout6engineEv() -> i32;
  // proto: int QTextLayout::preeditAreaPosition();
  fn _ZNK11QTextLayout19preeditAreaPositionEv() -> i32;
  // proto: void QTextLayout::clearAdditionalFormats();
  fn _ZN11QTextLayout22clearAdditionalFormatsEv() -> i32;
  // proto: int QTextLayout::leftCursorPosition(int oldPos);
  fn _ZNK11QTextLayout18leftCursorPositionEi(arg0: c_int) -> i32;
  // proto: int QTextLayout::lineCount();
  fn _ZNK11QTextLayout9lineCountEv() -> i32;
  // proto: void QTextLayout::FreeQTextLayout();
  fn _ZN11QTextLayoutD0Ev() -> i32;
  // proto: void QTextLayout::setCacheEnabled(bool enable);
  fn _ZN11QTextLayout15setCacheEnabledEb(arg0: int8_t) -> i32;
  // proto: QTextLine QTextLayout::lineAt(int i);
  fn _ZNK11QTextLayout6lineAtEi(arg0: c_int) -> i32;
  // proto: int QTextLayout::rightCursorPosition(int oldPos);
  fn _ZNK11QTextLayout19rightCursorPositionEi(arg0: c_int) -> i32;
  // proto: void QTextLayout::NewQTextLayout(const QTextBlock & b);
  fn _ZN11QTextLayoutC1ERK10QTextBlock(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: double QTextLayout::minimumWidth();
  fn _ZNK11QTextLayout12minimumWidthEv() -> i32;
  // proto: void QTextLayout::drawCursor(QPainter * p, const QPointF & pos, int cursorPosition);
  fn _ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFi(arg0: *mut c_void, arg1: *const c_void, arg2: c_int) -> i32;
  // proto: QFont QTextLayout::font();
  fn _ZNK11QTextLayout4fontEv() -> i32;
  // proto: void QTextLayout::setPreeditArea(int position, const QString & text);
  fn _ZN11QTextLayout14setPreeditAreaEiRK7QString(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QTextLayout::beginLayout();
  fn _ZN11QTextLayout11beginLayoutEv() -> i32;
  // proto: void QTextLayout::NewQTextLayout(const QString & text);
  fn _ZN11QTextLayoutC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QTextLayout::setFlags(int flags);
  fn _ZN11QTextLayout8setFlagsEi(arg0: c_int) -> i32;
  // proto: QPointF QTextLayout::position();
  fn _ZNK11QTextLayout8positionEv() -> i32;
  // proto: void QTextLayout::clearLayout();
  fn _ZN11QTextLayout11clearLayoutEv() -> i32;
  // proto: bool QTextLayout::cacheEnabled();
  fn _ZNK11QTextLayout12cacheEnabledEv() -> i32;
  // proto: double QTextLayout::maximumWidth();
  fn _ZNK11QTextLayout12maximumWidthEv() -> i32;
  // proto: QString QTextLayout::text();
  fn _ZNK11QTextLayout4textEv() -> i32;
  // proto: void QTextLayout::NewQTextLayout(const QTextLayout & );
  fn _ZN11QTextLayoutC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QTextLine QTextLayout::createLine();
  fn _ZN11QTextLayout10createLineEv() -> i32;
  // proto: QString QTextLayout::preeditAreaText();
  fn _ZNK11QTextLayout15preeditAreaTextEv() -> i32;
  // proto: void QTextLayout::drawCursor(QPainter * p, const QPointF & pos, int cursorPosition, int width);
  fn _ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFii(arg0: *mut c_void, arg1: *const c_void, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QTextLayout::endLayout();
  fn _ZN11QTextLayout9endLayoutEv() -> i32;
  // proto: void QTextLayout::NewQTextLayout();
  fn _ZN11QTextLayoutC1Ev(qthis: *mut c_void) -> i32;
  // proto: QList<QGlyphRun> QTextLayout::glyphRuns(int from, int length);
  fn _ZNK11QTextLayout9glyphRunsEii(arg0: c_int, arg1: c_int) -> i32;
}

// body block begin
// class sizeof(QTextLayout)=8
pub struct QTextLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextLayout {
  pub fn setFont<T: QTextLayout_setFont>(&mut self, value: T) -> i32 {
    value.setFont(self);
    return 1;
  }
}

pub trait QTextLayout_setFont {
  fn setFont(self, this: &mut QTextLayout) -> i32;
}

// proto: void QTextLayout::setFont(const QFont & f);
impl<'a> /*trait*/ QTextLayout_setFont for (&'a  QFont) {
  fn setFont(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QTextLayout7setFontERK5QFont(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn setText<T: QTextLayout_setText>(&mut self, value: T) -> i32 {
    value.setText(self);
    return 1;
  }
}

pub trait QTextLayout_setText {
  fn setText(self, this: &mut QTextLayout) -> i32;
}

// proto: void QTextLayout::setText(const QString & string);
impl<'a> /*trait*/ QTextLayout_setText for (&'a  QString) {
  fn setText(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout7setTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QTextLayout7setTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn isValidCursorPosition<T: QTextLayout_isValidCursorPosition>(&mut self, value: T) -> i32 {
    value.isValidCursorPosition(self);
    return 1;
  }
}

pub trait QTextLayout_isValidCursorPosition {
  fn isValidCursorPosition(self, this: &mut QTextLayout) -> i32;
}

// proto: bool QTextLayout::isValidCursorPosition(int pos);
impl<'a> /*trait*/ QTextLayout_isValidCursorPosition for (i32) {
  fn isValidCursorPosition(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout21isValidCursorPositionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QTextLayout21isValidCursorPositionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn boundingRect<T: QTextLayout_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QTextLayout_boundingRect {
  fn boundingRect(self, this: &mut QTextLayout) -> i32;
}

// proto: QRectF QTextLayout::boundingRect();
impl<'a> /*trait*/ QTextLayout_boundingRect for () {
  fn boundingRect(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout12boundingRectEv()};
    unsafe {_ZNK11QTextLayout12boundingRectEv()};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn setRawFont<T: QTextLayout_setRawFont>(&mut self, value: T) -> i32 {
    value.setRawFont(self);
    return 1;
  }
}

pub trait QTextLayout_setRawFont {
  fn setRawFont(self, this: &mut QTextLayout) -> i32;
}

// proto: void QTextLayout::setRawFont(const QRawFont & rawFont);
impl<'a> /*trait*/ QTextLayout_setRawFont for (&'a  QRawFont) {
  fn setRawFont(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout10setRawFontERK8QRawFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QTextLayout10setRawFontERK8QRawFont(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn setTextOption<T: QTextLayout_setTextOption>(&mut self, value: T) -> i32 {
    value.setTextOption(self);
    return 1;
  }
}

pub trait QTextLayout_setTextOption {
  fn setTextOption(self, this: &mut QTextLayout) -> i32;
}

// proto: void QTextLayout::setTextOption(const QTextOption & option);
impl<'a> /*trait*/ QTextLayout_setTextOption for (&'a  QTextOption) {
  fn setTextOption(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout13setTextOptionERK11QTextOption()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QTextLayout13setTextOptionERK11QTextOption(arg0)};
    return 1;
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
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN11QTextLayoutC1ERK7QStringRK5QFontP12QPaintDevice(qthis, arg0, arg1, arg2)};
    let rsthis = QTextLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn setPosition<T: QTextLayout_setPosition>(&mut self, value: T) -> i32 {
    value.setPosition(self);
    return 1;
  }
}

pub trait QTextLayout_setPosition {
  fn setPosition(self, this: &mut QTextLayout) -> i32;
}

// proto: void QTextLayout::setPosition(const QPointF & p);
impl<'a> /*trait*/ QTextLayout_setPosition for (&'a  QPointF) {
  fn setPosition(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout11setPositionERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QTextLayout11setPositionERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn lineForTextPosition<T: QTextLayout_lineForTextPosition>(&mut self, value: T) -> i32 {
    value.lineForTextPosition(self);
    return 1;
  }
}

pub trait QTextLayout_lineForTextPosition {
  fn lineForTextPosition(self, this: &mut QTextLayout) -> i32;
}

// proto: QTextLine QTextLayout::lineForTextPosition(int pos);
impl<'a> /*trait*/ QTextLayout_lineForTextPosition for (i32) {
  fn lineForTextPosition(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout19lineForTextPositionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QTextLayout19lineForTextPositionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn textOption<T: QTextLayout_textOption>(&mut self, value: T) -> i32 {
    value.textOption(self);
    return 1;
  }
}

pub trait QTextLayout_textOption {
  fn textOption(self, this: &mut QTextLayout) -> i32;
}

// proto: const QTextOption & QTextLayout::textOption();
impl<'a> /*trait*/ QTextLayout_textOption for () {
  fn textOption(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout10textOptionEv()};
    unsafe {_ZNK11QTextLayout10textOptionEv()};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn engine<T: QTextLayout_engine>(&mut self, value: T) -> i32 {
    value.engine(self);
    return 1;
  }
}

pub trait QTextLayout_engine {
  fn engine(self, this: &mut QTextLayout) -> i32;
}

// proto: QTextEngine * QTextLayout::engine();
impl<'a> /*trait*/ QTextLayout_engine for () {
  fn engine(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout6engineEv()};
    unsafe {_ZNK11QTextLayout6engineEv()};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn preeditAreaPosition<T: QTextLayout_preeditAreaPosition>(&mut self, value: T) -> i32 {
    value.preeditAreaPosition(self);
    return 1;
  }
}

pub trait QTextLayout_preeditAreaPosition {
  fn preeditAreaPosition(self, this: &mut QTextLayout) -> i32;
}

// proto: int QTextLayout::preeditAreaPosition();
impl<'a> /*trait*/ QTextLayout_preeditAreaPosition for () {
  fn preeditAreaPosition(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout19preeditAreaPositionEv()};
    unsafe {_ZNK11QTextLayout19preeditAreaPositionEv()};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn clearAdditionalFormats<T: QTextLayout_clearAdditionalFormats>(&mut self, value: T) -> i32 {
    value.clearAdditionalFormats(self);
    return 1;
  }
}

pub trait QTextLayout_clearAdditionalFormats {
  fn clearAdditionalFormats(self, this: &mut QTextLayout) -> i32;
}

// proto: void QTextLayout::clearAdditionalFormats();
impl<'a> /*trait*/ QTextLayout_clearAdditionalFormats for () {
  fn clearAdditionalFormats(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout22clearAdditionalFormatsEv()};
    unsafe {_ZN11QTextLayout22clearAdditionalFormatsEv()};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn leftCursorPosition<T: QTextLayout_leftCursorPosition>(&mut self, value: T) -> i32 {
    value.leftCursorPosition(self);
    return 1;
  }
}

pub trait QTextLayout_leftCursorPosition {
  fn leftCursorPosition(self, this: &mut QTextLayout) -> i32;
}

// proto: int QTextLayout::leftCursorPosition(int oldPos);
impl<'a> /*trait*/ QTextLayout_leftCursorPosition for (i32) {
  fn leftCursorPosition(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout18leftCursorPositionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QTextLayout18leftCursorPositionEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn lineCount<T: QTextLayout_lineCount>(&mut self, value: T) -> i32 {
    value.lineCount(self);
    return 1;
  }
}

pub trait QTextLayout_lineCount {
  fn lineCount(self, this: &mut QTextLayout) -> i32;
}

// proto: int QTextLayout::lineCount();
impl<'a> /*trait*/ QTextLayout_lineCount for () {
  fn lineCount(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout9lineCountEv()};
    unsafe {_ZNK11QTextLayout9lineCountEv()};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn FreeQTextLayout<T: QTextLayout_FreeQTextLayout>(&mut self, value: T) -> i32 {
    value.FreeQTextLayout(self);
    return 1;
  }
}

pub trait QTextLayout_FreeQTextLayout {
  fn FreeQTextLayout(self, this: &mut QTextLayout) -> i32;
}

// proto: void QTextLayout::FreeQTextLayout();
impl<'a> /*trait*/ QTextLayout_FreeQTextLayout for () {
  fn FreeQTextLayout(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutD0Ev()};
    unsafe {_ZN11QTextLayoutD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn setCacheEnabled<T: QTextLayout_setCacheEnabled>(&mut self, value: T) -> i32 {
    value.setCacheEnabled(self);
    return 1;
  }
}

pub trait QTextLayout_setCacheEnabled {
  fn setCacheEnabled(self, this: &mut QTextLayout) -> i32;
}

// proto: void QTextLayout::setCacheEnabled(bool enable);
impl<'a> /*trait*/ QTextLayout_setCacheEnabled for (i8) {
  fn setCacheEnabled(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout15setCacheEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QTextLayout15setCacheEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn lineAt<T: QTextLayout_lineAt>(&mut self, value: T) -> i32 {
    value.lineAt(self);
    return 1;
  }
}

pub trait QTextLayout_lineAt {
  fn lineAt(self, this: &mut QTextLayout) -> i32;
}

// proto: QTextLine QTextLayout::lineAt(int i);
impl<'a> /*trait*/ QTextLayout_lineAt for (i32) {
  fn lineAt(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout6lineAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QTextLayout6lineAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn rightCursorPosition<T: QTextLayout_rightCursorPosition>(&mut self, value: T) -> i32 {
    value.rightCursorPosition(self);
    return 1;
  }
}

pub trait QTextLayout_rightCursorPosition {
  fn rightCursorPosition(self, this: &mut QTextLayout) -> i32;
}

// proto: int QTextLayout::rightCursorPosition(int oldPos);
impl<'a> /*trait*/ QTextLayout_rightCursorPosition for (i32) {
  fn rightCursorPosition(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout19rightCursorPositionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QTextLayout19rightCursorPositionEi(arg0)};
    return 1;
  }
}

// proto: void QTextLayout::NewQTextLayout(const QTextBlock & b);
impl<'a> /*trait*/ QTextLayout_NewQTextLayout for (&'a  QTextBlock) {
  fn NewQTextLayout(self) -> QTextLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutC1ERK10QTextBlock()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QTextLayoutC1ERK10QTextBlock(qthis, arg0)};
    let rsthis = QTextLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn minimumWidth<T: QTextLayout_minimumWidth>(&mut self, value: T) -> i32 {
    value.minimumWidth(self);
    return 1;
  }
}

pub trait QTextLayout_minimumWidth {
  fn minimumWidth(self, this: &mut QTextLayout) -> i32;
}

// proto: double QTextLayout::minimumWidth();
impl<'a> /*trait*/ QTextLayout_minimumWidth for () {
  fn minimumWidth(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout12minimumWidthEv()};
    unsafe {_ZNK11QTextLayout12minimumWidthEv()};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn drawCursor<T: QTextLayout_drawCursor>(&mut self, value: T) -> i32 {
    value.drawCursor(self);
    return 1;
  }
}

pub trait QTextLayout_drawCursor {
  fn drawCursor(self, this: &mut QTextLayout) -> i32;
}

// proto: void QTextLayout::drawCursor(QPainter * p, const QPointF & pos, int cursorPosition);
impl<'a> /*trait*/ QTextLayout_drawCursor for (&'a mut QPainter, &'a  QPointF, i32) {
  fn drawCursor(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFi(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn font<T: QTextLayout_font>(&mut self, value: T) -> i32 {
    value.font(self);
    return 1;
  }
}

pub trait QTextLayout_font {
  fn font(self, this: &mut QTextLayout) -> i32;
}

// proto: QFont QTextLayout::font();
impl<'a> /*trait*/ QTextLayout_font for () {
  fn font(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout4fontEv()};
    unsafe {_ZNK11QTextLayout4fontEv()};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn setPreeditArea<T: QTextLayout_setPreeditArea>(&mut self, value: T) -> i32 {
    value.setPreeditArea(self);
    return 1;
  }
}

pub trait QTextLayout_setPreeditArea {
  fn setPreeditArea(self, this: &mut QTextLayout) -> i32;
}

// proto: void QTextLayout::setPreeditArea(int position, const QString & text);
impl<'a> /*trait*/ QTextLayout_setPreeditArea for (i32, &'a  QString) {
  fn setPreeditArea(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout14setPreeditAreaEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN11QTextLayout14setPreeditAreaEiRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn beginLayout<T: QTextLayout_beginLayout>(&mut self, value: T) -> i32 {
    value.beginLayout(self);
    return 1;
  }
}

pub trait QTextLayout_beginLayout {
  fn beginLayout(self, this: &mut QTextLayout) -> i32;
}

// proto: void QTextLayout::beginLayout();
impl<'a> /*trait*/ QTextLayout_beginLayout for () {
  fn beginLayout(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout11beginLayoutEv()};
    unsafe {_ZN11QTextLayout11beginLayoutEv()};
    return 1;
  }
}

// proto: void QTextLayout::NewQTextLayout(const QString & text);
impl<'a> /*trait*/ QTextLayout_NewQTextLayout for (&'a  QString) {
  fn NewQTextLayout(self) -> QTextLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutC1ERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QTextLayoutC1ERK7QString(qthis, arg0)};
    let rsthis = QTextLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn setFlags<T: QTextLayout_setFlags>(&mut self, value: T) -> i32 {
    value.setFlags(self);
    return 1;
  }
}

pub trait QTextLayout_setFlags {
  fn setFlags(self, this: &mut QTextLayout) -> i32;
}

// proto: void QTextLayout::setFlags(int flags);
impl<'a> /*trait*/ QTextLayout_setFlags for (i32) {
  fn setFlags(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout8setFlagsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QTextLayout8setFlagsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn position<T: QTextLayout_position>(&mut self, value: T) -> i32 {
    value.position(self);
    return 1;
  }
}

pub trait QTextLayout_position {
  fn position(self, this: &mut QTextLayout) -> i32;
}

// proto: QPointF QTextLayout::position();
impl<'a> /*trait*/ QTextLayout_position for () {
  fn position(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout8positionEv()};
    unsafe {_ZNK11QTextLayout8positionEv()};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn clearLayout<T: QTextLayout_clearLayout>(&mut self, value: T) -> i32 {
    value.clearLayout(self);
    return 1;
  }
}

pub trait QTextLayout_clearLayout {
  fn clearLayout(self, this: &mut QTextLayout) -> i32;
}

// proto: void QTextLayout::clearLayout();
impl<'a> /*trait*/ QTextLayout_clearLayout for () {
  fn clearLayout(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout11clearLayoutEv()};
    unsafe {_ZN11QTextLayout11clearLayoutEv()};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn cacheEnabled<T: QTextLayout_cacheEnabled>(&mut self, value: T) -> i32 {
    value.cacheEnabled(self);
    return 1;
  }
}

pub trait QTextLayout_cacheEnabled {
  fn cacheEnabled(self, this: &mut QTextLayout) -> i32;
}

// proto: bool QTextLayout::cacheEnabled();
impl<'a> /*trait*/ QTextLayout_cacheEnabled for () {
  fn cacheEnabled(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout12cacheEnabledEv()};
    unsafe {_ZNK11QTextLayout12cacheEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn maximumWidth<T: QTextLayout_maximumWidth>(&mut self, value: T) -> i32 {
    value.maximumWidth(self);
    return 1;
  }
}

pub trait QTextLayout_maximumWidth {
  fn maximumWidth(self, this: &mut QTextLayout) -> i32;
}

// proto: double QTextLayout::maximumWidth();
impl<'a> /*trait*/ QTextLayout_maximumWidth for () {
  fn maximumWidth(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout12maximumWidthEv()};
    unsafe {_ZNK11QTextLayout12maximumWidthEv()};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn text<T: QTextLayout_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QTextLayout_text {
  fn text(self, this: &mut QTextLayout) -> i32;
}

// proto: QString QTextLayout::text();
impl<'a> /*trait*/ QTextLayout_text for () {
  fn text(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout4textEv()};
    unsafe {_ZNK11QTextLayout4textEv()};
    return 1;
  }
}

// proto: void QTextLayout::NewQTextLayout(const QTextLayout & );
impl<'a> /*trait*/ QTextLayout_NewQTextLayout for (&'a  QTextLayout) {
  fn NewQTextLayout(self) -> QTextLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QTextLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QTextLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn createLine<T: QTextLayout_createLine>(&mut self, value: T) -> i32 {
    value.createLine(self);
    return 1;
  }
}

pub trait QTextLayout_createLine {
  fn createLine(self, this: &mut QTextLayout) -> i32;
}

// proto: QTextLine QTextLayout::createLine();
impl<'a> /*trait*/ QTextLayout_createLine for () {
  fn createLine(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout10createLineEv()};
    unsafe {_ZN11QTextLayout10createLineEv()};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn preeditAreaText<T: QTextLayout_preeditAreaText>(&mut self, value: T) -> i32 {
    value.preeditAreaText(self);
    return 1;
  }
}

pub trait QTextLayout_preeditAreaText {
  fn preeditAreaText(self, this: &mut QTextLayout) -> i32;
}

// proto: QString QTextLayout::preeditAreaText();
impl<'a> /*trait*/ QTextLayout_preeditAreaText for () {
  fn preeditAreaText(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout15preeditAreaTextEv()};
    unsafe {_ZNK11QTextLayout15preeditAreaTextEv()};
    return 1;
  }
}

// proto: void QTextLayout::drawCursor(QPainter * p, const QPointF & pos, int cursorPosition, int width);
impl<'a> /*trait*/ QTextLayout_drawCursor for (&'a mut QPainter, &'a  QPointF, i32, i32) {
  fn drawCursor(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn endLayout<T: QTextLayout_endLayout>(&mut self, value: T) -> i32 {
    value.endLayout(self);
    return 1;
  }
}

pub trait QTextLayout_endLayout {
  fn endLayout(self, this: &mut QTextLayout) -> i32;
}

// proto: void QTextLayout::endLayout();
impl<'a> /*trait*/ QTextLayout_endLayout for () {
  fn endLayout(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout9endLayoutEv()};
    unsafe {_ZN11QTextLayout9endLayoutEv()};
    return 1;
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
  pub fn glyphRuns<T: QTextLayout_glyphRuns>(&mut self, value: T) -> i32 {
    value.glyphRuns(self);
    return 1;
  }
}

pub trait QTextLayout_glyphRuns {
  fn glyphRuns(self, this: &mut QTextLayout) -> i32;
}

// proto: QList<QGlyphRun> QTextLayout::glyphRuns(int from, int length);
impl<'a> /*trait*/ QTextLayout_glyphRuns for (i32, i32) {
  fn glyphRuns(self, this: &mut QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout9glyphRunsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK11QTextLayout9glyphRunsEii(arg0, arg1)};
    return 1;
  }
}

