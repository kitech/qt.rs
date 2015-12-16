// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QMimeType::FreeQMimeType();
  fn _ZN9QMimeTypeD0Ev(qthis: *mut c_void) ;
  // proto:  QString QMimeType::comment();
  fn _ZNK9QMimeType7commentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QStringList QMimeType::aliases();
  fn _ZNK9QMimeType7aliasesEv(qthis: *mut c_void) ;
  // proto:  QString QMimeType::filterString();
  fn _ZNK9QMimeType12filterStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QStringList QMimeType::parentMimeTypes();
  fn _ZNK9QMimeType15parentMimeTypesEv(qthis: *mut c_void) ;
  // proto:  void QMimeType::NewQMimeType(const QMimeType & other);
  fn _ZN9QMimeTypeC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QMimeType::inherits(const QString & mimeTypeName);
  fn _ZNK9QMimeType8inheritsERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QMimeType::isDefault();
  fn _ZNK9QMimeType9isDefaultEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QMimeType::isValid();
  fn _ZNK9QMimeType7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QMimeType::NewQMimeType();
  fn _ZN9QMimeTypeC1Ev(qthis: *mut c_void) ;
  // proto:  void QMimeType::swap(QMimeType & other);
  fn _ZN9QMimeType4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringList QMimeType::suffixes();
  fn _ZNK9QMimeType8suffixesEv(qthis: *mut c_void) ;
  // proto:  QString QMimeType::genericIconName();
  fn _ZNK9QMimeType15genericIconNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QMimeType::iconName();
  fn _ZNK9QMimeType8iconNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QStringList QMimeType::allAncestors();
  fn _ZNK9QMimeType12allAncestorsEv(qthis: *mut c_void) ;
  // proto:  QStringList QMimeType::globPatterns();
  fn _ZNK9QMimeType12globPatternsEv(qthis: *mut c_void) ;
  // proto:  QString QMimeType::name();
  fn _ZNK9QMimeType4nameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QMimeType::preferredSuffix();
  fn _ZNK9QMimeType15preferredSuffixEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QMimeType)=1
pub struct QMimeType {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMimeType {
  pub fn FreeQMimeType<T: QMimeType_FreeQMimeType>(&mut self, value: T)  {
     value.FreeQMimeType(self);
    // return 1;
  }
}

pub trait QMimeType_FreeQMimeType {
  fn FreeQMimeType(self, rsthis: &mut QMimeType) ;
}

// proto:  void QMimeType::FreeQMimeType();
impl<'a> /*trait*/ QMimeType_FreeQMimeType for () {
  fn FreeQMimeType(self, rsthis: &mut QMimeType)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeTypeD0Ev()};
     unsafe {_ZN9QMimeTypeD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn comment<T: QMimeType_comment>(&mut self, value: T) -> QString {
    return value.comment(self);
    // return 1;
  }
}

pub trait QMimeType_comment {
  fn comment(self, rsthis: &mut QMimeType) -> QString;
}

// proto:  QString QMimeType::comment();
impl<'a> /*trait*/ QMimeType_comment for () {
  fn comment(self, rsthis: &mut QMimeType) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType7commentEv()};
    let mut ret = unsafe {_ZNK9QMimeType7commentEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn aliases<T: QMimeType_aliases>(&mut self, value: T)  {
     value.aliases(self);
    // return 1;
  }
}

pub trait QMimeType_aliases {
  fn aliases(self, rsthis: &mut QMimeType) ;
}

// proto:  QStringList QMimeType::aliases();
impl<'a> /*trait*/ QMimeType_aliases for () {
  fn aliases(self, rsthis: &mut QMimeType)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType7aliasesEv()};
     unsafe {_ZNK9QMimeType7aliasesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn filterString<T: QMimeType_filterString>(&mut self, value: T) -> QString {
    return value.filterString(self);
    // return 1;
  }
}

pub trait QMimeType_filterString {
  fn filterString(self, rsthis: &mut QMimeType) -> QString;
}

// proto:  QString QMimeType::filterString();
impl<'a> /*trait*/ QMimeType_filterString for () {
  fn filterString(self, rsthis: &mut QMimeType) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType12filterStringEv()};
    let mut ret = unsafe {_ZNK9QMimeType12filterStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn parentMimeTypes<T: QMimeType_parentMimeTypes>(&mut self, value: T)  {
     value.parentMimeTypes(self);
    // return 1;
  }
}

pub trait QMimeType_parentMimeTypes {
  fn parentMimeTypes(self, rsthis: &mut QMimeType) ;
}

// proto:  QStringList QMimeType::parentMimeTypes();
impl<'a> /*trait*/ QMimeType_parentMimeTypes for () {
  fn parentMimeTypes(self, rsthis: &mut QMimeType)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType15parentMimeTypesEv()};
     unsafe {_ZNK9QMimeType15parentMimeTypesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn NewQMimeType<T: QMimeType_NewQMimeType>(value: T) -> QMimeType {
    let rsthis = value.NewQMimeType();
    return rsthis;
    // return 1;
  }
}

pub trait QMimeType_NewQMimeType {
  fn NewQMimeType(self) -> QMimeType;
}

// proto: void QMimeType::NewQMimeType(const QMimeType & other);
impl<'a> /*trait*/ QMimeType_NewQMimeType for (&'a  QMimeType) {
  fn NewQMimeType(self) -> QMimeType {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeTypeC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QMimeTypeC1ERKS_(qthis, arg0)};
    let rsthis = QMimeType{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn inherits<T: QMimeType_inherits>(&mut self, value: T) -> i8 {
    return value.inherits(self);
    // return 1;
  }
}

pub trait QMimeType_inherits {
  fn inherits(self, rsthis: &mut QMimeType) -> i8;
}

// proto:  bool QMimeType::inherits(const QString & mimeTypeName);
impl<'a> /*trait*/ QMimeType_inherits for (&'a  QString) {
  fn inherits(self, rsthis: &mut QMimeType) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType8inheritsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QMimeType8inheritsERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn isDefault<T: QMimeType_isDefault>(&mut self, value: T) -> i8 {
    return value.isDefault(self);
    // return 1;
  }
}

