// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstyleoptionviewitem::QStyleOptionViewItem;
use super::qmodelindex::QModelIndex;
use super::qsize::QSize;
use super::qwidget::QWidget;
use super::qitemeditorfactory::QItemEditorFactory;
use super::qobject::QObject;
use super::qpainter::QPainter;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  const QMetaObject * QItemDelegate::metaObject();
  fn _ZNK13QItemDelegate10metaObjectEv(qthis: *mut c_void);
  // proto:  QSize QItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK13QItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QItemDelegate::setItemEditorFactory(QItemEditorFactory * factory);
  fn _ZN13QItemDelegate20setItemEditorFactoryEP18QItemEditorFactory(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QItemDelegate::setClipping(bool clip);
  fn _ZN13QItemDelegate11setClippingEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK13QItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QItemEditorFactory * QItemDelegate::itemEditorFactory();
  fn _ZNK13QItemDelegate17itemEditorFactoryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK13QItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QItemDelegate::QItemDelegate(QObject * parent);
  fn _ZN13QItemDelegateC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QItemDelegate::QItemDelegate(const QItemDelegate & );
  fn _ZN13QItemDelegateC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QItemDelegate::hasClipping();
  fn _ZNK13QItemDelegate11hasClippingEv(qthis: *mut c_void) -> c_char;
  // proto:  void QItemDelegate::~QItemDelegate();
  fn _ZN13QItemDelegateD0Ev(qthis: *mut c_void);
  // proto:  void QItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK13QItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
  fn _ZNK13QItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
}

// body block begin
// class sizeof(QItemDelegate)=1
pub struct QItemDelegate {
  pub qclsinst: *mut c_void,
}

  // proto:  const QMetaObject * QItemDelegate::metaObject();
impl /*struct*/ QItemDelegate {
  pub fn metaObject<RetType, T: QItemDelegate_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QItemDelegate_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QItemDelegate) -> RetType;
}

  // proto:  const QMetaObject * QItemDelegate::metaObject();
impl<'a> /*trait*/ QItemDelegate_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate10metaObjectEv()};
     unsafe {_ZNK13QItemDelegate10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QItemDelegate {
  pub fn sizeHint<RetType, T: QItemDelegate_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QItemDelegate_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QItemDelegate) -> RetType;
}

  // proto:  QSize QItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QItemDelegate_sizeHint<QSize> for (QStyleOptionViewItem, QModelIndex) {
  fn sizeHint(self , rsthis: &mut QItemDelegate) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QItemDelegate::setItemEditorFactory(QItemEditorFactory * factory);
impl /*struct*/ QItemDelegate {
  pub fn setItemEditorFactory<RetType, T: QItemDelegate_setItemEditorFactory<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setItemEditorFactory(self);
    // return 1;
  }
}

pub trait QItemDelegate_setItemEditorFactory<RetType> {
  fn setItemEditorFactory(self , rsthis: &mut QItemDelegate) -> RetType;
}

  // proto:  void QItemDelegate::setItemEditorFactory(QItemEditorFactory * factory);
impl<'a> /*trait*/ QItemDelegate_setItemEditorFactory<()> for (QItemEditorFactory) {
  fn setItemEditorFactory(self , rsthis: &mut QItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QItemDelegate20setItemEditorFactoryEP18QItemEditorFactory()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QItemDelegate20setItemEditorFactoryEP18QItemEditorFactory(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QItemDelegate::setClipping(bool clip);
impl /*struct*/ QItemDelegate {
  pub fn setClipping<RetType, T: QItemDelegate_setClipping<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setClipping(self);
    // return 1;
  }
}

pub trait QItemDelegate_setClipping<RetType> {
  fn setClipping(self , rsthis: &mut QItemDelegate) -> RetType;
}

  // proto:  void QItemDelegate::setClipping(bool clip);
impl<'a> /*trait*/ QItemDelegate_setClipping<()> for (i8) {
  fn setClipping(self , rsthis: &mut QItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QItemDelegate11setClippingEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QItemDelegate11setClippingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QItemDelegate {
  pub fn updateEditorGeometry<RetType, T: QItemDelegate_updateEditorGeometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.updateEditorGeometry(self);
    // return 1;
  }
}

pub trait QItemDelegate_updateEditorGeometry<RetType> {
  fn updateEditorGeometry(self , rsthis: &mut QItemDelegate) -> RetType;
}

  // proto:  void QItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QItemDelegate_updateEditorGeometry<()> for (QWidget, QStyleOptionViewItem, QModelIndex) {
  fn updateEditorGeometry(self , rsthis: &mut QItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZNK13QItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QItemEditorFactory * QItemDelegate::itemEditorFactory();
impl /*struct*/ QItemDelegate {
  pub fn itemEditorFactory<RetType, T: QItemDelegate_itemEditorFactory<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemEditorFactory(self);
    // return 1;
  }
}

pub trait QItemDelegate_itemEditorFactory<RetType> {
  fn itemEditorFactory(self , rsthis: &mut QItemDelegate) -> RetType;
}

  // proto:  QItemEditorFactory * QItemDelegate::itemEditorFactory();
impl<'a> /*trait*/ QItemDelegate_itemEditorFactory<QItemEditorFactory> for () {
  fn itemEditorFactory(self , rsthis: &mut QItemDelegate) -> QItemEditorFactory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate17itemEditorFactoryEv()};
    let mut ret = unsafe {_ZNK13QItemDelegate17itemEditorFactoryEv(rsthis.qclsinst)};
    let mut ret1 = QItemEditorFactory{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QItemDelegate {
  pub fn createEditor<RetType, T: QItemDelegate_createEditor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.createEditor(self);
    // return 1;
  }
}

