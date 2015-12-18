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
  // proto:  void QTextTableCellFormat::NewQTextTableCellFormat();
  fn _ZN20QTextTableCellFormatC1Ev(qthis: *mut c_void) ;
  // proto:  void QTextTableCellFormat::setLeftPadding(qreal padding);
  fn _ZN20QTextTableCellFormat14setLeftPaddingEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  bool QTextTableCellFormat::isValid();
  fn _ZNK20QTextTableCellFormat7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextTableCellFormat::setTopPadding(qreal padding);
  fn _ZN20QTextTableCellFormat13setTopPaddingEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QTextTableCellFormat::leftPadding();
  fn _ZNK20QTextTableCellFormat11leftPaddingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextTableCellFormat::setPadding(qreal padding);
  fn _ZN20QTextTableCellFormat10setPaddingEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QTextTableCellFormat::topPadding();
  fn _ZNK20QTextTableCellFormat10topPaddingEv(qthis: *mut c_void) -> c_double;
  // proto:  double QTextTableCellFormat::rightPadding();
  fn _ZNK20QTextTableCellFormat12rightPaddingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextTableCellFormat::NewQTextTableCellFormat(const QTextFormat & fmt);
  fn _ZN20QTextTableCellFormatC1ERK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QTextTableCellFormat::bottomPadding();
  fn _ZNK20QTextTableCellFormat13bottomPaddingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextTableCellFormat::setRightPadding(qreal padding);
  fn _ZN20QTextTableCellFormat15setRightPaddingEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QTextTableCellFormat::setBottomPadding(qreal padding);
  fn _ZN20QTextTableCellFormat16setBottomPaddingEd(qthis: *mut c_void, arg0: c_double) ;
}

