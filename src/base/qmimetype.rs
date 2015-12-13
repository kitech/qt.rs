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
  fn _ZN9QMimeTypeD0Ev() -> i32;
  fn _ZNK9QMimeType7commentEv() -> i32;
  fn _ZNK9QMimeType7aliasesEv() -> i32;
  fn _ZNK9QMimeType12filterStringEv() -> i32;
  fn _ZNK9QMimeType15parentMimeTypesEv() -> i32;
  fn _ZN9QMimeTypeC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK9QMimeType8inheritsERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK9QMimeType9isDefaultEv() -> i32;
  fn _ZNK9QMimeType7isValidEv() -> i32;
  fn _ZN9QMimeTypeC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN9QMimeType4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZNK9QMimeType8suffixesEv() -> i32;
  fn _ZNK9QMimeType15genericIconNameEv() -> i32;
  fn _ZNK9QMimeType8iconNameEv() -> i32;
  fn _ZNK9QMimeType12allAncestorsEv() -> i32;
  fn _ZNK9QMimeType12globPatternsEv() -> i32;
  fn _ZNK9QMimeType4nameEv() -> i32;
  fn _ZNK9QMimeType15preferredSuffixEv() -> i32;
}

// body block begin
// class sizeof(QMimeType)=1
pub struct QMimeType {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMimeType {
  pub fn FreeQMimeType<T: QMimeType_FreeQMimeType>(&mut self, value: T) -> i32 {
    value.FreeQMimeType(self);
    return 1;
  }
}

pub trait QMimeType_FreeQMimeType {
  fn FreeQMimeType(self, this: &mut QMimeType) -> i32;
}

// proto: void QMimeType::FreeQMimeType();
impl<'a> /*trait*/ QMimeType_FreeQMimeType for () {
  fn FreeQMimeType(self, this: &mut QMimeType) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeTypeD0Ev()};
    unsafe {_ZN9QMimeTypeD0Ev()};
    return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn comment<T: QMimeType_comment>(&mut self, value: T) -> i32 {
    value.comment(self);
    return 1;
  }
}

pub trait QMimeType_comment {
  fn comment(self, this: &mut QMimeType) -> i32;
}

// proto: QString QMimeType::comment();
impl<'a> /*trait*/ QMimeType_comment for () {
  fn comment(self, this: &mut QMimeType) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType7commentEv()};
    unsafe {_ZNK9QMimeType7commentEv()};
    return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn aliases<T: QMimeType_aliases>(&mut self, value: T) -> i32 {
    value.aliases(self);
    return 1;
  }
}

pub trait QMimeType_aliases {
  fn aliases(self, this: &mut QMimeType) -> i32;
}

// proto: QStringList QMimeType::aliases();
impl<'a> /*trait*/ QMimeType_aliases for () {
  fn aliases(self, this: &mut QMimeType) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType7aliasesEv()};
    unsafe {_ZNK9QMimeType7aliasesEv()};
    return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn filterString<T: QMimeType_filterString>(&mut self, value: T) -> i32 {
    value.filterString(self);
    return 1;
  }
}

pub trait QMimeType_filterString {
  fn filterString(self, this: &mut QMimeType) -> i32;
}

// proto: QString QMimeType::filterString();
impl<'a> /*trait*/ QMimeType_filterString for () {
  fn filterString(self, this: &mut QMimeType) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType12filterStringEv()};
    unsafe {_ZNK9QMimeType12filterStringEv()};
    return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn parentMimeTypes<T: QMimeType_parentMimeTypes>(&mut self, value: T) -> i32 {
    value.parentMimeTypes(self);
    return 1;
  }
}

pub trait QMimeType_parentMimeTypes {
  fn parentMimeTypes(self, this: &mut QMimeType) -> i32;
}

// proto: QStringList QMimeType::parentMimeTypes();
impl<'a> /*trait*/ QMimeType_parentMimeTypes for () {
  fn parentMimeTypes(self, this: &mut QMimeType) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType15parentMimeTypesEv()};
    unsafe {_ZNK9QMimeType15parentMimeTypesEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QMimeTypeC1ERKS_(qthis, arg0)};
    let rsthis = QMimeType{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn inherits<T: QMimeType_inherits>(&mut self, value: T) -> i32 {
    value.inherits(self);
    return 1;
  }
}

pub trait QMimeType_inherits {
  fn inherits(self, this: &mut QMimeType) -> i32;
}

// proto: bool QMimeType::inherits(const QString & mimeTypeName);
impl<'a> /*trait*/ QMimeType_inherits for (&'a  QString) {
  fn inherits(self, this: &mut QMimeType) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType8inheritsERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK9QMimeType8inheritsERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn isDefault<T: QMimeType_isDefault>(&mut self, value: T) -> i32 {
    value.isDefault(self);
    return 1;
  }
}

pub trait QMimeType_isDefault {
  fn isDefault(self, this: &mut QMimeType) -> i32;
}

