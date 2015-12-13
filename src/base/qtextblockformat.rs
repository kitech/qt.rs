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
  // proto: int QTextBlockFormat::indent();
  fn _ZNK16QTextBlockFormat6indentEv() -> i32;
  // proto: void QTextBlockFormat::setTextIndent(qreal aindent);
  fn _ZN16QTextBlockFormat13setTextIndentEd(arg0: c_double) -> i32;
  // proto: void QTextBlockFormat::setNonBreakableLines(bool b);
  fn _ZN16QTextBlockFormat20setNonBreakableLinesEb(arg0: int8_t) -> i32;
  // proto: void QTextBlockFormat::setIndent(int indent);
  fn _ZN16QTextBlockFormat9setIndentEi(arg0: c_int) -> i32;
  // proto: double QTextBlockFormat::textIndent();
  fn _ZNK16QTextBlockFormat10textIndentEv() -> i32;
  // proto: double QTextBlockFormat::lineHeight();
  fn _ZNK16QTextBlockFormat10lineHeightEv() -> i32;
  // proto: void QTextBlockFormat::NewQTextBlockFormat(const QTextFormat & fmt);
  fn _ZN16QTextBlockFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: double QTextBlockFormat::lineHeight(qreal scriptLineHeight, qreal scaling);
  fn _ZNK16QTextBlockFormat10lineHeightEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: void QTextBlockFormat::setRightMargin(qreal margin);
  fn _ZN16QTextBlockFormat14setRightMarginEd(arg0: c_double) -> i32;
  // proto: double QTextBlockFormat::topMargin();
  fn _ZNK16QTextBlockFormat9topMarginEv() -> i32;
  // proto: void QTextBlockFormat::NewQTextBlockFormat();
  fn _ZN16QTextBlockFormatC1Ev(qthis: *mut c_void) -> i32;
  // proto: double QTextBlockFormat::rightMargin();
  fn _ZNK16QTextBlockFormat11rightMarginEv() -> i32;
  // proto: double QTextBlockFormat::bottomMargin();
  fn _ZNK16QTextBlockFormat12bottomMarginEv() -> i32;
  // proto: void QTextBlockFormat::setTopMargin(qreal margin);
  fn _ZN16QTextBlockFormat12setTopMarginEd(arg0: c_double) -> i32;
  // proto: double QTextBlockFormat::leftMargin();
  fn _ZNK16QTextBlockFormat10leftMarginEv() -> i32;
  // proto: void QTextBlockFormat::setLineHeight(qreal height, int heightType);
  fn _ZN16QTextBlockFormat13setLineHeightEdi(arg0: c_double, arg1: c_int) -> i32;
  // proto: void QTextBlockFormat::setBottomMargin(qreal margin);
  fn _ZN16QTextBlockFormat15setBottomMarginEd(arg0: c_double) -> i32;
  // proto: int QTextBlockFormat::lineHeightType();
  fn _ZNK16QTextBlockFormat14lineHeightTypeEv() -> i32;
  // proto: void QTextBlockFormat::setLeftMargin(qreal margin);
  fn _ZN16QTextBlockFormat13setLeftMarginEd(arg0: c_double) -> i32;
  // proto: bool QTextBlockFormat::isValid();
  fn _ZNK16QTextBlockFormat7isValidEv() -> i32;
  // proto: bool QTextBlockFormat::nonBreakableLines();
  fn _ZNK16QTextBlockFormat17nonBreakableLinesEv() -> i32;
}

