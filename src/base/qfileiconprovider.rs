// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qfileinfo::QFileInfo;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QString QFileIconProvider::type_(const QFileInfo & info);
  fn _ZNK17QFileIconProvider4typeERK9QFileInfo(arg0: *const c_void) -> i32;
  // proto: QIcon QFileIconProvider::icon(const QFileInfo & info);
  fn _ZNK17QFileIconProvider4iconERK9QFileInfo(arg0: *const c_void) -> i32;
  // proto: void QFileIconProvider::NewQFileIconProvider(const QFileIconProvider & );
  fn _ZN17QFileIconProviderC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QFileIconProvider::NewQFileIconProvider();
  fn _ZN17QFileIconProviderC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QFileIconProvider::FreeQFileIconProvider();
  fn _ZN17QFileIconProviderD0Ev() -> i32;
}

// body block begin
// class sizeof(QFileIconProvider)=1
pub struct QFileIconProvider {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFileIconProvider {
  pub fn type_<T: QFileIconProvider_type_>(&mut self, value: T) -> i32 {
    value.type_(self);
    return 1;
  }
}

pub trait QFileIconProvider_type_ {
  fn type_(self, this: &mut QFileIconProvider) -> i32;
}

// proto: QString QFileIconProvider::type_(const QFileInfo & info);
impl<'a> /*trait*/ QFileIconProvider_type_ for (&'a  QFileInfo) {
  fn type_(self, this: &mut QFileIconProvider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QFileIconProvider4typeERK9QFileInfo()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK17QFileIconProvider4typeERK9QFileInfo(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileIconProvider {
  pub fn icon<T: QFileIconProvider_icon>(&mut self, value: T) -> i32 {
    value.icon(self);
    return 1;
  }
}

pub trait QFileIconProvider_icon {
  fn icon(self, this: &mut QFileIconProvider) -> i32;
}

// proto: QIcon QFileIconProvider::icon(const QFileInfo & info);
impl<'a> /*trait*/ QFileIconProvider_icon for (&'a  QFileInfo) {
  fn icon(self, this: &mut QFileIconProvider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QFileIconProvider4iconERK9QFileInfo()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK17QFileIconProvider4iconERK9QFileInfo(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileIconProvider {
  pub fn NewQFileIconProvider<T: QFileIconProvider_NewQFileIconProvider>(value: T) -> QFileIconProvider {
    let rsthis = value.NewQFileIconProvider();
    return rsthis;
    // return 1;
  }
}

pub trait QFileIconProvider_NewQFileIconProvider {
  fn NewQFileIconProvider(self) -> QFileIconProvider;
}

// proto: void QFileIconProvider::NewQFileIconProvider(const QFileIconProvider & );
impl<'a> /*trait*/ QFileIconProvider_NewQFileIconProvider for (&'a  QFileIconProvider) {
  fn NewQFileIconProvider(self) -> QFileIconProvider {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QFileIconProviderC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN17QFileIconProviderC1ERKS_(qthis, arg0)};
    let rsthis = QFileIconProvider{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QFileIconProvider::NewQFileIconProvider();
impl<'a> /*trait*/ QFileIconProvider_NewQFileIconProvider for () {
  fn NewQFileIconProvider(self) -> QFileIconProvider {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QFileIconProviderC1Ev()};
    unsafe {_ZN17QFileIconProviderC1Ev(qthis)};
    let rsthis = QFileIconProvider{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileIconProvider {
  pub fn FreeQFileIconProvider<T: QFileIconProvider_FreeQFileIconProvider>(&mut self, value: T) -> i32 {
    value.FreeQFileIconProvider(self);
    return 1;
  }
}

pub trait QFileIconProvider_FreeQFileIconProvider {
  fn FreeQFileIconProvider(self, this: &mut QFileIconProvider) -> i32;
}

// proto: void QFileIconProvider::FreeQFileIconProvider();
impl<'a> /*trait*/ QFileIconProvider_FreeQFileIconProvider for () {
  fn FreeQFileIconProvider(self, this: &mut QFileIconProvider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QFileIconProviderD0Ev()};
    unsafe {_ZN17QFileIconProviderD0Ev()};
    return 1;
  }
}