pub trait QItemDelegate_createEditor<RetType> {
  fn createEditor(self , rsthis: &mut QItemDelegate) -> RetType;
}

  // proto:  QWidget * QItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QItemDelegate_createEditor<QWidget> for (QWidget, QStyleOptionViewItem, QModelIndex) {
  fn createEditor(self , rsthis: &mut QItemDelegate) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QItemDelegate::QItemDelegate(QObject * parent);
impl /*struct*/ QItemDelegate {
  pub fn NewQItemDelegate<T: QItemDelegate_NewQItemDelegate>(value: T) -> QItemDelegate {
    let rsthis = value.NewQItemDelegate();
    return rsthis;
    // return 1;
  }
}

pub trait QItemDelegate_NewQItemDelegate {
  fn NewQItemDelegate(self) -> QItemDelegate;
}

  // proto:  void QItemDelegate::QItemDelegate(QObject * parent);
impl<'a> /*trait*/ QItemDelegate_NewQItemDelegate for (QObject) {
  fn NewQItemDelegate(self) -> QItemDelegate {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QItemDelegateC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QItemDelegateC1EP7QObject(qthis, arg0)};
    let rsthis = QItemDelegate{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QItemDelegate::QItemDelegate(const QItemDelegate & );
impl<'a> /*trait*/ QItemDelegate_NewQItemDelegate for (QItemDelegate) {
  fn NewQItemDelegate(self) -> QItemDelegate {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QItemDelegateC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QItemDelegateC1ERKS_(qthis, arg0)};
    let rsthis = QItemDelegate{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QItemDelegate::hasClipping();
impl /*struct*/ QItemDelegate {
  pub fn hasClipping<RetType, T: QItemDelegate_hasClipping<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasClipping(self);
    // return 1;
  }
}

pub trait QItemDelegate_hasClipping<RetType> {
  fn hasClipping(self , rsthis: &mut QItemDelegate) -> RetType;
}

  // proto:  bool QItemDelegate::hasClipping();
impl<'a> /*trait*/ QItemDelegate_hasClipping<i8> for () {
  fn hasClipping(self , rsthis: &mut QItemDelegate) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate11hasClippingEv()};
    let mut ret = unsafe {_ZNK13QItemDelegate11hasClippingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QItemDelegate::~QItemDelegate();
impl /*struct*/ QItemDelegate {
  pub fn FreeQItemDelegate<RetType, T: QItemDelegate_FreeQItemDelegate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQItemDelegate(self);
    // return 1;
  }
}

pub trait QItemDelegate_FreeQItemDelegate<RetType> {
  fn FreeQItemDelegate(self , rsthis: &mut QItemDelegate) -> RetType;
}

  // proto:  void QItemDelegate::~QItemDelegate();
impl<'a> /*trait*/ QItemDelegate_FreeQItemDelegate<()> for () {
  fn FreeQItemDelegate(self , rsthis: &mut QItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QItemDelegateD0Ev()};
     unsafe {_ZN13QItemDelegateD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QItemDelegate {
  pub fn paint<RetType, T: QItemDelegate_paint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QItemDelegate_paint<RetType> {
  fn paint(self , rsthis: &mut QItemDelegate) -> RetType;
}

  // proto:  void QItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QItemDelegate_paint<()> for (QPainter, QStyleOptionViewItem, QModelIndex) {
  fn paint(self , rsthis: &mut QItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZNK13QItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
impl /*struct*/ QItemDelegate {
  pub fn setEditorData<RetType, T: QItemDelegate_setEditorData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setEditorData(self);
    // return 1;
  }
}

pub trait QItemDelegate_setEditorData<RetType> {
  fn setEditorData(self , rsthis: &mut QItemDelegate) -> RetType;
}

  // proto:  void QItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
impl<'a> /*trait*/ QItemDelegate_setEditorData<()> for (QWidget, QModelIndex) {
  fn setEditorData(self , rsthis: &mut QItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZNK13QItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

