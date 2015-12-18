// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qlocale::QLocale;
use super::qchar::QChar;
use super::qstring::QString;
use super::qcollatorsortkey::QCollatorSortKey;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QCollator::numericMode();
  fn _ZNK9QCollator11numericModeEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QCollator::NewQCollator(const QLocale & locale);
  fn _ZN9QCollatorC1ERK7QLocale(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QCollator::setLocale(const QLocale & locale);
  fn _ZN9QCollator9setLocaleERK7QLocale(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QCollator::setNumericMode(bool on);
  fn _ZN9QCollator14setNumericModeEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QCollator::compare(const QChar * s1, int len1, const QChar * s2, int len2);
  fn _ZNK9QCollator7compareEPK5QChariS2_i(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void, arg3: c_int) -> c_int;
  // proto:  QCollatorSortKey QCollator::sortKey(const QString & string);
  fn _ZNK9QCollator7sortKeyERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QCollator::compare(const QString & s1, const QString & s2);
  fn _ZNK9QCollator7compareERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_int;
  // proto:  void QCollator::FreeQCollator();
  fn _ZN9QCollatorD0Ev(qthis: *mut c_void) ;
  // proto:  bool QCollator::ignorePunctuation();
  fn _ZNK9QCollator17ignorePunctuationEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QCollator::NewQCollator(const QCollator & );
  fn _ZN9QCollatorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QLocale QCollator::locale();
  fn _ZNK9QCollator6localeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCollator::swap(QCollator & other);
  fn _ZN9QCollator4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QCollator::setIgnorePunctuation(bool on);
  fn _ZN9QCollator20setIgnorePunctuationEb(qthis: *mut c_void, arg0: int8_t) ;
}

// body block begin
// class sizeof(QCollator)=8
pub struct QCollator {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QCollator {
  pub fn numericMode<RetType, T: QCollator_numericMode<RetType>>(&mut self, value: T) -> RetType {
    return value.numericMode(self);
    // return 1;
  }
}

pub trait QCollator_numericMode<RetType> {
  fn numericMode(self, rsthis: &mut QCollator) -> RetType;
}

// proto:  bool QCollator::numericMode();
impl<'a> /*trait*/ QCollator_numericMode<i8> for () {
  fn numericMode(self, rsthis: &mut QCollator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCollator11numericModeEv()};
    let mut ret = unsafe {_ZNK9QCollator11numericModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QCollator {
  pub fn NewQCollator<T: QCollator_NewQCollator>(value: T) -> QCollator {
    let rsthis = value.NewQCollator();
    return rsthis;
    // return 1;
  }
}

pub trait QCollator_NewQCollator {
  fn NewQCollator(self) -> QCollator;
}

// proto: void QCollator::NewQCollator(const QLocale & locale);
impl<'a> /*trait*/ QCollator_NewQCollator for (&'a  QLocale) {
  fn NewQCollator(self) -> QCollator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCollatorC1ERK7QLocale()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QCollatorC1ERK7QLocale(qthis, arg0)};
    let rsthis = QCollator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCollator {
  pub fn setLocale<RetType, T: QCollator_setLocale<RetType>>(&mut self, value: T) -> RetType {
    return value.setLocale(self);
    // return 1;
  }
}

pub trait QCollator_setLocale<RetType> {
  fn setLocale(self, rsthis: &mut QCollator) -> RetType;
}

// proto:  void QCollator::setLocale(const QLocale & locale);
impl<'a> /*trait*/ QCollator_setLocale<()> for (&'a  QLocale) {
  fn setLocale(self, rsthis: &mut QCollator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCollator9setLocaleERK7QLocale()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QCollator9setLocaleERK7QLocale(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCollator {
  pub fn setNumericMode<RetType, T: QCollator_setNumericMode<RetType>>(&mut self, value: T) -> RetType {
    return value.setNumericMode(self);
    // return 1;
  }
}

pub trait QCollator_setNumericMode<RetType> {
  fn setNumericMode(self, rsthis: &mut QCollator) -> RetType;
}

// proto:  void QCollator::setNumericMode(bool on);
impl<'a> /*trait*/ QCollator_setNumericMode<()> for (i8) {
  fn setNumericMode(self, rsthis: &mut QCollator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCollator14setNumericModeEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QCollator14setNumericModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCollator {
  pub fn compare<RetType, T: QCollator_compare<RetType>>(&mut self, value: T) -> RetType {
    return value.compare(self);
    // return 1;
  }
}

pub trait QCollator_compare<RetType> {
  fn compare(self, rsthis: &mut QCollator) -> RetType;
}

// proto:  int QCollator::compare(const QChar * s1, int len1, const QChar * s2, int len2);
impl<'a> /*trait*/ QCollator_compare<i32> for (&'a  QChar, i32, &'a  QChar, i32) {
  fn compare(self, rsthis: &mut QCollator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCollator7compareEPK5QChariS2_i()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZNK9QCollator7compareEPK5QChariS2_i(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QCollator {
  pub fn sortKey<RetType, T: QCollator_sortKey<RetType>>(&mut self, value: T) -> RetType {
    return value.sortKey(self);
    // return 1;
  }
}

pub trait QCollator_sortKey<RetType> {
  fn sortKey(self, rsthis: &mut QCollator) -> RetType;
}

// proto:  QCollatorSortKey QCollator::sortKey(const QString & string);
impl<'a> /*trait*/ QCollator_sortKey<QCollatorSortKey> for (&'a  QString) {
  fn sortKey(self, rsthis: &mut QCollator) -> QCollatorSortKey {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCollator7sortKeyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QCollator7sortKeyERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QCollatorSortKey{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QCollator::compare(const QString & s1, const QString & s2);
impl<'a> /*trait*/ QCollator_compare<i32> for (&'a  QString, &'a  QString) {
  fn compare(self, rsthis: &mut QCollator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCollator7compareERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QCollator7compareERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QCollator {
  pub fn FreeQCollator<RetType, T: QCollator_FreeQCollator<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQCollator(self);
    // return 1;
  }
}

pub trait QCollator_FreeQCollator<RetType> {
  fn FreeQCollator(self, rsthis: &mut QCollator) -> RetType;
}

// proto:  void QCollator::FreeQCollator();
impl<'a> /*trait*/ QCollator_FreeQCollator<()> for () {
  fn FreeQCollator(self, rsthis: &mut QCollator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCollatorD0Ev()};
     unsafe {_ZN9QCollatorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCollator {
  pub fn ignorePunctuation<RetType, T: QCollator_ignorePunctuation<RetType>>(&mut self, value: T) -> RetType {
    return value.ignorePunctuation(self);
    // return 1;
  }
}

pub trait QCollator_ignorePunctuation<RetType> {
  fn ignorePunctuation(self, rsthis: &mut QCollator) -> RetType;
}

// proto:  bool QCollator::ignorePunctuation();
impl<'a> /*trait*/ QCollator_ignorePunctuation<i8> for () {
  fn ignorePunctuation(self, rsthis: &mut QCollator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCollator17ignorePunctuationEv()};
    let mut ret = unsafe {_ZNK9QCollator17ignorePunctuationEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QCollator::NewQCollator(const QCollator & );
impl<'a> /*trait*/ QCollator_NewQCollator for (&'a  QCollator) {
  fn NewQCollator(self) -> QCollator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCollatorC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QCollatorC1ERKS_(qthis, arg0)};
    let rsthis = QCollator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCollator {
  pub fn locale<RetType, T: QCollator_locale<RetType>>(&mut self, value: T) -> RetType {
    return value.locale(self);
    // return 1;
  }
}

pub trait QCollator_locale<RetType> {
  fn locale(self, rsthis: &mut QCollator) -> RetType;
}

// proto:  QLocale QCollator::locale();
impl<'a> /*trait*/ QCollator_locale<QLocale> for () {
  fn locale(self, rsthis: &mut QCollator) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCollator6localeEv()};
    let mut ret = unsafe {_ZNK9QCollator6localeEv(rsthis.qclsinst)};
    let mut ret1 = QLocale{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCollator {
  pub fn swap<RetType, T: QCollator_swap<RetType>>(&mut self, value: T) -> RetType {
    return value.swap(self);
    // return 1;
  }
}

pub trait QCollator_swap<RetType> {
  fn swap(self, rsthis: &mut QCollator) -> RetType;
}

// proto:  void QCollator::swap(QCollator & other);
impl<'a> /*trait*/ QCollator_swap<()> for (&'a mut QCollator) {
  fn swap(self, rsthis: &mut QCollator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCollator4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QCollator4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCollator {
  pub fn setIgnorePunctuation<RetType, T: QCollator_setIgnorePunctuation<RetType>>(&mut self, value: T) -> RetType {
    return value.setIgnorePunctuation(self);
    // return 1;
  }
}

pub trait QCollator_setIgnorePunctuation<RetType> {
  fn setIgnorePunctuation(self, rsthis: &mut QCollator) -> RetType;
}

// proto:  void QCollator::setIgnorePunctuation(bool on);
impl<'a> /*trait*/ QCollator_setIgnorePunctuation<()> for (i8) {
  fn setIgnorePunctuation(self, rsthis: &mut QCollator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCollator20setIgnorePunctuationEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QCollator20setIgnorePunctuationEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

