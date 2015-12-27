// auto generated, do not modify.
// created: Sun Dec 27 22:52:02 2015
// src-file: /QtGui/qtextobject.h
// dst-file: /src/gui/qtextobject.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::qtextdocument::QTextDocument; // 773
use super::qtextformat::QTextFormat; // 773
use super::super::core::qstring::QString; // 771
use super::qtextformat::QTextCharFormat; // 773
// use super::qtextobject::QTextBlockUserData; // 773
use super::qtextlist::QTextList; // 773
use super::qtextlayout::QTextLayout; // 773
use super::qtextformat::QTextBlockFormat; // 773
// use super::qtextobject::QTextObject; // 773
use super::qtextformat::QTextFrameFormat; // 773
// use super::qtextobject::QTextFrameLayoutData; // 773
use super::qtextcursor::QTextCursor; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTextObject_Class_Size() -> c_int;
  // proto:  QTextDocumentPrivate * QTextObject::docHandle();
  fn _ZNK11QTextObject9docHandleEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextObject::~QTextObject();
  fn _ZN11QTextObjectD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextObject::QTextObject(const QTextObject & );
  fn dector_ZN11QTextObjectC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QTextObjectC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QTextFormat QTextObject::format();
  fn _ZNK11QTextObject6formatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTextObject::formatIndex();
  fn _ZNK11QTextObject11formatIndexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QTextDocument * QTextObject::document();
  fn _ZNK11QTextObject8documentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTextObject::objectIndex();
  fn _ZNK11QTextObject11objectIndexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextObject::QTextObject(QTextDocument * doc);
  fn dector_ZN11QTextObjectC1EP13QTextDocument(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QTextObjectC1EP13QTextDocument(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QTextObject::metaObject();
  fn _ZNK11QTextObject10metaObjectEv(qthis: u64 /* *mut c_void*/);
  fn QTextBlockUserData_Class_Size() -> c_int;
  // proto:  void QTextBlockUserData::~QTextBlockUserData();
  fn _ZN18QTextBlockUserDataD0Ev(qthis: u64 /* *mut c_void*/);
  fn QTextFragment_Class_Size() -> c_int;
  // proto:  int QTextFragment::charFormatIndex();
  fn _ZNK13QTextFragment15charFormatIndexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QTextFragment::position();
  fn _ZNK13QTextFragment8positionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextFragment::QTextFragment(const QTextFragment & o);
  fn dector_ZN13QTextFragmentC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn demth_ZN13QTextFragmentC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QTextFragment::contains(int position);
  fn _ZNK13QTextFragment8containsEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QTextFragment::QTextFragment();
  fn dector_ZN13QTextFragmentC1Ev() -> *mut c_void;
  fn demth_ZN13QTextFragmentC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QString QTextFragment::text();
  fn _ZNK13QTextFragment4textEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<QGlyphRun> QTextFragment::glyphRuns(int from, int length);
  fn _ZNK13QTextFragment9glyphRunsEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  bool QTextFragment::isValid();
  fn demth_ZNK13QTextFragment7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QTextCharFormat QTextFragment::charFormat();
  fn _ZNK13QTextFragment10charFormatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTextFragment::length();
  fn _ZNK13QTextFragment6lengthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  fn QTextFrameLayoutData_Class_Size() -> c_int;
  // proto:  void QTextFrameLayoutData::~QTextFrameLayoutData();
  fn _ZN20QTextFrameLayoutDataD0Ev(qthis: u64 /* *mut c_void*/);
  fn QTextBlock_Class_Size() -> c_int;
  // proto:  const QTextDocument * QTextBlock::document();
  fn _ZNK10QTextBlock8documentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTextBlock QTextBlock::previous();
  fn _ZNK10QTextBlock8previousEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTextBlock::length();
  fn _ZNK10QTextBlock6lengthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QTextBlockUserData * QTextBlock::userData();
  fn _ZNK10QTextBlock8userDataEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextBlock::QTextBlock(const QTextBlock & o);
  fn dector_ZN10QTextBlockC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn demth_ZN10QTextBlockC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QTextBlock::text();
  fn _ZNK10QTextBlock4textEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTextBlock::lineCount();
  fn _ZNK10QTextBlock9lineCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QTextBlock::contains(int position);
  fn _ZNK10QTextBlock8containsEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  int QTextBlock::blockNumber();
  fn _ZNK10QTextBlock11blockNumberEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextBlock::setRevision(int rev);
  fn _ZN10QTextBlock11setRevisionEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTextBlock::setVisible(bool visible);
  fn _ZN10QTextBlock10setVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTextBlock::clearLayout();
  fn _ZN10QTextBlock11clearLayoutEv(qthis: u64 /* *mut c_void*/);
  // proto:  QTextDocumentPrivate * QTextBlock::docHandle();
  fn demth_ZNK10QTextBlock9docHandleEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QTextBlock::userState();
  fn _ZNK10QTextBlock9userStateEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QTextBlock::charFormatIndex();
  fn _ZNK10QTextBlock15charFormatIndexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QTextBlock::revision();
  fn _ZNK10QTextBlock8revisionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QTextBlock::position();
  fn _ZNK10QTextBlock8positionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QTextBlock::isValid();
  fn _ZNK10QTextBlock7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QTextList * QTextBlock::textList();
  fn _ZNK10QTextBlock8textListEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTextLayout * QTextBlock::layout();
  fn _ZNK10QTextBlock6layoutEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextBlock::setUserData(QTextBlockUserData * data);
  fn _ZN10QTextBlock11setUserDataEP18QTextBlockUserData(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QTextBlock::blockFormatIndex();
  fn _ZNK10QTextBlock16blockFormatIndexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextBlock::setUserState(int state);
  fn _ZN10QTextBlock12setUserStateEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QTextBlock::fragmentIndex();
  fn demth_ZNK10QTextBlock13fragmentIndexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QTextBlock::isVisible();
  fn _ZNK10QTextBlock9isVisibleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTextBlock::setLineCount(int count);
  fn _ZN10QTextBlock12setLineCountEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QTextBlock QTextBlock::next();
  fn _ZNK10QTextBlock4nextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTextBlockFormat QTextBlock::blockFormat();
  fn _ZNK10QTextBlock11blockFormatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextBlock::QTextBlock();
  fn dector_ZN10QTextBlockC1Ev() -> *mut c_void;
  fn demth_ZN10QTextBlockC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QTextBlock::firstLineNumber();
  fn _ZNK10QTextBlock15firstLineNumberEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QTextCharFormat QTextBlock::charFormat();
  fn _ZNK10QTextBlock10charFormatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QTextBlockGroup_Class_Size() -> c_int;
  // proto:  void QTextBlockGroup::QTextBlockGroup(const QTextBlockGroup & );
  fn dector_ZN15QTextBlockGroupC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN15QTextBlockGroupC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QTextBlockGroup::metaObject();
  fn _ZNK15QTextBlockGroup10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextBlockGroup::~QTextBlockGroup();
  fn _ZN15QTextBlockGroupD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextBlockGroup::QTextBlockGroup(QTextDocument * doc);
  fn dector_ZN15QTextBlockGroupC1EP13QTextDocument(arg0: *mut c_void) -> *mut c_void;
  fn _ZN15QTextBlockGroupC1EP13QTextDocument(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QTextFrame_Class_Size() -> c_int;
  // proto:  QTextFrameFormat QTextFrame::frameFormat();
  fn _ZNK10QTextFrame11frameFormatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTextFrameLayoutData * QTextFrame::layoutData();
  fn _ZNK10QTextFrame10layoutDataEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextFrame::setLayoutData(QTextFrameLayoutData * data);
  fn _ZN10QTextFrame13setLayoutDataEP20QTextFrameLayoutData(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextFrame::setFrameFormat(const QTextFrameFormat & format);
  fn demth_ZN10QTextFrame14setFrameFormatERK16QTextFrameFormat(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextFrame::QTextFrame(const QTextFrame & );
  fn dector_ZN10QTextFrameC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN10QTextFrameC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QTextFrame::metaObject();
  fn _ZNK10QTextFrame10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QTextFrame * QTextFrame::parentFrame();
  fn _ZNK10QTextFrame11parentFrameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTextFrame::firstPosition();
  fn _ZNK10QTextFrame13firstPositionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QList<QTextFrame *> QTextFrame::childFrames();
  fn _ZNK10QTextFrame11childFramesEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextFrame::~QTextFrame();
  fn _ZN10QTextFrameD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QTextCursor QTextFrame::lastCursorPosition();
  fn _ZNK10QTextFrame18lastCursorPositionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextFrame::QTextFrame(QTextDocument * doc);
  fn dector_ZN10QTextFrameC1EP13QTextDocument(arg0: *mut c_void) -> *mut c_void;
  fn _ZN10QTextFrameC1EP13QTextDocument(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QTextFrame::lastPosition();
  fn _ZNK10QTextFrame12lastPositionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QTextCursor QTextFrame::firstCursorPosition();
  fn _ZNK10QTextFrame19firstCursorPositionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QTextObject)=1
#[derive(Default)]
pub struct QTextObject {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QTextBlockUserData)=8
#[derive(Default)]
pub struct QTextBlockUserData {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QTextFragment)=16
#[derive(Default)]
pub struct QTextFragment {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QTextFrameLayoutData)=8
#[derive(Default)]
pub struct QTextFrameLayoutData {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QTextBlock)=16
#[derive(Default)]
pub struct QTextBlock {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QTextBlockGroup)=1
#[derive(Default)]
pub struct QTextBlockGroup {
  qbase: QTextObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QTextFrame)=1
#[derive(Default)]
pub struct QTextFrame {
  qbase: QTextObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTextObject {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextObject {
    return QTextObject{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTextObject {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QTextObject {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  QTextDocumentPrivate * QTextObject::docHandle();
impl /*struct*/ QTextObject {
  pub fn docHandle<RetType, T: QTextObject_docHandle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.docHandle(self);
    // return 1;
  }
}

pub trait QTextObject_docHandle<RetType> {
  fn docHandle(self , rsthis: & QTextObject) -> RetType;
}

  // proto:  QTextDocumentPrivate * QTextObject::docHandle();
impl<'a> /*trait*/ QTextObject_docHandle<()> for () {
  fn docHandle(self , rsthis: & QTextObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextObject9docHandleEv()};
     unsafe {_ZNK11QTextObject9docHandleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextObject::~QTextObject();
impl /*struct*/ QTextObject {
  pub fn Free<RetType, T: QTextObject_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTextObject_Free<RetType> {
  fn Free(self , rsthis: & QTextObject) -> RetType;
}

  // proto:  void QTextObject::~QTextObject();
impl<'a> /*trait*/ QTextObject_Free<()> for () {
  fn Free(self , rsthis: & QTextObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextObjectD0Ev()};
     unsafe {_ZN11QTextObjectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextObject::QTextObject(const QTextObject & );
impl /*struct*/ QTextObject {
  pub fn New<T: QTextObject_New>(value: T) -> QTextObject {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTextObject_New {
  fn New(self) -> QTextObject;
}

  // proto:  void QTextObject::QTextObject(const QTextObject & );
impl<'a> /*trait*/ QTextObject_New for (&'a QTextObject) {
  fn New(self) -> QTextObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextObjectC1ERKS_()};
    let ctysz: c_int = unsafe{QTextObject_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QTextObjectC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN11QTextObjectC1ERKS_(arg0)} as u64;
    let rsthis = QTextObject{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QTextFormat QTextObject::format();
impl /*struct*/ QTextObject {
  pub fn format<RetType, T: QTextObject_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QTextObject_format<RetType> {
  fn format(self , rsthis: & QTextObject) -> RetType;
}

  // proto:  QTextFormat QTextObject::format();
impl<'a> /*trait*/ QTextObject_format<QTextFormat> for () {
  fn format(self , rsthis: & QTextObject) -> QTextFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextObject6formatEv()};
    let mut ret = unsafe {_ZNK11QTextObject6formatEv(rsthis.qclsinst)};
    let mut ret1 = QTextFormat::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextObject::formatIndex();
impl /*struct*/ QTextObject {
  pub fn formatIndex<RetType, T: QTextObject_formatIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.formatIndex(self);
    // return 1;
  }
}

pub trait QTextObject_formatIndex<RetType> {
  fn formatIndex(self , rsthis: & QTextObject) -> RetType;
}

  // proto:  int QTextObject::formatIndex();
impl<'a> /*trait*/ QTextObject_formatIndex<i32> for () {
  fn formatIndex(self , rsthis: & QTextObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextObject11formatIndexEv()};
    let mut ret = unsafe {_ZNK11QTextObject11formatIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QTextDocument * QTextObject::document();
impl /*struct*/ QTextObject {
  pub fn document<RetType, T: QTextObject_document<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.document(self);
    // return 1;
  }
}

pub trait QTextObject_document<RetType> {
  fn document(self , rsthis: & QTextObject) -> RetType;
}

  // proto:  QTextDocument * QTextObject::document();
impl<'a> /*trait*/ QTextObject_document<QTextDocument> for () {
  fn document(self , rsthis: & QTextObject) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextObject8documentEv()};
    let mut ret = unsafe {_ZNK11QTextObject8documentEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocument::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextObject::objectIndex();
impl /*struct*/ QTextObject {
  pub fn objectIndex<RetType, T: QTextObject_objectIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.objectIndex(self);
    // return 1;
  }
}

pub trait QTextObject_objectIndex<RetType> {
  fn objectIndex(self , rsthis: & QTextObject) -> RetType;
}

  // proto:  int QTextObject::objectIndex();
impl<'a> /*trait*/ QTextObject_objectIndex<i32> for () {
  fn objectIndex(self , rsthis: & QTextObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextObject11objectIndexEv()};
    let mut ret = unsafe {_ZNK11QTextObject11objectIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextObject::QTextObject(QTextDocument * doc);
impl<'a> /*trait*/ QTextObject_New for (&'a QTextDocument) {
  fn New(self) -> QTextObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextObjectC1EP13QTextDocument()};
    let ctysz: c_int = unsafe{QTextObject_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QTextObjectC1EP13QTextDocument(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN11QTextObjectC1EP13QTextDocument(arg0)} as u64;
    let rsthis = QTextObject{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QTextObject::metaObject();
impl /*struct*/ QTextObject {
  pub fn metaObject<RetType, T: QTextObject_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTextObject_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTextObject) -> RetType;
}

  // proto:  const QMetaObject * QTextObject::metaObject();
impl<'a> /*trait*/ QTextObject_metaObject<()> for () {
  fn metaObject(self , rsthis: & QTextObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextObject10metaObjectEv()};
     unsafe {_ZNK11QTextObject10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextBlockUserData {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextBlockUserData {
    return QTextBlockUserData{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QTextBlockUserData::~QTextBlockUserData();
impl /*struct*/ QTextBlockUserData {
  pub fn Free<RetType, T: QTextBlockUserData_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTextBlockUserData_Free<RetType> {
  fn Free(self , rsthis: & QTextBlockUserData) -> RetType;
}

  // proto:  void QTextBlockUserData::~QTextBlockUserData();
impl<'a> /*trait*/ QTextBlockUserData_Free<()> for () {
  fn Free(self , rsthis: & QTextBlockUserData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QTextBlockUserDataD0Ev()};
     unsafe {_ZN18QTextBlockUserDataD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextFragment {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextFragment {
    return QTextFragment{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  int QTextFragment::charFormatIndex();
impl /*struct*/ QTextFragment {
  pub fn charFormatIndex<RetType, T: QTextFragment_charFormatIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.charFormatIndex(self);
    // return 1;
  }
}

pub trait QTextFragment_charFormatIndex<RetType> {
  fn charFormatIndex(self , rsthis: & QTextFragment) -> RetType;
}

  // proto:  int QTextFragment::charFormatIndex();
impl<'a> /*trait*/ QTextFragment_charFormatIndex<i32> for () {
  fn charFormatIndex(self , rsthis: & QTextFragment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextFragment15charFormatIndexEv()};
    let mut ret = unsafe {_ZNK13QTextFragment15charFormatIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTextFragment::position();
impl /*struct*/ QTextFragment {
  pub fn position<RetType, T: QTextFragment_position<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QTextFragment_position<RetType> {
  fn position(self , rsthis: & QTextFragment) -> RetType;
}

  // proto:  int QTextFragment::position();
impl<'a> /*trait*/ QTextFragment_position<i32> for () {
  fn position(self , rsthis: & QTextFragment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextFragment8positionEv()};
    let mut ret = unsafe {_ZNK13QTextFragment8positionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextFragment::QTextFragment(const QTextFragment & o);
impl /*struct*/ QTextFragment {
  pub fn New<T: QTextFragment_New>(value: T) -> QTextFragment {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTextFragment_New {
  fn New(self) -> QTextFragment;
}

  // proto:  void QTextFragment::QTextFragment(const QTextFragment & o);
impl<'a> /*trait*/ QTextFragment_New for (&'a QTextFragment) {
  fn New(self) -> QTextFragment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextFragmentC1ERKS_()};
    let ctysz: c_int = unsafe{QTextFragment_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QTextFragmentC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN13QTextFragmentC1ERKS_(arg0)} as u64;
    let rsthis = QTextFragment{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTextFragment::contains(int position);
impl /*struct*/ QTextFragment {
  pub fn contains<RetType, T: QTextFragment_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QTextFragment_contains<RetType> {
  fn contains(self , rsthis: & QTextFragment) -> RetType;
}

  // proto:  bool QTextFragment::contains(int position);
impl<'a> /*trait*/ QTextFragment_contains<i8> for (i32) {
  fn contains(self , rsthis: & QTextFragment) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextFragment8containsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QTextFragment8containsEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextFragment::QTextFragment();
impl<'a> /*trait*/ QTextFragment_New for () {
  fn New(self) -> QTextFragment {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextFragmentC1Ev()};
    let ctysz: c_int = unsafe{QTextFragment_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN13QTextFragmentC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN13QTextFragmentC1Ev()} as u64;
    let rsthis = QTextFragment{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QTextFragment::text();
impl /*struct*/ QTextFragment {
  pub fn text<RetType, T: QTextFragment_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QTextFragment_text<RetType> {
  fn text(self , rsthis: & QTextFragment) -> RetType;
}

  // proto:  QString QTextFragment::text();
impl<'a> /*trait*/ QTextFragment_text<QString> for () {
  fn text(self , rsthis: & QTextFragment) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextFragment4textEv()};
    let mut ret = unsafe {_ZNK13QTextFragment4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QGlyphRun> QTextFragment::glyphRuns(int from, int length);
impl /*struct*/ QTextFragment {
  pub fn glyphRuns<RetType, T: QTextFragment_glyphRuns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glyphRuns(self);
    // return 1;
  }
}

pub trait QTextFragment_glyphRuns<RetType> {
  fn glyphRuns(self , rsthis: & QTextFragment) -> RetType;
}

  // proto:  QList<QGlyphRun> QTextFragment::glyphRuns(int from, int length);
impl<'a> /*trait*/ QTextFragment_glyphRuns<()> for (i32, i32) {
  fn glyphRuns(self , rsthis: & QTextFragment) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextFragment9glyphRunsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZNK13QTextFragment9glyphRunsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QTextFragment::isValid();
impl /*struct*/ QTextFragment {
  pub fn isValid<RetType, T: QTextFragment_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextFragment_isValid<RetType> {
  fn isValid(self , rsthis: & QTextFragment) -> RetType;
}

  // proto:  bool QTextFragment::isValid();
impl<'a> /*trait*/ QTextFragment_isValid<i8> for () {
  fn isValid(self , rsthis: & QTextFragment) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextFragment7isValidEv()};
    let mut ret = unsafe {demth_ZNK13QTextFragment7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QTextCharFormat QTextFragment::charFormat();
impl /*struct*/ QTextFragment {
  pub fn charFormat<RetType, T: QTextFragment_charFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.charFormat(self);
    // return 1;
  }
}

pub trait QTextFragment_charFormat<RetType> {
  fn charFormat(self , rsthis: & QTextFragment) -> RetType;
}

  // proto:  QTextCharFormat QTextFragment::charFormat();
impl<'a> /*trait*/ QTextFragment_charFormat<QTextCharFormat> for () {
  fn charFormat(self , rsthis: & QTextFragment) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextFragment10charFormatEv()};
    let mut ret = unsafe {_ZNK13QTextFragment10charFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextFragment::length();
impl /*struct*/ QTextFragment {
  pub fn length<RetType, T: QTextFragment_length<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.length(self);
    // return 1;
  }
}

pub trait QTextFragment_length<RetType> {
  fn length(self , rsthis: & QTextFragment) -> RetType;
}

  // proto:  int QTextFragment::length();
impl<'a> /*trait*/ QTextFragment_length<i32> for () {
  fn length(self , rsthis: & QTextFragment) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextFragment6lengthEv()};
    let mut ret = unsafe {_ZNK13QTextFragment6lengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextFrameLayoutData {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextFrameLayoutData {
    return QTextFrameLayoutData{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QTextFrameLayoutData::~QTextFrameLayoutData();
impl /*struct*/ QTextFrameLayoutData {
  pub fn Free<RetType, T: QTextFrameLayoutData_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTextFrameLayoutData_Free<RetType> {
  fn Free(self , rsthis: & QTextFrameLayoutData) -> RetType;
}

  // proto:  void QTextFrameLayoutData::~QTextFrameLayoutData();
impl<'a> /*trait*/ QTextFrameLayoutData_Free<()> for () {
  fn Free(self , rsthis: & QTextFrameLayoutData) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextFrameLayoutDataD0Ev()};
     unsafe {_ZN20QTextFrameLayoutDataD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextBlock {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextBlock {
    return QTextBlock{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  const QTextDocument * QTextBlock::document();
impl /*struct*/ QTextBlock {
  pub fn document<RetType, T: QTextBlock_document<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.document(self);
    // return 1;
  }
}

pub trait QTextBlock_document<RetType> {
  fn document(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  const QTextDocument * QTextBlock::document();
impl<'a> /*trait*/ QTextBlock_document<QTextDocument> for () {
  fn document(self , rsthis: & QTextBlock) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8documentEv()};
    let mut ret = unsafe {_ZNK10QTextBlock8documentEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocument::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextBlock QTextBlock::previous();
impl /*struct*/ QTextBlock {
  pub fn previous<RetType, T: QTextBlock_previous<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.previous(self);
    // return 1;
  }
}

pub trait QTextBlock_previous<RetType> {
  fn previous(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  QTextBlock QTextBlock::previous();
impl<'a> /*trait*/ QTextBlock_previous<QTextBlock> for () {
  fn previous(self , rsthis: & QTextBlock) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8previousEv()};
    let mut ret = unsafe {_ZNK10QTextBlock8previousEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextBlock::length();
impl /*struct*/ QTextBlock {
  pub fn length<RetType, T: QTextBlock_length<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.length(self);
    // return 1;
  }
}

pub trait QTextBlock_length<RetType> {
  fn length(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::length();
impl<'a> /*trait*/ QTextBlock_length<i32> for () {
  fn length(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock6lengthEv()};
    let mut ret = unsafe {_ZNK10QTextBlock6lengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QTextBlockUserData * QTextBlock::userData();
impl /*struct*/ QTextBlock {
  pub fn userData<RetType, T: QTextBlock_userData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.userData(self);
    // return 1;
  }
}

pub trait QTextBlock_userData<RetType> {
  fn userData(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  QTextBlockUserData * QTextBlock::userData();
impl<'a> /*trait*/ QTextBlock_userData<QTextBlockUserData> for () {
  fn userData(self , rsthis: & QTextBlock) -> QTextBlockUserData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8userDataEv()};
    let mut ret = unsafe {_ZNK10QTextBlock8userDataEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlockUserData::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextBlock::QTextBlock(const QTextBlock & o);
impl /*struct*/ QTextBlock {
  pub fn New<T: QTextBlock_New>(value: T) -> QTextBlock {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBlock_New {
  fn New(self) -> QTextBlock;
}

  // proto:  void QTextBlock::QTextBlock(const QTextBlock & o);
impl<'a> /*trait*/ QTextBlock_New for (&'a QTextBlock) {
  fn New(self) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlockC1ERKS_()};
    let ctysz: c_int = unsafe{QTextBlock_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QTextBlockC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN10QTextBlockC1ERKS_(arg0)} as u64;
    let rsthis = QTextBlock{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QTextBlock::text();
impl /*struct*/ QTextBlock {
  pub fn text<RetType, T: QTextBlock_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QTextBlock_text<RetType> {
  fn text(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  QString QTextBlock::text();
impl<'a> /*trait*/ QTextBlock_text<QString> for () {
  fn text(self , rsthis: & QTextBlock) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock4textEv()};
    let mut ret = unsafe {_ZNK10QTextBlock4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextBlock::lineCount();
impl /*struct*/ QTextBlock {
  pub fn lineCount<RetType, T: QTextBlock_lineCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineCount(self);
    // return 1;
  }
}

pub trait QTextBlock_lineCount<RetType> {
  fn lineCount(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::lineCount();
impl<'a> /*trait*/ QTextBlock_lineCount<i32> for () {
  fn lineCount(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock9lineCountEv()};
    let mut ret = unsafe {_ZNK10QTextBlock9lineCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QTextBlock::contains(int position);
impl /*struct*/ QTextBlock {
  pub fn contains<RetType, T: QTextBlock_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QTextBlock_contains<RetType> {
  fn contains(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  bool QTextBlock::contains(int position);
impl<'a> /*trait*/ QTextBlock_contains<i8> for (i32) {
  fn contains(self , rsthis: & QTextBlock) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8containsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTextBlock8containsEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QTextBlock::blockNumber();
impl /*struct*/ QTextBlock {
  pub fn blockNumber<RetType, T: QTextBlock_blockNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.blockNumber(self);
    // return 1;
  }
}

pub trait QTextBlock_blockNumber<RetType> {
  fn blockNumber(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::blockNumber();
impl<'a> /*trait*/ QTextBlock_blockNumber<i32> for () {
  fn blockNumber(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock11blockNumberEv()};
    let mut ret = unsafe {_ZNK10QTextBlock11blockNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextBlock::setRevision(int rev);
impl /*struct*/ QTextBlock {
  pub fn setRevision<RetType, T: QTextBlock_setRevision<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRevision(self);
    // return 1;
  }
}

pub trait QTextBlock_setRevision<RetType> {
  fn setRevision(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  void QTextBlock::setRevision(int rev);
impl<'a> /*trait*/ QTextBlock_setRevision<()> for (i32) {
  fn setRevision(self , rsthis: & QTextBlock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock11setRevisionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTextBlock11setRevisionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextBlock::setVisible(bool visible);
impl /*struct*/ QTextBlock {
  pub fn setVisible<RetType, T: QTextBlock_setVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QTextBlock_setVisible<RetType> {
  fn setVisible(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  void QTextBlock::setVisible(bool visible);
impl<'a> /*trait*/ QTextBlock_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: & QTextBlock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QTextBlock10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextBlock::clearLayout();
impl /*struct*/ QTextBlock {
  pub fn clearLayout<RetType, T: QTextBlock_clearLayout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearLayout(self);
    // return 1;
  }
}

pub trait QTextBlock_clearLayout<RetType> {
  fn clearLayout(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  void QTextBlock::clearLayout();
impl<'a> /*trait*/ QTextBlock_clearLayout<()> for () {
  fn clearLayout(self , rsthis: & QTextBlock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock11clearLayoutEv()};
     unsafe {_ZN10QTextBlock11clearLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QTextDocumentPrivate * QTextBlock::docHandle();
impl /*struct*/ QTextBlock {
  pub fn docHandle<RetType, T: QTextBlock_docHandle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.docHandle(self);
    // return 1;
  }
}

pub trait QTextBlock_docHandle<RetType> {
  fn docHandle(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  QTextDocumentPrivate * QTextBlock::docHandle();
impl<'a> /*trait*/ QTextBlock_docHandle<()> for () {
  fn docHandle(self , rsthis: & QTextBlock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock9docHandleEv()};
     unsafe {demth_ZNK10QTextBlock9docHandleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTextBlock::userState();
impl /*struct*/ QTextBlock {
  pub fn userState<RetType, T: QTextBlock_userState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.userState(self);
    // return 1;
  }
}

pub trait QTextBlock_userState<RetType> {
  fn userState(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::userState();
impl<'a> /*trait*/ QTextBlock_userState<i32> for () {
  fn userState(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock9userStateEv()};
    let mut ret = unsafe {_ZNK10QTextBlock9userStateEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTextBlock::charFormatIndex();
impl /*struct*/ QTextBlock {
  pub fn charFormatIndex<RetType, T: QTextBlock_charFormatIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.charFormatIndex(self);
    // return 1;
  }
}

pub trait QTextBlock_charFormatIndex<RetType> {
  fn charFormatIndex(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::charFormatIndex();
impl<'a> /*trait*/ QTextBlock_charFormatIndex<i32> for () {
  fn charFormatIndex(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock15charFormatIndexEv()};
    let mut ret = unsafe {_ZNK10QTextBlock15charFormatIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTextBlock::revision();
impl /*struct*/ QTextBlock {
  pub fn revision<RetType, T: QTextBlock_revision<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.revision(self);
    // return 1;
  }
}

pub trait QTextBlock_revision<RetType> {
  fn revision(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::revision();
impl<'a> /*trait*/ QTextBlock_revision<i32> for () {
  fn revision(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8revisionEv()};
    let mut ret = unsafe {_ZNK10QTextBlock8revisionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTextBlock::position();
impl /*struct*/ QTextBlock {
  pub fn position<RetType, T: QTextBlock_position<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QTextBlock_position<RetType> {
  fn position(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::position();
impl<'a> /*trait*/ QTextBlock_position<i32> for () {
  fn position(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8positionEv()};
    let mut ret = unsafe {_ZNK10QTextBlock8positionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QTextBlock::isValid();
impl /*struct*/ QTextBlock {
  pub fn isValid<RetType, T: QTextBlock_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextBlock_isValid<RetType> {
  fn isValid(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  bool QTextBlock::isValid();
impl<'a> /*trait*/ QTextBlock_isValid<i8> for () {
  fn isValid(self , rsthis: & QTextBlock) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock7isValidEv()};
    let mut ret = unsafe {_ZNK10QTextBlock7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QTextList * QTextBlock::textList();
impl /*struct*/ QTextBlock {
  pub fn textList<RetType, T: QTextBlock_textList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textList(self);
    // return 1;
  }
}

pub trait QTextBlock_textList<RetType> {
  fn textList(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  QTextList * QTextBlock::textList();
impl<'a> /*trait*/ QTextBlock_textList<QTextList> for () {
  fn textList(self , rsthis: & QTextBlock) -> QTextList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock8textListEv()};
    let mut ret = unsafe {_ZNK10QTextBlock8textListEv(rsthis.qclsinst)};
    let mut ret1 = QTextList::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextLayout * QTextBlock::layout();
impl /*struct*/ QTextBlock {
  pub fn layout<RetType, T: QTextBlock_layout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.layout(self);
    // return 1;
  }
}

pub trait QTextBlock_layout<RetType> {
  fn layout(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  QTextLayout * QTextBlock::layout();
impl<'a> /*trait*/ QTextBlock_layout<QTextLayout> for () {
  fn layout(self , rsthis: & QTextBlock) -> QTextLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock6layoutEv()};
    let mut ret = unsafe {_ZNK10QTextBlock6layoutEv(rsthis.qclsinst)};
    let mut ret1 = QTextLayout::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextBlock::setUserData(QTextBlockUserData * data);
impl /*struct*/ QTextBlock {
  pub fn setUserData<RetType, T: QTextBlock_setUserData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUserData(self);
    // return 1;
  }
}

pub trait QTextBlock_setUserData<RetType> {
  fn setUserData(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  void QTextBlock::setUserData(QTextBlockUserData * data);
impl<'a> /*trait*/ QTextBlock_setUserData<()> for (&'a QTextBlockUserData) {
  fn setUserData(self , rsthis: & QTextBlock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock11setUserDataEP18QTextBlockUserData()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTextBlock11setUserDataEP18QTextBlockUserData(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextBlock::blockFormatIndex();
impl /*struct*/ QTextBlock {
  pub fn blockFormatIndex<RetType, T: QTextBlock_blockFormatIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.blockFormatIndex(self);
    // return 1;
  }
}

pub trait QTextBlock_blockFormatIndex<RetType> {
  fn blockFormatIndex(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::blockFormatIndex();
impl<'a> /*trait*/ QTextBlock_blockFormatIndex<i32> for () {
  fn blockFormatIndex(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock16blockFormatIndexEv()};
    let mut ret = unsafe {_ZNK10QTextBlock16blockFormatIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextBlock::setUserState(int state);
impl /*struct*/ QTextBlock {
  pub fn setUserState<RetType, T: QTextBlock_setUserState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUserState(self);
    // return 1;
  }
}

pub trait QTextBlock_setUserState<RetType> {
  fn setUserState(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  void QTextBlock::setUserState(int state);
impl<'a> /*trait*/ QTextBlock_setUserState<()> for (i32) {
  fn setUserState(self , rsthis: & QTextBlock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock12setUserStateEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTextBlock12setUserStateEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextBlock::fragmentIndex();
impl /*struct*/ QTextBlock {
  pub fn fragmentIndex<RetType, T: QTextBlock_fragmentIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fragmentIndex(self);
    // return 1;
  }
}

pub trait QTextBlock_fragmentIndex<RetType> {
  fn fragmentIndex(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::fragmentIndex();
impl<'a> /*trait*/ QTextBlock_fragmentIndex<i32> for () {
  fn fragmentIndex(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock13fragmentIndexEv()};
    let mut ret = unsafe {demth_ZNK10QTextBlock13fragmentIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QTextBlock::isVisible();
impl /*struct*/ QTextBlock {
  pub fn isVisible<RetType, T: QTextBlock_isVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isVisible(self);
    // return 1;
  }
}

pub trait QTextBlock_isVisible<RetType> {
  fn isVisible(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  bool QTextBlock::isVisible();
impl<'a> /*trait*/ QTextBlock_isVisible<i8> for () {
  fn isVisible(self , rsthis: & QTextBlock) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock9isVisibleEv()};
    let mut ret = unsafe {_ZNK10QTextBlock9isVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextBlock::setLineCount(int count);
impl /*struct*/ QTextBlock {
  pub fn setLineCount<RetType, T: QTextBlock_setLineCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLineCount(self);
    // return 1;
  }
}

pub trait QTextBlock_setLineCount<RetType> {
  fn setLineCount(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  void QTextBlock::setLineCount(int count);
impl<'a> /*trait*/ QTextBlock_setLineCount<()> for (i32) {
  fn setLineCount(self , rsthis: & QTextBlock) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlock12setLineCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTextBlock12setLineCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextBlock QTextBlock::next();
impl /*struct*/ QTextBlock {
  pub fn next<RetType, T: QTextBlock_next<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.next(self);
    // return 1;
  }
}

pub trait QTextBlock_next<RetType> {
  fn next(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  QTextBlock QTextBlock::next();
impl<'a> /*trait*/ QTextBlock_next<QTextBlock> for () {
  fn next(self , rsthis: & QTextBlock) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock4nextEv()};
    let mut ret = unsafe {_ZNK10QTextBlock4nextEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextBlockFormat QTextBlock::blockFormat();
impl /*struct*/ QTextBlock {
  pub fn blockFormat<RetType, T: QTextBlock_blockFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.blockFormat(self);
    // return 1;
  }
}

pub trait QTextBlock_blockFormat<RetType> {
  fn blockFormat(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  QTextBlockFormat QTextBlock::blockFormat();
impl<'a> /*trait*/ QTextBlock_blockFormat<QTextBlockFormat> for () {
  fn blockFormat(self , rsthis: & QTextBlock) -> QTextBlockFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock11blockFormatEv()};
    let mut ret = unsafe {_ZNK10QTextBlock11blockFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlockFormat::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextBlock::QTextBlock();
impl<'a> /*trait*/ QTextBlock_New for () {
  fn New(self) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextBlockC1Ev()};
    let ctysz: c_int = unsafe{QTextBlock_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN10QTextBlockC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN10QTextBlockC1Ev()} as u64;
    let rsthis = QTextBlock{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QTextBlock::firstLineNumber();
impl /*struct*/ QTextBlock {
  pub fn firstLineNumber<RetType, T: QTextBlock_firstLineNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.firstLineNumber(self);
    // return 1;
  }
}

pub trait QTextBlock_firstLineNumber<RetType> {
  fn firstLineNumber(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  int QTextBlock::firstLineNumber();
impl<'a> /*trait*/ QTextBlock_firstLineNumber<i32> for () {
  fn firstLineNumber(self , rsthis: & QTextBlock) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock15firstLineNumberEv()};
    let mut ret = unsafe {_ZNK10QTextBlock15firstLineNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QTextCharFormat QTextBlock::charFormat();
impl /*struct*/ QTextBlock {
  pub fn charFormat<RetType, T: QTextBlock_charFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.charFormat(self);
    // return 1;
  }
}

pub trait QTextBlock_charFormat<RetType> {
  fn charFormat(self , rsthis: & QTextBlock) -> RetType;
}

  // proto:  QTextCharFormat QTextBlock::charFormat();
impl<'a> /*trait*/ QTextBlock_charFormat<QTextCharFormat> for () {
  fn charFormat(self , rsthis: & QTextBlock) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextBlock10charFormatEv()};
    let mut ret = unsafe {_ZNK10QTextBlock10charFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextBlockGroup {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextBlockGroup {
    return QTextBlockGroup{qbase: QTextObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTextBlockGroup {
  type Target = QTextObject;

  fn deref(&self) -> &QTextObject {
    return & self.qbase;
  }
}
impl AsRef<QTextObject> for QTextBlockGroup {
  fn as_ref(& self) -> & QTextObject {
    return & self.qbase;
  }
}
  // proto:  void QTextBlockGroup::QTextBlockGroup(const QTextBlockGroup & );
impl /*struct*/ QTextBlockGroup {
  pub fn New<T: QTextBlockGroup_New>(value: T) -> QTextBlockGroup {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBlockGroup_New {
  fn New(self) -> QTextBlockGroup;
}

  // proto:  void QTextBlockGroup::QTextBlockGroup(const QTextBlockGroup & );
impl<'a> /*trait*/ QTextBlockGroup_New for (&'a QTextBlockGroup) {
  fn New(self) -> QTextBlockGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextBlockGroupC1ERKS_()};
    let ctysz: c_int = unsafe{QTextBlockGroup_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN15QTextBlockGroupC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN15QTextBlockGroupC1ERKS_(arg0)} as u64;
    let rsthis = QTextBlockGroup{qbase: QTextObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QTextBlockGroup::metaObject();
impl /*struct*/ QTextBlockGroup {
  pub fn metaObject<RetType, T: QTextBlockGroup_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTextBlockGroup_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTextBlockGroup) -> RetType;
}

  // proto:  const QMetaObject * QTextBlockGroup::metaObject();
impl<'a> /*trait*/ QTextBlockGroup_metaObject<()> for () {
  fn metaObject(self , rsthis: & QTextBlockGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextBlockGroup10metaObjectEv()};
     unsafe {_ZNK15QTextBlockGroup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextBlockGroup::~QTextBlockGroup();
impl /*struct*/ QTextBlockGroup {
  pub fn Free<RetType, T: QTextBlockGroup_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTextBlockGroup_Free<RetType> {
  fn Free(self , rsthis: & QTextBlockGroup) -> RetType;
}

  // proto:  void QTextBlockGroup::~QTextBlockGroup();
impl<'a> /*trait*/ QTextBlockGroup_Free<()> for () {
  fn Free(self , rsthis: & QTextBlockGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextBlockGroupD0Ev()};
     unsafe {_ZN15QTextBlockGroupD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextBlockGroup::QTextBlockGroup(QTextDocument * doc);
impl<'a> /*trait*/ QTextBlockGroup_New for (&'a QTextDocument) {
  fn New(self) -> QTextBlockGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextBlockGroupC1EP13QTextDocument()};
    let ctysz: c_int = unsafe{QTextBlockGroup_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN15QTextBlockGroupC1EP13QTextDocument(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN15QTextBlockGroupC1EP13QTextDocument(arg0)} as u64;
    let rsthis = QTextBlockGroup{qbase: QTextObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextFrame {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextFrame {
    return QTextFrame{qbase: QTextObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTextFrame {
  type Target = QTextObject;

  fn deref(&self) -> &QTextObject {
    return & self.qbase;
  }
}
impl AsRef<QTextObject> for QTextFrame {
  fn as_ref(& self) -> & QTextObject {
    return & self.qbase;
  }
}
  // proto:  QTextFrameFormat QTextFrame::frameFormat();
impl /*struct*/ QTextFrame {
  pub fn frameFormat<RetType, T: QTextFrame_frameFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.frameFormat(self);
    // return 1;
  }
}

pub trait QTextFrame_frameFormat<RetType> {
  fn frameFormat(self , rsthis: & QTextFrame) -> RetType;
}

  // proto:  QTextFrameFormat QTextFrame::frameFormat();
impl<'a> /*trait*/ QTextFrame_frameFormat<QTextFrameFormat> for () {
  fn frameFormat(self , rsthis: & QTextFrame) -> QTextFrameFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame11frameFormatEv()};
    let mut ret = unsafe {_ZNK10QTextFrame11frameFormatEv(rsthis.qclsinst)};
    let mut ret1 = QTextFrameFormat::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextFrameLayoutData * QTextFrame::layoutData();
impl /*struct*/ QTextFrame {
  pub fn layoutData<RetType, T: QTextFrame_layoutData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.layoutData(self);
    // return 1;
  }
}

pub trait QTextFrame_layoutData<RetType> {
  fn layoutData(self , rsthis: & QTextFrame) -> RetType;
}

  // proto:  QTextFrameLayoutData * QTextFrame::layoutData();
impl<'a> /*trait*/ QTextFrame_layoutData<QTextFrameLayoutData> for () {
  fn layoutData(self , rsthis: & QTextFrame) -> QTextFrameLayoutData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame10layoutDataEv()};
    let mut ret = unsafe {_ZNK10QTextFrame10layoutDataEv(rsthis.qclsinst)};
    let mut ret1 = QTextFrameLayoutData::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextFrame::setLayoutData(QTextFrameLayoutData * data);
impl /*struct*/ QTextFrame {
  pub fn setLayoutData<RetType, T: QTextFrame_setLayoutData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLayoutData(self);
    // return 1;
  }
}

pub trait QTextFrame_setLayoutData<RetType> {
  fn setLayoutData(self , rsthis: & QTextFrame) -> RetType;
}

  // proto:  void QTextFrame::setLayoutData(QTextFrameLayoutData * data);
impl<'a> /*trait*/ QTextFrame_setLayoutData<()> for (&'a QTextFrameLayoutData) {
  fn setLayoutData(self , rsthis: & QTextFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextFrame13setLayoutDataEP20QTextFrameLayoutData()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTextFrame13setLayoutDataEP20QTextFrameLayoutData(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextFrame::setFrameFormat(const QTextFrameFormat & format);
impl /*struct*/ QTextFrame {
  pub fn setFrameFormat<RetType, T: QTextFrame_setFrameFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFrameFormat(self);
    // return 1;
  }
}

pub trait QTextFrame_setFrameFormat<RetType> {
  fn setFrameFormat(self , rsthis: & QTextFrame) -> RetType;
}

  // proto:  void QTextFrame::setFrameFormat(const QTextFrameFormat & format);
impl<'a> /*trait*/ QTextFrame_setFrameFormat<()> for (&'a QTextFrameFormat) {
  fn setFrameFormat(self , rsthis: & QTextFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextFrame14setFrameFormatERK16QTextFrameFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {demth_ZN10QTextFrame14setFrameFormatERK16QTextFrameFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextFrame::QTextFrame(const QTextFrame & );
impl /*struct*/ QTextFrame {
  pub fn New<T: QTextFrame_New>(value: T) -> QTextFrame {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTextFrame_New {
  fn New(self) -> QTextFrame;
}

  // proto:  void QTextFrame::QTextFrame(const QTextFrame & );
impl<'a> /*trait*/ QTextFrame_New for (&'a QTextFrame) {
  fn New(self) -> QTextFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextFrameC1ERKS_()};
    let ctysz: c_int = unsafe{QTextFrame_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QTextFrameC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN10QTextFrameC1ERKS_(arg0)} as u64;
    let rsthis = QTextFrame{qbase: QTextObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QTextFrame::metaObject();
impl /*struct*/ QTextFrame {
  pub fn metaObject<RetType, T: QTextFrame_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTextFrame_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTextFrame) -> RetType;
}

  // proto:  const QMetaObject * QTextFrame::metaObject();
impl<'a> /*trait*/ QTextFrame_metaObject<()> for () {
  fn metaObject(self , rsthis: & QTextFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame10metaObjectEv()};
     unsafe {_ZNK10QTextFrame10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QTextFrame * QTextFrame::parentFrame();
impl /*struct*/ QTextFrame {
  pub fn parentFrame<RetType, T: QTextFrame_parentFrame<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parentFrame(self);
    // return 1;
  }
}

pub trait QTextFrame_parentFrame<RetType> {
  fn parentFrame(self , rsthis: & QTextFrame) -> RetType;
}

  // proto:  QTextFrame * QTextFrame::parentFrame();
impl<'a> /*trait*/ QTextFrame_parentFrame<QTextFrame> for () {
  fn parentFrame(self , rsthis: & QTextFrame) -> QTextFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame11parentFrameEv()};
    let mut ret = unsafe {_ZNK10QTextFrame11parentFrameEv(rsthis.qclsinst)};
    let mut ret1 = QTextFrame::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextFrame::firstPosition();
impl /*struct*/ QTextFrame {
  pub fn firstPosition<RetType, T: QTextFrame_firstPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.firstPosition(self);
    // return 1;
  }
}

pub trait QTextFrame_firstPosition<RetType> {
  fn firstPosition(self , rsthis: & QTextFrame) -> RetType;
}

  // proto:  int QTextFrame::firstPosition();
impl<'a> /*trait*/ QTextFrame_firstPosition<i32> for () {
  fn firstPosition(self , rsthis: & QTextFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame13firstPositionEv()};
    let mut ret = unsafe {_ZNK10QTextFrame13firstPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QList<QTextFrame *> QTextFrame::childFrames();
impl /*struct*/ QTextFrame {
  pub fn childFrames<RetType, T: QTextFrame_childFrames<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.childFrames(self);
    // return 1;
  }
}

pub trait QTextFrame_childFrames<RetType> {
  fn childFrames(self , rsthis: & QTextFrame) -> RetType;
}

  // proto:  QList<QTextFrame *> QTextFrame::childFrames();
impl<'a> /*trait*/ QTextFrame_childFrames<()> for () {
  fn childFrames(self , rsthis: & QTextFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame11childFramesEv()};
     unsafe {_ZNK10QTextFrame11childFramesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextFrame::~QTextFrame();
impl /*struct*/ QTextFrame {
  pub fn Free<RetType, T: QTextFrame_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTextFrame_Free<RetType> {
  fn Free(self , rsthis: & QTextFrame) -> RetType;
}

  // proto:  void QTextFrame::~QTextFrame();
impl<'a> /*trait*/ QTextFrame_Free<()> for () {
  fn Free(self , rsthis: & QTextFrame) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextFrameD0Ev()};
     unsafe {_ZN10QTextFrameD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QTextCursor QTextFrame::lastCursorPosition();
impl /*struct*/ QTextFrame {
  pub fn lastCursorPosition<RetType, T: QTextFrame_lastCursorPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastCursorPosition(self);
    // return 1;
  }
}

pub trait QTextFrame_lastCursorPosition<RetType> {
  fn lastCursorPosition(self , rsthis: & QTextFrame) -> RetType;
}

  // proto:  QTextCursor QTextFrame::lastCursorPosition();
impl<'a> /*trait*/ QTextFrame_lastCursorPosition<QTextCursor> for () {
  fn lastCursorPosition(self , rsthis: & QTextFrame) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame18lastCursorPositionEv()};
    let mut ret = unsafe {_ZNK10QTextFrame18lastCursorPositionEv(rsthis.qclsinst)};
    let mut ret1 = QTextCursor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextFrame::QTextFrame(QTextDocument * doc);
impl<'a> /*trait*/ QTextFrame_New for (&'a QTextDocument) {
  fn New(self) -> QTextFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextFrameC1EP13QTextDocument()};
    let ctysz: c_int = unsafe{QTextFrame_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QTextFrameC1EP13QTextDocument(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN10QTextFrameC1EP13QTextDocument(arg0)} as u64;
    let rsthis = QTextFrame{qbase: QTextObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QTextFrame::lastPosition();
impl /*struct*/ QTextFrame {
  pub fn lastPosition<RetType, T: QTextFrame_lastPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastPosition(self);
    // return 1;
  }
}

pub trait QTextFrame_lastPosition<RetType> {
  fn lastPosition(self , rsthis: & QTextFrame) -> RetType;
}

  // proto:  int QTextFrame::lastPosition();
impl<'a> /*trait*/ QTextFrame_lastPosition<i32> for () {
  fn lastPosition(self , rsthis: & QTextFrame) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame12lastPositionEv()};
    let mut ret = unsafe {_ZNK10QTextFrame12lastPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QTextCursor QTextFrame::firstCursorPosition();
impl /*struct*/ QTextFrame {
  pub fn firstCursorPosition<RetType, T: QTextFrame_firstCursorPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.firstCursorPosition(self);
    // return 1;
  }
}

pub trait QTextFrame_firstCursorPosition<RetType> {
  fn firstCursorPosition(self , rsthis: & QTextFrame) -> RetType;
}

  // proto:  QTextCursor QTextFrame::firstCursorPosition();
impl<'a> /*trait*/ QTextFrame_firstCursorPosition<QTextCursor> for () {
  fn firstCursorPosition(self , rsthis: & QTextFrame) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextFrame19firstCursorPositionEv()};
    let mut ret = unsafe {_ZNK10QTextFrame19firstCursorPositionEv(rsthis.qclsinst)};
    let mut ret1 = QTextCursor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

