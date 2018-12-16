

// mod ::gui::QSessionManager
// package qtgui
// /usr/include/qt/QtGui/qsessionmanager.h
// #include <qsessionmanager.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 44
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QSessionManager)=16
pub struct QSessionManager {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSessionManager_ITF interface {
//    qtcore.QObject_ITF
//    QSessionManager_PTR() *QSessionManager
//}
//func (ptr *QSessionManager) QSessionManager_PTR() *QSessionManager { return ptr }

impl /*struct*/ QSessionManager {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSessionManager {
    return QSessionManager{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSessionManager {
//  type Target = QSessionManagerBASE;
//
//  fn deref(&self) -> &QSessionManagerBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSessionManagerBASE> for QSessionManager {
//  fn as_ref(& self) -> & QSessionManagerBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qsessionmanager.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QSessionManager {
  pub fn metaObject_0<RetType, T: QSessionManager_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QSessionManager_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QSessionManager) -> RetType;
}
impl<'a> /*trait*/ QSessionManager_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QSessionManager) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSessionManager10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsessionmanager.h:65
// index:0
// Public Visibility=Default Availability=Available
// [8] QString sessionId() const

/*
Returns the identifier of the current session.

If the application has been restored from an earlier session, this identifier is the same as it was in the earlier session.

See also sessionKey() and QGuiApplication::sessionId().
*/
impl /*struct*/ QSessionManager {
  pub fn sessionId_0<RetType, T: QSessionManager_sessionId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sessionId_0(self);
    // return 1;
  }
}
pub trait QSessionManager_sessionId_0<RetType> {
  fn sessionId_0(self , rsthis: & QSessionManager) -> RetType;
}
impl<'a> /*trait*/ QSessionManager_sessionId_0<usize> for () {
  fn sessionId_0(self , rsthis: & QSessionManager) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSessionManager9sessionIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsessionmanager.h:66
// index:0
// Public Visibility=Default Availability=Available
// [8] QString sessionKey() const

/*
Returns the session key in the current session.

If the application has been restored from an earlier session, this key is the same as it was when the previous session ended.

The session key changes with every call of commitData() or saveState().

See also sessionId() and QGuiApplication::sessionKey().
*/
impl /*struct*/ QSessionManager {
  pub fn sessionKey_0<RetType, T: QSessionManager_sessionKey_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sessionKey_0(self);
    // return 1;
  }
}
pub trait QSessionManager_sessionKey_0<RetType> {
  fn sessionKey_0(self , rsthis: & QSessionManager) -> RetType;
}
impl<'a> /*trait*/ QSessionManager_sessionKey_0<usize> for () {
  fn sessionKey_0(self , rsthis: & QSessionManager) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSessionManager10sessionKeyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsessionmanager.h:68
// index:0
// Public Visibility=Default Availability=Available
// [1] bool allowsInteraction()

/*
Asks the session manager for permission to interact with the user. Returns true if interaction is permitted; otherwise returns false.

The rationale behind this mechanism is to make it possible to synchronize user interaction during a shutdown. Advanced session managers may ask all applications simultaneously to commit their data, resulting in a much faster shutdown.

When the interaction is completed we strongly recommend releasing the user interaction semaphore with a call to release(). This way, other applications may get the chance to interact with the user while your application is still busy saving data. (The semaphore is implicitly released when the application exits.)

If the user decides to cancel the shutdown process during the interaction phase, you must tell the session manager that this has happened by calling cancel().

Here's an example of how an application's QGuiApplication::commitDataRequest() might be implemented:


  MyMainWidget::MyMainWidget(QWidget *parent)
      :QWidget(parent)
  {
      QGuiApplication::setFallbackSessionManagementEnabled(false);
      connect(qApp, SIGNAL(commitDataRequest(QSessionManager)), SLOT(commitData(QSessionManager)));
  }

  void MyMainWidget::commitData(QSessionManager& manager)
  {
      if (manager.allowsInteraction()) {
          int ret = QMessageBox::warning(
                      mainWindow,
                      tr("My Application"),
                      tr("Save changes to document?"),
                      QMessageBox::Save | QMessageBox::Discard | QMessageBox::Cancel);

          switch (ret) {
          case QMessageBox::Save:
              manager.release();
              if (!saveDocument())
                  manager.cancel();
              break;
          case QMessageBox::Discard:
              break;
          case QMessageBox::Cancel:
          default:
              manager.cancel();
          }
      } else {
          // we did not get permission to interact, then
          // do something reasonable instead
      }
  }



If an error occurred within the application while saving its data, you may want to try allowsErrorInteraction() instead.

See also QGuiApplication::commitDataRequest(), release(), and cancel().
*/
impl /*struct*/ QSessionManager {
  pub fn allowsInteraction_0<RetType, T: QSessionManager_allowsInteraction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.allowsInteraction_0(self);
    // return 1;
  }
}
pub trait QSessionManager_allowsInteraction_0<RetType> {
  fn allowsInteraction_0(self , rsthis: & QSessionManager) -> RetType;
}
impl<'a> /*trait*/ QSessionManager_allowsInteraction_0<bool> for () {
  fn allowsInteraction_0(self , rsthis: & QSessionManager) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QSessionManager17allowsInteractionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsessionmanager.h:69
// index:0
// Public Visibility=Default Availability=Available
// [1] bool allowsErrorInteraction()

/*
Returns true if error interaction is permitted; otherwise returns false.

This is similar to allowsInteraction(), but also enables the application to tell the user about any errors that occur. Session managers may give error interaction requests higher priority, which means that it is more likely that an error interaction is permitted. However, you are still not guaranteed that the session manager will allow interaction.

See also allowsInteraction(), release(), and cancel().
*/
impl /*struct*/ QSessionManager {
  pub fn allowsErrorInteraction_0<RetType, T: QSessionManager_allowsErrorInteraction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.allowsErrorInteraction_0(self);
    // return 1;
  }
}
pub trait QSessionManager_allowsErrorInteraction_0<RetType> {
  fn allowsErrorInteraction_0(self , rsthis: & QSessionManager) -> RetType;
}
impl<'a> /*trait*/ QSessionManager_allowsErrorInteraction_0<bool> for () {
  fn allowsErrorInteraction_0(self , rsthis: & QSessionManager) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QSessionManager22allowsErrorInteractionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsessionmanager.h:70
