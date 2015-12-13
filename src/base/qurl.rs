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
  fn _ZNK4QUrl11isLocalFileEv() -> i32;
  fn _ZNK4QUrl4hostE6QFlagsINS_25ComponentFormattingOptionEE(arg0: c_int) -> i32;
  fn _ZNK4QUrl8userNameE6QFlagsINS_25ComponentFormattingOptionEE(arg0: c_int) -> i32;
  fn _ZNK4QUrl7isEmptyEv() -> i32;
  fn _ZN4QUrl8setQueryERK9QUrlQuery(arg0: *const c_void) -> i32;
  fn _ZNK4QUrl14topLevelDomainE6QFlagsINS_25ComponentFormattingOptionEE(arg0: c_int) -> i32;
  fn _ZNK4QUrl8userInfoE6QFlagsINS_25ComponentFormattingOptionEE(arg0: c_int) -> i32;
  fn _ZNK4QUrl3urlE12QUrlTwoFlagsINS_19UrlFormattingOptionENS_25ComponentFormattingOptionEE(arg0: c_int) -> i32;
  fn _ZNK4QUrl5queryE6QFlagsINS_25ComponentFormattingOptionEE(arg0: c_int) -> i32;
  fn _ZN4QUrl12idnWhitelistEv() -> i32;
  fn _ZN4QUrlD0Ev() -> i32;
  fn _ZN4QUrl9setSchemeERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK4QUrl10isParentOfERKS_(arg0: *const c_void) -> i32;
  fn _ZNK4QUrl11errorStringEv() -> i32;
  fn _ZNK4QUrl7matchesERKS_12QUrlTwoFlagsINS_19UrlFormattingOptionENS_25ComponentFormattingOptionEE(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZNK4QUrl4portEi(arg0: c_int) -> i32;
  fn _ZN4QUrl7setPortEi(arg0: c_int) -> i32;
  fn _ZN4QUrlC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN4QUrl7fromAceERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZNK4QUrl8resolvedERKS_(arg0: *const c_void) -> i32;
  fn _ZNK4QUrl4pathE6QFlagsINS_25ComponentFormattingOptionEE(arg0: c_int) -> i32;
  fn _ZNK4QUrl11toLocalFileEv() -> i32;
  fn _ZN4QUrl6detachEv() -> i32;
  fn _ZNK4QUrl9authorityE6QFlagsINS_25ComponentFormattingOptionEE(arg0: c_int) -> i32;
  fn _ZNK4QUrl11hasFragmentEv() -> i32;
  fn _ZNK4QUrl8adjustedE12QUrlTwoFlagsINS_19UrlFormattingOptionENS_25ComponentFormattingOptionEE(arg0: c_int) -> i32;
  fn _ZNK4QUrl15toDisplayStringE12QUrlTwoFlagsINS_19UrlFormattingOptionENS_25ComponentFormattingOptionEE(arg0: c_int) -> i32;
  fn _ZN4QUrl5toAceERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK4QUrl9toEncodedE12QUrlTwoFlagsINS_19UrlFormattingOptionENS_25ComponentFormattingOptionEE(arg0: c_int) -> i32;
  fn _ZNK4QUrl8hasQueryEv() -> i32;
  fn _ZN4QUrl13fromLocalFileERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK4QUrl7isValidEv() -> i32;
  fn _ZN4QUrlC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK4QUrl10isDetachedEv() -> i32;
  fn _ZNK4QUrl10isRelativeEv() -> i32;
  fn _ZN4QUrl13fromUserInputERK7QStringS2_6QFlagsINS_25UserInputResolutionOptionEE(arg0: *const c_void, arg1: *const c_void, arg2: c_int) -> i32;
  fn _ZNK4QUrl8toStringE12QUrlTwoFlagsINS_19UrlFormattingOptionENS_25ComponentFormattingOptionEE(arg0: c_int) -> i32;
  fn _ZNK4QUrl6schemeEv() -> i32;
  fn _ZNK4QUrl8fileNameE6QFlagsINS_25ComponentFormattingOptionEE(arg0: c_int) -> i32;
  fn _ZNK4QUrl8passwordE6QFlagsINS_25ComponentFormattingOptionEE(arg0: c_int) -> i32;
  fn _ZNK4QUrl8fragmentE6QFlagsINS_25ComponentFormattingOptionEE(arg0: c_int) -> i32;
  fn _ZN4QUrl17toPercentEncodingERK7QStringRK10QByteArrayS5_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  fn _ZN4QUrl15setIdnWhitelistERK11QStringList(arg0: *const c_void) -> i32;
  fn _ZN4QUrl4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZN4QUrl19fromPercentEncodingERK10QByteArray(arg0: *const c_void) -> i32;
  fn _ZN4QUrl13fromUserInputERK7QString(arg0: *const c_void) -> i32;
  fn _ZN4QUrl5clearEv() -> i32;
}

