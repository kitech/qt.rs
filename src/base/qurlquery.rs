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
  fn _ZN9QUrlQueryC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN9QUrlQuery5clearEv() -> i32;
  fn _ZN9QUrlQuery8setQueryERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK9QUrlQuery19queryValueDelimiterEv() -> i32;
  fn _ZNK9QUrlQuery18queryPairDelimiterEv() -> i32;
  fn _ZN9QUrlQuery26defaultQueryValueDelimiterEv() -> i32;
  fn _ZN9QUrlQuery4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZNK9QUrlQuery10isDetachedEv() -> i32;
  fn _ZN9QUrlQueryC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN9QUrlQuery18setQueryDelimitersE5QCharS0_(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  fn _ZN9QUrlQueryD0Ev() -> i32;
  fn _ZN9QUrlQuery19removeAllQueryItemsERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK9QUrlQuery7isEmptyEv() -> i32;
  fn _ZN9QUrlQuery15removeQueryItemERK7QString(arg0: *const c_void) -> i32;
  fn _ZN9QUrlQuery25defaultQueryPairDelimiterEv() -> i32;
  fn _ZN9QUrlQueryC1ERK4QUrl(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN9QUrlQuery12addQueryItemERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN9QUrlQueryC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK9QUrlQuery12hasQueryItemERK7QString(arg0: *const c_void) -> i32;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QUrlQueryC1ERK7QString(qthis, arg0)};
    let rsthis = QUrlQuery{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn clear<T: QUrlQuery_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QUrlQuery_clear {
  fn clear(self, this: &mut QUrlQuery) -> i32;
}

// proto: void QUrlQuery::clear();
impl<'a> /*trait*/ QUrlQuery_clear for () {
  fn clear(self, this: &mut QUrlQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery5clearEv()};
    unsafe {_ZN9QUrlQuery5clearEv()};
    return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn setQuery<T: QUrlQuery_setQuery>(&mut self, value: T) -> i32 {
    value.setQuery(self);
    return 1;
  }
}

pub trait QUrlQuery_setQuery {
  fn setQuery(self, this: &mut QUrlQuery) -> i32;
}

// proto: void QUrlQuery::setQuery(const QString & queryString);
impl<'a> /*trait*/ QUrlQuery_setQuery for (&'a  QString) {
  fn setQuery(self, this: &mut QUrlQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery8setQueryERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QUrlQuery8setQueryERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn queryValueDelimiter<T: QUrlQuery_queryValueDelimiter>(&mut self, value: T) -> i32 {
    value.queryValueDelimiter(self);
    return 1;
  }
}

pub trait QUrlQuery_queryValueDelimiter {
  fn queryValueDelimiter(self, this: &mut QUrlQuery) -> i32;
}

// proto: QChar QUrlQuery::queryValueDelimiter();
impl<'a> /*trait*/ QUrlQuery_queryValueDelimiter for () {
  fn queryValueDelimiter(self, this: &mut QUrlQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery19queryValueDelimiterEv()};
    unsafe {_ZNK9QUrlQuery19queryValueDelimiterEv()};
    return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn queryPairDelimiter<T: QUrlQuery_queryPairDelimiter>(&mut self, value: T) -> i32 {
    value.queryPairDelimiter(self);
    return 1;
  }
}

pub trait QUrlQuery_queryPairDelimiter {
  fn queryPairDelimiter(self, this: &mut QUrlQuery) -> i32;
}

// proto: QChar QUrlQuery::queryPairDelimiter();
impl<'a> /*trait*/ QUrlQuery_queryPairDelimiter for () {
  fn queryPairDelimiter(self, this: &mut QUrlQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery18queryPairDelimiterEv()};
    unsafe {_ZNK9QUrlQuery18queryPairDelimiterEv()};
    return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn defaultQueryValueDelimiter<T: QUrlQuery_defaultQueryValueDelimiter>(&mut self, value: T) -> i32 {
    value.defaultQueryValueDelimiter(self);
    return 1;
  }
}