// body block begin
// class sizeof(QTextBlockFormat)=1
pub struct QTextBlockFormat {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextBlockFormat {
  pub fn indent<T: QTextBlockFormat_indent>(&mut self, value: T) -> i32 {
    value.indent(self);
    return 1;
  }
}

pub trait QTextBlockFormat_indent {
  fn indent(self, this: &mut QTextBlockFormat) -> i32;
}

// proto: int QTextBlockFormat::indent();
impl<'a> /*trait*/ QTextBlockFormat_indent for () {
  fn indent(self, this: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat6indentEv()};
    unsafe {_ZNK16QTextBlockFormat6indentEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn setTextIndent<T: QTextBlockFormat_setTextIndent>(&mut self, value: T) -> i32 {
    value.setTextIndent(self);
    return 1;
  }
}

pub trait QTextBlockFormat_setTextIndent {
  fn setTextIndent(self, this: &mut QTextBlockFormat) -> i32;
}

// proto: void QTextBlockFormat::setTextIndent(qreal aindent);
impl<'a> /*trait*/ QTextBlockFormat_setTextIndent for (f64) {
  fn setTextIndent(self, this: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat13setTextIndentEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QTextBlockFormat13setTextIndentEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn setNonBreakableLines<T: QTextBlockFormat_setNonBreakableLines>(&mut self, value: T) -> i32 {
    value.setNonBreakableLines(self);
    return 1;
  }
}

pub trait QTextBlockFormat_setNonBreakableLines {
  fn setNonBreakableLines(self, this: &mut QTextBlockFormat) -> i32;
}

// proto: void QTextBlockFormat::setNonBreakableLines(bool b);
impl<'a> /*trait*/ QTextBlockFormat_setNonBreakableLines for (i8) {
  fn setNonBreakableLines(self, this: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat20setNonBreakableLinesEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN16QTextBlockFormat20setNonBreakableLinesEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn setIndent<T: QTextBlockFormat_setIndent>(&mut self, value: T) -> i32 {
    value.setIndent(self);
    return 1;
  }
}

pub trait QTextBlockFormat_setIndent {
  fn setIndent(self, this: &mut QTextBlockFormat) -> i32;
}

// proto: void QTextBlockFormat::setIndent(int indent);
impl<'a> /*trait*/ QTextBlockFormat_setIndent for (i32) {
  fn setIndent(self, this: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat9setIndentEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN16QTextBlockFormat9setIndentEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn textIndent<T: QTextBlockFormat_textIndent>(&mut self, value: T) -> i32 {
    value.textIndent(self);
    return 1;
  }
}

pub trait QTextBlockFormat_textIndent {
  fn textIndent(self, this: &mut QTextBlockFormat) -> i32;
}

// proto: double QTextBlockFormat::textIndent();
impl<'a> /*trait*/ QTextBlockFormat_textIndent for () {
  fn textIndent(self, this: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat10textIndentEv()};
    unsafe {_ZNK16QTextBlockFormat10textIndentEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn lineHeight<T: QTextBlockFormat_lineHeight>(&mut self, value: T) -> i32 {
    value.lineHeight(self);
    return 1;
  }
}

pub trait QTextBlockFormat_lineHeight {
  fn lineHeight(self, this: &mut QTextBlockFormat) -> i32;
}

// proto: double QTextBlockFormat::lineHeight();
impl<'a> /*trait*/ QTextBlockFormat_lineHeight for () {
  fn lineHeight(self, this: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat10lineHeightEv()};
    unsafe {_ZNK16QTextBlockFormat10lineHeightEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTextBlockFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextBlockFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: double QTextBlockFormat::lineHeight(qreal scriptLineHeight, qreal scaling);
impl<'a> /*trait*/ QTextBlockFormat_lineHeight for (f64, f64) {
  fn lineHeight(self, this: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat10lineHeightEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZNK16QTextBlockFormat10lineHeightEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn setRightMargin<T: QTextBlockFormat_setRightMargin>(&mut self, value: T) -> i32 {
    value.setRightMargin(self);
    return 1;
  }
}

pub trait QTextBlockFormat_setRightMargin {
  fn setRightMargin(self, this: &mut QTextBlockFormat) -> i32;
}

// proto: void QTextBlockFormat::setRightMargin(qreal margin);
impl<'a> /*trait*/ QTextBlockFormat_setRightMargin for (f64) {
  fn setRightMargin(self, this: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat14setRightMarginEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QTextBlockFormat14setRightMarginEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn topMargin<T: QTextBlockFormat_topMargin>(&mut self, value: T) -> i32 {
    value.topMargin(self);
    return 1;
  }
}

pub trait QTextBlockFormat_topMargin {
  fn topMargin(self, this: &mut QTextBlockFormat) -> i32;
}

// proto: double QTextBlockFormat::topMargin();
impl<'a> /*trait*/ QTextBlockFormat_topMargin for () {
  fn topMargin(self, this: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat9topMarginEv()};
    unsafe {_ZNK16QTextBlockFormat9topMarginEv()};
    return 1;
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
  pub fn rightMargin<T: QTextBlockFormat_rightMargin>(&mut self, value: T) -> i32 {
    value.rightMargin(self);
    return 1;
  }
}

pub trait QTextBlockFormat_rightMargin {
  fn rightMargin(self, this: &mut QTextBlockFormat) -> i32;
}

// proto: double QTextBlockFormat::rightMargin();
impl<'a> /*trait*/ QTextBlockFormat_rightMargin for () {
  fn rightMargin(self, this: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat11rightMarginEv()};
    unsafe {_ZNK16QTextBlockFormat11rightMarginEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn bottomMargin<T: QTextBlockFormat_bottomMargin>(&mut self, value: T) -> i32 {
    value.bottomMargin(self);
    return 1;
  }
}

pub trait QTextBlockFormat_bottomMargin {
  fn bottomMargin(self, this: &mut QTextBlockFormat) -> i32;
}

// proto: double QTextBlockFormat::bottomMargin();
impl<'a> /*trait*/ QTextBlockFormat_bottomMargin for () {
  fn bottomMargin(self, this: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat12bottomMarginEv()};
    unsafe {_ZNK16QTextBlockFormat12bottomMarginEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn setTopMargin<T: QTextBlockFormat_setTopMargin>(&mut self, value: T) -> i32 {
    value.setTopMargin(self);
    return 1;
  }
}

pub trait QTextBlockFormat_setTopMargin {
  fn setTopMargin(self, this: &mut QTextBlockFormat) -> i32;
}

// proto: void QTextBlockFormat::setTopMargin(qreal margin);
impl<'a> /*trait*/ QTextBlockFormat_setTopMargin for (f64) {
  fn setTopMargin(self, this: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat12setTopMarginEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QTextBlockFormat12setTopMarginEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn leftMargin<T: QTextBlockFormat_leftMargin>(&mut self, value: T) -> i32 {
    value.leftMargin(self);
    return 1;
  }
}

pub trait QTextBlockFormat_leftMargin {
  fn leftMargin(self, this: &mut QTextBlockFormat) -> i32;
}

// proto: double QTextBlockFormat::leftMargin();
impl<'a> /*trait*/ QTextBlockFormat_leftMargin for () {
  fn leftMargin(self, this: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat10leftMarginEv()};
    unsafe {_ZNK16QTextBlockFormat10leftMarginEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn setLineHeight<T: QTextBlockFormat_setLineHeight>(&mut self, value: T) -> i32 {
    value.setLineHeight(self);
    return 1;
  }
}

pub trait QTextBlockFormat_setLineHeight {
  fn setLineHeight(self, this: &mut QTextBlockFormat) -> i32;
}

// proto: void QTextBlockFormat::setLineHeight(qreal height, int heightType);
impl<'a> /*trait*/ QTextBlockFormat_setLineHeight for (f64, i32) {
  fn setLineHeight(self, this: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat13setLineHeightEdi()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_int;
    unsafe {_ZN16QTextBlockFormat13setLineHeightEdi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn setBottomMargin<T: QTextBlockFormat_setBottomMargin>(&mut self, value: T) -> i32 {
    value.setBottomMargin(self);
    return 1;
  }
}

pub trait QTextBlockFormat_setBottomMargin {
  fn setBottomMargin(self, this: &mut QTextBlockFormat) -> i32;
}

// proto: void QTextBlockFormat::setBottomMargin(qreal margin);
impl<'a> /*trait*/ QTextBlockFormat_setBottomMargin for (f64) {
  fn setBottomMargin(self, this: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat15setBottomMarginEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QTextBlockFormat15setBottomMarginEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn lineHeightType<T: QTextBlockFormat_lineHeightType>(&mut self, value: T) -> i32 {
    value.lineHeightType(self);
    return 1;
  }
}

pub trait QTextBlockFormat_lineHeightType {
  fn lineHeightType(self, this: &mut QTextBlockFormat) -> i32;
}

// proto: int QTextBlockFormat::lineHeightType();
impl<'a> /*trait*/ QTextBlockFormat_lineHeightType for () {
  fn lineHeightType(self, this: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat14lineHeightTypeEv()};
    unsafe {_ZNK16QTextBlockFormat14lineHeightTypeEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn setLeftMargin<T: QTextBlockFormat_setLeftMargin>(&mut self, value: T) -> i32 {
    value.setLeftMargin(self);
    return 1;
  }
}

pub trait QTextBlockFormat_setLeftMargin {
  fn setLeftMargin(self, this: &mut QTextBlockFormat) -> i32;
}

// proto: void QTextBlockFormat::setLeftMargin(qreal margin);
impl<'a> /*trait*/ QTextBlockFormat_setLeftMargin for (f64) {
  fn setLeftMargin(self, this: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextBlockFormat13setLeftMarginEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QTextBlockFormat13setLeftMarginEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn isValid<T: QTextBlockFormat_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QTextBlockFormat_isValid {
  fn isValid(self, this: &mut QTextBlockFormat) -> i32;
}

// proto: bool QTextBlockFormat::isValid();
impl<'a> /*trait*/ QTextBlockFormat_isValid for () {
  fn isValid(self, this: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat7isValidEv()};
    unsafe {_ZNK16QTextBlockFormat7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlockFormat {
  pub fn nonBreakableLines<T: QTextBlockFormat_nonBreakableLines>(&mut self, value: T) -> i32 {
    value.nonBreakableLines(self);
    return 1;
  }
}

pub trait QTextBlockFormat_nonBreakableLines {
  fn nonBreakableLines(self, this: &mut QTextBlockFormat) -> i32;
}

// proto: bool QTextBlockFormat::nonBreakableLines();
impl<'a> /*trait*/ QTextBlockFormat_nonBreakableLines for () {
  fn nonBreakableLines(self, this: &mut QTextBlockFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextBlockFormat17nonBreakableLinesEv()};
    unsafe {_ZNK16QTextBlockFormat17nonBreakableLinesEv()};
    return 1;
  }
}

