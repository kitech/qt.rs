// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtWidgets/qtablewidget.h
// dst-file: /src/widgets/qtablewidget.rs
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
use super::qtableview::*; // 773
// use super::qlist::*; // 775
use super::super::core::qstring::*; // 771
use super::super::core::qobjectdefs::*; // 771
// use super::qtablewidget::QTableWidgetItem; // 773
use super::super::core::qstringlist::*; // 771
use super::super::core::qrect::*; // 771
use super::qwidget::*; // 773
// use super::qtablewidget::QTableWidgetSelectionRange; // 773
use super::super::core::qpoint::*; // 771
use super::super::gui::qcolor::*; // 771
use super::super::core::qvariant::*; // 771
use super::super::core::qsize::*; // 771
use super::super::gui::qbrush::*; // 771
use super::super::gui::qfont::*; // 771
use super::super::gui::qicon::*; // 771
use super::super::core::qdatastream::*; // 771
// use super::qtablewidget::QTableWidget; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTableWidgetSelectionRange_Class_Size() -> c_int;
  // proto:  void QTableWidgetSelectionRange::QTableWidgetSelectionRange(int top, int left, int bottom, int right);
  fn C_ZN26QTableWidgetSelectionRangeC2Eiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> u64;
  // proto:  int QTableWidgetSelectionRange::columnCount();
  fn C_ZNK26QTableWidgetSelectionRange11columnCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QTableWidgetSelectionRange::rowCount();
  fn C_ZNK26QTableWidgetSelectionRange8rowCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QTableWidgetSelectionRange::leftColumn();
  fn C_ZNK26QTableWidgetSelectionRange10leftColumnEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTableWidgetSelectionRange::~QTableWidgetSelectionRange();
  fn C_ZN26QTableWidgetSelectionRangeD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QTableWidgetSelectionRange::topRow();
  fn C_ZNK26QTableWidgetSelectionRange6topRowEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QTableWidgetSelectionRange::rightColumn();
  fn C_ZNK26QTableWidgetSelectionRange11rightColumnEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTableWidgetSelectionRange::QTableWidgetSelectionRange();
  fn C_ZN26QTableWidgetSelectionRangeC2Ev() -> u64;
  // proto:  void QTableWidgetSelectionRange::QTableWidgetSelectionRange(const QTableWidgetSelectionRange & other);
  fn C_ZN26QTableWidgetSelectionRangeC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  int QTableWidgetSelectionRange::bottomRow();
  fn C_ZNK26QTableWidgetSelectionRange9bottomRowEv(qthis: u64 /* *mut c_void*/) -> c_int;
  fn QTableWidget_Class_Size() -> c_int;
  // proto:  void QTableWidget::setColumnCount(int columns);
  fn C_ZN12QTableWidget14setColumnCountEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTableWidget::~QTableWidget();
  fn C_ZN12QTableWidgetD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QList<QTableWidgetItem *> QTableWidget::selectedItems();
  fn C_ZNK12QTableWidget13selectedItemsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QTableWidget::isSortingEnabled();
  fn C_ZNK12QTableWidget16isSortingEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QMetaObject * QTableWidget::metaObject();
  fn C_ZNK12QTableWidget10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTableWidget::closePersistentEditor(QTableWidgetItem * item);
  fn C_ZN12QTableWidget21closePersistentEditorEP16QTableWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTableWidget::setHorizontalHeaderLabels(const QStringList & labels);
  fn C_ZN12QTableWidget25setHorizontalHeaderLabelsERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTableWidget::setItemSelected(const QTableWidgetItem * item, bool select);
  fn C_ZN12QTableWidget15setItemSelectedEPK16QTableWidgetItemb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_char);
  // proto:  QTableWidgetItem * QTableWidget::takeItem(int row, int column);
  fn C_ZN12QTableWidget8takeItemEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QTableWidget::removeCellWidget(int row, int column);
  fn C_ZN12QTableWidget16removeCellWidgetEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QTableWidget::setVerticalHeaderItem(int row, QTableWidgetItem * item);
  fn C_ZN12QTableWidget21setVerticalHeaderItemEiP16QTableWidgetItem(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  QRect QTableWidget::visualItemRect(const QTableWidgetItem * item);
  fn C_ZNK12QTableWidget14visualItemRectEPK16QTableWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QTableWidgetItem * QTableWidget::currentItem();
  fn C_ZNK12QTableWidget11currentItemEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTableWidget::row(const QTableWidgetItem * item);
  fn C_ZNK12QTableWidget3rowEPK16QTableWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  void QTableWidget::removeRow(int row);
  fn C_ZN12QTableWidget9removeRowEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTableWidget::setItemPrototype(const QTableWidgetItem * item);
  fn C_ZN12QTableWidget16setItemPrototypeEPK16QTableWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTableWidget::QTableWidget(int rows, int columns, QWidget * parent);
  fn C_ZN12QTableWidgetC2EiiP7QWidget(arg0: c_int, arg1: c_int, arg2: *mut c_void) -> u64;
  // proto:  int QTableWidget::visualRow(int logicalRow);
  fn C_ZNK12QTableWidget9visualRowEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  void QTableWidget::setCellWidget(int row, int column, QWidget * widget);
  fn C_ZN12QTableWidget13setCellWidgetEiiP7QWidget(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void);
  // proto:  void QTableWidget::openPersistentEditor(QTableWidgetItem * item);
  fn C_ZN12QTableWidget20openPersistentEditorEP16QTableWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QTableWidget::columnCount();
  fn C_ZNK12QTableWidget11columnCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QTableWidget::currentRow();
  fn C_ZNK12QTableWidget10currentRowEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTableWidget::setCurrentItem(QTableWidgetItem * item);
  fn C_ZN12QTableWidget14setCurrentItemEP16QTableWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWidget * QTableWidget::cellWidget(int row, int column);
  fn C_ZNK12QTableWidget10cellWidgetEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QTableWidget::setSortingEnabled(bool enable);
  fn C_ZN12QTableWidget17setSortingEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTableWidget::setItem(int row, int column, QTableWidgetItem * item);
  fn C_ZN12QTableWidget7setItemEiiP16QTableWidgetItem(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void);
  // proto:  QTableWidgetItem * QTableWidget::horizontalHeaderItem(int column);
  fn C_ZNK12QTableWidget20horizontalHeaderItemEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QTableWidget::editItem(QTableWidgetItem * item);
  fn C_ZN12QTableWidget8editItemEP16QTableWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QList<QTableWidgetSelectionRange> QTableWidget::selectedRanges();
  fn C_ZNK12QTableWidget14selectedRangesEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTableWidget::currentColumn();
  fn C_ZNK12QTableWidget13currentColumnEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTableWidget::removeColumn(int column);
  fn C_ZN12QTableWidget12removeColumnEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTableWidget::setRangeSelected(const QTableWidgetSelectionRange & range, bool select);
  fn C_ZN12QTableWidget16setRangeSelectedERK26QTableWidgetSelectionRangeb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_char);
  // proto:  int QTableWidget::column(const QTableWidgetItem * item);
  fn C_ZNK12QTableWidget6columnEPK16QTableWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  bool QTableWidget::isItemSelected(const QTableWidgetItem * item);
  fn C_ZNK12QTableWidget14isItemSelectedEPK16QTableWidgetItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QTableWidgetItem * QTableWidget::takeVerticalHeaderItem(int row);
  fn C_ZN12QTableWidget22takeVerticalHeaderItemEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QTableWidget::insertRow(int row);
  fn C_ZN12QTableWidget9insertRowEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QTableWidget::rowCount();
  fn C_ZNK12QTableWidget8rowCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QTableWidgetItem * QTableWidget::item(int row, int column);
  fn C_ZNK12QTableWidget4itemEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QTableWidget::QTableWidget(QWidget * parent);
  fn C_ZN12QTableWidgetC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  void QTableWidget::setVerticalHeaderLabels(const QStringList & labels);
  fn C_ZN12QTableWidget23setVerticalHeaderLabelsERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QTableWidgetItem * QTableWidget::itemPrototype();
  fn C_ZNK12QTableWidget13itemPrototypeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTableWidgetItem * QTableWidget::itemAt(const QPoint & p);
  fn C_ZNK12QTableWidget6itemAtERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTableWidget::clearContents();
  fn C_ZN12QTableWidget13clearContentsEv(qthis: u64 /* *mut c_void*/);
  // proto:  QTableWidgetItem * QTableWidget::itemAt(int x, int y);
  fn C_ZNK12QTableWidget6itemAtEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QTableWidget::setCurrentCell(int row, int column);
  fn C_ZN12QTableWidget14setCurrentCellEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QTableWidget::setRowCount(int rows);
  fn C_ZN12QTableWidget11setRowCountEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTableWidget::setHorizontalHeaderItem(int column, QTableWidgetItem * item);
  fn C_ZN12QTableWidget23setHorizontalHeaderItemEiP16QTableWidgetItem(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  int QTableWidget::visualColumn(int logicalColumn);
  fn C_ZNK12QTableWidget12visualColumnEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  QTableWidgetItem * QTableWidget::takeHorizontalHeaderItem(int column);
  fn C_ZN12QTableWidget24takeHorizontalHeaderItemEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QTableWidgetItem * QTableWidget::verticalHeaderItem(int row);
  fn C_ZNK12QTableWidget18verticalHeaderItemEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QTableWidget::clear();
  fn C_ZN12QTableWidget5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTableWidget::insertColumn(int column);
  fn C_ZN12QTableWidget12insertColumnEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  fn QTableWidgetItem_Class_Size() -> c_int;
  // proto:  QColor QTableWidgetItem::backgroundColor();
  fn C_ZNK16QTableWidgetItem15backgroundColorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QVariant QTableWidgetItem::data(int role);
  fn C_ZNK16QTableWidgetItem4dataEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QTableWidgetItem::setSelected(bool select);
  fn C_ZN16QTableWidgetItem11setSelectedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTableWidgetItem::setStatusTip(const QString & statusTip);
  fn C_ZN16QTableWidgetItem12setStatusTipERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QColor QTableWidgetItem::textColor();
  fn C_ZNK16QTableWidgetItem9textColorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTableWidgetItem::~QTableWidgetItem();
  fn C_ZN16QTableWidgetItemD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QString QTableWidgetItem::text();
  fn C_ZNK16QTableWidgetItem4textEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTableWidgetItem::setSizeHint(const QSize & size);
  fn C_ZN16QTableWidgetItem11setSizeHintERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QBrush QTableWidgetItem::foreground();
  fn C_ZNK16QTableWidgetItem10foregroundEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTableWidgetItem::type();
  fn C_ZNK16QTableWidgetItem4typeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QTableWidgetItem::column();
  fn C_ZNK16QTableWidgetItem6columnEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTableWidgetItem::setTextAlignment(int alignment);
  fn C_ZN16QTableWidgetItem16setTextAlignmentEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QFont QTableWidgetItem::font();
  fn C_ZNK16QTableWidgetItem4fontEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QIcon QTableWidgetItem::icon();
  fn C_ZNK16QTableWidgetItem4iconEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTableWidgetItem::write(QDataStream & out);
  fn C_ZNK16QTableWidgetItem5writeER11QDataStream(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTableWidgetItem::QTableWidgetItem(const QTableWidgetItem & other);
  fn C_ZN16QTableWidgetItemC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  QBrush QTableWidgetItem::background();
  fn C_ZNK16QTableWidgetItem10backgroundEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTableWidgetItem::setIcon(const QIcon & icon);
  fn C_ZN16QTableWidgetItem7setIconERK5QIcon(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTableWidgetItem::QTableWidgetItem(const QString & text, int type);
  fn C_ZN16QTableWidgetItemC2ERK7QStringi(arg0: *mut c_void, arg1: c_int) -> u64;
  // proto:  QString QTableWidgetItem::statusTip();
  fn C_ZNK16QTableWidgetItem9statusTipEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTableWidgetItem * QTableWidgetItem::clone();
  fn C_ZNK16QTableWidgetItem5cloneEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTableWidgetItem::QTableWidgetItem(int type);
  fn C_ZN16QTableWidgetItemC2Ei(arg0: c_int) -> u64;
  // proto:  void QTableWidgetItem::setWhatsThis(const QString & whatsThis);
  fn C_ZN16QTableWidgetItem12setWhatsThisERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSize QTableWidgetItem::sizeHint();
  fn C_ZNK16QTableWidgetItem8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTableWidgetItem::setForeground(const QBrush & brush);
  fn C_ZN16QTableWidgetItem13setForegroundERK6QBrush(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QTableWidgetItem::row();
  fn C_ZNK16QTableWidgetItem3rowEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTableWidgetItem::setData(int role, const QVariant & value);
  fn C_ZN16QTableWidgetItem7setDataEiRK8QVariant(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  QTableWidget * QTableWidgetItem::tableWidget();
  fn C_ZNK16QTableWidgetItem11tableWidgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTableWidgetItem::QTableWidgetItem(const QIcon & icon, const QString & text, int type);
  fn C_ZN16QTableWidgetItemC2ERK5QIconRK7QStringi(arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> u64;
  // proto:  int QTableWidgetItem::textAlignment();
  fn C_ZNK16QTableWidgetItem13textAlignmentEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTableWidgetItem::read(QDataStream & in);
  fn C_ZN16QTableWidgetItem4readER11QDataStream(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QTableWidgetItem::toolTip();
  fn C_ZNK16QTableWidgetItem7toolTipEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QTableWidgetItem::isSelected();
  fn C_ZNK16QTableWidgetItem10isSelectedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTableWidgetItem::setBackgroundColor(const QColor & color);
  fn C_ZN16QTableWidgetItem18setBackgroundColorERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTableWidgetItem::setBackground(const QBrush & brush);
  fn C_ZN16QTableWidgetItem13setBackgroundERK6QBrush(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTableWidgetItem::setFont(const QFont & font);
  fn C_ZN16QTableWidgetItem7setFontERK5QFont(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTableWidgetItem::setTextColor(const QColor & color);
  fn C_ZN16QTableWidgetItem12setTextColorERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTableWidgetItem::setText(const QString & text);
  fn C_ZN16QTableWidgetItem7setTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QTableWidgetItem::whatsThis();
  fn C_ZNK16QTableWidgetItem9whatsThisEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTableWidgetItem::setToolTip(const QString & toolTip);
  fn C_ZN16QTableWidgetItem10setToolTipERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QTableWidget_SlotProxy_connect__ZN12QTableWidget13itemActivatedEP16QTableWidgetItem(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTableWidget_SlotProxy_connect__ZN12QTableWidget20itemSelectionChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTableWidget_SlotProxy_connect__ZN12QTableWidget11itemChangedEP16QTableWidgetItem(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTableWidget_SlotProxy_connect__ZN12QTableWidget11itemPressedEP16QTableWidgetItem(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTableWidget_SlotProxy_connect__ZN12QTableWidget18currentItemChangedEP16QTableWidgetItemS1_(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTableWidget_SlotProxy_connect__ZN12QTableWidget18currentCellChangedEiiii(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTableWidget_SlotProxy_connect__ZN12QTableWidget11itemEnteredEP16QTableWidgetItem(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTableWidget_SlotProxy_connect__ZN12QTableWidget11cellEnteredEii(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTableWidget_SlotProxy_connect__ZN12QTableWidget13cellActivatedEii(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTableWidget_SlotProxy_connect__ZN12QTableWidget11itemClickedEP16QTableWidgetItem(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTableWidget_SlotProxy_connect__ZN12QTableWidget11cellClickedEii(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTableWidget_SlotProxy_connect__ZN12QTableWidget17itemDoubleClickedEP16QTableWidgetItem(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTableWidget_SlotProxy_connect__ZN12QTableWidget17cellDoubleClickedEii(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTableWidget_SlotProxy_connect__ZN12QTableWidget11cellChangedEii(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTableWidget_SlotProxy_connect__ZN12QTableWidget11cellPressedEii(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QTableWidgetSelectionRange)=16
#[derive(Default)]
pub struct QTableWidgetSelectionRange {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QTableWidget)=1
#[derive(Default)]
pub struct QTableWidget {
  qbase: QTableView,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _itemDoubleClicked: QTableWidget_itemDoubleClicked_signal,
  pub _cellEntered: QTableWidget_cellEntered_signal,
  pub _itemClicked: QTableWidget_itemClicked_signal,
  pub _currentItemChanged: QTableWidget_currentItemChanged_signal,
  pub _itemEntered: QTableWidget_itemEntered_signal,
  pub _itemPressed: QTableWidget_itemPressed_signal,
  pub _cellClicked: QTableWidget_cellClicked_signal,
  pub _itemSelectionChanged: QTableWidget_itemSelectionChanged_signal,
  pub _cellChanged: QTableWidget_cellChanged_signal,
  pub _itemActivated: QTableWidget_itemActivated_signal,
  pub _cellActivated: QTableWidget_cellActivated_signal,
  pub _itemChanged: QTableWidget_itemChanged_signal,
  pub _currentCellChanged: QTableWidget_currentCellChanged_signal,
  pub _cellDoubleClicked: QTableWidget_cellDoubleClicked_signal,
  pub _cellPressed: QTableWidget_cellPressed_signal,
}

// class sizeof(QTableWidgetItem)=1
#[derive(Default)]
pub struct QTableWidgetItem {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTableWidgetSelectionRange {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTableWidgetSelectionRange {
    return QTableWidgetSelectionRange{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QTableWidgetSelectionRange::QTableWidgetSelectionRange(int top, int left, int bottom, int right);
impl /*struct*/ QTableWidgetSelectionRange {
  pub fn new<T: QTableWidgetSelectionRange_new>(value: T) -> QTableWidgetSelectionRange {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTableWidgetSelectionRange_new {
  fn new(self) -> QTableWidgetSelectionRange;
}

  // proto:  void QTableWidgetSelectionRange::QTableWidgetSelectionRange(int top, int left, int bottom, int right);
impl<'a> /*trait*/ QTableWidgetSelectionRange_new for (i32, i32, i32, i32) {
  fn new(self) -> QTableWidgetSelectionRange {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QTableWidgetSelectionRangeC2Eiiii()};
    let ctysz: c_int = unsafe{QTableWidgetSelectionRange_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let qthis: u64 = unsafe {C_ZN26QTableWidgetSelectionRangeC2Eiiii(arg0, arg1, arg2, arg3)};
    let rsthis = QTableWidgetSelectionRange{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QTableWidgetSelectionRange::columnCount();
impl /*struct*/ QTableWidgetSelectionRange {
  pub fn columnCount<RetType, T: QTableWidgetSelectionRange_columnCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QTableWidgetSelectionRange_columnCount<RetType> {
  fn columnCount(self , rsthis: & QTableWidgetSelectionRange) -> RetType;
}

  // proto:  int QTableWidgetSelectionRange::columnCount();
impl<'a> /*trait*/ QTableWidgetSelectionRange_columnCount<i32> for () {
  fn columnCount(self , rsthis: & QTableWidgetSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QTableWidgetSelectionRange11columnCountEv()};
    let mut ret = unsafe {C_ZNK26QTableWidgetSelectionRange11columnCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QTableWidgetSelectionRange::rowCount();
impl /*struct*/ QTableWidgetSelectionRange {
  pub fn rowCount<RetType, T: QTableWidgetSelectionRange_rowCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QTableWidgetSelectionRange_rowCount<RetType> {
  fn rowCount(self , rsthis: & QTableWidgetSelectionRange) -> RetType;
}

  // proto:  int QTableWidgetSelectionRange::rowCount();
impl<'a> /*trait*/ QTableWidgetSelectionRange_rowCount<i32> for () {
  fn rowCount(self , rsthis: & QTableWidgetSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QTableWidgetSelectionRange8rowCountEv()};
    let mut ret = unsafe {C_ZNK26QTableWidgetSelectionRange8rowCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QTableWidgetSelectionRange::leftColumn();
impl /*struct*/ QTableWidgetSelectionRange {
  pub fn leftColumn<RetType, T: QTableWidgetSelectionRange_leftColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.leftColumn(self);
    // return 1;
  }
}

pub trait QTableWidgetSelectionRange_leftColumn<RetType> {
  fn leftColumn(self , rsthis: & QTableWidgetSelectionRange) -> RetType;
}

  // proto:  int QTableWidgetSelectionRange::leftColumn();
impl<'a> /*trait*/ QTableWidgetSelectionRange_leftColumn<i32> for () {
  fn leftColumn(self , rsthis: & QTableWidgetSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QTableWidgetSelectionRange10leftColumnEv()};
    let mut ret = unsafe {C_ZNK26QTableWidgetSelectionRange10leftColumnEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTableWidgetSelectionRange::~QTableWidgetSelectionRange();
impl /*struct*/ QTableWidgetSelectionRange {
  pub fn free<RetType, T: QTableWidgetSelectionRange_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTableWidgetSelectionRange_free<RetType> {
  fn free(self , rsthis: & QTableWidgetSelectionRange) -> RetType;
}

  // proto:  void QTableWidgetSelectionRange::~QTableWidgetSelectionRange();
impl<'a> /*trait*/ QTableWidgetSelectionRange_free<()> for () {
  fn free(self , rsthis: & QTableWidgetSelectionRange) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QTableWidgetSelectionRangeD2Ev()};
     unsafe {C_ZN26QTableWidgetSelectionRangeD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTableWidgetSelectionRange::topRow();
impl /*struct*/ QTableWidgetSelectionRange {
  pub fn topRow<RetType, T: QTableWidgetSelectionRange_topRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.topRow(self);
    // return 1;
  }
}

pub trait QTableWidgetSelectionRange_topRow<RetType> {
  fn topRow(self , rsthis: & QTableWidgetSelectionRange) -> RetType;
}

  // proto:  int QTableWidgetSelectionRange::topRow();
impl<'a> /*trait*/ QTableWidgetSelectionRange_topRow<i32> for () {
  fn topRow(self , rsthis: & QTableWidgetSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QTableWidgetSelectionRange6topRowEv()};
    let mut ret = unsafe {C_ZNK26QTableWidgetSelectionRange6topRowEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QTableWidgetSelectionRange::rightColumn();
impl /*struct*/ QTableWidgetSelectionRange {
  pub fn rightColumn<RetType, T: QTableWidgetSelectionRange_rightColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rightColumn(self);
    // return 1;
  }
}

pub trait QTableWidgetSelectionRange_rightColumn<RetType> {
  fn rightColumn(self , rsthis: & QTableWidgetSelectionRange) -> RetType;
}

  // proto:  int QTableWidgetSelectionRange::rightColumn();
impl<'a> /*trait*/ QTableWidgetSelectionRange_rightColumn<i32> for () {
  fn rightColumn(self , rsthis: & QTableWidgetSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QTableWidgetSelectionRange11rightColumnEv()};
    let mut ret = unsafe {C_ZNK26QTableWidgetSelectionRange11rightColumnEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTableWidgetSelectionRange::QTableWidgetSelectionRange();
impl<'a> /*trait*/ QTableWidgetSelectionRange_new for () {
  fn new(self) -> QTableWidgetSelectionRange {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QTableWidgetSelectionRangeC2Ev()};
    let ctysz: c_int = unsafe{QTableWidgetSelectionRange_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN26QTableWidgetSelectionRangeC2Ev()};
    let rsthis = QTableWidgetSelectionRange{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTableWidgetSelectionRange::QTableWidgetSelectionRange(const QTableWidgetSelectionRange & other);
impl<'a> /*trait*/ QTableWidgetSelectionRange_new for (&'a QTableWidgetSelectionRange) {
  fn new(self) -> QTableWidgetSelectionRange {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QTableWidgetSelectionRangeC2ERKS_()};
    let ctysz: c_int = unsafe{QTableWidgetSelectionRange_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN26QTableWidgetSelectionRangeC2ERKS_(arg0)};
    let rsthis = QTableWidgetSelectionRange{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QTableWidgetSelectionRange::bottomRow();
impl /*struct*/ QTableWidgetSelectionRange {
  pub fn bottomRow<RetType, T: QTableWidgetSelectionRange_bottomRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bottomRow(self);
    // return 1;
  }
}

pub trait QTableWidgetSelectionRange_bottomRow<RetType> {
  fn bottomRow(self , rsthis: & QTableWidgetSelectionRange) -> RetType;
}

  // proto:  int QTableWidgetSelectionRange::bottomRow();
impl<'a> /*trait*/ QTableWidgetSelectionRange_bottomRow<i32> for () {
  fn bottomRow(self , rsthis: & QTableWidgetSelectionRange) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QTableWidgetSelectionRange9bottomRowEv()};
    let mut ret = unsafe {C_ZNK26QTableWidgetSelectionRange9bottomRowEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTableWidget {
    return QTableWidget{qbase: QTableView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTableWidget {
  type Target = QTableView;

  fn deref(&self) -> &QTableView {
    return & self.qbase;
  }
}
impl AsRef<QTableView> for QTableWidget {
  fn as_ref(& self) -> & QTableView {
    return & self.qbase;
  }
}
  // proto:  void QTableWidget::setColumnCount(int columns);
impl /*struct*/ QTableWidget {
  pub fn setColumnCount<RetType, T: QTableWidget_setColumnCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColumnCount(self);
    // return 1;
  }
}

pub trait QTableWidget_setColumnCount<RetType> {
  fn setColumnCount(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::setColumnCount(int columns);
impl<'a> /*trait*/ QTableWidget_setColumnCount<()> for (i32) {
  fn setColumnCount(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget14setColumnCountEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QTableWidget14setColumnCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableWidget::~QTableWidget();
impl /*struct*/ QTableWidget {
  pub fn free<RetType, T: QTableWidget_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTableWidget_free<RetType> {
  fn free(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::~QTableWidget();
impl<'a> /*trait*/ QTableWidget_free<()> for () {
  fn free(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidgetD2Ev()};
     unsafe {C_ZN12QTableWidgetD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QTableWidgetItem *> QTableWidget::selectedItems();
impl /*struct*/ QTableWidget {
  pub fn selectedItems<RetType, T: QTableWidget_selectedItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectedItems(self);
    // return 1;
  }
}

pub trait QTableWidget_selectedItems<RetType> {
  fn selectedItems(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  QList<QTableWidgetItem *> QTableWidget::selectedItems();
impl<'a> /*trait*/ QTableWidget_selectedItems<u64> for () {
  fn selectedItems(self , rsthis: & QTableWidget) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget13selectedItemsEv()};
    let mut ret = unsafe {C_ZNK12QTableWidget13selectedItemsEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  bool QTableWidget::isSortingEnabled();
impl /*struct*/ QTableWidget {
  pub fn isSortingEnabled<RetType, T: QTableWidget_isSortingEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSortingEnabled(self);
    // return 1;
  }
}

pub trait QTableWidget_isSortingEnabled<RetType> {
  fn isSortingEnabled(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  bool QTableWidget::isSortingEnabled();
impl<'a> /*trait*/ QTableWidget_isSortingEnabled<i8> for () {
  fn isSortingEnabled(self , rsthis: & QTableWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget16isSortingEnabledEv()};
    let mut ret = unsafe {C_ZNK12QTableWidget16isSortingEnabledEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  const QMetaObject * QTableWidget::metaObject();
impl /*struct*/ QTableWidget {
  pub fn metaObject<RetType, T: QTableWidget_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTableWidget_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  const QMetaObject * QTableWidget::metaObject();
impl<'a> /*trait*/ QTableWidget_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QTableWidget) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget10metaObjectEv()};
    let mut ret = unsafe {C_ZNK12QTableWidget10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTableWidget::closePersistentEditor(QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn closePersistentEditor<RetType, T: QTableWidget_closePersistentEditor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.closePersistentEditor(self);
    // return 1;
  }
}

pub trait QTableWidget_closePersistentEditor<RetType> {
  fn closePersistentEditor(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::closePersistentEditor(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_closePersistentEditor<()> for (&'a QTableWidgetItem) {
  fn closePersistentEditor(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget21closePersistentEditorEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QTableWidget21closePersistentEditorEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableWidget::setHorizontalHeaderLabels(const QStringList & labels);
impl /*struct*/ QTableWidget {
  pub fn setHorizontalHeaderLabels<RetType, T: QTableWidget_setHorizontalHeaderLabels<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalHeaderLabels(self);
    // return 1;
  }
}

pub trait QTableWidget_setHorizontalHeaderLabels<RetType> {
  fn setHorizontalHeaderLabels(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::setHorizontalHeaderLabels(const QStringList & labels);
impl<'a> /*trait*/ QTableWidget_setHorizontalHeaderLabels<()> for (&'a QStringList) {
  fn setHorizontalHeaderLabels(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget25setHorizontalHeaderLabelsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QTableWidget25setHorizontalHeaderLabelsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableWidget::setItemSelected(const QTableWidgetItem * item, bool select);
impl /*struct*/ QTableWidget {
  pub fn setItemSelected<RetType, T: QTableWidget_setItemSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemSelected(self);
    // return 1;
  }
}

pub trait QTableWidget_setItemSelected<RetType> {
  fn setItemSelected(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::setItemSelected(const QTableWidgetItem * item, bool select);
impl<'a> /*trait*/ QTableWidget_setItemSelected<()> for (&'a QTableWidgetItem, i8) {
  fn setItemSelected(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget15setItemSelectedEPK16QTableWidgetItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {C_ZN12QTableWidget15setItemSelectedEPK16QTableWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QTableWidgetItem * QTableWidget::takeItem(int row, int column);
impl /*struct*/ QTableWidget {
  pub fn takeItem<RetType, T: QTableWidget_takeItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeItem(self);
    // return 1;
  }
}

pub trait QTableWidget_takeItem<RetType> {
  fn takeItem(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  QTableWidgetItem * QTableWidget::takeItem(int row, int column);
impl<'a> /*trait*/ QTableWidget_takeItem<QTableWidgetItem> for (i32, i32) {
  fn takeItem(self , rsthis: & QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget8takeItemEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZN12QTableWidget8takeItemEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTableWidgetItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTableWidget::removeCellWidget(int row, int column);
impl /*struct*/ QTableWidget {
  pub fn removeCellWidget<RetType, T: QTableWidget_removeCellWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeCellWidget(self);
    // return 1;
  }
}

pub trait QTableWidget_removeCellWidget<RetType> {
  fn removeCellWidget(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::removeCellWidget(int row, int column);
impl<'a> /*trait*/ QTableWidget_removeCellWidget<()> for (i32, i32) {
  fn removeCellWidget(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget16removeCellWidgetEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN12QTableWidget16removeCellWidgetEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTableWidget::setVerticalHeaderItem(int row, QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn setVerticalHeaderItem<RetType, T: QTableWidget_setVerticalHeaderItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVerticalHeaderItem(self);
    // return 1;
  }
}

pub trait QTableWidget_setVerticalHeaderItem<RetType> {
  fn setVerticalHeaderItem(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::setVerticalHeaderItem(int row, QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_setVerticalHeaderItem<()> for (i32, &'a QTableWidgetItem) {
  fn setVerticalHeaderItem(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget21setVerticalHeaderItemEiP16QTableWidgetItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN12QTableWidget21setVerticalHeaderItemEiP16QTableWidgetItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QRect QTableWidget::visualItemRect(const QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn visualItemRect<RetType, T: QTableWidget_visualItemRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.visualItemRect(self);
    // return 1;
  }
}

pub trait QTableWidget_visualItemRect<RetType> {
  fn visualItemRect(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  QRect QTableWidget::visualItemRect(const QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_visualItemRect<QRect> for (&'a QTableWidgetItem) {
  fn visualItemRect(self , rsthis: & QTableWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget14visualItemRectEPK16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK12QTableWidget14visualItemRectEPK16QTableWidgetItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTableWidgetItem * QTableWidget::currentItem();
impl /*struct*/ QTableWidget {
  pub fn currentItem<RetType, T: QTableWidget_currentItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentItem(self);
    // return 1;
  }
}

pub trait QTableWidget_currentItem<RetType> {
  fn currentItem(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  QTableWidgetItem * QTableWidget::currentItem();
impl<'a> /*trait*/ QTableWidget_currentItem<QTableWidgetItem> for () {
  fn currentItem(self , rsthis: & QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget11currentItemEv()};
    let mut ret = unsafe {C_ZNK12QTableWidget11currentItemEv(rsthis.qclsinst)};
    let mut ret1 = QTableWidgetItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTableWidget::row(const QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn row<RetType, T: QTableWidget_row<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.row(self);
    // return 1;
  }
}

pub trait QTableWidget_row<RetType> {
  fn row(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  int QTableWidget::row(const QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_row<i32> for (&'a QTableWidgetItem) {
  fn row(self , rsthis: & QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget3rowEPK16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK12QTableWidget3rowEPK16QTableWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTableWidget::removeRow(int row);
impl /*struct*/ QTableWidget {
  pub fn removeRow<RetType, T: QTableWidget_removeRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeRow(self);
    // return 1;
  }
}

pub trait QTableWidget_removeRow<RetType> {
  fn removeRow(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::removeRow(int row);
impl<'a> /*trait*/ QTableWidget_removeRow<()> for (i32) {
  fn removeRow(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget9removeRowEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QTableWidget9removeRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableWidget::setItemPrototype(const QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn setItemPrototype<RetType, T: QTableWidget_setItemPrototype<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemPrototype(self);
    // return 1;
  }
}

pub trait QTableWidget_setItemPrototype<RetType> {
  fn setItemPrototype(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::setItemPrototype(const QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_setItemPrototype<()> for (&'a QTableWidgetItem) {
  fn setItemPrototype(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget16setItemPrototypeEPK16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QTableWidget16setItemPrototypeEPK16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableWidget::QTableWidget(int rows, int columns, QWidget * parent);
impl /*struct*/ QTableWidget {
  pub fn new<T: QTableWidget_new>(value: T) -> QTableWidget {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTableWidget_new {
  fn new(self) -> QTableWidget;
}

  // proto:  void QTableWidget::QTableWidget(int rows, int columns, QWidget * parent);
impl<'a> /*trait*/ QTableWidget_new for (i32, i32, &'a QWidget) {
  fn new(self) -> QTableWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidgetC2EiiP7QWidget()};
    let ctysz: c_int = unsafe{QTableWidget_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QTableWidgetC2EiiP7QWidget(arg0, arg1, arg2)};
    let rsthis = QTableWidget{qbase: QTableView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QTableWidget::visualRow(int logicalRow);
impl /*struct*/ QTableWidget {
  pub fn visualRow<RetType, T: QTableWidget_visualRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.visualRow(self);
    // return 1;
  }
}

pub trait QTableWidget_visualRow<RetType> {
  fn visualRow(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  int QTableWidget::visualRow(int logicalRow);
impl<'a> /*trait*/ QTableWidget_visualRow<i32> for (i32) {
  fn visualRow(self , rsthis: & QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget9visualRowEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK12QTableWidget9visualRowEi(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTableWidget::setCellWidget(int row, int column, QWidget * widget);
impl /*struct*/ QTableWidget {
  pub fn setCellWidget<RetType, T: QTableWidget_setCellWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCellWidget(self);
    // return 1;
  }
}

pub trait QTableWidget_setCellWidget<RetType> {
  fn setCellWidget(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::setCellWidget(int row, int column, QWidget * widget);
impl<'a> /*trait*/ QTableWidget_setCellWidget<()> for (i32, i32, &'a QWidget) {
  fn setCellWidget(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget13setCellWidgetEiiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN12QTableWidget13setCellWidgetEiiP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QTableWidget::openPersistentEditor(QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn openPersistentEditor<RetType, T: QTableWidget_openPersistentEditor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.openPersistentEditor(self);
    // return 1;
  }
}

pub trait QTableWidget_openPersistentEditor<RetType> {
  fn openPersistentEditor(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::openPersistentEditor(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_openPersistentEditor<()> for (&'a QTableWidgetItem) {
  fn openPersistentEditor(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget20openPersistentEditorEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QTableWidget20openPersistentEditorEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTableWidget::columnCount();
impl /*struct*/ QTableWidget {
  pub fn columnCount<RetType, T: QTableWidget_columnCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QTableWidget_columnCount<RetType> {
  fn columnCount(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  int QTableWidget::columnCount();
impl<'a> /*trait*/ QTableWidget_columnCount<i32> for () {
  fn columnCount(self , rsthis: & QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget11columnCountEv()};
    let mut ret = unsafe {C_ZNK12QTableWidget11columnCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QTableWidget::currentRow();
impl /*struct*/ QTableWidget {
  pub fn currentRow<RetType, T: QTableWidget_currentRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentRow(self);
    // return 1;
  }
}

pub trait QTableWidget_currentRow<RetType> {
  fn currentRow(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  int QTableWidget::currentRow();
impl<'a> /*trait*/ QTableWidget_currentRow<i32> for () {
  fn currentRow(self , rsthis: & QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget10currentRowEv()};
    let mut ret = unsafe {C_ZNK12QTableWidget10currentRowEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTableWidget::setCurrentItem(QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn setCurrentItem<RetType, T: QTableWidget_setCurrentItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentItem(self);
    // return 1;
  }
}

pub trait QTableWidget_setCurrentItem<RetType> {
  fn setCurrentItem(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::setCurrentItem(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_setCurrentItem<()> for (&'a QTableWidgetItem) {
  fn setCurrentItem(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget14setCurrentItemEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QTableWidget14setCurrentItemEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QTableWidget::cellWidget(int row, int column);
impl /*struct*/ QTableWidget {
  pub fn cellWidget<RetType, T: QTableWidget_cellWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cellWidget(self);
    // return 1;
  }
}

pub trait QTableWidget_cellWidget<RetType> {
  fn cellWidget(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  QWidget * QTableWidget::cellWidget(int row, int column);
impl<'a> /*trait*/ QTableWidget_cellWidget<QWidget> for (i32, i32) {
  fn cellWidget(self , rsthis: & QTableWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget10cellWidgetEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK12QTableWidget10cellWidgetEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTableWidget::setSortingEnabled(bool enable);
impl /*struct*/ QTableWidget {
  pub fn setSortingEnabled<RetType, T: QTableWidget_setSortingEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSortingEnabled(self);
    // return 1;
  }
}

pub trait QTableWidget_setSortingEnabled<RetType> {
  fn setSortingEnabled(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::setSortingEnabled(bool enable);
impl<'a> /*trait*/ QTableWidget_setSortingEnabled<()> for (i8) {
  fn setSortingEnabled(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget17setSortingEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN12QTableWidget17setSortingEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableWidget::setItem(int row, int column, QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn setItem<RetType, T: QTableWidget_setItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItem(self);
    // return 1;
  }
}

pub trait QTableWidget_setItem<RetType> {
  fn setItem(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::setItem(int row, int column, QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_setItem<()> for (i32, i32, &'a QTableWidgetItem) {
  fn setItem(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget7setItemEiiP16QTableWidgetItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN12QTableWidget7setItemEiiP16QTableWidgetItem(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QTableWidgetItem * QTableWidget::horizontalHeaderItem(int column);
impl /*struct*/ QTableWidget {
  pub fn horizontalHeaderItem<RetType, T: QTableWidget_horizontalHeaderItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.horizontalHeaderItem(self);
    // return 1;
  }
}

pub trait QTableWidget_horizontalHeaderItem<RetType> {
  fn horizontalHeaderItem(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  QTableWidgetItem * QTableWidget::horizontalHeaderItem(int column);
impl<'a> /*trait*/ QTableWidget_horizontalHeaderItem<QTableWidgetItem> for (i32) {
  fn horizontalHeaderItem(self , rsthis: & QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget20horizontalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK12QTableWidget20horizontalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTableWidgetItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTableWidget::editItem(QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn editItem<RetType, T: QTableWidget_editItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.editItem(self);
    // return 1;
  }
}

pub trait QTableWidget_editItem<RetType> {
  fn editItem(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::editItem(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_editItem<()> for (&'a QTableWidgetItem) {
  fn editItem(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget8editItemEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QTableWidget8editItemEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<QTableWidgetSelectionRange> QTableWidget::selectedRanges();
impl /*struct*/ QTableWidget {
  pub fn selectedRanges<RetType, T: QTableWidget_selectedRanges<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectedRanges(self);
    // return 1;
  }
}

pub trait QTableWidget_selectedRanges<RetType> {
  fn selectedRanges(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  QList<QTableWidgetSelectionRange> QTableWidget::selectedRanges();
impl<'a> /*trait*/ QTableWidget_selectedRanges<u64> for () {
  fn selectedRanges(self , rsthis: & QTableWidget) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget14selectedRangesEv()};
    let mut ret = unsafe {C_ZNK12QTableWidget14selectedRangesEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  int QTableWidget::currentColumn();
impl /*struct*/ QTableWidget {
  pub fn currentColumn<RetType, T: QTableWidget_currentColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentColumn(self);
    // return 1;
  }
}

pub trait QTableWidget_currentColumn<RetType> {
  fn currentColumn(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  int QTableWidget::currentColumn();
impl<'a> /*trait*/ QTableWidget_currentColumn<i32> for () {
  fn currentColumn(self , rsthis: & QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget13currentColumnEv()};
    let mut ret = unsafe {C_ZNK12QTableWidget13currentColumnEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTableWidget::removeColumn(int column);
impl /*struct*/ QTableWidget {
  pub fn removeColumn<RetType, T: QTableWidget_removeColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeColumn(self);
    // return 1;
  }
}

pub trait QTableWidget_removeColumn<RetType> {
  fn removeColumn(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::removeColumn(int column);
impl<'a> /*trait*/ QTableWidget_removeColumn<()> for (i32) {
  fn removeColumn(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget12removeColumnEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QTableWidget12removeColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableWidget::setRangeSelected(const QTableWidgetSelectionRange & range, bool select);
impl /*struct*/ QTableWidget {
  pub fn setRangeSelected<RetType, T: QTableWidget_setRangeSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRangeSelected(self);
    // return 1;
  }
}

pub trait QTableWidget_setRangeSelected<RetType> {
  fn setRangeSelected(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::setRangeSelected(const QTableWidgetSelectionRange & range, bool select);
impl<'a> /*trait*/ QTableWidget_setRangeSelected<()> for (&'a QTableWidgetSelectionRange, i8) {
  fn setRangeSelected(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget16setRangeSelectedERK26QTableWidgetSelectionRangeb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {C_ZN12QTableWidget16setRangeSelectedERK26QTableWidgetSelectionRangeb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QTableWidget::column(const QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn column<RetType, T: QTableWidget_column<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.column(self);
    // return 1;
  }
}

pub trait QTableWidget_column<RetType> {
  fn column(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  int QTableWidget::column(const QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_column<i32> for (&'a QTableWidgetItem) {
  fn column(self , rsthis: & QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget6columnEPK16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK12QTableWidget6columnEPK16QTableWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QTableWidget::isItemSelected(const QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn isItemSelected<RetType, T: QTableWidget_isItemSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isItemSelected(self);
    // return 1;
  }
}

pub trait QTableWidget_isItemSelected<RetType> {
  fn isItemSelected(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  bool QTableWidget::isItemSelected(const QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_isItemSelected<i8> for (&'a QTableWidgetItem) {
  fn isItemSelected(self , rsthis: & QTableWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget14isItemSelectedEPK16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK12QTableWidget14isItemSelectedEPK16QTableWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QTableWidgetItem * QTableWidget::takeVerticalHeaderItem(int row);
impl /*struct*/ QTableWidget {
  pub fn takeVerticalHeaderItem<RetType, T: QTableWidget_takeVerticalHeaderItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeVerticalHeaderItem(self);
    // return 1;
  }
}

pub trait QTableWidget_takeVerticalHeaderItem<RetType> {
  fn takeVerticalHeaderItem(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  QTableWidgetItem * QTableWidget::takeVerticalHeaderItem(int row);
impl<'a> /*trait*/ QTableWidget_takeVerticalHeaderItem<QTableWidgetItem> for (i32) {
  fn takeVerticalHeaderItem(self , rsthis: & QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget22takeVerticalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN12QTableWidget22takeVerticalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTableWidgetItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTableWidget::insertRow(int row);
impl /*struct*/ QTableWidget {
  pub fn insertRow<RetType, T: QTableWidget_insertRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertRow(self);
    // return 1;
  }
}

pub trait QTableWidget_insertRow<RetType> {
  fn insertRow(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::insertRow(int row);
impl<'a> /*trait*/ QTableWidget_insertRow<()> for (i32) {
  fn insertRow(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget9insertRowEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QTableWidget9insertRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTableWidget::rowCount();
impl /*struct*/ QTableWidget {
  pub fn rowCount<RetType, T: QTableWidget_rowCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QTableWidget_rowCount<RetType> {
  fn rowCount(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  int QTableWidget::rowCount();
impl<'a> /*trait*/ QTableWidget_rowCount<i32> for () {
  fn rowCount(self , rsthis: & QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget8rowCountEv()};
    let mut ret = unsafe {C_ZNK12QTableWidget8rowCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QTableWidgetItem * QTableWidget::item(int row, int column);
impl /*struct*/ QTableWidget {
  pub fn item<RetType, T: QTableWidget_item<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.item(self);
    // return 1;
  }
}

pub trait QTableWidget_item<RetType> {
  fn item(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  QTableWidgetItem * QTableWidget::item(int row, int column);
impl<'a> /*trait*/ QTableWidget_item<QTableWidgetItem> for (i32, i32) {
  fn item(self , rsthis: & QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget4itemEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK12QTableWidget4itemEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTableWidgetItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTableWidget::QTableWidget(QWidget * parent);
impl<'a> /*trait*/ QTableWidget_new for (&'a QWidget) {
  fn new(self) -> QTableWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidgetC2EP7QWidget()};
    let ctysz: c_int = unsafe{QTableWidget_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QTableWidgetC2EP7QWidget(arg0)};
    let rsthis = QTableWidget{qbase: QTableView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTableWidget::setVerticalHeaderLabels(const QStringList & labels);
impl /*struct*/ QTableWidget {
  pub fn setVerticalHeaderLabels<RetType, T: QTableWidget_setVerticalHeaderLabels<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVerticalHeaderLabels(self);
    // return 1;
  }
}

pub trait QTableWidget_setVerticalHeaderLabels<RetType> {
  fn setVerticalHeaderLabels(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::setVerticalHeaderLabels(const QStringList & labels);
impl<'a> /*trait*/ QTableWidget_setVerticalHeaderLabels<()> for (&'a QStringList) {
  fn setVerticalHeaderLabels(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget23setVerticalHeaderLabelsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QTableWidget23setVerticalHeaderLabelsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QTableWidgetItem * QTableWidget::itemPrototype();
impl /*struct*/ QTableWidget {
  pub fn itemPrototype<RetType, T: QTableWidget_itemPrototype<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemPrototype(self);
    // return 1;
  }
}

pub trait QTableWidget_itemPrototype<RetType> {
  fn itemPrototype(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  const QTableWidgetItem * QTableWidget::itemPrototype();
impl<'a> /*trait*/ QTableWidget_itemPrototype<QTableWidgetItem> for () {
  fn itemPrototype(self , rsthis: & QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget13itemPrototypeEv()};
    let mut ret = unsafe {C_ZNK12QTableWidget13itemPrototypeEv(rsthis.qclsinst)};
    let mut ret1 = QTableWidgetItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTableWidgetItem * QTableWidget::itemAt(const QPoint & p);
impl /*struct*/ QTableWidget {
  pub fn itemAt<RetType, T: QTableWidget_itemAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemAt(self);
    // return 1;
  }
}

pub trait QTableWidget_itemAt<RetType> {
  fn itemAt(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  QTableWidgetItem * QTableWidget::itemAt(const QPoint & p);
impl<'a> /*trait*/ QTableWidget_itemAt<QTableWidgetItem> for (&'a QPoint) {
  fn itemAt(self , rsthis: & QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget6itemAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK12QTableWidget6itemAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QTableWidgetItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTableWidget::clearContents();
impl /*struct*/ QTableWidget {
  pub fn clearContents<RetType, T: QTableWidget_clearContents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearContents(self);
    // return 1;
  }
}

pub trait QTableWidget_clearContents<RetType> {
  fn clearContents(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::clearContents();
impl<'a> /*trait*/ QTableWidget_clearContents<()> for () {
  fn clearContents(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget13clearContentsEv()};
     unsafe {C_ZN12QTableWidget13clearContentsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QTableWidgetItem * QTableWidget::itemAt(int x, int y);
impl<'a> /*trait*/ QTableWidget_itemAt<QTableWidgetItem> for (i32, i32) {
  fn itemAt(self , rsthis: & QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget6itemAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK12QTableWidget6itemAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTableWidgetItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTableWidget::setCurrentCell(int row, int column);
impl /*struct*/ QTableWidget {
  pub fn setCurrentCell<RetType, T: QTableWidget_setCurrentCell<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentCell(self);
    // return 1;
  }
}

pub trait QTableWidget_setCurrentCell<RetType> {
  fn setCurrentCell(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::setCurrentCell(int row, int column);
impl<'a> /*trait*/ QTableWidget_setCurrentCell<()> for (i32, i32) {
  fn setCurrentCell(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget14setCurrentCellEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN12QTableWidget14setCurrentCellEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTableWidget::setRowCount(int rows);
impl /*struct*/ QTableWidget {
  pub fn setRowCount<RetType, T: QTableWidget_setRowCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRowCount(self);
    // return 1;
  }
}

pub trait QTableWidget_setRowCount<RetType> {
  fn setRowCount(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::setRowCount(int rows);
impl<'a> /*trait*/ QTableWidget_setRowCount<()> for (i32) {
  fn setRowCount(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget11setRowCountEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QTableWidget11setRowCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableWidget::setHorizontalHeaderItem(int column, QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn setHorizontalHeaderItem<RetType, T: QTableWidget_setHorizontalHeaderItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalHeaderItem(self);
    // return 1;
  }
}

pub trait QTableWidget_setHorizontalHeaderItem<RetType> {
  fn setHorizontalHeaderItem(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::setHorizontalHeaderItem(int column, QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_setHorizontalHeaderItem<()> for (i32, &'a QTableWidgetItem) {
  fn setHorizontalHeaderItem(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget23setHorizontalHeaderItemEiP16QTableWidgetItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN12QTableWidget23setHorizontalHeaderItemEiP16QTableWidgetItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QTableWidget::visualColumn(int logicalColumn);
impl /*struct*/ QTableWidget {
  pub fn visualColumn<RetType, T: QTableWidget_visualColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.visualColumn(self);
    // return 1;
  }
}

pub trait QTableWidget_visualColumn<RetType> {
  fn visualColumn(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  int QTableWidget::visualColumn(int logicalColumn);
impl<'a> /*trait*/ QTableWidget_visualColumn<i32> for (i32) {
  fn visualColumn(self , rsthis: & QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget12visualColumnEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK12QTableWidget12visualColumnEi(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QTableWidgetItem * QTableWidget::takeHorizontalHeaderItem(int column);
impl /*struct*/ QTableWidget {
  pub fn takeHorizontalHeaderItem<RetType, T: QTableWidget_takeHorizontalHeaderItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeHorizontalHeaderItem(self);
    // return 1;
  }
}

pub trait QTableWidget_takeHorizontalHeaderItem<RetType> {
  fn takeHorizontalHeaderItem(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  QTableWidgetItem * QTableWidget::takeHorizontalHeaderItem(int column);
impl<'a> /*trait*/ QTableWidget_takeHorizontalHeaderItem<QTableWidgetItem> for (i32) {
  fn takeHorizontalHeaderItem(self , rsthis: & QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget24takeHorizontalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN12QTableWidget24takeHorizontalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTableWidgetItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTableWidgetItem * QTableWidget::verticalHeaderItem(int row);
impl /*struct*/ QTableWidget {
  pub fn verticalHeaderItem<RetType, T: QTableWidget_verticalHeaderItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.verticalHeaderItem(self);
    // return 1;
  }
}

pub trait QTableWidget_verticalHeaderItem<RetType> {
  fn verticalHeaderItem(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  QTableWidgetItem * QTableWidget::verticalHeaderItem(int row);
impl<'a> /*trait*/ QTableWidget_verticalHeaderItem<QTableWidgetItem> for (i32) {
  fn verticalHeaderItem(self , rsthis: & QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget18verticalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK12QTableWidget18verticalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTableWidgetItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTableWidget::clear();
impl /*struct*/ QTableWidget {
  pub fn clear<RetType, T: QTableWidget_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QTableWidget_clear<RetType> {
  fn clear(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::clear();
impl<'a> /*trait*/ QTableWidget_clear<()> for () {
  fn clear(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget5clearEv()};
     unsafe {C_ZN12QTableWidget5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTableWidget::insertColumn(int column);
impl /*struct*/ QTableWidget {
  pub fn insertColumn<RetType, T: QTableWidget_insertColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertColumn(self);
    // return 1;
  }
}

pub trait QTableWidget_insertColumn<RetType> {
  fn insertColumn(self , rsthis: & QTableWidget) -> RetType;
}

  // proto:  void QTableWidget::insertColumn(int column);
impl<'a> /*trait*/ QTableWidget_insertColumn<()> for (i32) {
  fn insertColumn(self , rsthis: & QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget12insertColumnEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QTableWidget12insertColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTableWidgetItem {
    return QTableWidgetItem{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QColor QTableWidgetItem::backgroundColor();
impl /*struct*/ QTableWidgetItem {
  pub fn backgroundColor<RetType, T: QTableWidgetItem_backgroundColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.backgroundColor(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_backgroundColor<RetType> {
  fn backgroundColor(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  QColor QTableWidgetItem::backgroundColor();
impl<'a> /*trait*/ QTableWidgetItem_backgroundColor<QColor> for () {
  fn backgroundColor(self , rsthis: & QTableWidgetItem) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem15backgroundColorEv()};
    let mut ret = unsafe {C_ZNK16QTableWidgetItem15backgroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QTableWidgetItem::data(int role);
impl /*struct*/ QTableWidgetItem {
  pub fn data<RetType, T: QTableWidgetItem_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_data<RetType> {
  fn data(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  QVariant QTableWidgetItem::data(int role);
impl<'a> /*trait*/ QTableWidgetItem_data<QVariant> for (i32) {
  fn data(self , rsthis: & QTableWidgetItem) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4dataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK16QTableWidgetItem4dataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::setSelected(bool select);
impl /*struct*/ QTableWidgetItem {
  pub fn setSelected<RetType, T: QTableWidgetItem_setSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSelected(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setSelected<RetType> {
  fn setSelected(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  void QTableWidgetItem::setSelected(bool select);
impl<'a> /*trait*/ QTableWidgetItem_setSelected<()> for (i8) {
  fn setSelected(self , rsthis: & QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem11setSelectedEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN16QTableWidgetItem11setSelectedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::setStatusTip(const QString & statusTip);
impl /*struct*/ QTableWidgetItem {
  pub fn setStatusTip<RetType, T: QTableWidgetItem_setStatusTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStatusTip(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setStatusTip<RetType> {
  fn setStatusTip(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  void QTableWidgetItem::setStatusTip(const QString & statusTip);
impl<'a> /*trait*/ QTableWidgetItem_setStatusTip<()> for (&'a QString) {
  fn setStatusTip(self , rsthis: & QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem12setStatusTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QTableWidgetItem12setStatusTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QColor QTableWidgetItem::textColor();
impl /*struct*/ QTableWidgetItem {
  pub fn textColor<RetType, T: QTableWidgetItem_textColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textColor(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_textColor<RetType> {
  fn textColor(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  QColor QTableWidgetItem::textColor();
impl<'a> /*trait*/ QTableWidgetItem_textColor<QColor> for () {
  fn textColor(self , rsthis: & QTableWidgetItem) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem9textColorEv()};
    let mut ret = unsafe {C_ZNK16QTableWidgetItem9textColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::~QTableWidgetItem();
impl /*struct*/ QTableWidgetItem {
  pub fn free<RetType, T: QTableWidgetItem_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_free<RetType> {
  fn free(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  void QTableWidgetItem::~QTableWidgetItem();
impl<'a> /*trait*/ QTableWidgetItem_free<()> for () {
  fn free(self , rsthis: & QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItemD2Ev()};
     unsafe {C_ZN16QTableWidgetItemD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QTableWidgetItem::text();
impl /*struct*/ QTableWidgetItem {
  pub fn text<RetType, T: QTableWidgetItem_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_text<RetType> {
  fn text(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  QString QTableWidgetItem::text();
impl<'a> /*trait*/ QTableWidgetItem_text<QString> for () {
  fn text(self , rsthis: & QTableWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4textEv()};
    let mut ret = unsafe {C_ZNK16QTableWidgetItem4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::setSizeHint(const QSize & size);
impl /*struct*/ QTableWidgetItem {
  pub fn setSizeHint<RetType, T: QTableWidgetItem_setSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSizeHint(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setSizeHint<RetType> {
  fn setSizeHint(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  void QTableWidgetItem::setSizeHint(const QSize & size);
impl<'a> /*trait*/ QTableWidgetItem_setSizeHint<()> for (&'a QSize) {
  fn setSizeHint(self , rsthis: & QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem11setSizeHintERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QTableWidgetItem11setSizeHintERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QBrush QTableWidgetItem::foreground();
impl /*struct*/ QTableWidgetItem {
  pub fn foreground<RetType, T: QTableWidgetItem_foreground<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.foreground(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_foreground<RetType> {
  fn foreground(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  QBrush QTableWidgetItem::foreground();
impl<'a> /*trait*/ QTableWidgetItem_foreground<QBrush> for () {
  fn foreground(self , rsthis: & QTableWidgetItem) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem10foregroundEv()};
    let mut ret = unsafe {C_ZNK16QTableWidgetItem10foregroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTableWidgetItem::type();
impl /*struct*/ QTableWidgetItem {
  pub fn type_<RetType, T: QTableWidgetItem_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_type_<RetType> {
  fn type_(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  int QTableWidgetItem::type();
impl<'a> /*trait*/ QTableWidgetItem_type_<i32> for () {
  fn type_(self , rsthis: & QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4typeEv()};
    let mut ret = unsafe {C_ZNK16QTableWidgetItem4typeEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QTableWidgetItem::column();
impl /*struct*/ QTableWidgetItem {
  pub fn column<RetType, T: QTableWidgetItem_column<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.column(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_column<RetType> {
  fn column(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  int QTableWidgetItem::column();
impl<'a> /*trait*/ QTableWidgetItem_column<i32> for () {
  fn column(self , rsthis: & QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem6columnEv()};
    let mut ret = unsafe {C_ZNK16QTableWidgetItem6columnEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::setTextAlignment(int alignment);
impl /*struct*/ QTableWidgetItem {
  pub fn setTextAlignment<RetType, T: QTableWidgetItem_setTextAlignment<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextAlignment(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setTextAlignment<RetType> {
  fn setTextAlignment(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  void QTableWidgetItem::setTextAlignment(int alignment);
impl<'a> /*trait*/ QTableWidgetItem_setTextAlignment<()> for (i32) {
  fn setTextAlignment(self , rsthis: & QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem16setTextAlignmentEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN16QTableWidgetItem16setTextAlignmentEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QFont QTableWidgetItem::font();
impl /*struct*/ QTableWidgetItem {
  pub fn font<RetType, T: QTableWidgetItem_font<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_font<RetType> {
  fn font(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  QFont QTableWidgetItem::font();
impl<'a> /*trait*/ QTableWidgetItem_font<QFont> for () {
  fn font(self , rsthis: & QTableWidgetItem) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4fontEv()};
    let mut ret = unsafe {C_ZNK16QTableWidgetItem4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QIcon QTableWidgetItem::icon();
impl /*struct*/ QTableWidgetItem {
  pub fn icon<RetType, T: QTableWidgetItem_icon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.icon(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_icon<RetType> {
  fn icon(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  QIcon QTableWidgetItem::icon();
impl<'a> /*trait*/ QTableWidgetItem_icon<QIcon> for () {
  fn icon(self , rsthis: & QTableWidgetItem) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4iconEv()};
    let mut ret = unsafe {C_ZNK16QTableWidgetItem4iconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::write(QDataStream & out);
impl /*struct*/ QTableWidgetItem {
  pub fn write<RetType, T: QTableWidgetItem_write<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.write(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_write<RetType> {
  fn write(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  void QTableWidgetItem::write(QDataStream & out);
impl<'a> /*trait*/ QTableWidgetItem_write<()> for (&'a QDataStream) {
  fn write(self , rsthis: & QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem5writeER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZNK16QTableWidgetItem5writeER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::QTableWidgetItem(const QTableWidgetItem & other);
impl /*struct*/ QTableWidgetItem {
  pub fn new<T: QTableWidgetItem_new>(value: T) -> QTableWidgetItem {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTableWidgetItem_new {
  fn new(self) -> QTableWidgetItem;
}

  // proto:  void QTableWidgetItem::QTableWidgetItem(const QTableWidgetItem & other);
impl<'a> /*trait*/ QTableWidgetItem_new for (&'a QTableWidgetItem) {
  fn new(self) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItemC2ERKS_()};
    let ctysz: c_int = unsafe{QTableWidgetItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN16QTableWidgetItemC2ERKS_(arg0)};
    let rsthis = QTableWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QBrush QTableWidgetItem::background();
impl /*struct*/ QTableWidgetItem {
  pub fn background<RetType, T: QTableWidgetItem_background<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.background(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_background<RetType> {
  fn background(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  QBrush QTableWidgetItem::background();
impl<'a> /*trait*/ QTableWidgetItem_background<QBrush> for () {
  fn background(self , rsthis: & QTableWidgetItem) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem10backgroundEv()};
    let mut ret = unsafe {C_ZNK16QTableWidgetItem10backgroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::setIcon(const QIcon & icon);
impl /*struct*/ QTableWidgetItem {
  pub fn setIcon<RetType, T: QTableWidgetItem_setIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIcon(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setIcon<RetType> {
  fn setIcon(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  void QTableWidgetItem::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QTableWidgetItem_setIcon<()> for (&'a QIcon) {
  fn setIcon(self , rsthis: & QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QTableWidgetItem7setIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::QTableWidgetItem(const QString & text, int type);
impl<'a> /*trait*/ QTableWidgetItem_new for (&'a QString, i32) {
  fn new(self) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItemC2ERK7QStringi()};
    let ctysz: c_int = unsafe{QTableWidgetItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let qthis: u64 = unsafe {C_ZN16QTableWidgetItemC2ERK7QStringi(arg0, arg1)};
    let rsthis = QTableWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QTableWidgetItem::statusTip();
impl /*struct*/ QTableWidgetItem {
  pub fn statusTip<RetType, T: QTableWidgetItem_statusTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.statusTip(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_statusTip<RetType> {
  fn statusTip(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  QString QTableWidgetItem::statusTip();
impl<'a> /*trait*/ QTableWidgetItem_statusTip<QString> for () {
  fn statusTip(self , rsthis: & QTableWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem9statusTipEv()};
    let mut ret = unsafe {C_ZNK16QTableWidgetItem9statusTipEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTableWidgetItem * QTableWidgetItem::clone();
impl /*struct*/ QTableWidgetItem {
  pub fn clone<RetType, T: QTableWidgetItem_clone<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clone(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_clone<RetType> {
  fn clone(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  QTableWidgetItem * QTableWidgetItem::clone();
impl<'a> /*trait*/ QTableWidgetItem_clone<QTableWidgetItem> for () {
  fn clone(self , rsthis: & QTableWidgetItem) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem5cloneEv()};
    let mut ret = unsafe {C_ZNK16QTableWidgetItem5cloneEv(rsthis.qclsinst)};
    let mut ret1 = QTableWidgetItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::QTableWidgetItem(int type);
impl<'a> /*trait*/ QTableWidgetItem_new for (i32) {
  fn new(self) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItemC2Ei()};
    let ctysz: c_int = unsafe{QTableWidgetItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self  as c_int;
    let qthis: u64 = unsafe {C_ZN16QTableWidgetItemC2Ei(arg0)};
    let rsthis = QTableWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::setWhatsThis(const QString & whatsThis);
impl /*struct*/ QTableWidgetItem {
  pub fn setWhatsThis<RetType, T: QTableWidgetItem_setWhatsThis<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWhatsThis(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setWhatsThis<RetType> {
  fn setWhatsThis(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  void QTableWidgetItem::setWhatsThis(const QString & whatsThis);
impl<'a> /*trait*/ QTableWidgetItem_setWhatsThis<()> for (&'a QString) {
  fn setWhatsThis(self , rsthis: & QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QTableWidgetItem12setWhatsThisERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QTableWidgetItem::sizeHint();
impl /*struct*/ QTableWidgetItem {
  pub fn sizeHint<RetType, T: QTableWidgetItem_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  QSize QTableWidgetItem::sizeHint();
impl<'a> /*trait*/ QTableWidgetItem_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QTableWidgetItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem8sizeHintEv()};
    let mut ret = unsafe {C_ZNK16QTableWidgetItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::setForeground(const QBrush & brush);
impl /*struct*/ QTableWidgetItem {
  pub fn setForeground<RetType, T: QTableWidgetItem_setForeground<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setForeground(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setForeground<RetType> {
  fn setForeground(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  void QTableWidgetItem::setForeground(const QBrush & brush);
impl<'a> /*trait*/ QTableWidgetItem_setForeground<()> for (&'a QBrush) {
  fn setForeground(self , rsthis: & QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem13setForegroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QTableWidgetItem13setForegroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTableWidgetItem::row();
impl /*struct*/ QTableWidgetItem {
  pub fn row<RetType, T: QTableWidgetItem_row<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.row(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_row<RetType> {
  fn row(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  int QTableWidgetItem::row();
impl<'a> /*trait*/ QTableWidgetItem_row<i32> for () {
  fn row(self , rsthis: & QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem3rowEv()};
    let mut ret = unsafe {C_ZNK16QTableWidgetItem3rowEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::setData(int role, const QVariant & value);
impl /*struct*/ QTableWidgetItem {
  pub fn setData<RetType, T: QTableWidgetItem_setData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setData<RetType> {
  fn setData(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  void QTableWidgetItem::setData(int role, const QVariant & value);
impl<'a> /*trait*/ QTableWidgetItem_setData<()> for (i32, &'a QVariant) {
  fn setData(self , rsthis: & QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem7setDataEiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN16QTableWidgetItem7setDataEiRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QTableWidget * QTableWidgetItem::tableWidget();
impl /*struct*/ QTableWidgetItem {
  pub fn tableWidget<RetType, T: QTableWidgetItem_tableWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tableWidget(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_tableWidget<RetType> {
  fn tableWidget(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  QTableWidget * QTableWidgetItem::tableWidget();
impl<'a> /*trait*/ QTableWidgetItem_tableWidget<QTableWidget> for () {
  fn tableWidget(self , rsthis: & QTableWidgetItem) -> QTableWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem11tableWidgetEv()};
    let mut ret = unsafe {C_ZNK16QTableWidgetItem11tableWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QTableWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::QTableWidgetItem(const QIcon & icon, const QString & text, int type);
impl<'a> /*trait*/ QTableWidgetItem_new for (&'a QIcon, &'a QString, i32) {
  fn new(self) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItemC2ERK5QIconRK7QStringi()};
    let ctysz: c_int = unsafe{QTableWidgetItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let qthis: u64 = unsafe {C_ZN16QTableWidgetItemC2ERK5QIconRK7QStringi(arg0, arg1, arg2)};
    let rsthis = QTableWidgetItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QTableWidgetItem::textAlignment();
impl /*struct*/ QTableWidgetItem {
  pub fn textAlignment<RetType, T: QTableWidgetItem_textAlignment<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textAlignment(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_textAlignment<RetType> {
  fn textAlignment(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  int QTableWidgetItem::textAlignment();
impl<'a> /*trait*/ QTableWidgetItem_textAlignment<i32> for () {
  fn textAlignment(self , rsthis: & QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem13textAlignmentEv()};
    let mut ret = unsafe {C_ZNK16QTableWidgetItem13textAlignmentEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::read(QDataStream & in);
impl /*struct*/ QTableWidgetItem {
  pub fn read<RetType, T: QTableWidgetItem_read<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.read(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_read<RetType> {
  fn read(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  void QTableWidgetItem::read(QDataStream & in);
impl<'a> /*trait*/ QTableWidgetItem_read<()> for (&'a QDataStream) {
  fn read(self , rsthis: & QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem4readER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QTableWidgetItem4readER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QTableWidgetItem::toolTip();
impl /*struct*/ QTableWidgetItem {
  pub fn toolTip<RetType, T: QTableWidgetItem_toolTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toolTip(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_toolTip<RetType> {
  fn toolTip(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  QString QTableWidgetItem::toolTip();
impl<'a> /*trait*/ QTableWidgetItem_toolTip<QString> for () {
  fn toolTip(self , rsthis: & QTableWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem7toolTipEv()};
    let mut ret = unsafe {C_ZNK16QTableWidgetItem7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTableWidgetItem::isSelected();
impl /*struct*/ QTableWidgetItem {
  pub fn isSelected<RetType, T: QTableWidgetItem_isSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSelected(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_isSelected<RetType> {
  fn isSelected(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  bool QTableWidgetItem::isSelected();
impl<'a> /*trait*/ QTableWidgetItem_isSelected<i8> for () {
  fn isSelected(self , rsthis: & QTableWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem10isSelectedEv()};
    let mut ret = unsafe {C_ZNK16QTableWidgetItem10isSelectedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::setBackgroundColor(const QColor & color);
impl /*struct*/ QTableWidgetItem {
  pub fn setBackgroundColor<RetType, T: QTableWidgetItem_setBackgroundColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBackgroundColor(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setBackgroundColor<RetType> {
  fn setBackgroundColor(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  void QTableWidgetItem::setBackgroundColor(const QColor & color);
impl<'a> /*trait*/ QTableWidgetItem_setBackgroundColor<()> for (&'a QColor) {
  fn setBackgroundColor(self , rsthis: & QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem18setBackgroundColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QTableWidgetItem18setBackgroundColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::setBackground(const QBrush & brush);
impl /*struct*/ QTableWidgetItem {
  pub fn setBackground<RetType, T: QTableWidgetItem_setBackground<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBackground(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setBackground<RetType> {
  fn setBackground(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  void QTableWidgetItem::setBackground(const QBrush & brush);
impl<'a> /*trait*/ QTableWidgetItem_setBackground<()> for (&'a QBrush) {
  fn setBackground(self , rsthis: & QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QTableWidgetItem13setBackgroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::setFont(const QFont & font);
impl /*struct*/ QTableWidgetItem {
  pub fn setFont<RetType, T: QTableWidgetItem_setFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFont(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setFont<RetType> {
  fn setFont(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  void QTableWidgetItem::setFont(const QFont & font);
impl<'a> /*trait*/ QTableWidgetItem_setFont<()> for (&'a QFont) {
  fn setFont(self , rsthis: & QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QTableWidgetItem7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::setTextColor(const QColor & color);
impl /*struct*/ QTableWidgetItem {
  pub fn setTextColor<RetType, T: QTableWidgetItem_setTextColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextColor(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setTextColor<RetType> {
  fn setTextColor(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  void QTableWidgetItem::setTextColor(const QColor & color);
impl<'a> /*trait*/ QTableWidgetItem_setTextColor<()> for (&'a QColor) {
  fn setTextColor(self , rsthis: & QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem12setTextColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QTableWidgetItem12setTextColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::setText(const QString & text);
impl /*struct*/ QTableWidgetItem {
  pub fn setText<RetType, T: QTableWidgetItem_setText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setText<RetType> {
  fn setText(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  void QTableWidgetItem::setText(const QString & text);
impl<'a> /*trait*/ QTableWidgetItem_setText<()> for (&'a QString) {
  fn setText(self , rsthis: & QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QTableWidgetItem7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QTableWidgetItem::whatsThis();
impl /*struct*/ QTableWidgetItem {
  pub fn whatsThis<RetType, T: QTableWidgetItem_whatsThis<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.whatsThis(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_whatsThis<RetType> {
  fn whatsThis(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  QString QTableWidgetItem::whatsThis();
impl<'a> /*trait*/ QTableWidgetItem_whatsThis<QString> for () {
  fn whatsThis(self , rsthis: & QTableWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem9whatsThisEv()};
    let mut ret = unsafe {C_ZNK16QTableWidgetItem9whatsThisEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTableWidgetItem::setToolTip(const QString & toolTip);
impl /*struct*/ QTableWidgetItem {
  pub fn setToolTip<RetType, T: QTableWidgetItem_setToolTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setToolTip(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setToolTip<RetType> {
  fn setToolTip(self , rsthis: & QTableWidgetItem) -> RetType;
}

  // proto:  void QTableWidgetItem::setToolTip(const QString & toolTip);
impl<'a> /*trait*/ QTableWidgetItem_setToolTip<()> for (&'a QString) {
  fn setToolTip(self , rsthis: & QTableWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QTableWidgetItem10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QTableWidget_itemDoubleClicked
pub struct QTableWidget_itemDoubleClicked_signal{poi:u64}
impl /* struct */ QTableWidget {
  pub fn itemDoubleClicked(&self) -> QTableWidget_itemDoubleClicked_signal {
     return QTableWidget_itemDoubleClicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTableWidget_itemDoubleClicked_signal {
  pub fn connect<T: QTableWidget_itemDoubleClicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTableWidget_itemDoubleClicked_signal_connect {
  fn connect(self, sigthis: QTableWidget_itemDoubleClicked_signal);
}

#[derive(Default)] // for QTableWidget_cellEntered
pub struct QTableWidget_cellEntered_signal{poi:u64}
impl /* struct */ QTableWidget {
  pub fn cellEntered(&self) -> QTableWidget_cellEntered_signal {
     return QTableWidget_cellEntered_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTableWidget_cellEntered_signal {
  pub fn connect<T: QTableWidget_cellEntered_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTableWidget_cellEntered_signal_connect {
  fn connect(self, sigthis: QTableWidget_cellEntered_signal);
}

#[derive(Default)] // for QTableWidget_itemClicked
pub struct QTableWidget_itemClicked_signal{poi:u64}
impl /* struct */ QTableWidget {
  pub fn itemClicked(&self) -> QTableWidget_itemClicked_signal {
     return QTableWidget_itemClicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTableWidget_itemClicked_signal {
  pub fn connect<T: QTableWidget_itemClicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTableWidget_itemClicked_signal_connect {
  fn connect(self, sigthis: QTableWidget_itemClicked_signal);
}

#[derive(Default)] // for QTableWidget_currentItemChanged
pub struct QTableWidget_currentItemChanged_signal{poi:u64}
impl /* struct */ QTableWidget {
  pub fn currentItemChanged(&self) -> QTableWidget_currentItemChanged_signal {
     return QTableWidget_currentItemChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTableWidget_currentItemChanged_signal {
  pub fn connect<T: QTableWidget_currentItemChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTableWidget_currentItemChanged_signal_connect {
  fn connect(self, sigthis: QTableWidget_currentItemChanged_signal);
}

#[derive(Default)] // for QTableWidget_itemEntered
pub struct QTableWidget_itemEntered_signal{poi:u64}
impl /* struct */ QTableWidget {
  pub fn itemEntered(&self) -> QTableWidget_itemEntered_signal {
     return QTableWidget_itemEntered_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTableWidget_itemEntered_signal {
  pub fn connect<T: QTableWidget_itemEntered_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTableWidget_itemEntered_signal_connect {
  fn connect(self, sigthis: QTableWidget_itemEntered_signal);
}

#[derive(Default)] // for QTableWidget_itemPressed
pub struct QTableWidget_itemPressed_signal{poi:u64}
impl /* struct */ QTableWidget {
  pub fn itemPressed(&self) -> QTableWidget_itemPressed_signal {
     return QTableWidget_itemPressed_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTableWidget_itemPressed_signal {
  pub fn connect<T: QTableWidget_itemPressed_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTableWidget_itemPressed_signal_connect {
  fn connect(self, sigthis: QTableWidget_itemPressed_signal);
}

#[derive(Default)] // for QTableWidget_cellClicked
pub struct QTableWidget_cellClicked_signal{poi:u64}
impl /* struct */ QTableWidget {
  pub fn cellClicked(&self) -> QTableWidget_cellClicked_signal {
     return QTableWidget_cellClicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTableWidget_cellClicked_signal {
  pub fn connect<T: QTableWidget_cellClicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTableWidget_cellClicked_signal_connect {
  fn connect(self, sigthis: QTableWidget_cellClicked_signal);
}

#[derive(Default)] // for QTableWidget_itemSelectionChanged
pub struct QTableWidget_itemSelectionChanged_signal{poi:u64}
impl /* struct */ QTableWidget {
  pub fn itemSelectionChanged(&self) -> QTableWidget_itemSelectionChanged_signal {
     return QTableWidget_itemSelectionChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTableWidget_itemSelectionChanged_signal {
  pub fn connect<T: QTableWidget_itemSelectionChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTableWidget_itemSelectionChanged_signal_connect {
  fn connect(self, sigthis: QTableWidget_itemSelectionChanged_signal);
}

#[derive(Default)] // for QTableWidget_cellChanged
pub struct QTableWidget_cellChanged_signal{poi:u64}
impl /* struct */ QTableWidget {
  pub fn cellChanged(&self) -> QTableWidget_cellChanged_signal {
     return QTableWidget_cellChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTableWidget_cellChanged_signal {
  pub fn connect<T: QTableWidget_cellChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTableWidget_cellChanged_signal_connect {
  fn connect(self, sigthis: QTableWidget_cellChanged_signal);
}

#[derive(Default)] // for QTableWidget_itemActivated
pub struct QTableWidget_itemActivated_signal{poi:u64}
impl /* struct */ QTableWidget {
  pub fn itemActivated(&self) -> QTableWidget_itemActivated_signal {
     return QTableWidget_itemActivated_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTableWidget_itemActivated_signal {
  pub fn connect<T: QTableWidget_itemActivated_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTableWidget_itemActivated_signal_connect {
  fn connect(self, sigthis: QTableWidget_itemActivated_signal);
}

#[derive(Default)] // for QTableWidget_cellActivated
pub struct QTableWidget_cellActivated_signal{poi:u64}
impl /* struct */ QTableWidget {
  pub fn cellActivated(&self) -> QTableWidget_cellActivated_signal {
     return QTableWidget_cellActivated_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTableWidget_cellActivated_signal {
  pub fn connect<T: QTableWidget_cellActivated_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTableWidget_cellActivated_signal_connect {
  fn connect(self, sigthis: QTableWidget_cellActivated_signal);
}

#[derive(Default)] // for QTableWidget_itemChanged
pub struct QTableWidget_itemChanged_signal{poi:u64}
impl /* struct */ QTableWidget {
  pub fn itemChanged(&self) -> QTableWidget_itemChanged_signal {
     return QTableWidget_itemChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTableWidget_itemChanged_signal {
  pub fn connect<T: QTableWidget_itemChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTableWidget_itemChanged_signal_connect {
  fn connect(self, sigthis: QTableWidget_itemChanged_signal);
}

#[derive(Default)] // for QTableWidget_currentCellChanged
pub struct QTableWidget_currentCellChanged_signal{poi:u64}
impl /* struct */ QTableWidget {
  pub fn currentCellChanged(&self) -> QTableWidget_currentCellChanged_signal {
     return QTableWidget_currentCellChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTableWidget_currentCellChanged_signal {
  pub fn connect<T: QTableWidget_currentCellChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTableWidget_currentCellChanged_signal_connect {
  fn connect(self, sigthis: QTableWidget_currentCellChanged_signal);
}

#[derive(Default)] // for QTableWidget_cellDoubleClicked
pub struct QTableWidget_cellDoubleClicked_signal{poi:u64}
impl /* struct */ QTableWidget {
  pub fn cellDoubleClicked(&self) -> QTableWidget_cellDoubleClicked_signal {
     return QTableWidget_cellDoubleClicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTableWidget_cellDoubleClicked_signal {
  pub fn connect<T: QTableWidget_cellDoubleClicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTableWidget_cellDoubleClicked_signal_connect {
  fn connect(self, sigthis: QTableWidget_cellDoubleClicked_signal);
}

#[derive(Default)] // for QTableWidget_cellPressed
pub struct QTableWidget_cellPressed_signal{poi:u64}
impl /* struct */ QTableWidget {
  pub fn cellPressed(&self) -> QTableWidget_cellPressed_signal {
     return QTableWidget_cellPressed_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTableWidget_cellPressed_signal {
  pub fn connect<T: QTableWidget_cellPressed_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTableWidget_cellPressed_signal_connect {
  fn connect(self, sigthis: QTableWidget_cellPressed_signal);
}

// itemActivated(class QTableWidgetItem *)
extern fn QTableWidget_itemActivated_signal_connect_cb_0(rsfptr:fn(QTableWidgetItem), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QTableWidgetItem::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QTableWidget_itemActivated_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QTableWidgetItem)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QTableWidgetItem::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTableWidget_itemActivated_signal_connect for fn(QTableWidgetItem) {
  fn connect(self, sigthis: QTableWidget_itemActivated_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_itemActivated_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget13itemActivatedEP16QTableWidgetItem(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTableWidget_itemActivated_signal_connect for Box<Fn(QTableWidgetItem)> {
  fn connect(self, sigthis: QTableWidget_itemActivated_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_itemActivated_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget13itemActivatedEP16QTableWidgetItem(arg0, arg1, arg2)};
  }
}
// itemSelectionChanged()
extern fn QTableWidget_itemSelectionChanged_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QTableWidget_itemSelectionChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QTableWidget_itemSelectionChanged_signal_connect for fn() {
  fn connect(self, sigthis: QTableWidget_itemSelectionChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_itemSelectionChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget20itemSelectionChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTableWidget_itemSelectionChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QTableWidget_itemSelectionChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_itemSelectionChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget20itemSelectionChangedEv(arg0, arg1, arg2)};
  }
}
// itemChanged(class QTableWidgetItem *)
extern fn QTableWidget_itemChanged_signal_connect_cb_2(rsfptr:fn(QTableWidgetItem), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QTableWidgetItem::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QTableWidget_itemChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(QTableWidgetItem)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QTableWidgetItem::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTableWidget_itemChanged_signal_connect for fn(QTableWidgetItem) {
  fn connect(self, sigthis: QTableWidget_itemChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_itemChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget11itemChangedEP16QTableWidgetItem(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTableWidget_itemChanged_signal_connect for Box<Fn(QTableWidgetItem)> {
  fn connect(self, sigthis: QTableWidget_itemChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_itemChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget11itemChangedEP16QTableWidgetItem(arg0, arg1, arg2)};
  }
}
// itemPressed(class QTableWidgetItem *)
extern fn QTableWidget_itemPressed_signal_connect_cb_3(rsfptr:fn(QTableWidgetItem), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QTableWidgetItem::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QTableWidget_itemPressed_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(QTableWidgetItem)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QTableWidgetItem::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTableWidget_itemPressed_signal_connect for fn(QTableWidgetItem) {
  fn connect(self, sigthis: QTableWidget_itemPressed_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_itemPressed_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget11itemPressedEP16QTableWidgetItem(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTableWidget_itemPressed_signal_connect for Box<Fn(QTableWidgetItem)> {
  fn connect(self, sigthis: QTableWidget_itemPressed_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_itemPressed_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget11itemPressedEP16QTableWidgetItem(arg0, arg1, arg2)};
  }
}
// currentItemChanged(class QTableWidgetItem *, class QTableWidgetItem *)
extern fn QTableWidget_currentItemChanged_signal_connect_cb_4(rsfptr:fn(QTableWidgetItem, QTableWidgetItem), arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QTableWidgetItem::inheritFrom(arg0 as u64);
  let rsarg1 = QTableWidgetItem::inheritFrom(arg1 as u64);
  rsfptr(rsarg0,rsarg1);
}
extern fn QTableWidget_currentItemChanged_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn(QTableWidgetItem, QTableWidgetItem)>, arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QTableWidgetItem::inheritFrom(arg0 as u64);
  let rsarg1 = QTableWidgetItem::inheritFrom(arg1 as u64);
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QTableWidget_currentItemChanged_signal_connect for fn(QTableWidgetItem, QTableWidgetItem) {
  fn connect(self, sigthis: QTableWidget_currentItemChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_currentItemChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget18currentItemChangedEP16QTableWidgetItemS1_(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTableWidget_currentItemChanged_signal_connect for Box<Fn(QTableWidgetItem, QTableWidgetItem)> {
  fn connect(self, sigthis: QTableWidget_currentItemChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_currentItemChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget18currentItemChangedEP16QTableWidgetItemS1_(arg0, arg1, arg2)};
  }
}
// currentCellChanged(int, int, int, int)
extern fn QTableWidget_currentCellChanged_signal_connect_cb_5(rsfptr:fn(i32, i32, i32, i32), arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  let rsarg2 = arg2 as i32;
  let rsarg3 = arg3 as i32;
  rsfptr(rsarg0,rsarg1,rsarg2,rsarg3);
}
extern fn QTableWidget_currentCellChanged_signal_connect_cb_box_5(rsfptr_raw:*mut Box<Fn(i32, i32, i32, i32)>, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  let rsarg2 = arg2 as i32;
  let rsarg3 = arg3 as i32;
  // rsfptr(rsarg0,rsarg1,rsarg2,rsarg3);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1,rsarg2,rsarg3)};
}
impl /* trait */ QTableWidget_currentCellChanged_signal_connect for fn(i32, i32, i32, i32) {
  fn connect(self, sigthis: QTableWidget_currentCellChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_currentCellChanged_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget18currentCellChangedEiiii(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTableWidget_currentCellChanged_signal_connect for Box<Fn(i32, i32, i32, i32)> {
  fn connect(self, sigthis: QTableWidget_currentCellChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_currentCellChanged_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget18currentCellChangedEiiii(arg0, arg1, arg2)};
  }
}
// itemEntered(class QTableWidgetItem *)
extern fn QTableWidget_itemEntered_signal_connect_cb_6(rsfptr:fn(QTableWidgetItem), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QTableWidgetItem::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QTableWidget_itemEntered_signal_connect_cb_box_6(rsfptr_raw:*mut Box<Fn(QTableWidgetItem)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QTableWidgetItem::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTableWidget_itemEntered_signal_connect for fn(QTableWidgetItem) {
  fn connect(self, sigthis: QTableWidget_itemEntered_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_itemEntered_signal_connect_cb_6 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget11itemEnteredEP16QTableWidgetItem(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTableWidget_itemEntered_signal_connect for Box<Fn(QTableWidgetItem)> {
  fn connect(self, sigthis: QTableWidget_itemEntered_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_itemEntered_signal_connect_cb_box_6 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget11itemEnteredEP16QTableWidgetItem(arg0, arg1, arg2)};
  }
}
// cellEntered(int, int)
extern fn QTableWidget_cellEntered_signal_connect_cb_7(rsfptr:fn(i32, i32), arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  rsfptr(rsarg0,rsarg1);
}
extern fn QTableWidget_cellEntered_signal_connect_cb_box_7(rsfptr_raw:*mut Box<Fn(i32, i32)>, arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QTableWidget_cellEntered_signal_connect for fn(i32, i32) {
  fn connect(self, sigthis: QTableWidget_cellEntered_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_cellEntered_signal_connect_cb_7 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget11cellEnteredEii(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTableWidget_cellEntered_signal_connect for Box<Fn(i32, i32)> {
  fn connect(self, sigthis: QTableWidget_cellEntered_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_cellEntered_signal_connect_cb_box_7 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget11cellEnteredEii(arg0, arg1, arg2)};
  }
}
// cellActivated(int, int)
extern fn QTableWidget_cellActivated_signal_connect_cb_8(rsfptr:fn(i32, i32), arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  rsfptr(rsarg0,rsarg1);
}
extern fn QTableWidget_cellActivated_signal_connect_cb_box_8(rsfptr_raw:*mut Box<Fn(i32, i32)>, arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QTableWidget_cellActivated_signal_connect for fn(i32, i32) {
  fn connect(self, sigthis: QTableWidget_cellActivated_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_cellActivated_signal_connect_cb_8 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget13cellActivatedEii(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTableWidget_cellActivated_signal_connect for Box<Fn(i32, i32)> {
  fn connect(self, sigthis: QTableWidget_cellActivated_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_cellActivated_signal_connect_cb_box_8 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget13cellActivatedEii(arg0, arg1, arg2)};
  }
}
// itemClicked(class QTableWidgetItem *)
extern fn QTableWidget_itemClicked_signal_connect_cb_9(rsfptr:fn(QTableWidgetItem), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QTableWidgetItem::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QTableWidget_itemClicked_signal_connect_cb_box_9(rsfptr_raw:*mut Box<Fn(QTableWidgetItem)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QTableWidgetItem::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTableWidget_itemClicked_signal_connect for fn(QTableWidgetItem) {
  fn connect(self, sigthis: QTableWidget_itemClicked_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_itemClicked_signal_connect_cb_9 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget11itemClickedEP16QTableWidgetItem(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTableWidget_itemClicked_signal_connect for Box<Fn(QTableWidgetItem)> {
  fn connect(self, sigthis: QTableWidget_itemClicked_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_itemClicked_signal_connect_cb_box_9 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget11itemClickedEP16QTableWidgetItem(arg0, arg1, arg2)};
  }
}
// cellClicked(int, int)
extern fn QTableWidget_cellClicked_signal_connect_cb_10(rsfptr:fn(i32, i32), arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  rsfptr(rsarg0,rsarg1);
}
extern fn QTableWidget_cellClicked_signal_connect_cb_box_10(rsfptr_raw:*mut Box<Fn(i32, i32)>, arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QTableWidget_cellClicked_signal_connect for fn(i32, i32) {
  fn connect(self, sigthis: QTableWidget_cellClicked_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_cellClicked_signal_connect_cb_10 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget11cellClickedEii(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTableWidget_cellClicked_signal_connect for Box<Fn(i32, i32)> {
  fn connect(self, sigthis: QTableWidget_cellClicked_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_cellClicked_signal_connect_cb_box_10 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget11cellClickedEii(arg0, arg1, arg2)};
  }
}
// itemDoubleClicked(class QTableWidgetItem *)
extern fn QTableWidget_itemDoubleClicked_signal_connect_cb_11(rsfptr:fn(QTableWidgetItem), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QTableWidgetItem::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QTableWidget_itemDoubleClicked_signal_connect_cb_box_11(rsfptr_raw:*mut Box<Fn(QTableWidgetItem)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QTableWidgetItem::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTableWidget_itemDoubleClicked_signal_connect for fn(QTableWidgetItem) {
  fn connect(self, sigthis: QTableWidget_itemDoubleClicked_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_itemDoubleClicked_signal_connect_cb_11 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget17itemDoubleClickedEP16QTableWidgetItem(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTableWidget_itemDoubleClicked_signal_connect for Box<Fn(QTableWidgetItem)> {
  fn connect(self, sigthis: QTableWidget_itemDoubleClicked_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_itemDoubleClicked_signal_connect_cb_box_11 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget17itemDoubleClickedEP16QTableWidgetItem(arg0, arg1, arg2)};
  }
}
// cellDoubleClicked(int, int)
extern fn QTableWidget_cellDoubleClicked_signal_connect_cb_12(rsfptr:fn(i32, i32), arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  rsfptr(rsarg0,rsarg1);
}
extern fn QTableWidget_cellDoubleClicked_signal_connect_cb_box_12(rsfptr_raw:*mut Box<Fn(i32, i32)>, arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QTableWidget_cellDoubleClicked_signal_connect for fn(i32, i32) {
  fn connect(self, sigthis: QTableWidget_cellDoubleClicked_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_cellDoubleClicked_signal_connect_cb_12 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget17cellDoubleClickedEii(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTableWidget_cellDoubleClicked_signal_connect for Box<Fn(i32, i32)> {
  fn connect(self, sigthis: QTableWidget_cellDoubleClicked_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_cellDoubleClicked_signal_connect_cb_box_12 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget17cellDoubleClickedEii(arg0, arg1, arg2)};
  }
}
// cellChanged(int, int)
extern fn QTableWidget_cellChanged_signal_connect_cb_13(rsfptr:fn(i32, i32), arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  rsfptr(rsarg0,rsarg1);
}
extern fn QTableWidget_cellChanged_signal_connect_cb_box_13(rsfptr_raw:*mut Box<Fn(i32, i32)>, arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QTableWidget_cellChanged_signal_connect for fn(i32, i32) {
  fn connect(self, sigthis: QTableWidget_cellChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_cellChanged_signal_connect_cb_13 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget11cellChangedEii(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTableWidget_cellChanged_signal_connect for Box<Fn(i32, i32)> {
  fn connect(self, sigthis: QTableWidget_cellChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_cellChanged_signal_connect_cb_box_13 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget11cellChangedEii(arg0, arg1, arg2)};
  }
}
// cellPressed(int, int)
extern fn QTableWidget_cellPressed_signal_connect_cb_14(rsfptr:fn(i32, i32), arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  rsfptr(rsarg0,rsarg1);
}
extern fn QTableWidget_cellPressed_signal_connect_cb_box_14(rsfptr_raw:*mut Box<Fn(i32, i32)>, arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QTableWidget_cellPressed_signal_connect for fn(i32, i32) {
  fn connect(self, sigthis: QTableWidget_cellPressed_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_cellPressed_signal_connect_cb_14 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget11cellPressedEii(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTableWidget_cellPressed_signal_connect for Box<Fn(i32, i32)> {
  fn connect(self, sigthis: QTableWidget_cellPressed_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTableWidget_cellPressed_signal_connect_cb_box_14 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTableWidget_SlotProxy_connect__ZN12QTableWidget11cellPressedEii(arg0, arg1, arg2)};
  }
}
// <= body block end