// body block begin
// class sizeof(QUrl)=8
pub struct QUrl {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QUrl {
  pub fn isLocalFile<T: QUrl_isLocalFile>(&mut self, value: T) -> i32 {
    value.isLocalFile(self);
    return 1;
  }
}

pub trait QUrl_isLocalFile {
  fn isLocalFile(self, this: &mut QUrl) -> i32;
}

// proto: bool QUrl::isLocalFile();
impl<'a> /*trait*/ QUrl_isLocalFile for () {
  fn isLocalFile(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl11isLocalFileEv()};
    unsafe {_ZNK4QUrl11isLocalFileEv()};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn host<T: QUrl_host>(&mut self, value: T) -> i32 {
    value.host(self);
    return 1;
  }
}

pub trait QUrl_host {
  fn host(self, this: &mut QUrl) -> i32;
}

// proto: QString QUrl::host(ComponentFormattingOptions );
impl<'a> /*trait*/ QUrl_host for (i32) {
  fn host(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl4hostE6QFlagsINS_25ComponentFormattingOptionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZNK4QUrl4hostE6QFlagsINS_25ComponentFormattingOptionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn userName<T: QUrl_userName>(&mut self, value: T) -> i32 {
    value.userName(self);
    return 1;
  }
}

pub trait QUrl_userName {
  fn userName(self, this: &mut QUrl) -> i32;
}

// proto: QString QUrl::userName(ComponentFormattingOptions options);
impl<'a> /*trait*/ QUrl_userName for (i32) {
  fn userName(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl8userNameE6QFlagsINS_25ComponentFormattingOptionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZNK4QUrl8userNameE6QFlagsINS_25ComponentFormattingOptionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn isEmpty<T: QUrl_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QUrl_isEmpty {
  fn isEmpty(self, this: &mut QUrl) -> i32;
}

// proto: bool QUrl::isEmpty();
impl<'a> /*trait*/ QUrl_isEmpty for () {
  fn isEmpty(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl7isEmptyEv()};
    unsafe {_ZNK4QUrl7isEmptyEv()};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn setQuery<T: QUrl_setQuery>(&mut self, value: T) -> i32 {
    value.setQuery(self);
    return 1;
  }
}

pub trait QUrl_setQuery {
  fn setQuery(self, this: &mut QUrl) -> i32;
}

