// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextformat::QTextFormat;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  int QTextBlockFormat::indent();
  fn _ZNK16QTextBlockFormat6indentEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextBlockFormat::setTextIndent(qreal aindent);
  fn _ZN16QTextBlockFormat13setTextIndentEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QTextBlockFormat::setNonBreakableLines(bool b);
  fn _ZN16QTextBlockFormat20setNonBreakableLinesEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextBlockFormat::setIndent(int indent);
  fn _ZN16QTextBlockFormat9setIndentEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  double QTextBlockFormat::textIndent();
  fn _ZNK16QTextBlockFormat10textIndentEv(qthis: *mut c_void) -> c_double;
  // proto:  double QTextBlockFormat::lineHeight();
  fn _ZNK16QTextBlockFormat10lineHeightEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextBlockFormat::NewQTextBlockFormat(const QTextFormat & fmt);
  fn _ZN16QTextBlockFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QTextBlockFormat::lineHeight(qreal scriptLineHeight, qreal scaling);
  fn _ZNK16QTextBlockFormat10lineHeightEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> c_double;
  // proto:  void QTextBlockFormat::setRightMargin(qreal margin);
  fn _ZN16QTextBlockFormat14setRightMarginEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QTextBlockFormat::topMargin();
  fn _ZNK16QTextBlockFormat9topMarginEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextBlockFormat::NewQTextBlockFormat();
  fn _ZN16QTextBlockFormatC1Ev(qthis: *mut c_void) ;
  // proto:  double QTextBlockFormat::rightMargin();
  fn _ZNK16QTextBlockFormat11rightMarginEv(qthis: *mut c_void) -> c_double;
  // proto:  double QTextBlockFormat::bottomMargin();
  fn _ZNK16QTextBlockFormat12bottomMarginEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextBlockFormat::setTopMargin(qreal margin);
  fn _ZN16QTextBlockFormat12setTopMarginEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QTextBlockFormat::leftMargin();
  fn _ZNK16QTextBlockFormat10leftMarginEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextBlockFormat::setLineHeight(qreal height, int heightType);
  fn _ZN16QTextBlockFormat13setLineHeightEdi(qthis: *mut c_void, arg0: c_double, arg1: c_int) ;
  // proto:  void QTextBlockFormat::setBottomMargin(qreal margin);
  fn _ZN16QTextBlockFormat15setBottomMarginEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  int QTextBlockFormat::lineHeightType();
  fn _ZNK16QTextBlockFormat14lineHeightTypeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextBlockFormat::setLeftMargin(qreal margin);
  fn _ZN16QTextBlockFormat13setLeftMarginEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  bool QTextBlockFormat::isValid();
  fn _ZNK16QTextBlockFormat7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QTextBlockFormat::nonBreakableLines();
  fn _ZNK16QTextBlockFormat17nonBreakableLinesEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QTextBlockFormat)=1
