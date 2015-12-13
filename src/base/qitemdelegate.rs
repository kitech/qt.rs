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
use super::qitemeditorfactory::QItemEditorFactory;
use super::qwidget::QWidget;
use super::qobject::QObject;
use super::qpainter::QPainter;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: const QMetaObject * QItemDelegate::metaObject();
  fn _ZNK13QItemDelegate10metaObjectEv() -> i32;
  // proto: QSize QItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK13QItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QItemDelegate::setItemEditorFactory(QItemEditorFactory * factory);
  fn _ZN13QItemDelegate20setItemEditorFactoryEP18QItemEditorFactory(arg0: *mut c_void) -> i32;
  // proto: void QItemDelegate::setClipping(bool clip);
  fn _ZN13QItemDelegate11setClippingEb(arg0: int8_t) -> i32;
  // proto: void QItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK13QItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(arg0: *mut c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: QItemEditorFactory * QItemDelegate::itemEditorFactory();
  fn _ZNK13QItemDelegate17itemEditorFactoryEv() -> i32;
  // proto: QWidget * QItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK13QItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(arg0: *mut c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: void QItemDelegate::NewQItemDelegate(QObject * parent);
  fn _ZN13QItemDelegateC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QItemDelegate::NewQItemDelegate(const QItemDelegate & );
  fn _ZN13QItemDelegateC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QItemDelegate::hasClipping();
  fn _ZNK13QItemDelegate11hasClippingEv() -> i32;
  // proto: void QItemDelegate::FreeQItemDelegate();
  fn _ZN13QItemDelegateD0Ev() -> i32;
  // proto: void QItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK13QItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex(arg0: *mut c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: void QItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
  fn _ZNK13QItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex(arg0: *mut c_void, arg1: *const c_void) -> i32;
}

// body block begin
// class sizeof(QItemDelegate)=1
pub struct QItemDelegate {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QItemDelegate {
  pub fn metaObject<T: QItemDelegate_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QItemDelegate_metaObject {
  fn metaObject(self, this: &mut QItemDelegate) -> i32;
}

// proto: const QMetaObject * QItemDelegate::metaObject();
impl<'a> /*trait*/ QItemDelegate_metaObject for () {
  fn metaObject(self, this: &mut QItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate10metaObjectEv()};
    unsafe {_ZNK13QItemDelegate10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QItemDelegate {
  pub fn sizeHint<T: QItemDelegate_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QItemDelegate_sizeHint {
  fn sizeHint(self, this: &mut QItemDelegate) -> i32;
}

// proto: QSize QItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QItemDelegate_sizeHint for (&'a  QStyleOptionViewItem, &'a  QModelIndex) {
  fn sizeHint(self, this: &mut QItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QItemDelegate {
  pub fn setItemEditorFactory<T: QItemDelegate_setItemEditorFactory>(&mut self, value: T) -> i32 {
    value.setItemEditorFactory(self);
    return 1;
  }
}

pub trait QItemDelegate_setItemEditorFactory {
  fn setItemEditorFactory(self, this: &mut QItemDelegate) -> i32;
}

// proto: void QItemDelegate::setItemEditorFactory(QItemEditorFactory * factory);
impl<'a> /*trait*/ QItemDelegate_setItemEditorFactory for (&'a mut QItemEditorFactory) {
  fn setItemEditorFactory(self, this: &mut QItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QItemDelegate20setItemEditorFactoryEP18QItemEditorFactory()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QItemDelegate20setItemEditorFactoryEP18QItemEditorFactory(arg0)};
    return 1;
  }
}

impl /*struct*/ QItemDelegate {
  pub fn setClipping<T: QItemDelegate_setClipping>(&mut self, value: T) -> i32 {
    value.setClipping(self);
    return 1;
  }
}

pub trait QItemDelegate_setClipping {
  fn setClipping(self, this: &mut QItemDelegate) -> i32;
}

// proto: void QItemDelegate::setClipping(bool clip);
impl<'a> /*trait*/ QItemDelegate_setClipping for (i8) {
  fn setClipping(self, this: &mut QItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QItemDelegate11setClippingEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QItemDelegate11setClippingEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QItemDelegate {
  pub fn updateEditorGeometry<T: QItemDelegate_updateEditorGeometry>(&mut self, value: T) -> i32 {
    value.updateEditorGeometry(self);
    return 1;
  }
}

pub trait QItemDelegate_updateEditorGeometry {
  fn updateEditorGeometry(self, this: &mut QItemDelegate) -> i32;
}

// proto: void QItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QItemDelegate_updateEditorGeometry for (&'a mut QWidget, &'a  QStyleOptionViewItem, &'a  QModelIndex) {
  fn updateEditorGeometry(self, this: &mut QItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZNK13QItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QItemDelegate {
  pub fn itemEditorFactory<T: QItemDelegate_itemEditorFactory>(&mut self, value: T) -> i32 {
    value.itemEditorFactory(self);
    return 1;
  }
}

pub trait QItemDelegate_itemEditorFactory {
  fn itemEditorFactory(self, this: &mut QItemDelegate) -> i32;
}

// proto: QItemEditorFactory * QItemDelegate::itemEditorFactory();
impl<'a> /*trait*/ QItemDelegate_itemEditorFactory for () {
  fn itemEditorFactory(self, this: &mut QItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate17itemEditorFactoryEv()};
    unsafe {_ZNK13QItemDelegate17itemEditorFactoryEv()};
    return 1;
  }
}

impl /*struct*/ QItemDelegate {
  pub fn createEditor<T: QItemDelegate_createEditor>(&mut self, value: T) -> i32 {
    value.createEditor(self);
    return 1;
  }
}

pub trait QItemDelegate_createEditor {
  fn createEditor(self, this: &mut QItemDelegate) -> i32;
}

// proto: QWidget * QItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QItemDelegate_createEditor for (&'a mut QWidget, &'a  QStyleOptionViewItem, &'a  QModelIndex) {
  fn createEditor(self, this: &mut QItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZNK13QItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(arg0, arg1, arg2)};
    return 1;
  }
}

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

// proto: void QItemDelegate::NewQItemDelegate(QObject * parent);
impl<'a> /*trait*/ QItemDelegate_NewQItemDelegate for (&'a mut QObject) {
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

// proto: void QItemDelegate::NewQItemDelegate(const QItemDelegate & );
impl<'a> /*trait*/ QItemDelegate_NewQItemDelegate for (&'a  QItemDelegate) {
  fn NewQItemDelegate(self) -> QItemDelegate {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QItemDelegateC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QItemDelegateC1ERKS_(qthis, arg0)};
    let rsthis = QItemDelegate{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QItemDelegate {
  pub fn hasClipping<T: QItemDelegate_hasClipping>(&mut self, value: T) -> i32 {
    value.hasClipping(self);
    return 1;
  }
}

pub trait QItemDelegate_hasClipping {
  fn hasClipping(self, this: &mut QItemDelegate) -> i32;
}

// proto: bool QItemDelegate::hasClipping();
impl<'a> /*trait*/ QItemDelegate_hasClipping for () {
  fn hasClipping(self, this: &mut QItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate11hasClippingEv()};
    unsafe {_ZNK13QItemDelegate11hasClippingEv()};
    return 1;
  }
}

