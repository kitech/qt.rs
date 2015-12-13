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
use super::qrectf::QRectF;
use super::qfontmetrics::QFontMetrics;
use super::qfont::QFont;
use super::qpaintdevice::QPaintDevice;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QFontMetricsF::inFont(QChar );
  fn _ZNK13QFontMetricsF6inFontE5QChar(arg0: *mut c_void) -> i32;
  // proto: QSizeF QFontMetricsF::size(int flags, const QString & str, int tabstops, int * tabarray);
  fn _ZNK13QFontMetricsF4sizeEiRK7QStringiPi(arg0: c_int, arg1: *const c_void, arg2: c_int, arg3: *mut c_int) -> i32;
  // proto: double QFontMetricsF::minRightBearing();
  fn _ZNK13QFontMetricsF15minRightBearingEv() -> i32;
  // proto: void QFontMetricsF::NewQFontMetricsF(const QFontMetricsF & );
  fn _ZN13QFontMetricsFC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: double QFontMetricsF::xHeight();
  fn _ZNK13QFontMetricsF7xHeightEv() -> i32;
  // proto: double QFontMetricsF::width(QChar );
  fn _ZNK13QFontMetricsF5widthE5QChar(arg0: *mut c_void) -> i32;
  // proto: void QFontMetricsF::FreeQFontMetricsF();
  fn _ZN13QFontMetricsFD0Ev() -> i32;
  // proto: QRectF QFontMetricsF::boundingRect(const QRectF & r, int flags, const QString & string, int tabstops, int * tabarray);
  fn _ZNK13QFontMetricsF12boundingRectERK6QRectFiRK7QStringiPi(arg0: *const c_void, arg1: c_int, arg2: *const c_void, arg3: c_int, arg4: *mut c_int) -> i32;
  // proto: void QFontMetricsF::swap(QFontMetricsF & other);
  fn _ZN13QFontMetricsF4swapERS_(arg0: *mut c_void) -> i32;
  // proto: QRectF QFontMetricsF::tightBoundingRect(const QString & text);
  fn _ZNK13QFontMetricsF17tightBoundingRectERK7QString(arg0: *const c_void) -> i32;
  // proto: double QFontMetricsF::leftBearing(QChar );
  fn _ZNK13QFontMetricsF11leftBearingE5QChar(arg0: *mut c_void) -> i32;
  // proto: double QFontMetricsF::rightBearing(QChar );
  fn _ZNK13QFontMetricsF12rightBearingE5QChar(arg0: *mut c_void) -> i32;
  // proto: double QFontMetricsF::overlinePos();
  fn _ZNK13QFontMetricsF11overlinePosEv() -> i32;
  // proto: double QFontMetricsF::height();
  fn _ZNK13QFontMetricsF6heightEv() -> i32;
  // proto: double QFontMetricsF::descent();
  fn _ZNK13QFontMetricsF7descentEv() -> i32;
  // proto: QRectF QFontMetricsF::boundingRect(const QString & string);
  fn _ZNK13QFontMetricsF12boundingRectERK7QString(arg0: *const c_void) -> i32;
  // proto: double QFontMetricsF::lineWidth();
  fn _ZNK13QFontMetricsF9lineWidthEv() -> i32;
  // proto: void QFontMetricsF::NewQFontMetricsF(const QFontMetrics & );
  fn _ZN13QFontMetricsFC1ERK12QFontMetrics(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: double QFontMetricsF::width(const QString & string);
  fn _ZNK13QFontMetricsF5widthERK7QString(arg0: *const c_void) -> i32;
  // proto: double QFontMetricsF::strikeOutPos();
  fn _ZNK13QFontMetricsF12strikeOutPosEv() -> i32;
  // proto: double QFontMetricsF::lineSpacing();
  fn _ZNK13QFontMetricsF11lineSpacingEv() -> i32;
  // proto: double QFontMetricsF::averageCharWidth();
  fn _ZNK13QFontMetricsF16averageCharWidthEv() -> i32;
  // proto: void QFontMetricsF::NewQFontMetricsF(const QFont & , QPaintDevice * pd);
  fn _ZN13QFontMetricsFC1ERK5QFontP12QPaintDevice(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: double QFontMetricsF::leading();
  fn _ZNK13QFontMetricsF7leadingEv() -> i32;
  // proto: void QFontMetricsF::NewQFontMetricsF(const QFont & );
  fn _ZN13QFontMetricsFC1ERK5QFont(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QRectF QFontMetricsF::boundingRect(QChar );
  fn _ZNK13QFontMetricsF12boundingRectE5QChar(arg0: *mut c_void) -> i32;
  // proto: bool QFontMetricsF::inFontUcs4(uint ucs4);
  fn _ZNK13QFontMetricsF10inFontUcs4Ej(arg0: c_uint) -> i32;
  // proto: double QFontMetricsF::minLeftBearing();
  fn _ZNK13QFontMetricsF14minLeftBearingEv() -> i32;
  // proto: double QFontMetricsF::ascent();
  fn _ZNK13QFontMetricsF6ascentEv() -> i32;
  // proto: double QFontMetricsF::maxWidth();
  fn _ZNK13QFontMetricsF8maxWidthEv() -> i32;
  // proto: double QFontMetricsF::underlinePos();
  fn _ZNK13QFontMetricsF12underlinePosEv() -> i32;
}

// body block begin
// class sizeof(QFontMetricsF)=1
pub struct QFontMetricsF {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFontMetricsF {
  pub fn inFont<T: QFontMetricsF_inFont>(&mut self, value: T) -> i32 {
    value.inFont(self);
    return 1;
  }
}

pub trait QFontMetricsF_inFont {
  fn inFont(self, this: &mut QFontMetricsF) -> i32;
}

// proto: bool QFontMetricsF::inFont(QChar );
impl<'a> /*trait*/ QFontMetricsF_inFont for (QChar) {
  fn inFont(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF6inFontE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK13QFontMetricsF6inFontE5QChar(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn size<T: QFontMetricsF_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QFontMetricsF_size {
  fn size(self, this: &mut QFontMetricsF) -> i32;
}

// proto: QSizeF QFontMetricsF::size(int flags, const QString & str, int tabstops, int * tabarray);
impl<'a> /*trait*/ QFontMetricsF_size for (i32, &'a  QString, i32, &'a mut i32) {
  fn size(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF4sizeEiRK7QStringiPi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as *mut c_int;
    unsafe {_ZNK13QFontMetricsF4sizeEiRK7QStringiPi(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn minRightBearing<T: QFontMetricsF_minRightBearing>(&mut self, value: T) -> i32 {
    value.minRightBearing(self);
    return 1;
  }
}

pub trait QFontMetricsF_minRightBearing {
  fn minRightBearing(self, this: &mut QFontMetricsF) -> i32;
}

// proto: double QFontMetricsF::minRightBearing();
impl<'a> /*trait*/ QFontMetricsF_minRightBearing for () {
  fn minRightBearing(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF15minRightBearingEv()};
    unsafe {_ZNK13QFontMetricsF15minRightBearingEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QFontMetricsFC1ERKS_(qthis, arg0)};
    let rsthis = QFontMetricsF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn xHeight<T: QFontMetricsF_xHeight>(&mut self, value: T) -> i32 {
    value.xHeight(self);
    return 1;
  }
}

pub trait QFontMetricsF_xHeight {
  fn xHeight(self, this: &mut QFontMetricsF) -> i32;
}

// proto: double QFontMetricsF::xHeight();
impl<'a> /*trait*/ QFontMetricsF_xHeight for () {
  fn xHeight(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF7xHeightEv()};
    unsafe {_ZNK13QFontMetricsF7xHeightEv()};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn width<T: QFontMetricsF_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QFontMetricsF_width {
  fn width(self, this: &mut QFontMetricsF) -> i32;
}

// proto: double QFontMetricsF::width(QChar );
impl<'a> /*trait*/ QFontMetricsF_width for (QChar) {
  fn width(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF5widthE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK13QFontMetricsF5widthE5QChar(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn FreeQFontMetricsF<T: QFontMetricsF_FreeQFontMetricsF>(&mut self, value: T) -> i32 {
    value.FreeQFontMetricsF(self);
    return 1;
  }
}

pub trait QFontMetricsF_FreeQFontMetricsF {
  fn FreeQFontMetricsF(self, this: &mut QFontMetricsF) -> i32;
}

// proto: void QFontMetricsF::FreeQFontMetricsF();
impl<'a> /*trait*/ QFontMetricsF_FreeQFontMetricsF for () {
  fn FreeQFontMetricsF(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontMetricsFD0Ev()};
    unsafe {_ZN13QFontMetricsFD0Ev()};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn boundingRect<T: QFontMetricsF_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QFontMetricsF_boundingRect {
  fn boundingRect(self, this: &mut QFontMetricsF) -> i32;
}

// proto: QRectF QFontMetricsF::boundingRect(const QRectF & r, int flags, const QString & string, int tabstops, int * tabarray);
impl<'a> /*trait*/ QFontMetricsF_boundingRect for (&'a  QRectF, i32, &'a  QString, i32, &'a mut i32) {
  fn boundingRect(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF12boundingRectERK6QRectFiRK7QStringiPi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as *mut c_int;
    unsafe {_ZNK13QFontMetricsF12boundingRectERK6QRectFiRK7QStringiPi(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn swap<T: QFontMetricsF_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QFontMetricsF_swap {
  fn swap(self, this: &mut QFontMetricsF) -> i32;
}

// proto: void QFontMetricsF::swap(QFontMetricsF & other);
impl<'a> /*trait*/ QFontMetricsF_swap for (&'a mut QFontMetricsF) {
  fn swap(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontMetricsF4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QFontMetricsF4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn tightBoundingRect<T: QFontMetricsF_tightBoundingRect>(&mut self, value: T) -> i32 {
    value.tightBoundingRect(self);
    return 1;
  }
}

pub trait QFontMetricsF_tightBoundingRect {
  fn tightBoundingRect(self, this: &mut QFontMetricsF) -> i32;
}

// proto: QRectF QFontMetricsF::tightBoundingRect(const QString & text);
impl<'a> /*trait*/ QFontMetricsF_tightBoundingRect for (&'a  QString) {
  fn tightBoundingRect(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF17tightBoundingRectERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QFontMetricsF17tightBoundingRectERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn leftBearing<T: QFontMetricsF_leftBearing>(&mut self, value: T) -> i32 {
    value.leftBearing(self);
    return 1;
  }
}

pub trait QFontMetricsF_leftBearing {
  fn leftBearing(self, this: &mut QFontMetricsF) -> i32;
}

// proto: double QFontMetricsF::leftBearing(QChar );
impl<'a> /*trait*/ QFontMetricsF_leftBearing for (QChar) {
  fn leftBearing(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF11leftBearingE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK13QFontMetricsF11leftBearingE5QChar(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn rightBearing<T: QFontMetricsF_rightBearing>(&mut self, value: T) -> i32 {
    value.rightBearing(self);
    return 1;
  }
}

pub trait QFontMetricsF_rightBearing {
  fn rightBearing(self, this: &mut QFontMetricsF) -> i32;
}

// proto: double QFontMetricsF::rightBearing(QChar );
impl<'a> /*trait*/ QFontMetricsF_rightBearing for (QChar) {
  fn rightBearing(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF12rightBearingE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK13QFontMetricsF12rightBearingE5QChar(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn overlinePos<T: QFontMetricsF_overlinePos>(&mut self, value: T) -> i32 {
    value.overlinePos(self);
    return 1;
  }
}

pub trait QFontMetricsF_overlinePos {
  fn overlinePos(self, this: &mut QFontMetricsF) -> i32;
}

// proto: double QFontMetricsF::overlinePos();
impl<'a> /*trait*/ QFontMetricsF_overlinePos for () {
  fn overlinePos(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF11overlinePosEv()};
    unsafe {_ZNK13QFontMetricsF11overlinePosEv()};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn height<T: QFontMetricsF_height>(&mut self, value: T) -> i32 {
    value.height(self);
    return 1;
  }
}

pub trait QFontMetricsF_height {
  fn height(self, this: &mut QFontMetricsF) -> i32;
}

// proto: double QFontMetricsF::height();
impl<'a> /*trait*/ QFontMetricsF_height for () {
  fn height(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF6heightEv()};
    unsafe {_ZNK13QFontMetricsF6heightEv()};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn descent<T: QFontMetricsF_descent>(&mut self, value: T) -> i32 {
    value.descent(self);
    return 1;
  }
}

pub trait QFontMetricsF_descent {
  fn descent(self, this: &mut QFontMetricsF) -> i32;
}

// proto: double QFontMetricsF::descent();
impl<'a> /*trait*/ QFontMetricsF_descent for () {
  fn descent(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF7descentEv()};
    unsafe {_ZNK13QFontMetricsF7descentEv()};
    return 1;
  }
}

// proto: QRectF QFontMetricsF::boundingRect(const QString & string);
impl<'a> /*trait*/ QFontMetricsF_boundingRect for (&'a  QString) {
  fn boundingRect(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF12boundingRectERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QFontMetricsF12boundingRectERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn lineWidth<T: QFontMetricsF_lineWidth>(&mut self, value: T) -> i32 {
    value.lineWidth(self);
    return 1;
  }
}

pub trait QFontMetricsF_lineWidth {
  fn lineWidth(self, this: &mut QFontMetricsF) -> i32;
}

// proto: double QFontMetricsF::lineWidth();
impl<'a> /*trait*/ QFontMetricsF_lineWidth for () {
  fn lineWidth(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF9lineWidthEv()};
    unsafe {_ZNK13QFontMetricsF9lineWidthEv()};
    return 1;
  }
}

// proto: void QFontMetricsF::NewQFontMetricsF(const QFontMetrics & );
impl<'a> /*trait*/ QFontMetricsF_NewQFontMetricsF for (&'a  QFontMetrics) {
  fn NewQFontMetricsF(self) -> QFontMetricsF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontMetricsFC1ERK12QFontMetrics()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QFontMetricsFC1ERK12QFontMetrics(qthis, arg0)};
    let rsthis = QFontMetricsF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: double QFontMetricsF::width(const QString & string);
impl<'a> /*trait*/ QFontMetricsF_width for (&'a  QString) {
  fn width(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF5widthERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QFontMetricsF5widthERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn strikeOutPos<T: QFontMetricsF_strikeOutPos>(&mut self, value: T) -> i32 {
    value.strikeOutPos(self);
    return 1;
  }
}

pub trait QFontMetricsF_strikeOutPos {
  fn strikeOutPos(self, this: &mut QFontMetricsF) -> i32;
}

// proto: double QFontMetricsF::strikeOutPos();
impl<'a> /*trait*/ QFontMetricsF_strikeOutPos for () {
  fn strikeOutPos(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF12strikeOutPosEv()};
    unsafe {_ZNK13QFontMetricsF12strikeOutPosEv()};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn lineSpacing<T: QFontMetricsF_lineSpacing>(&mut self, value: T) -> i32 {
    value.lineSpacing(self);
    return 1;
  }
}

pub trait QFontMetricsF_lineSpacing {
  fn lineSpacing(self, this: &mut QFontMetricsF) -> i32;
}

// proto: double QFontMetricsF::lineSpacing();
impl<'a> /*trait*/ QFontMetricsF_lineSpacing for () {
  fn lineSpacing(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF11lineSpacingEv()};
    unsafe {_ZNK13QFontMetricsF11lineSpacingEv()};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn averageCharWidth<T: QFontMetricsF_averageCharWidth>(&mut self, value: T) -> i32 {
    value.averageCharWidth(self);
    return 1;
  }
}

pub trait QFontMetricsF_averageCharWidth {
  fn averageCharWidth(self, this: &mut QFontMetricsF) -> i32;
}

// proto: double QFontMetricsF::averageCharWidth();
impl<'a> /*trait*/ QFontMetricsF_averageCharWidth for () {
  fn averageCharWidth(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF16averageCharWidthEv()};
    unsafe {_ZNK13QFontMetricsF16averageCharWidthEv()};
    return 1;
  }
}

// proto: void QFontMetricsF::NewQFontMetricsF(const QFont & , QPaintDevice * pd);
impl<'a> /*trait*/ QFontMetricsF_NewQFontMetricsF for (&'a  QFont, &'a mut QPaintDevice) {
  fn NewQFontMetricsF(self) -> QFontMetricsF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontMetricsFC1ERK5QFontP12QPaintDevice()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QFontMetricsFC1ERK5QFontP12QPaintDevice(qthis, arg0, arg1)};
    let rsthis = QFontMetricsF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn leading<T: QFontMetricsF_leading>(&mut self, value: T) -> i32 {
    value.leading(self);
    return 1;
  }
}

