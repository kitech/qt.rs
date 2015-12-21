// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtGui/qfontmetrics.h
// dst-file: /src/gui/qfontmetrics.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::super::core::qrect::QRect; // 771
use super::super::core::qstring::QString; // 771
use super::super::core::qchar::QChar; // 771
use super::super::core::qsize::QSize; // 771
use super::qfont::QFont; // 773
use super::qpaintdevice::QPaintDevice; // 773
use super::super::core::qsize::QSizeF; // 771
use super::super::core::qrect::QRectF; // 771
// use super::qfontmetrics::QFontMetrics; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  int QFontMetrics::maxWidth();
  fn _ZNK12QFontMetrics8maxWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QFontMetrics::~QFontMetrics();
  fn _ZN12QFontMetricsD0Ev(qthis: *mut c_void);
  // proto:  int QFontMetrics::lineWidth();
  fn _ZNK12QFontMetrics9lineWidthEv(qthis: *mut c_void) -> c_int;
  // proto:  QRect QFontMetrics::boundingRect(const QRect & r, int flags, const QString & text, int tabstops, int * tabarray);
  fn _ZNK12QFontMetrics12boundingRectERK5QRectiRK7QStringiPi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void, arg3: c_int, arg4: *mut c_int) -> *mut c_void;
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
  fn _ZNK12QFontMetrics6inFontE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  int QFontMetrics::height();
  fn _ZNK12QFontMetrics6heightEv(qthis: *mut c_void) -> c_int;
  // proto:  int QFontMetrics::width(QChar );
  fn _ZNK12QFontMetrics5widthE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QRect QFontMetrics::boundingRect(const QString & text);
  fn _ZNK12QFontMetrics12boundingRectERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QFontMetrics::xHeight();
  fn _ZNK12QFontMetrics7xHeightEv(qthis: *mut c_void) -> c_int;
  // proto:  int QFontMetrics::width(const QString & , int len, int flags);
  fn _ZNK12QFontMetrics5widthERK7QStringii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) -> c_int;
  // proto:  int QFontMetrics::strikeOutPos();
  fn _ZNK12QFontMetrics12strikeOutPosEv(qthis: *mut c_void) -> c_int;
  // proto:  int QFontMetrics::lineSpacing();
  fn _ZNK12QFontMetrics11lineSpacingEv(qthis: *mut c_void) -> c_int;
  // proto:  void QFontMetrics::QFontMetrics(const QFontMetrics & );
  fn _ZN12QFontMetricsC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFontMetrics::QFontMetrics(const QFont & , QPaintDevice * pd);
  fn _ZN12QFontMetricsC1ERK5QFontP12QPaintDevice(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  int QFontMetrics::minRightBearing();
  fn _ZNK12QFontMetrics15minRightBearingEv(qthis: *mut c_void) -> c_int;
  // proto:  void QFontMetrics::swap(QFontMetrics & other);
  fn _ZN12QFontMetrics4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRect QFontMetrics::boundingRect(QChar );
  fn _ZNK12QFontMetrics12boundingRectE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QFontMetrics::QFontMetrics(const QFont & );
  fn _ZN12QFontMetricsC1ERK5QFont(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QFontMetrics::width(const QString & , int len);
  fn _ZNK12QFontMetrics5widthERK7QStringi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  QRect QFontMetrics::boundingRect(int x, int y, int w, int h, int flags, const QString & text, int tabstops, int * tabarray);
  fn _ZNK12QFontMetrics12boundingRectEiiiiiRK7QStringiPi(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: *mut c_void, arg6: c_int, arg7: *mut c_int) -> *mut c_void;
  // proto:  int QFontMetrics::charWidth(const QString & str, int pos);
  fn _ZNK12QFontMetrics9charWidthERK7QStringi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QFontMetrics::leftBearing(QChar );
  fn _ZNK12QFontMetrics11leftBearingE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  bool QFontMetrics::inFontUcs4(uint ucs4);
  fn _ZNK12QFontMetrics10inFontUcs4Ej(qthis: *mut c_void, arg0: c_uint) -> c_char;
  // proto:  int QFontMetrics::descent();
  fn _ZNK12QFontMetrics7descentEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QFontMetricsF::inFont(QChar );
  fn _ZNK13QFontMetricsF6inFontE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QSizeF QFontMetricsF::size(int flags, const QString & str, int tabstops, int * tabarray);
  fn _ZNK13QFontMetricsF4sizeEiRK7QStringiPi(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_int, arg3: *mut c_int) -> *mut c_void;
  // proto:  qreal QFontMetricsF::minRightBearing();
  fn _ZNK13QFontMetricsF15minRightBearingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QFontMetricsF::QFontMetricsF(const QFontMetricsF & );
  fn _ZN13QFontMetricsFC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QFontMetricsF::xHeight();
  fn _ZNK13QFontMetricsF7xHeightEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QFontMetricsF::width(QChar );
  fn _ZNK13QFontMetricsF5widthE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> c_double;
  // proto:  void QFontMetricsF::~QFontMetricsF();
  fn _ZN13QFontMetricsFD0Ev(qthis: *mut c_void);
  // proto:  QRectF QFontMetricsF::boundingRect(const QRectF & r, int flags, const QString & string, int tabstops, int * tabarray);
  fn _ZNK13QFontMetricsF12boundingRectERK6QRectFiRK7QStringiPi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void, arg3: c_int, arg4: *mut c_int) -> *mut c_void;
  // proto:  void QFontMetricsF::swap(QFontMetricsF & other);
  fn _ZN13QFontMetricsF4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRectF QFontMetricsF::tightBoundingRect(const QString & text);
  fn _ZNK13QFontMetricsF17tightBoundingRectERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  qreal QFontMetricsF::leftBearing(QChar );
  fn _ZNK13QFontMetricsF11leftBearingE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> c_double;
  // proto:  qreal QFontMetricsF::rightBearing(QChar );
  fn _ZNK13QFontMetricsF12rightBearingE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> c_double;
  // proto:  qreal QFontMetricsF::overlinePos();
  fn _ZNK13QFontMetricsF11overlinePosEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QFontMetricsF::height();
  fn _ZNK13QFontMetricsF6heightEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QFontMetricsF::descent();
  fn _ZNK13QFontMetricsF7descentEv(qthis: *mut c_void) -> c_double;
  // proto:  QRectF QFontMetricsF::boundingRect(const QString & string);
  fn _ZNK13QFontMetricsF12boundingRectERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  qreal QFontMetricsF::lineWidth();
  fn _ZNK13QFontMetricsF9lineWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QFontMetricsF::QFontMetricsF(const QFontMetrics & );
  fn _ZN13QFontMetricsFC1ERK12QFontMetrics(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QFontMetricsF::width(const QString & string);
  fn _ZNK13QFontMetricsF5widthERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_double;
  // proto:  qreal QFontMetricsF::strikeOutPos();
  fn _ZNK13QFontMetricsF12strikeOutPosEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QFontMetricsF::lineSpacing();
  fn _ZNK13QFontMetricsF11lineSpacingEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QFontMetricsF::averageCharWidth();
  fn _ZNK13QFontMetricsF16averageCharWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QFontMetricsF::QFontMetricsF(const QFont & , QPaintDevice * pd);
  fn _ZN13QFontMetricsFC1ERK5QFontP12QPaintDevice(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  qreal QFontMetricsF::leading();
  fn _ZNK13QFontMetricsF7leadingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QFontMetricsF::QFontMetricsF(const QFont & );
  fn _ZN13QFontMetricsFC1ERK5QFont(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRectF QFontMetricsF::boundingRect(QChar );
  fn _ZNK13QFontMetricsF12boundingRectE5QChar(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QFontMetricsF::inFontUcs4(uint ucs4);
  fn _ZNK13QFontMetricsF10inFontUcs4Ej(qthis: *mut c_void, arg0: c_uint) -> c_char;
  // proto:  qreal QFontMetricsF::minLeftBearing();
  fn _ZNK13QFontMetricsF14minLeftBearingEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QFontMetricsF::ascent();
  fn _ZNK13QFontMetricsF6ascentEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QFontMetricsF::maxWidth();
  fn _ZNK13QFontMetricsF8maxWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QFontMetricsF::underlinePos();
  fn _ZNK13QFontMetricsF12underlinePosEv(qthis: *mut c_void) -> c_double;
} // <= ext block end

// body block begin =>
// class sizeof(QFontMetrics)=1
pub struct QFontMetrics {
  pub qclsinst: *mut c_void,
}

// class sizeof(QFontMetricsF)=1
pub struct QFontMetricsF {
  pub qclsinst: *mut c_void,
}

  // proto:  int QFontMetrics::maxWidth();
impl /*struct*/ QFontMetrics {
  pub fn maxWidth<RetType, T: QFontMetrics_maxWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maxWidth(self);
    // return 1;
  }
}

pub trait QFontMetrics_maxWidth<RetType> {
  fn maxWidth(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  int QFontMetrics::maxWidth();
impl<'a> /*trait*/ QFontMetrics_maxWidth<i32> for () {
  fn maxWidth(self , rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics8maxWidthEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics8maxWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QFontMetrics::~QFontMetrics();
impl /*struct*/ QFontMetrics {
  pub fn FreeQFontMetrics<RetType, T: QFontMetrics_FreeQFontMetrics<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQFontMetrics(self);
    // return 1;
  }
}

pub trait QFontMetrics_FreeQFontMetrics<RetType> {
  fn FreeQFontMetrics(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  void QFontMetrics::~QFontMetrics();
impl<'a> /*trait*/ QFontMetrics_FreeQFontMetrics<()> for () {
  fn FreeQFontMetrics(self , rsthis: &mut QFontMetrics) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QFontMetricsD0Ev()};
     unsafe {_ZN12QFontMetricsD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QFontMetrics::lineWidth();
impl /*struct*/ QFontMetrics {
  pub fn lineWidth<RetType, T: QFontMetrics_lineWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lineWidth(self);
    // return 1;
  }
}

pub trait QFontMetrics_lineWidth<RetType> {
  fn lineWidth(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  int QFontMetrics::lineWidth();
impl<'a> /*trait*/ QFontMetrics_lineWidth<i32> for () {
  fn lineWidth(self , rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics9lineWidthEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics9lineWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QRect QFontMetrics::boundingRect(const QRect & r, int flags, const QString & text, int tabstops, int * tabarray);
impl /*struct*/ QFontMetrics {
  pub fn boundingRect<RetType, T: QFontMetrics_boundingRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QFontMetrics_boundingRect<RetType> {
  fn boundingRect(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  QRect QFontMetrics::boundingRect(const QRect & r, int flags, const QString & text, int tabstops, int * tabarray);
impl<'a> /*trait*/ QFontMetrics_boundingRect<QRect> for (QRect, i32, QString, i32, &'a mut Vec<i32>) {
  fn boundingRect(self , rsthis: &mut QFontMetrics) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics12boundingRectERK5QRectiRK7QStringiPi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let arg4 = self.4.as_ptr()  as *mut c_int;
    let mut ret = unsafe {_ZNK12QFontMetrics12boundingRectERK5QRectiRK7QStringiPi(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QFontMetrics::minLeftBearing();
impl /*struct*/ QFontMetrics {
  pub fn minLeftBearing<RetType, T: QFontMetrics_minLeftBearing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minLeftBearing(self);
    // return 1;
  }
}

pub trait QFontMetrics_minLeftBearing<RetType> {
  fn minLeftBearing(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  int QFontMetrics::minLeftBearing();
impl<'a> /*trait*/ QFontMetrics_minLeftBearing<i32> for () {
  fn minLeftBearing(self , rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics14minLeftBearingEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics14minLeftBearingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QFontMetrics::rightBearing(QChar );
impl /*struct*/ QFontMetrics {
  pub fn rightBearing<RetType, T: QFontMetrics_rightBearing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rightBearing(self);
    // return 1;
  }
}

pub trait QFontMetrics_rightBearing<RetType> {
  fn rightBearing(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  int QFontMetrics::rightBearing(QChar );
impl<'a> /*trait*/ QFontMetrics_rightBearing<i32> for (QChar) {
  fn rightBearing(self , rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics12rightBearingE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QFontMetrics12rightBearingE5QChar(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QFontMetrics::ascent();
impl /*struct*/ QFontMetrics {
  pub fn ascent<RetType, T: QFontMetrics_ascent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.ascent(self);
    // return 1;
  }
}

pub trait QFontMetrics_ascent<RetType> {
  fn ascent(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  int QFontMetrics::ascent();
impl<'a> /*trait*/ QFontMetrics_ascent<i32> for () {
  fn ascent(self , rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics6ascentEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics6ascentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSize QFontMetrics::size(int flags, const QString & str, int tabstops, int * tabarray);
impl /*struct*/ QFontMetrics {
  pub fn size<RetType, T: QFontMetrics_size<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QFontMetrics_size<RetType> {
  fn size(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  QSize QFontMetrics::size(int flags, const QString & str, int tabstops, int * tabarray);
impl<'a> /*trait*/ QFontMetrics_size<QSize> for (i32, QString, i32, &'a mut Vec<i32>) {
  fn size(self , rsthis: &mut QFontMetrics) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics4sizeEiRK7QStringiPi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.as_ptr()  as *mut c_int;
    let mut ret = unsafe {_ZNK12QFontMetrics4sizeEiRK7QStringiPi(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QFontMetrics::overlinePos();
impl /*struct*/ QFontMetrics {
  pub fn overlinePos<RetType, T: QFontMetrics_overlinePos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.overlinePos(self);
    // return 1;
  }
}

pub trait QFontMetrics_overlinePos<RetType> {
  fn overlinePos(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  int QFontMetrics::overlinePos();
impl<'a> /*trait*/ QFontMetrics_overlinePos<i32> for () {
  fn overlinePos(self , rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics11overlinePosEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics11overlinePosEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QFontMetrics::leading();
impl /*struct*/ QFontMetrics {
  pub fn leading<RetType, T: QFontMetrics_leading<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.leading(self);
    // return 1;
  }
}

pub trait QFontMetrics_leading<RetType> {
  fn leading(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  int QFontMetrics::leading();
impl<'a> /*trait*/ QFontMetrics_leading<i32> for () {
  fn leading(self , rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics7leadingEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics7leadingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QRect QFontMetrics::tightBoundingRect(const QString & text);
impl /*struct*/ QFontMetrics {
  pub fn tightBoundingRect<RetType, T: QFontMetrics_tightBoundingRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tightBoundingRect(self);
    // return 1;
  }
}

pub trait QFontMetrics_tightBoundingRect<RetType> {
  fn tightBoundingRect(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  QRect QFontMetrics::tightBoundingRect(const QString & text);
impl<'a> /*trait*/ QFontMetrics_tightBoundingRect<QRect> for (QString) {
  fn tightBoundingRect(self , rsthis: &mut QFontMetrics) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics17tightBoundingRectERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QFontMetrics17tightBoundingRectERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QFontMetrics::averageCharWidth();
impl /*struct*/ QFontMetrics {
  pub fn averageCharWidth<RetType, T: QFontMetrics_averageCharWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.averageCharWidth(self);
    // return 1;
  }
}

pub trait QFontMetrics_averageCharWidth<RetType> {
  fn averageCharWidth(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  int QFontMetrics::averageCharWidth();
impl<'a> /*trait*/ QFontMetrics_averageCharWidth<i32> for () {
  fn averageCharWidth(self , rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics16averageCharWidthEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics16averageCharWidthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QFontMetrics::underlinePos();
impl /*struct*/ QFontMetrics {
  pub fn underlinePos<RetType, T: QFontMetrics_underlinePos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.underlinePos(self);
    // return 1;
  }
}

pub trait QFontMetrics_underlinePos<RetType> {
  fn underlinePos(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  int QFontMetrics::underlinePos();
impl<'a> /*trait*/ QFontMetrics_underlinePos<i32> for () {
  fn underlinePos(self , rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics12underlinePosEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics12underlinePosEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QFontMetrics::inFont(QChar );
impl /*struct*/ QFontMetrics {
  pub fn inFont<RetType, T: QFontMetrics_inFont<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.inFont(self);
    // return 1;
  }
}

pub trait QFontMetrics_inFont<RetType> {
  fn inFont(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  bool QFontMetrics::inFont(QChar );
impl<'a> /*trait*/ QFontMetrics_inFont<i8> for (QChar) {
  fn inFont(self , rsthis: &mut QFontMetrics) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics6inFontE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QFontMetrics6inFontE5QChar(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QFontMetrics::height();
impl /*struct*/ QFontMetrics {
  pub fn height<RetType, T: QFontMetrics_height<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QFontMetrics_height<RetType> {
  fn height(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  int QFontMetrics::height();
impl<'a> /*trait*/ QFontMetrics_height<i32> for () {
  fn height(self , rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics6heightEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QFontMetrics::width(QChar );
impl /*struct*/ QFontMetrics {
  pub fn width<RetType, T: QFontMetrics_width<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QFontMetrics_width<RetType> {
  fn width(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  int QFontMetrics::width(QChar );
impl<'a> /*trait*/ QFontMetrics_width<i32> for (QChar) {
  fn width(self , rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics5widthE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QFontMetrics5widthE5QChar(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QRect QFontMetrics::boundingRect(const QString & text);
impl<'a> /*trait*/ QFontMetrics_boundingRect<QRect> for (QString) {
  fn boundingRect(self , rsthis: &mut QFontMetrics) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics12boundingRectERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QFontMetrics12boundingRectERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QFontMetrics::xHeight();
impl /*struct*/ QFontMetrics {
  pub fn xHeight<RetType, T: QFontMetrics_xHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.xHeight(self);
    // return 1;
  }
}

pub trait QFontMetrics_xHeight<RetType> {
  fn xHeight(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  int QFontMetrics::xHeight();
impl<'a> /*trait*/ QFontMetrics_xHeight<i32> for () {
  fn xHeight(self , rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics7xHeightEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics7xHeightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QFontMetrics::width(const QString & , int len, int flags);
impl<'a> /*trait*/ QFontMetrics_width<i32> for (QString, i32, i32) {
  fn width(self , rsthis: &mut QFontMetrics) -> i32 {
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

  // proto:  int QFontMetrics::strikeOutPos();
impl /*struct*/ QFontMetrics {
  pub fn strikeOutPos<RetType, T: QFontMetrics_strikeOutPos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.strikeOutPos(self);
    // return 1;
  }
}

pub trait QFontMetrics_strikeOutPos<RetType> {
  fn strikeOutPos(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  int QFontMetrics::strikeOutPos();
impl<'a> /*trait*/ QFontMetrics_strikeOutPos<i32> for () {
  fn strikeOutPos(self , rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics12strikeOutPosEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics12strikeOutPosEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QFontMetrics::lineSpacing();
impl /*struct*/ QFontMetrics {
  pub fn lineSpacing<RetType, T: QFontMetrics_lineSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lineSpacing(self);
    // return 1;
  }
}

pub trait QFontMetrics_lineSpacing<RetType> {
  fn lineSpacing(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  int QFontMetrics::lineSpacing();
impl<'a> /*trait*/ QFontMetrics_lineSpacing<i32> for () {
  fn lineSpacing(self , rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics11lineSpacingEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics11lineSpacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QFontMetrics::QFontMetrics(const QFontMetrics & );
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

  // proto:  void QFontMetrics::QFontMetrics(const QFontMetrics & );
impl<'a> /*trait*/ QFontMetrics_NewQFontMetrics for (QFontMetrics) {
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

  // proto:  void QFontMetrics::QFontMetrics(const QFont & , QPaintDevice * pd);
impl<'a> /*trait*/ QFontMetrics_NewQFontMetrics for (QFont, QPaintDevice) {
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

  // proto:  int QFontMetrics::minRightBearing();
impl /*struct*/ QFontMetrics {
  pub fn minRightBearing<RetType, T: QFontMetrics_minRightBearing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minRightBearing(self);
    // return 1;
  }
}

pub trait QFontMetrics_minRightBearing<RetType> {
  fn minRightBearing(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  int QFontMetrics::minRightBearing();
impl<'a> /*trait*/ QFontMetrics_minRightBearing<i32> for () {
  fn minRightBearing(self , rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics15minRightBearingEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics15minRightBearingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QFontMetrics::swap(QFontMetrics & other);
impl /*struct*/ QFontMetrics {
  pub fn swap<RetType, T: QFontMetrics_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QFontMetrics_swap<RetType> {
  fn swap(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  void QFontMetrics::swap(QFontMetrics & other);
impl<'a> /*trait*/ QFontMetrics_swap<()> for (QFontMetrics) {
  fn swap(self , rsthis: &mut QFontMetrics) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QFontMetrics4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QFontMetrics4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QFontMetrics::boundingRect(QChar );
impl<'a> /*trait*/ QFontMetrics_boundingRect<QRect> for (QChar) {
  fn boundingRect(self , rsthis: &mut QFontMetrics) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics12boundingRectE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QFontMetrics12boundingRectE5QChar(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QFontMetrics::QFontMetrics(const QFont & );
impl<'a> /*trait*/ QFontMetrics_NewQFontMetrics for (QFont) {
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
impl<'a> /*trait*/ QFontMetrics_width<i32> for (QString, i32) {
  fn width(self , rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics5widthERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK12QFontMetrics5widthERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QRect QFontMetrics::boundingRect(int x, int y, int w, int h, int flags, const QString & text, int tabstops, int * tabarray);
impl<'a> /*trait*/ QFontMetrics_boundingRect<QRect> for (i32, i32, i32, i32, i32, QString, i32, &'a mut Vec<i32>) {
  fn boundingRect(self , rsthis: &mut QFontMetrics) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics12boundingRectEiiiiiRK7QStringiPi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5.qclsinst  as *mut c_void;
    let arg6 = self.6  as c_int;
    let arg7 = self.7.as_ptr()  as *mut c_int;
    let mut ret = unsafe {_ZNK12QFontMetrics12boundingRectEiiiiiRK7QStringiPi(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QFontMetrics::charWidth(const QString & str, int pos);
impl /*struct*/ QFontMetrics {
  pub fn charWidth<RetType, T: QFontMetrics_charWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.charWidth(self);
    // return 1;
  }
}

pub trait QFontMetrics_charWidth<RetType> {
  fn charWidth(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  int QFontMetrics::charWidth(const QString & str, int pos);
impl<'a> /*trait*/ QFontMetrics_charWidth<i32> for (QString, i32) {
  fn charWidth(self , rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics9charWidthERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK12QFontMetrics9charWidthERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QFontMetrics::leftBearing(QChar );
impl /*struct*/ QFontMetrics {
  pub fn leftBearing<RetType, T: QFontMetrics_leftBearing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.leftBearing(self);
    // return 1;
  }
}

pub trait QFontMetrics_leftBearing<RetType> {
  fn leftBearing(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  int QFontMetrics::leftBearing(QChar );
impl<'a> /*trait*/ QFontMetrics_leftBearing<i32> for (QChar) {
  fn leftBearing(self , rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics11leftBearingE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QFontMetrics11leftBearingE5QChar(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QFontMetrics::inFontUcs4(uint ucs4);
impl /*struct*/ QFontMetrics {
  pub fn inFontUcs4<RetType, T: QFontMetrics_inFontUcs4<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.inFontUcs4(self);
    // return 1;
  }
}

pub trait QFontMetrics_inFontUcs4<RetType> {
  fn inFontUcs4(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  bool QFontMetrics::inFontUcs4(uint ucs4);
impl<'a> /*trait*/ QFontMetrics_inFontUcs4<i8> for (u32) {
  fn inFontUcs4(self , rsthis: &mut QFontMetrics) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics10inFontUcs4Ej()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZNK12QFontMetrics10inFontUcs4Ej(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QFontMetrics::descent();
impl /*struct*/ QFontMetrics {
  pub fn descent<RetType, T: QFontMetrics_descent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.descent(self);
    // return 1;
  }
}

pub trait QFontMetrics_descent<RetType> {
  fn descent(self , rsthis: &mut QFontMetrics) -> RetType;
}

  // proto:  int QFontMetrics::descent();
impl<'a> /*trait*/ QFontMetrics_descent<i32> for () {
  fn descent(self , rsthis: &mut QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QFontMetrics7descentEv()};
    let mut ret = unsafe {_ZNK12QFontMetrics7descentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QFontMetricsF::inFont(QChar );
impl /*struct*/ QFontMetricsF {
  pub fn inFont<RetType, T: QFontMetricsF_inFont<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.inFont(self);
    // return 1;
  }
}

pub trait QFontMetricsF_inFont<RetType> {
  fn inFont(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  bool QFontMetricsF::inFont(QChar );
impl<'a> /*trait*/ QFontMetricsF_inFont<i8> for (QChar) {
  fn inFont(self , rsthis: &mut QFontMetricsF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF6inFontE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontMetricsF6inFontE5QChar(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSizeF QFontMetricsF::size(int flags, const QString & str, int tabstops, int * tabarray);
impl /*struct*/ QFontMetricsF {
  pub fn size<RetType, T: QFontMetricsF_size<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QFontMetricsF_size<RetType> {
  fn size(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  QSizeF QFontMetricsF::size(int flags, const QString & str, int tabstops, int * tabarray);
impl<'a> /*trait*/ QFontMetricsF_size<QSizeF> for (i32, QString, i32, &'a mut Vec<i32>) {
  fn size(self , rsthis: &mut QFontMetricsF) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF4sizeEiRK7QStringiPi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.as_ptr()  as *mut c_int;
    let mut ret = unsafe {_ZNK13QFontMetricsF4sizeEiRK7QStringiPi(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QFontMetricsF::minRightBearing();
impl /*struct*/ QFontMetricsF {
  pub fn minRightBearing<RetType, T: QFontMetricsF_minRightBearing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minRightBearing(self);
    // return 1;
  }
}

pub trait QFontMetricsF_minRightBearing<RetType> {
  fn minRightBearing(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  qreal QFontMetricsF::minRightBearing();
impl<'a> /*trait*/ QFontMetricsF_minRightBearing<f64> for () {
  fn minRightBearing(self , rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF15minRightBearingEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF15minRightBearingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QFontMetricsF::QFontMetricsF(const QFontMetricsF & );
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

  // proto:  void QFontMetricsF::QFontMetricsF(const QFontMetricsF & );
impl<'a> /*trait*/ QFontMetricsF_NewQFontMetricsF for (QFontMetricsF) {
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

  // proto:  qreal QFontMetricsF::xHeight();
impl /*struct*/ QFontMetricsF {
  pub fn xHeight<RetType, T: QFontMetricsF_xHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.xHeight(self);
    // return 1;
  }
}

pub trait QFontMetricsF_xHeight<RetType> {
  fn xHeight(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  qreal QFontMetricsF::xHeight();
impl<'a> /*trait*/ QFontMetricsF_xHeight<f64> for () {
  fn xHeight(self , rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF7xHeightEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF7xHeightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QFontMetricsF::width(QChar );
impl /*struct*/ QFontMetricsF {
  pub fn width<RetType, T: QFontMetricsF_width<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QFontMetricsF_width<RetType> {
  fn width(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  qreal QFontMetricsF::width(QChar );
impl<'a> /*trait*/ QFontMetricsF_width<f64> for (QChar) {
  fn width(self , rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF5widthE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontMetricsF5widthE5QChar(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QFontMetricsF::~QFontMetricsF();
impl /*struct*/ QFontMetricsF {
  pub fn FreeQFontMetricsF<RetType, T: QFontMetricsF_FreeQFontMetricsF<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQFontMetricsF(self);
    // return 1;
  }
}

pub trait QFontMetricsF_FreeQFontMetricsF<RetType> {
  fn FreeQFontMetricsF(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  void QFontMetricsF::~QFontMetricsF();
impl<'a> /*trait*/ QFontMetricsF_FreeQFontMetricsF<()> for () {
  fn FreeQFontMetricsF(self , rsthis: &mut QFontMetricsF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontMetricsFD0Ev()};
     unsafe {_ZN13QFontMetricsFD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRectF QFontMetricsF::boundingRect(const QRectF & r, int flags, const QString & string, int tabstops, int * tabarray);
impl /*struct*/ QFontMetricsF {
  pub fn boundingRect<RetType, T: QFontMetricsF_boundingRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QFontMetricsF_boundingRect<RetType> {
  fn boundingRect(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  QRectF QFontMetricsF::boundingRect(const QRectF & r, int flags, const QString & string, int tabstops, int * tabarray);
impl<'a> /*trait*/ QFontMetricsF_boundingRect<QRectF> for (QRectF, i32, QString, i32, &'a mut Vec<i32>) {
  fn boundingRect(self , rsthis: &mut QFontMetricsF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF12boundingRectERK6QRectFiRK7QStringiPi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let arg4 = self.4.as_ptr()  as *mut c_int;
    let mut ret = unsafe {_ZNK13QFontMetricsF12boundingRectERK6QRectFiRK7QStringiPi(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QFontMetricsF::swap(QFontMetricsF & other);
impl /*struct*/ QFontMetricsF {
  pub fn swap<RetType, T: QFontMetricsF_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QFontMetricsF_swap<RetType> {
  fn swap(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  void QFontMetricsF::swap(QFontMetricsF & other);
impl<'a> /*trait*/ QFontMetricsF_swap<()> for (QFontMetricsF) {
  fn swap(self , rsthis: &mut QFontMetricsF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFontMetricsF4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QFontMetricsF4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QFontMetricsF::tightBoundingRect(const QString & text);
impl /*struct*/ QFontMetricsF {
  pub fn tightBoundingRect<RetType, T: QFontMetricsF_tightBoundingRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tightBoundingRect(self);
    // return 1;
  }
}

pub trait QFontMetricsF_tightBoundingRect<RetType> {
  fn tightBoundingRect(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  QRectF QFontMetricsF::tightBoundingRect(const QString & text);
impl<'a> /*trait*/ QFontMetricsF_tightBoundingRect<QRectF> for (QString) {
  fn tightBoundingRect(self , rsthis: &mut QFontMetricsF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF17tightBoundingRectERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontMetricsF17tightBoundingRectERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QFontMetricsF::leftBearing(QChar );
impl /*struct*/ QFontMetricsF {
  pub fn leftBearing<RetType, T: QFontMetricsF_leftBearing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.leftBearing(self);
    // return 1;
  }
}

pub trait QFontMetricsF_leftBearing<RetType> {
  fn leftBearing(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  qreal QFontMetricsF::leftBearing(QChar );
impl<'a> /*trait*/ QFontMetricsF_leftBearing<f64> for (QChar) {
  fn leftBearing(self , rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF11leftBearingE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontMetricsF11leftBearingE5QChar(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QFontMetricsF::rightBearing(QChar );
impl /*struct*/ QFontMetricsF {
  pub fn rightBearing<RetType, T: QFontMetricsF_rightBearing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rightBearing(self);
    // return 1;
  }
}

pub trait QFontMetricsF_rightBearing<RetType> {
  fn rightBearing(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  qreal QFontMetricsF::rightBearing(QChar );
impl<'a> /*trait*/ QFontMetricsF_rightBearing<f64> for (QChar) {
  fn rightBearing(self , rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF12rightBearingE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontMetricsF12rightBearingE5QChar(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QFontMetricsF::overlinePos();
impl /*struct*/ QFontMetricsF {
  pub fn overlinePos<RetType, T: QFontMetricsF_overlinePos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.overlinePos(self);
    // return 1;
  }
}

pub trait QFontMetricsF_overlinePos<RetType> {
  fn overlinePos(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  qreal QFontMetricsF::overlinePos();
impl<'a> /*trait*/ QFontMetricsF_overlinePos<f64> for () {
  fn overlinePos(self , rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF11overlinePosEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF11overlinePosEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QFontMetricsF::height();
impl /*struct*/ QFontMetricsF {
  pub fn height<RetType, T: QFontMetricsF_height<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QFontMetricsF_height<RetType> {
  fn height(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  qreal QFontMetricsF::height();
impl<'a> /*trait*/ QFontMetricsF_height<f64> for () {
  fn height(self , rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF6heightEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF6heightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QFontMetricsF::descent();
impl /*struct*/ QFontMetricsF {
  pub fn descent<RetType, T: QFontMetricsF_descent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.descent(self);
    // return 1;
  }
}

pub trait QFontMetricsF_descent<RetType> {
  fn descent(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  qreal QFontMetricsF::descent();
impl<'a> /*trait*/ QFontMetricsF_descent<f64> for () {
  fn descent(self , rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF7descentEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF7descentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QRectF QFontMetricsF::boundingRect(const QString & string);
impl<'a> /*trait*/ QFontMetricsF_boundingRect<QRectF> for (QString) {
  fn boundingRect(self , rsthis: &mut QFontMetricsF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF12boundingRectERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontMetricsF12boundingRectERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QFontMetricsF::lineWidth();
impl /*struct*/ QFontMetricsF {
  pub fn lineWidth<RetType, T: QFontMetricsF_lineWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lineWidth(self);
    // return 1;
  }
}

pub trait QFontMetricsF_lineWidth<RetType> {
  fn lineWidth(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  qreal QFontMetricsF::lineWidth();
impl<'a> /*trait*/ QFontMetricsF_lineWidth<f64> for () {
  fn lineWidth(self , rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF9lineWidthEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF9lineWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QFontMetricsF::QFontMetricsF(const QFontMetrics & );
impl<'a> /*trait*/ QFontMetricsF_NewQFontMetricsF for (QFontMetrics) {
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

  // proto:  qreal QFontMetricsF::width(const QString & string);
impl<'a> /*trait*/ QFontMetricsF_width<f64> for (QString) {
  fn width(self , rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF5widthERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontMetricsF5widthERK7QString(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QFontMetricsF::strikeOutPos();
impl /*struct*/ QFontMetricsF {
  pub fn strikeOutPos<RetType, T: QFontMetricsF_strikeOutPos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.strikeOutPos(self);
    // return 1;
  }
}

pub trait QFontMetricsF_strikeOutPos<RetType> {
  fn strikeOutPos(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  qreal QFontMetricsF::strikeOutPos();
impl<'a> /*trait*/ QFontMetricsF_strikeOutPos<f64> for () {
  fn strikeOutPos(self , rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF12strikeOutPosEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF12strikeOutPosEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QFontMetricsF::lineSpacing();
impl /*struct*/ QFontMetricsF {
  pub fn lineSpacing<RetType, T: QFontMetricsF_lineSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lineSpacing(self);
    // return 1;
  }
}

pub trait QFontMetricsF_lineSpacing<RetType> {
  fn lineSpacing(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  qreal QFontMetricsF::lineSpacing();
impl<'a> /*trait*/ QFontMetricsF_lineSpacing<f64> for () {
  fn lineSpacing(self , rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF11lineSpacingEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF11lineSpacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QFontMetricsF::averageCharWidth();
impl /*struct*/ QFontMetricsF {
  pub fn averageCharWidth<RetType, T: QFontMetricsF_averageCharWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.averageCharWidth(self);
    // return 1;
  }
}

pub trait QFontMetricsF_averageCharWidth<RetType> {
  fn averageCharWidth(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  qreal QFontMetricsF::averageCharWidth();
impl<'a> /*trait*/ QFontMetricsF_averageCharWidth<f64> for () {
  fn averageCharWidth(self , rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF16averageCharWidthEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF16averageCharWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QFontMetricsF::QFontMetricsF(const QFont & , QPaintDevice * pd);
impl<'a> /*trait*/ QFontMetricsF_NewQFontMetricsF for (QFont, QPaintDevice) {
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

  // proto:  qreal QFontMetricsF::leading();
impl /*struct*/ QFontMetricsF {
  pub fn leading<RetType, T: QFontMetricsF_leading<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.leading(self);
    // return 1;
  }
}

pub trait QFontMetricsF_leading<RetType> {
  fn leading(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  qreal QFontMetricsF::leading();
impl<'a> /*trait*/ QFontMetricsF_leading<f64> for () {
  fn leading(self , rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF7leadingEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF7leadingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QFontMetricsF::QFontMetricsF(const QFont & );
impl<'a> /*trait*/ QFontMetricsF_NewQFontMetricsF for (QFont) {
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

  // proto:  QRectF QFontMetricsF::boundingRect(QChar );
impl<'a> /*trait*/ QFontMetricsF_boundingRect<QRectF> for (QChar) {
  fn boundingRect(self , rsthis: &mut QFontMetricsF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF12boundingRectE5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFontMetricsF12boundingRectE5QChar(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QFontMetricsF::inFontUcs4(uint ucs4);
impl /*struct*/ QFontMetricsF {
  pub fn inFontUcs4<RetType, T: QFontMetricsF_inFontUcs4<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.inFontUcs4(self);
    // return 1;
  }
}

pub trait QFontMetricsF_inFontUcs4<RetType> {
  fn inFontUcs4(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  bool QFontMetricsF::inFontUcs4(uint ucs4);
impl<'a> /*trait*/ QFontMetricsF_inFontUcs4<i8> for (u32) {
  fn inFontUcs4(self , rsthis: &mut QFontMetricsF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF10inFontUcs4Ej()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZNK13QFontMetricsF10inFontUcs4Ej(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QFontMetricsF::minLeftBearing();
impl /*struct*/ QFontMetricsF {
  pub fn minLeftBearing<RetType, T: QFontMetricsF_minLeftBearing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minLeftBearing(self);
    // return 1;
  }
}

pub trait QFontMetricsF_minLeftBearing<RetType> {
  fn minLeftBearing(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  qreal QFontMetricsF::minLeftBearing();
impl<'a> /*trait*/ QFontMetricsF_minLeftBearing<f64> for () {
  fn minLeftBearing(self , rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF14minLeftBearingEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF14minLeftBearingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QFontMetricsF::ascent();
impl /*struct*/ QFontMetricsF {
  pub fn ascent<RetType, T: QFontMetricsF_ascent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.ascent(self);
    // return 1;
  }
}

pub trait QFontMetricsF_ascent<RetType> {
  fn ascent(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  qreal QFontMetricsF::ascent();
impl<'a> /*trait*/ QFontMetricsF_ascent<f64> for () {
  fn ascent(self , rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF6ascentEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF6ascentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QFontMetricsF::maxWidth();
impl /*struct*/ QFontMetricsF {
  pub fn maxWidth<RetType, T: QFontMetricsF_maxWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maxWidth(self);
    // return 1;
  }
}

pub trait QFontMetricsF_maxWidth<RetType> {
  fn maxWidth(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  qreal QFontMetricsF::maxWidth();
impl<'a> /*trait*/ QFontMetricsF_maxWidth<f64> for () {
  fn maxWidth(self , rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF8maxWidthEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF8maxWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QFontMetricsF::underlinePos();
impl /*struct*/ QFontMetricsF {
  pub fn underlinePos<RetType, T: QFontMetricsF_underlinePos<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.underlinePos(self);
    // return 1;
  }
}

pub trait QFontMetricsF_underlinePos<RetType> {
  fn underlinePos(self , rsthis: &mut QFontMetricsF) -> RetType;
}

  // proto:  qreal QFontMetricsF::underlinePos();
impl<'a> /*trait*/ QFontMetricsF_underlinePos<f64> for () {
  fn underlinePos(self , rsthis: &mut QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFontMetricsF12underlinePosEv()};
    let mut ret = unsafe {_ZNK13QFontMetricsF12underlinePosEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// <= body block end
