// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
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
use std::ops::Deref;
use super::qstring::*; // 773
use super::qchar::*; // 773
use super::qurl::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QUrlQuery_Class_Size() -> c_int;
  // proto:  void QUrlQuery::QUrlQuery(const QString & queryString);
  fn C_ZN9QUrlQueryC2ERK7QString(arg0: *mut c_void) -> u64;
  // proto:  void QUrlQuery::clear();
  fn C_ZN9QUrlQuery5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QUrlQuery::setQuery(const QString & queryString);
  fn C_ZN9QUrlQuery8setQueryERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QChar QUrlQuery::queryValueDelimiter();
  fn C_ZNK9QUrlQuery19queryValueDelimiterEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QChar QUrlQuery::queryPairDelimiter();
  fn C_ZNK9QUrlQuery18queryPairDelimiterEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static QChar QUrlQuery::defaultQueryValueDelimiter();
  fn C_ZN9QUrlQuery26defaultQueryValueDelimiterEv() -> *mut c_void;
  // proto:  void QUrlQuery::swap(QUrlQuery & other);
  fn C_ZN9QUrlQuery4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QUrlQuery::isDetached();
  fn C_ZNK9QUrlQuery10isDetachedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QUrlQuery::QUrlQuery();
  fn C_ZN9QUrlQueryC2Ev() -> u64;
  // proto:  void QUrlQuery::setQueryDelimiters(QChar valueDelimiter, QChar pairDelimiter);
  fn C_ZN9QUrlQuery18setQueryDelimitersE5QCharS0_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QUrlQuery::~QUrlQuery();
  fn C_ZN9QUrlQueryD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QUrlQuery::removeAllQueryItems(const QString & key);
  fn C_ZN9QUrlQuery19removeAllQueryItemsERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QUrlQuery::isEmpty();
  fn C_ZNK9QUrlQuery7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QUrlQuery::removeQueryItem(const QString & key);
  fn C_ZN9QUrlQuery15removeQueryItemERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static QChar QUrlQuery::defaultQueryPairDelimiter();
  fn C_ZN9QUrlQuery25defaultQueryPairDelimiterEv() -> *mut c_void;
  // proto:  void QUrlQuery::QUrlQuery(const QUrl & url);
  fn C_ZN9QUrlQueryC2ERK4QUrl(arg0: *mut c_void) -> u64;
  // proto:  void QUrlQuery::addQueryItem(const QString & key, const QString & value);
  fn C_ZN9QUrlQuery12addQueryItemERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QUrlQuery::QUrlQuery(const QUrlQuery & other);
  fn C_ZN9QUrlQueryC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  bool QUrlQuery::hasQueryItem(const QString & key);
  fn C_ZNK9QUrlQuery12hasQueryItemERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QUrlQuery)=1
