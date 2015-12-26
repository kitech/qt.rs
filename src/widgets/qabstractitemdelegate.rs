// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtWidgets/qabstractitemdelegate.h
// dst-file: /src/widgets/qabstractitemdelegate.rs
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
use super::qwidget::QWidget; // 773
use super::super::core::qabstractitemmodel::QModelIndex; // 771
use super::qstyleoption::QStyleOptionViewItem; // 773
use super::super::gui::qfontmetrics::QFontMetrics; // 771
use super::super::core::qstring::QString; // 771
use super::super::gui::qpainter::QPainter; // 771
use super::super::core::qsize::QSize; // 771
use super::super::core::qcoreevent::QEvent; // 771
use super::super::gui::qevent::QHelpEvent; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QAbstractItemDelegate_Class_Size() -> c_int;
  // proto:  void QAbstractItemDelegate::commitData(QWidget * editor);
  fn _ZN21QAbstractItemDelegate10commitDataEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QVector<int> QAbstractItemDelegate::paintingRoles();
  fn _ZNK21QAbstractItemDelegate13paintingRolesEv(qthis: *mut c_void);
  // proto:  void QAbstractItemDelegate::sizeHintChanged(const QModelIndex & );
  fn _ZN21QAbstractItemDelegate15sizeHintChangedERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QAbstractItemDelegate::metaObject();
  fn _ZNK21QAbstractItemDelegate10metaObjectEv(qthis: *mut c_void);
  // proto:  void QAbstractItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK21QAbstractItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QAbstractItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
  fn _ZNK21QAbstractItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QAbstractItemDelegate::QAbstractItemDelegate(QObject * parent);
  fn dector_ZN21QAbstractItemDelegateC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN21QAbstractItemDelegateC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QWidget * QAbstractItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK21QAbstractItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK21QAbstractItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QSize QAbstractItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK21QAbstractItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractItemDelegate::~QAbstractItemDelegate();
  fn _ZN21QAbstractItemDelegateD0Ev(qthis: *mut c_void);
  // proto:  void QAbstractItemDelegate::destroyEditor(QWidget * editor, const QModelIndex & index);
  fn _ZNK21QAbstractItemDelegate13destroyEditorEP7QWidgetRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractItemDelegate)=1
