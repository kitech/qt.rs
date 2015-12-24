// auto generated, do not modify.
// created: Thu Dec 24 23:00:39 2015
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
use super::super::core::qrect::QRect; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]

// #[link(name = "QtInline")]

extern {
  // proto:  void QListWidgetItem::~QListWidgetItem();
  fn _ZN15QListWidgetItemD0Ev(qthis: *mut c_void);
  // proto:  bool QListWidgetItem::isHidden();
  fn _ZNK15QListWidgetItem8isHiddenEv(qthis: *mut c_void) -> c_char;
  // proto:  void QListWidgetItem::setData(int role, const QVariant & value);
  fn _ZN15QListWidgetItem7setDataEiRK8QVariant(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QListWidgetItem::setBackground(const QBrush & brush);
  fn _ZN15QListWidgetItem13setBackgroundERK6QBrush(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidgetItem::setSelected(bool select);
  fn _ZN15QListWidgetItem11setSelectedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QFont QListWidgetItem::font();
  fn _ZNK15QListWidgetItem4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QListWidgetItem::setTextAlignment(int alignment);
  fn _ZN15QListWidgetItem16setTextAlignmentEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QListWidgetItem::QListWidgetItem(QListWidget * view, int type);
  fn _ZN15QListWidgetItemC1EP11QListWidgeti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QListWidgetItem::write(QDataStream & out);
  fn _ZNK15QListWidgetItem5writeER11QDataStream(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QListWidgetItem::whatsThis();
  fn _ZNK15QListWidgetItem9whatsThisEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QListWidgetItem::type();
  fn _ZNK15QListWidgetItem4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QListWidgetItem::QListWidgetItem(const QIcon & icon, const QString & text, QListWidget * view, int type);
  fn _ZN15QListWidgetItemC1ERK5QIconRK7QStringP11QListWidgeti(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: c_int);
  // proto:  QIcon QListWidgetItem::icon();
  fn _ZNK15QListWidgetItem4iconEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QColor QListWidgetItem::textColor();
  fn _ZNK15QListWidgetItem9textColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QBrush QListWidgetItem::foreground();
  fn _ZNK15QListWidgetItem10foregroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QBrush QListWidgetItem::background();
  fn _ZNK15QListWidgetItem10backgroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QListWidgetItem::setStatusTip(const QString & statusTip);
  fn _ZN15QListWidgetItem12setStatusTipERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QListWidgetItem::text();
  fn _ZNK15QListWidgetItem4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QColor QListWidgetItem::backgroundColor();
  fn _ZNK15QListWidgetItem15backgroundColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QListWidgetItem::isSelected();
  fn _ZNK15QListWidgetItem10isSelectedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QListWidgetItem::setFont(const QFont & font);
  fn _ZN15QListWidgetItem7setFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidgetItem::setText(const QString & text);
  fn _ZN15QListWidgetItem7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidgetItem::QListWidgetItem(const QString & text, QListWidget * view, int type);
  fn _ZN15QListWidgetItemC1ERK7QStringP11QListWidgeti(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int);
  // proto:  QVariant QListWidgetItem::data(int role);
  fn _ZNK15QListWidgetItem4dataEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QSize QListWidgetItem::sizeHint();
  fn _ZNK15QListWidgetItem8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QListWidgetItem::setWhatsThis(const QString & whatsThis);
  fn _ZN15QListWidgetItem12setWhatsThisERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidgetItem::read(QDataStream & in);
  fn _ZN15QListWidgetItem4readER11QDataStream(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidgetItem::setTextColor(const QColor & color);
  fn _ZN15QListWidgetItem12setTextColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidgetItem::setSizeHint(const QSize & size);
  fn _ZN15QListWidgetItem11setSizeHintERK5QSize(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QListWidget * QListWidgetItem::listWidget();
  fn _ZNK15QListWidgetItem10listWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QListWidgetItem::setIcon(const QIcon & icon);
  fn _ZN15QListWidgetItem7setIconERK5QIcon(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QListWidgetItem * QListWidgetItem::clone();
  fn _ZNK15QListWidgetItem5cloneEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QListWidgetItem::setBackgroundColor(const QColor & color);
  fn _ZN15QListWidgetItem18setBackgroundColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidgetItem::setForeground(const QBrush & brush);
  fn _ZN15QListWidgetItem13setForegroundERK6QBrush(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidgetItem::QListWidgetItem(const QListWidgetItem & other);
  fn _ZN15QListWidgetItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidgetItem::setHidden(bool hide);
  fn _ZN15QListWidgetItem9setHiddenEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QString QListWidgetItem::toolTip();
  fn _ZNK15QListWidgetItem7toolTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QListWidgetItem::textAlignment();
  fn _ZNK15QListWidgetItem13textAlignmentEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QListWidgetItem::statusTip();
  fn _ZNK15QListWidgetItem9statusTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QListWidgetItem::setToolTip(const QString & toolTip);
  fn _ZN15QListWidgetItem10setToolTipERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidget::dropEvent(QDropEvent * event);
  fn _ZN11QListWidget9dropEventEP10QDropEvent(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidget::itemClicked(QListWidgetItem * item);
  fn _ZN11QListWidget11itemClickedEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QWidget * QListWidget::itemWidget(QListWidgetItem * item);
  fn _ZNK11QListWidget10itemWidgetEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QListWidget::itemPressed(QListWidgetItem * item);
  fn _ZN11QListWidget11itemPressedEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidget::QListWidget(QWidget * parent);
  fn _ZN11QListWidgetC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidget::currentRowChanged(int currentRow);
  fn _ZN11QListWidget17currentRowChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QListWidget::currentRow();
  fn _ZNK11QListWidget10currentRowEv(qthis: *mut c_void) -> c_int;
  // proto:  QListWidgetItem * QListWidget::item(int row);
  fn _ZNK11QListWidget4itemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QListWidgetItem * QListWidget::itemAt(const QPoint & p);
  fn _ZNK11QListWidget6itemAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QListWidget::insertItem(int row, const QString & label);
  fn _ZN11QListWidget10insertItemEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  int QListWidget::row(const QListWidgetItem * item);
  fn _ZNK11QListWidget3rowEPK15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QListWidget::openPersistentEditor(QListWidgetItem * item);
  fn _ZN11QListWidget20openPersistentEditorEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidget::clear();
  fn _ZN11QListWidget5clearEv(qthis: *mut c_void);
  // proto:  void QListWidget::editItem(QListWidgetItem * item);
  fn _ZN11QListWidget8editItemEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QListWidget::count();
  fn _ZNK11QListWidget5countEv(qthis: *mut c_void) -> c_int;
  // proto:  void QListWidget::setItemHidden(const QListWidgetItem * item, bool hide);
  fn _ZN11QListWidget13setItemHiddenEPK15QListWidgetItemb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  void QListWidget::~QListWidget();
  fn _ZN11QListWidgetD0Ev(qthis: *mut c_void);
  // proto:  void QListWidget::addItem(QListWidgetItem * item);
  fn _ZN11QListWidget7addItemEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidget::itemSelectionChanged();
  fn _ZN11QListWidget20itemSelectionChangedEv(qthis: *mut c_void);
  // proto:  QListWidgetItem * QListWidget::takeItem(int row);
  fn _ZN11QListWidget8takeItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  bool QListWidget::isSortingEnabled();
  fn _ZNK11QListWidget16isSortingEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  void QListWidget::addItems(const QStringList & labels);
  fn _ZN11QListWidget8addItemsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QList<QListWidgetItem *> QListWidget::selectedItems();
  fn _ZNK11QListWidget13selectedItemsEv(qthis: *mut c_void);
  // proto:  const QMetaObject * QListWidget::metaObject();
  fn _ZNK11QListWidget10metaObjectEv(qthis: *mut c_void);
  // proto:  void QListWidget::itemDoubleClicked(QListWidgetItem * item);
  fn _ZN11QListWidget17itemDoubleClickedEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidget::setItemSelected(const QListWidgetItem * item, bool select);
  fn _ZN11QListWidget15setItemSelectedEPK15QListWidgetItemb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  void QListWidget::insertItem(int row, QListWidgetItem * item);
  fn _ZN11QListWidget10insertItemEiP15QListWidgetItem(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QListWidget::setCurrentRow(int row);
  fn _ZN11QListWidget13setCurrentRowEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QListWidget::setSortingEnabled(bool enable);
  fn _ZN11QListWidget17setSortingEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QRect QListWidget::visualItemRect(const QListWidgetItem * item);
  fn _ZNK11QListWidget14visualItemRectEPK15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QListWidget::removeItemWidget(QListWidgetItem * item);
  fn _ZN11QListWidget16removeItemWidgetEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidget::itemActivated(QListWidgetItem * item);
  fn _ZN11QListWidget13itemActivatedEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidget::closePersistentEditor(QListWidgetItem * item);
  fn _ZN11QListWidget21closePersistentEditorEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidget::itemEntered(QListWidgetItem * item);
  fn _ZN11QListWidget11itemEnteredEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QListWidget::isItemHidden(const QListWidgetItem * item);
  fn _ZNK11QListWidget12isItemHiddenEPK15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QListWidget::itemChanged(QListWidgetItem * item);
  fn _ZN11QListWidget11itemChangedEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QListWidgetItem * QListWidget::itemAt(int x, int y);
  fn _ZNK11QListWidget6itemAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QListWidget::currentItemChanged(QListWidgetItem * current, QListWidgetItem * previous);
  fn _ZN11QListWidget18currentItemChangedEP15QListWidgetItemS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QListWidget::addItem(const QString & label);
  fn _ZN11QListWidget7addItemERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidget::insertItems(int row, const QStringList & labels);
  fn _ZN11QListWidget11insertItemsEiRK11QStringList(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QListWidget::currentTextChanged(const QString & currentText);
  fn _ZN11QListWidget18currentTextChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QListWidgetItem * QListWidget::currentItem();
  fn _ZNK11QListWidget11currentItemEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QListWidget::setCurrentItem(QListWidgetItem * item);
  fn _ZN11QListWidget14setCurrentItemEP15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QListWidget::setItemWidget(QListWidgetItem * item, QWidget * widget);
  fn _ZN11QListWidget13setItemWidgetEP15QListWidgetItemP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QListWidget::isItemSelected(const QListWidgetItem * item);
  fn _ZNK11QListWidget14isItemSelectedEPK15QListWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QListWidget::QListWidget(const QListWidget & );
  fn _ZN11QListWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QListWidgetItem)=1
pub struct QListWidgetItem {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QListWidget)=1
pub struct QListWidget {
  qbase: QListView,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QListWidgetItem {
  pub fn inheritFrom(qthis: *mut c_void) -> QListWidgetItem {
    return QListWidgetItem{qclsinst: qthis};
  }
}
  // proto:  void QListWidgetItem::~QListWidgetItem();
impl /*struct*/ QListWidgetItem {
  pub fn Free<RetType, T: QListWidgetItem_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QListWidgetItem_Free<RetType> {
  fn Free(self , rsthis: & QListWidgetItem) -> RetType;
}

  // proto:  void QListWidgetItem::~QListWidgetItem();
impl<'a> /*trait*/ QListWidgetItem_Free<()> for () {
  fn Free(self , rsthis: & QListWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItemD0Ev()};
     unsafe {_ZN15QListWidgetItemD0Ev(rsthis.qclsinst)};
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
    let mut ret = unsafe {_ZNK15QListWidgetItem8isHiddenEv(rsthis.qclsinst)};
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
     unsafe {_ZN15QListWidgetItem7setDataEiRK8QVariant(rsthis.qclsinst, arg0, arg1)};
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
     unsafe {_ZN15QListWidgetItem13setBackgroundERK6QBrush(rsthis.qclsinst, arg0)};
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
     unsafe {_ZN15QListWidgetItem11setSelectedEb(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK15QListWidgetItem4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret);
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
     unsafe {_ZN15QListWidgetItem16setTextAlignmentEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidgetItem::QListWidgetItem(QListWidget * view, int type);
impl /*struct*/ QListWidgetItem {
  pub fn New<T: QListWidgetItem_New>(value: T) -> QListWidgetItem {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QListWidgetItem_New {
  fn New(self) -> QListWidgetItem;
}

  // proto:  void QListWidgetItem::QListWidgetItem(QListWidget * view, int type);
impl<'a> /*trait*/ QListWidgetItem_New for (&'a QListWidget, i32) {
  fn New(self) -> QListWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItemC1EP11QListWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN15QListWidgetItemC1EP11QListWidgeti(qthis, arg0, arg1)};
    let rsthis = QListWidgetItem{qclsinst: qthis};
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
     unsafe {_ZNK15QListWidgetItem5writeER11QDataStream(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK15QListWidgetItem9whatsThisEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
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
    let mut ret = unsafe {_ZNK15QListWidgetItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QListWidgetItem::QListWidgetItem(const QIcon & icon, const QString & text, QListWidget * view, int type);
impl<'a> /*trait*/ QListWidgetItem_New for (&'a QIcon, &'a QString, &'a QListWidget, i32) {
  fn New(self) -> QListWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItemC1ERK5QIconRK7QStringP11QListWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    unsafe {_ZN15QListWidgetItemC1ERK5QIconRK7QStringP11QListWidgeti(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QListWidgetItem{qclsinst: qthis};
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
    let mut ret = unsafe {_ZNK15QListWidgetItem4iconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon::inheritFrom(ret);
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
    let mut ret = unsafe {_ZNK15QListWidgetItem9textColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret);
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
    let mut ret = unsafe {_ZNK15QListWidgetItem10foregroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret);
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
    let mut ret = unsafe {_ZNK15QListWidgetItem10backgroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret);
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
     unsafe {_ZN15QListWidgetItem12setStatusTipERK7QString(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK15QListWidgetItem4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
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
    let mut ret = unsafe {_ZNK15QListWidgetItem15backgroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret);
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
    let mut ret = unsafe {_ZNK15QListWidgetItem10isSelectedEv(rsthis.qclsinst)};
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
     unsafe {_ZN15QListWidgetItem7setFontERK5QFont(rsthis.qclsinst, arg0)};
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
     unsafe {_ZN15QListWidgetItem7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidgetItem::QListWidgetItem(const QString & text, QListWidget * view, int type);
impl<'a> /*trait*/ QListWidgetItem_New for (&'a QString, &'a QListWidget, i32) {
  fn New(self) -> QListWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItemC1ERK7QStringP11QListWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN15QListWidgetItemC1ERK7QStringP11QListWidgeti(qthis, arg0, arg1, arg2)};
    let rsthis = QListWidgetItem{qclsinst: qthis};
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
    let mut ret = unsafe {_ZNK15QListWidgetItem4dataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret);
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
    let mut ret = unsafe {_ZNK15QListWidgetItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
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
     unsafe {_ZN15QListWidgetItem12setWhatsThisERK7QString(rsthis.qclsinst, arg0)};
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
     unsafe {_ZN15QListWidgetItem4readER11QDataStream(rsthis.qclsinst, arg0)};
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
     unsafe {_ZN15QListWidgetItem12setTextColorERK6QColor(rsthis.qclsinst, arg0)};
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
     unsafe {_ZN15QListWidgetItem11setSizeHintERK5QSize(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK15QListWidgetItem10listWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QListWidget::inheritFrom(ret);
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
     unsafe {_ZN15QListWidgetItem7setIconERK5QIcon(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK15QListWidgetItem5cloneEv(rsthis.qclsinst)};
    let mut ret1 = QListWidgetItem::inheritFrom(ret);
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
     unsafe {_ZN15QListWidgetItem18setBackgroundColorERK6QColor(rsthis.qclsinst, arg0)};
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
     unsafe {_ZN15QListWidgetItem13setForegroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidgetItem::QListWidgetItem(const QListWidgetItem & other);
impl<'a> /*trait*/ QListWidgetItem_New for (&'a QListWidgetItem) {
  fn New(self) -> QListWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QListWidgetItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QListWidgetItemC1ERKS_(qthis, arg0)};
    let rsthis = QListWidgetItem{qclsinst: qthis};
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
     unsafe {_ZN15QListWidgetItem9setHiddenEb(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK15QListWidgetItem7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
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
    let mut ret = unsafe {_ZNK15QListWidgetItem13textAlignmentEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {_ZNK15QListWidgetItem9statusTipEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
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
     unsafe {_ZN15QListWidgetItem10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QListWidget {
  pub fn inheritFrom(qthis: *mut c_void) -> QListWidget {
    return QListWidget{qbase: QListView::inheritFrom(qthis), qclsinst: qthis};
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
     unsafe {_ZN11QListWidget9dropEventEP10QDropEvent(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidget::itemClicked(QListWidgetItem * item);
impl /*struct*/ QListWidget {
  pub fn itemClicked<RetType, T: QListWidget_itemClicked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemClicked(self);
    // return 1;
  }
}

pub trait QListWidget_itemClicked<RetType> {
  fn itemClicked(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::itemClicked(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemClicked<()> for (&'a QListWidgetItem) {
  fn itemClicked(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget11itemClickedEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget11itemClickedEP15QListWidgetItem(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK11QListWidget10itemWidgetEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QListWidget::itemPressed(QListWidgetItem * item);
impl /*struct*/ QListWidget {
  pub fn itemPressed<RetType, T: QListWidget_itemPressed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemPressed(self);
    // return 1;
  }
}

pub trait QListWidget_itemPressed<RetType> {
  fn itemPressed(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::itemPressed(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemPressed<()> for (&'a QListWidgetItem) {
  fn itemPressed(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget11itemPressedEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget11itemPressedEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidget::QListWidget(QWidget * parent);
impl /*struct*/ QListWidget {
  pub fn New<T: QListWidget_New>(value: T) -> QListWidget {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QListWidget_New {
  fn New(self) -> QListWidget;
}

  // proto:  void QListWidget::QListWidget(QWidget * parent);
impl<'a> /*trait*/ QListWidget_New for (&'a QWidget) {
  fn New(self) -> QListWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidgetC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidgetC1EP7QWidget(qthis, arg0)};
    let rsthis = QListWidget{/**/qbase: QListView::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QListWidget::currentRowChanged(int currentRow);
impl /*struct*/ QListWidget {
  pub fn currentRowChanged<RetType, T: QListWidget_currentRowChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentRowChanged(self);
    // return 1;
  }
}

pub trait QListWidget_currentRowChanged<RetType> {
  fn currentRowChanged(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::currentRowChanged(int currentRow);
impl<'a> /*trait*/ QListWidget_currentRowChanged<()> for (i32) {
  fn currentRowChanged(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget17currentRowChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QListWidget17currentRowChangedEi(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK11QListWidget10currentRowEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {_ZNK11QListWidget4itemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QListWidgetItem::inheritFrom(ret);
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
    let mut ret = unsafe {_ZNK11QListWidget6itemAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QListWidgetItem::inheritFrom(ret);
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
     unsafe {_ZN11QListWidget10insertItemEiRK7QString(rsthis.qclsinst, arg0, arg1)};
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
    let mut ret = unsafe {_ZNK11QListWidget3rowEPK15QListWidgetItem(rsthis.qclsinst, arg0)};
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
     unsafe {_ZN11QListWidget20openPersistentEditorEP15QListWidgetItem(rsthis.qclsinst, arg0)};
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
     unsafe {_ZN11QListWidget5clearEv(rsthis.qclsinst)};
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
     unsafe {_ZN11QListWidget8editItemEP15QListWidgetItem(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK11QListWidget5countEv(rsthis.qclsinst)};
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
     unsafe {_ZN11QListWidget13setItemHiddenEPK15QListWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QListWidget::~QListWidget();
impl /*struct*/ QListWidget {
  pub fn Free<RetType, T: QListWidget_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QListWidget_Free<RetType> {
  fn Free(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::~QListWidget();
impl<'a> /*trait*/ QListWidget_Free<()> for () {
  fn Free(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidgetD0Ev()};
     unsafe {_ZN11QListWidgetD0Ev(rsthis.qclsinst)};
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
     unsafe {_ZN11QListWidget7addItemEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidget::itemSelectionChanged();
impl /*struct*/ QListWidget {
  pub fn itemSelectionChanged<RetType, T: QListWidget_itemSelectionChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemSelectionChanged(self);
    // return 1;
  }
}

pub trait QListWidget_itemSelectionChanged<RetType> {
  fn itemSelectionChanged(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::itemSelectionChanged();
impl<'a> /*trait*/ QListWidget_itemSelectionChanged<()> for () {
  fn itemSelectionChanged(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget20itemSelectionChangedEv()};
     unsafe {_ZN11QListWidget20itemSelectionChangedEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {_ZN11QListWidget8takeItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QListWidgetItem::inheritFrom(ret);
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
    let mut ret = unsafe {_ZNK11QListWidget16isSortingEnabledEv(rsthis.qclsinst)};
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
     unsafe {_ZN11QListWidget8addItemsERK11QStringList(rsthis.qclsinst, arg0)};
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
     unsafe {_ZNK11QListWidget13selectedItemsEv(rsthis.qclsinst)};
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
impl<'a> /*trait*/ QListWidget_metaObject<()> for () {
  fn metaObject(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QListWidget10metaObjectEv()};
     unsafe {_ZNK11QListWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QListWidget::itemDoubleClicked(QListWidgetItem * item);
impl /*struct*/ QListWidget {
  pub fn itemDoubleClicked<RetType, T: QListWidget_itemDoubleClicked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemDoubleClicked(self);
    // return 1;
  }
}

pub trait QListWidget_itemDoubleClicked<RetType> {
  fn itemDoubleClicked(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::itemDoubleClicked(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemDoubleClicked<()> for (&'a QListWidgetItem) {
  fn itemDoubleClicked(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget17itemDoubleClickedEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget17itemDoubleClickedEP15QListWidgetItem(rsthis.qclsinst, arg0)};
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
     unsafe {_ZN11QListWidget15setItemSelectedEPK15QListWidgetItemb(rsthis.qclsinst, arg0, arg1)};
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
     unsafe {_ZN11QListWidget10insertItemEiP15QListWidgetItem(rsthis.qclsinst, arg0, arg1)};
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
     unsafe {_ZN11QListWidget13setCurrentRowEi(rsthis.qclsinst, arg0)};
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
     unsafe {_ZN11QListWidget17setSortingEnabledEb(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK11QListWidget14visualItemRectEPK15QListWidgetItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret);
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
     unsafe {_ZN11QListWidget16removeItemWidgetEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidget::itemActivated(QListWidgetItem * item);
impl /*struct*/ QListWidget {
  pub fn itemActivated<RetType, T: QListWidget_itemActivated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemActivated(self);
    // return 1;
  }
}

pub trait QListWidget_itemActivated<RetType> {
  fn itemActivated(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::itemActivated(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemActivated<()> for (&'a QListWidgetItem) {
  fn itemActivated(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget13itemActivatedEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget13itemActivatedEP15QListWidgetItem(rsthis.qclsinst, arg0)};
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
     unsafe {_ZN11QListWidget21closePersistentEditorEP15QListWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QListWidget::itemEntered(QListWidgetItem * item);
impl /*struct*/ QListWidget {
  pub fn itemEntered<RetType, T: QListWidget_itemEntered<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemEntered(self);
    // return 1;
  }
}

pub trait QListWidget_itemEntered<RetType> {
  fn itemEntered(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::itemEntered(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemEntered<()> for (&'a QListWidgetItem) {
  fn itemEntered(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget11itemEnteredEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget11itemEnteredEP15QListWidgetItem(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK11QListWidget12isItemHiddenEPK15QListWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QListWidget::itemChanged(QListWidgetItem * item);
impl /*struct*/ QListWidget {
  pub fn itemChanged<RetType, T: QListWidget_itemChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemChanged(self);
    // return 1;
  }
}

pub trait QListWidget_itemChanged<RetType> {
  fn itemChanged(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::itemChanged(QListWidgetItem * item);
impl<'a> /*trait*/ QListWidget_itemChanged<()> for (&'a QListWidgetItem) {
  fn itemChanged(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget11itemChangedEP15QListWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget11itemChangedEP15QListWidgetItem(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK11QListWidget6itemAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QListWidgetItem::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QListWidget::currentItemChanged(QListWidgetItem * current, QListWidgetItem * previous);
impl /*struct*/ QListWidget {
  pub fn currentItemChanged<RetType, T: QListWidget_currentItemChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentItemChanged(self);
    // return 1;
  }
}

pub trait QListWidget_currentItemChanged<RetType> {
  fn currentItemChanged(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::currentItemChanged(QListWidgetItem * current, QListWidgetItem * previous);
impl<'a> /*trait*/ QListWidget_currentItemChanged<()> for (&'a QListWidgetItem, &'a QListWidgetItem) {
  fn currentItemChanged(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget18currentItemChangedEP15QListWidgetItemS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget18currentItemChangedEP15QListWidgetItemS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QListWidget::addItem(const QString & label);
impl<'a> /*trait*/ QListWidget_addItem<()> for (&'a QString) {
  fn addItem(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget7addItemERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget7addItemERK7QString(rsthis.qclsinst, arg0)};
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
     unsafe {_ZN11QListWidget11insertItemsEiRK11QStringList(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QListWidget::currentTextChanged(const QString & currentText);
impl /*struct*/ QListWidget {
  pub fn currentTextChanged<RetType, T: QListWidget_currentTextChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentTextChanged(self);
    // return 1;
  }
}

pub trait QListWidget_currentTextChanged<RetType> {
  fn currentTextChanged(self , rsthis: & QListWidget) -> RetType;
}

  // proto:  void QListWidget::currentTextChanged(const QString & currentText);
impl<'a> /*trait*/ QListWidget_currentTextChanged<()> for (&'a QString) {
  fn currentTextChanged(self , rsthis: & QListWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidget18currentTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QListWidget18currentTextChangedERK7QString(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK11QListWidget11currentItemEv(rsthis.qclsinst)};
    let mut ret1 = QListWidgetItem::inheritFrom(ret);
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
     unsafe {_ZN11QListWidget14setCurrentItemEP15QListWidgetItem(rsthis.qclsinst, arg0)};
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
     unsafe {_ZN11QListWidget13setItemWidgetEP15QListWidgetItemP7QWidget(rsthis.qclsinst, arg0, arg1)};
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
    let mut ret = unsafe {_ZNK11QListWidget14isItemSelectedEPK15QListWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QListWidget::QListWidget(const QListWidget & );
impl<'a> /*trait*/ QListWidget_New for (&'a QListWidget) {
  fn New(self) -> QListWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QListWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QListWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QListWidget{/**/qbase: QListView::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

