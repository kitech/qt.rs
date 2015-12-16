// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qurlquery::QUrlQuery;
use super::qstring::QString;
use super::qbytearray::QByteArray;
use super::qstringlist::QStringList;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QUrl::isLocalFile();
  fn _ZNK4QUrl11isLocalFileEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QUrl::isEmpty();
  fn _ZNK4QUrl7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QUrl::setQuery(const QUrlQuery & query);
  fn _ZN4QUrl8setQueryERK9QUrlQuery(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QStringList QUrl::idnWhitelist();
  fn _ZN4QUrl12idnWhitelistEv() ;
  // proto:  void QUrl::FreeQUrl();
  fn _ZN4QUrlD0Ev(qthis: *mut c_void) ;
  // proto:  void QUrl::setScheme(const QString & scheme);
  fn _ZN4QUrl9setSchemeERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QUrl::isParentOf(const QUrl & url);
  fn _ZNK4QUrl10isParentOfERKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QString QUrl::errorString();
  fn _ZNK4QUrl11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QUrl::port(int defaultPort);
  fn _ZNK4QUrl4portEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QUrl::setPort(int port);
  fn _ZN4QUrl7setPortEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QUrl::NewQUrl(const QUrl & copy);
  fn _ZN4QUrlC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QString QUrl::fromAce(const QByteArray & );
  fn _ZN4QUrl7fromAceERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto:  QUrl QUrl::resolved(const QUrl & relative);
  fn _ZNK4QUrl8resolvedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QUrl::toLocalFile();
  fn _ZNK4QUrl11toLocalFileEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QUrl::detach();
  fn _ZN4QUrl6detachEv(qthis: *mut c_void) ;
  // proto:  bool QUrl::hasFragment();
  fn _ZNK4QUrl11hasFragmentEv(qthis: *mut c_void) -> int8_t;
  // proto: static QByteArray QUrl::toAce(const QString & );
  fn _ZN4QUrl5toAceERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QUrl::hasQuery();
  fn _ZNK4QUrl8hasQueryEv(qthis: *mut c_void) -> int8_t;
  // proto: static QUrl QUrl::fromLocalFile(const QString & localfile);
  fn _ZN4QUrl13fromLocalFileERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QUrl::isValid();
  fn _ZNK4QUrl7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QUrl::NewQUrl();
  fn _ZN4QUrlC1Ev(qthis: *mut c_void) ;
  // proto:  bool QUrl::isDetached();
  fn _ZNK4QUrl10isDetachedEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QUrl::isRelative();
  fn _ZNK4QUrl10isRelativeEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QUrl::scheme();
  fn _ZNK4QUrl6schemeEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QByteArray QUrl::toPercentEncoding(const QString & , const QByteArray & exclude, const QByteArray & include);
  fn _ZN4QUrl17toPercentEncodingERK7QStringRK10QByteArrayS5_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto: static void QUrl::setIdnWhitelist(const QStringList & );
  fn _ZN4QUrl15setIdnWhitelistERK11QStringList(arg0: *mut c_void) ;
  // proto:  void QUrl::swap(QUrl & other);
  fn _ZN4QUrl4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QString QUrl::fromPercentEncoding(const QByteArray & );
  fn _ZN4QUrl19fromPercentEncodingERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto: static QUrl QUrl::fromUserInput(const QString & userInput);
  fn _ZN4QUrl13fromUserInputERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QUrl::clear();
  fn _ZN4QUrl5clearEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QUrl)=8
