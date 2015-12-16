// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qchar::QChar;
use super::qurl::QUrl;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QUrlQuery::NewQUrlQuery(const QString & queryString);
  fn _ZN9QUrlQueryC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QUrlQuery::clear();
  fn _ZN9QUrlQuery5clearEv(qthis: *mut c_void) ;
  // proto:  void QUrlQuery::setQuery(const QString & queryString);
  fn _ZN9QUrlQuery8setQueryERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QChar QUrlQuery::queryValueDelimiter();
  fn _ZNK9QUrlQuery19queryValueDelimiterEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QChar QUrlQuery::queryPairDelimiter();
  fn _ZNK9QUrlQuery18queryPairDelimiterEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QChar QUrlQuery::defaultQueryValueDelimiter();
  fn _ZN9QUrlQuery26defaultQueryValueDelimiterEv() -> *mut c_void;
  // proto:  void QUrlQuery::swap(QUrlQuery & other);
  fn _ZN9QUrlQuery4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QUrlQuery::isDetached();
  fn _ZNK9QUrlQuery10isDetachedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QUrlQuery::NewQUrlQuery();
  fn _ZN9QUrlQueryC1Ev(qthis: *mut c_void) ;
  // proto:  void QUrlQuery::setQueryDelimiters(QChar valueDelimiter, QChar pairDelimiter);
  fn _ZN9QUrlQuery18setQueryDelimitersE5QCharS0_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QUrlQuery::FreeQUrlQuery();
  fn _ZN9QUrlQueryD0Ev(qthis: *mut c_void) ;
  // proto:  void QUrlQuery::removeAllQueryItems(const QString & key);
  fn _ZN9QUrlQuery19removeAllQueryItemsERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QUrlQuery::isEmpty();
  fn _ZNK9QUrlQuery7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QUrlQuery::removeQueryItem(const QString & key);
  fn _ZN9QUrlQuery15removeQueryItemERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QChar QUrlQuery::defaultQueryPairDelimiter();
  fn _ZN9QUrlQuery25defaultQueryPairDelimiterEv() -> *mut c_void;
  // proto:  void QUrlQuery::NewQUrlQuery(const QUrl & url);
  fn _ZN9QUrlQueryC1ERK4QUrl(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QUrlQuery::addQueryItem(const QString & key, const QString & value);
  fn _ZN9QUrlQuery12addQueryItemERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QUrlQuery::NewQUrlQuery(const QUrlQuery & other);
  fn _ZN9QUrlQueryC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QUrlQuery::hasQueryItem(const QString & key);
  fn _ZNK9QUrlQuery12hasQueryItemERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QUrlQuery)=1
pub struct QUrlQuery {
  pub qclsinst: *mut c_void,
}

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

// proto: void QUrlQuery::NewQUrlQuery(const QString & queryString);
impl<'a> /*trait*/ QUrlQuery_NewQUrlQuery for (&'a  QString) {
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

impl /*struct*/ QUrlQuery {
  pub fn clear<T: QUrlQuery_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QUrlQuery_clear {
  fn clear(self, rsthis: &mut QUrlQuery) ;
}

// proto:  void QUrlQuery::clear();
impl<'a> /*trait*/ QUrlQuery_clear for () {
  fn clear(self, rsthis: &mut QUrlQuery)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery5clearEv()};
     unsafe {_ZN9QUrlQuery5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn setQuery<T: QUrlQuery_setQuery>(&mut self, value: T)  {
     value.setQuery(self);
    // return 1;
  }
}

pub trait QUrlQuery_setQuery {
  fn setQuery(self, rsthis: &mut QUrlQuery) ;
}

// proto:  void QUrlQuery::setQuery(const QString & queryString);
impl<'a> /*trait*/ QUrlQuery_setQuery for (&'a  QString) {
  fn setQuery(self, rsthis: &mut QUrlQuery)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery8setQueryERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QUrlQuery8setQueryERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn queryValueDelimiter<T: QUrlQuery_queryValueDelimiter>(&mut self, value: T) -> QChar {
    return value.queryValueDelimiter(self);
    // return 1;
  }
}

pub trait QUrlQuery_queryValueDelimiter {
  fn queryValueDelimiter(self, rsthis: &mut QUrlQuery) -> QChar;
}

