// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtGui/qaccessible.h
// dst-file: /src/gui/qaccessible.rs
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
// use super::qaccessible::QAccessibleInterface; // 773
use super::qtextcursor::QTextCursor; // 773
use super::super::core::qobject::QObject; // 771
// use super::qaccessible::QAccessibleEvent; // 773
use super::super::core::qpoint::QPoint; // 771
use super::super::core::qstring::QString; // 771
use super::super::core::qrect::QRect; // 771
// use super::qaccessible::QAccessibleImageInterface; // 773
// use super::qaccessible::QAccessibleTableInterface; // 773
// use super::qaccessible::QAccessibleEditableTextInterface; // 773
// use super::qaccessible::QAccessibleValueInterface; // 773
// use super::qaccessible::QAccessibleActionInterface; // 773
// use super::qaccessible::QAccessibleTableCellInterface; // 773
use super::qcolor::QColor; // 773
use super::qwindow::QWindow; // 773
// use super::qaccessible::QAccessibleTextInterface; // 773
// use super::qaccessible::QAccessibleTableModelChangeEvent; // 773
// use super::qaccessible::QAccessibleTextCursorEvent; // 773
use super::super::core::qsize::QSize; // 771
use super::super::core::qvariant::QVariant; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto: static bool QAccessible::isActive();
  fn _ZN11QAccessible8isActiveEv() -> c_char;
  // proto: static Id QAccessible::uniqueId(QAccessibleInterface * iface);
  fn _ZN11QAccessible8uniqueIdEP20QAccessibleInterface(arg0: *mut c_void) -> c_uint;
  // proto: static Id QAccessible::registerAccessibleInterface(QAccessibleInterface * iface);
  fn _ZN11QAccessible27registerAccessibleInterfaceEP20QAccessibleInterface(arg0: *mut c_void) -> c_uint;
  // proto: static void QAccessible::setActive(bool active);
  fn _ZN11QAccessible9setActiveEb(arg0: c_char);
  // proto: static QAccessibleInterface * QAccessible::queryAccessibleInterface(QObject * );
  fn _ZN11QAccessible24queryAccessibleInterfaceEP7QObject(arg0: *mut c_void) -> *mut c_void;
  // proto: static void QAccessible::updateAccessibility(QAccessibleEvent * event);
  fn _ZN11QAccessible19updateAccessibilityEP16QAccessibleEvent(arg0: *mut c_void);
  // proto: static void QAccessible::cleanup();
  fn _ZN11QAccessible7cleanupEv();
  // proto: static void QAccessible::setRootObject(QObject * object);
  fn _ZN11QAccessible13setRootObjectEP7QObject(arg0: *mut c_void);
  // proto: static void QAccessible::deleteAccessibleInterface(Id uniqueId);
  fn _ZN11QAccessible25deleteAccessibleInterfaceEj(arg0: c_uint);
  // proto: static QAccessibleInterface * QAccessible::accessibleInterface(Id uniqueId);
  fn _ZN11QAccessible19accessibleInterfaceEj(arg0: c_uint) -> *mut c_void;
  // proto:  void QAccessible::QAccessible();
  fn _ZN11QAccessibleC1Ev(qthis: *mut c_void);
  // proto:  void QAccessibleTableModelChangeEvent::setFirstColumn(int col);
  fn _ZN32QAccessibleTableModelChangeEvent14setFirstColumnEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QAccessibleTableModelChangeEvent::setFirstRow(int row);
  fn _ZN32QAccessibleTableModelChangeEvent11setFirstRowEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QAccessibleTableModelChangeEvent::firstRow();
  fn _ZNK32QAccessibleTableModelChangeEvent8firstRowEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTableModelChangeEvent::setLastColumn(int col);
  fn _ZN32QAccessibleTableModelChangeEvent13setLastColumnEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QAccessibleTableModelChangeEvent::firstColumn();
  fn _ZNK32QAccessibleTableModelChangeEvent11firstColumnEv(qthis: *mut c_void) -> c_int;
  // proto:  int QAccessibleTableModelChangeEvent::lastColumn();
  fn _ZNK32QAccessibleTableModelChangeEvent10lastColumnEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTableModelChangeEvent::setLastRow(int row);
  fn _ZN32QAccessibleTableModelChangeEvent10setLastRowEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QAccessibleTableModelChangeEvent::lastRow();
  fn _ZNK32QAccessibleTableModelChangeEvent7lastRowEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTextInterface::selection(int selectionIndex, int * startOffset, int * endOffset);
  fn _ZNK24QAccessibleTextInterface9selectionEiPiS0_(qthis: *mut c_void, arg0: c_int, arg1: *mut c_int, arg2: *mut c_int);
  // proto:  void QAccessibleTextInterface::setCursorPosition(int position);
  fn _ZN24QAccessibleTextInterface17setCursorPositionEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QAccessibleTextInterface::offsetAtPoint(const QPoint & point);
  fn _ZNK24QAccessibleTextInterface13offsetAtPointERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QString QAccessibleTextInterface::attributes(int offset, int * startOffset, int * endOffset);
  fn _ZNK24QAccessibleTextInterface10attributesEiPiS0_(qthis: *mut c_void, arg0: c_int, arg1: *mut c_int, arg2: *mut c_int) -> *mut c_void;
  // proto:  int QAccessibleTextInterface::selectionCount();
  fn _ZNK24QAccessibleTextInterface14selectionCountEv(qthis: *mut c_void) -> c_int;
  // proto:  int QAccessibleTextInterface::characterCount();
  fn _ZNK24QAccessibleTextInterface14characterCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTextInterface::~QAccessibleTextInterface();
  fn _ZN24QAccessibleTextInterfaceD0Ev(qthis: *mut c_void);
  // proto:  QString QAccessibleTextInterface::text(int startOffset, int endOffset);
  fn _ZNK24QAccessibleTextInterface4textEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  QRect QAccessibleTextInterface::characterRect(int offset);
  fn _ZNK24QAccessibleTextInterface13characterRectEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QAccessibleTextInterface::removeSelection(int selectionIndex);
  fn _ZN24QAccessibleTextInterface15removeSelectionEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QAccessibleTextInterface::addSelection(int startOffset, int endOffset);
  fn _ZN24QAccessibleTextInterface12addSelectionEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QAccessibleTextInterface::scrollToSubstring(int startIndex, int endIndex);
  fn _ZN24QAccessibleTextInterface17scrollToSubstringEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  int QAccessibleTextInterface::cursorPosition();
  fn _ZNK24QAccessibleTextInterface14cursorPositionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTextInterface::setSelection(int selectionIndex, int startOffset, int endOffset);
  fn _ZN24QAccessibleTextInterface12setSelectionEiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int);
  // proto:  QObject * QAccessibleEvent::object();
  fn _ZNK16QAccessibleEvent6objectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleEvent::setChild(int chld);
  fn _ZN16QAccessibleEvent8setChildEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QAccessibleInterface * QAccessibleEvent::accessibleInterface();
  fn _ZNK16QAccessibleEvent19accessibleInterfaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleEvent::QAccessibleEvent(const QAccessibleEvent & );
  fn _ZN16QAccessibleEventC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QAccessibleEvent::child();
  fn _ZNK16QAccessibleEvent5childEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleEvent::~QAccessibleEvent();
  fn _ZN16QAccessibleEventD0Ev(qthis: *mut c_void);
  // proto: static QString QAccessibleActionInterface::scrollUpAction();
  fn _ZN26QAccessibleActionInterface14scrollUpActionEv() -> *mut c_void;
  // proto:  QStringList QAccessibleActionInterface::actionNames();
  fn _ZNK26QAccessibleActionInterface11actionNamesEv(qthis: *mut c_void);
  // proto: static const QString & QAccessibleActionInterface::decreaseAction();
  fn _ZN26QAccessibleActionInterface14decreaseActionEv() -> *mut c_void;
  // proto: static const QString & QAccessibleActionInterface::toggleAction();
  fn _ZN26QAccessibleActionInterface12toggleActionEv() -> *mut c_void;
  // proto:  QString QAccessibleActionInterface::localizedActionName(const QString & name);
  fn _ZNK26QAccessibleActionInterface19localizedActionNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QAccessibleActionInterface::localizedActionDescription(const QString & name);
  fn _ZNK26QAccessibleActionInterface26localizedActionDescriptionERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto: static QString QAccessibleActionInterface::scrollLeftAction();
  fn _ZN26QAccessibleActionInterface16scrollLeftActionEv() -> *mut c_void;
  // proto: static QString QAccessibleActionInterface::previousPageAction();
  fn _ZN26QAccessibleActionInterface18previousPageActionEv() -> *mut c_void;
  // proto: static const QString & QAccessibleActionInterface::showMenuAction();
  fn _ZN26QAccessibleActionInterface14showMenuActionEv() -> *mut c_void;
  // proto: static QString QAccessibleActionInterface::scrollRightAction();
  fn _ZN26QAccessibleActionInterface17scrollRightActionEv() -> *mut c_void;
  // proto: static const QString & QAccessibleActionInterface::setFocusAction();
  fn _ZN26QAccessibleActionInterface14setFocusActionEv() -> *mut c_void;
  // proto: static QString QAccessibleActionInterface::nextPageAction();
  fn _ZN26QAccessibleActionInterface14nextPageActionEv() -> *mut c_void;
  // proto:  void QAccessibleActionInterface::~QAccessibleActionInterface();
  fn _ZN26QAccessibleActionInterfaceD0Ev(qthis: *mut c_void);
  // proto: static const QString & QAccessibleActionInterface::pressAction();
  fn _ZN26QAccessibleActionInterface11pressActionEv() -> *mut c_void;
  // proto:  void QAccessibleActionInterface::doAction(const QString & actionName);
  fn _ZN26QAccessibleActionInterface8doActionERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static const QString & QAccessibleActionInterface::increaseAction();
  fn _ZN26QAccessibleActionInterface14increaseActionEv() -> *mut c_void;
  // proto:  QStringList QAccessibleActionInterface::keyBindingsForAction(const QString & actionName);
  fn _ZNK26QAccessibleActionInterface20keyBindingsForActionERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QString QAccessibleActionInterface::scrollDownAction();
  fn _ZN26QAccessibleActionInterface16scrollDownActionEv() -> *mut c_void;
  // proto:  QAccessibleImageInterface * QAccessibleInterface::imageInterface();
  fn _ZN20QAccessibleInterface14imageInterfaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleTableInterface * QAccessibleInterface::tableInterface();
  fn _ZN20QAccessibleInterface14tableInterfaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleEditableTextInterface * QAccessibleInterface::editableTextInterface();
  fn _ZN20QAccessibleInterface21editableTextInterfaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleValueInterface * QAccessibleInterface::valueInterface();
  fn _ZN20QAccessibleInterface14valueInterfaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRect QAccessibleInterface::rect();
  fn _ZNK20QAccessibleInterface4rectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QObject * QAccessibleInterface::object();
  fn _ZNK20QAccessibleInterface6objectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleActionInterface * QAccessibleInterface::actionInterface();
  fn _ZN20QAccessibleInterface15actionInterfaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleInterface * QAccessibleInterface::parent();
  fn _ZNK20QAccessibleInterface6parentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleInterface * QAccessibleInterface::childAt(int x, int y);
  fn _ZNK20QAccessibleInterface7childAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  int QAccessibleInterface::childCount();
  fn _ZNK20QAccessibleInterface10childCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QAccessibleTableCellInterface * QAccessibleInterface::tableCellInterface();
  fn _ZN20QAccessibleInterface18tableCellInterfaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QAccessibleInterface::indexOfChild(const QAccessibleInterface * );
  fn _ZNK20QAccessibleInterface12indexOfChildEPKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QColor QAccessibleInterface::foregroundColor();
  fn _ZNK20QAccessibleInterface15foregroundColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QAccessibleInterface::isValid();
  fn _ZNK20QAccessibleInterface7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  QWindow * QAccessibleInterface::window();
  fn _ZNK20QAccessibleInterface6windowEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleInterface::virtual_hook(int id, void * data);
  fn _ZN20QAccessibleInterface12virtual_hookEiPv(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  QAccessibleInterface * QAccessibleInterface::focusChild();
  fn _ZNK20QAccessibleInterface10focusChildEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleInterface * QAccessibleInterface::child(int index);
  fn _ZNK20QAccessibleInterface5childEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QAccessibleTextInterface * QAccessibleInterface::textInterface();
  fn _ZN20QAccessibleInterface13textInterfaceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QColor QAccessibleInterface::backgroundColor();
  fn _ZNK20QAccessibleInterface15backgroundColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleInterface::~QAccessibleInterface();
  fn _ZN20QAccessibleInterfaceD0Ev(qthis: *mut c_void);
  // proto:  void QAccessibleEditableTextInterface::insertText(int offset, const QString & text);
  fn _ZN32QAccessibleEditableTextInterface10insertTextEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QAccessibleEditableTextInterface::replaceText(int startOffset, int endOffset, const QString & text);
  fn _ZN32QAccessibleEditableTextInterface11replaceTextEiiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void);
  // proto:  void QAccessibleEditableTextInterface::deleteText(int startOffset, int endOffset);
  fn _ZN32QAccessibleEditableTextInterface10deleteTextEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QAccessibleEditableTextInterface::~QAccessibleEditableTextInterface();
  fn _ZN32QAccessibleEditableTextInterfaceD0Ev(qthis: *mut c_void);
  // proto:  int QAccessibleTableCellInterface::columnIndex();
  fn _ZNK29QAccessibleTableCellInterface11columnIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTableCellInterface::~QAccessibleTableCellInterface();
  fn _ZN29QAccessibleTableCellInterfaceD0Ev(qthis: *mut c_void);
  // proto:  int QAccessibleTableCellInterface::columnExtent();
  fn _ZNK29QAccessibleTableCellInterface12columnExtentEv(qthis: *mut c_void) -> c_int;
  // proto:  int QAccessibleTableCellInterface::rowIndex();
  fn _ZNK29QAccessibleTableCellInterface8rowIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  QAccessibleInterface * QAccessibleTableCellInterface::table();
  fn _ZNK29QAccessibleTableCellInterface5tableEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QAccessibleTableCellInterface::rowExtent();
  fn _ZNK29QAccessibleTableCellInterface9rowExtentEv(qthis: *mut c_void) -> c_int;
  // proto:  QList<QAccessibleInterface *> QAccessibleTableCellInterface::rowHeaderCells();
  fn _ZNK29QAccessibleTableCellInterface14rowHeaderCellsEv(qthis: *mut c_void);
  // proto:  QList<QAccessibleInterface *> QAccessibleTableCellInterface::columnHeaderCells();
  fn _ZNK29QAccessibleTableCellInterface17columnHeaderCellsEv(qthis: *mut c_void);
  // proto:  bool QAccessibleTableCellInterface::isSelected();
  fn _ZNK29QAccessibleTableCellInterface10isSelectedEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QAccessibleTableInterface::unselectColumn(int column);
  fn _ZN25QAccessibleTableInterface14unselectColumnEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  QString QAccessibleTableInterface::columnDescription(int column);
  fn _ZNK25QAccessibleTableInterface17columnDescriptionEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QAccessibleTableInterface::selectedCellCount();
  fn _ZNK25QAccessibleTableInterface17selectedCellCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QList<QAccessibleInterface *> QAccessibleTableInterface::selectedCells();
  fn _ZNK25QAccessibleTableInterface13selectedCellsEv(qthis: *mut c_void);
  // proto:  bool QAccessibleTableInterface::selectRow(int row);
  fn _ZN25QAccessibleTableInterface9selectRowEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  int QAccessibleTableInterface::selectedRowCount();
  fn _ZNK25QAccessibleTableInterface16selectedRowCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTableInterface::~QAccessibleTableInterface();
  fn _ZN25QAccessibleTableInterfaceD0Ev(qthis: *mut c_void);
  // proto:  QList<int> QAccessibleTableInterface::selectedColumns();
  fn _ZNK25QAccessibleTableInterface15selectedColumnsEv(qthis: *mut c_void);
  // proto:  QAccessibleInterface * QAccessibleTableInterface::cellAt(int row, int column);
  fn _ZNK25QAccessibleTableInterface6cellAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  QList<int> QAccessibleTableInterface::selectedRows();
  fn _ZNK25QAccessibleTableInterface12selectedRowsEv(qthis: *mut c_void);
  // proto:  void QAccessibleTableInterface::modelChange(QAccessibleTableModelChangeEvent * event);
  fn _ZN25QAccessibleTableInterface11modelChangeEP32QAccessibleTableModelChangeEvent(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QAccessibleTableInterface::columnCount();
  fn _ZNK25QAccessibleTableInterface11columnCountEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QAccessibleTableInterface::selectColumn(int column);
  fn _ZN25QAccessibleTableInterface12selectColumnEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  bool QAccessibleTableInterface::unselectRow(int row);
  fn _ZN25QAccessibleTableInterface11unselectRowEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  int QAccessibleTableInterface::rowCount();
  fn _ZNK25QAccessibleTableInterface8rowCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QAccessibleTableInterface::rowDescription(int row);
  fn _ZNK25QAccessibleTableInterface14rowDescriptionEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QAccessibleInterface * QAccessibleTableInterface::summary();
  fn _ZNK25QAccessibleTableInterface7summaryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QAccessibleTableInterface::isColumnSelected(int column);
  fn _ZNK25QAccessibleTableInterface16isColumnSelectedEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  QAccessibleInterface * QAccessibleTableInterface::caption();
  fn _ZNK25QAccessibleTableInterface7captionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QAccessibleTableInterface::isRowSelected(int row);
  fn _ZNK25QAccessibleTableInterface13isRowSelectedEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  int QAccessibleTableInterface::selectedColumnCount();
  fn _ZNK25QAccessibleTableInterface19selectedColumnCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QAccessibleTextUpdateEvent::textInserted();
  fn _ZNK26QAccessibleTextUpdateEvent12textInsertedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleTextUpdateEvent::QAccessibleTextUpdateEvent(QAccessibleInterface * iface, int position, const QString & oldText, const QString & text);
  fn _ZN26QAccessibleTextUpdateEventC1EP20QAccessibleInterfaceiRK7QStringS4_(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void, arg3: *mut c_void);
  // proto:  QString QAccessibleTextUpdateEvent::textRemoved();
  fn _ZNK26QAccessibleTextUpdateEvent11textRemovedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QAccessibleTextUpdateEvent::changePosition();
  fn _ZNK26QAccessibleTextUpdateEvent14changePositionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTextUpdateEvent::QAccessibleTextUpdateEvent(QObject * obj, int position, const QString & oldText, const QString & text);
  fn _ZN26QAccessibleTextUpdateEventC1EP7QObjectiRK7QStringS4_(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void, arg3: *mut c_void);
  // proto:  QString QAccessibleImageInterface::imageDescription();
  fn _ZNK25QAccessibleImageInterface16imageDescriptionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QAccessibleImageInterface::imagePosition();
  fn _ZNK25QAccessibleImageInterface13imagePositionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleImageInterface::~QAccessibleImageInterface();
  fn _ZN25QAccessibleImageInterfaceD0Ev(qthis: *mut c_void);
  // proto:  QSize QAccessibleImageInterface::imageSize();
  fn _ZNK25QAccessibleImageInterface9imageSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QAccessibleTextInsertEvent::textInserted();
  fn _ZNK26QAccessibleTextInsertEvent12textInsertedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QAccessibleTextInsertEvent::changePosition();
  fn _ZNK26QAccessibleTextInsertEvent14changePositionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTextInsertEvent::QAccessibleTextInsertEvent(QAccessibleInterface * iface, int position, const QString & text);
  fn _ZN26QAccessibleTextInsertEventC1EP20QAccessibleInterfaceiRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void);
  // proto:  void QAccessibleTextInsertEvent::QAccessibleTextInsertEvent(QObject * obj, int position, const QString & text);
  fn _ZN26QAccessibleTextInsertEventC1EP7QObjectiRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void);
  // proto:  QVariant QAccessibleValueInterface::maximumValue();
  fn _ZNK25QAccessibleValueInterface12maximumValueEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QVariant QAccessibleValueInterface::minimumStepSize();
  fn _ZNK25QAccessibleValueInterface15minimumStepSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QVariant QAccessibleValueInterface::currentValue();
  fn _ZNK25QAccessibleValueInterface12currentValueEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QVariant QAccessibleValueInterface::minimumValue();
  fn _ZNK25QAccessibleValueInterface12minimumValueEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleValueInterface::~QAccessibleValueInterface();
  fn _ZN25QAccessibleValueInterfaceD0Ev(qthis: *mut c_void);
  // proto:  void QAccessibleValueInterface::setCurrentValue(const QVariant & value);
  fn _ZN25QAccessibleValueInterface15setCurrentValueERK8QVariant(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAccessibleTextRemoveEvent::QAccessibleTextRemoveEvent(QObject * obj, int position, const QString & text);
  fn _ZN26QAccessibleTextRemoveEventC1EP7QObjectiRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void);
  // proto:  QString QAccessibleTextRemoveEvent::textRemoved();
  fn _ZNK26QAccessibleTextRemoveEvent11textRemovedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleTextRemoveEvent::QAccessibleTextRemoveEvent(QAccessibleInterface * iface, int position, const QString & text);
  fn _ZN26QAccessibleTextRemoveEventC1EP20QAccessibleInterfaceiRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void);
  // proto:  int QAccessibleTextRemoveEvent::changePosition();
  fn _ZNK26QAccessibleTextRemoveEvent14changePositionEv(qthis: *mut c_void) -> c_int;
  // proto:  int QAccessibleTextSelectionEvent::selectionEnd();
  fn _ZNK29QAccessibleTextSelectionEvent12selectionEndEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTextSelectionEvent::QAccessibleTextSelectionEvent(QAccessibleInterface * iface, int start, int end);
  fn _ZN29QAccessibleTextSelectionEventC1EP20QAccessibleInterfaceii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  int QAccessibleTextSelectionEvent::selectionStart();
  fn _ZNK29QAccessibleTextSelectionEvent14selectionStartEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTextSelectionEvent::QAccessibleTextSelectionEvent(QObject * obj, int start, int end);
  fn _ZN29QAccessibleTextSelectionEventC1EP7QObjectii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  void QAccessibleTextSelectionEvent::setSelection(int start, int end);
  fn _ZN29QAccessibleTextSelectionEvent12setSelectionEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QAccessibleTextCursorEvent::QAccessibleTextCursorEvent(QAccessibleInterface * iface, int cursorPos);
  fn _ZN26QAccessibleTextCursorEventC1EP20QAccessibleInterfacei(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QAccessibleTextCursorEvent::setCursorPosition(int position);
  fn _ZN26QAccessibleTextCursorEvent17setCursorPositionEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QAccessibleTextCursorEvent::cursorPosition();
  fn _ZNK26QAccessibleTextCursorEvent14cursorPositionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTextCursorEvent::QAccessibleTextCursorEvent(QObject * obj, int cursorPos);
  fn _ZN26QAccessibleTextCursorEventC1EP7QObjecti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QAccessibleValueChangeEvent::QAccessibleValueChangeEvent(QObject * obj, const QVariant & val);
  fn _ZN27QAccessibleValueChangeEventC1EP7QObjectRK8QVariant(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QAccessibleValueChangeEvent::QAccessibleValueChangeEvent(QAccessibleInterface * iface, const QVariant & val);
  fn _ZN27QAccessibleValueChangeEventC1EP20QAccessibleInterfaceRK8QVariant(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QAccessibleValueChangeEvent::setValue(const QVariant & val);
  fn _ZN27QAccessibleValueChangeEvent8setValueERK8QVariant(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QVariant QAccessibleValueChangeEvent::value();
  fn _ZNK27QAccessibleValueChangeEvent5valueEv(qthis: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QAccessible)=1
pub struct QAccessible {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QAccessibleTableModelChangeEvent)=48
pub struct QAccessibleTableModelChangeEvent {
  qbase: QAccessibleEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QAccessibleTextInterface)=8
pub struct QAccessibleTextInterface {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QAccessibleEvent)=32
pub struct QAccessibleEvent {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QAccessibleActionInterface)=8
pub struct QAccessibleActionInterface {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QAccessibleInterface)=8
pub struct QAccessibleInterface {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QAccessibleEditableTextInterface)=8
pub struct QAccessibleEditableTextInterface {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QAccessibleTableCellInterface)=8
pub struct QAccessibleTableCellInterface {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QAccessibleTableInterface)=8
pub struct QAccessibleTableInterface {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QAccessibleTextUpdateEvent)=56
pub struct QAccessibleTextUpdateEvent {
  qbase: QAccessibleTextCursorEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QAccessibleStateChangeEvent)=40
pub struct QAccessibleStateChangeEvent {
  qbase: QAccessibleEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QAccessibleImageInterface)=8
pub struct QAccessibleImageInterface {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QAccessibleTextInsertEvent)=48
pub struct QAccessibleTextInsertEvent {
  qbase: QAccessibleTextCursorEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QAccessibleValueInterface)=8
pub struct QAccessibleValueInterface {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QAccessibleTextRemoveEvent)=48
pub struct QAccessibleTextRemoveEvent {
  qbase: QAccessibleTextCursorEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QAccessibleTextSelectionEvent)=40
pub struct QAccessibleTextSelectionEvent {
  qbase: QAccessibleTextCursorEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QAccessibleTextCursorEvent)=32
pub struct QAccessibleTextCursorEvent {
  qbase: QAccessibleEvent,
  pub qclsinst: *mut c_void,
}

// class sizeof(QAccessibleValueChangeEvent)=48
pub struct QAccessibleValueChangeEvent {
  qbase: QAccessibleEvent,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessible {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessible {
    return QAccessible{qclsinst: qthis};
  }
}
  // proto: static bool QAccessible::isActive();
impl /*struct*/ QAccessible {
  pub fn isActive_s<RetType, T: QAccessible_isActive_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isActive_s();
    // return 1;
  }
}

pub trait QAccessible_isActive_s<RetType> {
  fn isActive_s(self ) -> RetType;
}

  // proto: static bool QAccessible::isActive();
impl<'a> /*trait*/ QAccessible_isActive_s<i8> for () {
  fn isActive_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible8isActiveEv()};
    let mut ret = unsafe {_ZN11QAccessible8isActiveEv()};
    return ret as i8;
    // return 1;
  }
}

  // proto: static Id QAccessible::uniqueId(QAccessibleInterface * iface);
impl /*struct*/ QAccessible {
  pub fn uniqueId_s<RetType, T: QAccessible_uniqueId_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.uniqueId_s();
    // return 1;
  }
}

