// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qitemdelegate.h
// dst-file: /src/widgets/qitemdelegate.rs
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
use super::qabstractitemdelegate::*; // 773
use std::ops::Deref;
use super::super::core::qobjectdefs::*; // 771
use super::qstyleoption::*; // 773
use super::super::core::qabstractitemmodel::*; // 771
use super::super::core::qsize::*; // 771
use super::qwidget::*; // 773
use super::qitemeditorfactory::*; // 773
use super::super::core::qobject::*; // 771
use super::super::gui::qpainter::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QItemDelegate_Class_Size() -> c_int;
  // proto:  const QMetaObject * QItemDelegate::metaObject();
  fn C_ZNK13QItemDelegate10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
  fn C_ZNK13QItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QItemDelegate::setModelData(QWidget * editor, QAbstractItemModel * model, const QModelIndex & index);
  fn C_ZNK13QItemDelegate12setModelDataEP7QWidgetP18QAbstractItemModelRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QItemDelegate::setItemEditorFactory(QItemEditorFactory * factory);
  fn C_ZN13QItemDelegate20setItemEditorFactoryEP18QItemEditorFactory(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QItemDelegate::setClipping(bool clip);
  fn C_ZN13QItemDelegate11setClippingEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn C_ZNK13QItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QItemEditorFactory * QItemDelegate::itemEditorFactory();
  fn C_ZNK13QItemDelegate17itemEditorFactoryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QWidget * QItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn C_ZNK13QItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QItemDelegate::QItemDelegate(QObject * parent);
  fn C_ZN13QItemDelegateC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  bool QItemDelegate::hasClipping();
  fn C_ZNK13QItemDelegate11hasClippingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QItemDelegate::~QItemDelegate();
  fn C_ZN13QItemDelegateD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn C_ZNK13QItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
  fn C_ZNK13QItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QItemDelegate)=1