pub struct QTextBlockFormat {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextBlockFormat {
  pub fn indent<T: QTextBlockFormat_indent>(&mut self, value: T) -> i32 {
    return value.indent(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_indent {
  fn indent(self, rsthis: &mut QTextBlockFormat) -> i32;
}

// proto:  int QTextBlockFormat::indent();
impl<'a> /*trait*/ QTextBlockFormat_indent for () {
  fn indent(self, rsthis: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat6indentEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat6indentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn setTextIndent<T: QTextBlockFormat_setTextIndent>(&mut self, value: T)  {
     value.setTextIndent(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_setTextIndent {
  fn setTextIndent(self, rsthis: &mut QTextBlockFormat) ;
}

// proto:  void QTextBlockFormat::setTextIndent(qreal aindent);
impl<'a> /*trait*/ QTextBlockFormat_setTextIndent for (f64) {
  fn setTextIndent(self, rsthis: &mut QTextBlockFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat13setTextIndentEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextBlockFormat13setTextIndentEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn setNonBreakableLines<T: QTextBlockFormat_setNonBreakableLines>(&mut self, value: T)  {
     value.setNonBreakableLines(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_setNonBreakableLines {
  fn setNonBreakableLines(self, rsthis: &mut QTextBlockFormat) ;
}

// proto:  void QTextBlockFormat::setNonBreakableLines(bool b);
impl<'a> /*trait*/ QTextBlockFormat_setNonBreakableLines for (i8) {
  fn setNonBreakableLines(self, rsthis: &mut QTextBlockFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat20setNonBreakableLinesEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN16QTextBlockFormat20setNonBreakableLinesEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn setIndent<T: QTextBlockFormat_setIndent>(&mut self, value: T)  {
     value.setIndent(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_setIndent {
  fn setIndent(self, rsthis: &mut QTextBlockFormat) ;
}

// proto:  void QTextBlockFormat::setIndent(int indent);
impl<'a> /*trait*/ QTextBlockFormat_setIndent for (i32) {
  fn setIndent(self, rsthis: &mut QTextBlockFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat9setIndentEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QTextBlockFormat9setIndentEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn textIndent<T: QTextBlockFormat_textIndent>(&mut self, value: T) -> f64 {
    return value.textIndent(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_textIndent {
  fn textIndent(self, rsthis: &mut QTextBlockFormat) -> f64;
}

// proto:  double QTextBlockFormat::textIndent();
impl<'a> /*trait*/ QTextBlockFormat_textIndent for () {
  fn textIndent(self, rsthis: &mut QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat10textIndentEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat10textIndentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn lineHeight<T: QTextBlockFormat_lineHeight>(&mut self, value: T) -> f64 {
    return value.lineHeight(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_lineHeight {
  fn lineHeight(self, rsthis: &mut QTextBlockFormat) -> f64;
}

// proto:  double QTextBlockFormat::lineHeight();
impl<'a> /*trait*/ QTextBlockFormat_lineHeight for () {
  fn lineHeight(self, rsthis: &mut QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat10lineHeightEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat10lineHeightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn NewQTextBlockFormat<T: QTextBlockFormat_NewQTextBlockFormat>(value: T) -> QTextBlockFormat {
    let rsthis = value.NewQTextBlockFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBlockFormat_NewQTextBlockFormat {
  fn NewQTextBlockFormat(self) -> QTextBlockFormat;
}

// proto: void QTextBlockFormat::NewQTextBlockFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextBlockFormat_NewQTextBlockFormat for (&'a  QTextFormat) {
  fn NewQTextBlockFormat(self) -> QTextBlockFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormatC1ERK11QTextFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QTextBlockFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextBlockFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  double QTextBlockFormat::lineHeight(qreal scriptLineHeight, qreal scaling);
impl<'a> /*trait*/ QTextBlockFormat_lineHeight for (f64, f64) {
  fn lineHeight(self, rsthis: &mut QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat10lineHeightEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK16QTextBlockFormat10lineHeightEdd(rsthis.qclsinst, arg0, arg1)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn setRightMargin<T: QTextBlockFormat_setRightMargin>(&mut self, value: T)  {
     value.setRightMargin(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_setRightMargin {
  fn setRightMargin(self, rsthis: &mut QTextBlockFormat) ;
}

// proto:  void QTextBlockFormat::setRightMargin(qreal margin);
impl<'a> /*trait*/ QTextBlockFormat_setRightMargin for (f64) {
  fn setRightMargin(self, rsthis: &mut QTextBlockFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat14setRightMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextBlockFormat14setRightMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn topMargin<T: QTextBlockFormat_topMargin>(&mut self, value: T) -> f64 {
    return value.topMargin(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_topMargin {
  fn topMargin(self, rsthis: &mut QTextBlockFormat) -> f64;
}

// proto:  double QTextBlockFormat::topMargin();
impl<'a> /*trait*/ QTextBlockFormat_topMargin for () {
  fn topMargin(self, rsthis: &mut QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat9topMarginEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat9topMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto: void QTextBlockFormat::NewQTextBlockFormat();
impl<'a> /*trait*/ QTextBlockFormat_NewQTextBlockFormat for () {
  fn NewQTextBlockFormat(self) -> QTextBlockFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormatC1Ev()};
    unsafe {_ZN16QTextBlockFormatC1Ev(qthis)};
    let rsthis = QTextBlockFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn rightMargin<T: QTextBlockFormat_rightMargin>(&mut self, value: T) -> f64 {
    return value.rightMargin(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_rightMargin {
  fn rightMargin(self, rsthis: &mut QTextBlockFormat) -> f64;
}

// proto:  double QTextBlockFormat::rightMargin();
impl<'a> /*trait*/ QTextBlockFormat_rightMargin for () {
  fn rightMargin(self, rsthis: &mut QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat11rightMarginEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat11rightMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn bottomMargin<T: QTextBlockFormat_bottomMargin>(&mut self, value: T) -> f64 {
    return value.bottomMargin(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_bottomMargin {
  fn bottomMargin(self, rsthis: &mut QTextBlockFormat) -> f64;
}

// proto:  double QTextBlockFormat::bottomMargin();
impl<'a> /*trait*/ QTextBlockFormat_bottomMargin for () {
  fn bottomMargin(self, rsthis: &mut QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat12bottomMarginEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat12bottomMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn setTopMargin<T: QTextBlockFormat_setTopMargin>(&mut self, value: T)  {
     value.setTopMargin(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_setTopMargin {
  fn setTopMargin(self, rsthis: &mut QTextBlockFormat) ;
}

// proto:  void QTextBlockFormat::setTopMargin(qreal margin);
impl<'a> /*trait*/ QTextBlockFormat_setTopMargin for (f64) {
  fn setTopMargin(self, rsthis: &mut QTextBlockFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat12setTopMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextBlockFormat12setTopMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn leftMargin<T: QTextBlockFormat_leftMargin>(&mut self, value: T) -> f64 {
    return value.leftMargin(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_leftMargin {
  fn leftMargin(self, rsthis: &mut QTextBlockFormat) -> f64;
}

// proto:  double QTextBlockFormat::leftMargin();
impl<'a> /*trait*/ QTextBlockFormat_leftMargin for () {
  fn leftMargin(self, rsthis: &mut QTextBlockFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat10leftMarginEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat10leftMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn setLineHeight<T: QTextBlockFormat_setLineHeight>(&mut self, value: T)  {
     value.setLineHeight(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_setLineHeight {
  fn setLineHeight(self, rsthis: &mut QTextBlockFormat) ;
}

// proto:  void QTextBlockFormat::setLineHeight(qreal height, int heightType);
impl<'a> /*trait*/ QTextBlockFormat_setLineHeight for (f64, i32) {
  fn setLineHeight(self, rsthis: &mut QTextBlockFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat13setLineHeightEdi()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_int;
     unsafe {_ZN16QTextBlockFormat13setLineHeightEdi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn setBottomMargin<T: QTextBlockFormat_setBottomMargin>(&mut self, value: T)  {
     value.setBottomMargin(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_setBottomMargin {
  fn setBottomMargin(self, rsthis: &mut QTextBlockFormat) ;
}

// proto:  void QTextBlockFormat::setBottomMargin(qreal margin);
impl<'a> /*trait*/ QTextBlockFormat_setBottomMargin for (f64) {
  fn setBottomMargin(self, rsthis: &mut QTextBlockFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat15setBottomMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextBlockFormat15setBottomMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn lineHeightType<T: QTextBlockFormat_lineHeightType>(&mut self, value: T) -> i32 {
    return value.lineHeightType(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_lineHeightType {
  fn lineHeightType(self, rsthis: &mut QTextBlockFormat) -> i32;
}

// proto:  int QTextBlockFormat::lineHeightType();
impl<'a> /*trait*/ QTextBlockFormat_lineHeightType for () {
  fn lineHeightType(self, rsthis: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat14lineHeightTypeEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat14lineHeightTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn setLeftMargin<T: QTextBlockFormat_setLeftMargin>(&mut self, value: T)  {
     value.setLeftMargin(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_setLeftMargin {
  fn setLeftMargin(self, rsthis: &mut QTextBlockFormat) ;
}

// proto:  void QTextBlockFormat::setLeftMargin(qreal margin);
impl<'a> /*trait*/ QTextBlockFormat_setLeftMargin for (f64) {
  fn setLeftMargin(self, rsthis: &mut QTextBlockFormat)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat13setLeftMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextBlockFormat13setLeftMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn isValid<T: QTextBlockFormat_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_isValid {
  fn isValid(self, rsthis: &mut QTextBlockFormat) -> i8;
}

// proto:  bool QTextBlockFormat::isValid();
impl<'a> /*trait*/ QTextBlockFormat_isValid for () {
  fn isValid(self, rsthis: &mut QTextBlockFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat7isValidEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn nonBreakableLines<T: QTextBlockFormat_nonBreakableLines>(&mut self, value: T) -> i8 {
    return value.nonBreakableLines(self);
    // return 1;
  }
}

pub trait QTextBlockFormat_nonBreakableLines {
  fn nonBreakableLines(self, rsthis: &mut QTextBlockFormat) -> i8;
}

// proto:  bool QTextBlockFormat::nonBreakableLines();
impl<'a> /*trait*/ QTextBlockFormat_nonBreakableLines for () {
  fn nonBreakableLines(self, rsthis: &mut QTextBlockFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat17nonBreakableLinesEv()};
    let mut ret = unsafe {_ZNK16QTextBlockFormat17nonBreakableLinesEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