pub struct QAbstractItemDelegate {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAbstractItemDelegate {
  pub fn inheritFrom(qthis: *mut c_void) -> QAbstractItemDelegate {
    return QAbstractItemDelegate{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QAbstractItemDelegate {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QAbstractItemDelegate {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QAbstractItemDelegate::commitData(QWidget * editor);
impl /*struct*/ QAbstractItemDelegate {
  pub fn commitData<RetType, T: QAbstractItemDelegate_commitData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.commitData(self);
    // return 1;
  }
}

pub trait QAbstractItemDelegate_commitData<RetType> {
  fn commitData(self , rsthis: & QAbstractItemDelegate) -> RetType;
}

  // proto:  void QAbstractItemDelegate::commitData(QWidget * editor);
impl<'a> /*trait*/ QAbstractItemDelegate_commitData<()> for (&'a QWidget) {
  fn commitData(self , rsthis: & QAbstractItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QAbstractItemDelegate10commitDataEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QAbstractItemDelegate10commitDataEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVector<int> QAbstractItemDelegate::paintingRoles();
impl /*struct*/ QAbstractItemDelegate {
  pub fn paintingRoles<RetType, T: QAbstractItemDelegate_paintingRoles<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paintingRoles(self);
    // return 1;
  }
}

pub trait QAbstractItemDelegate_paintingRoles<RetType> {
  fn paintingRoles(self , rsthis: & QAbstractItemDelegate) -> RetType;
}

  // proto:  QVector<int> QAbstractItemDelegate::paintingRoles();
impl<'a> /*trait*/ QAbstractItemDelegate_paintingRoles<()> for () {
  fn paintingRoles(self , rsthis: & QAbstractItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QAbstractItemDelegate13paintingRolesEv()};
     unsafe {_ZNK21QAbstractItemDelegate13paintingRolesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractItemDelegate::sizeHintChanged(const QModelIndex & );
impl /*struct*/ QAbstractItemDelegate {
  pub fn sizeHintChanged<RetType, T: QAbstractItemDelegate_sizeHintChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHintChanged(self);
    // return 1;
  }
}

pub trait QAbstractItemDelegate_sizeHintChanged<RetType> {
  fn sizeHintChanged(self , rsthis: & QAbstractItemDelegate) -> RetType;
}

  // proto:  void QAbstractItemDelegate::sizeHintChanged(const QModelIndex & );
impl<'a> /*trait*/ QAbstractItemDelegate_sizeHintChanged<()> for (&'a QModelIndex) {
  fn sizeHintChanged(self , rsthis: & QAbstractItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QAbstractItemDelegate15sizeHintChangedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QAbstractItemDelegate15sizeHintChangedERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractItemDelegate::metaObject();
impl /*struct*/ QAbstractItemDelegate {
  pub fn metaObject<RetType, T: QAbstractItemDelegate_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractItemDelegate_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAbstractItemDelegate) -> RetType;
}

  // proto:  const QMetaObject * QAbstractItemDelegate::metaObject();
impl<'a> /*trait*/ QAbstractItemDelegate_metaObject<()> for () {
  fn metaObject(self , rsthis: & QAbstractItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QAbstractItemDelegate10metaObjectEv()};
     unsafe {_ZNK21QAbstractItemDelegate10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QAbstractItemDelegate {
  pub fn updateEditorGeometry<RetType, T: QAbstractItemDelegate_updateEditorGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.updateEditorGeometry(self);
    // return 1;
  }
}

pub trait QAbstractItemDelegate_updateEditorGeometry<RetType> {
  fn updateEditorGeometry(self , rsthis: & QAbstractItemDelegate) -> RetType;
}

  // proto:  void QAbstractItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemDelegate_updateEditorGeometry<()> for (&'a QWidget, &'a QStyleOptionViewItem, &'a QModelIndex) {
  fn updateEditorGeometry(self , rsthis: & QAbstractItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QAbstractItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZNK21QAbstractItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QAbstractItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
impl /*struct*/ QAbstractItemDelegate {
  pub fn setEditorData<RetType, T: QAbstractItemDelegate_setEditorData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEditorData(self);
    // return 1;
  }
}

pub trait QAbstractItemDelegate_setEditorData<RetType> {
  fn setEditorData(self , rsthis: & QAbstractItemDelegate) -> RetType;
}

  // proto:  void QAbstractItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemDelegate_setEditorData<()> for (&'a QWidget, &'a QModelIndex) {
  fn setEditorData(self , rsthis: & QAbstractItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QAbstractItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZNK21QAbstractItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QAbstractItemDelegate::QAbstractItemDelegate(QObject * parent);
impl /*struct*/ QAbstractItemDelegate {
  pub fn New<T: QAbstractItemDelegate_New>(value: T) -> QAbstractItemDelegate {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractItemDelegate_New {
  fn New(self) -> QAbstractItemDelegate;
}

  // proto:  void QAbstractItemDelegate::QAbstractItemDelegate(QObject * parent);
impl<'a> /*trait*/ QAbstractItemDelegate_New for (&'a QObject) {
  fn New(self) -> QAbstractItemDelegate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QAbstractItemDelegateC1EP7QObject()};
    let ctysz: c_int = unsafe{QAbstractItemDelegate_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN21QAbstractItemDelegateC1EP7QObject(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN21QAbstractItemDelegateC1EP7QObject(arg0)};
    let rsthis = QAbstractItemDelegate{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QWidget * QAbstractItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QAbstractItemDelegate {
  pub fn createEditor<RetType, T: QAbstractItemDelegate_createEditor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createEditor(self);
    // return 1;
  }
}

pub trait QAbstractItemDelegate_createEditor<RetType> {
  fn createEditor(self , rsthis: & QAbstractItemDelegate) -> RetType;
}

  // proto:  QWidget * QAbstractItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemDelegate_createEditor<QWidget> for (&'a QWidget, &'a QStyleOptionViewItem, &'a QModelIndex) {
  fn createEditor(self , rsthis: & QAbstractItemDelegate) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QAbstractItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QAbstractItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QAbstractItemDelegate {
  pub fn paint<RetType, T: QAbstractItemDelegate_paint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QAbstractItemDelegate_paint<RetType> {
  fn paint(self , rsthis: & QAbstractItemDelegate) -> RetType;
}

  // proto:  void QAbstractItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemDelegate_paint<()> for (&'a QPainter, &'a QStyleOptionViewItem, &'a QModelIndex) {
  fn paint(self , rsthis: & QAbstractItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QAbstractItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZNK21QAbstractItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QSize QAbstractItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QAbstractItemDelegate {
  pub fn sizeHint<RetType, T: QAbstractItemDelegate_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QAbstractItemDelegate_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QAbstractItemDelegate) -> RetType;
}

  // proto:  QSize QAbstractItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemDelegate_sizeHint<QSize> for (&'a QStyleOptionViewItem, &'a QModelIndex) {
  fn sizeHint(self , rsthis: & QAbstractItemDelegate) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QAbstractItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QAbstractItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractItemDelegate::~QAbstractItemDelegate();
impl /*struct*/ QAbstractItemDelegate {
  pub fn Free<RetType, T: QAbstractItemDelegate_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QAbstractItemDelegate_Free<RetType> {
  fn Free(self , rsthis: & QAbstractItemDelegate) -> RetType;
}

  // proto:  void QAbstractItemDelegate::~QAbstractItemDelegate();
impl<'a> /*trait*/ QAbstractItemDelegate_Free<()> for () {
  fn Free(self , rsthis: & QAbstractItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QAbstractItemDelegateD0Ev()};
     unsafe {_ZN21QAbstractItemDelegateD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractItemDelegate::destroyEditor(QWidget * editor, const QModelIndex & index);
impl /*struct*/ QAbstractItemDelegate {
  pub fn destroyEditor<RetType, T: QAbstractItemDelegate_destroyEditor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.destroyEditor(self);
    // return 1;
  }
}

pub trait QAbstractItemDelegate_destroyEditor<RetType> {
  fn destroyEditor(self , rsthis: & QAbstractItemDelegate) -> RetType;
}

  // proto:  void QAbstractItemDelegate::destroyEditor(QWidget * editor, const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemDelegate_destroyEditor<()> for (&'a QWidget, &'a QModelIndex) {
  fn destroyEditor(self , rsthis: & QAbstractItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QAbstractItemDelegate13destroyEditorEP7QWidgetRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZNK21QAbstractItemDelegate13destroyEditorEP7QWidgetRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// <= body block end

