// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qstring::QString;
use super::qpixmap::QPixmap;
use super::qpushbutton::QPushButton;
use super::qobject::QObject;
use super::qcheckbox::QCheckBox;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: static int QMessageBox::critical(QWidget * parent, const QString & title, const QString & text, int button0, int button1, int button2);
  fn _ZN11QMessageBox8criticalEP7QWidgetRK7QStringS4_iii(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: c_int, arg4: c_int, arg5: c_int) -> c_int;
  // proto:  void QMessageBox::setButtonText(int button, const QString & text);
  fn _ZN11QMessageBox13setButtonTextEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QMessageBox::FreeQMessageBox();
  fn _ZN11QMessageBoxD0Ev(qthis: *mut c_void) ;
  // proto:  void QMessageBox::setText(const QString & text);
  fn _ZN11QMessageBox7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMessageBox::setIconPixmap(const QPixmap & pixmap);
  fn _ZN11QMessageBox13setIconPixmapERK7QPixmap(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static void QMessageBox::about(QWidget * parent, const QString & title, const QString & text);
  fn _ZN11QMessageBox5aboutEP7QWidgetRK7QStringS4_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  QString QMessageBox::text();
  fn _ZNK11QMessageBox4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static int QMessageBox::question(QWidget * parent, const QString & title, const QString & text, const QString & button0Text, const QString & button1Text, const QString & button2Text, int defaultButtonNumber, int escapeButtonNumber);
  fn _ZN11QMessageBox8questionEP7QWidgetRK7QStringS4_S4_S4_S4_ii(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: c_int, arg7: c_int) -> c_int;
  // proto: static int QMessageBox::warning(QWidget * parent, const QString & title, const QString & text, int button0, int button1, int button2);
  fn _ZN11QMessageBox7warningEP7QWidgetRK7QStringS4_iii(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: c_int, arg4: c_int, arg5: c_int) -> c_int;
  // proto:  void QMessageBox::NewQMessageBox(const QMessageBox & );
  fn _ZN11QMessageBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QMessageBox::metaObject();
  fn _ZNK11QMessageBox10metaObjectEv(qthis: *mut c_void) ;
  // proto: static int QMessageBox::question(QWidget * parent, const QString & title, const QString & text, int button0, int button1, int button2);
  fn _ZN11QMessageBox8questionEP7QWidgetRK7QStringS4_iii(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: c_int, arg4: c_int, arg5: c_int) -> c_int;
  // proto:  QPushButton * QMessageBox::defaultButton();
  fn _ZNK11QMessageBox13defaultButtonEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMessageBox::open(QObject * receiver, const char * member);
  fn _ZN11QMessageBox4openEP7QObjectPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char) ;
  // proto:  QList<QAbstractButton *> QMessageBox::buttons();
  fn _ZNK11QMessageBox7buttonsEv(qthis: *mut c_void) ;
  // proto: static void QMessageBox::aboutQt(QWidget * parent, const QString & title);
  fn _ZN11QMessageBox7aboutQtEP7QWidgetRK7QString(arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QString QMessageBox::informativeText();
  fn _ZNK11QMessageBox15informativeTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMessageBox::setInformativeText(const QString & text);
  fn _ZN11QMessageBox18setInformativeTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMessageBox::setDetailedText(const QString & text);
  fn _ZN11QMessageBox15setDetailedTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMessageBox::NewQMessageBox(QWidget * parent);
  fn _ZN11QMessageBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static int QMessageBox::critical(QWidget * parent, const QString & title, const QString & text, const QString & button0Text, const QString & button1Text, const QString & button2Text, int defaultButtonNumber, int escapeButtonNumber);
  fn _ZN11QMessageBox8criticalEP7QWidgetRK7QStringS4_S4_S4_S4_ii(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: c_int, arg7: c_int) -> c_int;
  // proto:  QAbstractButton * QMessageBox::clickedButton();
  fn _ZNK11QMessageBox13clickedButtonEv(qthis: *mut c_void) ;
  // proto:  void QMessageBox::setDefaultButton(QPushButton * button);
  fn _ZN11QMessageBox16setDefaultButtonEP11QPushButton(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static int QMessageBox::warning(QWidget * parent, const QString & title, const QString & text, const QString & button0Text, const QString & button1Text, const QString & button2Text, int defaultButtonNumber, int escapeButtonNumber);
  fn _ZN11QMessageBox7warningEP7QWidgetRK7QStringS4_S4_S4_S4_ii(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: c_int, arg7: c_int) -> c_int;
  // proto: static int QMessageBox::information(QWidget * parent, const QString & title, const QString & text, int button0, int button1, int button2);
  fn _ZN11QMessageBox11informationEP7QWidgetRK7QStringS4_iii(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: c_int, arg4: c_int, arg5: c_int) -> c_int;
  // proto:  void QMessageBox::setCheckBox(QCheckBox * cb);
  fn _ZN11QMessageBox11setCheckBoxEP9QCheckBox(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMessageBox::setWindowTitle(const QString & title);
  fn _ZN11QMessageBox14setWindowTitleERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QAbstractButton * QMessageBox::escapeButton();
  fn _ZNK11QMessageBox12escapeButtonEv(qthis: *mut c_void) ;
  // proto:  QPixmap QMessageBox::iconPixmap();
  fn _ZNK11QMessageBox10iconPixmapEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QMessageBox::detailedText();
  fn _ZNK11QMessageBox12detailedTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QCheckBox * QMessageBox::checkBox();
  fn _ZNK11QMessageBox8checkBoxEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QMessageBox::buttonText(int button);
  fn _ZNK11QMessageBox10buttonTextEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto: static int QMessageBox::information(QWidget * parent, const QString & title, const QString & text, const QString & button0Text, const QString & button1Text, const QString & button2Text, int defaultButtonNumber, int escapeButtonNumber);
  fn _ZN11QMessageBox11informationEP7QWidgetRK7QStringS4_S4_S4_S4_ii(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: c_int, arg7: c_int) -> c_int;
}

// body block begin
// class sizeof(QMessageBox)=1
pub struct QMessageBox {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMessageBox {
  pub fn critical<T: QMessageBox_critical>(&mut self, value: T) -> i32 {
    return value.critical(self);
    // return 1;
  }
}

pub trait QMessageBox_critical {
  fn critical(self, rsthis: &mut QMessageBox) -> i32;
}

// proto: static int QMessageBox::critical(QWidget * parent, const QString & title, const QString & text, int button0, int button1, int button2);
impl<'a> /*trait*/ QMessageBox_critical for (&'a mut QWidget, &'a  QString, &'a  QString, i32, i32, i32) {
  fn critical(self, rsthis: &mut QMessageBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox8criticalEP7QWidgetRK7QStringS4_iii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    let mut ret = unsafe {_ZN11QMessageBox8criticalEP7QWidgetRK7QStringS4_iii(arg0, arg1, arg2, arg3, arg4, arg5)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn setButtonText<T: QMessageBox_setButtonText>(&mut self, value: T)  {
     value.setButtonText(self);
    // return 1;
  }
}

pub trait QMessageBox_setButtonText {
  fn setButtonText(self, rsthis: &mut QMessageBox) ;
}

// proto:  void QMessageBox::setButtonText(int button, const QString & text);
impl<'a> /*trait*/ QMessageBox_setButtonText for (i32, &'a  QString) {
  fn setButtonText(self, rsthis: &mut QMessageBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox13setButtonTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox13setButtonTextEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn FreeQMessageBox<T: QMessageBox_FreeQMessageBox>(&mut self, value: T)  {
     value.FreeQMessageBox(self);
    // return 1;
  }
}

pub trait QMessageBox_FreeQMessageBox {
  fn FreeQMessageBox(self, rsthis: &mut QMessageBox) ;
}

// proto:  void QMessageBox::FreeQMessageBox();
impl<'a> /*trait*/ QMessageBox_FreeQMessageBox for () {
  fn FreeQMessageBox(self, rsthis: &mut QMessageBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBoxD0Ev()};
     unsafe {_ZN11QMessageBoxD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn setText<T: QMessageBox_setText>(&mut self, value: T)  {
     value.setText(self);
    // return 1;
  }
}

pub trait QMessageBox_setText {
  fn setText(self, rsthis: &mut QMessageBox) ;
}

// proto:  void QMessageBox::setText(const QString & text);
impl<'a> /*trait*/ QMessageBox_setText for (&'a  QString) {
  fn setText(self, rsthis: &mut QMessageBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn setIconPixmap<T: QMessageBox_setIconPixmap>(&mut self, value: T)  {
     value.setIconPixmap(self);
    // return 1;
  }
}

pub trait QMessageBox_setIconPixmap {
  fn setIconPixmap(self, rsthis: &mut QMessageBox) ;
}

// proto:  void QMessageBox::setIconPixmap(const QPixmap & pixmap);
impl<'a> /*trait*/ QMessageBox_setIconPixmap for (&'a  QPixmap) {
  fn setIconPixmap(self, rsthis: &mut QMessageBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox13setIconPixmapERK7QPixmap()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox13setIconPixmapERK7QPixmap(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn about<T: QMessageBox_about>(&mut self, value: T)  {
     value.about(self);
    // return 1;
  }
}

pub trait QMessageBox_about {
  fn about(self, rsthis: &mut QMessageBox) ;
}

// proto: static void QMessageBox::about(QWidget * parent, const QString & title, const QString & text);
impl<'a> /*trait*/ QMessageBox_about for (&'a mut QWidget, &'a  QString, &'a  QString) {
  fn about(self, rsthis: &mut QMessageBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox5aboutEP7QWidgetRK7QStringS4_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox5aboutEP7QWidgetRK7QStringS4_(arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn text<T: QMessageBox_text>(&mut self, value: T) -> QString {
    return value.text(self);
    // return 1;
  }
}

pub trait QMessageBox_text {
  fn text(self, rsthis: &mut QMessageBox) -> QString;
}

// proto:  QString QMessageBox::text();
impl<'a> /*trait*/ QMessageBox_text for () {
  fn text(self, rsthis: &mut QMessageBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox4textEv()};
    let mut ret = unsafe {_ZNK11QMessageBox4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn question<T: QMessageBox_question>(&mut self, value: T) -> i32 {
    return value.question(self);
    // return 1;
  }
}

pub trait QMessageBox_question {
  fn question(self, rsthis: &mut QMessageBox) -> i32;
}

// proto: static int QMessageBox::question(QWidget * parent, const QString & title, const QString & text, const QString & button0Text, const QString & button1Text, const QString & button2Text, int defaultButtonNumber, int escapeButtonNumber);
impl<'a> /*trait*/ QMessageBox_question for (&'a mut QWidget, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, i32, i32) {
  fn question(self, rsthis: &mut QMessageBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox8questionEP7QWidgetRK7QStringS4_S4_S4_S4_ii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5.qclsinst  as *mut c_void;
    let arg6 = self.6  as c_int;
    let arg7 = self.7  as c_int;
    let mut ret = unsafe {_ZN11QMessageBox8questionEP7QWidgetRK7QStringS4_S4_S4_S4_ii(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn warning<T: QMessageBox_warning>(&mut self, value: T) -> i32 {
    return value.warning(self);
    // return 1;
  }
}

pub trait QMessageBox_warning {
  fn warning(self, rsthis: &mut QMessageBox) -> i32;
}

// proto: static int QMessageBox::warning(QWidget * parent, const QString & title, const QString & text, int button0, int button1, int button2);
impl<'a> /*trait*/ QMessageBox_warning for (&'a mut QWidget, &'a  QString, &'a  QString, i32, i32, i32) {
  fn warning(self, rsthis: &mut QMessageBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox7warningEP7QWidgetRK7QStringS4_iii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    let mut ret = unsafe {_ZN11QMessageBox7warningEP7QWidgetRK7QStringS4_iii(arg0, arg1, arg2, arg3, arg4, arg5)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn NewQMessageBox<T: QMessageBox_NewQMessageBox>(value: T) -> QMessageBox {
    let rsthis = value.NewQMessageBox();
    return rsthis;
    // return 1;
  }
}

pub trait QMessageBox_NewQMessageBox {
  fn NewQMessageBox(self) -> QMessageBox;
}

// proto: void QMessageBox::NewQMessageBox(const QMessageBox & );
impl<'a> /*trait*/ QMessageBox_NewQMessageBox for (&'a  QMessageBox) {
  fn NewQMessageBox(self) -> QMessageBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QMessageBoxC1ERKS_(qthis, arg0)};
    let rsthis = QMessageBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn metaObject<T: QMessageBox_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QMessageBox_metaObject {
  fn metaObject(self, rsthis: &mut QMessageBox) ;
}

// proto:  const QMetaObject * QMessageBox::metaObject();
impl<'a> /*trait*/ QMessageBox_metaObject for () {
  fn metaObject(self, rsthis: &mut QMessageBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox10metaObjectEv()};
     unsafe {_ZNK11QMessageBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: static int QMessageBox::question(QWidget * parent, const QString & title, const QString & text, int button0, int button1, int button2);
impl<'a> /*trait*/ QMessageBox_question for (&'a mut QWidget, &'a  QString, &'a  QString, i32, i32, i32) {
  fn question(self, rsthis: &mut QMessageBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox8questionEP7QWidgetRK7QStringS4_iii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    let mut ret = unsafe {_ZN11QMessageBox8questionEP7QWidgetRK7QStringS4_iii(arg0, arg1, arg2, arg3, arg4, arg5)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn defaultButton<T: QMessageBox_defaultButton>(&mut self, value: T) -> QPushButton {
    return value.defaultButton(self);
    // return 1;
  }
}

pub trait QMessageBox_defaultButton {
  fn defaultButton(self, rsthis: &mut QMessageBox) -> QPushButton;
}

// proto:  QPushButton * QMessageBox::defaultButton();
impl<'a> /*trait*/ QMessageBox_defaultButton for () {
  fn defaultButton(self, rsthis: &mut QMessageBox) -> QPushButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox13defaultButtonEv()};
    let mut ret = unsafe {_ZNK11QMessageBox13defaultButtonEv(rsthis.qclsinst)};
    let mut ret1 = QPushButton{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn open<T: QMessageBox_open>(&mut self, value: T)  {
     value.open(self);
    // return 1;
  }
}

pub trait QMessageBox_open {
  fn open(self, rsthis: &mut QMessageBox) ;
}

// proto:  void QMessageBox::open(QObject * receiver, const char * member);
impl<'a> /*trait*/ QMessageBox_open for (&'a mut QObject, &'a  String) {
  fn open(self, rsthis: &mut QMessageBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox4openEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
     unsafe {_ZN11QMessageBox4openEP7QObjectPKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn buttons<T: QMessageBox_buttons>(&mut self, value: T)  {
     value.buttons(self);
    // return 1;
  }
}

pub trait QMessageBox_buttons {
  fn buttons(self, rsthis: &mut QMessageBox) ;
}

// proto:  QList<QAbstractButton *> QMessageBox::buttons();
impl<'a> /*trait*/ QMessageBox_buttons for () {
  fn buttons(self, rsthis: &mut QMessageBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox7buttonsEv()};
     unsafe {_ZNK11QMessageBox7buttonsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn aboutQt<T: QMessageBox_aboutQt>(&mut self, value: T)  {
     value.aboutQt(self);
    // return 1;
  }
}

pub trait QMessageBox_aboutQt {
  fn aboutQt(self, rsthis: &mut QMessageBox) ;
}

// proto: static void QMessageBox::aboutQt(QWidget * parent, const QString & title);
impl<'a> /*trait*/ QMessageBox_aboutQt for (&'a mut QWidget, &'a  QString) {
  fn aboutQt(self, rsthis: &mut QMessageBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox7aboutQtEP7QWidgetRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox7aboutQtEP7QWidgetRK7QString(arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn informativeText<T: QMessageBox_informativeText>(&mut self, value: T) -> QString {
    return value.informativeText(self);
    // return 1;
  }
}

pub trait QMessageBox_informativeText {
  fn informativeText(self, rsthis: &mut QMessageBox) -> QString;
}

// proto:  QString QMessageBox::informativeText();
impl<'a> /*trait*/ QMessageBox_informativeText for () {
  fn informativeText(self, rsthis: &mut QMessageBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox15informativeTextEv()};
    let mut ret = unsafe {_ZNK11QMessageBox15informativeTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn setInformativeText<T: QMessageBox_setInformativeText>(&mut self, value: T)  {
     value.setInformativeText(self);
    // return 1;
  }
}

pub trait QMessageBox_setInformativeText {
  fn setInformativeText(self, rsthis: &mut QMessageBox) ;
}

// proto:  void QMessageBox::setInformativeText(const QString & text);
impl<'a> /*trait*/ QMessageBox_setInformativeText for (&'a  QString) {
  fn setInformativeText(self, rsthis: &mut QMessageBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox18setInformativeTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox18setInformativeTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn setDetailedText<T: QMessageBox_setDetailedText>(&mut self, value: T)  {
     value.setDetailedText(self);
    // return 1;
  }
}

pub trait QMessageBox_setDetailedText {
  fn setDetailedText(self, rsthis: &mut QMessageBox) ;
}

// proto:  void QMessageBox::setDetailedText(const QString & text);
impl<'a> /*trait*/ QMessageBox_setDetailedText for (&'a  QString) {
  fn setDetailedText(self, rsthis: &mut QMessageBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox15setDetailedTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox15setDetailedTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QMessageBox::NewQMessageBox(QWidget * parent);
impl<'a> /*trait*/ QMessageBox_NewQMessageBox for (&'a mut QWidget) {
  fn NewQMessageBox(self) -> QMessageBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBoxC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QMessageBoxC1EP7QWidget(qthis, arg0)};
    let rsthis = QMessageBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: static int QMessageBox::critical(QWidget * parent, const QString & title, const QString & text, const QString & button0Text, const QString & button1Text, const QString & button2Text, int defaultButtonNumber, int escapeButtonNumber);
impl<'a> /*trait*/ QMessageBox_critical for (&'a mut QWidget, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, i32, i32) {
  fn critical(self, rsthis: &mut QMessageBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox8criticalEP7QWidgetRK7QStringS4_S4_S4_S4_ii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5.qclsinst  as *mut c_void;
    let arg6 = self.6  as c_int;
    let arg7 = self.7  as c_int;
    let mut ret = unsafe {_ZN11QMessageBox8criticalEP7QWidgetRK7QStringS4_S4_S4_S4_ii(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn clickedButton<T: QMessageBox_clickedButton>(&mut self, value: T)  {
     value.clickedButton(self);
    // return 1;
  }
}

pub trait QMessageBox_clickedButton {
  fn clickedButton(self, rsthis: &mut QMessageBox) ;
}

// proto:  QAbstractButton * QMessageBox::clickedButton();
impl<'a> /*trait*/ QMessageBox_clickedButton for () {
  fn clickedButton(self, rsthis: &mut QMessageBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox13clickedButtonEv()};
     unsafe {_ZNK11QMessageBox13clickedButtonEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn setDefaultButton<T: QMessageBox_setDefaultButton>(&mut self, value: T)  {
     value.setDefaultButton(self);
    // return 1;
  }
}

pub trait QMessageBox_setDefaultButton {
  fn setDefaultButton(self, rsthis: &mut QMessageBox) ;
}

// proto:  void QMessageBox::setDefaultButton(QPushButton * button);
impl<'a> /*trait*/ QMessageBox_setDefaultButton for (&'a mut QPushButton) {
  fn setDefaultButton(self, rsthis: &mut QMessageBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox16setDefaultButtonEP11QPushButton()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox16setDefaultButtonEP11QPushButton(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: static int QMessageBox::warning(QWidget * parent, const QString & title, const QString & text, const QString & button0Text, const QString & button1Text, const QString & button2Text, int defaultButtonNumber, int escapeButtonNumber);
impl<'a> /*trait*/ QMessageBox_warning for (&'a mut QWidget, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, i32, i32) {
  fn warning(self, rsthis: &mut QMessageBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox7warningEP7QWidgetRK7QStringS4_S4_S4_S4_ii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5.qclsinst  as *mut c_void;
    let arg6 = self.6  as c_int;
    let arg7 = self.7  as c_int;
    let mut ret = unsafe {_ZN11QMessageBox7warningEP7QWidgetRK7QStringS4_S4_S4_S4_ii(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn information<T: QMessageBox_information>(&mut self, value: T) -> i32 {
    return value.information(self);
    // return 1;
  }
}

pub trait QMessageBox_information {
  fn information(self, rsthis: &mut QMessageBox) -> i32;
}

// proto: static int QMessageBox::information(QWidget * parent, const QString & title, const QString & text, int button0, int button1, int button2);
impl<'a> /*trait*/ QMessageBox_information for (&'a mut QWidget, &'a  QString, &'a  QString, i32, i32, i32) {
  fn information(self, rsthis: &mut QMessageBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox11informationEP7QWidgetRK7QStringS4_iii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    let mut ret = unsafe {_ZN11QMessageBox11informationEP7QWidgetRK7QStringS4_iii(arg0, arg1, arg2, arg3, arg4, arg5)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn setCheckBox<T: QMessageBox_setCheckBox>(&mut self, value: T)  {
     value.setCheckBox(self);
    // return 1;
  }
}

pub trait QMessageBox_setCheckBox {
  fn setCheckBox(self, rsthis: &mut QMessageBox) ;
}

// proto:  void QMessageBox::setCheckBox(QCheckBox * cb);
impl<'a> /*trait*/ QMessageBox_setCheckBox for (&'a mut QCheckBox) {
  fn setCheckBox(self, rsthis: &mut QMessageBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox11setCheckBoxEP9QCheckBox()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox11setCheckBoxEP9QCheckBox(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn setWindowTitle<T: QMessageBox_setWindowTitle>(&mut self, value: T)  {
     value.setWindowTitle(self);
    // return 1;
  }
}

pub trait QMessageBox_setWindowTitle {
  fn setWindowTitle(self, rsthis: &mut QMessageBox) ;
}

// proto:  void QMessageBox::setWindowTitle(const QString & title);
impl<'a> /*trait*/ QMessageBox_setWindowTitle for (&'a  QString) {
  fn setWindowTitle(self, rsthis: &mut QMessageBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox14setWindowTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox14setWindowTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn escapeButton<T: QMessageBox_escapeButton>(&mut self, value: T)  {
     value.escapeButton(self);
    // return 1;
  }
}

pub trait QMessageBox_escapeButton {
  fn escapeButton(self, rsthis: &mut QMessageBox) ;
}

// proto:  QAbstractButton * QMessageBox::escapeButton();
impl<'a> /*trait*/ QMessageBox_escapeButton for () {
  fn escapeButton(self, rsthis: &mut QMessageBox)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox12escapeButtonEv()};
     unsafe {_ZNK11QMessageBox12escapeButtonEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn iconPixmap<T: QMessageBox_iconPixmap>(&mut self, value: T) -> QPixmap {
    return value.iconPixmap(self);
    // return 1;
  }
}

pub trait QMessageBox_iconPixmap {
  fn iconPixmap(self, rsthis: &mut QMessageBox) -> QPixmap;
}

// proto:  QPixmap QMessageBox::iconPixmap();
impl<'a> /*trait*/ QMessageBox_iconPixmap for () {
  fn iconPixmap(self, rsthis: &mut QMessageBox) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox10iconPixmapEv()};
    let mut ret = unsafe {_ZNK11QMessageBox10iconPixmapEv(rsthis.qclsinst)};
    let mut ret1 = QPixmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn detailedText<T: QMessageBox_detailedText>(&mut self, value: T) -> QString {
    return value.detailedText(self);
    // return 1;
  }
}

pub trait QMessageBox_detailedText {
  fn detailedText(self, rsthis: &mut QMessageBox) -> QString;
}

// proto:  QString QMessageBox::detailedText();
impl<'a> /*trait*/ QMessageBox_detailedText for () {
  fn detailedText(self, rsthis: &mut QMessageBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox12detailedTextEv()};
    let mut ret = unsafe {_ZNK11QMessageBox12detailedTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn checkBox<T: QMessageBox_checkBox>(&mut self, value: T) -> QCheckBox {
    return value.checkBox(self);
    // return 1;
  }
}

pub trait QMessageBox_checkBox {
  fn checkBox(self, rsthis: &mut QMessageBox) -> QCheckBox;
}

// proto:  QCheckBox * QMessageBox::checkBox();
impl<'a> /*trait*/ QMessageBox_checkBox for () {
  fn checkBox(self, rsthis: &mut QMessageBox) -> QCheckBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox8checkBoxEv()};
    let mut ret = unsafe {_ZNK11QMessageBox8checkBoxEv(rsthis.qclsinst)};
    let mut ret1 = QCheckBox{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMessageBox {
  pub fn buttonText<T: QMessageBox_buttonText>(&mut self, value: T) -> QString {
    return value.buttonText(self);
    // return 1;
  }
}

pub trait QMessageBox_buttonText {
  fn buttonText(self, rsthis: &mut QMessageBox) -> QString;
}

// proto:  QString QMessageBox::buttonText(int button);
impl<'a> /*trait*/ QMessageBox_buttonText for (i32) {
  fn buttonText(self, rsthis: &mut QMessageBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox10buttonTextEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QMessageBox10buttonTextEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static int QMessageBox::information(QWidget * parent, const QString & title, const QString & text, const QString & button0Text, const QString & button1Text, const QString & button2Text, int defaultButtonNumber, int escapeButtonNumber);
impl<'a> /*trait*/ QMessageBox_information for (&'a mut QWidget, &'a  QString, &'a  QString, &'a  QString, &'a  QString, &'a  QString, i32, i32) {
  fn information(self, rsthis: &mut QMessageBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox11informationEP7QWidgetRK7QStringS4_S4_S4_S4_ii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5.qclsinst  as *mut c_void;
    let arg6 = self.6  as c_int;
    let arg7 = self.7  as c_int;
    let mut ret = unsafe {_ZN11QMessageBox11informationEP7QWidgetRK7QStringS4_S4_S4_S4_ii(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)};
    return ret as i32;
    // return 1;
  }
}

