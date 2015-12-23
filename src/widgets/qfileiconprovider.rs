// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
// src-file: /QtWidgets/qfileiconprovider.h
// dst-file: /src/widgets/qfileiconprovider.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use std::ops::Deref;
use super::super::core::qfileinfo::QFileInfo; // 771
use super::super::core::qstring::QString; // 771
use super::super::gui::qicon::QIcon; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QString QFileIconProvider::type(const QFileInfo & info);
  fn _ZNK17QFileIconProvider4typeERK9QFileInfo(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QIcon QFileIconProvider::icon(const QFileInfo & info);
  fn _ZNK17QFileIconProvider4iconERK9QFileInfo(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QFileIconProvider::QFileIconProvider(const QFileIconProvider & );
  fn _ZN17QFileIconProviderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFileIconProvider::QFileIconProvider();
  fn _ZN17QFileIconProviderC1Ev(qthis: *mut c_void);
  // proto:  void QFileIconProvider::~QFileIconProvider();
  fn _ZN17QFileIconProviderD0Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QFileIconProvider)=1
pub struct QFileIconProvider {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFileIconProvider {
  pub fn inheritFrom(qthis: *mut c_void) -> QFileIconProvider {
    return QFileIconProvider{qclsinst: qthis};
  }
}
  // proto:  QString QFileIconProvider::type(const QFileInfo & info);
impl /*struct*/ QFileIconProvider {
  pub fn type_<RetType, T: QFileIconProvider_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QFileIconProvider_type_<RetType> {
  fn type_(self , rsthis: & QFileIconProvider) -> RetType;
}

  // proto:  QString QFileIconProvider::type(const QFileInfo & info);
impl<'a> /*trait*/ QFileIconProvider_type_<QString> for (&'a QFileInfo) {
  fn type_(self , rsthis: & QFileIconProvider) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QFileIconProvider4typeERK9QFileInfo()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QFileIconProvider4typeERK9QFileInfo(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QIcon QFileIconProvider::icon(const QFileInfo & info);
impl /*struct*/ QFileIconProvider {
  pub fn icon<RetType, T: QFileIconProvider_icon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.icon(self);
    // return 1;
  }
}

pub trait QFileIconProvider_icon<RetType> {
  fn icon(self , rsthis: & QFileIconProvider) -> RetType;
}

  // proto:  QIcon QFileIconProvider::icon(const QFileInfo & info);
impl<'a> /*trait*/ QFileIconProvider_icon<QIcon> for (&'a QFileInfo) {
  fn icon(self , rsthis: & QFileIconProvider) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QFileIconProvider4iconERK9QFileInfo()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QFileIconProvider4iconERK9QFileInfo(rsthis.qclsinst, arg0)};
    let mut ret1 = QIcon::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFileIconProvider::QFileIconProvider(const QFileIconProvider & );
impl /*struct*/ QFileIconProvider {
  pub fn New<T: QFileIconProvider_New>(value: T) -> QFileIconProvider {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QFileIconProvider_New {
  fn New(self) -> QFileIconProvider;
}

  // proto:  void QFileIconProvider::QFileIconProvider(const QFileIconProvider & );
impl<'a> /*trait*/ QFileIconProvider_New for (&'a QFileIconProvider) {
  fn New(self) -> QFileIconProvider {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QFileIconProviderC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QFileIconProviderC1ERKS_(qthis, arg0)};
    let rsthis = QFileIconProvider{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFileIconProvider::QFileIconProvider();
impl<'a> /*trait*/ QFileIconProvider_New for () {
  fn New(self) -> QFileIconProvider {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QFileIconProviderC1Ev()};
    unsafe {_ZN17QFileIconProviderC1Ev(qthis)};
    let rsthis = QFileIconProvider{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFileIconProvider::~QFileIconProvider();
impl /*struct*/ QFileIconProvider {
  pub fn Free<RetType, T: QFileIconProvider_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QFileIconProvider_Free<RetType> {
  fn Free(self , rsthis: & QFileIconProvider) -> RetType;
}

  // proto:  void QFileIconProvider::~QFileIconProvider();
impl<'a> /*trait*/ QFileIconProvider_Free<()> for () {
  fn Free(self , rsthis: & QFileIconProvider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QFileIconProviderD0Ev()};
     unsafe {_ZN17QFileIconProviderD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

