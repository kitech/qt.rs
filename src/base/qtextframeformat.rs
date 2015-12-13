// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbrush::QBrush;
use super::qtextlength::QTextLength;
use super::qtextformat::QTextFormat;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QTextFrameFormat::isValid();
  fn _ZNK16QTextFrameFormat7isValidEv() -> i32;
  // proto: void QTextFrameFormat::setHeight(qreal height);
  fn _ZN16QTextFrameFormat9setHeightEd(arg0: c_double) -> i32;
  // proto: void QTextFrameFormat::setBorderBrush(const QBrush & brush);
  fn _ZN16QTextFrameFormat14setBorderBrushERK6QBrush(arg0: *const c_void) -> i32;
  // proto: double QTextFrameFormat::margin();
  fn _ZNK16QTextFrameFormat6marginEv() -> i32;
  // proto: QBrush QTextFrameFormat::borderBrush();
  fn _ZNK16QTextFrameFormat11borderBrushEv() -> i32;
  // proto: void QTextFrameFormat::setRightMargin(qreal margin);
  fn _ZN16QTextFrameFormat14setRightMarginEd(arg0: c_double) -> i32;
  // proto: void QTextFrameFormat::setMargin(qreal margin);
  fn _ZN16QTextFrameFormat9setMarginEd(arg0: c_double) -> i32;
  // proto: void QTextFrameFormat::setBorder(qreal border);
  fn _ZN16QTextFrameFormat9setBorderEd(arg0: c_double) -> i32;
  // proto: void QTextFrameFormat::setHeight(const QTextLength & height);
  fn _ZN16QTextFrameFormat9setHeightERK11QTextLength(arg0: *const c_void) -> i32;
  // proto: void QTextFrameFormat::setWidth(const QTextLength & length);
  fn _ZN16QTextFrameFormat8setWidthERK11QTextLength(arg0: *const c_void) -> i32;
  // proto: double QTextFrameFormat::bottomMargin();
  fn _ZNK16QTextFrameFormat12bottomMarginEv() -> i32;
  // proto: void QTextFrameFormat::setBottomMargin(qreal margin);
  fn _ZN16QTextFrameFormat15setBottomMarginEd(arg0: c_double) -> i32;
  // proto: QTextLength QTextFrameFormat::height();
  fn _ZNK16QTextFrameFormat6heightEv() -> i32;
  // proto: void QTextFrameFormat::setWidth(qreal width);
  fn _ZN16QTextFrameFormat8setWidthEd(arg0: c_double) -> i32;
  // proto: double QTextFrameFormat::rightMargin();
  fn _ZNK16QTextFrameFormat11rightMarginEv() -> i32;
  // proto: void QTextFrameFormat::setPadding(qreal padding);
  fn _ZN16QTextFrameFormat10setPaddingEd(arg0: c_double) -> i32;
  // proto: void QTextFrameFormat::setTopMargin(qreal margin);
  fn _ZN16QTextFrameFormat12setTopMarginEd(arg0: c_double) -> i32;
  // proto: double QTextFrameFormat::topMargin();
  fn _ZNK16QTextFrameFormat9topMarginEv() -> i32;
  // proto: QTextLength QTextFrameFormat::width();
  fn _ZNK16QTextFrameFormat5widthEv() -> i32;
  // proto: void QTextFrameFormat::NewQTextFrameFormat(const QTextFormat & fmt);
  fn _ZN16QTextFrameFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: double QTextFrameFormat::padding();
  fn _ZNK16QTextFrameFormat7paddingEv() -> i32;
  // proto: void QTextFrameFormat::setLeftMargin(qreal margin);
  fn _ZN16QTextFrameFormat13setLeftMarginEd(arg0: c_double) -> i32;
  // proto: double QTextFrameFormat::border();
  fn _ZNK16QTextFrameFormat6borderEv() -> i32;
  // proto: void QTextFrameFormat::NewQTextFrameFormat();
  fn _ZN16QTextFrameFormatC1Ev(qthis: *mut c_void) -> i32;
  // proto: double QTextFrameFormat::leftMargin();
  fn _ZNK16QTextFrameFormat10leftMarginEv() -> i32;
}

