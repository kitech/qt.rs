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
use super::qpainter::QPainter;
use super::qobject::QObject;
use super::qvariant::QVariant;
use super::qlocale::QLocale;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QSize QStyledItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QStyledItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QStyledItemDelegate::FreeQStyledItemDelegate();
  fn _ZN19QStyledItemDelegateD0Ev(qthis: *mut c_void) ;
  // proto:  void QStyledItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QStyledItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QStyledItemDelegate::NewQStyledItemDelegate(const QStyledItemDelegate & );
  fn _ZN19QStyledItemDelegateC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStyledItemDelegate::setItemEditorFactory(QItemEditorFactory * factory);
  fn _ZN19QStyledItemDelegate20setItemEditorFactoryEP18QItemEditorFactory(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStyledItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  QItemEditorFactory * QStyledItemDelegate::itemEditorFactory();
  fn _ZNK19QStyledItemDelegate17itemEditorFactoryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStyledItemDelegate::NewQStyledItemDelegate(QObject * parent);
  fn _ZN19QStyledItemDelegateC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QStyledItemDelegate::displayText(const QVariant & value, const QLocale & locale);
  fn _ZNK19QStyledItemDelegate11displayTextERK8QVariantRK7QLocale(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QStyledItemDelegate::metaObject();
  fn _ZNK19QStyledItemDelegate10metaObjectEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QStyledItemDelegate)=1
pub struct QStyledItemDelegate {
  pub qclsinst: *mut c_void,
}

// proto:  QSize QStyledItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QStyledItemDelegate {
  pub fn sizeHint<RetType, T: QStyledItemDelegate_sizeHint<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

// proto:  QSize QStyledItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_sizeHint<QSize> for (&'a  QStyleOptionViewItem, &'a  QModelIndex) {
  fn sizeHint(self , rsthis: &mut QStyledItemDelegate) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QStyledItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QWidget * QStyledItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QStyledItemDelegate {
  pub fn createEditor<RetType, T: QStyledItemDelegate_createEditor<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.createEditor(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_createEditor<RetType> {
  fn createEditor(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

// proto:  QWidget * QStyledItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_createEditor<QWidget> for (&'a mut QWidget, &'a  QStyleOptionViewItem, &'a  QModelIndex) {
  fn createEditor(self , rsthis: &mut QStyledItemDelegate) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QStyledItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QStyledItemDelegate::FreeQStyledItemDelegate();
impl /*struct*/ QStyledItemDelegate {
  pub fn FreeQStyledItemDelegate<RetType, T: QStyledItemDelegate_FreeQStyledItemDelegate<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQStyledItemDelegate(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_FreeQStyledItemDelegate<RetType> {
  fn FreeQStyledItemDelegate(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

// proto:  void QStyledItemDelegate::FreeQStyledItemDelegate();
impl<'a> /*trait*/ QStyledItemDelegate_FreeQStyledItemDelegate<()> for () {
  fn FreeQStyledItemDelegate(self , rsthis: &mut QStyledItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyledItemDelegateD0Ev()};
     unsafe {_ZN19QStyledItemDelegateD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QStyledItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QStyledItemDelegate {
  pub fn updateEditorGeometry<RetType, T: QStyledItemDelegate_updateEditorGeometry<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.updateEditorGeometry(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_updateEditorGeometry<RetType> {
  fn updateEditorGeometry(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

// proto:  void QStyledItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_updateEditorGeometry<()> for (&'a mut QWidget, &'a  QStyleOptionViewItem, &'a  QModelIndex) {
  fn updateEditorGeometry(self , rsthis: &mut QStyledItemDelegate) -> () {
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
  pub fn setEditorData<RetType, T: QStyledItemDelegate_setEditorData<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setEditorData(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_setEditorData<RetType> {
  fn setEditorData(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

// proto:  void QStyledItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_setEditorData<()> for (&'a mut QWidget, &'a  QModelIndex) {
  fn setEditorData(self , rsthis: &mut QStyledItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZNK19QStyledItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QStyledItemDelegateC1ERKS_(qthis, arg0)};
    let rsthis = QStyledItemDelegate{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QStyledItemDelegate::setItemEditorFactory(QItemEditorFactory * factory);
impl /*struct*/ QStyledItemDelegate {
  pub fn setItemEditorFactory<RetType, T: QStyledItemDelegate_setItemEditorFactory<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setItemEditorFactory(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_setItemEditorFactory<RetType> {
  fn setItemEditorFactory(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

// proto:  void QStyledItemDelegate::setItemEditorFactory(QItemEditorFactory * factory);
impl<'a> /*trait*/ QStyledItemDelegate_setItemEditorFactory<()> for (&'a mut QItemEditorFactory) {
  fn setItemEditorFactory(self , rsthis: &mut QStyledItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyledItemDelegate20setItemEditorFactoryEP18QItemEditorFactory()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QStyledItemDelegate20setItemEditorFactoryEP18QItemEditorFactory(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QStyledItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QStyledItemDelegate {
  pub fn paint<RetType, T: QStyledItemDelegate_paint<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_paint<RetType> {
  fn paint(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

// proto:  void QStyledItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_paint<()> for (&'a mut QPainter, &'a  QStyleOptionViewItem, &'a  QModelIndex) {
  fn paint(self , rsthis: &mut QStyledItemDelegate) -> () {
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
  pub fn itemEditorFactory<RetType, T: QStyledItemDelegate_itemEditorFactory<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.itemEditorFactory(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_itemEditorFactory<RetType> {
  fn itemEditorFactory(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

// proto:  QItemEditorFactory * QStyledItemDelegate::itemEditorFactory();
impl<'a> /*trait*/ QStyledItemDelegate_itemEditorFactory<QItemEditorFactory> for () {
  fn itemEditorFactory(self , rsthis: &mut QStyledItemDelegate) -> QItemEditorFactory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate17itemEditorFactoryEv()};
    let mut ret = unsafe {_ZNK19QStyledItemDelegate17itemEditorFactoryEv(rsthis.qclsinst)};
    let mut ret1 = QItemEditorFactory{qclsinst: ret};
    return ret1;
    // return 1;
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

// proto:  QString QStyledItemDelegate::displayText(const QVariant & value, const QLocale & locale);
impl /*struct*/ QStyledItemDelegate {
  pub fn displayText<RetType, T: QStyledItemDelegate_displayText<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.displayText(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_displayText<RetType> {
  fn displayText(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

// proto:  QString QStyledItemDelegate::displayText(const QVariant & value, const QLocale & locale);
impl<'a> /*trait*/ QStyledItemDelegate_displayText<QString> for (&'a  QVariant, &'a  QLocale) {
  fn displayText(self , rsthis: &mut QStyledItemDelegate) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate11displayTextERK8QVariantRK7QLocale()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QStyledItemDelegate11displayTextERK8QVariantRK7QLocale(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QMetaObject * QStyledItemDelegate::metaObject();
impl /*struct*/ QStyledItemDelegate {
  pub fn metaObject<RetType, T: QStyledItemDelegate_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

// proto:  const QMetaObject * QStyledItemDelegate::metaObject();
impl<'a> /*trait*/ QStyledItemDelegate_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QStyledItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate10metaObjectEv()};
     unsafe {_ZNK19QStyledItemDelegate10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

