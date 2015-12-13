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
use super::qwidget::QWidget;
use super::qitemeditorfactory::QItemEditorFactory;
use super::qpainter::QPainter;
use super::qobject::QObject;
use super::qvariant::QVariant;
use super::qlocale::QLocale;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QSize QStyledItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QWidget * QStyledItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(arg0: *mut c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: void QStyledItemDelegate::FreeQStyledItemDelegate();
  fn _ZN19QStyledItemDelegateD0Ev() -> i32;
  // proto: void QStyledItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(arg0: *mut c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: void QStyledItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex(arg0: *mut c_void, arg1: *const c_void) -> i32;
  // proto: void QStyledItemDelegate::NewQStyledItemDelegate(const QStyledItemDelegate & );
  fn _ZN19QStyledItemDelegateC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QStyledItemDelegate::setItemEditorFactory(QItemEditorFactory * factory);
  fn _ZN19QStyledItemDelegate20setItemEditorFactoryEP18QItemEditorFactory(arg0: *mut c_void) -> i32;
  // proto: void QStyledItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex(arg0: *mut c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: QItemEditorFactory * QStyledItemDelegate::itemEditorFactory();
  fn _ZNK19QStyledItemDelegate17itemEditorFactoryEv() -> i32;
  // proto: void QStyledItemDelegate::NewQStyledItemDelegate(QObject * parent);
  fn _ZN19QStyledItemDelegateC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QString QStyledItemDelegate::displayText(const QVariant & value, const QLocale & locale);
  fn _ZNK19QStyledItemDelegate11displayTextERK8QVariantRK7QLocale(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: const QMetaObject * QStyledItemDelegate::metaObject();
  fn _ZNK19QStyledItemDelegate10metaObjectEv() -> i32;
}

// body block begin
// class sizeof(QStyledItemDelegate)=1
pub struct QStyledItemDelegate {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyledItemDelegate {
  pub fn sizeHint<T: QStyledItemDelegate_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QStyledItemDelegate_sizeHint {
  fn sizeHint(self, this: &mut QStyledItemDelegate) -> i32;
}

// proto: QSize QStyledItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_sizeHint for (&'a  QStyleOptionViewItem, &'a  QModelIndex) {
  fn sizeHint(self, this: &mut QStyledItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK19QStyledItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QStyledItemDelegate {
  pub fn createEditor<T: QStyledItemDelegate_createEditor>(&mut self, value: T) -> i32 {
    value.createEditor(self);
    return 1;
  }
}

pub trait QStyledItemDelegate_createEditor {
  fn createEditor(self, this: &mut QStyledItemDelegate) -> i32;
}

// proto: QWidget * QStyledItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_createEditor for (&'a mut QWidget, &'a  QStyleOptionViewItem, &'a  QModelIndex) {
  fn createEditor(self, this: &mut QStyledItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZNK19QStyledItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QStyledItemDelegate {
  pub fn FreeQStyledItemDelegate<T: QStyledItemDelegate_FreeQStyledItemDelegate>(&mut self, value: T) -> i32 {
    value.FreeQStyledItemDelegate(self);
    return 1;
  }
}

pub trait QStyledItemDelegate_FreeQStyledItemDelegate {
  fn FreeQStyledItemDelegate(self, this: &mut QStyledItemDelegate) -> i32;
}

// proto: void QStyledItemDelegate::FreeQStyledItemDelegate();
impl<'a> /*trait*/ QStyledItemDelegate_FreeQStyledItemDelegate for () {
  fn FreeQStyledItemDelegate(self, this: &mut QStyledItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyledItemDelegateD0Ev()};
    unsafe {_ZN19QStyledItemDelegateD0Ev()};
    return 1;
  }
}

impl /*struct*/ QStyledItemDelegate {
  pub fn updateEditorGeometry<T: QStyledItemDelegate_updateEditorGeometry>(&mut self, value: T) -> i32 {
    value.updateEditorGeometry(self);
    return 1;
  }
}

pub trait QStyledItemDelegate_updateEditorGeometry {
  fn updateEditorGeometry(self, this: &mut QStyledItemDelegate) -> i32;
}

// proto: void QStyledItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_updateEditorGeometry for (&'a mut QWidget, &'a  QStyleOptionViewItem, &'a  QModelIndex) {
  fn updateEditorGeometry(self, this: &mut QStyledItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZNK19QStyledItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QStyledItemDelegate {
  pub fn setEditorData<T: QStyledItemDelegate_setEditorData>(&mut self, value: T) -> i32 {
    value.setEditorData(self);
    return 1;
  }
}

pub trait QStyledItemDelegate_setEditorData {
  fn setEditorData(self, this: &mut QStyledItemDelegate) -> i32;
}

// proto: void QStyledItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_setEditorData for (&'a mut QWidget, &'a  QModelIndex) {
  fn setEditorData(self, this: &mut QStyledItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK19QStyledItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QStyledItemDelegate {
  pub fn NewQStyledItemDelegate<T: QStyledItemDelegate_NewQStyledItemDelegate>(value: T) -> QStyledItemDelegate {
    let rsthis = value.NewQStyledItemDelegate();
    return rsthis;
    // return 1;
  }
}

pub trait QStyledItemDelegate_NewQStyledItemDelegate {
  fn NewQStyledItemDelegate(self) -> QStyledItemDelegate;
}

// proto: void QStyledItemDelegate::NewQStyledItemDelegate(const QStyledItemDelegate & );
impl<'a> /*trait*/ QStyledItemDelegate_NewQStyledItemDelegate for (&'a  QStyledItemDelegate) {
  fn NewQStyledItemDelegate(self) -> QStyledItemDelegate {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyledItemDelegateC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QStyledItemDelegateC1ERKS_(qthis, arg0)};
    let rsthis = QStyledItemDelegate{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyledItemDelegate {
  pub fn setItemEditorFactory<T: QStyledItemDelegate_setItemEditorFactory>(&mut self, value: T) -> i32 {
    value.setItemEditorFactory(self);
    return 1;
  }
}

pub trait QStyledItemDelegate_setItemEditorFactory {
  fn setItemEditorFactory(self, this: &mut QStyledItemDelegate) -> i32;
}

// proto: void QStyledItemDelegate::setItemEditorFactory(QItemEditorFactory * factory);
impl<'a> /*trait*/ QStyledItemDelegate_setItemEditorFactory for (&'a mut QItemEditorFactory) {
  fn setItemEditorFactory(self, this: &mut QStyledItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyledItemDelegate20setItemEditorFactoryEP18QItemEditorFactory()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QStyledItemDelegate20setItemEditorFactoryEP18QItemEditorFactory(arg0)};
    return 1;
  }
}

impl /*struct*/ QStyledItemDelegate {
  pub fn paint<T: QStyledItemDelegate_paint>(&mut self, value: T) -> i32 {
    value.paint(self);
    return 1;
  }
}

pub trait QStyledItemDelegate_paint {
  fn paint(self, this: &mut QStyledItemDelegate) -> i32;
}

// proto: void QStyledItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_paint for (&'a mut QPainter, &'a  QStyleOptionViewItem, &'a  QModelIndex) {
  fn paint(self, this: &mut QStyledItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZNK19QStyledItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QStyledItemDelegate {
  pub fn itemEditorFactory<T: QStyledItemDelegate_itemEditorFactory>(&mut self, value: T) -> i32 {
    value.itemEditorFactory(self);
    return 1;
  }
}

pub trait QStyledItemDelegate_itemEditorFactory {
  fn itemEditorFactory(self, this: &mut QStyledItemDelegate) -> i32;
}

// proto: QItemEditorFactory * QStyledItemDelegate::itemEditorFactory();
impl<'a> /*trait*/ QStyledItemDelegate_itemEditorFactory for () {
  fn itemEditorFactory(self, this: &mut QStyledItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate17itemEditorFactoryEv()};
    unsafe {_ZNK19QStyledItemDelegate17itemEditorFactoryEv()};
    return 1;
  }
}

// proto: void QStyledItemDelegate::NewQStyledItemDelegate(QObject * parent);
impl<'a> /*trait*/ QStyledItemDelegate_NewQStyledItemDelegate for (&'a mut QObject) {
  fn NewQStyledItemDelegate(self) -> QStyledItemDelegate {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyledItemDelegateC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QStyledItemDelegateC1EP7QObject(qthis, arg0)};
    let rsthis = QStyledItemDelegate{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyledItemDelegate {
  pub fn displayText<T: QStyledItemDelegate_displayText>(&mut self, value: T) -> i32 {
    value.displayText(self);
    return 1;
  }
}

pub trait QStyledItemDelegate_displayText {
  fn displayText(self, this: &mut QStyledItemDelegate) -> i32;
}

// proto: QString QStyledItemDelegate::displayText(const QVariant & value, const QLocale & locale);
impl<'a> /*trait*/ QStyledItemDelegate_displayText for (&'a  QVariant, &'a  QLocale) {
  fn displayText(self, this: &mut QStyledItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate11displayTextERK8QVariantRK7QLocale()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZNK19QStyledItemDelegate11displayTextERK8QVariantRK7QLocale(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QStyledItemDelegate {
  pub fn metaObject<T: QStyledItemDelegate_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QStyledItemDelegate_metaObject {
  fn metaObject(self, this: &mut QStyledItemDelegate) -> i32;
}

// proto: const QMetaObject * QStyledItemDelegate::metaObject();
impl<'a> /*trait*/ QStyledItemDelegate_metaObject for () {
  fn metaObject(self, this: &mut QStyledItemDelegate) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate10metaObjectEv()};
    unsafe {_ZNK19QStyledItemDelegate10metaObjectEv()};
    return 1;
  }
}

