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
  // proto:  QStringList QFileSelector::allSelectors();
  fn _ZNK13QFileSelector12allSelectorsEv(qthis: *mut c_void);
  // proto:  const QMetaObject * QFileSelector::metaObject();
  fn _ZNK13QFileSelector10metaObjectEv(qthis: *mut c_void);
  // proto:  QUrl QFileSelector::select(const QUrl & filePath);
  fn _ZNK13QFileSelector6selectERK4QUrl(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QFileSelector::QFileSelector(QObject * parent);
  fn _ZN13QFileSelectorC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFileSelector::setExtraSelectors(const QStringList & list);
  fn _ZN13QFileSelector17setExtraSelectorsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QFileSelector::select(const QString & filePath);
  fn _ZNK13QFileSelector6selectERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QFileSelector::~QFileSelector();
  fn _ZN13QFileSelectorD0Ev(qthis: *mut c_void);
  // proto:  QStringList QFileSelector::extraSelectors();
  fn _ZNK13QFileSelector14extraSelectorsEv(qthis: *mut c_void);
}

// body block begin
// class sizeof(QFileSelector)=1
pub struct QFileSelector {
  pub qclsinst: *mut c_void,
}

  // proto:  QStringList QFileSelector::allSelectors();
impl /*struct*/ QFileSelector {
  pub fn allSelectors<RetType, T: QFileSelector_allSelectors<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.allSelectors(self);
    // return 1;
  }
}

pub trait QFileSelector_allSelectors<RetType> {
  fn allSelectors(self , rsthis: &mut QFileSelector) -> RetType;
}

  // proto:  QStringList QFileSelector::allSelectors();
impl<'a> /*trait*/ QFileSelector_allSelectors<()> for () {
  fn allSelectors(self , rsthis: &mut QFileSelector) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFileSelector12allSelectorsEv()};
     unsafe {_ZNK13QFileSelector12allSelectorsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QFileSelector::metaObject();
impl /*struct*/ QFileSelector {
  pub fn metaObject<RetType, T: QFileSelector_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QFileSelector_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QFileSelector) -> RetType;
}

  // proto:  const QMetaObject * QFileSelector::metaObject();
impl<'a> /*trait*/ QFileSelector_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QFileSelector) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFileSelector10metaObjectEv()};
     unsafe {_ZNK13QFileSelector10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QUrl QFileSelector::select(const QUrl & filePath);
impl /*struct*/ QFileSelector {
  pub fn select<RetType, T: QFileSelector_select<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.select(self);
    // return 1;
  }
}

pub trait QFileSelector_select<RetType> {
  fn select(self , rsthis: &mut QFileSelector) -> RetType;
}

  // proto:  QUrl QFileSelector::select(const QUrl & filePath);
impl<'a> /*trait*/ QFileSelector_select<QUrl> for (QUrl) {
  fn select(self , rsthis: &mut QFileSelector) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFileSelector6selectERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFileSelector6selectERK4QUrl(rsthis.qclsinst, arg0)};
    let mut ret1 = QUrl{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QFileSelector::QFileSelector(QObject * parent);
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

  // proto:  void QFileSelector::QFileSelector(QObject * parent);
impl<'a> /*trait*/ QFileSelector_NewQFileSelector for (QObject) {
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

  // proto:  void QFileSelector::setExtraSelectors(const QStringList & list);
impl /*struct*/ QFileSelector {
  pub fn setExtraSelectors<RetType, T: QFileSelector_setExtraSelectors<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setExtraSelectors(self);
    // return 1;
  }
}

pub trait QFileSelector_setExtraSelectors<RetType> {
  fn setExtraSelectors(self , rsthis: &mut QFileSelector) -> RetType;
}

  // proto:  void QFileSelector::setExtraSelectors(const QStringList & list);
impl<'a> /*trait*/ QFileSelector_setExtraSelectors<()> for (QStringList) {
  fn setExtraSelectors(self , rsthis: &mut QFileSelector) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFileSelector17setExtraSelectorsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QFileSelector17setExtraSelectorsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QFileSelector::select(const QString & filePath);
impl<'a> /*trait*/ QFileSelector_select<QString> for (QString) {
  fn select(self , rsthis: &mut QFileSelector) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFileSelector6selectERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFileSelector6selectERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QFileSelector::~QFileSelector();
impl /*struct*/ QFileSelector {
  pub fn FreeQFileSelector<RetType, T: QFileSelector_FreeQFileSelector<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQFileSelector(self);
    // return 1;
  }
}

pub trait QFileSelector_FreeQFileSelector<RetType> {
  fn FreeQFileSelector(self , rsthis: &mut QFileSelector) -> RetType;
}

  // proto:  void QFileSelector::~QFileSelector();
impl<'a> /*trait*/ QFileSelector_FreeQFileSelector<()> for () {
  fn FreeQFileSelector(self , rsthis: &mut QFileSelector) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFileSelectorD0Ev()};
     unsafe {_ZN13QFileSelectorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringList QFileSelector::extraSelectors();
impl /*struct*/ QFileSelector {
  pub fn extraSelectors<RetType, T: QFileSelector_extraSelectors<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.extraSelectors(self);
    // return 1;
  }
}

pub trait QFileSelector_extraSelectors<RetType> {
  fn extraSelectors(self , rsthis: &mut QFileSelector) -> RetType;
}

  // proto:  QStringList QFileSelector::extraSelectors();
impl<'a> /*trait*/ QFileSelector_extraSelectors<()> for () {
  fn extraSelectors(self , rsthis: &mut QFileSelector) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFileSelector14extraSelectorsEv()};
     unsafe {_ZNK13QFileSelector14extraSelectorsEv(rsthis.qclsinst)};
    // return 1;
  }
}