// index:0
// Public Visibility=Default Availability=Available
// [-2] void release()

/*
Releases the session manager's interaction semaphore after an interaction phase.

See also allowsInteraction() and allowsErrorInteraction().
*/
impl /*struct*/ QSessionManager {
  pub fn release_0<RetType, T: QSessionManager_release_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.release_0(self);
    // return 1;
  }
}
pub trait QSessionManager_release_0<RetType> {
  fn release_0(self , rsthis: & QSessionManager) -> RetType;
}
impl<'a> /*trait*/ QSessionManager_release_0<(/*void*/)> for () {
  fn release_0(self , rsthis: & QSessionManager) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QSessionManager7releaseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsessionmanager.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cancel()

/*
Tells the session manager to cancel the shutdown process. Applications should not call this function without asking the user first.

See also allowsInteraction() and allowsErrorInteraction().
*/
impl /*struct*/ QSessionManager {
  pub fn cancel_0<RetType, T: QSessionManager_cancel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cancel_0(self);
    // return 1;
  }
}
pub trait QSessionManager_cancel_0<RetType> {
  fn cancel_0(self , rsthis: & QSessionManager) -> RetType;
}
impl<'a> /*trait*/ QSessionManager_cancel_0<(/*void*/)> for () {
  fn cancel_0(self , rsthis: & QSessionManager) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QSessionManager6cancelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsessionmanager.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRestartHint(QSessionManager::RestartHint)

/*
Sets the application's restart hint to hint. On application startup, the hint is set to RestartIfRunning.

Note: These flags are only hints, a session manager may or may not respect them.

We recommend setting the restart hint in QGuiApplication::saveStateRequest() because most session managers perform a checkpoint shortly after an application's startup.

See also restartHint().
*/
impl /*struct*/ QSessionManager {
  pub fn setRestartHint_0<RetType, T: QSessionManager_setRestartHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRestartHint_0(self);
    // return 1;
  }
}
pub trait QSessionManager_setRestartHint_0<RetType> {
  fn setRestartHint_0(self , rsthis: & QSessionManager) -> RetType;
}
impl<'a> /*trait*/ QSessionManager_setRestartHint_0<(/*void*/)> for (i32) {
  fn setRestartHint_0(self , rsthis: & QSessionManager) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QSessionManager14setRestartHintENS_11RestartHintE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsessionmanager.h:81
// index:0
// Public Visibility=Default Availability=Available
// [4] QSessionManager::RestartHint restartHint() const

