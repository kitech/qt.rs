// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstringlist::QStringList;
use super::qstring::QString;
use super::qpixmap::QPixmap;
use super::qiconengine::QIconEngine;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QIcon::NewQIcon(const QIcon & other);
  fn _ZN5QIconC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QStringList QIcon::themeSearchPaths();
  fn _ZN5QIcon16themeSearchPathsEv() -> i32;
  // proto: void QIcon::detach();
  fn _ZN5QIcon6detachEv() -> i32;
  // proto: bool QIcon::isNull();
  fn _ZNK5QIcon6isNullEv() -> i32;
  // proto: void QIcon::setThemeSearchPaths(const QStringList & searchpath);
  fn _ZN5QIcon19setThemeSearchPathsERK11QStringList(arg0: *const c_void) -> i32;
  // proto: bool QIcon::hasThemeIcon(const QString & name);
  fn _ZN5QIcon12hasThemeIconERK7QString(arg0: *const c_void) -> i32;
  // proto: void QIcon::NewQIcon(const QPixmap & pixmap);
  fn _ZN5QIconC1ERK7QPixmap(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QIcon QIcon::fromTheme(const QString & name, const QIcon & fallback);
  fn _ZN5QIcon9fromThemeERK7QStringRKS_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QString QIcon::themeName();
  fn _ZN5QIcon9themeNameEv() -> i32;
  // proto: QString QIcon::name();
  fn _ZNK5QIcon4nameEv() -> i32;
  // proto: void QIcon::NewQIcon();
  fn _ZN5QIconC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QIcon::NewQIcon(QIconEngine * engine);
  fn _ZN5QIconC1EP11QIconEngine(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QIcon::FreeQIcon();
  fn _ZN5QIconD0Ev() -> i32;
  // proto: bool QIcon::isDetached();
  fn _ZNK5QIcon10isDetachedEv() -> i32;
  // proto: void QIcon::NewQIcon(const QString & fileName);
  fn _ZN5QIconC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: long long QIcon::cacheKey();
  fn _ZNK5QIcon8cacheKeyEv() -> i32;
  // proto: void QIcon::swap(QIcon & other);
  fn _ZN5QIcon4swapERS_(arg0: *mut c_void) -> i32;
  // proto: void QIcon::setThemeName(const QString & path);
  fn _ZN5QIcon12setThemeNameERK7QString(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QIcon)=8
pub struct QIcon {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QIcon {
  pub fn NewQIcon<T: QIcon_NewQIcon>(value: T) -> QIcon {
    let rsthis = value.NewQIcon();
    return rsthis;
    // return 1;
  }
}

pub trait QIcon_NewQIcon {
  fn NewQIcon(self) -> QIcon;
}

// proto: void QIcon::NewQIcon(const QIcon & other);
impl<'a> /*trait*/ QIcon_NewQIcon for (&'a  QIcon) {
  fn NewQIcon(self) -> QIcon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIconC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QIconC1ERKS_(qthis, arg0)};
    let rsthis = QIcon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QIcon {
  pub fn themeSearchPaths<T: QIcon_themeSearchPaths>(&mut self, value: T) -> i32 {
    value.themeSearchPaths(self);
    return 1;
  }
}

pub trait QIcon_themeSearchPaths {
  fn themeSearchPaths(self, this: &mut QIcon) -> i32;
}

// proto: QStringList QIcon::themeSearchPaths();
impl<'a> /*trait*/ QIcon_themeSearchPaths for () {
  fn themeSearchPaths(self, this: &mut QIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon16themeSearchPathsEv()};
    unsafe {_ZN5QIcon16themeSearchPathsEv()};
    return 1;
  }
}

impl /*struct*/ QIcon {
  pub fn detach<T: QIcon_detach>(&mut self, value: T) -> i32 {
    value.detach(self);
    return 1;
  }
}

pub trait QIcon_detach {
  fn detach(self, this: &mut QIcon) -> i32;
}

// proto: void QIcon::detach();
impl<'a> /*trait*/ QIcon_detach for () {
  fn detach(self, this: &mut QIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon6detachEv()};
    unsafe {_ZN5QIcon6detachEv()};
    return 1;
  }
}

impl /*struct*/ QIcon {
  pub fn isNull<T: QIcon_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QIcon_isNull {
  fn isNull(self, this: &mut QIcon) -> i32;
}

// proto: bool QIcon::isNull();
impl<'a> /*trait*/ QIcon_isNull for () {
  fn isNull(self, this: &mut QIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QIcon6isNullEv()};
    unsafe {_ZNK5QIcon6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QIcon {
  pub fn setThemeSearchPaths<T: QIcon_setThemeSearchPaths>(&mut self, value: T) -> i32 {
    value.setThemeSearchPaths(self);
    return 1;
  }
}

pub trait QIcon_setThemeSearchPaths {
  fn setThemeSearchPaths(self, this: &mut QIcon) -> i32;
}

// proto: void QIcon::setThemeSearchPaths(const QStringList & searchpath);
impl<'a> /*trait*/ QIcon_setThemeSearchPaths for (&'a  QStringList) {
  fn setThemeSearchPaths(self, this: &mut QIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon19setThemeSearchPathsERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QIcon19setThemeSearchPathsERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QIcon {
  pub fn hasThemeIcon<T: QIcon_hasThemeIcon>(&mut self, value: T) -> i32 {
    value.hasThemeIcon(self);
    return 1;
  }
}

pub trait QIcon_hasThemeIcon {
  fn hasThemeIcon(self, this: &mut QIcon) -> i32;
}

// proto: bool QIcon::hasThemeIcon(const QString & name);
impl<'a> /*trait*/ QIcon_hasThemeIcon for (&'a  QString) {
  fn hasThemeIcon(self, this: &mut QIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon12hasThemeIconERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QIcon12hasThemeIconERK7QString(arg0)};
    return 1;
  }
}

// proto: void QIcon::NewQIcon(const QPixmap & pixmap);
impl<'a> /*trait*/ QIcon_NewQIcon for (&'a  QPixmap) {
  fn NewQIcon(self) -> QIcon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIconC1ERK7QPixmap()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QIconC1ERK7QPixmap(qthis, arg0)};
    let rsthis = QIcon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QIcon {
  pub fn fromTheme<T: QIcon_fromTheme>(&mut self, value: T) -> i32 {
    value.fromTheme(self);
    return 1;
  }
}

