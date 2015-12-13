// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrect::QRect;
use super::qstring::QString;
use super::qchar::QChar;
use super::qfont::QFont;
use super::qpaintdevice::QPaintDevice;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: int QFontMetrics::maxWidth();
  fn _ZNK12QFontMetrics8maxWidthEv() -> i32;
  // proto: void QFontMetrics::FreeQFontMetrics();
  fn _ZN12QFontMetricsD0Ev() -> i32;
  // proto: int QFontMetrics::lineWidth();
  fn _ZNK12QFontMetrics9lineWidthEv() -> i32;
  // proto: QRect QFontMetrics::boundingRect(const QRect & r, int flags, const QString & text, int tabstops, int * tabarray);
  fn _ZNK12QFontMetrics12boundingRectERK5QRectiRK7QStringiPi(arg0: *const c_void, arg1: c_int, arg2: *const c_void, arg3: c_int, arg4: *mut c_int) -> i32;
  // proto: int QFontMetrics::minLeftBearing();
  fn _ZNK12QFontMetrics14minLeftBearingEv() -> i32;
  // proto: int QFontMetrics::rightBearing(QChar );
  fn _ZNK12QFontMetrics12rightBearingE5QChar(arg0: *mut c_void) -> i32;
  // proto: int QFontMetrics::ascent();
  fn _ZNK12QFontMetrics6ascentEv() -> i32;
  // proto: QSize QFontMetrics::size(int flags, const QString & str, int tabstops, int * tabarray);
  fn _ZNK12QFontMetrics4sizeEiRK7QStringiPi(arg0: c_int, arg1: *const c_void, arg2: c_int, arg3: *mut c_int) -> i32;
  // proto: int QFontMetrics::overlinePos();
  fn _ZNK12QFontMetrics11overlinePosEv() -> i32;
  // proto: int QFontMetrics::leading();
  fn _ZNK12QFontMetrics7leadingEv() -> i32;
  // proto: QRect QFontMetrics::tightBoundingRect(const QString & text);
  fn _ZNK12QFontMetrics17tightBoundingRectERK7QString(arg0: *const c_void) -> i32;
  // proto: int QFontMetrics::averageCharWidth();
  fn _ZNK12QFontMetrics16averageCharWidthEv() -> i32;
  // proto: int QFontMetrics::underlinePos();
  fn _ZNK12QFontMetrics12underlinePosEv() -> i32;
  // proto: bool QFontMetrics::inFont(QChar );
  fn _ZNK12QFontMetrics6inFontE5QChar(arg0: *mut c_void) -> i32;
  // proto: int QFontMetrics::height();
  fn _ZNK12QFontMetrics6heightEv() -> i32;
  // proto: int QFontMetrics::width(QChar );
  fn _ZNK12QFontMetrics5widthE5QChar(arg0: *mut c_void) -> i32;
  // proto: QRect QFontMetrics::boundingRect(const QString & text);
  fn _ZNK12QFontMetrics12boundingRectERK7QString(arg0: *const c_void) -> i32;
  // proto: int QFontMetrics::xHeight();
  fn _ZNK12QFontMetrics7xHeightEv() -> i32;
  // proto: int QFontMetrics::width(const QString & , int len, int flags);
  fn _ZNK12QFontMetrics5widthERK7QStringii(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: int QFontMetrics::strikeOutPos();
  fn _ZNK12QFontMetrics12strikeOutPosEv() -> i32;
  // proto: int QFontMetrics::lineSpacing();
  fn _ZNK12QFontMetrics11lineSpacingEv() -> i32;
  // proto: void QFontMetrics::NewQFontMetrics(const QFontMetrics & );
  fn _ZN12QFontMetricsC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QFontMetrics::NewQFontMetrics(const QFont & , QPaintDevice * pd);
  fn _ZN12QFontMetricsC1ERK5QFontP12QPaintDevice(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: int QFontMetrics::minRightBearing();
  fn _ZNK12QFontMetrics15minRightBearingEv() -> i32;
  // proto: void QFontMetrics::swap(QFontMetrics & other);
  fn _ZN12QFontMetrics4swapERS_(arg0: *mut c_void) -> i32;
  // proto: QRect QFontMetrics::boundingRect(QChar );
  fn _ZNK12QFontMetrics12boundingRectE5QChar(arg0: *mut c_void) -> i32;
  // proto: void QFontMetrics::NewQFontMetrics(const QFont & );
  fn _ZN12QFontMetricsC1ERK5QFont(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: int QFontMetrics::width(const QString & , int len);
  fn _ZNK12QFontMetrics5widthERK7QStringi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: QRect QFontMetrics::boundingRect(int x, int y, int w, int h, int flags, const QString & text, int tabstops, int * tabarray);
  fn _ZNK12QFontMetrics12boundingRectEiiiiiRK7QStringiPi(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: *const c_void, arg6: c_int, arg7: *mut c_int) -> i32;
  // proto: int QFontMetrics::charWidth(const QString & str, int pos);
  fn _ZNK12QFontMetrics9charWidthERK7QStringi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: int QFontMetrics::leftBearing(QChar );
  fn _ZNK12QFontMetrics11leftBearingE5QChar(arg0: *mut c_void) -> i32;
  // proto: bool QFontMetrics::inFontUcs4(uint ucs4);
  fn _ZNK12QFontMetrics10inFontUcs4Ej(arg0: c_uint) -> i32;
  // proto: int QFontMetrics::descent();
  fn _ZNK12QFontMetrics7descentEv() -> i32;
}

// body block begin
// class sizeof(QFontMetrics)=1
pub struct QFontMetrics {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFontMetrics {
  pub fn maxWidth<T: QFontMetrics_maxWidth>(&mut self, value: T) -> i32 {
    value.maxWidth(self);
    return 1;
  }
}

pub trait QFontMetrics_maxWidth {
  fn maxWidth(self, this: &mut QFontMetrics) -> i32;
}

// proto: int QFontMetrics::maxWidth();
impl<'a> /*trait*/ QFontMetrics_maxWidth for () {
  fn maxWidth(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics8maxWidthEv()};
    unsafe {_ZNK12QFontMetrics8maxWidthEv()};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn FreeQFontMetrics<T: QFontMetrics_FreeQFontMetrics>(&mut self, value: T) -> i32 {
    value.FreeQFontMetrics(self);
    return 1;
  }
}

pub trait QFontMetrics_FreeQFontMetrics {
  fn FreeQFontMetrics(self, this: &mut QFontMetrics) -> i32;
}

// proto: void QFontMetrics::FreeQFontMetrics();
impl<'a> /*trait*/ QFontMetrics_FreeQFontMetrics for () {
  fn FreeQFontMetrics(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QFontMetricsD0Ev()};
    unsafe {_ZN12QFontMetricsD0Ev()};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn lineWidth<T: QFontMetrics_lineWidth>(&mut self, value: T) -> i32 {
    value.lineWidth(self);
    return 1;
  }
}

pub trait QFontMetrics_lineWidth {
  fn lineWidth(self, this: &mut QFontMetrics) -> i32;
}

// proto: int QFontMetrics::lineWidth();
impl<'a> /*trait*/ QFontMetrics_lineWidth for () {
  fn lineWidth(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics9lineWidthEv()};
    unsafe {_ZNK12QFontMetrics9lineWidthEv()};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn boundingRect<T: QFontMetrics_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QFontMetrics_boundingRect {
  fn boundingRect(self, this: &mut QFontMetrics) -> i32;
}

// proto: QRect QFontMetrics::boundingRect(const QRect & r, int flags, const QString & text, int tabstops, int * tabarray);
impl<'a> /*trait*/ QFontMetrics_boundingRect for (&'a  QRect, i32, &'a  QString, i32, &'a mut i32) {
  fn boundingRect(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics12boundingRectERK5QRectiRK7QStringiPi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as *mut c_int;
    unsafe {_ZNK12QFontMetrics12boundingRectERK5QRectiRK7QStringiPi(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn minLeftBearing<T: QFontMetrics_minLeftBearing>(&mut self, value: T) -> i32 {
    value.minLeftBearing(self);
    return 1;
  }
}

pub trait QFontMetrics_minLeftBearing {
  fn minLeftBearing(self, this: &mut QFontMetrics) -> i32;
}

// proto: int QFontMetrics::minLeftBearing();
impl<'a> /*trait*/ QFontMetrics_minLeftBearing for () {
  fn minLeftBearing(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics14minLeftBearingEv()};
    unsafe {_ZNK12QFontMetrics14minLeftBearingEv()};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn rightBearing<T: QFontMetrics_rightBearing>(&mut self, value: T) -> i32 {
    value.rightBearing(self);
    return 1;
  }
}

pub trait QFontMetrics_rightBearing {
  fn rightBearing(self, this: &mut QFontMetrics) -> i32;
}

// proto: int QFontMetrics::rightBearing(QChar );
impl<'a> /*trait*/ QFontMetrics_rightBearing for (QChar) {
  fn rightBearing(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics12rightBearingE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK12QFontMetrics12rightBearingE5QChar(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn ascent<T: QFontMetrics_ascent>(&mut self, value: T) -> i32 {
    value.ascent(self);
    return 1;
  }
}

pub trait QFontMetrics_ascent {
  fn ascent(self, this: &mut QFontMetrics) -> i32;
}

// proto: int QFontMetrics::ascent();
impl<'a> /*trait*/ QFontMetrics_ascent for () {
  fn ascent(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics6ascentEv()};
    unsafe {_ZNK12QFontMetrics6ascentEv()};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn size<T: QFontMetrics_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QFontMetrics_size {
  fn size(self, this: &mut QFontMetrics) -> i32;
}

// proto: QSize QFontMetrics::size(int flags, const QString & str, int tabstops, int * tabarray);
impl<'a> /*trait*/ QFontMetrics_size for (i32, &'a  QString, i32, &'a mut i32) {
  fn size(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics4sizeEiRK7QStringiPi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as *mut c_int;
    unsafe {_ZNK12QFontMetrics4sizeEiRK7QStringiPi(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn overlinePos<T: QFontMetrics_overlinePos>(&mut self, value: T) -> i32 {
    value.overlinePos(self);
    return 1;
  }
}

pub trait QFontMetrics_overlinePos {
  fn overlinePos(self, this: &mut QFontMetrics) -> i32;
}

// proto: int QFontMetrics::overlinePos();
impl<'a> /*trait*/ QFontMetrics_overlinePos for () {
  fn overlinePos(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics11overlinePosEv()};
    unsafe {_ZNK12QFontMetrics11overlinePosEv()};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn leading<T: QFontMetrics_leading>(&mut self, value: T) -> i32 {
    value.leading(self);
    return 1;
  }
}

pub trait QFontMetrics_leading {
  fn leading(self, this: &mut QFontMetrics) -> i32;
}

// proto: int QFontMetrics::leading();
impl<'a> /*trait*/ QFontMetrics_leading for () {
  fn leading(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics7leadingEv()};
    unsafe {_ZNK12QFontMetrics7leadingEv()};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn tightBoundingRect<T: QFontMetrics_tightBoundingRect>(&mut self, value: T) -> i32 {
    value.tightBoundingRect(self);
    return 1;
  }
}

pub trait QFontMetrics_tightBoundingRect {
  fn tightBoundingRect(self, this: &mut QFontMetrics) -> i32;
}

// proto: QRect QFontMetrics::tightBoundingRect(const QString & text);
impl<'a> /*trait*/ QFontMetrics_tightBoundingRect for (&'a  QString) {
  fn tightBoundingRect(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics17tightBoundingRectERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK12QFontMetrics17tightBoundingRectERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn averageCharWidth<T: QFontMetrics_averageCharWidth>(&mut self, value: T) -> i32 {
    value.averageCharWidth(self);
    return 1;
  }
}

pub trait QFontMetrics_averageCharWidth {
  fn averageCharWidth(self, this: &mut QFontMetrics) -> i32;
}

// proto: int QFontMetrics::averageCharWidth();
impl<'a> /*trait*/ QFontMetrics_averageCharWidth for () {
  fn averageCharWidth(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics16averageCharWidthEv()};
    unsafe {_ZNK12QFontMetrics16averageCharWidthEv()};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn underlinePos<T: QFontMetrics_underlinePos>(&mut self, value: T) -> i32 {
    value.underlinePos(self);
    return 1;
  }
}

pub trait QFontMetrics_underlinePos {
  fn underlinePos(self, this: &mut QFontMetrics) -> i32;
}

// proto: int QFontMetrics::underlinePos();
impl<'a> /*trait*/ QFontMetrics_underlinePos for () {
  fn underlinePos(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics12underlinePosEv()};
    unsafe {_ZNK12QFontMetrics12underlinePosEv()};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn inFont<T: QFontMetrics_inFont>(&mut self, value: T) -> i32 {
    value.inFont(self);
    return 1;
  }
}

pub trait QFontMetrics_inFont {
  fn inFont(self, this: &mut QFontMetrics) -> i32;
}

// proto: bool QFontMetrics::inFont(QChar );
impl<'a> /*trait*/ QFontMetrics_inFont for (QChar) {
  fn inFont(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics6inFontE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK12QFontMetrics6inFontE5QChar(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn height<T: QFontMetrics_height>(&mut self, value: T) -> i32 {
    value.height(self);
    return 1;
  }
}

pub trait QFontMetrics_height {
  fn height(self, this: &mut QFontMetrics) -> i32;
}

// proto: int QFontMetrics::height();
impl<'a> /*trait*/ QFontMetrics_height for () {
  fn height(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics6heightEv()};
    unsafe {_ZNK12QFontMetrics6heightEv()};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn width<T: QFontMetrics_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QFontMetrics_width {
  fn width(self, this: &mut QFontMetrics) -> i32;
}

// proto: int QFontMetrics::width(QChar );
impl<'a> /*trait*/ QFontMetrics_width for (QChar) {
  fn width(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics5widthE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK12QFontMetrics5widthE5QChar(arg0)};
    return 1;
  }
}