pub trait QAccessible_uniqueId_s<RetType> {
  fn uniqueId_s(self ) -> RetType;
}

  // proto: static Id QAccessible::uniqueId(QAccessibleInterface * iface);
impl<'a> /*trait*/ QAccessible_uniqueId_s<u32> for (QAccessibleInterface) {
  fn uniqueId_s(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible8uniqueIdEP20QAccessibleInterface()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QAccessible8uniqueIdEP20QAccessibleInterface(arg0)};
    return ret as u32;
    // return 1;
  }
}

  // proto: static Id QAccessible::registerAccessibleInterface(QAccessibleInterface * iface);
impl /*struct*/ QAccessible {
  pub fn registerAccessibleInterface_s<RetType, T: QAccessible_registerAccessibleInterface_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.registerAccessibleInterface_s();
    // return 1;
  }
}

pub trait QAccessible_registerAccessibleInterface_s<RetType> {
  fn registerAccessibleInterface_s(self ) -> RetType;
}

  // proto: static Id QAccessible::registerAccessibleInterface(QAccessibleInterface * iface);
impl<'a> /*trait*/ QAccessible_registerAccessibleInterface_s<u32> for (QAccessibleInterface) {
  fn registerAccessibleInterface_s(self ) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible27registerAccessibleInterfaceEP20QAccessibleInterface()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QAccessible27registerAccessibleInterfaceEP20QAccessibleInterface(arg0)};
    return ret as u32;
    // return 1;
  }
}

  // proto: static void QAccessible::setActive(bool active);
impl /*struct*/ QAccessible {
  pub fn setActive_s<RetType, T: QAccessible_setActive_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setActive_s();
    // return 1;
  }
}

pub trait QAccessible_setActive_s<RetType> {
  fn setActive_s(self ) -> RetType;
}

  // proto: static void QAccessible::setActive(bool active);
impl<'a> /*trait*/ QAccessible_setActive_s<()> for (i8) {
  fn setActive_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible9setActiveEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QAccessible9setActiveEb(arg0)};
    // return 1;
  }
}

  // proto: static QAccessibleInterface * QAccessible::queryAccessibleInterface(QObject * );
impl /*struct*/ QAccessible {
  pub fn queryAccessibleInterface_s<RetType, T: QAccessible_queryAccessibleInterface_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.queryAccessibleInterface_s();
    // return 1;
  }
}

pub trait QAccessible_queryAccessibleInterface_s<RetType> {
  fn queryAccessibleInterface_s(self ) -> RetType;
}

  // proto: static QAccessibleInterface * QAccessible::queryAccessibleInterface(QObject * );
impl<'a> /*trait*/ QAccessible_queryAccessibleInterface_s<QAccessibleInterface> for (QObject) {
  fn queryAccessibleInterface_s(self ) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible24queryAccessibleInterfaceEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QAccessible24queryAccessibleInterfaceEP7QObject(arg0)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static void QAccessible::updateAccessibility(QAccessibleEvent * event);
impl /*struct*/ QAccessible {
  pub fn updateAccessibility_s<RetType, T: QAccessible_updateAccessibility_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.updateAccessibility_s();
    // return 1;
  }
}

pub trait QAccessible_updateAccessibility_s<RetType> {
  fn updateAccessibility_s(self ) -> RetType;
}

  // proto: static void QAccessible::updateAccessibility(QAccessibleEvent * event);
impl<'a> /*trait*/ QAccessible_updateAccessibility_s<()> for (QAccessibleEvent) {
  fn updateAccessibility_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible19updateAccessibilityEP16QAccessibleEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QAccessible19updateAccessibilityEP16QAccessibleEvent(arg0)};
    // return 1;
  }
}

  // proto: static void QAccessible::cleanup();
impl /*struct*/ QAccessible {
  pub fn cleanup_s<RetType, T: QAccessible_cleanup_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.cleanup_s();
    // return 1;
  }
}

pub trait QAccessible_cleanup_s<RetType> {
  fn cleanup_s(self ) -> RetType;
}

  // proto: static void QAccessible::cleanup();
impl<'a> /*trait*/ QAccessible_cleanup_s<()> for () {
  fn cleanup_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible7cleanupEv()};
     unsafe {_ZN11QAccessible7cleanupEv()};
    // return 1;
  }
}

  // proto: static void QAccessible::setRootObject(QObject * object);
impl /*struct*/ QAccessible {
  pub fn setRootObject_s<RetType, T: QAccessible_setRootObject_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setRootObject_s();
    // return 1;
  }
}

pub trait QAccessible_setRootObject_s<RetType> {
  fn setRootObject_s(self ) -> RetType;
}

  // proto: static void QAccessible::setRootObject(QObject * object);
impl<'a> /*trait*/ QAccessible_setRootObject_s<()> for (QObject) {
  fn setRootObject_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible13setRootObjectEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QAccessible13setRootObjectEP7QObject(arg0)};
    // return 1;
  }
}

  // proto: static void QAccessible::deleteAccessibleInterface(Id uniqueId);
impl /*struct*/ QAccessible {
  pub fn deleteAccessibleInterface_s<RetType, T: QAccessible_deleteAccessibleInterface_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.deleteAccessibleInterface_s();
    // return 1;
  }
}

pub trait QAccessible_deleteAccessibleInterface_s<RetType> {
  fn deleteAccessibleInterface_s(self ) -> RetType;
}

  // proto: static void QAccessible::deleteAccessibleInterface(Id uniqueId);
impl<'a> /*trait*/ QAccessible_deleteAccessibleInterface_s<()> for (u32) {
  fn deleteAccessibleInterface_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible25deleteAccessibleInterfaceEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN11QAccessible25deleteAccessibleInterfaceEj(arg0)};
    // return 1;
  }
}

  // proto: static QAccessibleInterface * QAccessible::accessibleInterface(Id uniqueId);
impl /*struct*/ QAccessible {
  pub fn accessibleInterface_s<RetType, T: QAccessible_accessibleInterface_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.accessibleInterface_s();
    // return 1;
  }
}

pub trait QAccessible_accessibleInterface_s<RetType> {
  fn accessibleInterface_s(self ) -> RetType;
}

  // proto: static QAccessibleInterface * QAccessible::accessibleInterface(Id uniqueId);
impl<'a> /*trait*/ QAccessible_accessibleInterface_s<QAccessibleInterface> for (u32) {
  fn accessibleInterface_s(self ) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessible19accessibleInterfaceEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZN11QAccessible19accessibleInterfaceEj(arg0)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAccessible::QAccessible();
