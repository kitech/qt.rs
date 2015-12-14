// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qchar::QChar;
use super::qstring::QString;
use super::qsize::QSize;
use super::qrect::QRect;
use super::qfont::QFont;
use super::qpaintdevice::QPaintDevice;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  int QFontMetrics::maxWidth();
  fn _ZNK12QFontMetrics8maxWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QFontMetrics::FreeQFontMetrics();
  fn _ZN12QFontMetricsD0Ev(qthis: *mut c_void) ;
  // proto:  int QFontMetrics::lineWidth();
  fn _ZNK12QFontMetrics9lineWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  int QFontMetrics::minLeftBearing();
  fn _ZNK12QFontMetrics14minLeftBearingEv(qthis: *mut c_void) -> c_int;
  // proto:  int QFontMetrics::rightBearing(QChar );
  fn _ZNK12QFontMetrics12rightBearingE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  int QFontMetrics::ascent();
  fn _ZNK12QFontMetrics6ascentEv(qthis: *mut c_void) -> c_int;
  // proto:  QSize QFontMetrics::size(int flags, const QString & str, int tabstops, int * tabarray);
  fn _ZNK12QFontMetrics4sizeEiRK7QStringiPi(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_int, arg3: *mut c_int) -> *mut c_void;
  // proto:  int QFontMetrics::overlinePos();
  fn _ZNK12QFontMetrics11overlinePosEv(qthis: *mut c_void) -> c_int;
  // proto:  int QFontMetrics::leading();
  fn _ZNK12QFontMetrics7leadingEv(qthis: *mut c_void) -> c_int;
  // proto:  QRect QFontMetrics::tightBoundingRect(const QString & text);
  fn _ZNK12QFontMetrics17tightBoundingRectERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QFontMetrics::averageCharWidth();
  fn _ZNK12QFontMetrics16averageCharWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  int QFontMetrics::underlinePos();
  fn _ZNK12QFontMetrics12underlinePosEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QFontMetrics::inFont(QChar );
  fn _ZNK12QFontMetrics6inFontE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  int QFontMetrics::height();
  fn _ZNK12QFontMetrics6heightEv(qthis: *mut c_void) -> c_int;
  // proto:  int QFontMetrics::width(QChar );
  fn _ZNK12QFontMetrics5widthE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  int QFontMetrics::xHeight();
  fn _ZNK12QFontMetrics7xHeightEv(qthis: *mut c_void) -> c_int;
  // proto:  int QFontMetrics::width(const QString & , int len, int flags);
  fn _ZNK12QFontMetrics5widthERK7QStringii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) -> c_int;
  // proto:  int QFontMetrics::strikeOutPos();
  fn _ZNK12QFontMetrics12strikeOutPosEv(qthis: *mut c_void) -> c_int;
  // proto:  int QFontMetrics::lineSpacing();
  fn _ZNK12QFontMetrics11lineSpacingEv(qthis: *mut c_void) -> c_int;
  // proto:  void QFontMetrics::NewQFontMetrics(const QFontMetrics & );
  fn _ZN12QFontMetricsC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFontMetrics::NewQFontMetrics(const QFont & , QPaintDevice * pd);
  fn _ZN12QFontMetricsC1ERK5QFontP12QPaintDevice(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  int QFontMetrics::minRightBearing();
  fn _ZNK12QFontMetrics15minRightBearingEv(qthis: *mut c_void) -> c_int;
  // proto:  void QFontMetrics::swap(QFontMetrics & other);
  fn _ZN12QFontMetrics4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFontMetrics::NewQFontMetrics(const QFont & );
  fn _ZN12QFontMetricsC1ERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QFontMetrics::width(const QString & , int len);
  fn _ZNK12QFontMetrics5widthERK7QStringi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QFontMetrics::charWidth(const QString & str, int pos);
  fn _ZNK12QFontMetrics9charWidthERK7QStringi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QFontMetrics::leftBearing(QChar );
  fn _ZNK12QFontMetrics11leftBearingE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  bool QFontMetrics::inFontUcs4(uint ucs4);
  fn _ZNK12QFontMetrics10inFontUcs4Ej(qthis: *mut c_void, arg0: c_uint) -> int8_t;
  // proto:  int QFontMetrics::descent();
  fn _ZNK12QFontMetrics7descentEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QFontMetrics)=1