/*
Returns the application's current restart hint. The default is RestartIfRunning.

See also setRestartHint().
*/
impl /*struct*/ QSessionManager {
  pub fn restartHint_0<RetType, T: QSessionManager_restartHint_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.restartHint_0(self);
    // return 1;
  }
}
pub trait QSessionManager_restartHint_0<RetType> {
  fn restartHint_0(self , rsthis: & QSessionManager) -> RetType;
}
impl<'a> /*trait*/ QSessionManager_restartHint_0<i32> for () {
  fn restartHint_0(self , rsthis: & QSessionManager) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSessionManager11restartHintEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsessionmanager.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRestartCommand(const QStringList &)

/*
If the session manager is capable of restoring sessions it will execute command in order to restore the application. The command defaults to


  appname -session id



The -session option is mandatory; otherwise QGuiApplication cannot tell whether it has been restored or what the current session identifier is. See QGuiApplication::isSessionRestored() and QGuiApplication::sessionId() for details.

If your application is very simple, it may be possible to store the entire application state in additional command line options. This is usually a very bad idea because command lines are often limited to a few hundred bytes. Instead, use QSettings, temporary files, or a database for this purpose. By marking the data with the unique sessionId(), you will be able to restore the application in a future session.

See also restartCommand(), setDiscardCommand(), and setRestartHint().
*/
impl /*struct*/ QSessionManager {
  pub fn setRestartCommand_0<RetType, T: QSessionManager_setRestartCommand_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRestartCommand_0(self);
    // return 1;
  }
}
pub trait QSessionManager_setRestartCommand_0<RetType> {
  fn setRestartCommand_0(self , rsthis: & QSessionManager) -> RetType;
}
impl<'a> /*trait*/ QSessionManager_setRestartCommand_0<(/*void*/)> for (usize) {
  fn setRestartCommand_0(self , rsthis: & QSessionManager) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QSessionManager17setRestartCommandERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsessionmanager.h:84
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList restartCommand() const

/*
Returns the currently set restart command.

To iterate over the list, you can use the foreach pseudo-keyword:


  foreach (const QString &command, mySession.restartCommand())
      do_something(command);



See also setRestartCommand() and restartHint().
*/
impl /*struct*/ QSessionManager {
  pub fn restartCommand_0<RetType, T: QSessionManager_restartCommand_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.restartCommand_0(self);
    // return 1;
  }
}
pub trait QSessionManager_restartCommand_0<RetType> {
  fn restartCommand_0(self , rsthis: & QSessionManager) -> RetType;
}
impl<'a> /*trait*/ QSessionManager_restartCommand_0<usize> for () {
  fn restartCommand_0(self , rsthis: & QSessionManager) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSessionManager14restartCommandEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsessionmanager.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDiscardCommand(const QStringList &)

/*
Sets the discard command to the given command.

See also discardCommand() and setRestartCommand().
*/
impl /*struct*/ QSessionManager {
  pub fn setDiscardCommand_0<RetType, T: QSessionManager_setDiscardCommand_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDiscardCommand_0(self);
    // return 1;
  }
}
pub trait QSessionManager_setDiscardCommand_0<RetType> {
  fn setDiscardCommand_0(self , rsthis: & QSessionManager) -> RetType;
}
impl<'a> /*trait*/ QSessionManager_setDiscardCommand_0<(/*void*/)> for (usize) {
  fn setDiscardCommand_0(self , rsthis: & QSessionManager) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QSessionManager17setDiscardCommandERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsessionmanager.h:86
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList discardCommand() const

/*
Returns the currently set discard command.

To iterate over the list, you can use the foreach pseudo-keyword:


  foreach (const QString &command, mySession.discardCommand())
      do_something(command);



See also setDiscardCommand(), restartCommand(), and setRestartCommand().
*/
impl /*struct*/ QSessionManager {
  pub fn discardCommand_0<RetType, T: QSessionManager_discardCommand_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.discardCommand_0(self);
    // return 1;
  }
}
pub trait QSessionManager_discardCommand_0<RetType> {
  fn discardCommand_0(self , rsthis: & QSessionManager) -> RetType;
}
impl<'a> /*trait*/ QSessionManager_discardCommand_0<usize> for () {
  fn discardCommand_0(self , rsthis: & QSessionManager) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSessionManager14discardCommandEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsessionmanager.h:88
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setManagerProperty(const QString &, const QString &)

