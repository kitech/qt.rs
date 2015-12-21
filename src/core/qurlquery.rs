// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qurlquery.h
// dst-file: /src/core/qurlquery.rs
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
use super::qstring::QString; // 773
use super::qchar::QChar; // 773
use super::qurl::QUrl; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QUrlQuery::QUrlQuery(const QString & queryString);
  fn _ZN9QUrlQueryC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QUrlQuery::clear();
  fn _ZN9QUrlQuery5clearEv(qthis: *mut c_void);
  // proto:  void QUrlQuery::setQuery(const QString & queryString);
  fn _ZN9QUrlQuery8setQueryERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QChar QUrlQuery::queryValueDelimiter();
  fn _ZNK9QUrlQuery19queryValueDelimiterEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QChar QUrlQuery::queryPairDelimiter();
  fn _ZNK9QUrlQuery18queryPairDelimiterEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QChar QUrlQuery::defaultQueryValueDelimiter();
  fn _ZN9QUrlQuery26defaultQueryValueDelimiterEv() -> *mut c_void;
  // proto:  void QUrlQuery::swap(QUrlQuery & other);
  fn _ZN9QUrlQuery4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QUrlQuery::isDetached();
  fn _ZNK9QUrlQuery10isDetachedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QUrlQuery::QUrlQuery();
  fn _ZN9QUrlQueryC1Ev(qthis: *mut c_void);
  // proto:  void QUrlQuery::setQueryDelimiters(QChar valueDelimiter, QChar pairDelimiter);
  fn _ZN9QUrlQuery18setQueryDelimitersE5QCharS0_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QUrlQuery::~QUrlQuery();
  fn _ZN9QUrlQueryD0Ev(qthis: *mut c_void);
  // proto:  void QUrlQuery::removeAllQueryItems(const QString & key);
  fn _ZN9QUrlQuery19removeAllQueryItemsERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QUrlQuery::isEmpty();
  fn _ZNK9QUrlQuery7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  void QUrlQuery::removeQueryItem(const QString & key);
  fn _ZN9QUrlQuery15removeQueryItemERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QChar QUrlQuery::defaultQueryPairDelimiter();
  fn _ZN9QUrlQuery25defaultQueryPairDelimiterEv() -> *mut c_void;
  // proto:  void QUrlQuery::QUrlQuery(const QUrl & url);
  fn _ZN9QUrlQueryC1ERK4QUrl(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QUrlQuery::addQueryItem(const QString & key, const QString & value);
  fn _ZN9QUrlQuery12addQueryItemERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QUrlQuery::QUrlQuery(const QUrlQuery & other);
  fn _ZN9QUrlQueryC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QUrlQuery::hasQueryItem(const QString & key);
  fn _ZNK9QUrlQuery12hasQueryItemERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QUrlQuery)=1
pub struct QUrlQuery {
  pub qclsinst: *mut c_void,
}

  // proto:  void QUrlQuery::QUrlQuery(const QString & queryString);
impl /*struct*/ QUrlQuery {
  pub fn NewQUrlQuery<T: QUrlQuery_NewQUrlQuery>(value: T) -> QUrlQuery {
    let rsthis = value.NewQUrlQuery();
    return rsthis;
    // return 1;
  }
}

pub trait QUrlQuery_NewQUrlQuery {
  fn NewQUrlQuery(self) -> QUrlQuery;
}

  // proto:  void QUrlQuery::QUrlQuery(const QString & queryString);