// proto: void QUrl::setQuery(const QUrlQuery & query);
impl<'a> /*trait*/ QUrl_setQuery for (&'a  QUrlQuery) {
  fn setQuery(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl8setQueryERK9QUrlQuery()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QUrl8setQueryERK9QUrlQuery(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn topLevelDomain<T: QUrl_topLevelDomain>(&mut self, value: T) -> i32 {
    value.topLevelDomain(self);
    return 1;
  }
}

pub trait QUrl_topLevelDomain {
  fn topLevelDomain(self, this: &mut QUrl) -> i32;
}

// proto: QString QUrl::topLevelDomain(ComponentFormattingOptions options);
impl<'a> /*trait*/ QUrl_topLevelDomain for (i32) {
  fn topLevelDomain(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl14topLevelDomainE6QFlagsINS_25ComponentFormattingOptionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZNK4QUrl14topLevelDomainE6QFlagsINS_25ComponentFormattingOptionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn userInfo<T: QUrl_userInfo>(&mut self, value: T) -> i32 {
    value.userInfo(self);
    return 1;
  }
}

pub trait QUrl_userInfo {
  fn userInfo(self, this: &mut QUrl) -> i32;
}

// proto: QString QUrl::userInfo(ComponentFormattingOptions options);
impl<'a> /*trait*/ QUrl_userInfo for (i32) {
  fn userInfo(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl8userInfoE6QFlagsINS_25ComponentFormattingOptionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZNK4QUrl8userInfoE6QFlagsINS_25ComponentFormattingOptionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn url<T: QUrl_url>(&mut self, value: T) -> i32 {
    value.url(self);
    return 1;
  }
}

pub trait QUrl_url {
  fn url(self, this: &mut QUrl) -> i32;
}

// proto: QString QUrl::url(FormattingOptions options);
impl<'a> /*trait*/ QUrl_url for (i32) {
  fn url(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl3urlE12QUrlTwoFlagsINS_19UrlFormattingOptionENS_25ComponentFormattingOptionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZNK4QUrl3urlE12QUrlTwoFlagsINS_19UrlFormattingOptionENS_25ComponentFormattingOptionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn query<T: QUrl_query>(&mut self, value: T) -> i32 {
    value.query(self);
    return 1;
  }
}

pub trait QUrl_query {
  fn query(self, this: &mut QUrl) -> i32;
}

// proto: QString QUrl::query(ComponentFormattingOptions );
impl<'a> /*trait*/ QUrl_query for (i32) {
  fn query(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl5queryE6QFlagsINS_25ComponentFormattingOptionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZNK4QUrl5queryE6QFlagsINS_25ComponentFormattingOptionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn idnWhitelist<T: QUrl_idnWhitelist>(&mut self, value: T) -> i32 {
    value.idnWhitelist(self);
    return 1;
  }
}

pub trait QUrl_idnWhitelist {
  fn idnWhitelist(self, this: &mut QUrl) -> i32;
}

// proto: QStringList QUrl::idnWhitelist();
impl<'a> /*trait*/ QUrl_idnWhitelist for () {
  fn idnWhitelist(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl12idnWhitelistEv()};
    unsafe {_ZN4QUrl12idnWhitelistEv()};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn FreeQUrl<T: QUrl_FreeQUrl>(&mut self, value: T) -> i32 {
    value.FreeQUrl(self);
    return 1;
  }
}

pub trait QUrl_FreeQUrl {
  fn FreeQUrl(self, this: &mut QUrl) -> i32;
}

// proto: void QUrl::FreeQUrl();
impl<'a> /*trait*/ QUrl_FreeQUrl for () {
  fn FreeQUrl(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrlD0Ev()};
    unsafe {_ZN4QUrlD0Ev()};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn setScheme<T: QUrl_setScheme>(&mut self, value: T) -> i32 {
    value.setScheme(self);
    return 1;
  }
}

pub trait QUrl_setScheme {
  fn setScheme(self, this: &mut QUrl) -> i32;
}

// proto: void QUrl::setScheme(const QString & scheme);
impl<'a> /*trait*/ QUrl_setScheme for (&'a  QString) {
  fn setScheme(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl9setSchemeERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QUrl9setSchemeERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn isParentOf<T: QUrl_isParentOf>(&mut self, value: T) -> i32 {
    value.isParentOf(self);
    return 1;
  }
}

pub trait QUrl_isParentOf {
  fn isParentOf(self, this: &mut QUrl) -> i32;
}

// proto: bool QUrl::isParentOf(const QUrl & url);
impl<'a> /*trait*/ QUrl_isParentOf for (&'a  QUrl) {
  fn isParentOf(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl10isParentOfERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK4QUrl10isParentOfERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn errorString<T: QUrl_errorString>(&mut self, value: T) -> i32 {
    value.errorString(self);
    return 1;
  }
}

pub trait QUrl_errorString {
  fn errorString(self, this: &mut QUrl) -> i32;
}

// proto: QString QUrl::errorString();
impl<'a> /*trait*/ QUrl_errorString for () {
  fn errorString(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl11errorStringEv()};
    unsafe {_ZNK4QUrl11errorStringEv()};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn matches<T: QUrl_matches>(&mut self, value: T) -> i32 {
    value.matches(self);
    return 1;
  }
}

