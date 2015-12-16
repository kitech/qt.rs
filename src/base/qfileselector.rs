// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qstringlist::QStringList;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QStringList QFileSelector::allSelectors();
  fn _ZNK13QFileSelector12allSelectorsEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QFileSelector::metaObject();
  fn _ZNK13QFileSelector10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QFileSelector::NewQFileSelector(QObject * parent);
  fn _ZN13QFileSelectorC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileSelector::setExtraSelectors(const QStringList & list);
  fn _ZN13QFileSelector17setExtraSelectorsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileSelector::FreeQFileSelector();
  fn _ZN13QFileSelectorD0Ev(qthis: *mut c_void) ;
  // proto:  QStringList QFileSelector::extraSelectors();
  fn _ZNK13QFileSelector14extraSelectorsEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QFileSelector)=1
pub struct QFileSelector {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFileSelector {
  pub fn allSelectors<T: QFileSelector_allSelectors>(&mut self, value: T)  {
     value.allSelectors(self);
    // return 1;
  }
}

pub trait QFileSelector_allSelectors {
  fn allSelectors(self, rsthis: &mut QFileSelector) ;
}

// proto:  QStringList QFileSelector::allSelectors();
impl<'a> /*trait*/ QFileSelector_allSelectors for () {
  fn allSelectors(self, rsthis: &mut QFileSelector)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFileSelector12allSelectorsEv()};
     unsafe {_ZNK13QFileSelector12allSelectorsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileSelector {
  pub fn metaObject<T: QFileSelector_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QFileSelector_metaObject {
  fn metaObject(self, rsthis: &mut QFileSelector) ;
}

// proto:  const QMetaObject * QFileSelector::metaObject();
impl<'a> /*trait*/ QFileSelector_metaObject for () {
  fn metaObject(self, rsthis: &mut QFileSelector)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFileSelector10metaObjectEv()};
     unsafe {_ZNK13QFileSelector10metaObjectEv(rsthis.qclsinst)};
    // return 1;
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
  pub fn setExtraSelectors<T: QFileSelector_setExtraSelectors>(&mut self, value: T)  {
     value.setExtraSelectors(self);
    // return 1;
  }
}

pub trait QFileSelector_setExtraSelectors {
  fn setExtraSelectors(self, rsthis: &mut QFileSelector) ;
}

// proto:  void QFileSelector::setExtraSelectors(const QStringList & list);
impl<'a> /*trait*/ QFileSelector_setExtraSelectors for (&'a  QStringList) {
  fn setExtraSelectors(self, rsthis: &mut QFileSelector)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFileSelector17setExtraSelectorsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QFileSelector17setExtraSelectorsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileSelector {
  pub fn FreeQFileSelector<T: QFileSelector_FreeQFileSelector>(&mut self, value: T)  {
     value.FreeQFileSelector(self);
    // return 1;
  }
}

pub trait QFileSelector_FreeQFileSelector {
  fn FreeQFileSelector(self, rsthis: &mut QFileSelector) ;
}

// proto:  void QFileSelector::FreeQFileSelector();
impl<'a> /*trait*/ QFileSelector_FreeQFileSelector for () {
  fn FreeQFileSelector(self, rsthis: &mut QFileSelector)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFileSelectorD0Ev()};
     unsafe {_ZN13QFileSelectorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileSelector {
  pub fn extraSelectors<T: QFileSelector_extraSelectors>(&mut self, value: T)  {
     value.extraSelectors(self);
    // return 1;
  }
}

pub trait QFileSelector_extraSelectors {
  fn extraSelectors(self, rsthis: &mut QFileSelector) ;
}

// proto:  QStringList QFileSelector::extraSelectors();
impl<'a> /*trait*/ QFileSelector_extraSelectors for () {
  fn extraSelectors(self, rsthis: &mut QFileSelector)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFileSelector14extraSelectorsEv()};
     unsafe {_ZNK13QFileSelector14extraSelectorsEv(rsthis.qclsinst)};
    // return 1;
  }
}