impl<'a> /*trait*/ QUrlQuery_NewQUrlQuery for (QString) {
  fn NewQUrlQuery(self) -> QUrlQuery {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQueryC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QUrlQueryC1ERK7QString(qthis, arg0)};
    let rsthis = QUrlQuery{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QUrlQuery::clear();
impl /*struct*/ QUrlQuery {
  pub fn clear<RetType, T: QUrlQuery_clear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QUrlQuery_clear<RetType> {
  fn clear(self , rsthis: &mut QUrlQuery) -> RetType;
}

  // proto:  void QUrlQuery::clear();
impl<'a> /*trait*/ QUrlQuery_clear<()> for () {
  fn clear(self , rsthis: &mut QUrlQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery5clearEv()};
     unsafe {_ZN9QUrlQuery5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QUrlQuery::setQuery(const QString & queryString);
impl /*struct*/ QUrlQuery {
  pub fn setQuery<RetType, T: QUrlQuery_setQuery<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setQuery(self);
    // return 1;
  }
}

pub trait QUrlQuery_setQuery<RetType> {
  fn setQuery(self , rsthis: &mut QUrlQuery) -> RetType;
}

  // proto:  void QUrlQuery::setQuery(const QString & queryString);
impl<'a> /*trait*/ QUrlQuery_setQuery<()> for (QString) {
  fn setQuery(self , rsthis: &mut QUrlQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery8setQueryERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QUrlQuery8setQueryERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QChar QUrlQuery::queryValueDelimiter();
impl /*struct*/ QUrlQuery {
  pub fn queryValueDelimiter<RetType, T: QUrlQuery_queryValueDelimiter<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.queryValueDelimiter(self);
    // return 1;
  }
}

pub trait QUrlQuery_queryValueDelimiter<RetType> {
  fn queryValueDelimiter(self , rsthis: &mut QUrlQuery) -> RetType;
}

  // proto:  QChar QUrlQuery::queryValueDelimiter();
impl<'a> /*trait*/ QUrlQuery_queryValueDelimiter<QChar> for () {
  fn queryValueDelimiter(self , rsthis: &mut QUrlQuery) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery19queryValueDelimiterEv()};
    let mut ret = unsafe {_ZNK9QUrlQuery19queryValueDelimiterEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QChar QUrlQuery::queryPairDelimiter();
impl /*struct*/ QUrlQuery {
  pub fn queryPairDelimiter<RetType, T: QUrlQuery_queryPairDelimiter<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.queryPairDelimiter(self);
    // return 1;
  }
}

pub trait QUrlQuery_queryPairDelimiter<RetType> {
  fn queryPairDelimiter(self , rsthis: &mut QUrlQuery) -> RetType;
}

  // proto:  QChar QUrlQuery::queryPairDelimiter();
impl<'a> /*trait*/ QUrlQuery_queryPairDelimiter<QChar> for () {
  fn queryPairDelimiter(self , rsthis: &mut QUrlQuery) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery18queryPairDelimiterEv()};
    let mut ret = unsafe {_ZNK9QUrlQuery18queryPairDelimiterEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static QChar QUrlQuery::defaultQueryValueDelimiter();
impl /*struct*/ QUrlQuery {
  pub fn defaultQueryValueDelimiter_s<RetType, T: QUrlQuery_defaultQueryValueDelimiter_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.defaultQueryValueDelimiter_s();
    // return 1;
  }
}

pub trait QUrlQuery_defaultQueryValueDelimiter_s<RetType> {
  fn defaultQueryValueDelimiter_s(self ) -> RetType;
}

  // proto: static QChar QUrlQuery::defaultQueryValueDelimiter();
impl<'a> /*trait*/ QUrlQuery_defaultQueryValueDelimiter_s<QChar> for () {
  fn defaultQueryValueDelimiter_s(self ) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery26defaultQueryValueDelimiterEv()};
    let mut ret = unsafe {_ZN9QUrlQuery26defaultQueryValueDelimiterEv()};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QUrlQuery::swap(QUrlQuery & other);
impl /*struct*/ QUrlQuery {
  pub fn swap<RetType, T: QUrlQuery_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QUrlQuery_swap<RetType> {
  fn swap(self , rsthis: &mut QUrlQuery) -> RetType;
}

