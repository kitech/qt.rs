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
use super::qsizef::QSizeF;
use super::qrectf::QRectF;
use super::qfontmetrics::QFontMetrics;
use super::qfont::QFont;
use super::qpaintdevice::QPaintDevice;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QFontMetricsF::inFont(QChar );
  fn _ZNK13QFontMetricsF6inFontE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QSizeF QFontMetricsF::size(int flags, const QString & str, int tabstops, int * tabarray);
  fn _ZNK13QFontMetricsF4sizeEiRK7QStringiPi(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_int, arg3: *mut c_int) -> *mut c_void;
  // proto:  double QFontMetricsF::minRightBearing();
  fn _ZNK13QFontMetricsF15minRightBearingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QFontMetricsF::NewQFontMetricsF(const QFontMetricsF & );
  fn _ZN13QFontMetricsFC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QFontMetricsF::xHeight();
  fn _ZNK13QFontMetricsF7xHeightEv(qthis: *mut c_void) -> c_double;
  // proto:  double QFontMetricsF::width(QChar );
  fn _ZNK13QFontMetricsF5widthE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> c_double;
  // proto:  void QFontMetricsF::FreeQFontMetricsF();
  fn _ZN13QFontMetricsFD0Ev(qthis: *mut c_void) ;
  // proto:  void QFontMetricsF::swap(QFontMetricsF & other);
  fn _ZN13QFontMetricsF4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRectF QFontMetricsF::tightBoundingRect(const QString & text);
  fn _ZNK13QFontMetricsF17tightBoundingRectERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  double QFontMetricsF::leftBearing(QChar );
  fn _ZNK13QFontMetricsF11leftBearingE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> c_double;
  // proto:  double QFontMetricsF::rightBearing(QChar );
  fn _ZNK13QFontMetricsF12rightBearingE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> c_double;
  // proto:  double QFontMetricsF::overlinePos();
  fn _ZNK13QFontMetricsF11overlinePosEv(qthis: *mut c_void) -> c_double;
  // proto:  double QFontMetricsF::height();
  fn _ZNK13QFontMetricsF6heightEv(qthis: *mut c_void) -> c_double;
  // proto:  double QFontMetricsF::descent();
  fn _ZNK13QFontMetricsF7descentEv(qthis: *mut c_void) -> c_double;
  // proto:  double QFontMetricsF::lineWidth();
  fn _ZNK13QFontMetricsF9lineWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QFontMetricsF::NewQFontMetricsF(const QFontMetrics & );
  fn _ZN13QFontMetricsFC1ERK12QFontMetrics(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QFontMetricsF::width(const QString & string);
  fn _ZNK13QFontMetricsF5widthERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_double;
  // proto:  double QFontMetricsF::strikeOutPos();
  fn _ZNK13QFontMetricsF12strikeOutPosEv(qthis: *mut c_void) -> c_double;
  // proto:  double QFontMetricsF::lineSpacing();
  fn _ZNK13QFontMetricsF11lineSpacingEv(qthis: *mut c_void) -> c_double;
  // proto:  double QFontMetricsF::averageCharWidth();
  fn _ZNK13QFontMetricsF16averageCharWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QFontMetricsF::NewQFontMetricsF(const QFont & , QPaintDevice * pd);
  fn _ZN13QFontMetricsFC1ERK5QFontP12QPaintDevice(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  double QFontMetricsF::leading();
  fn _ZNK13QFontMetricsF7leadingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QFontMetricsF::NewQFontMetricsF(const QFont & );
  fn _ZN13QFontMetricsFC1ERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QFontMetricsF::inFontUcs4(uint ucs4);
  fn _ZNK13QFontMetricsF10inFontUcs4Ej(qthis: *mut c_void, arg0: c_uint) -> int8_t;
  // proto:  double QFontMetricsF::minLeftBearing();
  fn _ZNK13QFontMetricsF14minLeftBearingEv(qthis: *mut c_void) -> c_double;
  // proto:  double QFontMetricsF::ascent();
  fn _ZNK13QFontMetricsF6ascentEv(qthis: *mut c_void) -> c_double;
  // proto:  double QFontMetricsF::maxWidth();
  fn _ZNK13QFontMetricsF8maxWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  double QFontMetricsF::underlinePos();
  fn _ZNK13QFontMetricsF12underlinePosEv(qthis: *mut c_void) -> c_double;
}

// body block begin
// class sizeof(QFontMetricsF)=1
pub struct QFontMetricsF {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFontMetricsF {
  pub fn inFont<T: QFontMetricsF_inFont>(&mut self, value: T) -> i8 {
    return value.inFont(self);
    // return 1;
  }
}

pub trait QFontMetricsF_inFont {
  fn inFont(self, rsthis: &mut QFontMetricsF) -> i8;
}

// proto:  bool QFontMetricsF::inFont(QChar );
impl<'a> /*trait*/ QFontMetricsF_inFont for (QChar) {
  fn inFont(self, rsthis: &mut QFontMetricsF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF6inFontE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontMetricsF6inFontE5QChar(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn size<T: QFontMetricsF_size>(&mut self, value: T) -> QSizeF {
    return value.size(self);
    // return 1;
  }
}

pub trait QFontMetricsF_size {
  fn size(self, rsthis: &mut QFontMetricsF) -> QSizeF;
}

// proto:  QSizeF QFontMetricsF::size(int flags, const QString & str, int tabstops, int * tabarray);
impl<'a> /*trait*/ QFontMetricsF_size for (i32, &'a  QString, i32, &'a mut i32) {
  fn size(self, rsthis: &mut QFontMetricsF) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF4sizeEiRK7QStringiPi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as *mut c_int;
    let mut ret = unsafe {_ZNK13QFontMetricsF4sizeEiRK7QStringiPi(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn minRightBearing<T: QFontMetricsF_minRightBearing>(&mut self, value: T) -> f64 {
    return value.minRightBearing(self);
    // return 1;
  }
}

pub trait QFontMetricsF_minRightBearing {
  fn minRightBearing(self, rsthis: &mut QFontMetricsF) -> f64;
}

// proto:  double QFontMetricsF::minRightBearing();
impl<'a> /*trait*/ QFontMetricsF_minRightBearing for () {
  fn minRightBearing(self, rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF15minRightBearingEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF15minRightBearingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn NewQFontMetricsF<T: QFontMetricsF_NewQFontMetricsF>(value: T) -> QFontMetricsF {
    let rsthis = value.NewQFontMetricsF();
    return rsthis;
    // return 1;
  }
}

pub trait QFontMetricsF_NewQFontMetricsF {
  fn NewQFontMetricsF(self) -> QFontMetricsF;
}

// proto: void QFontMetricsF::NewQFontMetricsF(const QFontMetricsF & );
impl<'a> /*trait*/ QFontMetricsF_NewQFontMetricsF for (&'a  QFontMetricsF) {
  fn NewQFontMetricsF(self) -> QFontMetricsF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontMetricsFC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QFontMetricsFC1ERKS_(qthis, arg0)};
    let rsthis = QFontMetricsF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn xHeight<T: QFontMetricsF_xHeight>(&mut self, value: T) -> f64 {
    return value.xHeight(self);
    // return 1;
  }
}

pub trait QFontMetricsF_xHeight {
  fn xHeight(self, rsthis: &mut QFontMetricsF) -> f64;
}

// proto:  double QFontMetricsF::xHeight();
impl<'a> /*trait*/ QFontMetricsF_xHeight for () {
  fn xHeight(self, rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF7xHeightEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF7xHeightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn width<T: QFontMetricsF_width>(&mut self, value: T) -> f64 {
    return value.width(self);
    // return 1;
  }
}

pub trait QFontMetricsF_width {
  fn width(self, rsthis: &mut QFontMetricsF) -> f64;
}

// proto:  double QFontMetricsF::width(QChar );
impl<'a> /*trait*/ QFontMetricsF_width for (QChar) {
  fn width(self, rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF5widthE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontMetricsF5widthE5QChar(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn FreeQFontMetricsF<T: QFontMetricsF_FreeQFontMetricsF>(&mut self, value: T)  {
     value.FreeQFontMetricsF(self);
    // return 1;
  }
}

pub trait QFontMetricsF_FreeQFontMetricsF {
  fn FreeQFontMetricsF(self, rsthis: &mut QFontMetricsF) ;
}

// proto:  void QFontMetricsF::FreeQFontMetricsF();
impl<'a> /*trait*/ QFontMetricsF_FreeQFontMetricsF for () {
  fn FreeQFontMetricsF(self, rsthis: &mut QFontMetricsF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontMetricsFD0Ev()};
     unsafe {_ZN13QFontMetricsFD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn swap<T: QFontMetricsF_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QFontMetricsF_swap {
  fn swap(self, rsthis: &mut QFontMetricsF) ;
}

// proto:  void QFontMetricsF::swap(QFontMetricsF & other);
impl<'a> /*trait*/ QFontMetricsF_swap for (&'a mut QFontMetricsF) {
  fn swap(self, rsthis: &mut QFontMetricsF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontMetricsF4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QFontMetricsF4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn tightBoundingRect<T: QFontMetricsF_tightBoundingRect>(&mut self, value: T) -> QRectF {
    return value.tightBoundingRect(self);
    // return 1;
  }
}

pub trait QFontMetricsF_tightBoundingRect {
  fn tightBoundingRect(self, rsthis: &mut QFontMetricsF) -> QRectF;
}

// proto:  QRectF QFontMetricsF::tightBoundingRect(const QString & text);
impl<'a> /*trait*/ QFontMetricsF_tightBoundingRect for (&'a  QString) {
  fn tightBoundingRect(self, rsthis: &mut QFontMetricsF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF17tightBoundingRectERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontMetricsF17tightBoundingRectERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn leftBearing<T: QFontMetricsF_leftBearing>(&mut self, value: T) -> f64 {
    return value.leftBearing(self);
    // return 1;
  }
}

pub trait QFontMetricsF_leftBearing {
  fn leftBearing(self, rsthis: &mut QFontMetricsF) -> f64;
}

// proto:  double QFontMetricsF::leftBearing(QChar );
impl<'a> /*trait*/ QFontMetricsF_leftBearing for (QChar) {
  fn leftBearing(self, rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF11leftBearingE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontMetricsF11leftBearingE5QChar(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn rightBearing<T: QFontMetricsF_rightBearing>(&mut self, value: T) -> f64 {
    return value.rightBearing(self);
    // return 1;
  }
}

pub trait QFontMetricsF_rightBearing {
  fn rightBearing(self, rsthis: &mut QFontMetricsF) -> f64;
}

// proto:  double QFontMetricsF::rightBearing(QChar );
impl<'a> /*trait*/ QFontMetricsF_rightBearing for (QChar) {
  fn rightBearing(self, rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF12rightBearingE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontMetricsF12rightBearingE5QChar(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn overlinePos<T: QFontMetricsF_overlinePos>(&mut self, value: T) -> f64 {
    return value.overlinePos(self);
    // return 1;
  }
}

pub trait QFontMetricsF_overlinePos {
  fn overlinePos(self, rsthis: &mut QFontMetricsF) -> f64;
}

// proto:  double QFontMetricsF::overlinePos();
impl<'a> /*trait*/ QFontMetricsF_overlinePos for () {
  fn overlinePos(self, rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF11overlinePosEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF11overlinePosEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn height<T: QFontMetricsF_height>(&mut self, value: T) -> f64 {
    return value.height(self);
    // return 1;
  }
}

pub trait QFontMetricsF_height {
  fn height(self, rsthis: &mut QFontMetricsF) -> f64;
}

// proto:  double QFontMetricsF::height();
impl<'a> /*trait*/ QFontMetricsF_height for () {
  fn height(self, rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF6heightEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF6heightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn descent<T: QFontMetricsF_descent>(&mut self, value: T) -> f64 {
    return value.descent(self);
    // return 1;
  }
}

pub trait QFontMetricsF_descent {
  fn descent(self, rsthis: &mut QFontMetricsF) -> f64;
}

// proto:  double QFontMetricsF::descent();
impl<'a> /*trait*/ QFontMetricsF_descent for () {
  fn descent(self, rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF7descentEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF7descentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn lineWidth<T: QFontMetricsF_lineWidth>(&mut self, value: T) -> f64 {
    return value.lineWidth(self);
    // return 1;
  }
}

pub trait QFontMetricsF_lineWidth {
  fn lineWidth(self, rsthis: &mut QFontMetricsF) -> f64;
}

// proto:  double QFontMetricsF::lineWidth();
impl<'a> /*trait*/ QFontMetricsF_lineWidth for () {
  fn lineWidth(self, rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF9lineWidthEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF9lineWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto: void QFontMetricsF::NewQFontMetricsF(const QFontMetrics & );
impl<'a> /*trait*/ QFontMetricsF_NewQFontMetricsF for (&'a  QFontMetrics) {
  fn NewQFontMetricsF(self) -> QFontMetricsF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontMetricsFC1ERK12QFontMetrics()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QFontMetricsFC1ERK12QFontMetrics(qthis, arg0)};
    let rsthis = QFontMetricsF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  double QFontMetricsF::width(const QString & string);
impl<'a> /*trait*/ QFontMetricsF_width for (&'a  QString) {
  fn width(self, rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF5widthERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontMetricsF5widthERK7QString(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn strikeOutPos<T: QFontMetricsF_strikeOutPos>(&mut self, value: T) -> f64 {
    return value.strikeOutPos(self);
    // return 1;
  }
}

pub trait QFontMetricsF_strikeOutPos {
  fn strikeOutPos(self, rsthis: &mut QFontMetricsF) -> f64;
}

// proto:  double QFontMetricsF::strikeOutPos();
impl<'a> /*trait*/ QFontMetricsF_strikeOutPos for () {
  fn strikeOutPos(self, rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF12strikeOutPosEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF12strikeOutPosEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn lineSpacing<T: QFontMetricsF_lineSpacing>(&mut self, value: T) -> f64 {
    return value.lineSpacing(self);
    // return 1;
  }
}

pub trait QFontMetricsF_lineSpacing {
  fn lineSpacing(self, rsthis: &mut QFontMetricsF) -> f64;
}

// proto:  double QFontMetricsF::lineSpacing();
impl<'a> /*trait*/ QFontMetricsF_lineSpacing for () {
  fn lineSpacing(self, rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF11lineSpacingEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF11lineSpacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn averageCharWidth<T: QFontMetricsF_averageCharWidth>(&mut self, value: T) -> f64 {
    return value.averageCharWidth(self);
    // return 1;
  }
}

pub trait QFontMetricsF_averageCharWidth {
  fn averageCharWidth(self, rsthis: &mut QFontMetricsF) -> f64;
}

// proto:  double QFontMetricsF::averageCharWidth();
impl<'a> /*trait*/ QFontMetricsF_averageCharWidth for () {
  fn averageCharWidth(self, rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF16averageCharWidthEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF16averageCharWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto: void QFontMetricsF::NewQFontMetricsF(const QFont & , QPaintDevice * pd);
impl<'a> /*trait*/ QFontMetricsF_NewQFontMetricsF for (&'a  QFont, &'a mut QPaintDevice) {
  fn NewQFontMetricsF(self) -> QFontMetricsF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontMetricsFC1ERK5QFontP12QPaintDevice()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QFontMetricsFC1ERK5QFontP12QPaintDevice(qthis, arg0, arg1)};
    let rsthis = QFontMetricsF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn leading<T: QFontMetricsF_leading>(&mut self, value: T) -> f64 {
    return value.leading(self);
    // return 1;
  }
}

pub trait QFontMetricsF_leading {
  fn leading(self, rsthis: &mut QFontMetricsF) -> f64;
}

// proto:  double QFontMetricsF::leading();
impl<'a> /*trait*/ QFontMetricsF_leading for () {
  fn leading(self, rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF7leadingEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF7leadingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto: void QFontMetricsF::NewQFontMetricsF(const QFont & );
impl<'a> /*trait*/ QFontMetricsF_NewQFontMetricsF for (&'a  QFont) {
  fn NewQFontMetricsF(self) -> QFontMetricsF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontMetricsFC1ERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QFontMetricsFC1ERK5QFont(qthis, arg0)};
    let rsthis = QFontMetricsF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn inFontUcs4<T: QFontMetricsF_inFontUcs4>(&mut self, value: T) -> i8 {
    return value.inFontUcs4(self);
    // return 1;
  }
}

pub trait QFontMetricsF_inFontUcs4 {
  fn inFontUcs4(self, rsthis: &mut QFontMetricsF) -> i8;
}

// proto:  bool QFontMetricsF::inFontUcs4(uint ucs4);
impl<'a> /*trait*/ QFontMetricsF_inFontUcs4 for (u32) {
  fn inFontUcs4(self, rsthis: &mut QFontMetricsF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF10inFontUcs4Ej()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZNK13QFontMetricsF10inFontUcs4Ej(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn minLeftBearing<T: QFontMetricsF_minLeftBearing>(&mut self, value: T) -> f64 {
    return value.minLeftBearing(self);
    // return 1;
  }
}

pub trait QFontMetricsF_minLeftBearing {
  fn minLeftBearing(self, rsthis: &mut QFontMetricsF) -> f64;
}

// proto:  double QFontMetricsF::minLeftBearing();
impl<'a> /*trait*/ QFontMetricsF_minLeftBearing for () {
  fn minLeftBearing(self, rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF14minLeftBearingEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF14minLeftBearingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn ascent<T: QFontMetricsF_ascent>(&mut self, value: T) -> f64 {
    return value.ascent(self);
    // return 1;
  }
}

pub trait QFontMetricsF_ascent {
  fn ascent(self, rsthis: &mut QFontMetricsF) -> f64;
}

// proto:  double QFontMetricsF::ascent();
impl<'a> /*trait*/ QFontMetricsF_ascent for () {
  fn ascent(self, rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF6ascentEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF6ascentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn maxWidth<T: QFontMetricsF_maxWidth>(&mut self, value: T) -> f64 {
    return value.maxWidth(self);
    // return 1;
  }
}

pub trait QFontMetricsF_maxWidth {
  fn maxWidth(self, rsthis: &mut QFontMetricsF) -> f64;
}

// proto:  double QFontMetricsF::maxWidth();
impl<'a> /*trait*/ QFontMetricsF_maxWidth for () {
  fn maxWidth(self, rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF8maxWidthEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF8maxWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn underlinePos<T: QFontMetricsF_underlinePos>(&mut self, value: T) -> f64 {
    return value.underlinePos(self);
    // return 1;
  }
}

pub trait QFontMetricsF_underlinePos {
  fn underlinePos(self, rsthis: &mut QFontMetricsF) -> f64;
}

// proto:  double QFontMetricsF::underlinePos();
impl<'a> /*trait*/ QFontMetricsF_underlinePos for () {
  fn underlinePos(self, rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF12underlinePosEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF12underlinePosEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

