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
  pub fn FreeQMimeType<RetType, T: QMimeType_FreeQMimeType<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQMimeType(self);
    // return 1;
  }
}

pub trait QMimeType_FreeQMimeType<RetType> {
  fn FreeQMimeType(self, rsthis: &mut QMimeType) -> RetType;
}

// proto:  void QMimeType::FreeQMimeType();
impl<'a> /*trait*/ QMimeType_FreeQMimeType<()> for () {
  fn FreeQMimeType(self, rsthis: &mut QMimeType) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeTypeD0Ev()};
     unsafe {_ZN9QMimeTypeD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn comment<RetType, T: QMimeType_comment<RetType>>(&mut self, value: T) -> RetType {
    return value.comment(self);
    // return 1;
  }
}

pub trait QMimeType_comment<RetType> {
  fn comment(self, rsthis: &mut QMimeType) -> RetType;
}

// proto:  QString QMimeType::comment();
impl<'a> /*trait*/ QMimeType_comment<QString> for () {
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
  pub fn aliases<RetType, T: QMimeType_aliases<RetType>>(&mut self, value: T) -> RetType {
    return value.aliases(self);
    // return 1;
  }
}

pub trait QMimeType_aliases<RetType> {
  fn aliases(self, rsthis: &mut QMimeType) -> RetType;
}

// proto:  QStringList QMimeType::aliases();
impl<'a> /*trait*/ QMimeType_aliases<()> for () {
  fn aliases(self, rsthis: &mut QMimeType) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType7aliasesEv()};
     unsafe {_ZNK9QMimeType7aliasesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn filterString<RetType, T: QMimeType_filterString<RetType>>(&mut self, value: T) -> RetType {
    return value.filterString(self);
    // return 1;
  }
}

pub trait QMimeType_filterString<RetType> {
  fn filterString(self, rsthis: &mut QMimeType) -> RetType;
}

// proto:  QString QMimeType::filterString();
impl<'a> /*trait*/ QMimeType_filterString<QString> for () {
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
  pub fn parentMimeTypes<RetType, T: QMimeType_parentMimeTypes<RetType>>(&mut self, value: T) -> RetType {
    return value.parentMimeTypes(self);
    // return 1;
  }
}

pub trait QMimeType_parentMimeTypes<RetType> {
  fn parentMimeTypes(self, rsthis: &mut QMimeType) -> RetType;
}

// proto:  QStringList QMimeType::parentMimeTypes();
impl<'a> /*trait*/ QMimeType_parentMimeTypes<()> for () {
  fn parentMimeTypes(self, rsthis: &mut QMimeType) -> () {
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
  pub fn inherits<RetType, T: QMimeType_inherits<RetType>>(&mut self, value: T) -> RetType {
    return value.inherits(self);
    // return 1;
  }
}

pub trait QMimeType_inherits<RetType> {
  fn inherits(self, rsthis: &mut QMimeType) -> RetType;
}

// proto:  bool QMimeType::inherits(const QString & mimeTypeName);
impl<'a> /*trait*/ QMimeType_inherits<i8> for (&'a  QString) {
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
  pub fn isDefault<RetType, T: QMimeType_isDefault<RetType>>(&mut self, value: T) -> RetType {
    return value.isDefault(self);
    // return 1;
  }
}

pub trait QMimeType_isDefault<RetType> {
  fn isDefault(self, rsthis: &mut QMimeType) -> RetType;
}

// proto:  bool QMimeType::isDefault();
impl<'a> /*trait*/ QMimeType_isDefault<i8> for () {
  fn isDefault(self, rsthis: &mut QMimeType) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType9isDefaultEv()};
    let mut ret = unsafe {_ZNK9QMimeType9isDefaultEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn isValid<RetType, T: QMimeType_isValid<RetType>>(&mut self, value: T) -> RetType {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QMimeType_isValid<RetType> {
  fn isValid(self, rsthis: &mut QMimeType) -> RetType;
}

// proto:  bool QMimeType::isValid();
impl<'a> /*trait*/ QMimeType_isValid<i8> for () {
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
  pub fn swap<RetType, T: QMimeType_swap<RetType>>(&mut self, value: T) -> RetType {
    return value.swap(self);
    // return 1;
  }
}

pub trait QMimeType_swap<RetType> {
  fn swap(self, rsthis: &mut QMimeType) -> RetType;
}

// proto:  void QMimeType::swap(QMimeType & other);
impl<'a> /*trait*/ QMimeType_swap<()> for (&'a mut QMimeType) {
  fn swap(self, rsthis: &mut QMimeType) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeType4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QMimeType4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn suffixes<RetType, T: QMimeType_suffixes<RetType>>(&mut self, value: T) -> RetType {
    return value.suffixes(self);
    // return 1;
  }
}