  // proto:  void QUrlQuery::swap(QUrlQuery & other);
impl<'a> /*trait*/ QUrlQuery_swap<()> for (QUrlQuery) {
  fn swap(self , rsthis: &mut QUrlQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QUrlQuery4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QUrlQuery::isDetached();
impl /*struct*/ QUrlQuery {
  pub fn isDetached<RetType, T: QUrlQuery_isDetached<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isDetached(self);
    // return 1;
  }
}

pub trait QUrlQuery_isDetached<RetType> {
  fn isDetached(self , rsthis: &mut QUrlQuery) -> RetType;
}

  // proto:  bool QUrlQuery::isDetached();
impl<'a> /*trait*/ QUrlQuery_isDetached<i8> for () {
  fn isDetached(self , rsthis: &mut QUrlQuery) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery10isDetachedEv()};
    let mut ret = unsafe {_ZNK9QUrlQuery10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QUrlQuery::QUrlQuery();
impl<'a> /*trait*/ QUrlQuery_NewQUrlQuery for () {
  fn NewQUrlQuery(self) -> QUrlQuery {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQueryC1Ev()};
    unsafe {_ZN9QUrlQueryC1Ev(qthis)};
    let rsthis = QUrlQuery{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QUrlQuery::setQueryDelimiters(QChar valueDelimiter, QChar pairDelimiter);
impl /*struct*/ QUrlQuery {
  pub fn setQueryDelimiters<RetType, T: QUrlQuery_setQueryDelimiters<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setQueryDelimiters(self);
    // return 1;
  }
}

pub trait QUrlQuery_setQueryDelimiters<RetType> {
  fn setQueryDelimiters(self , rsthis: &mut QUrlQuery) -> RetType;
}

  // proto:  void QUrlQuery::setQueryDelimiters(QChar valueDelimiter, QChar pairDelimiter);
impl<'a> /*trait*/ QUrlQuery_setQueryDelimiters<()> for (QChar, QChar) {
  fn setQueryDelimiters(self , rsthis: &mut QUrlQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery18setQueryDelimitersE5QCharS0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QUrlQuery18setQueryDelimitersE5QCharS0_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QUrlQuery::~QUrlQuery();
impl /*struct*/ QUrlQuery {
  pub fn FreeQUrlQuery<RetType, T: QUrlQuery_FreeQUrlQuery<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQUrlQuery(self);
    // return 1;
  }
}

pub trait QUrlQuery_FreeQUrlQuery<RetType> {
  fn FreeQUrlQuery(self , rsthis: &mut QUrlQuery) -> RetType;
}

  // proto:  void QUrlQuery::~QUrlQuery();
impl<'a> /*trait*/ QUrlQuery_FreeQUrlQuery<()> for () {
  fn FreeQUrlQuery(self , rsthis: &mut QUrlQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQueryD0Ev()};
     unsafe {_ZN9QUrlQueryD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QUrlQuery::removeAllQueryItems(const QString & key);
impl /*struct*/ QUrlQuery {
  pub fn removeAllQueryItems<RetType, T: QUrlQuery_removeAllQueryItems<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removeAllQueryItems(self);
    // return 1;
  }
}

pub trait QUrlQuery_removeAllQueryItems<RetType> {
  fn removeAllQueryItems(self , rsthis: &mut QUrlQuery) -> RetType;
}

  // proto:  void QUrlQuery::removeAllQueryItems(const QString & key);
impl<'a> /*trait*/ QUrlQuery_removeAllQueryItems<()> for (QString) {
  fn removeAllQueryItems(self , rsthis: &mut QUrlQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery19removeAllQueryItemsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QUrlQuery19removeAllQueryItemsERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QUrlQuery::isEmpty();
impl /*struct*/ QUrlQuery {
  pub fn isEmpty<RetType, T: QUrlQuery_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QUrlQuery_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QUrlQuery) -> RetType;
}

  // proto:  bool QUrlQuery::isEmpty();
impl<'a> /*trait*/ QUrlQuery_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QUrlQuery) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery7isEmptyEv()};
    let mut ret = unsafe {_ZNK9QUrlQuery7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QUrlQuery::removeQueryItem(const QString & key);
impl /*struct*/ QUrlQuery {
  pub fn removeQueryItem<RetType, T: QUrlQuery_removeQueryItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removeQueryItem(self);
    // return 1;
  }
}

pub trait QUrlQuery_removeQueryItem<RetType> {
  fn removeQueryItem(self , rsthis: &mut QUrlQuery) -> RetType;
}

  // proto:  void QUrlQuery::removeQueryItem(const QString & key);
impl<'a> /*trait*/ QUrlQuery_removeQueryItem<()> for (QString) {
  fn removeQueryItem(self , rsthis: &mut QUrlQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery15removeQueryItemERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QUrlQuery15removeQueryItemERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QChar QUrlQuery::defaultQueryPairDelimiter();
impl /*struct*/ QUrlQuery {
  pub fn defaultQueryPairDelimiter_s<RetType, T: QUrlQuery_defaultQueryPairDelimiter_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.defaultQueryPairDelimiter_s();
    // return 1;
  }
}

pub trait QUrlQuery_defaultQueryPairDelimiter_s<RetType> {
  fn defaultQueryPairDelimiter_s(self ) -> RetType;
}

  // proto: static QChar QUrlQuery::defaultQueryPairDelimiter();
impl<'a> /*trait*/ QUrlQuery_defaultQueryPairDelimiter_s<QChar> for () {
  fn defaultQueryPairDelimiter_s(self ) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery25defaultQueryPairDelimiterEv()};
    let mut ret = unsafe {_ZN9QUrlQuery25defaultQueryPairDelimiterEv()};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QUrlQuery::QUrlQuery(const QUrl & url);
