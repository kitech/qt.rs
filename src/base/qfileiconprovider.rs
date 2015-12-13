// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qfileinfo::QFileInfo;
use super::qstring::QString;
use super::qicon::QIcon;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QString QFileIconProvider::type_(const QFileInfo & info);
  fn _ZNK17QFileIconProvider4typeERK9QFileInfo(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QIcon QFileIconProvider::icon(const QFileInfo & info);
  fn _ZNK17QFileIconProvider4iconERK9QFileInfo(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QFileIconProvider::NewQFileIconProvider(const QFileIconProvider & );
  fn _ZN17QFileIconProviderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileIconProvider::NewQFileIconProvider();
  fn _ZN17QFileIconProviderC1Ev(qthis: *mut c_void) ;
  // proto:  void QFileIconProvider::FreeQFileIconProvider();
  fn _ZN17QFileIconProviderD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QFileIconProvider)=1
pub struct QFileIconProvider {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFileIconProvider {
  pub fn type_<T: QFileIconProvider_type_>(&mut self, value: T) -> QString {
    return value.type_(self);
    // return 1;
  }
}

pub trait QFileIconProvider_type_ {
  fn type_(self, rsthis: &mut QFileIconProvider) -> QString;
}

// proto:  QString QFileIconProvider::type_(const QFileInfo & info);
impl<'a> /*trait*/ QFileIconProvider_type_ for (&'a  QFileInfo) {
  fn type_(self, rsthis: &mut QFileIconProvider) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QFileIconProvider4typeERK9QFileInfo()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QFileIconProvider4typeERK9QFileInfo(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileIconProvider {
  pub fn icon<T: QFileIconProvider_icon>(&mut self, value: T) -> QIcon {
    return value.icon(self);
    // return 1;
  }
}

pub trait QFileIconProvider_icon {
  fn icon(self, rsthis: &mut QFileIconProvider) -> QIcon;
}

// proto:  QIcon QFileIconProvider::icon(const QFileInfo & info);
impl<'a> /*trait*/ QFileIconProvider_icon for (&'a  QFileInfo) {
  fn icon(self, rsthis: &mut QFileIconProvider) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QFileIconProvider4iconERK9QFileInfo()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QFileIconProvider4iconERK9QFileInfo(rsthis.qclsinst, arg0)};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
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
  pub fn FreeQFileIconProvider<T: QFileIconProvider_FreeQFileIconProvider>(&mut self, value: T)  {
     value.FreeQFileIconProvider(self);
    // return 1;
  }
}

pub trait QFileIconProvider_FreeQFileIconProvider {
  fn FreeQFileIconProvider(self, rsthis: &mut QFileIconProvider) ;
}

// proto:  void QFileIconProvider::FreeQFileIconProvider();
impl<'a> /*trait*/ QFileIconProvider_FreeQFileIconProvider for () {
  fn FreeQFileIconProvider(self, rsthis: &mut QFileIconProvider)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QFileIconProviderD0Ev()};
     unsafe {_ZN17QFileIconProviderD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