pub struct QUrl {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QUrl {
  pub fn isLocalFile<T: QUrl_isLocalFile>(&mut self, value: T) -> i8 {
    return value.isLocalFile(self);
    // return 1;
  }
}

pub trait QUrl_isLocalFile {
  fn isLocalFile(self, rsthis: &mut QUrl) -> i8;
}

// proto:  bool QUrl::isLocalFile();
impl<'a> /*trait*/ QUrl_isLocalFile for () {
  fn isLocalFile(self, rsthis: &mut QUrl) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl11isLocalFileEv()};
    let mut ret = unsafe {_ZNK4QUrl11isLocalFileEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn isEmpty<T: QUrl_isEmpty>(&mut self, value: T) -> i8 {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QUrl_isEmpty {
  fn isEmpty(self, rsthis: &mut QUrl) -> i8;
}

// proto:  bool QUrl::isEmpty();
impl<'a> /*trait*/ QUrl_isEmpty for () {
  fn isEmpty(self, rsthis: &mut QUrl) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl7isEmptyEv()};
    let mut ret = unsafe {_ZNK4QUrl7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn setQuery<T: QUrl_setQuery>(&mut self, value: T)  {
     value.setQuery(self);
    // return 1;
  }
}

pub trait QUrl_setQuery {
  fn setQuery(self, rsthis: &mut QUrl) ;
}

// proto:  void QUrl::setQuery(const QUrlQuery & query);
impl<'a> /*trait*/ QUrl_setQuery for (&'a  QUrlQuery) {
  fn setQuery(self, rsthis: &mut QUrl)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl8setQueryERK9QUrlQuery()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QUrl8setQueryERK9QUrlQuery(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn idnWhitelist<T: QUrl_idnWhitelist>(&mut self, value: T)  {
     value.idnWhitelist(self);
    // return 1;
  }
}

pub trait QUrl_idnWhitelist {
  fn idnWhitelist(self, rsthis: &mut QUrl) ;
}

// proto: static QStringList QUrl::idnWhitelist();
impl<'a> /*trait*/ QUrl_idnWhitelist for () {
  fn idnWhitelist(self, rsthis: &mut QUrl)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl12idnWhitelistEv()};
     unsafe {_ZN4QUrl12idnWhitelistEv()};
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn FreeQUrl<T: QUrl_FreeQUrl>(&mut self, value: T)  {
     value.FreeQUrl(self);
    // return 1;
  }
}

pub trait QUrl_FreeQUrl {
  fn FreeQUrl(self, rsthis: &mut QUrl) ;
}

// proto:  void QUrl::FreeQUrl();
impl<'a> /*trait*/ QUrl_FreeQUrl for () {
  fn FreeQUrl(self, rsthis: &mut QUrl)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrlD0Ev()};
     unsafe {_ZN4QUrlD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn setScheme<T: QUrl_setScheme>(&mut self, value: T)  {
     value.setScheme(self);
    // return 1;
  }
}

pub trait QUrl_setScheme {
  fn setScheme(self, rsthis: &mut QUrl) ;
}

// proto:  void QUrl::setScheme(const QString & scheme);
impl<'a> /*trait*/ QUrl_setScheme for (&'a  QString) {
  fn setScheme(self, rsthis: &mut QUrl)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl9setSchemeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QUrl9setSchemeERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn isParentOf<T: QUrl_isParentOf>(&mut self, value: T) -> i8 {
    return value.isParentOf(self);
    // return 1;
  }
}

pub trait QUrl_isParentOf {
  fn isParentOf(self, rsthis: &mut QUrl) -> i8;
}

// proto:  bool QUrl::isParentOf(const QUrl & url);
impl<'a> /*trait*/ QUrl_isParentOf for (&'a  QUrl) {
  fn isParentOf(self, rsthis: &mut QUrl) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl10isParentOfERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QUrl10isParentOfERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn errorString<T: QUrl_errorString>(&mut self, value: T) -> QString {
    return value.errorString(self);
    // return 1;
  }
}

pub trait QUrl_errorString {
  fn errorString(self, rsthis: &mut QUrl) -> QString;
}

// proto:  QString QUrl::errorString();
impl<'a> /*trait*/ QUrl_errorString for () {
  fn errorString(self, rsthis: &mut QUrl) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl11errorStringEv()};
    let mut ret = unsafe {_ZNK4QUrl11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn port<T: QUrl_port>(&mut self, value: T) -> i32 {
    return value.port(self);
    // return 1;
  }
}

