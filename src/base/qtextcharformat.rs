// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qfont::QFont;
use super::qcolor::QColor;
use super::qstring::QString;
use super::qpen::QPen;
use super::qtextformat::QTextFormat;
use super::qstringlist::QStringList;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QTextCharFormat::setFontLetterSpacing(qreal spacing);
  fn _ZN15QTextCharFormat20setFontLetterSpacingEd(arg0: c_double) -> i32;
  // proto: bool QTextCharFormat::isAnchor();
  fn _ZNK15QTextCharFormat8isAnchorEv() -> i32;
  // proto: void QTextCharFormat::setFont(const QFont & font);
  fn _ZN15QTextCharFormat7setFontERK5QFont(arg0: *const c_void) -> i32;
  // proto: bool QTextCharFormat::fontOverline();
  fn _ZNK15QTextCharFormat12fontOverlineEv() -> i32;
  // proto: QFont QTextCharFormat::font();
  fn _ZNK15QTextCharFormat4fontEv() -> i32;
  // proto: QString QTextCharFormat::fontFamily();
  fn _ZNK15QTextCharFormat10fontFamilyEv() -> i32;
  // proto: bool QTextCharFormat::fontStrikeOut();
  fn _ZNK15QTextCharFormat13fontStrikeOutEv() -> i32;
  // proto: void QTextCharFormat::setFontPointSize(qreal size);
  fn _ZN15QTextCharFormat16setFontPointSizeEd(arg0: c_double) -> i32;
  // proto: void QTextCharFormat::setUnderlineColor(const QColor & color);
  fn _ZN15QTextCharFormat17setUnderlineColorERK6QColor(arg0: *const c_void) -> i32;
  // proto: int QTextCharFormat::tableCellRowSpan();
  fn _ZNK15QTextCharFormat16tableCellRowSpanEv() -> i32;
  // proto: void QTextCharFormat::setFontUnderline(bool underline);
  fn _ZN15QTextCharFormat16setFontUnderlineEb(arg0: int8_t) -> i32;
  // proto: bool QTextCharFormat::isValid();
  fn _ZNK15QTextCharFormat7isValidEv() -> i32;
  // proto: bool QTextCharFormat::fontItalic();
  fn _ZNK15QTextCharFormat10fontItalicEv() -> i32;
  // proto: void QTextCharFormat::setToolTip(const QString & tip);
  fn _ZN15QTextCharFormat10setToolTipERK7QString(arg0: *const c_void) -> i32;
  // proto: void QTextCharFormat::setTextOutline(const QPen & pen);
  fn _ZN15QTextCharFormat14setTextOutlineERK4QPen(arg0: *const c_void) -> i32;
  // proto: void QTextCharFormat::setTableCellRowSpan(int tableCellRowSpan);
  fn _ZN15QTextCharFormat19setTableCellRowSpanEi(arg0: c_int) -> i32;
  // proto: void QTextCharFormat::setAnchor(bool anchor);
  fn _ZN15QTextCharFormat9setAnchorEb(arg0: int8_t) -> i32;
  // proto: double QTextCharFormat::fontPointSize();
  fn _ZNK15QTextCharFormat13fontPointSizeEv() -> i32;
  // proto: void QTextCharFormat::NewQTextCharFormat(const QTextFormat & fmt);
  fn _ZN15QTextCharFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QTextCharFormat::setFontStrikeOut(bool strikeOut);
  fn _ZN15QTextCharFormat16setFontStrikeOutEb(arg0: int8_t) -> i32;
  // proto: double QTextCharFormat::fontWordSpacing();
  fn _ZNK15QTextCharFormat15fontWordSpacingEv() -> i32;
  // proto: QString QTextCharFormat::toolTip();
  fn _ZNK15QTextCharFormat7toolTipEv() -> i32;
  // proto: void QTextCharFormat::setAnchorNames(const QStringList & names);
  fn _ZN15QTextCharFormat14setAnchorNamesERK11QStringList(arg0: *const c_void) -> i32;
  // proto: QStringList QTextCharFormat::anchorNames();
  fn _ZNK15QTextCharFormat11anchorNamesEv() -> i32;
  // proto: void QTextCharFormat::setFontFixedPitch(bool fixedPitch);
  fn _ZN15QTextCharFormat17setFontFixedPitchEb(arg0: int8_t) -> i32;
  // proto: void QTextCharFormat::setFontItalic(bool italic);
  fn _ZN15QTextCharFormat13setFontItalicEb(arg0: int8_t) -> i32;
  // proto: void QTextCharFormat::setFontFamily(const QString & family);
  fn _ZN15QTextCharFormat13setFontFamilyERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QTextCharFormat::fontFixedPitch();
  fn _ZNK15QTextCharFormat14fontFixedPitchEv() -> i32;
  // proto: void QTextCharFormat::setAnchorHref(const QString & value);
  fn _ZN15QTextCharFormat13setAnchorHrefERK7QString(arg0: *const c_void) -> i32;
  // proto: int QTextCharFormat::fontStretch();
  fn _ZNK15QTextCharFormat11fontStretchEv() -> i32;
  // proto: void QTextCharFormat::setFontKerning(bool enable);
  fn _ZN15QTextCharFormat14setFontKerningEb(arg0: int8_t) -> i32;
  // proto: int QTextCharFormat::tableCellColumnSpan();
  fn _ZNK15QTextCharFormat19tableCellColumnSpanEv() -> i32;
  // proto: void QTextCharFormat::NewQTextCharFormat();
  fn _ZN15QTextCharFormatC1Ev(qthis: *mut c_void) -> i32;
  // proto: double QTextCharFormat::fontLetterSpacing();
  fn _ZNK15QTextCharFormat17fontLetterSpacingEv() -> i32;
  // proto: QString QTextCharFormat::anchorHref();
  fn _ZNK15QTextCharFormat10anchorHrefEv() -> i32;
  // proto: QString QTextCharFormat::anchorName();
  fn _ZNK15QTextCharFormat10anchorNameEv() -> i32;
  // proto: void QTextCharFormat::setFontStretch(int factor);
  fn _ZN15QTextCharFormat14setFontStretchEi(arg0: c_int) -> i32;
  // proto: void QTextCharFormat::setAnchorName(const QString & name);
  fn _ZN15QTextCharFormat13setAnchorNameERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QTextCharFormat::fontKerning();
  fn _ZNK15QTextCharFormat11fontKerningEv() -> i32;
  // proto: void QTextCharFormat::setFontWeight(int weight);
  fn _ZN15QTextCharFormat13setFontWeightEi(arg0: c_int) -> i32;
  // proto: bool QTextCharFormat::fontUnderline();
  fn _ZNK15QTextCharFormat13fontUnderlineEv() -> i32;
  // proto: void QTextCharFormat::setFontWordSpacing(qreal spacing);
  fn _ZN15QTextCharFormat18setFontWordSpacingEd(arg0: c_double) -> i32;
  // proto: QColor QTextCharFormat::underlineColor();
  fn _ZNK15QTextCharFormat14underlineColorEv() -> i32;
  // proto: int QTextCharFormat::fontWeight();
  fn _ZNK15QTextCharFormat10fontWeightEv() -> i32;
  // proto: void QTextCharFormat::setFontOverline(bool overline);
  fn _ZN15QTextCharFormat15setFontOverlineEb(arg0: int8_t) -> i32;
  // proto: void QTextCharFormat::setTableCellColumnSpan(int tableCellColumnSpan);
  fn _ZN15QTextCharFormat22setTableCellColumnSpanEi(arg0: c_int) -> i32;
  // proto: QPen QTextCharFormat::textOutline();
  fn _ZNK15QTextCharFormat11textOutlineEv() -> i32;
}