pub trait QUrl_matches {
  fn matches(self, this: &mut QUrl) -> i32;
}

// proto: bool QUrl::matches(const QUrl & url, FormattingOptions options);
impl<'a> /*trait*/ QUrl_matches for (&'a  QUrl, i32) {
  fn matches(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl7matchesERKS_12QUrlTwoFlagsINS_19UrlFormattingOptionENS_25ComponentFormattingOptionEE()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK4QUrl7matchesERKS_12QUrlTwoFlagsINS_19UrlFormattingOptionENS_25ComponentFormattingOptionEE(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn port<T: QUrl_port>(&mut self, value: T) -> i32 {
    value.port(self);
    return 1;
  }
}

pub trait QUrl_port {
  fn port(self, this: &mut QUrl) -> i32;
}

// proto: int QUrl::port(int defaultPort);
impl<'a> /*trait*/ QUrl_port for (i32) {
  fn port(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl4portEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK4QUrl4portEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn setPort<T: QUrl_setPort>(&mut self, value: T) -> i32 {
    value.setPort(self);
    return 1;
  }
}

pub trait QUrl_setPort {
  fn setPort(self, this: &mut QUrl) -> i32;
}

// proto: void QUrl::setPort(int port);
impl<'a> /*trait*/ QUrl_setPort for (i32) {
  fn setPort(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl7setPortEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN4QUrl7setPortEi(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QUrlC1ERKS_(qthis, arg0)};
    let rsthis = QUrl{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn fromAce<T: QUrl_fromAce>(&mut self, value: T) -> i32 {
    value.fromAce(self);
    return 1;
  }
}

pub trait QUrl_fromAce {
  fn fromAce(self, this: &mut QUrl) -> i32;
}

// proto: QString QUrl::fromAce(const QByteArray & );
impl<'a> /*trait*/ QUrl_fromAce for (&'a  QByteArray) {
  fn fromAce(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl7fromAceERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QUrl7fromAceERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn resolved<T: QUrl_resolved>(&mut self, value: T) -> i32 {
    value.resolved(self);
    return 1;
  }
}

pub trait QUrl_resolved {
  fn resolved(self, this: &mut QUrl) -> i32;
}

// proto: QUrl QUrl::resolved(const QUrl & relative);
impl<'a> /*trait*/ QUrl_resolved for (&'a  QUrl) {
  fn resolved(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl8resolvedERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK4QUrl8resolvedERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn path<T: QUrl_path>(&mut self, value: T) -> i32 {
    value.path(self);
    return 1;
  }
}

pub trait QUrl_path {
  fn path(self, this: &mut QUrl) -> i32;
}

// proto: QString QUrl::path(ComponentFormattingOptions options);
impl<'a> /*trait*/ QUrl_path for (i32) {
  fn path(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl4pathE6QFlagsINS_25ComponentFormattingOptionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZNK4QUrl4pathE6QFlagsINS_25ComponentFormattingOptionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn toLocalFile<T: QUrl_toLocalFile>(&mut self, value: T) -> i32 {
    value.toLocalFile(self);
    return 1;
  }
}

pub trait QUrl_toLocalFile {
  fn toLocalFile(self, this: &mut QUrl) -> i32;
}

// proto: QString QUrl::toLocalFile();
impl<'a> /*trait*/ QUrl_toLocalFile for () {
  fn toLocalFile(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl11toLocalFileEv()};
    unsafe {_ZNK4QUrl11toLocalFileEv()};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn detach<T: QUrl_detach>(&mut self, value: T) -> i32 {
    value.detach(self);
    return 1;
  }
}

pub trait QUrl_detach {
  fn detach(self, this: &mut QUrl) -> i32;
}

// proto: void QUrl::detach();
impl<'a> /*trait*/ QUrl_detach for () {
  fn detach(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl6detachEv()};
    unsafe {_ZN4QUrl6detachEv()};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn authority<T: QUrl_authority>(&mut self, value: T) -> i32 {
    value.authority(self);
    return 1;
  }
}

pub trait QUrl_authority {
  fn authority(self, this: &mut QUrl) -> i32;
}

// proto: QString QUrl::authority(ComponentFormattingOptions options);
impl<'a> /*trait*/ QUrl_authority for (i32) {
  fn authority(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl9authorityE6QFlagsINS_25ComponentFormattingOptionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZNK4QUrl9authorityE6QFlagsINS_25ComponentFormattingOptionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn hasFragment<T: QUrl_hasFragment>(&mut self, value: T) -> i32 {
    value.hasFragment(self);
    return 1;
  }
}

pub trait QUrl_hasFragment {
  fn hasFragment(self, this: &mut QUrl) -> i32;
}

// proto: bool QUrl::hasFragment();
impl<'a> /*trait*/ QUrl_hasFragment for () {
  fn hasFragment(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl11hasFragmentEv()};
    unsafe {_ZNK4QUrl11hasFragmentEv()};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn adjusted<T: QUrl_adjusted>(&mut self, value: T) -> i32 {
    value.adjusted(self);
    return 1;
  }
}

