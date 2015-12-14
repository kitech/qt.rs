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
use super::qcolor::QColor;
use super::qpen::QPen;
use super::qtextformat::QTextFormat;
use super::qstringlist::QStringList;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTextCharFormat::setFontLetterSpacing(qreal spacing);
  fn _ZN15QTextCharFormat20setFontLetterSpacingEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  bool QTextCharFormat::isAnchor();
  fn _ZNK15QTextCharFormat8isAnchorEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextCharFormat::setFont(const QFont & font);
  fn _ZN15QTextCharFormat7setFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTextCharFormat::fontOverline();
  fn _ZNK15QTextCharFormat12fontOverlineEv(qthis: *mut c_void) -> int8_t;
  // proto:  QFont QTextCharFormat::font();
  fn _ZNK15QTextCharFormat4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QTextCharFormat::fontFamily();
  fn _ZNK15QTextCharFormat10fontFamilyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTextCharFormat::fontStrikeOut();
  fn _ZNK15QTextCharFormat13fontStrikeOutEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextCharFormat::setFontPointSize(qreal size);
  fn _ZN15QTextCharFormat16setFontPointSizeEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QTextCharFormat::setUnderlineColor(const QColor & color);
  fn _ZN15QTextCharFormat17setUnderlineColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTextCharFormat::tableCellRowSpan();
  fn _ZNK15QTextCharFormat16tableCellRowSpanEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextCharFormat::setFontUnderline(bool underline);
  fn _ZN15QTextCharFormat16setFontUnderlineEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QTextCharFormat::isValid();
  fn _ZNK15QTextCharFormat7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QTextCharFormat::fontItalic();
  fn _ZNK15QTextCharFormat10fontItalicEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextCharFormat::setToolTip(const QString & tip);
  fn _ZN15QTextCharFormat10setToolTipERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextCharFormat::setTextOutline(const QPen & pen);
  fn _ZN15QTextCharFormat14setTextOutlineERK4QPen(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextCharFormat::setTableCellRowSpan(int tableCellRowSpan);
  fn _ZN15QTextCharFormat19setTableCellRowSpanEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTextCharFormat::setAnchor(bool anchor);
  fn _ZN15QTextCharFormat9setAnchorEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  double QTextCharFormat::fontPointSize();
  fn _ZNK15QTextCharFormat13fontPointSizeEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextCharFormat::NewQTextCharFormat(const QTextFormat & fmt);
  fn _ZN15QTextCharFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextCharFormat::setFontStrikeOut(bool strikeOut);
  fn _ZN15QTextCharFormat16setFontStrikeOutEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  double QTextCharFormat::fontWordSpacing();
  fn _ZNK15QTextCharFormat15fontWordSpacingEv(qthis: *mut c_void) -> c_double;
  // proto:  QString QTextCharFormat::toolTip();
  fn _ZNK15QTextCharFormat7toolTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextCharFormat::setAnchorNames(const QStringList & names);
  fn _ZN15QTextCharFormat14setAnchorNamesERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringList QTextCharFormat::anchorNames();
  fn _ZNK15QTextCharFormat11anchorNamesEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextCharFormat::setFontFixedPitch(bool fixedPitch);
  fn _ZN15QTextCharFormat17setFontFixedPitchEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextCharFormat::setFontItalic(bool italic);
  fn _ZN15QTextCharFormat13setFontItalicEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextCharFormat::setFontFamily(const QString & family);
  fn _ZN15QTextCharFormat13setFontFamilyERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTextCharFormat::fontFixedPitch();
  fn _ZNK15QTextCharFormat14fontFixedPitchEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextCharFormat::setAnchorHref(const QString & value);
  fn _ZN15QTextCharFormat13setAnchorHrefERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTextCharFormat::fontStretch();
  fn _ZNK15QTextCharFormat11fontStretchEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextCharFormat::setFontKerning(bool enable);
  fn _ZN15QTextCharFormat14setFontKerningEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QTextCharFormat::tableCellColumnSpan();
  fn _ZNK15QTextCharFormat19tableCellColumnSpanEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextCharFormat::NewQTextCharFormat();
  fn _ZN15QTextCharFormatC1Ev(qthis: *mut c_void) ;
  // proto:  double QTextCharFormat::fontLetterSpacing();
  fn _ZNK15QTextCharFormat17fontLetterSpacingEv(qthis: *mut c_void) -> c_double;
  // proto:  QString QTextCharFormat::anchorHref();
  fn _ZNK15QTextCharFormat10anchorHrefEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QTextCharFormat::anchorName();
  fn _ZNK15QTextCharFormat10anchorNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextCharFormat::setFontStretch(int factor);
  fn _ZN15QTextCharFormat14setFontStretchEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTextCharFormat::setAnchorName(const QString & name);
  fn _ZN15QTextCharFormat13setAnchorNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTextCharFormat::fontKerning();
  fn _ZNK15QTextCharFormat11fontKerningEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextCharFormat::setFontWeight(int weight);
  fn _ZN15QTextCharFormat13setFontWeightEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QTextCharFormat::fontUnderline();
  fn _ZNK15QTextCharFormat13fontUnderlineEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextCharFormat::setFontWordSpacing(qreal spacing);
  fn _ZN15QTextCharFormat18setFontWordSpacingEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QColor QTextCharFormat::underlineColor();
  fn _ZNK15QTextCharFormat14underlineColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextCharFormat::fontWeight();
  fn _ZNK15QTextCharFormat10fontWeightEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextCharFormat::setFontOverline(bool overline);
  fn _ZN15QTextCharFormat15setFontOverlineEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextCharFormat::setTableCellColumnSpan(int tableCellColumnSpan);
  fn _ZN15QTextCharFormat22setTableCellColumnSpanEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QPen QTextCharFormat::textOutline();
  fn _ZNK15QTextCharFormat11textOutlineEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QTextCharFormat)=1
