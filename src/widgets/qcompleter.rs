// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
// src-file: /QtWidgets/qcompleter.h
// dst-file: /src/widgets/qcompleter.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::core::qrect::QRect; // 771
use super::super::core::qabstractitemmodel::QModelIndex; // 771
use super::super::core::qstringlist::QStringList; // 771
use super::super::core::qstring::QString; // 771
use super::super::core::qabstractitemmodel::QAbstractItemModel; // 771
use super::qabstractitemview::QAbstractItemView; // 773
use super::qwidget::QWidget; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QCompleter_Class_Size() -> c_int;
  // proto:  void QCompleter::QCompleter(QObject * parent);
  fn dector_ZN10QCompleterC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN10QCompleterC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QCompleter::metaObject();
  fn _ZNK10QCompleter10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QAbstractItemView * QCompleter::popup();
  fn _ZNK10QCompleter5popupEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QCompleter::complete(const QRect & rect);
  fn _ZN10QCompleter8completeERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCompleter::activated(const QModelIndex & index);
  fn _ZN10QCompleter9activatedERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCompleter::setCompletionRole(int role);
  fn _ZN10QCompleter17setCompletionRoleEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QCompleter::completionCount();
  fn _ZNK10QCompleter15completionCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QCompleter::QCompleter(const QStringList & completions, QObject * parent);
  fn dector_ZN10QCompleterC1ERK11QStringListP7QObject(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN10QCompleterC1ERK11QStringListP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QModelIndex QCompleter::currentIndex();
  fn _ZNK10QCompleter12currentIndexEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QCompleter::pathFromIndex(const QModelIndex & index);
  fn _ZNK10QCompleter13pathFromIndexERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QCompleter::highlighted(const QModelIndex & index);
  fn _ZN10QCompleter11highlightedERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCompleter::setMaxVisibleItems(int maxItems);
  fn _ZN10QCompleter18setMaxVisibleItemsEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QCompleter::completionColumn();
  fn _ZNK10QCompleter16completionColumnEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QCompleter::maxVisibleItems();
  fn _ZNK10QCompleter15maxVisibleItemsEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QCompleter::~QCompleter();
  fn _ZN10QCompleterD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QCompleter::setWrapAround(bool wrap);
  fn _ZN10QCompleter13setWrapAroundEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QStringList QCompleter::splitPath(const QString & path);
  fn _ZNK10QCompleter9splitPathERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QAbstractItemModel * QCompleter::model();
  fn _ZNK10QCompleter5modelEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QCompleter::currentCompletion();
  fn _ZNK10QCompleter17currentCompletionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QCompleter::setCompletionColumn(int column);
  fn _ZN10QCompleter19setCompletionColumnEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QCompleter::highlighted(const QString & text);
  fn _ZN10QCompleter11highlightedERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCompleter::setCompletionPrefix(const QString & prefix);
  fn _ZN10QCompleter19setCompletionPrefixERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QAbstractItemModel * QCompleter::completionModel();
  fn _ZNK10QCompleter15completionModelEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QCompleter::setCurrentRow(int row);
  fn _ZN10QCompleter13setCurrentRowEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  int QCompleter::currentRow();
  fn _ZNK10QCompleter10currentRowEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QCompleter::activated(const QString & text);
  fn _ZN10QCompleter9activatedERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCompleter::setModel(QAbstractItemModel * c);
  fn _ZN10QCompleter8setModelEP18QAbstractItemModel(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QCompleter::wrapAround();
  fn _ZNK10QCompleter10wrapAroundEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QCompleter::QCompleter(QAbstractItemModel * model, QObject * parent);
  fn dector_ZN10QCompleterC1EP18QAbstractItemModelP7QObject(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN10QCompleterC1EP18QAbstractItemModelP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QCompleter::setPopup(QAbstractItemView * popup);
  fn _ZN10QCompleter8setPopupEP17QAbstractItemView(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCompleter::QCompleter(const QCompleter & );
  fn dector_ZN10QCompleterC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN10QCompleterC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWidget * QCompleter::widget();
  fn _ZNK10QCompleter6widgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QCompleter::completionRole();
  fn _ZNK10QCompleter14completionRoleEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QString QCompleter::completionPrefix();
  fn _ZNK10QCompleter16completionPrefixEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QCompleter::setWidget(QWidget * widget);
  fn _ZN10QCompleter9setWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QCompleter_SlotProxy_connect__ZN10QCompleter11highlightedERK11QModelIndex(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QCompleter_SlotProxy_connect_box__ZN10QCompleter11highlightedERK11QModelIndex(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QCompleter_SlotProxy_connect__ZN10QCompleter9activatedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QCompleter_SlotProxy_connect_box__ZN10QCompleter9activatedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QCompleter_SlotProxy_connect__ZN10QCompleter9activatedERK11QModelIndex(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QCompleter_SlotProxy_connect_box__ZN10QCompleter9activatedERK11QModelIndex(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QCompleter_SlotProxy_connect__ZN10QCompleter11highlightedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QCompleter_SlotProxy_connect_box__ZN10QCompleter11highlightedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QCompleter)=1
#[derive(Default)]
pub struct QCompleter {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _highlighted_1: QCompleter_highlighted_signal,
  pub _activated_1: QCompleter_activated_signal,
}

impl /*struct*/ QCompleter {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QCompleter {
    return QCompleter{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QCompleter {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QCompleter {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QCompleter::QCompleter(QObject * parent);
impl /*struct*/ QCompleter {
  pub fn New<T: QCompleter_New>(value: T) -> QCompleter {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QCompleter_New {
  fn New(self) -> QCompleter;
}

  // proto:  void QCompleter::QCompleter(QObject * parent);
impl<'a> /*trait*/ QCompleter_New for (&'a QObject) {
  fn New(self) -> QCompleter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleterC1EP7QObject()};
    let ctysz: c_int = unsafe{QCompleter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QCompleterC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN10QCompleterC1EP7QObject(arg0)} as u64;
    let rsthis = QCompleter{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QCompleter::metaObject();
impl /*struct*/ QCompleter {
  pub fn metaObject<RetType, T: QCompleter_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QCompleter_metaObject<RetType> {
  fn metaObject(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  const QMetaObject * QCompleter::metaObject();
impl<'a> /*trait*/ QCompleter_metaObject<()> for () {
  fn metaObject(self , rsthis: & QCompleter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter10metaObjectEv()};
     unsafe {_ZNK10QCompleter10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAbstractItemView * QCompleter::popup();
impl /*struct*/ QCompleter {
  pub fn popup<RetType, T: QCompleter_popup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.popup(self);
    // return 1;
  }
}

pub trait QCompleter_popup<RetType> {
  fn popup(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  QAbstractItemView * QCompleter::popup();
impl<'a> /*trait*/ QCompleter_popup<()> for () {
  fn popup(self , rsthis: & QCompleter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter5popupEv()};
     unsafe {_ZNK10QCompleter5popupEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCompleter::complete(const QRect & rect);
impl /*struct*/ QCompleter {
  pub fn complete<RetType, T: QCompleter_complete<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.complete(self);
    // return 1;
  }
}

pub trait QCompleter_complete<RetType> {
  fn complete(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  void QCompleter::complete(const QRect & rect);
impl<'a> /*trait*/ QCompleter_complete<()> for (&'a QRect) {
  fn complete(self , rsthis: & QCompleter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter8completeERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QCompleter8completeERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCompleter::activated(const QModelIndex & index);
impl /*struct*/ QCompleter {
  pub fn activated<RetType, T: QCompleter_activated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activated(self);
    // return 1;
  }
}

pub trait QCompleter_activated<RetType> {
  fn activated(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  void QCompleter::activated(const QModelIndex & index);
impl<'a> /*trait*/ QCompleter_activated<()> for (&'a QModelIndex) {
  fn activated(self , rsthis: & QCompleter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter9activatedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QCompleter9activatedERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCompleter::setCompletionRole(int role);
impl /*struct*/ QCompleter {
  pub fn setCompletionRole<RetType, T: QCompleter_setCompletionRole<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCompletionRole(self);
    // return 1;
  }
}

pub trait QCompleter_setCompletionRole<RetType> {
  fn setCompletionRole(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  void QCompleter::setCompletionRole(int role);
impl<'a> /*trait*/ QCompleter_setCompletionRole<()> for (i32) {
  fn setCompletionRole(self , rsthis: & QCompleter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter17setCompletionRoleEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QCompleter17setCompletionRoleEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QCompleter::completionCount();
impl /*struct*/ QCompleter {
  pub fn completionCount<RetType, T: QCompleter_completionCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.completionCount(self);
    // return 1;
  }
}

pub trait QCompleter_completionCount<RetType> {
  fn completionCount(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  int QCompleter::completionCount();
impl<'a> /*trait*/ QCompleter_completionCount<i32> for () {
  fn completionCount(self , rsthis: & QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter15completionCountEv()};
    let mut ret = unsafe {_ZNK10QCompleter15completionCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QCompleter::QCompleter(const QStringList & completions, QObject * parent);
impl<'a> /*trait*/ QCompleter_New for (&'a QStringList, &'a QObject) {
  fn New(self) -> QCompleter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleterC1ERK11QStringListP7QObject()};
    let ctysz: c_int = unsafe{QCompleter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN10QCompleterC1ERK11QStringListP7QObject(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN10QCompleterC1ERK11QStringListP7QObject(arg0, arg1)} as u64;
    let rsthis = QCompleter{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QModelIndex QCompleter::currentIndex();
impl /*struct*/ QCompleter {
  pub fn currentIndex<RetType, T: QCompleter_currentIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentIndex(self);
    // return 1;
  }
}

pub trait QCompleter_currentIndex<RetType> {
  fn currentIndex(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  QModelIndex QCompleter::currentIndex();
impl<'a> /*trait*/ QCompleter_currentIndex<QModelIndex> for () {
  fn currentIndex(self , rsthis: & QCompleter) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter12currentIndexEv()};
    let mut ret = unsafe {_ZNK10QCompleter12currentIndexEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QCompleter::pathFromIndex(const QModelIndex & index);
impl /*struct*/ QCompleter {
  pub fn pathFromIndex<RetType, T: QCompleter_pathFromIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pathFromIndex(self);
    // return 1;
  }
}

pub trait QCompleter_pathFromIndex<RetType> {
  fn pathFromIndex(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  QString QCompleter::pathFromIndex(const QModelIndex & index);
impl<'a> /*trait*/ QCompleter_pathFromIndex<QString> for (&'a QModelIndex) {
  fn pathFromIndex(self , rsthis: & QCompleter) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter13pathFromIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QCompleter13pathFromIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QCompleter::highlighted(const QModelIndex & index);
impl /*struct*/ QCompleter {
  pub fn highlighted<RetType, T: QCompleter_highlighted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.highlighted(self);
    // return 1;
  }
}

pub trait QCompleter_highlighted<RetType> {
  fn highlighted(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  void QCompleter::highlighted(const QModelIndex & index);
impl<'a> /*trait*/ QCompleter_highlighted<()> for (&'a QModelIndex) {
  fn highlighted(self , rsthis: & QCompleter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter11highlightedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QCompleter11highlightedERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCompleter::setMaxVisibleItems(int maxItems);
impl /*struct*/ QCompleter {
  pub fn setMaxVisibleItems<RetType, T: QCompleter_setMaxVisibleItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaxVisibleItems(self);
    // return 1;
  }
}

pub trait QCompleter_setMaxVisibleItems<RetType> {
  fn setMaxVisibleItems(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  void QCompleter::setMaxVisibleItems(int maxItems);
impl<'a> /*trait*/ QCompleter_setMaxVisibleItems<()> for (i32) {
  fn setMaxVisibleItems(self , rsthis: & QCompleter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter18setMaxVisibleItemsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QCompleter18setMaxVisibleItemsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QCompleter::completionColumn();
impl /*struct*/ QCompleter {
  pub fn completionColumn<RetType, T: QCompleter_completionColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.completionColumn(self);
    // return 1;
  }
}

pub trait QCompleter_completionColumn<RetType> {
  fn completionColumn(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  int QCompleter::completionColumn();
impl<'a> /*trait*/ QCompleter_completionColumn<i32> for () {
  fn completionColumn(self , rsthis: & QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter16completionColumnEv()};
    let mut ret = unsafe {_ZNK10QCompleter16completionColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QCompleter::maxVisibleItems();
impl /*struct*/ QCompleter {
  pub fn maxVisibleItems<RetType, T: QCompleter_maxVisibleItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maxVisibleItems(self);
    // return 1;
  }
}

pub trait QCompleter_maxVisibleItems<RetType> {
  fn maxVisibleItems(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  int QCompleter::maxVisibleItems();
impl<'a> /*trait*/ QCompleter_maxVisibleItems<i32> for () {
  fn maxVisibleItems(self , rsthis: & QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter15maxVisibleItemsEv()};
    let mut ret = unsafe {_ZNK10QCompleter15maxVisibleItemsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QCompleter::~QCompleter();
impl /*struct*/ QCompleter {
  pub fn Free<RetType, T: QCompleter_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QCompleter_Free<RetType> {
  fn Free(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  void QCompleter::~QCompleter();
impl<'a> /*trait*/ QCompleter_Free<()> for () {
  fn Free(self , rsthis: & QCompleter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleterD0Ev()};
     unsafe {_ZN10QCompleterD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCompleter::setWrapAround(bool wrap);
impl /*struct*/ QCompleter {
  pub fn setWrapAround<RetType, T: QCompleter_setWrapAround<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWrapAround(self);
    // return 1;
  }
}

pub trait QCompleter_setWrapAround<RetType> {
  fn setWrapAround(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  void QCompleter::setWrapAround(bool wrap);
impl<'a> /*trait*/ QCompleter_setWrapAround<()> for (i8) {
  fn setWrapAround(self , rsthis: & QCompleter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter13setWrapAroundEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QCompleter13setWrapAroundEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringList QCompleter::splitPath(const QString & path);
impl /*struct*/ QCompleter {
  pub fn splitPath<RetType, T: QCompleter_splitPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.splitPath(self);
    // return 1;
  }
}

pub trait QCompleter_splitPath<RetType> {
  fn splitPath(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  QStringList QCompleter::splitPath(const QString & path);
impl<'a> /*trait*/ QCompleter_splitPath<()> for (&'a QString) {
  fn splitPath(self , rsthis: & QCompleter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter9splitPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK10QCompleter9splitPathERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAbstractItemModel * QCompleter::model();
impl /*struct*/ QCompleter {
  pub fn model<RetType, T: QCompleter_model<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.model(self);
    // return 1;
  }
}

pub trait QCompleter_model<RetType> {
  fn model(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  QAbstractItemModel * QCompleter::model();
impl<'a> /*trait*/ QCompleter_model<()> for () {
  fn model(self , rsthis: & QCompleter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter5modelEv()};
     unsafe {_ZNK10QCompleter5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QCompleter::currentCompletion();
impl /*struct*/ QCompleter {
  pub fn currentCompletion<RetType, T: QCompleter_currentCompletion<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentCompletion(self);
    // return 1;
  }
}

pub trait QCompleter_currentCompletion<RetType> {
  fn currentCompletion(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  QString QCompleter::currentCompletion();
impl<'a> /*trait*/ QCompleter_currentCompletion<QString> for () {
  fn currentCompletion(self , rsthis: & QCompleter) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter17currentCompletionEv()};
    let mut ret = unsafe {_ZNK10QCompleter17currentCompletionEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QCompleter::setCompletionColumn(int column);
impl /*struct*/ QCompleter {
  pub fn setCompletionColumn<RetType, T: QCompleter_setCompletionColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCompletionColumn(self);
    // return 1;
  }
}

pub trait QCompleter_setCompletionColumn<RetType> {
  fn setCompletionColumn(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  void QCompleter::setCompletionColumn(int column);
impl<'a> /*trait*/ QCompleter_setCompletionColumn<()> for (i32) {
  fn setCompletionColumn(self , rsthis: & QCompleter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter19setCompletionColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QCompleter19setCompletionColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCompleter::highlighted(const QString & text);
impl<'a> /*trait*/ QCompleter_highlighted<()> for (&'a QString) {
  fn highlighted(self , rsthis: & QCompleter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter11highlightedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QCompleter11highlightedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCompleter::setCompletionPrefix(const QString & prefix);
impl /*struct*/ QCompleter {
  pub fn setCompletionPrefix<RetType, T: QCompleter_setCompletionPrefix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCompletionPrefix(self);
    // return 1;
  }
}

pub trait QCompleter_setCompletionPrefix<RetType> {
  fn setCompletionPrefix(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  void QCompleter::setCompletionPrefix(const QString & prefix);
impl<'a> /*trait*/ QCompleter_setCompletionPrefix<()> for (&'a QString) {
  fn setCompletionPrefix(self , rsthis: & QCompleter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter19setCompletionPrefixERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QCompleter19setCompletionPrefixERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAbstractItemModel * QCompleter::completionModel();
impl /*struct*/ QCompleter {
  pub fn completionModel<RetType, T: QCompleter_completionModel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.completionModel(self);
    // return 1;
  }
}

pub trait QCompleter_completionModel<RetType> {
  fn completionModel(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  QAbstractItemModel * QCompleter::completionModel();
impl<'a> /*trait*/ QCompleter_completionModel<()> for () {
  fn completionModel(self , rsthis: & QCompleter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter15completionModelEv()};
     unsafe {_ZNK10QCompleter15completionModelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QCompleter::setCurrentRow(int row);
impl /*struct*/ QCompleter {
  pub fn setCurrentRow<RetType, T: QCompleter_setCurrentRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentRow(self);
    // return 1;
  }
}

pub trait QCompleter_setCurrentRow<RetType> {
  fn setCurrentRow(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  bool QCompleter::setCurrentRow(int row);
impl<'a> /*trait*/ QCompleter_setCurrentRow<i8> for (i32) {
  fn setCurrentRow(self , rsthis: & QCompleter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter13setCurrentRowEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN10QCompleter13setCurrentRowEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QCompleter::currentRow();
impl /*struct*/ QCompleter {
  pub fn currentRow<RetType, T: QCompleter_currentRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentRow(self);
    // return 1;
  }
}

pub trait QCompleter_currentRow<RetType> {
  fn currentRow(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  int QCompleter::currentRow();
impl<'a> /*trait*/ QCompleter_currentRow<i32> for () {
  fn currentRow(self , rsthis: & QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter10currentRowEv()};
    let mut ret = unsafe {_ZNK10QCompleter10currentRowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QCompleter::activated(const QString & text);
impl<'a> /*trait*/ QCompleter_activated<()> for (&'a QString) {
  fn activated(self , rsthis: & QCompleter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter9activatedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QCompleter9activatedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCompleter::setModel(QAbstractItemModel * c);
impl /*struct*/ QCompleter {
  pub fn setModel<RetType, T: QCompleter_setModel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setModel(self);
    // return 1;
  }
}

pub trait QCompleter_setModel<RetType> {
  fn setModel(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  void QCompleter::setModel(QAbstractItemModel * c);
impl<'a> /*trait*/ QCompleter_setModel<()> for (&'a QAbstractItemModel) {
  fn setModel(self , rsthis: & QCompleter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter8setModelEP18QAbstractItemModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QCompleter8setModelEP18QAbstractItemModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QCompleter::wrapAround();
impl /*struct*/ QCompleter {
  pub fn wrapAround<RetType, T: QCompleter_wrapAround<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.wrapAround(self);
    // return 1;
  }
}

pub trait QCompleter_wrapAround<RetType> {
  fn wrapAround(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  bool QCompleter::wrapAround();
impl<'a> /*trait*/ QCompleter_wrapAround<i8> for () {
  fn wrapAround(self , rsthis: & QCompleter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter10wrapAroundEv()};
    let mut ret = unsafe {_ZNK10QCompleter10wrapAroundEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QCompleter::QCompleter(QAbstractItemModel * model, QObject * parent);
impl<'a> /*trait*/ QCompleter_New for (&'a QAbstractItemModel, &'a QObject) {
  fn New(self) -> QCompleter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleterC1EP18QAbstractItemModelP7QObject()};
    let ctysz: c_int = unsafe{QCompleter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN10QCompleterC1EP18QAbstractItemModelP7QObject(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN10QCompleterC1EP18QAbstractItemModelP7QObject(arg0, arg1)} as u64;
    let rsthis = QCompleter{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QCompleter::setPopup(QAbstractItemView * popup);
impl /*struct*/ QCompleter {
  pub fn setPopup<RetType, T: QCompleter_setPopup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPopup(self);
    // return 1;
  }
}

pub trait QCompleter_setPopup<RetType> {
  fn setPopup(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  void QCompleter::setPopup(QAbstractItemView * popup);
impl<'a> /*trait*/ QCompleter_setPopup<()> for (&'a QAbstractItemView) {
  fn setPopup(self , rsthis: & QCompleter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter8setPopupEP17QAbstractItemView()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QCompleter8setPopupEP17QAbstractItemView(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCompleter::QCompleter(const QCompleter & );
impl<'a> /*trait*/ QCompleter_New for (&'a QCompleter) {
  fn New(self) -> QCompleter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleterC1ERKS_()};
    let ctysz: c_int = unsafe{QCompleter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QCompleterC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN10QCompleterC1ERKS_(arg0)} as u64;
    let rsthis = QCompleter{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QWidget * QCompleter::widget();
impl /*struct*/ QCompleter {
  pub fn widget<RetType, T: QCompleter_widget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QCompleter_widget<RetType> {
  fn widget(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  QWidget * QCompleter::widget();
impl<'a> /*trait*/ QCompleter_widget<QWidget> for () {
  fn widget(self , rsthis: & QCompleter) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter6widgetEv()};
    let mut ret = unsafe {_ZNK10QCompleter6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QCompleter::completionRole();
impl /*struct*/ QCompleter {
  pub fn completionRole<RetType, T: QCompleter_completionRole<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.completionRole(self);
    // return 1;
  }
}

pub trait QCompleter_completionRole<RetType> {
  fn completionRole(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  int QCompleter::completionRole();
impl<'a> /*trait*/ QCompleter_completionRole<i32> for () {
  fn completionRole(self , rsthis: & QCompleter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter14completionRoleEv()};
    let mut ret = unsafe {_ZNK10QCompleter14completionRoleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString QCompleter::completionPrefix();
impl /*struct*/ QCompleter {
  pub fn completionPrefix<RetType, T: QCompleter_completionPrefix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.completionPrefix(self);
    // return 1;
  }
}

pub trait QCompleter_completionPrefix<RetType> {
  fn completionPrefix(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  QString QCompleter::completionPrefix();
impl<'a> /*trait*/ QCompleter_completionPrefix<QString> for () {
  fn completionPrefix(self , rsthis: & QCompleter) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QCompleter16completionPrefixEv()};
    let mut ret = unsafe {_ZNK10QCompleter16completionPrefixEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QCompleter::setWidget(QWidget * widget);
impl /*struct*/ QCompleter {
  pub fn setWidget<RetType, T: QCompleter_setWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWidget(self);
    // return 1;
  }
}

pub trait QCompleter_setWidget<RetType> {
  fn setWidget(self , rsthis: & QCompleter) -> RetType;
}

  // proto:  void QCompleter::setWidget(QWidget * widget);
impl<'a> /*trait*/ QCompleter_setWidget<()> for (&'a QWidget) {
  fn setWidget(self , rsthis: & QCompleter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QCompleter9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QCompleter9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QCompleter_highlighted
pub struct QCompleter_highlighted_signal{poi:u64}
impl /* struct */ QCompleter {
  pub fn highlighted_1(&self) -> QCompleter_highlighted_signal {
     return QCompleter_highlighted_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QCompleter_highlighted_signal {
  pub fn connect<T: QCompleter_highlighted_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QCompleter_highlighted_signal_connect {
  fn connect(self, sigthis: QCompleter_highlighted_signal);
}

#[derive(Default)] // for QCompleter_activated
pub struct QCompleter_activated_signal{poi:u64}
impl /* struct */ QCompleter {
  pub fn activated_1(&self) -> QCompleter_activated_signal {
     return QCompleter_activated_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QCompleter_activated_signal {
  pub fn connect<T: QCompleter_activated_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QCompleter_activated_signal_connect {
  fn connect(self, sigthis: QCompleter_activated_signal);
}

// highlighted(const class QModelIndex &)
extern fn QCompleter_highlighted_signal_connect_cb_0(rsfptr:fn(QModelIndex), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
extern fn QCompleter_highlighted_signal_connect_cb_box_0(rsfptr_raw:*mut c_void, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QCompleter_highlighted_signal_connect for fn(QModelIndex) {
  fn connect(self, sigthis: QCompleter_highlighted_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QCompleter_highlighted_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QCompleter_SlotProxy_connect__ZN10QCompleter11highlightedERK11QModelIndex(arg0, arg1, arg2)};
  }
}
impl /* trait */ QCompleter_highlighted_signal_connect for Box<fn(QModelIndex)> {
  fn connect(self, sigthis: QCompleter_highlighted_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QCompleter_highlighted_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QCompleter_SlotProxy_connect__ZN10QCompleter11highlightedERK11QModelIndex(arg0, arg1, arg2)};
  }
}
// activated(const class QString &)
extern fn QCompleter_activated_signal_connect_cb_1(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
extern fn QCompleter_activated_signal_connect_cb_box_1(rsfptr_raw:*mut c_void, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QCompleter_activated_signal_connect for fn(QString) {
  fn connect(self, sigthis: QCompleter_activated_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QCompleter_activated_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QCompleter_SlotProxy_connect__ZN10QCompleter9activatedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QCompleter_activated_signal_connect for Box<fn(QString)> {
  fn connect(self, sigthis: QCompleter_activated_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QCompleter_activated_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QCompleter_SlotProxy_connect__ZN10QCompleter9activatedERK7QString(arg0, arg1, arg2)};
  }
}
// activated(const class QModelIndex &)
extern fn QCompleter_activated_signal_connect_cb_2(rsfptr:fn(QModelIndex), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
extern fn QCompleter_activated_signal_connect_cb_box_2(rsfptr_raw:*mut c_void, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QCompleter_activated_signal_connect for fn(QModelIndex) {
  fn connect(self, sigthis: QCompleter_activated_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QCompleter_activated_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QCompleter_SlotProxy_connect__ZN10QCompleter9activatedERK11QModelIndex(arg0, arg1, arg2)};
  }
}
impl /* trait */ QCompleter_activated_signal_connect for Box<fn(QModelIndex)> {
  fn connect(self, sigthis: QCompleter_activated_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QCompleter_activated_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QCompleter_SlotProxy_connect__ZN10QCompleter9activatedERK11QModelIndex(arg0, arg1, arg2)};
  }
}
// highlighted(const class QString &)
extern fn QCompleter_highlighted_signal_connect_cb_3(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
extern fn QCompleter_highlighted_signal_connect_cb_box_3(rsfptr_raw:*mut c_void, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QCompleter_highlighted_signal_connect for fn(QString) {
  fn connect(self, sigthis: QCompleter_highlighted_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QCompleter_highlighted_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QCompleter_SlotProxy_connect__ZN10QCompleter11highlightedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QCompleter_highlighted_signal_connect for Box<fn(QString)> {
  fn connect(self, sigthis: QCompleter_highlighted_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QCompleter_highlighted_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QCompleter_SlotProxy_connect__ZN10QCompleter11highlightedERK7QString(arg0, arg1, arg2)};
  }
}
// <= body block end