pub trait QUrlQuery_defaultQueryValueDelimiter {
  fn defaultQueryValueDelimiter(self, this: &mut QUrlQuery) -> i32;
}

// proto: QChar QUrlQuery::defaultQueryValueDelimiter();
impl<'a> /*trait*/ QUrlQuery_defaultQueryValueDelimiter for () {
  fn defaultQueryValueDelimiter(self, this: &mut QUrlQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery26defaultQueryValueDelimiterEv()};
    unsafe {_ZN9QUrlQuery26defaultQueryValueDelimiterEv()};
    return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn swap<T: QUrlQuery_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QUrlQuery_swap {
  fn swap(self, this: &mut QUrlQuery) -> i32;
}

// proto: void QUrlQuery::swap(QUrlQuery & other);
impl<'a> /*trait*/ QUrlQuery_swap for (&'a mut QUrlQuery) {
  fn swap(self, this: &mut QUrlQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QUrlQuery4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn isDetached<T: QUrlQuery_isDetached>(&mut self, value: T) -> i32 {
    value.isDetached(self);
    return 1;
  }
}

pub trait QUrlQuery_isDetached {
  fn isDetached(self, this: &mut QUrlQuery) -> i32;
}

// proto: bool QUrlQuery::isDetached();
impl<'a> /*trait*/ QUrlQuery_isDetached for () {
  fn isDetached(self, this: &mut QUrlQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery10isDetachedEv()};
    unsafe {_ZNK9QUrlQuery10isDetachedEv()};
    return 1;
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
  pub fn setQueryDelimiters<T: QUrlQuery_setQueryDelimiters>(&mut self, value: T) -> i32 {
    value.setQueryDelimiters(self);
    return 1;
  }
}

pub trait QUrlQuery_setQueryDelimiters {
  fn setQueryDelimiters(self, this: &mut QUrlQuery) -> i32;
}

// proto: void QUrlQuery::setQueryDelimiters(QChar valueDelimiter, QChar pairDelimiter);
impl<'a> /*trait*/ QUrlQuery_setQueryDelimiters for (QChar, QChar) {
  fn setQueryDelimiters(self, this: &mut QUrlQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery18setQueryDelimitersE5QCharS0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QUrlQuery18setQueryDelimitersE5QCharS0_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn FreeQUrlQuery<T: QUrlQuery_FreeQUrlQuery>(&mut self, value: T) -> i32 {
    value.FreeQUrlQuery(self);
    return 1;
  }
}

pub trait QUrlQuery_FreeQUrlQuery {
  fn FreeQUrlQuery(self, this: &mut QUrlQuery) -> i32;
}

// proto: void QUrlQuery::FreeQUrlQuery();
impl<'a> /*trait*/ QUrlQuery_FreeQUrlQuery for () {
  fn FreeQUrlQuery(self, this: &mut QUrlQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQueryD0Ev()};
    unsafe {_ZN9QUrlQueryD0Ev()};
    return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn removeAllQueryItems<T: QUrlQuery_removeAllQueryItems>(&mut self, value: T) -> i32 {
    value.removeAllQueryItems(self);
    return 1;
  }
}

pub trait QUrlQuery_removeAllQueryItems {
  fn removeAllQueryItems(self, this: &mut QUrlQuery) -> i32;
}