pub trait QFontMetricsF_leading {
  fn leading(self, this: &mut QFontMetricsF) -> i32;
}

// proto: double QFontMetricsF::leading();
impl<'a> /*trait*/ QFontMetricsF_leading for () {
  fn leading(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF7leadingEv()};
    unsafe {_ZNK13QFontMetricsF7leadingEv()};
    return 1;
  }
}

// proto: void QFontMetricsF::NewQFontMetricsF(const QFont & );
impl<'a> /*trait*/ QFontMetricsF_NewQFontMetricsF for (&'a  QFont) {
  fn NewQFontMetricsF(self) -> QFontMetricsF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontMetricsFC1ERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QFontMetricsFC1ERK5QFont(qthis, arg0)};
    let rsthis = QFontMetricsF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: QRectF QFontMetricsF::boundingRect(QChar );
impl<'a> /*trait*/ QFontMetricsF_boundingRect for (QChar) {
  fn boundingRect(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF12boundingRectE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK13QFontMetricsF12boundingRectE5QChar(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn inFontUcs4<T: QFontMetricsF_inFontUcs4>(&mut self, value: T) -> i32 {
    value.inFontUcs4(self);
    return 1;
  }
}

pub trait QFontMetricsF_inFontUcs4 {
  fn inFontUcs4(self, this: &mut QFontMetricsF) -> i32;
}

// proto: bool QFontMetricsF::inFontUcs4(uint ucs4);
impl<'a> /*trait*/ QFontMetricsF_inFontUcs4 for (u32) {
  fn inFontUcs4(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF10inFontUcs4Ej()};
    let arg0 = self  as c_uint;
    unsafe {_ZNK13QFontMetricsF10inFontUcs4Ej(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn minLeftBearing<T: QFontMetricsF_minLeftBearing>(&mut self, value: T) -> i32 {
    value.minLeftBearing(self);
    return 1;
  }
}

pub trait QFontMetricsF_minLeftBearing {
  fn minLeftBearing(self, this: &mut QFontMetricsF) -> i32;
}

// proto: double QFontMetricsF::minLeftBearing();
impl<'a> /*trait*/ QFontMetricsF_minLeftBearing for () {
  fn minLeftBearing(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF14minLeftBearingEv()};
    unsafe {_ZNK13QFontMetricsF14minLeftBearingEv()};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn ascent<T: QFontMetricsF_ascent>(&mut self, value: T) -> i32 {
    value.ascent(self);
    return 1;
  }
}

pub trait QFontMetricsF_ascent {
  fn ascent(self, this: &mut QFontMetricsF) -> i32;
}

// proto: double QFontMetricsF::ascent();
impl<'a> /*trait*/ QFontMetricsF_ascent for () {
  fn ascent(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF6ascentEv()};
    unsafe {_ZNK13QFontMetricsF6ascentEv()};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn maxWidth<T: QFontMetricsF_maxWidth>(&mut self, value: T) -> i32 {
    value.maxWidth(self);
    return 1;
  }
}

pub trait QFontMetricsF_maxWidth {
  fn maxWidth(self, this: &mut QFontMetricsF) -> i32;
}

// proto: double QFontMetricsF::maxWidth();
impl<'a> /*trait*/ QFontMetricsF_maxWidth for () {
  fn maxWidth(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF8maxWidthEv()};
    unsafe {_ZNK13QFontMetricsF8maxWidthEv()};
    return 1;
  }
}

impl /*struct*/ QFontMetricsF {
  pub fn underlinePos<T: QFontMetricsF_underlinePos>(&mut self, value: T) -> i32 {
    value.underlinePos(self);
    return 1;
  }
}

pub trait QFontMetricsF_underlinePos {
  fn underlinePos(self, this: &mut QFontMetricsF) -> i32;
}

// proto: double QFontMetricsF::underlinePos();
impl<'a> /*trait*/ QFontMetricsF_underlinePos for () {
  fn underlinePos(self, this: &mut QFontMetricsF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF12underlinePosEv()};
    unsafe {_ZNK13QFontMetricsF12underlinePosEv()};
    return 1;
  }
}