// body block begin
// class sizeof(QTextCharFormat)=1
pub struct QTextCharFormat {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontLetterSpacing<T: QTextCharFormat_setFontLetterSpacing>(&mut self, value: T) -> i32 {
    value.setFontLetterSpacing(self);
    return 1;
  }
}

pub trait QTextCharFormat_setFontLetterSpacing {
  fn setFontLetterSpacing(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setFontLetterSpacing(qreal spacing);
impl<'a> /*trait*/ QTextCharFormat_setFontLetterSpacing for (f64) {
  fn setFontLetterSpacing(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat20setFontLetterSpacingEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN15QTextCharFormat20setFontLetterSpacingEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn isAnchor<T: QTextCharFormat_isAnchor>(&mut self, value: T) -> i32 {
    value.isAnchor(self);
    return 1;
  }
}

pub trait QTextCharFormat_isAnchor {
  fn isAnchor(self, this: &mut QTextCharFormat) -> i32;
}

// proto: bool QTextCharFormat::isAnchor();
impl<'a> /*trait*/ QTextCharFormat_isAnchor for () {
  fn isAnchor(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat8isAnchorEv()};
    unsafe {_ZNK15QTextCharFormat8isAnchorEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFont<T: QTextCharFormat_setFont>(&mut self, value: T) -> i32 {
    value.setFont(self);
    return 1;
  }
}

pub trait QTextCharFormat_setFont {
  fn setFont(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setFont(const QFont & font);
impl<'a> /*trait*/ QTextCharFormat_setFont for (&'a  QFont) {
  fn setFont(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QTextCharFormat7setFontERK5QFont(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontOverline<T: QTextCharFormat_fontOverline>(&mut self, value: T) -> i32 {
    value.fontOverline(self);
    return 1;
  }
}

pub trait QTextCharFormat_fontOverline {
  fn fontOverline(self, this: &mut QTextCharFormat) -> i32;
}

// proto: bool QTextCharFormat::fontOverline();
impl<'a> /*trait*/ QTextCharFormat_fontOverline for () {
  fn fontOverline(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat12fontOverlineEv()};
    unsafe {_ZNK15QTextCharFormat12fontOverlineEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn font<T: QTextCharFormat_font>(&mut self, value: T) -> i32 {
    value.font(self);
    return 1;
  }
}

pub trait QTextCharFormat_font {
  fn font(self, this: &mut QTextCharFormat) -> i32;
}

// proto: QFont QTextCharFormat::font();
impl<'a> /*trait*/ QTextCharFormat_font for () {
  fn font(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat4fontEv()};
    unsafe {_ZNK15QTextCharFormat4fontEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontFamily<T: QTextCharFormat_fontFamily>(&mut self, value: T) -> i32 {
    value.fontFamily(self);
    return 1;
  }
}

pub trait QTextCharFormat_fontFamily {
  fn fontFamily(self, this: &mut QTextCharFormat) -> i32;
}

// proto: QString QTextCharFormat::fontFamily();
impl<'a> /*trait*/ QTextCharFormat_fontFamily for () {
  fn fontFamily(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10fontFamilyEv()};
    unsafe {_ZNK15QTextCharFormat10fontFamilyEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontStrikeOut<T: QTextCharFormat_fontStrikeOut>(&mut self, value: T) -> i32 {
    value.fontStrikeOut(self);
    return 1;
  }
}

pub trait QTextCharFormat_fontStrikeOut {
  fn fontStrikeOut(self, this: &mut QTextCharFormat) -> i32;
}

// proto: bool QTextCharFormat::fontStrikeOut();
impl<'a> /*trait*/ QTextCharFormat_fontStrikeOut for () {
  fn fontStrikeOut(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat13fontStrikeOutEv()};
    unsafe {_ZNK15QTextCharFormat13fontStrikeOutEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontPointSize<T: QTextCharFormat_setFontPointSize>(&mut self, value: T) -> i32 {
    value.setFontPointSize(self);
    return 1;
  }
}

pub trait QTextCharFormat_setFontPointSize {
  fn setFontPointSize(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setFontPointSize(qreal size);
impl<'a> /*trait*/ QTextCharFormat_setFontPointSize for (f64) {
  fn setFontPointSize(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat16setFontPointSizeEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN15QTextCharFormat16setFontPointSizeEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setUnderlineColor<T: QTextCharFormat_setUnderlineColor>(&mut self, value: T) -> i32 {
    value.setUnderlineColor(self);
    return 1;
  }
}

pub trait QTextCharFormat_setUnderlineColor {
  fn setUnderlineColor(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setUnderlineColor(const QColor & color);
impl<'a> /*trait*/ QTextCharFormat_setUnderlineColor for (&'a  QColor) {
  fn setUnderlineColor(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat17setUnderlineColorERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QTextCharFormat17setUnderlineColorERK6QColor(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn tableCellRowSpan<T: QTextCharFormat_tableCellRowSpan>(&mut self, value: T) -> i32 {
    value.tableCellRowSpan(self);
    return 1;
  }
}

pub trait QTextCharFormat_tableCellRowSpan {
  fn tableCellRowSpan(self, this: &mut QTextCharFormat) -> i32;
}

// proto: int QTextCharFormat::tableCellRowSpan();
impl<'a> /*trait*/ QTextCharFormat_tableCellRowSpan for () {
  fn tableCellRowSpan(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat16tableCellRowSpanEv()};
    unsafe {_ZNK15QTextCharFormat16tableCellRowSpanEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontUnderline<T: QTextCharFormat_setFontUnderline>(&mut self, value: T) -> i32 {
    value.setFontUnderline(self);
    return 1;
  }
}

pub trait QTextCharFormat_setFontUnderline {
  fn setFontUnderline(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setFontUnderline(bool underline);
impl<'a> /*trait*/ QTextCharFormat_setFontUnderline for (i8) {
  fn setFontUnderline(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat16setFontUnderlineEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QTextCharFormat16setFontUnderlineEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn isValid<T: QTextCharFormat_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QTextCharFormat_isValid {
  fn isValid(self, this: &mut QTextCharFormat) -> i32;
}

// proto: bool QTextCharFormat::isValid();
impl<'a> /*trait*/ QTextCharFormat_isValid for () {
  fn isValid(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat7isValidEv()};
    unsafe {_ZNK15QTextCharFormat7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontItalic<T: QTextCharFormat_fontItalic>(&mut self, value: T) -> i32 {
    value.fontItalic(self);
    return 1;
  }
}

pub trait QTextCharFormat_fontItalic {
  fn fontItalic(self, this: &mut QTextCharFormat) -> i32;
}

// proto: bool QTextCharFormat::fontItalic();
impl<'a> /*trait*/ QTextCharFormat_fontItalic for () {
  fn fontItalic(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10fontItalicEv()};
    unsafe {_ZNK15QTextCharFormat10fontItalicEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setToolTip<T: QTextCharFormat_setToolTip>(&mut self, value: T) -> i32 {
    value.setToolTip(self);
    return 1;
  }
}

pub trait QTextCharFormat_setToolTip {
  fn setToolTip(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setToolTip(const QString & tip);
impl<'a> /*trait*/ QTextCharFormat_setToolTip for (&'a  QString) {
  fn setToolTip(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QTextCharFormat10setToolTipERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setTextOutline<T: QTextCharFormat_setTextOutline>(&mut self, value: T) -> i32 {
    value.setTextOutline(self);
    return 1;
  }
}

pub trait QTextCharFormat_setTextOutline {
  fn setTextOutline(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setTextOutline(const QPen & pen);
impl<'a> /*trait*/ QTextCharFormat_setTextOutline for (&'a  QPen) {
  fn setTextOutline(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat14setTextOutlineERK4QPen()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QTextCharFormat14setTextOutlineERK4QPen(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setTableCellRowSpan<T: QTextCharFormat_setTableCellRowSpan>(&mut self, value: T) -> i32 {
    value.setTableCellRowSpan(self);
    return 1;
  }
}

pub trait QTextCharFormat_setTableCellRowSpan {
  fn setTableCellRowSpan(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setTableCellRowSpan(int tableCellRowSpan);
impl<'a> /*trait*/ QTextCharFormat_setTableCellRowSpan for (i32) {
  fn setTableCellRowSpan(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat19setTableCellRowSpanEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QTextCharFormat19setTableCellRowSpanEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setAnchor<T: QTextCharFormat_setAnchor>(&mut self, value: T) -> i32 {
    value.setAnchor(self);
    return 1;
  }
}

pub trait QTextCharFormat_setAnchor {
  fn setAnchor(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setAnchor(bool anchor);
impl<'a> /*trait*/ QTextCharFormat_setAnchor for (i8) {
  fn setAnchor(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat9setAnchorEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QTextCharFormat9setAnchorEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontPointSize<T: QTextCharFormat_fontPointSize>(&mut self, value: T) -> i32 {
    value.fontPointSize(self);
    return 1;
  }
}

pub trait QTextCharFormat_fontPointSize {
  fn fontPointSize(self, this: &mut QTextCharFormat) -> i32;
}

// proto: double QTextCharFormat::fontPointSize();
impl<'a> /*trait*/ QTextCharFormat_fontPointSize for () {
  fn fontPointSize(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat13fontPointSizeEv()};
    unsafe {_ZNK15QTextCharFormat13fontPointSizeEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn NewQTextCharFormat<T: QTextCharFormat_NewQTextCharFormat>(value: T) -> QTextCharFormat {
    let rsthis = value.NewQTextCharFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QTextCharFormat_NewQTextCharFormat {
  fn NewQTextCharFormat(self) -> QTextCharFormat;
}

// proto: void QTextCharFormat::NewQTextCharFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextCharFormat_NewQTextCharFormat for (&'a  QTextFormat) {
  fn NewQTextCharFormat(self) -> QTextCharFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormatC1ERK11QTextFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QTextCharFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextCharFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontStrikeOut<T: QTextCharFormat_setFontStrikeOut>(&mut self, value: T) -> i32 {
    value.setFontStrikeOut(self);
    return 1;
  }
}

pub trait QTextCharFormat_setFontStrikeOut {
  fn setFontStrikeOut(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setFontStrikeOut(bool strikeOut);
impl<'a> /*trait*/ QTextCharFormat_setFontStrikeOut for (i8) {
  fn setFontStrikeOut(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat16setFontStrikeOutEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QTextCharFormat16setFontStrikeOutEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontWordSpacing<T: QTextCharFormat_fontWordSpacing>(&mut self, value: T) -> i32 {
    value.fontWordSpacing(self);
    return 1;
  }
}

pub trait QTextCharFormat_fontWordSpacing {
  fn fontWordSpacing(self, this: &mut QTextCharFormat) -> i32;
}

// proto: double QTextCharFormat::fontWordSpacing();
impl<'a> /*trait*/ QTextCharFormat_fontWordSpacing for () {
  fn fontWordSpacing(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat15fontWordSpacingEv()};
    unsafe {_ZNK15QTextCharFormat15fontWordSpacingEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn toolTip<T: QTextCharFormat_toolTip>(&mut self, value: T) -> i32 {
    value.toolTip(self);
    return 1;
  }
}

pub trait QTextCharFormat_toolTip {
  fn toolTip(self, this: &mut QTextCharFormat) -> i32;
}

// proto: QString QTextCharFormat::toolTip();
impl<'a> /*trait*/ QTextCharFormat_toolTip for () {
  fn toolTip(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat7toolTipEv()};
    unsafe {_ZNK15QTextCharFormat7toolTipEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setAnchorNames<T: QTextCharFormat_setAnchorNames>(&mut self, value: T) -> i32 {
    value.setAnchorNames(self);
    return 1;
  }
}

pub trait QTextCharFormat_setAnchorNames {
  fn setAnchorNames(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setAnchorNames(const QStringList & names);
impl<'a> /*trait*/ QTextCharFormat_setAnchorNames for (&'a  QStringList) {
  fn setAnchorNames(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat14setAnchorNamesERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QTextCharFormat14setAnchorNamesERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn anchorNames<T: QTextCharFormat_anchorNames>(&mut self, value: T) -> i32 {
    value.anchorNames(self);
    return 1;
  }
}

pub trait QTextCharFormat_anchorNames {
  fn anchorNames(self, this: &mut QTextCharFormat) -> i32;
}

// proto: QStringList QTextCharFormat::anchorNames();
impl<'a> /*trait*/ QTextCharFormat_anchorNames for () {
  fn anchorNames(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat11anchorNamesEv()};
    unsafe {_ZNK15QTextCharFormat11anchorNamesEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontFixedPitch<T: QTextCharFormat_setFontFixedPitch>(&mut self, value: T) -> i32 {
    value.setFontFixedPitch(self);
    return 1;
  }
}

pub trait QTextCharFormat_setFontFixedPitch {
  fn setFontFixedPitch(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setFontFixedPitch(bool fixedPitch);
impl<'a> /*trait*/ QTextCharFormat_setFontFixedPitch for (i8) {
  fn setFontFixedPitch(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat17setFontFixedPitchEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QTextCharFormat17setFontFixedPitchEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontItalic<T: QTextCharFormat_setFontItalic>(&mut self, value: T) -> i32 {
    value.setFontItalic(self);
    return 1;
  }
}

pub trait QTextCharFormat_setFontItalic {
  fn setFontItalic(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setFontItalic(bool italic);
impl<'a> /*trait*/ QTextCharFormat_setFontItalic for (i8) {
  fn setFontItalic(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setFontItalicEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QTextCharFormat13setFontItalicEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontFamily<T: QTextCharFormat_setFontFamily>(&mut self, value: T) -> i32 {
    value.setFontFamily(self);
    return 1;
  }
}

pub trait QTextCharFormat_setFontFamily {
  fn setFontFamily(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setFontFamily(const QString & family);
impl<'a> /*trait*/ QTextCharFormat_setFontFamily for (&'a  QString) {
  fn setFontFamily(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setFontFamilyERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QTextCharFormat13setFontFamilyERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontFixedPitch<T: QTextCharFormat_fontFixedPitch>(&mut self, value: T) -> i32 {
    value.fontFixedPitch(self);
    return 1;
  }
}

pub trait QTextCharFormat_fontFixedPitch {
  fn fontFixedPitch(self, this: &mut QTextCharFormat) -> i32;
}

// proto: bool QTextCharFormat::fontFixedPitch();
impl<'a> /*trait*/ QTextCharFormat_fontFixedPitch for () {
  fn fontFixedPitch(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat14fontFixedPitchEv()};
    unsafe {_ZNK15QTextCharFormat14fontFixedPitchEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setAnchorHref<T: QTextCharFormat_setAnchorHref>(&mut self, value: T) -> i32 {
    value.setAnchorHref(self);
    return 1;
  }
}

pub trait QTextCharFormat_setAnchorHref {
  fn setAnchorHref(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setAnchorHref(const QString & value);
impl<'a> /*trait*/ QTextCharFormat_setAnchorHref for (&'a  QString) {
  fn setAnchorHref(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setAnchorHrefERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QTextCharFormat13setAnchorHrefERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontStretch<T: QTextCharFormat_fontStretch>(&mut self, value: T) -> i32 {
    value.fontStretch(self);
    return 1;
  }
}

pub trait QTextCharFormat_fontStretch {
  fn fontStretch(self, this: &mut QTextCharFormat) -> i32;
}

// proto: int QTextCharFormat::fontStretch();
impl<'a> /*trait*/ QTextCharFormat_fontStretch for () {
  fn fontStretch(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat11fontStretchEv()};
    unsafe {_ZNK15QTextCharFormat11fontStretchEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontKerning<T: QTextCharFormat_setFontKerning>(&mut self, value: T) -> i32 {
    value.setFontKerning(self);
    return 1;
  }
}

pub trait QTextCharFormat_setFontKerning {
  fn setFontKerning(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setFontKerning(bool enable);
impl<'a> /*trait*/ QTextCharFormat_setFontKerning for (i8) {
  fn setFontKerning(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat14setFontKerningEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QTextCharFormat14setFontKerningEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn tableCellColumnSpan<T: QTextCharFormat_tableCellColumnSpan>(&mut self, value: T) -> i32 {
    value.tableCellColumnSpan(self);
    return 1;
  }
}

pub trait QTextCharFormat_tableCellColumnSpan {
  fn tableCellColumnSpan(self, this: &mut QTextCharFormat) -> i32;
}

// proto: int QTextCharFormat::tableCellColumnSpan();
impl<'a> /*trait*/ QTextCharFormat_tableCellColumnSpan for () {
  fn tableCellColumnSpan(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat19tableCellColumnSpanEv()};
    unsafe {_ZNK15QTextCharFormat19tableCellColumnSpanEv()};
    return 1;
  }
}

// proto: void QTextCharFormat::NewQTextCharFormat();
impl<'a> /*trait*/ QTextCharFormat_NewQTextCharFormat for () {
  fn NewQTextCharFormat(self) -> QTextCharFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormatC1Ev()};
    unsafe {_ZN15QTextCharFormatC1Ev(qthis)};
    let rsthis = QTextCharFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontLetterSpacing<T: QTextCharFormat_fontLetterSpacing>(&mut self, value: T) -> i32 {
    value.fontLetterSpacing(self);
    return 1;
  }
}

pub trait QTextCharFormat_fontLetterSpacing {
  fn fontLetterSpacing(self, this: &mut QTextCharFormat) -> i32;
}

// proto: double QTextCharFormat::fontLetterSpacing();
impl<'a> /*trait*/ QTextCharFormat_fontLetterSpacing for () {
  fn fontLetterSpacing(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat17fontLetterSpacingEv()};
    unsafe {_ZNK15QTextCharFormat17fontLetterSpacingEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn anchorHref<T: QTextCharFormat_anchorHref>(&mut self, value: T) -> i32 {
    value.anchorHref(self);
    return 1;
  }
}

pub trait QTextCharFormat_anchorHref {
  fn anchorHref(self, this: &mut QTextCharFormat) -> i32;
}

// proto: QString QTextCharFormat::anchorHref();
impl<'a> /*trait*/ QTextCharFormat_anchorHref for () {
  fn anchorHref(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10anchorHrefEv()};
    unsafe {_ZNK15QTextCharFormat10anchorHrefEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn anchorName<T: QTextCharFormat_anchorName>(&mut self, value: T) -> i32 {
    value.anchorName(self);
    return 1;
  }
}

pub trait QTextCharFormat_anchorName {
  fn anchorName(self, this: &mut QTextCharFormat) -> i32;
}

// proto: QString QTextCharFormat::anchorName();
impl<'a> /*trait*/ QTextCharFormat_anchorName for () {
  fn anchorName(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10anchorNameEv()};
    unsafe {_ZNK15QTextCharFormat10anchorNameEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontStretch<T: QTextCharFormat_setFontStretch>(&mut self, value: T) -> i32 {
    value.setFontStretch(self);
    return 1;
  }
}

pub trait QTextCharFormat_setFontStretch {
  fn setFontStretch(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setFontStretch(int factor);
impl<'a> /*trait*/ QTextCharFormat_setFontStretch for (i32) {
  fn setFontStretch(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat14setFontStretchEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QTextCharFormat14setFontStretchEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setAnchorName<T: QTextCharFormat_setAnchorName>(&mut self, value: T) -> i32 {
    value.setAnchorName(self);
    return 1;
  }
}

pub trait QTextCharFormat_setAnchorName {
  fn setAnchorName(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setAnchorName(const QString & name);
impl<'a> /*trait*/ QTextCharFormat_setAnchorName for (&'a  QString) {
  fn setAnchorName(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setAnchorNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QTextCharFormat13setAnchorNameERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontKerning<T: QTextCharFormat_fontKerning>(&mut self, value: T) -> i32 {
    value.fontKerning(self);
    return 1;
  }
}

pub trait QTextCharFormat_fontKerning {
  fn fontKerning(self, this: &mut QTextCharFormat) -> i32;
}

// proto: bool QTextCharFormat::fontKerning();
impl<'a> /*trait*/ QTextCharFormat_fontKerning for () {
  fn fontKerning(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat11fontKerningEv()};
    unsafe {_ZNK15QTextCharFormat11fontKerningEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontWeight<T: QTextCharFormat_setFontWeight>(&mut self, value: T) -> i32 {
    value.setFontWeight(self);
    return 1;
  }
}

pub trait QTextCharFormat_setFontWeight {
  fn setFontWeight(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setFontWeight(int weight);
impl<'a> /*trait*/ QTextCharFormat_setFontWeight for (i32) {
  fn setFontWeight(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setFontWeightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QTextCharFormat13setFontWeightEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontUnderline<T: QTextCharFormat_fontUnderline>(&mut self, value: T) -> i32 {
    value.fontUnderline(self);
    return 1;
  }
}

pub trait QTextCharFormat_fontUnderline {
  fn fontUnderline(self, this: &mut QTextCharFormat) -> i32;
}

// proto: bool QTextCharFormat::fontUnderline();
impl<'a> /*trait*/ QTextCharFormat_fontUnderline for () {
  fn fontUnderline(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat13fontUnderlineEv()};
    unsafe {_ZNK15QTextCharFormat13fontUnderlineEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontWordSpacing<T: QTextCharFormat_setFontWordSpacing>(&mut self, value: T) -> i32 {
    value.setFontWordSpacing(self);
    return 1;
  }
}

pub trait QTextCharFormat_setFontWordSpacing {
  fn setFontWordSpacing(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setFontWordSpacing(qreal spacing);
impl<'a> /*trait*/ QTextCharFormat_setFontWordSpacing for (f64) {
  fn setFontWordSpacing(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat18setFontWordSpacingEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN15QTextCharFormat18setFontWordSpacingEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn underlineColor<T: QTextCharFormat_underlineColor>(&mut self, value: T) -> i32 {
    value.underlineColor(self);
    return 1;
  }
}

pub trait QTextCharFormat_underlineColor {
  fn underlineColor(self, this: &mut QTextCharFormat) -> i32;
}

// proto: QColor QTextCharFormat::underlineColor();
impl<'a> /*trait*/ QTextCharFormat_underlineColor for () {
  fn underlineColor(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat14underlineColorEv()};
    unsafe {_ZNK15QTextCharFormat14underlineColorEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontWeight<T: QTextCharFormat_fontWeight>(&mut self, value: T) -> i32 {
    value.fontWeight(self);
    return 1;
  }
}

pub trait QTextCharFormat_fontWeight {
  fn fontWeight(self, this: &mut QTextCharFormat) -> i32;
}

// proto: int QTextCharFormat::fontWeight();
impl<'a> /*trait*/ QTextCharFormat_fontWeight for () {
  fn fontWeight(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10fontWeightEv()};
    unsafe {_ZNK15QTextCharFormat10fontWeightEv()};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontOverline<T: QTextCharFormat_setFontOverline>(&mut self, value: T) -> i32 {
    value.setFontOverline(self);
    return 1;
  }
}

pub trait QTextCharFormat_setFontOverline {
  fn setFontOverline(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setFontOverline(bool overline);
impl<'a> /*trait*/ QTextCharFormat_setFontOverline for (i8) {
  fn setFontOverline(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat15setFontOverlineEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QTextCharFormat15setFontOverlineEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setTableCellColumnSpan<T: QTextCharFormat_setTableCellColumnSpan>(&mut self, value: T) -> i32 {
    value.setTableCellColumnSpan(self);
    return 1;
  }
}

pub trait QTextCharFormat_setTableCellColumnSpan {
  fn setTableCellColumnSpan(self, this: &mut QTextCharFormat) -> i32;
}

// proto: void QTextCharFormat::setTableCellColumnSpan(int tableCellColumnSpan);
impl<'a> /*trait*/ QTextCharFormat_setTableCellColumnSpan for (i32) {
  fn setTableCellColumnSpan(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat22setTableCellColumnSpanEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QTextCharFormat22setTableCellColumnSpanEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn textOutline<T: QTextCharFormat_textOutline>(&mut self, value: T) -> i32 {
    value.textOutline(self);
    return 1;
  }
}

pub trait QTextCharFormat_textOutline {
  fn textOutline(self, this: &mut QTextCharFormat) -> i32;
}

// proto: QPen QTextCharFormat::textOutline();
impl<'a> /*trait*/ QTextCharFormat_textOutline for () {
  fn textOutline(self, this: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat11textOutlineEv()};
    unsafe {_ZNK15QTextCharFormat11textOutlineEv()};
    return 1;
  }
}

