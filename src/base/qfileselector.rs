// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qurl::QUrl;
use super::qobject::QObject;
use super::qstringlist::QStringList;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK13QFileSelector12allSelectorsEv() -> i32;
  fn _ZNK13QFileSelector10metaObjectEv() -> i32;
  fn _ZNK13QFileSelector6selectERK4QUrl(arg0: *const c_void) -> i32;
  fn _ZN13QFileSelectorC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN13QFileSelector17setExtraSelectorsERK11QStringList(arg0: *const c_void) -> i32;
  fn _ZNK13QFileSelector6selectERK7QString(arg0: *const c_void) -> i32;
  fn _ZN13QFileSelectorD0Ev() -> i32;
  fn _ZNK13QFileSelector14extraSelectorsEv() -> i32;
}

// body block begin
// class sizeof(QFileSelector)=1
pub struct QFileSelector {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFileSelector {
  pub fn allSelectors<T: QFileSelector_allSelectors>(&mut self, value: T) -> i32 {
    value.allSelectors(self);
    return 1;
  }
}

pub trait QFileSelector_allSelectors {
  fn allSelectors(self, this: &mut QFileSelector) -> i32;
}

// proto: QStringList QFileSelector::allSelectors();
impl<'a> /*trait*/ QFileSelector_allSelectors for () {
  fn allSelectors(self, this: &mut QFileSelector) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFileSelector12allSelectorsEv()};
    unsafe {_ZNK13QFileSelector12allSelectorsEv()};
    return 1;
  }
}

impl /*struct*/ QFileSelector {
  pub fn metaObject<T: QFileSelector_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QFileSelector_metaObject {
  fn metaObject(self, this: &mut QFileSelector) -> i32;
}

// proto: const QMetaObject * QFileSelector::metaObject();
impl<'a> /*trait*/ QFileSelector_metaObject for () {
  fn metaObject(self, this: &mut QFileSelector) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFileSelector10metaObjectEv()};
    unsafe {_ZNK13QFileSelector10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QFileSelector {
  pub fn select<T: QFileSelector_select>(&mut self, value: T) -> i32 {
    value.select(self);
    return 1;
  }
}

pub trait QFileSelector_select {
  fn select(self, this: &mut QFileSelector) -> i32;
}

// proto: QUrl QFileSelector::select(const QUrl & filePath);
impl<'a> /*trait*/ QFileSelector_select for (&'a  QUrl) {
  fn select(self, this: &mut QFileSelector) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFileSelector6selectERK4QUrl()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QFileSelector6selectERK4QUrl(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSelector {
  pub fn NewQFileSelector<T: QFileSelector_NewQFileSelector>(value: T) -> QFileSelector {
    let rsthis = value.NewQFileSelector();
    return rsthis;
    // return 1;
  }
}

pub trait QFileSelector_NewQFileSelector {
  fn NewQFileSelector(self) -> QFileSelector;
}

// proto: void QFileSelector::NewQFileSelector(QObject * parent);
impl<'a> /*trait*/ QFileSelector_NewQFileSelector for (&'a mut QObject) {
  fn NewQFileSelector(self) -> QFileSelector {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFileSelectorC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QFileSelectorC1EP7QObject(qthis, arg0)};
    let rsthis = QFileSelector{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileSelector {
  pub fn setExtraSelectors<T: QFileSelector_setExtraSelectors>(&mut self, value: T) -> i32 {
    value.setExtraSelectors(self);
    return 1;
  }
}

pub trait QFileSelector_setExtraSelectors {
  fn setExtraSelectors(self, this: &mut QFileSelector) -> i32;
}

// proto: void QFileSelector::setExtraSelectors(const QStringList & list);
impl<'a> /*trait*/ QFileSelector_setExtraSelectors for (&'a  QStringList) {
  fn setExtraSelectors(self, this: &mut QFileSelector) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFileSelector17setExtraSelectorsERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QFileSelector17setExtraSelectorsERK11QStringList(arg0)};
    return 1;
  }
}

// proto: QString QFileSelector::select(const QString & filePath);
impl<'a> /*trait*/ QFileSelector_select for (&'a  QString) {
  fn select(self, this: &mut QFileSelector) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFileSelector6selectERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QFileSelector6selectERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSelector {
  pub fn FreeQFileSelector<T: QFileSelector_FreeQFileSelector>(&mut self, value: T) -> i32 {
    value.FreeQFileSelector(self);
    return 1;
  }
}

pub trait QFileSelector_FreeQFileSelector {
  fn FreeQFileSelector(self, this: &mut QFileSelector) -> i32;
}

// proto: void QFileSelector::FreeQFileSelector();
impl<'a> /*trait*/ QFileSelector_FreeQFileSelector for () {
  fn FreeQFileSelector(self, this: &mut QFileSelector) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFileSelectorD0Ev()};
    unsafe {_ZN13QFileSelectorD0Ev()};
    return 1;
  }
}

impl /*struct*/ QFileSelector {
  pub fn extraSelectors<T: QFileSelector_extraSelectors>(&mut self, value: T) -> i32 {
    value.extraSelectors(self);
    return 1;
  }
}

pub trait QFileSelector_extraSelectors {
  fn extraSelectors(self, this: &mut QFileSelector) -> i32;
}

// proto: QStringList QFileSelector::extraSelectors();
impl<'a> /*trait*/ QFileSelector_extraSelectors for () {
  fn extraSelectors(self, this: &mut QFileSelector) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFileSelector14extraSelectorsEv()};
    unsafe {_ZNK13QFileSelector14extraSelectorsEv()};
    return 1;
  }
}

