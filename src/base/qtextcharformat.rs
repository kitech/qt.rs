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
  fn _ZNK15QTextCharFormat11anchorNamesEv(qthis: *mut c_void) ;
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

// proto:  void QTextCharFormat::setFontLetterSpacing(qreal spacing);
impl /*struct*/ QTextCharFormat {
  pub fn setFontLetterSpacing<RetType, T: QTextCharFormat_setFontLetterSpacing<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFontLetterSpacing(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontLetterSpacing<RetType> {
  fn setFontLetterSpacing(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setFontLetterSpacing(qreal spacing);
impl<'a> /*trait*/ QTextCharFormat_setFontLetterSpacing<()> for (f64) {
  fn setFontLetterSpacing(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat20setFontLetterSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QTextCharFormat20setFontLetterSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QTextCharFormat::isAnchor();
impl /*struct*/ QTextCharFormat {
  pub fn isAnchor<RetType, T: QTextCharFormat_isAnchor<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isAnchor(self);
    // return 1;
  }
}

pub trait QTextCharFormat_isAnchor<RetType> {
  fn isAnchor(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  bool QTextCharFormat::isAnchor();
impl<'a> /*trait*/ QTextCharFormat_isAnchor<i8> for () {
  fn isAnchor(self , rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat8isAnchorEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat8isAnchorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QTextCharFormat::setFont(const QFont & font);
impl /*struct*/ QTextCharFormat {
  pub fn setFont<RetType, T: QTextCharFormat_setFont<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFont(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFont<RetType> {
  fn setFont(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setFont(const QFont & font);
impl<'a> /*trait*/ QTextCharFormat_setFont<()> for (&'a  QFont) {
  fn setFont(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QTextCharFormat::fontOverline();
impl /*struct*/ QTextCharFormat {
  pub fn fontOverline<RetType, T: QTextCharFormat_fontOverline<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.fontOverline(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontOverline<RetType> {
  fn fontOverline(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  bool QTextCharFormat::fontOverline();
impl<'a> /*trait*/ QTextCharFormat_fontOverline<i8> for () {
  fn fontOverline(self , rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat12fontOverlineEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat12fontOverlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QFont QTextCharFormat::font();
impl /*struct*/ QTextCharFormat {
  pub fn font<RetType, T: QTextCharFormat_font<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QTextCharFormat_font<RetType> {
  fn font(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  QFont QTextCharFormat::font();
impl<'a> /*trait*/ QTextCharFormat_font<QFont> for () {
  fn font(self , rsthis: &mut QTextCharFormat) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat4fontEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString QTextCharFormat::fontFamily();
impl /*struct*/ QTextCharFormat {
  pub fn fontFamily<RetType, T: QTextCharFormat_fontFamily<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.fontFamily(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontFamily<RetType> {
  fn fontFamily(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  QString QTextCharFormat::fontFamily();
impl<'a> /*trait*/ QTextCharFormat_fontFamily<QString> for () {
  fn fontFamily(self , rsthis: &mut QTextCharFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10fontFamilyEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat10fontFamilyEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QTextCharFormat::fontStrikeOut();
impl /*struct*/ QTextCharFormat {
  pub fn fontStrikeOut<RetType, T: QTextCharFormat_fontStrikeOut<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.fontStrikeOut(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontStrikeOut<RetType> {
  fn fontStrikeOut(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  bool QTextCharFormat::fontStrikeOut();
impl<'a> /*trait*/ QTextCharFormat_fontStrikeOut<i8> for () {
  fn fontStrikeOut(self , rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat13fontStrikeOutEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat13fontStrikeOutEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QTextCharFormat::setFontPointSize(qreal size);
impl /*struct*/ QTextCharFormat {
  pub fn setFontPointSize<RetType, T: QTextCharFormat_setFontPointSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFontPointSize(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontPointSize<RetType> {
  fn setFontPointSize(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setFontPointSize(qreal size);
impl<'a> /*trait*/ QTextCharFormat_setFontPointSize<()> for (f64) {
  fn setFontPointSize(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat16setFontPointSizeEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QTextCharFormat16setFontPointSizeEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTextCharFormat::setUnderlineColor(const QColor & color);
impl /*struct*/ QTextCharFormat {
  pub fn setUnderlineColor<RetType, T: QTextCharFormat_setUnderlineColor<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setUnderlineColor(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setUnderlineColor<RetType> {
  fn setUnderlineColor(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setUnderlineColor(const QColor & color);
impl<'a> /*trait*/ QTextCharFormat_setUnderlineColor<()> for (&'a  QColor) {
  fn setUnderlineColor(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat17setUnderlineColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat17setUnderlineColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QTextCharFormat::tableCellRowSpan();
impl /*struct*/ QTextCharFormat {
  pub fn tableCellRowSpan<RetType, T: QTextCharFormat_tableCellRowSpan<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.tableCellRowSpan(self);
    // return 1;
  }
}

pub trait QTextCharFormat_tableCellRowSpan<RetType> {
  fn tableCellRowSpan(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  int QTextCharFormat::tableCellRowSpan();
impl<'a> /*trait*/ QTextCharFormat_tableCellRowSpan<i32> for () {
  fn tableCellRowSpan(self , rsthis: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat16tableCellRowSpanEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat16tableCellRowSpanEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QTextCharFormat::setFontUnderline(bool underline);
impl /*struct*/ QTextCharFormat {
  pub fn setFontUnderline<RetType, T: QTextCharFormat_setFontUnderline<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFontUnderline(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontUnderline<RetType> {
  fn setFontUnderline(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setFontUnderline(bool underline);
impl<'a> /*trait*/ QTextCharFormat_setFontUnderline<()> for (i8) {
  fn setFontUnderline(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat16setFontUnderlineEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QTextCharFormat16setFontUnderlineEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QTextCharFormat::isValid();
impl /*struct*/ QTextCharFormat {
  pub fn isValid<RetType, T: QTextCharFormat_isValid<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextCharFormat_isValid<RetType> {
  fn isValid(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  bool QTextCharFormat::isValid();
impl<'a> /*trait*/ QTextCharFormat_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat7isValidEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QTextCharFormat::fontItalic();
impl /*struct*/ QTextCharFormat {
  pub fn fontItalic<RetType, T: QTextCharFormat_fontItalic<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.fontItalic(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontItalic<RetType> {
  fn fontItalic(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  bool QTextCharFormat::fontItalic();
impl<'a> /*trait*/ QTextCharFormat_fontItalic<i8> for () {
  fn fontItalic(self , rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10fontItalicEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat10fontItalicEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QTextCharFormat::setToolTip(const QString & tip);
impl /*struct*/ QTextCharFormat {
  pub fn setToolTip<RetType, T: QTextCharFormat_setToolTip<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setToolTip(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setToolTip<RetType> {
  fn setToolTip(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setToolTip(const QString & tip);
impl<'a> /*trait*/ QTextCharFormat_setToolTip<()> for (&'a  QString) {
  fn setToolTip(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTextCharFormat::setTextOutline(const QPen & pen);
impl /*struct*/ QTextCharFormat {
  pub fn setTextOutline<RetType, T: QTextCharFormat_setTextOutline<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setTextOutline(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setTextOutline<RetType> {
  fn setTextOutline(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setTextOutline(const QPen & pen);
impl<'a> /*trait*/ QTextCharFormat_setTextOutline<()> for (&'a  QPen) {
  fn setTextOutline(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat14setTextOutlineERK4QPen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat14setTextOutlineERK4QPen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTextCharFormat::setTableCellRowSpan(int tableCellRowSpan);
impl /*struct*/ QTextCharFormat {
  pub fn setTableCellRowSpan<RetType, T: QTextCharFormat_setTableCellRowSpan<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setTableCellRowSpan(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setTableCellRowSpan<RetType> {
  fn setTableCellRowSpan(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setTableCellRowSpan(int tableCellRowSpan);
impl<'a> /*trait*/ QTextCharFormat_setTableCellRowSpan<()> for (i32) {
  fn setTableCellRowSpan(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat19setTableCellRowSpanEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QTextCharFormat19setTableCellRowSpanEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTextCharFormat::setAnchor(bool anchor);
impl /*struct*/ QTextCharFormat {
  pub fn setAnchor<RetType, T: QTextCharFormat_setAnchor<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setAnchor(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setAnchor<RetType> {
  fn setAnchor(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setAnchor(bool anchor);
impl<'a> /*trait*/ QTextCharFormat_setAnchor<()> for (i8) {
  fn setAnchor(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat9setAnchorEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QTextCharFormat9setAnchorEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  double QTextCharFormat::fontPointSize();
impl /*struct*/ QTextCharFormat {
  pub fn fontPointSize<RetType, T: QTextCharFormat_fontPointSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.fontPointSize(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontPointSize<RetType> {
  fn fontPointSize(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  double QTextCharFormat::fontPointSize();
impl<'a> /*trait*/ QTextCharFormat_fontPointSize<f64> for () {
  fn fontPointSize(self , rsthis: &mut QTextCharFormat) -> f64 {
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

// proto:  void QTextCharFormat::setFontStrikeOut(bool strikeOut);
impl /*struct*/ QTextCharFormat {
  pub fn setFontStrikeOut<RetType, T: QTextCharFormat_setFontStrikeOut<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFontStrikeOut(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontStrikeOut<RetType> {
  fn setFontStrikeOut(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setFontStrikeOut(bool strikeOut);
impl<'a> /*trait*/ QTextCharFormat_setFontStrikeOut<()> for (i8) {
  fn setFontStrikeOut(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat16setFontStrikeOutEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QTextCharFormat16setFontStrikeOutEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  double QTextCharFormat::fontWordSpacing();
impl /*struct*/ QTextCharFormat {
  pub fn fontWordSpacing<RetType, T: QTextCharFormat_fontWordSpacing<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.fontWordSpacing(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontWordSpacing<RetType> {
  fn fontWordSpacing(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  double QTextCharFormat::fontWordSpacing();
impl<'a> /*trait*/ QTextCharFormat_fontWordSpacing<f64> for () {
  fn fontWordSpacing(self , rsthis: &mut QTextCharFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat15fontWordSpacingEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat15fontWordSpacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  QString QTextCharFormat::toolTip();
impl /*struct*/ QTextCharFormat {
  pub fn toolTip<RetType, T: QTextCharFormat_toolTip<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.toolTip(self);
    // return 1;
  }
}

pub trait QTextCharFormat_toolTip<RetType> {
  fn toolTip(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  QString QTextCharFormat::toolTip();
impl<'a> /*trait*/ QTextCharFormat_toolTip<QString> for () {
  fn toolTip(self , rsthis: &mut QTextCharFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat7toolTipEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTextCharFormat::setAnchorNames(const QStringList & names);
impl /*struct*/ QTextCharFormat {
  pub fn setAnchorNames<RetType, T: QTextCharFormat_setAnchorNames<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setAnchorNames(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setAnchorNames<RetType> {
  fn setAnchorNames(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setAnchorNames(const QStringList & names);
impl<'a> /*trait*/ QTextCharFormat_setAnchorNames<()> for (&'a  QStringList) {
  fn setAnchorNames(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat14setAnchorNamesERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat14setAnchorNamesERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QStringList QTextCharFormat::anchorNames();
impl /*struct*/ QTextCharFormat {
  pub fn anchorNames<RetType, T: QTextCharFormat_anchorNames<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.anchorNames(self);
    // return 1;
  }
}

pub trait QTextCharFormat_anchorNames<RetType> {
  fn anchorNames(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  QStringList QTextCharFormat::anchorNames();
impl<'a> /*trait*/ QTextCharFormat_anchorNames<()> for () {
  fn anchorNames(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat11anchorNamesEv()};
     unsafe {_ZNK15QTextCharFormat11anchorNamesEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QTextCharFormat::setFontFixedPitch(bool fixedPitch);
impl /*struct*/ QTextCharFormat {
  pub fn setFontFixedPitch<RetType, T: QTextCharFormat_setFontFixedPitch<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFontFixedPitch(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontFixedPitch<RetType> {
  fn setFontFixedPitch(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setFontFixedPitch(bool fixedPitch);
impl<'a> /*trait*/ QTextCharFormat_setFontFixedPitch<()> for (i8) {
  fn setFontFixedPitch(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat17setFontFixedPitchEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QTextCharFormat17setFontFixedPitchEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTextCharFormat::setFontItalic(bool italic);
impl /*struct*/ QTextCharFormat {
  pub fn setFontItalic<RetType, T: QTextCharFormat_setFontItalic<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFontItalic(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontItalic<RetType> {
  fn setFontItalic(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setFontItalic(bool italic);
impl<'a> /*trait*/ QTextCharFormat_setFontItalic<()> for (i8) {
  fn setFontItalic(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setFontItalicEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QTextCharFormat13setFontItalicEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTextCharFormat::setFontFamily(const QString & family);
impl /*struct*/ QTextCharFormat {
  pub fn setFontFamily<RetType, T: QTextCharFormat_setFontFamily<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFontFamily(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontFamily<RetType> {
  fn setFontFamily(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setFontFamily(const QString & family);
impl<'a> /*trait*/ QTextCharFormat_setFontFamily<()> for (&'a  QString) {
  fn setFontFamily(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setFontFamilyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat13setFontFamilyERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QTextCharFormat::fontFixedPitch();
impl /*struct*/ QTextCharFormat {
  pub fn fontFixedPitch<RetType, T: QTextCharFormat_fontFixedPitch<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.fontFixedPitch(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontFixedPitch<RetType> {
  fn fontFixedPitch(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  bool QTextCharFormat::fontFixedPitch();
impl<'a> /*trait*/ QTextCharFormat_fontFixedPitch<i8> for () {
  fn fontFixedPitch(self , rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat14fontFixedPitchEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat14fontFixedPitchEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QTextCharFormat::setAnchorHref(const QString & value);
impl /*struct*/ QTextCharFormat {
  pub fn setAnchorHref<RetType, T: QTextCharFormat_setAnchorHref<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setAnchorHref(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setAnchorHref<RetType> {
  fn setAnchorHref(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setAnchorHref(const QString & value);
impl<'a> /*trait*/ QTextCharFormat_setAnchorHref<()> for (&'a  QString) {
  fn setAnchorHref(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setAnchorHrefERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat13setAnchorHrefERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QTextCharFormat::fontStretch();
impl /*struct*/ QTextCharFormat {
  pub fn fontStretch<RetType, T: QTextCharFormat_fontStretch<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.fontStretch(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontStretch<RetType> {
  fn fontStretch(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  int QTextCharFormat::fontStretch();
impl<'a> /*trait*/ QTextCharFormat_fontStretch<i32> for () {
  fn fontStretch(self , rsthis: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat11fontStretchEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat11fontStretchEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QTextCharFormat::setFontKerning(bool enable);
impl /*struct*/ QTextCharFormat {
  pub fn setFontKerning<RetType, T: QTextCharFormat_setFontKerning<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFontKerning(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontKerning<RetType> {
  fn setFontKerning(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setFontKerning(bool enable);
impl<'a> /*trait*/ QTextCharFormat_setFontKerning<()> for (i8) {
  fn setFontKerning(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat14setFontKerningEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QTextCharFormat14setFontKerningEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QTextCharFormat::tableCellColumnSpan();
impl /*struct*/ QTextCharFormat {
  pub fn tableCellColumnSpan<RetType, T: QTextCharFormat_tableCellColumnSpan<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.tableCellColumnSpan(self);
    // return 1;
  }
}

pub trait QTextCharFormat_tableCellColumnSpan<RetType> {
  fn tableCellColumnSpan(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  int QTextCharFormat::tableCellColumnSpan();
impl<'a> /*trait*/ QTextCharFormat_tableCellColumnSpan<i32> for () {
  fn tableCellColumnSpan(self , rsthis: &mut QTextCharFormat) -> i32 {
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

// proto:  double QTextCharFormat::fontLetterSpacing();
impl /*struct*/ QTextCharFormat {
  pub fn fontLetterSpacing<RetType, T: QTextCharFormat_fontLetterSpacing<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.fontLetterSpacing(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontLetterSpacing<RetType> {
  fn fontLetterSpacing(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  double QTextCharFormat::fontLetterSpacing();
impl<'a> /*trait*/ QTextCharFormat_fontLetterSpacing<f64> for () {
  fn fontLetterSpacing(self , rsthis: &mut QTextCharFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat17fontLetterSpacingEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat17fontLetterSpacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  QString QTextCharFormat::anchorHref();
impl /*struct*/ QTextCharFormat {
  pub fn anchorHref<RetType, T: QTextCharFormat_anchorHref<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.anchorHref(self);
    // return 1;
  }
}

pub trait QTextCharFormat_anchorHref<RetType> {
  fn anchorHref(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  QString QTextCharFormat::anchorHref();
impl<'a> /*trait*/ QTextCharFormat_anchorHref<QString> for () {
  fn anchorHref(self , rsthis: &mut QTextCharFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10anchorHrefEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat10anchorHrefEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QString QTextCharFormat::anchorName();
impl /*struct*/ QTextCharFormat {
  pub fn anchorName<RetType, T: QTextCharFormat_anchorName<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.anchorName(self);
    // return 1;
  }
}

pub trait QTextCharFormat_anchorName<RetType> {
  fn anchorName(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  QString QTextCharFormat::anchorName();
impl<'a> /*trait*/ QTextCharFormat_anchorName<QString> for () {
  fn anchorName(self , rsthis: &mut QTextCharFormat) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10anchorNameEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat10anchorNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTextCharFormat::setFontStretch(int factor);
impl /*struct*/ QTextCharFormat {
  pub fn setFontStretch<RetType, T: QTextCharFormat_setFontStretch<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFontStretch(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontStretch<RetType> {
  fn setFontStretch(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setFontStretch(int factor);
impl<'a> /*trait*/ QTextCharFormat_setFontStretch<()> for (i32) {
  fn setFontStretch(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat14setFontStretchEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QTextCharFormat14setFontStretchEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTextCharFormat::setAnchorName(const QString & name);
impl /*struct*/ QTextCharFormat {
  pub fn setAnchorName<RetType, T: QTextCharFormat_setAnchorName<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setAnchorName(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setAnchorName<RetType> {
  fn setAnchorName(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setAnchorName(const QString & name);
impl<'a> /*trait*/ QTextCharFormat_setAnchorName<()> for (&'a  QString) {
  fn setAnchorName(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setAnchorNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTextCharFormat13setAnchorNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QTextCharFormat::fontKerning();
impl /*struct*/ QTextCharFormat {
  pub fn fontKerning<RetType, T: QTextCharFormat_fontKerning<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.fontKerning(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontKerning<RetType> {
  fn fontKerning(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  bool QTextCharFormat::fontKerning();
impl<'a> /*trait*/ QTextCharFormat_fontKerning<i8> for () {
  fn fontKerning(self , rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat11fontKerningEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat11fontKerningEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QTextCharFormat::setFontWeight(int weight);
impl /*struct*/ QTextCharFormat {
  pub fn setFontWeight<RetType, T: QTextCharFormat_setFontWeight<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFontWeight(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontWeight<RetType> {
  fn setFontWeight(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setFontWeight(int weight);
impl<'a> /*trait*/ QTextCharFormat_setFontWeight<()> for (i32) {
  fn setFontWeight(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat13setFontWeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QTextCharFormat13setFontWeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QTextCharFormat::fontUnderline();
impl /*struct*/ QTextCharFormat {
  pub fn fontUnderline<RetType, T: QTextCharFormat_fontUnderline<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.fontUnderline(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontUnderline<RetType> {
  fn fontUnderline(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  bool QTextCharFormat::fontUnderline();
impl<'a> /*trait*/ QTextCharFormat_fontUnderline<i8> for () {
  fn fontUnderline(self , rsthis: &mut QTextCharFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat13fontUnderlineEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat13fontUnderlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QTextCharFormat::setFontWordSpacing(qreal spacing);
impl /*struct*/ QTextCharFormat {
  pub fn setFontWordSpacing<RetType, T: QTextCharFormat_setFontWordSpacing<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFontWordSpacing(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontWordSpacing<RetType> {
  fn setFontWordSpacing(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setFontWordSpacing(qreal spacing);
impl<'a> /*trait*/ QTextCharFormat_setFontWordSpacing<()> for (f64) {
  fn setFontWordSpacing(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat18setFontWordSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QTextCharFormat18setFontWordSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QColor QTextCharFormat::underlineColor();
impl /*struct*/ QTextCharFormat {
  pub fn underlineColor<RetType, T: QTextCharFormat_underlineColor<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.underlineColor(self);
    // return 1;
  }
}

pub trait QTextCharFormat_underlineColor<RetType> {
  fn underlineColor(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  QColor QTextCharFormat::underlineColor();
impl<'a> /*trait*/ QTextCharFormat_underlineColor<QColor> for () {
  fn underlineColor(self , rsthis: &mut QTextCharFormat) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat14underlineColorEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat14underlineColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QTextCharFormat::fontWeight();
impl /*struct*/ QTextCharFormat {
  pub fn fontWeight<RetType, T: QTextCharFormat_fontWeight<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.fontWeight(self);
    // return 1;
  }
}

pub trait QTextCharFormat_fontWeight<RetType> {
  fn fontWeight(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  int QTextCharFormat::fontWeight();
impl<'a> /*trait*/ QTextCharFormat_fontWeight<i32> for () {
  fn fontWeight(self , rsthis: &mut QTextCharFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat10fontWeightEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat10fontWeightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QTextCharFormat::setFontOverline(bool overline);
impl /*struct*/ QTextCharFormat {
  pub fn setFontOverline<RetType, T: QTextCharFormat_setFontOverline<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setFontOverline(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setFontOverline<RetType> {
  fn setFontOverline(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setFontOverline(bool overline);
impl<'a> /*trait*/ QTextCharFormat_setFontOverline<()> for (i8) {
  fn setFontOverline(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat15setFontOverlineEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QTextCharFormat15setFontOverlineEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTextCharFormat::setTableCellColumnSpan(int tableCellColumnSpan);
impl /*struct*/ QTextCharFormat {
  pub fn setTableCellColumnSpan<RetType, T: QTextCharFormat_setTableCellColumnSpan<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setTableCellColumnSpan(self);
    // return 1;
  }
}

pub trait QTextCharFormat_setTableCellColumnSpan<RetType> {
  fn setTableCellColumnSpan(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  void QTextCharFormat::setTableCellColumnSpan(int tableCellColumnSpan);
impl<'a> /*trait*/ QTextCharFormat_setTableCellColumnSpan<()> for (i32) {
  fn setTableCellColumnSpan(self , rsthis: &mut QTextCharFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextCharFormat22setTableCellColumnSpanEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QTextCharFormat22setTableCellColumnSpanEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QPen QTextCharFormat::textOutline();
impl /*struct*/ QTextCharFormat {
  pub fn textOutline<RetType, T: QTextCharFormat_textOutline<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.textOutline(self);
    // return 1;
  }
}

pub trait QTextCharFormat_textOutline<RetType> {
  fn textOutline(self , rsthis: &mut QTextCharFormat) -> RetType;
}

// proto:  QPen QTextCharFormat::textOutline();
impl<'a> /*trait*/ QTextCharFormat_textOutline<QPen> for () {
  fn textOutline(self , rsthis: &mut QTextCharFormat) -> QPen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextCharFormat11textOutlineEv()};
    let mut ret = unsafe {_ZNK15QTextCharFormat11textOutlineEv(rsthis.qclsinst)};
    let mut ret1 = QPen{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