// body block begin
// class sizeof(QTextFrameFormat)=1
pub struct QTextFrameFormat {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextFrameFormat {
  pub fn isValid<T: QTextFrameFormat_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QTextFrameFormat_isValid {
  fn isValid(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: bool QTextFrameFormat::isValid();
impl<'a> /*trait*/ QTextFrameFormat_isValid for () {
  fn isValid(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat7isValidEv()};
    unsafe {_ZNK16QTextFrameFormat7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setHeight<T: QTextFrameFormat_setHeight>(&mut self, value: T) -> i32 {
    value.setHeight(self);
    return 1;
  }
}

pub trait QTextFrameFormat_setHeight {
  fn setHeight(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: void QTextFrameFormat::setHeight(qreal height);
impl<'a> /*trait*/ QTextFrameFormat_setHeight for (f64) {
  fn setHeight(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat9setHeightEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QTextFrameFormat9setHeightEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setBorderBrush<T: QTextFrameFormat_setBorderBrush>(&mut self, value: T) -> i32 {
    value.setBorderBrush(self);
    return 1;
  }
}

pub trait QTextFrameFormat_setBorderBrush {
  fn setBorderBrush(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: void QTextFrameFormat::setBorderBrush(const QBrush & brush);
impl<'a> /*trait*/ QTextFrameFormat_setBorderBrush for (&'a  QBrush) {
  fn setBorderBrush(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat14setBorderBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTextFrameFormat14setBorderBrushERK6QBrush(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn margin<T: QTextFrameFormat_margin>(&mut self, value: T) -> i32 {
    value.margin(self);
    return 1;
  }
}

pub trait QTextFrameFormat_margin {
  fn margin(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: double QTextFrameFormat::margin();
impl<'a> /*trait*/ QTextFrameFormat_margin for () {
  fn margin(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat6marginEv()};
    unsafe {_ZNK16QTextFrameFormat6marginEv()};
    return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn borderBrush<T: QTextFrameFormat_borderBrush>(&mut self, value: T) -> i32 {
    value.borderBrush(self);
    return 1;
  }
}

pub trait QTextFrameFormat_borderBrush {
  fn borderBrush(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: QBrush QTextFrameFormat::borderBrush();
impl<'a> /*trait*/ QTextFrameFormat_borderBrush for () {
  fn borderBrush(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat11borderBrushEv()};
    unsafe {_ZNK16QTextFrameFormat11borderBrushEv()};
    return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setRightMargin<T: QTextFrameFormat_setRightMargin>(&mut self, value: T) -> i32 {
    value.setRightMargin(self);
    return 1;
  }
}

pub trait QTextFrameFormat_setRightMargin {
  fn setRightMargin(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: void QTextFrameFormat::setRightMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setRightMargin for (f64) {
  fn setRightMargin(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat14setRightMarginEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QTextFrameFormat14setRightMarginEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setMargin<T: QTextFrameFormat_setMargin>(&mut self, value: T) -> i32 {
    value.setMargin(self);
    return 1;
  }
}

pub trait QTextFrameFormat_setMargin {
  fn setMargin(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: void QTextFrameFormat::setMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setMargin for (f64) {
  fn setMargin(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat9setMarginEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QTextFrameFormat9setMarginEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setBorder<T: QTextFrameFormat_setBorder>(&mut self, value: T) -> i32 {
    value.setBorder(self);
    return 1;
  }
}

pub trait QTextFrameFormat_setBorder {
  fn setBorder(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: void QTextFrameFormat::setBorder(qreal border);
impl<'a> /*trait*/ QTextFrameFormat_setBorder for (f64) {
  fn setBorder(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat9setBorderEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QTextFrameFormat9setBorderEd(arg0)};
    return 1;
  }
}

// proto: void QTextFrameFormat::setHeight(const QTextLength & height);
impl<'a> /*trait*/ QTextFrameFormat_setHeight for (&'a  QTextLength) {
  fn setHeight(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat9setHeightERK11QTextLength()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTextFrameFormat9setHeightERK11QTextLength(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setWidth<T: QTextFrameFormat_setWidth>(&mut self, value: T) -> i32 {
    value.setWidth(self);
    return 1;
  }
}

pub trait QTextFrameFormat_setWidth {
  fn setWidth(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: void QTextFrameFormat::setWidth(const QTextLength & length);
impl<'a> /*trait*/ QTextFrameFormat_setWidth for (&'a  QTextLength) {
  fn setWidth(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat8setWidthERK11QTextLength()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTextFrameFormat8setWidthERK11QTextLength(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn bottomMargin<T: QTextFrameFormat_bottomMargin>(&mut self, value: T) -> i32 {
    value.bottomMargin(self);
    return 1;
  }
}

pub trait QTextFrameFormat_bottomMargin {
  fn bottomMargin(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: double QTextFrameFormat::bottomMargin();
impl<'a> /*trait*/ QTextFrameFormat_bottomMargin for () {
  fn bottomMargin(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat12bottomMarginEv()};
    unsafe {_ZNK16QTextFrameFormat12bottomMarginEv()};
    return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setBottomMargin<T: QTextFrameFormat_setBottomMargin>(&mut self, value: T) -> i32 {
    value.setBottomMargin(self);
    return 1;
  }
}

pub trait QTextFrameFormat_setBottomMargin {
  fn setBottomMargin(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: void QTextFrameFormat::setBottomMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setBottomMargin for (f64) {
  fn setBottomMargin(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat15setBottomMarginEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QTextFrameFormat15setBottomMarginEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn height<T: QTextFrameFormat_height>(&mut self, value: T) -> i32 {
    value.height(self);
    return 1;
  }
}

pub trait QTextFrameFormat_height {
  fn height(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: QTextLength QTextFrameFormat::height();
impl<'a> /*trait*/ QTextFrameFormat_height for () {
  fn height(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat6heightEv()};
    unsafe {_ZNK16QTextFrameFormat6heightEv()};
    return 1;
  }
}

// proto: void QTextFrameFormat::setWidth(qreal width);
impl<'a> /*trait*/ QTextFrameFormat_setWidth for (f64) {
  fn setWidth(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat8setWidthEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QTextFrameFormat8setWidthEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn rightMargin<T: QTextFrameFormat_rightMargin>(&mut self, value: T) -> i32 {
    value.rightMargin(self);
    return 1;
  }
}

pub trait QTextFrameFormat_rightMargin {
  fn rightMargin(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: double QTextFrameFormat::rightMargin();
impl<'a> /*trait*/ QTextFrameFormat_rightMargin for () {
  fn rightMargin(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat11rightMarginEv()};
    unsafe {_ZNK16QTextFrameFormat11rightMarginEv()};
    return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setPadding<T: QTextFrameFormat_setPadding>(&mut self, value: T) -> i32 {
    value.setPadding(self);
    return 1;
  }
}

pub trait QTextFrameFormat_setPadding {
  fn setPadding(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: void QTextFrameFormat::setPadding(qreal padding);
impl<'a> /*trait*/ QTextFrameFormat_setPadding for (f64) {
  fn setPadding(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat10setPaddingEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QTextFrameFormat10setPaddingEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setTopMargin<T: QTextFrameFormat_setTopMargin>(&mut self, value: T) -> i32 {
    value.setTopMargin(self);
    return 1;
  }
}

pub trait QTextFrameFormat_setTopMargin {
  fn setTopMargin(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: void QTextFrameFormat::setTopMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setTopMargin for (f64) {
  fn setTopMargin(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat12setTopMarginEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QTextFrameFormat12setTopMarginEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn topMargin<T: QTextFrameFormat_topMargin>(&mut self, value: T) -> i32 {
    value.topMargin(self);
    return 1;
  }
}

pub trait QTextFrameFormat_topMargin {
  fn topMargin(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: double QTextFrameFormat::topMargin();
impl<'a> /*trait*/ QTextFrameFormat_topMargin for () {
  fn topMargin(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat9topMarginEv()};
    unsafe {_ZNK16QTextFrameFormat9topMarginEv()};
    return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn width<T: QTextFrameFormat_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QTextFrameFormat_width {
  fn width(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: QTextLength QTextFrameFormat::width();
impl<'a> /*trait*/ QTextFrameFormat_width for () {
  fn width(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat5widthEv()};
    unsafe {_ZNK16QTextFrameFormat5widthEv()};
    return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn NewQTextFrameFormat<T: QTextFrameFormat_NewQTextFrameFormat>(value: T) -> QTextFrameFormat {
    let rsthis = value.NewQTextFrameFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QTextFrameFormat_NewQTextFrameFormat {
  fn NewQTextFrameFormat(self) -> QTextFrameFormat;
}

// proto: void QTextFrameFormat::NewQTextFrameFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextFrameFormat_NewQTextFrameFormat for (&'a  QTextFormat) {
  fn NewQTextFrameFormat(self) -> QTextFrameFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormatC1ERK11QTextFormat()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QTextFrameFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextFrameFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn padding<T: QTextFrameFormat_padding>(&mut self, value: T) -> i32 {
    value.padding(self);
    return 1;
  }
}

pub trait QTextFrameFormat_padding {
  fn padding(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: double QTextFrameFormat::padding();
impl<'a> /*trait*/ QTextFrameFormat_padding for () {
  fn padding(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat7paddingEv()};
    unsafe {_ZNK16QTextFrameFormat7paddingEv()};
    return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setLeftMargin<T: QTextFrameFormat_setLeftMargin>(&mut self, value: T) -> i32 {
    value.setLeftMargin(self);
    return 1;
  }
}

pub trait QTextFrameFormat_setLeftMargin {
  fn setLeftMargin(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: void QTextFrameFormat::setLeftMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setLeftMargin for (f64) {
  fn setLeftMargin(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat13setLeftMarginEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN16QTextFrameFormat13setLeftMarginEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn border<T: QTextFrameFormat_border>(&mut self, value: T) -> i32 {
    value.border(self);
    return 1;
  }
}

pub trait QTextFrameFormat_border {
  fn border(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: double QTextFrameFormat::border();
impl<'a> /*trait*/ QTextFrameFormat_border for () {
  fn border(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat6borderEv()};
    unsafe {_ZNK16QTextFrameFormat6borderEv()};
    return 1;
  }
}

// proto: void QTextFrameFormat::NewQTextFrameFormat();
impl<'a> /*trait*/ QTextFrameFormat_NewQTextFrameFormat for () {
  fn NewQTextFrameFormat(self) -> QTextFrameFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormatC1Ev()};
    unsafe {_ZN16QTextFrameFormatC1Ev(qthis)};
    let rsthis = QTextFrameFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn leftMargin<T: QTextFrameFormat_leftMargin>(&mut self, value: T) -> i32 {
    value.leftMargin(self);
    return 1;
  }
}

pub trait QTextFrameFormat_leftMargin {
  fn leftMargin(self, this: &mut QTextFrameFormat) -> i32;
}

// proto: double QTextFrameFormat::leftMargin();
impl<'a> /*trait*/ QTextFrameFormat_leftMargin for () {
  fn leftMargin(self, this: &mut QTextFrameFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat10leftMarginEv()};
    unsafe {_ZNK16QTextFrameFormat10leftMarginEv()};
    return 1;
  }
}