// proto: QRect QFontMetrics::boundingRect(const QString & text);
impl<'a> /*trait*/ QFontMetrics_boundingRect for (&'a  QString) {
  fn boundingRect(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics12boundingRectERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK12QFontMetrics12boundingRectERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn xHeight<T: QFontMetrics_xHeight>(&mut self, value: T) -> i32 {
    value.xHeight(self);
    return 1;
  }
}

pub trait QFontMetrics_xHeight {
  fn xHeight(self, this: &mut QFontMetrics) -> i32;
}

// proto: int QFontMetrics::xHeight();
impl<'a> /*trait*/ QFontMetrics_xHeight for () {
  fn xHeight(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics7xHeightEv()};
    unsafe {_ZNK12QFontMetrics7xHeightEv()};
    return 1;
  }
}

// proto: int QFontMetrics::width(const QString & , int len, int flags);
impl<'a> /*trait*/ QFontMetrics_width for (&'a  QString, i32, i32) {
  fn width(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics5widthERK7QStringii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZNK12QFontMetrics5widthERK7QStringii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn strikeOutPos<T: QFontMetrics_strikeOutPos>(&mut self, value: T) -> i32 {
    value.strikeOutPos(self);
    return 1;
  }
}

pub trait QFontMetrics_strikeOutPos {
  fn strikeOutPos(self, this: &mut QFontMetrics) -> i32;
}

// proto: int QFontMetrics::strikeOutPos();
impl<'a> /*trait*/ QFontMetrics_strikeOutPos for () {
  fn strikeOutPos(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics12strikeOutPosEv()};
    unsafe {_ZNK12QFontMetrics12strikeOutPosEv()};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn lineSpacing<T: QFontMetrics_lineSpacing>(&mut self, value: T) -> i32 {
    value.lineSpacing(self);
    return 1;
  }
}