// proto: bool QMimeType::isDefault();
impl<'a> /*trait*/ QMimeType_isDefault for () {
  fn isDefault(self, this: &mut QMimeType) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType9isDefaultEv()};
    unsafe {_ZNK9QMimeType9isDefaultEv()};
    return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn isValid<T: QMimeType_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QMimeType_isValid {
  fn isValid(self, this: &mut QMimeType) -> i32;
}

// proto: bool QMimeType::isValid();
impl<'a> /*trait*/ QMimeType_isValid for () {
  fn isValid(self, this: &mut QMimeType) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType7isValidEv()};
    unsafe {_ZNK9QMimeType7isValidEv()};
    return 1;
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
  pub fn swap<T: QMimeType_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QMimeType_swap {
  fn swap(self, this: &mut QMimeType) -> i32;
}

// proto: void QMimeType::swap(QMimeType & other);
impl<'a> /*trait*/ QMimeType_swap for (&'a mut QMimeType) {
  fn swap(self, this: &mut QMimeType) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QMimeType4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QMimeType4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn suffixes<T: QMimeType_suffixes>(&mut self, value: T) -> i32 {
    value.suffixes(self);
    return 1;
  }
}

pub trait QMimeType_suffixes {
  fn suffixes(self, this: &mut QMimeType) -> i32;
}

// proto: QStringList QMimeType::suffixes();
impl<'a> /*trait*/ QMimeType_suffixes for () {
  fn suffixes(self, this: &mut QMimeType) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType8suffixesEv()};
    unsafe {_ZNK9QMimeType8suffixesEv()};
    return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn genericIconName<T: QMimeType_genericIconName>(&mut self, value: T) -> i32 {
    value.genericIconName(self);
    return 1;
  }
}

pub trait QMimeType_genericIconName {
  fn genericIconName(self, this: &mut QMimeType) -> i32;
}

// proto: QString QMimeType::genericIconName();
impl<'a> /*trait*/ QMimeType_genericIconName for () {
  fn genericIconName(self, this: &mut QMimeType) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType15genericIconNameEv()};
    unsafe {_ZNK9QMimeType15genericIconNameEv()};
    return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn iconName<T: QMimeType_iconName>(&mut self, value: T) -> i32 {
    value.iconName(self);
    return 1;
  }
}

pub trait QMimeType_iconName {
  fn iconName(self, this: &mut QMimeType) -> i32;
}

// proto: QString QMimeType::iconName();
impl<'a> /*trait*/ QMimeType_iconName for () {
  fn iconName(self, this: &mut QMimeType) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType8iconNameEv()};
    unsafe {_ZNK9QMimeType8iconNameEv()};
    return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn allAncestors<T: QMimeType_allAncestors>(&mut self, value: T) -> i32 {
    value.allAncestors(self);
    return 1;
  }
}

pub trait QMimeType_allAncestors {
  fn allAncestors(self, this: &mut QMimeType) -> i32;
}

// proto: QStringList QMimeType::allAncestors();
impl<'a> /*trait*/ QMimeType_allAncestors for () {
  fn allAncestors(self, this: &mut QMimeType) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType12allAncestorsEv()};
    unsafe {_ZNK9QMimeType12allAncestorsEv()};
    return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn globPatterns<T: QMimeType_globPatterns>(&mut self, value: T) -> i32 {
    value.globPatterns(self);
    return 1;
  }
}

pub trait QMimeType_globPatterns {
  fn globPatterns(self, this: &mut QMimeType) -> i32;
}

// proto: QStringList QMimeType::globPatterns();
impl<'a> /*trait*/ QMimeType_globPatterns for () {
  fn globPatterns(self, this: &mut QMimeType) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType12globPatternsEv()};
    unsafe {_ZNK9QMimeType12globPatternsEv()};
    return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn name<T: QMimeType_name>(&mut self, value: T) -> i32 {
    value.name(self);
    return 1;
  }
}

pub trait QMimeType_name {
  fn name(self, this: &mut QMimeType) -> i32;
}

// proto: QString QMimeType::name();
impl<'a> /*trait*/ QMimeType_name for () {
  fn name(self, this: &mut QMimeType) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType4nameEv()};
    unsafe {_ZNK9QMimeType4nameEv()};
    return 1;
  }
}

impl /*struct*/ QMimeType {
  pub fn preferredSuffix<T: QMimeType_preferredSuffix>(&mut self, value: T) -> i32 {
    value.preferredSuffix(self);
    return 1;
  }
}

pub trait QMimeType_preferredSuffix {
  fn preferredSuffix(self, this: &mut QMimeType) -> i32;
}

// proto: QString QMimeType::preferredSuffix();
impl<'a> /*trait*/ QMimeType_preferredSuffix for () {
  fn preferredSuffix(self, this: &mut QMimeType) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QMimeType15preferredSuffixEv()};
    unsafe {_ZNK9QMimeType15preferredSuffixEv()};
    return 1;
  }
}

