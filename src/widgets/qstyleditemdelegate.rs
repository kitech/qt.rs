// auto generated, do not modify.
// created: Fri Jan  1 15:54:32 2016
// src-file: /QtWidgets/qstyleditemdelegate.h
// dst-file: /src/widgets/qstyleditemdelegate.rs
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
use super::qabstractitemdelegate::QAbstractItemDelegate; // 773
use std::ops::Deref;
use super::qstyleoption::QStyleOptionViewItem; // 773
use super::super::core::qabstractitemmodel::QModelIndex; // 771
use super::super::core::qsize::QSize; // 771
use super::qwidget::QWidget; // 773
use super::super::core::qabstractitemmodel::QAbstractItemModel; // 771
use super::qitemeditorfactory::QItemEditorFactory; // 773
use super::super::gui::qpainter::QPainter; // 771
use super::super::core::qobject::QObject; // 771
use super::super::core::qvariant::QVariant; // 771
use super::super::core::qlocale::QLocale; // 771
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStyledItemDelegate_Class_Size() -> c_int;
  // proto:  QSize QStyledItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QStyledItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QStyledItemDelegate::~QStyledItemDelegate();
  fn _ZN19QStyledItemDelegateD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QStyledItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QStyledItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QStyledItemDelegate::setModelData(QWidget * editor, QAbstractItemModel * model, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate12setModelDataEP7QWidgetP18QAbstractItemModelRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QStyledItemDelegate::QStyledItemDelegate(const QStyledItemDelegate & );
  fn dector_ZN19QStyledItemDelegateC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QStyledItemDelegateC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStyledItemDelegate::setItemEditorFactory(QItemEditorFactory * factory);
  fn _ZN19QStyledItemDelegate20setItemEditorFactoryEP18QItemEditorFactory(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStyledItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QItemEditorFactory * QStyledItemDelegate::itemEditorFactory();
  fn _ZNK19QStyledItemDelegate17itemEditorFactoryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStyledItemDelegate::QStyledItemDelegate(QObject * parent);
  fn dector_ZN19QStyledItemDelegateC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QStyledItemDelegateC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QStyledItemDelegate::displayText(const QVariant & value, const QLocale & locale);
  fn _ZNK19QStyledItemDelegate11displayTextERK8QVariantRK7QLocale(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QStyledItemDelegate::metaObject();
  fn _ZNK19QStyledItemDelegate10metaObjectEv(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QStyledItemDelegate)=1
#[derive(Default)]
pub struct QStyledItemDelegate {
  qbase: QAbstractItemDelegate,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QStyledItemDelegate {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStyledItemDelegate {
    return QStyledItemDelegate{qbase: QAbstractItemDelegate::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QStyledItemDelegate {
  type Target = QAbstractItemDelegate;

  fn deref(&self) -> &QAbstractItemDelegate {
    return & self.qbase;
  }
}
impl AsRef<QAbstractItemDelegate> for QStyledItemDelegate {
  fn as_ref(& self) -> & QAbstractItemDelegate {
    return & self.qbase;
  }
}
  // proto:  QSize QStyledItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QStyledItemDelegate {
  pub fn sizeHint<RetType, T: QStyledItemDelegate_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QStyledItemDelegate) -> RetType;
}

  // proto:  QSize QStyledItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_sizeHint<QSize> for (&'a QStyleOptionViewItem, &'a QModelIndex) {
  fn sizeHint(self , rsthis: & QStyledItemDelegate) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QStyledItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QStyledItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QStyledItemDelegate {
  pub fn createEditor<RetType, T: QStyledItemDelegate_createEditor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createEditor(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_createEditor<RetType> {
  fn createEditor(self , rsthis: & QStyledItemDelegate) -> RetType;
}

  // proto:  QWidget * QStyledItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_createEditor<QWidget> for (&'a QWidget, &'a QStyleOptionViewItem, &'a QModelIndex) {
  fn createEditor(self , rsthis: & QStyledItemDelegate) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QStyledItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStyledItemDelegate::~QStyledItemDelegate();
impl /*struct*/ QStyledItemDelegate {
  pub fn free<RetType, T: QStyledItemDelegate_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_free<RetType> {
  fn free(self , rsthis: & QStyledItemDelegate) -> RetType;
}

  // proto:  void QStyledItemDelegate::~QStyledItemDelegate();
impl<'a> /*trait*/ QStyledItemDelegate_free<()> for () {
  fn free(self , rsthis: & QStyledItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyledItemDelegateD0Ev()};
     unsafe {_ZN19QStyledItemDelegateD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStyledItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QStyledItemDelegate {
  pub fn updateEditorGeometry<RetType, T: QStyledItemDelegate_updateEditorGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.updateEditorGeometry(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_updateEditorGeometry<RetType> {
  fn updateEditorGeometry(self , rsthis: & QStyledItemDelegate) -> RetType;
}

  // proto:  void QStyledItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_updateEditorGeometry<()> for (&'a QWidget, &'a QStyleOptionViewItem, &'a QModelIndex) {
  fn updateEditorGeometry(self , rsthis: & QStyledItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZNK19QStyledItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QStyledItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
impl /*struct*/ QStyledItemDelegate {
  pub fn setEditorData<RetType, T: QStyledItemDelegate_setEditorData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEditorData(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_setEditorData<RetType> {
  fn setEditorData(self , rsthis: & QStyledItemDelegate) -> RetType;
}

  // proto:  void QStyledItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_setEditorData<()> for (&'a QWidget, &'a QModelIndex) {
  fn setEditorData(self , rsthis: & QStyledItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZNK19QStyledItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QStyledItemDelegate::setModelData(QWidget * editor, QAbstractItemModel * model, const QModelIndex & index);
impl /*struct*/ QStyledItemDelegate {
  pub fn setModelData<RetType, T: QStyledItemDelegate_setModelData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setModelData(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_setModelData<RetType> {
  fn setModelData(self , rsthis: & QStyledItemDelegate) -> RetType;
}

  // proto:  void QStyledItemDelegate::setModelData(QWidget * editor, QAbstractItemModel * model, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_setModelData<()> for (&'a QWidget, &'a QAbstractItemModel, &'a QModelIndex) {
  fn setModelData(self , rsthis: & QStyledItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate12setModelDataEP7QWidgetP18QAbstractItemModelRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZNK19QStyledItemDelegate12setModelDataEP7QWidgetP18QAbstractItemModelRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QStyledItemDelegate::QStyledItemDelegate(const QStyledItemDelegate & );
impl /*struct*/ QStyledItemDelegate {
  pub fn new<T: QStyledItemDelegate_new>(value: T) -> QStyledItemDelegate {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStyledItemDelegate_new {
  fn new(self) -> QStyledItemDelegate;
}

  // proto:  void QStyledItemDelegate::QStyledItemDelegate(const QStyledItemDelegate & );
impl<'a> /*trait*/ QStyledItemDelegate_new for (&'a QStyledItemDelegate) {
  fn new(self) -> QStyledItemDelegate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyledItemDelegateC1ERKS_()};
    let ctysz: c_int = unsafe{QStyledItemDelegate_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QStyledItemDelegateC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN19QStyledItemDelegateC1ERKS_(arg0)} as u64;
    let rsthis = QStyledItemDelegate{qbase: QAbstractItemDelegate::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyledItemDelegate::setItemEditorFactory(QItemEditorFactory * factory);
impl /*struct*/ QStyledItemDelegate {
  pub fn setItemEditorFactory<RetType, T: QStyledItemDelegate_setItemEditorFactory<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemEditorFactory(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_setItemEditorFactory<RetType> {
  fn setItemEditorFactory(self , rsthis: & QStyledItemDelegate) -> RetType;
}

  // proto:  void QStyledItemDelegate::setItemEditorFactory(QItemEditorFactory * factory);
impl<'a> /*trait*/ QStyledItemDelegate_setItemEditorFactory<()> for (&'a QItemEditorFactory) {
  fn setItemEditorFactory(self , rsthis: & QStyledItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyledItemDelegate20setItemEditorFactoryEP18QItemEditorFactory()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QStyledItemDelegate20setItemEditorFactoryEP18QItemEditorFactory(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStyledItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QStyledItemDelegate {
  pub fn paint<RetType, T: QStyledItemDelegate_paint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_paint<RetType> {
  fn paint(self , rsthis: & QStyledItemDelegate) -> RetType;
}

  // proto:  void QStyledItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_paint<()> for (&'a QPainter, &'a QStyleOptionViewItem, &'a QModelIndex) {
  fn paint(self , rsthis: & QStyledItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZNK19QStyledItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QItemEditorFactory * QStyledItemDelegate::itemEditorFactory();
impl /*struct*/ QStyledItemDelegate {
  pub fn itemEditorFactory<RetType, T: QStyledItemDelegate_itemEditorFactory<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemEditorFactory(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_itemEditorFactory<RetType> {
  fn itemEditorFactory(self , rsthis: & QStyledItemDelegate) -> RetType;
}

  // proto:  QItemEditorFactory * QStyledItemDelegate::itemEditorFactory();
impl<'a> /*trait*/ QStyledItemDelegate_itemEditorFactory<QItemEditorFactory> for () {
  fn itemEditorFactory(self , rsthis: & QStyledItemDelegate) -> QItemEditorFactory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate17itemEditorFactoryEv()};
    let mut ret = unsafe {_ZNK19QStyledItemDelegate17itemEditorFactoryEv(rsthis.qclsinst)};
    let mut ret1 = QItemEditorFactory::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStyledItemDelegate::QStyledItemDelegate(QObject * parent);
impl<'a> /*trait*/ QStyledItemDelegate_new for (&'a QObject) {
  fn new(self) -> QStyledItemDelegate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyledItemDelegateC1EP7QObject()};
    let ctysz: c_int = unsafe{QStyledItemDelegate_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QStyledItemDelegateC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN19QStyledItemDelegateC1EP7QObject(arg0)} as u64;
    let rsthis = QStyledItemDelegate{qbase: QAbstractItemDelegate::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QStyledItemDelegate::displayText(const QVariant & value, const QLocale & locale);
impl /*struct*/ QStyledItemDelegate {
  pub fn displayText<RetType, T: QStyledItemDelegate_displayText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.displayText(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_displayText<RetType> {
  fn displayText(self , rsthis: & QStyledItemDelegate) -> RetType;
}

  // proto:  QString QStyledItemDelegate::displayText(const QVariant & value, const QLocale & locale);
impl<'a> /*trait*/ QStyledItemDelegate_displayText<QString> for (&'a QVariant, &'a QLocale) {
  fn displayText(self , rsthis: & QStyledItemDelegate) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate11displayTextERK8QVariantRK7QLocale()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QStyledItemDelegate11displayTextERK8QVariantRK7QLocale(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QStyledItemDelegate::metaObject();
impl /*struct*/ QStyledItemDelegate {
  pub fn metaObject<RetType, T: QStyledItemDelegate_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_metaObject<RetType> {
  fn metaObject(self , rsthis: & QStyledItemDelegate) -> RetType;
}

  // proto:  const QMetaObject * QStyledItemDelegate::metaObject();
impl<'a> /*trait*/ QStyledItemDelegate_metaObject<()> for () {
  fn metaObject(self , rsthis: & QStyledItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate10metaObjectEv()};
     unsafe {_ZNK19QStyledItemDelegate10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