pub trait QMimeType_suffixes<RetType> {
  fn suffixes(self, rsthis: &mut QMimeType) -> RetType;
}

// proto:  QStringList QMimeType::suffixes();
impl<'a> /*trait*/ QMimeType_suffixes<()> for () {
  fn suffixes(self, rsthis: &mut QMimeType) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType8suffixesEv()};
     unsafe {_ZNK9QMimeType8suffixesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn genericIconName<RetType, T: QMimeType_genericIconName<RetType>>(&mut self, value: T) -> RetType {
    return value.genericIconName(self);
    // return 1;
  }
}

pub trait QMimeType_genericIconName<RetType> {
  fn genericIconName(self, rsthis: &mut QMimeType) -> RetType;
}

// proto:  QString QMimeType::genericIconName();
impl<'a> /*trait*/ QMimeType_genericIconName<QString> for () {
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
  pub fn iconName<RetType, T: QMimeType_iconName<RetType>>(&mut self, value: T) -> RetType {
    return value.iconName(self);
    // return 1;
  }
}

pub trait QMimeType_iconName<RetType> {
  fn iconName(self, rsthis: &mut QMimeType) -> RetType;
}

// proto:  QString QMimeType::iconName();
impl<'a> /*trait*/ QMimeType_iconName<QString> for () {
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
  pub fn allAncestors<RetType, T: QMimeType_allAncestors<RetType>>(&mut self, value: T) -> RetType {
    return value.allAncestors(self);
    // return 1;
  }
}

pub trait QMimeType_allAncestors<RetType> {
  fn allAncestors(self, rsthis: &mut QMimeType) -> RetType;
}

// proto:  QStringList QMimeType::allAncestors();
impl<'a> /*trait*/ QMimeType_allAncestors<()> for () {
  fn allAncestors(self, rsthis: &mut QMimeType) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType12allAncestorsEv()};
     unsafe {_ZNK9QMimeType12allAncestorsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn globPatterns<RetType, T: QMimeType_globPatterns<RetType>>(&mut self, value: T) -> RetType {
    return value.globPatterns(self);
    // return 1;
  }
}

pub trait QMimeType_globPatterns<RetType> {
  fn globPatterns(self, rsthis: &mut QMimeType) -> RetType;
}

// proto:  QStringList QMimeType::globPatterns();
impl<'a> /*trait*/ QMimeType_globPatterns<()> for () {
  fn globPatterns(self, rsthis: &mut QMimeType) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType12globPatternsEv()};
     unsafe {_ZNK9QMimeType12globPatternsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn name<RetType, T: QMimeType_name<RetType>>(&mut self, value: T) -> RetType {
    return value.name(self);
    // return 1;
  }
}

pub trait QMimeType_name<RetType> {
  fn name(self, rsthis: &mut QMimeType) -> RetType;
}

// proto:  QString QMimeType::name();
impl<'a> /*trait*/ QMimeType_name<QString> for () {
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
  pub fn preferredSuffix<RetType, T: QMimeType_preferredSuffix<RetType>>(&mut self, value: T) -> RetType {
    return value.preferredSuffix(self);
    // return 1;
  }
}

pub trait QMimeType_preferredSuffix<RetType> {
  fn preferredSuffix(self, rsthis: &mut QMimeType) -> RetType;
}

// proto:  QString QMimeType::preferredSuffix();
impl<'a> /*trait*/ QMimeType_preferredSuffix<QString> for () {
  fn preferredSuffix(self, rsthis: &mut QMimeType) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType15preferredSuffixEv()};
    let mut ret = unsafe {_ZNK9QMimeType15preferredSuffixEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

