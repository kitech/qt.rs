// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsize::QSize;
use super::qicon::QIcon;
use super::qstring::QString;
use super::qvariant::QVariant;
use super::qstringlist::QStringList;
use super::qcompleter::QCompleter;
use super::qwidget::QWidget;
use super::qmodelindex::QModelIndex;
use super::qevent::QEvent;
use super::qlineedit::QLineEdit;
use super::qvalidator::QValidator;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QComboBox::clearEditText();
  fn _ZN9QComboBox13clearEditTextEv() -> i32;
  // proto: void QComboBox::setAutoCompletion(bool enable);
  fn _ZN9QComboBox17setAutoCompletionEb(arg0: int8_t) -> i32;
  // proto: void QComboBox::setFrame(bool );
  fn _ZN9QComboBox8setFrameEb(arg0: int8_t) -> i32;
  // proto: void QComboBox::setIconSize(const QSize & size);
  fn _ZN9QComboBox11setIconSizeERK5QSize(arg0: *const c_void) -> i32;
  // proto: QAbstractItemView * QComboBox::view();
  fn _ZNK9QComboBox4viewEv() -> i32;
  // proto: QSize QComboBox::minimumSizeHint();
  fn _ZNK9QComboBox15minimumSizeHintEv() -> i32;
  // proto: void QComboBox::clear();
  fn _ZN9QComboBox5clearEv() -> i32;
  // proto: int QComboBox::maxCount();
  fn _ZNK9QComboBox8maxCountEv() -> i32;
  // proto: void QComboBox::addItem(const QIcon & icon, const QString & text, const QVariant & userData);
  fn _ZN9QComboBox7addItemERK5QIconRK7QStringRK8QVariant(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: void QComboBox::insertItems(int index, const QStringList & texts);
  fn _ZN9QComboBox11insertItemsEiRK11QStringList(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: QSize QComboBox::iconSize();
  fn _ZNK9QComboBox8iconSizeEv() -> i32;
  // proto: QModelIndex QComboBox::rootModelIndex();
  fn _ZNK9QComboBox14rootModelIndexEv() -> i32;
  // proto: void QComboBox::setEditable(bool editable);
  fn _ZN9QComboBox11setEditableEb(arg0: int8_t) -> i32;
  // proto: void QComboBox::setItemIcon(int index, const QIcon & icon);
  fn _ZN9QComboBox11setItemIconEiRK5QIcon(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QComboBox::currentTextChanged(const QString & );
  fn _ZN9QComboBox18currentTextChangedERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QComboBox::autoCompletion();
  fn _ZNK9QComboBox14autoCompletionEv() -> i32;
  // proto: QVariant QComboBox::currentData(int role);
  fn _ZNK9QComboBox11currentDataEi(arg0: c_int) -> i32;
  // proto: bool QComboBox::hasFrame();
  fn _ZNK9QComboBox8hasFrameEv() -> i32;
  // proto: const QValidator * QComboBox::validator();
  fn _ZNK9QComboBox9validatorEv() -> i32;
  // proto: QString QComboBox::itemText(int index);
  fn _ZNK9QComboBox8itemTextEi(arg0: c_int) -> i32;
  // proto: void QComboBox::setItemData(int index, const QVariant & value, int role);
  fn _ZN9QComboBox11setItemDataEiRK8QVarianti(arg0: c_int, arg1: *const c_void, arg2: c_int) -> i32;
  // proto: void QComboBox::highlighted(int index);
  fn _ZN9QComboBox11highlightedEi(arg0: c_int) -> i32;
  // proto: void QComboBox::hidePopup();
  fn _ZN9QComboBox9hidePopupEv() -> i32;
  // proto: void QComboBox::insertItem(int index, const QIcon & icon, const QString & text, const QVariant & userData);
  fn _ZN9QComboBox10insertItemEiRK5QIconRK7QStringRK8QVariant(arg0: c_int, arg1: *const c_void, arg2: *const c_void, arg3: *const c_void) -> i32;
  // proto: void QComboBox::setCurrentText(const QString & text);
  fn _ZN9QComboBox14setCurrentTextERK7QString(arg0: *const c_void) -> i32;
  // proto: void QComboBox::highlighted(const QString & );
  fn _ZN9QComboBox11highlightedERK7QString(arg0: *const c_void) -> i32;
  // proto: void QComboBox::editTextChanged(const QString & );
  fn _ZN9QComboBox15editTextChangedERK7QString(arg0: *const c_void) -> i32;
  // proto: int QComboBox::modelColumn();
  fn _ZNK9QComboBox11modelColumnEv() -> i32;
  // proto: QSize QComboBox::sizeHint();
  fn _ZNK9QComboBox8sizeHintEv() -> i32;
  // proto: QVariant QComboBox::itemData(int index, int role);
  fn _ZNK9QComboBox8itemDataEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QComboBox::activated(int index);
  fn _ZN9QComboBox9activatedEi(arg0: c_int) -> i32;
  // proto: void QComboBox::setCompleter(QCompleter * c);
  fn _ZN9QComboBox12setCompleterEP10QCompleter(arg0: *mut c_void) -> i32;
  // proto: void QComboBox::activated(const QString & );
  fn _ZN9QComboBox9activatedERK7QString(arg0: *const c_void) -> i32;
  // proto: int QComboBox::maxVisibleItems();
  fn _ZNK9QComboBox15maxVisibleItemsEv() -> i32;
  // proto: void QComboBox::NewQComboBox(QWidget * parent);
  fn _ZN9QComboBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QComboBox::currentIndexChanged(const QString & );
  fn _ZN9QComboBox19currentIndexChangedERK7QString(arg0: *const c_void) -> i32;
  // proto: void QComboBox::setCurrentIndex(int index);
  fn _ZN9QComboBox15setCurrentIndexEi(arg0: c_int) -> i32;
  // proto: void QComboBox::NewQComboBox(const QComboBox & );
  fn _ZN9QComboBoxC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QComboBox::setRootModelIndex(const QModelIndex & index);
  fn _ZN9QComboBox17setRootModelIndexERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: void QComboBox::setEditText(const QString & text);
  fn _ZN9QComboBox11setEditTextERK7QString(arg0: *const c_void) -> i32;
  // proto: void QComboBox::addItem(const QString & text, const QVariant & userData);
  fn _ZN9QComboBox7addItemERK7QStringRK8QVariant(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QCompleter * QComboBox::completer();
  fn _ZNK9QComboBox9completerEv() -> i32;
  // proto: void QComboBox::removeItem(int index);
  fn _ZN9QComboBox10removeItemEi(arg0: c_int) -> i32;
  // proto: int QComboBox::count();
  fn _ZNK9QComboBox5countEv() -> i32;
  // proto: void QComboBox::addItems(const QStringList & texts);
  fn _ZN9QComboBox8addItemsERK11QStringList(arg0: *const c_void) -> i32;
  // proto: void QComboBox::setMinimumContentsLength(int characters);
  fn _ZN9QComboBox24setMinimumContentsLengthEi(arg0: c_int) -> i32;
  // proto: bool QComboBox::duplicatesEnabled();
  fn _ZNK9QComboBox17duplicatesEnabledEv() -> i32;
  // proto: void QComboBox::FreeQComboBox();
  fn _ZN9QComboBoxD0Ev() -> i32;
  // proto: QAbstractItemModel * QComboBox::model();
  fn _ZNK9QComboBox5modelEv() -> i32;
  // proto: int QComboBox::minimumContentsLength();
  fn _ZNK9QComboBox21minimumContentsLengthEv() -> i32;
  // proto: bool QComboBox::isEditable();
  fn _ZNK9QComboBox10isEditableEv() -> i32;
  // proto: void QComboBox::setMaxCount(int max);
  fn _ZN9QComboBox11setMaxCountEi(arg0: c_int) -> i32;
  // proto: int QComboBox::currentIndex();
  fn _ZNK9QComboBox12currentIndexEv() -> i32;
  // proto: void QComboBox::setDuplicatesEnabled(bool enable);
  fn _ZN9QComboBox20setDuplicatesEnabledEb(arg0: int8_t) -> i32;
  // proto: QString QComboBox::currentText();
  fn _ZNK9QComboBox11currentTextEv() -> i32;
  // proto: void QComboBox::showPopup();
  fn _ZN9QComboBox9showPopupEv() -> i32;
  // proto: QLineEdit * QComboBox::lineEdit();
  fn _ZNK9QComboBox8lineEditEv() -> i32;
  // proto: QAbstractItemDelegate * QComboBox::itemDelegate();
  fn _ZNK9QComboBox12itemDelegateEv() -> i32;
  // proto: void QComboBox::currentIndexChanged(int index);
  fn _ZN9QComboBox19currentIndexChangedEi(arg0: c_int) -> i32;
  // proto: void QComboBox::setMaxVisibleItems(int maxItems);
  fn _ZN9QComboBox18setMaxVisibleItemsEi(arg0: c_int) -> i32;
  // proto: bool QComboBox::event(QEvent * event);
  fn _ZN9QComboBox5eventEP6QEvent(arg0: *mut c_void) -> i32;
  // proto: void QComboBox::setModelColumn(int visibleColumn);
  fn _ZN9QComboBox14setModelColumnEi(arg0: c_int) -> i32;
  // proto: void QComboBox::setItemText(int index, const QString & text);
  fn _ZN9QComboBox11setItemTextEiRK7QString(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QComboBox::setLineEdit(QLineEdit * edit);
  fn _ZN9QComboBox11setLineEditEP9QLineEdit(arg0: *mut c_void) -> i32;
  // proto: QIcon QComboBox::itemIcon(int index);
  fn _ZNK9QComboBox8itemIconEi(arg0: c_int) -> i32;
  // proto: void QComboBox::insertItem(int index, const QString & text, const QVariant & userData);
  fn _ZN9QComboBox10insertItemEiRK7QStringRK8QVariant(arg0: c_int, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: void QComboBox::setValidator(const QValidator * v);
  fn _ZN9QComboBox12setValidatorEPK10QValidator(arg0: *const c_void) -> i32;
  // proto: void QComboBox::insertSeparator(int index);
  fn _ZN9QComboBox15insertSeparatorEi(arg0: c_int) -> i32;
  // proto: const QMetaObject * QComboBox::metaObject();
  fn _ZNK9QComboBox10metaObjectEv() -> i32;
}

// body block begin
// class sizeof(QComboBox)=1
pub struct QComboBox {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QComboBox {
  pub fn clearEditText<T: QComboBox_clearEditText>(&mut self, value: T) -> i32 {
    value.clearEditText(self);
    return 1;
  }
}

pub trait QComboBox_clearEditText {
  fn clearEditText(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::clearEditText();
impl<'a> /*trait*/ QComboBox_clearEditText for () {
  fn clearEditText(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox13clearEditTextEv()};
    unsafe {_ZN9QComboBox13clearEditTextEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn setAutoCompletion<T: QComboBox_setAutoCompletion>(&mut self, value: T) -> i32 {
    value.setAutoCompletion(self);
    return 1;
  }
}

pub trait QComboBox_setAutoCompletion {
  fn setAutoCompletion(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::setAutoCompletion(bool enable);
impl<'a> /*trait*/ QComboBox_setAutoCompletion for (i8) {
  fn setAutoCompletion(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox17setAutoCompletionEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QComboBox17setAutoCompletionEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn setFrame<T: QComboBox_setFrame>(&mut self, value: T) -> i32 {
    value.setFrame(self);
    return 1;
  }
}

pub trait QComboBox_setFrame {
  fn setFrame(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::setFrame(bool );
impl<'a> /*trait*/ QComboBox_setFrame for (i8) {
  fn setFrame(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox8setFrameEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QComboBox8setFrameEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn setIconSize<T: QComboBox_setIconSize>(&mut self, value: T) -> i32 {
    value.setIconSize(self);
    return 1;
  }
}

pub trait QComboBox_setIconSize {
  fn setIconSize(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::setIconSize(const QSize & size);
impl<'a> /*trait*/ QComboBox_setIconSize for (&'a  QSize) {
  fn setIconSize(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QComboBox11setIconSizeERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn view<T: QComboBox_view>(&mut self, value: T) -> i32 {
    value.view(self);
    return 1;
  }
}

pub trait QComboBox_view {
  fn view(self, this: &mut QComboBox) -> i32;
}

// proto: QAbstractItemView * QComboBox::view();
impl<'a> /*trait*/ QComboBox_view for () {
  fn view(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox4viewEv()};
    unsafe {_ZNK9QComboBox4viewEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn minimumSizeHint<T: QComboBox_minimumSizeHint>(&mut self, value: T) -> i32 {
    value.minimumSizeHint(self);
    return 1;
  }
}

pub trait QComboBox_minimumSizeHint {
  fn minimumSizeHint(self, this: &mut QComboBox) -> i32;
}

// proto: QSize QComboBox::minimumSizeHint();
impl<'a> /*trait*/ QComboBox_minimumSizeHint for () {
  fn minimumSizeHint(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox15minimumSizeHintEv()};
    unsafe {_ZNK9QComboBox15minimumSizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn clear<T: QComboBox_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QComboBox_clear {
  fn clear(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::clear();
impl<'a> /*trait*/ QComboBox_clear for () {
  fn clear(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox5clearEv()};
    unsafe {_ZN9QComboBox5clearEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn maxCount<T: QComboBox_maxCount>(&mut self, value: T) -> i32 {
    value.maxCount(self);
    return 1;
  }
}

pub trait QComboBox_maxCount {
  fn maxCount(self, this: &mut QComboBox) -> i32;
}

// proto: int QComboBox::maxCount();
impl<'a> /*trait*/ QComboBox_maxCount for () {
  fn maxCount(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8maxCountEv()};
    unsafe {_ZNK9QComboBox8maxCountEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn addItem<T: QComboBox_addItem>(&mut self, value: T) -> i32 {
    value.addItem(self);
    return 1;
  }
}

pub trait QComboBox_addItem {
  fn addItem(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::addItem(const QIcon & icon, const QString & text, const QVariant & userData);
impl<'a> /*trait*/ QComboBox_addItem for (&'a  QIcon, &'a  QString, &'a  QVariant) {
  fn addItem(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox7addItemERK5QIconRK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN9QComboBox7addItemERK5QIconRK7QStringRK8QVariant(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn insertItems<T: QComboBox_insertItems>(&mut self, value: T) -> i32 {
    value.insertItems(self);
    return 1;
  }
}

pub trait QComboBox_insertItems {
  fn insertItems(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::insertItems(int index, const QStringList & texts);
impl<'a> /*trait*/ QComboBox_insertItems for (i32, &'a  QStringList) {
  fn insertItems(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11insertItemsEiRK11QStringList()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN9QComboBox11insertItemsEiRK11QStringList(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn iconSize<T: QComboBox_iconSize>(&mut self, value: T) -> i32 {
    value.iconSize(self);
    return 1;
  }
}

pub trait QComboBox_iconSize {
  fn iconSize(self, this: &mut QComboBox) -> i32;
}

// proto: QSize QComboBox::iconSize();
impl<'a> /*trait*/ QComboBox_iconSize for () {
  fn iconSize(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8iconSizeEv()};
    unsafe {_ZNK9QComboBox8iconSizeEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn rootModelIndex<T: QComboBox_rootModelIndex>(&mut self, value: T) -> i32 {
    value.rootModelIndex(self);
    return 1;
  }
}

pub trait QComboBox_rootModelIndex {
  fn rootModelIndex(self, this: &mut QComboBox) -> i32;
}

// proto: QModelIndex QComboBox::rootModelIndex();
impl<'a> /*trait*/ QComboBox_rootModelIndex for () {
  fn rootModelIndex(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox14rootModelIndexEv()};
    unsafe {_ZNK9QComboBox14rootModelIndexEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn setEditable<T: QComboBox_setEditable>(&mut self, value: T) -> i32 {
    value.setEditable(self);
    return 1;
  }
}

pub trait QComboBox_setEditable {
  fn setEditable(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::setEditable(bool editable);
impl<'a> /*trait*/ QComboBox_setEditable for (i8) {
  fn setEditable(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setEditableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QComboBox11setEditableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn setItemIcon<T: QComboBox_setItemIcon>(&mut self, value: T) -> i32 {
    value.setItemIcon(self);
    return 1;
  }
}

pub trait QComboBox_setItemIcon {
  fn setItemIcon(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::setItemIcon(int index, const QIcon & icon);
impl<'a> /*trait*/ QComboBox_setItemIcon for (i32, &'a  QIcon) {
  fn setItemIcon(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setItemIconEiRK5QIcon()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN9QComboBox11setItemIconEiRK5QIcon(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn currentTextChanged<T: QComboBox_currentTextChanged>(&mut self, value: T) -> i32 {
    value.currentTextChanged(self);
    return 1;
  }
}

pub trait QComboBox_currentTextChanged {
  fn currentTextChanged(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::currentTextChanged(const QString & );
impl<'a> /*trait*/ QComboBox_currentTextChanged for (&'a  QString) {
  fn currentTextChanged(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox18currentTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QComboBox18currentTextChangedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn autoCompletion<T: QComboBox_autoCompletion>(&mut self, value: T) -> i32 {
    value.autoCompletion(self);
    return 1;
  }
}

pub trait QComboBox_autoCompletion {
  fn autoCompletion(self, this: &mut QComboBox) -> i32;
}

// proto: bool QComboBox::autoCompletion();
impl<'a> /*trait*/ QComboBox_autoCompletion for () {
  fn autoCompletion(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox14autoCompletionEv()};
    unsafe {_ZNK9QComboBox14autoCompletionEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn currentData<T: QComboBox_currentData>(&mut self, value: T) -> i32 {
    value.currentData(self);
    return 1;
  }
}

pub trait QComboBox_currentData {
  fn currentData(self, this: &mut QComboBox) -> i32;
}

// proto: QVariant QComboBox::currentData(int role);
impl<'a> /*trait*/ QComboBox_currentData for (i32) {
  fn currentData(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox11currentDataEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QComboBox11currentDataEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn hasFrame<T: QComboBox_hasFrame>(&mut self, value: T) -> i32 {
    value.hasFrame(self);
    return 1;
  }
}

pub trait QComboBox_hasFrame {
  fn hasFrame(self, this: &mut QComboBox) -> i32;
}

// proto: bool QComboBox::hasFrame();
impl<'a> /*trait*/ QComboBox_hasFrame for () {
  fn hasFrame(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8hasFrameEv()};
    unsafe {_ZNK9QComboBox8hasFrameEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn validator<T: QComboBox_validator>(&mut self, value: T) -> i32 {
    value.validator(self);
    return 1;
  }
}

pub trait QComboBox_validator {
  fn validator(self, this: &mut QComboBox) -> i32;
}

// proto: const QValidator * QComboBox::validator();
impl<'a> /*trait*/ QComboBox_validator for () {
  fn validator(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox9validatorEv()};
    unsafe {_ZNK9QComboBox9validatorEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn itemText<T: QComboBox_itemText>(&mut self, value: T) -> i32 {
    value.itemText(self);
    return 1;
  }
}

pub trait QComboBox_itemText {
  fn itemText(self, this: &mut QComboBox) -> i32;
}

// proto: QString QComboBox::itemText(int index);
impl<'a> /*trait*/ QComboBox_itemText for (i32) {
  fn itemText(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8itemTextEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QComboBox8itemTextEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn setItemData<T: QComboBox_setItemData>(&mut self, value: T) -> i32 {
    value.setItemData(self);
    return 1;
  }
}

pub trait QComboBox_setItemData {
  fn setItemData(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::setItemData(int index, const QVariant & value, int role);
impl<'a> /*trait*/ QComboBox_setItemData for (i32, &'a  QVariant, i32) {
  fn setItemData(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setItemDataEiRK8QVarianti()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN9QComboBox11setItemDataEiRK8QVarianti(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn highlighted<T: QComboBox_highlighted>(&mut self, value: T) -> i32 {
    value.highlighted(self);
    return 1;
  }
}

pub trait QComboBox_highlighted {
  fn highlighted(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::highlighted(int index);
impl<'a> /*trait*/ QComboBox_highlighted for (i32) {
  fn highlighted(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11highlightedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QComboBox11highlightedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn hidePopup<T: QComboBox_hidePopup>(&mut self, value: T) -> i32 {
    value.hidePopup(self);
    return 1;
  }
}

pub trait QComboBox_hidePopup {
  fn hidePopup(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::hidePopup();
impl<'a> /*trait*/ QComboBox_hidePopup for () {
  fn hidePopup(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox9hidePopupEv()};
    unsafe {_ZN9QComboBox9hidePopupEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn insertItem<T: QComboBox_insertItem>(&mut self, value: T) -> i32 {
    value.insertItem(self);
    return 1;
  }
}

pub trait QComboBox_insertItem {
  fn insertItem(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::insertItem(int index, const QIcon & icon, const QString & text, const QVariant & userData);
impl<'a> /*trait*/ QComboBox_insertItem for (i32, &'a  QIcon, &'a  QString, &'a  QVariant) {
  fn insertItem(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox10insertItemEiRK5QIconRK7QStringRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    unsafe {_ZN9QComboBox10insertItemEiRK5QIconRK7QStringRK8QVariant(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn setCurrentText<T: QComboBox_setCurrentText>(&mut self, value: T) -> i32 {
    value.setCurrentText(self);
    return 1;
  }
}

pub trait QComboBox_setCurrentText {
  fn setCurrentText(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::setCurrentText(const QString & text);
impl<'a> /*trait*/ QComboBox_setCurrentText for (&'a  QString) {
  fn setCurrentText(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox14setCurrentTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QComboBox14setCurrentTextERK7QString(arg0)};
    return 1;
  }
}

// proto: void QComboBox::highlighted(const QString & );
impl<'a> /*trait*/ QComboBox_highlighted for (&'a  QString) {
  fn highlighted(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11highlightedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QComboBox11highlightedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn editTextChanged<T: QComboBox_editTextChanged>(&mut self, value: T) -> i32 {
    value.editTextChanged(self);
    return 1;
  }
}

pub trait QComboBox_editTextChanged {
  fn editTextChanged(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::editTextChanged(const QString & );
impl<'a> /*trait*/ QComboBox_editTextChanged for (&'a  QString) {
  fn editTextChanged(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox15editTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QComboBox15editTextChangedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn modelColumn<T: QComboBox_modelColumn>(&mut self, value: T) -> i32 {
    value.modelColumn(self);
    return 1;
  }
}

pub trait QComboBox_modelColumn {
  fn modelColumn(self, this: &mut QComboBox) -> i32;
}

// proto: int QComboBox::modelColumn();
impl<'a> /*trait*/ QComboBox_modelColumn for () {
  fn modelColumn(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox11modelColumnEv()};
    unsafe {_ZNK9QComboBox11modelColumnEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn sizeHint<T: QComboBox_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QComboBox_sizeHint {
  fn sizeHint(self, this: &mut QComboBox) -> i32;
}

// proto: QSize QComboBox::sizeHint();
impl<'a> /*trait*/ QComboBox_sizeHint for () {
  fn sizeHint(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8sizeHintEv()};
    unsafe {_ZNK9QComboBox8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn itemData<T: QComboBox_itemData>(&mut self, value: T) -> i32 {
    value.itemData(self);
    return 1;
  }
}

pub trait QComboBox_itemData {
  fn itemData(self, this: &mut QComboBox) -> i32;
}

// proto: QVariant QComboBox::itemData(int index, int role);
impl<'a> /*trait*/ QComboBox_itemData for (i32, i32) {
  fn itemData(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8itemDataEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK9QComboBox8itemDataEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn activated<T: QComboBox_activated>(&mut self, value: T) -> i32 {
    value.activated(self);
    return 1;
  }
}

pub trait QComboBox_activated {
  fn activated(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::activated(int index);
impl<'a> /*trait*/ QComboBox_activated for (i32) {
  fn activated(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox9activatedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QComboBox9activatedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn setCompleter<T: QComboBox_setCompleter>(&mut self, value: T) -> i32 {
    value.setCompleter(self);
    return 1;
  }
}

pub trait QComboBox_setCompleter {
  fn setCompleter(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::setCompleter(QCompleter * c);
impl<'a> /*trait*/ QComboBox_setCompleter for (&'a mut QCompleter) {
  fn setCompleter(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox12setCompleterEP10QCompleter()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QComboBox12setCompleterEP10QCompleter(arg0)};
    return 1;
  }
}

// proto: void QComboBox::activated(const QString & );
impl<'a> /*trait*/ QComboBox_activated for (&'a  QString) {
  fn activated(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox9activatedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QComboBox9activatedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn maxVisibleItems<T: QComboBox_maxVisibleItems>(&mut self, value: T) -> i32 {
    value.maxVisibleItems(self);
    return 1;
  }
}

pub trait QComboBox_maxVisibleItems {
  fn maxVisibleItems(self, this: &mut QComboBox) -> i32;
}

// proto: int QComboBox::maxVisibleItems();
impl<'a> /*trait*/ QComboBox_maxVisibleItems for () {
  fn maxVisibleItems(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox15maxVisibleItemsEv()};
    unsafe {_ZNK9QComboBox15maxVisibleItemsEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn NewQComboBox<T: QComboBox_NewQComboBox>(value: T) -> QComboBox {
    let rsthis = value.NewQComboBox();
    return rsthis;
    // return 1;
  }
}

pub trait QComboBox_NewQComboBox {
  fn NewQComboBox(self) -> QComboBox;
}

// proto: void QComboBox::NewQComboBox(QWidget * parent);
impl<'a> /*trait*/ QComboBox_NewQComboBox for (&'a mut QWidget) {
  fn NewQComboBox(self) -> QComboBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBoxC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QComboBoxC1EP7QWidget(qthis, arg0)};
    let rsthis = QComboBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn currentIndexChanged<T: QComboBox_currentIndexChanged>(&mut self, value: T) -> i32 {
    value.currentIndexChanged(self);
    return 1;
  }
}

pub trait QComboBox_currentIndexChanged {
  fn currentIndexChanged(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::currentIndexChanged(const QString & );
impl<'a> /*trait*/ QComboBox_currentIndexChanged for (&'a  QString) {
  fn currentIndexChanged(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox19currentIndexChangedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QComboBox19currentIndexChangedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn setCurrentIndex<T: QComboBox_setCurrentIndex>(&mut self, value: T) -> i32 {
    value.setCurrentIndex(self);
    return 1;
  }
}

pub trait QComboBox_setCurrentIndex {
  fn setCurrentIndex(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::setCurrentIndex(int index);
impl<'a> /*trait*/ QComboBox_setCurrentIndex for (i32) {
  fn setCurrentIndex(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox15setCurrentIndexEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QComboBox15setCurrentIndexEi(arg0)};
    return 1;
  }
}

// proto: void QComboBox::NewQComboBox(const QComboBox & );
impl<'a> /*trait*/ QComboBox_NewQComboBox for (&'a  QComboBox) {
  fn NewQComboBox(self) -> QComboBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QComboBoxC1ERKS_(qthis, arg0)};
    let rsthis = QComboBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn setRootModelIndex<T: QComboBox_setRootModelIndex>(&mut self, value: T) -> i32 {
    value.setRootModelIndex(self);
    return 1;
  }
}

pub trait QComboBox_setRootModelIndex {
  fn setRootModelIndex(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::setRootModelIndex(const QModelIndex & index);
impl<'a> /*trait*/ QComboBox_setRootModelIndex for (&'a  QModelIndex) {
  fn setRootModelIndex(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox17setRootModelIndexERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QComboBox17setRootModelIndexERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn setEditText<T: QComboBox_setEditText>(&mut self, value: T) -> i32 {
    value.setEditText(self);
    return 1;
  }
}

pub trait QComboBox_setEditText {
  fn setEditText(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::setEditText(const QString & text);
impl<'a> /*trait*/ QComboBox_setEditText for (&'a  QString) {
  fn setEditText(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setEditTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QComboBox11setEditTextERK7QString(arg0)};
    return 1;
  }
}

// proto: void QComboBox::addItem(const QString & text, const QVariant & userData);
impl<'a> /*trait*/ QComboBox_addItem for (&'a  QString, &'a  QVariant) {
  fn addItem(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox7addItemERK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN9QComboBox7addItemERK7QStringRK8QVariant(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn completer<T: QComboBox_completer>(&mut self, value: T) -> i32 {
    value.completer(self);
    return 1;
  }
}

pub trait QComboBox_completer {
  fn completer(self, this: &mut QComboBox) -> i32;
}

// proto: QCompleter * QComboBox::completer();
impl<'a> /*trait*/ QComboBox_completer for () {
  fn completer(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox9completerEv()};
    unsafe {_ZNK9QComboBox9completerEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn removeItem<T: QComboBox_removeItem>(&mut self, value: T) -> i32 {
    value.removeItem(self);
    return 1;
  }
}

pub trait QComboBox_removeItem {
  fn removeItem(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::removeItem(int index);
impl<'a> /*trait*/ QComboBox_removeItem for (i32) {
  fn removeItem(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox10removeItemEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QComboBox10removeItemEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn count<T: QComboBox_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QComboBox_count {
  fn count(self, this: &mut QComboBox) -> i32;
}

// proto: int QComboBox::count();
impl<'a> /*trait*/ QComboBox_count for () {
  fn count(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox5countEv()};
    unsafe {_ZNK9QComboBox5countEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn addItems<T: QComboBox_addItems>(&mut self, value: T) -> i32 {
    value.addItems(self);
    return 1;
  }
}

pub trait QComboBox_addItems {
  fn addItems(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::addItems(const QStringList & texts);
impl<'a> /*trait*/ QComboBox_addItems for (&'a  QStringList) {
  fn addItems(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox8addItemsERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QComboBox8addItemsERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn setMinimumContentsLength<T: QComboBox_setMinimumContentsLength>(&mut self, value: T) -> i32 {
    value.setMinimumContentsLength(self);
    return 1;
  }
}

pub trait QComboBox_setMinimumContentsLength {
  fn setMinimumContentsLength(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::setMinimumContentsLength(int characters);
impl<'a> /*trait*/ QComboBox_setMinimumContentsLength for (i32) {
  fn setMinimumContentsLength(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox24setMinimumContentsLengthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QComboBox24setMinimumContentsLengthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn duplicatesEnabled<T: QComboBox_duplicatesEnabled>(&mut self, value: T) -> i32 {
    value.duplicatesEnabled(self);
    return 1;
  }
}

pub trait QComboBox_duplicatesEnabled {
  fn duplicatesEnabled(self, this: &mut QComboBox) -> i32;
}

// proto: bool QComboBox::duplicatesEnabled();
impl<'a> /*trait*/ QComboBox_duplicatesEnabled for () {
  fn duplicatesEnabled(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox17duplicatesEnabledEv()};
    unsafe {_ZNK9QComboBox17duplicatesEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn FreeQComboBox<T: QComboBox_FreeQComboBox>(&mut self, value: T) -> i32 {
    value.FreeQComboBox(self);
    return 1;
  }
}

pub trait QComboBox_FreeQComboBox {
  fn FreeQComboBox(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::FreeQComboBox();
impl<'a> /*trait*/ QComboBox_FreeQComboBox for () {
  fn FreeQComboBox(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBoxD0Ev()};
    unsafe {_ZN9QComboBoxD0Ev()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn model<T: QComboBox_model>(&mut self, value: T) -> i32 {
    value.model(self);
    return 1;
  }
}

pub trait QComboBox_model {
  fn model(self, this: &mut QComboBox) -> i32;
}

// proto: QAbstractItemModel * QComboBox::model();
impl<'a> /*trait*/ QComboBox_model for () {
  fn model(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox5modelEv()};
    unsafe {_ZNK9QComboBox5modelEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn minimumContentsLength<T: QComboBox_minimumContentsLength>(&mut self, value: T) -> i32 {
    value.minimumContentsLength(self);
    return 1;
  }
}

pub trait QComboBox_minimumContentsLength {
  fn minimumContentsLength(self, this: &mut QComboBox) -> i32;
}

// proto: int QComboBox::minimumContentsLength();
impl<'a> /*trait*/ QComboBox_minimumContentsLength for () {
  fn minimumContentsLength(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox21minimumContentsLengthEv()};
    unsafe {_ZNK9QComboBox21minimumContentsLengthEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn isEditable<T: QComboBox_isEditable>(&mut self, value: T) -> i32 {
    value.isEditable(self);
    return 1;
  }
}

pub trait QComboBox_isEditable {
  fn isEditable(self, this: &mut QComboBox) -> i32;
}

// proto: bool QComboBox::isEditable();
impl<'a> /*trait*/ QComboBox_isEditable for () {
  fn isEditable(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox10isEditableEv()};
    unsafe {_ZNK9QComboBox10isEditableEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn setMaxCount<T: QComboBox_setMaxCount>(&mut self, value: T) -> i32 {
    value.setMaxCount(self);
    return 1;
  }
}

pub trait QComboBox_setMaxCount {
  fn setMaxCount(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::setMaxCount(int max);
impl<'a> /*trait*/ QComboBox_setMaxCount for (i32) {
  fn setMaxCount(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setMaxCountEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QComboBox11setMaxCountEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn currentIndex<T: QComboBox_currentIndex>(&mut self, value: T) -> i32 {
    value.currentIndex(self);
    return 1;
  }
}

pub trait QComboBox_currentIndex {
  fn currentIndex(self, this: &mut QComboBox) -> i32;
}

// proto: int QComboBox::currentIndex();
impl<'a> /*trait*/ QComboBox_currentIndex for () {
  fn currentIndex(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox12currentIndexEv()};
    unsafe {_ZNK9QComboBox12currentIndexEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn setDuplicatesEnabled<T: QComboBox_setDuplicatesEnabled>(&mut self, value: T) -> i32 {
    value.setDuplicatesEnabled(self);
    return 1;
  }
}

pub trait QComboBox_setDuplicatesEnabled {
  fn setDuplicatesEnabled(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::setDuplicatesEnabled(bool enable);
impl<'a> /*trait*/ QComboBox_setDuplicatesEnabled for (i8) {
  fn setDuplicatesEnabled(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox20setDuplicatesEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QComboBox20setDuplicatesEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn currentText<T: QComboBox_currentText>(&mut self, value: T) -> i32 {
    value.currentText(self);
    return 1;
  }
}

pub trait QComboBox_currentText {
  fn currentText(self, this: &mut QComboBox) -> i32;
}

// proto: QString QComboBox::currentText();
impl<'a> /*trait*/ QComboBox_currentText for () {
  fn currentText(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox11currentTextEv()};
    unsafe {_ZNK9QComboBox11currentTextEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn showPopup<T: QComboBox_showPopup>(&mut self, value: T) -> i32 {
    value.showPopup(self);
    return 1;
  }
}

pub trait QComboBox_showPopup {
  fn showPopup(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::showPopup();
impl<'a> /*trait*/ QComboBox_showPopup for () {
  fn showPopup(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox9showPopupEv()};
    unsafe {_ZN9QComboBox9showPopupEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn lineEdit<T: QComboBox_lineEdit>(&mut self, value: T) -> i32 {
    value.lineEdit(self);
    return 1;
  }
}

pub trait QComboBox_lineEdit {
  fn lineEdit(self, this: &mut QComboBox) -> i32;
}

// proto: QLineEdit * QComboBox::lineEdit();
impl<'a> /*trait*/ QComboBox_lineEdit for () {
  fn lineEdit(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8lineEditEv()};
    unsafe {_ZNK9QComboBox8lineEditEv()};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn itemDelegate<T: QComboBox_itemDelegate>(&mut self, value: T) -> i32 {
    value.itemDelegate(self);
    return 1;
  }
}

pub trait QComboBox_itemDelegate {
  fn itemDelegate(self, this: &mut QComboBox) -> i32;
}

// proto: QAbstractItemDelegate * QComboBox::itemDelegate();
impl<'a> /*trait*/ QComboBox_itemDelegate for () {
  fn itemDelegate(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox12itemDelegateEv()};
    unsafe {_ZNK9QComboBox12itemDelegateEv()};
    return 1;
  }
}

// proto: void QComboBox::currentIndexChanged(int index);
impl<'a> /*trait*/ QComboBox_currentIndexChanged for (i32) {
  fn currentIndexChanged(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox19currentIndexChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QComboBox19currentIndexChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn setMaxVisibleItems<T: QComboBox_setMaxVisibleItems>(&mut self, value: T) -> i32 {
    value.setMaxVisibleItems(self);
    return 1;
  }
}

pub trait QComboBox_setMaxVisibleItems {
  fn setMaxVisibleItems(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::setMaxVisibleItems(int maxItems);
impl<'a> /*trait*/ QComboBox_setMaxVisibleItems for (i32) {
  fn setMaxVisibleItems(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox18setMaxVisibleItemsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QComboBox18setMaxVisibleItemsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn event<T: QComboBox_event>(&mut self, value: T) -> i32 {
    value.event(self);
    return 1;
  }
}

pub trait QComboBox_event {
  fn event(self, this: &mut QComboBox) -> i32;
}

// proto: bool QComboBox::event(QEvent * event);
impl<'a> /*trait*/ QComboBox_event for (&'a mut QEvent) {
  fn event(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QComboBox5eventEP6QEvent(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn setModelColumn<T: QComboBox_setModelColumn>(&mut self, value: T) -> i32 {
    value.setModelColumn(self);
    return 1;
  }
}

pub trait QComboBox_setModelColumn {
  fn setModelColumn(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::setModelColumn(int visibleColumn);
impl<'a> /*trait*/ QComboBox_setModelColumn for (i32) {
  fn setModelColumn(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox14setModelColumnEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QComboBox14setModelColumnEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn setItemText<T: QComboBox_setItemText>(&mut self, value: T) -> i32 {
    value.setItemText(self);
    return 1;
  }
}

pub trait QComboBox_setItemText {
  fn setItemText(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::setItemText(int index, const QString & text);
impl<'a> /*trait*/ QComboBox_setItemText for (i32, &'a  QString) {
  fn setItemText(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setItemTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN9QComboBox11setItemTextEiRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn setLineEdit<T: QComboBox_setLineEdit>(&mut self, value: T) -> i32 {
    value.setLineEdit(self);
    return 1;
  }
}

pub trait QComboBox_setLineEdit {
  fn setLineEdit(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::setLineEdit(QLineEdit * edit);
impl<'a> /*trait*/ QComboBox_setLineEdit for (&'a mut QLineEdit) {
  fn setLineEdit(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox11setLineEditEP9QLineEdit()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QComboBox11setLineEditEP9QLineEdit(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn itemIcon<T: QComboBox_itemIcon>(&mut self, value: T) -> i32 {
    value.itemIcon(self);
    return 1;
  }
}

pub trait QComboBox_itemIcon {
  fn itemIcon(self, this: &mut QComboBox) -> i32;
}

// proto: QIcon QComboBox::itemIcon(int index);
impl<'a> /*trait*/ QComboBox_itemIcon for (i32) {
  fn itemIcon(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox8itemIconEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK9QComboBox8itemIconEi(arg0)};
    return 1;
  }
}

// proto: void QComboBox::insertItem(int index, const QString & text, const QVariant & userData);
impl<'a> /*trait*/ QComboBox_insertItem for (i32, &'a  QString, &'a  QVariant) {
  fn insertItem(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox10insertItemEiRK7QStringRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN9QComboBox10insertItemEiRK7QStringRK8QVariant(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn setValidator<T: QComboBox_setValidator>(&mut self, value: T) -> i32 {
    value.setValidator(self);
    return 1;
  }
}

pub trait QComboBox_setValidator {
  fn setValidator(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::setValidator(const QValidator * v);
impl<'a> /*trait*/ QComboBox_setValidator for (&'a  QValidator) {
  fn setValidator(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox12setValidatorEPK10QValidator()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QComboBox12setValidatorEPK10QValidator(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn insertSeparator<T: QComboBox_insertSeparator>(&mut self, value: T) -> i32 {
    value.insertSeparator(self);
    return 1;
  }
}

pub trait QComboBox_insertSeparator {
  fn insertSeparator(self, this: &mut QComboBox) -> i32;
}

// proto: void QComboBox::insertSeparator(int index);
impl<'a> /*trait*/ QComboBox_insertSeparator for (i32) {
  fn insertSeparator(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QComboBox15insertSeparatorEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QComboBox15insertSeparatorEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QComboBox {
  pub fn metaObject<T: QComboBox_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QComboBox_metaObject {
  fn metaObject(self, this: &mut QComboBox) -> i32;
}

// proto: const QMetaObject * QComboBox::metaObject();
impl<'a> /*trait*/ QComboBox_metaObject for () {
  fn metaObject(self, this: &mut QComboBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QComboBox10metaObjectEv()};
    unsafe {_ZNK9QComboBox10metaObjectEv()};
    return 1;
  }
}

