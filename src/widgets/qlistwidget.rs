// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtWidgets/qlistwidget.h
// dst-file: /src/widgets/qlistwidget.rs
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
use std::ops::Deref;
use super::super::core::qvariant::QVariant; // 771
use super::super::gui::qbrush::QBrush; // 771
use super::super::gui::qfont::QFont; // 771
// use super::qlistwidget::QListWidget; // 773
use super::super::core::qdatastream::QDataStream; // 771
use super::super::core::qstring::QString; // 771
use super::super::gui::qicon::QIcon; // 771
use super::super::gui::qcolor::QColor; // 771
use super::super::core::qsize::QSize; // 771
use super::qlistview::QListView; // 773
use super::super::gui::qevent::QDropEvent; // 771
// use super::qlistwidget::QListWidgetItem; // 773
use super::qwidget::QWidget; // 773
use super::super::core::qpoint::QPoint; // 771
use super::super::core::qstringlist::QStringList; // 771
use super::super::core::qobjectdefs::QMetaObject; // 771
use super::super::core::qrect::QRect; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QListWidgetItem_Class_Size() -> c_int;
  // proto:  void QListWidgetItem::~QListWidgetItem();
  fn C_ZN15QListWidgetItemD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QListWidgetItem::isHidden();
  fn C_ZNK15QListWidgetItem8isHiddenEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QListWidgetItem::setData(int role, const QVariant & value);
  fn C_ZN15QListWidgetItem7setDataEiRK8QVariant(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QListWidgetItem::setBackground(const QBrush & brush);
  fn C_ZN15QListWidgetItem13setBackgroundERK6QBrush(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QListWidgetItem::setSelected(bool select);
  fn C_ZN15QListWidgetItem11setSelectedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QFont QListWidgetItem::font();
  fn C_ZNK15QListWidgetItem4fontEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QListWidgetItem::setTextAlignment(int alignment);
  fn C_ZN15QListWidgetItem16setTextAlignmentEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QListWidgetItem::QListWidgetItem(QListWidget * view, int type);
  fn C_ZN15QListWidgetItemC2EP11QListWidgeti(arg0: *mut c_void, arg1: c_int) -> u64;
  // proto:  void QListWidgetItem::write(QDataStream & out);
  fn C_ZNK15QListWidgetItem5writeER11QDataStream(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QListWidgetItem::whatsThis();
  fn C_ZNK15QListWidgetItem9whatsThisEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QListWidgetItem::type();
  fn C_ZNK15QListWidgetItem4typeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QListWidgetItem::QListWidgetItem(const QIcon & icon, const QString & text, QListWidget * view, int type);
  fn C_ZN15QListWidgetItemC2ERK5QIconRK7QStringP11QListWidgeti(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: c_int) -> u64;
  // proto:  QIcon QListWidgetItem::icon();
  fn C_ZNK15QListWidgetItem4iconEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QColor QListWidgetItem::textColor();
  fn C_ZNK15QListWidgetItem9textColorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QBrush QListWidgetItem::foreground();
  fn C_ZNK15QListWidgetItem10foregroundEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QBrush QListWidgetItem::background();
  fn C_ZNK15QListWidgetItem10backgroundEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QListWidgetItem::setStatusTip(const QString & statusTip);
  fn C_ZN15QListWidgetItem12setStatusTipERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QListWidgetItem::text();
  fn C_ZNK15QListWidgetItem4textEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QColor QListWidgetItem::backgroundColor();
  fn C_ZNK15QListWidgetItem15backgroundColorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QListWidgetItem::isSelected();
  fn C_ZNK15QListWidgetItem10isSelectedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QListWidgetItem::setFont(const QFont & font);
  fn C_ZN15QListWidgetItem7setFontERK5QFont(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QListWidgetItem::setText(const QString & text);
  fn C_ZN15QListWidgetItem7setTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QListWidgetItem::QListWidgetItem(const QString & text, QListWidget * view, int type);
  fn C_ZN15QListWidgetItemC2ERK7QStringP11QListWidgeti(arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> u64;
  // proto:  QVariant QListWidgetItem::data(int role);
  fn C_ZNK15QListWidgetItem4dataEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QSize QListWidgetItem::sizeHint();
  fn C_ZNK15QListWidgetItem8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QListWidgetItem::setWhatsThis(const QString & whatsThis);
  fn C_ZN15QListWidgetItem12setWhatsThisERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QListWidgetItem::read(QDataStream & in);
  fn C_ZN15QListWidgetItem4readER11QDataStream(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QListWidgetItem::setTextColor(const QColor & color);
  fn C_ZN15QListWidgetItem12setTextColorERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QListWidgetItem::setSizeHint(const QSize & size);
  fn C_ZN15QListWidgetItem11setSizeHintERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QListWidget * QListWidgetItem::listWidget();
  fn C_ZNK15QListWidgetItem10listWidgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QListWidgetItem::setIcon(const QIcon & icon);
  fn C_ZN15QListWidgetItem7setIconERK5QIcon(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QListWidgetItem * QListWidgetItem::clone();
  fn C_ZNK15QListWidgetItem5cloneEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QListWidgetItem::setBackgroundColor(const QColor & color);
  fn C_ZN15QListWidgetItem18setBackgroundColorERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QListWidgetItem::setForeground(const QBrush & brush);
  fn C_ZN15QListWidgetItem13setForegroundERK6QBrush(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QListWidgetItem::QListWidgetItem(const QListWidgetItem & other);
  fn C_ZN15QListWidgetItemC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QListWidgetItem::setHidden(bool hide);
  fn C_ZN15QListWidgetItem9setHiddenEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QString QListWidgetItem::toolTip();
  fn C_ZNK15QListWidgetItem7toolTipEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QListWidgetItem::textAlignment();
  fn C_ZNK15QListWidgetItem13textAlignmentEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QString QListWidgetItem::statusTip();
  fn C_ZNK15QListWidgetItem9statusTipEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QListWidgetItem::setToolTip(const QString & toolTip);
  fn C_ZN15QListWidgetItem10setToolTipERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QListWidget_Class_Size() -> c_int;
  // proto:  void QListWidget::dropEvent(QDropEvent * event);
  fn C_ZN11QListWidget9dropEventEP10QDropEvent(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWidget * QListWidget::itemWidget(QListWidgetItem * item);
  fn C_ZNK11QListWidget10itemWidgetEP15QListWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QListWidget::QListWidget(QWidget * parent);
  fn C_ZN11QListWidgetC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  int QListWidget::currentRow();
  fn C_ZNK11QListWidget10currentRowEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QListWidgetItem * QListWidget::item(int row);
  fn C_ZNK11QListWidget4itemEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QListWidgetItem * QListWidget::itemAt(const QPoint & p);
  fn C_ZNK11QListWidget6itemAtERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QListWidget::insertItem(int row, const QString & label);
  fn C_ZN11QListWidget10insertItemEiRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  int QListWidget::row(const QListWidgetItem * item);
  fn C_ZNK11QListWidget3rowEPK15QListWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  void QListWidget::openPersistentEditor(QListWidgetItem * item);
  fn C_ZN11QListWidget20openPersistentEditorEP15QListWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QListWidget::clear();
  fn C_ZN11QListWidget5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QListWidget::editItem(QListWidgetItem * item);
  fn C_ZN11QListWidget8editItemEP15QListWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QListWidget::count();
  fn C_ZNK11QListWidget5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QListWidget::setItemHidden(const QListWidgetItem * item, bool hide);
  fn C_ZN11QListWidget13setItemHiddenEPK15QListWidgetItemb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_char);
  // proto:  void QListWidget::~QListWidget();
  fn C_ZN11QListWidgetD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QListWidget::addItem(QListWidgetItem * item);
  fn C_ZN11QListWidget7addItemEP15QListWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QListWidgetItem * QListWidget::takeItem(int row);
  fn C_ZN11QListWidget8takeItemEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  bool QListWidget::isSortingEnabled();
  fn C_ZNK11QListWidget16isSortingEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QListWidget::addItems(const QStringList & labels);
  fn C_ZN11QListWidget8addItemsERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QList<QListWidgetItem *> QListWidget::selectedItems();
  fn C_ZNK11QListWidget13selectedItemsEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QListWidget::metaObject();
  fn C_ZNK11QListWidget10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QListWidget::setItemSelected(const QListWidgetItem * item, bool select);
  fn C_ZN11QListWidget15setItemSelectedEPK15QListWidgetItemb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_char);
  // proto:  void QListWidget::insertItem(int row, QListWidgetItem * item);
  fn C_ZN11QListWidget10insertItemEiP15QListWidgetItem(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QListWidget::setCurrentRow(int row);
  fn C_ZN11QListWidget13setCurrentRowEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QListWidget::setSortingEnabled(bool enable);
  fn C_ZN11QListWidget17setSortingEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QRect QListWidget::visualItemRect(const QListWidgetItem * item);
  fn C_ZNK11QListWidget14visualItemRectEPK15QListWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QListWidget::removeItemWidget(QListWidgetItem * item);
  fn C_ZN11QListWidget16removeItemWidgetEP15QListWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QListWidget::closePersistentEditor(QListWidgetItem * item);
  fn C_ZN11QListWidget21closePersistentEditorEP15QListWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QListWidget::isItemHidden(const QListWidgetItem * item);
  fn C_ZNK11QListWidget12isItemHiddenEPK15QListWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QListWidgetItem * QListWidget::itemAt(int x, int y);
  fn C_ZNK11QListWidget6itemAtEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QListWidget::addItem(const QString & label);
  fn C_ZN11QListWidget7addItemERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QListWidget::insertItems(int row, const QStringList & labels);
  fn C_ZN11QListWidget11insertItemsEiRK11QStringList(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  QListWidgetItem * QListWidget::currentItem();
  fn C_ZNK11QListWidget11currentItemEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QListWidget::setCurrentItem(QListWidgetItem * item);
  fn C_ZN11QListWidget14setCurrentItemEP15QListWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QListWidget::setItemWidget(QListWidgetItem * item, QWidget * widget);
  fn C_ZN11QListWidget13setItemWidgetEP15QListWidgetItemP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QListWidget::isItemSelected(const QListWidgetItem * item);
  fn C_ZNK11QListWidget14isItemSelectedEPK15QListWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  fn QListWidget_SlotProxy_connect__ZN11QListWidget17itemDoubleClickedEP15QListWidgetItem(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QListWidget_SlotProxy_connect__ZN11QListWidget11itemEnteredEP15QListWidgetItem(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QListWidget_SlotProxy_connect__ZN11QListWidget11itemClickedEP15QListWidgetItem(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QListWidget_SlotProxy_connect__ZN11QListWidget18currentItemChangedEP15QListWidgetItemS1_(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QListWidget_SlotProxy_connect__ZN11QListWidget18currentTextChangedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QListWidget_SlotProxy_connect__ZN11QListWidget11itemChangedEP15QListWidgetItem(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QListWidget_SlotProxy_connect__ZN11QListWidget20itemSelectionChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QListWidget_SlotProxy_connect__ZN11QListWidget11itemPressedEP15QListWidgetItem(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QListWidget_SlotProxy_connect__ZN11QListWidget13itemActivatedEP15QListWidgetItem(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QListWidget_SlotProxy_connect__ZN11QListWidget17currentRowChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QListWidgetItem)=1
#[derive(Default)]
pub struct QListWidgetItem {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QListWidget)=1
#[derive(Default)]
pub struct QListWidget {
  qbase: QListView,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _itemDoubleClicked: QListWidget_itemDoubleClicked_signal,
  pub _itemClicked: QListWidget_itemClicked_signal,
  pub _currentItemChanged: QListWidget_currentItemChanged_signal,
  pub _itemEntered: QListWidget_itemEntered_signal,
  pub _itemPressed: QListWidget_itemPressed_signal,
  pub _itemSelectionChanged: QListWidget_itemSelectionChanged_signal,
  pub _itemActivated: QListWidget_itemActivated_signal,
  pub _itemChanged: QListWidget_itemChanged_signal,
  pub _currentRowChanged: QListWidget_currentRowChanged_signal,
  pub _currentTextChanged: QListWidget_currentTextChanged_signal,
}

impl /*struct*/ QListWidgetItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QListWidgetItem {
    return QListWidgetItem{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QListWidgetItem::~QListWidgetItem();
impl /*struct*/ QListWidgetItem {
  pub fn free<RetType, T: QListWidgetItem_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QListWidgetItem_free<RetType> {
  fn free(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  void QListWidgetItem::~QListWidgetItem();
impl<'a> /*trait*/ QListWidgetItem_free<()> for () {
  fn free(self , rsthis: & QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItemD2Ev()};
     unsafe {C_ZN15QListWidgetItemD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QListWidgetItem::isHidden();
impl /*struct*/ QListWidgetItem {
  pub fn isHidden<RetType, T: QListWidgetItem_isHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isHidden(self);
    // return 1;
  }
}

pub trait QListWidgetItem_isHidden<RetType> {
  fn isHidden(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  bool QListWidgetItem::isHidden();
impl<'a> /*trait*/ QListWidgetItem_isHidden<i8> for () {
  fn isHidden(self , rsthis: & QListWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem8isHiddenEv()};
    let mut ret = unsafe {C_ZNK15QListWidgetItem8isHiddenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QListWidgetItem::setData(int role, const QVariant & value);
impl /*struct*/ QListWidgetItem {
  pub fn setData<RetType, T: QListWidgetItem_setData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setData<RetType> {
  fn setData(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  void QListWidgetItem::setData(int role, const QVariant & value);
impl<'a> /*trait*/ QListWidgetItem_setData<()> for (i32, &'a QVariant) {
  fn setData(self , rsthis: & QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem7setDataEiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN15QListWidgetItem7setDataEiRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QListWidgetItem::setBackground(const QBrush & brush);
impl /*struct*/ QListWidgetItem {
  pub fn setBackground<RetType, T: QListWidgetItem_setBackground<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBackground(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setBackground<RetType> {
  fn setBackground(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  void QListWidgetItem::setBackground(const QBrush & brush);
impl<'a> /*trait*/ QListWidgetItem_setBackground<()> for (&'a QBrush) {
  fn setBackground(self , rsthis: & QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QListWidgetItem13setBackgroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidgetItem::setSelected(bool select);
impl /*struct*/ QListWidgetItem {
  pub fn setSelected<RetType, T: QListWidgetItem_setSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSelected(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setSelected<RetType> {
  fn setSelected(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  void QListWidgetItem::setSelected(bool select);
impl<'a> /*trait*/ QListWidgetItem_setSelected<()> for (i8) {
  fn setSelected(self , rsthis: & QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem11setSelectedEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN15QListWidgetItem11setSelectedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QFont QListWidgetItem::font();
impl /*struct*/ QListWidgetItem {
  pub fn font<RetType, T: QListWidgetItem_font<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QListWidgetItem_font<RetType> {
  fn font(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  QFont QListWidgetItem::font();
impl<'a> /*trait*/ QListWidgetItem_font<QFont> for () {
  fn font(self , rsthis: & QListWidgetItem) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem4fontEv()};
    let mut ret = unsafe {C_ZNK15QListWidgetItem4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QListWidgetItem::setTextAlignment(int alignment);
impl /*struct*/ QListWidgetItem {
  pub fn setTextAlignment<RetType, T: QListWidgetItem_setTextAlignment<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextAlignment(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setTextAlignment<RetType> {
  fn setTextAlignment(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  void QListWidgetItem::setTextAlignment(int alignment);
impl<'a> /*trait*/ QListWidgetItem_setTextAlignment<()> for (i32) {
  fn setTextAlignment(self , rsthis: & QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem16setTextAlignmentEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN15QListWidgetItem16setTextAlignmentEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidgetItem::QListWidgetItem(QListWidget * view, int type);
impl /*struct*/ QListWidgetItem {
  pub fn new<T: QListWidgetItem_new>(value: T) -> QListWidgetItem {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QListWidgetItem_new {
  fn new(self) -> QListWidgetItem;
}

  // proto:  void QListWidgetItem::QListWidgetItem(QListWidget * view, int type);
impl<'a> /*trait*/ QListWidgetItem_new for (&'a QListWidget, i32) {
  fn new(self) -> QListWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItemC2EP11QListWidgeti()};
    let ctysz: c_int = unsafe{QListWidgetItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let qthis: u64 = unsafe {C_ZN15QListWidgetItemC2EP11QListWidgeti(arg0, arg1)};
    let rsthis = QListWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QListWidgetItem::write(QDataStream & out);
impl /*struct*/ QListWidgetItem {
  pub fn write<RetType, T: QListWidgetItem_write<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.write(self);
    // return 1;
  }
}

pub trait QListWidgetItem_write<RetType> {
  fn write(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  void QListWidgetItem::write(QDataStream & out);
impl<'a> /*trait*/ QListWidgetItem_write<()> for (&'a QDataStream) {
  fn write(self , rsthis: & QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem5writeER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZNK15QListWidgetItem5writeER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QListWidgetItem::whatsThis();
impl /*struct*/ QListWidgetItem {
  pub fn whatsThis<RetType, T: QListWidgetItem_whatsThis<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.whatsThis(self);
    // return 1;
  }
}

pub trait QListWidgetItem_whatsThis<RetType> {
  fn whatsThis(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  QString QListWidgetItem::whatsThis();
impl<'a> /*trait*/ QListWidgetItem_whatsThis<QString> for () {
  fn whatsThis(self , rsthis: & QListWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem9whatsThisEv()};
    let mut ret = unsafe {C_ZNK15QListWidgetItem9whatsThisEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QListWidgetItem::type();
impl /*struct*/ QListWidgetItem {
  pub fn type_<RetType, T: QListWidgetItem_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QListWidgetItem_type_<RetType> {
  fn type_(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  int QListWidgetItem::type();
impl<'a> /*trait*/ QListWidgetItem_type_<i32> for () {
  fn type_(self , rsthis: & QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem4typeEv()};
    let mut ret = unsafe {C_ZNK15QListWidgetItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QListWidgetItem::QListWidgetItem(const QIcon & icon, const QString & text, QListWidget * view, int type);
impl<'a> /*trait*/ QListWidgetItem_new for (&'a QIcon, &'a QString, &'a QListWidget, i32) {
  fn new(self) -> QListWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItemC2ERK5QIconRK7QStringP11QListWidgeti()};
    let ctysz: c_int = unsafe{QListWidgetItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let qthis: u64 = unsafe {C_ZN15QListWidgetItemC2ERK5QIconRK7QStringP11QListWidgeti(arg0, arg1, arg2, arg3)};
    let rsthis = QListWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QIcon QListWidgetItem::icon();
impl /*struct*/ QListWidgetItem {
  pub fn icon<RetType, T: QListWidgetItem_icon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.icon(self);
    // return 1;
  }
}

pub trait QListWidgetItem_icon<RetType> {
  fn icon(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  QIcon QListWidgetItem::icon();
impl<'a> /*trait*/ QListWidgetItem_icon<QIcon> for () {
  fn icon(self , rsthis: & QListWidgetItem) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem4iconEv()};
    let mut ret = unsafe {C_ZNK15QListWidgetItem4iconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QColor QListWidgetItem::textColor();
impl /*struct*/ QListWidgetItem {
  pub fn textColor<RetType, T: QListWidgetItem_textColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textColor(self);
    // return 1;
  }
}

pub trait QListWidgetItem_textColor<RetType> {
  fn textColor(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  QColor QListWidgetItem::textColor();
impl<'a> /*trait*/ QListWidgetItem_textColor<QColor> for () {
  fn textColor(self , rsthis: & QListWidgetItem) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem9textColorEv()};
    let mut ret = unsafe {C_ZNK15QListWidgetItem9textColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QBrush QListWidgetItem::foreground();
impl /*struct*/ QListWidgetItem {
  pub fn foreground<RetType, T: QListWidgetItem_foreground<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.foreground(self);
    // return 1;
  }
}

pub trait QListWidgetItem_foreground<RetType> {
  fn foreground(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  QBrush QListWidgetItem::foreground();
impl<'a> /*trait*/ QListWidgetItem_foreground<QBrush> for () {
  fn foreground(self , rsthis: & QListWidgetItem) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem10foregroundEv()};
    let mut ret = unsafe {C_ZNK15QListWidgetItem10foregroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QBrush QListWidgetItem::background();
impl /*struct*/ QListWidgetItem {
  pub fn background<RetType, T: QListWidgetItem_background<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.background(self);
    // return 1;
  }
}

pub trait QListWidgetItem_background<RetType> {
  fn background(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  QBrush QListWidgetItem::background();
impl<'a> /*trait*/ QListWidgetItem_background<QBrush> for () {
  fn background(self , rsthis: & QListWidgetItem) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem10backgroundEv()};
    let mut ret = unsafe {C_ZNK15QListWidgetItem10backgroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QListWidgetItem::setStatusTip(const QString & statusTip);
impl /*struct*/ QListWidgetItem {
  pub fn setStatusTip<RetType, T: QListWidgetItem_setStatusTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStatusTip(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setStatusTip<RetType> {
  fn setStatusTip(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  void QListWidgetItem::setStatusTip(const QString & statusTip);
impl<'a> /*trait*/ QListWidgetItem_setStatusTip<()> for (&'a QString) {
  fn setStatusTip(self , rsthis: & QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem12setStatusTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QListWidgetItem12setStatusTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QListWidgetItem::text();
impl /*struct*/ QListWidgetItem {
  pub fn text<RetType, T: QListWidgetItem_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QListWidgetItem_text<RetType> {
  fn text(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  QString QListWidgetItem::text();
impl<'a> /*trait*/ QListWidgetItem_text<QString> for () {
  fn text(self , rsthis: & QListWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem4textEv()};
    let mut ret = unsafe {C_ZNK15QListWidgetItem4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QColor QListWidgetItem::backgroundColor();
impl /*struct*/ QListWidgetItem {
  pub fn backgroundColor<RetType, T: QListWidgetItem_backgroundColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.backgroundColor(self);
    // return 1;
  }
}

pub trait QListWidgetItem_backgroundColor<RetType> {
  fn backgroundColor(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  QColor QListWidgetItem::backgroundColor();
impl<'a> /*trait*/ QListWidgetItem_backgroundColor<QColor> for () {
  fn backgroundColor(self , rsthis: & QListWidgetItem) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem15backgroundColorEv()};
    let mut ret = unsafe {C_ZNK15QListWidgetItem15backgroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QListWidgetItem::isSelected();
impl /*struct*/ QListWidgetItem {
  pub fn isSelected<RetType, T: QListWidgetItem_isSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSelected(self);
    // return 1;
  }
}

pub trait QListWidgetItem_isSelected<RetType> {
  fn isSelected(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  bool QListWidgetItem::isSelected();
impl<'a> /*trait*/ QListWidgetItem_isSelected<i8> for () {
  fn isSelected(self , rsthis: & QListWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem10isSelectedEv()};
    let mut ret = unsafe {C_ZNK15QListWidgetItem10isSelectedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QListWidgetItem::setFont(const QFont & font);
impl /*struct*/ QListWidgetItem {
  pub fn setFont<RetType, T: QListWidgetItem_setFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFont(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setFont<RetType> {
  fn setFont(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  void QListWidgetItem::setFont(const QFont & font);
impl<'a> /*trait*/ QListWidgetItem_setFont<()> for (&'a QFont) {
  fn setFont(self , rsthis: & QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QListWidgetItem7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidgetItem::setText(const QString & text);
impl /*struct*/ QListWidgetItem {
  pub fn setText<RetType, T: QListWidgetItem_setText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setText<RetType> {
  fn setText(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  void QListWidgetItem::setText(const QString & text);
impl<'a> /*trait*/ QListWidgetItem_setText<()> for (&'a QString) {
  fn setText(self , rsthis: & QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QListWidgetItem7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidgetItem::QListWidgetItem(const QString & text, QListWidget * view, int type);
impl<'a> /*trait*/ QListWidgetItem_new for (&'a QString, &'a QListWidget, i32) {
  fn new(self) -> QListWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItemC2ERK7QStringP11QListWidgeti()};
    let ctysz: c_int = unsafe{QListWidgetItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let qthis: u64 = unsafe {C_ZN15QListWidgetItemC2ERK7QStringP11QListWidgeti(arg0, arg1, arg2)};
    let rsthis = QListWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QVariant QListWidgetItem::data(int role);
impl /*struct*/ QListWidgetItem {
  pub fn data<RetType, T: QListWidgetItem_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QListWidgetItem_data<RetType> {
  fn data(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  QVariant QListWidgetItem::data(int role);
impl<'a> /*trait*/ QListWidgetItem_data<QVariant> for (i32) {
  fn data(self , rsthis: & QListWidgetItem) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem4dataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK15QListWidgetItem4dataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QListWidgetItem::sizeHint();
impl /*struct*/ QListWidgetItem {
  pub fn sizeHint<RetType, T: QListWidgetItem_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QListWidgetItem_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  QSize QListWidgetItem::sizeHint();
impl<'a> /*trait*/ QListWidgetItem_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QListWidgetItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem8sizeHintEv()};
    let mut ret = unsafe {C_ZNK15QListWidgetItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QListWidgetItem::setWhatsThis(const QString & whatsThis);
impl /*struct*/ QListWidgetItem {
  pub fn setWhatsThis<RetType, T: QListWidgetItem_setWhatsThis<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWhatsThis(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setWhatsThis<RetType> {
  fn setWhatsThis(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  void QListWidgetItem::setWhatsThis(const QString & whatsThis);
impl<'a> /*trait*/ QListWidgetItem_setWhatsThis<()> for (&'a QString) {
  fn setWhatsThis(self , rsthis: & QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QListWidgetItem12setWhatsThisERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidgetItem::read(QDataStream & in);
impl /*struct*/ QListWidgetItem {
  pub fn read<RetType, T: QListWidgetItem_read<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.read(self);
    // return 1;
  }
}

pub trait QListWidgetItem_read<RetType> {
  fn read(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  void QListWidgetItem::read(QDataStream & in);
impl<'a> /*trait*/ QListWidgetItem_read<()> for (&'a QDataStream) {
  fn read(self , rsthis: & QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem4readER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QListWidgetItem4readER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidgetItem::setTextColor(const QColor & color);
impl /*struct*/ QListWidgetItem {
  pub fn setTextColor<RetType, T: QListWidgetItem_setTextColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextColor(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setTextColor<RetType> {
  fn setTextColor(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  void QListWidgetItem::setTextColor(const QColor & color);
impl<'a> /*trait*/ QListWidgetItem_setTextColor<()> for (&'a QColor) {
  fn setTextColor(self , rsthis: & QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem12setTextColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QListWidgetItem12setTextColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidgetItem::setSizeHint(const QSize & size);
impl /*struct*/ QListWidgetItem {
  pub fn setSizeHint<RetType, T: QListWidgetItem_setSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSizeHint(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setSizeHint<RetType> {
  fn setSizeHint(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  void QListWidgetItem::setSizeHint(const QSize & size);
impl<'a> /*trait*/ QListWidgetItem_setSizeHint<()> for (&'a QSize) {
  fn setSizeHint(self , rsthis: & QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem11setSizeHintERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QListWidgetItem11setSizeHintERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QListWidget * QListWidgetItem::listWidget();
impl /*struct*/ QListWidgetItem {
  pub fn listWidget<RetType, T: QListWidgetItem_listWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.listWidget(self);
    // return 1;
  }
}

pub trait QListWidgetItem_listWidget<RetType> {
  fn listWidget(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  QListWidget * QListWidgetItem::listWidget();
impl<'a> /*trait*/ QListWidgetItem_listWidget<QListWidget> for () {
  fn listWidget(self , rsthis: & QListWidgetItem) -> QListWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem10listWidgetEv()};
    let mut ret = unsafe {C_ZNK15QListWidgetItem10listWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QListWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QListWidgetItem::setIcon(const QIcon & icon);
impl /*struct*/ QListWidgetItem {
  pub fn setIcon<RetType, T: QListWidgetItem_setIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIcon(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setIcon<RetType> {
  fn setIcon(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  void QListWidgetItem::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QListWidgetItem_setIcon<()> for (&'a QIcon) {
  fn setIcon(self , rsthis: & QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QListWidgetItem7setIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QListWidgetItem * QListWidgetItem::clone();
impl /*struct*/ QListWidgetItem {
  pub fn clone<RetType, T: QListWidgetItem_clone<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clone(self);
    // return 1;
  }
}

pub trait QListWidgetItem_clone<RetType> {
  fn clone(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  QListWidgetItem * QListWidgetItem::clone();
impl<'a> /*trait*/ QListWidgetItem_clone<QListWidgetItem> for () {
  fn clone(self , rsthis: & QListWidgetItem) -> QListWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem5cloneEv()};
    let mut ret = unsafe {C_ZNK15QListWidgetItem5cloneEv(rsthis.qclsinst)};
    let mut ret1 = QListWidgetItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QListWidgetItem::setBackgroundColor(const QColor & color);
impl /*struct*/ QListWidgetItem {
  pub fn setBackgroundColor<RetType, T: QListWidgetItem_setBackgroundColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBackgroundColor(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setBackgroundColor<RetType> {
  fn setBackgroundColor(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  void QListWidgetItem::setBackgroundColor(const QColor & color);
impl<'a> /*trait*/ QListWidgetItem_setBackgroundColor<()> for (&'a QColor) {
  fn setBackgroundColor(self , rsthis: & QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem18setBackgroundColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QListWidgetItem18setBackgroundColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidgetItem::setForeground(const QBrush & brush);
impl /*struct*/ QListWidgetItem {
  pub fn setForeground<RetType, T: QListWidgetItem_setForeground<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setForeground(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setForeground<RetType> {
  fn setForeground(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  void QListWidgetItem::setForeground(const QBrush & brush);
impl<'a> /*trait*/ QListWidgetItem_setForeground<()> for (&'a QBrush) {
  fn setForeground(self , rsthis: & QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem13setForegroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QListWidgetItem13setForegroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidgetItem::QListWidgetItem(const QListWidgetItem & other);
impl<'a> /*trait*/ QListWidgetItem_new for (&'a QListWidgetItem) {
  fn new(self) -> QListWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItemC2ERKS_()};
    let ctysz: c_int = unsafe{QListWidgetItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN15QListWidgetItemC2ERKS_(arg0)};
    let rsthis = QListWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QListWidgetItem::setHidden(bool hide);
impl /*struct*/ QListWidgetItem {
  pub fn setHidden<RetType, T: QListWidgetItem_setHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHidden(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setHidden<RetType> {
  fn setHidden(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  void QListWidgetItem::setHidden(bool hide);
impl<'a> /*trait*/ QListWidgetItem_setHidden<()> for (i8) {
  fn setHidden(self , rsthis: & QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem9setHiddenEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN15QListWidgetItem9setHiddenEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QListWidgetItem::toolTip();
impl /*struct*/ QListWidgetItem {
  pub fn toolTip<RetType, T: QListWidgetItem_toolTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toolTip(self);
    // return 1;
  }
}

pub trait QListWidgetItem_toolTip<RetType> {
  fn toolTip(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  QString QListWidgetItem::toolTip();
impl<'a> /*trait*/ QListWidgetItem_toolTip<QString> for () {
  fn toolTip(self , rsthis: & QListWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem7toolTipEv()};
    let mut ret = unsafe {C_ZNK15QListWidgetItem7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QListWidgetItem::textAlignment();
impl /*struct*/ QListWidgetItem {
  pub fn textAlignment<RetType, T: QListWidgetItem_textAlignment<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textAlignment(self);
    // return 1;
  }
}

pub trait QListWidgetItem_textAlignment<RetType> {
  fn textAlignment(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  int QListWidgetItem::textAlignment();
impl<'a> /*trait*/ QListWidgetItem_textAlignment<i32> for () {
  fn textAlignment(self , rsthis: & QListWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem13textAlignmentEv()};
    let mut ret = unsafe {C_ZNK15QListWidgetItem13textAlignmentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString QListWidgetItem::statusTip();
impl /*struct*/ QListWidgetItem {
  pub fn statusTip<RetType, T: QListWidgetItem_statusTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.statusTip(self);
    // return 1;
  }
}

pub trait QListWidgetItem_statusTip<RetType> {
  fn statusTip(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  QString QListWidgetItem::statusTip();
impl<'a> /*trait*/ QListWidgetItem_statusTip<QString> for () {
  fn statusTip(self , rsthis: & QListWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QListWidgetItem9statusTipEv()};
    let mut ret = unsafe {C_ZNK15QListWidgetItem9statusTipEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QListWidgetItem::setToolTip(const QString & toolTip);
impl /*struct*/ QListWidgetItem {
  pub fn setToolTip<RetType, T: QListWidgetItem_setToolTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setToolTip(self);
    // return 1;
  }
}

pub trait QListWidgetItem_setToolTip<RetType> {
  fn setToolTip(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  void QListWidgetItem::setToolTip(const QString & toolTip);
impl<'a> /*trait*/ QListWidgetItem_setToolTip<()> for (&'a QString) {
  fn setToolTip(self , rsthis: & QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItem10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QListWidgetItem10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QListWidget {
    return QListWidget{qbase: QListView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QListWidget {
  type Target = QListView;

  fn deref(&self) -> &QListView {
    return & self.qbase;
  }
}
impl AsRef<QListView> for QListWidget {
  fn as_ref(& self) -> & QListView {
    return & self.qbase;
  }
}
  // proto:  void QListWidget::dropEvent(QDropEvent * event);
impl /*struct*/ QListWidget {
  pub fn dropEvent<RetType, T: QListWidget_dropEvent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dropEvent(self);
    // return 1;
  }
}

pub trait QListWidget_dropEvent<RetType> {
  fn dropEvent(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::dropEvent(QDropEvent * event);
impl<'a> /*trait*/ QListWidget_dropEvent<()> for (&'a QDropEvent) {
  fn dropEvent(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget9dropEventEP10QDropEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QListWidget9dropEventEP10QDropEvent(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QListWidget::itemWidget(QListWidgetItem * item);
impl /*struct*/ QListWidget {
  pub fn itemWidget<RetType, T: QListWidget_itemWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemWidget(self);
    // return 1;
  }
}

pub trait QListWidget_itemWidget<RetType> {
  fn itemWidget(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  QWidget * QListWidget::itemWidget(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemWidget<QWidget> for (&'a QListWidgetItem) {
  fn itemWidget(self , rsthis: & QListWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget10itemWidgetEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK11QListWidget10itemWidgetEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QListWidget::QListWidget(QWidget * parent);
impl /*struct*/ QListWidget {
  pub fn new<T: QListWidget_new>(value: T) -> QListWidget {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QListWidget_new {
  fn new(self) -> QListWidget;
}

  // proto:  void QListWidget::QListWidget(QWidget * parent);
impl<'a> /*trait*/ QListWidget_new for (&'a QWidget) {
  fn new(self) -> QListWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidgetC2EP7QWidget()};
    let ctysz: c_int = unsafe{QListWidget_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QListWidgetC2EP7QWidget(arg0)};
    let rsthis = QListWidget{qbase: QListView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QListWidget::currentRow();
impl /*struct*/ QListWidget {
  pub fn currentRow<RetType, T: QListWidget_currentRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentRow(self);
    // return 1;
  }
}

pub trait QListWidget_currentRow<RetType> {
  fn currentRow(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  int QListWidget::currentRow();
impl<'a> /*trait*/ QListWidget_currentRow<i32> for () {
  fn currentRow(self , rsthis: & QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget10currentRowEv()};
    let mut ret = unsafe {C_ZNK11QListWidget10currentRowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QListWidgetItem * QListWidget::item(int row);
impl /*struct*/ QListWidget {
  pub fn item<RetType, T: QListWidget_item<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.item(self);
    // return 1;
  }
}

pub trait QListWidget_item<RetType> {
  fn item(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  QListWidgetItem * QListWidget::item(int row);
impl<'a> /*trait*/ QListWidget_item<QListWidgetItem> for (i32) {
  fn item(self , rsthis: & QListWidget) -> QListWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget4itemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK11QListWidget4itemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QListWidgetItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QListWidgetItem * QListWidget::itemAt(const QPoint & p);
impl /*struct*/ QListWidget {
  pub fn itemAt<RetType, T: QListWidget_itemAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemAt(self);
    // return 1;
  }
}

pub trait QListWidget_itemAt<RetType> {
  fn itemAt(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  QListWidgetItem * QListWidget::itemAt(const QPoint & p);
impl<'a> /*trait*/ QListWidget_itemAt<QListWidgetItem> for (&'a QPoint) {
  fn itemAt(self , rsthis: & QListWidget) -> QListWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget6itemAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK11QListWidget6itemAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QListWidgetItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QListWidget::insertItem(int row, const QString & label);
impl /*struct*/ QListWidget {
  pub fn insertItem<RetType, T: QListWidget_insertItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertItem(self);
    // return 1;
  }
}

pub trait QListWidget_insertItem<RetType> {
  fn insertItem(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::insertItem(int row, const QString & label);
impl<'a> /*trait*/ QListWidget_insertItem<()> for (i32, &'a QString) {
  fn insertItem(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget10insertItemEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN11QListWidget10insertItemEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QListWidget::row(const QListWidgetItem * item);
impl /*struct*/ QListWidget {
  pub fn row<RetType, T: QListWidget_row<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.row(self);
    // return 1;
  }
}

pub trait QListWidget_row<RetType> {
  fn row(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  int QListWidget::row(const QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_row<i32> for (&'a QListWidgetItem) {
  fn row(self , rsthis: & QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget3rowEPK15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK11QListWidget3rowEPK15QListWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QListWidget::openPersistentEditor(QListWidgetItem * item);
impl /*struct*/ QListWidget {
  pub fn openPersistentEditor<RetType, T: QListWidget_openPersistentEditor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.openPersistentEditor(self);
    // return 1;
  }
}

pub trait QListWidget_openPersistentEditor<RetType> {
  fn openPersistentEditor(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::openPersistentEditor(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_openPersistentEditor<()> for (&'a QListWidgetItem) {
  fn openPersistentEditor(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget20openPersistentEditorEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QListWidget20openPersistentEditorEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidget::clear();
impl /*struct*/ QListWidget {
  pub fn clear<RetType, T: QListWidget_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QListWidget_clear<RetType> {
  fn clear(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::clear();
impl<'a> /*trait*/ QListWidget_clear<()> for () {
  fn clear(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget5clearEv()};
     unsafe {C_ZN11QListWidget5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QListWidget::editItem(QListWidgetItem * item);
impl /*struct*/ QListWidget {
  pub fn editItem<RetType, T: QListWidget_editItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.editItem(self);
    // return 1;
  }
}

pub trait QListWidget_editItem<RetType> {
  fn editItem(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::editItem(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_editItem<()> for (&'a QListWidgetItem) {
  fn editItem(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget8editItemEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QListWidget8editItemEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QListWidget::count();
impl /*struct*/ QListWidget {
  pub fn count<RetType, T: QListWidget_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QListWidget_count<RetType> {
  fn count(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  int QListWidget::count();
impl<'a> /*trait*/ QListWidget_count<i32> for () {
  fn count(self , rsthis: & QListWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget5countEv()};
    let mut ret = unsafe {C_ZNK11QListWidget5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QListWidget::setItemHidden(const QListWidgetItem * item, bool hide);
impl /*struct*/ QListWidget {
  pub fn setItemHidden<RetType, T: QListWidget_setItemHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemHidden(self);
    // return 1;
  }
}

pub trait QListWidget_setItemHidden<RetType> {
  fn setItemHidden(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::setItemHidden(const QListWidgetItem * item, bool hide);
impl<'a> /*trait*/ QListWidget_setItemHidden<()> for (&'a QListWidgetItem, i8) {
  fn setItemHidden(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget13setItemHiddenEPK15QListWidgetItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {C_ZN11QListWidget13setItemHiddenEPK15QListWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QListWidget::~QListWidget();
impl /*struct*/ QListWidget {
  pub fn free<RetType, T: QListWidget_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QListWidget_free<RetType> {
  fn free(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::~QListWidget();
impl<'a> /*trait*/ QListWidget_free<()> for () {
  fn free(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidgetD2Ev()};
     unsafe {C_ZN11QListWidgetD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QListWidget::addItem(QListWidgetItem * item);
impl /*struct*/ QListWidget {
  pub fn addItem<RetType, T: QListWidget_addItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addItem(self);
    // return 1;
  }
}

pub trait QListWidget_addItem<RetType> {
  fn addItem(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::addItem(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_addItem<()> for (&'a QListWidgetItem) {
  fn addItem(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget7addItemEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QListWidget7addItemEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QListWidgetItem * QListWidget::takeItem(int row);
impl /*struct*/ QListWidget {
  pub fn takeItem<RetType, T: QListWidget_takeItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeItem(self);
    // return 1;
  }
}

pub trait QListWidget_takeItem<RetType> {
  fn takeItem(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  QListWidgetItem * QListWidget::takeItem(int row);
impl<'a> /*trait*/ QListWidget_takeItem<QListWidgetItem> for (i32) {
  fn takeItem(self , rsthis: & QListWidget) -> QListWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget8takeItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN11QListWidget8takeItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QListWidgetItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QListWidget::isSortingEnabled();
impl /*struct*/ QListWidget {
  pub fn isSortingEnabled<RetType, T: QListWidget_isSortingEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSortingEnabled(self);
    // return 1;
  }
}

pub trait QListWidget_isSortingEnabled<RetType> {
  fn isSortingEnabled(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  bool QListWidget::isSortingEnabled();
impl<'a> /*trait*/ QListWidget_isSortingEnabled<i8> for () {
  fn isSortingEnabled(self , rsthis: & QListWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget16isSortingEnabledEv()};
    let mut ret = unsafe {C_ZNK11QListWidget16isSortingEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QListWidget::addItems(const QStringList & labels);
impl /*struct*/ QListWidget {
  pub fn addItems<RetType, T: QListWidget_addItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addItems(self);
    // return 1;
  }
}

pub trait QListWidget_addItems<RetType> {
  fn addItems(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::addItems(const QStringList & labels);
impl<'a> /*trait*/ QListWidget_addItems<()> for (&'a QStringList) {
  fn addItems(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget8addItemsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QListWidget8addItemsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<QListWidgetItem *> QListWidget::selectedItems();
impl /*struct*/ QListWidget {
  pub fn selectedItems<RetType, T: QListWidget_selectedItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectedItems(self);
    // return 1;
  }
}

pub trait QListWidget_selectedItems<RetType> {
  fn selectedItems(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  QList<QListWidgetItem *> QListWidget::selectedItems();
impl<'a> /*trait*/ QListWidget_selectedItems<()> for () {
  fn selectedItems(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget13selectedItemsEv()};
     unsafe {C_ZNK11QListWidget13selectedItemsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QListWidget::metaObject();
impl /*struct*/ QListWidget {
  pub fn metaObject<RetType, T: QListWidget_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QListWidget_metaObject<RetType> {
  fn metaObject(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  const QMetaObject * QListWidget::metaObject();
impl<'a> /*trait*/ QListWidget_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QListWidget) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget10metaObjectEv()};
    let mut ret = unsafe {C_ZNK11QListWidget10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QListWidget::setItemSelected(const QListWidgetItem * item, bool select);
impl /*struct*/ QListWidget {
  pub fn setItemSelected<RetType, T: QListWidget_setItemSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemSelected(self);
    // return 1;
  }
}

pub trait QListWidget_setItemSelected<RetType> {
  fn setItemSelected(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::setItemSelected(const QListWidgetItem * item, bool select);
impl<'a> /*trait*/ QListWidget_setItemSelected<()> for (&'a QListWidgetItem, i8) {
  fn setItemSelected(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget15setItemSelectedEPK15QListWidgetItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {C_ZN11QListWidget15setItemSelectedEPK15QListWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QListWidget::insertItem(int row, QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_insertItem<()> for (i32, &'a QListWidgetItem) {
  fn insertItem(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget10insertItemEiP15QListWidgetItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN11QListWidget10insertItemEiP15QListWidgetItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QListWidget::setCurrentRow(int row);
impl /*struct*/ QListWidget {
  pub fn setCurrentRow<RetType, T: QListWidget_setCurrentRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentRow(self);
    // return 1;
  }
}

pub trait QListWidget_setCurrentRow<RetType> {
  fn setCurrentRow(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::setCurrentRow(int row);
impl<'a> /*trait*/ QListWidget_setCurrentRow<()> for (i32) {
  fn setCurrentRow(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget13setCurrentRowEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN11QListWidget13setCurrentRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidget::setSortingEnabled(bool enable);
impl /*struct*/ QListWidget {
  pub fn setSortingEnabled<RetType, T: QListWidget_setSortingEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSortingEnabled(self);
    // return 1;
  }
}

pub trait QListWidget_setSortingEnabled<RetType> {
  fn setSortingEnabled(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::setSortingEnabled(bool enable);
impl<'a> /*trait*/ QListWidget_setSortingEnabled<()> for (i8) {
  fn setSortingEnabled(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget17setSortingEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN11QListWidget17setSortingEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QListWidget::visualItemRect(const QListWidgetItem * item);
impl /*struct*/ QListWidget {
  pub fn visualItemRect<RetType, T: QListWidget_visualItemRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.visualItemRect(self);
    // return 1;
  }
}

pub trait QListWidget_visualItemRect<RetType> {
  fn visualItemRect(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  QRect QListWidget::visualItemRect(const QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_visualItemRect<QRect> for (&'a QListWidgetItem) {
  fn visualItemRect(self , rsthis: & QListWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget14visualItemRectEPK15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK11QListWidget14visualItemRectEPK15QListWidgetItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QListWidget::removeItemWidget(QListWidgetItem * item);
impl /*struct*/ QListWidget {
  pub fn removeItemWidget<RetType, T: QListWidget_removeItemWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeItemWidget(self);
    // return 1;
  }
}

pub trait QListWidget_removeItemWidget<RetType> {
  fn removeItemWidget(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::removeItemWidget(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_removeItemWidget<()> for (&'a QListWidgetItem) {
  fn removeItemWidget(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget16removeItemWidgetEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QListWidget16removeItemWidgetEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidget::closePersistentEditor(QListWidgetItem * item);
impl /*struct*/ QListWidget {
  pub fn closePersistentEditor<RetType, T: QListWidget_closePersistentEditor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.closePersistentEditor(self);
    // return 1;
  }
}

pub trait QListWidget_closePersistentEditor<RetType> {
  fn closePersistentEditor(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::closePersistentEditor(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_closePersistentEditor<()> for (&'a QListWidgetItem) {
  fn closePersistentEditor(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget21closePersistentEditorEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QListWidget21closePersistentEditorEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QListWidget::isItemHidden(const QListWidgetItem * item);
impl /*struct*/ QListWidget {
  pub fn isItemHidden<RetType, T: QListWidget_isItemHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isItemHidden(self);
    // return 1;
  }
}

pub trait QListWidget_isItemHidden<RetType> {
  fn isItemHidden(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  bool QListWidget::isItemHidden(const QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_isItemHidden<i8> for (&'a QListWidgetItem) {
  fn isItemHidden(self , rsthis: & QListWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget12isItemHiddenEPK15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK11QListWidget12isItemHiddenEPK15QListWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QListWidgetItem * QListWidget::itemAt(int x, int y);
impl<'a> /*trait*/ QListWidget_itemAt<QListWidgetItem> for (i32, i32) {
  fn itemAt(self , rsthis: & QListWidget) -> QListWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget6itemAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK11QListWidget6itemAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QListWidgetItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QListWidget::addItem(const QString & label);
impl<'a> /*trait*/ QListWidget_addItem<()> for (&'a QString) {
  fn addItem(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget7addItemERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QListWidget7addItemERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidget::insertItems(int row, const QStringList & labels);
impl /*struct*/ QListWidget {
  pub fn insertItems<RetType, T: QListWidget_insertItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertItems(self);
    // return 1;
  }
}

pub trait QListWidget_insertItems<RetType> {
  fn insertItems(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::insertItems(int row, const QStringList & labels);
impl<'a> /*trait*/ QListWidget_insertItems<()> for (i32, &'a QStringList) {
  fn insertItems(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget11insertItemsEiRK11QStringList()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN11QListWidget11insertItemsEiRK11QStringList(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QListWidgetItem * QListWidget::currentItem();
impl /*struct*/ QListWidget {
  pub fn currentItem<RetType, T: QListWidget_currentItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentItem(self);
    // return 1;
  }
}

pub trait QListWidget_currentItem<RetType> {
  fn currentItem(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  QListWidgetItem * QListWidget::currentItem();
impl<'a> /*trait*/ QListWidget_currentItem<QListWidgetItem> for () {
  fn currentItem(self , rsthis: & QListWidget) -> QListWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget11currentItemEv()};
    let mut ret = unsafe {C_ZNK11QListWidget11currentItemEv(rsthis.qclsinst)};
    let mut ret1 = QListWidgetItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QListWidget::setCurrentItem(QListWidgetItem * item);
impl /*struct*/ QListWidget {
  pub fn setCurrentItem<RetType, T: QListWidget_setCurrentItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentItem(self);
    // return 1;
  }
}

pub trait QListWidget_setCurrentItem<RetType> {
  fn setCurrentItem(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::setCurrentItem(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_setCurrentItem<()> for (&'a QListWidgetItem) {
  fn setCurrentItem(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget14setCurrentItemEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QListWidget14setCurrentItemEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidget::setItemWidget(QListWidgetItem * item, QWidget * widget);
impl /*struct*/ QListWidget {
  pub fn setItemWidget<RetType, T: QListWidget_setItemWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemWidget(self);
    // return 1;
  }
}

pub trait QListWidget_setItemWidget<RetType> {
  fn setItemWidget(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::setItemWidget(QListWidgetItem * item, QWidget * widget);
impl<'a> /*trait*/ QListWidget_setItemWidget<()> for (&'a QListWidgetItem, &'a QWidget) {
  fn setItemWidget(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget13setItemWidgetEP15QListWidgetItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN11QListWidget13setItemWidgetEP15QListWidgetItemP7QWidget(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QListWidget::isItemSelected(const QListWidgetItem * item);
impl /*struct*/ QListWidget {
  pub fn isItemSelected<RetType, T: QListWidget_isItemSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isItemSelected(self);
    // return 1;
  }
}

pub trait QListWidget_isItemSelected<RetType> {
  fn isItemSelected(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  bool QListWidget::isItemSelected(const QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_isItemSelected<i8> for (&'a QListWidgetItem) {
  fn isItemSelected(self , rsthis: & QListWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget14isItemSelectedEPK15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK11QListWidget14isItemSelectedEPK15QListWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

#[derive(Default)] // for QListWidget_itemDoubleClicked
pub struct QListWidget_itemDoubleClicked_signal{poi:u64}
impl /* struct */ QListWidget {
  pub fn itemDoubleClicked(&self) -> QListWidget_itemDoubleClicked_signal {
     return QListWidget_itemDoubleClicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QListWidget_itemDoubleClicked_signal {
  pub fn connect<T: QListWidget_itemDoubleClicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QListWidget_itemDoubleClicked_signal_connect {
  fn connect(self, sigthis: QListWidget_itemDoubleClicked_signal);
}

#[derive(Default)] // for QListWidget_itemClicked
pub struct QListWidget_itemClicked_signal{poi:u64}
impl /* struct */ QListWidget {
  pub fn itemClicked(&self) -> QListWidget_itemClicked_signal {
     return QListWidget_itemClicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QListWidget_itemClicked_signal {
  pub fn connect<T: QListWidget_itemClicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QListWidget_itemClicked_signal_connect {
  fn connect(self, sigthis: QListWidget_itemClicked_signal);
}

#[derive(Default)] // for QListWidget_currentItemChanged
pub struct QListWidget_currentItemChanged_signal{poi:u64}
impl /* struct */ QListWidget {
  pub fn currentItemChanged(&self) -> QListWidget_currentItemChanged_signal {
     return QListWidget_currentItemChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QListWidget_currentItemChanged_signal {
  pub fn connect<T: QListWidget_currentItemChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QListWidget_currentItemChanged_signal_connect {
  fn connect(self, sigthis: QListWidget_currentItemChanged_signal);
}

#[derive(Default)] // for QListWidget_itemEntered
pub struct QListWidget_itemEntered_signal{poi:u64}
impl /* struct */ QListWidget {
  pub fn itemEntered(&self) -> QListWidget_itemEntered_signal {
     return QListWidget_itemEntered_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QListWidget_itemEntered_signal {
  pub fn connect<T: QListWidget_itemEntered_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QListWidget_itemEntered_signal_connect {
  fn connect(self, sigthis: QListWidget_itemEntered_signal);
}

#[derive(Default)] // for QListWidget_itemPressed
pub struct QListWidget_itemPressed_signal{poi:u64}
impl /* struct */ QListWidget {
  pub fn itemPressed(&self) -> QListWidget_itemPressed_signal {
     return QListWidget_itemPressed_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QListWidget_itemPressed_signal {
  pub fn connect<T: QListWidget_itemPressed_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QListWidget_itemPressed_signal_connect {
  fn connect(self, sigthis: QListWidget_itemPressed_signal);
}

#[derive(Default)] // for QListWidget_itemSelectionChanged
pub struct QListWidget_itemSelectionChanged_signal{poi:u64}
impl /* struct */ QListWidget {
  pub fn itemSelectionChanged(&self) -> QListWidget_itemSelectionChanged_signal {
     return QListWidget_itemSelectionChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QListWidget_itemSelectionChanged_signal {
  pub fn connect<T: QListWidget_itemSelectionChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QListWidget_itemSelectionChanged_signal_connect {
  fn connect(self, sigthis: QListWidget_itemSelectionChanged_signal);
}

#[derive(Default)] // for QListWidget_itemActivated
pub struct QListWidget_itemActivated_signal{poi:u64}
impl /* struct */ QListWidget {
  pub fn itemActivated(&self) -> QListWidget_itemActivated_signal {
     return QListWidget_itemActivated_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QListWidget_itemActivated_signal {
  pub fn connect<T: QListWidget_itemActivated_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QListWidget_itemActivated_signal_connect {
  fn connect(self, sigthis: QListWidget_itemActivated_signal);
}

#[derive(Default)] // for QListWidget_itemChanged
pub struct QListWidget_itemChanged_signal{poi:u64}
impl /* struct */ QListWidget {
  pub fn itemChanged(&self) -> QListWidget_itemChanged_signal {
     return QListWidget_itemChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QListWidget_itemChanged_signal {
  pub fn connect<T: QListWidget_itemChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QListWidget_itemChanged_signal_connect {
  fn connect(self, sigthis: QListWidget_itemChanged_signal);
}

#[derive(Default)] // for QListWidget_currentRowChanged
pub struct QListWidget_currentRowChanged_signal{poi:u64}
impl /* struct */ QListWidget {
  pub fn currentRowChanged(&self) -> QListWidget_currentRowChanged_signal {
     return QListWidget_currentRowChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QListWidget_currentRowChanged_signal {
  pub fn connect<T: QListWidget_currentRowChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QListWidget_currentRowChanged_signal_connect {
  fn connect(self, sigthis: QListWidget_currentRowChanged_signal);
}

#[derive(Default)] // for QListWidget_currentTextChanged
pub struct QListWidget_currentTextChanged_signal{poi:u64}
impl /* struct */ QListWidget {
  pub fn currentTextChanged(&self) -> QListWidget_currentTextChanged_signal {
     return QListWidget_currentTextChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QListWidget_currentTextChanged_signal {
  pub fn connect<T: QListWidget_currentTextChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QListWidget_currentTextChanged_signal_connect {
  fn connect(self, sigthis: QListWidget_currentTextChanged_signal);
}

// itemDoubleClicked(class QListWidgetItem *)
extern fn QListWidget_itemDoubleClicked_signal_connect_cb_0(rsfptr:fn(QListWidgetItem), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QListWidgetItem::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QListWidget_itemDoubleClicked_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QListWidgetItem)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QListWidgetItem::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QListWidget_itemDoubleClicked_signal_connect for fn(QListWidgetItem) {
  fn connect(self, sigthis: QListWidget_itemDoubleClicked_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_itemDoubleClicked_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget17itemDoubleClickedEP15QListWidgetItem(arg0, arg1, arg2)};
  }
}
impl /* trait */ QListWidget_itemDoubleClicked_signal_connect for Box<Fn(QListWidgetItem)> {
  fn connect(self, sigthis: QListWidget_itemDoubleClicked_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_itemDoubleClicked_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget17itemDoubleClickedEP15QListWidgetItem(arg0, arg1, arg2)};
  }
}
// itemEntered(class QListWidgetItem *)
extern fn QListWidget_itemEntered_signal_connect_cb_1(rsfptr:fn(QListWidgetItem), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QListWidgetItem::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QListWidget_itemEntered_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QListWidgetItem)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QListWidgetItem::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QListWidget_itemEntered_signal_connect for fn(QListWidgetItem) {
  fn connect(self, sigthis: QListWidget_itemEntered_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_itemEntered_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget11itemEnteredEP15QListWidgetItem(arg0, arg1, arg2)};
  }
}
impl /* trait */ QListWidget_itemEntered_signal_connect for Box<Fn(QListWidgetItem)> {
  fn connect(self, sigthis: QListWidget_itemEntered_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_itemEntered_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget11itemEnteredEP15QListWidgetItem(arg0, arg1, arg2)};
  }
}
// itemClicked(class QListWidgetItem *)
extern fn QListWidget_itemClicked_signal_connect_cb_2(rsfptr:fn(QListWidgetItem), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QListWidgetItem::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QListWidget_itemClicked_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(QListWidgetItem)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QListWidgetItem::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QListWidget_itemClicked_signal_connect for fn(QListWidgetItem) {
  fn connect(self, sigthis: QListWidget_itemClicked_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_itemClicked_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget11itemClickedEP15QListWidgetItem(arg0, arg1, arg2)};
  }
}
impl /* trait */ QListWidget_itemClicked_signal_connect for Box<Fn(QListWidgetItem)> {
  fn connect(self, sigthis: QListWidget_itemClicked_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_itemClicked_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget11itemClickedEP15QListWidgetItem(arg0, arg1, arg2)};
  }
}
// currentItemChanged(class QListWidgetItem *, class QListWidgetItem *)
extern fn QListWidget_currentItemChanged_signal_connect_cb_3(rsfptr:fn(QListWidgetItem, QListWidgetItem), arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QListWidgetItem::inheritFrom(arg0 as u64);
  let rsarg1 = QListWidgetItem::inheritFrom(arg1 as u64);
  rsfptr(rsarg0,rsarg1);
}
extern fn QListWidget_currentItemChanged_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(QListWidgetItem, QListWidgetItem)>, arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QListWidgetItem::inheritFrom(arg0 as u64);
  let rsarg1 = QListWidgetItem::inheritFrom(arg1 as u64);
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QListWidget_currentItemChanged_signal_connect for fn(QListWidgetItem, QListWidgetItem) {
  fn connect(self, sigthis: QListWidget_currentItemChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_currentItemChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget18currentItemChangedEP15QListWidgetItemS1_(arg0, arg1, arg2)};
  }
}
impl /* trait */ QListWidget_currentItemChanged_signal_connect for Box<Fn(QListWidgetItem, QListWidgetItem)> {
  fn connect(self, sigthis: QListWidget_currentItemChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_currentItemChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget18currentItemChangedEP15QListWidgetItemS1_(arg0, arg1, arg2)};
  }
}
// currentTextChanged(const class QString &)
extern fn QListWidget_currentTextChanged_signal_connect_cb_4(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QListWidget_currentTextChanged_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QListWidget_currentTextChanged_signal_connect for fn(QString) {
  fn connect(self, sigthis: QListWidget_currentTextChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_currentTextChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget18currentTextChangedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QListWidget_currentTextChanged_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QListWidget_currentTextChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_currentTextChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget18currentTextChangedERK7QString(arg0, arg1, arg2)};
  }
}
// itemChanged(class QListWidgetItem *)
extern fn QListWidget_itemChanged_signal_connect_cb_5(rsfptr:fn(QListWidgetItem), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QListWidgetItem::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QListWidget_itemChanged_signal_connect_cb_box_5(rsfptr_raw:*mut Box<Fn(QListWidgetItem)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QListWidgetItem::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QListWidget_itemChanged_signal_connect for fn(QListWidgetItem) {
  fn connect(self, sigthis: QListWidget_itemChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_itemChanged_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget11itemChangedEP15QListWidgetItem(arg0, arg1, arg2)};
  }
}
impl /* trait */ QListWidget_itemChanged_signal_connect for Box<Fn(QListWidgetItem)> {
  fn connect(self, sigthis: QListWidget_itemChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_itemChanged_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget11itemChangedEP15QListWidgetItem(arg0, arg1, arg2)};
  }
}
// itemSelectionChanged()
extern fn QListWidget_itemSelectionChanged_signal_connect_cb_6(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QListWidget_itemSelectionChanged_signal_connect_cb_box_6(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QListWidget_itemSelectionChanged_signal_connect for fn() {
  fn connect(self, sigthis: QListWidget_itemSelectionChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_itemSelectionChanged_signal_connect_cb_6 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget20itemSelectionChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QListWidget_itemSelectionChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QListWidget_itemSelectionChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_itemSelectionChanged_signal_connect_cb_box_6 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget20itemSelectionChangedEv(arg0, arg1, arg2)};
  }
}
// itemPressed(class QListWidgetItem *)
extern fn QListWidget_itemPressed_signal_connect_cb_7(rsfptr:fn(QListWidgetItem), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QListWidgetItem::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QListWidget_itemPressed_signal_connect_cb_box_7(rsfptr_raw:*mut Box<Fn(QListWidgetItem)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QListWidgetItem::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QListWidget_itemPressed_signal_connect for fn(QListWidgetItem) {
  fn connect(self, sigthis: QListWidget_itemPressed_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_itemPressed_signal_connect_cb_7 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget11itemPressedEP15QListWidgetItem(arg0, arg1, arg2)};
  }
}
impl /* trait */ QListWidget_itemPressed_signal_connect for Box<Fn(QListWidgetItem)> {
  fn connect(self, sigthis: QListWidget_itemPressed_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_itemPressed_signal_connect_cb_box_7 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget11itemPressedEP15QListWidgetItem(arg0, arg1, arg2)};
  }
}
// itemActivated(class QListWidgetItem *)
extern fn QListWidget_itemActivated_signal_connect_cb_8(rsfptr:fn(QListWidgetItem), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QListWidgetItem::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QListWidget_itemActivated_signal_connect_cb_box_8(rsfptr_raw:*mut Box<Fn(QListWidgetItem)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QListWidgetItem::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QListWidget_itemActivated_signal_connect for fn(QListWidgetItem) {
  fn connect(self, sigthis: QListWidget_itemActivated_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_itemActivated_signal_connect_cb_8 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget13itemActivatedEP15QListWidgetItem(arg0, arg1, arg2)};
  }
}
impl /* trait */ QListWidget_itemActivated_signal_connect for Box<Fn(QListWidgetItem)> {
  fn connect(self, sigthis: QListWidget_itemActivated_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_itemActivated_signal_connect_cb_box_8 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget13itemActivatedEP15QListWidgetItem(arg0, arg1, arg2)};
  }
}
// currentRowChanged(int)
extern fn QListWidget_currentRowChanged_signal_connect_cb_9(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QListWidget_currentRowChanged_signal_connect_cb_box_9(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QListWidget_currentRowChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QListWidget_currentRowChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_currentRowChanged_signal_connect_cb_9 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget17currentRowChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QListWidget_currentRowChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QListWidget_currentRowChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QListWidget_currentRowChanged_signal_connect_cb_box_9 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QListWidget_SlotProxy_connect__ZN11QListWidget17currentRowChangedEi(arg0, arg1, arg2)};
  }
}
// <= body block end