// body block begin
// class sizeof(QTextTableCellFormat)=1
pub struct QTextTableCellFormat {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextTableCellFormat {
  pub fn NewQTextTableCellFormat<T: QTextTableCellFormat_NewQTextTableCellFormat>(value: T) -> QTextTableCellFormat {
    let rsthis = value.NewQTextTableCellFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QTextTableCellFormat_NewQTextTableCellFormat {
  fn NewQTextTableCellFormat(self) -> QTextTableCellFormat;
}

// proto: void QTextTableCellFormat::NewQTextTableCellFormat();
impl<'a> /*trait*/ QTextTableCellFormat_NewQTextTableCellFormat for () {
  fn NewQTextTableCellFormat(self) -> QTextTableCellFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormatC1Ev()};
    unsafe {_ZN20QTextTableCellFormatC1Ev(qthis)};
    let rsthis = QTextTableCellFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn setLeftPadding<RetType, T: QTextTableCellFormat_setLeftPadding<RetType>>(&mut self, value: T) -> RetType {
    return value.setLeftPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_setLeftPadding<RetType> {
  fn setLeftPadding(self, rsthis: &mut QTextTableCellFormat) -> RetType;
}

// proto:  void QTextTableCellFormat::setLeftPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setLeftPadding<()> for (f64) {
  fn setLeftPadding(self, rsthis: &mut QTextTableCellFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormat14setLeftPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN20QTextTableCellFormat14setLeftPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn isValid<RetType, T: QTextTableCellFormat_isValid<RetType>>(&mut self, value: T) -> RetType {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_isValid<RetType> {
  fn isValid(self, rsthis: &mut QTextTableCellFormat) -> RetType;
}

// proto:  bool QTextTableCellFormat::isValid();
impl<'a> /*trait*/ QTextTableCellFormat_isValid<i8> for () {
  fn isValid(self, rsthis: &mut QTextTableCellFormat) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QTextTableCellFormat7isValidEv()};
    let mut ret = unsafe {_ZNK20QTextTableCellFormat7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn setTopPadding<RetType, T: QTextTableCellFormat_setTopPadding<RetType>>(&mut self, value: T) -> RetType {
    return value.setTopPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_setTopPadding<RetType> {
  fn setTopPadding(self, rsthis: &mut QTextTableCellFormat) -> RetType;
}

// proto:  void QTextTableCellFormat::setTopPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setTopPadding<()> for (f64) {
  fn setTopPadding(self, rsthis: &mut QTextTableCellFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormat13setTopPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN20QTextTableCellFormat13setTopPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn leftPadding<RetType, T: QTextTableCellFormat_leftPadding<RetType>>(&mut self, value: T) -> RetType {
    return value.leftPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_leftPadding<RetType> {
  fn leftPadding(self, rsthis: &mut QTextTableCellFormat) -> RetType;
}

// proto:  double QTextTableCellFormat::leftPadding();
impl<'a> /*trait*/ QTextTableCellFormat_leftPadding<f64> for () {
  fn leftPadding(self, rsthis: &mut QTextTableCellFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QTextTableCellFormat11leftPaddingEv()};
    let mut ret = unsafe {_ZNK20QTextTableCellFormat11leftPaddingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn setPadding<RetType, T: QTextTableCellFormat_setPadding<RetType>>(&mut self, value: T) -> RetType {
    return value.setPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_setPadding<RetType> {
  fn setPadding(self, rsthis: &mut QTextTableCellFormat) -> RetType;
}

// proto:  void QTextTableCellFormat::setPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setPadding<()> for (f64) {
  fn setPadding(self, rsthis: &mut QTextTableCellFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormat10setPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN20QTextTableCellFormat10setPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn topPadding<RetType, T: QTextTableCellFormat_topPadding<RetType>>(&mut self, value: T) -> RetType {
    return value.topPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_topPadding<RetType> {
  fn topPadding(self, rsthis: &mut QTextTableCellFormat) -> RetType;
}

// proto:  double QTextTableCellFormat::topPadding();
impl<'a> /*trait*/ QTextTableCellFormat_topPadding<f64> for () {
  fn topPadding(self, rsthis: &mut QTextTableCellFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QTextTableCellFormat10topPaddingEv()};
    let mut ret = unsafe {_ZNK20QTextTableCellFormat10topPaddingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn rightPadding<RetType, T: QTextTableCellFormat_rightPadding<RetType>>(&mut self, value: T) -> RetType {
    return value.rightPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_rightPadding<RetType> {
  fn rightPadding(self, rsthis: &mut QTextTableCellFormat) -> RetType;
}

// proto:  double QTextTableCellFormat::rightPadding();
impl<'a> /*trait*/ QTextTableCellFormat_rightPadding<f64> for () {
  fn rightPadding(self, rsthis: &mut QTextTableCellFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QTextTableCellFormat12rightPaddingEv()};
    let mut ret = unsafe {_ZNK20QTextTableCellFormat12rightPaddingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto: void QTextTableCellFormat::NewQTextTableCellFormat(const QTextFormat & fmt);
impl<'a> /*trait*/ QTextTableCellFormat_NewQTextTableCellFormat for (&'a  QTextFormat) {
  fn NewQTextTableCellFormat(self) -> QTextTableCellFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormatC1ERK11QTextFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QTextTableCellFormatC1ERK11QTextFormat(qthis, arg0)};
    let rsthis = QTextTableCellFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn bottomPadding<RetType, T: QTextTableCellFormat_bottomPadding<RetType>>(&mut self, value: T) -> RetType {
    return value.bottomPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_bottomPadding<RetType> {
  fn bottomPadding(self, rsthis: &mut QTextTableCellFormat) -> RetType;
}

// proto:  double QTextTableCellFormat::bottomPadding();
impl<'a> /*trait*/ QTextTableCellFormat_bottomPadding<f64> for () {
  fn bottomPadding(self, rsthis: &mut QTextTableCellFormat) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QTextTableCellFormat13bottomPaddingEv()};
    let mut ret = unsafe {_ZNK20QTextTableCellFormat13bottomPaddingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn setRightPadding<RetType, T: QTextTableCellFormat_setRightPadding<RetType>>(&mut self, value: T) -> RetType {
    return value.setRightPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_setRightPadding<RetType> {
  fn setRightPadding(self, rsthis: &mut QTextTableCellFormat) -> RetType;
}

// proto:  void QTextTableCellFormat::setRightPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setRightPadding<()> for (f64) {
  fn setRightPadding(self, rsthis: &mut QTextTableCellFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormat15setRightPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN20QTextTableCellFormat15setRightPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextTableCellFormat {
  pub fn setBottomPadding<RetType, T: QTextTableCellFormat_setBottomPadding<RetType>>(&mut self, value: T) -> RetType {
    return value.setBottomPadding(self);
    // return 1;
  }
}

pub trait QTextTableCellFormat_setBottomPadding<RetType> {
  fn setBottomPadding(self, rsthis: &mut QTextTableCellFormat) -> RetType;
}

// proto:  void QTextTableCellFormat::setBottomPadding(qreal padding);
impl<'a> /*trait*/ QTextTableCellFormat_setBottomPadding<()> for (f64) {
  fn setBottomPadding(self, rsthis: &mut QTextTableCellFormat) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextTableCellFormat16setBottomPaddingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN20QTextTableCellFormat16setBottomPaddingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