pub trait QIcon_fromTheme {
  fn fromTheme(self, this: &mut QIcon) -> i32;
}

// proto: QIcon QIcon::fromTheme(const QString & name, const QIcon & fallback);
impl<'a> /*trait*/ QIcon_fromTheme for (&'a  QString, &'a  QIcon) {
  fn fromTheme(self, this: &mut QIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon9fromThemeERK7QStringRKS_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QIcon9fromThemeERK7QStringRKS_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QIcon {
  pub fn themeName<T: QIcon_themeName>(&mut self, value: T) -> i32 {
    value.themeName(self);
    return 1;
  }
}

pub trait QIcon_themeName {
  fn themeName(self, this: &mut QIcon) -> i32;
}

// proto: QString QIcon::themeName();
impl<'a> /*trait*/ QIcon_themeName for () {
  fn themeName(self, this: &mut QIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon9themeNameEv()};
    unsafe {_ZN5QIcon9themeNameEv()};
    return 1;
  }
}

impl /*struct*/ QIcon {
  pub fn name<T: QIcon_name>(&mut self, value: T) -> i32 {
    value.name(self);
    return 1;
  }
}

pub trait QIcon_name {
  fn name(self, this: &mut QIcon) -> i32;
}

// proto: QString QIcon::name();
impl<'a> /*trait*/ QIcon_name for () {
  fn name(self, this: &mut QIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QIcon4nameEv()};
    unsafe {_ZNK5QIcon4nameEv()};
    return 1;
  }
}

// proto: void QIcon::NewQIcon();
impl<'a> /*trait*/ QIcon_NewQIcon for () {
  fn NewQIcon(self) -> QIcon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIconC1Ev()};
    unsafe {_ZN5QIconC1Ev(qthis)};
    let rsthis = QIcon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QIcon::NewQIcon(QIconEngine * engine);
impl<'a> /*trait*/ QIcon_NewQIcon for (&'a mut QIconEngine) {
  fn NewQIcon(self) -> QIcon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIconC1EP11QIconEngine()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QIconC1EP11QIconEngine(qthis, arg0)};
    let rsthis = QIcon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QIcon {
  pub fn FreeQIcon<T: QIcon_FreeQIcon>(&mut self, value: T) -> i32 {
    value.FreeQIcon(self);
    return 1;
  }
}

pub trait QIcon_FreeQIcon {
  fn FreeQIcon(self, this: &mut QIcon) -> i32;
}

// proto: void QIcon::FreeQIcon();
impl<'a> /*trait*/ QIcon_FreeQIcon for () {
  fn FreeQIcon(self, this: &mut QIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIconD0Ev()};
    unsafe {_ZN5QIconD0Ev()};
    return 1;
  }
}

impl /*struct*/ QIcon {
  pub fn isDetached<T: QIcon_isDetached>(&mut self, value: T) -> i32 {
    value.isDetached(self);
    return 1;
  }
}

pub trait QIcon_isDetached {
  fn isDetached(self, this: &mut QIcon) -> i32;
}

// proto: bool QIcon::isDetached();
impl<'a> /*trait*/ QIcon_isDetached for () {
  fn isDetached(self, this: &mut QIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QIcon10isDetachedEv()};
    unsafe {_ZNK5QIcon10isDetachedEv()};
    return 1;
  }
}

// proto: void QIcon::NewQIcon(const QString & fileName);
impl<'a> /*trait*/ QIcon_NewQIcon for (&'a  QString) {
  fn NewQIcon(self) -> QIcon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIconC1ERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QIconC1ERK7QString(qthis, arg0)};
    let rsthis = QIcon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QIcon {
  pub fn cacheKey<T: QIcon_cacheKey>(&mut self, value: T) -> i32 {
    value.cacheKey(self);
    return 1;
  }
}

pub trait QIcon_cacheKey {
  fn cacheKey(self, this: &mut QIcon) -> i32;
}

// proto: long long QIcon::cacheKey();
impl<'a> /*trait*/ QIcon_cacheKey for () {
  fn cacheKey(self, this: &mut QIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QIcon8cacheKeyEv()};
    unsafe {_ZNK5QIcon8cacheKeyEv()};
    return 1;
  }
}

impl /*struct*/ QIcon {
  pub fn swap<T: QIcon_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QIcon_swap {
  fn swap(self, this: &mut QIcon) -> i32;
}

// proto: void QIcon::swap(QIcon & other);
impl<'a> /*trait*/ QIcon_swap for (&'a mut QIcon) {
  fn swap(self, this: &mut QIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QIcon4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QIcon {
  pub fn setThemeName<T: QIcon_setThemeName>(&mut self, value: T) -> i32 {
    value.setThemeName(self);
    return 1;
  }
}

pub trait QIcon_setThemeName {
  fn setThemeName(self, this: &mut QIcon) -> i32;
}

// proto: void QIcon::setThemeName(const QString & path);
impl<'a> /*trait*/ QIcon_setThemeName for (&'a  QString) {
  fn setThemeName(self, this: &mut QIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon12setThemeNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QIcon12setThemeNameERK7QString(arg0)};
    return 1;
  }
}