pub struct QFontMetrics {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFontMetrics {
  pub fn maxWidth<T: QFontMetrics_maxWidth>(&mut self, value: T) -> i32 {
    return value.maxWidth(self);
    // return 1;
  }
}

pub trait QFontMetrics_maxWidth {
  fn maxWidth(self, rsthis: &mut QFontMetrics) -> i32;
}

// proto:  int QFontMetrics::maxWidth();
impl<'a> /*trait*/ QFontMetrics_maxWidth for () {
  fn maxWidth(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics8maxWidthEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics8maxWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn FreeQFontMetrics<T: QFontMetrics_FreeQFontMetrics>(&mut self, value: T)  {
     value.FreeQFontMetrics(self);
    // return 1;
  }
}

pub trait QFontMetrics_FreeQFontMetrics {
  fn FreeQFontMetrics(self, rsthis: &mut QFontMetrics) ;
}

// proto:  void QFontMetrics::FreeQFontMetrics();
impl<'a> /*trait*/ QFontMetrics_FreeQFontMetrics for () {
  fn FreeQFontMetrics(self, rsthis: &mut QFontMetrics)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QFontMetricsD0Ev()};
     unsafe {_ZN12QFontMetricsD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn lineWidth<T: QFontMetrics_lineWidth>(&mut self, value: T) -> i32 {
    return value.lineWidth(self);
    // return 1;
  }
}

pub trait QFontMetrics_lineWidth {
  fn lineWidth(self, rsthis: &mut QFontMetrics) -> i32;
}

// proto:  int QFontMetrics::lineWidth();
impl<'a> /*trait*/ QFontMetrics_lineWidth for () {
  fn lineWidth(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics9lineWidthEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics9lineWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn minLeftBearing<T: QFontMetrics_minLeftBearing>(&mut self, value: T) -> i32 {
    return value.minLeftBearing(self);
    // return 1;
  }
}

pub trait QFontMetrics_minLeftBearing {
  fn minLeftBearing(self, rsthis: &mut QFontMetrics) -> i32;
}

// proto:  int QFontMetrics::minLeftBearing();
impl<'a> /*trait*/ QFontMetrics_minLeftBearing for () {
  fn minLeftBearing(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics14minLeftBearingEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics14minLeftBearingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn rightBearing<T: QFontMetrics_rightBearing>(&mut self, value: T) -> i32 {
    return value.rightBearing(self);
    // return 1;
  }
}

pub trait QFontMetrics_rightBearing {
  fn rightBearing(self, rsthis: &mut QFontMetrics) -> i32;
}

