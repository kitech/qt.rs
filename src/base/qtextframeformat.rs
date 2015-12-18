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
  // proto:  bool QTextFrameFormat::isValid();
  fn _ZNK16QTextFrameFormat7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextFrameFormat::setHeight(qreal height);
  fn _ZN16QTextFrameFormat9setHeightEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QTextFrameFormat::setBorderBrush(const QBrush & brush);
  fn _ZN16QTextFrameFormat14setBorderBrushERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QTextFrameFormat::margin();
  fn _ZNK16QTextFrameFormat6marginEv(qthis: *mut c_void) -> c_double;
  // proto:  QBrush QTextFrameFormat::borderBrush();
  fn _ZNK16QTextFrameFormat11borderBrushEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextFrameFormat::setRightMargin(qreal margin);
  fn _ZN16QTextFrameFormat14setRightMarginEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QTextFrameFormat::setMargin(qreal margin);
  fn _ZN16QTextFrameFormat9setMarginEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QTextFrameFormat::setBorder(qreal border);
  fn _ZN16QTextFrameFormat9setBorderEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QTextFrameFormat::setHeight(const QTextLength & height);
  fn _ZN16QTextFrameFormat9setHeightERK11QTextLength(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextFrameFormat::setWidth(const QTextLength & length);
  fn _ZN16QTextFrameFormat8setWidthERK11QTextLength(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QTextFrameFormat::bottomMargin();
  fn _ZNK16QTextFrameFormat12bottomMarginEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextFrameFormat::setBottomMargin(qreal margin);
  fn _ZN16QTextFrameFormat15setBottomMarginEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QTextLength QTextFrameFormat::height();
  fn _ZNK16QTextFrameFormat6heightEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextFrameFormat::setWidth(qreal width);
  fn _ZN16QTextFrameFormat8setWidthEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QTextFrameFormat::rightMargin();
  fn _ZNK16QTextFrameFormat11rightMarginEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextFrameFormat::setPadding(qreal padding);
  fn _ZN16QTextFrameFormat10setPaddingEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QTextFrameFormat::setTopMargin(qreal margin);
  fn _ZN16QTextFrameFormat12setTopMarginEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QTextFrameFormat::topMargin();
  fn _ZNK16QTextFrameFormat9topMarginEv(qthis: *mut c_void) -> c_double;
  // proto:  QTextLength QTextFrameFormat::width();
  fn _ZNK16QTextFrameFormat5widthEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextFrameFormat::NewQTextFrameFormat(const QTextFormat & fmt);
  fn _ZN16QTextFrameFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QTextFrameFormat::padding();
  fn _ZNK16QTextFrameFormat7paddingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextFrameFormat::setLeftMargin(qreal margin);
  fn _ZN16QTextFrameFormat13setLeftMarginEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QTextFrameFormat::border();
  fn _ZNK16QTextFrameFormat6borderEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextFrameFormat::NewQTextFrameFormat();
  fn _ZN16QTextFrameFormatC1Ev(qthis: *mut c_void) ;
  // proto:  double QTextFrameFormat::leftMargin();
  fn _ZNK16QTextFrameFormat10leftMarginEv(qthis: *mut c_void) -> c_double;
}

// body block begin
// class sizeof(QTextFrameFormat)=1
pub struct QTextFrameFormat {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextFrameFormat {
  pub fn isValid<RetType, T: QTextFrameFormat_isValid<RetType>>(&mut self, value: T) -> RetType {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_isValid<RetType> {
  fn isValid(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  bool QTextFrameFormat::isValid();
impl<'a> /*trait*/ QTextFrameFormat_isValid<i8> for () {
  fn isValid(self, rsthis: &mut QTextFrameFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat7isValidEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setHeight<RetType, T: QTextFrameFormat_setHeight<RetType>>(&mut self, value: T) -> RetType {
    return value.setHeight(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setHeight<RetType> {
  fn setHeight(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  void QTextFrameFormat::setHeight(qreal height);
impl<'a> /*trait*/ QTextFrameFormat_setHeight<()> for (f64) {
  fn setHeight(self, rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat9setHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat9setHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setBorderBrush<RetType, T: QTextFrameFormat_setBorderBrush<RetType>>(&mut self, value: T) -> RetType {
    return value.setBorderBrush(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setBorderBrush<RetType> {
  fn setBorderBrush(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  void QTextFrameFormat::setBorderBrush(const QBrush & brush);
impl<'a> /*trait*/ QTextFrameFormat_setBorderBrush<()> for (&'a  QBrush) {
  fn setBorderBrush(self, rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat14setBorderBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTextFrameFormat14setBorderBrushERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn margin<RetType, T: QTextFrameFormat_margin<RetType>>(&mut self, value: T) -> RetType {
    return value.margin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_margin<RetType> {
  fn margin(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  double QTextFrameFormat::margin();
impl<'a> /*trait*/ QTextFrameFormat_margin<f64> for () {
  fn margin(self, rsthis: &mut QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat6marginEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat6marginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn borderBrush<RetType, T: QTextFrameFormat_borderBrush<RetType>>(&mut self, value: T) -> RetType {
    return value.borderBrush(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_borderBrush<RetType> {
  fn borderBrush(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  QBrush QTextFrameFormat::borderBrush();
impl<'a> /*trait*/ QTextFrameFormat_borderBrush<QBrush> for () {
  fn borderBrush(self, rsthis: &mut QTextFrameFormat) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat11borderBrushEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat11borderBrushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setRightMargin<RetType, T: QTextFrameFormat_setRightMargin<RetType>>(&mut self, value: T) -> RetType {
    return value.setRightMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setRightMargin<RetType> {
  fn setRightMargin(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  void QTextFrameFormat::setRightMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setRightMargin<()> for (f64) {
  fn setRightMargin(self, rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat14setRightMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat14setRightMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setMargin<RetType, T: QTextFrameFormat_setMargin<RetType>>(&mut self, value: T) -> RetType {
    return value.setMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setMargin<RetType> {
  fn setMargin(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  void QTextFrameFormat::setMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setMargin<()> for (f64) {
  fn setMargin(self, rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat9setMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat9setMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setBorder<RetType, T: QTextFrameFormat_setBorder<RetType>>(&mut self, value: T) -> RetType {
    return value.setBorder(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setBorder<RetType> {
  fn setBorder(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  void QTextFrameFormat::setBorder(qreal border);
impl<'a> /*trait*/ QTextFrameFormat_setBorder<()> for (f64) {
  fn setBorder(self, rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat9setBorderEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat9setBorderEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTextFrameFormat::setHeight(const QTextLength & height);
impl<'a> /*trait*/ QTextFrameFormat_setHeight<()> for (&'a  QTextLength) {
  fn setHeight(self, rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat9setHeightERK11QTextLength()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTextFrameFormat9setHeightERK11QTextLength(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setWidth<RetType, T: QTextFrameFormat_setWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.setWidth(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setWidth<RetType> {
  fn setWidth(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  void QTextFrameFormat::setWidth(const QTextLength & length);
impl<'a> /*trait*/ QTextFrameFormat_setWidth<()> for (&'a  QTextLength) {
  fn setWidth(self, rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat8setWidthERK11QTextLength()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTextFrameFormat8setWidthERK11QTextLength(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn bottomMargin<RetType, T: QTextFrameFormat_bottomMargin<RetType>>(&mut self, value: T) -> RetType {
    return value.bottomMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_bottomMargin<RetType> {
  fn bottomMargin(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  double QTextFrameFormat::bottomMargin();
impl<'a> /*trait*/ QTextFrameFormat_bottomMargin<f64> for () {
  fn bottomMargin(self, rsthis: &mut QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat12bottomMarginEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat12bottomMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setBottomMargin<RetType, T: QTextFrameFormat_setBottomMargin<RetType>>(&mut self, value: T) -> RetType {
    return value.setBottomMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setBottomMargin<RetType> {
  fn setBottomMargin(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  void QTextFrameFormat::setBottomMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setBottomMargin<()> for (f64) {
  fn setBottomMargin(self, rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat15setBottomMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat15setBottomMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn height<RetType, T: QTextFrameFormat_height<RetType>>(&mut self, value: T) -> RetType {
    return value.height(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_height<RetType> {
  fn height(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  QTextLength QTextFrameFormat::height();
impl<'a> /*trait*/ QTextFrameFormat_height<QTextLength> for () {
  fn height(self, rsthis: &mut QTextFrameFormat) -> QTextLength {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat6heightEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat6heightEv(rsthis.qclsinst)};
    let mut ret1 = QTextLength{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTextFrameFormat::setWidth(qreal width);
impl<'a> /*trait*/ QTextFrameFormat_setWidth<()> for (f64) {
  fn setWidth(self, rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat8setWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat8setWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn rightMargin<RetType, T: QTextFrameFormat_rightMargin<RetType>>(&mut self, value: T) -> RetType {
    return value.rightMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_rightMargin<RetType> {
  fn rightMargin(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  double QTextFrameFormat::rightMargin();
impl<'a> /*trait*/ QTextFrameFormat_rightMargin<f64> for () {
  fn rightMargin(self, rsthis: &mut QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat11rightMarginEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat11rightMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setPadding<RetType, T: QTextFrameFormat_setPadding<RetType>>(&mut self, value: T) -> RetType {
    return value.setPadding(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setPadding<RetType> {
  fn setPadding(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  void QTextFrameFormat::setPadding(qreal padding);
impl<'a> /*trait*/ QTextFrameFormat_setPadding<()> for (f64) {
  fn setPadding(self, rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat10setPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat10setPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setTopMargin<RetType, T: QTextFrameFormat_setTopMargin<RetType>>(&mut self, value: T) -> RetType {
    return value.setTopMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setTopMargin<RetType> {
  fn setTopMargin(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  void QTextFrameFormat::setTopMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setTopMargin<()> for (f64) {
  fn setTopMargin(self, rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat12setTopMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat12setTopMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn topMargin<RetType, T: QTextFrameFormat_topMargin<RetType>>(&mut self, value: T) -> RetType {
    return value.topMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_topMargin<RetType> {
  fn topMargin(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  double QTextFrameFormat::topMargin();
impl<'a> /*trait*/ QTextFrameFormat_topMargin<f64> for () {
  fn topMargin(self, rsthis: &mut QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat9topMarginEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat9topMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn width<RetType, T: QTextFrameFormat_width<RetType>>(&mut self, value: T) -> RetType {
    return value.width(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_width<RetType> {
  fn width(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  QTextLength QTextFrameFormat::width();
impl<'a> /*trait*/ QTextFrameFormat_width<QTextLength> for () {
  fn width(self, rsthis: &mut QTextFrameFormat) -> QTextLength {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat5widthEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat5widthEv(rsthis.qclsinst)};
    let mut ret1 = QTextLength{qclsinst: ret};
    return ret1;
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QTextFrameFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextFrameFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn padding<RetType, T: QTextFrameFormat_padding<RetType>>(&mut self, value: T) -> RetType {
    return value.padding(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_padding<RetType> {
  fn padding(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  double QTextFrameFormat::padding();
impl<'a> /*trait*/ QTextFrameFormat_padding<f64> for () {
  fn padding(self, rsthis: &mut QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat7paddingEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat7paddingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn setLeftMargin<RetType, T: QTextFrameFormat_setLeftMargin<RetType>>(&mut self, value: T) -> RetType {
    return value.setLeftMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_setLeftMargin<RetType> {
  fn setLeftMargin(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  void QTextFrameFormat::setLeftMargin(qreal margin);
impl<'a> /*trait*/ QTextFrameFormat_setLeftMargin<()> for (f64) {
  fn setLeftMargin(self, rsthis: &mut QTextFrameFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTextFrameFormat13setLeftMarginEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN16QTextFrameFormat13setLeftMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextFrameFormat {
  pub fn border<RetType, T: QTextFrameFormat_border<RetType>>(&mut self, value: T) -> RetType {
    return value.border(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_border<RetType> {
  fn border(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  double QTextFrameFormat::border();
impl<'a> /*trait*/ QTextFrameFormat_border<f64> for () {
  fn border(self, rsthis: &mut QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat6borderEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat6borderEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
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
  pub fn leftMargin<RetType, T: QTextFrameFormat_leftMargin<RetType>>(&mut self, value: T) -> RetType {
    return value.leftMargin(self);
    // return 1;
  }
}

pub trait QTextFrameFormat_leftMargin<RetType> {
  fn leftMargin(self, rsthis: &mut QTextFrameFormat) -> RetType;
}

// proto:  double QTextFrameFormat::leftMargin();
impl<'a> /*trait*/ QTextFrameFormat_leftMargin<f64> for () {
  fn leftMargin(self, rsthis: &mut QTextFrameFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTextFrameFormat10leftMarginEv()};
    let mut ret = unsafe {_ZNK16QTextFrameFormat10leftMarginEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