pub trait QUrl_adjusted {
  fn adjusted(self, this: &mut QUrl) -> i32;
}

// proto: QUrl QUrl::adjusted(FormattingOptions options);
impl<'a> /*trait*/ QUrl_adjusted for (i32) {
  fn adjusted(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl8adjustedE12QUrlTwoFlagsINS_19UrlFormattingOptionENS_25ComponentFormattingOptionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZNK4QUrl8adjustedE12QUrlTwoFlagsINS_19UrlFormattingOptionENS_25ComponentFormattingOptionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn toDisplayString<T: QUrl_toDisplayString>(&mut self, value: T) -> i32 {
    value.toDisplayString(self);
    return 1;
  }
}

pub trait QUrl_toDisplayString {
  fn toDisplayString(self, this: &mut QUrl) -> i32;
}

// proto: QString QUrl::toDisplayString(FormattingOptions options);
impl<'a> /*trait*/ QUrl_toDisplayString for (i32) {
  fn toDisplayString(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl15toDisplayStringE12QUrlTwoFlagsINS_19UrlFormattingOptionENS_25ComponentFormattingOptionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZNK4QUrl15toDisplayStringE12QUrlTwoFlagsINS_19UrlFormattingOptionENS_25ComponentFormattingOptionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn toAce<T: QUrl_toAce>(&mut self, value: T) -> i32 {
    value.toAce(self);
    return 1;
  }
}

pub trait QUrl_toAce {
  fn toAce(self, this: &mut QUrl) -> i32;
}

// proto: QByteArray QUrl::toAce(const QString & );
impl<'a> /*trait*/ QUrl_toAce for (&'a  QString) {
  fn toAce(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl5toAceERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QUrl5toAceERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn toEncoded<T: QUrl_toEncoded>(&mut self, value: T) -> i32 {
    value.toEncoded(self);
    return 1;
  }
}

pub trait QUrl_toEncoded {
  fn toEncoded(self, this: &mut QUrl) -> i32;
}

// proto: QByteArray QUrl::toEncoded(FormattingOptions options);
impl<'a> /*trait*/ QUrl_toEncoded for (i32) {
  fn toEncoded(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl9toEncodedE12QUrlTwoFlagsINS_19UrlFormattingOptionENS_25ComponentFormattingOptionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZNK4QUrl9toEncodedE12QUrlTwoFlagsINS_19UrlFormattingOptionENS_25ComponentFormattingOptionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn hasQuery<T: QUrl_hasQuery>(&mut self, value: T) -> i32 {
    value.hasQuery(self);
    return 1;
  }
}

pub trait QUrl_hasQuery {
  fn hasQuery(self, this: &mut QUrl) -> i32;
}

// proto: bool QUrl::hasQuery();
impl<'a> /*trait*/ QUrl_hasQuery for () {
  fn hasQuery(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl8hasQueryEv()};
    unsafe {_ZNK4QUrl8hasQueryEv()};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn fromLocalFile<T: QUrl_fromLocalFile>(&mut self, value: T) -> i32 {
    value.fromLocalFile(self);
    return 1;
  }
}

