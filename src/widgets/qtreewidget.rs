// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtWidgets/qtreewidget.h
// dst-file: /src/widgets/qtreewidget.rs
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
use super::qtreeview::QTreeView; // 773
use std::ops::Deref;
// use super::qtreewidget::QTreeWidgetItem; // 773
use super::qwidget::QWidget; // 773
use super::super::core::qstring::QString; // 771
use super::super::core::qstringlist::QStringList; // 771
use super::super::core::qpoint::QPoint; // 771
use super::super::core::qitemselectionmodel::QItemSelectionModel; // 771
use super::super::core::qrect::QRect; // 771
use super::super::gui::qfont::QFont; // 771
use super::super::core::qvariant::QVariant; // 771
use super::super::core::qdatastream::QDataStream; // 771
use super::super::gui::qicon::QIcon; // 771
// use super::qtreewidget::QTreeWidget; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTreeWidget_Class_Size() -> c_int;
  // proto:  void QTreeWidget::itemDoubleClicked(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget17itemDoubleClickedEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QTreeWidget::itemExpanded(QTreeWidgetItem * item);
  fn _ZN11QTreeWidget12itemExpandedEP15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTreeWidget::setColumnCount(int columns);
  fn _ZN11QTreeWidget14setColumnCountEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTreeWidget::~QTreeWidget();
  fn _ZN11QTreeWidgetD0Ev(qthis: *mut c_void);
  // proto:  QList<QTreeWidgetItem *> QTreeWidget::selectedItems();
  fn _ZNK11QTreeWidget13selectedItemsEv(qthis: *mut c_void);
  // proto:  bool QTreeWidget::isItemExpanded(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget14isItemExpandedEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QTreeWidget::QTreeWidget(const QTreeWidget & );
  fn dector_ZN11QTreeWidgetC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QTreeWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTreeWidget::setItemHidden(const QTreeWidgetItem * item, bool hide);
  fn _ZN11QTreeWidget13setItemHiddenEPK15QTreeWidgetItemb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  int QTreeWidget::indexOfTopLevelItem(QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget19indexOfTopLevelItemEP15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QTreeWidget::insertTopLevelItem(int index, QTreeWidgetItem * item);
  fn _ZN11QTreeWidget18insertTopLevelItemEiP15QTreeWidgetItem(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QTreeWidget::setItemWidget(QTreeWidgetItem * item, int column, QWidget * widget);
  fn _ZN11QTreeWidget13setItemWidgetEP15QTreeWidgetItemiP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void);
  // proto:  bool QTreeWidget::isItemSelected(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget14isItemSelectedEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  int QTreeWidget::currentColumn();
  fn _ZNK11QTreeWidget13currentColumnEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTreeWidget::isFirstItemColumnSpanned(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget24isFirstItemColumnSpannedEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QTreeWidget::clear();
  fn _ZN11QTreeWidget5clearEv(qthis: *mut c_void);
  // proto:  void QTreeWidget::itemPressed(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget11itemPressedEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QTreeWidget::setHeaderLabels(const QStringList & labels);
  fn _ZN11QTreeWidget15setHeaderLabelsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTreeWidgetItem * QTreeWidget::invisibleRootItem();
  fn _ZNK11QTreeWidget17invisibleRootItemEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QTreeWidget::metaObject();
  fn _ZNK11QTreeWidget10metaObjectEv(qthis: *mut c_void);
  // proto:  void QTreeWidget::itemEntered(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget11itemEnteredEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  QTreeWidgetItem * QTreeWidget::itemBelow(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget9itemBelowEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QTreeWidget::sortColumn();
  fn _ZNK11QTreeWidget10sortColumnEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTreeWidget::collapseItem(const QTreeWidgetItem * item);
  fn _ZN11QTreeWidget12collapseItemEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTreeWidgetItem * QTreeWidget::currentItem();
  fn _ZNK11QTreeWidget11currentItemEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTreeWidget::itemCollapsed(QTreeWidgetItem * item);
  fn _ZN11QTreeWidget13itemCollapsedEP15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTreeWidgetItem * QTreeWidget::itemAt(int x, int y);
  fn _ZNK11QTreeWidget6itemAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  QTreeWidgetItem * QTreeWidget::itemAt(const QPoint & p);
  fn _ZNK11QTreeWidget6itemAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTreeWidget::setCurrentItem(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QTreeWidget::itemClicked(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget11itemClickedEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  QTreeWidgetItem * QTreeWidget::topLevelItem(int index);
  fn _ZNK11QTreeWidget12topLevelItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QTreeWidget::topLevelItemCount();
  fn _ZNK11QTreeWidget17topLevelItemCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QTreeWidgetItem * QTreeWidget::headerItem();
  fn _ZNK11QTreeWidget10headerItemEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTreeWidget::setFirstItemColumnSpanned(const QTreeWidgetItem * item, bool span);
  fn _ZN11QTreeWidget25setFirstItemColumnSpannedEPK15QTreeWidgetItemb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  void QTreeWidget::removeItemWidget(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget16removeItemWidgetEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  QTreeWidgetItem * QTreeWidget::itemAbove(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget9itemAboveEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTreeWidget::expandItem(const QTreeWidgetItem * item);
  fn _ZN11QTreeWidget10expandItemEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTreeWidget::itemSelectionChanged();
  fn _ZN11QTreeWidget20itemSelectionChangedEv(qthis: *mut c_void);
  // proto:  void QTreeWidget::setHeaderItem(QTreeWidgetItem * item);
  fn _ZN11QTreeWidget13setHeaderItemEP15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTreeWidget::itemChanged(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget11itemChangedEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  QTreeWidgetItem * QTreeWidget::takeTopLevelItem(int index);
  fn _ZN11QTreeWidget16takeTopLevelItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QWidget * QTreeWidget::itemWidget(QTreeWidgetItem * item, int column);
  fn _ZNK11QTreeWidget10itemWidgetEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  void QTreeWidget::editItem(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget8editItemEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QTreeWidget::setItemExpanded(const QTreeWidgetItem * item, bool expand);
  fn _ZN11QTreeWidget15setItemExpandedEPK15QTreeWidgetItemb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  void QTreeWidget::addTopLevelItem(QTreeWidgetItem * item);
  fn _ZN11QTreeWidget15addTopLevelItemEP15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTreeWidget::closePersistentEditor(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget21closePersistentEditorEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QTreeWidget::QTreeWidget(QWidget * parent);
  fn dector_ZN11QTreeWidgetC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QTreeWidgetC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTreeWidget::setSelectionModel(QItemSelectionModel * selectionModel);
  fn _ZN11QTreeWidget17setSelectionModelEP19QItemSelectionModel(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRect QTreeWidget::visualItemRect(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget14visualItemRectEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTreeWidget::setHeaderLabel(const QString & label);
  fn _ZN11QTreeWidget14setHeaderLabelERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTreeWidget::currentItemChanged(QTreeWidgetItem * current, QTreeWidgetItem * previous);
  fn _ZN11QTreeWidget18currentItemChangedEP15QTreeWidgetItemS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QTreeWidget::isItemHidden(const QTreeWidgetItem * item);
  fn _ZNK11QTreeWidget12isItemHiddenEPK15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QTreeWidget::openPersistentEditor(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget20openPersistentEditorEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  int QTreeWidget::columnCount();
  fn _ZNK11QTreeWidget11columnCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTreeWidget::setCurrentItem(QTreeWidgetItem * item);
  fn _ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTreeWidget::setItemSelected(const QTreeWidgetItem * item, bool select);
  fn _ZN11QTreeWidget15setItemSelectedEPK15QTreeWidgetItemb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  void QTreeWidget::itemActivated(QTreeWidgetItem * item, int column);
  fn _ZN11QTreeWidget13itemActivatedEP15QTreeWidgetItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  fn QTreeWidgetItem_Class_Size() -> c_int;
  // proto:  void QTreeWidgetItem::setFirstColumnSpanned(bool span);
  fn _ZN15QTreeWidgetItem21setFirstColumnSpannedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  int QTreeWidgetItem::indexOfChild(QTreeWidgetItem * child);
  fn _ZNK15QTreeWidgetItem12indexOfChildEPS_(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QTreeWidgetItem::setFont(int column, const QFont & font);
  fn _ZN15QTreeWidgetItem7setFontEiRK5QFont(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QTreeWidgetItem::setData(int column, int role, const QVariant & value);
  fn _ZN15QTreeWidgetItem7setDataEiiRK8QVariant(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void);
  // proto:  void QTreeWidgetItem::setStatusTip(int column, const QString & statusTip);
  fn _ZN15QTreeWidgetItem12setStatusTipEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QTreeWidgetItem::setExpanded(bool expand);
  fn _ZN15QTreeWidgetItem11setExpandedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTreeWidgetItem::write(QDataStream & out);
  fn _ZNK15QTreeWidgetItem5writeER11QDataStream(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QTreeWidgetItem::isExpanded();
  fn _ZNK15QTreeWidgetItem10isExpandedEv(qthis: *mut c_void) -> c_char;
  // proto:  QList<QTreeWidgetItem *> QTreeWidgetItem::takeChildren();
  fn _ZN15QTreeWidgetItem12takeChildrenEv(qthis: *mut c_void);
  // proto:  void QTreeWidgetItem::QTreeWidgetItem(QTreeWidgetItem * parent, int type);
  fn dector_ZN15QTreeWidgetItemC1EPS_i(arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  fn _ZN15QTreeWidgetItemC1EPS_i(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QTreeWidgetItem::setIcon(int column, const QIcon & icon);
  fn _ZN15QTreeWidgetItem7setIconEiRK5QIcon(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QTreeWidgetItem::QTreeWidgetItem(QTreeWidgetItem * parent, QTreeWidgetItem * after, int type);
  fn dector_ZN15QTreeWidgetItemC1EPS_S0_i(arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> *mut c_void;
  fn _ZN15QTreeWidgetItemC1EPS_S0_i(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int);
  // proto:  bool QTreeWidgetItem::isHidden();
  fn _ZNK15QTreeWidgetItem8isHiddenEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTreeWidgetItem::QTreeWidgetItem(QTreeWidget * view, QTreeWidgetItem * after, int type);
  fn dector_ZN15QTreeWidgetItemC1EP11QTreeWidgetPS_i(arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> *mut c_void;
  fn _ZN15QTreeWidgetItemC1EP11QTreeWidgetPS_i(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int);
  // proto:  void QTreeWidgetItem::insertChild(int index, QTreeWidgetItem * child);
  fn _ZN15QTreeWidgetItem11insertChildEiPS_(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QTreeWidgetItem::QTreeWidgetItem(const QTreeWidgetItem & other);
  fn dector_ZN15QTreeWidgetItemC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN15QTreeWidgetItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QTreeWidgetItem::isDisabled();
  fn _ZNK15QTreeWidgetItem10isDisabledEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTreeWidgetItem::setText(int column, const QString & text);
  fn _ZN15QTreeWidgetItem7setTextEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QTreeWidgetItem::read(QDataStream & in);
  fn _ZN15QTreeWidgetItem4readER11QDataStream(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTreeWidgetItem::QTreeWidgetItem(int type);
  fn dector_ZN15QTreeWidgetItemC1Ei(arg0: c_int) -> *mut c_void;
  fn _ZN15QTreeWidgetItemC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  QVariant QTreeWidgetItem::data(int column, int role);
  fn _ZNK15QTreeWidgetItem4dataEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QTreeWidgetItem::setToolTip(int column, const QString & toolTip);
  fn _ZN15QTreeWidgetItem10setToolTipEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QTreeWidgetItem::QTreeWidgetItem(QTreeWidget * view, const QStringList & strings, int type);
  fn dector_ZN15QTreeWidgetItemC1EP11QTreeWidgetRK11QStringListi(arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> *mut c_void;
  fn _ZN15QTreeWidgetItemC1EP11QTreeWidgetRK11QStringListi(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int);
  // proto:  bool QTreeWidgetItem::isFirstColumnSpanned();
  fn _ZNK15QTreeWidgetItem20isFirstColumnSpannedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTreeWidgetItem::QTreeWidgetItem(const QStringList & strings, int type);
  fn dector_ZN15QTreeWidgetItemC1ERK11QStringListi(arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  fn _ZN15QTreeWidgetItemC1ERK11QStringListi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QTreeWidgetItem::setSelected(bool select);
  fn _ZN15QTreeWidgetItem11setSelectedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTreeWidgetItem::~QTreeWidgetItem();
  fn _ZN15QTreeWidgetItemD0Ev(qthis: *mut c_void);
  // proto:  void QTreeWidgetItem::setHidden(bool hide);
  fn _ZN15QTreeWidgetItem9setHiddenEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QTreeWidgetItem * QTreeWidgetItem::takeChild(int index);
  fn _ZN15QTreeWidgetItem9takeChildEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTreeWidgetItem::QTreeWidgetItem(QTreeWidgetItem * parent, const QStringList & strings, int type);
  fn dector_ZN15QTreeWidgetItemC1EPS_RK11QStringListi(arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> *mut c_void;
  fn _ZN15QTreeWidgetItemC1EPS_RK11QStringListi(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int);
  // proto:  void QTreeWidgetItem::setDisabled(bool disabled);
  fn _ZN15QTreeWidgetItem11setDisabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTreeWidgetItem::removeChild(QTreeWidgetItem * child);
  fn _ZN15QTreeWidgetItem11removeChildEPS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTreeWidgetItem * QTreeWidgetItem::clone();
  fn _ZNK15QTreeWidgetItem5cloneEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTreeWidgetItem::QTreeWidgetItem(QTreeWidget * view, int type);
  fn dector_ZN15QTreeWidgetItemC1EP11QTreeWidgeti(arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  fn _ZN15QTreeWidgetItemC1EP11QTreeWidgeti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QTreeWidgetItem::addChild(QTreeWidgetItem * child);
  fn _ZN15QTreeWidgetItem8addChildEPS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTreeWidgetItem::setWhatsThis(int column, const QString & whatsThis);
  fn _ZN15QTreeWidgetItem12setWhatsThisEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  bool QTreeWidgetItem::isSelected();
  fn _ZNK15QTreeWidgetItem10isSelectedEv(qthis: *mut c_void) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QTreeWidget)=1
pub struct QTreeWidget {
  qbase: QTreeView,
  pub qclsinst: *mut c_void,
}

// class sizeof(QTreeWidgetItem)=1
pub struct QTreeWidgetItem {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTreeWidget {
  pub fn inheritFrom(qthis: *mut c_void) -> QTreeWidget {
    return QTreeWidget{qbase: QTreeView::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QTreeWidget {
  type Target = QTreeView;

  fn deref(&self) -> &QTreeView {
    return & self.qbase;
  }
}
impl AsRef<QTreeView> for QTreeWidget {
  fn as_ref(& self) -> & QTreeView {
    return & self.qbase;
  }
}
  // proto:  void QTreeWidget::itemDoubleClicked(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn itemDoubleClicked<RetType, T: QTreeWidget_itemDoubleClicked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemDoubleClicked(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemDoubleClicked<RetType> {
  fn itemDoubleClicked(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::itemDoubleClicked(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemDoubleClicked<()> for (&'a QTreeWidgetItem, i32) {
  fn itemDoubleClicked(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget17itemDoubleClickedEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget17itemDoubleClickedEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::itemExpanded(QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn itemExpanded<RetType, T: QTreeWidget_itemExpanded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemExpanded(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemExpanded<RetType> {
  fn itemExpanded(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::itemExpanded(QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_itemExpanded<()> for (&'a QTreeWidgetItem) {
  fn itemExpanded(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget12itemExpandedEP15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget12itemExpandedEP15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::setColumnCount(int columns);
impl /*struct*/ QTreeWidget {
  pub fn setColumnCount<RetType, T: QTreeWidget_setColumnCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColumnCount(self);
    // return 1;
  }
}

pub trait QTreeWidget_setColumnCount<RetType> {
  fn setColumnCount(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setColumnCount(int columns);
impl<'a> /*trait*/ QTreeWidget_setColumnCount<()> for (i32) {
  fn setColumnCount(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget14setColumnCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QTreeWidget14setColumnCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::~QTreeWidget();
impl /*struct*/ QTreeWidget {
  pub fn Free<RetType, T: QTreeWidget_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTreeWidget_Free<RetType> {
  fn Free(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::~QTreeWidget();
impl<'a> /*trait*/ QTreeWidget_Free<()> for () {
  fn Free(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidgetD0Ev()};
     unsafe {_ZN11QTreeWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QTreeWidgetItem *> QTreeWidget::selectedItems();
impl /*struct*/ QTreeWidget {
  pub fn selectedItems<RetType, T: QTreeWidget_selectedItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectedItems(self);
    // return 1;
  }
}

pub trait QTreeWidget_selectedItems<RetType> {
  fn selectedItems(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  QList<QTreeWidgetItem *> QTreeWidget::selectedItems();
impl<'a> /*trait*/ QTreeWidget_selectedItems<()> for () {
  fn selectedItems(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget13selectedItemsEv()};
     unsafe {_ZNK11QTreeWidget13selectedItemsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTreeWidget::isItemExpanded(const QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn isItemExpanded<RetType, T: QTreeWidget_isItemExpanded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isItemExpanded(self);
    // return 1;
  }
}

pub trait QTreeWidget_isItemExpanded<RetType> {
  fn isItemExpanded(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  bool QTreeWidget::isItemExpanded(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_isItemExpanded<i8> for (&'a QTreeWidgetItem) {
  fn isItemExpanded(self , rsthis: & QTreeWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget14isItemExpandedEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget14isItemExpandedEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTreeWidget::QTreeWidget(const QTreeWidget & );
impl /*struct*/ QTreeWidget {
  pub fn New<T: QTreeWidget_New>(value: T) -> QTreeWidget {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeWidget_New {
  fn New(self) -> QTreeWidget;
}

  // proto:  void QTreeWidget::QTreeWidget(const QTreeWidget & );
impl<'a> /*trait*/ QTreeWidget_New for (&'a QTreeWidget) {
  fn New(self) -> QTreeWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidgetC1ERKS_()};
    let ctysz: c_int = unsafe{QTreeWidget_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QTreeWidgetC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QTreeWidgetC1ERKS_(arg0)};
    let rsthis = QTreeWidget{/**/qbase: QTreeView::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTreeWidget::setItemHidden(const QTreeWidgetItem * item, bool hide);
impl /*struct*/ QTreeWidget {
  pub fn setItemHidden<RetType, T: QTreeWidget_setItemHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemHidden(self);
    // return 1;
  }
}

pub trait QTreeWidget_setItemHidden<RetType> {
  fn setItemHidden(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setItemHidden(const QTreeWidgetItem * item, bool hide);
impl<'a> /*trait*/ QTreeWidget_setItemHidden<()> for (&'a QTreeWidgetItem, i8) {
  fn setItemHidden(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget13setItemHiddenEPK15QTreeWidgetItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {_ZN11QTreeWidget13setItemHiddenEPK15QTreeWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QTreeWidget::indexOfTopLevelItem(QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn indexOfTopLevelItem<RetType, T: QTreeWidget_indexOfTopLevelItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexOfTopLevelItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_indexOfTopLevelItem<RetType> {
  fn indexOfTopLevelItem(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  int QTreeWidget::indexOfTopLevelItem(QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_indexOfTopLevelItem<i32> for (&'a QTreeWidgetItem) {
  fn indexOfTopLevelItem(self , rsthis: & QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget19indexOfTopLevelItemEP15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget19indexOfTopLevelItemEP15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTreeWidget::insertTopLevelItem(int index, QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn insertTopLevelItem<RetType, T: QTreeWidget_insertTopLevelItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertTopLevelItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_insertTopLevelItem<RetType> {
  fn insertTopLevelItem(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::insertTopLevelItem(int index, QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_insertTopLevelItem<()> for (i32, &'a QTreeWidgetItem) {
  fn insertTopLevelItem(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget18insertTopLevelItemEiP15QTreeWidgetItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget18insertTopLevelItemEiP15QTreeWidgetItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::setItemWidget(QTreeWidgetItem * item, int column, QWidget * widget);
impl /*struct*/ QTreeWidget {
  pub fn setItemWidget<RetType, T: QTreeWidget_setItemWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemWidget(self);
    // return 1;
  }
}

pub trait QTreeWidget_setItemWidget<RetType> {
  fn setItemWidget(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setItemWidget(QTreeWidgetItem * item, int column, QWidget * widget);
impl<'a> /*trait*/ QTreeWidget_setItemWidget<()> for (&'a QTreeWidgetItem, i32, &'a QWidget) {
  fn setItemWidget(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget13setItemWidgetEP15QTreeWidgetItemiP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget13setItemWidgetEP15QTreeWidgetItemiP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  bool QTreeWidget::isItemSelected(const QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn isItemSelected<RetType, T: QTreeWidget_isItemSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isItemSelected(self);
    // return 1;
  }
}

pub trait QTreeWidget_isItemSelected<RetType> {
  fn isItemSelected(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  bool QTreeWidget::isItemSelected(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_isItemSelected<i8> for (&'a QTreeWidgetItem) {
  fn isItemSelected(self , rsthis: & QTreeWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget14isItemSelectedEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget14isItemSelectedEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QTreeWidget::currentColumn();
impl /*struct*/ QTreeWidget {
  pub fn currentColumn<RetType, T: QTreeWidget_currentColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentColumn(self);
    // return 1;
  }
}

pub trait QTreeWidget_currentColumn<RetType> {
  fn currentColumn(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  int QTreeWidget::currentColumn();
impl<'a> /*trait*/ QTreeWidget_currentColumn<i32> for () {
  fn currentColumn(self , rsthis: & QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget13currentColumnEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget13currentColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QTreeWidget::isFirstItemColumnSpanned(const QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn isFirstItemColumnSpanned<RetType, T: QTreeWidget_isFirstItemColumnSpanned<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFirstItemColumnSpanned(self);
    // return 1;
  }
}

pub trait QTreeWidget_isFirstItemColumnSpanned<RetType> {
  fn isFirstItemColumnSpanned(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  bool QTreeWidget::isFirstItemColumnSpanned(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_isFirstItemColumnSpanned<i8> for (&'a QTreeWidgetItem) {
  fn isFirstItemColumnSpanned(self , rsthis: & QTreeWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget24isFirstItemColumnSpannedEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget24isFirstItemColumnSpannedEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTreeWidget::clear();
impl /*struct*/ QTreeWidget {
  pub fn clear<RetType, T: QTreeWidget_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QTreeWidget_clear<RetType> {
  fn clear(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::clear();
impl<'a> /*trait*/ QTreeWidget_clear<()> for () {
  fn clear(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget5clearEv()};
     unsafe {_ZN11QTreeWidget5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::itemPressed(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn itemPressed<RetType, T: QTreeWidget_itemPressed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemPressed(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemPressed<RetType> {
  fn itemPressed(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::itemPressed(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemPressed<()> for (&'a QTreeWidgetItem, i32) {
  fn itemPressed(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget11itemPressedEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget11itemPressedEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::setHeaderLabels(const QStringList & labels);
impl /*struct*/ QTreeWidget {
  pub fn setHeaderLabels<RetType, T: QTreeWidget_setHeaderLabels<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHeaderLabels(self);
    // return 1;
  }
}

pub trait QTreeWidget_setHeaderLabels<RetType> {
  fn setHeaderLabels(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setHeaderLabels(const QStringList & labels);
impl<'a> /*trait*/ QTreeWidget_setHeaderLabels<()> for (&'a QStringList) {
  fn setHeaderLabels(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget15setHeaderLabelsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget15setHeaderLabelsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTreeWidgetItem * QTreeWidget::invisibleRootItem();
impl /*struct*/ QTreeWidget {
  pub fn invisibleRootItem<RetType, T: QTreeWidget_invisibleRootItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.invisibleRootItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_invisibleRootItem<RetType> {
  fn invisibleRootItem(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  QTreeWidgetItem * QTreeWidget::invisibleRootItem();
impl<'a> /*trait*/ QTreeWidget_invisibleRootItem<QTreeWidgetItem> for () {
  fn invisibleRootItem(self , rsthis: & QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget17invisibleRootItemEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget17invisibleRootItemEv(rsthis.qclsinst)};
    let mut ret1 = QTreeWidgetItem::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QTreeWidget::metaObject();
impl /*struct*/ QTreeWidget {
  pub fn metaObject<RetType, T: QTreeWidget_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTreeWidget_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  const QMetaObject * QTreeWidget::metaObject();
impl<'a> /*trait*/ QTreeWidget_metaObject<()> for () {
  fn metaObject(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget10metaObjectEv()};
     unsafe {_ZNK11QTreeWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::itemEntered(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn itemEntered<RetType, T: QTreeWidget_itemEntered<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemEntered(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemEntered<RetType> {
  fn itemEntered(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::itemEntered(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemEntered<()> for (&'a QTreeWidgetItem, i32) {
  fn itemEntered(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget11itemEnteredEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget11itemEnteredEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QTreeWidgetItem * QTreeWidget::itemBelow(const QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn itemBelow<RetType, T: QTreeWidget_itemBelow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemBelow(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemBelow<RetType> {
  fn itemBelow(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  QTreeWidgetItem * QTreeWidget::itemBelow(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_itemBelow<QTreeWidgetItem> for (&'a QTreeWidgetItem) {
  fn itemBelow(self , rsthis: & QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget9itemBelowEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget9itemBelowEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QTreeWidgetItem::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTreeWidget::sortColumn();
impl /*struct*/ QTreeWidget {
  pub fn sortColumn<RetType, T: QTreeWidget_sortColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sortColumn(self);
    // return 1;
  }
}

pub trait QTreeWidget_sortColumn<RetType> {
  fn sortColumn(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  int QTreeWidget::sortColumn();
impl<'a> /*trait*/ QTreeWidget_sortColumn<i32> for () {
  fn sortColumn(self , rsthis: & QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget10sortColumnEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget10sortColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTreeWidget::collapseItem(const QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn collapseItem<RetType, T: QTreeWidget_collapseItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.collapseItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_collapseItem<RetType> {
  fn collapseItem(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::collapseItem(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_collapseItem<()> for (&'a QTreeWidgetItem) {
  fn collapseItem(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget12collapseItemEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget12collapseItemEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTreeWidgetItem * QTreeWidget::currentItem();
impl /*struct*/ QTreeWidget {
  pub fn currentItem<RetType, T: QTreeWidget_currentItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_currentItem<RetType> {
  fn currentItem(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  QTreeWidgetItem * QTreeWidget::currentItem();
impl<'a> /*trait*/ QTreeWidget_currentItem<QTreeWidgetItem> for () {
  fn currentItem(self , rsthis: & QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget11currentItemEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget11currentItemEv(rsthis.qclsinst)};
    let mut ret1 = QTreeWidgetItem::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTreeWidget::itemCollapsed(QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn itemCollapsed<RetType, T: QTreeWidget_itemCollapsed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemCollapsed(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemCollapsed<RetType> {
  fn itemCollapsed(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::itemCollapsed(QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_itemCollapsed<()> for (&'a QTreeWidgetItem) {
  fn itemCollapsed(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget13itemCollapsedEP15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget13itemCollapsedEP15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTreeWidgetItem * QTreeWidget::itemAt(int x, int y);
impl /*struct*/ QTreeWidget {
  pub fn itemAt<RetType, T: QTreeWidget_itemAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemAt(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemAt<RetType> {
  fn itemAt(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  QTreeWidgetItem * QTreeWidget::itemAt(int x, int y);
impl<'a> /*trait*/ QTreeWidget_itemAt<QTreeWidgetItem> for (i32, i32) {
  fn itemAt(self , rsthis: & QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget6itemAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QTreeWidget6itemAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTreeWidgetItem::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QTreeWidgetItem * QTreeWidget::itemAt(const QPoint & p);
impl<'a> /*trait*/ QTreeWidget_itemAt<QTreeWidgetItem> for (&'a QPoint) {
  fn itemAt(self , rsthis: & QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget6itemAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget6itemAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QTreeWidgetItem::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTreeWidget::setCurrentItem(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn setCurrentItem<RetType, T: QTreeWidget_setCurrentItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_setCurrentItem<RetType> {
  fn setCurrentItem(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setCurrentItem(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_setCurrentItem<()> for (&'a QTreeWidgetItem, i32) {
  fn setCurrentItem(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::itemClicked(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn itemClicked<RetType, T: QTreeWidget_itemClicked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemClicked(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemClicked<RetType> {
  fn itemClicked(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::itemClicked(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemClicked<()> for (&'a QTreeWidgetItem, i32) {
  fn itemClicked(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget11itemClickedEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget11itemClickedEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QTreeWidgetItem * QTreeWidget::topLevelItem(int index);
impl /*struct*/ QTreeWidget {
  pub fn topLevelItem<RetType, T: QTreeWidget_topLevelItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.topLevelItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_topLevelItem<RetType> {
  fn topLevelItem(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  QTreeWidgetItem * QTreeWidget::topLevelItem(int index);
impl<'a> /*trait*/ QTreeWidget_topLevelItem<QTreeWidgetItem> for (i32) {
  fn topLevelItem(self , rsthis: & QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget12topLevelItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTreeWidget12topLevelItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTreeWidgetItem::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTreeWidget::topLevelItemCount();
impl /*struct*/ QTreeWidget {
  pub fn topLevelItemCount<RetType, T: QTreeWidget_topLevelItemCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.topLevelItemCount(self);
    // return 1;
  }
}

pub trait QTreeWidget_topLevelItemCount<RetType> {
  fn topLevelItemCount(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  int QTreeWidget::topLevelItemCount();
impl<'a> /*trait*/ QTreeWidget_topLevelItemCount<i32> for () {
  fn topLevelItemCount(self , rsthis: & QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget17topLevelItemCountEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget17topLevelItemCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QTreeWidgetItem * QTreeWidget::headerItem();
impl /*struct*/ QTreeWidget {
  pub fn headerItem<RetType, T: QTreeWidget_headerItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.headerItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_headerItem<RetType> {
  fn headerItem(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  QTreeWidgetItem * QTreeWidget::headerItem();
impl<'a> /*trait*/ QTreeWidget_headerItem<QTreeWidgetItem> for () {
  fn headerItem(self , rsthis: & QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget10headerItemEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget10headerItemEv(rsthis.qclsinst)};
    let mut ret1 = QTreeWidgetItem::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTreeWidget::setFirstItemColumnSpanned(const QTreeWidgetItem * item, bool span);
impl /*struct*/ QTreeWidget {
  pub fn setFirstItemColumnSpanned<RetType, T: QTreeWidget_setFirstItemColumnSpanned<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFirstItemColumnSpanned(self);
    // return 1;
  }
}

pub trait QTreeWidget_setFirstItemColumnSpanned<RetType> {
  fn setFirstItemColumnSpanned(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setFirstItemColumnSpanned(const QTreeWidgetItem * item, bool span);
impl<'a> /*trait*/ QTreeWidget_setFirstItemColumnSpanned<()> for (&'a QTreeWidgetItem, i8) {
  fn setFirstItemColumnSpanned(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget25setFirstItemColumnSpannedEPK15QTreeWidgetItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {_ZN11QTreeWidget25setFirstItemColumnSpannedEPK15QTreeWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::removeItemWidget(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn removeItemWidget<RetType, T: QTreeWidget_removeItemWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeItemWidget(self);
    // return 1;
  }
}

pub trait QTreeWidget_removeItemWidget<RetType> {
  fn removeItemWidget(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::removeItemWidget(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_removeItemWidget<()> for (&'a QTreeWidgetItem, i32) {
  fn removeItemWidget(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget16removeItemWidgetEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget16removeItemWidgetEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QTreeWidgetItem * QTreeWidget::itemAbove(const QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn itemAbove<RetType, T: QTreeWidget_itemAbove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemAbove(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemAbove<RetType> {
  fn itemAbove(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  QTreeWidgetItem * QTreeWidget::itemAbove(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_itemAbove<QTreeWidgetItem> for (&'a QTreeWidgetItem) {
  fn itemAbove(self , rsthis: & QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget9itemAboveEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget9itemAboveEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QTreeWidgetItem::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTreeWidget::expandItem(const QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn expandItem<RetType, T: QTreeWidget_expandItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.expandItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_expandItem<RetType> {
  fn expandItem(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::expandItem(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_expandItem<()> for (&'a QTreeWidgetItem) {
  fn expandItem(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget10expandItemEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget10expandItemEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::itemSelectionChanged();
impl /*struct*/ QTreeWidget {
  pub fn itemSelectionChanged<RetType, T: QTreeWidget_itemSelectionChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemSelectionChanged(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemSelectionChanged<RetType> {
  fn itemSelectionChanged(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::itemSelectionChanged();
impl<'a> /*trait*/ QTreeWidget_itemSelectionChanged<()> for () {
  fn itemSelectionChanged(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget20itemSelectionChangedEv()};
     unsafe {_ZN11QTreeWidget20itemSelectionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::setHeaderItem(QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn setHeaderItem<RetType, T: QTreeWidget_setHeaderItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHeaderItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_setHeaderItem<RetType> {
  fn setHeaderItem(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setHeaderItem(QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_setHeaderItem<()> for (&'a QTreeWidgetItem) {
  fn setHeaderItem(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget13setHeaderItemEP15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget13setHeaderItemEP15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::itemChanged(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn itemChanged<RetType, T: QTreeWidget_itemChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemChanged(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemChanged<RetType> {
  fn itemChanged(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::itemChanged(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemChanged<()> for (&'a QTreeWidgetItem, i32) {
  fn itemChanged(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget11itemChangedEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget11itemChangedEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QTreeWidgetItem * QTreeWidget::takeTopLevelItem(int index);
impl /*struct*/ QTreeWidget {
  pub fn takeTopLevelItem<RetType, T: QTreeWidget_takeTopLevelItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeTopLevelItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_takeTopLevelItem<RetType> {
  fn takeTopLevelItem(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  QTreeWidgetItem * QTreeWidget::takeTopLevelItem(int index);
impl<'a> /*trait*/ QTreeWidget_takeTopLevelItem<QTreeWidgetItem> for (i32) {
  fn takeTopLevelItem(self , rsthis: & QTreeWidget) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget16takeTopLevelItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN11QTreeWidget16takeTopLevelItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTreeWidgetItem::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QTreeWidget::itemWidget(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn itemWidget<RetType, T: QTreeWidget_itemWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemWidget(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemWidget<RetType> {
  fn itemWidget(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  QWidget * QTreeWidget::itemWidget(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemWidget<QWidget> for (&'a QTreeWidgetItem, i32) {
  fn itemWidget(self , rsthis: & QTreeWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget10itemWidgetEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QTreeWidget10itemWidgetEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTreeWidget::editItem(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn editItem<RetType, T: QTreeWidget_editItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.editItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_editItem<RetType> {
  fn editItem(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::editItem(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_editItem<()> for (&'a QTreeWidgetItem, i32) {
  fn editItem(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget8editItemEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget8editItemEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::setItemExpanded(const QTreeWidgetItem * item, bool expand);
impl /*struct*/ QTreeWidget {
  pub fn setItemExpanded<RetType, T: QTreeWidget_setItemExpanded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemExpanded(self);
    // return 1;
  }
}

pub trait QTreeWidget_setItemExpanded<RetType> {
  fn setItemExpanded(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setItemExpanded(const QTreeWidgetItem * item, bool expand);
impl<'a> /*trait*/ QTreeWidget_setItemExpanded<()> for (&'a QTreeWidgetItem, i8) {
  fn setItemExpanded(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget15setItemExpandedEPK15QTreeWidgetItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {_ZN11QTreeWidget15setItemExpandedEPK15QTreeWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::addTopLevelItem(QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn addTopLevelItem<RetType, T: QTreeWidget_addTopLevelItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addTopLevelItem(self);
    // return 1;
  }
}

pub trait QTreeWidget_addTopLevelItem<RetType> {
  fn addTopLevelItem(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::addTopLevelItem(QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_addTopLevelItem<()> for (&'a QTreeWidgetItem) {
  fn addTopLevelItem(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget15addTopLevelItemEP15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget15addTopLevelItemEP15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::closePersistentEditor(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn closePersistentEditor<RetType, T: QTreeWidget_closePersistentEditor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.closePersistentEditor(self);
    // return 1;
  }
}

pub trait QTreeWidget_closePersistentEditor<RetType> {
  fn closePersistentEditor(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::closePersistentEditor(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_closePersistentEditor<()> for (&'a QTreeWidgetItem, i32) {
  fn closePersistentEditor(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget21closePersistentEditorEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget21closePersistentEditorEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::QTreeWidget(QWidget * parent);
impl<'a> /*trait*/ QTreeWidget_New for (&'a QWidget) {
  fn New(self) -> QTreeWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidgetC1EP7QWidget()};
    let ctysz: c_int = unsafe{QTreeWidget_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QTreeWidgetC1EP7QWidget(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QTreeWidgetC1EP7QWidget(arg0)};
    let rsthis = QTreeWidget{/**/qbase: QTreeView::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTreeWidget::setSelectionModel(QItemSelectionModel * selectionModel);
impl /*struct*/ QTreeWidget {
  pub fn setSelectionModel<RetType, T: QTreeWidget_setSelectionModel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSelectionModel(self);
    // return 1;
  }
}

pub trait QTreeWidget_setSelectionModel<RetType> {
  fn setSelectionModel(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setSelectionModel(QItemSelectionModel * selectionModel);
impl<'a> /*trait*/ QTreeWidget_setSelectionModel<()> for (&'a QItemSelectionModel) {
  fn setSelectionModel(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget17setSelectionModelEP19QItemSelectionModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget17setSelectionModelEP19QItemSelectionModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QTreeWidget::visualItemRect(const QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn visualItemRect<RetType, T: QTreeWidget_visualItemRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.visualItemRect(self);
    // return 1;
  }
}

pub trait QTreeWidget_visualItemRect<RetType> {
  fn visualItemRect(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  QRect QTreeWidget::visualItemRect(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_visualItemRect<QRect> for (&'a QTreeWidgetItem) {
  fn visualItemRect(self , rsthis: & QTreeWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget14visualItemRectEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget14visualItemRectEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTreeWidget::setHeaderLabel(const QString & label);
impl /*struct*/ QTreeWidget {
  pub fn setHeaderLabel<RetType, T: QTreeWidget_setHeaderLabel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHeaderLabel(self);
    // return 1;
  }
}

pub trait QTreeWidget_setHeaderLabel<RetType> {
  fn setHeaderLabel(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setHeaderLabel(const QString & label);
impl<'a> /*trait*/ QTreeWidget_setHeaderLabel<()> for (&'a QString) {
  fn setHeaderLabel(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget14setHeaderLabelERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget14setHeaderLabelERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::currentItemChanged(QTreeWidgetItem * current, QTreeWidgetItem * previous);
impl /*struct*/ QTreeWidget {
  pub fn currentItemChanged<RetType, T: QTreeWidget_currentItemChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentItemChanged(self);
    // return 1;
  }
}

pub trait QTreeWidget_currentItemChanged<RetType> {
  fn currentItemChanged(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::currentItemChanged(QTreeWidgetItem * current, QTreeWidgetItem * previous);
impl<'a> /*trait*/ QTreeWidget_currentItemChanged<()> for (&'a QTreeWidgetItem, &'a QTreeWidgetItem) {
  fn currentItemChanged(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget18currentItemChangedEP15QTreeWidgetItemS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget18currentItemChangedEP15QTreeWidgetItemS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QTreeWidget::isItemHidden(const QTreeWidgetItem * item);
impl /*struct*/ QTreeWidget {
  pub fn isItemHidden<RetType, T: QTreeWidget_isItemHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isItemHidden(self);
    // return 1;
  }
}

pub trait QTreeWidget_isItemHidden<RetType> {
  fn isItemHidden(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  bool QTreeWidget::isItemHidden(const QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_isItemHidden<i8> for (&'a QTreeWidgetItem) {
  fn isItemHidden(self , rsthis: & QTreeWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget12isItemHiddenEPK15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QTreeWidget12isItemHiddenEPK15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTreeWidget::openPersistentEditor(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn openPersistentEditor<RetType, T: QTreeWidget_openPersistentEditor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.openPersistentEditor(self);
    // return 1;
  }
}

pub trait QTreeWidget_openPersistentEditor<RetType> {
  fn openPersistentEditor(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::openPersistentEditor(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_openPersistentEditor<()> for (&'a QTreeWidgetItem, i32) {
  fn openPersistentEditor(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget20openPersistentEditorEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget20openPersistentEditorEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QTreeWidget::columnCount();
impl /*struct*/ QTreeWidget {
  pub fn columnCount<RetType, T: QTreeWidget_columnCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QTreeWidget_columnCount<RetType> {
  fn columnCount(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  int QTreeWidget::columnCount();
impl<'a> /*trait*/ QTreeWidget_columnCount<i32> for () {
  fn columnCount(self , rsthis: & QTreeWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTreeWidget11columnCountEv()};
    let mut ret = unsafe {_ZNK11QTreeWidget11columnCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTreeWidget::setCurrentItem(QTreeWidgetItem * item);
impl<'a> /*trait*/ QTreeWidget_setCurrentItem<()> for (&'a QTreeWidgetItem) {
  fn setCurrentItem(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTreeWidget14setCurrentItemEP15QTreeWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::setItemSelected(const QTreeWidgetItem * item, bool select);
impl /*struct*/ QTreeWidget {
  pub fn setItemSelected<RetType, T: QTreeWidget_setItemSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemSelected(self);
    // return 1;
  }
}

pub trait QTreeWidget_setItemSelected<RetType> {
  fn setItemSelected(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::setItemSelected(const QTreeWidgetItem * item, bool select);
impl<'a> /*trait*/ QTreeWidget_setItemSelected<()> for (&'a QTreeWidgetItem, i8) {
  fn setItemSelected(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget15setItemSelectedEPK15QTreeWidgetItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {_ZN11QTreeWidget15setItemSelectedEPK15QTreeWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidget::itemActivated(QTreeWidgetItem * item, int column);
impl /*struct*/ QTreeWidget {
  pub fn itemActivated<RetType, T: QTreeWidget_itemActivated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemActivated(self);
    // return 1;
  }
}

pub trait QTreeWidget_itemActivated<RetType> {
  fn itemActivated(self , rsthis: & QTreeWidget) -> RetType;
}

  // proto:  void QTreeWidget::itemActivated(QTreeWidgetItem * item, int column);
impl<'a> /*trait*/ QTreeWidget_itemActivated<()> for (&'a QTreeWidgetItem, i32) {
  fn itemActivated(self , rsthis: & QTreeWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTreeWidget13itemActivatedEP15QTreeWidgetItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QTreeWidget13itemActivatedEP15QTreeWidgetItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTreeWidgetItem {
  pub fn inheritFrom(qthis: *mut c_void) -> QTreeWidgetItem {
    return QTreeWidgetItem{qclsinst: qthis};
  }
}
  // proto:  void QTreeWidgetItem::setFirstColumnSpanned(bool span);
impl /*struct*/ QTreeWidgetItem {
  pub fn setFirstColumnSpanned<RetType, T: QTreeWidgetItem_setFirstColumnSpanned<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFirstColumnSpanned(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setFirstColumnSpanned<RetType> {
  fn setFirstColumnSpanned(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  void QTreeWidgetItem::setFirstColumnSpanned(bool span);
impl<'a> /*trait*/ QTreeWidgetItem_setFirstColumnSpanned<()> for (i8) {
  fn setFirstColumnSpanned(self , rsthis: & QTreeWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem21setFirstColumnSpannedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QTreeWidgetItem21setFirstColumnSpannedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTreeWidgetItem::indexOfChild(QTreeWidgetItem * child);
impl /*struct*/ QTreeWidgetItem {
  pub fn indexOfChild<RetType, T: QTreeWidgetItem_indexOfChild<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexOfChild(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_indexOfChild<RetType> {
  fn indexOfChild(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  int QTreeWidgetItem::indexOfChild(QTreeWidgetItem * child);
impl<'a> /*trait*/ QTreeWidgetItem_indexOfChild<i32> for (&'a QTreeWidgetItem) {
  fn indexOfChild(self , rsthis: & QTreeWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem12indexOfChildEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK15QTreeWidgetItem12indexOfChildEPS_(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::setFont(int column, const QFont & font);
impl /*struct*/ QTreeWidgetItem {
  pub fn setFont<RetType, T: QTreeWidgetItem_setFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFont(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setFont<RetType> {
  fn setFont(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  void QTreeWidgetItem::setFont(int column, const QFont & font);
impl<'a> /*trait*/ QTreeWidgetItem_setFont<()> for (i32, &'a QFont) {
  fn setFont(self , rsthis: & QTreeWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem7setFontEiRK5QFont()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem7setFontEiRK5QFont(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::setData(int column, int role, const QVariant & value);
impl /*struct*/ QTreeWidgetItem {
  pub fn setData<RetType, T: QTreeWidgetItem_setData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setData<RetType> {
  fn setData(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  void QTreeWidgetItem::setData(int column, int role, const QVariant & value);
impl<'a> /*trait*/ QTreeWidgetItem_setData<()> for (i32, i32, &'a QVariant) {
  fn setData(self , rsthis: & QTreeWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem7setDataEiiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem7setDataEiiRK8QVariant(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::setStatusTip(int column, const QString & statusTip);
impl /*struct*/ QTreeWidgetItem {
  pub fn setStatusTip<RetType, T: QTreeWidgetItem_setStatusTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStatusTip(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setStatusTip<RetType> {
  fn setStatusTip(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  void QTreeWidgetItem::setStatusTip(int column, const QString & statusTip);
impl<'a> /*trait*/ QTreeWidgetItem_setStatusTip<()> for (i32, &'a QString) {
  fn setStatusTip(self , rsthis: & QTreeWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem12setStatusTipEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem12setStatusTipEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::setExpanded(bool expand);
impl /*struct*/ QTreeWidgetItem {
  pub fn setExpanded<RetType, T: QTreeWidgetItem_setExpanded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setExpanded(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setExpanded<RetType> {
  fn setExpanded(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  void QTreeWidgetItem::setExpanded(bool expand);
impl<'a> /*trait*/ QTreeWidgetItem_setExpanded<()> for (i8) {
  fn setExpanded(self , rsthis: & QTreeWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem11setExpandedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QTreeWidgetItem11setExpandedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::write(QDataStream & out);
impl /*struct*/ QTreeWidgetItem {
  pub fn write<RetType, T: QTreeWidgetItem_write<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.write(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_write<RetType> {
  fn write(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  void QTreeWidgetItem::write(QDataStream & out);
impl<'a> /*trait*/ QTreeWidgetItem_write<()> for (&'a QDataStream) {
  fn write(self , rsthis: & QTreeWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem5writeER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK15QTreeWidgetItem5writeER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTreeWidgetItem::isExpanded();
impl /*struct*/ QTreeWidgetItem {
  pub fn isExpanded<RetType, T: QTreeWidgetItem_isExpanded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isExpanded(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_isExpanded<RetType> {
  fn isExpanded(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  bool QTreeWidgetItem::isExpanded();
impl<'a> /*trait*/ QTreeWidgetItem_isExpanded<i8> for () {
  fn isExpanded(self , rsthis: & QTreeWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem10isExpandedEv()};
    let mut ret = unsafe {_ZNK15QTreeWidgetItem10isExpandedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QList<QTreeWidgetItem *> QTreeWidgetItem::takeChildren();
impl /*struct*/ QTreeWidgetItem {
  pub fn takeChildren<RetType, T: QTreeWidgetItem_takeChildren<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeChildren(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_takeChildren<RetType> {
  fn takeChildren(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  QList<QTreeWidgetItem *> QTreeWidgetItem::takeChildren();
impl<'a> /*trait*/ QTreeWidgetItem_takeChildren<()> for () {
  fn takeChildren(self , rsthis: & QTreeWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem12takeChildrenEv()};
     unsafe {_ZN15QTreeWidgetItem12takeChildrenEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::QTreeWidgetItem(QTreeWidgetItem * parent, int type);
impl /*struct*/ QTreeWidgetItem {
  pub fn New<T: QTreeWidgetItem_New>(value: T) -> QTreeWidgetItem {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeWidgetItem_New {
  fn New(self) -> QTreeWidgetItem;
}

  // proto:  void QTreeWidgetItem::QTreeWidgetItem(QTreeWidgetItem * parent, int type);
impl<'a> /*trait*/ QTreeWidgetItem_New for (&'a QTreeWidgetItem, i32) {
  fn New(self) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1EPS_i()};
    let ctysz: c_int = unsafe{QTreeWidgetItem_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    // unsafe {_ZN15QTreeWidgetItemC1EPS_i(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN15QTreeWidgetItemC1EPS_i(arg0, arg1)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::setIcon(int column, const QIcon & icon);
impl /*struct*/ QTreeWidgetItem {
  pub fn setIcon<RetType, T: QTreeWidgetItem_setIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIcon(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setIcon<RetType> {
  fn setIcon(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  void QTreeWidgetItem::setIcon(int column, const QIcon & icon);
impl<'a> /*trait*/ QTreeWidgetItem_setIcon<()> for (i32, &'a QIcon) {
  fn setIcon(self , rsthis: & QTreeWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem7setIconEiRK5QIcon()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem7setIconEiRK5QIcon(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::QTreeWidgetItem(QTreeWidgetItem * parent, QTreeWidgetItem * after, int type);
impl<'a> /*trait*/ QTreeWidgetItem_New for (&'a QTreeWidgetItem, &'a QTreeWidgetItem, i32) {
  fn New(self) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1EPS_S0_i()};
    let ctysz: c_int = unsafe{QTreeWidgetItem_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    // unsafe {_ZN15QTreeWidgetItemC1EPS_S0_i(qthis, arg0, arg1, arg2)};
    let qthis: *mut c_void = unsafe {dector_ZN15QTreeWidgetItemC1EPS_S0_i(arg0, arg1, arg2)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTreeWidgetItem::isHidden();
impl /*struct*/ QTreeWidgetItem {
  pub fn isHidden<RetType, T: QTreeWidgetItem_isHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isHidden(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_isHidden<RetType> {
  fn isHidden(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  bool QTreeWidgetItem::isHidden();
impl<'a> /*trait*/ QTreeWidgetItem_isHidden<i8> for () {
  fn isHidden(self , rsthis: & QTreeWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem8isHiddenEv()};
    let mut ret = unsafe {_ZNK15QTreeWidgetItem8isHiddenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::QTreeWidgetItem(QTreeWidget * view, QTreeWidgetItem * after, int type);
impl<'a> /*trait*/ QTreeWidgetItem_New for (&'a QTreeWidget, &'a QTreeWidgetItem, i32) {
  fn New(self) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1EP11QTreeWidgetPS_i()};
    let ctysz: c_int = unsafe{QTreeWidgetItem_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    // unsafe {_ZN15QTreeWidgetItemC1EP11QTreeWidgetPS_i(qthis, arg0, arg1, arg2)};
    let qthis: *mut c_void = unsafe {dector_ZN15QTreeWidgetItemC1EP11QTreeWidgetPS_i(arg0, arg1, arg2)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::insertChild(int index, QTreeWidgetItem * child);
impl /*struct*/ QTreeWidgetItem {
  pub fn insertChild<RetType, T: QTreeWidgetItem_insertChild<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertChild(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_insertChild<RetType> {
  fn insertChild(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  void QTreeWidgetItem::insertChild(int index, QTreeWidgetItem * child);
impl<'a> /*trait*/ QTreeWidgetItem_insertChild<()> for (i32, &'a QTreeWidgetItem) {
  fn insertChild(self , rsthis: & QTreeWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem11insertChildEiPS_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem11insertChildEiPS_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::QTreeWidgetItem(const QTreeWidgetItem & other);
impl<'a> /*trait*/ QTreeWidgetItem_New for (&'a QTreeWidgetItem) {
  fn New(self) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1ERKS_()};
    let ctysz: c_int = unsafe{QTreeWidgetItem_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN15QTreeWidgetItemC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN15QTreeWidgetItemC1ERKS_(arg0)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTreeWidgetItem::isDisabled();
impl /*struct*/ QTreeWidgetItem {
  pub fn isDisabled<RetType, T: QTreeWidgetItem_isDisabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDisabled(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_isDisabled<RetType> {
  fn isDisabled(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  bool QTreeWidgetItem::isDisabled();
impl<'a> /*trait*/ QTreeWidgetItem_isDisabled<i8> for () {
  fn isDisabled(self , rsthis: & QTreeWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem10isDisabledEv()};
    let mut ret = unsafe {_ZNK15QTreeWidgetItem10isDisabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::setText(int column, const QString & text);
impl /*struct*/ QTreeWidgetItem {
  pub fn setText<RetType, T: QTreeWidgetItem_setText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setText<RetType> {
  fn setText(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  void QTreeWidgetItem::setText(int column, const QString & text);
impl<'a> /*trait*/ QTreeWidgetItem_setText<()> for (i32, &'a QString) {
  fn setText(self , rsthis: & QTreeWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem7setTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem7setTextEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::read(QDataStream & in);
impl /*struct*/ QTreeWidgetItem {
  pub fn read<RetType, T: QTreeWidgetItem_read<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.read(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_read<RetType> {
  fn read(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  void QTreeWidgetItem::read(QDataStream & in);
impl<'a> /*trait*/ QTreeWidgetItem_read<()> for (&'a QDataStream) {
  fn read(self , rsthis: & QTreeWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem4readER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem4readER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::QTreeWidgetItem(int type);
impl<'a> /*trait*/ QTreeWidgetItem_New for (i32) {
  fn New(self) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1Ei()};
    let ctysz: c_int = unsafe{QTreeWidgetItem_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self  as c_int;
    // unsafe {_ZN15QTreeWidgetItemC1Ei(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN15QTreeWidgetItemC1Ei(arg0)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QVariant QTreeWidgetItem::data(int column, int role);
impl /*struct*/ QTreeWidgetItem {
  pub fn data<RetType, T: QTreeWidgetItem_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_data<RetType> {
  fn data(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  QVariant QTreeWidgetItem::data(int column, int role);
impl<'a> /*trait*/ QTreeWidgetItem_data<QVariant> for (i32, i32) {
  fn data(self , rsthis: & QTreeWidgetItem) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem4dataEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK15QTreeWidgetItem4dataEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::setToolTip(int column, const QString & toolTip);
impl /*struct*/ QTreeWidgetItem {
  pub fn setToolTip<RetType, T: QTreeWidgetItem_setToolTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setToolTip(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setToolTip<RetType> {
  fn setToolTip(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  void QTreeWidgetItem::setToolTip(int column, const QString & toolTip);
impl<'a> /*trait*/ QTreeWidgetItem_setToolTip<()> for (i32, &'a QString) {
  fn setToolTip(self , rsthis: & QTreeWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem10setToolTipEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem10setToolTipEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::QTreeWidgetItem(QTreeWidget * view, const QStringList & strings, int type);
impl<'a> /*trait*/ QTreeWidgetItem_New for (&'a QTreeWidget, &'a QStringList, i32) {
  fn New(self) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1EP11QTreeWidgetRK11QStringListi()};
    let ctysz: c_int = unsafe{QTreeWidgetItem_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    // unsafe {_ZN15QTreeWidgetItemC1EP11QTreeWidgetRK11QStringListi(qthis, arg0, arg1, arg2)};
    let qthis: *mut c_void = unsafe {dector_ZN15QTreeWidgetItemC1EP11QTreeWidgetRK11QStringListi(arg0, arg1, arg2)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTreeWidgetItem::isFirstColumnSpanned();
impl /*struct*/ QTreeWidgetItem {
  pub fn isFirstColumnSpanned<RetType, T: QTreeWidgetItem_isFirstColumnSpanned<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFirstColumnSpanned(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_isFirstColumnSpanned<RetType> {
  fn isFirstColumnSpanned(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  bool QTreeWidgetItem::isFirstColumnSpanned();
impl<'a> /*trait*/ QTreeWidgetItem_isFirstColumnSpanned<i8> for () {
  fn isFirstColumnSpanned(self , rsthis: & QTreeWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem20isFirstColumnSpannedEv()};
    let mut ret = unsafe {_ZNK15QTreeWidgetItem20isFirstColumnSpannedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::QTreeWidgetItem(const QStringList & strings, int type);
impl<'a> /*trait*/ QTreeWidgetItem_New for (&'a QStringList, i32) {
  fn New(self) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1ERK11QStringListi()};
    let ctysz: c_int = unsafe{QTreeWidgetItem_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    // unsafe {_ZN15QTreeWidgetItemC1ERK11QStringListi(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN15QTreeWidgetItemC1ERK11QStringListi(arg0, arg1)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::setSelected(bool select);
impl /*struct*/ QTreeWidgetItem {
  pub fn setSelected<RetType, T: QTreeWidgetItem_setSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSelected(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setSelected<RetType> {
  fn setSelected(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  void QTreeWidgetItem::setSelected(bool select);
impl<'a> /*trait*/ QTreeWidgetItem_setSelected<()> for (i8) {
  fn setSelected(self , rsthis: & QTreeWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem11setSelectedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QTreeWidgetItem11setSelectedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::~QTreeWidgetItem();
impl /*struct*/ QTreeWidgetItem {
  pub fn Free<RetType, T: QTreeWidgetItem_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_Free<RetType> {
  fn Free(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  void QTreeWidgetItem::~QTreeWidgetItem();
impl<'a> /*trait*/ QTreeWidgetItem_Free<()> for () {
  fn Free(self , rsthis: & QTreeWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemD0Ev()};
     unsafe {_ZN15QTreeWidgetItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::setHidden(bool hide);
impl /*struct*/ QTreeWidgetItem {
  pub fn setHidden<RetType, T: QTreeWidgetItem_setHidden<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHidden(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setHidden<RetType> {
  fn setHidden(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  void QTreeWidgetItem::setHidden(bool hide);
impl<'a> /*trait*/ QTreeWidgetItem_setHidden<()> for (i8) {
  fn setHidden(self , rsthis: & QTreeWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem9setHiddenEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QTreeWidgetItem9setHiddenEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTreeWidgetItem * QTreeWidgetItem::takeChild(int index);
impl /*struct*/ QTreeWidgetItem {
  pub fn takeChild<RetType, T: QTreeWidgetItem_takeChild<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeChild(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_takeChild<RetType> {
  fn takeChild(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  QTreeWidgetItem * QTreeWidgetItem::takeChild(int index);
impl<'a> /*trait*/ QTreeWidgetItem_takeChild<QTreeWidgetItem> for (i32) {
  fn takeChild(self , rsthis: & QTreeWidgetItem) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem9takeChildEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN15QTreeWidgetItem9takeChildEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTreeWidgetItem::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::QTreeWidgetItem(QTreeWidgetItem * parent, const QStringList & strings, int type);
impl<'a> /*trait*/ QTreeWidgetItem_New for (&'a QTreeWidgetItem, &'a QStringList, i32) {
  fn New(self) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1EPS_RK11QStringListi()};
    let ctysz: c_int = unsafe{QTreeWidgetItem_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    // unsafe {_ZN15QTreeWidgetItemC1EPS_RK11QStringListi(qthis, arg0, arg1, arg2)};
    let qthis: *mut c_void = unsafe {dector_ZN15QTreeWidgetItemC1EPS_RK11QStringListi(arg0, arg1, arg2)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::setDisabled(bool disabled);
impl /*struct*/ QTreeWidgetItem {
  pub fn setDisabled<RetType, T: QTreeWidgetItem_setDisabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDisabled(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setDisabled<RetType> {
  fn setDisabled(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  void QTreeWidgetItem::setDisabled(bool disabled);
impl<'a> /*trait*/ QTreeWidgetItem_setDisabled<()> for (i8) {
  fn setDisabled(self , rsthis: & QTreeWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem11setDisabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QTreeWidgetItem11setDisabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::removeChild(QTreeWidgetItem * child);
impl /*struct*/ QTreeWidgetItem {
  pub fn removeChild<RetType, T: QTreeWidgetItem_removeChild<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeChild(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_removeChild<RetType> {
  fn removeChild(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  void QTreeWidgetItem::removeChild(QTreeWidgetItem * child);
impl<'a> /*trait*/ QTreeWidgetItem_removeChild<()> for (&'a QTreeWidgetItem) {
  fn removeChild(self , rsthis: & QTreeWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem11removeChildEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem11removeChildEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTreeWidgetItem * QTreeWidgetItem::clone();
impl /*struct*/ QTreeWidgetItem {
  pub fn clone<RetType, T: QTreeWidgetItem_clone<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clone(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_clone<RetType> {
  fn clone(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  QTreeWidgetItem * QTreeWidgetItem::clone();
impl<'a> /*trait*/ QTreeWidgetItem_clone<QTreeWidgetItem> for () {
  fn clone(self , rsthis: & QTreeWidgetItem) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem5cloneEv()};
    let mut ret = unsafe {_ZNK15QTreeWidgetItem5cloneEv(rsthis.qclsinst)};
    let mut ret1 = QTreeWidgetItem::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::QTreeWidgetItem(QTreeWidget * view, int type);
impl<'a> /*trait*/ QTreeWidgetItem_New for (&'a QTreeWidget, i32) {
  fn New(self) -> QTreeWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItemC1EP11QTreeWidgeti()};
    let ctysz: c_int = unsafe{QTreeWidgetItem_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    // unsafe {_ZN15QTreeWidgetItemC1EP11QTreeWidgeti(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN15QTreeWidgetItemC1EP11QTreeWidgeti(arg0, arg1)};
    let rsthis = QTreeWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::addChild(QTreeWidgetItem * child);
impl /*struct*/ QTreeWidgetItem {
  pub fn addChild<RetType, T: QTreeWidgetItem_addChild<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addChild(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_addChild<RetType> {
  fn addChild(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  void QTreeWidgetItem::addChild(QTreeWidgetItem * child);
impl<'a> /*trait*/ QTreeWidgetItem_addChild<()> for (&'a QTreeWidgetItem) {
  fn addChild(self , rsthis: & QTreeWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem8addChildEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem8addChildEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTreeWidgetItem::setWhatsThis(int column, const QString & whatsThis);
impl /*struct*/ QTreeWidgetItem {
  pub fn setWhatsThis<RetType, T: QTreeWidgetItem_setWhatsThis<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWhatsThis(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_setWhatsThis<RetType> {
  fn setWhatsThis(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  void QTreeWidgetItem::setWhatsThis(int column, const QString & whatsThis);
impl<'a> /*trait*/ QTreeWidgetItem_setWhatsThis<()> for (i32, &'a QString) {
  fn setWhatsThis(self , rsthis: & QTreeWidgetItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTreeWidgetItem12setWhatsThisEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QTreeWidgetItem12setWhatsThisEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QTreeWidgetItem::isSelected();
impl /*struct*/ QTreeWidgetItem {
  pub fn isSelected<RetType, T: QTreeWidgetItem_isSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSelected(self);
    // return 1;
  }
}

pub trait QTreeWidgetItem_isSelected<RetType> {
  fn isSelected(self , rsthis: & QTreeWidgetItem) -> RetType;
}

  // proto:  bool QTreeWidgetItem::isSelected();
impl<'a> /*trait*/ QTreeWidgetItem_isSelected<i8> for () {
  fn isSelected(self , rsthis: & QTreeWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTreeWidgetItem10isSelectedEv()};
    let mut ret = unsafe {_ZNK15QTreeWidgetItem10isSelectedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