pub trait QMimeType_isDefault {
  fn isDefault(self, rsthis: &mut QMimeType) -> i8;
}

// proto:  bool QMimeType::isDefault();
impl<'a> /*trait*/ QMimeType_isDefault for () {
  fn isDefault(self, rsthis: &mut QMimeType) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType9isDefaultEv()};
    let mut ret = unsafe {_ZNK9QMimeType9isDefaultEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn isValid<T: QMimeType_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QMimeType_isValid {
  fn isValid(self, rsthis: &mut QMimeType) -> i8;
}

// proto:  bool QMimeType::isValid();
impl<'a> /*trait*/ QMimeType_isValid for () {
  fn isValid(self, rsthis: &mut QMimeType) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType7isValidEv()};
    let mut ret = unsafe {_ZNK9QMimeType7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QMimeType::NewQMimeType();
impl<'a> /*trait*/ QMimeType_NewQMimeType for () {
  fn NewQMimeType(self) -> QMimeType {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeTypeC1Ev()};
    unsafe {_ZN9QMimeTypeC1Ev(qthis)};
    let rsthis = QMimeType{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn swap<T: QMimeType_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QMimeType_swap {
  fn swap(self, rsthis: &mut QMimeType) ;
}

// proto:  void QMimeType::swap(QMimeType & other);
impl<'a> /*trait*/ QMimeType_swap for (&'a mut QMimeType) {
  fn swap(self, rsthis: &mut QMimeType)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeType4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QMimeType4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn suffixes<T: QMimeType_suffixes>(&mut self, value: T)  {
     value.suffixes(self);
    // return 1;
  }
}

pub trait QMimeType_suffixes {
  fn suffixes(self, rsthis: &mut QMimeType) ;
}

// proto:  QStringList QMimeType::suffixes();
impl<'a> /*trait*/ QMimeType_suffixes for () {
  fn suffixes(self, rsthis: &mut QMimeType)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType8suffixesEv()};
     unsafe {_ZNK9QMimeType8suffixesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn genericIconName<T: QMimeType_genericIconName>(&mut self, value: T) -> QString {
    return value.genericIconName(self);
    // return 1;
  }
}

pub trait QMimeType_genericIconName {
  fn genericIconName(self, rsthis: &mut QMimeType) -> QString;
}

// proto:  QString QMimeType::genericIconName();
impl<'a> /*trait*/ QMimeType_genericIconName for () {
  fn genericIconName(self, rsthis: &mut QMimeType) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType15genericIconNameEv()};
    let mut ret = unsafe {_ZNK9QMimeType15genericIconNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn iconName<T: QMimeType_iconName>(&mut self, value: T) -> QString {
    return value.iconName(self);
    // return 1;
  }
}

pub trait QMimeType_iconName {
  fn iconName(self, rsthis: &mut QMimeType) -> QString;
}

// proto:  QString QMimeType::iconName();
impl<'a> /*trait*/ QMimeType_iconName for () {
  fn iconName(self, rsthis: &mut QMimeType) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType8iconNameEv()};
    let mut ret = unsafe {_ZNK9QMimeType8iconNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn allAncestors<T: QMimeType_allAncestors>(&mut self, value: T)  {
     value.allAncestors(self);
    // return 1;
  }
}

pub trait QMimeType_allAncestors {
  fn allAncestors(self, rsthis: &mut QMimeType) ;
}

// proto:  QStringList QMimeType::allAncestors();
impl<'a> /*trait*/ QMimeType_allAncestors for () {
  fn allAncestors(self, rsthis: &mut QMimeType)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType12allAncestorsEv()};
     unsafe {_ZNK9QMimeType12allAncestorsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn globPatterns<T: QMimeType_globPatterns>(&mut self, value: T)  {
     value.globPatterns(self);
    // return 1;
  }
}

pub trait QMimeType_globPatterns {
  fn globPatterns(self, rsthis: &mut QMimeType) ;
}

// proto:  QStringList QMimeType::globPatterns();
impl<'a> /*trait*/ QMimeType_globPatterns for () {
  fn globPatterns(self, rsthis: &mut QMimeType)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType12globPatternsEv()};
     unsafe {_ZNK9QMimeType12globPatternsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn name<T: QMimeType_name>(&mut self, value: T) -> QString {
    return value.name(self);
    // return 1;
  }
}

pub trait QMimeType_name {
  fn name(self, rsthis: &mut QMimeType) -> QString;
}

// proto:  QString QMimeType::name();
impl<'a> /*trait*/ QMimeType_name for () {
  fn name(self, rsthis: &mut QMimeType) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType4nameEv()};
    let mut ret = unsafe {_ZNK9QMimeType4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn preferredSuffix<T: QMimeType_preferredSuffix>(&mut self, value: T) -> QString {
    return value.preferredSuffix(self);
    // return 1;
  }
}

pub trait QMimeType_preferredSuffix {
  fn preferredSuffix(self, rsthis: &mut QMimeType) -> QString;
}

// proto:  QString QMimeType::preferredSuffix();
impl<'a> /*trait*/ QMimeType_preferredSuffix for () {
  fn preferredSuffix(self, rsthis: &mut QMimeType) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType15preferredSuffixEv()};
    let mut ret = unsafe {_ZNK9QMimeType15preferredSuffixEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

