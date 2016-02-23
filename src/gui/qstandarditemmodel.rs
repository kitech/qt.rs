// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtGui/qstandarditemmodel.h
// dst-file: /src/gui/qstandarditemmodel.rs
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
use super::super::core::qabstractitemmodel::*; // 771
use std::ops::Deref;
use super::super::core::qobject::*; // 771
use super::super::core::qstring::*; // 771
// use super::qstandarditemmodel::QStandardItem; // 773
use super::super::core::qvariant::*; // 771
// use super::qlist::*; // 775
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qstringlist::*; // 771
use super::super::core::qmimedata::*; // 771
// use super::qmap::*; // 775
// use super::qstandarditemmodel::QStandardItemModel; // 773
use super::qbrush::*; // 773
use super::qicon::*; // 773
use super::super::core::qdatastream::*; // 771
use super::qfont::*; // 773
use super::super::core::qsize::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStandardItemModel_Class_Size() -> c_int;
  // proto:  void QStandardItemModel::QStandardItemModel(int rows, int columns, QObject * parent);
  fn C_ZN18QStandardItemModelC2EiiP7QObject(arg0: c_int, arg1: c_int, arg2: *mut c_void) -> u64;
  // proto:  void QStandardItemModel::clear();
  fn C_ZN18QStandardItemModel5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  QStandardItem * QStandardItemModel::item(int row, int column);
  fn C_ZNK18QStandardItemModel4itemEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  bool QStandardItemModel::insertRow(int row, const QModelIndex & parent);
  fn C_ZN18QStandardItemModel9insertRowEiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  void QStandardItemModel::setItem(int row, QStandardItem * item);
  fn C_ZN18QStandardItemModel7setItemEiP13QStandardItem(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  QModelIndex QStandardItemModel::index(int row, int column, const QModelIndex & parent);
  fn C_ZNK18QStandardItemModel5indexEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  bool QStandardItemModel::setData(const QModelIndex & index, const QVariant & value, int role);
  fn C_ZN18QStandardItemModel7setDataERK11QModelIndexRK8QVarianti(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> c_char;
  // proto:  int QStandardItemModel::columnCount(const QModelIndex & parent);
  fn C_ZNK18QStandardItemModel11columnCountERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  QStandardItem * QStandardItemModel::takeItem(int row, int column);
  fn C_ZN18QStandardItemModel8takeItemEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QStandardItemModel::setRowCount(int rows);
  fn C_ZN18QStandardItemModel11setRowCountEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QStandardItem * QStandardItemModel::itemFromIndex(const QModelIndex & index);
  fn C_ZNK18QStandardItemModel13itemFromIndexERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QStandardItemModel::insertColumn(int column, const QModelIndex & parent);
  fn C_ZN18QStandardItemModel12insertColumnEiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  void QStandardItemModel::setVerticalHeaderItem(int row, QStandardItem * item);
  fn C_ZN18QStandardItemModel21setVerticalHeaderItemEiP13QStandardItem(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QStandardItemModel::QStandardItemModel(QObject * parent);
  fn C_ZN18QStandardItemModelC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  QList<QStandardItem *> QStandardItemModel::takeColumn(int column);
  fn C_ZN18QStandardItemModel10takeColumnEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QStandardItem * QStandardItemModel::takeVerticalHeaderItem(int row);
  fn C_ZN18QStandardItemModel22takeVerticalHeaderItemEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  bool QStandardItemModel::insertColumns(int column, int count, const QModelIndex & parent);
  fn C_ZN18QStandardItemModel13insertColumnsEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  const QMetaObject * QStandardItemModel::metaObject();
  fn C_ZNK18QStandardItemModel10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QStandardItemModel::insertRows(int row, int count, const QModelIndex & parent);
  fn C_ZN18QStandardItemModel10insertRowsEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  void QStandardItemModel::insertRow(int row, QStandardItem * item);
  fn C_ZN18QStandardItemModel9insertRowEiP13QStandardItem(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  QStandardItem * QStandardItemModel::invisibleRootItem();
  fn C_ZNK18QStandardItemModel17invisibleRootItemEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStandardItemModel::setItemPrototype(const QStandardItem * item);
  fn C_ZN18QStandardItemModel16setItemPrototypeEPK13QStandardItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStandardItemModel::setHorizontalHeaderLabels(const QStringList & labels);
  fn C_ZN18QStandardItemModel25setHorizontalHeaderLabelsERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QModelIndex QStandardItemModel::parent(const QModelIndex & child);
  fn C_ZNK18QStandardItemModel6parentERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QStandardItemModel::removeColumns(int column, int count, const QModelIndex & parent);
  fn C_ZN18QStandardItemModel13removeColumnsEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  QModelIndex QStandardItemModel::sibling(int row, int column, const QModelIndex & idx);
  fn C_ZNK18QStandardItemModel7siblingEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  int QStandardItemModel::sortRole();
  fn C_ZNK18QStandardItemModel8sortRoleEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QStandardItem * QStandardItemModel::takeHorizontalHeaderItem(int column);
  fn C_ZN18QStandardItemModel24takeHorizontalHeaderItemEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QModelIndex QStandardItemModel::indexFromItem(const QStandardItem * item);
  fn C_ZNK18QStandardItemModel13indexFromItemEPK13QStandardItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QStandardItem * QStandardItemModel::itemPrototype();
  fn C_ZNK18QStandardItemModel13itemPrototypeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStandardItemModel::setHorizontalHeaderItem(int column, QStandardItem * item);
  fn C_ZN18QStandardItemModel23setHorizontalHeaderItemEiP13QStandardItem(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  QStandardItem * QStandardItemModel::horizontalHeaderItem(int column);
  fn C_ZNK18QStandardItemModel20horizontalHeaderItemEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QStandardItemModel::appendRow(QStandardItem * item);
  fn C_ZN18QStandardItemModel9appendRowEP13QStandardItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QMap<int, QVariant> QStandardItemModel::itemData(const QModelIndex & index);
  fn C_ZNK18QStandardItemModel8itemDataERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QStandardItemModel::setSortRole(int role);
  fn C_ZN18QStandardItemModel11setSortRoleEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QStandardItemModel::hasChildren(const QModelIndex & parent);
  fn C_ZNK18QStandardItemModel11hasChildrenERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QStandardItemModel::~QStandardItemModel();
  fn C_ZN18QStandardItemModelD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QVariant QStandardItemModel::data(const QModelIndex & index, int role);
  fn C_ZNK18QStandardItemModel4dataERK11QModelIndexi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  QList<QStandardItem *> QStandardItemModel::takeRow(int row);
  fn C_ZN18QStandardItemModel7takeRowEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QStandardItemModel::setColumnCount(int columns);
  fn C_ZN18QStandardItemModel14setColumnCountEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QStandardItem * QStandardItemModel::verticalHeaderItem(int row);
  fn C_ZNK18QStandardItemModel18verticalHeaderItemEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  bool QStandardItemModel::removeRows(int row, int count, const QModelIndex & parent);
  fn C_ZN18QStandardItemModel10removeRowsEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  void QStandardItemModel::setVerticalHeaderLabels(const QStringList & labels);
  fn C_ZN18QStandardItemModel23setVerticalHeaderLabelsERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStandardItemModel::setItem(int row, int column, QStandardItem * item);
  fn C_ZN18QStandardItemModel7setItemEiiP13QStandardItem(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void);
  // proto:  QStringList QStandardItemModel::mimeTypes();
  fn C_ZNK18QStandardItemModel9mimeTypesEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QStandardItemModel::rowCount(const QModelIndex & parent);
  fn C_ZNK18QStandardItemModel8rowCountERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  fn QStandardItem_Class_Size() -> c_int;
  // proto:  void QStandardItem::setChild(int row, QStandardItem * item);
  fn C_ZN13QStandardItem8setChildEiPS_(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  QStandardItemModel * QStandardItem::model();
  fn C_ZNK13QStandardItem5modelEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStandardItem::insertColumns(int column, int count);
  fn C_ZN13QStandardItem13insertColumnsEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QStandardItem::setSelectable(bool selectable);
  fn C_ZN13QStandardItem13setSelectableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  int QStandardItem::column();
  fn C_ZNK13QStandardItem6columnEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QString QStandardItem::whatsThis();
  fn C_ZNK13QStandardItem9whatsThisEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<QStandardItem *> QStandardItem::takeColumn(int column);
  fn C_ZN13QStandardItem10takeColumnEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QStandardItem::setForeground(const QBrush & brush);
  fn C_ZN13QStandardItem13setForegroundERK6QBrush(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QStandardItem::isEditable();
  fn C_ZNK13QStandardItem10isEditableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QIcon QStandardItem::icon();
  fn C_ZNK13QStandardItem4iconEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStandardItem::setWhatsThis(const QString & whatsThis);
  fn C_ZN13QStandardItem12setWhatsThisERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QStandardItem * QStandardItem::takeChild(int row, int column);
  fn C_ZN13QStandardItem9takeChildEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  int QStandardItem::type();
  fn C_ZNK13QStandardItem4typeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QList<QStandardItem *> QStandardItem::takeRow(int row);
  fn C_ZN13QStandardItem7takeRowEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  int QStandardItem::row();
  fn C_ZNK13QStandardItem3rowEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QStandardItem::isCheckable();
  fn C_ZNK13QStandardItem11isCheckableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QStandardItem::text();
  fn C_ZNK13QStandardItem4textEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStandardItem::insertRows(int row, int count);
  fn C_ZN13QStandardItem10insertRowsEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  bool QStandardItem::isDropEnabled();
  fn C_ZNK13QStandardItem13isDropEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QStandardItem::hasChildren();
  fn C_ZNK13QStandardItem11hasChildrenEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QStandardItem::statusTip();
  fn C_ZNK13QStandardItem9statusTipEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStandardItem::setStatusTip(const QString & statusTip);
  fn C_ZN13QStandardItem12setStatusTipERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStandardItem::appendRow(QStandardItem * item);
  fn C_ZN13QStandardItem9appendRowEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStandardItem::setChild(int row, int column, QStandardItem * item);
  fn C_ZN13QStandardItem8setChildEiiPS_(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void);
  // proto:  QModelIndex QStandardItem::index();
  fn C_ZNK13QStandardItem5indexEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStandardItem::setIcon(const QIcon & icon);
  fn C_ZN13QStandardItem7setIconERK5QIcon(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStandardItem::setToolTip(const QString & toolTip);
  fn C_ZN13QStandardItem10setToolTipERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStandardItem::setData(const QVariant & value, int role);
  fn C_ZN13QStandardItem7setDataERK8QVarianti(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  QBrush QStandardItem::background();
  fn C_ZNK13QStandardItem10backgroundEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QVariant QStandardItem::data(int role);
  fn C_ZNK13QStandardItem4dataEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QStandardItem * QStandardItem::child(int row, int column);
  fn C_ZNK13QStandardItem5childEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  bool QStandardItem::isSelectable();
  fn C_ZNK13QStandardItem12isSelectableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QStandardItem::toolTip();
  fn C_ZNK13QStandardItem7toolTipEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStandardItem::setRowCount(int rows);
  fn C_ZN13QStandardItem11setRowCountEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QStandardItem::QStandardItem(const QString & text);
  fn C_ZN13QStandardItemC2ERK7QString(arg0: *mut c_void) -> u64;
  // proto:  void QStandardItem::write(QDataStream & out);
  fn C_ZNK13QStandardItem5writeER11QDataStream(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QStandardItem::isDragEnabled();
  fn C_ZNK13QStandardItem13isDragEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QStandardItem::setAccessibleText(const QString & accessibleText);
  fn C_ZN13QStandardItem17setAccessibleTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QStandardItem::rowCount();
  fn C_ZNK13QStandardItem8rowCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QStandardItem::removeColumn(int column);
  fn C_ZN13QStandardItem12removeColumnEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QStandardItem::removeRow(int row);
  fn C_ZN13QStandardItem9removeRowEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QStandardItem::columnCount();
  fn C_ZNK13QStandardItem11columnCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QStandardItem::isTristate();
  fn C_ZNK13QStandardItem10isTristateEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QStandardItem * QStandardItem::parent();
  fn C_ZNK13QStandardItem6parentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStandardItem::insertRow(int row, QStandardItem * item);
  fn C_ZN13QStandardItem9insertRowEiPS_(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QStandardItem::QStandardItem(const QIcon & icon, const QString & text);
  fn C_ZN13QStandardItemC2ERK5QIconRK7QString(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  void QStandardItem::setFont(const QFont & font);
  fn C_ZN13QStandardItem7setFontERK5QFont(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStandardItem::removeColumns(int column, int count);
  fn C_ZN13QStandardItem13removeColumnsEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QStandardItem::~QStandardItem();
  fn C_ZN13QStandardItemD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QStandardItem::QStandardItem();
  fn C_ZN13QStandardItemC2Ev() -> u64;
  // proto:  QFont QStandardItem::font();
  fn C_ZNK13QStandardItem4fontEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStandardItem::setEditable(bool editable);
  fn C_ZN13QStandardItem11setEditableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QStandardItem::setText(const QString & text);
  fn C_ZN13QStandardItem7setTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStandardItem::QStandardItem(int rows, int columns);
  fn C_ZN13QStandardItemC2Eii(arg0: c_int, arg1: c_int) -> u64;
  // proto:  bool QStandardItem::isEnabled();
  fn C_ZNK13QStandardItem9isEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QStandardItem::setDropEnabled(bool dropEnabled);
  fn C_ZN13QStandardItem14setDropEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QStandardItem::setColumnCount(int columns);
  fn C_ZN13QStandardItem14setColumnCountEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QString QStandardItem::accessibleText();
  fn C_ZNK13QStandardItem14accessibleTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStandardItem::read(QDataStream & in);
  fn C_ZN13QStandardItem4readER11QDataStream(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStandardItem::setCheckable(bool checkable);
  fn C_ZN13QStandardItem12setCheckableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QStandardItem::setDragEnabled(bool dragEnabled);
  fn C_ZN13QStandardItem14setDragEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QBrush QStandardItem::foreground();
  fn C_ZNK13QStandardItem10foregroundEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStandardItem * QStandardItem::clone();
  fn C_ZNK13QStandardItem5cloneEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStandardItem::removeRows(int row, int count);
  fn C_ZN13QStandardItem10removeRowsEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  QSize QStandardItem::sizeHint();
  fn C_ZNK13QStandardItem8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStandardItem::setEnabled(bool enabled);
  fn C_ZN13QStandardItem10setEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QStandardItem::setBackground(const QBrush & brush);
  fn C_ZN13QStandardItem13setBackgroundERK6QBrush(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStandardItem::setAccessibleDescription(const QString & accessibleDescription);
  fn C_ZN13QStandardItem24setAccessibleDescriptionERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStandardItem::setSizeHint(const QSize & sizeHint);
  fn C_ZN13QStandardItem11setSizeHintERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QStandardItem::accessibleDescription();
  fn C_ZNK13QStandardItem21accessibleDescriptionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QStandardItem::setTristate(bool tristate);
  fn C_ZN13QStandardItem11setTristateEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  fn QStandardItemModel_SlotProxy_connect__ZN18QStandardItemModel11itemChangedEP13QStandardItem(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QStandardItemModel)=1
#[derive(Default)]
pub struct QStandardItemModel {
  qbase: QAbstractItemModel,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _itemChanged: QStandardItemModel_itemChanged_signal,
}

// class sizeof(QStandardItem)=1
#[derive(Default)]
pub struct QStandardItem {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QStandardItemModel {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStandardItemModel {
    return QStandardItemModel{qbase: QAbstractItemModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QStandardItemModel {
  type Target = QAbstractItemModel;

  fn deref(&self) -> &QAbstractItemModel {
    return & self.qbase;
  }
}
impl AsRef<QAbstractItemModel> for QStandardItemModel {
  fn as_ref(& self) -> & QAbstractItemModel {
    return & self.qbase;
  }
}
  // proto:  void QStandardItemModel::QStandardItemModel(int rows, int columns, QObject * parent);
impl /*struct*/ QStandardItemModel {
  pub fn new<T: QStandardItemModel_new>(value: T) -> QStandardItemModel {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStandardItemModel_new {
  fn new(self) -> QStandardItemModel;
}

  // proto:  void QStandardItemModel::QStandardItemModel(int rows, int columns, QObject * parent);
impl<'a> /*trait*/ QStandardItemModel_new for (i32, i32, Option<&'a QObject>) {
  fn new(self) -> QStandardItemModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModelC2EiiP7QObject()};
    let ctysz: c_int = unsafe{QStandardItemModel_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = (if self.2.is_none() {0} else {self.2.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN18QStandardItemModelC2EiiP7QObject(arg0, arg1, arg2)};
    let rsthis = QStandardItemModel{qbase: QAbstractItemModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStandardItemModel::clear();
impl /*struct*/ QStandardItemModel {
  pub fn clear<RetType, T: QStandardItemModel_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QStandardItemModel_clear<RetType> {
  fn clear(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  void QStandardItemModel::clear();
impl<'a> /*trait*/ QStandardItemModel_clear<()> for () {
  fn clear(self , rsthis: & QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel5clearEv()};
     unsafe {C_ZN18QStandardItemModel5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStandardItem * QStandardItemModel::item(int row, int column);
impl /*struct*/ QStandardItemModel {
  pub fn item<RetType, T: QStandardItemModel_item<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.item(self);
    // return 1;
  }
}

pub trait QStandardItemModel_item<RetType> {
  fn item(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  QStandardItem * QStandardItemModel::item(int row, int column);
impl<'a> /*trait*/ QStandardItemModel_item<QStandardItem> for (i32, Option<i32>) {
  fn item(self , rsthis: & QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel4itemEii()};
    let arg0 = self.0  as c_int;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap()})  as c_int;
    let mut ret = unsafe {C_ZNK18QStandardItemModel4itemEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QStandardItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QStandardItemModel::insertRow(int row, const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn insertRow<RetType, T: QStandardItemModel_insertRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertRow(self);
    // return 1;
  }
}

pub trait QStandardItemModel_insertRow<RetType> {
  fn insertRow(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  bool QStandardItemModel::insertRow(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_insertRow<i8> for (i32, Option<&'a QModelIndex>) {
  fn insertRow(self , rsthis: & QStandardItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel9insertRowEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = (if self.1.is_none() {QModelIndex::new(()).qclsinst} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZN18QStandardItemModel9insertRowEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QStandardItemModel::setItem(int row, QStandardItem * item);
impl /*struct*/ QStandardItemModel {
  pub fn setItem<RetType, T: QStandardItemModel_setItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setItem<RetType> {
  fn setItem(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  void QStandardItemModel::setItem(int row, QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_setItem<()> for (i32, &'a QStandardItem) {
  fn setItem(self , rsthis: & QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel7setItemEiP13QStandardItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN18QStandardItemModel7setItemEiP13QStandardItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QModelIndex QStandardItemModel::index(int row, int column, const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn index<RetType, T: QStandardItemModel_index<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.index(self);
    // return 1;
  }
}

pub trait QStandardItemModel_index<RetType> {
  fn index(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  QModelIndex QStandardItemModel::index(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_index<QModelIndex> for (i32, i32, Option<&'a QModelIndex>) {
  fn index(self , rsthis: & QStandardItemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel5indexEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = (if self.2.is_none() {QModelIndex::new(()).qclsinst} else {self.2.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZNK18QStandardItemModel5indexEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QStandardItemModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl /*struct*/ QStandardItemModel {
  pub fn setData<RetType, T: QStandardItemModel_setData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setData<RetType> {
  fn setData(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  bool QStandardItemModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl<'a> /*trait*/ QStandardItemModel_setData<i8> for (&'a QModelIndex, &'a QVariant, Option<i32>) {
  fn setData(self , rsthis: & QStandardItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel7setDataERK11QModelIndexRK8QVarianti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = (if self.2.is_none() {0 as i32} else {self.2.unwrap()})  as c_int;
    let mut ret = unsafe {C_ZN18QStandardItemModel7setDataERK11QModelIndexRK8QVarianti(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QStandardItemModel::columnCount(const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn columnCount<RetType, T: QStandardItemModel_columnCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QStandardItemModel_columnCount<RetType> {
  fn columnCount(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  int QStandardItemModel::columnCount(const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_columnCount<i32> for (Option<&'a QModelIndex>) {
  fn columnCount(self , rsthis: & QStandardItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel11columnCountERK11QModelIndex()};
    let arg0 = (if self.is_none() {QModelIndex::new(()).qclsinst} else {self.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZNK18QStandardItemModel11columnCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QStandardItem * QStandardItemModel::takeItem(int row, int column);
impl /*struct*/ QStandardItemModel {
  pub fn takeItem<RetType, T: QStandardItemModel_takeItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_takeItem<RetType> {
  fn takeItem(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  QStandardItem * QStandardItemModel::takeItem(int row, int column);
impl<'a> /*trait*/ QStandardItemModel_takeItem<QStandardItem> for (i32, Option<i32>) {
  fn takeItem(self , rsthis: & QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel8takeItemEii()};
    let arg0 = self.0  as c_int;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap()})  as c_int;
    let mut ret = unsafe {C_ZN18QStandardItemModel8takeItemEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QStandardItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStandardItemModel::setRowCount(int rows);
impl /*struct*/ QStandardItemModel {
  pub fn setRowCount<RetType, T: QStandardItemModel_setRowCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRowCount(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setRowCount<RetType> {
  fn setRowCount(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  void QStandardItemModel::setRowCount(int rows);
impl<'a> /*trait*/ QStandardItemModel_setRowCount<()> for (i32) {
  fn setRowCount(self , rsthis: & QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel11setRowCountEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN18QStandardItemModel11setRowCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStandardItem * QStandardItemModel::itemFromIndex(const QModelIndex & index);
impl /*struct*/ QStandardItemModel {
  pub fn itemFromIndex<RetType, T: QStandardItemModel_itemFromIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemFromIndex(self);
    // return 1;
  }
}

pub trait QStandardItemModel_itemFromIndex<RetType> {
  fn itemFromIndex(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  QStandardItem * QStandardItemModel::itemFromIndex(const QModelIndex & index);
impl<'a> /*trait*/ QStandardItemModel_itemFromIndex<QStandardItem> for (&'a QModelIndex) {
  fn itemFromIndex(self , rsthis: & QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel13itemFromIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK18QStandardItemModel13itemFromIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QStandardItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QStandardItemModel::insertColumn(int column, const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn insertColumn<RetType, T: QStandardItemModel_insertColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertColumn(self);
    // return 1;
  }
}

pub trait QStandardItemModel_insertColumn<RetType> {
  fn insertColumn(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  bool QStandardItemModel::insertColumn(int column, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_insertColumn<i8> for (i32, Option<&'a QModelIndex>) {
  fn insertColumn(self , rsthis: & QStandardItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel12insertColumnEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = (if self.1.is_none() {QModelIndex::new(()).qclsinst} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZN18QStandardItemModel12insertColumnEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QStandardItemModel::setVerticalHeaderItem(int row, QStandardItem * item);
impl /*struct*/ QStandardItemModel {
  pub fn setVerticalHeaderItem<RetType, T: QStandardItemModel_setVerticalHeaderItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVerticalHeaderItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setVerticalHeaderItem<RetType> {
  fn setVerticalHeaderItem(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  void QStandardItemModel::setVerticalHeaderItem(int row, QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_setVerticalHeaderItem<()> for (i32, &'a QStandardItem) {
  fn setVerticalHeaderItem(self , rsthis: & QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel21setVerticalHeaderItemEiP13QStandardItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN18QStandardItemModel21setVerticalHeaderItemEiP13QStandardItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QStandardItemModel::QStandardItemModel(QObject * parent);
impl<'a> /*trait*/ QStandardItemModel_new for (Option<&'a QObject>) {
  fn new(self) -> QStandardItemModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModelC2EP7QObject()};
    let ctysz: c_int = unsafe{QStandardItemModel_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN18QStandardItemModelC2EP7QObject(arg0)};
    let rsthis = QStandardItemModel{qbase: QAbstractItemModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QList<QStandardItem *> QStandardItemModel::takeColumn(int column);
impl /*struct*/ QStandardItemModel {
  pub fn takeColumn<RetType, T: QStandardItemModel_takeColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeColumn(self);
    // return 1;
  }
}

pub trait QStandardItemModel_takeColumn<RetType> {
  fn takeColumn(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  QList<QStandardItem *> QStandardItemModel::takeColumn(int column);
impl<'a> /*trait*/ QStandardItemModel_takeColumn<u64> for (i32) {
  fn takeColumn(self , rsthis: & QStandardItemModel) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel10takeColumnEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN18QStandardItemModel10takeColumnEi(rsthis.qclsinst, arg0)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  QStandardItem * QStandardItemModel::takeVerticalHeaderItem(int row);
impl /*struct*/ QStandardItemModel {
  pub fn takeVerticalHeaderItem<RetType, T: QStandardItemModel_takeVerticalHeaderItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeVerticalHeaderItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_takeVerticalHeaderItem<RetType> {
  fn takeVerticalHeaderItem(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  QStandardItem * QStandardItemModel::takeVerticalHeaderItem(int row);
impl<'a> /*trait*/ QStandardItemModel_takeVerticalHeaderItem<QStandardItem> for (i32) {
  fn takeVerticalHeaderItem(self , rsthis: & QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel22takeVerticalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN18QStandardItemModel22takeVerticalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QStandardItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QStandardItemModel::insertColumns(int column, int count, const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn insertColumns<RetType, T: QStandardItemModel_insertColumns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertColumns(self);
    // return 1;
  }
}

pub trait QStandardItemModel_insertColumns<RetType> {
  fn insertColumns(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  bool QStandardItemModel::insertColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_insertColumns<i8> for (i32, i32, Option<&'a QModelIndex>) {
  fn insertColumns(self , rsthis: & QStandardItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel13insertColumnsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = (if self.2.is_none() {QModelIndex::new(()).qclsinst} else {self.2.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZN18QStandardItemModel13insertColumnsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  const QMetaObject * QStandardItemModel::metaObject();
impl /*struct*/ QStandardItemModel {
  pub fn metaObject<RetType, T: QStandardItemModel_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QStandardItemModel_metaObject<RetType> {
  fn metaObject(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  const QMetaObject * QStandardItemModel::metaObject();
impl<'a> /*trait*/ QStandardItemModel_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QStandardItemModel) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel10metaObjectEv()};
    let mut ret = unsafe {C_ZNK18QStandardItemModel10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QStandardItemModel::insertRows(int row, int count, const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn insertRows<RetType, T: QStandardItemModel_insertRows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertRows(self);
    // return 1;
  }
}

pub trait QStandardItemModel_insertRows<RetType> {
  fn insertRows(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  bool QStandardItemModel::insertRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_insertRows<i8> for (i32, i32, Option<&'a QModelIndex>) {
  fn insertRows(self , rsthis: & QStandardItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel10insertRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = (if self.2.is_none() {QModelIndex::new(()).qclsinst} else {self.2.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZN18QStandardItemModel10insertRowsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QStandardItemModel::insertRow(int row, QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_insertRow<()> for (i32, &'a QStandardItem) {
  fn insertRow(self , rsthis: & QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel9insertRowEiP13QStandardItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN18QStandardItemModel9insertRowEiP13QStandardItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QStandardItem * QStandardItemModel::invisibleRootItem();
impl /*struct*/ QStandardItemModel {
  pub fn invisibleRootItem<RetType, T: QStandardItemModel_invisibleRootItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.invisibleRootItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_invisibleRootItem<RetType> {
  fn invisibleRootItem(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  QStandardItem * QStandardItemModel::invisibleRootItem();
impl<'a> /*trait*/ QStandardItemModel_invisibleRootItem<QStandardItem> for () {
  fn invisibleRootItem(self , rsthis: & QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel17invisibleRootItemEv()};
    let mut ret = unsafe {C_ZNK18QStandardItemModel17invisibleRootItemEv(rsthis.qclsinst)};
    let mut ret1 = QStandardItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStandardItemModel::setItemPrototype(const QStandardItem * item);
impl /*struct*/ QStandardItemModel {
  pub fn setItemPrototype<RetType, T: QStandardItemModel_setItemPrototype<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemPrototype(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setItemPrototype<RetType> {
  fn setItemPrototype(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  void QStandardItemModel::setItemPrototype(const QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_setItemPrototype<()> for (&'a QStandardItem) {
  fn setItemPrototype(self , rsthis: & QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel16setItemPrototypeEPK13QStandardItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QStandardItemModel16setItemPrototypeEPK13QStandardItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStandardItemModel::setHorizontalHeaderLabels(const QStringList & labels);
impl /*struct*/ QStandardItemModel {
  pub fn setHorizontalHeaderLabels<RetType, T: QStandardItemModel_setHorizontalHeaderLabels<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalHeaderLabels(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setHorizontalHeaderLabels<RetType> {
  fn setHorizontalHeaderLabels(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  void QStandardItemModel::setHorizontalHeaderLabels(const QStringList & labels);
impl<'a> /*trait*/ QStandardItemModel_setHorizontalHeaderLabels<()> for (&'a QStringList) {
  fn setHorizontalHeaderLabels(self , rsthis: & QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel25setHorizontalHeaderLabelsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QStandardItemModel25setHorizontalHeaderLabelsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QModelIndex QStandardItemModel::parent(const QModelIndex & child);
impl /*struct*/ QStandardItemModel {
  pub fn parent<RetType, T: QStandardItemModel_parent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QStandardItemModel_parent<RetType> {
  fn parent(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  QModelIndex QStandardItemModel::parent(const QModelIndex & child);
impl<'a> /*trait*/ QStandardItemModel_parent<QModelIndex> for (&'a QModelIndex) {
  fn parent(self , rsthis: & QStandardItemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel6parentERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK18QStandardItemModel6parentERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QStandardItemModel::removeColumns(int column, int count, const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn removeColumns<RetType, T: QStandardItemModel_removeColumns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeColumns(self);
    // return 1;
  }
}

pub trait QStandardItemModel_removeColumns<RetType> {
  fn removeColumns(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  bool QStandardItemModel::removeColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_removeColumns<i8> for (i32, i32, Option<&'a QModelIndex>) {
  fn removeColumns(self , rsthis: & QStandardItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel13removeColumnsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = (if self.2.is_none() {QModelIndex::new(()).qclsinst} else {self.2.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZN18QStandardItemModel13removeColumnsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QModelIndex QStandardItemModel::sibling(int row, int column, const QModelIndex & idx);
impl /*struct*/ QStandardItemModel {
  pub fn sibling<RetType, T: QStandardItemModel_sibling<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sibling(self);
    // return 1;
  }
}

pub trait QStandardItemModel_sibling<RetType> {
  fn sibling(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  QModelIndex QStandardItemModel::sibling(int row, int column, const QModelIndex & idx);
impl<'a> /*trait*/ QStandardItemModel_sibling<QModelIndex> for (i32, i32, &'a QModelIndex) {
  fn sibling(self , rsthis: & QStandardItemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel7siblingEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK18QStandardItemModel7siblingEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QStandardItemModel::sortRole();
impl /*struct*/ QStandardItemModel {
  pub fn sortRole<RetType, T: QStandardItemModel_sortRole<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sortRole(self);
    // return 1;
  }
}

pub trait QStandardItemModel_sortRole<RetType> {
  fn sortRole(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  int QStandardItemModel::sortRole();
impl<'a> /*trait*/ QStandardItemModel_sortRole<i32> for () {
  fn sortRole(self , rsthis: & QStandardItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel8sortRoleEv()};
    let mut ret = unsafe {C_ZNK18QStandardItemModel8sortRoleEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QStandardItem * QStandardItemModel::takeHorizontalHeaderItem(int column);
impl /*struct*/ QStandardItemModel {
  pub fn takeHorizontalHeaderItem<RetType, T: QStandardItemModel_takeHorizontalHeaderItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeHorizontalHeaderItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_takeHorizontalHeaderItem<RetType> {
  fn takeHorizontalHeaderItem(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  QStandardItem * QStandardItemModel::takeHorizontalHeaderItem(int column);
impl<'a> /*trait*/ QStandardItemModel_takeHorizontalHeaderItem<QStandardItem> for (i32) {
  fn takeHorizontalHeaderItem(self , rsthis: & QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel24takeHorizontalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN18QStandardItemModel24takeHorizontalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QStandardItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QStandardItemModel::indexFromItem(const QStandardItem * item);
impl /*struct*/ QStandardItemModel {
  pub fn indexFromItem<RetType, T: QStandardItemModel_indexFromItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexFromItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_indexFromItem<RetType> {
  fn indexFromItem(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  QModelIndex QStandardItemModel::indexFromItem(const QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_indexFromItem<QModelIndex> for (&'a QStandardItem) {
  fn indexFromItem(self , rsthis: & QStandardItemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel13indexFromItemEPK13QStandardItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK18QStandardItemModel13indexFromItemEPK13QStandardItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QStandardItem * QStandardItemModel::itemPrototype();
impl /*struct*/ QStandardItemModel {
  pub fn itemPrototype<RetType, T: QStandardItemModel_itemPrototype<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemPrototype(self);
    // return 1;
  }
}

pub trait QStandardItemModel_itemPrototype<RetType> {
  fn itemPrototype(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  const QStandardItem * QStandardItemModel::itemPrototype();
impl<'a> /*trait*/ QStandardItemModel_itemPrototype<QStandardItem> for () {
  fn itemPrototype(self , rsthis: & QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel13itemPrototypeEv()};
    let mut ret = unsafe {C_ZNK18QStandardItemModel13itemPrototypeEv(rsthis.qclsinst)};
    let mut ret1 = QStandardItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStandardItemModel::setHorizontalHeaderItem(int column, QStandardItem * item);
impl /*struct*/ QStandardItemModel {
  pub fn setHorizontalHeaderItem<RetType, T: QStandardItemModel_setHorizontalHeaderItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalHeaderItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setHorizontalHeaderItem<RetType> {
  fn setHorizontalHeaderItem(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  void QStandardItemModel::setHorizontalHeaderItem(int column, QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_setHorizontalHeaderItem<()> for (i32, &'a QStandardItem) {
  fn setHorizontalHeaderItem(self , rsthis: & QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel23setHorizontalHeaderItemEiP13QStandardItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN18QStandardItemModel23setHorizontalHeaderItemEiP13QStandardItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QStandardItem * QStandardItemModel::horizontalHeaderItem(int column);
impl /*struct*/ QStandardItemModel {
  pub fn horizontalHeaderItem<RetType, T: QStandardItemModel_horizontalHeaderItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.horizontalHeaderItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_horizontalHeaderItem<RetType> {
  fn horizontalHeaderItem(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  QStandardItem * QStandardItemModel::horizontalHeaderItem(int column);
impl<'a> /*trait*/ QStandardItemModel_horizontalHeaderItem<QStandardItem> for (i32) {
  fn horizontalHeaderItem(self , rsthis: & QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel20horizontalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK18QStandardItemModel20horizontalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QStandardItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStandardItemModel::appendRow(QStandardItem * item);
impl /*struct*/ QStandardItemModel {
  pub fn appendRow<RetType, T: QStandardItemModel_appendRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.appendRow(self);
    // return 1;
  }
}

pub trait QStandardItemModel_appendRow<RetType> {
  fn appendRow(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  void QStandardItemModel::appendRow(QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_appendRow<()> for (&'a QStandardItem) {
  fn appendRow(self , rsthis: & QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel9appendRowEP13QStandardItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QStandardItemModel9appendRowEP13QStandardItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QMap<int, QVariant> QStandardItemModel::itemData(const QModelIndex & index);
impl /*struct*/ QStandardItemModel {
  pub fn itemData<RetType, T: QStandardItemModel_itemData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemData(self);
    // return 1;
  }
}

pub trait QStandardItemModel_itemData<RetType> {
  fn itemData(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  QMap<int, QVariant> QStandardItemModel::itemData(const QModelIndex & index);
impl<'a> /*trait*/ QStandardItemModel_itemData<u64> for (&'a QModelIndex) {
  fn itemData(self , rsthis: & QStandardItemModel) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel8itemDataERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK18QStandardItemModel8itemDataERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  void QStandardItemModel::setSortRole(int role);
impl /*struct*/ QStandardItemModel {
  pub fn setSortRole<RetType, T: QStandardItemModel_setSortRole<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSortRole(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setSortRole<RetType> {
  fn setSortRole(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  void QStandardItemModel::setSortRole(int role);
impl<'a> /*trait*/ QStandardItemModel_setSortRole<()> for (i32) {
  fn setSortRole(self , rsthis: & QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel11setSortRoleEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN18QStandardItemModel11setSortRoleEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QStandardItemModel::hasChildren(const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn hasChildren<RetType, T: QStandardItemModel_hasChildren<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasChildren(self);
    // return 1;
  }
}

pub trait QStandardItemModel_hasChildren<RetType> {
  fn hasChildren(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  bool QStandardItemModel::hasChildren(const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_hasChildren<i8> for (Option<&'a QModelIndex>) {
  fn hasChildren(self , rsthis: & QStandardItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel11hasChildrenERK11QModelIndex()};
    let arg0 = (if self.is_none() {QModelIndex::new(()).qclsinst} else {self.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZNK18QStandardItemModel11hasChildrenERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QStandardItemModel::~QStandardItemModel();
impl /*struct*/ QStandardItemModel {
  pub fn free<RetType, T: QStandardItemModel_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QStandardItemModel_free<RetType> {
  fn free(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  void QStandardItemModel::~QStandardItemModel();
impl<'a> /*trait*/ QStandardItemModel_free<()> for () {
  fn free(self , rsthis: & QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModelD2Ev()};
     unsafe {C_ZN18QStandardItemModelD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QVariant QStandardItemModel::data(const QModelIndex & index, int role);
impl /*struct*/ QStandardItemModel {
  pub fn data<RetType, T: QStandardItemModel_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QStandardItemModel_data<RetType> {
  fn data(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  QVariant QStandardItemModel::data(const QModelIndex & index, int role);
impl<'a> /*trait*/ QStandardItemModel_data<QVariant> for (&'a QModelIndex, Option<i32>) {
  fn data(self , rsthis: & QStandardItemModel) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel4dataERK11QModelIndexi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {0 as i32} else {self.1.unwrap()})  as c_int;
    let mut ret = unsafe {C_ZNK18QStandardItemModel4dataERK11QModelIndexi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QStandardItem *> QStandardItemModel::takeRow(int row);
impl /*struct*/ QStandardItemModel {
  pub fn takeRow<RetType, T: QStandardItemModel_takeRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeRow(self);
    // return 1;
  }
}

pub trait QStandardItemModel_takeRow<RetType> {
  fn takeRow(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  QList<QStandardItem *> QStandardItemModel::takeRow(int row);
impl<'a> /*trait*/ QStandardItemModel_takeRow<u64> for (i32) {
  fn takeRow(self , rsthis: & QStandardItemModel) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel7takeRowEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN18QStandardItemModel7takeRowEi(rsthis.qclsinst, arg0)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  void QStandardItemModel::setColumnCount(int columns);
impl /*struct*/ QStandardItemModel {
  pub fn setColumnCount<RetType, T: QStandardItemModel_setColumnCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColumnCount(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setColumnCount<RetType> {
  fn setColumnCount(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  void QStandardItemModel::setColumnCount(int columns);
impl<'a> /*trait*/ QStandardItemModel_setColumnCount<()> for (i32) {
  fn setColumnCount(self , rsthis: & QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel14setColumnCountEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN18QStandardItemModel14setColumnCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStandardItem * QStandardItemModel::verticalHeaderItem(int row);
impl /*struct*/ QStandardItemModel {
  pub fn verticalHeaderItem<RetType, T: QStandardItemModel_verticalHeaderItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.verticalHeaderItem(self);
    // return 1;
  }
}

pub trait QStandardItemModel_verticalHeaderItem<RetType> {
  fn verticalHeaderItem(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  QStandardItem * QStandardItemModel::verticalHeaderItem(int row);
impl<'a> /*trait*/ QStandardItemModel_verticalHeaderItem<QStandardItem> for (i32) {
  fn verticalHeaderItem(self , rsthis: & QStandardItemModel) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel18verticalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK18QStandardItemModel18verticalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QStandardItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QStandardItemModel::removeRows(int row, int count, const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn removeRows<RetType, T: QStandardItemModel_removeRows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeRows(self);
    // return 1;
  }
}

pub trait QStandardItemModel_removeRows<RetType> {
  fn removeRows(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  bool QStandardItemModel::removeRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_removeRows<i8> for (i32, i32, Option<&'a QModelIndex>) {
  fn removeRows(self , rsthis: & QStandardItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel10removeRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = (if self.2.is_none() {QModelIndex::new(()).qclsinst} else {self.2.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZN18QStandardItemModel10removeRowsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QStandardItemModel::setVerticalHeaderLabels(const QStringList & labels);
impl /*struct*/ QStandardItemModel {
  pub fn setVerticalHeaderLabels<RetType, T: QStandardItemModel_setVerticalHeaderLabels<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVerticalHeaderLabels(self);
    // return 1;
  }
}

pub trait QStandardItemModel_setVerticalHeaderLabels<RetType> {
  fn setVerticalHeaderLabels(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  void QStandardItemModel::setVerticalHeaderLabels(const QStringList & labels);
impl<'a> /*trait*/ QStandardItemModel_setVerticalHeaderLabels<()> for (&'a QStringList) {
  fn setVerticalHeaderLabels(self , rsthis: & QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel23setVerticalHeaderLabelsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QStandardItemModel23setVerticalHeaderLabelsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStandardItemModel::setItem(int row, int column, QStandardItem * item);
impl<'a> /*trait*/ QStandardItemModel_setItem<()> for (i32, i32, &'a QStandardItem) {
  fn setItem(self , rsthis: & QStandardItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStandardItemModel7setItemEiiP13QStandardItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN18QStandardItemModel7setItemEiiP13QStandardItem(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QStringList QStandardItemModel::mimeTypes();
impl /*struct*/ QStandardItemModel {
  pub fn mimeTypes<RetType, T: QStandardItemModel_mimeTypes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mimeTypes(self);
    // return 1;
  }
}

pub trait QStandardItemModel_mimeTypes<RetType> {
  fn mimeTypes(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  QStringList QStandardItemModel::mimeTypes();
impl<'a> /*trait*/ QStandardItemModel_mimeTypes<QStringList> for () {
  fn mimeTypes(self , rsthis: & QStandardItemModel) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel9mimeTypesEv()};
    let mut ret = unsafe {C_ZNK18QStandardItemModel9mimeTypesEv(rsthis.qclsinst)};
    let mut ret1 = QStringList::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QStandardItemModel::rowCount(const QModelIndex & parent);
impl /*struct*/ QStandardItemModel {
  pub fn rowCount<RetType, T: QStandardItemModel_rowCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QStandardItemModel_rowCount<RetType> {
  fn rowCount(self , rsthis: & QStandardItemModel) -> RetType;
}

  // proto:  int QStandardItemModel::rowCount(const QModelIndex & parent);
impl<'a> /*trait*/ QStandardItemModel_rowCount<i32> for (Option<&'a QModelIndex>) {
  fn rowCount(self , rsthis: & QStandardItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QStandardItemModel8rowCountERK11QModelIndex()};
    let arg0 = (if self.is_none() {QModelIndex::new(()).qclsinst} else {self.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZNK18QStandardItemModel8rowCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

impl /*struct*/ QStandardItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStandardItem {
    return QStandardItem{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QStandardItem::setChild(int row, QStandardItem * item);
impl /*struct*/ QStandardItem {
  pub fn setChild<RetType, T: QStandardItem_setChild<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setChild(self);
    // return 1;
  }
}

pub trait QStandardItem_setChild<RetType> {
  fn setChild(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setChild(int row, QStandardItem * item);
impl<'a> /*trait*/ QStandardItem_setChild<()> for (i32, &'a QStandardItem) {
  fn setChild(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem8setChildEiPS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStandardItem8setChildEiPS_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QStandardItemModel * QStandardItem::model();
impl /*struct*/ QStandardItem {
  pub fn model<RetType, T: QStandardItem_model<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.model(self);
    // return 1;
  }
}

pub trait QStandardItem_model<RetType> {
  fn model(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QStandardItemModel * QStandardItem::model();
impl<'a> /*trait*/ QStandardItem_model<QStandardItemModel> for () {
  fn model(self , rsthis: & QStandardItem) -> QStandardItemModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem5modelEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem5modelEv(rsthis.qclsinst)};
    let mut ret1 = QStandardItemModel::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStandardItem::insertColumns(int column, int count);
impl /*struct*/ QStandardItem {
  pub fn insertColumns<RetType, T: QStandardItem_insertColumns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertColumns(self);
    // return 1;
  }
}

pub trait QStandardItem_insertColumns<RetType> {
  fn insertColumns(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::insertColumns(int column, int count);
impl<'a> /*trait*/ QStandardItem_insertColumns<()> for (i32, i32) {
  fn insertColumns(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13insertColumnsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN13QStandardItem13insertColumnsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QStandardItem::setSelectable(bool selectable);
impl /*struct*/ QStandardItem {
  pub fn setSelectable<RetType, T: QStandardItem_setSelectable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSelectable(self);
    // return 1;
  }
}

pub trait QStandardItem_setSelectable<RetType> {
  fn setSelectable(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setSelectable(bool selectable);
impl<'a> /*trait*/ QStandardItem_setSelectable<()> for (i8) {
  fn setSelectable(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13setSelectableEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QStandardItem13setSelectableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QStandardItem::column();
impl /*struct*/ QStandardItem {
  pub fn column<RetType, T: QStandardItem_column<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.column(self);
    // return 1;
  }
}

pub trait QStandardItem_column<RetType> {
  fn column(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  int QStandardItem::column();
impl<'a> /*trait*/ QStandardItem_column<i32> for () {
  fn column(self , rsthis: & QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem6columnEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem6columnEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QString QStandardItem::whatsThis();
impl /*struct*/ QStandardItem {
  pub fn whatsThis<RetType, T: QStandardItem_whatsThis<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.whatsThis(self);
    // return 1;
  }
}

pub trait QStandardItem_whatsThis<RetType> {
  fn whatsThis(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QString QStandardItem::whatsThis();
impl<'a> /*trait*/ QStandardItem_whatsThis<QString> for () {
  fn whatsThis(self , rsthis: & QStandardItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem9whatsThisEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem9whatsThisEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QStandardItem *> QStandardItem::takeColumn(int column);
impl /*struct*/ QStandardItem {
  pub fn takeColumn<RetType, T: QStandardItem_takeColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeColumn(self);
    // return 1;
  }
}

pub trait QStandardItem_takeColumn<RetType> {
  fn takeColumn(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QList<QStandardItem *> QStandardItem::takeColumn(int column);
impl<'a> /*trait*/ QStandardItem_takeColumn<u64> for (i32) {
  fn takeColumn(self , rsthis: & QStandardItem) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10takeColumnEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN13QStandardItem10takeColumnEi(rsthis.qclsinst, arg0)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  void QStandardItem::setForeground(const QBrush & brush);
impl /*struct*/ QStandardItem {
  pub fn setForeground<RetType, T: QStandardItem_setForeground<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setForeground(self);
    // return 1;
  }
}

pub trait QStandardItem_setForeground<RetType> {
  fn setForeground(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setForeground(const QBrush & brush);
impl<'a> /*trait*/ QStandardItem_setForeground<()> for (&'a QBrush) {
  fn setForeground(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13setForegroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStandardItem13setForegroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QStandardItem::isEditable();
impl /*struct*/ QStandardItem {
  pub fn isEditable<RetType, T: QStandardItem_isEditable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEditable(self);
    // return 1;
  }
}

pub trait QStandardItem_isEditable<RetType> {
  fn isEditable(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  bool QStandardItem::isEditable();
impl<'a> /*trait*/ QStandardItem_isEditable<i8> for () {
  fn isEditable(self , rsthis: & QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem10isEditableEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem10isEditableEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QIcon QStandardItem::icon();
impl /*struct*/ QStandardItem {
  pub fn icon<RetType, T: QStandardItem_icon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.icon(self);
    // return 1;
  }
}

pub trait QStandardItem_icon<RetType> {
  fn icon(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QIcon QStandardItem::icon();
impl<'a> /*trait*/ QStandardItem_icon<QIcon> for () {
  fn icon(self , rsthis: & QStandardItem) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem4iconEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem4iconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStandardItem::setWhatsThis(const QString & whatsThis);
impl /*struct*/ QStandardItem {
  pub fn setWhatsThis<RetType, T: QStandardItem_setWhatsThis<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWhatsThis(self);
    // return 1;
  }
}

pub trait QStandardItem_setWhatsThis<RetType> {
  fn setWhatsThis(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setWhatsThis(const QString & whatsThis);
impl<'a> /*trait*/ QStandardItem_setWhatsThis<()> for (&'a QString) {
  fn setWhatsThis(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStandardItem12setWhatsThisERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStandardItem * QStandardItem::takeChild(int row, int column);
impl /*struct*/ QStandardItem {
  pub fn takeChild<RetType, T: QStandardItem_takeChild<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeChild(self);
    // return 1;
  }
}

pub trait QStandardItem_takeChild<RetType> {
  fn takeChild(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QStandardItem * QStandardItem::takeChild(int row, int column);
impl<'a> /*trait*/ QStandardItem_takeChild<QStandardItem> for (i32, Option<i32>) {
  fn takeChild(self , rsthis: & QStandardItem) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem9takeChildEii()};
    let arg0 = self.0  as c_int;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap()})  as c_int;
    let mut ret = unsafe {C_ZN13QStandardItem9takeChildEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QStandardItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QStandardItem::type();
impl /*struct*/ QStandardItem {
  pub fn type_<RetType, T: QStandardItem_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QStandardItem_type_<RetType> {
  fn type_(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  int QStandardItem::type();
impl<'a> /*trait*/ QStandardItem_type_<i32> for () {
  fn type_(self , rsthis: & QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem4typeEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem4typeEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QList<QStandardItem *> QStandardItem::takeRow(int row);
impl /*struct*/ QStandardItem {
  pub fn takeRow<RetType, T: QStandardItem_takeRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeRow(self);
    // return 1;
  }
}

pub trait QStandardItem_takeRow<RetType> {
  fn takeRow(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QList<QStandardItem *> QStandardItem::takeRow(int row);
impl<'a> /*trait*/ QStandardItem_takeRow<u64> for (i32) {
  fn takeRow(self , rsthis: & QStandardItem) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem7takeRowEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN13QStandardItem7takeRowEi(rsthis.qclsinst, arg0)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  int QStandardItem::row();
impl /*struct*/ QStandardItem {
  pub fn row<RetType, T: QStandardItem_row<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.row(self);
    // return 1;
  }
}

pub trait QStandardItem_row<RetType> {
  fn row(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  int QStandardItem::row();
impl<'a> /*trait*/ QStandardItem_row<i32> for () {
  fn row(self , rsthis: & QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem3rowEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem3rowEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QStandardItem::isCheckable();
impl /*struct*/ QStandardItem {
  pub fn isCheckable<RetType, T: QStandardItem_isCheckable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCheckable(self);
    // return 1;
  }
}

pub trait QStandardItem_isCheckable<RetType> {
  fn isCheckable(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  bool QStandardItem::isCheckable();
impl<'a> /*trait*/ QStandardItem_isCheckable<i8> for () {
  fn isCheckable(self , rsthis: & QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem11isCheckableEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem11isCheckableEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString QStandardItem::text();
impl /*struct*/ QStandardItem {
  pub fn text<RetType, T: QStandardItem_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QStandardItem_text<RetType> {
  fn text(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QString QStandardItem::text();
impl<'a> /*trait*/ QStandardItem_text<QString> for () {
  fn text(self , rsthis: & QStandardItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem4textEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStandardItem::insertRows(int row, int count);
impl /*struct*/ QStandardItem {
  pub fn insertRows<RetType, T: QStandardItem_insertRows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertRows(self);
    // return 1;
  }
}

pub trait QStandardItem_insertRows<RetType> {
  fn insertRows(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::insertRows(int row, int count);
impl<'a> /*trait*/ QStandardItem_insertRows<()> for (i32, i32) {
  fn insertRows(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10insertRowsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN13QStandardItem10insertRowsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QStandardItem::isDropEnabled();
impl /*struct*/ QStandardItem {
  pub fn isDropEnabled<RetType, T: QStandardItem_isDropEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDropEnabled(self);
    // return 1;
  }
}

pub trait QStandardItem_isDropEnabled<RetType> {
  fn isDropEnabled(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  bool QStandardItem::isDropEnabled();
impl<'a> /*trait*/ QStandardItem_isDropEnabled<i8> for () {
  fn isDropEnabled(self , rsthis: & QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem13isDropEnabledEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem13isDropEnabledEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QStandardItem::hasChildren();
impl /*struct*/ QStandardItem {
  pub fn hasChildren<RetType, T: QStandardItem_hasChildren<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasChildren(self);
    // return 1;
  }
}

pub trait QStandardItem_hasChildren<RetType> {
  fn hasChildren(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  bool QStandardItem::hasChildren();
impl<'a> /*trait*/ QStandardItem_hasChildren<i8> for () {
  fn hasChildren(self , rsthis: & QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem11hasChildrenEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem11hasChildrenEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString QStandardItem::statusTip();
impl /*struct*/ QStandardItem {
  pub fn statusTip<RetType, T: QStandardItem_statusTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.statusTip(self);
    // return 1;
  }
}

pub trait QStandardItem_statusTip<RetType> {
  fn statusTip(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QString QStandardItem::statusTip();
impl<'a> /*trait*/ QStandardItem_statusTip<QString> for () {
  fn statusTip(self , rsthis: & QStandardItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem9statusTipEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem9statusTipEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStandardItem::setStatusTip(const QString & statusTip);
impl /*struct*/ QStandardItem {
  pub fn setStatusTip<RetType, T: QStandardItem_setStatusTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStatusTip(self);
    // return 1;
  }
}

pub trait QStandardItem_setStatusTip<RetType> {
  fn setStatusTip(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setStatusTip(const QString & statusTip);
impl<'a> /*trait*/ QStandardItem_setStatusTip<()> for (&'a QString) {
  fn setStatusTip(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem12setStatusTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStandardItem12setStatusTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStandardItem::appendRow(QStandardItem * item);
impl /*struct*/ QStandardItem {
  pub fn appendRow<RetType, T: QStandardItem_appendRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.appendRow(self);
    // return 1;
  }
}

pub trait QStandardItem_appendRow<RetType> {
  fn appendRow(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::appendRow(QStandardItem * item);
impl<'a> /*trait*/ QStandardItem_appendRow<()> for (&'a QStandardItem) {
  fn appendRow(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem9appendRowEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStandardItem9appendRowEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStandardItem::setChild(int row, int column, QStandardItem * item);
impl<'a> /*trait*/ QStandardItem_setChild<()> for (i32, i32, &'a QStandardItem) {
  fn setChild(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem8setChildEiiPS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStandardItem8setChildEiiPS_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QModelIndex QStandardItem::index();
impl /*struct*/ QStandardItem {
  pub fn index<RetType, T: QStandardItem_index<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.index(self);
    // return 1;
  }
}

pub trait QStandardItem_index<RetType> {
  fn index(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QModelIndex QStandardItem::index();
impl<'a> /*trait*/ QStandardItem_index<QModelIndex> for () {
  fn index(self , rsthis: & QStandardItem) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem5indexEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem5indexEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStandardItem::setIcon(const QIcon & icon);
impl /*struct*/ QStandardItem {
  pub fn setIcon<RetType, T: QStandardItem_setIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIcon(self);
    // return 1;
  }
}

pub trait QStandardItem_setIcon<RetType> {
  fn setIcon(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QStandardItem_setIcon<()> for (&'a QIcon) {
  fn setIcon(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStandardItem7setIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStandardItem::setToolTip(const QString & toolTip);
impl /*struct*/ QStandardItem {
  pub fn setToolTip<RetType, T: QStandardItem_setToolTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setToolTip(self);
    // return 1;
  }
}

pub trait QStandardItem_setToolTip<RetType> {
  fn setToolTip(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setToolTip(const QString & toolTip);
impl<'a> /*trait*/ QStandardItem_setToolTip<()> for (&'a QString) {
  fn setToolTip(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStandardItem10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStandardItem::setData(const QVariant & value, int role);
impl /*struct*/ QStandardItem {
  pub fn setData<RetType, T: QStandardItem_setData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QStandardItem_setData<RetType> {
  fn setData(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setData(const QVariant & value, int role);
impl<'a> /*trait*/ QStandardItem_setData<()> for (&'a QVariant, Option<i32>) {
  fn setData(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem7setDataERK8QVarianti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {0 as i32} else {self.1.unwrap()})  as c_int;
     unsafe {C_ZN13QStandardItem7setDataERK8QVarianti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QBrush QStandardItem::background();
impl /*struct*/ QStandardItem {
  pub fn background<RetType, T: QStandardItem_background<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.background(self);
    // return 1;
  }
}

pub trait QStandardItem_background<RetType> {
  fn background(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QBrush QStandardItem::background();
impl<'a> /*trait*/ QStandardItem_background<QBrush> for () {
  fn background(self , rsthis: & QStandardItem) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem10backgroundEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem10backgroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QStandardItem::data(int role);
impl /*struct*/ QStandardItem {
  pub fn data<RetType, T: QStandardItem_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QStandardItem_data<RetType> {
  fn data(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QVariant QStandardItem::data(int role);
impl<'a> /*trait*/ QStandardItem_data<QVariant> for (Option<i32>) {
  fn data(self , rsthis: & QStandardItem) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem4dataEi()};
    let arg0 = (if self.is_none() {0 as i32} else {self.unwrap()})  as c_int;
    let mut ret = unsafe {C_ZNK13QStandardItem4dataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStandardItem * QStandardItem::child(int row, int column);
impl /*struct*/ QStandardItem {
  pub fn child<RetType, T: QStandardItem_child<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.child(self);
    // return 1;
  }
}

pub trait QStandardItem_child<RetType> {
  fn child(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QStandardItem * QStandardItem::child(int row, int column);
impl<'a> /*trait*/ QStandardItem_child<QStandardItem> for (i32, Option<i32>) {
  fn child(self , rsthis: & QStandardItem) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem5childEii()};
    let arg0 = self.0  as c_int;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap()})  as c_int;
    let mut ret = unsafe {C_ZNK13QStandardItem5childEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QStandardItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QStandardItem::isSelectable();
impl /*struct*/ QStandardItem {
  pub fn isSelectable<RetType, T: QStandardItem_isSelectable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSelectable(self);
    // return 1;
  }
}

pub trait QStandardItem_isSelectable<RetType> {
  fn isSelectable(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  bool QStandardItem::isSelectable();
impl<'a> /*trait*/ QStandardItem_isSelectable<i8> for () {
  fn isSelectable(self , rsthis: & QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem12isSelectableEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem12isSelectableEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString QStandardItem::toolTip();
impl /*struct*/ QStandardItem {
  pub fn toolTip<RetType, T: QStandardItem_toolTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toolTip(self);
    // return 1;
  }
}

pub trait QStandardItem_toolTip<RetType> {
  fn toolTip(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QString QStandardItem::toolTip();
impl<'a> /*trait*/ QStandardItem_toolTip<QString> for () {
  fn toolTip(self , rsthis: & QStandardItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem7toolTipEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStandardItem::setRowCount(int rows);
impl /*struct*/ QStandardItem {
  pub fn setRowCount<RetType, T: QStandardItem_setRowCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRowCount(self);
    // return 1;
  }
}

pub trait QStandardItem_setRowCount<RetType> {
  fn setRowCount(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setRowCount(int rows);
impl<'a> /*trait*/ QStandardItem_setRowCount<()> for (i32) {
  fn setRowCount(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem11setRowCountEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN13QStandardItem11setRowCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStandardItem::QStandardItem(const QString & text);
impl /*struct*/ QStandardItem {
  pub fn new<T: QStandardItem_new>(value: T) -> QStandardItem {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStandardItem_new {
  fn new(self) -> QStandardItem;
}

  // proto:  void QStandardItem::QStandardItem(const QString & text);
impl<'a> /*trait*/ QStandardItem_new for (&'a QString) {
  fn new(self) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItemC2ERK7QString()};
    let ctysz: c_int = unsafe{QStandardItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QStandardItemC2ERK7QString(arg0)};
    let rsthis = QStandardItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStandardItem::write(QDataStream & out);
impl /*struct*/ QStandardItem {
  pub fn write<RetType, T: QStandardItem_write<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.write(self);
    // return 1;
  }
}

pub trait QStandardItem_write<RetType> {
  fn write(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::write(QDataStream & out);
impl<'a> /*trait*/ QStandardItem_write<()> for (&'a QDataStream) {
  fn write(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem5writeER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZNK13QStandardItem5writeER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QStandardItem::isDragEnabled();
impl /*struct*/ QStandardItem {
  pub fn isDragEnabled<RetType, T: QStandardItem_isDragEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDragEnabled(self);
    // return 1;
  }
}

pub trait QStandardItem_isDragEnabled<RetType> {
  fn isDragEnabled(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  bool QStandardItem::isDragEnabled();
impl<'a> /*trait*/ QStandardItem_isDragEnabled<i8> for () {
  fn isDragEnabled(self , rsthis: & QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem13isDragEnabledEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem13isDragEnabledEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QStandardItem::setAccessibleText(const QString & accessibleText);
impl /*struct*/ QStandardItem {
  pub fn setAccessibleText<RetType, T: QStandardItem_setAccessibleText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAccessibleText(self);
    // return 1;
  }
}

pub trait QStandardItem_setAccessibleText<RetType> {
  fn setAccessibleText(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setAccessibleText(const QString & accessibleText);
impl<'a> /*trait*/ QStandardItem_setAccessibleText<()> for (&'a QString) {
  fn setAccessibleText(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem17setAccessibleTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStandardItem17setAccessibleTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QStandardItem::rowCount();
impl /*struct*/ QStandardItem {
  pub fn rowCount<RetType, T: QStandardItem_rowCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QStandardItem_rowCount<RetType> {
  fn rowCount(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  int QStandardItem::rowCount();
impl<'a> /*trait*/ QStandardItem_rowCount<i32> for () {
  fn rowCount(self , rsthis: & QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem8rowCountEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem8rowCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QStandardItem::removeColumn(int column);
impl /*struct*/ QStandardItem {
  pub fn removeColumn<RetType, T: QStandardItem_removeColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeColumn(self);
    // return 1;
  }
}

pub trait QStandardItem_removeColumn<RetType> {
  fn removeColumn(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::removeColumn(int column);
impl<'a> /*trait*/ QStandardItem_removeColumn<()> for (i32) {
  fn removeColumn(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem12removeColumnEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN13QStandardItem12removeColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStandardItem::removeRow(int row);
impl /*struct*/ QStandardItem {
  pub fn removeRow<RetType, T: QStandardItem_removeRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeRow(self);
    // return 1;
  }
}

pub trait QStandardItem_removeRow<RetType> {
  fn removeRow(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::removeRow(int row);
impl<'a> /*trait*/ QStandardItem_removeRow<()> for (i32) {
  fn removeRow(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem9removeRowEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN13QStandardItem9removeRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QStandardItem::columnCount();
impl /*struct*/ QStandardItem {
  pub fn columnCount<RetType, T: QStandardItem_columnCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QStandardItem_columnCount<RetType> {
  fn columnCount(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  int QStandardItem::columnCount();
impl<'a> /*trait*/ QStandardItem_columnCount<i32> for () {
  fn columnCount(self , rsthis: & QStandardItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem11columnCountEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem11columnCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QStandardItem::isTristate();
impl /*struct*/ QStandardItem {
  pub fn isTristate<RetType, T: QStandardItem_isTristate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTristate(self);
    // return 1;
  }
}

pub trait QStandardItem_isTristate<RetType> {
  fn isTristate(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  bool QStandardItem::isTristate();
impl<'a> /*trait*/ QStandardItem_isTristate<i8> for () {
  fn isTristate(self , rsthis: & QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem10isTristateEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem10isTristateEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QStandardItem * QStandardItem::parent();
impl /*struct*/ QStandardItem {
  pub fn parent<RetType, T: QStandardItem_parent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QStandardItem_parent<RetType> {
  fn parent(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QStandardItem * QStandardItem::parent();
impl<'a> /*trait*/ QStandardItem_parent<QStandardItem> for () {
  fn parent(self , rsthis: & QStandardItem) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem6parentEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem6parentEv(rsthis.qclsinst)};
    let mut ret1 = QStandardItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStandardItem::insertRow(int row, QStandardItem * item);
impl /*struct*/ QStandardItem {
  pub fn insertRow<RetType, T: QStandardItem_insertRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertRow(self);
    // return 1;
  }
}

pub trait QStandardItem_insertRow<RetType> {
  fn insertRow(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::insertRow(int row, QStandardItem * item);
impl<'a> /*trait*/ QStandardItem_insertRow<()> for (i32, &'a QStandardItem) {
  fn insertRow(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem9insertRowEiPS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStandardItem9insertRowEiPS_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QStandardItem::QStandardItem(const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QStandardItem_new for (&'a QIcon, &'a QString) {
  fn new(self) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItemC2ERK5QIconRK7QString()};
    let ctysz: c_int = unsafe{QStandardItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QStandardItemC2ERK5QIconRK7QString(arg0, arg1)};
    let rsthis = QStandardItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStandardItem::setFont(const QFont & font);
impl /*struct*/ QStandardItem {
  pub fn setFont<RetType, T: QStandardItem_setFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFont(self);
    // return 1;
  }
}

pub trait QStandardItem_setFont<RetType> {
  fn setFont(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setFont(const QFont & font);
impl<'a> /*trait*/ QStandardItem_setFont<()> for (&'a QFont) {
  fn setFont(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStandardItem7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStandardItem::removeColumns(int column, int count);
impl /*struct*/ QStandardItem {
  pub fn removeColumns<RetType, T: QStandardItem_removeColumns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeColumns(self);
    // return 1;
  }
}

pub trait QStandardItem_removeColumns<RetType> {
  fn removeColumns(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::removeColumns(int column, int count);
impl<'a> /*trait*/ QStandardItem_removeColumns<()> for (i32, i32) {
  fn removeColumns(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13removeColumnsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN13QStandardItem13removeColumnsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QStandardItem::~QStandardItem();
impl /*struct*/ QStandardItem {
  pub fn free<RetType, T: QStandardItem_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QStandardItem_free<RetType> {
  fn free(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::~QStandardItem();
impl<'a> /*trait*/ QStandardItem_free<()> for () {
  fn free(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItemD2Ev()};
     unsafe {C_ZN13QStandardItemD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStandardItem::QStandardItem();
impl<'a> /*trait*/ QStandardItem_new for () {
  fn new(self) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItemC2Ev()};
    let ctysz: c_int = unsafe{QStandardItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN13QStandardItemC2Ev()};
    let rsthis = QStandardItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QFont QStandardItem::font();
impl /*struct*/ QStandardItem {
  pub fn font<RetType, T: QStandardItem_font<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QStandardItem_font<RetType> {
  fn font(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QFont QStandardItem::font();
impl<'a> /*trait*/ QStandardItem_font<QFont> for () {
  fn font(self , rsthis: & QStandardItem) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem4fontEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStandardItem::setEditable(bool editable);
impl /*struct*/ QStandardItem {
  pub fn setEditable<RetType, T: QStandardItem_setEditable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEditable(self);
    // return 1;
  }
}

pub trait QStandardItem_setEditable<RetType> {
  fn setEditable(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setEditable(bool editable);
impl<'a> /*trait*/ QStandardItem_setEditable<()> for (i8) {
  fn setEditable(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem11setEditableEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QStandardItem11setEditableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStandardItem::setText(const QString & text);
impl /*struct*/ QStandardItem {
  pub fn setText<RetType, T: QStandardItem_setText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QStandardItem_setText<RetType> {
  fn setText(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setText(const QString & text);
impl<'a> /*trait*/ QStandardItem_setText<()> for (&'a QString) {
  fn setText(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStandardItem7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStandardItem::QStandardItem(int rows, int columns);
impl<'a> /*trait*/ QStandardItem_new for (i32, Option<i32>) {
  fn new(self) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItemC2Eii()};
    let ctysz: c_int = unsafe{QStandardItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = (if self.1.is_none() {1} else {self.1.unwrap()})  as c_int;
    let qthis: u64 = unsafe {C_ZN13QStandardItemC2Eii(arg0, arg1)};
    let rsthis = QStandardItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QStandardItem::isEnabled();
impl /*struct*/ QStandardItem {
  pub fn isEnabled<RetType, T: QStandardItem_isEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEnabled(self);
    // return 1;
  }
}

pub trait QStandardItem_isEnabled<RetType> {
  fn isEnabled(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  bool QStandardItem::isEnabled();
impl<'a> /*trait*/ QStandardItem_isEnabled<i8> for () {
  fn isEnabled(self , rsthis: & QStandardItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem9isEnabledEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem9isEnabledEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QStandardItem::setDropEnabled(bool dropEnabled);
impl /*struct*/ QStandardItem {
  pub fn setDropEnabled<RetType, T: QStandardItem_setDropEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDropEnabled(self);
    // return 1;
  }
}

pub trait QStandardItem_setDropEnabled<RetType> {
  fn setDropEnabled(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setDropEnabled(bool dropEnabled);
impl<'a> /*trait*/ QStandardItem_setDropEnabled<()> for (i8) {
  fn setDropEnabled(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem14setDropEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QStandardItem14setDropEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStandardItem::setColumnCount(int columns);
impl /*struct*/ QStandardItem {
  pub fn setColumnCount<RetType, T: QStandardItem_setColumnCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColumnCount(self);
    // return 1;
  }
}

pub trait QStandardItem_setColumnCount<RetType> {
  fn setColumnCount(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setColumnCount(int columns);
impl<'a> /*trait*/ QStandardItem_setColumnCount<()> for (i32) {
  fn setColumnCount(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem14setColumnCountEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN13QStandardItem14setColumnCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QStandardItem::accessibleText();
impl /*struct*/ QStandardItem {
  pub fn accessibleText<RetType, T: QStandardItem_accessibleText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.accessibleText(self);
    // return 1;
  }
}

pub trait QStandardItem_accessibleText<RetType> {
  fn accessibleText(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QString QStandardItem::accessibleText();
impl<'a> /*trait*/ QStandardItem_accessibleText<QString> for () {
  fn accessibleText(self , rsthis: & QStandardItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem14accessibleTextEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem14accessibleTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStandardItem::read(QDataStream & in);
impl /*struct*/ QStandardItem {
  pub fn read<RetType, T: QStandardItem_read<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.read(self);
    // return 1;
  }
}

pub trait QStandardItem_read<RetType> {
  fn read(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::read(QDataStream & in);
impl<'a> /*trait*/ QStandardItem_read<()> for (&'a QDataStream) {
  fn read(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem4readER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStandardItem4readER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStandardItem::setCheckable(bool checkable);
impl /*struct*/ QStandardItem {
  pub fn setCheckable<RetType, T: QStandardItem_setCheckable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCheckable(self);
    // return 1;
  }
}

pub trait QStandardItem_setCheckable<RetType> {
  fn setCheckable(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setCheckable(bool checkable);
impl<'a> /*trait*/ QStandardItem_setCheckable<()> for (i8) {
  fn setCheckable(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem12setCheckableEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QStandardItem12setCheckableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStandardItem::setDragEnabled(bool dragEnabled);
impl /*struct*/ QStandardItem {
  pub fn setDragEnabled<RetType, T: QStandardItem_setDragEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDragEnabled(self);
    // return 1;
  }
}

pub trait QStandardItem_setDragEnabled<RetType> {
  fn setDragEnabled(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setDragEnabled(bool dragEnabled);
impl<'a> /*trait*/ QStandardItem_setDragEnabled<()> for (i8) {
  fn setDragEnabled(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem14setDragEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QStandardItem14setDragEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QBrush QStandardItem::foreground();
impl /*struct*/ QStandardItem {
  pub fn foreground<RetType, T: QStandardItem_foreground<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.foreground(self);
    // return 1;
  }
}

pub trait QStandardItem_foreground<RetType> {
  fn foreground(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QBrush QStandardItem::foreground();
impl<'a> /*trait*/ QStandardItem_foreground<QBrush> for () {
  fn foreground(self , rsthis: & QStandardItem) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem10foregroundEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem10foregroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStandardItem * QStandardItem::clone();
impl /*struct*/ QStandardItem {
  pub fn clone<RetType, T: QStandardItem_clone<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clone(self);
    // return 1;
  }
}

pub trait QStandardItem_clone<RetType> {
  fn clone(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QStandardItem * QStandardItem::clone();
impl<'a> /*trait*/ QStandardItem_clone<QStandardItem> for () {
  fn clone(self , rsthis: & QStandardItem) -> QStandardItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem5cloneEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem5cloneEv(rsthis.qclsinst)};
    let mut ret1 = QStandardItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStandardItem::removeRows(int row, int count);
impl /*struct*/ QStandardItem {
  pub fn removeRows<RetType, T: QStandardItem_removeRows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeRows(self);
    // return 1;
  }
}

pub trait QStandardItem_removeRows<RetType> {
  fn removeRows(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::removeRows(int row, int count);
impl<'a> /*trait*/ QStandardItem_removeRows<()> for (i32, i32) {
  fn removeRows(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10removeRowsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN13QStandardItem10removeRowsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QSize QStandardItem::sizeHint();
impl /*struct*/ QStandardItem {
  pub fn sizeHint<RetType, T: QStandardItem_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QStandardItem_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QSize QStandardItem::sizeHint();
impl<'a> /*trait*/ QStandardItem_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QStandardItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem8sizeHintEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStandardItem::setEnabled(bool enabled);
impl /*struct*/ QStandardItem {
  pub fn setEnabled<RetType, T: QStandardItem_setEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEnabled(self);
    // return 1;
  }
}

pub trait QStandardItem_setEnabled<RetType> {
  fn setEnabled(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setEnabled(bool enabled);
impl<'a> /*trait*/ QStandardItem_setEnabled<()> for (i8) {
  fn setEnabled(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem10setEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QStandardItem10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStandardItem::setBackground(const QBrush & brush);
impl /*struct*/ QStandardItem {
  pub fn setBackground<RetType, T: QStandardItem_setBackground<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBackground(self);
    // return 1;
  }
}

pub trait QStandardItem_setBackground<RetType> {
  fn setBackground(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setBackground(const QBrush & brush);
impl<'a> /*trait*/ QStandardItem_setBackground<()> for (&'a QBrush) {
  fn setBackground(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStandardItem13setBackgroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStandardItem::setAccessibleDescription(const QString & accessibleDescription);
impl /*struct*/ QStandardItem {
  pub fn setAccessibleDescription<RetType, T: QStandardItem_setAccessibleDescription<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAccessibleDescription(self);
    // return 1;
  }
}

pub trait QStandardItem_setAccessibleDescription<RetType> {
  fn setAccessibleDescription(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setAccessibleDescription(const QString & accessibleDescription);
impl<'a> /*trait*/ QStandardItem_setAccessibleDescription<()> for (&'a QString) {
  fn setAccessibleDescription(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem24setAccessibleDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStandardItem24setAccessibleDescriptionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStandardItem::setSizeHint(const QSize & sizeHint);
impl /*struct*/ QStandardItem {
  pub fn setSizeHint<RetType, T: QStandardItem_setSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSizeHint(self);
    // return 1;
  }
}

pub trait QStandardItem_setSizeHint<RetType> {
  fn setSizeHint(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setSizeHint(const QSize & sizeHint);
impl<'a> /*trait*/ QStandardItem_setSizeHint<()> for (&'a QSize) {
  fn setSizeHint(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem11setSizeHintERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QStandardItem11setSizeHintERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QStandardItem::accessibleDescription();
impl /*struct*/ QStandardItem {
  pub fn accessibleDescription<RetType, T: QStandardItem_accessibleDescription<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.accessibleDescription(self);
    // return 1;
  }
}

pub trait QStandardItem_accessibleDescription<RetType> {
  fn accessibleDescription(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  QString QStandardItem::accessibleDescription();
impl<'a> /*trait*/ QStandardItem_accessibleDescription<QString> for () {
  fn accessibleDescription(self , rsthis: & QStandardItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStandardItem21accessibleDescriptionEv()};
    let mut ret = unsafe {C_ZNK13QStandardItem21accessibleDescriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStandardItem::setTristate(bool tristate);
impl /*struct*/ QStandardItem {
  pub fn setTristate<RetType, T: QStandardItem_setTristate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTristate(self);
    // return 1;
  }
}

pub trait QStandardItem_setTristate<RetType> {
  fn setTristate(self , rsthis: & QStandardItem) -> RetType;
}

  // proto:  void QStandardItem::setTristate(bool tristate);
impl<'a> /*trait*/ QStandardItem_setTristate<()> for (i8) {
  fn setTristate(self , rsthis: & QStandardItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStandardItem11setTristateEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QStandardItem11setTristateEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QStandardItemModel_itemChanged
pub struct QStandardItemModel_itemChanged_signal{poi:u64}
impl /* struct */ QStandardItemModel {
  pub fn itemChanged(&self) -> QStandardItemModel_itemChanged_signal {
     return QStandardItemModel_itemChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QStandardItemModel_itemChanged_signal {
  pub fn connect<T: QStandardItemModel_itemChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QStandardItemModel_itemChanged_signal_connect {
  fn connect(self, sigthis: QStandardItemModel_itemChanged_signal);
}

// itemChanged(class QStandardItem *)
extern fn QStandardItemModel_itemChanged_signal_connect_cb_0(rsfptr:fn(QStandardItem), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QStandardItem::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QStandardItemModel_itemChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QStandardItem)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QStandardItem::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QStandardItemModel_itemChanged_signal_connect for fn(QStandardItem) {
  fn connect(self, sigthis: QStandardItemModel_itemChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStandardItemModel_itemChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QStandardItemModel_SlotProxy_connect__ZN18QStandardItemModel11itemChangedEP13QStandardItem(arg0, arg1, arg2)};
  }
}
impl /* trait */ QStandardItemModel_itemChanged_signal_connect for Box<Fn(QStandardItem)> {
  fn connect(self, sigthis: QStandardItemModel_itemChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStandardItemModel_itemChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QStandardItemModel_SlotProxy_connect__ZN18QStandardItemModel11itemChangedEP13QStandardItem(arg0, arg1, arg2)};
  }
}
// <= body block end