pub trait QFontMetrics_lineSpacing {
  fn lineSpacing(self, this: &mut QFontMetrics) -> i32;
}

// proto: int QFontMetrics::lineSpacing();
impl<'a> /*trait*/ QFontMetrics_lineSpacing for () {
  fn lineSpacing(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics11lineSpacingEv()};
    unsafe {_ZNK12QFontMetrics11lineSpacingEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
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
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QFontMetricsC1ERK5QFontP12QPaintDevice(qthis, arg0, arg1)};
    let rsthis = QFontMetrics{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn minRightBearing<T: QFontMetrics_minRightBearing>(&mut self, value: T) -> i32 {
    value.minRightBearing(self);
    return 1;
  }
}

pub trait QFontMetrics_minRightBearing {
  fn minRightBearing(self, this: &mut QFontMetrics) -> i32;
}

// proto: int QFontMetrics::minRightBearing();
impl<'a> /*trait*/ QFontMetrics_minRightBearing for () {
  fn minRightBearing(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics15minRightBearingEv()};
    unsafe {_ZNK12QFontMetrics15minRightBearingEv()};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn swap<T: QFontMetrics_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QFontMetrics_swap {
  fn swap(self, this: &mut QFontMetrics) -> i32;
}

// proto: void QFontMetrics::swap(QFontMetrics & other);
impl<'a> /*trait*/ QFontMetrics_swap for (&'a mut QFontMetrics) {
  fn swap(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QFontMetrics4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QFontMetrics4swapERS_(arg0)};
    return 1;
  }
}

