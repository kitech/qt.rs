// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtablewidgetitem::QTableWidgetItem;
use super::qstringlist::QStringList;
use super::qrect::QRect;
use super::qwidget::QWidget;
use super::qtablewidgetselectionrange::QTableWidgetSelectionRange;
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTableWidget::itemActivated(QTableWidgetItem * item);
  fn _ZN12QTableWidget13itemActivatedEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidget::setColumnCount(int columns);
  fn _ZN12QTableWidget14setColumnCountEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableWidget::FreeQTableWidget();
  fn _ZN12QTableWidgetD0Ev(qthis: *mut c_void) ;
  // proto:  void QTableWidget::itemDoubleClicked(QTableWidgetItem * item);
  fn _ZN12QTableWidget17itemDoubleClickedEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidget::cellChanged(int row, int column);
  fn _ZN12QTableWidget11cellChangedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QList<QTableWidgetItem *> QTableWidget::selectedItems();
  fn _ZNK12QTableWidget13selectedItemsEv(qthis: *mut c_void) ;
  // proto:  bool QTableWidget::isSortingEnabled();
  fn _ZNK12QTableWidget16isSortingEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  const QMetaObject * QTableWidget::metaObject();
  fn _ZNK12QTableWidget10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QTableWidget::NewQTableWidget(const QTableWidget & );
  fn _ZN12QTableWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidget::closePersistentEditor(QTableWidgetItem * item);
  fn _ZN12QTableWidget21closePersistentEditorEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidget::setHorizontalHeaderLabels(const QStringList & labels);
  fn _ZN12QTableWidget25setHorizontalHeaderLabelsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidget::itemSelectionChanged();
  fn _ZN12QTableWidget20itemSelectionChangedEv(qthis: *mut c_void) ;
  // proto:  void QTableWidget::setItemSelected(const QTableWidgetItem * item, bool select);
  fn _ZN12QTableWidget15setItemSelectedEPK16QTableWidgetItemb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  QTableWidgetItem * QTableWidget::takeItem(int row, int column);
  fn _ZN12QTableWidget8takeItemEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QTableWidget::itemChanged(QTableWidgetItem * item);
  fn _ZN12QTableWidget11itemChangedEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidget::removeCellWidget(int row, int column);
  fn _ZN12QTableWidget16removeCellWidgetEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QTableWidget::setVerticalHeaderItem(int row, QTableWidgetItem * item);
  fn _ZN12QTableWidget21setVerticalHeaderItemEiP16QTableWidgetItem(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QTableWidget::cellClicked(int row, int column);
  fn _ZN12QTableWidget11cellClickedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QRect QTableWidget::visualItemRect(const QTableWidgetItem * item);
  fn _ZNK12QTableWidget14visualItemRectEPK16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QTableWidgetItem * QTableWidget::currentItem();
  fn _ZNK12QTableWidget11currentItemEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTableWidget::row(const QTableWidgetItem * item);
  fn _ZNK12QTableWidget3rowEPK16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QTableWidget::removeRow(int row);
  fn _ZN12QTableWidget9removeRowEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableWidget::setItemPrototype(const QTableWidgetItem * item);
  fn _ZN12QTableWidget16setItemPrototypeEPK16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidget::NewQTableWidget(int rows, int columns, QWidget * parent);
  fn _ZN12QTableWidgetC1EiiP7QWidget(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) ;
  // proto:  int QTableWidget::visualRow(int logicalRow);
  fn _ZNK12QTableWidget9visualRowEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QTableWidget::cellEntered(int row, int column);
  fn _ZN12QTableWidget11cellEnteredEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QTableWidget::setCellWidget(int row, int column, QWidget * widget);
  fn _ZN12QTableWidget13setCellWidgetEiiP7QWidget(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) ;
  // proto:  void QTableWidget::openPersistentEditor(QTableWidgetItem * item);
  fn _ZN12QTableWidget20openPersistentEditorEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTableWidget::columnCount();
  fn _ZNK12QTableWidget11columnCountEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTableWidget::currentRow();
  fn _ZNK12QTableWidget10currentRowEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTableWidget::currentItemChanged(QTableWidgetItem * current, QTableWidgetItem * previous);
  fn _ZN12QTableWidget18currentItemChangedEP16QTableWidgetItemS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QTableWidget::setCurrentItem(QTableWidgetItem * item);
  fn _ZN12QTableWidget14setCurrentItemEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QWidget * QTableWidget::cellWidget(int row, int column);
  fn _ZNK12QTableWidget10cellWidgetEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QTableWidget::setSortingEnabled(bool enable);
  fn _ZN12QTableWidget17setSortingEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTableWidget::setItem(int row, int column, QTableWidgetItem * item);
  fn _ZN12QTableWidget7setItemEiiP16QTableWidgetItem(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) ;
  // proto:  QTableWidgetItem * QTableWidget::horizontalHeaderItem(int column);
  fn _ZNK12QTableWidget20horizontalHeaderItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTableWidget::cellPressed(int row, int column);
  fn _ZN12QTableWidget11cellPressedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QTableWidget::editItem(QTableWidgetItem * item);
  fn _ZN12QTableWidget8editItemEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QList<QTableWidgetSelectionRange> QTableWidget::selectedRanges();
  fn _ZNK12QTableWidget14selectedRangesEv(qthis: *mut c_void) ;
  // proto:  int QTableWidget::currentColumn();
  fn _ZNK12QTableWidget13currentColumnEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTableWidget::removeColumn(int column);
  fn _ZN12QTableWidget12removeColumnEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableWidget::setRangeSelected(const QTableWidgetSelectionRange & range, bool select);
  fn _ZN12QTableWidget16setRangeSelectedERK26QTableWidgetSelectionRangeb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  int QTableWidget::column(const QTableWidgetItem * item);
  fn _ZNK12QTableWidget6columnEPK16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  bool QTableWidget::isItemSelected(const QTableWidgetItem * item);
  fn _ZNK12QTableWidget14isItemSelectedEPK16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QTableWidgetItem * QTableWidget::takeVerticalHeaderItem(int row);
  fn _ZN12QTableWidget22takeVerticalHeaderItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTableWidget::insertRow(int row);
  fn _ZN12QTableWidget9insertRowEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableWidget::currentCellChanged(int currentRow, int currentColumn, int previousRow, int previousColumn);
  fn _ZN12QTableWidget18currentCellChangedEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QTableWidget::cellDoubleClicked(int row, int column);
  fn _ZN12QTableWidget17cellDoubleClickedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  int QTableWidget::rowCount();
  fn _ZNK12QTableWidget8rowCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QTableWidgetItem * QTableWidget::item(int row, int column);
  fn _ZNK12QTableWidget4itemEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QTableWidget::NewQTableWidget(QWidget * parent);
  fn _ZN12QTableWidgetC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidget::setVerticalHeaderLabels(const QStringList & labels);
  fn _ZN12QTableWidget23setVerticalHeaderLabelsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QTableWidgetItem * QTableWidget::itemPrototype();
  fn _ZNK12QTableWidget13itemPrototypeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTableWidgetItem * QTableWidget::itemAt(const QPoint & p);
  fn _ZNK12QTableWidget6itemAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTableWidget::clearContents();
  fn _ZN12QTableWidget13clearContentsEv(qthis: *mut c_void) ;
  // proto:  void QTableWidget::itemPressed(QTableWidgetItem * item);
  fn _ZN12QTableWidget11itemPressedEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTableWidgetItem * QTableWidget::itemAt(int x, int y);
  fn _ZNK12QTableWidget6itemAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QTableWidget::setCurrentCell(int row, int column);
  fn _ZN12QTableWidget14setCurrentCellEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QTableWidget::setRowCount(int rows);
  fn _ZN12QTableWidget11setRowCountEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableWidget::setHorizontalHeaderItem(int column, QTableWidgetItem * item);
  fn _ZN12QTableWidget23setHorizontalHeaderItemEiP16QTableWidgetItem(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  int QTableWidget::visualColumn(int logicalColumn);
  fn _ZNK12QTableWidget12visualColumnEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QTableWidget::itemEntered(QTableWidgetItem * item);
  fn _ZN12QTableWidget11itemEnteredEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTableWidgetItem * QTableWidget::takeHorizontalHeaderItem(int column);
  fn _ZN12QTableWidget24takeHorizontalHeaderItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QTableWidgetItem * QTableWidget::verticalHeaderItem(int row);
  fn _ZNK12QTableWidget18verticalHeaderItemEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTableWidget::clear();
  fn _ZN12QTableWidget5clearEv(qthis: *mut c_void) ;
  // proto:  void QTableWidget::insertColumn(int column);
  fn _ZN12QTableWidget12insertColumnEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableWidget::cellActivated(int row, int column);
  fn _ZN12QTableWidget13cellActivatedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QTableWidget::itemClicked(QTableWidgetItem * item);
  fn _ZN12QTableWidget11itemClickedEP16QTableWidgetItem(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QTableWidget)=1
pub struct QTableWidget {
  pub qclsinst: *mut c_void,
}

// proto:  void QTableWidget::itemActivated(QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn itemActivated<RetType, T: QTableWidget_itemActivated<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.itemActivated(self);
    // return 1;
  }
}

