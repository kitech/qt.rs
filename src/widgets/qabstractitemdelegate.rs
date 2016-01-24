// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
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
use super::super::core::qobjectdefs::QMetaObject; // 771
use super::qwidget::QWidget; // 773
use super::qstyleoption::QStyleOptionViewItem; // 773
use super::super::core::qabstractitemmodel::QModelIndex; // 771
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
  // proto:  QVector<int> QAbstractItemDelegate::paintingRoles();
  fn C_ZNK21QAbstractItemDelegate13paintingRolesEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QAbstractItemDelegate::metaObject();
  fn C_ZNK21QAbstractItemDelegate10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAbstractItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn C_ZNK21QAbstractItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QAbstractItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
  fn C_ZNK21QAbstractItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QAbstractItemDelegate::QAbstractItemDelegate(QObject * parent);
  fn C_ZN21QAbstractItemDelegateC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  QWidget * QAbstractItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn C_ZNK21QAbstractItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn C_ZNK21QAbstractItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QSize QAbstractItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
  fn C_ZNK21QAbstractItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  bool QAbstractItemDelegate::editorEvent(QEvent * event, QAbstractItemModel * model, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn C_ZN21QAbstractItemDelegate11editorEventEP6QEventP18QAbstractItemModelRK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> c_char;
  // proto:  bool QAbstractItemDelegate::helpEvent(QHelpEvent * event, QAbstractItemView * view, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn C_ZN21QAbstractItemDelegate9helpEventEP10QHelpEventP17QAbstractItemViewRK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> c_char;
  // proto:  void QAbstractItemDelegate::~QAbstractItemDelegate();
  fn C_ZN21QAbstractItemDelegateD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractItemDelegate::setModelData(QWidget * editor, QAbstractItemModel * model, const QModelIndex & index);
  fn C_ZNK21QAbstractItemDelegate12setModelDataEP7QWidgetP18QAbstractItemModelRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QAbstractItemDelegate::destroyEditor(QWidget * editor, const QModelIndex & index);
  fn C_ZNK21QAbstractItemDelegate13destroyEditorEP7QWidgetRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  fn QAbstractItemDelegate_SlotProxy_connect__ZN21QAbstractItemDelegate15sizeHintChangedERK11QModelIndex(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractItemDelegate_SlotProxy_connect__ZN21QAbstractItemDelegate10commitDataEP7QWidget(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractItemDelegate)=1
#[derive(Default)]
pub struct QAbstractItemDelegate {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _closeEditor: QAbstractItemDelegate_closeEditor_signal,
  pub _commitData: QAbstractItemDelegate_commitData_signal,
  pub _sizeHintChanged: QAbstractItemDelegate_sizeHintChanged_signal,
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
     unsafe {C_ZNK21QAbstractItemDelegate13paintingRolesEv(rsthis.qclsinst)};
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
impl<'a> /*trait*/ QAbstractItemDelegate_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QAbstractItemDelegate) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QAbstractItemDelegate10metaObjectEv()};
    let mut ret = unsafe {C_ZNK21QAbstractItemDelegate10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
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
     unsafe {C_ZNK21QAbstractItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
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
     unsafe {C_ZNK21QAbstractItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QAbstractItemDelegate::QAbstractItemDelegate(QObject * parent);
impl /*struct*/ QAbstractItemDelegate {
  pub fn new<T: QAbstractItemDelegate_new>(value: T) -> QAbstractItemDelegate {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractItemDelegate_new {
  fn new(self) -> QAbstractItemDelegate;
}

  // proto:  void QAbstractItemDelegate::QAbstractItemDelegate(QObject * parent);
impl<'a> /*trait*/ QAbstractItemDelegate_new for (&'a QObject) {
  fn new(self) -> QAbstractItemDelegate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QAbstractItemDelegateC2EP7QObject()};
    let ctysz: c_int = unsafe{QAbstractItemDelegate_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN21QAbstractItemDelegateC2EP7QObject(arg0)};
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
    let mut ret = unsafe {C_ZNK21QAbstractItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
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
     unsafe {C_ZNK21QAbstractItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
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
    let mut ret = unsafe {C_ZNK21QAbstractItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
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
    let mut ret = unsafe {C_ZN21QAbstractItemDelegate11editorEventEP6QEventP18QAbstractItemModelRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
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
    let mut ret = unsafe {C_ZN21QAbstractItemDelegate9helpEventEP10QHelpEventP17QAbstractItemViewRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractItemDelegate::~QAbstractItemDelegate();
impl /*struct*/ QAbstractItemDelegate {
  pub fn free<RetType, T: QAbstractItemDelegate_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QAbstractItemDelegate_free<RetType> {
  fn free(self , rsthis: & QAbstractItemDelegate) -> RetType;
}

  // proto:  void QAbstractItemDelegate::~QAbstractItemDelegate();
impl<'a> /*trait*/ QAbstractItemDelegate_free<()> for () {
  fn free(self , rsthis: & QAbstractItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QAbstractItemDelegateD2Ev()};
     unsafe {C_ZN21QAbstractItemDelegateD2Ev(rsthis.qclsinst)};
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
     unsafe {C_ZNK21QAbstractItemDelegate12setModelDataEP7QWidgetP18QAbstractItemModelRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
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
     unsafe {C_ZNK21QAbstractItemDelegate13destroyEditorEP7QWidgetRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

#[derive(Default)] // for QAbstractItemDelegate_closeEditor
pub struct QAbstractItemDelegate_closeEditor_signal{poi:u64}
impl /* struct */ QAbstractItemDelegate {
  pub fn closeEditor(&self) -> QAbstractItemDelegate_closeEditor_signal {
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
  pub fn commitData(&self) -> QAbstractItemDelegate_commitData_signal {
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
  pub fn sizeHintChanged(&self) -> QAbstractItemDelegate_sizeHintChanged_signal {
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

// sizeHintChanged(const class QModelIndex &)
extern fn QAbstractItemDelegate_sizeHintChanged_signal_connect_cb_0(rsfptr:fn(QModelIndex), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QModelIndex::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QAbstractItemDelegate_sizeHintChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QModelIndex)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QModelIndex::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QAbstractItemDelegate_sizeHintChanged_signal_connect for fn(QModelIndex) {
  fn connect(self, sigthis: QAbstractItemDelegate_sizeHintChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractItemDelegate_sizeHintChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractItemDelegate_SlotProxy_connect__ZN21QAbstractItemDelegate15sizeHintChangedERK11QModelIndex(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractItemDelegate_sizeHintChanged_signal_connect for Box<Fn(QModelIndex)> {
  fn connect(self, sigthis: QAbstractItemDelegate_sizeHintChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractItemDelegate_sizeHintChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractItemDelegate_SlotProxy_connect__ZN21QAbstractItemDelegate15sizeHintChangedERK11QModelIndex(arg0, arg1, arg2)};
  }
}
// commitData(class QWidget *)
extern fn QAbstractItemDelegate_commitData_signal_connect_cb_1(rsfptr:fn(QWidget), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QWidget::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QAbstractItemDelegate_commitData_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QWidget)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QWidget::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QAbstractItemDelegate_commitData_signal_connect for fn(QWidget) {
  fn connect(self, sigthis: QAbstractItemDelegate_commitData_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractItemDelegate_commitData_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractItemDelegate_SlotProxy_connect__ZN21QAbstractItemDelegate10commitDataEP7QWidget(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractItemDelegate_commitData_signal_connect for Box<Fn(QWidget)> {
  fn connect(self, sigthis: QAbstractItemDelegate_commitData_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractItemDelegate_commitData_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractItemDelegate_SlotProxy_connect__ZN21QAbstractItemDelegate10commitDataEP7QWidget(arg0, arg1, arg2)};
  }
}
// <= body block end