// proto:  int QFontMetrics::rightBearing(QChar );
impl<'a> /*trait*/ QFontMetrics_rightBearing for (QChar) {
  fn rightBearing(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics12rightBearingE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QFontMetrics12rightBearingE5QChar(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn ascent<T: QFontMetrics_ascent>(&mut self, value: T) -> i32 {
    return value.ascent(self);
    // return 1;
  }
}

pub trait QFontMetrics_ascent {
  fn ascent(self, rsthis: &mut QFontMetrics) -> i32;
}

// proto:  int QFontMetrics::ascent();
impl<'a> /*trait*/ QFontMetrics_ascent for () {
  fn ascent(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics6ascentEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics6ascentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn size<T: QFontMetrics_size>(&mut self, value: T) -> QSize {
    return value.size(self);
    // return 1;
  }
}

pub trait QFontMetrics_size {
  fn size(self, rsthis: &mut QFontMetrics) -> QSize;
}

// proto:  QSize QFontMetrics::size(int flags, const QString & str, int tabstops, int * tabarray);
impl<'a> /*trait*/ QFontMetrics_size for (i32, &'a  QString, i32, &'a mut i32) {
  fn size(self, rsthis: &mut QFontMetrics) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics4sizeEiRK7QStringiPi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as *mut c_int;
    let mut ret = unsafe {_ZNK12QFontMetrics4sizeEiRK7QStringiPi(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn overlinePos<T: QFontMetrics_overlinePos>(&mut self, value: T) -> i32 {
    return value.overlinePos(self);
    // return 1;
  }
}

pub trait QFontMetrics_overlinePos {
  fn overlinePos(self, rsthis: &mut QFontMetrics) -> i32;
}

// proto:  int QFontMetrics::overlinePos();
impl<'a> /*trait*/ QFontMetrics_overlinePos for () {
  fn overlinePos(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics11overlinePosEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics11overlinePosEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn leading<T: QFontMetrics_leading>(&mut self, value: T) -> i32 {
    return value.leading(self);
    // return 1;
  }
}

pub trait QFontMetrics_leading {
  fn leading(self, rsthis: &mut QFontMetrics) -> i32;
}

// proto:  int QFontMetrics::leading();
impl<'a> /*trait*/ QFontMetrics_leading for () {
  fn leading(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics7leadingEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics7leadingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn tightBoundingRect<T: QFontMetrics_tightBoundingRect>(&mut self, value: T) -> QRect {
    return value.tightBoundingRect(self);
    // return 1;
  }
}

pub trait QFontMetrics_tightBoundingRect {
  fn tightBoundingRect(self, rsthis: &mut QFontMetrics) -> QRect;
}

// proto:  QRect QFontMetrics::tightBoundingRect(const QString & text);
impl<'a> /*trait*/ QFontMetrics_tightBoundingRect for (&'a  QString) {
  fn tightBoundingRect(self, rsthis: &mut QFontMetrics) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics17tightBoundingRectERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QFontMetrics17tightBoundingRectERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn averageCharWidth<T: QFontMetrics_averageCharWidth>(&mut self, value: T) -> i32 {
    return value.averageCharWidth(self);
    // return 1;
  }
}

pub trait QFontMetrics_averageCharWidth {
  fn averageCharWidth(self, rsthis: &mut QFontMetrics) -> i32;
}

// proto:  int QFontMetrics::averageCharWidth();
impl<'a> /*trait*/ QFontMetrics_averageCharWidth for () {
  fn averageCharWidth(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics16averageCharWidthEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics16averageCharWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn underlinePos<T: QFontMetrics_underlinePos>(&mut self, value: T) -> i32 {
    return value.underlinePos(self);
    // return 1;
  }
}

pub trait QFontMetrics_underlinePos {
  fn underlinePos(self, rsthis: &mut QFontMetrics) -> i32;
}

// proto:  int QFontMetrics::underlinePos();
impl<'a> /*trait*/ QFontMetrics_underlinePos for () {
  fn underlinePos(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics12underlinePosEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics12underlinePosEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn inFont<T: QFontMetrics_inFont>(&mut self, value: T) -> i8 {
    return value.inFont(self);
    // return 1;
  }
}

pub trait QFontMetrics_inFont {
  fn inFont(self, rsthis: &mut QFontMetrics) -> i8;
}

// proto:  bool QFontMetrics::inFont(QChar );
impl<'a> /*trait*/ QFontMetrics_inFont for (QChar) {
  fn inFont(self, rsthis: &mut QFontMetrics) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics6inFontE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QFontMetrics6inFontE5QChar(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn height<T: QFontMetrics_height>(&mut self, value: T) -> i32 {
    return value.height(self);
    // return 1;
  }
}

pub trait QFontMetrics_height {
  fn height(self, rsthis: &mut QFontMetrics) -> i32;
}

// proto:  int QFontMetrics::height();
impl<'a> /*trait*/ QFontMetrics_height for () {
  fn height(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics6heightEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn width<T: QFontMetrics_width>(&mut self, value: T) -> i32 {
    return value.width(self);
    // return 1;
  }
}

pub trait QFontMetrics_width {
  fn width(self, rsthis: &mut QFontMetrics) -> i32;
}

// proto:  int QFontMetrics::width(QChar );
impl<'a> /*trait*/ QFontMetrics_width for (QChar) {
  fn width(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics5widthE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QFontMetrics5widthE5QChar(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn xHeight<T: QFontMetrics_xHeight>(&mut self, value: T) -> i32 {
    return value.xHeight(self);
    // return 1;
  }
}

pub trait QFontMetrics_xHeight {
  fn xHeight(self, rsthis: &mut QFontMetrics) -> i32;
}

// proto:  int QFontMetrics::xHeight();
impl<'a> /*trait*/ QFontMetrics_xHeight for () {
  fn xHeight(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics7xHeightEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics7xHeightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QFontMetrics::width(const QString & , int len, int flags);
impl<'a> /*trait*/ QFontMetrics_width for (&'a  QString, i32, i32) {
  fn width(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics5widthERK7QStringii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZNK12QFontMetrics5widthERK7QStringii(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn strikeOutPos<T: QFontMetrics_strikeOutPos>(&mut self, value: T) -> i32 {
    return value.strikeOutPos(self);
    // return 1;
  }
}

pub trait QFontMetrics_strikeOutPos {
  fn strikeOutPos(self, rsthis: &mut QFontMetrics) -> i32;
}

// proto:  int QFontMetrics::strikeOutPos();
impl<'a> /*trait*/ QFontMetrics_strikeOutPos for () {
  fn strikeOutPos(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics12strikeOutPosEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics12strikeOutPosEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn lineSpacing<T: QFontMetrics_lineSpacing>(&mut self, value: T) -> i32 {
    return value.lineSpacing(self);
    // return 1;
  }
}

pub trait QFontMetrics_lineSpacing {
  fn lineSpacing(self, rsthis: &mut QFontMetrics) -> i32;
}

// proto:  int QFontMetrics::lineSpacing();
impl<'a> /*trait*/ QFontMetrics_lineSpacing for () {
  fn lineSpacing(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics11lineSpacingEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics11lineSpacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn NewQFontMetrics<T: QFontMetrics_NewQFontMetrics>(value: T) -> QFontMetrics {
    let rsthis = value.NewQFontMetrics();
    return rsthis;
    // return 1;
  }
}

pub trait QFontMetrics_NewQFontMetrics {
  fn NewQFontMetrics(self) -> QFontMetrics;
}

// proto: void QFontMetrics::NewQFontMetrics(const QFontMetrics & );
impl<'a> /*trait*/ QFontMetrics_NewQFontMetrics for (&'a  QFontMetrics) {
  fn NewQFontMetrics(self) -> QFontMetrics {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QFontMetricsC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QFontMetricsC1ERKS_(qthis, arg0)};
    let rsthis = QFontMetrics{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QFontMetrics::NewQFontMetrics(const QFont & , QPaintDevice * pd);
impl<'a> /*trait*/ QFontMetrics_NewQFontMetrics for (&'a  QFont, &'a mut QPaintDevice) {
  fn NewQFontMetrics(self) -> QFontMetrics {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QFontMetricsC1ERK5QFontP12QPaintDevice()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QFontMetricsC1ERK5QFontP12QPaintDevice(qthis, arg0, arg1)};
    let rsthis = QFontMetrics{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn minRightBearing<T: QFontMetrics_minRightBearing>(&mut self, value: T) -> i32 {
    return value.minRightBearing(self);
    // return 1;
  }
}

pub trait QFontMetrics_minRightBearing {
  fn minRightBearing(self, rsthis: &mut QFontMetrics) -> i32;
}

// proto:  int QFontMetrics::minRightBearing();
impl<'a> /*trait*/ QFontMetrics_minRightBearing for () {
  fn minRightBearing(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics15minRightBearingEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics15minRightBearingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn swap<T: QFontMetrics_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QFontMetrics_swap {
  fn swap(self, rsthis: &mut QFontMetrics) ;
}

// proto:  void QFontMetrics::swap(QFontMetrics & other);
impl<'a> /*trait*/ QFontMetrics_swap for (&'a mut QFontMetrics) {
  fn swap(self, rsthis: &mut QFontMetrics)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QFontMetrics4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QFontMetrics4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QFontMetrics::NewQFontMetrics(const QFont & );
impl<'a> /*trait*/ QFontMetrics_NewQFontMetrics for (&'a  QFont) {
  fn NewQFontMetrics(self) -> QFontMetrics {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QFontMetricsC1ERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QFontMetricsC1ERK5QFont(qthis, arg0)};
    let rsthis = QFontMetrics{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  int QFontMetrics::width(const QString & , int len);
impl<'a> /*trait*/ QFontMetrics_width for (&'a  QString, i32) {
  fn width(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics5widthERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK12QFontMetrics5widthERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn charWidth<T: QFontMetrics_charWidth>(&mut self, value: T) -> i32 {
    return value.charWidth(self);
    // return 1;
  }
}

pub trait QFontMetrics_charWidth {
  fn charWidth(self, rsthis: &mut QFontMetrics) -> i32;
}

// proto:  int QFontMetrics::charWidth(const QString & str, int pos);
impl<'a> /*trait*/ QFontMetrics_charWidth for (&'a  QString, i32) {
  fn charWidth(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics9charWidthERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK12QFontMetrics9charWidthERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn leftBearing<T: QFontMetrics_leftBearing>(&mut self, value: T) -> i32 {
    return value.leftBearing(self);
    // return 1;
  }
}

pub trait QFontMetrics_leftBearing {
  fn leftBearing(self, rsthis: &mut QFontMetrics) -> i32;
}

// proto:  int QFontMetrics::leftBearing(QChar );
impl<'a> /*trait*/ QFontMetrics_leftBearing for (QChar) {
  fn leftBearing(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics11leftBearingE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QFontMetrics11leftBearingE5QChar(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn inFontUcs4<T: QFontMetrics_inFontUcs4>(&mut self, value: T) -> i8 {
    return value.inFontUcs4(self);
    // return 1;
  }
}

pub trait QFontMetrics_inFontUcs4 {
  fn inFontUcs4(self, rsthis: &mut QFontMetrics) -> i8;
}

// proto:  bool QFontMetrics::inFontUcs4(uint ucs4);
impl<'a> /*trait*/ QFontMetrics_inFontUcs4 for (u32) {
  fn inFontUcs4(self, rsthis: &mut QFontMetrics) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics10inFontUcs4Ej()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZNK12QFontMetrics10inFontUcs4Ej(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn descent<T: QFontMetrics_descent>(&mut self, value: T) -> i32 {
    return value.descent(self);
    // return 1;
  }
}

pub trait QFontMetrics_descent {
  fn descent(self, rsthis: &mut QFontMetrics) -> i32;
}

// proto:  int QFontMetrics::descent();
impl<'a> /*trait*/ QFontMetrics_descent for () {
  fn descent(self, rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics7descentEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics7descentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