impl /*struct*/ QAccessible {
  pub fn NewQAccessible<T: QAccessible_NewQAccessible>(value: T) -> QAccessible {
    let rsthis = value.NewQAccessible();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessible_NewQAccessible {
  fn NewQAccessible(self) -> QAccessible;
}

  // proto:  void QAccessible::QAccessible();
impl<'a> /*trait*/ QAccessible_NewQAccessible for () {
  fn NewQAccessible(self) -> QAccessible {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QAccessibleC1Ev()};
    unsafe {_ZN11QAccessibleC1Ev(qthis)};
    let rsthis = QAccessible{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleTableModelChangeEvent {
    return QAccessibleTableModelChangeEvent{qbase: QAccessibleEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QAccessibleTableModelChangeEvent {
  type Target = QAccessibleEvent;

  fn deref(&self) -> &QAccessibleEvent {
    return &self.qbase;
  }
}
impl AsRef<QAccessibleEvent> for QAccessibleTableModelChangeEvent {
  fn as_ref(&self) -> &QAccessibleEvent {
    return &self.qbase;
  }
}
  // proto:  void QAccessibleTableModelChangeEvent::setFirstColumn(int col);
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setFirstColumn<RetType, T: QAccessibleTableModelChangeEvent_setFirstColumn<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFirstColumn(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_setFirstColumn<RetType> {
  fn setFirstColumn(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> RetType;
}

  // proto:  void QAccessibleTableModelChangeEvent::setFirstColumn(int col);
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setFirstColumn<()> for (i32) {
  fn setFirstColumn(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN32QAccessibleTableModelChangeEvent14setFirstColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN32QAccessibleTableModelChangeEvent14setFirstColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAccessibleTableModelChangeEvent::setFirstRow(int row);
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setFirstRow<RetType, T: QAccessibleTableModelChangeEvent_setFirstRow<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFirstRow(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_setFirstRow<RetType> {
  fn setFirstRow(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> RetType;
}

  // proto:  void QAccessibleTableModelChangeEvent::setFirstRow(int row);
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setFirstRow<()> for (i32) {
  fn setFirstRow(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN32QAccessibleTableModelChangeEvent11setFirstRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN32QAccessibleTableModelChangeEvent11setFirstRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QAccessibleTableModelChangeEvent::firstRow();
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn firstRow<RetType, T: QAccessibleTableModelChangeEvent_firstRow<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.firstRow(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_firstRow<RetType> {
  fn firstRow(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> RetType;
}

  // proto:  int QAccessibleTableModelChangeEvent::firstRow();
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_firstRow<i32> for () {
  fn firstRow(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK32QAccessibleTableModelChangeEvent8firstRowEv()};
    let mut ret = unsafe {_ZNK32QAccessibleTableModelChangeEvent8firstRowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAccessibleTableModelChangeEvent::setLastColumn(int col);
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setLastColumn<RetType, T: QAccessibleTableModelChangeEvent_setLastColumn<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setLastColumn(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_setLastColumn<RetType> {
  fn setLastColumn(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> RetType;
}

  // proto:  void QAccessibleTableModelChangeEvent::setLastColumn(int col);
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setLastColumn<()> for (i32) {
  fn setLastColumn(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN32QAccessibleTableModelChangeEvent13setLastColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN32QAccessibleTableModelChangeEvent13setLastColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QAccessibleTableModelChangeEvent::firstColumn();
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn firstColumn<RetType, T: QAccessibleTableModelChangeEvent_firstColumn<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.firstColumn(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_firstColumn<RetType> {
  fn firstColumn(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> RetType;
}

  // proto:  int QAccessibleTableModelChangeEvent::firstColumn();
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_firstColumn<i32> for () {
  fn firstColumn(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK32QAccessibleTableModelChangeEvent11firstColumnEv()};
    let mut ret = unsafe {_ZNK32QAccessibleTableModelChangeEvent11firstColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QAccessibleTableModelChangeEvent::lastColumn();
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn lastColumn<RetType, T: QAccessibleTableModelChangeEvent_lastColumn<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lastColumn(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_lastColumn<RetType> {
  fn lastColumn(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> RetType;
}

  // proto:  int QAccessibleTableModelChangeEvent::lastColumn();
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_lastColumn<i32> for () {
  fn lastColumn(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK32QAccessibleTableModelChangeEvent10lastColumnEv()};
    let mut ret = unsafe {_ZNK32QAccessibleTableModelChangeEvent10lastColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAccessibleTableModelChangeEvent::setLastRow(int row);
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn setLastRow<RetType, T: QAccessibleTableModelChangeEvent_setLastRow<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setLastRow(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_setLastRow<RetType> {
  fn setLastRow(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> RetType;
}

  // proto:  void QAccessibleTableModelChangeEvent::setLastRow(int row);
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_setLastRow<()> for (i32) {
  fn setLastRow(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN32QAccessibleTableModelChangeEvent10setLastRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN32QAccessibleTableModelChangeEvent10setLastRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QAccessibleTableModelChangeEvent::lastRow();
impl /*struct*/ QAccessibleTableModelChangeEvent {
  pub fn lastRow<RetType, T: QAccessibleTableModelChangeEvent_lastRow<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lastRow(self);
    // return 1;
  }
}

pub trait QAccessibleTableModelChangeEvent_lastRow<RetType> {
  fn lastRow(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> RetType;
}

  // proto:  int QAccessibleTableModelChangeEvent::lastRow();
impl<'a> /*trait*/ QAccessibleTableModelChangeEvent_lastRow<i32> for () {
  fn lastRow(self , rsthis: &mut QAccessibleTableModelChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK32QAccessibleTableModelChangeEvent7lastRowEv()};
    let mut ret = unsafe {_ZNK32QAccessibleTableModelChangeEvent7lastRowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextInterface {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleTextInterface {
    return QAccessibleTextInterface{qclsinst: qthis};
  }
}
  // proto:  void QAccessibleTextInterface::selection(int selectionIndex, int * startOffset, int * endOffset);
impl /*struct*/ QAccessibleTextInterface {
  pub fn selection<RetType, T: QAccessibleTextInterface_selection<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selection(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_selection<RetType> {
  fn selection(self , rsthis: &mut QAccessibleTextInterface) -> RetType;
}

  // proto:  void QAccessibleTextInterface::selection(int selectionIndex, int * startOffset, int * endOffset);
impl<'a> /*trait*/ QAccessibleTextInterface_selection<()> for (i32, &'a mut Vec<i32>, &'a mut Vec<i32>) {
  fn selection(self , rsthis: &mut QAccessibleTextInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface9selectionEiPiS0_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
     unsafe {_ZNK24QAccessibleTextInterface9selectionEiPiS0_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QAccessibleTextInterface::setCursorPosition(int position);
impl /*struct*/ QAccessibleTextInterface {
  pub fn setCursorPosition<RetType, T: QAccessibleTextInterface_setCursorPosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCursorPosition(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_setCursorPosition<RetType> {
  fn setCursorPosition(self , rsthis: &mut QAccessibleTextInterface) -> RetType;
}

  // proto:  void QAccessibleTextInterface::setCursorPosition(int position);
impl<'a> /*trait*/ QAccessibleTextInterface_setCursorPosition<()> for (i32) {
  fn setCursorPosition(self , rsthis: &mut QAccessibleTextInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAccessibleTextInterface17setCursorPositionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN24QAccessibleTextInterface17setCursorPositionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QAccessibleTextInterface::offsetAtPoint(const QPoint & point);
impl /*struct*/ QAccessibleTextInterface {
  pub fn offsetAtPoint<RetType, T: QAccessibleTextInterface_offsetAtPoint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.offsetAtPoint(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_offsetAtPoint<RetType> {
  fn offsetAtPoint(self , rsthis: &mut QAccessibleTextInterface) -> RetType;
}

  // proto:  int QAccessibleTextInterface::offsetAtPoint(const QPoint & point);
impl<'a> /*trait*/ QAccessibleTextInterface_offsetAtPoint<i32> for (QPoint) {
  fn offsetAtPoint(self , rsthis: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface13offsetAtPointERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK24QAccessibleTextInterface13offsetAtPointERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString QAccessibleTextInterface::attributes(int offset, int * startOffset, int * endOffset);
impl /*struct*/ QAccessibleTextInterface {
  pub fn attributes<RetType, T: QAccessibleTextInterface_attributes<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.attributes(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_attributes<RetType> {
  fn attributes(self , rsthis: &mut QAccessibleTextInterface) -> RetType;
}

  // proto:  QString QAccessibleTextInterface::attributes(int offset, int * startOffset, int * endOffset);
impl<'a> /*trait*/ QAccessibleTextInterface_attributes<QString> for (i32, &'a mut Vec<i32>, &'a mut Vec<i32>) {
  fn attributes(self , rsthis: &mut QAccessibleTextInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface10attributesEiPiS0_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.as_ptr()  as *mut c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
    let mut ret = unsafe {_ZNK24QAccessibleTextInterface10attributesEiPiS0_(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QAccessibleTextInterface::selectionCount();
impl /*struct*/ QAccessibleTextInterface {
  pub fn selectionCount<RetType, T: QAccessibleTextInterface_selectionCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectionCount(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_selectionCount<RetType> {
  fn selectionCount(self , rsthis: &mut QAccessibleTextInterface) -> RetType;
}

  // proto:  int QAccessibleTextInterface::selectionCount();
impl<'a> /*trait*/ QAccessibleTextInterface_selectionCount<i32> for () {
  fn selectionCount(self , rsthis: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface14selectionCountEv()};
    let mut ret = unsafe {_ZNK24QAccessibleTextInterface14selectionCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QAccessibleTextInterface::characterCount();
impl /*struct*/ QAccessibleTextInterface {
  pub fn characterCount<RetType, T: QAccessibleTextInterface_characterCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.characterCount(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_characterCount<RetType> {
  fn characterCount(self , rsthis: &mut QAccessibleTextInterface) -> RetType;
}

  // proto:  int QAccessibleTextInterface::characterCount();
impl<'a> /*trait*/ QAccessibleTextInterface_characterCount<i32> for () {
  fn characterCount(self , rsthis: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface14characterCountEv()};
    let mut ret = unsafe {_ZNK24QAccessibleTextInterface14characterCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAccessibleTextInterface::~QAccessibleTextInterface();
impl /*struct*/ QAccessibleTextInterface {
  pub fn FreeQAccessibleTextInterface<RetType, T: QAccessibleTextInterface_FreeQAccessibleTextInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleTextInterface(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_FreeQAccessibleTextInterface<RetType> {
  fn FreeQAccessibleTextInterface(self , rsthis: &mut QAccessibleTextInterface) -> RetType;
}

  // proto:  void QAccessibleTextInterface::~QAccessibleTextInterface();
impl<'a> /*trait*/ QAccessibleTextInterface_FreeQAccessibleTextInterface<()> for () {
  fn FreeQAccessibleTextInterface(self , rsthis: &mut QAccessibleTextInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAccessibleTextInterfaceD0Ev()};
     unsafe {_ZN24QAccessibleTextInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QAccessibleTextInterface::text(int startOffset, int endOffset);
impl /*struct*/ QAccessibleTextInterface {
  pub fn text<RetType, T: QAccessibleTextInterface_text<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_text<RetType> {
  fn text(self , rsthis: &mut QAccessibleTextInterface) -> RetType;
}

  // proto:  QString QAccessibleTextInterface::text(int startOffset, int endOffset);
impl<'a> /*trait*/ QAccessibleTextInterface_text<QString> for (i32, i32) {
  fn text(self , rsthis: &mut QAccessibleTextInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface4textEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK24QAccessibleTextInterface4textEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QAccessibleTextInterface::characterRect(int offset);
impl /*struct*/ QAccessibleTextInterface {
  pub fn characterRect<RetType, T: QAccessibleTextInterface_characterRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.characterRect(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_characterRect<RetType> {
  fn characterRect(self , rsthis: &mut QAccessibleTextInterface) -> RetType;
}

  // proto:  QRect QAccessibleTextInterface::characterRect(int offset);
impl<'a> /*trait*/ QAccessibleTextInterface_characterRect<QRect> for (i32) {
  fn characterRect(self , rsthis: &mut QAccessibleTextInterface) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface13characterRectEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK24QAccessibleTextInterface13characterRectEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAccessibleTextInterface::removeSelection(int selectionIndex);
impl /*struct*/ QAccessibleTextInterface {
  pub fn removeSelection<RetType, T: QAccessibleTextInterface_removeSelection<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removeSelection(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_removeSelection<RetType> {
  fn removeSelection(self , rsthis: &mut QAccessibleTextInterface) -> RetType;
}

  // proto:  void QAccessibleTextInterface::removeSelection(int selectionIndex);
impl<'a> /*trait*/ QAccessibleTextInterface_removeSelection<()> for (i32) {
  fn removeSelection(self , rsthis: &mut QAccessibleTextInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAccessibleTextInterface15removeSelectionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN24QAccessibleTextInterface15removeSelectionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAccessibleTextInterface::addSelection(int startOffset, int endOffset);
impl /*struct*/ QAccessibleTextInterface {
  pub fn addSelection<RetType, T: QAccessibleTextInterface_addSelection<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addSelection(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_addSelection<RetType> {
  fn addSelection(self , rsthis: &mut QAccessibleTextInterface) -> RetType;
}

  // proto:  void QAccessibleTextInterface::addSelection(int startOffset, int endOffset);
impl<'a> /*trait*/ QAccessibleTextInterface_addSelection<()> for (i32, i32) {
  fn addSelection(self , rsthis: &mut QAccessibleTextInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAccessibleTextInterface12addSelectionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN24QAccessibleTextInterface12addSelectionEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QAccessibleTextInterface::scrollToSubstring(int startIndex, int endIndex);
impl /*struct*/ QAccessibleTextInterface {
  pub fn scrollToSubstring<RetType, T: QAccessibleTextInterface_scrollToSubstring<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.scrollToSubstring(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_scrollToSubstring<RetType> {
  fn scrollToSubstring(self , rsthis: &mut QAccessibleTextInterface) -> RetType;
}

  // proto:  void QAccessibleTextInterface::scrollToSubstring(int startIndex, int endIndex);
impl<'a> /*trait*/ QAccessibleTextInterface_scrollToSubstring<()> for (i32, i32) {
  fn scrollToSubstring(self , rsthis: &mut QAccessibleTextInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAccessibleTextInterface17scrollToSubstringEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN24QAccessibleTextInterface17scrollToSubstringEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QAccessibleTextInterface::cursorPosition();
impl /*struct*/ QAccessibleTextInterface {
  pub fn cursorPosition<RetType, T: QAccessibleTextInterface_cursorPosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cursorPosition(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_cursorPosition<RetType> {
  fn cursorPosition(self , rsthis: &mut QAccessibleTextInterface) -> RetType;
}

  // proto:  int QAccessibleTextInterface::cursorPosition();
impl<'a> /*trait*/ QAccessibleTextInterface_cursorPosition<i32> for () {
  fn cursorPosition(self , rsthis: &mut QAccessibleTextInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QAccessibleTextInterface14cursorPositionEv()};
    let mut ret = unsafe {_ZNK24QAccessibleTextInterface14cursorPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAccessibleTextInterface::setSelection(int selectionIndex, int startOffset, int endOffset);
impl /*struct*/ QAccessibleTextInterface {
  pub fn setSelection<RetType, T: QAccessibleTextInterface_setSelection<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSelection(self);
    // return 1;
  }
}

pub trait QAccessibleTextInterface_setSelection<RetType> {
  fn setSelection(self , rsthis: &mut QAccessibleTextInterface) -> RetType;
}

  // proto:  void QAccessibleTextInterface::setSelection(int selectionIndex, int startOffset, int endOffset);
impl<'a> /*trait*/ QAccessibleTextInterface_setSelection<()> for (i32, i32, i32) {
  fn setSelection(self , rsthis: &mut QAccessibleTextInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QAccessibleTextInterface12setSelectionEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN24QAccessibleTextInterface12setSelectionEiii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleEvent {
    return QAccessibleEvent{qclsinst: qthis};
  }
}
  // proto:  QObject * QAccessibleEvent::object();
impl /*struct*/ QAccessibleEvent {
  pub fn object<RetType, T: QAccessibleEvent_object<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.object(self);
    // return 1;
  }
}

pub trait QAccessibleEvent_object<RetType> {
  fn object(self , rsthis: &mut QAccessibleEvent) -> RetType;
}

  // proto:  QObject * QAccessibleEvent::object();
impl<'a> /*trait*/ QAccessibleEvent_object<QObject> for () {
  fn object(self , rsthis: &mut QAccessibleEvent) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAccessibleEvent6objectEv()};
    let mut ret = unsafe {_ZNK16QAccessibleEvent6objectEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAccessibleEvent::setChild(int chld);
impl /*struct*/ QAccessibleEvent {
  pub fn setChild<RetType, T: QAccessibleEvent_setChild<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setChild(self);
    // return 1;
  }
}

pub trait QAccessibleEvent_setChild<RetType> {
  fn setChild(self , rsthis: &mut QAccessibleEvent) -> RetType;
}

  // proto:  void QAccessibleEvent::setChild(int chld);
impl<'a> /*trait*/ QAccessibleEvent_setChild<()> for (i32) {
  fn setChild(self , rsthis: &mut QAccessibleEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAccessibleEvent8setChildEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QAccessibleEvent8setChildEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QAccessibleEvent::accessibleInterface();
impl /*struct*/ QAccessibleEvent {
  pub fn accessibleInterface<RetType, T: QAccessibleEvent_accessibleInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.accessibleInterface(self);
    // return 1;
  }
}

pub trait QAccessibleEvent_accessibleInterface<RetType> {
  fn accessibleInterface(self , rsthis: &mut QAccessibleEvent) -> RetType;
}

  // proto:  QAccessibleInterface * QAccessibleEvent::accessibleInterface();
impl<'a> /*trait*/ QAccessibleEvent_accessibleInterface<QAccessibleInterface> for () {
  fn accessibleInterface(self , rsthis: &mut QAccessibleEvent) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAccessibleEvent19accessibleInterfaceEv()};
    let mut ret = unsafe {_ZNK16QAccessibleEvent19accessibleInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAccessibleEvent::QAccessibleEvent(const QAccessibleEvent & );
impl /*struct*/ QAccessibleEvent {
  pub fn NewQAccessibleEvent<T: QAccessibleEvent_NewQAccessibleEvent>(value: T) -> QAccessibleEvent {
    let rsthis = value.NewQAccessibleEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleEvent_NewQAccessibleEvent {
  fn NewQAccessibleEvent(self) -> QAccessibleEvent;
}

  // proto:  void QAccessibleEvent::QAccessibleEvent(const QAccessibleEvent & );
impl<'a> /*trait*/ QAccessibleEvent_NewQAccessibleEvent for (QAccessibleEvent) {
  fn NewQAccessibleEvent(self) -> QAccessibleEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAccessibleEventC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QAccessibleEventC1ERKS_(qthis, arg0)};
    let rsthis = QAccessibleEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QAccessibleEvent::child();
impl /*struct*/ QAccessibleEvent {
  pub fn child<RetType, T: QAccessibleEvent_child<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.child(self);
    // return 1;
  }
}

pub trait QAccessibleEvent_child<RetType> {
  fn child(self , rsthis: &mut QAccessibleEvent) -> RetType;
}

  // proto:  int QAccessibleEvent::child();
impl<'a> /*trait*/ QAccessibleEvent_child<i32> for () {
  fn child(self , rsthis: &mut QAccessibleEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAccessibleEvent5childEv()};
    let mut ret = unsafe {_ZNK16QAccessibleEvent5childEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAccessibleEvent::~QAccessibleEvent();
impl /*struct*/ QAccessibleEvent {
  pub fn FreeQAccessibleEvent<RetType, T: QAccessibleEvent_FreeQAccessibleEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleEvent(self);
    // return 1;
  }
}

pub trait QAccessibleEvent_FreeQAccessibleEvent<RetType> {
  fn FreeQAccessibleEvent(self , rsthis: &mut QAccessibleEvent) -> RetType;
}

  // proto:  void QAccessibleEvent::~QAccessibleEvent();
impl<'a> /*trait*/ QAccessibleEvent_FreeQAccessibleEvent<()> for () {
  fn FreeQAccessibleEvent(self , rsthis: &mut QAccessibleEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAccessibleEventD0Ev()};
     unsafe {_ZN16QAccessibleEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleActionInterface {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleActionInterface {
    return QAccessibleActionInterface{qclsinst: qthis};
  }
}
  // proto: static QString QAccessibleActionInterface::scrollUpAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn scrollUpAction_s<RetType, T: QAccessibleActionInterface_scrollUpAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.scrollUpAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_scrollUpAction_s<RetType> {
  fn scrollUpAction_s(self ) -> RetType;
}

  // proto: static QString QAccessibleActionInterface::scrollUpAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollUpAction_s<QString> for () {
  fn scrollUpAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14scrollUpActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface14scrollUpActionEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringList QAccessibleActionInterface::actionNames();
impl /*struct*/ QAccessibleActionInterface {
  pub fn actionNames<RetType, T: QAccessibleActionInterface_actionNames<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.actionNames(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_actionNames<RetType> {
  fn actionNames(self , rsthis: &mut QAccessibleActionInterface) -> RetType;
}

  // proto:  QStringList QAccessibleActionInterface::actionNames();
impl<'a> /*trait*/ QAccessibleActionInterface_actionNames<()> for () {
  fn actionNames(self , rsthis: &mut QAccessibleActionInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleActionInterface11actionNamesEv()};
     unsafe {_ZNK26QAccessibleActionInterface11actionNamesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static const QString & QAccessibleActionInterface::decreaseAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn decreaseAction_s<RetType, T: QAccessibleActionInterface_decreaseAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.decreaseAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_decreaseAction_s<RetType> {
  fn decreaseAction_s(self ) -> RetType;
}

  // proto: static const QString & QAccessibleActionInterface::decreaseAction();
impl<'a> /*trait*/ QAccessibleActionInterface_decreaseAction_s<QString> for () {
  fn decreaseAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14decreaseActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface14decreaseActionEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static const QString & QAccessibleActionInterface::toggleAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn toggleAction_s<RetType, T: QAccessibleActionInterface_toggleAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.toggleAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_toggleAction_s<RetType> {
  fn toggleAction_s(self ) -> RetType;
}

  // proto: static const QString & QAccessibleActionInterface::toggleAction();
impl<'a> /*trait*/ QAccessibleActionInterface_toggleAction_s<QString> for () {
  fn toggleAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface12toggleActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface12toggleActionEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QAccessibleActionInterface::localizedActionName(const QString & name);
impl /*struct*/ QAccessibleActionInterface {
  pub fn localizedActionName<RetType, T: QAccessibleActionInterface_localizedActionName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.localizedActionName(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_localizedActionName<RetType> {
  fn localizedActionName(self , rsthis: &mut QAccessibleActionInterface) -> RetType;
}

  // proto:  QString QAccessibleActionInterface::localizedActionName(const QString & name);
impl<'a> /*trait*/ QAccessibleActionInterface_localizedActionName<QString> for (QString) {
  fn localizedActionName(self , rsthis: &mut QAccessibleActionInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleActionInterface19localizedActionNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK26QAccessibleActionInterface19localizedActionNameERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QAccessibleActionInterface::localizedActionDescription(const QString & name);
impl /*struct*/ QAccessibleActionInterface {
  pub fn localizedActionDescription<RetType, T: QAccessibleActionInterface_localizedActionDescription<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.localizedActionDescription(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_localizedActionDescription<RetType> {
  fn localizedActionDescription(self , rsthis: &mut QAccessibleActionInterface) -> RetType;
}

  // proto:  QString QAccessibleActionInterface::localizedActionDescription(const QString & name);
impl<'a> /*trait*/ QAccessibleActionInterface_localizedActionDescription<QString> for (QString) {
  fn localizedActionDescription(self , rsthis: &mut QAccessibleActionInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleActionInterface26localizedActionDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK26QAccessibleActionInterface26localizedActionDescriptionERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QAccessibleActionInterface::scrollLeftAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn scrollLeftAction_s<RetType, T: QAccessibleActionInterface_scrollLeftAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.scrollLeftAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_scrollLeftAction_s<RetType> {
  fn scrollLeftAction_s(self ) -> RetType;
}

  // proto: static QString QAccessibleActionInterface::scrollLeftAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollLeftAction_s<QString> for () {
  fn scrollLeftAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface16scrollLeftActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface16scrollLeftActionEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QAccessibleActionInterface::previousPageAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn previousPageAction_s<RetType, T: QAccessibleActionInterface_previousPageAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.previousPageAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_previousPageAction_s<RetType> {
  fn previousPageAction_s(self ) -> RetType;
}

  // proto: static QString QAccessibleActionInterface::previousPageAction();
impl<'a> /*trait*/ QAccessibleActionInterface_previousPageAction_s<QString> for () {
  fn previousPageAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface18previousPageActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface18previousPageActionEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static const QString & QAccessibleActionInterface::showMenuAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn showMenuAction_s<RetType, T: QAccessibleActionInterface_showMenuAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.showMenuAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_showMenuAction_s<RetType> {
  fn showMenuAction_s(self ) -> RetType;
}

  // proto: static const QString & QAccessibleActionInterface::showMenuAction();
impl<'a> /*trait*/ QAccessibleActionInterface_showMenuAction_s<QString> for () {
  fn showMenuAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14showMenuActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface14showMenuActionEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QAccessibleActionInterface::scrollRightAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn scrollRightAction_s<RetType, T: QAccessibleActionInterface_scrollRightAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.scrollRightAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_scrollRightAction_s<RetType> {
  fn scrollRightAction_s(self ) -> RetType;
}

  // proto: static QString QAccessibleActionInterface::scrollRightAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollRightAction_s<QString> for () {
  fn scrollRightAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface17scrollRightActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface17scrollRightActionEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static const QString & QAccessibleActionInterface::setFocusAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn setFocusAction_s<RetType, T: QAccessibleActionInterface_setFocusAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setFocusAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_setFocusAction_s<RetType> {
  fn setFocusAction_s(self ) -> RetType;
}

  // proto: static const QString & QAccessibleActionInterface::setFocusAction();
impl<'a> /*trait*/ QAccessibleActionInterface_setFocusAction_s<QString> for () {
  fn setFocusAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14setFocusActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface14setFocusActionEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QAccessibleActionInterface::nextPageAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn nextPageAction_s<RetType, T: QAccessibleActionInterface_nextPageAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.nextPageAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_nextPageAction_s<RetType> {
  fn nextPageAction_s(self ) -> RetType;
}

  // proto: static QString QAccessibleActionInterface::nextPageAction();
impl<'a> /*trait*/ QAccessibleActionInterface_nextPageAction_s<QString> for () {
  fn nextPageAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14nextPageActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface14nextPageActionEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAccessibleActionInterface::~QAccessibleActionInterface();
impl /*struct*/ QAccessibleActionInterface {
  pub fn FreeQAccessibleActionInterface<RetType, T: QAccessibleActionInterface_FreeQAccessibleActionInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleActionInterface(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_FreeQAccessibleActionInterface<RetType> {
  fn FreeQAccessibleActionInterface(self , rsthis: &mut QAccessibleActionInterface) -> RetType;
}

  // proto:  void QAccessibleActionInterface::~QAccessibleActionInterface();
impl<'a> /*trait*/ QAccessibleActionInterface_FreeQAccessibleActionInterface<()> for () {
  fn FreeQAccessibleActionInterface(self , rsthis: &mut QAccessibleActionInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterfaceD0Ev()};
     unsafe {_ZN26QAccessibleActionInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static const QString & QAccessibleActionInterface::pressAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn pressAction_s<RetType, T: QAccessibleActionInterface_pressAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.pressAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_pressAction_s<RetType> {
  fn pressAction_s(self ) -> RetType;
}

  // proto: static const QString & QAccessibleActionInterface::pressAction();
impl<'a> /*trait*/ QAccessibleActionInterface_pressAction_s<QString> for () {
  fn pressAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface11pressActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface11pressActionEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAccessibleActionInterface::doAction(const QString & actionName);
impl /*struct*/ QAccessibleActionInterface {
  pub fn doAction<RetType, T: QAccessibleActionInterface_doAction<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.doAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_doAction<RetType> {
  fn doAction(self , rsthis: &mut QAccessibleActionInterface) -> RetType;
}

  // proto:  void QAccessibleActionInterface::doAction(const QString & actionName);
impl<'a> /*trait*/ QAccessibleActionInterface_doAction<()> for (QString) {
  fn doAction(self , rsthis: &mut QAccessibleActionInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface8doActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN26QAccessibleActionInterface8doActionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static const QString & QAccessibleActionInterface::increaseAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn increaseAction_s<RetType, T: QAccessibleActionInterface_increaseAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.increaseAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_increaseAction_s<RetType> {
  fn increaseAction_s(self ) -> RetType;
}

  // proto: static const QString & QAccessibleActionInterface::increaseAction();
impl<'a> /*trait*/ QAccessibleActionInterface_increaseAction_s<QString> for () {
  fn increaseAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface14increaseActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface14increaseActionEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringList QAccessibleActionInterface::keyBindingsForAction(const QString & actionName);
impl /*struct*/ QAccessibleActionInterface {
  pub fn keyBindingsForAction<RetType, T: QAccessibleActionInterface_keyBindingsForAction<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.keyBindingsForAction(self);
    // return 1;
  }
}

pub trait QAccessibleActionInterface_keyBindingsForAction<RetType> {
  fn keyBindingsForAction(self , rsthis: &mut QAccessibleActionInterface) -> RetType;
}

  // proto:  QStringList QAccessibleActionInterface::keyBindingsForAction(const QString & actionName);
impl<'a> /*trait*/ QAccessibleActionInterface_keyBindingsForAction<()> for (QString) {
  fn keyBindingsForAction(self , rsthis: &mut QAccessibleActionInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleActionInterface20keyBindingsForActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK26QAccessibleActionInterface20keyBindingsForActionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QString QAccessibleActionInterface::scrollDownAction();
impl /*struct*/ QAccessibleActionInterface {
  pub fn scrollDownAction_s<RetType, T: QAccessibleActionInterface_scrollDownAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.scrollDownAction_s();
    // return 1;
  }
}

pub trait QAccessibleActionInterface_scrollDownAction_s<RetType> {
  fn scrollDownAction_s(self ) -> RetType;
}

  // proto: static QString QAccessibleActionInterface::scrollDownAction();
impl<'a> /*trait*/ QAccessibleActionInterface_scrollDownAction_s<QString> for () {
  fn scrollDownAction_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleActionInterface16scrollDownActionEv()};
    let mut ret = unsafe {_ZN26QAccessibleActionInterface16scrollDownActionEv()};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleInterface {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleInterface {
    return QAccessibleInterface{qclsinst: qthis};
  }
}
  // proto:  QAccessibleImageInterface * QAccessibleInterface::imageInterface();
impl /*struct*/ QAccessibleInterface {
  pub fn imageInterface<RetType, T: QAccessibleInterface_imageInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.imageInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_imageInterface<RetType> {
  fn imageInterface(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  QAccessibleImageInterface * QAccessibleInterface::imageInterface();
impl<'a> /*trait*/ QAccessibleInterface_imageInterface<QAccessibleImageInterface> for () {
  fn imageInterface(self , rsthis: &mut QAccessibleInterface) -> QAccessibleImageInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface14imageInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface14imageInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleImageInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QAccessibleTableInterface * QAccessibleInterface::tableInterface();
impl /*struct*/ QAccessibleInterface {
  pub fn tableInterface<RetType, T: QAccessibleInterface_tableInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tableInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_tableInterface<RetType> {
  fn tableInterface(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  QAccessibleTableInterface * QAccessibleInterface::tableInterface();
impl<'a> /*trait*/ QAccessibleInterface_tableInterface<QAccessibleTableInterface> for () {
  fn tableInterface(self , rsthis: &mut QAccessibleInterface) -> QAccessibleTableInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface14tableInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface14tableInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleTableInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QAccessibleEditableTextInterface * QAccessibleInterface::editableTextInterface();
impl /*struct*/ QAccessibleInterface {
  pub fn editableTextInterface<RetType, T: QAccessibleInterface_editableTextInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.editableTextInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_editableTextInterface<RetType> {
  fn editableTextInterface(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  QAccessibleEditableTextInterface * QAccessibleInterface::editableTextInterface();
impl<'a> /*trait*/ QAccessibleInterface_editableTextInterface<QAccessibleEditableTextInterface> for () {
  fn editableTextInterface(self , rsthis: &mut QAccessibleInterface) -> QAccessibleEditableTextInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface21editableTextInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface21editableTextInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleEditableTextInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QAccessibleValueInterface * QAccessibleInterface::valueInterface();
impl /*struct*/ QAccessibleInterface {
  pub fn valueInterface<RetType, T: QAccessibleInterface_valueInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.valueInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_valueInterface<RetType> {
  fn valueInterface(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  QAccessibleValueInterface * QAccessibleInterface::valueInterface();
impl<'a> /*trait*/ QAccessibleInterface_valueInterface<QAccessibleValueInterface> for () {
  fn valueInterface(self , rsthis: &mut QAccessibleInterface) -> QAccessibleValueInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface14valueInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface14valueInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleValueInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QAccessibleInterface::rect();
impl /*struct*/ QAccessibleInterface {
  pub fn rect<RetType, T: QAccessibleInterface_rect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_rect<RetType> {
  fn rect(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  QRect QAccessibleInterface::rect();
impl<'a> /*trait*/ QAccessibleInterface_rect<QRect> for () {
  fn rect(self , rsthis: &mut QAccessibleInterface) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface4rectEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QObject * QAccessibleInterface::object();
impl /*struct*/ QAccessibleInterface {
  pub fn object<RetType, T: QAccessibleInterface_object<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.object(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_object<RetType> {
  fn object(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  QObject * QAccessibleInterface::object();
impl<'a> /*trait*/ QAccessibleInterface_object<QObject> for () {
  fn object(self , rsthis: &mut QAccessibleInterface) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface6objectEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface6objectEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QAccessibleActionInterface * QAccessibleInterface::actionInterface();
impl /*struct*/ QAccessibleInterface {
  pub fn actionInterface<RetType, T: QAccessibleInterface_actionInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.actionInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_actionInterface<RetType> {
  fn actionInterface(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  QAccessibleActionInterface * QAccessibleInterface::actionInterface();
impl<'a> /*trait*/ QAccessibleInterface_actionInterface<QAccessibleActionInterface> for () {
  fn actionInterface(self , rsthis: &mut QAccessibleInterface) -> QAccessibleActionInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface15actionInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface15actionInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleActionInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QAccessibleInterface::parent();
impl /*struct*/ QAccessibleInterface {
  pub fn parent<RetType, T: QAccessibleInterface_parent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_parent<RetType> {
  fn parent(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  QAccessibleInterface * QAccessibleInterface::parent();
impl<'a> /*trait*/ QAccessibleInterface_parent<QAccessibleInterface> for () {
  fn parent(self , rsthis: &mut QAccessibleInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface6parentEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface6parentEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QAccessibleInterface::childAt(int x, int y);
impl /*struct*/ QAccessibleInterface {
  pub fn childAt<RetType, T: QAccessibleInterface_childAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.childAt(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_childAt<RetType> {
  fn childAt(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  QAccessibleInterface * QAccessibleInterface::childAt(int x, int y);
impl<'a> /*trait*/ QAccessibleInterface_childAt<QAccessibleInterface> for (i32, i32) {
  fn childAt(self , rsthis: &mut QAccessibleInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface7childAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK20QAccessibleInterface7childAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QAccessibleInterface::childCount();
impl /*struct*/ QAccessibleInterface {
  pub fn childCount<RetType, T: QAccessibleInterface_childCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.childCount(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_childCount<RetType> {
  fn childCount(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  int QAccessibleInterface::childCount();
impl<'a> /*trait*/ QAccessibleInterface_childCount<i32> for () {
  fn childCount(self , rsthis: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface10childCountEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface10childCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QAccessibleTableCellInterface * QAccessibleInterface::tableCellInterface();
impl /*struct*/ QAccessibleInterface {
  pub fn tableCellInterface<RetType, T: QAccessibleInterface_tableCellInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tableCellInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_tableCellInterface<RetType> {
  fn tableCellInterface(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  QAccessibleTableCellInterface * QAccessibleInterface::tableCellInterface();
impl<'a> /*trait*/ QAccessibleInterface_tableCellInterface<QAccessibleTableCellInterface> for () {
  fn tableCellInterface(self , rsthis: &mut QAccessibleInterface) -> QAccessibleTableCellInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface18tableCellInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface18tableCellInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleTableCellInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QAccessibleInterface::indexOfChild(const QAccessibleInterface * );
impl /*struct*/ QAccessibleInterface {
  pub fn indexOfChild<RetType, T: QAccessibleInterface_indexOfChild<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.indexOfChild(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_indexOfChild<RetType> {
  fn indexOfChild(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  int QAccessibleInterface::indexOfChild(const QAccessibleInterface * );
impl<'a> /*trait*/ QAccessibleInterface_indexOfChild<i32> for (QAccessibleInterface) {
  fn indexOfChild(self , rsthis: &mut QAccessibleInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface12indexOfChildEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK20QAccessibleInterface12indexOfChildEPKS_(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QColor QAccessibleInterface::foregroundColor();
impl /*struct*/ QAccessibleInterface {
  pub fn foregroundColor<RetType, T: QAccessibleInterface_foregroundColor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.foregroundColor(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_foregroundColor<RetType> {
  fn foregroundColor(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  QColor QAccessibleInterface::foregroundColor();
impl<'a> /*trait*/ QAccessibleInterface_foregroundColor<QColor> for () {
  fn foregroundColor(self , rsthis: &mut QAccessibleInterface) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface15foregroundColorEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface15foregroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QAccessibleInterface::isValid();
impl /*struct*/ QAccessibleInterface {
  pub fn isValid<RetType, T: QAccessibleInterface_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_isValid<RetType> {
  fn isValid(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  bool QAccessibleInterface::isValid();
impl<'a> /*trait*/ QAccessibleInterface_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QAccessibleInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface7isValidEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QWindow * QAccessibleInterface::window();
impl /*struct*/ QAccessibleInterface {
  pub fn window<RetType, T: QAccessibleInterface_window<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.window(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_window<RetType> {
  fn window(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  QWindow * QAccessibleInterface::window();
impl<'a> /*trait*/ QAccessibleInterface_window<QWindow> for () {
  fn window(self , rsthis: &mut QAccessibleInterface) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface6windowEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface6windowEv(rsthis.qclsinst)};
    let mut ret1 = QWindow::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAccessibleInterface::virtual_hook(int id, void * data);
impl /*struct*/ QAccessibleInterface {
  pub fn virtual_hook<RetType, T: QAccessibleInterface_virtual_hook<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.virtual_hook(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_virtual_hook<RetType> {
  fn virtual_hook(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  void QAccessibleInterface::virtual_hook(int id, void * data);
impl<'a> /*trait*/ QAccessibleInterface_virtual_hook<()> for (i32, *mut c_void) {
  fn virtual_hook(self , rsthis: &mut QAccessibleInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface12virtual_hookEiPv()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_void;
     unsafe {_ZN20QAccessibleInterface12virtual_hookEiPv(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QAccessibleInterface::focusChild();
impl /*struct*/ QAccessibleInterface {
  pub fn focusChild<RetType, T: QAccessibleInterface_focusChild<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.focusChild(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_focusChild<RetType> {
  fn focusChild(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  QAccessibleInterface * QAccessibleInterface::focusChild();
impl<'a> /*trait*/ QAccessibleInterface_focusChild<QAccessibleInterface> for () {
  fn focusChild(self , rsthis: &mut QAccessibleInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface10focusChildEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface10focusChildEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QAccessibleInterface::child(int index);
impl /*struct*/ QAccessibleInterface {
  pub fn child<RetType, T: QAccessibleInterface_child<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.child(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_child<RetType> {
  fn child(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  QAccessibleInterface * QAccessibleInterface::child(int index);
impl<'a> /*trait*/ QAccessibleInterface_child<QAccessibleInterface> for (i32) {
  fn child(self , rsthis: &mut QAccessibleInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface5childEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK20QAccessibleInterface5childEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QAccessibleTextInterface * QAccessibleInterface::textInterface();
impl /*struct*/ QAccessibleInterface {
  pub fn textInterface<RetType, T: QAccessibleInterface_textInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.textInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_textInterface<RetType> {
  fn textInterface(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  QAccessibleTextInterface * QAccessibleInterface::textInterface();
impl<'a> /*trait*/ QAccessibleInterface_textInterface<QAccessibleTextInterface> for () {
  fn textInterface(self , rsthis: &mut QAccessibleInterface) -> QAccessibleTextInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterface13textInterfaceEv()};
    let mut ret = unsafe {_ZN20QAccessibleInterface13textInterfaceEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleTextInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QColor QAccessibleInterface::backgroundColor();
impl /*struct*/ QAccessibleInterface {
  pub fn backgroundColor<RetType, T: QAccessibleInterface_backgroundColor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.backgroundColor(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_backgroundColor<RetType> {
  fn backgroundColor(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  QColor QAccessibleInterface::backgroundColor();
impl<'a> /*trait*/ QAccessibleInterface_backgroundColor<QColor> for () {
  fn backgroundColor(self , rsthis: &mut QAccessibleInterface) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QAccessibleInterface15backgroundColorEv()};
    let mut ret = unsafe {_ZNK20QAccessibleInterface15backgroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAccessibleInterface::~QAccessibleInterface();
impl /*struct*/ QAccessibleInterface {
  pub fn FreeQAccessibleInterface<RetType, T: QAccessibleInterface_FreeQAccessibleInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleInterface(self);
    // return 1;
  }
}

pub trait QAccessibleInterface_FreeQAccessibleInterface<RetType> {
  fn FreeQAccessibleInterface(self , rsthis: &mut QAccessibleInterface) -> RetType;
}

  // proto:  void QAccessibleInterface::~QAccessibleInterface();
impl<'a> /*trait*/ QAccessibleInterface_FreeQAccessibleInterface<()> for () {
  fn FreeQAccessibleInterface(self , rsthis: &mut QAccessibleInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QAccessibleInterfaceD0Ev()};
     unsafe {_ZN20QAccessibleInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleEditableTextInterface {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleEditableTextInterface {
    return QAccessibleEditableTextInterface{qclsinst: qthis};
  }
}
  // proto:  void QAccessibleEditableTextInterface::insertText(int offset, const QString & text);
impl /*struct*/ QAccessibleEditableTextInterface {
  pub fn insertText<RetType, T: QAccessibleEditableTextInterface_insertText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.insertText(self);
    // return 1;
  }
}

pub trait QAccessibleEditableTextInterface_insertText<RetType> {
  fn insertText(self , rsthis: &mut QAccessibleEditableTextInterface) -> RetType;
}

  // proto:  void QAccessibleEditableTextInterface::insertText(int offset, const QString & text);
impl<'a> /*trait*/ QAccessibleEditableTextInterface_insertText<()> for (i32, QString) {
  fn insertText(self , rsthis: &mut QAccessibleEditableTextInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QAccessibleEditableTextInterface10insertTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN32QAccessibleEditableTextInterface10insertTextEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QAccessibleEditableTextInterface::replaceText(int startOffset, int endOffset, const QString & text);
impl /*struct*/ QAccessibleEditableTextInterface {
  pub fn replaceText<RetType, T: QAccessibleEditableTextInterface_replaceText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.replaceText(self);
    // return 1;
  }
}

pub trait QAccessibleEditableTextInterface_replaceText<RetType> {
  fn replaceText(self , rsthis: &mut QAccessibleEditableTextInterface) -> RetType;
}

  // proto:  void QAccessibleEditableTextInterface::replaceText(int startOffset, int endOffset, const QString & text);
impl<'a> /*trait*/ QAccessibleEditableTextInterface_replaceText<()> for (i32, i32, QString) {
  fn replaceText(self , rsthis: &mut QAccessibleEditableTextInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QAccessibleEditableTextInterface11replaceTextEiiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN32QAccessibleEditableTextInterface11replaceTextEiiRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QAccessibleEditableTextInterface::deleteText(int startOffset, int endOffset);
impl /*struct*/ QAccessibleEditableTextInterface {
  pub fn deleteText<RetType, T: QAccessibleEditableTextInterface_deleteText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.deleteText(self);
    // return 1;
  }
}

pub trait QAccessibleEditableTextInterface_deleteText<RetType> {
  fn deleteText(self , rsthis: &mut QAccessibleEditableTextInterface) -> RetType;
}

  // proto:  void QAccessibleEditableTextInterface::deleteText(int startOffset, int endOffset);
impl<'a> /*trait*/ QAccessibleEditableTextInterface_deleteText<()> for (i32, i32) {
  fn deleteText(self , rsthis: &mut QAccessibleEditableTextInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QAccessibleEditableTextInterface10deleteTextEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN32QAccessibleEditableTextInterface10deleteTextEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QAccessibleEditableTextInterface::~QAccessibleEditableTextInterface();
impl /*struct*/ QAccessibleEditableTextInterface {
  pub fn FreeQAccessibleEditableTextInterface<RetType, T: QAccessibleEditableTextInterface_FreeQAccessibleEditableTextInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleEditableTextInterface(self);
    // return 1;
  }
}

pub trait QAccessibleEditableTextInterface_FreeQAccessibleEditableTextInterface<RetType> {
  fn FreeQAccessibleEditableTextInterface(self , rsthis: &mut QAccessibleEditableTextInterface) -> RetType;
}

  // proto:  void QAccessibleEditableTextInterface::~QAccessibleEditableTextInterface();
impl<'a> /*trait*/ QAccessibleEditableTextInterface_FreeQAccessibleEditableTextInterface<()> for () {
  fn FreeQAccessibleEditableTextInterface(self , rsthis: &mut QAccessibleEditableTextInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QAccessibleEditableTextInterfaceD0Ev()};
     unsafe {_ZN32QAccessibleEditableTextInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableCellInterface {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleTableCellInterface {
    return QAccessibleTableCellInterface{qclsinst: qthis};
  }
}
  // proto:  int QAccessibleTableCellInterface::columnIndex();
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn columnIndex<RetType, T: QAccessibleTableCellInterface_columnIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columnIndex(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_columnIndex<RetType> {
  fn columnIndex(self , rsthis: &mut QAccessibleTableCellInterface) -> RetType;
}

  // proto:  int QAccessibleTableCellInterface::columnIndex();
impl<'a> /*trait*/ QAccessibleTableCellInterface_columnIndex<i32> for () {
  fn columnIndex(self , rsthis: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface11columnIndexEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTableCellInterface11columnIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAccessibleTableCellInterface::~QAccessibleTableCellInterface();
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn FreeQAccessibleTableCellInterface<RetType, T: QAccessibleTableCellInterface_FreeQAccessibleTableCellInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleTableCellInterface(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_FreeQAccessibleTableCellInterface<RetType> {
  fn FreeQAccessibleTableCellInterface(self , rsthis: &mut QAccessibleTableCellInterface) -> RetType;
}

  // proto:  void QAccessibleTableCellInterface::~QAccessibleTableCellInterface();
impl<'a> /*trait*/ QAccessibleTableCellInterface_FreeQAccessibleTableCellInterface<()> for () {
  fn FreeQAccessibleTableCellInterface(self , rsthis: &mut QAccessibleTableCellInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN29QAccessibleTableCellInterfaceD0Ev()};
     unsafe {_ZN29QAccessibleTableCellInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QAccessibleTableCellInterface::columnExtent();
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn columnExtent<RetType, T: QAccessibleTableCellInterface_columnExtent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columnExtent(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_columnExtent<RetType> {
  fn columnExtent(self , rsthis: &mut QAccessibleTableCellInterface) -> RetType;
}

  // proto:  int QAccessibleTableCellInterface::columnExtent();
impl<'a> /*trait*/ QAccessibleTableCellInterface_columnExtent<i32> for () {
  fn columnExtent(self , rsthis: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface12columnExtentEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTableCellInterface12columnExtentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QAccessibleTableCellInterface::rowIndex();
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn rowIndex<RetType, T: QAccessibleTableCellInterface_rowIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rowIndex(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_rowIndex<RetType> {
  fn rowIndex(self , rsthis: &mut QAccessibleTableCellInterface) -> RetType;
}

  // proto:  int QAccessibleTableCellInterface::rowIndex();
impl<'a> /*trait*/ QAccessibleTableCellInterface_rowIndex<i32> for () {
  fn rowIndex(self , rsthis: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface8rowIndexEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTableCellInterface8rowIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QAccessibleTableCellInterface::table();
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn table<RetType, T: QAccessibleTableCellInterface_table<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.table(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_table<RetType> {
  fn table(self , rsthis: &mut QAccessibleTableCellInterface) -> RetType;
}

  // proto:  QAccessibleInterface * QAccessibleTableCellInterface::table();
impl<'a> /*trait*/ QAccessibleTableCellInterface_table<QAccessibleInterface> for () {
  fn table(self , rsthis: &mut QAccessibleTableCellInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface5tableEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTableCellInterface5tableEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QAccessibleTableCellInterface::rowExtent();
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn rowExtent<RetType, T: QAccessibleTableCellInterface_rowExtent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rowExtent(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_rowExtent<RetType> {
  fn rowExtent(self , rsthis: &mut QAccessibleTableCellInterface) -> RetType;
}

  // proto:  int QAccessibleTableCellInterface::rowExtent();
impl<'a> /*trait*/ QAccessibleTableCellInterface_rowExtent<i32> for () {
  fn rowExtent(self , rsthis: &mut QAccessibleTableCellInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface9rowExtentEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTableCellInterface9rowExtentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QList<QAccessibleInterface *> QAccessibleTableCellInterface::rowHeaderCells();
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn rowHeaderCells<RetType, T: QAccessibleTableCellInterface_rowHeaderCells<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rowHeaderCells(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_rowHeaderCells<RetType> {
  fn rowHeaderCells(self , rsthis: &mut QAccessibleTableCellInterface) -> RetType;
}

  // proto:  QList<QAccessibleInterface *> QAccessibleTableCellInterface::rowHeaderCells();
impl<'a> /*trait*/ QAccessibleTableCellInterface_rowHeaderCells<()> for () {
  fn rowHeaderCells(self , rsthis: &mut QAccessibleTableCellInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface14rowHeaderCellsEv()};
     unsafe {_ZNK29QAccessibleTableCellInterface14rowHeaderCellsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QAccessibleInterface *> QAccessibleTableCellInterface::columnHeaderCells();
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn columnHeaderCells<RetType, T: QAccessibleTableCellInterface_columnHeaderCells<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columnHeaderCells(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_columnHeaderCells<RetType> {
  fn columnHeaderCells(self , rsthis: &mut QAccessibleTableCellInterface) -> RetType;
}

  // proto:  QList<QAccessibleInterface *> QAccessibleTableCellInterface::columnHeaderCells();
impl<'a> /*trait*/ QAccessibleTableCellInterface_columnHeaderCells<()> for () {
  fn columnHeaderCells(self , rsthis: &mut QAccessibleTableCellInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface17columnHeaderCellsEv()};
     unsafe {_ZNK29QAccessibleTableCellInterface17columnHeaderCellsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QAccessibleTableCellInterface::isSelected();
impl /*struct*/ QAccessibleTableCellInterface {
  pub fn isSelected<RetType, T: QAccessibleTableCellInterface_isSelected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isSelected(self);
    // return 1;
  }
}

pub trait QAccessibleTableCellInterface_isSelected<RetType> {
  fn isSelected(self , rsthis: &mut QAccessibleTableCellInterface) -> RetType;
}

  // proto:  bool QAccessibleTableCellInterface::isSelected();
impl<'a> /*trait*/ QAccessibleTableCellInterface_isSelected<i8> for () {
  fn isSelected(self , rsthis: &mut QAccessibleTableCellInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK29QAccessibleTableCellInterface10isSelectedEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTableCellInterface10isSelectedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTableInterface {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleTableInterface {
    return QAccessibleTableInterface{qclsinst: qthis};
  }
}
  // proto:  bool QAccessibleTableInterface::unselectColumn(int column);
impl /*struct*/ QAccessibleTableInterface {
  pub fn unselectColumn<RetType, T: QAccessibleTableInterface_unselectColumn<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unselectColumn(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_unselectColumn<RetType> {
  fn unselectColumn(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  bool QAccessibleTableInterface::unselectColumn(int column);
impl<'a> /*trait*/ QAccessibleTableInterface_unselectColumn<i8> for (i32) {
  fn unselectColumn(self , rsthis: &mut QAccessibleTableInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface14unselectColumnEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN25QAccessibleTableInterface14unselectColumnEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QAccessibleTableInterface::columnDescription(int column);
impl /*struct*/ QAccessibleTableInterface {
  pub fn columnDescription<RetType, T: QAccessibleTableInterface_columnDescription<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columnDescription(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_columnDescription<RetType> {
  fn columnDescription(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  QString QAccessibleTableInterface::columnDescription(int column);
impl<'a> /*trait*/ QAccessibleTableInterface_columnDescription<QString> for (i32) {
  fn columnDescription(self , rsthis: &mut QAccessibleTableInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface17columnDescriptionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface17columnDescriptionEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QAccessibleTableInterface::selectedCellCount();
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedCellCount<RetType, T: QAccessibleTableInterface_selectedCellCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectedCellCount(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectedCellCount<RetType> {
  fn selectedCellCount(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  int QAccessibleTableInterface::selectedCellCount();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedCellCount<i32> for () {
  fn selectedCellCount(self , rsthis: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface17selectedCellCountEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface17selectedCellCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QList<QAccessibleInterface *> QAccessibleTableInterface::selectedCells();
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedCells<RetType, T: QAccessibleTableInterface_selectedCells<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectedCells(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectedCells<RetType> {
  fn selectedCells(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  QList<QAccessibleInterface *> QAccessibleTableInterface::selectedCells();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedCells<()> for () {
  fn selectedCells(self , rsthis: &mut QAccessibleTableInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface13selectedCellsEv()};
     unsafe {_ZNK25QAccessibleTableInterface13selectedCellsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QAccessibleTableInterface::selectRow(int row);
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectRow<RetType, T: QAccessibleTableInterface_selectRow<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectRow(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectRow<RetType> {
  fn selectRow(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  bool QAccessibleTableInterface::selectRow(int row);
impl<'a> /*trait*/ QAccessibleTableInterface_selectRow<i8> for (i32) {
  fn selectRow(self , rsthis: &mut QAccessibleTableInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface9selectRowEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN25QAccessibleTableInterface9selectRowEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QAccessibleTableInterface::selectedRowCount();
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedRowCount<RetType, T: QAccessibleTableInterface_selectedRowCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectedRowCount(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectedRowCount<RetType> {
  fn selectedRowCount(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  int QAccessibleTableInterface::selectedRowCount();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedRowCount<i32> for () {
  fn selectedRowCount(self , rsthis: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface16selectedRowCountEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface16selectedRowCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAccessibleTableInterface::~QAccessibleTableInterface();
impl /*struct*/ QAccessibleTableInterface {
  pub fn FreeQAccessibleTableInterface<RetType, T: QAccessibleTableInterface_FreeQAccessibleTableInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleTableInterface(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_FreeQAccessibleTableInterface<RetType> {
  fn FreeQAccessibleTableInterface(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  void QAccessibleTableInterface::~QAccessibleTableInterface();
impl<'a> /*trait*/ QAccessibleTableInterface_FreeQAccessibleTableInterface<()> for () {
  fn FreeQAccessibleTableInterface(self , rsthis: &mut QAccessibleTableInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterfaceD0Ev()};
     unsafe {_ZN25QAccessibleTableInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<int> QAccessibleTableInterface::selectedColumns();
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedColumns<RetType, T: QAccessibleTableInterface_selectedColumns<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectedColumns(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectedColumns<RetType> {
  fn selectedColumns(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  QList<int> QAccessibleTableInterface::selectedColumns();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedColumns<()> for () {
  fn selectedColumns(self , rsthis: &mut QAccessibleTableInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface15selectedColumnsEv()};
     unsafe {_ZNK25QAccessibleTableInterface15selectedColumnsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QAccessibleTableInterface::cellAt(int row, int column);
impl /*struct*/ QAccessibleTableInterface {
  pub fn cellAt<RetType, T: QAccessibleTableInterface_cellAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cellAt(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_cellAt<RetType> {
  fn cellAt(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  QAccessibleInterface * QAccessibleTableInterface::cellAt(int row, int column);
impl<'a> /*trait*/ QAccessibleTableInterface_cellAt<QAccessibleInterface> for (i32, i32) {
  fn cellAt(self , rsthis: &mut QAccessibleTableInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface6cellAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface6cellAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<int> QAccessibleTableInterface::selectedRows();
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedRows<RetType, T: QAccessibleTableInterface_selectedRows<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectedRows(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectedRows<RetType> {
  fn selectedRows(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  QList<int> QAccessibleTableInterface::selectedRows();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedRows<()> for () {
  fn selectedRows(self , rsthis: &mut QAccessibleTableInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface12selectedRowsEv()};
     unsafe {_ZNK25QAccessibleTableInterface12selectedRowsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAccessibleTableInterface::modelChange(QAccessibleTableModelChangeEvent * event);
impl /*struct*/ QAccessibleTableInterface {
  pub fn modelChange<RetType, T: QAccessibleTableInterface_modelChange<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.modelChange(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_modelChange<RetType> {
  fn modelChange(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  void QAccessibleTableInterface::modelChange(QAccessibleTableModelChangeEvent * event);
impl<'a> /*trait*/ QAccessibleTableInterface_modelChange<()> for (QAccessibleTableModelChangeEvent) {
  fn modelChange(self , rsthis: &mut QAccessibleTableInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface11modelChangeEP32QAccessibleTableModelChangeEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN25QAccessibleTableInterface11modelChangeEP32QAccessibleTableModelChangeEvent(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QAccessibleTableInterface::columnCount();
impl /*struct*/ QAccessibleTableInterface {
  pub fn columnCount<RetType, T: QAccessibleTableInterface_columnCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_columnCount<RetType> {
  fn columnCount(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  int QAccessibleTableInterface::columnCount();
impl<'a> /*trait*/ QAccessibleTableInterface_columnCount<i32> for () {
  fn columnCount(self , rsthis: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface11columnCountEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface11columnCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QAccessibleTableInterface::selectColumn(int column);
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectColumn<RetType, T: QAccessibleTableInterface_selectColumn<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectColumn(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectColumn<RetType> {
  fn selectColumn(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  bool QAccessibleTableInterface::selectColumn(int column);
impl<'a> /*trait*/ QAccessibleTableInterface_selectColumn<i8> for (i32) {
  fn selectColumn(self , rsthis: &mut QAccessibleTableInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface12selectColumnEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN25QAccessibleTableInterface12selectColumnEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAccessibleTableInterface::unselectRow(int row);
impl /*struct*/ QAccessibleTableInterface {
  pub fn unselectRow<RetType, T: QAccessibleTableInterface_unselectRow<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unselectRow(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_unselectRow<RetType> {
  fn unselectRow(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  bool QAccessibleTableInterface::unselectRow(int row);
impl<'a> /*trait*/ QAccessibleTableInterface_unselectRow<i8> for (i32) {
  fn unselectRow(self , rsthis: &mut QAccessibleTableInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleTableInterface11unselectRowEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN25QAccessibleTableInterface11unselectRowEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QAccessibleTableInterface::rowCount();
impl /*struct*/ QAccessibleTableInterface {
  pub fn rowCount<RetType, T: QAccessibleTableInterface_rowCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_rowCount<RetType> {
  fn rowCount(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  int QAccessibleTableInterface::rowCount();
impl<'a> /*trait*/ QAccessibleTableInterface_rowCount<i32> for () {
  fn rowCount(self , rsthis: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface8rowCountEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface8rowCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString QAccessibleTableInterface::rowDescription(int row);
impl /*struct*/ QAccessibleTableInterface {
  pub fn rowDescription<RetType, T: QAccessibleTableInterface_rowDescription<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rowDescription(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_rowDescription<RetType> {
  fn rowDescription(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  QString QAccessibleTableInterface::rowDescription(int row);
impl<'a> /*trait*/ QAccessibleTableInterface_rowDescription<QString> for (i32) {
  fn rowDescription(self , rsthis: &mut QAccessibleTableInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface14rowDescriptionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface14rowDescriptionEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QAccessibleTableInterface::summary();
impl /*struct*/ QAccessibleTableInterface {
  pub fn summary<RetType, T: QAccessibleTableInterface_summary<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.summary(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_summary<RetType> {
  fn summary(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  QAccessibleInterface * QAccessibleTableInterface::summary();
impl<'a> /*trait*/ QAccessibleTableInterface_summary<QAccessibleInterface> for () {
  fn summary(self , rsthis: &mut QAccessibleTableInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface7summaryEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface7summaryEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QAccessibleTableInterface::isColumnSelected(int column);
impl /*struct*/ QAccessibleTableInterface {
  pub fn isColumnSelected<RetType, T: QAccessibleTableInterface_isColumnSelected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isColumnSelected(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_isColumnSelected<RetType> {
  fn isColumnSelected(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  bool QAccessibleTableInterface::isColumnSelected(int column);
impl<'a> /*trait*/ QAccessibleTableInterface_isColumnSelected<i8> for (i32) {
  fn isColumnSelected(self , rsthis: &mut QAccessibleTableInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface16isColumnSelectedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface16isColumnSelectedEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QAccessibleInterface * QAccessibleTableInterface::caption();
impl /*struct*/ QAccessibleTableInterface {
  pub fn caption<RetType, T: QAccessibleTableInterface_caption<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.caption(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_caption<RetType> {
  fn caption(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  QAccessibleInterface * QAccessibleTableInterface::caption();
impl<'a> /*trait*/ QAccessibleTableInterface_caption<QAccessibleInterface> for () {
  fn caption(self , rsthis: &mut QAccessibleTableInterface) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface7captionEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface7captionEv(rsthis.qclsinst)};
    let mut ret1 = QAccessibleInterface::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QAccessibleTableInterface::isRowSelected(int row);
impl /*struct*/ QAccessibleTableInterface {
  pub fn isRowSelected<RetType, T: QAccessibleTableInterface_isRowSelected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isRowSelected(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_isRowSelected<RetType> {
  fn isRowSelected(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  bool QAccessibleTableInterface::isRowSelected(int row);
impl<'a> /*trait*/ QAccessibleTableInterface_isRowSelected<i8> for (i32) {
  fn isRowSelected(self , rsthis: &mut QAccessibleTableInterface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface13isRowSelectedEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface13isRowSelectedEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QAccessibleTableInterface::selectedColumnCount();
impl /*struct*/ QAccessibleTableInterface {
  pub fn selectedColumnCount<RetType, T: QAccessibleTableInterface_selectedColumnCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectedColumnCount(self);
    // return 1;
  }
}

pub trait QAccessibleTableInterface_selectedColumnCount<RetType> {
  fn selectedColumnCount(self , rsthis: &mut QAccessibleTableInterface) -> RetType;
}

  // proto:  int QAccessibleTableInterface::selectedColumnCount();
impl<'a> /*trait*/ QAccessibleTableInterface_selectedColumnCount<i32> for () {
  fn selectedColumnCount(self , rsthis: &mut QAccessibleTableInterface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleTableInterface19selectedColumnCountEv()};
    let mut ret = unsafe {_ZNK25QAccessibleTableInterface19selectedColumnCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextUpdateEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleTextUpdateEvent {
    return QAccessibleTextUpdateEvent{qbase: QAccessibleTextCursorEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QAccessibleTextUpdateEvent {
  type Target = QAccessibleTextCursorEvent;

  fn deref(&self) -> &QAccessibleTextCursorEvent {
    return &self.qbase;
  }
}
impl AsRef<QAccessibleTextCursorEvent> for QAccessibleTextUpdateEvent {
  fn as_ref(&self) -> &QAccessibleTextCursorEvent {
    return &self.qbase;
  }
}
  // proto:  QString QAccessibleTextUpdateEvent::textInserted();
impl /*struct*/ QAccessibleTextUpdateEvent {
  pub fn textInserted<RetType, T: QAccessibleTextUpdateEvent_textInserted<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.textInserted(self);
    // return 1;
  }
}

pub trait QAccessibleTextUpdateEvent_textInserted<RetType> {
  fn textInserted(self , rsthis: &mut QAccessibleTextUpdateEvent) -> RetType;
}

  // proto:  QString QAccessibleTextUpdateEvent::textInserted();
impl<'a> /*trait*/ QAccessibleTextUpdateEvent_textInserted<QString> for () {
  fn textInserted(self , rsthis: &mut QAccessibleTextUpdateEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK26QAccessibleTextUpdateEvent12textInsertedEv()};
    let mut ret = unsafe {_ZNK26QAccessibleTextUpdateEvent12textInsertedEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAccessibleTextUpdateEvent::QAccessibleTextUpdateEvent(QAccessibleInterface * iface, int position, const QString & oldText, const QString & text);
impl /*struct*/ QAccessibleTextUpdateEvent {
  pub fn NewQAccessibleTextUpdateEvent<T: QAccessibleTextUpdateEvent_NewQAccessibleTextUpdateEvent>(value: T) -> QAccessibleTextUpdateEvent {
    let rsthis = value.NewQAccessibleTextUpdateEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextUpdateEvent_NewQAccessibleTextUpdateEvent {
  fn NewQAccessibleTextUpdateEvent(self) -> QAccessibleTextUpdateEvent;
}

  // proto:  void QAccessibleTextUpdateEvent::QAccessibleTextUpdateEvent(QAccessibleInterface * iface, int position, const QString & oldText, const QString & text);
impl<'a> /*trait*/ QAccessibleTextUpdateEvent_NewQAccessibleTextUpdateEvent for (QAccessibleInterface, i32, QString, QString) {
  fn NewQAccessibleTextUpdateEvent(self) -> QAccessibleTextUpdateEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN26QAccessibleTextUpdateEventC1EP20QAccessibleInterfaceiRK7QStringS4_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZN26QAccessibleTextUpdateEventC1EP20QAccessibleInterfaceiRK7QStringS4_(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QAccessibleTextUpdateEvent{/**/qbase: QAccessibleTextCursorEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QAccessibleTextUpdateEvent::textRemoved();
impl /*struct*/ QAccessibleTextUpdateEvent {
  pub fn textRemoved<RetType, T: QAccessibleTextUpdateEvent_textRemoved<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.textRemoved(self);
    // return 1;
  }
}

pub trait QAccessibleTextUpdateEvent_textRemoved<RetType> {
  fn textRemoved(self , rsthis: &mut QAccessibleTextUpdateEvent) -> RetType;
}

  // proto:  QString QAccessibleTextUpdateEvent::textRemoved();
impl<'a> /*trait*/ QAccessibleTextUpdateEvent_textRemoved<QString> for () {
  fn textRemoved(self , rsthis: &mut QAccessibleTextUpdateEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK26QAccessibleTextUpdateEvent11textRemovedEv()};
    let mut ret = unsafe {_ZNK26QAccessibleTextUpdateEvent11textRemovedEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QAccessibleTextUpdateEvent::changePosition();
impl /*struct*/ QAccessibleTextUpdateEvent {
  pub fn changePosition<RetType, T: QAccessibleTextUpdateEvent_changePosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.changePosition(self);
    // return 1;
  }
}

pub trait QAccessibleTextUpdateEvent_changePosition<RetType> {
  fn changePosition(self , rsthis: &mut QAccessibleTextUpdateEvent) -> RetType;
}

  // proto:  int QAccessibleTextUpdateEvent::changePosition();
impl<'a> /*trait*/ QAccessibleTextUpdateEvent_changePosition<i32> for () {
  fn changePosition(self , rsthis: &mut QAccessibleTextUpdateEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK26QAccessibleTextUpdateEvent14changePositionEv()};
    let mut ret = unsafe {_ZNK26QAccessibleTextUpdateEvent14changePositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAccessibleTextUpdateEvent::QAccessibleTextUpdateEvent(QObject * obj, int position, const QString & oldText, const QString & text);
impl<'a> /*trait*/ QAccessibleTextUpdateEvent_NewQAccessibleTextUpdateEvent for (QObject, i32, QString, QString) {
  fn NewQAccessibleTextUpdateEvent(self) -> QAccessibleTextUpdateEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN26QAccessibleTextUpdateEventC1EP7QObjectiRK7QStringS4_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZN26QAccessibleTextUpdateEventC1EP7QObjectiRK7QStringS4_(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QAccessibleTextUpdateEvent{/**/qbase: QAccessibleTextCursorEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleStateChangeEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleStateChangeEvent {
    return QAccessibleStateChangeEvent{qbase: QAccessibleEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QAccessibleStateChangeEvent {
  type Target = QAccessibleEvent;

  fn deref(&self) -> &QAccessibleEvent {
    return &self.qbase;
  }
}
impl AsRef<QAccessibleEvent> for QAccessibleStateChangeEvent {
  fn as_ref(&self) -> &QAccessibleEvent {
    return &self.qbase;
  }
}
impl /*struct*/ QAccessibleImageInterface {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleImageInterface {
    return QAccessibleImageInterface{qclsinst: qthis};
  }
}
  // proto:  QString QAccessibleImageInterface::imageDescription();
impl /*struct*/ QAccessibleImageInterface {
  pub fn imageDescription<RetType, T: QAccessibleImageInterface_imageDescription<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.imageDescription(self);
    // return 1;
  }
}

pub trait QAccessibleImageInterface_imageDescription<RetType> {
  fn imageDescription(self , rsthis: &mut QAccessibleImageInterface) -> RetType;
}

  // proto:  QString QAccessibleImageInterface::imageDescription();
impl<'a> /*trait*/ QAccessibleImageInterface_imageDescription<QString> for () {
  fn imageDescription(self , rsthis: &mut QAccessibleImageInterface) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleImageInterface16imageDescriptionEv()};
    let mut ret = unsafe {_ZNK25QAccessibleImageInterface16imageDescriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QAccessibleImageInterface::imagePosition();
impl /*struct*/ QAccessibleImageInterface {
  pub fn imagePosition<RetType, T: QAccessibleImageInterface_imagePosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.imagePosition(self);
    // return 1;
  }
}

pub trait QAccessibleImageInterface_imagePosition<RetType> {
  fn imagePosition(self , rsthis: &mut QAccessibleImageInterface) -> RetType;
}

  // proto:  QPoint QAccessibleImageInterface::imagePosition();
impl<'a> /*trait*/ QAccessibleImageInterface_imagePosition<QPoint> for () {
  fn imagePosition(self , rsthis: &mut QAccessibleImageInterface) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleImageInterface13imagePositionEv()};
    let mut ret = unsafe {_ZNK25QAccessibleImageInterface13imagePositionEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAccessibleImageInterface::~QAccessibleImageInterface();
impl /*struct*/ QAccessibleImageInterface {
  pub fn FreeQAccessibleImageInterface<RetType, T: QAccessibleImageInterface_FreeQAccessibleImageInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleImageInterface(self);
    // return 1;
  }
}

pub trait QAccessibleImageInterface_FreeQAccessibleImageInterface<RetType> {
  fn FreeQAccessibleImageInterface(self , rsthis: &mut QAccessibleImageInterface) -> RetType;
}

  // proto:  void QAccessibleImageInterface::~QAccessibleImageInterface();
impl<'a> /*trait*/ QAccessibleImageInterface_FreeQAccessibleImageInterface<()> for () {
  fn FreeQAccessibleImageInterface(self , rsthis: &mut QAccessibleImageInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleImageInterfaceD0Ev()};
     unsafe {_ZN25QAccessibleImageInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QAccessibleImageInterface::imageSize();
impl /*struct*/ QAccessibleImageInterface {
  pub fn imageSize<RetType, T: QAccessibleImageInterface_imageSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.imageSize(self);
    // return 1;
  }
}

pub trait QAccessibleImageInterface_imageSize<RetType> {
  fn imageSize(self , rsthis: &mut QAccessibleImageInterface) -> RetType;
}

  // proto:  QSize QAccessibleImageInterface::imageSize();
impl<'a> /*trait*/ QAccessibleImageInterface_imageSize<QSize> for () {
  fn imageSize(self , rsthis: &mut QAccessibleImageInterface) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleImageInterface9imageSizeEv()};
    let mut ret = unsafe {_ZNK25QAccessibleImageInterface9imageSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextInsertEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleTextInsertEvent {
    return QAccessibleTextInsertEvent{qbase: QAccessibleTextCursorEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QAccessibleTextInsertEvent {
  type Target = QAccessibleTextCursorEvent;

  fn deref(&self) -> &QAccessibleTextCursorEvent {
    return &self.qbase;
  }
}
impl AsRef<QAccessibleTextCursorEvent> for QAccessibleTextInsertEvent {
  fn as_ref(&self) -> &QAccessibleTextCursorEvent {
    return &self.qbase;
  }
}
  // proto:  QString QAccessibleTextInsertEvent::textInserted();
impl /*struct*/ QAccessibleTextInsertEvent {
  pub fn textInserted<RetType, T: QAccessibleTextInsertEvent_textInserted<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.textInserted(self);
    // return 1;
  }
}

pub trait QAccessibleTextInsertEvent_textInserted<RetType> {
  fn textInserted(self , rsthis: &mut QAccessibleTextInsertEvent) -> RetType;
}

  // proto:  QString QAccessibleTextInsertEvent::textInserted();
impl<'a> /*trait*/ QAccessibleTextInsertEvent_textInserted<QString> for () {
  fn textInserted(self , rsthis: &mut QAccessibleTextInsertEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK26QAccessibleTextInsertEvent12textInsertedEv()};
    let mut ret = unsafe {_ZNK26QAccessibleTextInsertEvent12textInsertedEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QAccessibleTextInsertEvent::changePosition();
impl /*struct*/ QAccessibleTextInsertEvent {
  pub fn changePosition<RetType, T: QAccessibleTextInsertEvent_changePosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.changePosition(self);
    // return 1;
  }
}

pub trait QAccessibleTextInsertEvent_changePosition<RetType> {
  fn changePosition(self , rsthis: &mut QAccessibleTextInsertEvent) -> RetType;
}

  // proto:  int QAccessibleTextInsertEvent::changePosition();
impl<'a> /*trait*/ QAccessibleTextInsertEvent_changePosition<i32> for () {
  fn changePosition(self , rsthis: &mut QAccessibleTextInsertEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK26QAccessibleTextInsertEvent14changePositionEv()};
    let mut ret = unsafe {_ZNK26QAccessibleTextInsertEvent14changePositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAccessibleTextInsertEvent::QAccessibleTextInsertEvent(QAccessibleInterface * iface, int position, const QString & text);
impl /*struct*/ QAccessibleTextInsertEvent {
  pub fn NewQAccessibleTextInsertEvent<T: QAccessibleTextInsertEvent_NewQAccessibleTextInsertEvent>(value: T) -> QAccessibleTextInsertEvent {
    let rsthis = value.NewQAccessibleTextInsertEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextInsertEvent_NewQAccessibleTextInsertEvent {
  fn NewQAccessibleTextInsertEvent(self) -> QAccessibleTextInsertEvent;
}

  // proto:  void QAccessibleTextInsertEvent::QAccessibleTextInsertEvent(QAccessibleInterface * iface, int position, const QString & text);
impl<'a> /*trait*/ QAccessibleTextInsertEvent_NewQAccessibleTextInsertEvent for (QAccessibleInterface, i32, QString) {
  fn NewQAccessibleTextInsertEvent(self) -> QAccessibleTextInsertEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN26QAccessibleTextInsertEventC1EP20QAccessibleInterfaceiRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN26QAccessibleTextInsertEventC1EP20QAccessibleInterfaceiRK7QString(qthis, arg0, arg1, arg2)};
    let rsthis = QAccessibleTextInsertEvent{/**/qbase: QAccessibleTextCursorEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAccessibleTextInsertEvent::QAccessibleTextInsertEvent(QObject * obj, int position, const QString & text);
impl<'a> /*trait*/ QAccessibleTextInsertEvent_NewQAccessibleTextInsertEvent for (QObject, i32, QString) {
  fn NewQAccessibleTextInsertEvent(self) -> QAccessibleTextInsertEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN26QAccessibleTextInsertEventC1EP7QObjectiRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN26QAccessibleTextInsertEventC1EP7QObjectiRK7QString(qthis, arg0, arg1, arg2)};
    let rsthis = QAccessibleTextInsertEvent{/**/qbase: QAccessibleTextCursorEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleValueInterface {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleValueInterface {
    return QAccessibleValueInterface{qclsinst: qthis};
  }
}
  // proto:  QVariant QAccessibleValueInterface::maximumValue();
impl /*struct*/ QAccessibleValueInterface {
  pub fn maximumValue<RetType, T: QAccessibleValueInterface_maximumValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumValue(self);
    // return 1;
  }
}

pub trait QAccessibleValueInterface_maximumValue<RetType> {
  fn maximumValue(self , rsthis: &mut QAccessibleValueInterface) -> RetType;
}

  // proto:  QVariant QAccessibleValueInterface::maximumValue();
impl<'a> /*trait*/ QAccessibleValueInterface_maximumValue<QVariant> for () {
  fn maximumValue(self , rsthis: &mut QAccessibleValueInterface) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleValueInterface12maximumValueEv()};
    let mut ret = unsafe {_ZNK25QAccessibleValueInterface12maximumValueEv(rsthis.qclsinst)};
    let mut ret1 = QVariant::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QAccessibleValueInterface::minimumStepSize();
impl /*struct*/ QAccessibleValueInterface {
  pub fn minimumStepSize<RetType, T: QAccessibleValueInterface_minimumStepSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumStepSize(self);
    // return 1;
  }
}

pub trait QAccessibleValueInterface_minimumStepSize<RetType> {
  fn minimumStepSize(self , rsthis: &mut QAccessibleValueInterface) -> RetType;
}

  // proto:  QVariant QAccessibleValueInterface::minimumStepSize();
impl<'a> /*trait*/ QAccessibleValueInterface_minimumStepSize<QVariant> for () {
  fn minimumStepSize(self , rsthis: &mut QAccessibleValueInterface) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleValueInterface15minimumStepSizeEv()};
    let mut ret = unsafe {_ZNK25QAccessibleValueInterface15minimumStepSizeEv(rsthis.qclsinst)};
    let mut ret1 = QVariant::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QAccessibleValueInterface::currentValue();
impl /*struct*/ QAccessibleValueInterface {
  pub fn currentValue<RetType, T: QAccessibleValueInterface_currentValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentValue(self);
    // return 1;
  }
}

pub trait QAccessibleValueInterface_currentValue<RetType> {
  fn currentValue(self , rsthis: &mut QAccessibleValueInterface) -> RetType;
}

  // proto:  QVariant QAccessibleValueInterface::currentValue();
impl<'a> /*trait*/ QAccessibleValueInterface_currentValue<QVariant> for () {
  fn currentValue(self , rsthis: &mut QAccessibleValueInterface) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleValueInterface12currentValueEv()};
    let mut ret = unsafe {_ZNK25QAccessibleValueInterface12currentValueEv(rsthis.qclsinst)};
    let mut ret1 = QVariant::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QAccessibleValueInterface::minimumValue();
impl /*struct*/ QAccessibleValueInterface {
  pub fn minimumValue<RetType, T: QAccessibleValueInterface_minimumValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumValue(self);
    // return 1;
  }
}

pub trait QAccessibleValueInterface_minimumValue<RetType> {
  fn minimumValue(self , rsthis: &mut QAccessibleValueInterface) -> RetType;
}

  // proto:  QVariant QAccessibleValueInterface::minimumValue();
impl<'a> /*trait*/ QAccessibleValueInterface_minimumValue<QVariant> for () {
  fn minimumValue(self , rsthis: &mut QAccessibleValueInterface) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QAccessibleValueInterface12minimumValueEv()};
    let mut ret = unsafe {_ZNK25QAccessibleValueInterface12minimumValueEv(rsthis.qclsinst)};
    let mut ret1 = QVariant::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAccessibleValueInterface::~QAccessibleValueInterface();
impl /*struct*/ QAccessibleValueInterface {
  pub fn FreeQAccessibleValueInterface<RetType, T: QAccessibleValueInterface_FreeQAccessibleValueInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAccessibleValueInterface(self);
    // return 1;
  }
}

pub trait QAccessibleValueInterface_FreeQAccessibleValueInterface<RetType> {
  fn FreeQAccessibleValueInterface(self , rsthis: &mut QAccessibleValueInterface) -> RetType;
}

  // proto:  void QAccessibleValueInterface::~QAccessibleValueInterface();
impl<'a> /*trait*/ QAccessibleValueInterface_FreeQAccessibleValueInterface<()> for () {
  fn FreeQAccessibleValueInterface(self , rsthis: &mut QAccessibleValueInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleValueInterfaceD0Ev()};
     unsafe {_ZN25QAccessibleValueInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAccessibleValueInterface::setCurrentValue(const QVariant & value);
impl /*struct*/ QAccessibleValueInterface {
  pub fn setCurrentValue<RetType, T: QAccessibleValueInterface_setCurrentValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCurrentValue(self);
    // return 1;
  }
}

pub trait QAccessibleValueInterface_setCurrentValue<RetType> {
  fn setCurrentValue(self , rsthis: &mut QAccessibleValueInterface) -> RetType;
}

  // proto:  void QAccessibleValueInterface::setCurrentValue(const QVariant & value);
impl<'a> /*trait*/ QAccessibleValueInterface_setCurrentValue<()> for (QVariant) {
  fn setCurrentValue(self , rsthis: &mut QAccessibleValueInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QAccessibleValueInterface15setCurrentValueERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN25QAccessibleValueInterface15setCurrentValueERK8QVariant(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextRemoveEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleTextRemoveEvent {
    return QAccessibleTextRemoveEvent{qbase: QAccessibleTextCursorEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QAccessibleTextRemoveEvent {
  type Target = QAccessibleTextCursorEvent;

  fn deref(&self) -> &QAccessibleTextCursorEvent {
    return &self.qbase;
  }
}
impl AsRef<QAccessibleTextCursorEvent> for QAccessibleTextRemoveEvent {
  fn as_ref(&self) -> &QAccessibleTextCursorEvent {
    return &self.qbase;
  }
}
  // proto:  void QAccessibleTextRemoveEvent::QAccessibleTextRemoveEvent(QObject * obj, int position, const QString & text);
impl /*struct*/ QAccessibleTextRemoveEvent {
  pub fn NewQAccessibleTextRemoveEvent<T: QAccessibleTextRemoveEvent_NewQAccessibleTextRemoveEvent>(value: T) -> QAccessibleTextRemoveEvent {
    let rsthis = value.NewQAccessibleTextRemoveEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextRemoveEvent_NewQAccessibleTextRemoveEvent {
  fn NewQAccessibleTextRemoveEvent(self) -> QAccessibleTextRemoveEvent;
}

  // proto:  void QAccessibleTextRemoveEvent::QAccessibleTextRemoveEvent(QObject * obj, int position, const QString & text);
impl<'a> /*trait*/ QAccessibleTextRemoveEvent_NewQAccessibleTextRemoveEvent for (QObject, i32, QString) {
  fn NewQAccessibleTextRemoveEvent(self) -> QAccessibleTextRemoveEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN26QAccessibleTextRemoveEventC1EP7QObjectiRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN26QAccessibleTextRemoveEventC1EP7QObjectiRK7QString(qthis, arg0, arg1, arg2)};
    let rsthis = QAccessibleTextRemoveEvent{/**/qbase: QAccessibleTextCursorEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QAccessibleTextRemoveEvent::textRemoved();
impl /*struct*/ QAccessibleTextRemoveEvent {
  pub fn textRemoved<RetType, T: QAccessibleTextRemoveEvent_textRemoved<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.textRemoved(self);
    // return 1;
  }
}

pub trait QAccessibleTextRemoveEvent_textRemoved<RetType> {
  fn textRemoved(self , rsthis: &mut QAccessibleTextRemoveEvent) -> RetType;
}

  // proto:  QString QAccessibleTextRemoveEvent::textRemoved();
impl<'a> /*trait*/ QAccessibleTextRemoveEvent_textRemoved<QString> for () {
  fn textRemoved(self , rsthis: &mut QAccessibleTextRemoveEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK26QAccessibleTextRemoveEvent11textRemovedEv()};
    let mut ret = unsafe {_ZNK26QAccessibleTextRemoveEvent11textRemovedEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAccessibleTextRemoveEvent::QAccessibleTextRemoveEvent(QAccessibleInterface * iface, int position, const QString & text);
impl<'a> /*trait*/ QAccessibleTextRemoveEvent_NewQAccessibleTextRemoveEvent for (QAccessibleInterface, i32, QString) {
  fn NewQAccessibleTextRemoveEvent(self) -> QAccessibleTextRemoveEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN26QAccessibleTextRemoveEventC1EP20QAccessibleInterfaceiRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN26QAccessibleTextRemoveEventC1EP20QAccessibleInterfaceiRK7QString(qthis, arg0, arg1, arg2)};
    let rsthis = QAccessibleTextRemoveEvent{/**/qbase: QAccessibleTextCursorEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QAccessibleTextRemoveEvent::changePosition();
impl /*struct*/ QAccessibleTextRemoveEvent {
  pub fn changePosition<RetType, T: QAccessibleTextRemoveEvent_changePosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.changePosition(self);
    // return 1;
  }
}

pub trait QAccessibleTextRemoveEvent_changePosition<RetType> {
  fn changePosition(self , rsthis: &mut QAccessibleTextRemoveEvent) -> RetType;
}

  // proto:  int QAccessibleTextRemoveEvent::changePosition();
impl<'a> /*trait*/ QAccessibleTextRemoveEvent_changePosition<i32> for () {
  fn changePosition(self , rsthis: &mut QAccessibleTextRemoveEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK26QAccessibleTextRemoveEvent14changePositionEv()};
    let mut ret = unsafe {_ZNK26QAccessibleTextRemoveEvent14changePositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextSelectionEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleTextSelectionEvent {
    return QAccessibleTextSelectionEvent{qbase: QAccessibleTextCursorEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QAccessibleTextSelectionEvent {
  type Target = QAccessibleTextCursorEvent;

  fn deref(&self) -> &QAccessibleTextCursorEvent {
    return &self.qbase;
  }
}
impl AsRef<QAccessibleTextCursorEvent> for QAccessibleTextSelectionEvent {
  fn as_ref(&self) -> &QAccessibleTextCursorEvent {
    return &self.qbase;
  }
}
  // proto:  int QAccessibleTextSelectionEvent::selectionEnd();
impl /*struct*/ QAccessibleTextSelectionEvent {
  pub fn selectionEnd<RetType, T: QAccessibleTextSelectionEvent_selectionEnd<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectionEnd(self);
    // return 1;
  }
}

pub trait QAccessibleTextSelectionEvent_selectionEnd<RetType> {
  fn selectionEnd(self , rsthis: &mut QAccessibleTextSelectionEvent) -> RetType;
}

  // proto:  int QAccessibleTextSelectionEvent::selectionEnd();
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_selectionEnd<i32> for () {
  fn selectionEnd(self , rsthis: &mut QAccessibleTextSelectionEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK29QAccessibleTextSelectionEvent12selectionEndEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTextSelectionEvent12selectionEndEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAccessibleTextSelectionEvent::QAccessibleTextSelectionEvent(QAccessibleInterface * iface, int start, int end);
impl /*struct*/ QAccessibleTextSelectionEvent {
  pub fn NewQAccessibleTextSelectionEvent<T: QAccessibleTextSelectionEvent_NewQAccessibleTextSelectionEvent>(value: T) -> QAccessibleTextSelectionEvent {
    let rsthis = value.NewQAccessibleTextSelectionEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextSelectionEvent_NewQAccessibleTextSelectionEvent {
  fn NewQAccessibleTextSelectionEvent(self) -> QAccessibleTextSelectionEvent;
}

  // proto:  void QAccessibleTextSelectionEvent::QAccessibleTextSelectionEvent(QAccessibleInterface * iface, int start, int end);
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_NewQAccessibleTextSelectionEvent for (QAccessibleInterface, i32, i32) {
  fn NewQAccessibleTextSelectionEvent(self) -> QAccessibleTextSelectionEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN29QAccessibleTextSelectionEventC1EP20QAccessibleInterfaceii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN29QAccessibleTextSelectionEventC1EP20QAccessibleInterfaceii(qthis, arg0, arg1, arg2)};
    let rsthis = QAccessibleTextSelectionEvent{/**/qbase: QAccessibleTextCursorEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QAccessibleTextSelectionEvent::selectionStart();
impl /*struct*/ QAccessibleTextSelectionEvent {
  pub fn selectionStart<RetType, T: QAccessibleTextSelectionEvent_selectionStart<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.selectionStart(self);
    // return 1;
  }
}

pub trait QAccessibleTextSelectionEvent_selectionStart<RetType> {
  fn selectionStart(self , rsthis: &mut QAccessibleTextSelectionEvent) -> RetType;
}

  // proto:  int QAccessibleTextSelectionEvent::selectionStart();
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_selectionStart<i32> for () {
  fn selectionStart(self , rsthis: &mut QAccessibleTextSelectionEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZNK29QAccessibleTextSelectionEvent14selectionStartEv()};
    let mut ret = unsafe {_ZNK29QAccessibleTextSelectionEvent14selectionStartEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAccessibleTextSelectionEvent::QAccessibleTextSelectionEvent(QObject * obj, int start, int end);
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_NewQAccessibleTextSelectionEvent for (QObject, i32, i32) {
  fn NewQAccessibleTextSelectionEvent(self) -> QAccessibleTextSelectionEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN29QAccessibleTextSelectionEventC1EP7QObjectii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN29QAccessibleTextSelectionEventC1EP7QObjectii(qthis, arg0, arg1, arg2)};
    let rsthis = QAccessibleTextSelectionEvent{/**/qbase: QAccessibleTextCursorEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAccessibleTextSelectionEvent::setSelection(int start, int end);
impl /*struct*/ QAccessibleTextSelectionEvent {
  pub fn setSelection<RetType, T: QAccessibleTextSelectionEvent_setSelection<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSelection(self);
    // return 1;
  }
}

pub trait QAccessibleTextSelectionEvent_setSelection<RetType> {
  fn setSelection(self , rsthis: &mut QAccessibleTextSelectionEvent) -> RetType;
}

  // proto:  void QAccessibleTextSelectionEvent::setSelection(int start, int end);
impl<'a> /*trait*/ QAccessibleTextSelectionEvent_setSelection<()> for (i32, i32) {
  fn setSelection(self , rsthis: &mut QAccessibleTextSelectionEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 40)};
    // unsafe{_ZN29QAccessibleTextSelectionEvent12setSelectionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN29QAccessibleTextSelectionEvent12setSelectionEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QAccessibleTextCursorEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleTextCursorEvent {
    return QAccessibleTextCursorEvent{qbase: QAccessibleEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QAccessibleTextCursorEvent {
  type Target = QAccessibleEvent;

  fn deref(&self) -> &QAccessibleEvent {
    return &self.qbase;
  }
}
impl AsRef<QAccessibleEvent> for QAccessibleTextCursorEvent {
  fn as_ref(&self) -> &QAccessibleEvent {
    return &self.qbase;
  }
}
  // proto:  void QAccessibleTextCursorEvent::QAccessibleTextCursorEvent(QAccessibleInterface * iface, int cursorPos);
impl /*struct*/ QAccessibleTextCursorEvent {
  pub fn NewQAccessibleTextCursorEvent<T: QAccessibleTextCursorEvent_NewQAccessibleTextCursorEvent>(value: T) -> QAccessibleTextCursorEvent {
    let rsthis = value.NewQAccessibleTextCursorEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextCursorEvent_NewQAccessibleTextCursorEvent {
  fn NewQAccessibleTextCursorEvent(self) -> QAccessibleTextCursorEvent;
}

  // proto:  void QAccessibleTextCursorEvent::QAccessibleTextCursorEvent(QAccessibleInterface * iface, int cursorPos);
impl<'a> /*trait*/ QAccessibleTextCursorEvent_NewQAccessibleTextCursorEvent for (QAccessibleInterface, i32) {
  fn NewQAccessibleTextCursorEvent(self) -> QAccessibleTextCursorEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleTextCursorEventC1EP20QAccessibleInterfacei()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN26QAccessibleTextCursorEventC1EP20QAccessibleInterfacei(qthis, arg0, arg1)};
    let rsthis = QAccessibleTextCursorEvent{/**/qbase: QAccessibleEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAccessibleTextCursorEvent::setCursorPosition(int position);
impl /*struct*/ QAccessibleTextCursorEvent {
  pub fn setCursorPosition<RetType, T: QAccessibleTextCursorEvent_setCursorPosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCursorPosition(self);
    // return 1;
  }
}

pub trait QAccessibleTextCursorEvent_setCursorPosition<RetType> {
  fn setCursorPosition(self , rsthis: &mut QAccessibleTextCursorEvent) -> RetType;
}

  // proto:  void QAccessibleTextCursorEvent::setCursorPosition(int position);
impl<'a> /*trait*/ QAccessibleTextCursorEvent_setCursorPosition<()> for (i32) {
  fn setCursorPosition(self , rsthis: &mut QAccessibleTextCursorEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleTextCursorEvent17setCursorPositionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN26QAccessibleTextCursorEvent17setCursorPositionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QAccessibleTextCursorEvent::cursorPosition();
impl /*struct*/ QAccessibleTextCursorEvent {
  pub fn cursorPosition<RetType, T: QAccessibleTextCursorEvent_cursorPosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cursorPosition(self);
    // return 1;
  }
}

pub trait QAccessibleTextCursorEvent_cursorPosition<RetType> {
  fn cursorPosition(self , rsthis: &mut QAccessibleTextCursorEvent) -> RetType;
}

  // proto:  int QAccessibleTextCursorEvent::cursorPosition();
impl<'a> /*trait*/ QAccessibleTextCursorEvent_cursorPosition<i32> for () {
  fn cursorPosition(self , rsthis: &mut QAccessibleTextCursorEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAccessibleTextCursorEvent14cursorPositionEv()};
    let mut ret = unsafe {_ZNK26QAccessibleTextCursorEvent14cursorPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAccessibleTextCursorEvent::QAccessibleTextCursorEvent(QObject * obj, int cursorPos);
impl<'a> /*trait*/ QAccessibleTextCursorEvent_NewQAccessibleTextCursorEvent for (QObject, i32) {
  fn NewQAccessibleTextCursorEvent(self) -> QAccessibleTextCursorEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAccessibleTextCursorEventC1EP7QObjecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN26QAccessibleTextCursorEventC1EP7QObjecti(qthis, arg0, arg1)};
    let rsthis = QAccessibleTextCursorEvent{/**/qbase: QAccessibleEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleValueChangeEvent {
  pub fn inheritFrom(qthis: *mut c_void) -> QAccessibleValueChangeEvent {
    return QAccessibleValueChangeEvent{qbase: QAccessibleEvent::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QAccessibleValueChangeEvent {
  type Target = QAccessibleEvent;

  fn deref(&self) -> &QAccessibleEvent {
    return &self.qbase;
  }
}
impl AsRef<QAccessibleEvent> for QAccessibleValueChangeEvent {
  fn as_ref(&self) -> &QAccessibleEvent {
    return &self.qbase;
  }
}
  // proto:  void QAccessibleValueChangeEvent::QAccessibleValueChangeEvent(QObject * obj, const QVariant & val);
impl /*struct*/ QAccessibleValueChangeEvent {
  pub fn NewQAccessibleValueChangeEvent<T: QAccessibleValueChangeEvent_NewQAccessibleValueChangeEvent>(value: T) -> QAccessibleValueChangeEvent {
    let rsthis = value.NewQAccessibleValueChangeEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleValueChangeEvent_NewQAccessibleValueChangeEvent {
  fn NewQAccessibleValueChangeEvent(self) -> QAccessibleValueChangeEvent;
}

  // proto:  void QAccessibleValueChangeEvent::QAccessibleValueChangeEvent(QObject * obj, const QVariant & val);
impl<'a> /*trait*/ QAccessibleValueChangeEvent_NewQAccessibleValueChangeEvent for (QObject, QVariant) {
  fn NewQAccessibleValueChangeEvent(self) -> QAccessibleValueChangeEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN27QAccessibleValueChangeEventC1EP7QObjectRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN27QAccessibleValueChangeEventC1EP7QObjectRK8QVariant(qthis, arg0, arg1)};
    let rsthis = QAccessibleValueChangeEvent{/**/qbase: QAccessibleEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAccessibleValueChangeEvent::QAccessibleValueChangeEvent(QAccessibleInterface * iface, const QVariant & val);
impl<'a> /*trait*/ QAccessibleValueChangeEvent_NewQAccessibleValueChangeEvent for (QAccessibleInterface, QVariant) {
  fn NewQAccessibleValueChangeEvent(self) -> QAccessibleValueChangeEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN27QAccessibleValueChangeEventC1EP20QAccessibleInterfaceRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN27QAccessibleValueChangeEventC1EP20QAccessibleInterfaceRK8QVariant(qthis, arg0, arg1)};
    let rsthis = QAccessibleValueChangeEvent{/**/qbase: QAccessibleEvent::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAccessibleValueChangeEvent::setValue(const QVariant & val);
impl /*struct*/ QAccessibleValueChangeEvent {
  pub fn setValue<RetType, T: QAccessibleValueChangeEvent_setValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setValue(self);
    // return 1;
  }
}

pub trait QAccessibleValueChangeEvent_setValue<RetType> {
  fn setValue(self , rsthis: &mut QAccessibleValueChangeEvent) -> RetType;
}

  // proto:  void QAccessibleValueChangeEvent::setValue(const QVariant & val);
impl<'a> /*trait*/ QAccessibleValueChangeEvent_setValue<()> for (QVariant) {
  fn setValue(self , rsthis: &mut QAccessibleValueChangeEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN27QAccessibleValueChangeEvent8setValueERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QAccessibleValueChangeEvent8setValueERK8QVariant(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVariant QAccessibleValueChangeEvent::value();
impl /*struct*/ QAccessibleValueChangeEvent {
  pub fn value<RetType, T: QAccessibleValueChangeEvent_value<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QAccessibleValueChangeEvent_value<RetType> {
  fn value(self , rsthis: &mut QAccessibleValueChangeEvent) -> RetType;
}

  // proto:  QVariant QAccessibleValueChangeEvent::value();
impl<'a> /*trait*/ QAccessibleValueChangeEvent_value<QVariant> for () {
  fn value(self , rsthis: &mut QAccessibleValueChangeEvent) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK27QAccessibleValueChangeEvent5valueEv()};
    let mut ret = unsafe {_ZNK27QAccessibleValueChangeEvent5valueEv(rsthis.qclsinst)};
    let mut ret1 = QVariant::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

