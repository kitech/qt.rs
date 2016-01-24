// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtCore/qcommandlineparser.h
// dst-file: /src/core/qcommandlineparser.rs
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
use super::qstringlist::QStringList; // 773
use super::qstring::QString; // 773
use super::qcommandlineoption::QCommandLineOption; // 773
use super::qcoreapplication::QCoreApplication; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QCommandLineParser_Class_Size() -> c_int;
  // proto:  void QCommandLineParser::process(const QStringList & arguments);
  fn C_ZN18QCommandLineParser7processERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QCommandLineParser::value(const QString & name);
  fn C_ZNK18QCommandLineParser5valueERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QCommandLineParser::errorText();
  fn C_ZNK18QCommandLineParser9errorTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QCommandLineParser::clearPositionalArguments();
  fn C_ZN18QCommandLineParser24clearPositionalArgumentsEv(qthis: u64 /* *mut c_void*/);
  // proto:  QStringList QCommandLineParser::values(const QCommandLineOption & option);
  fn C_ZNK18QCommandLineParser6valuesERK18QCommandLineOption(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QCommandLineParser::isSet(const QString & name);
  fn C_ZNK18QCommandLineParser5isSetERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QCommandLineParser::showHelp(int exitCode);
  fn C_ZN18QCommandLineParser8showHelpEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QCommandLineParser::addOption(const QCommandLineOption & commandLineOption);
  fn C_ZN18QCommandLineParser9addOptionERK18QCommandLineOption(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QCommandLineParser::showVersion();
  fn C_ZN18QCommandLineParser11showVersionEv(qthis: u64 /* *mut c_void*/);
  // proto:  QCommandLineOption QCommandLineParser::addHelpOption();
  fn C_ZN18QCommandLineParser13addHelpOptionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStringList QCommandLineParser::optionNames();
  fn C_ZNK18QCommandLineParser11optionNamesEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QCommandLineParser::isSet(const QCommandLineOption & option);
  fn C_ZNK18QCommandLineParser5isSetERK18QCommandLineOption(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QCommandLineParser::addPositionalArgument(const QString & name, const QString & description, const QString & syntax);
  fn C_ZN18QCommandLineParser21addPositionalArgumentERK7QStringS2_S2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QCommandLineParser::~QCommandLineParser();
  fn C_ZN18QCommandLineParserD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QCommandLineParser::process(const QCoreApplication & app);
  fn C_ZN18QCommandLineParser7processERK16QCoreApplication(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QCommandLineParser::helpText();
  fn C_ZNK18QCommandLineParser8helpTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStringList QCommandLineParser::values(const QString & name);
  fn C_ZNK18QCommandLineParser6valuesERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QCommandLineParser::applicationDescription();
  fn C_ZNK18QCommandLineParser22applicationDescriptionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QCommandLineParser::value(const QCommandLineOption & option);
  fn C_ZNK18QCommandLineParser5valueERK18QCommandLineOption(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QCommandLineOption QCommandLineParser::addVersionOption();
  fn C_ZN18QCommandLineParser16addVersionOptionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStringList QCommandLineParser::positionalArguments();
  fn C_ZNK18QCommandLineParser19positionalArgumentsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QCommandLineParser::setApplicationDescription(const QString & description);
  fn C_ZN18QCommandLineParser25setApplicationDescriptionERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCommandLineParser::QCommandLineParser();
  fn C_ZN18QCommandLineParserC2Ev() -> u64;
  // proto:  bool QCommandLineParser::parse(const QStringList & arguments);
  fn C_ZN18QCommandLineParser5parseERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QStringList QCommandLineParser::unknownOptionNames();
  fn C_ZNK18QCommandLineParser18unknownOptionNamesEv(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QCommandLineParser)=8
#[derive(Default)]
pub struct QCommandLineParser {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QCommandLineParser {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QCommandLineParser {
    return QCommandLineParser{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QCommandLineParser::process(const QStringList & arguments);
impl /*struct*/ QCommandLineParser {
  pub fn process<RetType, T: QCommandLineParser_process<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.process(self);
    // return 1;
  }
}

pub trait QCommandLineParser_process<RetType> {
  fn process(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  void QCommandLineParser::process(const QStringList & arguments);
impl<'a> /*trait*/ QCommandLineParser_process<()> for (&'a QStringList) {
  fn process(self , rsthis: & QCommandLineParser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineParser7processERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QCommandLineParser7processERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QCommandLineParser::value(const QString & name);
impl /*struct*/ QCommandLineParser {
  pub fn value<RetType, T: QCommandLineParser_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QCommandLineParser_value<RetType> {
  fn value(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  QString QCommandLineParser::value(const QString & name);
impl<'a> /*trait*/ QCommandLineParser_value<QString> for (&'a QString) {
  fn value(self , rsthis: & QCommandLineParser) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineParser5valueERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK18QCommandLineParser5valueERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QCommandLineParser::errorText();
impl /*struct*/ QCommandLineParser {
  pub fn errorText<RetType, T: QCommandLineParser_errorText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorText(self);
    // return 1;
  }
}

pub trait QCommandLineParser_errorText<RetType> {
  fn errorText(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  QString QCommandLineParser::errorText();
impl<'a> /*trait*/ QCommandLineParser_errorText<QString> for () {
  fn errorText(self , rsthis: & QCommandLineParser) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineParser9errorTextEv()};
    let mut ret = unsafe {C_ZNK18QCommandLineParser9errorTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QCommandLineParser::clearPositionalArguments();
impl /*struct*/ QCommandLineParser {
  pub fn clearPositionalArguments<RetType, T: QCommandLineParser_clearPositionalArguments<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearPositionalArguments(self);
    // return 1;
  }
}

pub trait QCommandLineParser_clearPositionalArguments<RetType> {
  fn clearPositionalArguments(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  void QCommandLineParser::clearPositionalArguments();
impl<'a> /*trait*/ QCommandLineParser_clearPositionalArguments<()> for () {
  fn clearPositionalArguments(self , rsthis: & QCommandLineParser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineParser24clearPositionalArgumentsEv()};
     unsafe {C_ZN18QCommandLineParser24clearPositionalArgumentsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringList QCommandLineParser::values(const QCommandLineOption & option);
impl /*struct*/ QCommandLineParser {
  pub fn values<RetType, T: QCommandLineParser_values<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.values(self);
    // return 1;
  }
}

pub trait QCommandLineParser_values<RetType> {
  fn values(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  QStringList QCommandLineParser::values(const QCommandLineOption & option);
impl<'a> /*trait*/ QCommandLineParser_values<()> for (&'a QCommandLineOption) {
  fn values(self , rsthis: & QCommandLineParser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineParser6valuesERK18QCommandLineOption()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZNK18QCommandLineParser6valuesERK18QCommandLineOption(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QCommandLineParser::isSet(const QString & name);
impl /*struct*/ QCommandLineParser {
  pub fn isSet<RetType, T: QCommandLineParser_isSet<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSet(self);
    // return 1;
  }
}

pub trait QCommandLineParser_isSet<RetType> {
  fn isSet(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  bool QCommandLineParser::isSet(const QString & name);
impl<'a> /*trait*/ QCommandLineParser_isSet<i8> for (&'a QString) {
  fn isSet(self , rsthis: & QCommandLineParser) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineParser5isSetERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK18QCommandLineParser5isSetERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QCommandLineParser::showHelp(int exitCode);
impl /*struct*/ QCommandLineParser {
  pub fn showHelp<RetType, T: QCommandLineParser_showHelp<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showHelp(self);
    // return 1;
  }
}

pub trait QCommandLineParser_showHelp<RetType> {
  fn showHelp(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  void QCommandLineParser::showHelp(int exitCode);
impl<'a> /*trait*/ QCommandLineParser_showHelp<()> for (i32) {
  fn showHelp(self , rsthis: & QCommandLineParser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineParser8showHelpEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN18QCommandLineParser8showHelpEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QCommandLineParser::addOption(const QCommandLineOption & commandLineOption);
impl /*struct*/ QCommandLineParser {
  pub fn addOption<RetType, T: QCommandLineParser_addOption<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addOption(self);
    // return 1;
  }
}

pub trait QCommandLineParser_addOption<RetType> {
  fn addOption(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  bool QCommandLineParser::addOption(const QCommandLineOption & commandLineOption);
impl<'a> /*trait*/ QCommandLineParser_addOption<i8> for (&'a QCommandLineOption) {
  fn addOption(self , rsthis: & QCommandLineParser) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineParser9addOptionERK18QCommandLineOption()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN18QCommandLineParser9addOptionERK18QCommandLineOption(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QCommandLineParser::showVersion();
impl /*struct*/ QCommandLineParser {
  pub fn showVersion<RetType, T: QCommandLineParser_showVersion<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showVersion(self);
    // return 1;
  }
}

pub trait QCommandLineParser_showVersion<RetType> {
  fn showVersion(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  void QCommandLineParser::showVersion();
impl<'a> /*trait*/ QCommandLineParser_showVersion<()> for () {
  fn showVersion(self , rsthis: & QCommandLineParser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineParser11showVersionEv()};
     unsafe {C_ZN18QCommandLineParser11showVersionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QCommandLineOption QCommandLineParser::addHelpOption();
impl /*struct*/ QCommandLineParser {
  pub fn addHelpOption<RetType, T: QCommandLineParser_addHelpOption<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addHelpOption(self);
    // return 1;
  }
}

pub trait QCommandLineParser_addHelpOption<RetType> {
  fn addHelpOption(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  QCommandLineOption QCommandLineParser::addHelpOption();
impl<'a> /*trait*/ QCommandLineParser_addHelpOption<QCommandLineOption> for () {
  fn addHelpOption(self , rsthis: & QCommandLineParser) -> QCommandLineOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineParser13addHelpOptionEv()};
    let mut ret = unsafe {C_ZN18QCommandLineParser13addHelpOptionEv(rsthis.qclsinst)};
    let mut ret1 = QCommandLineOption::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringList QCommandLineParser::optionNames();
impl /*struct*/ QCommandLineParser {
  pub fn optionNames<RetType, T: QCommandLineParser_optionNames<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.optionNames(self);
    // return 1;
  }
}

pub trait QCommandLineParser_optionNames<RetType> {
  fn optionNames(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  QStringList QCommandLineParser::optionNames();
impl<'a> /*trait*/ QCommandLineParser_optionNames<()> for () {
  fn optionNames(self , rsthis: & QCommandLineParser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineParser11optionNamesEv()};
     unsafe {C_ZNK18QCommandLineParser11optionNamesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QCommandLineParser::isSet(const QCommandLineOption & option);
impl<'a> /*trait*/ QCommandLineParser_isSet<i8> for (&'a QCommandLineOption) {
  fn isSet(self , rsthis: & QCommandLineParser) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineParser5isSetERK18QCommandLineOption()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK18QCommandLineParser5isSetERK18QCommandLineOption(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QCommandLineParser::addPositionalArgument(const QString & name, const QString & description, const QString & syntax);
impl /*struct*/ QCommandLineParser {
  pub fn addPositionalArgument<RetType, T: QCommandLineParser_addPositionalArgument<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addPositionalArgument(self);
    // return 1;
  }
}

pub trait QCommandLineParser_addPositionalArgument<RetType> {
  fn addPositionalArgument(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  void QCommandLineParser::addPositionalArgument(const QString & name, const QString & description, const QString & syntax);
impl<'a> /*trait*/ QCommandLineParser_addPositionalArgument<()> for (&'a QString, &'a QString, &'a QString) {
  fn addPositionalArgument(self , rsthis: & QCommandLineParser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineParser21addPositionalArgumentERK7QStringS2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN18QCommandLineParser21addPositionalArgumentERK7QStringS2_S2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QCommandLineParser::~QCommandLineParser();
impl /*struct*/ QCommandLineParser {
  pub fn free<RetType, T: QCommandLineParser_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QCommandLineParser_free<RetType> {
  fn free(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  void QCommandLineParser::~QCommandLineParser();
impl<'a> /*trait*/ QCommandLineParser_free<()> for () {
  fn free(self , rsthis: & QCommandLineParser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineParserD2Ev()};
     unsafe {C_ZN18QCommandLineParserD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCommandLineParser::process(const QCoreApplication & app);
impl<'a> /*trait*/ QCommandLineParser_process<()> for (&'a QCoreApplication) {
  fn process(self , rsthis: & QCommandLineParser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineParser7processERK16QCoreApplication()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QCommandLineParser7processERK16QCoreApplication(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QCommandLineParser::helpText();
impl /*struct*/ QCommandLineParser {
  pub fn helpText<RetType, T: QCommandLineParser_helpText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.helpText(self);
    // return 1;
  }
}

pub trait QCommandLineParser_helpText<RetType> {
  fn helpText(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  QString QCommandLineParser::helpText();
impl<'a> /*trait*/ QCommandLineParser_helpText<QString> for () {
  fn helpText(self , rsthis: & QCommandLineParser) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineParser8helpTextEv()};
    let mut ret = unsafe {C_ZNK18QCommandLineParser8helpTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringList QCommandLineParser::values(const QString & name);
impl<'a> /*trait*/ QCommandLineParser_values<()> for (&'a QString) {
  fn values(self , rsthis: & QCommandLineParser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineParser6valuesERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZNK18QCommandLineParser6valuesERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QCommandLineParser::applicationDescription();
impl /*struct*/ QCommandLineParser {
  pub fn applicationDescription<RetType, T: QCommandLineParser_applicationDescription<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.applicationDescription(self);
    // return 1;
  }
}

pub trait QCommandLineParser_applicationDescription<RetType> {
  fn applicationDescription(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  QString QCommandLineParser::applicationDescription();
impl<'a> /*trait*/ QCommandLineParser_applicationDescription<QString> for () {
  fn applicationDescription(self , rsthis: & QCommandLineParser) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineParser22applicationDescriptionEv()};
    let mut ret = unsafe {C_ZNK18QCommandLineParser22applicationDescriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QCommandLineParser::value(const QCommandLineOption & option);
impl<'a> /*trait*/ QCommandLineParser_value<QString> for (&'a QCommandLineOption) {
  fn value(self , rsthis: & QCommandLineParser) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineParser5valueERK18QCommandLineOption()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK18QCommandLineParser5valueERK18QCommandLineOption(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QCommandLineOption QCommandLineParser::addVersionOption();
impl /*struct*/ QCommandLineParser {
  pub fn addVersionOption<RetType, T: QCommandLineParser_addVersionOption<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addVersionOption(self);
    // return 1;
  }
}

pub trait QCommandLineParser_addVersionOption<RetType> {
  fn addVersionOption(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  QCommandLineOption QCommandLineParser::addVersionOption();
impl<'a> /*trait*/ QCommandLineParser_addVersionOption<QCommandLineOption> for () {
  fn addVersionOption(self , rsthis: & QCommandLineParser) -> QCommandLineOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineParser16addVersionOptionEv()};
    let mut ret = unsafe {C_ZN18QCommandLineParser16addVersionOptionEv(rsthis.qclsinst)};
    let mut ret1 = QCommandLineOption::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringList QCommandLineParser::positionalArguments();
impl /*struct*/ QCommandLineParser {
  pub fn positionalArguments<RetType, T: QCommandLineParser_positionalArguments<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.positionalArguments(self);
    // return 1;
  }
}

pub trait QCommandLineParser_positionalArguments<RetType> {
  fn positionalArguments(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  QStringList QCommandLineParser::positionalArguments();
impl<'a> /*trait*/ QCommandLineParser_positionalArguments<()> for () {
  fn positionalArguments(self , rsthis: & QCommandLineParser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineParser19positionalArgumentsEv()};
     unsafe {C_ZNK18QCommandLineParser19positionalArgumentsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCommandLineParser::setApplicationDescription(const QString & description);
impl /*struct*/ QCommandLineParser {
  pub fn setApplicationDescription<RetType, T: QCommandLineParser_setApplicationDescription<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setApplicationDescription(self);
    // return 1;
  }
}

pub trait QCommandLineParser_setApplicationDescription<RetType> {
  fn setApplicationDescription(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  void QCommandLineParser::setApplicationDescription(const QString & description);
impl<'a> /*trait*/ QCommandLineParser_setApplicationDescription<()> for (&'a QString) {
  fn setApplicationDescription(self , rsthis: & QCommandLineParser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineParser25setApplicationDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QCommandLineParser25setApplicationDescriptionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommandLineParser::QCommandLineParser();
impl /*struct*/ QCommandLineParser {
  pub fn new<T: QCommandLineParser_new>(value: T) -> QCommandLineParser {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QCommandLineParser_new {
  fn new(self) -> QCommandLineParser;
}

  // proto:  void QCommandLineParser::QCommandLineParser();
impl<'a> /*trait*/ QCommandLineParser_new for () {
  fn new(self) -> QCommandLineParser {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineParserC2Ev()};
    let ctysz: c_int = unsafe{QCommandLineParser_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN18QCommandLineParserC2Ev()};
    let rsthis = QCommandLineParser{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QCommandLineParser::parse(const QStringList & arguments);
impl /*struct*/ QCommandLineParser {
  pub fn parse<RetType, T: QCommandLineParser_parse<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parse(self);
    // return 1;
  }
}

pub trait QCommandLineParser_parse<RetType> {
  fn parse(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  bool QCommandLineParser::parse(const QStringList & arguments);
impl<'a> /*trait*/ QCommandLineParser_parse<i8> for (&'a QStringList) {
  fn parse(self , rsthis: & QCommandLineParser) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineParser5parseERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN18QCommandLineParser5parseERK11QStringList(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QStringList QCommandLineParser::unknownOptionNames();
impl /*struct*/ QCommandLineParser {
  pub fn unknownOptionNames<RetType, T: QCommandLineParser_unknownOptionNames<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unknownOptionNames(self);
    // return 1;
  }
}

pub trait QCommandLineParser_unknownOptionNames<RetType> {
  fn unknownOptionNames(self , rsthis: & QCommandLineParser) -> RetType;
}

  // proto:  QStringList QCommandLineParser::unknownOptionNames();
impl<'a> /*trait*/ QCommandLineParser_unknownOptionNames<()> for () {
  fn unknownOptionNames(self , rsthis: & QCommandLineParser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineParser18unknownOptionNamesEv()};
     unsafe {C_ZNK18QCommandLineParser18unknownOptionNamesEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