pub struct QTextCharFormat {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontLetterSpacing<T: QTextCharFormat_setFontLetterSpacing>(&mut self, value: T)  {
     value.setFontLetterSpacing(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontLetterSpacing {
  fn setFontLetterSpacing(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setFontLetterSpacing(qreal spacing);
impl<'a> /*trait*/ QTextCharFormat_setFontLetterSpacing for (f64) {
  fn setFontLetterSpacing(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat20setFontLetterSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QTextCharFormat20setFontLetterSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn isAnchor<T: QTextCharFormat_isAnchor>(&mut self, value: T) -> i8 {
    return value.isAnchor(self);
    // return 1;
  }
}

pub trait QTextCharFormat_isAnchor {
  fn isAnchor(self, rsthis: &mut QTextCharFormat) -> i8;
}

// proto:  bool QTextCharFormat::isAnchor();
impl<'a> /*trait*/ QTextCharFormat_isAnchor for () {
  fn isAnchor(self, rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat8isAnchorEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat8isAnchorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFont<T: QTextCharFormat_setFont>(&mut self, value: T)  {
     value.setFont(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFont {
  fn setFont(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setFont(const QFont & font);
impl<'a> /*trait*/ QTextCharFormat_setFont for (&'a  QFont) {
  fn setFont(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontOverline<T: QTextCharFormat_fontOverline>(&mut self, value: T) -> i8 {
    return value.fontOverline(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontOverline {
  fn fontOverline(self, rsthis: &mut QTextCharFormat) -> i8;
}

// proto:  bool QTextCharFormat::fontOverline();
impl<'a> /*trait*/ QTextCharFormat_fontOverline for () {
  fn fontOverline(self, rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat12fontOverlineEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat12fontOverlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn font<T: QTextCharFormat_font>(&mut self, value: T) -> QFont {
    return value.font(self);
    // return 1;
  }
}

pub trait QTextCharFormat_font {
  fn font(self, rsthis: &mut QTextCharFormat) -> QFont;
}

// proto:  QFont QTextCharFormat::font();
impl<'a> /*trait*/ QTextCharFormat_font for () {
  fn font(self, rsthis: &mut QTextCharFormat) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat4fontEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontFamily<T: QTextCharFormat_fontFamily>(&mut self, value: T) -> QString {
    return value.fontFamily(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontFamily {
  fn fontFamily(self, rsthis: &mut QTextCharFormat) -> QString;
}

// proto:  QString QTextCharFormat::fontFamily();
impl<'a> /*trait*/ QTextCharFormat_fontFamily for () {
  fn fontFamily(self, rsthis: &mut QTextCharFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10fontFamilyEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat10fontFamilyEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontStrikeOut<T: QTextCharFormat_fontStrikeOut>(&mut self, value: T) -> i8 {
    return value.fontStrikeOut(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontStrikeOut {
  fn fontStrikeOut(self, rsthis: &mut QTextCharFormat) -> i8;
}

// proto:  bool QTextCharFormat::fontStrikeOut();
impl<'a> /*trait*/ QTextCharFormat_fontStrikeOut for () {
  fn fontStrikeOut(self, rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat13fontStrikeOutEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat13fontStrikeOutEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontPointSize<T: QTextCharFormat_setFontPointSize>(&mut self, value: T)  {
     value.setFontPointSize(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontPointSize {
  fn setFontPointSize(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setFontPointSize(qreal size);
impl<'a> /*trait*/ QTextCharFormat_setFontPointSize for (f64) {
  fn setFontPointSize(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat16setFontPointSizeEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QTextCharFormat16setFontPointSizeEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setUnderlineColor<T: QTextCharFormat_setUnderlineColor>(&mut self, value: T)  {
     value.setUnderlineColor(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setUnderlineColor {
  fn setUnderlineColor(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setUnderlineColor(const QColor & color);
impl<'a> /*trait*/ QTextCharFormat_setUnderlineColor for (&'a  QColor) {
  fn setUnderlineColor(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat17setUnderlineColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat17setUnderlineColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn tableCellRowSpan<T: QTextCharFormat_tableCellRowSpan>(&mut self, value: T) -> i32 {
    return value.tableCellRowSpan(self);
    // return 1;
  }
}

pub trait QTextCharFormat_tableCellRowSpan {
  fn tableCellRowSpan(self, rsthis: &mut QTextCharFormat) -> i32;
}

// proto:  int QTextCharFormat::tableCellRowSpan();
impl<'a> /*trait*/ QTextCharFormat_tableCellRowSpan for () {
  fn tableCellRowSpan(self, rsthis: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat16tableCellRowSpanEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat16tableCellRowSpanEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontUnderline<T: QTextCharFormat_setFontUnderline>(&mut self, value: T)  {
     value.setFontUnderline(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontUnderline {
  fn setFontUnderline(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setFontUnderline(bool underline);
impl<'a> /*trait*/ QTextCharFormat_setFontUnderline for (i8) {
  fn setFontUnderline(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat16setFontUnderlineEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QTextCharFormat16setFontUnderlineEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn isValid<T: QTextCharFormat_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QTextCharFormat_isValid {
  fn isValid(self, rsthis: &mut QTextCharFormat) -> i8;
}

// proto:  bool QTextCharFormat::isValid();
impl<'a> /*trait*/ QTextCharFormat_isValid for () {
  fn isValid(self, rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat7isValidEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontItalic<T: QTextCharFormat_fontItalic>(&mut self, value: T) -> i8 {
    return value.fontItalic(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontItalic {
  fn fontItalic(self, rsthis: &mut QTextCharFormat) -> i8;
}

// proto:  bool QTextCharFormat::fontItalic();
impl<'a> /*trait*/ QTextCharFormat_fontItalic for () {
  fn fontItalic(self, rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10fontItalicEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat10fontItalicEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setToolTip<T: QTextCharFormat_setToolTip>(&mut self, value: T)  {
     value.setToolTip(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setToolTip {
  fn setToolTip(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setToolTip(const QString & tip);
impl<'a> /*trait*/ QTextCharFormat_setToolTip for (&'a  QString) {
  fn setToolTip(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setTextOutline<T: QTextCharFormat_setTextOutline>(&mut self, value: T)  {
     value.setTextOutline(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setTextOutline {
  fn setTextOutline(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setTextOutline(const QPen & pen);
impl<'a> /*trait*/ QTextCharFormat_setTextOutline for (&'a  QPen) {
  fn setTextOutline(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat14setTextOutlineERK4QPen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat14setTextOutlineERK4QPen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setTableCellRowSpan<T: QTextCharFormat_setTableCellRowSpan>(&mut self, value: T)  {
     value.setTableCellRowSpan(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setTableCellRowSpan {
  fn setTableCellRowSpan(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setTableCellRowSpan(int tableCellRowSpan);
impl<'a> /*trait*/ QTextCharFormat_setTableCellRowSpan for (i32) {
  fn setTableCellRowSpan(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat19setTableCellRowSpanEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QTextCharFormat19setTableCellRowSpanEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setAnchor<T: QTextCharFormat_setAnchor>(&mut self, value: T)  {
     value.setAnchor(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setAnchor {
  fn setAnchor(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setAnchor(bool anchor);
impl<'a> /*trait*/ QTextCharFormat_setAnchor for (i8) {
  fn setAnchor(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat9setAnchorEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QTextCharFormat9setAnchorEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontPointSize<T: QTextCharFormat_fontPointSize>(&mut self, value: T) -> f64 {
    return value.fontPointSize(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontPointSize {
  fn fontPointSize(self, rsthis: &mut QTextCharFormat) -> f64;
}

// proto:  double QTextCharFormat::fontPointSize();
impl<'a> /*trait*/ QTextCharFormat_fontPointSize for () {
  fn fontPointSize(self, rsthis: &mut QTextCharFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat13fontPointSizeEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat13fontPointSizeEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QTextCharFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextCharFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontStrikeOut<T: QTextCharFormat_setFontStrikeOut>(&mut self, value: T)  {
     value.setFontStrikeOut(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontStrikeOut {
  fn setFontStrikeOut(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setFontStrikeOut(bool strikeOut);
impl<'a> /*trait*/ QTextCharFormat_setFontStrikeOut for (i8) {
  fn setFontStrikeOut(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat16setFontStrikeOutEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QTextCharFormat16setFontStrikeOutEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontWordSpacing<T: QTextCharFormat_fontWordSpacing>(&mut self, value: T) -> f64 {
    return value.fontWordSpacing(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontWordSpacing {
  fn fontWordSpacing(self, rsthis: &mut QTextCharFormat) -> f64;
}

// proto:  double QTextCharFormat::fontWordSpacing();
impl<'a> /*trait*/ QTextCharFormat_fontWordSpacing for () {
  fn fontWordSpacing(self, rsthis: &mut QTextCharFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat15fontWordSpacingEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat15fontWordSpacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn toolTip<T: QTextCharFormat_toolTip>(&mut self, value: T) -> QString {
    return value.toolTip(self);
    // return 1;
  }
}

pub trait QTextCharFormat_toolTip {
  fn toolTip(self, rsthis: &mut QTextCharFormat) -> QString;
}

// proto:  QString QTextCharFormat::toolTip();
impl<'a> /*trait*/ QTextCharFormat_toolTip for () {
  fn toolTip(self, rsthis: &mut QTextCharFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat7toolTipEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setAnchorNames<T: QTextCharFormat_setAnchorNames>(&mut self, value: T)  {
     value.setAnchorNames(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setAnchorNames {
  fn setAnchorNames(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setAnchorNames(const QStringList & names);
impl<'a> /*trait*/ QTextCharFormat_setAnchorNames for (&'a  QStringList) {
  fn setAnchorNames(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat14setAnchorNamesERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat14setAnchorNamesERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn anchorNames<T: QTextCharFormat_anchorNames>(&mut self, value: T) -> QStringList {
    return value.anchorNames(self);
    // return 1;
  }
}

pub trait QTextCharFormat_anchorNames {
  fn anchorNames(self, rsthis: &mut QTextCharFormat) -> QStringList;
}

// proto:  QStringList QTextCharFormat::anchorNames();
impl<'a> /*trait*/ QTextCharFormat_anchorNames for () {
  fn anchorNames(self, rsthis: &mut QTextCharFormat) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat11anchorNamesEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat11anchorNamesEv(rsthis.qclsinst)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontFixedPitch<T: QTextCharFormat_setFontFixedPitch>(&mut self, value: T)  {
     value.setFontFixedPitch(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontFixedPitch {
  fn setFontFixedPitch(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setFontFixedPitch(bool fixedPitch);
impl<'a> /*trait*/ QTextCharFormat_setFontFixedPitch for (i8) {
  fn setFontFixedPitch(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat17setFontFixedPitchEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QTextCharFormat17setFontFixedPitchEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontItalic<T: QTextCharFormat_setFontItalic>(&mut self, value: T)  {
     value.setFontItalic(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontItalic {
  fn setFontItalic(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setFontItalic(bool italic);
impl<'a> /*trait*/ QTextCharFormat_setFontItalic for (i8) {
  fn setFontItalic(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setFontItalicEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QTextCharFormat13setFontItalicEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontFamily<T: QTextCharFormat_setFontFamily>(&mut self, value: T)  {
     value.setFontFamily(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontFamily {
  fn setFontFamily(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setFontFamily(const QString & family);
impl<'a> /*trait*/ QTextCharFormat_setFontFamily for (&'a  QString) {
  fn setFontFamily(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setFontFamilyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat13setFontFamilyERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontFixedPitch<T: QTextCharFormat_fontFixedPitch>(&mut self, value: T) -> i8 {
    return value.fontFixedPitch(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontFixedPitch {
  fn fontFixedPitch(self, rsthis: &mut QTextCharFormat) -> i8;
}

// proto:  bool QTextCharFormat::fontFixedPitch();
impl<'a> /*trait*/ QTextCharFormat_fontFixedPitch for () {
  fn fontFixedPitch(self, rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat14fontFixedPitchEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat14fontFixedPitchEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setAnchorHref<T: QTextCharFormat_setAnchorHref>(&mut self, value: T)  {
     value.setAnchorHref(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setAnchorHref {
  fn setAnchorHref(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setAnchorHref(const QString & value);
impl<'a> /*trait*/ QTextCharFormat_setAnchorHref for (&'a  QString) {
  fn setAnchorHref(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setAnchorHrefERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat13setAnchorHrefERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontStretch<T: QTextCharFormat_fontStretch>(&mut self, value: T) -> i32 {
    return value.fontStretch(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontStretch {
  fn fontStretch(self, rsthis: &mut QTextCharFormat) -> i32;
}

// proto:  int QTextCharFormat::fontStretch();
impl<'a> /*trait*/ QTextCharFormat_fontStretch for () {
  fn fontStretch(self, rsthis: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat11fontStretchEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat11fontStretchEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontKerning<T: QTextCharFormat_setFontKerning>(&mut self, value: T)  {
     value.setFontKerning(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontKerning {
  fn setFontKerning(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setFontKerning(bool enable);
impl<'a> /*trait*/ QTextCharFormat_setFontKerning for (i8) {
  fn setFontKerning(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat14setFontKerningEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QTextCharFormat14setFontKerningEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn tableCellColumnSpan<T: QTextCharFormat_tableCellColumnSpan>(&mut self, value: T) -> i32 {
    return value.tableCellColumnSpan(self);
    // return 1;
  }
}

pub trait QTextCharFormat_tableCellColumnSpan {
  fn tableCellColumnSpan(self, rsthis: &mut QTextCharFormat) -> i32;
}

// proto:  int QTextCharFormat::tableCellColumnSpan();
impl<'a> /*trait*/ QTextCharFormat_tableCellColumnSpan for () {
  fn tableCellColumnSpan(self, rsthis: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat19tableCellColumnSpanEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat19tableCellColumnSpanEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
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
  pub fn fontLetterSpacing<T: QTextCharFormat_fontLetterSpacing>(&mut self, value: T) -> f64 {
    return value.fontLetterSpacing(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontLetterSpacing {
  fn fontLetterSpacing(self, rsthis: &mut QTextCharFormat) -> f64;
}

// proto:  double QTextCharFormat::fontLetterSpacing();
impl<'a> /*trait*/ QTextCharFormat_fontLetterSpacing for () {
  fn fontLetterSpacing(self, rsthis: &mut QTextCharFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat17fontLetterSpacingEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat17fontLetterSpacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn anchorHref<T: QTextCharFormat_anchorHref>(&mut self, value: T) -> QString {
    return value.anchorHref(self);
    // return 1;
  }
}

pub trait QTextCharFormat_anchorHref {
  fn anchorHref(self, rsthis: &mut QTextCharFormat) -> QString;
}

// proto:  QString QTextCharFormat::anchorHref();
impl<'a> /*trait*/ QTextCharFormat_anchorHref for () {
  fn anchorHref(self, rsthis: &mut QTextCharFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10anchorHrefEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat10anchorHrefEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn anchorName<T: QTextCharFormat_anchorName>(&mut self, value: T) -> QString {
    return value.anchorName(self);
    // return 1;
  }
}

pub trait QTextCharFormat_anchorName {
  fn anchorName(self, rsthis: &mut QTextCharFormat) -> QString;
}

// proto:  QString QTextCharFormat::anchorName();
impl<'a> /*trait*/ QTextCharFormat_anchorName for () {
  fn anchorName(self, rsthis: &mut QTextCharFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10anchorNameEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat10anchorNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontStretch<T: QTextCharFormat_setFontStretch>(&mut self, value: T)  {
     value.setFontStretch(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontStretch {
  fn setFontStretch(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setFontStretch(int factor);
impl<'a> /*trait*/ QTextCharFormat_setFontStretch for (i32) {
  fn setFontStretch(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat14setFontStretchEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QTextCharFormat14setFontStretchEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setAnchorName<T: QTextCharFormat_setAnchorName>(&mut self, value: T)  {
     value.setAnchorName(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setAnchorName {
  fn setAnchorName(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setAnchorName(const QString & name);
impl<'a> /*trait*/ QTextCharFormat_setAnchorName for (&'a  QString) {
  fn setAnchorName(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setAnchorNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat13setAnchorNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontKerning<T: QTextCharFormat_fontKerning>(&mut self, value: T) -> i8 {
    return value.fontKerning(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontKerning {
  fn fontKerning(self, rsthis: &mut QTextCharFormat) -> i8;
}

// proto:  bool QTextCharFormat::fontKerning();
impl<'a> /*trait*/ QTextCharFormat_fontKerning for () {
  fn fontKerning(self, rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat11fontKerningEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat11fontKerningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontWeight<T: QTextCharFormat_setFontWeight>(&mut self, value: T)  {
     value.setFontWeight(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontWeight {
  fn setFontWeight(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setFontWeight(int weight);
impl<'a> /*trait*/ QTextCharFormat_setFontWeight for (i32) {
  fn setFontWeight(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setFontWeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QTextCharFormat13setFontWeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontUnderline<T: QTextCharFormat_fontUnderline>(&mut self, value: T) -> i8 {
    return value.fontUnderline(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontUnderline {
  fn fontUnderline(self, rsthis: &mut QTextCharFormat) -> i8;
}

// proto:  bool QTextCharFormat::fontUnderline();
impl<'a> /*trait*/ QTextCharFormat_fontUnderline for () {
  fn fontUnderline(self, rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat13fontUnderlineEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat13fontUnderlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontWordSpacing<T: QTextCharFormat_setFontWordSpacing>(&mut self, value: T)  {
     value.setFontWordSpacing(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontWordSpacing {
  fn setFontWordSpacing(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setFontWordSpacing(qreal spacing);
impl<'a> /*trait*/ QTextCharFormat_setFontWordSpacing for (f64) {
  fn setFontWordSpacing(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat18setFontWordSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QTextCharFormat18setFontWordSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn underlineColor<T: QTextCharFormat_underlineColor>(&mut self, value: T) -> QColor {
    return value.underlineColor(self);
    // return 1;
  }
}

pub trait QTextCharFormat_underlineColor {
  fn underlineColor(self, rsthis: &mut QTextCharFormat) -> QColor;
}

// proto:  QColor QTextCharFormat::underlineColor();
impl<'a> /*trait*/ QTextCharFormat_underlineColor for () {
  fn underlineColor(self, rsthis: &mut QTextCharFormat) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat14underlineColorEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat14underlineColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn fontWeight<T: QTextCharFormat_fontWeight>(&mut self, value: T) -> i32 {
    return value.fontWeight(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontWeight {
  fn fontWeight(self, rsthis: &mut QTextCharFormat) -> i32;
}

// proto:  int QTextCharFormat::fontWeight();
impl<'a> /*trait*/ QTextCharFormat_fontWeight for () {
  fn fontWeight(self, rsthis: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10fontWeightEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat10fontWeightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setFontOverline<T: QTextCharFormat_setFontOverline>(&mut self, value: T)  {
     value.setFontOverline(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontOverline {
  fn setFontOverline(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setFontOverline(bool overline);
impl<'a> /*trait*/ QTextCharFormat_setFontOverline for (i8) {
  fn setFontOverline(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat15setFontOverlineEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QTextCharFormat15setFontOverlineEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn setTableCellColumnSpan<T: QTextCharFormat_setTableCellColumnSpan>(&mut self, value: T)  {
     value.setTableCellColumnSpan(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setTableCellColumnSpan {
  fn setTableCellColumnSpan(self, rsthis: &mut QTextCharFormat) ;
}

// proto:  void QTextCharFormat::setTableCellColumnSpan(int tableCellColumnSpan);
impl<'a> /*trait*/ QTextCharFormat_setTableCellColumnSpan for (i32) {
  fn setTableCellColumnSpan(self, rsthis: &mut QTextCharFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat22setTableCellColumnSpanEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QTextCharFormat22setTableCellColumnSpanEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextCharFormat {
  pub fn textOutline<T: QTextCharFormat_textOutline>(&mut self, value: T) -> QPen {
    return value.textOutline(self);
    // return 1;
  }
}

pub trait QTextCharFormat_textOutline {
  fn textOutline(self, rsthis: &mut QTextCharFormat) -> QPen;
}

// proto:  QPen QTextCharFormat::textOutline();
impl<'a> /*trait*/ QTextCharFormat_textOutline for () {
  fn textOutline(self, rsthis: &mut QTextCharFormat) -> QPen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat11textOutlineEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat11textOutlineEv(rsthis.qclsinst)};
    let mut ret1 = QPen{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