/*
Low-level write access to the application's identification and state record are kept in the session manager.

The property called name has its value set to the string list value.
*/
impl /*struct*/ QSessionManager {
  pub fn setManagerProperty_0<RetType, T: QSessionManager_setManagerProperty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setManagerProperty_0(self);
    // return 1;
  }
}
pub trait QSessionManager_setManagerProperty_0<RetType> {
  fn setManagerProperty_0(self , rsthis: & QSessionManager) -> RetType;
}
impl<'a> /*trait*/ QSessionManager_setManagerProperty_0<(/*void*/)> for (usize,usize) {
  fn setManagerProperty_0(self , rsthis: & QSessionManager) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QSessionManager18setManagerPropertyERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsessionmanager.h:89
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setManagerProperty(const QString &, const QStringList &)

/*
Low-level write access to the application's identification and state record are kept in the session manager.

The property called name has its value set to the string list value.
*/
impl /*struct*/ QSessionManager {
  pub fn setManagerProperty_1<RetType, T: QSessionManager_setManagerProperty_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setManagerProperty_1(self);
    // return 1;
  }
}
pub trait QSessionManager_setManagerProperty_1<RetType> {
  fn setManagerProperty_1(self , rsthis: & QSessionManager) -> RetType;
}
impl<'a> /*trait*/ QSessionManager_setManagerProperty_1<(/*void*/)> for (usize,usize) {
  fn setManagerProperty_1(self , rsthis: & QSessionManager) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QSessionManager18setManagerPropertyERK7QStringRK11QStringList", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsessionmanager.h:91
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isPhase2() const

/*
Returns true if the session manager is currently performing a second session management phase; otherwise returns false.

See also requestPhase2().
*/
impl /*struct*/ QSessionManager {
  pub fn isPhase2_0<RetType, T: QSessionManager_isPhase2_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isPhase2_0(self);
    // return 1;
  }
}
pub trait QSessionManager_isPhase2_0<RetType> {
  fn isPhase2_0(self , rsthis: & QSessionManager) -> RetType;
}
impl<'a> /*trait*/ QSessionManager_isPhase2_0<bool> for () {
  fn isPhase2_0(self , rsthis: & QSessionManager) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSessionManager8isPhase2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsessionmanager.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void requestPhase2()

/*
Requests a second session management phase for the application. The application may then return immediately from the QGuiApplication::commitDataRequest() or QApplication::saveStateRequest() function, and they will be called again once most or all other applications have finished their session management.

The two phases are useful for applications such as the X11 window manager that need to store information about another application's windows and therefore have to wait until these applications have completed their respective session management tasks.

Note: If another application has requested a second phase it may get called before, simultaneously with, or after your application's second phase.

See also isPhase2().
*/
impl /*struct*/ QSessionManager {
  pub fn requestPhase2_0<RetType, T: QSessionManager_requestPhase2_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.requestPhase2_0(self);
    // return 1;
  }
}
pub trait QSessionManager_requestPhase2_0<RetType> {
  fn requestPhase2_0(self , rsthis: & QSessionManager) -> RetType;
}
impl<'a> /*trait*/ QSessionManager_requestPhase2_0<(/*void*/)> for () {
  fn requestPhase2_0(self , rsthis: & QSessionManager) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QSessionManager13requestPhase2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQSessionManager(this :*mut QSessionManager) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN15QSessionManagerD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum type defines the circumstances under which this application wants to be restarted by the session manager. The current values are:



The default hint is RestartIfRunning.

*/
pub type QSessionManager__RestartHint = i32;
// If the application is still running when the session is shut down, it wants to be restarted at the start of the next session.
pub const QSessionManager__RestartIfRunning :QSessionManager__RestartHint = 0;
// The application wants to be started at the start of the next session, no matter what. (This is useful for utilities that run just after startup and then quit.)
pub const QSessionManager__RestartAnyway :QSessionManager__RestartHint = 1;
// The application wants to be started immediately whenever it is not running.
pub const QSessionManager__RestartImmediately :QSessionManager__RestartHint = 2;
// The application does not want to be restarted automatically.
pub const QSessionManager__RestartNever :QSessionManager__RestartHint = 3;
pub fn QSessionManager_RestartHintItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QSessionManager", val);
}
pub fn QSessionManager_RestartHintItemName_s(val: i32) ->String {
  //var nilthis *QSessionManager
  //return nilthis.RestartHintItemName(val);
  return QSessionManager_RestartHintItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
