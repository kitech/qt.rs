// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrectf::QRectF;
use super::qpointf::QPointF;
use super::qrawfont::QRawFont;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGlyphRun::~QGlyphRun();
  fn _ZN9QGlyphRunD0Ev(qthis: *mut c_void);
  // proto:  void QGlyphRun::setBoundingRect(const QRectF & boundingRect);
  fn _ZN9QGlyphRun15setBoundingRectERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QGlyphRun::overline();
  fn _ZNK9QGlyphRun8overlineEv(qthis: *mut c_void) -> c_char;
  // proto:  void QGlyphRun::setRawData(const quint32 * glyphIndexArray, const QPointF * glyphPositionArray, int size);
  fn _ZN9QGlyphRun10setRawDataEPKjPK7QPointFi(qthis: *mut c_void, arg0: *mut c_uint, arg1: *mut c_void, arg2: c_int);
  // proto:  void QGlyphRun::setOverline(bool overline);
  fn _ZN9QGlyphRun11setOverlineEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QGlyphRun::swap(QGlyphRun & other);
  fn _ZN9QGlyphRun4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGlyphRun::setUnderline(bool underline);
  fn _ZN9QGlyphRun12setUnderlineEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QVector<QPointF> QGlyphRun::positions();
  fn _ZNK9QGlyphRun9positionsEv(qthis: *mut c_void);
  // proto:  void QGlyphRun::clear();
  fn _ZN9QGlyphRun5clearEv(qthis: *mut c_void);
  // proto:  bool QGlyphRun::strikeOut();
  fn _ZNK9QGlyphRun9strikeOutEv(qthis: *mut c_void) -> c_char;
  // proto:  void QGlyphRun::QGlyphRun();
  fn _ZN9QGlyphRunC1Ev(qthis: *mut c_void);
  // proto:  QRawFont QGlyphRun::rawFont();
  fn _ZNK9QGlyphRun7rawFontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGlyphRun::setRawFont(const QRawFont & rawFont);
  fn _ZN9QGlyphRun10setRawFontERK8QRawFont(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGlyphRun::QGlyphRun(const QGlyphRun & other);
  fn _ZN9QGlyphRunC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QGlyphRun::isRightToLeft();
  fn _ZNK9QGlyphRun13isRightToLeftEv(qthis: *mut c_void) -> c_char;
  // proto:  QVector<quint32> QGlyphRun::glyphIndexes();
  fn _ZNK9QGlyphRun12glyphIndexesEv(qthis: *mut c_void);
  // proto:  QRectF QGlyphRun::boundingRect();
  fn _ZNK9QGlyphRun12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGlyphRun::setRightToLeft(bool on);
  fn _ZN9QGlyphRun14setRightToLeftEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QGlyphRun::underline();
  fn _ZNK9QGlyphRun9underlineEv(qthis: *mut c_void) -> c_char;
  // proto:  void QGlyphRun::setStrikeOut(bool strikeOut);
  fn _ZN9QGlyphRun12setStrikeOutEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QGlyphRun::isEmpty();
  fn _ZNK9QGlyphRun7isEmptyEv(qthis: *mut c_void) -> c_char;
}

// body block begin
// class sizeof(QGlyphRun)=1
pub struct QGlyphRun {
  pub qclsinst: *mut c_void,
}

  // proto:  void QGlyphRun::~QGlyphRun();
impl /*struct*/ QGlyphRun {
  pub fn FreeQGlyphRun<RetType, T: QGlyphRun_FreeQGlyphRun<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGlyphRun(self);
    // return 1;
  }
}