pub trait QUrl_port {
  fn port(self, rsthis: &mut QUrl) -> i32;
}

// proto:  int QUrl::port(int defaultPort);
impl<'a> /*trait*/ QUrl_port for (i32) {
  fn port(self, rsthis: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl4portEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK4QUrl4portEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn setPort<T: QUrl_setPort>(&mut self, value: T)  {
     value.setPort(self);
    // return 1;
  }
}

pub trait QUrl_setPort {
  fn setPort(self, rsthis: &mut QUrl) ;
}

// proto:  void QUrl::setPort(int port);
impl<'a> /*trait*/ QUrl_setPort for (i32) {
  fn setPort(self, rsthis: &mut QUrl)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl7setPortEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN4QUrl7setPortEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn NewQUrl<T: QUrl_NewQUrl>(value: T) -> QUrl {
    let rsthis = value.NewQUrl();
    return rsthis;
    // return 1;
  }
}

pub trait QUrl_NewQUrl {
  fn NewQUrl(self) -> QUrl;
}

// proto: void QUrl::NewQUrl(const QUrl & copy);
impl<'a> /*trait*/ QUrl_NewQUrl for (&'a  QUrl) {
  fn NewQUrl(self) -> QUrl {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrlC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN4QUrlC1ERKS_(qthis, arg0)};
    let rsthis = QUrl{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn fromAce<T: QUrl_fromAce>(&mut self, value: T) -> QString {
    return value.fromAce(self);
    // return 1;
  }
}

pub trait QUrl_fromAce {
  fn fromAce(self, rsthis: &mut QUrl) -> QString;
}

// proto: static QString QUrl::fromAce(const QByteArray & );
impl<'a> /*trait*/ QUrl_fromAce for (&'a  QByteArray) {
  fn fromAce(self, rsthis: &mut QUrl) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl7fromAceERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QUrl7fromAceERK10QByteArray(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn resolved<T: QUrl_resolved>(&mut self, value: T) -> QUrl {
    return value.resolved(self);
    // return 1;
  }
}

pub trait QUrl_resolved {
  fn resolved(self, rsthis: &mut QUrl) -> QUrl;
}

// proto:  QUrl QUrl::resolved(const QUrl & relative);
impl<'a> /*trait*/ QUrl_resolved for (&'a  QUrl) {
  fn resolved(self, rsthis: &mut QUrl) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl8resolvedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK4QUrl8resolvedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QUrl{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn toLocalFile<T: QUrl_toLocalFile>(&mut self, value: T) -> QString {
    return value.toLocalFile(self);
    // return 1;
  }
}

pub trait QUrl_toLocalFile {
  fn toLocalFile(self, rsthis: &mut QUrl) -> QString;
}

// proto:  QString QUrl::toLocalFile();
impl<'a> /*trait*/ QUrl_toLocalFile for () {
  fn toLocalFile(self, rsthis: &mut QUrl) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl11toLocalFileEv()};
    let mut ret = unsafe {_ZNK4QUrl11toLocalFileEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn detach<T: QUrl_detach>(&mut self, value: T)  {
     value.detach(self);
    // return 1;
  }
}

pub trait QUrl_detach {
  fn detach(self, rsthis: &mut QUrl) ;
}

// proto:  void QUrl::detach();
impl<'a> /*trait*/ QUrl_detach for () {
  fn detach(self, rsthis: &mut QUrl)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl6detachEv()};
     unsafe {_ZN4QUrl6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn hasFragment<T: QUrl_hasFragment>(&mut self, value: T) -> i8 {
    return value.hasFragment(self);
    // return 1;
  }
}

pub trait QUrl_hasFragment {
  fn hasFragment(self, rsthis: &mut QUrl) -> i8;
}

