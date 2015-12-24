// auto generated, do not modify.
// created: Thu Dec 24 23:00:39 2015
// src-file: /QtWidgets/qcombobox.h
// dst-file: /src/widgets/qcombobox.rs
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
use super::qwidget::QWidget; // 773
use std::ops::Deref;
use super::super::core::qsize::QSize; // 771
use super::super::gui::qicon::QIcon; // 771
use super::super::core::qstring::QString; // 771
use super::super::core::qvariant::QVariant; // 771
use super::super::core::qstringlist::QStringList; // 771
use super::super::core::qabstractitemmodel::QModelIndex; // 771
use super::super::gui::qvalidator::QValidator; // 771
use super::qcompleter::QCompleter; // 773
use super::qlineedit::QLineEdit; // 773
use super::super::core::qcoreevent::QEvent; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]

// #[link(name = "QtInline")]

extern {
  // proto:  void QComboBox::clearEditText();
  fn _ZN9QComboBox13clearEditTextEv(qthis: *mut c_void);
  // proto:  void QComboBox::setAutoCompletion(bool enable);
  fn _ZN9QComboBox17setAutoCompletionEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QComboBox::setFrame(bool );
  fn _ZN9QComboBox8setFrameEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QComboBox::setIconSize(const QSize & size);
  fn _ZN9QComboBox11setIconSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QAbstractItemView * QComboBox::view();
  fn _ZNK9QComboBox4viewEv(qthis: *mut c_void);
  // proto:  QSize QComboBox::minimumSizeHint();
  fn _ZNK9QComboBox15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QComboBox::clear();
  fn _ZN9QComboBox5clearEv(qthis: *mut c_void);
  // proto:  int QComboBox::maxCount();
  fn _ZNK9QComboBox8maxCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QComboBox::addItem(const QIcon & icon, const QString & text, const QVariant & userData);
  fn _ZN9QComboBox7addItemERK5QIconRK7QStringRK8QVariant(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QComboBox::insertItems(int index, const QStringList & texts);
  fn _ZN9QComboBox11insertItemsEiRK11QStringList(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  QSize QComboBox::iconSize();
  fn _ZNK9QComboBox8iconSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QComboBox::rootModelIndex();
  fn _ZNK9QComboBox14rootModelIndexEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QComboBox::setEditable(bool editable);
  fn _ZN9QComboBox11setEditableEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QComboBox::setItemIcon(int index, const QIcon & icon);
  fn _ZN9QComboBox11setItemIconEiRK5QIcon(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QComboBox::currentTextChanged(const QString & );
  fn _ZN9QComboBox18currentTextChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QComboBox::autoCompletion();
  fn _ZNK9QComboBox14autoCompletionEv(qthis: *mut c_void) -> c_char;
  // proto:  QVariant QComboBox::currentData(int role);
  fn _ZNK9QComboBox11currentDataEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  bool QComboBox::hasFrame();
  fn _ZNK9QComboBox8hasFrameEv(qthis: *mut c_void) -> c_char;
  // proto:  const QValidator * QComboBox::validator();
  fn _ZNK9QComboBox9validatorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QComboBox::itemText(int index);
  fn _ZNK9QComboBox8itemTextEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QComboBox::setItemData(int index, const QVariant & value, int role);
  fn _ZN9QComboBox11setItemDataEiRK8QVarianti(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_int);
  // proto:  void QComboBox::highlighted(int index);
  fn _ZN9QComboBox11highlightedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QComboBox::hidePopup();
  fn _ZN9QComboBox9hidePopupEv(qthis: *mut c_void);
  // proto:  void QComboBox::insertItem(int index, const QIcon & icon, const QString & text, const QVariant & userData);
  fn _ZN9QComboBox10insertItemEiRK5QIconRK7QStringRK8QVariant(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void);
  // proto:  void QComboBox::setCurrentText(const QString & text);
  fn _ZN9QComboBox14setCurrentTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QComboBox::highlighted(const QString & );
  fn _ZN9QComboBox11highlightedERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QComboBox::editTextChanged(const QString & );
  fn _ZN9QComboBox15editTextChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QComboBox::modelColumn();
  fn _ZNK9QComboBox11modelColumnEv(qthis: *mut c_void) -> c_int;
  // proto:  QSize QComboBox::sizeHint();
  fn _ZNK9QComboBox8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QVariant QComboBox::itemData(int index, int role);
  fn _ZNK9QComboBox8itemDataEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QComboBox::activated(int index);
  fn _ZN9QComboBox9activatedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QComboBox::setCompleter(QCompleter * c);
  fn _ZN9QComboBox12setCompleterEP10QCompleter(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QComboBox::activated(const QString & );
  fn _ZN9QComboBox9activatedERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QComboBox::maxVisibleItems();
  fn _ZNK9QComboBox15maxVisibleItemsEv(qthis: *mut c_void) -> c_int;
  // proto:  void QComboBox::QComboBox(QWidget * parent);
  fn _ZN9QComboBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QComboBox::currentIndexChanged(const QString & );
  fn _ZN9QComboBox19currentIndexChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QComboBox::setCurrentIndex(int index);
  fn _ZN9QComboBox15setCurrentIndexEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QComboBox::QComboBox(const QComboBox & );
  fn _ZN9QComboBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QComboBox::setRootModelIndex(const QModelIndex & index);
  fn _ZN9QComboBox17setRootModelIndexERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QComboBox::setEditText(const QString & text);
  fn _ZN9QComboBox11setEditTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QComboBox::addItem(const QString & text, const QVariant & userData);
  fn _ZN9QComboBox7addItemERK7QStringRK8QVariant(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QCompleter * QComboBox::completer();
  fn _ZNK9QComboBox9completerEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QComboBox::removeItem(int index);
  fn _ZN9QComboBox10removeItemEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QComboBox::count();
  fn _ZNK9QComboBox5countEv(qthis: *mut c_void) -> c_int;
  // proto:  void QComboBox::addItems(const QStringList & texts);
  fn _ZN9QComboBox8addItemsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QComboBox::setMinimumContentsLength(int characters);
  fn _ZN9QComboBox24setMinimumContentsLengthEi(qthis: *mut c_void, arg0: c_int);
  // proto:  bool QComboBox::duplicatesEnabled();
  fn _ZNK9QComboBox17duplicatesEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  void QComboBox::~QComboBox();
  fn _ZN9QComboBoxD0Ev(qthis: *mut c_void);
  // proto:  QAbstractItemModel * QComboBox::model();
  fn _ZNK9QComboBox5modelEv(qthis: *mut c_void);
  // proto:  int QComboBox::minimumContentsLength();
  fn _ZNK9QComboBox21minimumContentsLengthEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QComboBox::isEditable();
  fn _ZNK9QComboBox10isEditableEv(qthis: *mut c_void) -> c_char;
  // proto:  void QComboBox::setMaxCount(int max);
  fn _ZN9QComboBox11setMaxCountEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QComboBox::currentIndex();
  fn _ZNK9QComboBox12currentIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  void QComboBox::setDuplicatesEnabled(bool enable);
  fn _ZN9QComboBox20setDuplicatesEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QString QComboBox::currentText();
  fn _ZNK9QComboBox11currentTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QComboBox::showPopup();
  fn _ZN9QComboBox9showPopupEv(qthis: *mut c_void);
  // proto:  QLineEdit * QComboBox::lineEdit();
  fn _ZNK9QComboBox8lineEditEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAbstractItemDelegate * QComboBox::itemDelegate();
  fn _ZNK9QComboBox12itemDelegateEv(qthis: *mut c_void);
  // proto:  void QComboBox::currentIndexChanged(int index);
  fn _ZN9QComboBox19currentIndexChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QComboBox::setMaxVisibleItems(int maxItems);
  fn _ZN9QComboBox18setMaxVisibleItemsEi(qthis: *mut c_void, arg0: c_int);
  // proto:  bool QComboBox::event(QEvent * event);
  fn _ZN9QComboBox5eventEP6QEvent(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QComboBox::setModelColumn(int visibleColumn);
  fn _ZN9QComboBox14setModelColumnEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QComboBox::setItemText(int index, const QString & text);
  fn _ZN9QComboBox11setItemTextEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QComboBox::setLineEdit(QLineEdit * edit);
  fn _ZN9QComboBox11setLineEditEP9QLineEdit(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QIcon QComboBox::itemIcon(int index);
  fn _ZNK9QComboBox8itemIconEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QComboBox::insertItem(int index, const QString & text, const QVariant & userData);
  fn _ZN9QComboBox10insertItemEiRK7QStringRK8QVariant(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QComboBox::setValidator(const QValidator * v);
  fn _ZN9QComboBox12setValidatorEPK10QValidator(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QComboBox::insertSeparator(int index);
  fn _ZN9QComboBox15insertSeparatorEi(qthis: *mut c_void, arg0: c_int);
  // proto:  const QMetaObject * QComboBox::metaObject();
  fn _ZNK9QComboBox10metaObjectEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QComboBox)=1
pub struct QComboBox {
  qbase: QWidget,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QComboBox {
  pub fn inheritFrom(qthis: *mut c_void) -> QComboBox {
    return QComboBox{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QComboBox {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QComboBox {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  void QComboBox::clearEditText();
impl /*struct*/ QComboBox {
  pub fn clearEditText<RetType, T: QComboBox_clearEditText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearEditText(self);
    // return 1;
  }
}

pub trait QComboBox_clearEditText<RetType> {
  fn clearEditText(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::clearEditText();
impl<'a> /*trait*/ QComboBox_clearEditText<()> for () {
  fn clearEditText(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox13clearEditTextEv()};
     unsafe {_ZN9QComboBox13clearEditTextEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QComboBox::setAutoCompletion(bool enable);
impl /*struct*/ QComboBox {
  pub fn setAutoCompletion<RetType, T: QComboBox_setAutoCompletion<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoCompletion(self);
    // return 1;
  }
}

pub trait QComboBox_setAutoCompletion<RetType> {
  fn setAutoCompletion(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setAutoCompletion(bool enable);
impl<'a> /*trait*/ QComboBox_setAutoCompletion<()> for (i8) {
  fn setAutoCompletion(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox17setAutoCompletionEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QComboBox17setAutoCompletionEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::setFrame(bool );
impl /*struct*/ QComboBox {
  pub fn setFrame<RetType, T: QComboBox_setFrame<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFrame(self);
    // return 1;
  }
}

pub trait QComboBox_setFrame<RetType> {
  fn setFrame(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setFrame(bool );
impl<'a> /*trait*/ QComboBox_setFrame<()> for (i8) {
  fn setFrame(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox8setFrameEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QComboBox8setFrameEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::setIconSize(const QSize & size);
impl /*struct*/ QComboBox {
  pub fn setIconSize<RetType, T: QComboBox_setIconSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIconSize(self);
    // return 1;
  }
}

pub trait QComboBox_setIconSize<RetType> {
  fn setIconSize(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setIconSize(const QSize & size);
impl<'a> /*trait*/ QComboBox_setIconSize<()> for (&'a QSize) {
  fn setIconSize(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox11setIconSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAbstractItemView * QComboBox::view();
impl /*struct*/ QComboBox {
  pub fn view<RetType, T: QComboBox_view<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.view(self);
    // return 1;
  }
}

pub trait QComboBox_view<RetType> {
  fn view(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QAbstractItemView * QComboBox::view();
impl<'a> /*trait*/ QComboBox_view<()> for () {
  fn view(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox4viewEv()};
     unsafe {_ZNK9QComboBox4viewEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QComboBox::minimumSizeHint();
impl /*struct*/ QComboBox {
  pub fn minimumSizeHint<RetType, T: QComboBox_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QComboBox_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QSize QComboBox::minimumSizeHint();
impl<'a> /*trait*/ QComboBox_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QComboBox) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK9QComboBox15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QComboBox::clear();
impl /*struct*/ QComboBox {
  pub fn clear<RetType, T: QComboBox_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QComboBox_clear<RetType> {
  fn clear(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::clear();
impl<'a> /*trait*/ QComboBox_clear<()> for () {
  fn clear(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox5clearEv()};
     unsafe {_ZN9QComboBox5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QComboBox::maxCount();
impl /*struct*/ QComboBox {
  pub fn maxCount<RetType, T: QComboBox_maxCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maxCount(self);
    // return 1;
  }
}

pub trait QComboBox_maxCount<RetType> {
  fn maxCount(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  int QComboBox::maxCount();
impl<'a> /*trait*/ QComboBox_maxCount<i32> for () {
  fn maxCount(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8maxCountEv()};
    let mut ret = unsafe {_ZNK9QComboBox8maxCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QComboBox::addItem(const QIcon & icon, const QString & text, const QVariant & userData);
impl /*struct*/ QComboBox {
  pub fn addItem<RetType, T: QComboBox_addItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addItem(self);
    // return 1;
  }
}

pub trait QComboBox_addItem<RetType> {
  fn addItem(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::addItem(const QIcon & icon, const QString & text, const QVariant & userData);
impl<'a> /*trait*/ QComboBox_addItem<()> for (&'a QIcon, &'a QString, &'a QVariant) {
  fn addItem(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox7addItemERK5QIconRK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox7addItemERK5QIconRK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QComboBox::insertItems(int index, const QStringList & texts);
impl /*struct*/ QComboBox {
  pub fn insertItems<RetType, T: QComboBox_insertItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertItems(self);
    // return 1;
  }
}

pub trait QComboBox_insertItems<RetType> {
  fn insertItems(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::insertItems(int index, const QStringList & texts);
impl<'a> /*trait*/ QComboBox_insertItems<()> for (i32, &'a QStringList) {
  fn insertItems(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11insertItemsEiRK11QStringList()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox11insertItemsEiRK11QStringList(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QSize QComboBox::iconSize();
impl /*struct*/ QComboBox {
  pub fn iconSize<RetType, T: QComboBox_iconSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.iconSize(self);
    // return 1;
  }
}

pub trait QComboBox_iconSize<RetType> {
  fn iconSize(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QSize QComboBox::iconSize();
impl<'a> /*trait*/ QComboBox_iconSize<QSize> for () {
  fn iconSize(self , rsthis: & QComboBox) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8iconSizeEv()};
    let mut ret = unsafe {_ZNK9QComboBox8iconSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QComboBox::rootModelIndex();
impl /*struct*/ QComboBox {
  pub fn rootModelIndex<RetType, T: QComboBox_rootModelIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rootModelIndex(self);
    // return 1;
  }
}

pub trait QComboBox_rootModelIndex<RetType> {
  fn rootModelIndex(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QModelIndex QComboBox::rootModelIndex();
impl<'a> /*trait*/ QComboBox_rootModelIndex<QModelIndex> for () {
  fn rootModelIndex(self , rsthis: & QComboBox) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox14rootModelIndexEv()};
    let mut ret = unsafe {_ZNK9QComboBox14rootModelIndexEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QComboBox::setEditable(bool editable);
impl /*struct*/ QComboBox {
  pub fn setEditable<RetType, T: QComboBox_setEditable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEditable(self);
    // return 1;
  }
}

pub trait QComboBox_setEditable<RetType> {
  fn setEditable(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setEditable(bool editable);
impl<'a> /*trait*/ QComboBox_setEditable<()> for (i8) {
  fn setEditable(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setEditableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QComboBox11setEditableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::setItemIcon(int index, const QIcon & icon);
impl /*struct*/ QComboBox {
  pub fn setItemIcon<RetType, T: QComboBox_setItemIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemIcon(self);
    // return 1;
  }
}

pub trait QComboBox_setItemIcon<RetType> {
  fn setItemIcon(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setItemIcon(int index, const QIcon & icon);
impl<'a> /*trait*/ QComboBox_setItemIcon<()> for (i32, &'a QIcon) {
  fn setItemIcon(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setItemIconEiRK5QIcon()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox11setItemIconEiRK5QIcon(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QComboBox::currentTextChanged(const QString & );
impl /*struct*/ QComboBox {
  pub fn currentTextChanged<RetType, T: QComboBox_currentTextChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentTextChanged(self);
    // return 1;
  }
}

pub trait QComboBox_currentTextChanged<RetType> {
  fn currentTextChanged(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::currentTextChanged(const QString & );
impl<'a> /*trait*/ QComboBox_currentTextChanged<()> for (&'a QString) {
  fn currentTextChanged(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox18currentTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox18currentTextChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QComboBox::autoCompletion();
impl /*struct*/ QComboBox {
  pub fn autoCompletion<RetType, T: QComboBox_autoCompletion<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoCompletion(self);
    // return 1;
  }
}

pub trait QComboBox_autoCompletion<RetType> {
  fn autoCompletion(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  bool QComboBox::autoCompletion();
impl<'a> /*trait*/ QComboBox_autoCompletion<i8> for () {
  fn autoCompletion(self , rsthis: & QComboBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox14autoCompletionEv()};
    let mut ret = unsafe {_ZNK9QComboBox14autoCompletionEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QVariant QComboBox::currentData(int role);
impl /*struct*/ QComboBox {
  pub fn currentData<RetType, T: QComboBox_currentData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentData(self);
    // return 1;
  }
}

pub trait QComboBox_currentData<RetType> {
  fn currentData(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QVariant QComboBox::currentData(int role);
impl<'a> /*trait*/ QComboBox_currentData<QVariant> for (i32) {
  fn currentData(self , rsthis: & QComboBox) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox11currentDataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QComboBox11currentDataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QComboBox::hasFrame();
impl /*struct*/ QComboBox {
  pub fn hasFrame<RetType, T: QComboBox_hasFrame<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasFrame(self);
    // return 1;
  }
}

pub trait QComboBox_hasFrame<RetType> {
  fn hasFrame(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  bool QComboBox::hasFrame();
impl<'a> /*trait*/ QComboBox_hasFrame<i8> for () {
  fn hasFrame(self , rsthis: & QComboBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8hasFrameEv()};
    let mut ret = unsafe {_ZNK9QComboBox8hasFrameEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QValidator * QComboBox::validator();
impl /*struct*/ QComboBox {
  pub fn validator<RetType, T: QComboBox_validator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.validator(self);
    // return 1;
  }
}

pub trait QComboBox_validator<RetType> {
  fn validator(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  const QValidator * QComboBox::validator();
impl<'a> /*trait*/ QComboBox_validator<QValidator> for () {
  fn validator(self , rsthis: & QComboBox) -> QValidator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox9validatorEv()};
    let mut ret = unsafe {_ZNK9QComboBox9validatorEv(rsthis.qclsinst)};
    let mut ret1 = QValidator::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QComboBox::itemText(int index);
impl /*struct*/ QComboBox {
  pub fn itemText<RetType, T: QComboBox_itemText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemText(self);
    // return 1;
  }
}

pub trait QComboBox_itemText<RetType> {
  fn itemText(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QString QComboBox::itemText(int index);
impl<'a> /*trait*/ QComboBox_itemText<QString> for (i32) {
  fn itemText(self , rsthis: & QComboBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8itemTextEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QComboBox8itemTextEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QComboBox::setItemData(int index, const QVariant & value, int role);
impl /*struct*/ QComboBox {
  pub fn setItemData<RetType, T: QComboBox_setItemData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemData(self);
    // return 1;
  }
}

pub trait QComboBox_setItemData<RetType> {
  fn setItemData(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setItemData(int index, const QVariant & value, int role);
impl<'a> /*trait*/ QComboBox_setItemData<()> for (i32, &'a QVariant, i32) {
  fn setItemData(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setItemDataEiRK8QVarianti()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZN9QComboBox11setItemDataEiRK8QVarianti(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QComboBox::highlighted(int index);
impl /*struct*/ QComboBox {
  pub fn highlighted<RetType, T: QComboBox_highlighted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.highlighted(self);
    // return 1;
  }
}

pub trait QComboBox_highlighted<RetType> {
  fn highlighted(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::highlighted(int index);
impl<'a> /*trait*/ QComboBox_highlighted<()> for (i32) {
  fn highlighted(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11highlightedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QComboBox11highlightedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::hidePopup();
impl /*struct*/ QComboBox {
  pub fn hidePopup<RetType, T: QComboBox_hidePopup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hidePopup(self);
    // return 1;
  }
}

pub trait QComboBox_hidePopup<RetType> {
  fn hidePopup(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::hidePopup();
impl<'a> /*trait*/ QComboBox_hidePopup<()> for () {
  fn hidePopup(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox9hidePopupEv()};
     unsafe {_ZN9QComboBox9hidePopupEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QComboBox::insertItem(int index, const QIcon & icon, const QString & text, const QVariant & userData);
impl /*struct*/ QComboBox {
  pub fn insertItem<RetType, T: QComboBox_insertItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertItem(self);
    // return 1;
  }
}

pub trait QComboBox_insertItem<RetType> {
  fn insertItem(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::insertItem(int index, const QIcon & icon, const QString & text, const QVariant & userData);
impl<'a> /*trait*/ QComboBox_insertItem<()> for (i32, &'a QIcon, &'a QString, &'a QVariant) {
  fn insertItem(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox10insertItemEiRK5QIconRK7QStringRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox10insertItemEiRK5QIconRK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QComboBox::setCurrentText(const QString & text);
impl /*struct*/ QComboBox {
  pub fn setCurrentText<RetType, T: QComboBox_setCurrentText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentText(self);
    // return 1;
  }
}

pub trait QComboBox_setCurrentText<RetType> {
  fn setCurrentText(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setCurrentText(const QString & text);
impl<'a> /*trait*/ QComboBox_setCurrentText<()> for (&'a QString) {
  fn setCurrentText(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox14setCurrentTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox14setCurrentTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::highlighted(const QString & );
impl<'a> /*trait*/ QComboBox_highlighted<()> for (&'a QString) {
  fn highlighted(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11highlightedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox11highlightedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::editTextChanged(const QString & );
impl /*struct*/ QComboBox {
  pub fn editTextChanged<RetType, T: QComboBox_editTextChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.editTextChanged(self);
    // return 1;
  }
}

pub trait QComboBox_editTextChanged<RetType> {
  fn editTextChanged(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::editTextChanged(const QString & );
impl<'a> /*trait*/ QComboBox_editTextChanged<()> for (&'a QString) {
  fn editTextChanged(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox15editTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox15editTextChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QComboBox::modelColumn();
impl /*struct*/ QComboBox {
  pub fn modelColumn<RetType, T: QComboBox_modelColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.modelColumn(self);
    // return 1;
  }
}

pub trait QComboBox_modelColumn<RetType> {
  fn modelColumn(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  int QComboBox::modelColumn();
impl<'a> /*trait*/ QComboBox_modelColumn<i32> for () {
  fn modelColumn(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox11modelColumnEv()};
    let mut ret = unsafe {_ZNK9QComboBox11modelColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSize QComboBox::sizeHint();
impl /*struct*/ QComboBox {
  pub fn sizeHint<RetType, T: QComboBox_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QComboBox_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QSize QComboBox::sizeHint();
impl<'a> /*trait*/ QComboBox_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QComboBox) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8sizeHintEv()};
    let mut ret = unsafe {_ZNK9QComboBox8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QComboBox::itemData(int index, int role);
impl /*struct*/ QComboBox {
  pub fn itemData<RetType, T: QComboBox_itemData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemData(self);
    // return 1;
  }
}

pub trait QComboBox_itemData<RetType> {
  fn itemData(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QVariant QComboBox::itemData(int index, int role);
impl<'a> /*trait*/ QComboBox_itemData<QVariant> for (i32, i32) {
  fn itemData(self , rsthis: & QComboBox) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8itemDataEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK9QComboBox8itemDataEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QComboBox::activated(int index);
impl /*struct*/ QComboBox {
  pub fn activated<RetType, T: QComboBox_activated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activated(self);
    // return 1;
  }
}

pub trait QComboBox_activated<RetType> {
  fn activated(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::activated(int index);
impl<'a> /*trait*/ QComboBox_activated<()> for (i32) {
  fn activated(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox9activatedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QComboBox9activatedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::setCompleter(QCompleter * c);
impl /*struct*/ QComboBox {
  pub fn setCompleter<RetType, T: QComboBox_setCompleter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCompleter(self);
    // return 1;
  }
}

pub trait QComboBox_setCompleter<RetType> {
  fn setCompleter(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setCompleter(QCompleter * c);
impl<'a> /*trait*/ QComboBox_setCompleter<()> for (&'a QCompleter) {
  fn setCompleter(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox12setCompleterEP10QCompleter()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox12setCompleterEP10QCompleter(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::activated(const QString & );
impl<'a> /*trait*/ QComboBox_activated<()> for (&'a QString) {
  fn activated(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox9activatedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox9activatedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QComboBox::maxVisibleItems();
impl /*struct*/ QComboBox {
  pub fn maxVisibleItems<RetType, T: QComboBox_maxVisibleItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maxVisibleItems(self);
    // return 1;
  }
}

pub trait QComboBox_maxVisibleItems<RetType> {
  fn maxVisibleItems(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  int QComboBox::maxVisibleItems();
impl<'a> /*trait*/ QComboBox_maxVisibleItems<i32> for () {
  fn maxVisibleItems(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox15maxVisibleItemsEv()};
    let mut ret = unsafe {_ZNK9QComboBox15maxVisibleItemsEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QComboBox::QComboBox(QWidget * parent);
impl /*struct*/ QComboBox {
  pub fn New<T: QComboBox_New>(value: T) -> QComboBox {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QComboBox_New {
  fn New(self) -> QComboBox;
}

  // proto:  void QComboBox::QComboBox(QWidget * parent);
impl<'a> /*trait*/ QComboBox_New for (&'a QWidget) {
  fn New(self) -> QComboBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBoxC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QComboBoxC1EP7QWidget(qthis, arg0)};
    let rsthis = QComboBox{/**/qbase: QWidget::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QComboBox::currentIndexChanged(const QString & );
impl /*struct*/ QComboBox {
  pub fn currentIndexChanged<RetType, T: QComboBox_currentIndexChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentIndexChanged(self);
    // return 1;
  }
}

pub trait QComboBox_currentIndexChanged<RetType> {
  fn currentIndexChanged(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::currentIndexChanged(const QString & );
impl<'a> /*trait*/ QComboBox_currentIndexChanged<()> for (&'a QString) {
  fn currentIndexChanged(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox19currentIndexChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox19currentIndexChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::setCurrentIndex(int index);
impl /*struct*/ QComboBox {
  pub fn setCurrentIndex<RetType, T: QComboBox_setCurrentIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex(self);
    // return 1;
  }
}

pub trait QComboBox_setCurrentIndex<RetType> {
  fn setCurrentIndex(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setCurrentIndex(int index);
impl<'a> /*trait*/ QComboBox_setCurrentIndex<()> for (i32) {
  fn setCurrentIndex(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox15setCurrentIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QComboBox15setCurrentIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::QComboBox(const QComboBox & );
impl<'a> /*trait*/ QComboBox_New for (&'a QComboBox) {
  fn New(self) -> QComboBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QComboBoxC1ERKS_(qthis, arg0)};
    let rsthis = QComboBox{/**/qbase: QWidget::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QComboBox::setRootModelIndex(const QModelIndex & index);
impl /*struct*/ QComboBox {
  pub fn setRootModelIndex<RetType, T: QComboBox_setRootModelIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRootModelIndex(self);
    // return 1;
  }
}

pub trait QComboBox_setRootModelIndex<RetType> {
  fn setRootModelIndex(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setRootModelIndex(const QModelIndex & index);
impl<'a> /*trait*/ QComboBox_setRootModelIndex<()> for (&'a QModelIndex) {
  fn setRootModelIndex(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox17setRootModelIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox17setRootModelIndexERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::setEditText(const QString & text);
impl /*struct*/ QComboBox {
  pub fn setEditText<RetType, T: QComboBox_setEditText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEditText(self);
    // return 1;
  }
}

pub trait QComboBox_setEditText<RetType> {
  fn setEditText(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setEditText(const QString & text);
impl<'a> /*trait*/ QComboBox_setEditText<()> for (&'a QString) {
  fn setEditText(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setEditTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox11setEditTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::addItem(const QString & text, const QVariant & userData);
impl<'a> /*trait*/ QComboBox_addItem<()> for (&'a QString, &'a QVariant) {
  fn addItem(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox7addItemERK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox7addItemERK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QCompleter * QComboBox::completer();
impl /*struct*/ QComboBox {
  pub fn completer<RetType, T: QComboBox_completer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.completer(self);
    // return 1;
  }
}

pub trait QComboBox_completer<RetType> {
  fn completer(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QCompleter * QComboBox::completer();
impl<'a> /*trait*/ QComboBox_completer<QCompleter> for () {
  fn completer(self , rsthis: & QComboBox) -> QCompleter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox9completerEv()};
    let mut ret = unsafe {_ZNK9QComboBox9completerEv(rsthis.qclsinst)};
    let mut ret1 = QCompleter::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QComboBox::removeItem(int index);
impl /*struct*/ QComboBox {
  pub fn removeItem<RetType, T: QComboBox_removeItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeItem(self);
    // return 1;
  }
}

pub trait QComboBox_removeItem<RetType> {
  fn removeItem(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::removeItem(int index);
impl<'a> /*trait*/ QComboBox_removeItem<()> for (i32) {
  fn removeItem(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox10removeItemEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QComboBox10removeItemEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QComboBox::count();
impl /*struct*/ QComboBox {
  pub fn count<RetType, T: QComboBox_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QComboBox_count<RetType> {
  fn count(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  int QComboBox::count();
impl<'a> /*trait*/ QComboBox_count<i32> for () {
  fn count(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox5countEv()};
    let mut ret = unsafe {_ZNK9QComboBox5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QComboBox::addItems(const QStringList & texts);
impl /*struct*/ QComboBox {
  pub fn addItems<RetType, T: QComboBox_addItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addItems(self);
    // return 1;
  }
}

pub trait QComboBox_addItems<RetType> {
  fn addItems(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::addItems(const QStringList & texts);
impl<'a> /*trait*/ QComboBox_addItems<()> for (&'a QStringList) {
  fn addItems(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox8addItemsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox8addItemsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::setMinimumContentsLength(int characters);
impl /*struct*/ QComboBox {
  pub fn setMinimumContentsLength<RetType, T: QComboBox_setMinimumContentsLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimumContentsLength(self);
    // return 1;
  }
}

pub trait QComboBox_setMinimumContentsLength<RetType> {
  fn setMinimumContentsLength(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setMinimumContentsLength(int characters);
impl<'a> /*trait*/ QComboBox_setMinimumContentsLength<()> for (i32) {
  fn setMinimumContentsLength(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox24setMinimumContentsLengthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QComboBox24setMinimumContentsLengthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QComboBox::duplicatesEnabled();
impl /*struct*/ QComboBox {
  pub fn duplicatesEnabled<RetType, T: QComboBox_duplicatesEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.duplicatesEnabled(self);
    // return 1;
  }
}

pub trait QComboBox_duplicatesEnabled<RetType> {
  fn duplicatesEnabled(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  bool QComboBox::duplicatesEnabled();
impl<'a> /*trait*/ QComboBox_duplicatesEnabled<i8> for () {
  fn duplicatesEnabled(self , rsthis: & QComboBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox17duplicatesEnabledEv()};
    let mut ret = unsafe {_ZNK9QComboBox17duplicatesEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QComboBox::~QComboBox();
impl /*struct*/ QComboBox {
  pub fn Free<RetType, T: QComboBox_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QComboBox_Free<RetType> {
  fn Free(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::~QComboBox();
impl<'a> /*trait*/ QComboBox_Free<()> for () {
  fn Free(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBoxD0Ev()};
     unsafe {_ZN9QComboBoxD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QAbstractItemModel * QComboBox::model();
impl /*struct*/ QComboBox {
  pub fn model<RetType, T: QComboBox_model<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.model(self);
    // return 1;
  }
}

pub trait QComboBox_model<RetType> {
  fn model(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QAbstractItemModel * QComboBox::model();
impl<'a> /*trait*/ QComboBox_model<()> for () {
  fn model(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox5modelEv()};
     unsafe {_ZNK9QComboBox5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QComboBox::minimumContentsLength();
impl /*struct*/ QComboBox {
  pub fn minimumContentsLength<RetType, T: QComboBox_minimumContentsLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumContentsLength(self);
    // return 1;
  }
}

pub trait QComboBox_minimumContentsLength<RetType> {
  fn minimumContentsLength(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  int QComboBox::minimumContentsLength();
impl<'a> /*trait*/ QComboBox_minimumContentsLength<i32> for () {
  fn minimumContentsLength(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox21minimumContentsLengthEv()};
    let mut ret = unsafe {_ZNK9QComboBox21minimumContentsLengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QComboBox::isEditable();
impl /*struct*/ QComboBox {
  pub fn isEditable<RetType, T: QComboBox_isEditable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEditable(self);
    // return 1;
  }
}

pub trait QComboBox_isEditable<RetType> {
  fn isEditable(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  bool QComboBox::isEditable();
impl<'a> /*trait*/ QComboBox_isEditable<i8> for () {
  fn isEditable(self , rsthis: & QComboBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox10isEditableEv()};
    let mut ret = unsafe {_ZNK9QComboBox10isEditableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QComboBox::setMaxCount(int max);
impl /*struct*/ QComboBox {
  pub fn setMaxCount<RetType, T: QComboBox_setMaxCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaxCount(self);
    // return 1;
  }
}

pub trait QComboBox_setMaxCount<RetType> {
  fn setMaxCount(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setMaxCount(int max);
impl<'a> /*trait*/ QComboBox_setMaxCount<()> for (i32) {
  fn setMaxCount(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setMaxCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QComboBox11setMaxCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QComboBox::currentIndex();
impl /*struct*/ QComboBox {
  pub fn currentIndex<RetType, T: QComboBox_currentIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentIndex(self);
    // return 1;
  }
}

pub trait QComboBox_currentIndex<RetType> {
  fn currentIndex(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  int QComboBox::currentIndex();
impl<'a> /*trait*/ QComboBox_currentIndex<i32> for () {
  fn currentIndex(self , rsthis: & QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox12currentIndexEv()};
    let mut ret = unsafe {_ZNK9QComboBox12currentIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QComboBox::setDuplicatesEnabled(bool enable);
impl /*struct*/ QComboBox {
  pub fn setDuplicatesEnabled<RetType, T: QComboBox_setDuplicatesEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDuplicatesEnabled(self);
    // return 1;
  }
}

pub trait QComboBox_setDuplicatesEnabled<RetType> {
  fn setDuplicatesEnabled(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setDuplicatesEnabled(bool enable);
impl<'a> /*trait*/ QComboBox_setDuplicatesEnabled<()> for (i8) {
  fn setDuplicatesEnabled(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox20setDuplicatesEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QComboBox20setDuplicatesEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QComboBox::currentText();
impl /*struct*/ QComboBox {
  pub fn currentText<RetType, T: QComboBox_currentText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentText(self);
    // return 1;
  }
}

pub trait QComboBox_currentText<RetType> {
  fn currentText(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QString QComboBox::currentText();
impl<'a> /*trait*/ QComboBox_currentText<QString> for () {
  fn currentText(self , rsthis: & QComboBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox11currentTextEv()};
    let mut ret = unsafe {_ZNK9QComboBox11currentTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QComboBox::showPopup();
impl /*struct*/ QComboBox {
  pub fn showPopup<RetType, T: QComboBox_showPopup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showPopup(self);
    // return 1;
  }
}

pub trait QComboBox_showPopup<RetType> {
  fn showPopup(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::showPopup();
impl<'a> /*trait*/ QComboBox_showPopup<()> for () {
  fn showPopup(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox9showPopupEv()};
     unsafe {_ZN9QComboBox9showPopupEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QLineEdit * QComboBox::lineEdit();
impl /*struct*/ QComboBox {
  pub fn lineEdit<RetType, T: QComboBox_lineEdit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineEdit(self);
    // return 1;
  }
}

pub trait QComboBox_lineEdit<RetType> {
  fn lineEdit(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QLineEdit * QComboBox::lineEdit();
impl<'a> /*trait*/ QComboBox_lineEdit<QLineEdit> for () {
  fn lineEdit(self , rsthis: & QComboBox) -> QLineEdit {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8lineEditEv()};
    let mut ret = unsafe {_ZNK9QComboBox8lineEditEv(rsthis.qclsinst)};
    let mut ret1 = QLineEdit::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QAbstractItemDelegate * QComboBox::itemDelegate();
impl /*struct*/ QComboBox {
  pub fn itemDelegate<RetType, T: QComboBox_itemDelegate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemDelegate(self);
    // return 1;
  }
}

pub trait QComboBox_itemDelegate<RetType> {
  fn itemDelegate(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QAbstractItemDelegate * QComboBox::itemDelegate();
impl<'a> /*trait*/ QComboBox_itemDelegate<()> for () {
  fn itemDelegate(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox12itemDelegateEv()};
     unsafe {_ZNK9QComboBox12itemDelegateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QComboBox::currentIndexChanged(int index);
impl<'a> /*trait*/ QComboBox_currentIndexChanged<()> for (i32) {
  fn currentIndexChanged(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox19currentIndexChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QComboBox19currentIndexChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::setMaxVisibleItems(int maxItems);
impl /*struct*/ QComboBox {
  pub fn setMaxVisibleItems<RetType, T: QComboBox_setMaxVisibleItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaxVisibleItems(self);
    // return 1;
  }
}

pub trait QComboBox_setMaxVisibleItems<RetType> {
  fn setMaxVisibleItems(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setMaxVisibleItems(int maxItems);
impl<'a> /*trait*/ QComboBox_setMaxVisibleItems<()> for (i32) {
  fn setMaxVisibleItems(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox18setMaxVisibleItemsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QComboBox18setMaxVisibleItemsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QComboBox::event(QEvent * event);
impl /*struct*/ QComboBox {
  pub fn event<RetType, T: QComboBox_event<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.event(self);
    // return 1;
  }
}

pub trait QComboBox_event<RetType> {
  fn event(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  bool QComboBox::event(QEvent * event);
impl<'a> /*trait*/ QComboBox_event<i8> for (&'a QEvent) {
  fn event(self , rsthis: & QComboBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QComboBox5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QComboBox::setModelColumn(int visibleColumn);
impl /*struct*/ QComboBox {
  pub fn setModelColumn<RetType, T: QComboBox_setModelColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setModelColumn(self);
    // return 1;
  }
}

pub trait QComboBox_setModelColumn<RetType> {
  fn setModelColumn(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setModelColumn(int visibleColumn);
impl<'a> /*trait*/ QComboBox_setModelColumn<()> for (i32) {
  fn setModelColumn(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox14setModelColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QComboBox14setModelColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::setItemText(int index, const QString & text);
impl /*struct*/ QComboBox {
  pub fn setItemText<RetType, T: QComboBox_setItemText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemText(self);
    // return 1;
  }
}

pub trait QComboBox_setItemText<RetType> {
  fn setItemText(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setItemText(int index, const QString & text);
impl<'a> /*trait*/ QComboBox_setItemText<()> for (i32, &'a QString) {
  fn setItemText(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setItemTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox11setItemTextEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QComboBox::setLineEdit(QLineEdit * edit);
impl /*struct*/ QComboBox {
  pub fn setLineEdit<RetType, T: QComboBox_setLineEdit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLineEdit(self);
    // return 1;
  }
}

pub trait QComboBox_setLineEdit<RetType> {
  fn setLineEdit(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setLineEdit(QLineEdit * edit);
impl<'a> /*trait*/ QComboBox_setLineEdit<()> for (&'a QLineEdit) {
  fn setLineEdit(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setLineEditEP9QLineEdit()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox11setLineEditEP9QLineEdit(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QIcon QComboBox::itemIcon(int index);
impl /*struct*/ QComboBox {
  pub fn itemIcon<RetType, T: QComboBox_itemIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemIcon(self);
    // return 1;
  }
}

pub trait QComboBox_itemIcon<RetType> {
  fn itemIcon(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  QIcon QComboBox::itemIcon(int index);
impl<'a> /*trait*/ QComboBox_itemIcon<QIcon> for (i32) {
  fn itemIcon(self , rsthis: & QComboBox) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8itemIconEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QComboBox8itemIconEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QIcon::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QComboBox::insertItem(int index, const QString & text, const QVariant & userData);
impl<'a> /*trait*/ QComboBox_insertItem<()> for (i32, &'a QString, &'a QVariant) {
  fn insertItem(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox10insertItemEiRK7QStringRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox10insertItemEiRK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QComboBox::setValidator(const QValidator * v);
impl /*struct*/ QComboBox {
  pub fn setValidator<RetType, T: QComboBox_setValidator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setValidator(self);
    // return 1;
  }
}

pub trait QComboBox_setValidator<RetType> {
  fn setValidator(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::setValidator(const QValidator * v);
impl<'a> /*trait*/ QComboBox_setValidator<()> for (&'a QValidator) {
  fn setValidator(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox12setValidatorEPK10QValidator()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QComboBox12setValidatorEPK10QValidator(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QComboBox::insertSeparator(int index);
impl /*struct*/ QComboBox {
  pub fn insertSeparator<RetType, T: QComboBox_insertSeparator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertSeparator(self);
    // return 1;
  }
}

pub trait QComboBox_insertSeparator<RetType> {
  fn insertSeparator(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  void QComboBox::insertSeparator(int index);
impl<'a> /*trait*/ QComboBox_insertSeparator<()> for (i32) {
  fn insertSeparator(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox15insertSeparatorEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QComboBox15insertSeparatorEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QComboBox::metaObject();
impl /*struct*/ QComboBox {
  pub fn metaObject<RetType, T: QComboBox_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QComboBox_metaObject<RetType> {
  fn metaObject(self , rsthis: & QComboBox) -> RetType;
}

  // proto:  const QMetaObject * QComboBox::metaObject();
impl<'a> /*trait*/ QComboBox_metaObject<()> for () {
  fn metaObject(self , rsthis: & QComboBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox10metaObjectEv()};
     unsafe {_ZNK9QComboBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