pub trait QGlyphRun_FreeQGlyphRun<RetType> {
  fn FreeQGlyphRun(self , rsthis: &mut QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::~QGlyphRun();
impl<'a> /*trait*/ QGlyphRun_FreeQGlyphRun<()> for () {
  fn FreeQGlyphRun(self , rsthis: &mut QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRunD0Ev()};
     unsafe {_ZN9QGlyphRunD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGlyphRun::setBoundingRect(const QRectF & boundingRect);
impl /*struct*/ QGlyphRun {
  pub fn setBoundingRect<RetType, T: QGlyphRun_setBoundingRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBoundingRect(self);
    // return 1;
  }
}

pub trait QGlyphRun_setBoundingRect<RetType> {
  fn setBoundingRect(self , rsthis: &mut QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::setBoundingRect(const QRectF & boundingRect);
impl<'a> /*trait*/ QGlyphRun_setBoundingRect<()> for (QRectF) {
  fn setBoundingRect(self , rsthis: &mut QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun15setBoundingRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QGlyphRun15setBoundingRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QGlyphRun::overline();
impl /*struct*/ QGlyphRun {
  pub fn overline<RetType, T: QGlyphRun_overline<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.overline(self);
    // return 1;
  }
}

pub trait QGlyphRun_overline<RetType> {
  fn overline(self , rsthis: &mut QGlyphRun) -> RetType;
}

  // proto:  bool QGlyphRun::overline();
impl<'a> /*trait*/ QGlyphRun_overline<i8> for () {
  fn overline(self , rsthis: &mut QGlyphRun) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun8overlineEv()};
    let mut ret = unsafe {_ZNK9QGlyphRun8overlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGlyphRun::setRawData(const quint32 * glyphIndexArray, const QPointF * glyphPositionArray, int size);
impl /*struct*/ QGlyphRun {
  pub fn setRawData<RetType, T: QGlyphRun_setRawData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRawData(self);
    // return 1;
  }
}

pub trait QGlyphRun_setRawData<RetType> {
  fn setRawData(self , rsthis: &mut QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::setRawData(const quint32 * glyphIndexArray, const QPointF * glyphPositionArray, int size);
impl<'a> /*trait*/ QGlyphRun_setRawData<()> for (&'a  Vec<u32>, QPointF, i32) {
  fn setRawData(self , rsthis: &mut QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun10setRawDataEPKjPK7QPointFi()};
    let arg0 = self.0.as_ptr()  as *mut c_uint;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN9QGlyphRun10setRawDataEPKjPK7QPointFi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QGlyphRun::setOverline(bool overline);
impl /*struct*/ QGlyphRun {
  pub fn setOverline<RetType, T: QGlyphRun_setOverline<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setOverline(self);
    // return 1;
  }
}

pub trait QGlyphRun_setOverline<RetType> {
  fn setOverline(self , rsthis: &mut QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::setOverline(bool overline);
impl<'a> /*trait*/ QGlyphRun_setOverline<()> for (i8) {
  fn setOverline(self , rsthis: &mut QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun11setOverlineEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QGlyphRun11setOverlineEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGlyphRun::swap(QGlyphRun & other);
impl /*struct*/ QGlyphRun {
  pub fn swap<RetType, T: QGlyphRun_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QGlyphRun_swap<RetType> {
  fn swap(self , rsthis: &mut QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::swap(QGlyphRun & other);
impl<'a> /*trait*/ QGlyphRun_swap<()> for (QGlyphRun) {
  fn swap(self , rsthis: &mut QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QGlyphRun4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGlyphRun::setUnderline(bool underline);
impl /*struct*/ QGlyphRun {
  pub fn setUnderline<RetType, T: QGlyphRun_setUnderline<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setUnderline(self);
    // return 1;
  }
}

pub trait QGlyphRun_setUnderline<RetType> {
  fn setUnderline(self , rsthis: &mut QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::setUnderline(bool underline);
impl<'a> /*trait*/ QGlyphRun_setUnderline<()> for (i8) {
  fn setUnderline(self , rsthis: &mut QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun12setUnderlineEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QGlyphRun12setUnderlineEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVector<QPointF> QGlyphRun::positions();
impl /*struct*/ QGlyphRun {
  pub fn positions<RetType, T: QGlyphRun_positions<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.positions(self);
    // return 1;
  }
}

pub trait QGlyphRun_positions<RetType> {
  fn positions(self , rsthis: &mut QGlyphRun) -> RetType;
}

  // proto:  QVector<QPointF> QGlyphRun::positions();
impl<'a> /*trait*/ QGlyphRun_positions<()> for () {
  fn positions(self , rsthis: &mut QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun9positionsEv()};
     unsafe {_ZNK9QGlyphRun9positionsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGlyphRun::clear();
impl /*struct*/ QGlyphRun {
  pub fn clear<RetType, T: QGlyphRun_clear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QGlyphRun_clear<RetType> {
  fn clear(self , rsthis: &mut QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::clear();
impl<'a> /*trait*/ QGlyphRun_clear<()> for () {
  fn clear(self , rsthis: &mut QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun5clearEv()};
     unsafe {_ZN9QGlyphRun5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QGlyphRun::strikeOut();
impl /*struct*/ QGlyphRun {
  pub fn strikeOut<RetType, T: QGlyphRun_strikeOut<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.strikeOut(self);
    // return 1;
  }
}

pub trait QGlyphRun_strikeOut<RetType> {
  fn strikeOut(self , rsthis: &mut QGlyphRun) -> RetType;
}

  // proto:  bool QGlyphRun::strikeOut();
impl<'a> /*trait*/ QGlyphRun_strikeOut<i8> for () {
  fn strikeOut(self , rsthis: &mut QGlyphRun) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun9strikeOutEv()};
    let mut ret = unsafe {_ZNK9QGlyphRun9strikeOutEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGlyphRun::QGlyphRun();
impl /*struct*/ QGlyphRun {
  pub fn NewQGlyphRun<T: QGlyphRun_NewQGlyphRun>(value: T) -> QGlyphRun {
    let rsthis = value.NewQGlyphRun();
    return rsthis;
    // return 1;
  }
}

pub trait QGlyphRun_NewQGlyphRun {
  fn NewQGlyphRun(self) -> QGlyphRun;
}

  // proto:  void QGlyphRun::QGlyphRun();
impl<'a> /*trait*/ QGlyphRun_NewQGlyphRun for () {
  fn NewQGlyphRun(self) -> QGlyphRun {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRunC1Ev()};
    unsafe {_ZN9QGlyphRunC1Ev(qthis)};
    let rsthis = QGlyphRun{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QRawFont QGlyphRun::rawFont();
impl /*struct*/ QGlyphRun {
  pub fn rawFont<RetType, T: QGlyphRun_rawFont<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rawFont(self);
    // return 1;
  }
}

pub trait QGlyphRun_rawFont<RetType> {
  fn rawFont(self , rsthis: &mut QGlyphRun) -> RetType;
}

  // proto:  QRawFont QGlyphRun::rawFont();
impl<'a> /*trait*/ QGlyphRun_rawFont<QRawFont> for () {
  fn rawFont(self , rsthis: &mut QGlyphRun) -> QRawFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun7rawFontEv()};
    let mut ret = unsafe {_ZNK9QGlyphRun7rawFontEv(rsthis.qclsinst)};
    let mut ret1 = QRawFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGlyphRun::setRawFont(const QRawFont & rawFont);
impl /*struct*/ QGlyphRun {
  pub fn setRawFont<RetType, T: QGlyphRun_setRawFont<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRawFont(self);
    // return 1;
  }
}

pub trait QGlyphRun_setRawFont<RetType> {
  fn setRawFont(self , rsthis: &mut QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::setRawFont(const QRawFont & rawFont);
impl<'a> /*trait*/ QGlyphRun_setRawFont<()> for (QRawFont) {
  fn setRawFont(self , rsthis: &mut QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun10setRawFontERK8QRawFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QGlyphRun10setRawFontERK8QRawFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGlyphRun::QGlyphRun(const QGlyphRun & other);
impl<'a> /*trait*/ QGlyphRun_NewQGlyphRun for (QGlyphRun) {
  fn NewQGlyphRun(self) -> QGlyphRun {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRunC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QGlyphRunC1ERKS_(qthis, arg0)};
    let rsthis = QGlyphRun{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QGlyphRun::isRightToLeft();
impl /*struct*/ QGlyphRun {
  pub fn isRightToLeft<RetType, T: QGlyphRun_isRightToLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isRightToLeft(self);
    // return 1;
  }
}

pub trait QGlyphRun_isRightToLeft<RetType> {
  fn isRightToLeft(self , rsthis: &mut QGlyphRun) -> RetType;
}

  // proto:  bool QGlyphRun::isRightToLeft();
impl<'a> /*trait*/ QGlyphRun_isRightToLeft<i8> for () {
  fn isRightToLeft(self , rsthis: &mut QGlyphRun) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun13isRightToLeftEv()};
    let mut ret = unsafe {_ZNK9QGlyphRun13isRightToLeftEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QVector<quint32> QGlyphRun::glyphIndexes();
impl /*struct*/ QGlyphRun {
  pub fn glyphIndexes<RetType, T: QGlyphRun_glyphIndexes<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.glyphIndexes(self);
    // return 1;
  }
}

pub trait QGlyphRun_glyphIndexes<RetType> {
  fn glyphIndexes(self , rsthis: &mut QGlyphRun) -> RetType;
}

  // proto:  QVector<quint32> QGlyphRun::glyphIndexes();
impl<'a> /*trait*/ QGlyphRun_glyphIndexes<()> for () {
  fn glyphIndexes(self , rsthis: &mut QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun12glyphIndexesEv()};
     unsafe {_ZNK9QGlyphRun12glyphIndexesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRectF QGlyphRun::boundingRect();
impl /*struct*/ QGlyphRun {
  pub fn boundingRect<RetType, T: QGlyphRun_boundingRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QGlyphRun_boundingRect<RetType> {
  fn boundingRect(self , rsthis: &mut QGlyphRun) -> RetType;
}

  // proto:  QRectF QGlyphRun::boundingRect();
impl<'a> /*trait*/ QGlyphRun_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: &mut QGlyphRun) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun12boundingRectEv()};
    let mut ret = unsafe {_ZNK9QGlyphRun12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGlyphRun::setRightToLeft(bool on);
impl /*struct*/ QGlyphRun {
  pub fn setRightToLeft<RetType, T: QGlyphRun_setRightToLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRightToLeft(self);
    // return 1;
  }
}

pub trait QGlyphRun_setRightToLeft<RetType> {
  fn setRightToLeft(self , rsthis: &mut QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::setRightToLeft(bool on);
impl<'a> /*trait*/ QGlyphRun_setRightToLeft<()> for (i8) {
  fn setRightToLeft(self , rsthis: &mut QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun14setRightToLeftEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QGlyphRun14setRightToLeftEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QGlyphRun::underline();
impl /*struct*/ QGlyphRun {
  pub fn underline<RetType, T: QGlyphRun_underline<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.underline(self);
    // return 1;
  }
}

pub trait QGlyphRun_underline<RetType> {
  fn underline(self , rsthis: &mut QGlyphRun) -> RetType;
}

  // proto:  bool QGlyphRun::underline();
impl<'a> /*trait*/ QGlyphRun_underline<i8> for () {
  fn underline(self , rsthis: &mut QGlyphRun) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun9underlineEv()};
    let mut ret = unsafe {_ZNK9QGlyphRun9underlineEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGlyphRun::setStrikeOut(bool strikeOut);
impl /*struct*/ QGlyphRun {
  pub fn setStrikeOut<RetType, T: QGlyphRun_setStrikeOut<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setStrikeOut(self);
    // return 1;
  }
}

pub trait QGlyphRun_setStrikeOut<RetType> {
  fn setStrikeOut(self , rsthis: &mut QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::setStrikeOut(bool strikeOut);
impl<'a> /*trait*/ QGlyphRun_setStrikeOut<()> for (i8) {
  fn setStrikeOut(self , rsthis: &mut QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun12setStrikeOutEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QGlyphRun12setStrikeOutEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QGlyphRun::isEmpty();
impl /*struct*/ QGlyphRun {
  pub fn isEmpty<RetType, T: QGlyphRun_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QGlyphRun_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QGlyphRun) -> RetType;
}

  // proto:  bool QGlyphRun::isEmpty();
impl<'a> /*trait*/ QGlyphRun_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QGlyphRun) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun7isEmptyEv()};
    let mut ret = unsafe {_ZNK9QGlyphRun7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