// proto:  bool QUrl::hasFragment();
impl<'a> /*trait*/ QUrl_hasFragment for () {
  fn hasFragment(self, rsthis: &mut QUrl) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl11hasFragmentEv()};
    let mut ret = unsafe {_ZNK4QUrl11hasFragmentEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn toAce<T: QUrl_toAce>(&mut self, value: T) -> QByteArray {
    return value.toAce(self);
    // return 1;
  }
}

pub trait QUrl_toAce {
  fn toAce(self, rsthis: &mut QUrl) -> QByteArray;
}

// proto: static QByteArray QUrl::toAce(const QString & );
impl<'a> /*trait*/ QUrl_toAce for (&'a  QString) {
  fn toAce(self, rsthis: &mut QUrl) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl5toAceERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QUrl5toAceERK7QString(arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn hasQuery<T: QUrl_hasQuery>(&mut self, value: T) -> i8 {
    return value.hasQuery(self);
    // return 1;
  }
}

pub trait QUrl_hasQuery {
  fn hasQuery(self, rsthis: &mut QUrl) -> i8;
}

// proto:  bool QUrl::hasQuery();
impl<'a> /*trait*/ QUrl_hasQuery for () {
  fn hasQuery(self, rsthis: &mut QUrl) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl8hasQueryEv()};
    let mut ret = unsafe {_ZNK4QUrl8hasQueryEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn fromLocalFile<T: QUrl_fromLocalFile>(&mut self, value: T) -> QUrl {
    return value.fromLocalFile(self);
    // return 1;
  }
}

pub trait QUrl_fromLocalFile {
  fn fromLocalFile(self, rsthis: &mut QUrl) -> QUrl;
}

// proto: static QUrl QUrl::fromLocalFile(const QString & localfile);
impl<'a> /*trait*/ QUrl_fromLocalFile for (&'a  QString) {
  fn fromLocalFile(self, rsthis: &mut QUrl) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl13fromLocalFileERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QUrl13fromLocalFileERK7QString(arg0)};
    let mut ret1 = QUrl{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn isValid<T: QUrl_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QUrl_isValid {
  fn isValid(self, rsthis: &mut QUrl) -> i8;
}

// proto:  bool QUrl::isValid();
impl<'a> /*trait*/ QUrl_isValid for () {
  fn isValid(self, rsthis: &mut QUrl) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl7isValidEv()};
    let mut ret = unsafe {_ZNK4QUrl7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QUrl::NewQUrl();
impl<'a> /*trait*/ QUrl_NewQUrl for () {
  fn NewQUrl(self) -> QUrl {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrlC1Ev()};
    unsafe {_ZN4QUrlC1Ev(qthis)};
    let rsthis = QUrl{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn isDetached<T: QUrl_isDetached>(&mut self, value: T) -> i8 {
    return value.isDetached(self);
    // return 1;
  }
}

pub trait QUrl_isDetached {
  fn isDetached(self, rsthis: &mut QUrl) -> i8;
}

// proto:  bool QUrl::isDetached();
impl<'a> /*trait*/ QUrl_isDetached for () {
  fn isDetached(self, rsthis: &mut QUrl) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl10isDetachedEv()};
    let mut ret = unsafe {_ZNK4QUrl10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn isRelative<T: QUrl_isRelative>(&mut self, value: T) -> i8 {
    return value.isRelative(self);
    // return 1;
  }
}

pub trait QUrl_isRelative {
  fn isRelative(self, rsthis: &mut QUrl) -> i8;
}

// proto:  bool QUrl::isRelative();
impl<'a> /*trait*/ QUrl_isRelative for () {
  fn isRelative(self, rsthis: &mut QUrl) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl10isRelativeEv()};
    let mut ret = unsafe {_ZNK4QUrl10isRelativeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn scheme<T: QUrl_scheme>(&mut self, value: T) -> QString {
    return value.scheme(self);
    // return 1;
  }
}