// proto:  QChar QUrlQuery::queryValueDelimiter();
impl<'a> /*trait*/ QUrlQuery_queryValueDelimiter for () {
  fn queryValueDelimiter(self, rsthis: &mut QUrlQuery) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery19queryValueDelimiterEv()};
    let mut ret = unsafe {_ZNK9QUrlQuery19queryValueDelimiterEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn queryPairDelimiter<T: QUrlQuery_queryPairDelimiter>(&mut self, value: T) -> QChar {
    return value.queryPairDelimiter(self);
    // return 1;
  }
}

pub trait QUrlQuery_queryPairDelimiter {
  fn queryPairDelimiter(self, rsthis: &mut QUrlQuery) -> QChar;
}

// proto:  QChar QUrlQuery::queryPairDelimiter();
impl<'a> /*trait*/ QUrlQuery_queryPairDelimiter for () {
  fn queryPairDelimiter(self, rsthis: &mut QUrlQuery) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery18queryPairDelimiterEv()};
    let mut ret = unsafe {_ZNK9QUrlQuery18queryPairDelimiterEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn defaultQueryValueDelimiter<T: QUrlQuery_defaultQueryValueDelimiter>(&mut self, value: T) -> QChar {
    return value.defaultQueryValueDelimiter(self);
    // return 1;
  }
}

pub trait QUrlQuery_defaultQueryValueDelimiter {
  fn defaultQueryValueDelimiter(self, rsthis: &mut QUrlQuery) -> QChar;
}

// proto: static QChar QUrlQuery::defaultQueryValueDelimiter();
impl<'a> /*trait*/ QUrlQuery_defaultQueryValueDelimiter for () {
  fn defaultQueryValueDelimiter(self, rsthis: &mut QUrlQuery) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery26defaultQueryValueDelimiterEv()};
    let mut ret = unsafe {_ZN9QUrlQuery26defaultQueryValueDelimiterEv()};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn swap<T: QUrlQuery_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QUrlQuery_swap {
  fn swap(self, rsthis: &mut QUrlQuery) ;
}

// proto:  void QUrlQuery::swap(QUrlQuery & other);
impl<'a> /*trait*/ QUrlQuery_swap for (&'a mut QUrlQuery) {
  fn swap(self, rsthis: &mut QUrlQuery)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QUrlQuery4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn isDetached<T: QUrlQuery_isDetached>(&mut self, value: T) -> i8 {
    return value.isDetached(self);
    // return 1;
  }
}

pub trait QUrlQuery_isDetached {
  fn isDetached(self, rsthis: &mut QUrlQuery) -> i8;
}

// proto:  bool QUrlQuery::isDetached();
impl<'a> /*trait*/ QUrlQuery_isDetached for () {
  fn isDetached(self, rsthis: &mut QUrlQuery) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery10isDetachedEv()};
    let mut ret = unsafe {_ZNK9QUrlQuery10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QUrlQuery::NewQUrlQuery();
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

impl /*struct*/ QUrlQuery {
  pub fn setQueryDelimiters<T: QUrlQuery_setQueryDelimiters>(&mut self, value: T)  {
     value.setQueryDelimiters(self);
    // return 1;
  }
}

pub trait QUrlQuery_setQueryDelimiters {
  fn setQueryDelimiters(self, rsthis: &mut QUrlQuery) ;
}

// proto:  void QUrlQuery::setQueryDelimiters(QChar valueDelimiter, QChar pairDelimiter);
impl<'a> /*trait*/ QUrlQuery_setQueryDelimiters for (QChar, QChar) {
  fn setQueryDelimiters(self, rsthis: &mut QUrlQuery)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery18setQueryDelimitersE5QCharS0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QUrlQuery18setQueryDelimitersE5QCharS0_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn FreeQUrlQuery<T: QUrlQuery_FreeQUrlQuery>(&mut self, value: T)  {
     value.FreeQUrlQuery(self);
    // return 1;
  }
}

pub trait QUrlQuery_FreeQUrlQuery {
  fn FreeQUrlQuery(self, rsthis: &mut QUrlQuery) ;
}

// proto:  void QUrlQuery::FreeQUrlQuery();
impl<'a> /*trait*/ QUrlQuery_FreeQUrlQuery for () {
  fn FreeQUrlQuery(self, rsthis: &mut QUrlQuery)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQueryD0Ev()};
     unsafe {_ZN9QUrlQueryD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn removeAllQueryItems<T: QUrlQuery_removeAllQueryItems>(&mut self, value: T)  {
     value.removeAllQueryItems(self);
    // return 1;
  }
}