#[derive(Default)]
pub struct QUrlQuery {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QUrlQuery {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QUrlQuery {
    return QUrlQuery{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QUrlQuery::QUrlQuery(const QString & queryString);
impl /*struct*/ QUrlQuery {
  pub fn new<T: QUrlQuery_new>(value: T) -> QUrlQuery {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QUrlQuery_new {
  fn new(self) -> QUrlQuery;
}

  // proto:  void QUrlQuery::QUrlQuery(const QString & queryString);
impl<'a> /*trait*/ QUrlQuery_new for (&'a QString) {
  fn new(self) -> QUrlQuery {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQueryC2ERK7QString()};
    let ctysz: c_int = unsafe{QUrlQuery_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QUrlQueryC2ERK7QString(arg0)};
    let rsthis = QUrlQuery{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QUrlQuery::clear();
impl /*struct*/ QUrlQuery {
  pub fn clear<RetType, T: QUrlQuery_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QUrlQuery_clear<RetType> {
  fn clear(self , rsthis: & QUrlQuery) -> RetType;
}

  // proto:  void QUrlQuery::clear();
impl<'a> /*trait*/ QUrlQuery_clear<()> for () {
  fn clear(self , rsthis: & QUrlQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery5clearEv()};
     unsafe {C_ZN9QUrlQuery5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QUrlQuery::setQuery(const QString & queryString);
impl /*struct*/ QUrlQuery {
  pub fn setQuery<RetType, T: QUrlQuery_setQuery<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setQuery(self);
    // return 1;
  }
}

pub trait QUrlQuery_setQuery<RetType> {
  fn setQuery(self , rsthis: & QUrlQuery) -> RetType;
}

  // proto:  void QUrlQuery::setQuery(const QString & queryString);
impl<'a> /*trait*/ QUrlQuery_setQuery<()> for (&'a QString) {
  fn setQuery(self , rsthis: & QUrlQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery8setQueryERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QUrlQuery8setQueryERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QChar QUrlQuery::queryValueDelimiter();
impl /*struct*/ QUrlQuery {
  pub fn queryValueDelimiter<RetType, T: QUrlQuery_queryValueDelimiter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.queryValueDelimiter(self);
    // return 1;
  }
}

pub trait QUrlQuery_queryValueDelimiter<RetType> {
  fn queryValueDelimiter(self , rsthis: & QUrlQuery) -> RetType;
}

  // proto:  QChar QUrlQuery::queryValueDelimiter();
impl<'a> /*trait*/ QUrlQuery_queryValueDelimiter<QChar> for () {
  fn queryValueDelimiter(self , rsthis: & QUrlQuery) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery19queryValueDelimiterEv()};
    let mut ret = unsafe {C_ZNK9QUrlQuery19queryValueDelimiterEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QChar QUrlQuery::queryPairDelimiter();
impl /*struct*/ QUrlQuery {
  pub fn queryPairDelimiter<RetType, T: QUrlQuery_queryPairDelimiter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.queryPairDelimiter(self);
    // return 1;
  }
}

pub trait QUrlQuery_queryPairDelimiter<RetType> {
  fn queryPairDelimiter(self , rsthis: & QUrlQuery) -> RetType;
}

  // proto:  QChar QUrlQuery::queryPairDelimiter();
impl<'a> /*trait*/ QUrlQuery_queryPairDelimiter<QChar> for () {
  fn queryPairDelimiter(self , rsthis: & QUrlQuery) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery18queryPairDelimiterEv()};
    let mut ret = unsafe {C_ZNK9QUrlQuery18queryPairDelimiterEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
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
    let mut ret = unsafe {C_ZN9QUrlQuery26defaultQueryValueDelimiterEv()};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QUrlQuery::swap(QUrlQuery & other);
impl /*struct*/ QUrlQuery {
  pub fn swap<RetType, T: QUrlQuery_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QUrlQuery_swap<RetType> {
  fn swap(self , rsthis: & QUrlQuery) -> RetType;
}

  // proto:  void QUrlQuery::swap(QUrlQuery & other);
impl<'a> /*trait*/ QUrlQuery_swap<()> for (&'a QUrlQuery) {
  fn swap(self , rsthis: & QUrlQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QUrlQuery4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QUrlQuery::isDetached();
impl /*struct*/ QUrlQuery {
  pub fn isDetached<RetType, T: QUrlQuery_isDetached<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDetached(self);
    // return 1;
  }
}

pub trait QUrlQuery_isDetached<RetType> {
  fn isDetached(self , rsthis: & QUrlQuery) -> RetType;
}

  // proto:  bool QUrlQuery::isDetached();
impl<'a> /*trait*/ QUrlQuery_isDetached<i8> for () {
  fn isDetached(self , rsthis: & QUrlQuery) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery10isDetachedEv()};
    let mut ret = unsafe {C_ZNK9QUrlQuery10isDetachedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QUrlQuery::QUrlQuery();
impl<'a> /*trait*/ QUrlQuery_new for () {
  fn new(self) -> QUrlQuery {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQueryC2Ev()};
    let ctysz: c_int = unsafe{QUrlQuery_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN9QUrlQueryC2Ev()};
    let rsthis = QUrlQuery{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QUrlQuery::setQueryDelimiters(QChar valueDelimiter, QChar pairDelimiter);
impl /*struct*/ QUrlQuery {
  pub fn setQueryDelimiters<RetType, T: QUrlQuery_setQueryDelimiters<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setQueryDelimiters(self);
    // return 1;
  }
}

pub trait QUrlQuery_setQueryDelimiters<RetType> {
  fn setQueryDelimiters(self , rsthis: & QUrlQuery) -> RetType;
}

  // proto:  void QUrlQuery::setQueryDelimiters(QChar valueDelimiter, QChar pairDelimiter);
impl<'a> /*trait*/ QUrlQuery_setQueryDelimiters<()> for (QChar, QChar) {
  fn setQueryDelimiters(self , rsthis: & QUrlQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery18setQueryDelimitersE5QCharS0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN9QUrlQuery18setQueryDelimitersE5QCharS0_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QUrlQuery::~QUrlQuery();
impl /*struct*/ QUrlQuery {
  pub fn free<RetType, T: QUrlQuery_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QUrlQuery_free<RetType> {
  fn free(self , rsthis: & QUrlQuery) -> RetType;
}

  // proto:  void QUrlQuery::~QUrlQuery();
impl<'a> /*trait*/ QUrlQuery_free<()> for () {
  fn free(self , rsthis: & QUrlQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQueryD2Ev()};
     unsafe {C_ZN9QUrlQueryD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QUrlQuery::removeAllQueryItems(const QString & key);
impl /*struct*/ QUrlQuery {
  pub fn removeAllQueryItems<RetType, T: QUrlQuery_removeAllQueryItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeAllQueryItems(self);
    // return 1;
  }
}

pub trait QUrlQuery_removeAllQueryItems<RetType> {
  fn removeAllQueryItems(self , rsthis: & QUrlQuery) -> RetType;
}

  // proto:  void QUrlQuery::removeAllQueryItems(const QString & key);
impl<'a> /*trait*/ QUrlQuery_removeAllQueryItems<()> for (&'a QString) {
  fn removeAllQueryItems(self , rsthis: & QUrlQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery19removeAllQueryItemsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QUrlQuery19removeAllQueryItemsERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QUrlQuery::isEmpty();
impl /*struct*/ QUrlQuery {
  pub fn isEmpty<RetType, T: QUrlQuery_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QUrlQuery_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QUrlQuery) -> RetType;
}

  // proto:  bool QUrlQuery::isEmpty();
impl<'a> /*trait*/ QUrlQuery_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QUrlQuery) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery7isEmptyEv()};
    let mut ret = unsafe {C_ZNK9QUrlQuery7isEmptyEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QUrlQuery::removeQueryItem(const QString & key);
impl /*struct*/ QUrlQuery {
  pub fn removeQueryItem<RetType, T: QUrlQuery_removeQueryItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeQueryItem(self);
    // return 1;
  }
}

pub trait QUrlQuery_removeQueryItem<RetType> {
  fn removeQueryItem(self , rsthis: & QUrlQuery) -> RetType;
}

  // proto:  void QUrlQuery::removeQueryItem(const QString & key);
impl<'a> /*trait*/ QUrlQuery_removeQueryItem<()> for (&'a QString) {
  fn removeQueryItem(self , rsthis: & QUrlQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery15removeQueryItemERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QUrlQuery15removeQueryItemERK7QString(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {C_ZN9QUrlQuery25defaultQueryPairDelimiterEv()};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QUrlQuery::QUrlQuery(const QUrl & url);
impl<'a> /*trait*/ QUrlQuery_new for (&'a QUrl) {
  fn new(self) -> QUrlQuery {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQueryC2ERK4QUrl()};
    let ctysz: c_int = unsafe{QUrlQuery_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QUrlQueryC2ERK4QUrl(arg0)};
    let rsthis = QUrlQuery{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QUrlQuery::addQueryItem(const QString & key, const QString & value);
impl /*struct*/ QUrlQuery {
  pub fn addQueryItem<RetType, T: QUrlQuery_addQueryItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addQueryItem(self);
    // return 1;
  }
}

pub trait QUrlQuery_addQueryItem<RetType> {
  fn addQueryItem(self , rsthis: & QUrlQuery) -> RetType;
}

  // proto:  void QUrlQuery::addQueryItem(const QString & key, const QString & value);
impl<'a> /*trait*/ QUrlQuery_addQueryItem<()> for (&'a QString, &'a QString) {
  fn addQueryItem(self , rsthis: & QUrlQuery) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery12addQueryItemERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN9QUrlQuery12addQueryItemERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QUrlQuery::QUrlQuery(const QUrlQuery & other);
impl<'a> /*trait*/ QUrlQuery_new for (&'a QUrlQuery) {
  fn new(self) -> QUrlQuery {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQueryC2ERKS_()};
    let ctysz: c_int = unsafe{QUrlQuery_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QUrlQueryC2ERKS_(arg0)};
    let rsthis = QUrlQuery{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QUrlQuery::hasQueryItem(const QString & key);
impl /*struct*/ QUrlQuery {
  pub fn hasQueryItem<RetType, T: QUrlQuery_hasQueryItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasQueryItem(self);
    // return 1;
  }
}

pub trait QUrlQuery_hasQueryItem<RetType> {
  fn hasQueryItem(self , rsthis: & QUrlQuery) -> RetType;
}

  // proto:  bool QUrlQuery::hasQueryItem(const QString & key);
impl<'a> /*trait*/ QUrlQuery_hasQueryItem<i8> for (&'a QString) {
  fn hasQueryItem(self , rsthis: & QUrlQuery) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery12hasQueryItemERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK9QUrlQuery12hasQueryItemERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

// <= body block end