impl /*struct*/ QItemDelegate {
  pub fn FreeQItemDelegate<T: QItemDelegate_FreeQItemDelegate>(&mut self, value: T) -> i32 {
    value.FreeQItemDelegate(self);
    return 1;
  }
}

pub trait QItemDelegate_FreeQItemDelegate {
  fn FreeQItemDelegate(self, this: &mut QItemDelegate) -> i32;
}

// proto: void QItemDelegate::FreeQItemDelegate();
impl<'a> /*trait*/ QItemDelegate_FreeQItemDelegate for () {
  fn FreeQItemDelegate(self, this: &mut QItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QItemDelegateD0Ev()};
    unsafe {_ZN13QItemDelegateD0Ev()};
    return 1;
  }
}

impl /*struct*/ QItemDelegate {
  pub fn paint<T: QItemDelegate_paint>(&mut self, value: T) -> i32 {
    value.paint(self);
    return 1;
  }
}

pub trait QItemDelegate_paint {
  fn paint(self, this: &mut QItemDelegate) -> i32;
}

// proto: void QItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QItemDelegate_paint for (&'a mut QPainter, &'a  QStyleOptionViewItem, &'a  QModelIndex) {
  fn paint(self, this: &mut QItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZNK13QItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QItemDelegate {
  pub fn setEditorData<T: QItemDelegate_setEditorData>(&mut self, value: T) -> i32 {
    value.setEditorData(self);
    return 1;
  }
}

pub trait QItemDelegate_setEditorData {
  fn setEditorData(self, this: &mut QItemDelegate) -> i32;
}

// proto: void QItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
impl<'a> /*trait*/ QItemDelegate_setEditorData for (&'a mut QWidget, &'a  QModelIndex) {
  fn setEditorData(self, this: &mut QItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK13QItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex(arg0, arg1)};
    return 1;
  }
}