// proto: void QUrlQuery::removeAllQueryItems(const QString & key);
impl<'a> /*trait*/ QUrlQuery_removeAllQueryItems for (&'a  QString) {
  fn removeAllQueryItems(self, this: &mut QUrlQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery19removeAllQueryItemsERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QUrlQuery19removeAllQueryItemsERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn isEmpty<T: QUrlQuery_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QUrlQuery_isEmpty {
  fn isEmpty(self, this: &mut QUrlQuery) -> i32;
}

// proto: bool QUrlQuery::isEmpty();
impl<'a> /*trait*/ QUrlQuery_isEmpty for () {
  fn isEmpty(self, this: &mut QUrlQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery7isEmptyEv()};
    unsafe {_ZNK9QUrlQuery7isEmptyEv()};
    return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn removeQueryItem<T: QUrlQuery_removeQueryItem>(&mut self, value: T) -> i32 {
    value.removeQueryItem(self);
    return 1;
  }
}

pub trait QUrlQuery_removeQueryItem {
  fn removeQueryItem(self, this: &mut QUrlQuery) -> i32;
}

// proto: void QUrlQuery::removeQueryItem(const QString & key);
impl<'a> /*trait*/ QUrlQuery_removeQueryItem for (&'a  QString) {
  fn removeQueryItem(self, this: &mut QUrlQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery15removeQueryItemERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QUrlQuery15removeQueryItemERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn defaultQueryPairDelimiter<T: QUrlQuery_defaultQueryPairDelimiter>(&mut self, value: T) -> i32 {
    value.defaultQueryPairDelimiter(self);
    return 1;
  }
}

pub trait QUrlQuery_defaultQueryPairDelimiter {
  fn defaultQueryPairDelimiter(self, this: &mut QUrlQuery) -> i32;
}

// proto: QChar QUrlQuery::defaultQueryPairDelimiter();
impl<'a> /*trait*/ QUrlQuery_defaultQueryPairDelimiter for () {
  fn defaultQueryPairDelimiter(self, this: &mut QUrlQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery25defaultQueryPairDelimiterEv()};
    unsafe {_ZN9QUrlQuery25defaultQueryPairDelimiterEv()};
    return 1;
  }
}

// proto: void QUrlQuery::NewQUrlQuery(const QUrl & url);
impl<'a> /*trait*/ QUrlQuery_NewQUrlQuery for (&'a  QUrl) {
  fn NewQUrlQuery(self) -> QUrlQuery {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQueryC1ERK4QUrl()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QUrlQueryC1ERK4QUrl(qthis, arg0)};
    let rsthis = QUrlQuery{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn addQueryItem<T: QUrlQuery_addQueryItem>(&mut self, value: T) -> i32 {
    value.addQueryItem(self);
    return 1;
  }
}

pub trait QUrlQuery_addQueryItem {
  fn addQueryItem(self, this: &mut QUrlQuery) -> i32;
}

// proto: void QUrlQuery::addQueryItem(const QString & key, const QString & value);
impl<'a> /*trait*/ QUrlQuery_addQueryItem for (&'a  QString, &'a  QString) {
  fn addQueryItem(self, this: &mut QUrlQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQuery12addQueryItemERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN9QUrlQuery12addQueryItemERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

// proto: void QUrlQuery::NewQUrlQuery(const QUrlQuery & other);
impl<'a> /*trait*/ QUrlQuery_NewQUrlQuery for (&'a  QUrlQuery) {
  fn NewQUrlQuery(self) -> QUrlQuery {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUrlQueryC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QUrlQueryC1ERKS_(qthis, arg0)};
    let rsthis = QUrlQuery{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUrlQuery {
  pub fn hasQueryItem<T: QUrlQuery_hasQueryItem>(&mut self, value: T) -> i32 {
    value.hasQueryItem(self);
    return 1;
  }
}

pub trait QUrlQuery_hasQueryItem {
  fn hasQueryItem(self, this: &mut QUrlQuery) -> i32;
}

// proto: bool QUrlQuery::hasQueryItem(const QString & key);
impl<'a> /*trait*/ QUrlQuery_hasQueryItem for (&'a  QString) {
  fn hasQueryItem(self, this: &mut QUrlQuery) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUrlQuery12hasQueryItemERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QUrlQuery12hasQueryItemERK7QString(arg0)};
    return 1;
  }
}