pub trait QUrlQuery_removeAllQueryItems {
  fn removeAllQueryItems(self, rsthis: &mut QUrlQuery) ;
}

// proto:  void QUrlQuery::removeAllQueryItems(const QString & key);
impl<'a> /*trait*/ QUrlQuery_removeAllQueryItems for (&'a  QString) {
  fn removeAllQueryItems(self, rsthis: &mut QUrlQuery)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery19removeAllQueryItemsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QUrlQuery19removeAllQueryItemsERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn isEmpty<T: QUrlQuery_isEmpty>(&mut self, value: T) -> i8 {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QUrlQuery_isEmpty {
  fn isEmpty(self, rsthis: &mut QUrlQuery) -> i8;
}

// proto:  bool QUrlQuery::isEmpty();
impl<'a> /*trait*/ QUrlQuery_isEmpty for () {
  fn isEmpty(self, rsthis: &mut QUrlQuery) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery7isEmptyEv()};
    let mut ret = unsafe {_ZNK9QUrlQuery7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn removeQueryItem<T: QUrlQuery_removeQueryItem>(&mut self, value: T)  {
     value.removeQueryItem(self);
    // return 1;
  }
}

pub trait QUrlQuery_removeQueryItem {
  fn removeQueryItem(self, rsthis: &mut QUrlQuery) ;
}

// proto:  void QUrlQuery::removeQueryItem(const QString & key);
impl<'a> /*trait*/ QUrlQuery_removeQueryItem for (&'a  QString) {
  fn removeQueryItem(self, rsthis: &mut QUrlQuery)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery15removeQueryItemERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QUrlQuery15removeQueryItemERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn defaultQueryPairDelimiter<T: QUrlQuery_defaultQueryPairDelimiter>(&mut self, value: T) -> QChar {
    return value.defaultQueryPairDelimiter(self);
    // return 1;
  }
}

pub trait QUrlQuery_defaultQueryPairDelimiter {
  fn defaultQueryPairDelimiter(self, rsthis: &mut QUrlQuery) -> QChar;
}

// proto: static QChar QUrlQuery::defaultQueryPairDelimiter();
impl<'a> /*trait*/ QUrlQuery_defaultQueryPairDelimiter for () {
  fn defaultQueryPairDelimiter(self, rsthis: &mut QUrlQuery) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery25defaultQueryPairDelimiterEv()};
    let mut ret = unsafe {_ZN9QUrlQuery25defaultQueryPairDelimiterEv()};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QUrlQuery::NewQUrlQuery(const QUrl & url);
impl<'a> /*trait*/ QUrlQuery_NewQUrlQuery for (&'a  QUrl) {
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

impl /*struct*/ QUrlQuery {
  pub fn addQueryItem<T: QUrlQuery_addQueryItem>(&mut self, value: T)  {
     value.addQueryItem(self);
    // return 1;
  }
}

pub trait QUrlQuery_addQueryItem {
  fn addQueryItem(self, rsthis: &mut QUrlQuery) ;
}

// proto:  void QUrlQuery::addQueryItem(const QString & key, const QString & value);
impl<'a> /*trait*/ QUrlQuery_addQueryItem for (&'a  QString, &'a  QString) {
  fn addQueryItem(self, rsthis: &mut QUrlQuery)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery12addQueryItemERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QUrlQuery12addQueryItemERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto: void QUrlQuery::NewQUrlQuery(const QUrlQuery & other);
impl<'a> /*trait*/ QUrlQuery_NewQUrlQuery for (&'a  QUrlQuery) {
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

impl /*struct*/ QUrlQuery {
  pub fn hasQueryItem<T: QUrlQuery_hasQueryItem>(&mut self, value: T) -> i8 {
    return value.hasQueryItem(self);
    // return 1;
  }
}

pub trait QUrlQuery_hasQueryItem {
  fn hasQueryItem(self, rsthis: &mut QUrlQuery) -> i8;
}

// proto:  bool QUrlQuery::hasQueryItem(const QString & key);
impl<'a> /*trait*/ QUrlQuery_hasQueryItem for (&'a  QString) {
  fn hasQueryItem(self, rsthis: &mut QUrlQuery) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery12hasQueryItemERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QUrlQuery12hasQueryItemERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

