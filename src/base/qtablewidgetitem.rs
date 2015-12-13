// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qcolor::QColor;
use super::qvariant::QVariant;
use super::qstring::QString;
use super::qsize::QSize;
use super::qbrush::QBrush;
use super::qfont::QFont;
use super::qicon::QIcon;
use super::qdatastream::QDataStream;
use super::qtablewidget::QTableWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QColor QTableWidgetItem::backgroundColor();
  fn _ZNK16QTableWidgetItem15backgroundColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QVariant QTableWidgetItem::data(int role);
  fn _ZNK16QTableWidgetItem4dataEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTableWidgetItem::setSelected(bool select);
  fn _ZN16QTableWidgetItem11setSelectedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTableWidgetItem::setStatusTip(const QString & statusTip);
  fn _ZN16QTableWidgetItem12setStatusTipERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QColor QTableWidgetItem::textColor();
  fn _ZNK16QTableWidgetItem9textColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTableWidgetItem::FreeQTableWidgetItem();
  fn _ZN16QTableWidgetItemD0Ev(qthis: *mut c_void) ;
  // proto:  QString QTableWidgetItem::text();
  fn _ZNK16QTableWidgetItem4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTableWidgetItem::setSizeHint(const QSize & size);
  fn _ZN16QTableWidgetItem11setSizeHintERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QBrush QTableWidgetItem::foreground();
  fn _ZNK16QTableWidgetItem10foregroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTableWidgetItem::type_();
  fn _ZNK16QTableWidgetItem4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTableWidgetItem::column();
  fn _ZNK16QTableWidgetItem6columnEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTableWidgetItem::setTextAlignment(int alignment);
  fn _ZN16QTableWidgetItem16setTextAlignmentEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QFont QTableWidgetItem::font();
  fn _ZNK16QTableWidgetItem4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QIcon QTableWidgetItem::icon();
  fn _ZNK16QTableWidgetItem4iconEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTableWidgetItem::write(QDataStream & out);
  fn _ZNK16QTableWidgetItem5writeER11QDataStream(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidgetItem::NewQTableWidgetItem(const QTableWidgetItem & other);
  fn _ZN16QTableWidgetItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QBrush QTableWidgetItem::background();
  fn _ZNK16QTableWidgetItem10backgroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTableWidgetItem::setIcon(const QIcon & icon);
  fn _ZN16QTableWidgetItem7setIconERK5QIcon(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidgetItem::NewQTableWidgetItem(const QString & text, int type);
  fn _ZN16QTableWidgetItemC1ERK7QStringi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  QString QTableWidgetItem::statusTip();
  fn _ZNK16QTableWidgetItem9statusTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTableWidgetItem * QTableWidgetItem::clone();
  fn _ZNK16QTableWidgetItem5cloneEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTableWidgetItem::NewQTableWidgetItem(int type);
  fn _ZN16QTableWidgetItemC1Ei(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTableWidgetItem::setWhatsThis(const QString & whatsThis);
  fn _ZN16QTableWidgetItem12setWhatsThisERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSize QTableWidgetItem::sizeHint();
  fn _ZNK16QTableWidgetItem8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTableWidgetItem::setForeground(const QBrush & brush);
  fn _ZN16QTableWidgetItem13setForegroundERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTableWidgetItem::row();
  fn _ZNK16QTableWidgetItem3rowEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTableWidgetItem::setData(int role, const QVariant & value);
  fn _ZN16QTableWidgetItem7setDataEiRK8QVariant(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  QTableWidget * QTableWidgetItem::tableWidget();
  fn _ZNK16QTableWidgetItem11tableWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTableWidgetItem::NewQTableWidgetItem(const QIcon & icon, const QString & text, int type);
  fn _ZN16QTableWidgetItemC1ERK5QIconRK7QStringi(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) ;
  // proto:  int QTableWidgetItem::textAlignment();
  fn _ZNK16QTableWidgetItem13textAlignmentEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTableWidgetItem::read(QDataStream & in);
  fn _ZN16QTableWidgetItem4readER11QDataStream(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QTableWidgetItem::toolTip();
  fn _ZNK16QTableWidgetItem7toolTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTableWidgetItem::isSelected();
  fn _ZNK16QTableWidgetItem10isSelectedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTableWidgetItem::setBackgroundColor(const QColor & color);
  fn _ZN16QTableWidgetItem18setBackgroundColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidgetItem::setBackground(const QBrush & brush);
  fn _ZN16QTableWidgetItem13setBackgroundERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidgetItem::setFont(const QFont & font);
  fn _ZN16QTableWidgetItem7setFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidgetItem::setTextColor(const QColor & color);
  fn _ZN16QTableWidgetItem12setTextColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTableWidgetItem::setText(const QString & text);
  fn _ZN16QTableWidgetItem7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QTableWidgetItem::whatsThis();
  fn _ZNK16QTableWidgetItem9whatsThisEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTableWidgetItem::setToolTip(const QString & toolTip);
  fn _ZN16QTableWidgetItem10setToolTipERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QTableWidgetItem)=1
pub struct QTableWidgetItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTableWidgetItem {
  pub fn backgroundColor<T: QTableWidgetItem_backgroundColor>(&mut self, value: T) -> QColor {
    return value.backgroundColor(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_backgroundColor {
  fn backgroundColor(self, rsthis: &mut QTableWidgetItem) -> QColor;
}

// proto:  QColor QTableWidgetItem::backgroundColor();
impl<'a> /*trait*/ QTableWidgetItem_backgroundColor for () {
  fn backgroundColor(self, rsthis: &mut QTableWidgetItem) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem15backgroundColorEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem15backgroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn data<T: QTableWidgetItem_data>(&mut self, value: T) -> QVariant {
    return value.data(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_data {
  fn data(self, rsthis: &mut QTableWidgetItem) -> QVariant;
}

// proto:  QVariant QTableWidgetItem::data(int role);
impl<'a> /*trait*/ QTableWidgetItem_data for (i32) {
  fn data(self, rsthis: &mut QTableWidgetItem) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4dataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK16QTableWidgetItem4dataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setSelected<T: QTableWidgetItem_setSelected>(&mut self, value: T)  {
     value.setSelected(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setSelected {
  fn setSelected(self, rsthis: &mut QTableWidgetItem) ;
}

// proto:  void QTableWidgetItem::setSelected(bool select);
impl<'a> /*trait*/ QTableWidgetItem_setSelected for (i8) {
  fn setSelected(self, rsthis: &mut QTableWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem11setSelectedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN16QTableWidgetItem11setSelectedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setStatusTip<T: QTableWidgetItem_setStatusTip>(&mut self, value: T)  {
     value.setStatusTip(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setStatusTip {
  fn setStatusTip(self, rsthis: &mut QTableWidgetItem) ;
}

// proto:  void QTableWidgetItem::setStatusTip(const QString & statusTip);
impl<'a> /*trait*/ QTableWidgetItem_setStatusTip for (&'a  QString) {
  fn setStatusTip(self, rsthis: &mut QTableWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem12setStatusTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem12setStatusTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn textColor<T: QTableWidgetItem_textColor>(&mut self, value: T) -> QColor {
    return value.textColor(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_textColor {
  fn textColor(self, rsthis: &mut QTableWidgetItem) -> QColor;
}

// proto:  QColor QTableWidgetItem::textColor();
impl<'a> /*trait*/ QTableWidgetItem_textColor for () {
  fn textColor(self, rsthis: &mut QTableWidgetItem) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem9textColorEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem9textColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn FreeQTableWidgetItem<T: QTableWidgetItem_FreeQTableWidgetItem>(&mut self, value: T)  {
     value.FreeQTableWidgetItem(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_FreeQTableWidgetItem {
  fn FreeQTableWidgetItem(self, rsthis: &mut QTableWidgetItem) ;
}

// proto:  void QTableWidgetItem::FreeQTableWidgetItem();
impl<'a> /*trait*/ QTableWidgetItem_FreeQTableWidgetItem for () {
  fn FreeQTableWidgetItem(self, rsthis: &mut QTableWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItemD0Ev()};
     unsafe {_ZN16QTableWidgetItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn text<T: QTableWidgetItem_text>(&mut self, value: T) -> QString {
    return value.text(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_text {
  fn text(self, rsthis: &mut QTableWidgetItem) -> QString;
}

// proto:  QString QTableWidgetItem::text();
impl<'a> /*trait*/ QTableWidgetItem_text for () {
  fn text(self, rsthis: &mut QTableWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4textEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setSizeHint<T: QTableWidgetItem_setSizeHint>(&mut self, value: T)  {
     value.setSizeHint(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setSizeHint {
  fn setSizeHint(self, rsthis: &mut QTableWidgetItem) ;
}

// proto:  void QTableWidgetItem::setSizeHint(const QSize & size);
impl<'a> /*trait*/ QTableWidgetItem_setSizeHint for (&'a  QSize) {
  fn setSizeHint(self, rsthis: &mut QTableWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem11setSizeHintERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem11setSizeHintERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn foreground<T: QTableWidgetItem_foreground>(&mut self, value: T) -> QBrush {
    return value.foreground(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_foreground {
  fn foreground(self, rsthis: &mut QTableWidgetItem) -> QBrush;
}

// proto:  QBrush QTableWidgetItem::foreground();
impl<'a> /*trait*/ QTableWidgetItem_foreground for () {
  fn foreground(self, rsthis: &mut QTableWidgetItem) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem10foregroundEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem10foregroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn type_<T: QTableWidgetItem_type_>(&mut self, value: T) -> i32 {
    return value.type_(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_type_ {
  fn type_(self, rsthis: &mut QTableWidgetItem) -> i32;
}

// proto:  int QTableWidgetItem::type_();
impl<'a> /*trait*/ QTableWidgetItem_type_ for () {
  fn type_(self, rsthis: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4typeEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn column<T: QTableWidgetItem_column>(&mut self, value: T) -> i32 {
    return value.column(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_column {
  fn column(self, rsthis: &mut QTableWidgetItem) -> i32;
}

// proto:  int QTableWidgetItem::column();
impl<'a> /*trait*/ QTableWidgetItem_column for () {
  fn column(self, rsthis: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem6columnEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem6columnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setTextAlignment<T: QTableWidgetItem_setTextAlignment>(&mut self, value: T)  {
     value.setTextAlignment(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setTextAlignment {
  fn setTextAlignment(self, rsthis: &mut QTableWidgetItem) ;
}

// proto:  void QTableWidgetItem::setTextAlignment(int alignment);
impl<'a> /*trait*/ QTableWidgetItem_setTextAlignment for (i32) {
  fn setTextAlignment(self, rsthis: &mut QTableWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem16setTextAlignmentEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN16QTableWidgetItem16setTextAlignmentEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn font<T: QTableWidgetItem_font>(&mut self, value: T) -> QFont {
    return value.font(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_font {
  fn font(self, rsthis: &mut QTableWidgetItem) -> QFont;
}

// proto:  QFont QTableWidgetItem::font();
impl<'a> /*trait*/ QTableWidgetItem_font for () {
  fn font(self, rsthis: &mut QTableWidgetItem) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4fontEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn icon<T: QTableWidgetItem_icon>(&mut self, value: T) -> QIcon {
    return value.icon(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_icon {
  fn icon(self, rsthis: &mut QTableWidgetItem) -> QIcon;
}

// proto:  QIcon QTableWidgetItem::icon();
impl<'a> /*trait*/ QTableWidgetItem_icon for () {
  fn icon(self, rsthis: &mut QTableWidgetItem) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem4iconEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem4iconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn write<T: QTableWidgetItem_write>(&mut self, value: T)  {
     value.write(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_write {
  fn write(self, rsthis: &mut QTableWidgetItem) ;
}

// proto:  void QTableWidgetItem::write(QDataStream & out);
impl<'a> /*trait*/ QTableWidgetItem_write for (&'a mut QDataStream) {
  fn write(self, rsthis: &mut QTableWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem5writeER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK16QTableWidgetItem5writeER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn NewQTableWidgetItem<T: QTableWidgetItem_NewQTableWidgetItem>(value: T) -> QTableWidgetItem {
    let rsthis = value.NewQTableWidgetItem();
    return rsthis;
    // return 1;
  }
}

pub trait QTableWidgetItem_NewQTableWidgetItem {
  fn NewQTableWidgetItem(self) -> QTableWidgetItem;
}

// proto: void QTableWidgetItem::NewQTableWidgetItem(const QTableWidgetItem & other);
impl<'a> /*trait*/ QTableWidgetItem_NewQTableWidgetItem for (&'a  QTableWidgetItem) {
  fn NewQTableWidgetItem(self) -> QTableWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QTableWidgetItemC1ERKS_(qthis, arg0)};
    let rsthis = QTableWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn background<T: QTableWidgetItem_background>(&mut self, value: T) -> QBrush {
    return value.background(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_background {
  fn background(self, rsthis: &mut QTableWidgetItem) -> QBrush;
}

// proto:  QBrush QTableWidgetItem::background();
impl<'a> /*trait*/ QTableWidgetItem_background for () {
  fn background(self, rsthis: &mut QTableWidgetItem) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem10backgroundEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem10backgroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setIcon<T: QTableWidgetItem_setIcon>(&mut self, value: T)  {
     value.setIcon(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setIcon {
  fn setIcon(self, rsthis: &mut QTableWidgetItem) ;
}

// proto:  void QTableWidgetItem::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QTableWidgetItem_setIcon for (&'a  QIcon) {
  fn setIcon(self, rsthis: &mut QTableWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem7setIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QTableWidgetItem::NewQTableWidgetItem(const QString & text, int type);
impl<'a> /*trait*/ QTableWidgetItem_NewQTableWidgetItem for (&'a  QString, i32) {
  fn NewQTableWidgetItem(self) -> QTableWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItemC1ERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN16QTableWidgetItemC1ERK7QStringi(qthis, arg0, arg1)};
    let rsthis = QTableWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn statusTip<T: QTableWidgetItem_statusTip>(&mut self, value: T) -> QString {
    return value.statusTip(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_statusTip {
  fn statusTip(self, rsthis: &mut QTableWidgetItem) -> QString;
}

// proto:  QString QTableWidgetItem::statusTip();
impl<'a> /*trait*/ QTableWidgetItem_statusTip for () {
  fn statusTip(self, rsthis: &mut QTableWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem9statusTipEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem9statusTipEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn clone<T: QTableWidgetItem_clone>(&mut self, value: T) -> QTableWidgetItem {
    return value.clone(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_clone {
  fn clone(self, rsthis: &mut QTableWidgetItem) -> QTableWidgetItem;
}

// proto:  QTableWidgetItem * QTableWidgetItem::clone();
impl<'a> /*trait*/ QTableWidgetItem_clone for () {
  fn clone(self, rsthis: &mut QTableWidgetItem) -> QTableWidgetItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem5cloneEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem5cloneEv(rsthis.qclsinst)};
    let mut ret1 = QTableWidgetItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTableWidgetItem::NewQTableWidgetItem(int type);
impl<'a> /*trait*/ QTableWidgetItem_NewQTableWidgetItem for (i32) {
  fn NewQTableWidgetItem(self) -> QTableWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItemC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN16QTableWidgetItemC1Ei(qthis, arg0)};
    let rsthis = QTableWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setWhatsThis<T: QTableWidgetItem_setWhatsThis>(&mut self, value: T)  {
     value.setWhatsThis(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setWhatsThis {
  fn setWhatsThis(self, rsthis: &mut QTableWidgetItem) ;
}

// proto:  void QTableWidgetItem::setWhatsThis(const QString & whatsThis);
impl<'a> /*trait*/ QTableWidgetItem_setWhatsThis for (&'a  QString) {
  fn setWhatsThis(self, rsthis: &mut QTableWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem12setWhatsThisERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn sizeHint<T: QTableWidgetItem_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_sizeHint {
  fn sizeHint(self, rsthis: &mut QTableWidgetItem) -> QSize;
}

// proto:  QSize QTableWidgetItem::sizeHint();
impl<'a> /*trait*/ QTableWidgetItem_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QTableWidgetItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem8sizeHintEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setForeground<T: QTableWidgetItem_setForeground>(&mut self, value: T)  {
     value.setForeground(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setForeground {
  fn setForeground(self, rsthis: &mut QTableWidgetItem) ;
}

// proto:  void QTableWidgetItem::setForeground(const QBrush & brush);
impl<'a> /*trait*/ QTableWidgetItem_setForeground for (&'a  QBrush) {
  fn setForeground(self, rsthis: &mut QTableWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem13setForegroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem13setForegroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn row<T: QTableWidgetItem_row>(&mut self, value: T) -> i32 {
    return value.row(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_row {
  fn row(self, rsthis: &mut QTableWidgetItem) -> i32;
}

// proto:  int QTableWidgetItem::row();
impl<'a> /*trait*/ QTableWidgetItem_row for () {
  fn row(self, rsthis: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem3rowEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem3rowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setData<T: QTableWidgetItem_setData>(&mut self, value: T)  {
     value.setData(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setData {
  fn setData(self, rsthis: &mut QTableWidgetItem) ;
}

// proto:  void QTableWidgetItem::setData(int role, const QVariant & value);
impl<'a> /*trait*/ QTableWidgetItem_setData for (i32, &'a  QVariant) {
  fn setData(self, rsthis: &mut QTableWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem7setDataEiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem7setDataEiRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn tableWidget<T: QTableWidgetItem_tableWidget>(&mut self, value: T) -> QTableWidget {
    return value.tableWidget(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_tableWidget {
  fn tableWidget(self, rsthis: &mut QTableWidgetItem) -> QTableWidget;
}

// proto:  QTableWidget * QTableWidgetItem::tableWidget();
impl<'a> /*trait*/ QTableWidgetItem_tableWidget for () {
  fn tableWidget(self, rsthis: &mut QTableWidgetItem) -> QTableWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem11tableWidgetEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem11tableWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QTableWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QTableWidgetItem::NewQTableWidgetItem(const QIcon & icon, const QString & text, int type);
impl<'a> /*trait*/ QTableWidgetItem_NewQTableWidgetItem for (&'a  QIcon, &'a  QString, i32) {
  fn NewQTableWidgetItem(self) -> QTableWidgetItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItemC1ERK5QIconRK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN16QTableWidgetItemC1ERK5QIconRK7QStringi(qthis, arg0, arg1, arg2)};
    let rsthis = QTableWidgetItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn textAlignment<T: QTableWidgetItem_textAlignment>(&mut self, value: T) -> i32 {
    return value.textAlignment(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_textAlignment {
  fn textAlignment(self, rsthis: &mut QTableWidgetItem) -> i32;
}

// proto:  int QTableWidgetItem::textAlignment();
impl<'a> /*trait*/ QTableWidgetItem_textAlignment for () {
  fn textAlignment(self, rsthis: &mut QTableWidgetItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem13textAlignmentEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem13textAlignmentEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn read<T: QTableWidgetItem_read>(&mut self, value: T)  {
     value.read(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_read {
  fn read(self, rsthis: &mut QTableWidgetItem) ;
}

// proto:  void QTableWidgetItem::read(QDataStream & in);
impl<'a> /*trait*/ QTableWidgetItem_read for (&'a mut QDataStream) {
  fn read(self, rsthis: &mut QTableWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem4readER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem4readER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn toolTip<T: QTableWidgetItem_toolTip>(&mut self, value: T) -> QString {
    return value.toolTip(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_toolTip {
  fn toolTip(self, rsthis: &mut QTableWidgetItem) -> QString;
}

// proto:  QString QTableWidgetItem::toolTip();
impl<'a> /*trait*/ QTableWidgetItem_toolTip for () {
  fn toolTip(self, rsthis: &mut QTableWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem7toolTipEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn isSelected<T: QTableWidgetItem_isSelected>(&mut self, value: T) -> i8 {
    return value.isSelected(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_isSelected {
  fn isSelected(self, rsthis: &mut QTableWidgetItem) -> i8;
}

// proto:  bool QTableWidgetItem::isSelected();
impl<'a> /*trait*/ QTableWidgetItem_isSelected for () {
  fn isSelected(self, rsthis: &mut QTableWidgetItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem10isSelectedEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem10isSelectedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setBackgroundColor<T: QTableWidgetItem_setBackgroundColor>(&mut self, value: T)  {
     value.setBackgroundColor(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setBackgroundColor {
  fn setBackgroundColor(self, rsthis: &mut QTableWidgetItem) ;
}

// proto:  void QTableWidgetItem::setBackgroundColor(const QColor & color);
impl<'a> /*trait*/ QTableWidgetItem_setBackgroundColor for (&'a  QColor) {
  fn setBackgroundColor(self, rsthis: &mut QTableWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem18setBackgroundColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem18setBackgroundColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setBackground<T: QTableWidgetItem_setBackground>(&mut self, value: T)  {
     value.setBackground(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setBackground {
  fn setBackground(self, rsthis: &mut QTableWidgetItem) ;
}

// proto:  void QTableWidgetItem::setBackground(const QBrush & brush);
impl<'a> /*trait*/ QTableWidgetItem_setBackground for (&'a  QBrush) {
  fn setBackground(self, rsthis: &mut QTableWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem13setBackgroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setFont<T: QTableWidgetItem_setFont>(&mut self, value: T)  {
     value.setFont(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setFont {
  fn setFont(self, rsthis: &mut QTableWidgetItem) ;
}

// proto:  void QTableWidgetItem::setFont(const QFont & font);
impl<'a> /*trait*/ QTableWidgetItem_setFont for (&'a  QFont) {
  fn setFont(self, rsthis: &mut QTableWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setTextColor<T: QTableWidgetItem_setTextColor>(&mut self, value: T)  {
     value.setTextColor(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setTextColor {
  fn setTextColor(self, rsthis: &mut QTableWidgetItem) ;
}

// proto:  void QTableWidgetItem::setTextColor(const QColor & color);
impl<'a> /*trait*/ QTableWidgetItem_setTextColor for (&'a  QColor) {
  fn setTextColor(self, rsthis: &mut QTableWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem12setTextColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem12setTextColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setText<T: QTableWidgetItem_setText>(&mut self, value: T)  {
     value.setText(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setText {
  fn setText(self, rsthis: &mut QTableWidgetItem) ;
}

// proto:  void QTableWidgetItem::setText(const QString & text);
impl<'a> /*trait*/ QTableWidgetItem_setText for (&'a  QString) {
  fn setText(self, rsthis: &mut QTableWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn whatsThis<T: QTableWidgetItem_whatsThis>(&mut self, value: T) -> QString {
    return value.whatsThis(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_whatsThis {
  fn whatsThis(self, rsthis: &mut QTableWidgetItem) -> QString;
}

// proto:  QString QTableWidgetItem::whatsThis();
impl<'a> /*trait*/ QTableWidgetItem_whatsThis for () {
  fn whatsThis(self, rsthis: &mut QTableWidgetItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QTableWidgetItem9whatsThisEv()};
    let mut ret = unsafe {_ZNK16QTableWidgetItem9whatsThisEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTableWidgetItem {
  pub fn setToolTip<T: QTableWidgetItem_setToolTip>(&mut self, value: T)  {
     value.setToolTip(self);
    // return 1;
  }
}

pub trait QTableWidgetItem_setToolTip {
  fn setToolTip(self, rsthis: &mut QTableWidgetItem) ;
}

// proto:  void QTableWidgetItem::setToolTip(const QString & toolTip);
impl<'a> /*trait*/ QTableWidgetItem_setToolTip for (&'a  QString) {
  fn setToolTip(self, rsthis: &mut QTableWidgetItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QTableWidgetItem10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QTableWidgetItem10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