impl<'a> /*trait*/ QUrlQuery_NewQUrlQuery for (QUrl) {
  fn NewQUrlQuery(self) -> QUrlQuery {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQueryC1ERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QUrlQueryC1ERK4QUrl(qthis, arg0)};
    let rsthis = QUrlQuery{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QUrlQuery::addQueryItem(const QString & key, const QString & value);
impl /*struct*/ QUrlQuery {
  pub fn addQueryItem<RetType, T: QUrlQuery_addQueryItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addQueryItem(self);
    // return 1;
  }
}

pub trait QUrlQuery_addQueryItem<RetType> {
  fn addQueryItem(self , rsthis: &mut QUrlQuery) -> RetType;
}

  // proto:  void QUrlQuery::addQueryItem(const QString & key, const QString & value);
impl<'a> /*trait*/ QUrlQuery_addQueryItem<()> for (QString, QString) {
  fn addQueryItem(self , rsthis: &mut QUrlQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery12addQueryItemERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QUrlQuery12addQueryItemERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QUrlQuery::QUrlQuery(const QUrlQuery & other);
impl<'a> /*trait*/ QUrlQuery_NewQUrlQuery for (QUrlQuery) {
  fn NewQUrlQuery(self) -> QUrlQuery {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQueryC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QUrlQueryC1ERKS_(qthis, arg0)};
    let rsthis = QUrlQuery{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QUrlQuery::hasQueryItem(const QString & key);
impl /*struct*/ QUrlQuery {
  pub fn hasQueryItem<RetType, T: QUrlQuery_hasQueryItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasQueryItem(self);
    // return 1;
  }
}

pub trait QUrlQuery_hasQueryItem<RetType> {
  fn hasQueryItem(self , rsthis: &mut QUrlQuery) -> RetType;
}

  // proto:  bool QUrlQuery::hasQueryItem(const QString & key);
impl<'a> /*trait*/ QUrlQuery_hasQueryItem<i8> for (QString) {
  fn hasQueryItem(self , rsthis: &mut QUrlQuery) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery12hasQueryItemERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QUrlQuery12hasQueryItemERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