#[derive(Default)]
pub struct QItemDelegate {
  qbase: QAbstractItemDelegate,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QItemDelegate {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QItemDelegate {
    return QItemDelegate{qbase: QAbstractItemDelegate::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QItemDelegate {
  type Target = QAbstractItemDelegate;

  fn deref(&self) -> &QAbstractItemDelegate {
    return & self.qbase;
  }
}
impl AsRef<QAbstractItemDelegate> for QItemDelegate {
  fn as_ref(& self) -> & QAbstractItemDelegate {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QItemDelegate::metaObject();
impl /*struct*/ QItemDelegate {
  pub fn metaObject<RetType, T: QItemDelegate_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QItemDelegate_metaObject<RetType> {
  fn metaObject(self , rsthis: & QItemDelegate) -> RetType;
}

  // proto:  const QMetaObject * QItemDelegate::metaObject();
impl<'a> /*trait*/ QItemDelegate_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QItemDelegate) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate10metaObjectEv()};
    let mut ret = unsafe {C_ZNK13QItemDelegate10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QItemDelegate {
  pub fn sizeHint<RetType, T: QItemDelegate_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QItemDelegate_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QItemDelegate) -> RetType;
}

  // proto:  QSize QItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QItemDelegate_sizeHint<QSize> for (&'a QStyleOptionViewItem, &'a QModelIndex) {
  fn sizeHint(self , rsthis: & QItemDelegate) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QItemDelegate::setModelData(QWidget * editor, QAbstractItemModel * model, const QModelIndex & index);
impl /*struct*/ QItemDelegate {
  pub fn setModelData<RetType, T: QItemDelegate_setModelData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setModelData(self);
    // return 1;
  }
}

pub trait QItemDelegate_setModelData<RetType> {
  fn setModelData(self , rsthis: & QItemDelegate) -> RetType;
}

  // proto:  void QItemDelegate::setModelData(QWidget * editor, QAbstractItemModel * model, const QModelIndex & index);
impl<'a> /*trait*/ QItemDelegate_setModelData<()> for (&'a QWidget, &'a QAbstractItemModel, &'a QModelIndex) {
  fn setModelData(self , rsthis: & QItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate12setModelDataEP7QWidgetP18QAbstractItemModelRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZNK13QItemDelegate12setModelDataEP7QWidgetP18QAbstractItemModelRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QItemDelegate::setItemEditorFactory(QItemEditorFactory * factory);
impl /*struct*/ QItemDelegate {
  pub fn setItemEditorFactory<RetType, T: QItemDelegate_setItemEditorFactory<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemEditorFactory(self);
    // return 1;
  }
}

pub trait QItemDelegate_setItemEditorFactory<RetType> {
  fn setItemEditorFactory(self , rsthis: & QItemDelegate) -> RetType;
}

  // proto:  void QItemDelegate::setItemEditorFactory(QItemEditorFactory * factory);
impl<'a> /*trait*/ QItemDelegate_setItemEditorFactory<()> for (&'a QItemEditorFactory) {
  fn setItemEditorFactory(self , rsthis: & QItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QItemDelegate20setItemEditorFactoryEP18QItemEditorFactory()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QItemDelegate20setItemEditorFactoryEP18QItemEditorFactory(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QItemDelegate::setClipping(bool clip);
impl /*struct*/ QItemDelegate {
  pub fn setClipping<RetType, T: QItemDelegate_setClipping<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setClipping(self);
    // return 1;
  }
}

pub trait QItemDelegate_setClipping<RetType> {
  fn setClipping(self , rsthis: & QItemDelegate) -> RetType;
}

  // proto:  void QItemDelegate::setClipping(bool clip);
impl<'a> /*trait*/ QItemDelegate_setClipping<()> for (i8) {
  fn setClipping(self , rsthis: & QItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QItemDelegate11setClippingEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QItemDelegate11setClippingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QItemDelegate {
  pub fn updateEditorGeometry<RetType, T: QItemDelegate_updateEditorGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.updateEditorGeometry(self);
    // return 1;
  }
}

pub trait QItemDelegate_updateEditorGeometry<RetType> {
  fn updateEditorGeometry(self , rsthis: & QItemDelegate) -> RetType;
}

  // proto:  void QItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QItemDelegate_updateEditorGeometry<()> for (&'a QWidget, &'a QStyleOptionViewItem, &'a QModelIndex) {
  fn updateEditorGeometry(self , rsthis: & QItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZNK13QItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QItemEditorFactory * QItemDelegate::itemEditorFactory();
impl /*struct*/ QItemDelegate {
  pub fn itemEditorFactory<RetType, T: QItemDelegate_itemEditorFactory<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemEditorFactory(self);
    // return 1;
  }
}

pub trait QItemDelegate_itemEditorFactory<RetType> {
  fn itemEditorFactory(self , rsthis: & QItemDelegate) -> RetType;
}

  // proto:  QItemEditorFactory * QItemDelegate::itemEditorFactory();
impl<'a> /*trait*/ QItemDelegate_itemEditorFactory<QItemEditorFactory> for () {
  fn itemEditorFactory(self , rsthis: & QItemDelegate) -> QItemEditorFactory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate17itemEditorFactoryEv()};
    let mut ret = unsafe {C_ZNK13QItemDelegate17itemEditorFactoryEv(rsthis.qclsinst)};
    let mut ret1 = QItemEditorFactory::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QItemDelegate {
  pub fn createEditor<RetType, T: QItemDelegate_createEditor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createEditor(self);
    // return 1;
  }
}

pub trait QItemDelegate_createEditor<RetType> {
  fn createEditor(self , rsthis: & QItemDelegate) -> RetType;
}

  // proto:  QWidget * QItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QItemDelegate_createEditor<QWidget> for (&'a QWidget, &'a QStyleOptionViewItem, &'a QModelIndex) {
  fn createEditor(self , rsthis: & QItemDelegate) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QItemDelegate::QItemDelegate(QObject * parent);
impl /*struct*/ QItemDelegate {
  pub fn new<T: QItemDelegate_new>(value: T) -> QItemDelegate {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QItemDelegate_new {
  fn new(self) -> QItemDelegate;
}

  // proto:  void QItemDelegate::QItemDelegate(QObject * parent);
impl<'a> /*trait*/ QItemDelegate_new for (Option<&'a QObject>) {
  fn new(self) -> QItemDelegate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QItemDelegateC2EP7QObject()};
    let ctysz: c_int = unsafe{QItemDelegate_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QItemDelegateC2EP7QObject(arg0)};
    let rsthis = QItemDelegate{qbase: QAbstractItemDelegate::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QItemDelegate::hasClipping();
impl /*struct*/ QItemDelegate {
  pub fn hasClipping<RetType, T: QItemDelegate_hasClipping<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasClipping(self);
    // return 1;
  }
}

pub trait QItemDelegate_hasClipping<RetType> {
  fn hasClipping(self , rsthis: & QItemDelegate) -> RetType;
}

  // proto:  bool QItemDelegate::hasClipping();
impl<'a> /*trait*/ QItemDelegate_hasClipping<i8> for () {
  fn hasClipping(self , rsthis: & QItemDelegate) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate11hasClippingEv()};
    let mut ret = unsafe {C_ZNK13QItemDelegate11hasClippingEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QItemDelegate::~QItemDelegate();
impl /*struct*/ QItemDelegate {
  pub fn free<RetType, T: QItemDelegate_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QItemDelegate_free<RetType> {
  fn free(self , rsthis: & QItemDelegate) -> RetType;
}

  // proto:  void QItemDelegate::~QItemDelegate();
impl<'a> /*trait*/ QItemDelegate_free<()> for () {
  fn free(self , rsthis: & QItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QItemDelegateD2Ev()};
     unsafe {C_ZN13QItemDelegateD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QItemDelegate {
  pub fn paint<RetType, T: QItemDelegate_paint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QItemDelegate_paint<RetType> {
  fn paint(self , rsthis: & QItemDelegate) -> RetType;
}

  // proto:  void QItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QItemDelegate_paint<()> for (&'a QPainter, &'a QStyleOptionViewItem, &'a QModelIndex) {
  fn paint(self , rsthis: & QItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZNK13QItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
impl /*struct*/ QItemDelegate {
  pub fn setEditorData<RetType, T: QItemDelegate_setEditorData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEditorData(self);
    // return 1;
  }
}

pub trait QItemDelegate_setEditorData<RetType> {
  fn setEditorData(self , rsthis: & QItemDelegate) -> RetType;
}

  // proto:  void QItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
impl<'a> /*trait*/ QItemDelegate_setEditorData<()> for (&'a QWidget, &'a QModelIndex) {
  fn setEditorData(self , rsthis: & QItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZNK13QItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// <= body block end