pub trait QUrl_fromLocalFile {
  fn fromLocalFile(self, this: &mut QUrl) -> i32;
}

// proto: QUrl QUrl::fromLocalFile(const QString & localfile);
impl<'a> /*trait*/ QUrl_fromLocalFile for (&'a  QString) {
  fn fromLocalFile(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl13fromLocalFileERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QUrl13fromLocalFileERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn isValid<T: QUrl_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QUrl_isValid {
  fn isValid(self, this: &mut QUrl) -> i32;
}

// proto: bool QUrl::isValid();
impl<'a> /*trait*/ QUrl_isValid for () {
  fn isValid(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl7isValidEv()};
    unsafe {_ZNK4QUrl7isValidEv()};
    return 1;
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
  pub fn isDetached<T: QUrl_isDetached>(&mut self, value: T) -> i32 {
    value.isDetached(self);
    return 1;
  }
}

pub trait QUrl_isDetached {
  fn isDetached(self, this: &mut QUrl) -> i32;
}

// proto: bool QUrl::isDetached();
impl<'a> /*trait*/ QUrl_isDetached for () {
  fn isDetached(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl10isDetachedEv()};
    unsafe {_ZNK4QUrl10isDetachedEv()};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn isRelative<T: QUrl_isRelative>(&mut self, value: T) -> i32 {
    value.isRelative(self);
    return 1;
  }
}

pub trait QUrl_isRelative {
  fn isRelative(self, this: &mut QUrl) -> i32;
}

// proto: bool QUrl::isRelative();
impl<'a> /*trait*/ QUrl_isRelative for () {
  fn isRelative(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl10isRelativeEv()};
    unsafe {_ZNK4QUrl10isRelativeEv()};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn fromUserInput<T: QUrl_fromUserInput>(&mut self, value: T) -> i32 {
    value.fromUserInput(self);
    return 1;
  }
}

pub trait QUrl_fromUserInput {
  fn fromUserInput(self, this: &mut QUrl) -> i32;
}

// proto: QUrl QUrl::fromUserInput(const QString & userInput, const QString & workingDirectory, UserInputResolutionOptions options);
impl<'a> /*trait*/ QUrl_fromUserInput for (&'a  QString, &'a  QString, i32) {
  fn fromUserInput(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl13fromUserInputERK7QStringS2_6QFlagsINS_25UserInputResolutionOptionEE()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN4QUrl13fromUserInputERK7QStringS2_6QFlagsINS_25UserInputResolutionOptionEE(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn toString<T: QUrl_toString>(&mut self, value: T) -> i32 {
    value.toString(self);
    return 1;
  }
}

pub trait QUrl_toString {
  fn toString(self, this: &mut QUrl) -> i32;
}

// proto: QString QUrl::toString(FormattingOptions options);
impl<'a> /*trait*/ QUrl_toString for (i32) {
  fn toString(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl8toStringE12QUrlTwoFlagsINS_19UrlFormattingOptionENS_25ComponentFormattingOptionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZNK4QUrl8toStringE12QUrlTwoFlagsINS_19UrlFormattingOptionENS_25ComponentFormattingOptionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn scheme<T: QUrl_scheme>(&mut self, value: T) -> i32 {
    value.scheme(self);
    return 1;
  }
}

pub trait QUrl_scheme {
  fn scheme(self, this: &mut QUrl) -> i32;
}

// proto: QString QUrl::scheme();
impl<'a> /*trait*/ QUrl_scheme for () {
  fn scheme(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl6schemeEv()};
    unsafe {_ZNK4QUrl6schemeEv()};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn fileName<T: QUrl_fileName>(&mut self, value: T) -> i32 {
    value.fileName(self);
    return 1;
  }
}

pub trait QUrl_fileName {
  fn fileName(self, this: &mut QUrl) -> i32;
}

// proto: QString QUrl::fileName(ComponentFormattingOptions options);
impl<'a> /*trait*/ QUrl_fileName for (i32) {
  fn fileName(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl8fileNameE6QFlagsINS_25ComponentFormattingOptionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZNK4QUrl8fileNameE6QFlagsINS_25ComponentFormattingOptionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn password<T: QUrl_password>(&mut self, value: T) -> i32 {
    value.password(self);
    return 1;
  }
}