pub trait QTableWidget_itemActivated<RetType> {
  fn itemActivated(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::itemActivated(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_itemActivated<()> for (&'a mut QTableWidgetItem) {
  fn itemActivated(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget13itemActivatedEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget13itemActivatedEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTableWidget::setColumnCount(int columns);
impl /*struct*/ QTableWidget {
  pub fn setColumnCount<RetType, T: QTableWidget_setColumnCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setColumnCount(self);
    // return 1;
  }
}

pub trait QTableWidget_setColumnCount<RetType> {
  fn setColumnCount(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::setColumnCount(int columns);
impl<'a> /*trait*/ QTableWidget_setColumnCount<()> for (i32) {
  fn setColumnCount(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget14setColumnCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QTableWidget14setColumnCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTableWidget::FreeQTableWidget();
impl /*struct*/ QTableWidget {
  pub fn FreeQTableWidget<RetType, T: QTableWidget_FreeQTableWidget<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQTableWidget(self);
    // return 1;
  }
}

pub trait QTableWidget_FreeQTableWidget<RetType> {
  fn FreeQTableWidget(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::FreeQTableWidget();
impl<'a> /*trait*/ QTableWidget_FreeQTableWidget<()> for () {
  fn FreeQTableWidget(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidgetD0Ev()};
     unsafe {_ZN12QTableWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QTableWidget::itemDoubleClicked(QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn itemDoubleClicked<RetType, T: QTableWidget_itemDoubleClicked<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.itemDoubleClicked(self);
    // return 1;
  }
}

pub trait QTableWidget_itemDoubleClicked<RetType> {
  fn itemDoubleClicked(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::itemDoubleClicked(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_itemDoubleClicked<()> for (&'a mut QTableWidgetItem) {
  fn itemDoubleClicked(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget17itemDoubleClickedEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget17itemDoubleClickedEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTableWidget::cellChanged(int row, int column);
impl /*struct*/ QTableWidget {
  pub fn cellChanged<RetType, T: QTableWidget_cellChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.cellChanged(self);
    // return 1;
  }
}

pub trait QTableWidget_cellChanged<RetType> {
  fn cellChanged(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::cellChanged(int row, int column);
impl<'a> /*trait*/ QTableWidget_cellChanged<()> for (i32, i32) {
  fn cellChanged(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget11cellChangedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QTableWidget11cellChangedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QList<QTableWidgetItem *> QTableWidget::selectedItems();
impl /*struct*/ QTableWidget {
  pub fn selectedItems<RetType, T: QTableWidget_selectedItems<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.selectedItems(self);
    // return 1;
  }
}

pub trait QTableWidget_selectedItems<RetType> {
  fn selectedItems(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  QList<QTableWidgetItem *> QTableWidget::selectedItems();
impl<'a> /*trait*/ QTableWidget_selectedItems<()> for () {
  fn selectedItems(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget13selectedItemsEv()};
     unsafe {_ZNK12QTableWidget13selectedItemsEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  bool QTableWidget::isSortingEnabled();
impl /*struct*/ QTableWidget {
  pub fn isSortingEnabled<RetType, T: QTableWidget_isSortingEnabled<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isSortingEnabled(self);
    // return 1;
  }
}

pub trait QTableWidget_isSortingEnabled<RetType> {
  fn isSortingEnabled(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  bool QTableWidget::isSortingEnabled();
impl<'a> /*trait*/ QTableWidget_isSortingEnabled<i8> for () {
  fn isSortingEnabled(self , rsthis: &mut QTableWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget16isSortingEnabledEv()};
    let mut ret = unsafe {_ZNK12QTableWidget16isSortingEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  const QMetaObject * QTableWidget::metaObject();
impl /*struct*/ QTableWidget {
  pub fn metaObject<RetType, T: QTableWidget_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTableWidget_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  const QMetaObject * QTableWidget::metaObject();
impl<'a> /*trait*/ QTableWidget_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget10metaObjectEv()};
     unsafe {_ZNK12QTableWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTableWidget {
  pub fn NewQTableWidget<T: QTableWidget_NewQTableWidget>(value: T) -> QTableWidget {
    let rsthis = value.NewQTableWidget();
    return rsthis;
    // return 1;
  }
}

pub trait QTableWidget_NewQTableWidget {
  fn NewQTableWidget(self) -> QTableWidget;
}

// proto: void QTableWidget::NewQTableWidget(const QTableWidget & );
impl<'a> /*trait*/ QTableWidget_NewQTableWidget for (&'a  QTableWidget) {
  fn NewQTableWidget(self) -> QTableWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QTableWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QTableWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QTableWidget::closePersistentEditor(QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn closePersistentEditor<RetType, T: QTableWidget_closePersistentEditor<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.closePersistentEditor(self);
    // return 1;
  }
}

pub trait QTableWidget_closePersistentEditor<RetType> {
  fn closePersistentEditor(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::closePersistentEditor(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_closePersistentEditor<()> for (&'a mut QTableWidgetItem) {
  fn closePersistentEditor(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget21closePersistentEditorEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget21closePersistentEditorEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTableWidget::setHorizontalHeaderLabels(const QStringList & labels);
impl /*struct*/ QTableWidget {
  pub fn setHorizontalHeaderLabels<RetType, T: QTableWidget_setHorizontalHeaderLabels<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setHorizontalHeaderLabels(self);
    // return 1;
  }
}

pub trait QTableWidget_setHorizontalHeaderLabels<RetType> {
  fn setHorizontalHeaderLabels(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::setHorizontalHeaderLabels(const QStringList & labels);
impl<'a> /*trait*/ QTableWidget_setHorizontalHeaderLabels<()> for (&'a  QStringList) {
  fn setHorizontalHeaderLabels(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget25setHorizontalHeaderLabelsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget25setHorizontalHeaderLabelsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTableWidget::itemSelectionChanged();
impl /*struct*/ QTableWidget {
  pub fn itemSelectionChanged<RetType, T: QTableWidget_itemSelectionChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.itemSelectionChanged(self);
    // return 1;
  }
}

pub trait QTableWidget_itemSelectionChanged<RetType> {
  fn itemSelectionChanged(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::itemSelectionChanged();
impl<'a> /*trait*/ QTableWidget_itemSelectionChanged<()> for () {
  fn itemSelectionChanged(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget20itemSelectionChangedEv()};
     unsafe {_ZN12QTableWidget20itemSelectionChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QTableWidget::setItemSelected(const QTableWidgetItem * item, bool select);
impl /*struct*/ QTableWidget {
  pub fn setItemSelected<RetType, T: QTableWidget_setItemSelected<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setItemSelected(self);
    // return 1;
  }
}

pub trait QTableWidget_setItemSelected<RetType> {
  fn setItemSelected(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::setItemSelected(const QTableWidgetItem * item, bool select);
impl<'a> /*trait*/ QTableWidget_setItemSelected<()> for (&'a  QTableWidgetItem, i8) {
  fn setItemSelected(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget15setItemSelectedEPK16QTableWidgetItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN12QTableWidget15setItemSelectedEPK16QTableWidgetItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QTableWidgetItem * QTableWidget::takeItem(int row, int column);
impl /*struct*/ QTableWidget {
  pub fn takeItem<RetType, T: QTableWidget_takeItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.takeItem(self);
    // return 1;
  }
}

pub trait QTableWidget_takeItem<RetType> {
  fn takeItem(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  QTableWidgetItem * QTableWidget::takeItem(int row, int column);
impl<'a> /*trait*/ QTableWidget_takeItem<QTableWidgetItem> for (i32, i32) {
  fn takeItem(self , rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget8takeItemEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN12QTableWidget8takeItemEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTableWidget::itemChanged(QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn itemChanged<RetType, T: QTableWidget_itemChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.itemChanged(self);
    // return 1;
  }
}

pub trait QTableWidget_itemChanged<RetType> {
  fn itemChanged(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::itemChanged(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_itemChanged<()> for (&'a mut QTableWidgetItem) {
  fn itemChanged(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget11itemChangedEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget11itemChangedEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTableWidget::removeCellWidget(int row, int column);
impl /*struct*/ QTableWidget {
  pub fn removeCellWidget<RetType, T: QTableWidget_removeCellWidget<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.removeCellWidget(self);
    // return 1;
  }
}

pub trait QTableWidget_removeCellWidget<RetType> {
  fn removeCellWidget(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::removeCellWidget(int row, int column);
impl<'a> /*trait*/ QTableWidget_removeCellWidget<()> for (i32, i32) {
  fn removeCellWidget(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget16removeCellWidgetEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QTableWidget16removeCellWidgetEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QTableWidget::setVerticalHeaderItem(int row, QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn setVerticalHeaderItem<RetType, T: QTableWidget_setVerticalHeaderItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setVerticalHeaderItem(self);
    // return 1;
  }
}

pub trait QTableWidget_setVerticalHeaderItem<RetType> {
  fn setVerticalHeaderItem(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::setVerticalHeaderItem(int row, QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_setVerticalHeaderItem<()> for (i32, &'a mut QTableWidgetItem) {
  fn setVerticalHeaderItem(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget21setVerticalHeaderItemEiP16QTableWidgetItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget21setVerticalHeaderItemEiP16QTableWidgetItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QTableWidget::cellClicked(int row, int column);
impl /*struct*/ QTableWidget {
  pub fn cellClicked<RetType, T: QTableWidget_cellClicked<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.cellClicked(self);
    // return 1;
  }
}

pub trait QTableWidget_cellClicked<RetType> {
  fn cellClicked(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::cellClicked(int row, int column);
impl<'a> /*trait*/ QTableWidget_cellClicked<()> for (i32, i32) {
  fn cellClicked(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget11cellClickedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QTableWidget11cellClickedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QRect QTableWidget::visualItemRect(const QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn visualItemRect<RetType, T: QTableWidget_visualItemRect<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.visualItemRect(self);
    // return 1;
  }
}

pub trait QTableWidget_visualItemRect<RetType> {
  fn visualItemRect(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  QRect QTableWidget::visualItemRect(const QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_visualItemRect<QRect> for (&'a  QTableWidgetItem) {
  fn visualItemRect(self , rsthis: &mut QTableWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget14visualItemRectEPK16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QTableWidget14visualItemRectEPK16QTableWidgetItem(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QTableWidgetItem * QTableWidget::currentItem();
impl /*struct*/ QTableWidget {
  pub fn currentItem<RetType, T: QTableWidget_currentItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.currentItem(self);
    // return 1;
  }
}

pub trait QTableWidget_currentItem<RetType> {
  fn currentItem(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  QTableWidgetItem * QTableWidget::currentItem();
impl<'a> /*trait*/ QTableWidget_currentItem<QTableWidgetItem> for () {
  fn currentItem(self , rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget11currentItemEv()};
    let mut ret = unsafe {_ZNK12QTableWidget11currentItemEv(rsthis.qclsinst)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QTableWidget::row(const QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn row<RetType, T: QTableWidget_row<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.row(self);
    // return 1;
  }
}

pub trait QTableWidget_row<RetType> {
  fn row(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  int QTableWidget::row(const QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_row<i32> for (&'a  QTableWidgetItem) {
  fn row(self , rsthis: &mut QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget3rowEPK16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QTableWidget3rowEPK16QTableWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QTableWidget::removeRow(int row);
impl /*struct*/ QTableWidget {
  pub fn removeRow<RetType, T: QTableWidget_removeRow<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.removeRow(self);
    // return 1;
  }
}

pub trait QTableWidget_removeRow<RetType> {
  fn removeRow(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::removeRow(int row);
impl<'a> /*trait*/ QTableWidget_removeRow<()> for (i32) {
  fn removeRow(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget9removeRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QTableWidget9removeRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTableWidget::setItemPrototype(const QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn setItemPrototype<RetType, T: QTableWidget_setItemPrototype<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setItemPrototype(self);
    // return 1;
  }
}

pub trait QTableWidget_setItemPrototype<RetType> {
  fn setItemPrototype(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::setItemPrototype(const QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_setItemPrototype<()> for (&'a  QTableWidgetItem) {
  fn setItemPrototype(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget16setItemPrototypeEPK16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget16setItemPrototypeEPK16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QTableWidget::NewQTableWidget(int rows, int columns, QWidget * parent);
impl<'a> /*trait*/ QTableWidget_NewQTableWidget for (i32, i32, &'a mut QWidget) {
  fn NewQTableWidget(self) -> QTableWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidgetC1EiiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN12QTableWidgetC1EiiP7QWidget(qthis, arg0, arg1, arg2)};
    let rsthis = QTableWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  int QTableWidget::visualRow(int logicalRow);
impl /*struct*/ QTableWidget {
  pub fn visualRow<RetType, T: QTableWidget_visualRow<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.visualRow(self);
    // return 1;
  }
}

pub trait QTableWidget_visualRow<RetType> {
  fn visualRow(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  int QTableWidget::visualRow(int logicalRow);
impl<'a> /*trait*/ QTableWidget_visualRow<i32> for (i32) {
  fn visualRow(self , rsthis: &mut QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget9visualRowEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK12QTableWidget9visualRowEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QTableWidget::cellEntered(int row, int column);
impl /*struct*/ QTableWidget {
  pub fn cellEntered<RetType, T: QTableWidget_cellEntered<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.cellEntered(self);
    // return 1;
  }
}

pub trait QTableWidget_cellEntered<RetType> {
  fn cellEntered(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::cellEntered(int row, int column);
impl<'a> /*trait*/ QTableWidget_cellEntered<()> for (i32, i32) {
  fn cellEntered(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget11cellEnteredEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QTableWidget11cellEnteredEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QTableWidget::setCellWidget(int row, int column, QWidget * widget);
impl /*struct*/ QTableWidget {
  pub fn setCellWidget<RetType, T: QTableWidget_setCellWidget<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setCellWidget(self);
    // return 1;
  }
}

pub trait QTableWidget_setCellWidget<RetType> {
  fn setCellWidget(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::setCellWidget(int row, int column, QWidget * widget);
impl<'a> /*trait*/ QTableWidget_setCellWidget<()> for (i32, i32, &'a mut QWidget) {
  fn setCellWidget(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget13setCellWidgetEiiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget13setCellWidgetEiiP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  void QTableWidget::openPersistentEditor(QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn openPersistentEditor<RetType, T: QTableWidget_openPersistentEditor<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.openPersistentEditor(self);
    // return 1;
  }
}

pub trait QTableWidget_openPersistentEditor<RetType> {
  fn openPersistentEditor(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::openPersistentEditor(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_openPersistentEditor<()> for (&'a mut QTableWidgetItem) {
  fn openPersistentEditor(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget20openPersistentEditorEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget20openPersistentEditorEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QTableWidget::columnCount();
impl /*struct*/ QTableWidget {
  pub fn columnCount<RetType, T: QTableWidget_columnCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QTableWidget_columnCount<RetType> {
  fn columnCount(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  int QTableWidget::columnCount();
impl<'a> /*trait*/ QTableWidget_columnCount<i32> for () {
  fn columnCount(self , rsthis: &mut QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget11columnCountEv()};
    let mut ret = unsafe {_ZNK12QTableWidget11columnCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QTableWidget::currentRow();
impl /*struct*/ QTableWidget {
  pub fn currentRow<RetType, T: QTableWidget_currentRow<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.currentRow(self);
    // return 1;
  }
}

pub trait QTableWidget_currentRow<RetType> {
  fn currentRow(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  int QTableWidget::currentRow();
impl<'a> /*trait*/ QTableWidget_currentRow<i32> for () {
  fn currentRow(self , rsthis: &mut QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget10currentRowEv()};
    let mut ret = unsafe {_ZNK12QTableWidget10currentRowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QTableWidget::currentItemChanged(QTableWidgetItem * current, QTableWidgetItem * previous);
impl /*struct*/ QTableWidget {
  pub fn currentItemChanged<RetType, T: QTableWidget_currentItemChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.currentItemChanged(self);
    // return 1;
  }
}

pub trait QTableWidget_currentItemChanged<RetType> {
  fn currentItemChanged(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::currentItemChanged(QTableWidgetItem * current, QTableWidgetItem * previous);
impl<'a> /*trait*/ QTableWidget_currentItemChanged<()> for (&'a mut QTableWidgetItem, &'a mut QTableWidgetItem) {
  fn currentItemChanged(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget18currentItemChangedEP16QTableWidgetItemS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget18currentItemChangedEP16QTableWidgetItemS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QTableWidget::setCurrentItem(QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn setCurrentItem<RetType, T: QTableWidget_setCurrentItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setCurrentItem(self);
    // return 1;
  }
}

pub trait QTableWidget_setCurrentItem<RetType> {
  fn setCurrentItem(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::setCurrentItem(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_setCurrentItem<()> for (&'a mut QTableWidgetItem) {
  fn setCurrentItem(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget14setCurrentItemEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget14setCurrentItemEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QWidget * QTableWidget::cellWidget(int row, int column);
impl /*struct*/ QTableWidget {
  pub fn cellWidget<RetType, T: QTableWidget_cellWidget<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.cellWidget(self);
    // return 1;
  }
}

pub trait QTableWidget_cellWidget<RetType> {
  fn cellWidget(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  QWidget * QTableWidget::cellWidget(int row, int column);
impl<'a> /*trait*/ QTableWidget_cellWidget<QWidget> for (i32, i32) {
  fn cellWidget(self , rsthis: &mut QTableWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget10cellWidgetEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK12QTableWidget10cellWidgetEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTableWidget::setSortingEnabled(bool enable);
impl /*struct*/ QTableWidget {
  pub fn setSortingEnabled<RetType, T: QTableWidget_setSortingEnabled<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setSortingEnabled(self);
    // return 1;
  }
}

pub trait QTableWidget_setSortingEnabled<RetType> {
  fn setSortingEnabled(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::setSortingEnabled(bool enable);
impl<'a> /*trait*/ QTableWidget_setSortingEnabled<()> for (i8) {
  fn setSortingEnabled(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget17setSortingEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QTableWidget17setSortingEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTableWidget::setItem(int row, int column, QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn setItem<RetType, T: QTableWidget_setItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setItem(self);
    // return 1;
  }
}

pub trait QTableWidget_setItem<RetType> {
  fn setItem(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::setItem(int row, int column, QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_setItem<()> for (i32, i32, &'a mut QTableWidgetItem) {
  fn setItem(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget7setItemEiiP16QTableWidgetItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget7setItemEiiP16QTableWidgetItem(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  QTableWidgetItem * QTableWidget::horizontalHeaderItem(int column);
impl /*struct*/ QTableWidget {
  pub fn horizontalHeaderItem<RetType, T: QTableWidget_horizontalHeaderItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.horizontalHeaderItem(self);
    // return 1;
  }
}

pub trait QTableWidget_horizontalHeaderItem<RetType> {
  fn horizontalHeaderItem(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  QTableWidgetItem * QTableWidget::horizontalHeaderItem(int column);
impl<'a> /*trait*/ QTableWidget_horizontalHeaderItem<QTableWidgetItem> for (i32) {
  fn horizontalHeaderItem(self , rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget20horizontalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK12QTableWidget20horizontalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTableWidget::cellPressed(int row, int column);
impl /*struct*/ QTableWidget {
  pub fn cellPressed<RetType, T: QTableWidget_cellPressed<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.cellPressed(self);
    // return 1;
  }
}

pub trait QTableWidget_cellPressed<RetType> {
  fn cellPressed(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::cellPressed(int row, int column);
impl<'a> /*trait*/ QTableWidget_cellPressed<()> for (i32, i32) {
  fn cellPressed(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget11cellPressedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QTableWidget11cellPressedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QTableWidget::editItem(QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn editItem<RetType, T: QTableWidget_editItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.editItem(self);
    // return 1;
  }
}

pub trait QTableWidget_editItem<RetType> {
  fn editItem(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::editItem(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_editItem<()> for (&'a mut QTableWidgetItem) {
  fn editItem(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget8editItemEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget8editItemEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QList<QTableWidgetSelectionRange> QTableWidget::selectedRanges();
impl /*struct*/ QTableWidget {
  pub fn selectedRanges<RetType, T: QTableWidget_selectedRanges<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.selectedRanges(self);
    // return 1;
  }
}

pub trait QTableWidget_selectedRanges<RetType> {
  fn selectedRanges(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  QList<QTableWidgetSelectionRange> QTableWidget::selectedRanges();
impl<'a> /*trait*/ QTableWidget_selectedRanges<()> for () {
  fn selectedRanges(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget14selectedRangesEv()};
     unsafe {_ZNK12QTableWidget14selectedRangesEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  int QTableWidget::currentColumn();
impl /*struct*/ QTableWidget {
  pub fn currentColumn<RetType, T: QTableWidget_currentColumn<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.currentColumn(self);
    // return 1;
  }
}

pub trait QTableWidget_currentColumn<RetType> {
  fn currentColumn(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  int QTableWidget::currentColumn();
impl<'a> /*trait*/ QTableWidget_currentColumn<i32> for () {
  fn currentColumn(self , rsthis: &mut QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget13currentColumnEv()};
    let mut ret = unsafe {_ZNK12QTableWidget13currentColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QTableWidget::removeColumn(int column);
impl /*struct*/ QTableWidget {
  pub fn removeColumn<RetType, T: QTableWidget_removeColumn<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.removeColumn(self);
    // return 1;
  }
}

pub trait QTableWidget_removeColumn<RetType> {
  fn removeColumn(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::removeColumn(int column);
impl<'a> /*trait*/ QTableWidget_removeColumn<()> for (i32) {
  fn removeColumn(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget12removeColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QTableWidget12removeColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTableWidget::setRangeSelected(const QTableWidgetSelectionRange & range, bool select);
impl /*struct*/ QTableWidget {
  pub fn setRangeSelected<RetType, T: QTableWidget_setRangeSelected<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setRangeSelected(self);
    // return 1;
  }
}

pub trait QTableWidget_setRangeSelected<RetType> {
  fn setRangeSelected(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::setRangeSelected(const QTableWidgetSelectionRange & range, bool select);
impl<'a> /*trait*/ QTableWidget_setRangeSelected<()> for (&'a  QTableWidgetSelectionRange, i8) {
  fn setRangeSelected(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget16setRangeSelectedERK26QTableWidgetSelectionRangeb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN12QTableWidget16setRangeSelectedERK26QTableWidgetSelectionRangeb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  int QTableWidget::column(const QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn column<RetType, T: QTableWidget_column<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.column(self);
    // return 1;
  }
}

pub trait QTableWidget_column<RetType> {
  fn column(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  int QTableWidget::column(const QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_column<i32> for (&'a  QTableWidgetItem) {
  fn column(self , rsthis: &mut QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget6columnEPK16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QTableWidget6columnEPK16QTableWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  bool QTableWidget::isItemSelected(const QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn isItemSelected<RetType, T: QTableWidget_isItemSelected<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isItemSelected(self);
    // return 1;
  }
}

pub trait QTableWidget_isItemSelected<RetType> {
  fn isItemSelected(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  bool QTableWidget::isItemSelected(const QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_isItemSelected<i8> for (&'a  QTableWidgetItem) {
  fn isItemSelected(self , rsthis: &mut QTableWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget14isItemSelectedEPK16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QTableWidget14isItemSelectedEPK16QTableWidgetItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QTableWidgetItem * QTableWidget::takeVerticalHeaderItem(int row);
impl /*struct*/ QTableWidget {
  pub fn takeVerticalHeaderItem<RetType, T: QTableWidget_takeVerticalHeaderItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.takeVerticalHeaderItem(self);
    // return 1;
  }
}

pub trait QTableWidget_takeVerticalHeaderItem<RetType> {
  fn takeVerticalHeaderItem(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  QTableWidgetItem * QTableWidget::takeVerticalHeaderItem(int row);
impl<'a> /*trait*/ QTableWidget_takeVerticalHeaderItem<QTableWidgetItem> for (i32) {
  fn takeVerticalHeaderItem(self , rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget22takeVerticalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN12QTableWidget22takeVerticalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTableWidget::insertRow(int row);
impl /*struct*/ QTableWidget {
  pub fn insertRow<RetType, T: QTableWidget_insertRow<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.insertRow(self);
    // return 1;
  }
}

pub trait QTableWidget_insertRow<RetType> {
  fn insertRow(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::insertRow(int row);
impl<'a> /*trait*/ QTableWidget_insertRow<()> for (i32) {
  fn insertRow(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget9insertRowEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QTableWidget9insertRowEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTableWidget::currentCellChanged(int currentRow, int currentColumn, int previousRow, int previousColumn);
impl /*struct*/ QTableWidget {
  pub fn currentCellChanged<RetType, T: QTableWidget_currentCellChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.currentCellChanged(self);
    // return 1;
  }
}

pub trait QTableWidget_currentCellChanged<RetType> {
  fn currentCellChanged(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::currentCellChanged(int currentRow, int currentColumn, int previousRow, int previousColumn);
impl<'a> /*trait*/ QTableWidget_currentCellChanged<()> for (i32, i32, i32, i32) {
  fn currentCellChanged(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget18currentCellChangedEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN12QTableWidget18currentCellChangedEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto:  void QTableWidget::cellDoubleClicked(int row, int column);
impl /*struct*/ QTableWidget {
  pub fn cellDoubleClicked<RetType, T: QTableWidget_cellDoubleClicked<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.cellDoubleClicked(self);
    // return 1;
  }
}

pub trait QTableWidget_cellDoubleClicked<RetType> {
  fn cellDoubleClicked(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::cellDoubleClicked(int row, int column);
impl<'a> /*trait*/ QTableWidget_cellDoubleClicked<()> for (i32, i32) {
  fn cellDoubleClicked(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget17cellDoubleClickedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QTableWidget17cellDoubleClickedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  int QTableWidget::rowCount();
impl /*struct*/ QTableWidget {
  pub fn rowCount<RetType, T: QTableWidget_rowCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QTableWidget_rowCount<RetType> {
  fn rowCount(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  int QTableWidget::rowCount();
impl<'a> /*trait*/ QTableWidget_rowCount<i32> for () {
  fn rowCount(self , rsthis: &mut QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget8rowCountEv()};
    let mut ret = unsafe {_ZNK12QTableWidget8rowCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QTableWidgetItem * QTableWidget::item(int row, int column);
impl /*struct*/ QTableWidget {
  pub fn item<RetType, T: QTableWidget_item<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.item(self);
    // return 1;
  }
}

pub trait QTableWidget_item<RetType> {
  fn item(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  QTableWidgetItem * QTableWidget::item(int row, int column);
impl<'a> /*trait*/ QTableWidget_item<QTableWidgetItem> for (i32, i32) {
  fn item(self , rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget4itemEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK12QTableWidget4itemEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTableWidget::NewQTableWidget(QWidget * parent);
impl<'a> /*trait*/ QTableWidget_NewQTableWidget for (&'a mut QWidget) {
  fn NewQTableWidget(self) -> QTableWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidgetC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QTableWidgetC1EP7QWidget(qthis, arg0)};
    let rsthis = QTableWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QTableWidget::setVerticalHeaderLabels(const QStringList & labels);
impl /*struct*/ QTableWidget {
  pub fn setVerticalHeaderLabels<RetType, T: QTableWidget_setVerticalHeaderLabels<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setVerticalHeaderLabels(self);
    // return 1;
  }
}

pub trait QTableWidget_setVerticalHeaderLabels<RetType> {
  fn setVerticalHeaderLabels(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::setVerticalHeaderLabels(const QStringList & labels);
impl<'a> /*trait*/ QTableWidget_setVerticalHeaderLabels<()> for (&'a  QStringList) {
  fn setVerticalHeaderLabels(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget23setVerticalHeaderLabelsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget23setVerticalHeaderLabelsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  const QTableWidgetItem * QTableWidget::itemPrototype();
impl /*struct*/ QTableWidget {
  pub fn itemPrototype<RetType, T: QTableWidget_itemPrototype<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.itemPrototype(self);
    // return 1;
  }
}

pub trait QTableWidget_itemPrototype<RetType> {
  fn itemPrototype(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  const QTableWidgetItem * QTableWidget::itemPrototype();
impl<'a> /*trait*/ QTableWidget_itemPrototype<QTableWidgetItem> for () {
  fn itemPrototype(self , rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget13itemPrototypeEv()};
    let mut ret = unsafe {_ZNK12QTableWidget13itemPrototypeEv(rsthis.qclsinst)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QTableWidgetItem * QTableWidget::itemAt(const QPoint & p);
impl /*struct*/ QTableWidget {
  pub fn itemAt<RetType, T: QTableWidget_itemAt<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.itemAt(self);
    // return 1;
  }
}

pub trait QTableWidget_itemAt<RetType> {
  fn itemAt(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  QTableWidgetItem * QTableWidget::itemAt(const QPoint & p);
impl<'a> /*trait*/ QTableWidget_itemAt<QTableWidgetItem> for (&'a  QPoint) {
  fn itemAt(self , rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget6itemAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QTableWidget6itemAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTableWidget::clearContents();
impl /*struct*/ QTableWidget {
  pub fn clearContents<RetType, T: QTableWidget_clearContents<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.clearContents(self);
    // return 1;
  }
}

pub trait QTableWidget_clearContents<RetType> {
  fn clearContents(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::clearContents();
impl<'a> /*trait*/ QTableWidget_clearContents<()> for () {
  fn clearContents(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget13clearContentsEv()};
     unsafe {_ZN12QTableWidget13clearContentsEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QTableWidget::itemPressed(QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn itemPressed<RetType, T: QTableWidget_itemPressed<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.itemPressed(self);
    // return 1;
  }
}

pub trait QTableWidget_itemPressed<RetType> {
  fn itemPressed(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::itemPressed(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_itemPressed<()> for (&'a mut QTableWidgetItem) {
  fn itemPressed(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget11itemPressedEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget11itemPressedEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QTableWidgetItem * QTableWidget::itemAt(int x, int y);
impl<'a> /*trait*/ QTableWidget_itemAt<QTableWidgetItem> for (i32, i32) {
  fn itemAt(self , rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget6itemAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK12QTableWidget6itemAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTableWidget::setCurrentCell(int row, int column);
impl /*struct*/ QTableWidget {
  pub fn setCurrentCell<RetType, T: QTableWidget_setCurrentCell<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setCurrentCell(self);
    // return 1;
  }
}

pub trait QTableWidget_setCurrentCell<RetType> {
  fn setCurrentCell(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::setCurrentCell(int row, int column);
impl<'a> /*trait*/ QTableWidget_setCurrentCell<()> for (i32, i32) {
  fn setCurrentCell(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget14setCurrentCellEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QTableWidget14setCurrentCellEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QTableWidget::setRowCount(int rows);
impl /*struct*/ QTableWidget {
  pub fn setRowCount<RetType, T: QTableWidget_setRowCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setRowCount(self);
    // return 1;
  }
}

pub trait QTableWidget_setRowCount<RetType> {
  fn setRowCount(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::setRowCount(int rows);
impl<'a> /*trait*/ QTableWidget_setRowCount<()> for (i32) {
  fn setRowCount(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget11setRowCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QTableWidget11setRowCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTableWidget::setHorizontalHeaderItem(int column, QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn setHorizontalHeaderItem<RetType, T: QTableWidget_setHorizontalHeaderItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setHorizontalHeaderItem(self);
    // return 1;
  }
}

pub trait QTableWidget_setHorizontalHeaderItem<RetType> {
  fn setHorizontalHeaderItem(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::setHorizontalHeaderItem(int column, QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_setHorizontalHeaderItem<()> for (i32, &'a mut QTableWidgetItem) {
  fn setHorizontalHeaderItem(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget23setHorizontalHeaderItemEiP16QTableWidgetItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget23setHorizontalHeaderItemEiP16QTableWidgetItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  int QTableWidget::visualColumn(int logicalColumn);
impl /*struct*/ QTableWidget {
  pub fn visualColumn<RetType, T: QTableWidget_visualColumn<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.visualColumn(self);
    // return 1;
  }
}

pub trait QTableWidget_visualColumn<RetType> {
  fn visualColumn(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  int QTableWidget::visualColumn(int logicalColumn);
impl<'a> /*trait*/ QTableWidget_visualColumn<i32> for (i32) {
  fn visualColumn(self , rsthis: &mut QTableWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget12visualColumnEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK12QTableWidget12visualColumnEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QTableWidget::itemEntered(QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn itemEntered<RetType, T: QTableWidget_itemEntered<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.itemEntered(self);
    // return 1;
  }
}

pub trait QTableWidget_itemEntered<RetType> {
  fn itemEntered(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::itemEntered(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_itemEntered<()> for (&'a mut QTableWidgetItem) {
  fn itemEntered(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget11itemEnteredEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget11itemEnteredEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QTableWidgetItem * QTableWidget::takeHorizontalHeaderItem(int column);
impl /*struct*/ QTableWidget {
  pub fn takeHorizontalHeaderItem<RetType, T: QTableWidget_takeHorizontalHeaderItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.takeHorizontalHeaderItem(self);
    // return 1;
  }
}

pub trait QTableWidget_takeHorizontalHeaderItem<RetType> {
  fn takeHorizontalHeaderItem(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  QTableWidgetItem * QTableWidget::takeHorizontalHeaderItem(int column);
impl<'a> /*trait*/ QTableWidget_takeHorizontalHeaderItem<QTableWidgetItem> for (i32) {
  fn takeHorizontalHeaderItem(self , rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget24takeHorizontalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN12QTableWidget24takeHorizontalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QTableWidgetItem * QTableWidget::verticalHeaderItem(int row);
impl /*struct*/ QTableWidget {
  pub fn verticalHeaderItem<RetType, T: QTableWidget_verticalHeaderItem<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.verticalHeaderItem(self);
    // return 1;
  }
}

pub trait QTableWidget_verticalHeaderItem<RetType> {
  fn verticalHeaderItem(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  QTableWidgetItem * QTableWidget::verticalHeaderItem(int row);
impl<'a> /*trait*/ QTableWidget_verticalHeaderItem<QTableWidgetItem> for (i32) {
  fn verticalHeaderItem(self , rsthis: &mut QTableWidget) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTableWidget18verticalHeaderItemEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK12QTableWidget18verticalHeaderItemEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QTableWidget::clear();
impl /*struct*/ QTableWidget {
  pub fn clear<RetType, T: QTableWidget_clear<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QTableWidget_clear<RetType> {
  fn clear(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::clear();
impl<'a> /*trait*/ QTableWidget_clear<()> for () {
  fn clear(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget5clearEv()};
     unsafe {_ZN12QTableWidget5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QTableWidget::insertColumn(int column);
impl /*struct*/ QTableWidget {
  pub fn insertColumn<RetType, T: QTableWidget_insertColumn<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.insertColumn(self);
    // return 1;
  }
}

pub trait QTableWidget_insertColumn<RetType> {
  fn insertColumn(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::insertColumn(int column);
impl<'a> /*trait*/ QTableWidget_insertColumn<()> for (i32) {
  fn insertColumn(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget12insertColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QTableWidget12insertColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QTableWidget::cellActivated(int row, int column);
impl /*struct*/ QTableWidget {
  pub fn cellActivated<RetType, T: QTableWidget_cellActivated<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.cellActivated(self);
    // return 1;
  }
}

pub trait QTableWidget_cellActivated<RetType> {
  fn cellActivated(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::cellActivated(int row, int column);
impl<'a> /*trait*/ QTableWidget_cellActivated<()> for (i32, i32) {
  fn cellActivated(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget13cellActivatedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QTableWidget13cellActivatedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QTableWidget::itemClicked(QTableWidgetItem * item);
impl /*struct*/ QTableWidget {
  pub fn itemClicked<RetType, T: QTableWidget_itemClicked<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.itemClicked(self);
    // return 1;
  }
}

pub trait QTableWidget_itemClicked<RetType> {
  fn itemClicked(self , rsthis: &mut QTableWidget) -> RetType;
}

// proto:  void QTableWidget::itemClicked(QTableWidgetItem * item);
impl<'a> /*trait*/ QTableWidget_itemClicked<()> for (&'a mut QTableWidgetItem) {
  fn itemClicked(self , rsthis: &mut QTableWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTableWidget11itemClickedEP16QTableWidgetItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTableWidget11itemClickedEP16QTableWidgetItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

