// auto generated, do not modify.
// created: Sun Dec 27 22:52:02 2015
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
use super::super::core::qabstractitemmodel::QAbstractItemModel; // 771
use super::super::gui::qevent::QHelpEvent; // 771
use super::qabstractitemview::QAbstractItemView; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QAbstractItemDelegate_Class_Size() -> c_int;
  // proto:  void QAbstractItemDelegate::commitData(QWidget * editor);
  fn _ZN21QAbstractItemDelegate10commitDataEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QVector<int> QAbstractItemDelegate::paintingRoles();
  fn _ZNK21QAbstractItemDelegate13paintingRolesEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractItemDelegate::sizeHintChanged(const QModelIndex & );
  fn _ZN21QAbstractItemDelegate15sizeHintChangedERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QAbstractItemDelegate::metaObject();
  fn _ZNK21QAbstractItemDelegate10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK21QAbstractItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QAbstractItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
  fn _ZNK21QAbstractItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QAbstractItemDelegate::QAbstractItemDelegate(QObject * parent);
  fn dector_ZN21QAbstractItemDelegateC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN21QAbstractItemDelegateC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWidget * QAbstractItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK21QAbstractItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK21QAbstractItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QAbstractItemDelegate::QAbstractItemDelegate(const QAbstractItemDelegate & );
  fn dector_ZN21QAbstractItemDelegateC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN21QAbstractItemDelegateC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSize QAbstractItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK21QAbstractItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  bool QAbstractItemDelegate::editorEvent(QEvent * event, QAbstractItemModel * model, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZN21QAbstractItemDelegate11editorEventEP6QEventP18QAbstractItemModelRK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> c_char;
  // proto:  bool QAbstractItemDelegate::helpEvent(QHelpEvent * event, QAbstractItemView * view, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZN21QAbstractItemDelegate9helpEventEP10QHelpEventP17QAbstractItemViewRK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> c_char;
  // proto:  void QAbstractItemDelegate::~QAbstractItemDelegate();
  fn _ZN21QAbstractItemDelegateD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractItemDelegate::setModelData(QWidget * editor, QAbstractItemModel * model, const QModelIndex & index);
  fn _ZNK21QAbstractItemDelegate12setModelDataEP7QWidgetP18QAbstractItemModelRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QAbstractItemDelegate::destroyEditor(QWidget * editor, const QModelIndex & index);
  fn _ZNK21QAbstractItemDelegate13destroyEditorEP7QWidgetRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  fn QAbstractItemDelegate_SlotProxy_connect__ZN21QAbstractItemDelegate11closeEditorEP7QWidgetNS_11EndEditHintE(qthis: *mut c_void, fptr: *mut c_void);
  fn QAbstractItemDelegate_SlotProxy_connect__ZN21QAbstractItemDelegate15sizeHintChangedERK11QModelIndex(qthis: *mut c_void, fptr: *mut c_void);
  fn QAbstractItemDelegate_SlotProxy_connect__ZN21QAbstractItemDelegate10commitDataEP7QWidget(qthis: *mut c_void, fptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractItemDelegate)=1
#[derive(Default)]
pub struct QAbstractItemDelegate {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _closeEditor_1: QAbstractItemDelegate_closeEditor_signal,
  pub _commitData_1: QAbstractItemDelegate_commitData_signal,
  pub _sizeHintChanged_1: QAbstractItemDelegate_sizeHintChanged_signal,
}

impl /*struct*/ QAbstractItemDelegate {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractItemDelegate {
    return QAbstractItemDelegate{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN21QAbstractItemDelegateC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN21QAbstractItemDelegateC1EP7QObject(arg0)} as u64;
    let rsthis = QAbstractItemDelegate{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret1 = QWidget::inheritFrom(ret as u64);
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

  // proto:  void QAbstractItemDelegate::QAbstractItemDelegate(const QAbstractItemDelegate & );
impl<'a> /*trait*/ QAbstractItemDelegate_New for (&'a QAbstractItemDelegate) {
  fn New(self) -> QAbstractItemDelegate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QAbstractItemDelegateC1ERKS_()};
    let ctysz: c_int = unsafe{QAbstractItemDelegate_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN21QAbstractItemDelegateC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN21QAbstractItemDelegateC1ERKS_(arg0)} as u64;
    let rsthis = QAbstractItemDelegate{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
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
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QAbstractItemDelegate::editorEvent(QEvent * event, QAbstractItemModel * model, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QAbstractItemDelegate {
  pub fn editorEvent<RetType, T: QAbstractItemDelegate_editorEvent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.editorEvent(self);
    // return 1;
  }
}

pub trait QAbstractItemDelegate_editorEvent<RetType> {
  fn editorEvent(self , rsthis: & QAbstractItemDelegate) -> RetType;
}

  // proto:  bool QAbstractItemDelegate::editorEvent(QEvent * event, QAbstractItemModel * model, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemDelegate_editorEvent<i8> for (&'a QEvent, &'a QAbstractItemModel, &'a QStyleOptionViewItem, &'a QModelIndex) {
  fn editorEvent(self , rsthis: & QAbstractItemDelegate) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QAbstractItemDelegate11editorEventEP6QEventP18QAbstractItemModelRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QAbstractItemDelegate11editorEventEP6QEventP18QAbstractItemModelRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAbstractItemDelegate::helpEvent(QHelpEvent * event, QAbstractItemView * view, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QAbstractItemDelegate {
  pub fn helpEvent<RetType, T: QAbstractItemDelegate_helpEvent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.helpEvent(self);
    // return 1;
  }
}

pub trait QAbstractItemDelegate_helpEvent<RetType> {
  fn helpEvent(self , rsthis: & QAbstractItemDelegate) -> RetType;
}

  // proto:  bool QAbstractItemDelegate::helpEvent(QHelpEvent * event, QAbstractItemView * view, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemDelegate_helpEvent<i8> for (&'a QHelpEvent, &'a QAbstractItemView, &'a QStyleOptionViewItem, &'a QModelIndex) {
  fn helpEvent(self , rsthis: & QAbstractItemDelegate) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QAbstractItemDelegate9helpEventEP10QHelpEventP17QAbstractItemViewRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QAbstractItemDelegate9helpEventEP10QHelpEventP17QAbstractItemViewRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i8;
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

  // proto:  void QAbstractItemDelegate::setModelData(QWidget * editor, QAbstractItemModel * model, const QModelIndex & index);
impl /*struct*/ QAbstractItemDelegate {
  pub fn setModelData<RetType, T: QAbstractItemDelegate_setModelData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setModelData(self);
    // return 1;
  }
}

pub trait QAbstractItemDelegate_setModelData<RetType> {
  fn setModelData(self , rsthis: & QAbstractItemDelegate) -> RetType;
}

  // proto:  void QAbstractItemDelegate::setModelData(QWidget * editor, QAbstractItemModel * model, const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemDelegate_setModelData<()> for (&'a QWidget, &'a QAbstractItemModel, &'a QModelIndex) {
  fn setModelData(self , rsthis: & QAbstractItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QAbstractItemDelegate12setModelDataEP7QWidgetP18QAbstractItemModelRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZNK21QAbstractItemDelegate12setModelDataEP7QWidgetP18QAbstractItemModelRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
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

#[derive(Default)] // for QAbstractItemDelegate_closeEditor
pub struct QAbstractItemDelegate_closeEditor_signal{poi:u64}
impl /* struct */ QAbstractItemDelegate {
  pub fn closeEditor_1(self) -> QAbstractItemDelegate_closeEditor_signal {
     return QAbstractItemDelegate_closeEditor_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemDelegate_closeEditor_signal {
  pub fn connect<T: QAbstractItemDelegate_closeEditor_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemDelegate_closeEditor_signal_connect {
  fn connect(self, sigthis: QAbstractItemDelegate_closeEditor_signal);
}

#[derive(Default)] // for QAbstractItemDelegate_commitData
pub struct QAbstractItemDelegate_commitData_signal{poi:u64}
impl /* struct */ QAbstractItemDelegate {
  pub fn commitData_1(self) -> QAbstractItemDelegate_commitData_signal {
     return QAbstractItemDelegate_commitData_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemDelegate_commitData_signal {
  pub fn connect<T: QAbstractItemDelegate_commitData_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemDelegate_commitData_signal_connect {
  fn connect(self, sigthis: QAbstractItemDelegate_commitData_signal);
}

#[derive(Default)] // for QAbstractItemDelegate_sizeHintChanged
pub struct QAbstractItemDelegate_sizeHintChanged_signal{poi:u64}
impl /* struct */ QAbstractItemDelegate {
  pub fn sizeHintChanged_1(self) -> QAbstractItemDelegate_sizeHintChanged_signal {
     return QAbstractItemDelegate_sizeHintChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemDelegate_sizeHintChanged_signal {
  pub fn connect<T: QAbstractItemDelegate_sizeHintChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemDelegate_sizeHintChanged_signal_connect {
  fn connect(self, sigthis: QAbstractItemDelegate_sizeHintChanged_signal);
}

// closeEditor(class QWidget *, class QAbstractItemDelegate::EndEditHint)
extern fn QAbstractItemDelegate_closeEditor_signal_connect_cb_0(arg0: *mut c_void, arg1: c_int) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QAbstractItemDelegate_closeEditor_signal_connect for (extern fn(QWidget, i32)) {
  fn connect(self, sigthis: QAbstractItemDelegate_closeEditor_signal) {
    // do smth...
    unsafe {QAbstractItemDelegate_SlotProxy_connect__ZN21QAbstractItemDelegate11closeEditorEP7QWidgetNS_11EndEditHintE(sigthis.poi as *mut c_void, QAbstractItemDelegate_closeEditor_signal_connect_cb_0 as *mut c_void)};
  }
}
// sizeHintChanged(const class QModelIndex &)
extern fn QAbstractItemDelegate_sizeHintChanged_signal_connect_cb_1(arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QAbstractItemDelegate_sizeHintChanged_signal_connect for (extern fn(QModelIndex)) {
  fn connect(self, sigthis: QAbstractItemDelegate_sizeHintChanged_signal) {
    // do smth...
    unsafe {QAbstractItemDelegate_SlotProxy_connect__ZN21QAbstractItemDelegate15sizeHintChangedERK11QModelIndex(sigthis.poi as *mut c_void, QAbstractItemDelegate_sizeHintChanged_signal_connect_cb_1 as *mut c_void)};
  }
}
// commitData(class QWidget *)
extern fn QAbstractItemDelegate_commitData_signal_connect_cb_2(arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QAbstractItemDelegate_commitData_signal_connect for (extern fn(QWidget)) {
  fn connect(self, sigthis: QAbstractItemDelegate_commitData_signal) {
    // do smth...
    unsafe {QAbstractItemDelegate_SlotProxy_connect__ZN21QAbstractItemDelegate10commitDataEP7QWidget(sigthis.poi as *mut c_void, QAbstractItemDelegate_commitData_signal_connect_cb_2 as *mut c_void)};
  }
}
// <= body block end