pub trait QUrl_scheme {
  fn scheme(self, rsthis: &mut QUrl) -> QString;
}

// proto:  QString QUrl::scheme();
impl<'a> /*trait*/ QUrl_scheme for () {
  fn scheme(self, rsthis: &mut QUrl) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl6schemeEv()};
    let mut ret = unsafe {_ZNK4QUrl6schemeEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn toPercentEncoding<T: QUrl_toPercentEncoding>(&mut self, value: T) -> QByteArray {
    return value.toPercentEncoding(self);
    // return 1;
  }
}

pub trait QUrl_toPercentEncoding {
  fn toPercentEncoding(self, rsthis: &mut QUrl) -> QByteArray;
}

// proto: static QByteArray QUrl::toPercentEncoding(const QString & , const QByteArray & exclude, const QByteArray & include);
impl<'a> /*trait*/ QUrl_toPercentEncoding for (&'a  QString, &'a  QByteArray, &'a  QByteArray) {
  fn toPercentEncoding(self, rsthis: &mut QUrl) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl17toPercentEncodingERK7QStringRK10QByteArrayS5_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QUrl17toPercentEncodingERK7QStringRK10QByteArrayS5_(arg0, arg1, arg2)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn setIdnWhitelist<T: QUrl_setIdnWhitelist>(&mut self, value: T)  {
     value.setIdnWhitelist(self);
    // return 1;
  }
}

pub trait QUrl_setIdnWhitelist {
  fn setIdnWhitelist(self, rsthis: &mut QUrl) ;
}

// proto: static void QUrl::setIdnWhitelist(const QStringList & );
impl<'a> /*trait*/ QUrl_setIdnWhitelist for (&'a  QStringList) {
  fn setIdnWhitelist(self, rsthis: &mut QUrl)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl15setIdnWhitelistERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QUrl15setIdnWhitelistERK11QStringList(arg0)};
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn swap<T: QUrl_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QUrl_swap {
  fn swap(self, rsthis: &mut QUrl) ;
}

// proto:  void QUrl::swap(QUrl & other);
impl<'a> /*trait*/ QUrl_swap for (&'a mut QUrl) {
  fn swap(self, rsthis: &mut QUrl)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN4QUrl4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn fromPercentEncoding<T: QUrl_fromPercentEncoding>(&mut self, value: T) -> QString {
    return value.fromPercentEncoding(self);
    // return 1;
  }
}

pub trait QUrl_fromPercentEncoding {
  fn fromPercentEncoding(self, rsthis: &mut QUrl) -> QString;
}

// proto: static QString QUrl::fromPercentEncoding(const QByteArray & );
impl<'a> /*trait*/ QUrl_fromPercentEncoding for (&'a  QByteArray) {
  fn fromPercentEncoding(self, rsthis: &mut QUrl) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl19fromPercentEncodingERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QUrl19fromPercentEncodingERK10QByteArray(arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn fromUserInput<T: QUrl_fromUserInput>(&mut self, value: T) -> QUrl {
    return value.fromUserInput(self);
    // return 1;
  }
}

pub trait QUrl_fromUserInput {
  fn fromUserInput(self, rsthis: &mut QUrl) -> QUrl;
}

// proto: static QUrl QUrl::fromUserInput(const QString & userInput);
impl<'a> /*trait*/ QUrl_fromUserInput for (&'a  QString) {
  fn fromUserInput(self, rsthis: &mut QUrl) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl13fromUserInputERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN4QUrl13fromUserInputERK7QString(arg0)};
    let mut ret1 = QUrl{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn clear<T: QUrl_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QUrl_clear {
  fn clear(self, rsthis: &mut QUrl) ;
}

// proto:  void QUrl::clear();
impl<'a> /*trait*/ QUrl_clear for () {
  fn clear(self, rsthis: &mut QUrl)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl5clearEv()};
     unsafe {_ZN4QUrl5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