pub trait QUrl_password {
  fn password(self, this: &mut QUrl) -> i32;
}

// proto: QString QUrl::password(ComponentFormattingOptions );
impl<'a> /*trait*/ QUrl_password for (i32) {
  fn password(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl8passwordE6QFlagsINS_25ComponentFormattingOptionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZNK4QUrl8passwordE6QFlagsINS_25ComponentFormattingOptionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn fragment<T: QUrl_fragment>(&mut self, value: T) -> i32 {
    value.fragment(self);
    return 1;
  }
}

pub trait QUrl_fragment {
  fn fragment(self, this: &mut QUrl) -> i32;
}

// proto: QString QUrl::fragment(ComponentFormattingOptions options);
impl<'a> /*trait*/ QUrl_fragment for (i32) {
  fn fragment(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK4QUrl8fragmentE6QFlagsINS_25ComponentFormattingOptionEE()};
    let arg0 = self  as c_int;
    unsafe {_ZNK4QUrl8fragmentE6QFlagsINS_25ComponentFormattingOptionEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn toPercentEncoding<T: QUrl_toPercentEncoding>(&mut self, value: T) -> i32 {
    value.toPercentEncoding(self);
    return 1;
  }
}

pub trait QUrl_toPercentEncoding {
  fn toPercentEncoding(self, this: &mut QUrl) -> i32;
}

// proto: QByteArray QUrl::toPercentEncoding(const QString & , const QByteArray & exclude, const QByteArray & include);
impl<'a> /*trait*/ QUrl_toPercentEncoding for (&'a  QString, &'a  QByteArray, &'a  QByteArray) {
  fn toPercentEncoding(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl17toPercentEncodingERK7QStringRK10QByteArrayS5_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN4QUrl17toPercentEncodingERK7QStringRK10QByteArrayS5_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn setIdnWhitelist<T: QUrl_setIdnWhitelist>(&mut self, value: T) -> i32 {
    value.setIdnWhitelist(self);
    return 1;
  }
}

pub trait QUrl_setIdnWhitelist {
  fn setIdnWhitelist(self, this: &mut QUrl) -> i32;
}

// proto: void QUrl::setIdnWhitelist(const QStringList & );
impl<'a> /*trait*/ QUrl_setIdnWhitelist for (&'a  QStringList) {
  fn setIdnWhitelist(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl15setIdnWhitelistERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QUrl15setIdnWhitelistERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn swap<T: QUrl_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QUrl_swap {
  fn swap(self, this: &mut QUrl) -> i32;
}

// proto: void QUrl::swap(QUrl & other);
impl<'a> /*trait*/ QUrl_swap for (&'a mut QUrl) {
  fn swap(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN4QUrl4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn fromPercentEncoding<T: QUrl_fromPercentEncoding>(&mut self, value: T) -> i32 {
    value.fromPercentEncoding(self);
    return 1;
  }
}

pub trait QUrl_fromPercentEncoding {
  fn fromPercentEncoding(self, this: &mut QUrl) -> i32;
}

// proto: QString QUrl::fromPercentEncoding(const QByteArray & );
impl<'a> /*trait*/ QUrl_fromPercentEncoding for (&'a  QByteArray) {
  fn fromPercentEncoding(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl19fromPercentEncodingERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QUrl19fromPercentEncodingERK10QByteArray(arg0)};
    return 1;
  }
}

// proto: QUrl QUrl::fromUserInput(const QString & userInput);
impl<'a> /*trait*/ QUrl_fromUserInput for (&'a  QString) {
  fn fromUserInput(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl13fromUserInputERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN4QUrl13fromUserInputERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QUrl {
  pub fn clear<T: QUrl_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QUrl_clear {
  fn clear(self, this: &mut QUrl) -> i32;
}

// proto: void QUrl::clear();
impl<'a> /*trait*/ QUrl_clear for () {
  fn clear(self, this: &mut QUrl) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN4QUrl5clearEv()};
    unsafe {_ZN4QUrl5clearEv()};
    return 1;
  }
}