// proto: QRect QFontMetrics::boundingRect(QChar );
impl<'a> /*trait*/ QFontMetrics_boundingRect for (QChar) {
  fn boundingRect(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics12boundingRectE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK12QFontMetrics12boundingRectE5QChar(arg0)};
    return 1;
  }
}

// proto: void QFontMetrics::NewQFontMetrics(const QFont & );
impl<'a> /*trait*/ QFontMetrics_NewQFontMetrics for (&'a  QFont) {
  fn NewQFontMetrics(self) -> QFontMetrics {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QFontMetricsC1ERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QFontMetricsC1ERK5QFont(qthis, arg0)};
    let rsthis = QFontMetrics{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: int QFontMetrics::width(const QString & , int len);
impl<'a> /*trait*/ QFontMetrics_width for (&'a  QString, i32) {
  fn width(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics5widthERK7QStringi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK12QFontMetrics5widthERK7QStringi(arg0, arg1)};
    return 1;
  }
}

// proto: QRect QFontMetrics::boundingRect(int x, int y, int w, int h, int flags, const QString & text, int tabstops, int * tabarray);
impl<'a> /*trait*/ QFontMetrics_boundingRect for (i32, i32, i32, i32, i32, &'a  QString, i32, &'a mut i32) {
  fn boundingRect(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics12boundingRectEiiiiiRK7QStringiPi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5.qclsinst  as *const c_void;
    let arg6 = self.6  as c_int;
    let arg7 = self.7  as *mut c_int;
    unsafe {_ZNK12QFontMetrics12boundingRectEiiiiiRK7QStringiPi(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn charWidth<T: QFontMetrics_charWidth>(&mut self, value: T) -> i32 {
    value.charWidth(self);
    return 1;
  }
}

pub trait QFontMetrics_charWidth {
  fn charWidth(self, this: &mut QFontMetrics) -> i32;
}

// proto: int QFontMetrics::charWidth(const QString & str, int pos);
impl<'a> /*trait*/ QFontMetrics_charWidth for (&'a  QString, i32) {
  fn charWidth(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics9charWidthERK7QStringi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK12QFontMetrics9charWidthERK7QStringi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn leftBearing<T: QFontMetrics_leftBearing>(&mut self, value: T) -> i32 {
    value.leftBearing(self);
    return 1;
  }
}

pub trait QFontMetrics_leftBearing {
  fn leftBearing(self, this: &mut QFontMetrics) -> i32;
}

// proto: int QFontMetrics::leftBearing(QChar );
impl<'a> /*trait*/ QFontMetrics_leftBearing for (QChar) {
  fn leftBearing(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics11leftBearingE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK12QFontMetrics11leftBearingE5QChar(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn inFontUcs4<T: QFontMetrics_inFontUcs4>(&mut self, value: T) -> i32 {
    value.inFontUcs4(self);
    return 1;
  }
}

pub trait QFontMetrics_inFontUcs4 {
  fn inFontUcs4(self, this: &mut QFontMetrics) -> i32;
}

// proto: bool QFontMetrics::inFontUcs4(uint ucs4);
impl<'a> /*trait*/ QFontMetrics_inFontUcs4 for (u32) {
  fn inFontUcs4(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics10inFontUcs4Ej()};
    let arg0 = self  as c_uint;
    unsafe {_ZNK12QFontMetrics10inFontUcs4Ej(arg0)};
    return 1;
  }
}

impl /*struct*/ QFontMetrics {
  pub fn descent<T: QFontMetrics_descent>(&mut self, value: T) -> i32 {
    value.descent(self);
    return 1;
  }
}

pub trait QFontMetrics_descent {
  fn descent(self, this: &mut QFontMetrics) -> i32;
}

// proto: int QFontMetrics::descent();
impl<'a> /*trait*/ QFontMetrics_descent for () {
  fn descent(self, this: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics7descentEv()};
    unsafe {_ZNK12QFontMetrics7descentEv()};
    return 1;
  }
}

