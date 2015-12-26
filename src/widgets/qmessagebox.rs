// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtWidgets/qmessagebox.h
// dst-file: /src/widgets/qmessagebox.rs
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
use super::qdialog::QDialog; // 773
use std::ops::Deref;
use super::qwidget::QWidget; // 773
use super::super::core::qstring::QString; // 771
use super::super::gui::qpixmap::QPixmap; // 771
use super::qpushbutton::QPushButton; // 773
use super::super::core::qobject::QObject; // 771
use super::qcheckbox::QCheckBox; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QMessageBox_Class_Size() -> c_int;
  // proto: static int QMessageBox::critical(QWidget * parent, const QString & title, const QString & text, int button0, int button1, int button2);
  fn _ZN11QMessageBox8criticalEP7QWidgetRK7QStringS4_iii(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: c_int, arg4: c_int, arg5: c_int) -> c_int;
  // proto:  void QMessageBox::setButtonText(int button, const QString & text);
  fn _ZN11QMessageBox13setButtonTextEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QMessageBox::~QMessageBox();
  fn _ZN11QMessageBoxD0Ev(qthis: *mut c_void);
  // proto:  void QMessageBox::setText(const QString & text);
  fn _ZN11QMessageBox7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMessageBox::setIconPixmap(const QPixmap & pixmap);
  fn _ZN11QMessageBox13setIconPixmapERK7QPixmap(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static void QMessageBox::about(QWidget * parent, const QString & title, const QString & text);
  fn _ZN11QMessageBox5aboutEP7QWidgetRK7QStringS4_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QString QMessageBox::text();
  fn _ZNK11QMessageBox4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static int QMessageBox::question(QWidget * parent, const QString & title, const QString & text, const QString & button0Text, const QString & button1Text, const QString & button2Text, int defaultButtonNumber, int escapeButtonNumber);
  fn _ZN11QMessageBox8questionEP7QWidgetRK7QStringS4_S4_S4_S4_ii(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: c_int, arg7: c_int) -> c_int;
  // proto: static int QMessageBox::warning(QWidget * parent, const QString & title, const QString & text, int button0, int button1, int button2);
  fn _ZN11QMessageBox7warningEP7QWidgetRK7QStringS4_iii(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: c_int, arg4: c_int, arg5: c_int) -> c_int;
  // proto:  void QMessageBox::QMessageBox(const QMessageBox & );
  fn dector_ZN11QMessageBoxC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QMessageBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QMessageBox::metaObject();
  fn _ZNK11QMessageBox10metaObjectEv(qthis: *mut c_void);
  // proto: static int QMessageBox::question(QWidget * parent, const QString & title, const QString & text, int button0, int button1, int button2);
  fn _ZN11QMessageBox8questionEP7QWidgetRK7QStringS4_iii(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: c_int, arg4: c_int, arg5: c_int) -> c_int;
  // proto:  QPushButton * QMessageBox::defaultButton();
  fn _ZNK11QMessageBox13defaultButtonEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMessageBox::open(QObject * receiver, const char * member);
  fn _ZN11QMessageBox4openEP7QObjectPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char);
  // proto:  QList<QAbstractButton *> QMessageBox::buttons();
  fn _ZNK11QMessageBox7buttonsEv(qthis: *mut c_void);
  // proto: static void QMessageBox::aboutQt(QWidget * parent, const QString & title);
  fn _ZN11QMessageBox7aboutQtEP7QWidgetRK7QString(arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QString QMessageBox::informativeText();
  fn _ZNK11QMessageBox15informativeTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMessageBox::setInformativeText(const QString & text);
  fn _ZN11QMessageBox18setInformativeTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMessageBox::setDetailedText(const QString & text);
  fn _ZN11QMessageBox15setDetailedTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMessageBox::QMessageBox(QWidget * parent);
  fn dector_ZN11QMessageBoxC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QMessageBoxC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static int QMessageBox::critical(QWidget * parent, const QString & title, const QString & text, const QString & button0Text, const QString & button1Text, const QString & button2Text, int defaultButtonNumber, int escapeButtonNumber);
  fn _ZN11QMessageBox8criticalEP7QWidgetRK7QStringS4_S4_S4_S4_ii(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: c_int, arg7: c_int) -> c_int;
  // proto:  QAbstractButton * QMessageBox::clickedButton();
  fn _ZNK11QMessageBox13clickedButtonEv(qthis: *mut c_void);
  // proto:  void QMessageBox::setDefaultButton(QPushButton * button);
  fn _ZN11QMessageBox16setDefaultButtonEP11QPushButton(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static int QMessageBox::warning(QWidget * parent, const QString & title, const QString & text, const QString & button0Text, const QString & button1Text, const QString & button2Text, int defaultButtonNumber, int escapeButtonNumber);
  fn _ZN11QMessageBox7warningEP7QWidgetRK7QStringS4_S4_S4_S4_ii(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void, arg5: *mut c_void, arg6: c_int, arg7: c_int) -> c_int;
  // proto: static int QMessageBox::information(QWidget * parent, const QString & title, const QString & text, int button0, int button1, int button2);
  fn _ZN11QMessageBox11informationEP7QWidgetRK7QStringS4_iii(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: c_int, arg4: c_int, arg5: c_int) -> c_int;
  // proto:  void QMessageBox::setCheckBox(QCheckBox * cb);
  fn _ZN11QMessageBox11setCheckBoxEP9QCheckBox(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMessageBox::setWindowTitle(const QString & title);
  fn _ZN11QMessageBox14setWindowTitleERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QAbstractButton * QMessageBox::escapeButton();
  fn _ZNK11QMessageBox12escapeButtonEv(qthis: *mut c_void);
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
} // <= ext block end

// body block begin =>
// class sizeof(QMessageBox)=1
pub struct QMessageBox {
  qbase: QDialog,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMessageBox {
  pub fn inheritFrom(qthis: *mut c_void) -> QMessageBox {
    return QMessageBox{qbase: QDialog::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QMessageBox {
  type Target = QDialog;

  fn deref(&self) -> &QDialog {
    return & self.qbase;
  }
}
impl AsRef<QDialog> for QMessageBox {
  fn as_ref(& self) -> & QDialog {
    return & self.qbase;
  }
}
  // proto: static int QMessageBox::critical(QWidget * parent, const QString & title, const QString & text, int button0, int button1, int button2);
impl /*struct*/ QMessageBox {
  pub fn critical_s<RetType, T: QMessageBox_critical_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.critical_s();
    // return 1;
  }
}

pub trait QMessageBox_critical_s<RetType> {
  fn critical_s(self ) -> RetType;
}

  // proto: static int QMessageBox::critical(QWidget * parent, const QString & title, const QString & text, int button0, int button1, int button2);
impl<'a> /*trait*/ QMessageBox_critical_s<i32> for (&'a QWidget, &'a QString, &'a QString, i32, i32, i32) {
  fn critical_s(self ) -> i32 {
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

  // proto:  void QMessageBox::setButtonText(int button, const QString & text);
impl /*struct*/ QMessageBox {
  pub fn setButtonText<RetType, T: QMessageBox_setButtonText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setButtonText(self);
    // return 1;
  }
}

pub trait QMessageBox_setButtonText<RetType> {
  fn setButtonText(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  void QMessageBox::setButtonText(int button, const QString & text);
impl<'a> /*trait*/ QMessageBox_setButtonText<()> for (i32, &'a QString) {
  fn setButtonText(self , rsthis: & QMessageBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox13setButtonTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox13setButtonTextEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QMessageBox::~QMessageBox();
impl /*struct*/ QMessageBox {
  pub fn Free<RetType, T: QMessageBox_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QMessageBox_Free<RetType> {
  fn Free(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  void QMessageBox::~QMessageBox();
impl<'a> /*trait*/ QMessageBox_Free<()> for () {
  fn Free(self , rsthis: & QMessageBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBoxD0Ev()};
     unsafe {_ZN11QMessageBoxD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMessageBox::setText(const QString & text);
impl /*struct*/ QMessageBox {
  pub fn setText<RetType, T: QMessageBox_setText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QMessageBox_setText<RetType> {
  fn setText(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  void QMessageBox::setText(const QString & text);
impl<'a> /*trait*/ QMessageBox_setText<()> for (&'a QString) {
  fn setText(self , rsthis: & QMessageBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMessageBox::setIconPixmap(const QPixmap & pixmap);
impl /*struct*/ QMessageBox {
  pub fn setIconPixmap<RetType, T: QMessageBox_setIconPixmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIconPixmap(self);
    // return 1;
  }
}

pub trait QMessageBox_setIconPixmap<RetType> {
  fn setIconPixmap(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  void QMessageBox::setIconPixmap(const QPixmap & pixmap);
impl<'a> /*trait*/ QMessageBox_setIconPixmap<()> for (&'a QPixmap) {
  fn setIconPixmap(self , rsthis: & QMessageBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox13setIconPixmapERK7QPixmap()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox13setIconPixmapERK7QPixmap(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static void QMessageBox::about(QWidget * parent, const QString & title, const QString & text);
impl /*struct*/ QMessageBox {
  pub fn about_s<RetType, T: QMessageBox_about_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.about_s();
    // return 1;
  }
}

pub trait QMessageBox_about_s<RetType> {
  fn about_s(self ) -> RetType;
}

  // proto: static void QMessageBox::about(QWidget * parent, const QString & title, const QString & text);
impl<'a> /*trait*/ QMessageBox_about_s<()> for (&'a QWidget, &'a QString, &'a QString) {
  fn about_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox5aboutEP7QWidgetRK7QStringS4_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox5aboutEP7QWidgetRK7QStringS4_(arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QString QMessageBox::text();
impl /*struct*/ QMessageBox {
  pub fn text<RetType, T: QMessageBox_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QMessageBox_text<RetType> {
  fn text(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  QString QMessageBox::text();
impl<'a> /*trait*/ QMessageBox_text<QString> for () {
  fn text(self , rsthis: & QMessageBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox4textEv()};
    let mut ret = unsafe {_ZNK11QMessageBox4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static int QMessageBox::question(QWidget * parent, const QString & title, const QString & text, const QString & button0Text, const QString & button1Text, const QString & button2Text, int defaultButtonNumber, int escapeButtonNumber);
impl /*struct*/ QMessageBox {
  pub fn question_s<RetType, T: QMessageBox_question_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.question_s();
    // return 1;
  }
}

pub trait QMessageBox_question_s<RetType> {
  fn question_s(self ) -> RetType;
}

  // proto: static int QMessageBox::question(QWidget * parent, const QString & title, const QString & text, const QString & button0Text, const QString & button1Text, const QString & button2Text, int defaultButtonNumber, int escapeButtonNumber);
impl<'a> /*trait*/ QMessageBox_question_s<i32> for (&'a QWidget, &'a QString, &'a QString, &'a QString, &'a QString, &'a QString, i32, i32) {
  fn question_s(self ) -> i32 {
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

  // proto: static int QMessageBox::warning(QWidget * parent, const QString & title, const QString & text, int button0, int button1, int button2);
impl /*struct*/ QMessageBox {
  pub fn warning_s<RetType, T: QMessageBox_warning_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.warning_s();
    // return 1;
  }
}

pub trait QMessageBox_warning_s<RetType> {
  fn warning_s(self ) -> RetType;
}

  // proto: static int QMessageBox::warning(QWidget * parent, const QString & title, const QString & text, int button0, int button1, int button2);
impl<'a> /*trait*/ QMessageBox_warning_s<i32> for (&'a QWidget, &'a QString, &'a QString, i32, i32, i32) {
  fn warning_s(self ) -> i32 {
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

  // proto:  void QMessageBox::QMessageBox(const QMessageBox & );
impl /*struct*/ QMessageBox {
  pub fn New<T: QMessageBox_New>(value: T) -> QMessageBox {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QMessageBox_New {
  fn New(self) -> QMessageBox;
}

  // proto:  void QMessageBox::QMessageBox(const QMessageBox & );
impl<'a> /*trait*/ QMessageBox_New for (&'a QMessageBox) {
  fn New(self) -> QMessageBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBoxC1ERKS_()};
    let ctysz: c_int = unsafe{QMessageBox_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QMessageBoxC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QMessageBoxC1ERKS_(arg0)};
    let rsthis = QMessageBox{/**/qbase: QDialog::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QMessageBox::metaObject();
impl /*struct*/ QMessageBox {
  pub fn metaObject<RetType, T: QMessageBox_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QMessageBox_metaObject<RetType> {
  fn metaObject(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  const QMetaObject * QMessageBox::metaObject();
impl<'a> /*trait*/ QMessageBox_metaObject<()> for () {
  fn metaObject(self , rsthis: & QMessageBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox10metaObjectEv()};
     unsafe {_ZNK11QMessageBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static int QMessageBox::question(QWidget * parent, const QString & title, const QString & text, int button0, int button1, int button2);
impl<'a> /*trait*/ QMessageBox_question_s<i32> for (&'a QWidget, &'a QString, &'a QString, i32, i32, i32) {
  fn question_s(self ) -> i32 {
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

  // proto:  QPushButton * QMessageBox::defaultButton();
impl /*struct*/ QMessageBox {
  pub fn defaultButton<RetType, T: QMessageBox_defaultButton<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultButton(self);
    // return 1;
  }
}

pub trait QMessageBox_defaultButton<RetType> {
  fn defaultButton(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  QPushButton * QMessageBox::defaultButton();
impl<'a> /*trait*/ QMessageBox_defaultButton<QPushButton> for () {
  fn defaultButton(self , rsthis: & QMessageBox) -> QPushButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox13defaultButtonEv()};
    let mut ret = unsafe {_ZNK11QMessageBox13defaultButtonEv(rsthis.qclsinst)};
    let mut ret1 = QPushButton::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMessageBox::open(QObject * receiver, const char * member);
impl /*struct*/ QMessageBox {
  pub fn open<RetType, T: QMessageBox_open<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.open(self);
    // return 1;
  }
}

pub trait QMessageBox_open<RetType> {
  fn open(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  void QMessageBox::open(QObject * receiver, const char * member);
impl<'a> /*trait*/ QMessageBox_open<()> for (&'a QObject, &'a  String) {
  fn open(self , rsthis: & QMessageBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox4openEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {_ZN11QMessageBox4openEP7QObjectPKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QList<QAbstractButton *> QMessageBox::buttons();
impl /*struct*/ QMessageBox {
  pub fn buttons<RetType, T: QMessageBox_buttons<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.buttons(self);
    // return 1;
  }
}

pub trait QMessageBox_buttons<RetType> {
  fn buttons(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  QList<QAbstractButton *> QMessageBox::buttons();
impl<'a> /*trait*/ QMessageBox_buttons<()> for () {
  fn buttons(self , rsthis: & QMessageBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox7buttonsEv()};
     unsafe {_ZNK11QMessageBox7buttonsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static void QMessageBox::aboutQt(QWidget * parent, const QString & title);
impl /*struct*/ QMessageBox {
  pub fn aboutQt_s<RetType, T: QMessageBox_aboutQt_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.aboutQt_s();
    // return 1;
  }
}

pub trait QMessageBox_aboutQt_s<RetType> {
  fn aboutQt_s(self ) -> RetType;
}

  // proto: static void QMessageBox::aboutQt(QWidget * parent, const QString & title);
impl<'a> /*trait*/ QMessageBox_aboutQt_s<()> for (&'a QWidget, &'a QString) {
  fn aboutQt_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox7aboutQtEP7QWidgetRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox7aboutQtEP7QWidgetRK7QString(arg0, arg1)};
    // return 1;
  }
}

  // proto:  QString QMessageBox::informativeText();
impl /*struct*/ QMessageBox {
  pub fn informativeText<RetType, T: QMessageBox_informativeText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.informativeText(self);
    // return 1;
  }
}

pub trait QMessageBox_informativeText<RetType> {
  fn informativeText(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  QString QMessageBox::informativeText();
impl<'a> /*trait*/ QMessageBox_informativeText<QString> for () {
  fn informativeText(self , rsthis: & QMessageBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox15informativeTextEv()};
    let mut ret = unsafe {_ZNK11QMessageBox15informativeTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMessageBox::setInformativeText(const QString & text);
impl /*struct*/ QMessageBox {
  pub fn setInformativeText<RetType, T: QMessageBox_setInformativeText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setInformativeText(self);
    // return 1;
  }
}

pub trait QMessageBox_setInformativeText<RetType> {
  fn setInformativeText(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  void QMessageBox::setInformativeText(const QString & text);
impl<'a> /*trait*/ QMessageBox_setInformativeText<()> for (&'a QString) {
  fn setInformativeText(self , rsthis: & QMessageBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox18setInformativeTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox18setInformativeTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMessageBox::setDetailedText(const QString & text);
impl /*struct*/ QMessageBox {
  pub fn setDetailedText<RetType, T: QMessageBox_setDetailedText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDetailedText(self);
    // return 1;
  }
}

pub trait QMessageBox_setDetailedText<RetType> {
  fn setDetailedText(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  void QMessageBox::setDetailedText(const QString & text);
impl<'a> /*trait*/ QMessageBox_setDetailedText<()> for (&'a QString) {
  fn setDetailedText(self , rsthis: & QMessageBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox15setDetailedTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox15setDetailedTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMessageBox::QMessageBox(QWidget * parent);
impl<'a> /*trait*/ QMessageBox_New for (&'a QWidget) {
  fn New(self) -> QMessageBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBoxC1EP7QWidget()};
    let ctysz: c_int = unsafe{QMessageBox_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QMessageBoxC1EP7QWidget(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QMessageBoxC1EP7QWidget(arg0)};
    let rsthis = QMessageBox{/**/qbase: QDialog::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static int QMessageBox::critical(QWidget * parent, const QString & title, const QString & text, const QString & button0Text, const QString & button1Text, const QString & button2Text, int defaultButtonNumber, int escapeButtonNumber);
impl<'a> /*trait*/ QMessageBox_critical_s<i32> for (&'a QWidget, &'a QString, &'a QString, &'a QString, &'a QString, &'a QString, i32, i32) {
  fn critical_s(self ) -> i32 {
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

  // proto:  QAbstractButton * QMessageBox::clickedButton();
impl /*struct*/ QMessageBox {
  pub fn clickedButton<RetType, T: QMessageBox_clickedButton<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clickedButton(self);
    // return 1;
  }
}

pub trait QMessageBox_clickedButton<RetType> {
  fn clickedButton(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  QAbstractButton * QMessageBox::clickedButton();
impl<'a> /*trait*/ QMessageBox_clickedButton<()> for () {
  fn clickedButton(self , rsthis: & QMessageBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox13clickedButtonEv()};
     unsafe {_ZNK11QMessageBox13clickedButtonEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMessageBox::setDefaultButton(QPushButton * button);
impl /*struct*/ QMessageBox {
  pub fn setDefaultButton<RetType, T: QMessageBox_setDefaultButton<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDefaultButton(self);
    // return 1;
  }
}

pub trait QMessageBox_setDefaultButton<RetType> {
  fn setDefaultButton(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  void QMessageBox::setDefaultButton(QPushButton * button);
impl<'a> /*trait*/ QMessageBox_setDefaultButton<()> for (&'a QPushButton) {
  fn setDefaultButton(self , rsthis: & QMessageBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox16setDefaultButtonEP11QPushButton()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox16setDefaultButtonEP11QPushButton(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static int QMessageBox::warning(QWidget * parent, const QString & title, const QString & text, const QString & button0Text, const QString & button1Text, const QString & button2Text, int defaultButtonNumber, int escapeButtonNumber);
impl<'a> /*trait*/ QMessageBox_warning_s<i32> for (&'a QWidget, &'a QString, &'a QString, &'a QString, &'a QString, &'a QString, i32, i32) {
  fn warning_s(self ) -> i32 {
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

  // proto: static int QMessageBox::information(QWidget * parent, const QString & title, const QString & text, int button0, int button1, int button2);
impl /*struct*/ QMessageBox {
  pub fn information_s<RetType, T: QMessageBox_information_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.information_s();
    // return 1;
  }
}

pub trait QMessageBox_information_s<RetType> {
  fn information_s(self ) -> RetType;
}

  // proto: static int QMessageBox::information(QWidget * parent, const QString & title, const QString & text, int button0, int button1, int button2);
impl<'a> /*trait*/ QMessageBox_information_s<i32> for (&'a QWidget, &'a QString, &'a QString, i32, i32, i32) {
  fn information_s(self ) -> i32 {
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

  // proto:  void QMessageBox::setCheckBox(QCheckBox * cb);
impl /*struct*/ QMessageBox {
  pub fn setCheckBox<RetType, T: QMessageBox_setCheckBox<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCheckBox(self);
    // return 1;
  }
}

pub trait QMessageBox_setCheckBox<RetType> {
  fn setCheckBox(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  void QMessageBox::setCheckBox(QCheckBox * cb);
impl<'a> /*trait*/ QMessageBox_setCheckBox<()> for (&'a QCheckBox) {
  fn setCheckBox(self , rsthis: & QMessageBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox11setCheckBoxEP9QCheckBox()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox11setCheckBoxEP9QCheckBox(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMessageBox::setWindowTitle(const QString & title);
impl /*struct*/ QMessageBox {
  pub fn setWindowTitle<RetType, T: QMessageBox_setWindowTitle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWindowTitle(self);
    // return 1;
  }
}

pub trait QMessageBox_setWindowTitle<RetType> {
  fn setWindowTitle(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  void QMessageBox::setWindowTitle(const QString & title);
impl<'a> /*trait*/ QMessageBox_setWindowTitle<()> for (&'a QString) {
  fn setWindowTitle(self , rsthis: & QMessageBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMessageBox14setWindowTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMessageBox14setWindowTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAbstractButton * QMessageBox::escapeButton();
impl /*struct*/ QMessageBox {
  pub fn escapeButton<RetType, T: QMessageBox_escapeButton<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.escapeButton(self);
    // return 1;
  }
}

pub trait QMessageBox_escapeButton<RetType> {
  fn escapeButton(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  QAbstractButton * QMessageBox::escapeButton();
impl<'a> /*trait*/ QMessageBox_escapeButton<()> for () {
  fn escapeButton(self , rsthis: & QMessageBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox12escapeButtonEv()};
     unsafe {_ZNK11QMessageBox12escapeButtonEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPixmap QMessageBox::iconPixmap();
impl /*struct*/ QMessageBox {
  pub fn iconPixmap<RetType, T: QMessageBox_iconPixmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.iconPixmap(self);
    // return 1;
  }
}

pub trait QMessageBox_iconPixmap<RetType> {
  fn iconPixmap(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  QPixmap QMessageBox::iconPixmap();
impl<'a> /*trait*/ QMessageBox_iconPixmap<QPixmap> for () {
  fn iconPixmap(self , rsthis: & QMessageBox) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox10iconPixmapEv()};
    let mut ret = unsafe {_ZNK11QMessageBox10iconPixmapEv(rsthis.qclsinst)};
    let mut ret1 = QPixmap::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QMessageBox::detailedText();
impl /*struct*/ QMessageBox {
  pub fn detailedText<RetType, T: QMessageBox_detailedText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.detailedText(self);
    // return 1;
  }
}

pub trait QMessageBox_detailedText<RetType> {
  fn detailedText(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  QString QMessageBox::detailedText();
impl<'a> /*trait*/ QMessageBox_detailedText<QString> for () {
  fn detailedText(self , rsthis: & QMessageBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox12detailedTextEv()};
    let mut ret = unsafe {_ZNK11QMessageBox12detailedTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QCheckBox * QMessageBox::checkBox();
impl /*struct*/ QMessageBox {
  pub fn checkBox<RetType, T: QMessageBox_checkBox<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.checkBox(self);
    // return 1;
  }
}

pub trait QMessageBox_checkBox<RetType> {
  fn checkBox(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  QCheckBox * QMessageBox::checkBox();
impl<'a> /*trait*/ QMessageBox_checkBox<QCheckBox> for () {
  fn checkBox(self , rsthis: & QMessageBox) -> QCheckBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox8checkBoxEv()};
    let mut ret = unsafe {_ZNK11QMessageBox8checkBoxEv(rsthis.qclsinst)};
    let mut ret1 = QCheckBox::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QMessageBox::buttonText(int button);
impl /*struct*/ QMessageBox {
  pub fn buttonText<RetType, T: QMessageBox_buttonText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.buttonText(self);
    // return 1;
  }
}

pub trait QMessageBox_buttonText<RetType> {
  fn buttonText(self , rsthis: & QMessageBox) -> RetType;
}

  // proto:  QString QMessageBox::buttonText(int button);
impl<'a> /*trait*/ QMessageBox_buttonText<QString> for (i32) {
  fn buttonText(self , rsthis: & QMessageBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMessageBox10buttonTextEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QMessageBox10buttonTextEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static int QMessageBox::information(QWidget * parent, const QString & title, const QString & text, const QString & button0Text, const QString & button1Text, const QString & button2Text, int defaultButtonNumber, int escapeButtonNumber);
impl<'a> /*trait*/ QMessageBox_information_s<i32> for (&'a QWidget, &'a QString, &'a QString, &'a QString, &'a QString, &'a QString, i32, i32) {
  fn information_s(self ) -> i32 {
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

// <= body block end

